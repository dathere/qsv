use std::env;
#[cfg(feature = "self_update")]
use std::{fs, time::Duration};

use log::log_enabled;
#[cfg(feature = "self_update")]
use sysinfo::System;

use super::get_envvar_flag;
#[cfg(feature = "self_update")]
use super::{
    QSV_KIND, TARGET, UPDATE_CHECK_INTERVAL, create_reqwest_blocking_client, current_exe,
    default_user_agent, expand_tilde,
};
use crate::{CliError, CliResult};

pub fn init_logger() -> CliResult<(String, flexi_logger::LoggerHandle)> {
    use flexi_logger::{Cleanup, Criterion, FileSpec, Logger, Naming};

    let qsv_log_env = env::var("QSV_LOG_LEVEL").unwrap_or_else(|_| {
        #[cfg(feature = "qsvmcp")]
        {
            "info".to_string()
        }
        #[cfg(not(feature = "qsvmcp"))]
        {
            "off".to_string()
        }
    });
    let qsv_log_dir = env::var("QSV_LOG_DIR").unwrap_or_else(|_| ".".to_string());
    let write_mode = if get_envvar_flag("QSV_LOG_UNBUFFERED") {
        flexi_logger::WriteMode::Direct
    } else {
        flexi_logger::WriteMode::BufferAndFlush
    };

    let logger_handle = Logger::try_with_env_or_str(qsv_log_env);

    match logger_handle {
        Ok(logger) => {
            let logger = logger
                .use_utc()
                .log_to_file(
                    FileSpec::default()
                        .directory(qsv_log_dir)
                        .suppress_timestamp(),
                )
                .write_mode(write_mode)
                .format_for_files(flexi_logger::detailed_format)
                .o_append(true)
                .rotate(
                    Criterion::Size(20_000_000), // 20 mb
                    Naming::Numbers,
                    Cleanup::KeepLogAndCompressedFiles(10, 100),
                )
                .start()?;
            let qsv_args: String = if log_enabled!(log::Level::Info) {
                env::args().skip(1).collect::<Vec<_>>().join(" ")
            } else {
                String::new()
            };
            log::info!("START: {qsv_args}");
            Ok((qsv_args, logger))
        },
        Err(e) => Err(CliError::Other(format!("Failed to initialize logger: {e}"))),
    }
}

/// Turn a `self_update` error into an accurate, actionable message.
///
/// rc.4's `Error` is `#[non_exhaustive]` with structured HTTP variants, so we can
/// distinguish a genuine GitHub rate-limit / auth failure (403/401/429) from a network
/// timeout or connection failure - instead of the old blanket "Github is rate-limiting"
/// message that masked every failure mode (including the fixed pagination-URL 403 bug).
#[cfg(feature = "self_update")]
fn describe_self_update_error(e: &self_update::errors::Error) -> String {
    use self_update::errors::Error;

    match e {
        // 429 (and 403 below) are how GitHub signals secondary/primary rate limits
        Error::HttpStatus { status: 429, .. } => {
            format!(
                "GitHub is rate-limiting self-update checks (HTTP 429). Try again in an hour: {e}"
            )
        },
        Error::Unauthorized { status: 403, .. } => {
            format!("GitHub returned HTTP 403 - likely rate-limiting. Try again in an hour: {e}")
        },
        Error::Unauthorized { status, .. } => {
            format!("GitHub authorization failed (HTTP {status}): {e}")
        },
        Error::HttpStatus { status, .. } => format!("GitHub returned HTTP {status}: {e}"),
        Error::NotFound { .. } => format!("Release not found on GitHub: {e}"),
        Error::Transport(_) => {
            format!("Network error reaching GitHub (timeout or connection failure): {e}")
        },
        _ => format!("Self-update check failed: {e}"),
    }
}

#[cfg(feature = "self_update")]
pub fn qsv_check_for_update(check_only: bool, no_confirm: bool) -> Result<bool, String> {
    use self_update::cargo_crate_version;

    if get_envvar_flag("QSV_NO_UPDATE") {
        return Ok(false);
    }

    let bin_name = match current_exe() {
        Ok(pb) => {
            if let Some(fs) = pb.file_stem() {
                fs.to_string_lossy().into_owned()
            } else {
                return fail!("Can't get the exec stem name");
            }
        },
        Err(e) => return fail_format!("Can't get the exec path: {e}"),
    };

    winfo!("Checking GitHub for updates...");

    let curr_version = cargo_crate_version!();
    let releases = match self_update::backends::github::ReleaseList::configure()
        .repo_owner("dathere")
        .repo_name("qsv")
        // the release listing is a small JSON response, so bound it with a short timeout
        // to avoid hanging indefinitely on a stalled connection. Pair with a couple of
        // retries (exponential backoff, per self_update defaults) for transient failures.
        .timeout(Duration::from_secs(20))
        .retries(2)
        .build()
    {
        Ok(releases_list) => match releases_list.fetch() {
            Ok(releases) => releases,
            Err(e) => {
                return fail!(describe_self_update_error(&e));
            },
        },
        Err(e) => {
            return fail!(describe_self_update_error(&e));
        },
    };
    let Some(first_release) = releases.latest() else {
        return fail!("No releases found on GitHub.");
    };
    let latest_release = first_release.version();

    log::info!("Current version: {curr_version} Latest Release: {latest_release}");

    let mut updated = false;
    let Ok(latest_release_sv) = semver::Version::parse(latest_release) else {
        return fail_format!("Can't parse latest release version: {latest_release}");
    };
    let Ok(curr_version_sv) = semver::Version::parse(curr_version) else {
        return fail_format!("Can't parse current version: {curr_version}");
    };

    if latest_release_sv > curr_version_sv {
        eprintln!("Update {latest_release} available. Current version is {curr_version}.");
        eprintln!("Release notes: https://github.com/dathere/qsv/releases/tag/{latest_release}\n");
        if QSV_KIND.starts_with("prebuilt") && !check_only {
            match self_update::backends::github::Update::configure()
                .repo_owner("dathere")
                .repo_name("qsv")
                .bin_name(&bin_name)
                .show_download_progress(true)
                .show_output(false)
                .no_confirm(no_confirm)
                .current_version(curr_version)
                .verifying_keys([*include_bytes!("../qsv-zipsign-public.key")])
                // NOTE: on the Update builder, timeout() also bounds the multi-MB binary
                // download (a total-request timeout), so use a generous cap here - NOT the
                // short listing timeout - to avoid aborting legitimate slow downloads.
                // retries() only re-attempts before any bytes are streamed to disk.
                .timeout(Duration::from_secs(600))
                .retries(2)
                .build()
            {
                Ok(update_job) => match update_job.update() {
                    Ok(status) => {
                        updated = true;
                        let update_status = format!(
                            "Update successful for {}: `{}`!",
                            bin_name,
                            status.version()
                        );
                        winfo!("{update_status}");
                    },
                    Err(e) => werr!("Update job error: {}", describe_self_update_error(&e)),
                },
                Err(e) => werr!("Update builder error: {}", describe_self_update_error(&e)),
            }
        } else if check_only {
            winfo!("Use the --update option to upgrade {bin_name} to the latest release.");
        } else {
            // we don't want to overwrite manually curated/configured qsv installations.
            // If QSV_KIND is not "prebuilt", just inform the user of the new release, and let them
            // rebuild their qsvs the way they like it, instead of overwriting it with
            // our prebuilt binaries.
            winfo!(
                r"This qsv was {QSV_KIND}. self-update does not work for manually {QSV_KIND} binaries.
If you wish to update to the latest version of qsv, manually install/compile from source.
Self-update only works with prebuilt binaries released on GitHub https://github.com/dathere/qsv/releases/latest"
            );
        }
    } else {
        winfo!("Up to date ({curr_version})... no update required.");
    }

    if !check_only
        && let Ok(status_code) =
            send_hwsurvey(&bin_name, updated, latest_release, curr_version, false)
    {
        log::info!("HW survey sent. Status code: {status_code}");
    }

    Ok(updated)
}

#[cfg(not(feature = "self_update"))]
pub fn qsv_check_for_update(_check_only: bool, _no_confirm: bool) -> Result<bool, String> {
    Err("Self-update is disabled in this build.".to_string())
}

/// Background update check, throttled to at most twice a month via a persisted stamp file.
///
/// Used by the no-command screen and the `help` command so they don't hit GitHub on every
/// invocation. The explicit `--update`/`--updatenow` flags bypass this and call
/// `qsv_check_for_update` directly. Best-effort: all errors are swallowed.
#[cfg(feature = "self_update")]
pub fn qsv_check_for_update_throttled() {
    // honor the opt-out BEFORE touching the filesystem
    if get_envvar_flag("QSV_NO_UPDATE") {
        return;
    }

    // stamp lives in the qsv cache dir: QSV_CACHE_DIR override, else ~/.qsv-cache
    let raw = std::env::var("QSV_CACHE_DIR")
        .ok()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "~/.qsv-cache".to_string());
    let Some(cache_dir) = expand_tilde(&raw) else {
        return; // can't resolve home; skip silently
    };

    // create the parent dir so record_check() can persist. If it can't, should_check()
    // would return Ok(true) forever => a check on EVERY startup (worse than no throttle).
    if fs::create_dir_all(&cache_dir).is_err() {
        return;
    }

    let stamp_path = cache_dir.join(".last_update_check");
    let guard =
        self_update::check_interval::UpdateCheckGuard::new(stamp_path, UPDATE_CHECK_INTERVAL);

    // on a genuine IO error reading the stamp, DON'T check (avoid hammering GitHub);
    // a missing/corrupt stamp already yields Ok(true), so first run still checks.
    if guard.should_check().unwrap_or(false) {
        _ = qsv_check_for_update(true, false); // best-effort; ignore network errors
        _ = guard.record_check(); // record regardless of outcome => real throttle
    }
}

#[cfg(not(feature = "self_update"))]
pub fn qsv_check_for_update_throttled() {}

// the qsv hwsurvey allows us to keep a better
// track of qsv's usage in the wild, so we can do a
// better job of prioritizing platforms/features we support
// no personally identifiable information is collected
#[cfg(feature = "self_update")]
fn send_hwsurvey(
    bin_name: &str,
    updated: bool,
    latest_release: &str,
    curr_version: &str,
    dry_run: bool,
) -> Result<reqwest::StatusCode, String> {
    use serde_json::json;

    static HW_SURVEY_URL: &str =
        "https://4dhmneehnl.execute-api.us-east-1.amazonaws.com/dev/qsv-hwsurvey";

    let mut sys = System::new();
    sys.refresh_all();
    let total_mem = sys.total_memory();
    let kernel_version =
        sysinfo::System::kernel_version().unwrap_or_else(|| "Unknown kernel".to_string());
    let long_os_version =
        sysinfo::System::long_os_version().unwrap_or_else(|| "Unknown OS version".to_string());
    let cpu_count = sys.cpus().len();
    let physical_cpu_count = sysinfo::System::physical_core_count().unwrap_or_default();
    let cpu_vendor_id = sys.cpus()[0].vendor_id();
    let cpu_brand = sys.cpus()[0].brand().trim();
    let cpu_freq = sys.cpus()[0].frequency();
    let long_id: u128 = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    // the id doubles as a timestamp
    // we first get number of milliseconds since UNIX EPOCH
    // and then cast to u64 as serde_json cannot serialize u128
    let id: u64 = long_id.try_into().unwrap_or_default();
    let hwsurvey_json = json!(
        {
            "id": id,
            "variant": bin_name,
            "kind": QSV_KIND,
            "ver": if updated { latest_release } else { curr_version },
            "updated": updated,
            "prev_ver": curr_version,
            "cpu_phy_cores": physical_cpu_count,
            "cpu_log_cores": cpu_count,
            "cpu_vendor": cpu_vendor_id,
            "cpu_brand": cpu_brand,
            "cpu_freq": cpu_freq,
            "mem": total_mem,
            "kernel": kernel_version,
            "os": long_os_version,
            "target": TARGET,
        }
    );
    log::debug!("hwsurvey: {hwsurvey_json}");

    let mut survey_done = false;
    let mut status = reqwest::StatusCode::OK;
    if dry_run {
        log::info!("Survey dry run. hw survey compiled successfully, but not sent.");
    } else {
        let client = match create_reqwest_blocking_client(
            Some(default_user_agent()),
            30, // default timeout for hw survey
            None,
        ) {
            Ok(c) => c,
            Err(e) => return fail_format!("Cannot build hw_survey reqwest client: {e}"),
        };

        match client
            .post(HW_SURVEY_URL)
            .body(hwsurvey_json.to_string())
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .header(reqwest::header::HOST, "qsv.rs")
            .send()
        {
            Ok(resp) => {
                log::debug!("hw_survey response sent: {resp:?}");
                status = resp.status();
                survey_done = status.is_success();
            },
            Err(e) => {
                log::warn!("Cannot send hw survey: {e}");
                status = reqwest::StatusCode::BAD_REQUEST;
            },
        }
    }
    if survey_done || dry_run {
        Ok(status)
    } else {
        fail!("hw survey failed.")
    }
}

#[test]
#[cfg(feature = "self_update")]
fn test_hw_survey() {
    // we have this test primarily to exercise the sysinfo module
    assert!(send_hwsurvey("qsv", false, "0.0.2", "0.0.1", true).is_ok());
}
