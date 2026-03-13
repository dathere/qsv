static USAGE: &str = r#"
Pragmatic statistical toolkit.

Compute robust, median-of-pairwise statistics from the Pragmastat library.
Designed for messy, heavy-tailed, or outlier-prone data where mean/stddev can mislead.

This is a "smart" command that uses the stats cache to work smarter & faster.
When a stats cache is available, non-numeric columns are automatically filtered out
(unless --select is explicitly provided) and Date/DateTime columns are supported.

By default, one-sample mode appends 7 ps_* columns to the .stats.csv cache file
(like moarstats). Use --standalone for the old standalone CSV output. Two-sample,
compare1, and compare2 modes always produce standalone output.

Input handling
  * Only finite numeric values are used; non-numeric/NaN/Inf are ignored.
  * Date/DateTime columns are supported when a stats cache is available
    (run "qsv stats -E --infer-dates --stats-jsonl" first). Dates are converted to epoch
    milliseconds for analysis, then center/bounds are formatted as dates and spread/shift
    as days.
  * Each column is treated as its own sample (two-sample compares columns, not rows).
  * Non-numeric columns appear with n=0 and empty estimator cells.
  * NOTE: This command loads all numeric values into memory.

ONE-SAMPLE OUTPUT (default, per selected column)
  field, n, center, spread, center_lower, center_upper, spread_lower, spread_upper

  center             Robust location; median of pairwise averages (Hodges-Lehmann).
                     Like the mean but stable with outliers; tolerates up to 29% corrupted data.
  spread             Robust dispersion; median of pairwise absolute differences (Shamos).
                     Same units as data; also tolerates up to 29% corrupted data.
  center_lower/upper Bounds for center with error rate = misrate (exact under weak symmetry).
                     Use 1e-3 for everyday analysis or 1e-6 for critical decisions.
  spread_lower/upper Bounds for spread with error rate = misrate (randomized).

TWO-SAMPLE OUTPUT (--twosample, per unordered column pair)
  field_x, field_y, n_x, n_y, shift, ratio, disparity,
  shift_lower, shift_upper, ratio_lower, ratio_upper, disparity_lower, disparity_upper

  shift                 Robust difference in location; median of pairwise differences.
                        Negative => first column tends to be lower.
  ratio                 Robust multiplicative ratio; exp(shift(log x, log y)).
                        Use for positive-valued quantities (latency, price, concentration).
  disparity             Robust effect size = shift / (average spread of x and y).
  shift_lower/upper     Bounds for shift (exact; ties may be conservative).
                        If bounds exclude 0, the shift is reliable.
  ratio_lower/upper     Bounds for ratio (exact; requires all values > 0).
                        If bounds exclude 1, the ratio is reliable.
  disparity_lower/upper Bounds for disparity (randomized, Bonferroni combination).
                        If bounds exclude 0, the disparity is reliable.

When values are blank
  * Column has no numeric data (n=0).
  * Positivity required: ratio, ratio_* need all values > 0.
  * Sparity required: spread/spread_*/disparity/disparity_* need real variability (not tie-dominant).
  * Bounds require enough data for requested misrate; try higher misrate or more data.

MISRATE PARAMETER
  misrate is the probability that bounds miss the true value (lower => wider bounds).
    1e-3    Everyday analysis [default]
    1e-6    Critical decisions

COMPARE1 OUTPUT (--compare1, one-sample confirmatory analysis)
  field, n, metric, threshold, estimate, lower, upper, verdict

  Tests one-sample estimates (center/spread) against user-defined thresholds.
  Each threshold produces one row per column with a verdict:
    less          Estimate is statistically less than the threshold.
    greater       Estimate is statistically greater than the threshold.
    inconclusive  Not enough evidence to decide (interval contains threshold).

COMPARE2 OUTPUT (--compare2, two-sample confirmatory analysis)
  field_x, field_y, n_x, n_y, metric, threshold, estimate, lower, upper, verdict

  Tests two-sample estimates (shift/ratio/disparity) against user-defined thresholds.
  Each threshold produces one row per column pair with the same verdict semantics.

THRESHOLD FORMAT
  Both compare flags accept a comma-separated list of metric:value pairs.
    compare1 center:42.0             Single threshold
    compare1 center:42.0,spread:0.5  Multiple thresholds
    compare2 shift:0,disparity:0.8   Two-sample thresholds

  Valid metrics for compare1: center, spread
  Valid metrics for compare2: shift, ratio, disparity

Examples:
  # Append pragmastat columns to stats cache (default one-sample behavior)
  qsv pragmastat data.csv

  # Standalone one-sample output (old behavior)
  qsv pragmastat --standalone data.csv

  # One-sample statistics with selected columns
  qsv pragmastat --select latency_ms,price data.csv

  # Two-sample statistics with selected columns
  qsv pragmastat --twosample --select latency_ms,price data.csv

  # One-sample statistics with very tight bounds (lower misrate)
  qsv pragmastat --misrate 1e-6 data.csv

  # Compare one-sample center against a threshold
  qsv pragmastat --compare1 center:42.0 --select latitude data.csv

  # Compare one-sample center and spread against thresholds
  qsv pragmastat --compare1 center:42.0,spread:0.5 --select latitude data.csv

  # Compare two-sample shift and disparity against thresholds
  qsv pragmastat --compare2 shift:0,disparity:0.8 --select latency_ms,price data.csv

Full Pragmastat manual:
  https://github.com/AndreyAkinshin/pragmastat/releases/download/v12.0.0/pragmastat-v12.0.0.pdf
  https://pragmastat.dev/ (latest version)

Usage:
    qsv pragmastat [options] [<input>]
    qsv pragmastat --help

pragmastat options:
    -t, --twosample        Compute two-sample estimators for all column pairs.
        --compare1 <spec>  One-sample confirmatory analysis. Test center/spread against
                           thresholds. Format: metric:value[,metric:value,...].
                           Mutually exclusive with --twosample and --compare2.
        --compare2 <spec>  Two-sample confirmatory analysis. Test shift/ratio/disparity
                           against thresholds. Format: metric:value[,metric:value,...].
                           Mutually exclusive with --twosample and --compare1.
    -s, --select <cols>    Select columns for analysis. Uses qsv's column selection
                           syntax. Non-numeric columns appear with n=0.
                           In two-sample mode, all pairs of selected columns are computed.
    -m, --misrate <n>      Probability that bounds fail to contain the true parameter.
                           Lower values produce wider bounds.
                           Must be achievable for the given sample size.
                           [default: 0.001]
        --standalone       Output one-sample results as standalone CSV instead of
                           appending to the stats cache.
        --stats-options <arg>  Options to pass to the stats command if baseline stats need
                           to be generated. The options are passed as a single string
                           that will be split by whitespace.
                           [default: --infer-dates --infer-boolean --mad --quartiles --force --stats-jsonl]
        --round <n>        Round statistics to <n> decimal places. Rounding follows
                           Midpoint Nearest Even (Bankers Rounding) rule.
                           [default: 4]
        --force            Force recomputing ps_* columns even if they already exist
                           in the stats cache.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -d, --delimiter <c>    The field delimiter for reading/writing CSV data.
                           Must be a single character. (default: ,)
    -n, --no-headers       When set, the first row will not be treated as headers.
    -j, --jobs <arg>       The number of jobs to run in parallel.
                           When not set, the number of jobs is set to the
                           number of CPUs detected.
    --memcheck             Check if there is enough memory to load the entire
                           CSV into memory using CONSERVATIVE heuristics. Not valid for stdin.
"#;

use rayon::prelude::*;
use serde::Deserialize;
use threadpool::ThreadPool;

use crate::{
    CliResult,
    clitypes::CliError,
    config::{Config, Delimiter},
    select::SelectColumns,
    util,
};

/// Milliseconds per day — used to convert epoch-ms spreads/shifts to days.
const MS_IN_DAY: f64 = 86_400_000.0;
/// Decimal places for day-valued outputs (5 ≈ sub-second precision;
/// 1e-5 days ≈ 0.864 seconds).
const DAY_DECIMAL_PLACES: u32 = 5;

/// Tracks whether a column holds plain numbers or parsed dates so we can
/// format output appropriately (dates as RFC3339, spreads/shifts as days).
#[derive(Clone, Copy, PartialEq, Eq)]
enum ColType {
    Numeric,
    Date,
    DateTime,
}

#[derive(Deserialize)]
struct Args {
    arg_input:          Option<String>,
    flag_twosample:     bool,
    flag_compare1:      Option<String>,
    flag_compare2:      Option<String>,
    flag_select:        Option<SelectColumns>,
    flag_misrate:       f64,
    flag_standalone:    bool,
    flag_stats_options: String,
    flag_round:         u32,
    flag_force:         bool,
    flag_output:        Option<String>,
    flag_delimiter:     Option<Delimiter>,
    flag_no_headers:    bool,
    flag_jobs:          Option<usize>,
    flag_memcheck:      bool,
}

struct OneSampleResult<'a> {
    field:        &'a str,
    n:            usize,
    center:       Option<f64>,
    spread:       Option<f64>,
    center_lower: Option<f64>,
    center_upper: Option<f64>,
    spread_lower: Option<f64>,
    spread_upper: Option<f64>,
}

struct TwoSampleResult<'a> {
    field_x:         &'a str,
    field_y:         &'a str,
    n_x:             usize,
    n_y:             usize,
    shift:           Option<f64>,
    ratio:           Option<f64>,
    disparity:       Option<f64>,
    shift_lower:     Option<f64>,
    shift_upper:     Option<f64>,
    ratio_lower:     Option<f64>,
    ratio_upper:     Option<f64>,
    disparity_lower: Option<f64>,
    disparity_upper: Option<f64>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    validate_misrate(args.flag_misrate)?;

    // Mutual exclusivity: at most one mode flag
    let mode_count = usize::from(args.flag_twosample)
        + usize::from(args.flag_compare1.is_some())
        + usize::from(args.flag_compare2.is_some());
    if mode_count > 1 {
        return Err(CliError::IncorrectUsage(
            "--twosample, --compare1, and --compare2 are mutually exclusive.".to_string(),
        ));
    }

    let is_onesample =
        !args.flag_twosample && args.flag_compare1.is_none() && args.flag_compare2.is_none();

    if is_onesample && !args.flag_standalone && args.arg_input.is_some() {
        run_cache_append(&args)
    } else {
        util::njobs(args.flag_jobs);
        let (col_names, col_values, col_types) = read_columns(&args)?;
        write_results(&args, &col_names, &col_values, &col_types)?;
        Ok(())
    }
}

/// The 7 column names appended to the stats cache.
const PS_COLUMNS: [&str; 7] = [
    "ps_n",
    "ps_center",
    "ps_spread",
    "ps_center_lower",
    "ps_center_upper",
    "ps_spread_lower",
    "ps_spread_upper",
];

/// Append pragmastat one-sample columns to the .stats.csv cache file.
fn run_cache_append(args: &Args) -> CliResult<()> {
    use csv::{ReaderBuilder, WriterBuilder};

    let input_path_str = args.arg_input.as_ref().unwrap();
    let input_path = std::path::Path::new(input_path_str);
    let stats_csv_path = util::get_stats_csv_path(input_path)?;

    // Auto-generate stats if missing (--force only recomputes ps_* columns,
    // it does NOT regenerate the baseline stats to avoid clobbering moarstats columns).
    if !stats_csv_path.exists() {
        wwarn!(
            "Stats CSV file not found: {}\nComputing baseline stats...",
            stats_csv_path.display()
        );
        let mut stats_args_vec: Vec<&str> = args.flag_stats_options.split_whitespace().collect();
        // Pass through input-shaping flags so the stats command matches the input format
        let delim_str;
        if let Some(ref delim) = args.flag_delimiter {
            delim_str = String::from(delim.as_byte() as char);
            stats_args_vec.push("--delimiter");
            stats_args_vec.push(&delim_str);
        }
        if args.flag_no_headers {
            stats_args_vec.push("--no-headers");
        }
        let _ = util::run_qsv_cmd(
            "stats",
            &stats_args_vec,
            input_path_str,
            "Ran stats command to generate baseline stats...",
        )?;
        if !stats_csv_path.exists() {
            return fail_clierror!(
                "Stats CSV file was not created: {}",
                stats_csv_path.display()
            );
        }
    }

    // Read the stats CSV
    let stats_csv_content = std::fs::read_to_string(&stats_csv_path)?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_csv_content.as_bytes());

    let headers = rdr.headers()?.clone();
    let records: Vec<csv::StringRecord> = rdr.records().collect::<Result<_, _>>()?;

    // Find required column indices — these must exist in any valid stats CSV
    let field_idx = headers
        .iter()
        .position(|h| h == "field")
        .ok_or("Stats CSV is missing required 'field' column")?;
    let type_idx = headers
        .iter()
        .position(|h| h == "type")
        .ok_or("Stats CSV is missing required 'type' column")?;

    // Check if ps_* columns already exist
    let already_exists = headers.iter().any(|h| h == "ps_center");
    if already_exists && !args.flag_force {
        winfo!(
            "ps_* columns already exist in {}. Use --force to recompute.",
            stats_csv_path.display()
        );
        return Ok(());
    }

    // Read original CSV data and compute one-sample results
    util::njobs(args.flag_jobs);
    let (col_names, col_values, col_types) = read_columns(args)?;

    // Compute one-sample results in parallel
    let results: Vec<OneSampleResult> = col_names
        .par_iter()
        .enumerate()
        .map(|(i, name)| compute_one_sample(name, &col_values[i], args.flag_misrate))
        .collect();

    // Build a lookup map: field_name -> (result_index, col_type)
    let result_map: std::collections::HashMap<&str, (usize, ColType)> = col_names
        .iter()
        .enumerate()
        .map(|(i, name)| (name.as_str(), (i, col_types[i])))
        .collect();

    // Prepare output — when writing back to the stats cache (no --output),
    // write to a temp file first then rename for atomicity, so a partial
    // failure (e.g. disk full) doesn't corrupt the cache.
    let output_path = args
        .flag_output
        .as_ref()
        .map_or_else(|| stats_csv_path.clone(), std::path::PathBuf::from);
    let writing_in_place = output_path == stats_csv_path;
    let write_target = if writing_in_place {
        // Append ".tmp" to the full filename (e.g. "data.stats.csv" -> "data.stats.csv.tmp")
        let mut tmp_name = output_path.file_name().map_or_else(
            || std::ffi::OsString::from("stats.csv"),
            std::ffi::OsString::from,
        );
        tmp_name.push(".tmp");
        let mut tmp_path = output_path.clone();
        tmp_path.set_file_name(tmp_name);
        tmp_path
    } else {
        output_path.clone()
    };
    let mut wtr = WriterBuilder::new()
        .has_headers(true)
        .from_path(&write_target)?;

    // Write headers: strip existing ps_* columns if --force, then append new ones
    let mut header_record = csv::StringRecord::new();
    let mut skip_indices: Vec<usize> = Vec::new();
    for (i, h) in headers.iter().enumerate() {
        if already_exists && h.starts_with("ps_") {
            skip_indices.push(i);
        } else {
            header_record.push_field(h);
        }
    }
    for col in &PS_COLUMNS {
        header_record.push_field(col);
    }
    wtr.write_record(&header_record)?;

    // Process each record
    let round = args.flag_round;
    for record in &records {
        let field_name = record.get(field_idx).unwrap_or("");
        let field_type_str = record.get(type_idx).unwrap_or("");

        // Build output record: existing fields (skipping old ps_* columns) + new ps_* fields
        let mut out_record = csv::StringRecord::new();
        for (i, field) in record.iter().enumerate() {
            if !skip_indices.contains(&i) {
                out_record.push_field(field);
            }
        }

        let is_numeric_type = matches!(field_type_str, "Integer" | "Float" | "Date" | "DateTime");

        if is_numeric_type && let Some(&(result_idx, _)) = result_map.get(field_name) {
            // Derive ColType only when needed for numeric formatting
            let ct = match field_type_str {
                "Date" => ColType::Date,
                "DateTime" => ColType::DateTime,
                _ => ColType::Numeric,
            };
            let r = &results[result_idx];
            let mut itoa_buf = itoa::Buffer::new();
            out_record.push_field(itoa_buf.format(r.n));
            out_record.push_field(&fmt_point(r.center, ct, round));
            out_record.push_field(&fmt_spread(r.spread, ct, round));
            out_record.push_field(&fmt_point(r.center_lower, ct, round));
            out_record.push_field(&fmt_point(r.center_upper, ct, round));
            out_record.push_field(&fmt_spread(r.spread_lower, ct, round));
            out_record.push_field(&fmt_spread(r.spread_upper, ct, round));
        } else {
            // Non-numeric or not in result map: empty ps_* fields
            for _ in 0..7 {
                out_record.push_field("");
            }
        }
        wtr.write_record(&out_record)?;
    }

    wtr.flush()?;
    drop(wtr);

    // Atomically replace the original file if writing in place
    if writing_in_place {
        #[cfg(windows)]
        {
            // On Windows, std::fs::rename will not overwrite an existing file.
            // Use a backup strategy: rename original to .bak, rename .tmp to target,
            // then delete .bak. If we crash after removing .bak but before renaming
            // .tmp, the .bak file still exists as a recovery point.
            let bak_path = output_path.with_extension("stats.csv.bak");
            if output_path.exists() {
                std::fs::rename(&output_path, &bak_path)?;
            }
            std::fs::rename(&write_target, &output_path)?;
            // Clean up backup; non-fatal if this fails
            let _ = std::fs::remove_file(&bak_path);
        }
        #[cfg(not(windows))]
        {
            std::fs::rename(&write_target, &output_path)?;
        }
    }

    winfo!(
        "Added {} pragmastat columns to {}",
        PS_COLUMNS.len(),
        output_path.display()
    );

    Ok(())
}

fn validate_misrate(misrate: f64) -> CliResult<()> {
    if misrate.is_nan() || misrate <= 0.0 || misrate >= 1.0 {
        return Err(CliError::IncorrectUsage(
            "--misrate must be between 0 and 1 (exclusive).".to_string(),
        ));
    }
    Ok(())
}

fn read_columns(args: &Args) -> CliResult<(Vec<String>, Vec<Vec<f64>>, Vec<ColType>)> {
    let rconfig = Config::new(args.arg_input.as_ref())
        .delimiter(args.flag_delimiter)
        .no_headers_flag(args.flag_no_headers);

    if let Some(ref path) = rconfig.path {
        util::mem_file_check(path, false, args.flag_memcheck)?;
    }

    let idx_count = rconfig.indexed()?.map(|idx| idx.count() as usize);
    let row_count = idx_count.unwrap_or(1024);

    let mut rdr = rconfig.reader()?;
    let headers = rdr.byte_headers()?.clone();
    let mut selected = resolve_columns(&rconfig, &headers, args.flag_select.as_ref())?;

    // Try loading the stats cache to (a) filter non-numeric columns when --select
    // is not given, and (b) detect Date/DateTime columns for date-aware formatting.
    let cache_info = columns_from_cache(args, &headers);

    if args.flag_select.is_none()
        && let Some(ref info) = cache_info
    {
        let before = selected.len();
        selected.retain(|idx| info.iter().any(|(ci, _)| ci == idx));
        let skipped = before - selected.len();
        if skipped > 0 {
            winfo!("skipped {skipped} non-numeric column(s) via stats cache.");
        }
    }

    // Build per-column type map from cache info
    let col_type_map: Vec<(usize, ColType)> = cache_info.unwrap_or_default();

    // Use indexed parallel reading for large files (>= 10k rows)
    if let Some(count) = idx_count
        && count >= 10_000
        && rconfig.path.is_some()
    {
        collect_numeric_values_parallel(
            &rconfig,
            &headers,
            &selected,
            &col_type_map,
            count,
            args.flag_jobs,
        )
    } else {
        collect_numeric_values(
            &mut rdr,
            &headers,
            &selected,
            &col_type_map,
            rconfig.no_headers,
            row_count,
        )
    }
}

/// Try to read the stats cache and return `(column_index, ColType)` pairs for columns
/// that pragmastat can analyse (Integer, Float, Date, DateTime).
/// Returns `None` if the cache is unavailable or stale.
/// This is purely opportunistic — it never triggers a stats run.
fn columns_from_cache(args: &Args, headers: &csv::ByteRecord) -> Option<Vec<(usize, ColType)>> {
    use std::io::BufRead;

    use filetime::FileTime;

    // Only attempt if we have a file path (not stdin)
    let input_path = args.arg_input.as_ref()?;
    let canonical = std::path::Path::new(input_path).canonicalize().ok()?;
    let cache_path = canonical.with_extension("stats.csv.data.jsonl");

    // Only use if cache exists and is newer than input
    if !cache_path.exists() {
        return None;
    }
    let cache_mtime = FileTime::from_last_modification_time(&std::fs::metadata(&cache_path).ok()?);
    let input_mtime = FileTime::from_last_modification_time(&std::fs::metadata(input_path).ok()?);
    if cache_mtime <= input_mtime {
        return None;
    }

    // Read the JSONL cache directly — no routing through get_stats_records,
    // so we truly never trigger a stats run.
    let file = std::fs::File::open(&cache_path).ok()?;
    let reader = std::io::BufReader::new(file);

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().ok()?;

    // Validate that the cache has exactly as many records as the CSV has columns.
    // If it doesn't match (e.g. cache was generated with --select), ignore it.
    if lines.len() != headers.len() {
        return None;
    }

    let mut result = Vec::new();
    for (i, curr_line) in lines.iter().enumerate() {
        let mut s_slice = curr_line.as_bytes().to_vec();

        #[cfg(target_endian = "big")]
        let parse_result = serde_json::from_slice::<CacheRecord>(&s_slice);
        #[cfg(target_endian = "little")]
        let parse_result = simd_json::from_slice::<CacheRecord>(&mut s_slice);

        let Ok(record) = parse_result else {
            return None;
        };

        let col_type = match record.r#type.as_str() {
            "Integer" | "Float" => ColType::Numeric,
            "Date" => ColType::Date,
            "DateTime" => ColType::DateTime,
            _ => continue,
        };
        result.push((i, col_type));
    }

    if result.is_empty() {
        None
    } else {
        Some(result)
    }
}

/// Minimal struct to deserialize only the `type` field from a stats cache JSONL record.
#[derive(Deserialize)]
struct CacheRecord {
    r#type: String,
}

fn resolve_columns(
    rconfig: &Config,
    headers: &csv::ByteRecord,
    select: Option<&SelectColumns>,
) -> CliResult<Vec<usize>> {
    if let Some(sel) = select {
        let conf = rconfig.clone().select(sel.clone());
        Ok(conf
            .selection(headers)?
            .iter()
            .copied()
            .collect::<Vec<usize>>())
    } else {
        Ok((0..headers.len()).collect::<Vec<usize>>())
    }
}

fn write_results(
    args: &Args,
    col_names: &[String],
    col_values: &[Vec<f64>],
    col_types: &[ColType],
) -> CliResult<()> {
    let mut wtr = Config::new(args.flag_output.as_ref())
        .delimiter(args.flag_delimiter)
        .writer()?;
    let round = args.flag_round;

    if let Some(ref spec) = args.flag_compare1 {
        let thresholds = parse_thresholds(spec, args.flag_misrate, true)?;
        write_compare1_results(
            &mut wtr,
            col_names,
            col_values,
            col_types,
            &thresholds,
            round,
        )?;
    } else if let Some(ref spec) = args.flag_compare2 {
        let thresholds = parse_thresholds(spec, args.flag_misrate, false)?;
        write_compare2_results(
            &mut wtr,
            col_names,
            col_values,
            col_types,
            &thresholds,
            round,
        )?;
    } else if args.flag_twosample {
        write_twosample_results(
            &mut wtr,
            col_names,
            col_values,
            col_types,
            args.flag_misrate,
            round,
        )?;
    } else {
        write_onesample_results(
            &mut wtr,
            col_names,
            col_values,
            col_types,
            args.flag_misrate,
            round,
        )?;
    }

    wtr.flush()?;
    Ok(())
}

fn write_onesample_results(
    wtr: &mut csv::Writer<Box<dyn std::io::Write + 'static>>,
    col_names: &[String],
    col_values: &[Vec<f64>],
    col_types: &[ColType],
    misrate: f64,
    round: u32,
) -> CliResult<()> {
    write_onesample_header(wtr)?;

    let results: Vec<OneSampleResult> = col_names
        .par_iter()
        .enumerate()
        .map(|(i, name)| compute_one_sample(name, &col_values[i], misrate))
        .collect();

    for (i, result) in results.iter().enumerate() {
        write_onesample_row(wtr, result, col_types[i], round)?;
    }
    Ok(())
}

fn write_twosample_results(
    wtr: &mut csv::Writer<Box<dyn std::io::Write + 'static>>,
    col_names: &[String],
    col_values: &[Vec<f64>],
    col_types: &[ColType],
    misrate: f64,
    round: u32,
) -> CliResult<()> {
    write_twosample_header(wtr)?;

    let k = col_names.len();
    if k < 2 {
        return Ok(());
    }
    let num_pairs = k * (k - 1) / 2;
    if num_pairs > 100 {
        winfo!(
            "computing {num_pairs} column pairs from {k} columns. Use --select to limit columns \
             for faster results."
        );
    }

    // Pre-compute log-transformed arrays for ratio computation.
    // Each column participates in k-1 pairs, so this avoids redundant O(n) ln() passes.
    let log_values: Vec<Option<Vec<f64>>> = col_values
        .par_iter()
        .map(|vals| {
            if vals.iter().all(|&v| v > 0.0) {
                Some(vals.iter().map(|v| v.ln()).collect())
            } else {
                None
            }
        })
        .collect();

    let pairs: Vec<(usize, usize)> = (0..k)
        .flat_map(|i| ((i + 1)..k).map(move |j| (i, j)))
        .collect();

    let results: Vec<TwoSampleResult> = pairs
        .par_iter()
        .map(|&(i, j)| {
            compute_two_sample(
                &col_names[i],
                &col_names[j],
                &col_values[i],
                &col_values[j],
                log_values[i].as_deref(),
                log_values[j].as_deref(),
                misrate,
            )
        })
        .collect();

    for (pi, result) in results.iter().enumerate() {
        let (i, j) = pairs[pi];
        let pair_type = match (col_types[i], col_types[j]) {
            (ColType::Date, ColType::Date) => ColType::Date,
            (ColType::DateTime | ColType::Date, ColType::DateTime | ColType::Date) => {
                ColType::DateTime
            },
            (ColType::Date | ColType::DateTime, ColType::Numeric)
            | (ColType::Numeric, ColType::Date | ColType::DateTime) => {
                wwarn!(
                    "skipping mixed Date/Numeric pair ({}, {}) — comparison is not meaningful",
                    col_names[i],
                    col_names[j]
                );
                continue;
            },
            _ => ColType::Numeric,
        };
        write_twosample_row(wtr, result, pair_type, round)?;
    }
    Ok(())
}

fn collect_numeric_values(
    rdr: &mut csv::Reader<Box<dyn std::io::Read + Send + 'static>>,
    headers: &csv::ByteRecord,
    selected: &[usize],
    col_type_map: &[(usize, ColType)],
    no_headers: bool,
    row_count: usize,
) -> CliResult<(Vec<String>, Vec<Vec<f64>>, Vec<ColType>)> {
    let col_names: Vec<String> = selected
        .iter()
        .map(|&i| {
            if no_headers {
                (i + 1).to_string()
            } else {
                String::from_utf8_lossy(&headers[i]).into_owned()
            }
        })
        .collect();

    // Resolve per-selected-column type from the cache map
    let col_types: Vec<ColType> = selected
        .iter()
        .map(|idx| {
            col_type_map
                .iter()
                .find(|(ci, _)| ci == idx)
                .map_or(ColType::Numeric, |&(_, ct)| ct)
        })
        .collect();

    let mut col_values: Vec<Vec<f64>> = vec![Vec::with_capacity(row_count); selected.len()];

    for result in rdr.byte_records() {
        let record = result?;
        for (idx, &col_idx) in selected.iter().enumerate() {
            if let Some(field) = record.get(col_idx) {
                match col_types[idx] {
                    ColType::Date | ColType::DateTime => {
                        // Parse date string → epoch milliseconds
                        if let Ok(s) = simdutf8::basic::from_utf8(field)
                            && !s.is_empty()
                            && let Ok(dt) = qsv_dateparser::parse_with_preference(s, false)
                        {
                            #[allow(clippy::cast_precision_loss)]
                            col_values[idx].push(dt.timestamp_millis() as f64);
                        }
                    },
                    ColType::Numeric => {
                        if let Ok(val) = fast_float2::parse::<f64, _>(field)
                            && val.is_finite()
                        {
                            col_values[idx].push(val);
                        }
                    },
                }
            }
        }
    }

    Ok((col_names, col_values, col_types))
}

/// Parallel CSV reading for indexed files with >= 10k rows.
/// Splits the file into chunks, each read by a separate thread, then merges.
fn collect_numeric_values_parallel(
    rconfig: &Config,
    headers: &csv::ByteRecord,
    selected: &[usize],
    col_type_map: &[(usize, ColType)],
    idx_count: usize,
    flag_jobs: Option<usize>,
) -> CliResult<(Vec<String>, Vec<Vec<f64>>, Vec<ColType>)> {
    let col_names: Vec<String> = selected
        .iter()
        .map(|&i| {
            if rconfig.no_headers {
                (i + 1).to_string()
            } else {
                String::from_utf8_lossy(&headers[i]).into_owned()
            }
        })
        .collect();

    let col_types: Vec<ColType> = selected
        .iter()
        .map(|idx| {
            col_type_map
                .iter()
                .find(|(ci, _)| ci == idx)
                .map_or(ColType::Numeric, |&(_, ct)| ct)
        })
        .collect();

    let njobs = util::njobs(flag_jobs);
    let chunk_size = util::chunk_size(idx_count, njobs);
    let nchunks = util::num_of_chunks(idx_count, chunk_size);

    let pool = ThreadPool::new(njobs);
    let (send, recv) = crossbeam_channel::bounded(nchunks);

    let input_path_string = rconfig
        .path
        .as_ref()
        .unwrap()
        .to_str()
        .unwrap_or("")
        .to_string();
    let selected_vec = selected.to_vec();
    let col_types_vec = col_types.clone();
    let delimiter = Delimiter(rconfig.get_delimiter());
    let no_headers = rconfig.no_headers;

    for chunk_idx in 0..nchunks {
        let send = send.clone();
        let input_path_string = input_path_string.clone();
        let selected = selected_vec.clone();
        let col_types = col_types_vec.clone();
        let num_cols = selected.len();
        let records_in_chunk = if chunk_idx == nchunks - 1 {
            idx_count - chunk_idx * chunk_size
        } else {
            chunk_size
        };

        pool.execute(move || {
            let rconfig_chunk = Config::new(Some(&input_path_string))
                .delimiter(Some(delimiter))
                .no_headers_flag(no_headers);
            let Ok(Some(mut idx)) = rconfig_chunk.indexed() else {
                let _ = send.send(Err(CliError::Other(
                    "Failed to open index for parallel reading".to_string(),
                )));
                return;
            };

            if let Err(e) = idx.seek((chunk_idx * chunk_size) as u64) {
                let _ = send.send(Err(CliError::Other(format!("Seek failed: {e}"))));
                return;
            }

            let mut chunk_values: Vec<Vec<f64>> =
                vec![Vec::with_capacity(records_in_chunk); num_cols];

            for result in idx.byte_records().take(records_in_chunk) {
                let Ok(record) = result else {
                    continue;
                };
                for (col_pos, &col_idx) in selected.iter().enumerate() {
                    if let Some(field) = record.get(col_idx) {
                        match col_types[col_pos] {
                            ColType::Date | ColType::DateTime => {
                                if let Ok(s) = simdutf8::basic::from_utf8(field)
                                    && !s.is_empty()
                                    && let Ok(dt) = qsv_dateparser::parse_with_preference(s, false)
                                {
                                    #[allow(clippy::cast_precision_loss)]
                                    chunk_values[col_pos].push(dt.timestamp_millis() as f64);
                                }
                            },
                            ColType::Numeric => {
                                if let Ok(val) = fast_float2::parse::<f64, _>(field)
                                    && val.is_finite()
                                {
                                    chunk_values[col_pos].push(val);
                                }
                            },
                        }
                    }
                }
            }

            let _ = send.send(Ok((chunk_idx, chunk_values)));
        });
    }

    drop(send);

    // Collect chunk results, then merge in chunk-index order to preserve row ordering
    let mut chunks: Vec<(usize, Vec<Vec<f64>>)> = Vec::with_capacity(nchunks);
    for chunk_result in &recv {
        let (idx, values) = chunk_result?;
        chunks.push((idx, values));
    }
    chunks.sort_unstable_by_key(|(idx, _)| *idx);

    let mut col_values: Vec<Vec<f64>> = vec![Vec::with_capacity(idx_count); selected.len()];
    for (_chunk_idx, chunk_values) in chunks {
        for (col_pos, chunk_col) in chunk_values.into_iter().enumerate() {
            col_values[col_pos].extend(chunk_col);
        }
    }

    Ok((col_names, col_values, col_types))
}

fn compute_one_sample<'a>(name: &'a str, values: &[f64], misrate: f64) -> OneSampleResult<'a> {
    let n = values.len();

    if n == 0 {
        return OneSampleResult {
            field: name,
            n,
            center: None,
            spread: None,
            center_lower: None,
            center_upper: None,
            spread_lower: None,
            spread_upper: None,
        };
    }

    let center = pragmastat::estimators::raw::center(values).ok();
    let spread = pragmastat::estimators::raw::spread(values).ok();
    let center_bounds = pragmastat::estimators::raw::center_bounds(values, misrate).ok();
    let spread_bounds = pragmastat::estimators::raw::spread_bounds(values, misrate).ok();

    OneSampleResult {
        field: name,
        n,
        center,
        spread,
        center_lower: center_bounds.map(|b| b.lower),
        center_upper: center_bounds.map(|b| b.upper),
        spread_lower: spread_bounds.map(|b| b.lower),
        spread_upper: spread_bounds.map(|b| b.upper),
    }
}

fn compute_two_sample<'a>(
    name_x: &'a str,
    name_y: &'a str,
    x: &[f64],
    y: &[f64],
    log_x: Option<&[f64]>,
    log_y: Option<&[f64]>,
    misrate: f64,
) -> TwoSampleResult<'a> {
    let n_x = x.len();
    let n_y = y.len();

    if n_x == 0 || n_y == 0 {
        return TwoSampleResult {
            field_x: name_x,
            field_y: name_y,
            n_x,
            n_y,
            shift: None,
            ratio: None,
            disparity: None,
            shift_lower: None,
            shift_upper: None,
            ratio_lower: None,
            ratio_upper: None,
            disparity_lower: None,
            disparity_upper: None,
        };
    }

    let shift = pragmastat::estimators::raw::shift(x, y).ok();
    let shift_bounds = pragmastat::estimators::raw::shift_bounds(x, y, misrate).ok();
    let disparity = pragmastat::estimators::raw::disparity(x, y).ok();
    let disparity_bounds = pragmastat::estimators::raw::disparity_bounds(x, y, misrate).ok();

    // Use pre-computed log-transformed slices for ratio computation.
    // Both must be Some (all values > 0) for ratio to be valid.
    let (ratio, ratio_lower, ratio_upper) = if let (Some(lx), Some(ly)) = (log_x, log_y) {
        let ratio = pragmastat::estimators::raw::shift(lx, ly)
            .ok()
            .map(f64::exp);
        let ratio_bounds = pragmastat::estimators::raw::shift_bounds(lx, ly, misrate)
            .ok()
            .map(|b| (b.lower.exp(), b.upper.exp()));
        (
            ratio,
            ratio_bounds.map(|(lo, _)| lo),
            ratio_bounds.map(|(_, hi)| hi),
        )
    } else {
        (None, None, None)
    };

    TwoSampleResult {
        field_x: name_x,
        field_y: name_y,
        n_x,
        n_y,
        shift,
        ratio,
        disparity,
        shift_lower: shift_bounds.map(|b| b.lower),
        shift_upper: shift_bounds.map(|b| b.upper),
        ratio_lower,
        ratio_upper,
        disparity_lower: disparity_bounds.map(|b| b.lower),
        disparity_upper: disparity_bounds.map(|b| b.upper),
    }
}

fn fmt_opt(val: Option<f64>, round: u32) -> String {
    val.map_or_else(String::new, |v| util::round_num(v, round))
}

/// Format an epoch-ms value as a date or datetime string.
/// Returns an empty string for out-of-range timestamps.
fn fmt_timestamp(val: Option<f64>, ct: ColType) -> String {
    match val {
        None => String::new(),
        Some(v) => {
            #[allow(clippy::cast_possible_truncation)]
            let ts = v.round() as i64;
            match chrono::DateTime::from_timestamp_millis(ts) {
                None => String::new(),
                Some(dt) => {
                    if ct == ColType::Date {
                        dt.format("%Y-%m-%d").to_string()
                    } else {
                        dt.to_rfc3339()
                    }
                },
            }
        },
    }
}

/// Format an epoch-ms difference as days with sub-second precision.
fn fmt_days(val: Option<f64>) -> String {
    val.map_or_else(String::new, |v| {
        util::round_num(v / MS_IN_DAY, DAY_DECIMAL_PLACES)
    })
}

/// Pick the right formatter for a "point estimate" (center, bounds).
fn fmt_point(val: Option<f64>, ct: ColType, round: u32) -> String {
    if ct == ColType::Numeric {
        fmt_opt(val, round)
    } else {
        fmt_timestamp(val, ct)
    }
}

/// Pick the right formatter for a "spread/shift" value (spread, shift, bounds).
fn fmt_spread(val: Option<f64>, ct: ColType, round: u32) -> String {
    if ct == ColType::Numeric {
        fmt_opt(val, round)
    } else {
        fmt_days(val)
    }
}

fn write_onesample_header(
    wtr: &mut csv::Writer<Box<dyn std::io::Write + 'static>>,
) -> CliResult<()> {
    wtr.write_record([
        "field",
        "n",
        "center",
        "spread",
        "center_lower",
        "center_upper",
        "spread_lower",
        "spread_upper",
    ])?;
    Ok(())
}

fn write_onesample_row(
    wtr: &mut csv::Writer<Box<dyn std::io::Write + 'static>>,
    r: &OneSampleResult,
    ct: ColType,
    round: u32,
) -> CliResult<()> {
    let mut itoa_buf = itoa::Buffer::new();
    wtr.write_record([
        r.field,
        itoa_buf.format(r.n),
        &fmt_point(r.center, ct, round),
        &fmt_spread(r.spread, ct, round),
        &fmt_point(r.center_lower, ct, round),
        &fmt_point(r.center_upper, ct, round),
        &fmt_spread(r.spread_lower, ct, round),
        &fmt_spread(r.spread_upper, ct, round),
    ])?;
    Ok(())
}

fn write_twosample_header(
    wtr: &mut csv::Writer<Box<dyn std::io::Write + 'static>>,
) -> CliResult<()> {
    wtr.write_record([
        "field_x",
        "field_y",
        "n_x",
        "n_y",
        "shift",
        "ratio",
        "disparity",
        "shift_lower",
        "shift_upper",
        "ratio_lower",
        "ratio_upper",
        "disparity_lower",
        "disparity_upper",
    ])?;
    Ok(())
}

fn write_twosample_row(
    wtr: &mut csv::Writer<Box<dyn std::io::Write + 'static>>,
    r: &TwoSampleResult,
    ct: ColType,
    round: u32,
) -> CliResult<()> {
    let mut itoa_buf_x = itoa::Buffer::new();
    let mut itoa_buf_y = itoa::Buffer::new();
    let n_x_str = itoa_buf_x.format(r.n_x);
    let n_y_str = itoa_buf_y.format(r.n_y);
    wtr.write_record([
        r.field_x,
        r.field_y,
        n_x_str,
        n_y_str,
        &fmt_spread(r.shift, ct, round), // shift is a difference → days for dates
        &fmt_opt(r.ratio, round),        // ratio is dimensionless → always numeric
        &fmt_opt(r.disparity, round),    // disparity is dimensionless → always numeric
        &fmt_spread(r.shift_lower, ct, round),
        &fmt_spread(r.shift_upper, ct, round),
        &fmt_opt(r.ratio_lower, round),
        &fmt_opt(r.ratio_upper, round),
        &fmt_opt(r.disparity_lower, round),
        &fmt_opt(r.disparity_upper, round),
    ])?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Compare1 / Compare2 — confirmatory analysis
// ---------------------------------------------------------------------------

struct Compare1Result<'a> {
    field:     &'a str,
    n:         usize,
    metric:    &'static str,
    threshold: f64,
    estimate:  Option<f64>,
    lower:     Option<f64>,
    upper:     Option<f64>,
    verdict:   &'static str,
}

struct Compare2Result<'a> {
    field_x:   &'a str,
    field_y:   &'a str,
    n_x:       usize,
    n_y:       usize,
    metric:    &'static str,
    threshold: f64,
    estimate:  Option<f64>,
    lower:     Option<f64>,
    upper:     Option<f64>,
    verdict:   &'static str,
}

fn parse_thresholds(
    spec: &str,
    misrate: f64,
    is_compare1: bool,
) -> CliResult<Vec<pragmastat::Threshold>> {
    let valid_metrics: &[&str] = if is_compare1 {
        &["center", "spread"]
    } else {
        &["shift", "ratio", "disparity"]
    };
    let mode = if is_compare1 {
        "--compare1"
    } else {
        "--compare2"
    };

    let mut thresholds = Vec::new();
    for part in spec.split(',') {
        let part = part.trim();
        let Some((metric_str, value_str)) = part.split_once(':') else {
            return Err(CliError::IncorrectUsage(format!(
                "Invalid threshold format \"{part}\". Expected metric:value (e.g. center:42.0).",
            )));
        };
        let metric_str = metric_str.trim();
        let value_str = value_str.trim();
        let metric_lower = metric_str.to_ascii_lowercase();
        if !valid_metrics.contains(&metric_lower.as_str()) {
            return Err(CliError::IncorrectUsage(format!(
                "Invalid metric \"{metric_str}\" for {mode}. Valid metrics: {}.",
                valid_metrics.join(", "),
            )));
        }
        let value: f64 = value_str.parse().map_err(|_| {
            CliError::IncorrectUsage(format!(
                "Invalid threshold value \"{value_str}\". Expected a number.",
            ))
        })?;
        if !value.is_finite() {
            return Err(CliError::IncorrectUsage(format!(
                "Invalid threshold value \"{value_str}\". Thresholds must be finite real numbers.",
            )));
        }

        let metric = match metric_lower.as_str() {
            "center" => pragmastat::Metric::Center,
            "spread" => pragmastat::Metric::Spread,
            "shift" => pragmastat::Metric::Shift,
            "ratio" => pragmastat::Metric::Ratio,
            "disparity" => pragmastat::Metric::Disparity,
            _ => unreachable!(),
        };

        let threshold =
            pragmastat::Threshold::new(metric, pragmastat::Measurement::number(value), misrate)
                .map_err(|e| CliError::IncorrectUsage(format!("Threshold error: {e}")))?;
        thresholds.push(threshold);
    }

    if thresholds.is_empty() {
        return Err(CliError::IncorrectUsage(
            "At least one threshold must be specified.".to_string(),
        ));
    }
    Ok(thresholds)
}

fn compute_compare1<'a>(
    name: &'a str,
    values: &[f64],
    thresholds: &[pragmastat::Threshold],
) -> Vec<Compare1Result<'a>> {
    let n = values.len();

    let fallback = |t: &pragmastat::Threshold| Compare1Result {
        field: name,
        n,
        metric: t.metric().as_str(),
        threshold: t.value().value,
        estimate: None,
        lower: None,
        upper: None,
        verdict: "",
    };

    if n == 0 {
        return thresholds.iter().map(fallback).collect();
    }

    let Ok(sample) = pragmastat::Sample::new(values.to_vec()) else {
        return thresholds.iter().map(fallback).collect();
    };

    match pragmastat::compare1(&sample, thresholds) {
        Ok(projections) => projections
            .iter()
            .map(|p| {
                let est = p.estimate().value;
                Compare1Result {
                    field: name,
                    n,
                    metric: p.threshold().metric().as_str(),
                    threshold: p.threshold().value().value,
                    estimate: Some(est),
                    lower: Some(p.bounds().lower),
                    upper: Some(p.bounds().upper),
                    verdict: p.verdict().as_str(),
                }
            })
            .collect(),
        Err(_) => thresholds.iter().map(fallback).collect(),
    }
}

fn compute_compare2<'a>(
    name_x: &'a str,
    name_y: &'a str,
    x: &[f64],
    y: &[f64],
    thresholds: &[pragmastat::Threshold],
) -> Vec<Compare2Result<'a>> {
    let n_x = x.len();
    let n_y = y.len();

    let fallback = |t: &pragmastat::Threshold| Compare2Result {
        field_x: name_x,
        field_y: name_y,
        n_x,
        n_y,
        metric: t.metric().as_str(),
        threshold: t.value().value,
        estimate: None,
        lower: None,
        upper: None,
        verdict: "",
    };

    if n_x == 0 || n_y == 0 {
        return thresholds.iter().map(fallback).collect();
    }

    let (Ok(sample_x), Ok(sample_y)) = (
        pragmastat::Sample::new(x.to_vec()),
        pragmastat::Sample::new(y.to_vec()),
    ) else {
        return thresholds.iter().map(fallback).collect();
    };

    match pragmastat::compare2(&sample_x, &sample_y, thresholds) {
        Ok(projections) => projections
            .iter()
            .map(|p| {
                let est = p.estimate().value;
                Compare2Result {
                    field_x: name_x,
                    field_y: name_y,
                    n_x,
                    n_y,
                    metric: p.threshold().metric().as_str(),
                    threshold: p.threshold().value().value,
                    estimate: Some(est),
                    lower: Some(p.bounds().lower),
                    upper: Some(p.bounds().upper),
                    verdict: p.verdict().as_str(),
                }
            })
            .collect(),
        Err(_) => thresholds.iter().map(fallback).collect(),
    }
}

fn write_compare1_header(
    wtr: &mut csv::Writer<Box<dyn std::io::Write + 'static>>,
) -> CliResult<()> {
    wtr.write_record([
        "field",
        "n",
        "metric",
        "threshold",
        "estimate",
        "lower",
        "upper",
        "verdict",
    ])?;
    Ok(())
}

fn write_compare1_row(
    wtr: &mut csv::Writer<Box<dyn std::io::Write + 'static>>,
    r: &Compare1Result,
    ct: ColType,
    round: u32,
) -> CliResult<()> {
    // center metric → point estimate formatting; spread → dispersion formatting
    let is_center = r.metric == "center";
    let fmt_val = |v| {
        if is_center {
            fmt_point(v, ct, round)
        } else {
            fmt_spread(v, ct, round)
        }
    };
    let mut itoa_buf = itoa::Buffer::new();
    wtr.write_record([
        r.field,
        itoa_buf.format(r.n),
        r.metric,
        &fmt_val(Some(r.threshold)),
        &fmt_val(r.estimate),
        &fmt_val(r.lower),
        &fmt_val(r.upper),
        r.verdict,
    ])?;
    Ok(())
}

fn write_compare2_header(
    wtr: &mut csv::Writer<Box<dyn std::io::Write + 'static>>,
) -> CliResult<()> {
    wtr.write_record([
        "field_x",
        "field_y",
        "n_x",
        "n_y",
        "metric",
        "threshold",
        "estimate",
        "lower",
        "upper",
        "verdict",
    ])?;
    Ok(())
}

fn write_compare2_row(
    wtr: &mut csv::Writer<Box<dyn std::io::Write + 'static>>,
    r: &Compare2Result,
    ct: ColType,
    round: u32,
) -> CliResult<()> {
    // shift → days for dates; ratio/disparity → always numeric
    let is_shift = r.metric == "shift";
    let fmt_val = |v| {
        if is_shift {
            fmt_spread(v, ct, round)
        } else {
            fmt_opt(v, round)
        }
    };
    let mut itoa_buf_x = itoa::Buffer::new();
    let mut itoa_buf_y = itoa::Buffer::new();
    let n_x_str = itoa_buf_x.format(r.n_x);
    let n_y_str = itoa_buf_y.format(r.n_y);
    wtr.write_record([
        r.field_x,
        r.field_y,
        n_x_str,
        n_y_str,
        r.metric,
        &fmt_val(Some(r.threshold)),
        &fmt_val(r.estimate),
        &fmt_val(r.lower),
        &fmt_val(r.upper),
        r.verdict,
    ])?;
    Ok(())
}

fn write_compare1_results(
    wtr: &mut csv::Writer<Box<dyn std::io::Write + 'static>>,
    col_names: &[String],
    col_values: &[Vec<f64>],
    col_types: &[ColType],
    thresholds: &[pragmastat::Threshold],
    round: u32,
) -> CliResult<()> {
    write_compare1_header(wtr)?;

    let all_results: Vec<Vec<Compare1Result>> = col_names
        .par_iter()
        .enumerate()
        .map(|(i, name)| compute_compare1(name, &col_values[i], thresholds))
        .collect();

    for (i, results) in all_results.iter().enumerate() {
        for r in results {
            write_compare1_row(wtr, r, col_types[i], round)?;
        }
    }
    Ok(())
}

fn write_compare2_results(
    wtr: &mut csv::Writer<Box<dyn std::io::Write + 'static>>,
    col_names: &[String],
    col_values: &[Vec<f64>],
    col_types: &[ColType],
    thresholds: &[pragmastat::Threshold],
    round: u32,
) -> CliResult<()> {
    write_compare2_header(wtr)?;

    let k = col_names.len();
    if k < 2 {
        return Ok(());
    }
    let num_pairs = k * (k - 1) / 2;
    let num_rows = num_pairs * thresholds.len();
    if num_rows > 100 {
        winfo!(
            "computing {num_pairs} column pairs x {} thresholds = {num_rows} rows. Use --select \
             to limit columns for faster results.",
            thresholds.len()
        );
    }

    let pairs: Vec<(usize, usize)> = (0..k)
        .flat_map(|i| ((i + 1)..k).map(move |j| (i, j)))
        .collect();

    let all_results: Vec<Vec<Compare2Result>> = pairs
        .par_iter()
        .map(|&(i, j)| {
            compute_compare2(
                &col_names[i],
                &col_names[j],
                &col_values[i],
                &col_values[j],
                thresholds,
            )
        })
        .collect();

    for (pi, results) in all_results.iter().enumerate() {
        let (i, j) = pairs[pi];
        let pair_type = match (col_types[i], col_types[j]) {
            (ColType::Date, ColType::Date) => ColType::Date,
            (ColType::DateTime | ColType::Date, ColType::DateTime | ColType::Date) => {
                ColType::DateTime
            },
            (ColType::Date | ColType::DateTime, ColType::Numeric)
            | (ColType::Numeric, ColType::Date | ColType::DateTime) => {
                wwarn!(
                    "skipping mixed Date/Numeric pair ({}, {}) — comparison is not meaningful",
                    col_names[i],
                    col_names[j]
                );
                continue;
            },
            _ => ColType::Numeric,
        };
        for r in results {
            write_compare2_row(wtr, r, pair_type, round)?;
        }
    }
    Ok(())
}
