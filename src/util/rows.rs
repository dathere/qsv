#[cfg(any(feature = "feature_capable", feature = "lite"))]
use std::{fs::File, io::BufRead, io::BufReader};

use indicatif::{HumanCount, ProgressBar, ProgressStyle};

use super::ROW_COUNT;
#[cfg(feature = "polars")]
use crate::cmd::count::polars_count_input;
use crate::{CliError, config::Config};

/// Convert a CSV read error into a `CliError` with an actionable hint.
///
/// `stats` and `frequency` historically assumed valid CSV and read the record iterator with
/// `unwrap_unchecked()`, which is undefined behavior on the `Err` the csv crate returns for a
/// ragged record (issue #4611). They now surface the error through this helper so a malformed
/// file gets the same guidance `validate` gives instead of a silent non-unwinding abort.
pub fn csv_read_error(e: &csv::Error) -> CliError {
    // csv::Error's Display already starts with "CSV error:", so don't prefix it again
    if matches!(e.kind(), csv::ErrorKind::UnequalLengths { .. }) {
        return CliError::Other(format!(
            "{e}.\nUse `qsv fixlengths` to fix record length issues, or `--flexible` to process \
             the file anyway."
        ));
    }
    CliError::Other(format!("{e}"))
}

#[inline]
pub fn count_rows(conf: &Config) -> Result<u64, CliError> {
    // Check if ROW_COUNT is already initialized to avoid redundant counting
    if let Some(count) = ROW_COUNT.get() {
        return Ok(count.unwrap_or(0));
    }

    // If not, try using index if available
    if let Some(idx) = conf.indexed().unwrap_or(None) {
        return Ok(idx.count());
    }
    // index does not exist or is stale

    // Otherwise, count records by using polars mem-mapped reader if available
    // If polars is not enabled, count records by iterating through records
    // Do this only once per invocation and cache the result in ROW_COUNT,
    // so we don't have to re-count rows every time we need to know the
    // rowcount for CSVs that don't have an index.
    ROW_COUNT
        .get_or_init(|| {
            // Try different counting methods in order of preference
            count_rows_with_best_method(conf)
        })
        .ok_or_else(|| CliError::Other("Unable to get row count".to_string()))
}

#[cfg(feature = "polars")]
fn count_rows_with_best_method(conf: &Config) -> Option<u64> {
    if !conf.no_headers {
        // Try polars first for files with headers
        if let Ok(polars_count) = polars_count_input(conf) {
            // If count is greater than 0, return the polars accelerated count
            // as sometimes, polars returns a zero count even if the file is not empty
            // and the file is a proper CSV file.
            // Otherwise, double-check with the "regular" CSV reader
            if polars_count > 0 {
                return Some(polars_count);
            }
        }
    }

    // Fall back to CSV reader
    count_with_csv_reader(conf)
}

#[cfg(not(feature = "polars"))]
fn count_rows_with_best_method(conf: &Config) -> Option<u64> {
    count_with_csv_reader(conf)
}

fn count_with_csv_reader(conf: &Config) -> Option<u64> {
    let path_display = conf
        .path
        .as_deref()
        .map_or_else(|| "<stdin>".to_string(), |p| p.display().to_string());
    conf.clone()
        .skip_format_check(true)
        .reader()
        .ok()
        .map(|mut rdr| {
            let mut count = 0_u64;
            let mut record = csv::ByteRecord::new();
            loop {
                match rdr.read_byte_record(&mut record) {
                    Ok(true) => count += 1,
                    Ok(false) => break,
                    Err(e) => {
                        log::warn!(
                            "CSV read error during row count of {path_display} after {count} \
                             rows; returning partial count: {e}"
                        );
                        break;
                    },
                }
            }
            count
        })
}

/// Count rows using the "regular" CSV reader, never the polars mem-mapped
/// reader. The result is cached in `ROW_COUNT` so subsequent calls are free.
#[inline]
pub fn count_rows_regular(conf: &Config) -> Result<u64, CliError> {
    if let Some(idx) = conf.indexed().unwrap_or(None) {
        return Ok(idx.count());
    }
    // No index (or stale): defer to the shared CSV-reader helper so the
    // counting loop and warn-with-path logic live in one place. Cached in
    // ROW_COUNT so a later count_rows call hits the same value.
    ROW_COUNT
        .get_or_init(|| count_with_csv_reader(conf))
        .ok_or_else(|| CliError::Other("Unable to get row count".to_string()))
}

#[cfg(any(feature = "feature_capable", feature = "lite"))]
pub fn count_lines_in_file(file: &str) -> Result<u64, CliError> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);

    let line_count = reader.lines().count() as u64;
    Ok(line_count)
}

pub fn prep_progress(progress: &ProgressBar, record_count: u64) {
    progress.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{wide_bar} {percent}%{msg}] ({per_sec} - {eta})")
            .unwrap(),
    );
    progress.set_message(format!(" of {} records", HumanCount(record_count)));

    // draw progress bar for the first time using specified style
    progress.set_length(record_count);

    log::info!("Progress started... {record_count} records");
}

pub fn finish_progress(progress: &ProgressBar) {
    progress.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{wide_bar} {percent}%{msg}] ({per_sec})")
            .unwrap(),
    );

    if progress.length().unwrap_or_default() == progress.position() {
        progress.finish();
        log::info!("Progress done... {}", progress.message());
    } else {
        progress.abandon();
        log::info!("Progress abandoned... {}", progress.message());
    }
}
