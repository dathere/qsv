static USAGE: &str = r#"
Add additional statistics to an existing stats CSV file.

The `moarstats` command extends an existing stats CSV file (created by the `stats` command)
by computing "moar" statistics that can be derived from existing stats columns.

It looks for the `<FILESTEM>.stats.csv` file for a given CSV input. If the stats CSV file
does not exist, it will first run the `stats` command with configurable options to establish
the baseline stats, to which it will add more stats columns.

If the `.stats.csv` file is found, it will skip running stats and just append the additional
stats columns.

Currently computes the following additional statistics (Phase 1):
- Pearson's Second Skewness Coefficient: 3 * (mean - median) / stddev
- Range to Standard Deviation Ratio: range / stddev
- Quartile Coefficient of Dispersion: (Q3 - Q1) / (Q3 + Q1)
- Bowley's Skewness Coefficient: ((Q3 - Q2) - (Q2 - Q1)) / (Q3 - Q1)
- Z-Score of Mode: (mode - mean) / stddev

These statistics are only computed for numeric and date/datetime columns where the required
base statistics are available.

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
    fs,
    path::{Path, PathBuf},
    sync::OnceLock,
    time::Instant,
};

use csv::{ReaderBuilder, WriterBuilder};
use indexmap::IndexMap;
use serde::Deserialize;

use crate::{CliError, CliResult, util};

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
        if stddev_val > 0.0 {
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
        if stddev_val > 0.0 {
            Some(range_val / stddev_val)
        } else {
            None
        }
    } else {
        None
    }
}

/// Compute Quartile Coefficient of Dispersion: (Q3 - Q1) / (Q3 + Q1)
fn compute_quartile_coefficient_dispersion(q1: Option<f64>, q3: Option<f64>) -> Option<f64> {
    if let (Some(q1_val), Some(q3_val)) = (q1, q3) {
        let sum = q3_val + q1_val;
        // Only compute if the denominator is non-zero to avoid division by zero.
        if sum != 0.0 {
            Some((q3_val - q1_val) / sum)
        } else {
            None
        }
    } else {
        None
    }
}

/// Compute Bowley's Skewness Coefficient: ((Q3 - Q2) - (Q2 - Q1)) / (Q3 - Q1)
fn compute_bowley_skewness(q1: Option<f64>, q2: Option<f64>, q3: Option<f64>) -> Option<f64> {
    if let (Some(q1_val), Some(q2_val), Some(q3_val)) = (q1, q2, q3) {
        let iqr = q3_val - q1_val;
        if iqr > 0.0 {
            Some(((q3_val - q2_val) - (q2_val - q1_val)) / iqr)
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
        if stddev_val > 0.0 {
            Some((mode_val - mean_val) / stddev_val)
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

/// Check if a type is numeric or date/datetime
fn is_numeric_or_date_type(typ: &str) -> bool {
    matches!(typ, "Integer" | "Float" | "Date" | "DateTime" | "Boolean")
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

    // Check which new columns we can add (based on available base stats)
    let mut new_columns = Vec::new();
    let mut new_column_indices = IndexMap::new();

    if mean_idx.is_some()
        && (median_idx.is_some() || q2_median_idx.is_some())
        && stddev_idx.is_some()
    {
        new_columns.push("pearson_skewness");
        new_column_indices.insert("pearson_skewness".to_string(), new_columns.len() - 1);
    }

    if range_idx.is_some() && stddev_idx.is_some() {
        new_columns.push("range_stddev_ratio");
        new_column_indices.insert("range_stddev_ratio".to_string(), new_columns.len() - 1);
    }

    if q1_idx.is_some() && q3_idx.is_some() {
        new_columns.push("quartile_coefficient_dispersion");
        new_column_indices.insert(
            "quartile_coefficient_dispersion".to_string(),
            new_columns.len() - 1,
        );
    }

    if q1_idx.is_some() && (q2_median_idx.is_some() || median_idx.is_some()) && q3_idx.is_some() {
        new_columns.push("bowley_skewness");
        new_column_indices.insert("bowley_skewness".to_string(), new_columns.len() - 1);
    }

    if mode_idx.is_some() && mean_idx.is_some() && stddev_idx.is_some() {
        new_columns.push("mode_zscore");
        new_column_indices.insert("mode_zscore".to_string(), new_columns.len() - 1);
    }

    if new_columns.is_empty() {
        eprintln!(
            "Warning: No additional stats can be computed with the available base statistics."
        );
        eprintln!(
            "Consider running stats with --everything, or including --quartiles --median --mode \
             in your --stats-options."
        );
        return Ok(());
    }

    // Read all records
    let mut records = Vec::new();
    for result in rdr.records() {
        let record = result?;
        records.push(record);
    }

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
    for record in &records {
        let mut output_record = record.clone();

        // Get field type (skip dataset stats rows that might not have proper type)
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
                    let first_mode = s.split('|').next().unwrap_or(s).trim();
                    parse_float_opt(first_mode)
                }
            });

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
