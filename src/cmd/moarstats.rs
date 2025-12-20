static USAGE: &str = r#"
Add 11 additional statistics and 12 outlier metadata to an existing stats CSV file.

The `moarstats` command extends an existing stats CSV file (created by the `stats` command)
by computing "moar" statistics that can be derived from existing stats columns.

It looks for the `<FILESTEM>.stats.csv` file for a given CSV input. If the stats CSV file
does not exist, it will first run the `stats` command with configurable options to establish
the baseline stats, to which it will add more stats columns.

If the `.stats.csv` file is found, it will skip running stats and just append the additional
stats columns.

Currently computes the following 11 additional statistics:
 1. Pearson's Second Skewness Coefficient: 3 * (mean - median) / stddev
    Measures asymmetry of the distribution.
    Positive values indicate right skew, negative values indicate left skew.
 2. Range to Standard Deviation Ratio: range / stddev
    Normalizes the spread of data.
    Higher values indicate more extreme outliers relative to the variability.
 3. Quartile Coefficient of Dispersion: (Q3 - Q1) / (Q3 + Q1)
    Measures relative variability using quartiles.
    Useful for comparing dispersion across different scales.
 4. Bowley's Skewness Coefficient: ((Q3 - Q2) - (Q2 - Q1)) / (Q3 - Q1)
    Robust measure of skewness using quartiles.
    Values range from -1 (left skew) to +1 (right skew).
 5. Z-Score of Mode: (mode - mean) / stddev
    Indicates how typical the mode is relative to the distribution.
    Values near 0 suggest the mode is near the mean.
 6. Relative Standard Error: sem / mean
    Measures precision of the mean estimate relative to its magnitude.
    Lower values indicate more reliable estimates.
 7. Z-Score of Min: (min - mean) / stddev
    Shows how extreme the minimum value is.
    Large negative values indicate outliers or heavy left tail.
 8. Z-Score of Max: (max - mean) / stddev
    Shows how extreme the maximum value is.
    Large positive values indicate outliers or heavy right tail.
 9. Median-to-Mean Ratio: median / mean
    Indicates skewness direction.
    Ratio < 1 suggests right skew, > 1 suggests left skew, = 1 suggests symmetry.
10. IQR-to-Range Ratio: iqr / range
    Measures concentration of data.
    Higher values (closer to 1) indicate more data concentrated in the middle 50%.
11. MAD-to-StdDev Ratio: mad / stddev
    Compares robust vs non-robust spread measures.
    Higher values suggest presence of outliers affecting stddev.

In addition, it computes the following 12 outlier statistics.
(requires --quartiles or --everything in stats):
  - outliers_extreme_lower: Count of values below the lower outer fence
  - outliers_mild_lower: Count of values between lower outer and inner fences
  - outliers_normal: Count of values between inner fences (non-outliers)
  - outliers_mild_upper: Count of values between upper inner and outer fences
  - outliers_extreme_upper: Count of values above the upper outer fence
  - outliers_total: Total count of all outliers (sum of extreme and mild outliers)
  - outliers_mean: Mean value of outliers
  - non_outliers_mean: Mean value of non-outliers
  - outliers_to_normal_mean_ratio: Ratio of outlier mean to non-outlier mean
  - outliers_min: Minimum value among outliers
  - outliers_max: Maximum value among outliers
  - outliers_range: Range of outlier values (max - min)

  These ourlier statistics require reading the original CSV file and comparing each
  value against the fence thresholds.
  Fences are computed using the IQR method:
    inner fences at Q1/Q3 ± 1.5*IQR, outer fences at Q1/Q3 ± 3.0*IQR.

These statistics are only computed for numeric and date/datetime columns where the required
base statistics are available. Outlier statistics additionally require that quartiles (and thus
fences) were computed when generating the stats CSV.

Examples:

  # Add moar stats to existing stats file
  $ qsv moarstats data.csv

  # Generate baseline stats first with custom options, then add moar stats
  $ qsv moarstats data.csv --stats-options "--everything --infer-dates"

  # Output to different file
  $ qsv moarstats data.csv --output enhanced_stats.csv

Usage:
    qsv moarstats [options] [<input>]
    qsv moarstats --help

moarstats options:
    --stats-options <arg>  Options to pass to the stats command if baseline stats need
                           to be generated. The options are passed as a single string
                           that will be split by whitespace.
                           [default: --infer-dates --infer-boolean --mad --quartiles --percentiles --force --stats-jsonl]
    --round <n>            Round statistics to <n> decimal places. Rounding follows
                           Midpoint Nearest Even (Bankers Rounding) rule.
                           [default: 4]

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of overwriting the stats CSV file.
"#;

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    time::Instant,
};

use crossbeam_channel;
use csv::{ReaderBuilder, WriterBuilder};
use indexmap::IndexMap;
#[cfg(feature = "feature_capable")]
use qsv_dateparser::parse_with_preference;
use serde::Deserialize;
#[cfg(feature = "feature_capable")]
use simdutf8::basic::from_utf8;
use threadpool::ThreadPool;

use crate::{CliError, CliResult, config::Config, util};

#[derive(Debug, Deserialize)]
struct Args {
    arg_input:          Option<String>,
    flag_stats_options: String,
    flag_round:         u32,
    flag_output:        Option<String>,
}

/// Get the stats CSV file path for a given input CSV path
fn get_stats_csv_path(input_path: &Path) -> CliResult<PathBuf> {
    let parent = input_path.parent().unwrap_or_else(|| Path::new("."));
    let fstem = input_path
        .file_stem()
        .ok_or_else(|| CliError::Other("Invalid input path: no file name".to_string()))?;

    let stats_filename = format!("{}.stats.csv", fstem.to_string_lossy());
    Ok(parent.join(stats_filename))
}

/// Compute Pearson's Second Skewness Coefficient: 3 * (mean - median) / stddev
fn compute_pearson_skewness(
    mean: Option<f64>,
    median: Option<f64>,
    stddev: Option<f64>,
) -> Option<f64> {
    if let (Some(mean_val), Some(median_val), Some(stddev_val)) = (mean, median, stddev) {
        if stddev_val.abs() > f64::EPSILON {
            Some(3.0 * (mean_val - median_val) / stddev_val)
        } else {
            None
        }
    } else {
        None
    }
}

/// Compute Range to Standard Deviation Ratio: range / stddev
fn compute_range_stddev_ratio(range: Option<f64>, stddev: Option<f64>) -> Option<f64> {
    if let (Some(range_val), Some(stddev_val)) = (range, stddev) {
        if stddev_val.abs() > f64::EPSILON {
            Some(range_val / stddev_val)
        } else {
            None
        }
    } else {
        None
    }
}

/// Compute Quartile Coefficient of Dispersion: (Q3 - Q1) / (Q3 + Q1)
///
/// Note: If Q1 or Q3 are negative, especially if both are negative and equal in magnitude,
/// the denominator (Q3 + Q1) may be zero or near zero, causing the result to be `None`.
/// Also, the standard formula may not yield meaningful results if Q1 is negative and
/// Q1 >= Q3 (i.e., quartiles are not in the expected order).
/// Return None if quartiles are not in a valid order (Q1 < Q3), or denominator is 0.
fn compute_quartile_coefficient_dispersion(q1: Option<f64>, q3: Option<f64>) -> Option<f64> {
    if let (Some(q1_val), Some(q3_val)) = (q1, q3) {
        // Check that quartile order is valid (Q1 < Q3)
        if q1_val >= q3_val {
            return None;
        }
        let sum = q3_val + q1_val;
        // Only compute if the denominator is effectively non-zero to avoid division by zero and
        // instability.
        if sum.abs() <= f64::EPSILON {
            None
        } else {
            Some((q3_val - q1_val) / sum)
        }
    } else {
        None
    }
}

/// Compute Bowley's Skewness Coefficient: ((Q3 - Q2) - (Q2 - Q1)) / (Q3 - Q1)
/// Returns None if Q1, Q2, Q3 are not in valid order (Q1 <= Q2 <= Q3), or if any are None.
fn compute_bowley_skewness(q1: Option<f64>, q2: Option<f64>, q3: Option<f64>) -> Option<f64> {
    if let (Some(q1_val), Some(q2_val), Some(q3_val)) = (q1, q2, q3) {
        // Ensure quartiles are in valid order: Q1 <= Q2 <= Q3
        if q1_val <= q2_val && q2_val <= q3_val {
            let iqr = q3_val - q1_val;
            if iqr.abs() > f64::EPSILON {
                Some(((q3_val - q2_val) - (q2_val - q1_val)) / iqr)
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

/// Compute Z-Score of Mode: (mode - mean) / stddev
fn compute_mode_zscore(mode: Option<f64>, mean: Option<f64>, stddev: Option<f64>) -> Option<f64> {
    if let (Some(mode_val), Some(mean_val), Some(stddev_val)) = (mode, mean, stddev) {
        if stddev_val.abs() > f64::EPSILON {
            Some((mode_val - mean_val) / stddev_val)
        } else {
            None
        }
    } else {
        None
    }
}

/// Compute Relative Standard Error: sem / mean
fn compute_relative_standard_error(sem: Option<f64>, mean: Option<f64>) -> Option<f64> {
    if let (Some(sem_val), Some(mean_val)) = (sem, mean) {
        if mean_val.abs() > f64::EPSILON {
            Some(sem_val / mean_val)
        } else {
            None
        }
    } else {
        None
    }
}

/// Compute Z-Score: (value - mean) / stddev
fn compute_zscore(value: Option<f64>, mean: Option<f64>, stddev: Option<f64>) -> Option<f64> {
    if let (Some(val), Some(mean_val), Some(stddev_val)) = (value, mean, stddev) {
        if stddev_val.abs() > f64::EPSILON {
            Some((val - mean_val) / stddev_val)
        } else {
            None
        }
    } else {
        None
    }
}

/// Compute Median-to-Mean Ratio: median / mean
fn compute_median_mean_ratio(median: Option<f64>, mean: Option<f64>) -> Option<f64> {
    if let (Some(median_val), Some(mean_val)) = (median, mean) {
        if mean_val.abs() > f64::EPSILON {
            Some(median_val / mean_val)
        } else {
            None
        }
    } else {
        None
    }
}

/// Compute IQR-to-Range Ratio: iqr / range
fn compute_iqr_range_ratio(iqr: Option<f64>, range: Option<f64>) -> Option<f64> {
    if let (Some(iqr_val), Some(range_val)) = (iqr, range) {
        if range_val.abs() > f64::EPSILON {
            Some(iqr_val / range_val)
        } else {
            None
        }
    } else {
        None
    }
}

/// Compute MAD-to-StdDev Ratio: mad / stddev
fn compute_mad_stddev_ratio(mad: Option<f64>, stddev: Option<f64>) -> Option<f64> {
    if let (Some(mad_val), Some(stddev_val)) = (mad, stddev) {
        if stddev_val.abs() > f64::EPSILON {
            Some(mad_val / stddev_val)
        } else {
            None
        }
    } else {
        None
    }
}

/// Parse a numeric value from a string, handling empty strings and invalid values
fn parse_float_opt(s: &str) -> Option<f64> {
    if s.is_empty() {
        return None;
    }
    fast_float2::parse::<f64, &[u8]>(s.as_bytes()).ok()
}

/// Parse a numeric value from bytes, handling empty bytes and invalid values
fn parse_float_opt_from_bytes(bytes: &[u8]) -> Option<f64> {
    if bytes.is_empty() {
        return None;
    }
    fast_float2::parse::<f64, &[u8]>(bytes).ok()
}

/// Check if a type is numeric or date/datetime
fn is_numeric_or_date_type(typ: &str) -> bool {
    matches!(typ, "Integer" | "Float" | "Date" | "DateTime" | "Boolean")
}

/// Parse a date/datetime value and convert to days since epoch
/// Returns None if parsing fails or value is empty
#[cfg(feature = "feature_capable")]
fn parse_date_to_days(s: &str, prefer_dmy: bool) -> Option<f64> {
    if s.is_empty() {
        return None;
    }
    #[allow(clippy::cast_precision_loss)]
    parse_with_preference(s, prefer_dmy)
        .ok()
        .map(|dt| dt.timestamp_millis() as f64 / 86_400_000.0)
}

/// Field information needed for outlier counting
#[derive(Clone)]
struct OutlierFieldInfo {
    col_idx:     usize,
    field_type:  String,
    lower_outer: f64,
    lower_inner: f64,
    upper_inner: f64,
    upper_outer: f64,
}

/// Statistics tracked during outlier scanning
#[derive(Clone, Default)]
struct OutlierStats {
    // Counts: [extreme_lower, mild_lower, normal, mild_upper, extreme_upper, total]
    counts:       [u64; 6],
    // Sums
    sum_outliers: f64,
    sum_normal:   f64,
    sum_all:      f64,
    // Min/Max
    min_outliers: Option<f64>,
    max_outliers: Option<f64>,
    min_normal:   Option<f64>,
    max_normal:   Option<f64>,
}

/// Count outliers for a chunk of records and compute statistics
/// Returns a HashMap mapping field names to their outlier statistics
fn count_chunk_outliers<I>(
    fields_to_count: &HashMap<String, OutlierFieldInfo>,
    records: I,
) -> CliResult<HashMap<String, OutlierStats>>
where
    I: Iterator<Item = csv::Result<csv::ByteRecord>>,
{
    if fields_to_count.is_empty() {
        return Ok(HashMap::new());
    }

    // Initialize statistics for all fields
    let mut chunk_stats: HashMap<String, OutlierStats> = fields_to_count
        .keys()
        .map(|k| (k.clone(), OutlierStats::default()))
        .collect();

    #[cfg(feature = "feature_capable")]
    let prefer_dmy = util::get_envvar_flag("QSV_PREFER_DMY");

    // Process each record in the chunk
    for result in records {
        let record = result?;

        for (field_name, field_info) in fields_to_count {
            let value_bytes = record.get(field_info.col_idx).unwrap_or(&[]);

            if value_bytes.is_empty() {
                continue; // Skip null/empty values
            }

            // Parse the value based on field type
            let numeric_value = if matches!(field_info.field_type.as_str(), "Date" | "DateTime") {
                #[cfg(feature = "feature_capable")]
                {
                    // Convert bytes to string for date parsing
                    if let Ok(value_str) = from_utf8(value_bytes) {
                        parse_date_to_days(value_str, prefer_dmy)
                    } else {
                        None
                    }
                }
                #[cfg(not(feature = "feature_capable"))]
                {
                    // Without dateparser feature, try parsing as float
                    parse_float_opt_from_bytes(value_bytes)
                }
            } else {
                parse_float_opt_from_bytes(value_bytes)
            };

            let Some(val) = numeric_value else {
                continue; // Skip values that can't be parsed
            };

            // Get mutable reference to stats for this field
            let stats = chunk_stats.get_mut(field_name).unwrap();

            // Update sums
            stats.sum_all += val;

            // Count outliers and track statistics based on fence comparisons
            if val < field_info.lower_outer {
                stats.counts[0] += 1; // extreme_lower
                stats.counts[5] += 1; // total
                stats.sum_outliers += val;
                stats.min_outliers = Some(stats.min_outliers.map_or(val, |m| m.min(val)));
                stats.max_outliers = Some(stats.max_outliers.map_or(val, |m| m.max(val)));
            } else if val < field_info.lower_inner {
                stats.counts[1] += 1; // mild_lower
                stats.counts[5] += 1; // total
                stats.sum_outliers += val;
                stats.min_outliers = Some(stats.min_outliers.map_or(val, |m| m.min(val)));
                stats.max_outliers = Some(stats.max_outliers.map_or(val, |m| m.max(val)));
            } else if val <= field_info.upper_inner {
                stats.counts[2] += 1; // normal
                stats.sum_normal += val;
                stats.min_normal = Some(stats.min_normal.map_or(val, |m| m.min(val)));
                stats.max_normal = Some(stats.max_normal.map_or(val, |m| m.max(val)));
            } else if val <= field_info.upper_outer {
                stats.counts[3] += 1; // mild_upper
                stats.counts[5] += 1; // total
                stats.sum_outliers += val;
                stats.min_outliers = Some(stats.min_outliers.map_or(val, |m| m.min(val)));
                stats.max_outliers = Some(stats.max_outliers.map_or(val, |m| m.max(val)));
            } else {
                stats.counts[4] += 1; // extreme_upper
                stats.counts[5] += 1; // total
                stats.sum_outliers += val;
                stats.min_outliers = Some(stats.min_outliers.map_or(val, |m| m.min(val)));
                stats.max_outliers = Some(stats.max_outliers.map_or(val, |m| m.max(val)));
            }
        }
    }

    Ok(chunk_stats)
}

/// Count outliers for all fields, using parallel processing if index is available
/// Returns a HashMap mapping field names to their outlier statistics
fn count_all_outliers(
    fields_to_count: &HashMap<String, OutlierFieldInfo>,
    input_path: &Path,
) -> CliResult<HashMap<String, OutlierStats>> {
    if fields_to_count.is_empty() {
        return Ok(HashMap::new());
    }

    // Check if index exists for parallel processing
    let input_path_str = input_path
        .to_str()
        .ok_or_else(|| CliError::Other(format!("Invalid input path: {}", input_path.display())))?;
    let input_path_string = input_path_str.to_string();
    let rconfig = Config::new(Some(&input_path_string));
    let indexed_result = rconfig.indexed()?;

    if let Some(idx) = indexed_result {
        // Parallel processing path
        let idx_count = idx.count() as usize;
        if idx_count == 0 {
            return Ok(HashMap::new());
        }

        // Only parallelize if file is large enough (threshold: 10k records)
        if idx_count < 10_000 {
            // Fall back to sequential for small files
            let mut rdr = rconfig.reader_file()?;
            let _headers = rdr.headers()?.clone();
            return count_all_outliers_from_reader(fields_to_count, rdr);
        }

        let njobs = util::njobs(None);
        let chunk_size = util::chunk_size(idx_count, njobs);
        let nchunks = util::num_of_chunks(idx_count, chunk_size);

        log::info!("Parallelizing outlier counting: {nchunks} chunks, {njobs} jobs");

        let pool = ThreadPool::new(njobs);
        let (send, recv) = crossbeam_channel::bounded(nchunks);

        // Process each chunk in parallel
        let input_path_string = input_path.to_str().unwrap_or("").to_string();
        for i in 0..nchunks {
            let (send, fields_to_count_clone, input_path_string_clone) = (
                send.clone(),
                fields_to_count.clone(),
                input_path_string.clone(),
            );
            pool.execute(move || {
                // Open index for this thread
                let rconfig_chunk = Config::new(Some(&input_path_string_clone));
                // safety: we know the file is indexed and seekable
                let Ok(Some(mut idx_chunk)) = rconfig_chunk.indexed() else {
                    // If we can't open index, send empty result
                    let _ = send.send(Ok(HashMap::new()));
                    return;
                };

                // Seek to chunk start position
                if let Err(e) = idx_chunk.seek((i * chunk_size) as u64) {
                    let _ = send.send(Err(CliError::Other(format!("Seek failed: {e}"))));
                    return;
                }

                // Process chunk records
                let it = idx_chunk.byte_records().take(chunk_size);
                let result = count_chunk_outliers(&fields_to_count_clone, it);
                let _ = send.send(result);
            });
        }

        drop(send);

        // Aggregate results from all chunks
        let mut all_stats: HashMap<String, OutlierStats> = fields_to_count
            .keys()
            .map(|k| (k.clone(), OutlierStats::default()))
            .collect();

        for chunk_result in &recv {
            let chunk_stats = chunk_result?;
            for (field_name, stats) in chunk_stats {
                if let Some(total_stats) = all_stats.get_mut(&field_name) {
                    // Aggregate counts
                    for i in 0..6 {
                        total_stats.counts[i] += stats.counts[i];
                    }
                    // Aggregate sums
                    total_stats.sum_outliers += stats.sum_outliers;
                    total_stats.sum_normal += stats.sum_normal;
                    total_stats.sum_all += stats.sum_all;
                    // Aggregate min/max
                    if let Some(min) = stats.min_outliers {
                        total_stats.min_outliers =
                            Some(total_stats.min_outliers.map_or(min, |m| m.min(min)));
                    }
                    if let Some(max) = stats.max_outliers {
                        total_stats.max_outliers =
                            Some(total_stats.max_outliers.map_or(max, |m| m.max(max)));
                    }
                    if let Some(min) = stats.min_normal {
                        total_stats.min_normal =
                            Some(total_stats.min_normal.map_or(min, |m| m.min(min)));
                    }
                    if let Some(max) = stats.max_normal {
                        total_stats.max_normal =
                            Some(total_stats.max_normal.map_or(max, |m| m.max(max)));
                    }
                }
            }
        }

        Ok(all_stats)
    } else {
        // Sequential fallback when no index exists
        let mut rdr = rconfig.reader_file()?;
        let _headers = rdr.headers()?.clone();
        count_all_outliers_from_reader(fields_to_count, rdr)
    }
}

/// Count outliers for all fields in a single pass through the CSV (sequential)
/// The CSV reader should already be positioned after the headers
/// Returns a HashMap mapping field names to their outlier statistics
fn count_all_outliers_from_reader(
    fields_to_count: &HashMap<String, OutlierFieldInfo>,
    mut rdr: csv::Reader<std::fs::File>,
) -> CliResult<HashMap<String, OutlierStats>> {
    if fields_to_count.is_empty() {
        return Ok(HashMap::new());
    }

    // Initialize statistics for all fields
    let mut all_stats: HashMap<String, OutlierStats> = fields_to_count
        .keys()
        .map(|k| (k.clone(), OutlierStats::default()))
        .collect();

    #[cfg(feature = "feature_capable")]
    let prefer_dmy = util::get_envvar_flag("QSV_PREFER_DMY");

    // Process each record once, checking all fields
    for result in rdr.records() {
        let record = result?;

        for (field_name, field_info) in fields_to_count {
            let value_str = record.get(field_info.col_idx).unwrap_or("");

            if value_str.is_empty() {
                continue; // Skip null/empty values
            }

            // Parse the value based on field type
            let numeric_value = if matches!(field_info.field_type.as_str(), "Date" | "DateTime") {
                #[cfg(feature = "feature_capable")]
                {
                    parse_date_to_days(value_str, prefer_dmy)
                }
                #[cfg(not(feature = "feature_capable"))]
                {
                    // Without dateparser feature, try parsing as float
                    parse_float_opt(value_str)
                }
            } else {
                parse_float_opt(value_str)
            };

            let Some(val) = numeric_value else {
                continue; // Skip values that can't be parsed
            };

            // Get mutable reference to stats for this field
            let stats = all_stats.get_mut(field_name).unwrap();

            // Update sums
            stats.sum_all += val;

            // Count outliers and track statistics based on fence comparisons
            if val < field_info.lower_outer {
                stats.counts[0] += 1; // extreme_lower
                stats.counts[5] += 1; // total
                stats.sum_outliers += val;
                stats.min_outliers = Some(stats.min_outliers.map_or(val, |m| m.min(val)));
                stats.max_outliers = Some(stats.max_outliers.map_or(val, |m| m.max(val)));
            } else if val < field_info.lower_inner {
                stats.counts[1] += 1; // mild_lower
                stats.counts[5] += 1; // total
                stats.sum_outliers += val;
                stats.min_outliers = Some(stats.min_outliers.map_or(val, |m| m.min(val)));
                stats.max_outliers = Some(stats.max_outliers.map_or(val, |m| m.max(val)));
            } else if val <= field_info.upper_inner {
                stats.counts[2] += 1; // normal
                stats.sum_normal += val;
                stats.min_normal = Some(stats.min_normal.map_or(val, |m| m.min(val)));
                stats.max_normal = Some(stats.max_normal.map_or(val, |m| m.max(val)));
            } else if val <= field_info.upper_outer {
                stats.counts[3] += 1; // mild_upper
                stats.counts[5] += 1; // total
                stats.sum_outliers += val;
                stats.min_outliers = Some(stats.min_outliers.map_or(val, |m| m.min(val)));
                stats.max_outliers = Some(stats.max_outliers.map_or(val, |m| m.max(val)));
            } else {
                stats.counts[4] += 1; // extreme_upper
                stats.counts[5] += 1; // total
                stats.sum_outliers += val;
                stats.min_outliers = Some(stats.min_outliers.map_or(val, |m| m.min(val)));
                stats.max_outliers = Some(stats.max_outliers.map_or(val, |m| m.max(val)));
            }
        }
    }

    Ok(all_stats)
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let start_time = Instant::now();
    let args: Args = util::get_args(USAGE, argv)?;

    // Check if input file is provided
    let input_path_str = args
        .arg_input
        .ok_or_else(|| CliError::IncorrectUsage("No input file specified.".to_string()))?;

    let input_path = Path::new(&input_path_str);
    if !input_path.exists() {
        return fail_clierror!("Input file does not exist: {}", input_path.display());
    }

    // Determine stats CSV path
    let stats_csv_path = get_stats_csv_path(input_path)?;

    // Check if stats CSV exists, if not, run stats command
    if !stats_csv_path.exists() {
        eprintln!("Stats CSV file not found: {}", stats_csv_path.display());

        // Parse stats options
        let stats_args_vec: Vec<&str> = args.flag_stats_options.split_whitespace().collect();
        let _ = util::run_qsv_cmd(
            "stats",
            &stats_args_vec,
            &input_path_str,
            "Running stats command to generate baseline stats...",
        )?;
        if !stats_csv_path.exists() {
            return fail_clierror!(
                "Stats CSV file was not created: {}",
                stats_csv_path.display()
            );
        }
    }

    // Read the stats CSV file
    let stats_csv_content = fs::read_to_string(&stats_csv_path)?;

    // Parse the stats CSV
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_csv_content.as_bytes());

    let headers = rdr.headers()?.clone();

    let type_idx = headers
        .iter()
        .position(|h| h == "type")
        .ok_or_else(|| CliError::Other("Stats CSV missing 'type' column".to_string()))?;

    let mean_idx = headers.iter().position(|h| h == "mean");
    let median_idx = headers.iter().position(|h| h == "median");
    let q2_median_idx = headers.iter().position(|h| h == "q2_median");
    let stddev_idx = headers.iter().position(|h| h == "stddev");
    let range_idx = headers.iter().position(|h| h == "range");
    let q1_idx = headers.iter().position(|h| h == "q1");
    let q3_idx = headers.iter().position(|h| h == "q3");
    let mode_idx = headers.iter().position(|h| h == "mode");
    let sem_idx = headers.iter().position(|h| h == "sem");
    let min_idx = headers.iter().position(|h| h == "min");
    let max_idx = headers.iter().position(|h| h == "max");
    let iqr_idx = headers.iter().position(|h| h == "iqr");
    let mad_idx = headers.iter().position(|h| h == "mad");
    let field_idx = headers.iter().position(|h| h == "field");
    let lower_outer_fence_idx = headers.iter().position(|h| h == "lower_outer_fence");
    let lower_inner_fence_idx = headers.iter().position(|h| h == "lower_inner_fence");
    let upper_inner_fence_idx = headers.iter().position(|h| h == "upper_inner_fence");
    let upper_outer_fence_idx = headers.iter().position(|h| h == "upper_outer_fence");

    // Helper function to check if a column already exists in headers
    let column_exists = |col_name: &str| headers.iter().any(|h| h == col_name);

    // Check which new columns we can add (based on available base stats)
    // Skip columns that already exist to avoid duplicates
    let mut new_columns = Vec::new();
    let mut new_column_indices = IndexMap::new();

    if mean_idx.is_some()
        && (median_idx.is_some() || q2_median_idx.is_some())
        && stddev_idx.is_some()
        && !column_exists("pearson_skewness")
    {
        new_columns.push("pearson_skewness");
        new_column_indices.insert("pearson_skewness".to_string(), new_columns.len() - 1);
    }

    if range_idx.is_some() && stddev_idx.is_some() && !column_exists("range_stddev_ratio") {
        new_columns.push("range_stddev_ratio");
        new_column_indices.insert("range_stddev_ratio".to_string(), new_columns.len() - 1);
    }

    if q1_idx.is_some() && q3_idx.is_some() && !column_exists("quartile_coefficient_dispersion") {
        new_columns.push("quartile_coefficient_dispersion");
        new_column_indices.insert(
            "quartile_coefficient_dispersion".to_string(),
            new_columns.len() - 1,
        );
    }

    if q1_idx.is_some()
        && (q2_median_idx.is_some() || median_idx.is_some())
        && q3_idx.is_some()
        && !column_exists("bowley_skewness")
    {
        new_columns.push("bowley_skewness");
        new_column_indices.insert("bowley_skewness".to_string(), new_columns.len() - 1);
    }

    if mode_idx.is_some()
        && mean_idx.is_some()
        && stddev_idx.is_some()
        && !column_exists("mode_zscore")
    {
        new_columns.push("mode_zscore");
        new_column_indices.insert("mode_zscore".to_string(), new_columns.len() - 1);
    }

    if sem_idx.is_some() && mean_idx.is_some() && !column_exists("relative_standard_error") {
        new_columns.push("relative_standard_error");
        new_column_indices.insert("relative_standard_error".to_string(), new_columns.len() - 1);
    }

    if min_idx.is_some()
        && mean_idx.is_some()
        && stddev_idx.is_some()
        && !column_exists("min_zscore")
    {
        new_columns.push("min_zscore");
        new_column_indices.insert("min_zscore".to_string(), new_columns.len() - 1);
    }

    if max_idx.is_some()
        && mean_idx.is_some()
        && stddev_idx.is_some()
        && !column_exists("max_zscore")
    {
        new_columns.push("max_zscore");
        new_column_indices.insert("max_zscore".to_string(), new_columns.len() - 1);
    }

    if (median_idx.is_some() || q2_median_idx.is_some())
        && mean_idx.is_some()
        && !column_exists("median_mean_ratio")
    {
        new_columns.push("median_mean_ratio");
        new_column_indices.insert("median_mean_ratio".to_string(), new_columns.len() - 1);
    }

    if iqr_idx.is_some() && range_idx.is_some() && !column_exists("iqr_range_ratio") {
        new_columns.push("iqr_range_ratio");
        new_column_indices.insert("iqr_range_ratio".to_string(), new_columns.len() - 1);
    }

    if mad_idx.is_some() && stddev_idx.is_some() && !column_exists("mad_stddev_ratio") {
        new_columns.push("mad_stddev_ratio");
        new_column_indices.insert("mad_stddev_ratio".to_string(), new_columns.len() - 1);
    }

    // Add outlier count columns if all fences are available
    // Only add if at least one outlier column doesn't exist (to avoid partial duplicates)
    if lower_outer_fence_idx.is_some()
        && lower_inner_fence_idx.is_some()
        && upper_inner_fence_idx.is_some()
        && upper_outer_fence_idx.is_some()
        && !column_exists("outliers_extreme_lower")
    {
        new_columns.push("outliers_extreme_lower");
        new_column_indices.insert("outliers_extreme_lower".to_string(), new_columns.len() - 1);
        new_columns.push("outliers_mild_lower");
        new_column_indices.insert("outliers_mild_lower".to_string(), new_columns.len() - 1);
        new_columns.push("outliers_normal");
        new_column_indices.insert("outliers_normal".to_string(), new_columns.len() - 1);
        new_columns.push("outliers_mild_upper");
        new_column_indices.insert("outliers_mild_upper".to_string(), new_columns.len() - 1);
        new_columns.push("outliers_extreme_upper");
        new_column_indices.insert("outliers_extreme_upper".to_string(), new_columns.len() - 1);
        new_columns.push("outliers_total");
        new_column_indices.insert("outliers_total".to_string(), new_columns.len() - 1);
        // Additional statistics computed during outlier scanning
        new_columns.push("outliers_mean");
        new_column_indices.insert("outliers_mean".to_string(), new_columns.len() - 1);
        new_columns.push("non_outliers_mean");
        new_column_indices.insert("non_outliers_mean".to_string(), new_columns.len() - 1);
        new_columns.push("outliers_to_normal_mean_ratio");
        new_column_indices.insert(
            "outliers_to_normal_mean_ratio".to_string(),
            new_columns.len() - 1,
        );
        new_columns.push("outliers_min");
        new_column_indices.insert("outliers_min".to_string(), new_columns.len() - 1);
        new_columns.push("outliers_max");
        new_column_indices.insert("outliers_max".to_string(), new_columns.len() - 1);
        new_columns.push("outliers_range");
        new_column_indices.insert("outliers_range".to_string(), new_columns.len() - 1);
    }

    if new_columns.is_empty() {
        // Check if any moarstats columns already exist to determine the reason
        let moarstats_columns = [
            "pearson_skewness",
            "range_stddev_ratio",
            "quartile_coefficient_dispersion",
            "bowley_skewness",
            "mode_zscore",
            "relative_standard_error",
            "min_zscore",
            "max_zscore",
            "median_mean_ratio",
            "iqr_range_ratio",
            "mad_stddev_ratio",
            "outliers_extreme_lower",
        ];

        let any_exist = moarstats_columns.iter().any(|col| column_exists(col));

        if any_exist {
            eprintln!(
                "Warning: No additional stats can be computed. All available additional \
                 statistics have already been added to this stats CSV file."
            );
        } else {
            eprintln!(
                "Warning: No additional stats can be computed with the available base statistics."
            );
            eprintln!(
                "Consider running stats with --everything, or including --quartiles --median \
                 --mode in your --stats-options."
            );
        }
        return Ok(());
    }

    // Read all records
    let mut records = Vec::new();
    for result in rdr.records() {
        let record = result?;
        records.push(record);
    }

    // Collect fields that need outlier counting and count them in a single pass
    let mut fields_to_count: HashMap<String, OutlierFieldInfo> = HashMap::new();
    let needs_outlier_counting = new_column_indices.contains_key("outliers_extreme_lower");

    // First pass: collect field information from stats records
    if needs_outlier_counting {
        for record in &records {
            let field_name = field_idx.and_then(|idx| record.get(idx)).unwrap_or("");
            let field_type = record.get(type_idx).unwrap_or("");

            if field_name.is_empty()
                || field_type.is_empty()
                || !is_numeric_or_date_type(field_type)
            {
                continue;
            }

            // Parse fence values
            let lower_outer_fence = lower_outer_fence_idx
                .and_then(|idx| record.get(idx))
                .and_then(parse_float_opt);
            let lower_inner_fence = lower_inner_fence_idx
                .and_then(|idx| record.get(idx))
                .and_then(parse_float_opt);
            let upper_inner_fence = upper_inner_fence_idx
                .and_then(|idx| record.get(idx))
                .and_then(parse_float_opt);
            let upper_outer_fence = upper_outer_fence_idx
                .and_then(|idx| record.get(idx))
                .and_then(parse_float_opt);

            // All fences must be present
            if let (Some(lower_outer), Some(lower_inner), Some(upper_inner), Some(upper_outer)) = (
                lower_outer_fence,
                lower_inner_fence,
                upper_inner_fence,
                upper_outer_fence,
            ) {
                // We'll find the column index when we read the CSV
                fields_to_count.insert(
                    field_name.to_string(),
                    OutlierFieldInfo {
                        col_idx: 0, // Will be set when we read CSV headers
                        field_type: field_type.to_string(),
                        lower_outer,
                        lower_inner,
                        upper_inner,
                        upper_outer,
                    },
                );
            }
        }
    }

    // Count outliers for all fields in a single pass through the original CSV
    let outlier_counts = if fields_to_count.is_empty() {
        HashMap::new()
    } else {
        // Get headers to map field names to column indices
        let mut csv_rdr = ReaderBuilder::new()
            .has_headers(true)
            .from_path(input_path)?;
        let csv_headers = csv_rdr.headers()?.clone();

        // Update column indices in fields_to_count and remove fields not found in CSV
        fields_to_count.retain(|field_name, field_info| {
            if let Some(col_idx) = csv_headers.iter().position(|h| h == field_name) {
                field_info.col_idx = col_idx;
                true
            } else {
                false
            }
        });

        // Count outliers (will use parallel processing if index exists)
        count_all_outliers(&fields_to_count, input_path)?
    };

    // Prepare output
    let output_path: &Path = args.flag_output.as_ref().map_or(&stats_csv_path, Path::new);
    let mut wtr = WriterBuilder::new()
        .has_headers(true)
        .from_path(output_path)?;

    // Write headers with new columns appended
    let mut header_record = headers;
    for col in &new_columns {
        header_record.push_field(col);
    }
    wtr.write_record(&header_record)?;

    // Process each record
    #[allow(clippy::cast_precision_loss)]
    for record in &records {
        let mut output_record = record.clone();

        // Get field name and type (skip dataset stats rows that might not have proper type)
        let field_name = field_idx.and_then(|idx| record.get(idx)).unwrap_or("");
        let field_type = record.get(type_idx).unwrap_or("");

        // Only compute stats for numeric/date types
        if !field_type.is_empty() && is_numeric_or_date_type(field_type) {
            // Parse existing stats values
            let mean = mean_idx
                .and_then(|idx| record.get(idx))
                .and_then(parse_float_opt);
            let median = median_idx
                .and_then(|idx| record.get(idx))
                .and_then(parse_float_opt)
                .or_else(|| {
                    q2_median_idx
                        .and_then(|idx| record.get(idx))
                        .and_then(parse_float_opt)
                });
            let stddev = stddev_idx
                .and_then(|idx| record.get(idx))
                .and_then(parse_float_opt);
            let range = range_idx
                .and_then(|idx| record.get(idx))
                .and_then(parse_float_opt);
            let q1 = q1_idx
                .and_then(|idx| record.get(idx))
                .and_then(parse_float_opt);
            let q2 = q2_median_idx
                .and_then(|idx| record.get(idx))
                .and_then(parse_float_opt)
                .or_else(|| {
                    median_idx
                        .and_then(|idx| record.get(idx))
                        .and_then(parse_float_opt)
                });
            let q3 = q3_idx
                .and_then(|idx| record.get(idx))
                .and_then(parse_float_opt);

            // Parse mode (may be a string, need to try parsing as float)
            // If multiple modes are separated by "|", try parsing the first one
            let mode = mode_idx.and_then(|idx| record.get(idx)).and_then(|s| {
                if s.is_empty() {
                    None
                } else {
                    // Handle multiple modes separated by "|" - try first one
                    // safety: `split` on a non-empty string always yields at least one element,
                    // so `next` will always return `Some` and `unwrap` will not panic.
                    let first_mode = s.split('|').next().unwrap().trim();
                    parse_float_opt(first_mode)
                }
            });

            // Parse additional stats
            let sem = sem_idx
                .and_then(|idx| record.get(idx))
                .and_then(parse_float_opt);
            let min = min_idx
                .and_then(|idx| record.get(idx))
                .and_then(parse_float_opt);
            let max = max_idx
                .and_then(|idx| record.get(idx))
                .and_then(parse_float_opt);
            let iqr = iqr_idx
                .and_then(|idx| record.get(idx))
                .and_then(parse_float_opt);
            let mad = mad_idx
                .and_then(|idx| record.get(idx))
                .and_then(parse_float_opt);

            // Compute new stats
            let mut new_values = vec![String::new(); new_columns.len()];

            if let Some(idx) = new_column_indices.get("pearson_skewness")
                && let Some(val) = compute_pearson_skewness(mean, median, stddev)
            {
                new_values[*idx] = util::round_num(val, args.flag_round);
            }

            if let Some(idx) = new_column_indices.get("range_stddev_ratio")
                && let Some(val) = compute_range_stddev_ratio(range, stddev)
            {
                new_values[*idx] = util::round_num(val, args.flag_round);
            }

            if let Some(idx) = new_column_indices.get("quartile_coefficient_dispersion")
                && let Some(val) = compute_quartile_coefficient_dispersion(q1, q3)
            {
                new_values[*idx] = util::round_num(val, args.flag_round);
            }

            if let Some(idx) = new_column_indices.get("bowley_skewness")
                && let Some(val) = compute_bowley_skewness(q1, q2, q3)
            {
                new_values[*idx] = util::round_num(val, args.flag_round);
            }

            if let Some(idx) = new_column_indices.get("mode_zscore")
                && let Some(val) = compute_mode_zscore(mode, mean, stddev)
            {
                new_values[*idx] = util::round_num(val, args.flag_round);
            }

            if let Some(idx) = new_column_indices.get("relative_standard_error")
                && let Some(val) = compute_relative_standard_error(sem, mean)
            {
                new_values[*idx] = util::round_num(val, args.flag_round);
            }

            if let Some(idx) = new_column_indices.get("min_zscore")
                && let Some(val) = compute_zscore(min, mean, stddev)
            {
                new_values[*idx] = util::round_num(val, args.flag_round);
            }

            if let Some(idx) = new_column_indices.get("max_zscore")
                && let Some(val) = compute_zscore(max, mean, stddev)
            {
                new_values[*idx] = util::round_num(val, args.flag_round);
            }

            if let Some(idx) = new_column_indices.get("median_mean_ratio")
                && let Some(val) = compute_median_mean_ratio(median, mean)
            {
                new_values[*idx] = util::round_num(val, args.flag_round);
            }

            if let Some(idx) = new_column_indices.get("iqr_range_ratio")
                && let Some(val) = compute_iqr_range_ratio(iqr, range)
            {
                new_values[*idx] = util::round_num(val, args.flag_round);
            }

            if let Some(idx) = new_column_indices.get("mad_stddev_ratio")
                && let Some(val) = compute_mad_stddev_ratio(mad, stddev)
            {
                new_values[*idx] = util::round_num(val, args.flag_round);
            }

            // Get outlier statistics from pre-computed results
            if new_column_indices.contains_key("outliers_extreme_lower")
                && !field_name.is_empty()
                && let Some(stats) = outlier_counts.get(field_name)
            {
                // Write counts
                if let Some(idx) = new_column_indices.get("outliers_extreme_lower") {
                    new_values[*idx] = stats.counts[0].to_string();
                }
                if let Some(idx) = new_column_indices.get("outliers_mild_lower") {
                    new_values[*idx] = stats.counts[1].to_string();
                }
                if let Some(idx) = new_column_indices.get("outliers_normal") {
                    new_values[*idx] = stats.counts[2].to_string();
                }
                if let Some(idx) = new_column_indices.get("outliers_mild_upper") {
                    new_values[*idx] = stats.counts[3].to_string();
                }
                if let Some(idx) = new_column_indices.get("outliers_extreme_upper") {
                    new_values[*idx] = stats.counts[4].to_string();
                }
                if let Some(idx) = new_column_indices.get("outliers_total") {
                    new_values[*idx] = stats.counts[5].to_string();
                }

                // Compute and write additional statistics
                if stats.counts[5] > 0 {
                    // Mean of outliers
                    if let Some(idx) = new_column_indices.get("outliers_mean") {
                        let mean_outliers = stats.sum_outliers / stats.counts[5] as f64;
                        new_values[*idx] = util::round_num(mean_outliers, args.flag_round);
                    }
                }

                if stats.counts[2] > 0 {
                    // Mean of non-outliers
                    if let Some(idx) = new_column_indices.get("non_outliers_mean") {
                        let mean_normal = stats.sum_normal / stats.counts[2] as f64;
                        new_values[*idx] = util::round_num(mean_normal, args.flag_round);
                    }

                    // Outlier-to-normal mean ratio
                    if stats.counts[5] > 0
                        && let Some(idx) = new_column_indices.get("outliers_to_normal_mean_ratio")
                    {
                        let mean_outliers = stats.sum_outliers / stats.counts[5] as f64;
                        let mean_normal = stats.sum_normal / stats.counts[2] as f64;
                        if mean_normal.abs() > f64::EPSILON {
                            let ratio = mean_outliers / mean_normal;
                            new_values[*idx] = util::round_num(ratio, args.flag_round);
                        }
                    }
                }

                // Min/Max/Range of outliers
                if let Some(min_outliers) = stats.min_outliers
                    && let Some(idx) = new_column_indices.get("outliers_min")
                {
                    new_values[*idx] = util::round_num(min_outliers, args.flag_round);
                }
                if let Some(max_outliers) = stats.max_outliers {
                    if let Some(idx) = new_column_indices.get("outliers_max") {
                        new_values[*idx] = util::round_num(max_outliers, args.flag_round);
                    }
                    // Range of outliers
                    if let Some(min_outliers) = stats.min_outliers
                        && let Some(idx) = new_column_indices.get("outliers_range")
                    {
                        let range = max_outliers - min_outliers;
                        new_values[*idx] = util::round_num(range, args.flag_round);
                    }
                }
            }

            // Append new values to record
            for val in new_values {
                output_record.push_field(&val);
            }
        } else {
            // For non-numeric types, append empty strings
            for _ in &new_columns {
                output_record.push_field("");
            }
        }

        wtr.write_record(&output_record)?;
    }

    wtr.flush()?;

    eprintln!(
        "Added {} additional statistics columns to {}",
        new_columns.len(),
        output_path.display()
    );
    eprintln!("Elapsed: {:.2}s", start_time.elapsed().as_secs_f64());

    Ok(())
}
