use std::{
    io::Write,
    path::{Path, PathBuf},
    time::{Instant, SystemTime},
};

use log::{debug, info};

// Cache-dir resolution lives in the shared `diskcache` core; re-export so
// existing callers (`luau`, `validate`, `template`, `describegpt`) keep using
// `lookup::set_qsv_cache_dir`.
pub use crate::diskcache::set_qsv_cache_dir;
use crate::{diskcache, util};

/// The tabular extension to give a (remote) lookup table's cache file, derived
/// from the source `uri` so it is stable across fresh downloads and cache hits.
/// A compressed source's inner tabular extension wins (e.g. `foo.tsv.gz` → `tsv`),
/// so `Config` picks the right delimiter from the cached file's extension when the
/// caller did not specify one. `.zip` inner content can't be known from the URI,
/// so it (and anything unrecognized) defaults to `csv`.
fn lookup_cache_ext(uri: &str) -> &'static str {
    const COMPRESSION: [&str; 5] = ["zip", "sz", "gz", "zlib", "zst"];
    let path_part = uri.split(['?', '#']).next().unwrap_or(uri);
    let p = Path::new(path_part);
    let outer = p
        .extension()
        .and_then(|e| e.to_str())
        .map(str::to_ascii_lowercase);
    let candidate = match outer.as_deref() {
        // compressed: look at the stem's extension (foo.tsv.gz -> tsv)
        Some(o) if COMPRESSION.contains(&o) => Path::new(p.file_stem().unwrap_or_default())
            .extension()
            .and_then(|e| e.to_str())
            .map(str::to_ascii_lowercase),
        other => other.map(str::to_string),
    };
    match candidate.as_deref() {
        Some("tsv") => "tsv",
        Some("tab") => "tab",
        Some("ssv") => "ssv",
        _ => "csv",
    }
}

/// Tabular cache-file extensions a lookup table can resolve to. `csv` is the
/// default (and the "inner unknown" sentinel `lookup_cache_ext` returns for a
/// `.zip`, a `ckan://`/`dathere://` source, or a bare/unknown extension).
const TABULAR_EXTS: [&str; 4] = ["csv", "tsv", "tab", "ssv"];

/// The canonical tabular cache-file extension for a `decompress_source`
/// `inner_ext`, defaulting to `csv` for csv/unknown/none.
fn cache_ext_for_inner(inner_ext: Option<&str>) -> &'static str {
    match inner_ext.map(str::to_ascii_lowercase).as_deref() {
        Some("tsv") => "tsv",
        Some("tab") => "tab",
        Some("ssv") => "ssv",
        _ => "csv",
    }
}

/// The cache-file path to actually write, given the provisional `cache_file_path`
/// (whose stem is `{name}`) and the `inner_ext` discovered during decompression.
/// The extension is set explicitly from `inner_ext` — *including* resetting back
/// to `csv` when a previously non-csv source is refreshed with a csv inner file —
/// so the cached file's extension always matches its real delimiter and `Config`
/// infers it correctly.
fn corrected_cache_path(cache_file_path: &Path, inner_ext: Option<&str>) -> PathBuf {
    cache_file_path.with_extension(cache_ext_for_inner(inner_ext))
}

/// Find an existing cached lookup file for `name` among the tabular extensions.
/// Used when the cache file's real extension can't be known from the URL — a
/// `.zip`'s inner entry, or a `ckan://`/`dathere://` source that resolves to an
/// arbitrary data URL — so it may have been written under a non-`csv` extension
/// discovered at download time. Steady state holds a single file per name (the
/// downloader removes the provisional one), so the probe order is not significant.
fn existing_tabular_cache_file(cache_dir: &Path, name: &str) -> Option<PathBuf> {
    TABULAR_EXTS
        .iter()
        .map(|ext| cache_dir.join(format!("{name}.{ext}")))
        .find(|p| p.exists())
}

pub struct LookupTableOptions {
    pub name:           String,
    pub uri:            String,
    pub cache_age_secs: i64,
    pub cache_dir:      String,
    pub delimiter:      Option<crate::config::Delimiter>,
    pub ckan_api_url:   Option<String>,
    pub ckan_token:     Option<String>,
    pub timeout_secs:   u16,
}

pub struct LookupTableResult {
    pub filepath: String,
    pub headers:  csv::StringRecord,
    pub rowcount: usize,
}

/// Loads a lookup table from a local file, cache, or remote source.
///
/// # Arguments
///
/// * `opts` - Options for loading the lookup table, including:
///   - `name`: Name of the lookup table
///   - `uri`: URI/path to the lookup table file (http/https/ckan/dathere schemes supported)
///   - `cache_age_secs`: How long to keep cached files (negative to delete cache)
///   - `cache_dir`: Directory to store cached files
///   - `delimiter`: Optional CSV delimiter
///   - `ckan_api_url`: Optional CKAN API URL for CKAN resources
///   - `ckan_token`: Optional CKAN API token
///   - `timeout_secs`: Timeout in seconds for HTTP requests
///
/// # Returns
///
/// Returns a `LookupTableResult` containing:
/// - `filepath`: Path to the loaded lookup table file
/// - `headers`: CSV headers from the lookup table
///
/// # Functionality
///
/// 1. Checks if lookup table exists as local file
/// 2. If not local, checks cache:
///    - Uses cache if valid and not expired
///    - Deletes cache if `cache_age_secs` is negative
/// 3. For remote files:
///    - Handles dathere:// prefix for GitHub lookup tables
///    - Handles ckan:// prefix for CKAN resources
///    - Downloads HTTP(S) URLs to cache
/// 4. Reads and returns headers from the lookup table
///
/// # Errors
///
/// Returns error if:
/// - File operations fail (create/delete/read)
/// - Remote downloads fail
/// - CSV parsing fails
pub fn load_lookup_table(
    opts: &LookupTableOptions,
) -> Result<LookupTableResult, Box<dyn std::error::Error>> {
    let mut lookup_table_uri = opts.uri.clone();
    // Name the cache file with the source's (inner) tabular extension so a
    // decompressed remote `.tsv.gz`/`.ssv.gz` is read with the right delimiter,
    // consistently on both fresh download and cache hit. Defaults to `csv`.
    let cache_ext = lookup_cache_ext(&opts.uri);
    let cache_dir = Path::new(&opts.cache_dir);
    let default_path = cache_dir.join(format!("{}.{cache_ext}", opts.name));
    // The cached file's extension may differ from the URL-derived default when
    // the real (inner) tabular extension is only knowable after download — a
    // `.zip`'s inner entry, or a `ckan://`/`dathere://` source that resolves to
    // an arbitrary data URL. In those cases `lookup_cache_ext` returns the `csv`
    // default; if that file isn't present, probe for one the downloader wrote
    // under a discovered non-csv extension (tsv/tab/ssv) so cache hits — and
    // therefore `cache_age_secs` — still work instead of re-downloading. A
    // non-csv `cache_ext` came from an explicit URL extension, so it is exact.
    let cached_csv_path = if cache_ext == "csv" && !default_path.exists() {
        existing_tabular_cache_file(cache_dir, &opts.name).unwrap_or(default_path)
    } else {
        default_path
    };

    // Check if local file
    let lookup_table_path = Path::new(&lookup_table_uri);
    let lookup_table_is_file = lookup_table_path.exists();

    // Check cache status
    let (cached_csv_exists, cached_csv_age_secs, cached_csv_size, cache_csv_last_modified) =
        if cached_csv_path.exists() {
            if opts.cache_age_secs < 0 {
                // Delete cached file if negative cache age
                std::fs::remove_file(&cached_csv_path)?;
                (false, 0, 0, None)
            } else {
                let metadata = cached_csv_path.metadata()?;
                let last_modified = metadata.modified()?;
                let modified_secs = last_modified
                    .duration_since(SystemTime::UNIX_EPOCH)?
                    .as_secs();
                let now_secs = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)?
                    .as_secs();
                let age = if opts.cache_age_secs > 0 {
                    // saturating_sub guards against clock skew / future mtimes
                    // (NTP correction, restored caches, etc.) that would otherwise
                    // underflow this u64 subtraction.
                    now_secs
                        .saturating_sub(modified_secs)
                        .try_into()
                        .unwrap_or(i64::MAX)
                } else {
                    0_i64
                };
                (true, age, metadata.len(), Some(last_modified))
            }
        } else {
            (false, 0, 0, None)
        };

    // Use cached file if valid.
    //
    // Note: `cached_csv_age_secs` is derived from the cache file's mtime,
    // and a successful 304 response refreshes that mtime (see
    // `download_lookup_table`). Effect: a cache that keeps validating
    // against upstream stays "fresh" indefinitely from this check's
    // perspective — `cache_age_secs` bounds *re-validation cadence*,
    // not *absolute cache lifetime*.
    if !lookup_table_is_file
        && cached_csv_exists
        && cached_csv_age_secs <= opts.cache_age_secs
        && cached_csv_size > 0
    {
        lookup_table_uri = cached_csv_path.display().to_string();
        info!("Using cached lookup table {lookup_table_uri}");
    } else if !lookup_table_is_file {
        // Handle remote files: expand dathere:// / ckan:// prefixes via the
        // shared resolver so there is a single, audited resolution code path
        // (used by both this lookup module and the `get` command).
        let resolved =
            diskcache::resolve_uri_prefix(&lookup_table_uri, opts.ckan_api_url.as_deref());
        lookup_table_uri = resolved.url;

        let lookup_on_url = lookup_table_uri.to_lowercase().starts_with("http");

        if lookup_on_url {
            // The downloader returns the path it actually wrote: for a `.zip`
            // the provisional `.csv` target may be corrected to the discovered
            // inner tabular extension (tsv/tab/ssv).
            let written_path = download_lookup_table(
                &lookup_table_uri,
                &cached_csv_path,
                resolved.is_ckan,
                resolved.ckan_resource_search,
                cache_csv_last_modified,
                opts,
            )?;
            // download_lookup_table either writes/refreshes the cache file or
            // returns Err. Verify the file is actually present before reading
            // it; otherwise surface a clear error rather than a downstream
            // CSV "file not found".
            if !written_path.exists() {
                return Err(format!(
                    "Lookup table download from {lookup_table_uri} produced no cached file at {}",
                    written_path.display()
                )
                .into());
            }
            lookup_table_uri = written_path.to_string_lossy().to_string();
        }
    }

    // Read headers from the lookup table
    let conf = crate::config::Config::new(Some(lookup_table_uri.clone()).as_ref())
        .delimiter(opts.delimiter)
        .comment(Some(b'#'))
        .no_headers(false);

    let mut rdr = conf.reader()?;
    let headers = rdr.headers()?.clone();
    let rowcount = util::count_rows(&conf).unwrap_or_default() as usize;

    let lur = LookupTableResult {
        filepath: lookup_table_uri,
        headers,
        rowcount,
    };

    drop(rdr);

    Ok(lur)
}

fn download_lookup_table(
    lookup_table_uri: &str,
    cache_file_path: &Path,
    lookup_ckan: bool,
    resource_search: bool,
    cache_csv_last_modified: Option<SystemTime>,
    opts: &LookupTableOptions,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let client = crate::util::create_reqwest_blocking_client(
        None,
        opts.timeout_secs,
        Some(lookup_table_uri.to_string()),
    )
    .map_err(|e| Box::new(std::io::Error::other(e.to_string())))?;

    let download_start = Instant::now();
    let downloaded_at_dt: chrono::DateTime<chrono::Utc> = SystemTime::now().into();
    let downloaded_at_rfc2822 = downloaded_at_dt.to_rfc2822();

    // Also capture the URL actually fetched (the CKAN data URL for ckan://) so
    // compression detection sees the real file extension, not the ckan:// alias.
    let (lookup_csv_response, fetched_url) = if lookup_ckan {
        // Resolve the CKAN resource to its actual data URL (and whether the
        // bearer token may be sent), then fetch it.
        let ckan = diskcache::resolve_ckan_resource(
            &client,
            lookup_table_uri,
            resource_search,
            opts.ckan_api_url.as_deref(),
            opts.ckan_token.as_deref(),
        )?;
        let auth = if ckan.send_auth {
            opts.ckan_token.as_deref()
        } else {
            None
        };
        let resp = diskcache::http_get_conditional(&client, &ckan.data_url, None, auth)?;
        (resp, ckan.data_url)
    } else {
        let resp = diskcache::http_get_conditional(
            &client,
            lookup_table_uri,
            cache_csv_last_modified,
            None,
        )?;
        (resp, lookup_table_uri.to_string())
    };

    let status = lookup_csv_response.status();
    if status == reqwest::StatusCode::NOT_MODIFIED {
        debug!("Lookup CSV hasn't changed, so using cached CSV.");
        // Refresh the cache file's mtime so we don't re-issue a conditional
        // GET on every subsequent call. Best-effort; ignore platform errors.
        if cache_file_path.exists()
            && let Ok(file) = std::fs::OpenOptions::new()
                .write(true)
                .open(cache_file_path)
        {
            let times = std::fs::FileTimes::new().set_modified(SystemTime::now());
            let _ = file.set_times(times);
        }
        return Ok(cache_file_path.to_path_buf());
    }
    if !status.is_success() {
        return Err(format!(
            "Failed to download lookup table from {lookup_table_uri}: HTTP {status}"
        )
        .into());
    }

    // Read the raw bytes (NOT .text(), which would corrupt a compressed/binary
    // body), then decompress based on the fetched URL's extension. `.zip`/`.sz`
    // always work; `.gz`/`.zlib`/`.zst` need the relevant codec feature.
    let raw_body = lookup_csv_response.bytes()?;
    if raw_body.is_empty() {
        // Misconfigured CDN edges sometimes return 200 with Content-Length: 0.
        // Fall back to the existing cache when one is present; only error
        // when there is no cache to fall back to.
        if cache_file_path.exists() {
            debug!(
                "Lookup table download from {lookup_table_uri} returned an empty body; keeping \
                 existing cache file."
            );
            return Ok(cache_file_path.to_path_buf());
        }
        return Err(format!(
            "Lookup table download from {lookup_table_uri} returned an empty response and no \
             cache exists to fall back to"
        )
        .into());
    }

    let decompressed = diskcache::decompress_source(&fetched_url, raw_body.to_vec())
        .map_err(|e| std::io::Error::other(e.to_string()))?;

    // The cache file must carry the inner tabular extension so `Config` infers
    // the right delimiter. For most sources that already equals
    // `cache_file_path`'s URL-derived extension; for a `.zip` the inner ext is
    // only known now (post-extraction), so correct the provisional `.csv` path.
    let written_path = corrected_cache_path(cache_file_path, decompressed.inner_ext.as_deref());

    write_cache_file(
        &written_path,
        &decompressed.bytes,
        &downloaded_at_rfc2822,
        download_start,
        opts,
    )?;

    // Drop the provisional file written under the URL-derived extension (e.g. a
    // prior `.csv` default for this `.zip`) so the cache holds a single,
    // unambiguous file for this name. Best-effort.
    if written_path != *cache_file_path && cache_file_path.exists() {
        let _ = std::fs::remove_file(cache_file_path);
    }

    Ok(written_path)
}

fn write_cache_file(
    cache_file_path: &Path,
    contents: &[u8],
    downloaded_at: &str,
    download_start: Instant,
    opts: &LookupTableOptions,
) -> Result<(), Box<dyn std::error::Error>> {
    info!(
        "Writing lookup CSV to cache file: {}",
        cache_file_path.display()
    );
    let cache_file_handle = std::fs::File::create(cache_file_path)?;
    let mut cache_file = std::io::BufWriter::new(cache_file_handle);

    writeln!(
        cache_file,
        "# qsv_register_lookup({}, {}, {})",
        opts.name, opts.uri, opts.cache_age_secs
    )?;
    writeln!(cache_file, "# Downloaded-At: {downloaded_at}")?;
    writeln!(
        cache_file,
        "# Download-duration-ms: {}",
        download_start.elapsed().as_millis()
    )?;
    cache_file.write_all(contents)?;
    cache_file.flush()?;
    drop(cache_file);

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{corrected_cache_path, existing_tabular_cache_file, lookup_cache_ext};

    #[test]
    fn corrected_cache_path_sets_ext_from_inner() {
        let provisional = Path::new("/cache/mytable.csv");
        // A non-csv inner entry (tsv/tab/ssv) corrects the provisional .csv.
        assert_eq!(
            corrected_cache_path(provisional, Some("tsv")),
            Path::new("/cache/mytable.tsv")
        );
        assert_eq!(
            corrected_cache_path(provisional, Some("TAB")), // case-insensitive
            Path::new("/cache/mytable.tab")
        );
        assert_eq!(
            corrected_cache_path(provisional, Some("ssv")),
            Path::new("/cache/mytable.ssv")
        );
        // csv / unknown / none -> csv (the default).
        assert_eq!(corrected_cache_path(provisional, Some("csv")), provisional);
        assert_eq!(corrected_cache_path(provisional, Some("json")), provisional);
        assert_eq!(corrected_cache_path(provisional, None), provisional);
        // Refreshing a previously non-csv cache file back to a csv inner file
        // must RESET the extension to csv — otherwise csv content would be
        // written under a stale .tsv name and mis-parsed as tab-delimited.
        assert_eq!(
            corrected_cache_path(Path::new("/cache/mytable.tsv"), Some("csv")),
            Path::new("/cache/mytable.csv")
        );
        assert_eq!(
            corrected_cache_path(Path::new("/cache/mytable.tsv"), None),
            Path::new("/cache/mytable.csv")
        );
        // A name containing dots keeps its stem; only the final ext changes.
        assert_eq!(
            corrected_cache_path(Path::new("/cache/my.data.csv"), Some("tsv")),
            Path::new("/cache/my.data.tsv")
        );
    }

    #[test]
    fn existing_tabular_cache_file_finds_written_candidate() {
        let dir = tempfile::tempdir().unwrap();
        // No candidate yet.
        assert!(existing_tabular_cache_file(dir.path(), "boston").is_none());
        // The downloader wrote the corrected .tsv file; the cache-hit probe must
        // find it even though the URL (.zip / ckan://) implies a .csv default.
        let tsv = dir.path().join("boston.tsv");
        std::fs::write(&tsv, b"a\tb\n1\t2\n").unwrap();
        assert_eq!(existing_tabular_cache_file(dir.path(), "boston"), Some(tsv));
    }

    #[test]
    fn lookup_cache_ext_plain_and_compressed() {
        // plain tabular: outer extension is the tabular one
        assert_eq!(lookup_cache_ext("https://x/data.csv"), "csv");
        assert_eq!(lookup_cache_ext("https://x/data.tsv"), "tsv");
        assert_eq!(lookup_cache_ext("https://x/data.ssv"), "ssv");
        // compressed: the stem's tabular extension wins over the codec extension
        assert_eq!(lookup_cache_ext("https://x/data.csv.gz"), "csv");
        assert_eq!(lookup_cache_ext("https://x/data.tsv.gz"), "tsv");
        assert_eq!(lookup_cache_ext("https://x/data.ssv.zst"), "ssv");
        assert_eq!(lookup_cache_ext("https://x/data.tsv.zlib"), "tsv");
        assert_eq!(lookup_cache_ext("https://x/data.csv.sz"), "csv");
        // query/fragment are ignored
        assert_eq!(
            lookup_cache_ext("https://x/data.tsv.gz?token=abc#frag"),
            "tsv"
        );
        // unknown / non-tabular inner / bare compression / no extension -> csv
        assert_eq!(lookup_cache_ext("https://x/data.gz"), "csv");
        assert_eq!(lookup_cache_ext("https://x/data.zip"), "csv"); // inner unknown from URL
        assert_eq!(lookup_cache_ext("https://x/data.json.gz"), "csv");
        assert_eq!(lookup_cache_ext("https://x/data"), "csv");
    }
}
