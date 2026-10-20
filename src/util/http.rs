use std::{
    cmp::min,
    env,
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
    time::Duration,
};

#[cfg(any(feature = "feature_capable", feature = "lite"))]
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use log::log_enabled;
use reqwest::Client;

#[cfg(any(feature = "feature_capable", feature = "lite"))]
use super::get_envvar_flag;
use super::{CARGO_BIN_NAME, CARGO_PKG_VERSION, QSV_KIND, TARGET, timeout_secs};
use crate::{CURRENT_COMMAND, CliError, CliResult, config::DEFAULT_WTR_BUFFER_CAPACITY};

pub(super) fn default_user_agent() -> String {
    let unknown_command = "Unknown".to_string();
    let current_command = CURRENT_COMMAND.get().unwrap_or(&unknown_command);
    format!(
        "{CARGO_BIN_NAME}/{CARGO_PKG_VERSION} ({TARGET}; {current_command}; {QSV_KIND}; https://github.com/dathere/qsv)"
    )
}

/// Strip an API key out of any text that may quote a request URL.
///
/// NOT optional hygiene: `reqwest`'s own error `Display` embeds the full URL, query string
/// included, so interpolating an error into a message prints a `key=` credential to stderr or
/// the log verbatim. A key is a credential — it must not reach a log, a terminal, or a bug
/// report — so every error built from a key-bearing URL or an underlying error goes through here.
#[cfg(any(feature = "geocode", all(feature = "viz", feature = "feature_capable")))]
pub fn redact_api_key(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(at) = rest.find("key=") {
        // Redact unless "key=" is the tail of a longer word ("monkey="). Deliberately NOT
        // "preceded by ? or &": a wrapped or reformatted log line puts whitespace there, and a
        // credential that survives because a line broke in the wrong place is not protected.
        let is_param = at == 0
            || !matches!(
                rest.as_bytes()[at - 1],
                b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' | b'-'
            );
        out.push_str(&rest[..at + 4]);
        rest = &rest[at + 4..];
        if !is_param {
            continue;
        }
        let end = rest
            .find(|c: char| c == '&' || c == ')' || c == '\'' || c.is_whitespace())
            .unwrap_or(rest.len());
        out.push_str("REDACTED");
        rest = &rest[end..];
    }
    out.push_str(rest);
    out
}

/// sets custom user agent
/// if user agent is not set, then use the default user agent
/// it supports four special LITERALs: $`QSV_BIN_NAME`, $`QSV_VERSION`, $`QSV_TARGET`, $`QSV_KIND`
/// and $`QSV_COMMAND` which will be replaced with the actual values during runtime
pub fn set_user_agent(user_agent: Option<String>) -> CliResult<String> {
    use reqwest::header::HeaderValue;

    let ua = match user_agent {
        Some(ua_arg) => ua_arg,
        None => env::var("QSV_USER_AGENT").unwrap_or_else(|_| default_user_agent()),
    };

    let unknown_command = "Unknown".to_string();
    let current_command = CURRENT_COMMAND.get().unwrap_or(&unknown_command);

    // look for special literals - $QSV_VERSION and $QSV_TARGET and replace them
    let ua = ua
        .replace("$QSV_BIN_NAME", CARGO_BIN_NAME)
        .replace("$QSV_VERSION", CARGO_PKG_VERSION)
        .replace("$QSV_TARGET", TARGET)
        .replace("$QSV_KIND", QSV_KIND)
        .replace("$QSV_COMMAND", current_command);

    match HeaderValue::from_str(ua.as_str()) {
        Ok(_) => (),
        Err(e) => return fail_incorrectusage_clierror!("Invalid user-agent value: {e}"),
    }

    log::info!("set user agent: {ua}");
    Ok(ua)
}

/// Creates a standardized async reqwest client with common configuration options.
/// This function centralizes the client creation logic used across multiple commands.
///
/// # Arguments
///
/// * `user_agent` - Optional custom user agent string
/// * `timeout_secs` - Inactivity (idle) timeout in seconds for HTTP requests. If 0, no timeout.
/// * `base_url` - Optional base URL for retry configuration
///
/// # Returns
///
/// Returns a configured reqwest client with:
/// - Custom user agent (or default)
/// - Compression support (brotli, gzip, deflate, zstd)
/// - Rustls TLS backend
/// - HTTP/2 adaptive window
/// - Connection verbose logging (when debug/trace enabled)
/// - Idle/read timeout configuration (NOT a total-request deadline)
/// - Retry logic for service unavailable errors
pub fn create_reqwest_async_client(
    user_agent: Option<String>,
    timeout_secs: u16,
    base_url: Option<String>,
) -> CliResult<Client> {
    let base_url_for_retry = base_url.unwrap_or_default();

    let retries = reqwest::retry::for_host(base_url_for_retry).classify_fn(|req_rep| {
        if req_rep.status() == Some(reqwest::StatusCode::SERVICE_UNAVAILABLE) {
            req_rep.retryable()
        } else {
            req_rep.success()
        }
    });

    let mut builder = Client::builder()
        .user_agent(set_user_agent(user_agent)?)
        .brotli(true)
        .gzip(true)
        .deflate(true)
        .zstd(true)
        .use_rustls_tls()
        .http2_adaptive_window(true)
        .connection_verbose(log_enabled!(log::Level::Debug) || log_enabled!(log::Level::Trace))
        .retry(retries);

    if timeout_secs > 0 {
        // Use an INACTIVITY (idle) timeout, not a total-request deadline. `read_timeout`
        // applies to each read of the response body and RESETS after every successful read,
        // so a slow-but-steady download (e.g. a large CSV trickling in over a slow proxy) is
        // never aborted mid-stream — only a genuinely stalled connection (no bytes for
        // `timeout_secs`) trips it. `connect_timeout` still bounds a stalled connect/TLS
        // handshake (not reliably caught otherwise on flaky windows-arm64 CI runners).
        // There is deliberately no total-request `.timeout()` cap.
        let timeout_duration = Duration::from_secs(timeout_secs.into());
        builder = builder
            .read_timeout(timeout_duration)
            .connect_timeout(timeout_duration);
    }

    Ok(builder.build()?)
}

/// Creates a standardized blocking reqwest client with common configuration options.
/// This function centralizes the blocking client creation logic used across multiple commands.
///
/// # Arguments
///
/// * `user_agent` - Optional custom user agent string
/// * `timeout_secs` - Timeout in seconds for HTTP requests. If 0, no timeout is used.
/// * `base_url` - Optional base URL for retry configuration
///
/// # Returns
///
/// Returns a configured blocking reqwest client with the same options
/// as `create_reqwest_async_client`
///
/// NOTE: unlike the async client, this uses a TOTAL-request `.timeout()`, not an inactivity
/// (`read_timeout`) one. reqwest 0.13.4's blocking client reads the response body on the
/// calling thread via a park/unpark waker with NO Tokio timer running, so a `read_timeout`
/// body wrapper panics ("there is no reactor running") the moment the body is read. The
/// idle-timeout semantics therefore apply only to `create_reqwest_async_client`.
pub fn create_reqwest_blocking_client(
    user_agent: Option<String>,
    timeout_secs: u16,
    base_url: Option<String>,
) -> CliResult<reqwest::blocking::Client> {
    let timeout_duration = Duration::from_secs(timeout_secs.into());
    let base_url_for_retry = base_url.unwrap_or_default();

    let retries = reqwest::retry::for_host(base_url_for_retry).classify_fn(|req_rep| {
        if req_rep.status() == Some(reqwest::StatusCode::SERVICE_UNAVAILABLE) {
            req_rep.retryable()
        } else {
            req_rep.success()
        }
    });

    let mut builder = reqwest::blocking::Client::builder()
        .user_agent(set_user_agent(user_agent)?)
        .brotli(true)
        .gzip(true)
        .deflate(true)
        .zstd(true)
        .use_rustls_tls()
        .http2_adaptive_window(true)
        .connection_verbose(log_enabled!(log::Level::Debug) || log_enabled!(log::Level::Trace))
        .retry(retries);

    if timeout_secs > 0 {
        // bound the total request AND the connect/TLS handshake separately.
        // a stalled connect is not reliably caught by the total timeout alone
        // (e.g. flaky windows-arm64 CI runners), so set connect_timeout too.
        builder = builder
            .timeout(timeout_duration)
            .connect_timeout(timeout_duration);
    } else {
        // `timeout_secs == 0` means "no timeout" to every caller of this function (see
        // callers computing 0 to disable timeouts for local LLM endpoints), but
        // reqwest::blocking::ClientBuilder does NOT default to "no timeout" when
        // `.timeout()` is simply never called: its internal `Timeout` wrapper has its own
        // `Default` impl of `Some(Duration::from_secs(30))` (reqwest 0.13.4
        // src/blocking/client.rs), independent of the inner async client's `Config` (which
        // DOES default to `None`/no timeout). Skipping `.timeout()` here silently left
        // that 30s default active, killing legitimate long-running local-LLM completions
        // (e.g. describegpt --dictionary against LM Studio) exactly 30s in. Explicitly
        // passing `None` overrides it.
        builder = builder.timeout(None);
    }

    Ok(builder.build()?)
}

/// Transforms a GitHub blob URL to a raw.githubusercontent.com URL.
/// Returns the original URL unchanged if not a GitHub blob URL.
///
/// Query parameters (e.g., `?ref=v1.0`) and fragments (e.g., `#L10-L20`) are stripped
/// from the transformed URL since they don't apply to raw.githubusercontent.com.
///
/// Note: This only works for github.com URLs. GitHub Enterprise URLs
/// (e.g., `https://github.company.com/...`) are not transformed.
///
/// # Arguments
///
/// * `url` - The URL to potentially transform
///
/// # Returns
///
/// A String containing the transformed URL (for GitHub blob URLs)
/// or the original URL (for all other URLs).
///
/// # Examples
///
/// ```
/// // GitHub blob URLs are transformed to raw URLs:
/// // https://github.com/user/repo/blob/branch/path/file.csv
/// // becomes:
/// // https://raw.githubusercontent.com/user/repo/branch/path/file.csv
///
/// // Non-GitHub URLs are returned unchanged
/// ```
#[inline]
pub fn transform_github_url(url: &str) -> String {
    // Pattern: https://github.com/USER/REPO/blob/BRANCH/PATH
    // Transform: https://raw.githubusercontent.com/USER/REPO/BRANCH/PATH

    let github_prefix_https = "https://github.com/";
    let github_prefix_http = "http://github.com/";

    let remainder = if let Some(r) = url.strip_prefix(github_prefix_https) {
        r
    } else if let Some(r) = url.strip_prefix(github_prefix_http) {
        r
    } else {
        return url.to_string();
    };

    if let Some(blob_idx) = remainder.find("/blob/") {
        let user_repo = &remainder[..blob_idx];
        let mut branch_and_path = &remainder[blob_idx + 6..]; // skip "/blob/"

        // Strip query parameters and fragments - they don't apply to raw.githubusercontent.com
        if let Some(query_idx) = branch_and_path.find('?') {
            branch_and_path = &branch_and_path[..query_idx];
        }
        if let Some(fragment_idx) = branch_and_path.find('#') {
            branch_and_path = &branch_and_path[..fragment_idx];
        }

        let raw_url = format!("https://raw.githubusercontent.com/{user_repo}/{branch_and_path}");
        log::info!("Transformed GitHub blob URL to raw URL: {raw_url}");
        raw_url
    } else {
        url.to_string()
    }
}

/// downloads a file from a url and saves it to a path
/// if `show_progress` is true, a progress bar will be shown
/// if `custom_user_agent` is Some, it will be used as the user agent
/// if `download_timeout` is Some, it will be used as the timeout in seconds. The
/// value is validated by `timeout_secs` and must be in the range 1..=3600;
/// values of 0 or > 3600 cause this function to return an error. If
/// `sample_size` is Some, it will be used as the number of bytes to download.
pub async fn download_file(
    url: &str,
    path: PathBuf,
    #[allow(unused_variables)] show_progress: bool,
    custom_user_agent: Option<String>,
    download_timeout: Option<u16>,
    sample_size: Option<u64>,
) -> CliResult<()> {
    use futures_util::StreamExt;

    let download_timeout = match download_timeout {
        Some(t) => std::time::Duration::from_secs(
            timeout_secs(t)
                .map_err(|e| CliError::Other(format!("Invalid download timeout: {e}")))?,
        ),
        None => std::time::Duration::from_secs(30),
    };

    // setup the reqwest client. create_reqwest_async_client calls
    // set_user_agent internally, so pass custom_user_agent through.
    let client = create_reqwest_async_client(
        custom_user_agent,
        download_timeout.as_secs() as u16,
        Some(url.to_string()),
    )?;

    let res = client.get(url).send().await?;

    // A non-2xx response still HAS a body, and streaming it to disk as if it were the payload is
    // how a "Not Found" page ends up mistaken for the file that was asked for - a Geonames index,
    // a JSON Schema, a CSV. Every caller here downloads something it will then parse, so there is
    // no caller for which an error body is useful. Fail here, naming the status, instead of
    // failing much later and much less usefully on the contents.
    // `is_success()` rather than `error_for_status`, which only rejects 4xx/5xx: a FINAL 3xx that
    // was not followed (a 304, or a redirect the client stopped at) would otherwise be streamed to
    // disk as the payload, which is the same defect wearing a different status code.
    if !res.status().is_success() {
        let status = res.status();
        let reason = status.canonical_reason().unwrap_or("unexpected status");
        return fail_clierror!(
            "Download failed for {url} - the server returned {} {reason}.",
            status.as_u16()
        );
    }

    // if we can't get the content length, set it to sentinel value
    let total_size = res.content_length().unwrap_or(u64::MAX);

    // progressbar setup
    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    let show_progress = (show_progress || get_envvar_flag("QSV_PROGRESSBAR")) && total_size > 0;

    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    let pb = ProgressBar::with_draw_target(Some(total_size), ProgressDrawTarget::stderr_with_hz(5));

    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    if show_progress {
        pb.set_style(
            #[allow(clippy::to_string_in_format_args)]
            #[allow(clippy::literal_string_with_formatting_args)]
            ProgressStyle::default_bar()
                .template(if total_size == u64::MAX {
                    // only do a spinner if we don't know the total size
                    "{msg}\n{spinner:.green} ({bytes_per_sec})"
                } else {
                    "{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.white/blue}] \
                     {bytes}/{total_bytes} ({bytes_per_sec}, {eta})"
                })
                .unwrap(),
        );
        pb.set_message(format!("Downloading {url}"));
    } else {
        pb.set_draw_target(ProgressDrawTarget::hidden());
    }

    let sample_size = sample_size.unwrap_or(0);

    // download chunks
    let mut file = BufWriter::with_capacity(DEFAULT_WTR_BUFFER_CAPACITY, File::create(path)?);
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk)?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;

        #[cfg(any(feature = "feature_capable", feature = "lite"))]
        if show_progress {
            pb.set_position(new);
        }

        if sample_size > 0 && downloaded >= sample_size {
            break;
        }
    }

    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    if show_progress {
        pb.finish_with_message(format!("Downloaded {url}"));
        eprintln!(); // newline after progress bar
    }

    Ok(file.flush()?)
}
