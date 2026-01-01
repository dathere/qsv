static USAGE: &str = r#"
Add dozens of additional statistics, including extended outlier & robust statistics
to an existing stats CSV file. It also maps the field type to the most specific
W3C XML Schema Definition (XSD) datatype (https://www.w3.org/TR/xmlschema-2/).

The `moarstats` command extends an existing stats CSV file (created by the `stats` command)
by computing "moar" (https://www.dictionary.com/culture/slang/moar) statistics that can be
derived from existing stats columns and by scanning the original CSV file.

It looks for the `<FILESTEM>.stats.csv` file for a given CSV input. If the stats CSV file
does not exist, it will first run the `stats` command with configurable options to establish
the baseline stats, to which it will add more stats columns.

If the `.stats.csv` file is found, it will skip running stats and just append the additional
stats columns.

Currently computes the following 18 additional univariate statistics:
 1. Pearson's Second Skewness Coefficient: 3 * (mean - median) / stddev
    Measures asymmetry of the distribution.
    Positive values indicate right skew, negative values indicate left skew.
    https://en.wikipedia.org/wiki/Skewness
 2. Range to Standard Deviation Ratio: range / stddev
    Normalizes the spread of data.
    Higher values indicate more extreme outliers relative to the variability.
 3. Quartile Coefficient of Dispersion: (Q3 - Q1) / (Q3 + Q1)
    Measures relative variability using quartiles.
    Useful for comparing dispersion across different scales.
    https://en.wikipedia.org/wiki/Quartile_coefficient_of_dispersion
 4. Z-Score of Mode: (mode - mean) / stddev
    Indicates how typical the mode is relative to the distribution.
    Values near 0 suggest the mode is near the mean.
 5. Relative Standard Error: sem / mean
    Measures precision of the mean estimate relative to its magnitude.
    Lower values indicate more reliable estimates.
 6. Z-Score of Min: (min - mean) / stddev
    Shows how extreme the minimum value is.
    Large negative values indicate outliers or heavy left tail.
 7. Z-Score of Max: (max - mean) / stddev
    Shows how extreme the maximum value is.
    Large positive values indicate outliers or heavy right tail.
 8. Median-to-Mean Ratio: median / mean
    Indicates skewness direction.
    Ratio < 1 suggests right skew, > 1 suggests left skew, = 1 suggests symmetry.
 9. IQR-to-Range Ratio: iqr / range
    Measures concentration of data.
    Higher values (closer to 1) indicate more data concentrated in the middle 50%.
10. MAD-to-StdDev Ratio: mad / stddev
    Compares robust vs non-robust spread measures.
    Higher values suggest presence of outliers affecting stddev.
11. Kurtosis: Measures the "tailedness" of the distribution (excess kurtosis).
    Positive values indicate heavy tails, negative values indicate light tails.
    Values near 0 indicate a normal distribution.
    Requires --advanced flag.
    https://en.wikipedia.org/wiki/Kurtosis
12. Bimodality Coefficient: Measures whether a distribution has two modes (peaks) or is unimodal.
    BC < 0.555 indicates unimodal, BC >= 0.555 indicates bimodal/multimodal.
    Computed as (skewness² + 1) / (kurtosis + 3).
    Requires --advanced flag (needs skewness from base stats and kurtosis from --advanced flag).
    https://en.wikipedia.org/wiki/Bimodality
13. Gini Coefficient: Measures inequality/dispersion in the distribution.
    Values range from 0 (perfect equality) to 1 (maximum inequality).
    Requires --advanced flag.
    https://en.wikipedia.org/wiki/Gini_coefficient
14. Atkinson Index: Measures inequality in the distribution with a sensitivity parameter.
    Values range from 0 (perfect equality) to 1 (maximum inequality).
    The Atkinson Index is a more general form of the Gini coefficient that allows for
    different sensitivity to inequality. Sensitivity is configurable via --epsilon.
    Requires --advanced flag.
    https://en.wikipedia.org/wiki/Atkinson_index
15. Shannon Entropy: Measures the information content/uncertainty in the distribution.
    Higher values indicate more diversity, lower values indicate more concentration.
    Values range from 0 (all values identical) to log2(n) where n is the number of unique values.
    Requires --advanced flag.
    https://en.wikipedia.org/wiki/Entropy_(information_theory)
16. Normalized Entropy: Normalized version of Shannon Entropy scaled to [0, 1].
    Values range from 0 (all values identical) to 1 (all values equally distributed).
    Computed as shannon_entropy / log2(cardinality).
    Requires shannon_entropy (from --advanced flag) and cardinality (from base stats).
17. Winsorized Mean: Replaces values below/above thresholds with threshold values, then computes mean.
    All values are included in the calculation, but extreme values are capped at thresholds.
    https://en.wikipedia.org/wiki/Winsorized_mean
    Also computes: winsorized_stddev, winsorized_variance, winsorized_cv, winsorized_range,
    and winsorized_stddev_ratio (winsorized_stddev / overall_stddev).
18. Trimmed Mean: Excludes values outside thresholds, then computes mean.
    Only values within thresholds are included in the calculation.
    https://en.wikipedia.org/wiki/Truncated_mean
    Also computes: trimmed_stddev, trimmed_variance, trimmed_cv, trimmed_range,
    and trimmed_stddev_ratio (trimmed_stddev / overall_stddev).
    By default, uses Q1 and Q3 as thresholds (25% winsorization/trimming).
    With --use-percentiles, uses configurable percentiles (e.g., 5th/95th) as thresholds
    with --pct-thresholds.

In addition, it computes the following univariate outlier statistics (24 outlier statistics total).
https://en.wikipedia.org/wiki/Outlier
(requires --quartiles or --everything in stats):

Outlier Counts (7 statistics):
  - outliers_extreme_lower_cnt: Count of values below the lower outer fence
  - outliers_mild_lower_cnt: Count of values between lower outer and inner fences
  - outliers_normal_cnt: Count of values between inner fences (non-outliers)
  - outliers_mild_upper_cnt: Count of values between upper inner and outer fences
  - outliers_extreme_upper_cnt: Count of values above the upper outer fence
  - outliers_total_cnt: Total count of all outliers (sum of extreme and mild outliers)
  - outliers_percentage: Percentage of values that are outliers

Outlier Descriptive Statistics (6 statistics):
  - outliers_mean: Mean value of outliers
  - non_outliers_mean: Mean value of non-outliers
  - outliers_to_normal_mean_ratio: Ratio of outlier mean to non-outlier mean
  - outliers_min: Minimum value among outliers
  - outliers_max: Maximum value among outliers
  - outliers_range: Range of outlier values (max - min)

Outlier Variance/Spread Statistics (7 statistics):
  - outliers_stddev: Standard deviation of outlier values
  - outliers_variance: Variance of outlier values
  - non_outliers_stddev: Standard deviation of non-outlier values
  - non_outliers_variance: Variance of non-outlier values
  - outliers_cv: Coefficient of variation for outliers (stddev / mean)
  - non_outliers_cv: Coefficient of variation for non-outliers (stddev / mean)
  - outliers_normal_stddev_ratio: Ratio of outlier stddev to non-outlier stddev

Outlier Impact Statistics (2 statistics):
  - outlier_impact: Difference between overall mean and non-outlier mean
  - outlier_impact_ratio: Relative impact (outlier_impact / non_outlier_mean)

Outlier Boundary Statistics (2 statistics):
  - lower_outer_fence_zscore: Z-score of the lower outer fence boundary
  - upper_outer_fence_zscore: Z-score of the upper outer fence boundary

  These outlier statistics require reading the original CSV file and comparing each
  value against the fence thresholds.
  Fences are computed using the IQR method:
    inner fences at Q1/Q3 ± 1.5*IQR, outer fences at Q1/Q3 ± 3.0*IQR.

These univariate statistics are only computed for numeric and date/datetime columns
where the required base univariate statistics (mean, median, stddev, etc.) are available.
Univariate outlier statistics additionally require that quartiles (and thus fences) were
computed when generating the stats CSV.
Winsorized/trimmed means require either Q1/Q3 or percentiles to be available.
Kurtosis, Gini & Atkinson Index require reading the original CSV file to collect
all values for computation.

BIVARIATE STATISTICS:

The `moarstats` command also computes the following 6 bivariate statistics:
 1. Pearson's correlation
    Measures linear correlation between two numeric/date fields.
    Values range from -1 (perfect negative correlation) to +1 (perfect positive correlation).
    0 indicates no linear correlation.
    https://en.wikipedia.org/wiki/Pearson_correlation_coefficient
 2. Spearman's rank correlation
    Measures monotonic correlation between two numeric/date fields.
    Values range from -1 (perfect negative correlation) to +1 (perfect positive correlation).
    0 indicates no monotonic correlation.
    https://en.wikipedia.org/wiki/Spearman%27s_rank_correlation_coefficient
 3. Kendall's tau
    Measures monotonic correlation between two numeric/date fields.
    Values range from -1 (perfect negative correlation) to +1 (perfect positive correlation).
    0 indicates no monotonic correlation.
    https://en.wikipedia.org/wiki/Kendall_rank_correlation_coefficient
 4. Covariance
    Measures the linear relationship between two numeric/date fields.
    Values range from negative infinity to positive infinity.
    0 indicates no linear relationship.
    https://en.wikipedia.org/wiki/Covariance
 5. Mutual Information
    Measures the amount of information obtained about one field by observing another.
    Values range from 0 (independent) to positive infinity.
    https://en.wikipedia.org/wiki/Mutual_information
 6. Normalized Mutual Information
    Normalized version of mutual information, scaled by the geometric mean of individual entropies.
    Values range from 0 (independent) to 1 (perfectly dependent).
    https://en.wikipedia.org/wiki/Mutual_information#Normalized_variants

These bivariate statistics are computed when the `--bivariate` flag is used
and require an indexed CSV file (index will be auto-created if missing).
Bivariate statistics are output to a separate file: `<FILESTEM>.stats.bivariate.csv`.

Bivariate statistics require reading the entire CSV file and are computationally VERY expensive.
For large files (>= 10k records), parallel chunked processing is used when an index is available.
For smaller files or when no index exists, sequential processing is used.

MULTI-DATASET BIVARIATE STATISTICS:

When using the `--join-inputs` flag, multiple datasets can be joined internally before
computing bivariate statistics. This allows analyzing bivariate statistics across datasets
that share common join keys. The joined dataset is saved as a temporary file that is
automatically deleted after computing the bivariate statistics.
The bivariate statistics are saved to `<FILESTEM>.stats.bivariate.joined.csv`.

Examples:

  # Add moar stats to existing stats file
  $ qsv moarstats data.csv

  # Generate baseline stats first with custom options, then add moar stats
  $ qsv moarstats data.csv --stats-options "--everything --infer-dates"

  # Output to different file
  $ qsv moarstats data.csv --output enhanced_stats.csv

  # Compute bivariate statistics between fields
  $ qsv moarstats data.csv --bivariate

  # Join multiple datasets and compute bivariate statistics
  $ qsv moarstats data.csv -B --join-inputs customers.csv,products.csv --join-keys cust_id,prod_id

  # Join multiple datasets and compute bivariate statistics with different join type
  $ qsv moarstats data.csv -B -J customers.csv,products.csv -K cust_id,prod_id -T left

Usage:
    qsv moarstats [options] [<input>]
    qsv moarstats --help

moarstats options:
    --advanced             Compute Kurtosis, ShannonEntropy, Bimodality Coefficient,
                           Gini Coefficient and Atkinson Index.
                           These advanced statistics computations require reading the
                           original CSV file to collect all values
                           for computation and are computationally expensive.
                           Further, Entropy computation requires the frequency command
                           to be run with --limit 0 to collect all frequencies.
                           An index will be auto-created for the original CSV file
                           if it doesn't already exist to enable parallel processing.
    -e, --epsilon <n>      The Atkinson Index Inequality Aversion parameter.
                           Epsilon controls the sensitivity of the Atkinson Index to inequality.
                           The higher the epsilon, the more sensitive the index is to inequality.
                           Typical values are 0.5 (standard in economic research),
                           1.0 (natural boundary), or 2.0 (useful for poverty analysis).
                           [default: 1.0]
    --stats-options <arg>  Options to pass to the stats command if baseline stats need
                           to be generated. The options are passed as a single string
                           that will be split by whitespace.
                           [default: --infer-dates --infer-boolean --mad --quartiles --percentiles --force --stats-jsonl]
    --round <n>            Round statistics to <n> decimal places. Rounding follows
                           Midpoint Nearest Even (Bankers Rounding) rule.
                           [default: 4]
    --use-percentiles      Use percentiles instead of Q1/Q3 for winsorization/trimming.
                           Requires percentiles to be computed in the stats CSV.
   --pct-thresholds <arg>  Comma-separated percentile pair (e.g., "10,90") to use
                           for winsorization/trimming when --use-percentiles is set.
                           Both values must be between 0 and 100, and lower < upper.
                           [default: 5,95]
 --xsd-gdate-scan <mode>   Scan mode for Gregorian XSD date type detection.
                           "fast" (default): Use all available percentile values for detection.
                           "comprehensive": Scan min/max values from all records.
                           Fast mode checks all percentiles for pattern consistency.
                           Fast mode uses "??" suffix, comprehensive uses "?" suffix.
                           [default: fast]

                           BIVARIATE STATISTICS OPTIONS:
    -B, --bivariate        Enable bivariate statistics computation.
                           Requires indexed CSV file (index will be auto-created if missing).
                           Computes pairwise correlations, covariances, mutual information, and
                           normalized mutual information between columns. Outputs to
                           <FILESTEM>.stats.bivariate.csv.
    -S, --bivariate-stats <stats>
                           Comma-separated list of bivariate statistics to compute.
                           Options: pearson, spearman, kendall, covariance, mi (mutual information),
                           nmi (normalized mutual information)
                           Use "all" to compute all statistics or "fast" to compute only
                           pearson & covariance, which is much faster as it doesn't require storing
                           all values and uses streaming algorithms.
                           [default: fast]
    -C, --cardinality-threshold <n>
                           Skip mutual information computation for field pairs where either
                           field has cardinality exceeding this threshold. Helps avoid
                           expensive computations for high-cardinality fields.
                           [default: 1000000]
    -J, --join-inputs <files>
                           Additional datasets to join. Comma-separated list of CSV files to join
                           with the primary input.
                           e.g.: --join-inputs customers.csv,products.csv
    -K, --join-keys <keys>
                           Join keys for each dataset. Comma-separated list of join key column names,
                           one per dataset. Must specify same number of keys as datasets (primary + addl).
                           e.g.: --join-keys customer_id,customer_id,product_id
    -T, --join-type <type>
                           Join type when using --join-inputs.
                           Valid values: inner, left, right, full
                           [default: inner]
    -p, --progressbar      Show progress bars when computing bivariate statistics.

Common options:
    --force                Force recomputing stats even if valid precomputed stats
                           cache exists.
    -j, --jobs <arg>       The number of jobs to run in parallel.
                           This works only when the given CSV has an index.
                           Note that a file handle is opened for each job.
                           When not set, the number of jobs is set to the
                           number of CPUs detected.
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of overwriting the stats CSV file.
"#;

use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
    time::Instant,
};

use crossbeam_channel;
use csv::{ReaderBuilder, StringRecord, WriterBuilder};
use foldhash::{HashMap, HashMapExt, HashSet, HashSetExt};
use indexmap::IndexMap;
use indicatif::{HumanCount, ProgressBar, ProgressDrawTarget, ProgressStyle};
use qsv_dateparser::parse_with_preference;
use rayon::prelude::*;
use serde::Deserialize;
use simdutf8::basic::from_utf8;
use stats::{atkinson, gini, kurtosis};
use threadpool::ThreadPool;

use crate::{CliError, CliResult, config::Config, regex_oncelock, util};
#[derive(Debug, Deserialize)]
struct Args {
    arg_input:                  Option<String>,
    flag_stats_options:         String,
    flag_round:                 u32,
    flag_output:                Option<String>,
    flag_use_percentiles:       bool,
    flag_pct_thresholds:        Option<String>,
    flag_xsd_gdate_scan:        Option<String>,
    flag_advanced:              bool,
    flag_epsilon:               f64,
    flag_bivariate:             bool,
    flag_bivariate_stats:       String,
    flag_cardinality_threshold: Option<u64>,
    flag_join_inputs:           Option<String>,
    flag_join_keys:             Option<String>,
    flag_join_type:             Option<String>,
    flag_progressbar:           bool,
    flag_jobs:                  Option<usize>,
    flag_force:                 bool,
}

/// Configuration for which bivariate statistics to compute
#[derive(Clone, Copy, Debug, Default)]
struct BivariateStatsConfig {
    pearson:    bool,
    spearman:   bool,
    kendall:    bool,
    covariance: bool,
    mi:         bool, // mutual information
    nmi:        bool, // normalized mutual information
}

impl BivariateStatsConfig {
    /// Parse the --bivariate-stats flag value
    fn from_flag(flag_value: &str) -> CliResult<Self> {
        let mut config = Self::default();
        let mut invalid_stats = Vec::new();

        let flag_lower = flag_value.to_lowercase();
        for stat in flag_lower.split(',') {
            let stat_trimmed = stat.trim();
            if stat_trimmed.is_empty() {
                continue; // Skip empty entries from trailing commas
            }
            match stat_trimmed {
                "pearson" => config.pearson = true,
                "spearman" => config.spearman = true,
                "kendall" => config.kendall = true,
                "covariance" | "cov" => config.covariance = true,
                "mi" | "mutual_information" | "mutual-information" => config.mi = true,
                "nmi" | "normalized_mutual_information" | "normalized-mutual-information" => {
                    config.nmi = true;
                },
                "all" => return Ok(Self::all()),
                "fast" => {
                    config.pearson = true;
                    config.covariance = true;
                },
                _ => {
                    invalid_stats.push(stat_trimmed.to_string());
                },
            }
        }

        if !invalid_stats.is_empty() {
            return fail_clierror!(
                "Invalid bivariate statistics: {}. Valid options are: pearson, spearman, kendall, \
                 covariance (or cov), mi (or mutual_information or mutual-information), nmi (or \
                 normalized_mutual_information or normalized-mutual-information), fast, all",
                invalid_stats.join(", ")
            );
        }

        // Check if at least one stat was requested
        if !config.pearson
            && !config.spearman
            && !config.kendall
            && !config.covariance
            && !config.mi
            && !config.nmi
        {
            return fail_clierror!(
                "No valid bivariate statistics specified. Valid options are: pearson, spearman, \
                 kendall, covariance (or cov), mi (or mutual_information or mutual-information), \
                 nmi (or normalized_mutual_information or normalized-mutual-information), fast, \
                 all"
            );
        }

        Ok(config)
    }

    /// Enable all bivariate statistics
    const fn all() -> Self {
        Self {
            pearson:    true,
            spearman:   true,
            kendall:    true,
            covariance: true,
            mi:         true,
            nmi:        true,
        }
    }

    /// Check if we need to store all values (required for Spearman/Kendall)
    #[inline]
    const fn needs_all_values(self) -> bool {
        self.spearman || self.kendall
    }

    /// Check if we need frequency counts (required for mutual information and normalized mutual
    /// information)
    #[inline]
    const fn needs_frequency_counts(self) -> bool {
        self.mi || self.nmi
    }
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

/// Get the bivariate CSV file path for a given input CSV path
/// If `is_joined` is true, appends `.joined` to the filename before `.csv`
fn get_bivariate_csv_path(input_path: &Path, is_joined: bool) -> CliResult<PathBuf> {
    let parent = input_path.parent().unwrap_or_else(|| Path::new("."));
    let fstem = input_path
        .file_stem()
        .ok_or_else(|| CliError::Other("Invalid input path: no file name".to_string()))?;

    let bivariate_filename = if is_joined {
        format!("{}.stats.bivariate.joined.csv", fstem.to_string_lossy())
    } else {
        format!("{}.stats.bivariate.csv", fstem.to_string_lossy())
    };
    Ok(parent.join(bivariate_filename))
}

/// Join multiple datasets internally using join
fn join_datasets_internal(
    primary_input: &Path,
    additional_inputs: &[String],
    join_keys: &[String],
    join_type: &str,
) -> CliResult<PathBuf> {
    use tempfile::NamedTempFile;

    if additional_inputs.is_empty() {
        return fail_clierror!("No additional datasets provided for joining");
    }

    if join_keys.len() != additional_inputs.len() + 1 {
        return fail_clierror!(
            "Number of join keys ({}) must match number of datasets ({})",
            join_keys.len(),
            additional_inputs.len() + 1
        );
    }

    // Create temporary file for joined output with .csv extension
    let temp_dir =
        crate::config::TEMP_FILE_DIR.get_or_init(|| tempfile::TempDir::new().unwrap().keep());
    let temp_file = tempfile::Builder::new()
        .suffix(".csv")
        .tempfile_in(temp_dir)?;
    let temp_path = temp_file.path().to_path_buf();
    drop(temp_file); // Close the file so join can write to it

    let temp_path_str = temp_path
        .to_str()
        .ok_or_else(|| CliError::Other("Invalid temp path".to_string()))?
        .to_string();

    let primary_input_str = primary_input
        .to_str()
        .ok_or_else(|| CliError::Other("Invalid input path".to_string()))?
        .to_string();

    // Build join command arguments
    let join_type_flag: Option<&str> = match join_type {
        "left" => Some("--left"),
        "right" => Some("--right"),
        "full" => Some("--full"),
        _ => None, // inner is default
    };

    // Join datasets sequentially (join first additional to primary, then next to result, etc.)
    // This is simpler than handling multiple joins at once
    let mut current_input = primary_input_str;
    let mut current_key = join_keys[0].clone();

    // These are never read, but we need to declare them to avoid compiler errors
    #[allow(clippy::collection_is_never_read)]
    let mut intermediate_temps: Vec<NamedTempFile> = Vec::new();
    #[allow(clippy::collection_is_never_read)]
    let mut intermediate_path_strs: Vec<String> = Vec::new();

    for (i, (additional_input, next_key)) in additional_inputs
        .iter()
        .zip(join_keys[1..].iter())
        .enumerate()
    {
        let mut args: Vec<&str> = Vec::new();

        // Add join type flag if specified
        if let Some(flag) = join_type_flag {
            args.push(flag);
        }

        args.push(&current_key);
        args.push(&current_input);
        args.push(next_key);
        args.push(additional_input);

        let output_path_str = if i == additional_inputs.len() - 1 {
            // Last join - use final temp path
            temp_path_str.clone()
        } else {
            // Intermediate join - create another temp file with .csv extension
            let intermediate_temp = tempfile::Builder::new()
                .suffix(".csv")
                .tempfile_in(temp_dir)?;
            let intermediate_path = intermediate_temp.path().to_path_buf();
            intermediate_temps.push(intermediate_temp); // Keep temp file alive
            let intermediate_path_str = intermediate_path
                .to_str()
                .ok_or_else(|| CliError::Other("Invalid intermediate temp path".to_string()))?
                .to_string();
            intermediate_path_strs.push(intermediate_path_str.clone());
            intermediate_path_str
        };
        args.push("--output");
        args.push(&output_path_str);

        // Construct join command directly since it doesn't fit run_qsv_cmd pattern
        // (join takes two input files, not one)

        let qsv_path = env::current_exe()
            .map_err(|e| CliError::Other(format!("Failed to get current executable path: {e:?}")))?
            .to_string_lossy()
            .to_string();

        let mut cmd = Command::new(&qsv_path);
        cmd.arg("join").args(&args);

        let output = cmd
            .output()
            .map_err(|e| CliError::Other(format!("Error while executing join command: {e:?}")))?;

        if !output.status.success() {
            return fail_clierror!(
                "Command join failed: Output {{ status: {:?}, stdout: {:?}, stderr: {:?} }}",
                output.status,
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
        }

        log::info!("Joining datasets...");

        // Update for next iteration
        current_input = output_path_str;
        current_key.clone_from(next_key);
    }

    Ok(temp_path)
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

/// Compute Bimodality Coefficient: (skewness² + 1) / (kurtosis + 3)
/// BC < 0.555 indicates unimodal, BC >= 0.555 indicates bimodal/multimodal
fn compute_bimodality_coefficient(skewness: Option<f64>, kurtosis: Option<f64>) -> Option<f64> {
    if let (Some(skew_val), Some(kurt_val)) = (skewness, kurtosis) {
        let denominator = kurt_val + 3.0;
        if denominator.abs() > f64::EPSILON {
            Some(skew_val.mul_add(skew_val, 1.0) / denominator)
        } else {
            None
        }
    } else {
        None
    }
}

/// Compute Normalized Entropy: shannon_entropy / log2(cardinality)
/// Values range from 0 (all values identical) to 1 (all values equally distributed)
fn compute_normalized_entropy(
    shannon_entropy: Option<f64>,
    cardinality: Option<u64>,
) -> Option<f64> {
    if let (Some(entropy_val), Some(card_val)) = (shannon_entropy, cardinality) {
        if card_val > 1 {
            #[allow(clippy::cast_precision_loss)]
            let max_entropy = (card_val as f64).log2();
            if max_entropy.abs() > f64::EPSILON {
                Some(entropy_val / max_entropy)
            } else {
                None
            }
        } else {
            // If cardinality is 0 or 1, normalized entropy is 0
            Some(0.0)
        }
    } else {
        None
    }
}

/// Parse a numeric value from a string, handling empty strings and invalid values
#[inline]
fn parse_float_opt(s: &str) -> Option<f64> {
    if s.is_empty() {
        return None;
    }
    fast_float2::parse::<f64, &[u8]>(s.as_bytes()).ok()
}

/// Parse a numeric value from bytes, handling empty bytes and invalid values
#[inline]
fn parse_float_opt_from_bytes(bytes: &[u8]) -> Option<f64> {
    if bytes.is_empty() {
        return None;
    }
    fast_float2::parse::<f64, &[u8]>(bytes).ok()
}

/// Parse a percentile value from the percentiles column string
/// Format: "5: value1|10: value2|..." (separator from QSV_STATS_SEPARATOR env var, default "|")
/// For Date/DateTime types, values are RFC3339 date strings; for numeric types, they're numbers
/// Returns the numeric value (in days since epoch for dates) for the specified percentile label, or
/// None if not found
fn parse_percentile_value(
    percentile_str: &str,
    percentile_label: &str,
    field_type: FieldType,
) -> Option<f64> {
    if percentile_str.is_empty() {
        return None;
    }

    // Get the separator (default "|")
    let separator = std::env::var("QSV_STATS_SEPARATOR").unwrap_or_else(|_| "|".to_string());

    // Split by separator and find matching percentile
    for entry in percentile_str.split(&separator) {
        let entry = entry.trim();
        if let Some(colon_pos) = entry.find(':') {
            let label = entry[..colon_pos].trim();
            let value_str = entry[colon_pos + 1..].trim();

            if label == percentile_label {
                // For Date/DateTime types, parse as date string; for numeric types, parse as float
                return if field_type.is_date_or_datetime() {
                    let prefer_dmy = util::get_envvar_flag("QSV_PREFER_DMY");
                    parse_date_to_days(value_str, prefer_dmy)
                } else {
                    parse_float_opt(value_str)
                };
            }
        }
    }

    None
}

/// Parse all percentile string values from the percentiles column string
/// Format: "5: value1|10: value2|25: value3|..." (separator from QSV_STATS_SEPARATOR env var,
/// default "|") Returns a vector of all percentile value strings (the values after colons)
/// Used for pattern matching all percentile values in fast mode
fn parse_all_percentile_string_values(percentile_str: &str) -> Vec<&str> {
    if percentile_str.is_empty() {
        return Vec::new();
    }

    // Get the separator (default "|")
    let separator = std::env::var("QSV_STATS_SEPARATOR").unwrap_or_else(|_| "|".to_string());

    // Split by separator and extract all values after colons
    percentile_str
        .split(&separator)
        .filter_map(|entry| {
            let entry = entry.trim();
            if let Some(colon_pos) = entry.find(':') {
                let value_str = entry[colon_pos + 1..].trim();
                if !value_str.is_empty() {
                    return Some(value_str);
                }
            }
            None
        })
        .collect()
}

/// Field type enum for efficient comparisons
/// Matches the FieldType enum from stats.rs but kept local for performance
#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, PartialEq)]
enum FieldType {
    TNull,
    TString,
    TFloat,
    TInteger,
    TDate,
    TDateTime,
    TBoolean,
}

impl FieldType {
    /// Convert string representation to FieldType enum
    /// Returns None if the string doesn't match any known type
    fn from_str(s: &str) -> Option<FieldType> {
        match s {
            "NULL" => Some(FieldType::TNull),
            "String" => Some(FieldType::TString),
            "Float" => Some(FieldType::TFloat),
            "Integer" => Some(FieldType::TInteger),
            "Date" => Some(FieldType::TDate),
            "DateTime" => Some(FieldType::TDateTime),
            "Boolean" => Some(FieldType::TBoolean),
            _ => None,
        }
    }

    /// Check if this type is numeric or date/datetime
    #[inline]
    const fn is_numeric_or_date_type(self) -> bool {
        matches!(
            self,
            FieldType::TInteger
                | FieldType::TFloat
                | FieldType::TDate
                | FieldType::TDateTime
                | FieldType::TBoolean
        )
    }

    /// Check if this type is Date or DateTime
    #[inline]
    const fn is_date_or_datetime(self) -> bool {
        matches!(self, FieldType::TDate | FieldType::TDateTime)
    }
}

/// Parse a date/datetime value and convert to days since epoch
/// Returns None if parsing fails or value is empty
#[inline]
fn parse_date_to_days(s: &str, prefer_dmy: bool) -> Option<f64> {
    if s.is_empty() {
        return None;
    }
    #[allow(clippy::cast_precision_loss)]
    parse_with_preference(s, prefer_dmy)
        .ok()
        .map(|dt| dt.timestamp_millis() as f64 / 86_400_000.0)
}

/// Detect Gregorian date types (gYearMonth, gYear, gMonthDay, gDay, gMonth) using
/// optimized pattern matching with cheap checks first, regex only when necessary.
/// Returns Some("typeName?") or Some("typeName??") if detected, None otherwise.
/// Performance optimized: uses numeric comparisons for Integer gYear, cheap string
/// checks (length/prefix) before regex for String types.
/// Fast mode checks all percentile values for consistency, comprehensive mode checks min/max.
fn detect_gregorian_date_type(
    min_str: Option<&str>,
    max_str: Option<&str>,
    field_type_str: &str,
    min_val: Option<f64>,
    max_val: Option<f64>,
    scan_mode: &str,
    percentile_values: Option<&[&str]>,
) -> Option<String> {
    // Determine suffix based on scan mode
    let suffix = if scan_mode == "fast" { "??" } else { "?" };

    // Fast mode: check all percentile values
    if scan_mode == "fast" {
        if let Some(pct_values) = percentile_values {
            if pct_values.is_empty() {
                return None;
            }

            // Fast path for Integer gYear (no regex needed)
            if field_type_str == "Integer" {
                // Parse all percentile values as numbers and check if all are in year range
                // Skip empty strings but require all non-empty values to be in range
                let non_empty_values: Vec<&str> = pct_values
                    .iter()
                    .copied()
                    .filter(|&s| !s.is_empty())
                    .collect();
                if !non_empty_values.is_empty() {
                    let all_in_range = non_empty_values.iter().all(|&val_str| {
                        if let Some(val) = parse_float_opt(val_str) {
                            (1000.0..=3000.0).contains(&val)
                        } else {
                            false
                        }
                    });
                    if all_in_range {
                        return Some(format!("gYear{suffix}"));
                    }
                }
                return None;
            }

            // For String types, check all percentile values against patterns
            let check_value = |s: &str| -> Option<&str> {
                // gYearMonth: "1999-05" (length 7, dash at position 4)
                if s.len() == 7
                    && s.as_bytes().get(4) == Some(&b'-')
                    && regex_oncelock!(r"^\d{4}-\d{2}$").is_match(s)
                {
                    // Validate that the month portion is within 01-12
                    let month_str = &s[5..7];
                    if let Ok(month) = month_str.parse::<u8>() {
                        if (1..=12).contains(&month) {
                            return Some("gYearMonth");
                        }
                    }
                }

                // gYear: "1999" (length 4)
                if s.len() == 4 && regex_oncelock!(r"^\d{4}$").is_match(s) {
                    // Also validate the numeric year is within a reasonable range
                    if let Ok(year) = s.parse::<u32>() {
                        if (1000..=3000).contains(&year) {
                            return Some("gYear");
                        }
                    }
                }

                // gMonthDay: "--05-01" (length 7, starts with "--")
                if s.len() == 7
                    && s.starts_with("--")
                    && regex_oncelock!(r"^--\d{2}-\d{2}$").is_match(s)
                {
                    return Some("gMonthDay");
                }

                // gDay: "---01" (length 5, starts with "---")
                if s.len() == 5
                    && s.starts_with("---")
                    && regex_oncelock!(r"^---\d{2}$").is_match(s)
                {
                    return Some("gDay");
                }

                // gMonth: "--05" (length 4, starts with "--")
                if s.len() == 4 && s.starts_with("--") && regex_oncelock!(r"^--\d{2}$").is_match(s)
                {
                    return Some("gMonth");
                }

                None
            };

            // Check all percentile values - only return type if ALL match the same pattern
            let mut matched_type: Option<&str> = None;
            for &val_str in pct_values {
                if val_str.is_empty() {
                    continue; // Skip empty values
                }
                if let Some(pattern_type) = check_value(val_str) {
                    match matched_type {
                        None => matched_type = Some(pattern_type),
                        Some(existing_type) if existing_type == pattern_type => {
                            // Same pattern, continue
                        },
                        _ => {
                            // Different pattern or no match, not consistent
                            return None;
                        },
                    }
                } else {
                    // Value doesn't match any pattern
                    return None;
                }
            }

            // All values matched the same pattern
            if let Some(base_type) = matched_type {
                return Some(format!("{base_type}{suffix}"));
            }
        }
        return None;
    }

    // Comprehensive mode: check min/max values
    // Fast path for Integer gYear (no regex needed)
    if field_type_str == "Integer" {
        if let (Some(min), Some(max)) = (min_val, max_val) {
            // Check if values are in reasonable year range (1000-3000)
            if min >= 1000.0 && max <= 3000.0 {
                return Some(format!("gYear{suffix}"));
            }
        }
        // Not a year range, return None to continue with normal Integer inference
        return None;
    }

    // For String types, check both min and max to increase confidence
    let check_value = |s: &str| -> Option<&str> {
        // gYearMonth: "1999-05" (length 7, dash at position 4)
        if s.len() == 7
            && s.as_bytes().get(4) == Some(&b'-')
            && regex_oncelock!(r"^\d{4}-(0[1-9]|1[0-2])$").is_match(s)
        {
            return Some("gYearMonth");
        }

        // gYear: "1999" (length 4)
        if s.len() == 4 && regex_oncelock!(r"^\d{4}$").is_match(s) {
            return Some("gYear");
        }

        // gMonthDay: "--05-01" (length 7, starts with "--")
        if s.len() == 7 && s.starts_with("--") && regex_oncelock!(r"^--\d{2}-\d{2}$").is_match(s) {
            // validate numeric ranges: month 1-12, day 1-31
            if let (Ok(month), Ok(day)) = (s[2..4].parse::<u32>(), s[5..7].parse::<u32>()) {
                if (1..=12).contains(&month) && (1..=31).contains(&day) {
                    return Some("gMonthDay");
                }
            }
        }

        // gDay: "---01" (length 5, starts with "---")
        if s.len() == 5 && s.starts_with("---") && regex_oncelock!(r"^---\d{2}$").is_match(s) {
            // validate numeric range: day 1-31
            if let Ok(day) = s[3..5].parse::<u32>() {
                if (1..=31).contains(&day) {
                    return Some("gDay");
                }
            }
        }

        // gMonth: "--05" (length 4, starts with "--")
        if s.len() == 4 && s.starts_with("--") && regex_oncelock!(r"^--\d{2}$").is_match(s) {
            // validate numeric range: month 1-12
            if let Ok(month) = s[2..4].parse::<u32>() {
                if (1..=12).contains(&month) {
                    return Some("gMonth");
                }
            }
        }

        None
    };

    // Check min_str first
    if let Some(min_s) = min_str
        && !min_s.is_empty()
        && let Some(greg_type) = check_value(min_s)
    {
        // If max_str is available, verify it matches the same pattern for confidence
        if let Some(max_s) = max_str {
            if !max_s.is_empty()
                && let Some(max_type) = check_value(max_s)
            {
                // Both match the same type, return it
                if greg_type == max_type {
                    return Some(format!("{greg_type}{suffix}"));
                }
                // Different patterns, not confident - return None
                return None;
            }
            // Only min_str matched, still return it (with ? indicating uncertainty)
            return Some(format!("{greg_type}{suffix}"));
        }
    }

    // Check max_str if min_str didn't match
    if let Some(max_s) = max_str
        && !max_s.is_empty()
        && let Some(greg_type) = check_value(max_s)
    {
        return Some(format!("{greg_type}{suffix}"));
    }

    None
}

/// Infer the most specific W3C XML Schema datatype based on field type and min/max values
/// Returns the XSD type string (e.g., "byte", "int", "decimal", "string", "date", etc.)
/// Based on the analysis at https://github.com/user-attachments/files/23841656/xsd_analysis.md
fn infer_xsd_type(
    field_type_str: &str,
    min_val: Option<f64>,
    max_val: Option<f64>,
    field_type_enum: Option<FieldType>,
    scan_mode: &str,
    min_str: Option<&str>,
    max_str: Option<&str>,
    percentile_values: Option<&[&str]>,
) -> String {
    // Handle NULL type
    if field_type_str == "NULL" || field_type_str.is_empty() {
        return String::new();
    }

    // Handle Boolean type
    if field_type_str == "Boolean" {
        return "boolean".to_string();
    }

    // Check for Gregorian date types early (after NULL/Boolean, before other type checks)
    // This allows detection for both Integer and String fields
    if let Some(greg_type) = detect_gregorian_date_type(
        min_str,
        max_str,
        field_type_str,
        min_val,
        max_val,
        scan_mode,
        percentile_values,
    ) {
        return greg_type;
    }

    // Handle Date and DateTime types
    if field_type_enum == Some(FieldType::TDate) {
        return "date".to_string();
    }
    if field_type_enum == Some(FieldType::TDateTime) {
        return "dateTime".to_string();
    }

    // Handle String type
    if field_type_str == "String" {
        return "string".to_string();
    }

    // Handle Float type
    if field_type_str == "Float" {
        return "decimal".to_string();
    }

    // Handle Integer type with range-based refinement
    if field_type_str == "Integer" {
        let (Some(min), Some(max)) = (min_val, max_val) else {
            // If min/max not available, default to integer
            return "integer".to_string();
        };

        // Check for unsigned integer types first (most specific first)
        // Only check unsigned types if min >= 0
        if min >= 0.0 {
            if max <= 255.0 {
                return "unsignedByte".to_string();
            }
            if max <= 65_535.0 {
                return "unsignedShort".to_string();
            }
            if max <= 4_294_967_295.0 {
                return "unsignedInt".to_string();
            }
            // unsignedLong: 0 to 2^64-1 (18446744073709551615)
            // Check if max fits in u64 range
            if max <= 18_446_744_073_709_551_615.0 {
                return "unsignedLong".to_string();
            }
            // Check for special unsigned constraints (unbounded)
            if min > 0.0 {
                return "positiveInteger".to_string();
            }
            // min >= 0.0 (already checked above)
            return "nonNegativeInteger".to_string();
        }

        // Check for signed integer types (most specific first)
        // Only check signed types if min < 0 (or if we have negative values)
        // Use f64 comparisons to avoid clamping issues
        if min >= -128.0 && max <= 127.0 {
            return "byte".to_string();
        }
        if min >= -32_768.0 && max <= 32_767.0 {
            return "short".to_string();
        }
        if min >= -2_147_483_648.0 && max <= 2_147_483_647.0 {
            return "int".to_string();
        }
        if min >= -9_223_372_036_854_775_808.0 && max <= 9_223_372_036_854_775_807.0 {
            return "long".to_string();
        }

        // Check for special signed integer constraints
        if max < 0.0 {
            return "negativeInteger".to_string();
        }
        if max <= 0.0 {
            return "nonPositiveInteger".to_string();
        }

        // Default to unbounded integer
        return "integer".to_string();
    }

    // Fallback: return empty string for unrecognized types
    String::new()
}

/// Convert days since epoch to RFC3339 formatted date string
/// For Date types, returns only the date component (YYYY-MM-DD)
/// For DateTime types, returns full RFC3339 format with time and timezone
fn days_to_rfc3339(days: f64, field_type: FieldType) -> String {
    // Convert days to milliseconds
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let timestamp_ms = (days * 86_400_000.0) as i64;

    let date_val = chrono::DateTime::from_timestamp_millis(timestamp_ms)
        .unwrap_or_default()
        .to_rfc3339();

    // if type = Date, only return the date component
    if field_type == FieldType::TDate {
        return date_val[..10].to_string();
    }
    date_val
}

/// Field information needed for outlier counting and winsorized/trimmed means
#[derive(Clone)]
struct OutlierFieldInfo {
    col_idx:         usize,
    field_type:      FieldType, // Use enum for faster comparisons
    lower_outer:     f64,
    lower_inner:     f64,
    upper_inner:     f64,
    upper_outer:     f64,
    lower_threshold: f64, // For winsorization/trimming (Q1 or percentile)
    upper_threshold: f64, // For winsorization/trimming (Q3 or percentile)
}

/// Statistics tracked during outlier scanning
#[derive(Clone, Default)]
struct OutlierStats {
    // Counts: [extreme_lower, mild_lower, normal, mild_upper, extreme_upper, total]
    counts:                 [u64; 6],
    // Sums
    sum_outliers:           f64,
    sum_normal:             f64,
    sum_all:                f64,
    // Min/Max
    min_outliers:           Option<f64>,
    max_outliers:           Option<f64>,
    min_normal:             Option<f64>,
    max_normal:             Option<f64>,
    // Winsorized and trimmed means
    winsorized_sum:         f64,
    winsorized_count:       u64,
    trimmed_sum:            f64,
    trimmed_count:          u64,
    // For variance/stddev computation (using sum of squares)
    sum_squares_outliers:   f64,
    sum_squares_normal:     f64,
    sum_squares_trimmed:    f64,
    sum_squares_winsorized: f64,
    // For trimmed/winsorized range
    min_trimmed:            Option<f64>,
    max_trimmed:            Option<f64>,
    min_winsorized:         Option<f64>,
    max_winsorized:         Option<f64>,
    // Total count of all values processed
    count_all:              u64,
}

/// Statistics for Kurtosis, Gini & Atkinson Index
#[derive(Clone, Default)]
struct KGAStats {
    kurtosis:         Option<f64>,
    gini_coefficient: Option<f64>,
    atkinson_index:   Option<f64>,
}

/// Statistics for Shannon Entropy
#[derive(Clone, Default)]
struct EntropyStats {
    entropy: Option<f64>,
}

/// Online algorithm state for correlation/covariance computation
/// Uses Welford's online algorithm for aggregating across chunks
#[derive(Clone, Default)]
struct CorrelationState {
    count:  u64,
    mean_x: f64,
    mean_y: f64,
    m2_x:   f64, // sum of squared differences for x
    m2_y:   f64, // sum of squared differences for y
    cxy:    f64, // sum of (x - mean_x) * (y - mean_y)
}

/// Statistics tracked during bivariate computation for a field pair
#[derive(Clone, Default)]
struct BivariateChunkStats {
    correlation_state: CorrelationState,
    x_values:          Vec<f64>, // For Spearman/Kendall (need ranks)
    y_values:          Vec<f64>, // For Spearman/Kendall (need ranks)
    // Frequency counts for mutual information (more memory efficient than storing all strings)
    xy_counts:         HashMap<(String, String), u64>, // Joint frequencies
    x_counts:          HashMap<String, u64>,           // Marginal frequencies for x
    y_counts:          HashMap<String, u64>,           // Marginal frequencies for y
    total_pairs:       u64,                            // Total count of pairs
}

/// Final bivariate statistics for a field pair
#[derive(Clone, Default)]
struct BivariateStats {
    pearson: Option<f64>,
    spearman: Option<f64>,
    kendall: Option<f64>,
    covariance_sample: Option<f64>,
    covariance_population: Option<f64>,
    mutual_information: Option<f64>,
    normalized_mutual_information: Option<f64>,
    n_pairs: u64,
}

/// Field information for bivariate computation
#[derive(Clone)]
struct BivariateFieldInfo {
    col_idx:     usize,
    field_type:  FieldType,
    // Pre-computed statistics from stats CSV (used for optimizations)
    stddev:      Option<f64>, // Pre-computed standard deviation (used for filtering)
    variance:    Option<f64>, // Pre-computed variance (used for filtering)
    cardinality: Option<u64>, // Pre-computed cardinality (used for threshold filtering)
}

/// Update correlation state with a new pair of values using Welford's online algorithm
#[allow(clippy::cast_precision_loss)]
fn update_correlation_state(state: &mut CorrelationState, x: f64, y: f64) {
    state.count += 1;
    let n = state.count as f64;

    let delta_x = x - state.mean_x;
    let delta_y = y - state.mean_y;

    // Update means
    state.mean_x += delta_x / n;
    state.mean_y += delta_y / n;

    // Update sum of squared differences and covariance term
    let delta_x_new = x - state.mean_x;
    let delta_y_new = y - state.mean_y;

    state.m2_x += delta_x * delta_x_new;
    state.m2_y += delta_y * delta_y_new;
    state.cxy += delta_x * delta_y_new;
}

/// Merge two correlation states (for aggregating across chunks)
#[allow(clippy::cast_precision_loss)]
fn merge_correlation_states(
    state1: &CorrelationState,
    state2: &CorrelationState,
) -> CorrelationState {
    if state1.count == 0 {
        return state2.clone();
    }
    if state2.count == 0 {
        return state1.clone();
    }

    let n1 = state1.count as f64;
    let n2 = state2.count as f64;
    let n_total = n1 + n2;

    // NOTE: we use fused multiply-add extensively below
    // for more efficient, performant, accurate computations.
    // the original formula is in a comment above each FMA implementation.

    // Combined mean
    // let mean_x_combined = (state1.mean_x * n1 + state2.mean_x * n2) / n_total;
    let mean_x_combined = state1.mean_x.mul_add(n1, state2.mean_x * n2) / n_total;
    // let mean_y_combined = (state1.mean_y * n1 + state2.mean_y * n2) / n_total;
    let mean_y_combined = state1.mean_y.mul_add(n1, state2.mean_y * n2) / n_total;

    // Combined variance terms (using parallel algorithm formula)
    let delta_x1 = state1.mean_x - mean_x_combined;
    let delta_x2 = state2.mean_x - mean_x_combined;
    let delta_y1 = state1.mean_y - mean_y_combined;
    let delta_y2 = state2.mean_y - mean_y_combined;

    let m2_x_combined =
        // state1.m2_x + state2.m2_x + delta_x1 * delta_x1 * n1 + delta_x2 * delta_x2 * n2;
        (delta_x2 * delta_x2).mul_add(n2, (delta_x1 * delta_x1).mul_add(n1, state1.m2_x + state2.m2_x));
    let m2_y_combined =
        // state1.m2_y + state2.m2_y + delta_y1 * delta_y1 * n1 + delta_y2 * delta_y2 * n2;
        (delta_y2 * delta_y2).mul_add(n2, (delta_y1 * delta_y1).mul_add(n1, state1.m2_y + state2.m2_y));

    // Combined covariance term
    let cxy_combined =
        // state1.cxy + state2.cxy + delta_x1 * delta_y1 * n1 + delta_x2 * delta_y2 * n2;
        (delta_x2 * delta_y2).mul_add(n2, (delta_x1 * delta_y1).mul_add(n1, state1.cxy + state2.cxy));

    CorrelationState {
        count:  state1.count + state2.count,
        mean_x: mean_x_combined,
        mean_y: mean_y_combined,
        m2_x:   m2_x_combined,
        m2_y:   m2_y_combined,
        cxy:    cxy_combined,
    }
}

/// Compute final Pearson correlation coefficient from correlation state
#[allow(clippy::cast_precision_loss)]
fn finalize_pearson_correlation(state: &CorrelationState) -> Option<f64> {
    if state.count < 2 {
        return None;
    }

    let variance_x = state.m2_x / (state.count as f64 - 1.0);
    let variance_y = state.m2_y / (state.count as f64 - 1.0);

    if variance_x <= 0.0 || variance_y <= 0.0 {
        return None;
    }

    let covariance = state.cxy / (state.count as f64 - 1.0);
    let stddev_x = variance_x.sqrt();
    let stddev_y = variance_y.sqrt();

    if stddev_x.abs() > f64::EPSILON && stddev_y.abs() > f64::EPSILON {
        Some(covariance / (stddev_x * stddev_y))
    } else {
        None
    }
}

/// Compute final covariance from correlation state
#[allow(clippy::cast_precision_loss)]
fn finalize_covariance(state: &CorrelationState, sample: bool) -> Option<f64> {
    if state.count < 2 {
        return None;
    }

    let divisor = if sample {
        state.count as f64 - 1.0
    } else {
        state.count as f64
    };

    Some(state.cxy / divisor)
}

/// Compute Pearson correlation coefficient from two arrays of values
fn compute_pearson_correlation(x: &[f64], y: &[f64]) -> Option<f64> {
    if x.len() != y.len() || x.len() < 2 {
        return None;
    }

    let mut state = CorrelationState::default();
    for (xi, yi) in x.iter().zip(y.iter()) {
        update_correlation_state(&mut state, *xi, *yi);
    }

    finalize_pearson_correlation(&state)
}

/// Compute Spearman's rank correlation coefficient
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::many_single_char_names)]
fn compute_spearman_correlation(x: &[f64], y: &[f64]) -> Option<f64> {
    if x.len() != y.len() || x.len() < 2 {
        return None;
    }

    let n = x.len();

    // Pre-allocate with capacity to avoid reallocations
    let mut x_ranked: Vec<(usize, f64)> = Vec::with_capacity(n);
    x_ranked.extend(x.iter().enumerate().map(|(i, &v)| (i, v)));

    let mut y_ranked: Vec<(usize, f64)> = Vec::with_capacity(n);
    y_ranked.extend(y.iter().enumerate().map(|(i, &v)| (i, v)));

    // Use total_cmp for faster, more predictable sorting (handles NaNs consistently)
    // This is faster than partial_cmp and gives consistent ordering
    x_ranked.sort_unstable_by(|a, b| a.1.total_cmp(&b.1));
    y_ranked.sort_unstable_by(|a, b| a.1.total_cmp(&b.1));

    // Pre-allocate rank vectors
    let mut x_ranks = vec![0.0; n];
    let mut y_ranks = vec![0.0; n];

    // Rank x values (handle ties by averaging) - optimized loop
    let mut i = 0;
    while i < n {
        let mut j = i;
        let val = x_ranked[i].1;
        // Use total_cmp for tie detection - faster than abs diff
        while j < n && x_ranked[j].1.total_cmp(&val) == std::cmp::Ordering::Equal {
            j += 1;
        }
        let rank = (i + j - 1) as f64 / 2.0 + 1.0;
        // Use slice assignment for better cache locality
        for k in i..j {
            x_ranks[x_ranked[k].0] = rank;
        }
        i = j;
    }

    // Rank y values - use total_cmp for faster comparison
    i = 0;
    while i < n {
        let mut j = i;
        let val = y_ranked[i].1;
        while j < n && y_ranked[j].1.total_cmp(&val) == std::cmp::Ordering::Equal {
            j += 1;
        }
        let rank = (i + j - 1) as f64 / 2.0 + 1.0;
        for k in i..j {
            y_ranks[y_ranked[k].0] = rank;
        }
        i = j;
    }

    // Compute Pearson correlation on ranks
    compute_pearson_correlation(&x_ranks, &y_ranks)
}

/// Count inversions in y values when sorted by x using merge sort (O(n log n))
/// Returns number of inversions (discordant pairs)
#[allow(clippy::cast_precision_loss)]
fn count_inversions_merge(
    pairs: &mut [(f64, f64)],
    temp: &mut [(f64, f64)],
    left: usize,
    right: usize,
) -> i64 {
    if left >= right {
        return 0;
    }

    let mid = left + (right - left) / 2;
    let mut inversions = count_inversions_merge(pairs, temp, left, mid)
        + count_inversions_merge(pairs, temp, mid + 1, right);

    // Merge and count inversions - use total_cmp for faster comparison
    let mut i = left;
    let mut j = mid + 1;
    let mut k = left;

    while i <= mid && j <= right {
        // Use total_cmp instead of <= for faster comparison
        if pairs[i].1.total_cmp(&pairs[j].1) == std::cmp::Ordering::Greater {
            // Inversion found: pairs[i].1 > pairs[j].1
            // All remaining elements in left half form inversions with pairs[j]
            inversions += (mid - i + 1) as i64;
            temp[k] = pairs[j];
            j += 1;
        } else {
            // No inversion: pairs[i].1 <= pairs[j].1
            // Copy pairs[i] to temp and move to next element in left half
            temp[k] = pairs[i];
            i += 1;
        }
        k += 1; // Move to next position in temp array
    }

    // Copy remaining elements - use copy_from_slice for better performance
    if i <= mid {
        let remaining = mid - i + 1;
        temp[k..k + remaining].copy_from_slice(&pairs[i..i + remaining]);
    }
    if j <= right {
        let remaining = right - j + 1;
        temp[k..k + remaining].copy_from_slice(&pairs[j..j + remaining]);
    }

    // Copy back from temp
    pairs[left..=right].copy_from_slice(&temp[left..=right]);

    inversions
}

/// Compute Kendall's tau rank correlation coefficient using O(n log n) merge sort
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::many_single_char_names)]
fn compute_kendall_tau(x: &[f64], y: &[f64]) -> Option<f64> {
    if x.len() != y.len() || x.len() < 2 {
        return None;
    }

    let n = x.len() as f64;
    let pairs_len = x.len();

    // Pre-allocate indices vector
    let mut y_indices: Vec<usize> = Vec::with_capacity(pairs_len);
    y_indices.extend(0..pairs_len);

    // Use total_cmp for faster, more predictable sorting
    y_indices.sort_unstable_by(|&a, &b| y[a].total_cmp(&y[b]).then_with(|| x[a].total_cmp(&x[b])));

    // Count ties in y
    let mut ties_y = 0i64;
    let mut i = 0;
    while i < pairs_len {
        let mut j = i + 1;
        let val = y[y_indices[i]];
        // Use total_cmp instead of abs diff for tie detection
        while j < pairs_len && y[y_indices[j]].total_cmp(&val) == std::cmp::Ordering::Equal {
            j += 1;
        }
        let tie_count = (j - i) as i64;
        if tie_count > 1 {
            ties_y += tie_count * (tie_count - 1) / 2;
        }
        i = j;
    }

    // Pre-allocate pairs vector with capacity
    let mut pairs: Vec<(f64, f64)> = Vec::with_capacity(pairs_len);
    pairs.extend(x.iter().zip(y.iter()).map(|(&a, &b)| (a, b)));

    // Use total_cmp for faster sorting
    pairs.sort_unstable_by(|a, b| a.0.total_cmp(&b.0).then_with(|| a.1.total_cmp(&b.1)));

    // Count ties in x
    let mut ties_x = 0i64;
    i = 0;
    while i < pairs_len {
        let mut j = i + 1;
        let val = pairs[i].0;
        while j < pairs_len && pairs[j].0.total_cmp(&val) == std::cmp::Ordering::Equal {
            j += 1;
        }
        let tie_count = (j - i) as i64;
        if tie_count > 1 {
            ties_x += tie_count * (tie_count - 1) / 2;
        }
        i = j;
    }

    // Pre-allocate temp buffer once
    let mut temp = vec![(0.0, 0.0); pairs_len];
    let inversions = count_inversions_merge(&mut pairs, &mut temp, 0, pairs_len - 1);

    // Calculate concordant and discordant pairs
    let total_pairs = (n * (n - 1.0) / 2.0) as i64;
    let discordant = inversions;
    let concordant = total_pairs - discordant - ties_x - ties_y;

    let n0 = n * (n - 1.0) / 2.0;
    let n1 = ties_x as f64;
    let n2 = ties_y as f64;

    let denominator = ((n0 - n1) * (n0 - n2)).sqrt();

    if denominator.abs() < f64::EPSILON {
        return None;
    }

    let tau = ((concordant - discordant) as f64) / denominator;
    Some(tau)
}

/// Compute mutual information between two categorical/numeric fields from frequency counts
#[allow(clippy::cast_precision_loss)]
fn compute_mutual_information_from_counts(
    xy_counts: &HashMap<(String, String), u64>,
    x_counts: &HashMap<String, u64>,
    y_counts: &HashMap<String, u64>,
    total: u64,
) -> Option<f64> {
    if total == 0 {
        return None;
    }

    let total_f64 = total as f64;

    // Compute mutual information: MI(X,Y) = sum(p(x,y) * log2(p(x,y) / (p(x) * p(y))))
    let mut mi = 0.0;
    for ((x_val, y_val), &xy_count) in xy_counts {
        let p_xy = xy_count as f64 / total_f64;
        let p_x = x_counts.get(x_val).copied().unwrap_or(0) as f64 / total_f64;
        let p_y = y_counts.get(y_val).copied().unwrap_or(0) as f64 / total_f64;

        if p_x > 0.0 && p_y > 0.0 && p_xy > 0.0 {
            mi += p_xy * (p_xy / (p_x * p_y)).log2();
        }
    }

    Some(mi)
}

/// Compute Shannon entropy from frequency counts
/// Uses the same formula as compute_all_entropy(): H(X) = -Σ p_i * log2(p_i)
/// where p_i = count_i / total
#[allow(clippy::cast_precision_loss)]
fn compute_entropy_from_counts(counts: &HashMap<String, u64>, total: u64) -> Option<f64> {
    if total == 0 {
        return None;
    }

    let total_f64 = total as f64;
    let mut entropy = 0.0;

    for count in counts.values() {
        if *count > 0 {
            let p = *count as f64 / total_f64;
            entropy -= p * p.log2();
        }
    }

    Some(entropy)
}

/// Compute normalized mutual information from mutual information and entropies
/// NMI = MI / sqrt(H(X) * H(Y))
/// Returns None if either entropy is invalid (0, negative, or None) or if the denominator is 0
fn compute_normalized_mutual_information(
    mi: Option<f64>,
    h_x: Option<f64>,
    h_y: Option<f64>,
) -> Option<f64> {
    let (Some(mi_val), Some(h_x_val), Some(h_y_val)) = (mi, h_x, h_y) else {
        return None;
    };

    // Check for invalid entropy values (non-positive)
    if h_x_val <= 0.0 || h_y_val <= 0.0 {
        return None;
    }

    // Compute denominator: sqrt(H(X) * H(Y))
    let denominator = (h_x_val * h_y_val).sqrt();
    if denominator == 0.0 {
        return None;
    }

    // NMI = MI / sqrt(H(X) * H(Y))
    Some(mi_val / denominator)
}

/// Field information needed for Kurtosis, Gini & Atkinson Index computation (with precalculated
/// stats)
#[derive(Clone)]
struct KGAFieldInfo {
    col_idx:    usize,
    field_type: FieldType,
    mean:       Option<f64>,
    variance:   Option<f64>, // variance = stddev^2
    sum:        Option<f64>, // sum for Gini coefficient
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

    let prefer_dmy = util::get_envvar_flag("QSV_PREFER_DMY");
    #[allow(unused_assignments)]
    let mut record: csv::ByteRecord = csv::ByteRecord::new();
    let mut value_bytes;
    let mut numeric_value;

    // Process each record in the chunk
    for result in records {
        record = result?;

        for (field_name, field_info) in fields_to_count {
            value_bytes = record.get(field_info.col_idx).unwrap_or(&[]);

            if value_bytes.is_empty() {
                continue; // Skip null/empty values
            }

            // Parse the value based on field type
            numeric_value = if field_info.field_type.is_date_or_datetime() {
                // Convert bytes to string for date parsing
                if let Ok(value_str) = from_utf8(value_bytes) {
                    parse_date_to_days(value_str, prefer_dmy)
                } else {
                    None
                }
            } else {
                parse_float_opt_from_bytes(value_bytes)
            };

            let Some(val) = numeric_value else {
                continue; // Skip values that can't be parsed
            };

            // Get mutable reference to stats for this field
            let stats = chunk_stats.get_mut(field_name).unwrap();

            // Update sums and count
            stats.sum_all += val;
            stats.count_all += 1;

            // Compute winsorized and trimmed statistics
            let winsorized_val = val
                .max(field_info.lower_threshold)
                .min(field_info.upper_threshold);
            stats.winsorized_sum += winsorized_val;
            stats.winsorized_count += 1;
            // Track winsorized min/max and sum of squares
            stats.min_winsorized = Some(
                stats
                    .min_winsorized
                    .map_or(winsorized_val, |m| m.min(winsorized_val)),
            );
            stats.max_winsorized = Some(
                stats
                    .max_winsorized
                    .map_or(winsorized_val, |m| m.max(winsorized_val)),
            );
            stats.sum_squares_winsorized += winsorized_val * winsorized_val;

            // For trimmed mean, only include values within thresholds
            if val >= field_info.lower_threshold && val <= field_info.upper_threshold {
                stats.trimmed_sum += val;
                stats.trimmed_count += 1;
                // Track trimmed min/max and sum of squares
                stats.min_trimmed = Some(stats.min_trimmed.map_or(val, |m| m.min(val)));
                stats.max_trimmed = Some(stats.max_trimmed.map_or(val, |m| m.max(val)));
                stats.sum_squares_trimmed += val * val;
            }

            // Count outliers and track statistics based on fence comparisons
            if val < field_info.lower_outer {
                stats.counts[0] += 1; // extreme_lower
                stats.counts[5] += 1; // total
                stats.sum_outliers += val;
                stats.sum_squares_outliers += val * val;
                stats.min_outliers = Some(stats.min_outliers.map_or(val, |m| m.min(val)));
                stats.max_outliers = Some(stats.max_outliers.map_or(val, |m| m.max(val)));
            } else if val < field_info.lower_inner {
                stats.counts[1] += 1; // mild_lower
                stats.counts[5] += 1; // total
                stats.sum_outliers += val;
                stats.sum_squares_outliers += val * val;
                stats.min_outliers = Some(stats.min_outliers.map_or(val, |m| m.min(val)));
                stats.max_outliers = Some(stats.max_outliers.map_or(val, |m| m.max(val)));
            } else if val <= field_info.upper_inner {
                stats.counts[2] += 1; // normal
                stats.sum_normal += val;
                stats.sum_squares_normal += val * val;
                stats.min_normal = Some(stats.min_normal.map_or(val, |m| m.min(val)));
                stats.max_normal = Some(stats.max_normal.map_or(val, |m| m.max(val)));
            } else if val <= field_info.upper_outer {
                stats.counts[3] += 1; // mild_upper
                stats.counts[5] += 1; // total
                stats.sum_outliers += val;
                stats.sum_squares_outliers += val * val;
                stats.min_outliers = Some(stats.min_outliers.map_or(val, |m| m.min(val)));
                stats.max_outliers = Some(stats.max_outliers.map_or(val, |m| m.max(val)));
            } else {
                stats.counts[4] += 1; // extreme_upper
                stats.counts[5] += 1; // total
                stats.sum_outliers += val;
                stats.sum_squares_outliers += val * val;
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
    flag_jobs: Option<usize>,
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

        let njobs = util::njobs(flag_jobs);
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
                    total_stats.count_all += stats.count_all;
                    // Aggregate winsorized/trimmed stats
                    total_stats.winsorized_sum += stats.winsorized_sum;
                    total_stats.winsorized_count += stats.winsorized_count;
                    total_stats.trimmed_sum += stats.trimmed_sum;
                    total_stats.trimmed_count += stats.trimmed_count;
                    // Aggregate sum of squares
                    total_stats.sum_squares_outliers += stats.sum_squares_outliers;
                    total_stats.sum_squares_normal += stats.sum_squares_normal;
                    total_stats.sum_squares_trimmed += stats.sum_squares_trimmed;
                    total_stats.sum_squares_winsorized += stats.sum_squares_winsorized;
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
                    if let Some(min) = stats.min_trimmed {
                        total_stats.min_trimmed =
                            Some(total_stats.min_trimmed.map_or(min, |m| m.min(min)));
                    }
                    if let Some(max) = stats.max_trimmed {
                        total_stats.max_trimmed =
                            Some(total_stats.max_trimmed.map_or(max, |m| m.max(max)));
                    }
                    if let Some(min) = stats.min_winsorized {
                        total_stats.min_winsorized =
                            Some(total_stats.min_winsorized.map_or(min, |m| m.min(min)));
                    }
                    if let Some(max) = stats.max_winsorized {
                        total_stats.max_winsorized =
                            Some(total_stats.max_winsorized.map_or(max, |m| m.max(max)));
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

/// Process a chunk of records and update bivariate statistics
/// Similar to count_chunk_outliers but for bivariate computation
fn compute_chunk_bivariate<I>(
    field_pairs: &HashMap<(u16, u16), (BivariateFieldInfo, BivariateFieldInfo)>,
    records: I,
    stats_config: BivariateStatsConfig,
) -> CliResult<HashMap<(u16, u16), BivariateChunkStats>>
where
    I: Iterator<Item = csv::Result<csv::ByteRecord>>,
{
    if field_pairs.is_empty() {
        return Ok(HashMap::new());
    }

    // Check what we need to compute based on config
    let needs_all_values = stats_config.needs_all_values();
    let needs_freq_counts = stats_config.needs_frequency_counts();

    // Initialize statistics for all field pairs
    // Pre-allocate vectors with estimated capacity (typical chunk size is 1k-10k records)
    let estimated_capacity = 5000; // Reasonable estimate for chunk processing
    let estimated_unique_strings = estimated_capacity.min(1000); // Estimate for string frequency maps
    let mut chunk_stats: HashMap<(u16, u16), BivariateChunkStats> = field_pairs
        .keys()
        .map(|k| {
            let mut stats = BivariateChunkStats::default();
            // Only allocate value vectors if needed for Spearman/Kendall
            if needs_all_values {
                stats.x_values.reserve(estimated_capacity);
                stats.y_values.reserve(estimated_capacity);
            }
            // Only allocate frequency maps if needed for mutual information
            if needs_freq_counts {
                stats.xy_counts.reserve(estimated_unique_strings);
                stats.x_counts.reserve(estimated_unique_strings / 2);
                stats.y_counts.reserve(estimated_unique_strings / 2);
            }
            (*k, stats)
        })
        .collect();

    let prefer_dmy = util::get_envvar_flag("QSV_PREFER_DMY");

    // Optimization #1: Date parsing cache - Cache parsed dates to avoid re-parsing same strings
    let mut date_cache: HashMap<String, Option<f64>> = HashMap::with_capacity(estimated_capacity);

    // Optimization #6: String interning - Cache frequently used strings to reduce allocations
    // Only needed if we're computing mutual information
    let mut string_interner: HashMap<String, String> = if needs_freq_counts {
        HashMap::with_capacity(estimated_unique_strings)
    } else {
        HashMap::new()
    };

    #[allow(unused_assignments)]
    let mut record: csv::ByteRecord = csv::ByteRecord::new();
    let mut value_bytes_x;
    let mut value_bytes_y;
    let mut numeric_value_x;
    let mut numeric_value_y;

    // Process each record in the chunk
    for result in records {
        record = result?;

        // Optimization #4: Batch string conversions - convert record to strings once, reuse for all
        // field pairs Collect all column indices that need string conversion
        let mut col_indices_needed: HashSet<usize> = HashSet::new();
        for (field1_info, field2_info) in field_pairs.values() {
            col_indices_needed.insert(field1_info.col_idx);
            col_indices_needed.insert(field2_info.col_idx);
        }

        // Convert needed columns to strings once
        let mut record_strings: HashMap<usize, String> =
            HashMap::with_capacity(col_indices_needed.len());
        for &col_idx in &col_indices_needed {
            if let Some(bytes) = record.get(col_idx)
                && !bytes.is_empty()
                && let Ok(s) = from_utf8(bytes)
            {
                record_strings.insert(col_idx, s.to_string());
            }
        }

        for ((idx1, idx2), (field1_info, field2_info)) in field_pairs {
            // Optimization: Check record_strings first (already excludes empty values)
            // This avoids redundant empty checks and byte fetching for empty fields
            let (Some(x_str), Some(y_str)) = (
                record_strings.get(&field1_info.col_idx),
                record_strings.get(&field2_info.col_idx),
            ) else {
                continue; // Skip if either value is empty (not in record_strings)
            };

            // Get mutable reference to stats for this field pair
            let stats = chunk_stats.get_mut(&(*idx1, *idx2)).unwrap();

            // Get bytes only for numeric parsing (date fields use strings from cache)
            value_bytes_x = record.get(field1_info.col_idx).unwrap_or(&[]);
            value_bytes_y = record.get(field2_info.col_idx).unwrap_or(&[]);

            // Optimization #1: Use date parsing cache
            // Optimization #5: Skip date parsing for non-date fields
            numeric_value_x = if field1_info.field_type.is_date_or_datetime() {
                // Use cached parsed date or parse and cache
                *date_cache
                    .entry(x_str.clone())
                    .or_insert_with(|| parse_date_to_days(x_str, prefer_dmy))
            } else {
                // Direct float parsing (much faster than date parsing)
                parse_float_opt_from_bytes(value_bytes_x)
            };

            numeric_value_y = if field2_info.field_type.is_date_or_datetime() {
                // Use cached parsed date or parse and cache
                *date_cache
                    .entry(y_str.clone())
                    .or_insert_with(|| parse_date_to_days(y_str, prefer_dmy))
            } else {
                // Direct float parsing (much faster than date parsing)
                parse_float_opt_from_bytes(value_bytes_y)
            };

            // For numeric/date types, update correlation state and collect values
            if let (Some(x_val), Some(y_val)) = (numeric_value_x, numeric_value_y) {
                update_correlation_state(&mut stats.correlation_state, x_val, y_val);
                // Only store values if needed for Spearman/Kendall
                if needs_all_values {
                    stats.x_values.push(x_val);
                    stats.y_values.push(y_val);
                }
            }

            // Only compute frequency counts if needed for mutual information
            if needs_freq_counts {
                // Optimization #2 & #6: Optimized string interning - reduce clones
                // For occupied entries (common case with repeated strings): 1 clone instead of 3
                // For vacant entries: 2 clones (same as before, but more efficient)
                let x_str_interned = if let Some(cached) = string_interner.get(x_str) {
                    // String already interned - reuse it (1 clone instead of 3)
                    cached.clone()
                } else {
                    // String not interned - clone once and store reference to itself
                    let owned = x_str.clone();
                    string_interner.insert(owned.clone(), owned.clone());
                    owned
                };
                let y_str_interned = if let Some(cached) = string_interner.get(y_str) {
                    // String already interned - reuse it (1 clone instead of 3)
                    cached.clone()
                } else {
                    // String not interned - clone once and store reference to itself
                    let owned = y_str.clone();
                    string_interner.insert(owned.clone(), owned.clone());
                    owned
                };

                // Accumulate joint frequency counts (xy_counts) - these are needed for mutual
                // information. Marginal frequencies (x_counts, y_counts) will be computed
                // from xy_counts at finalization to ensure consistency.
                *stats
                    .xy_counts
                    .entry((x_str_interned, y_str_interned))
                    .or_insert(0) += 1;
                stats.total_pairs += 1;
            }
        }
    }

    Ok(chunk_stats)
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

    let prefer_dmy = util::get_envvar_flag("QSV_PREFER_DMY");

    // amortize allocations
    #[allow(unused_assignments)]
    let mut record: StringRecord = StringRecord::new();
    let mut value_str;
    let mut numeric_value;

    // Process each record once, checking all fields
    for result in rdr.records() {
        record = result?;

        for (field_name, field_info) in fields_to_count {
            value_str = record.get(field_info.col_idx).unwrap_or("");

            if value_str.is_empty() {
                continue; // Skip null/empty values
            }

            // Parse the value based on field type
            numeric_value = if field_info.field_type.is_date_or_datetime() {
                parse_date_to_days(value_str, prefer_dmy)
            } else {
                parse_float_opt(value_str)
            };

            let Some(val) = numeric_value else {
                continue; // Skip values that can't be parsed
            };

            // Get mutable reference to stats for this field
            let stats = all_stats.get_mut(field_name).unwrap();

            // Update sums and count
            stats.sum_all += val;
            stats.count_all += 1;

            // Compute winsorized and trimmed statistics
            let winsorized_val = val
                .max(field_info.lower_threshold)
                .min(field_info.upper_threshold);
            stats.winsorized_sum += winsorized_val;
            stats.winsorized_count += 1;
            // Track winsorized min/max and sum of squares
            stats.min_winsorized = Some(
                stats
                    .min_winsorized
                    .map_or(winsorized_val, |m| m.min(winsorized_val)),
            );
            stats.max_winsorized = Some(
                stats
                    .max_winsorized
                    .map_or(winsorized_val, |m| m.max(winsorized_val)),
            );
            stats.sum_squares_winsorized += winsorized_val * winsorized_val;

            // For trimmed mean, only include values within thresholds
            if val >= field_info.lower_threshold && val <= field_info.upper_threshold {
                stats.trimmed_sum += val;
                stats.trimmed_count += 1;
                // Track trimmed min/max and sum of squares
                stats.min_trimmed = Some(stats.min_trimmed.map_or(val, |m| m.min(val)));
                stats.max_trimmed = Some(stats.max_trimmed.map_or(val, |m| m.max(val)));
                stats.sum_squares_trimmed += val * val;
            }

            // Count outliers and track statistics based on fence comparisons
            if val < field_info.lower_outer {
                stats.counts[0] += 1; // extreme_lower
                stats.counts[5] += 1; // total
                stats.sum_outliers += val;
                stats.sum_squares_outliers += val * val;
                stats.min_outliers = Some(stats.min_outliers.map_or(val, |m| m.min(val)));
                stats.max_outliers = Some(stats.max_outliers.map_or(val, |m| m.max(val)));
            } else if val < field_info.lower_inner {
                stats.counts[1] += 1; // mild_lower
                stats.counts[5] += 1; // total
                stats.sum_outliers += val;
                stats.sum_squares_outliers += val * val;
                stats.min_outliers = Some(stats.min_outliers.map_or(val, |m| m.min(val)));
                stats.max_outliers = Some(stats.max_outliers.map_or(val, |m| m.max(val)));
            } else if val <= field_info.upper_inner {
                stats.counts[2] += 1; // normal
                stats.sum_normal += val;
                stats.sum_squares_normal += val * val;
                stats.min_normal = Some(stats.min_normal.map_or(val, |m| m.min(val)));
                stats.max_normal = Some(stats.max_normal.map_or(val, |m| m.max(val)));
            } else if val <= field_info.upper_outer {
                stats.counts[3] += 1; // mild_upper
                stats.counts[5] += 1; // total
                stats.sum_outliers += val;
                stats.sum_squares_outliers += val * val;
                stats.min_outliers = Some(stats.min_outliers.map_or(val, |m| m.min(val)));
                stats.max_outliers = Some(stats.max_outliers.map_or(val, |m| m.max(val)));
            } else {
                stats.counts[4] += 1; // extreme_upper
                stats.counts[5] += 1; // total
                stats.sum_outliers += val;
                stats.sum_squares_outliers += val * val;
                stats.min_outliers = Some(stats.min_outliers.map_or(val, |m| m.min(val)));
                stats.max_outliers = Some(stats.max_outliers.map_or(val, |m| m.max(val)));
            }
        }
    }

    Ok(all_stats)
}

/// Compute all bivariate statistics
/// Uses parallel chunked processing when an index is available and there
/// are more than 10,000 records.
/// Otherwise, uses sequential processing.
/// Returns a HashMap mapping field pairs to their bivariate statistics.
fn compute_all_bivariatestats(
    field_pairs: &HashMap<(u16, u16), (BivariateFieldInfo, BivariateFieldInfo)>,
    field_names: &[String],
    input_path: &Path,
    progress: Option<&ProgressBar>,
    cardinality_threshold: Option<u64>,
    stats_config: BivariateStatsConfig,
    flag_jobs: Option<usize>,
) -> CliResult<HashMap<(u16, u16), BivariateStats>> {
    if field_pairs.is_empty() {
        return Ok(HashMap::new());
    }

    // Check what we need based on config
    let needs_all_values = stats_config.needs_all_values();
    let needs_freq_counts = stats_config.needs_frequency_counts();

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
            winfo!("Computing bivariate statistics sequentially...");
            return compute_all_bivariatestats_sequential(
                field_pairs,
                field_names,
                rdr,
                progress,
                cardinality_threshold,
                stats_config,
            );
        }

        let njobs = util::njobs(flag_jobs);
        let chunk_size = util::chunk_size(idx_count, njobs);
        let nchunks = util::num_of_chunks(idx_count, chunk_size);

        winfo!("Parallelizing bivariate computation: {nchunks} chunks, {njobs} jobs");

        // Setup progress bar if requested
        if let Some(pb) = progress {
            pb.set_style(
                ProgressStyle::default_bar()
                    .template(
                        "[{elapsed_precise}] [{wide_bar} {percent}%{msg}] ({per_sec} - {eta})",
                    )
                    .unwrap(),
            );
            pb.set_message(format!(" of {} chunks", HumanCount(nchunks as u64)));
            pb.set_length(nchunks as u64);
            log::info!("Progress started... {nchunks} chunks");
        }

        let pool = ThreadPool::new(njobs);
        let (send, recv) = crossbeam_channel::bounded(nchunks);

        // Process each chunk in parallel
        let input_path_string = input_path.to_str().unwrap_or("").to_string();
        for i in 0..nchunks {
            let (send, field_pairs_clone, input_path_string_clone) =
                (send.clone(), field_pairs.clone(), input_path_string.clone());
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
                let result = compute_chunk_bivariate(&field_pairs_clone, it, stats_config);
                let _ = send.send(result);
            });
        }

        drop(send);

        // Aggregate results from all chunks
        // Pre-allocate based on idx_count to avoid repeated reallocations during extend
        let mut all_stats: HashMap<(u16, u16), BivariateChunkStats> = field_pairs
            .keys()
            .map(|k| {
                let mut stats = BivariateChunkStats::default();
                // Pre-allocate value vectors with total capacity if needed
                if needs_all_values {
                    stats.x_values.reserve(idx_count);
                    stats.y_values.reserve(idx_count);
                }
                (*k, stats)
            })
            .collect();

        for chunk_result in &recv {
            let chunk_stats = chunk_result?;
            for (pair_key, stats) in chunk_stats {
                if let Some(total_stats) = all_stats.get_mut(&pair_key) {
                    // Merge correlation states (always needed for Pearson/covariance)
                    total_stats.correlation_state = merge_correlation_states(
                        &total_stats.correlation_state,
                        &stats.correlation_state,
                    );
                    // Only merge values if needed for Spearman/Kendall
                    if needs_all_values {
                        total_stats.x_values.extend(stats.x_values);
                        total_stats.y_values.extend(stats.y_values);
                    }
                    // Only merge frequency counts if needed for mutual information
                    // Note: Only xy_counts and total_pairs are collected during chunk processing
                    // Marginal frequencies (x_counts, y_counts) are computed from xy_counts at
                    // finalization
                    if needs_freq_counts {
                        for ((x_val, y_val), count) in stats.xy_counts {
                            *total_stats.xy_counts.entry((x_val, y_val)).or_insert(0) += count;
                        }
                        total_stats.total_pairs += stats.total_pairs;
                    }
                }
            }
            // Update progress bar
            if let Some(pb) = progress {
                pb.inc(1);
            }
        }

        winfo!("Finalizing bivariate statistics...");
        // Update progress bar for Phase 2: final statistics computation
        let num_field_pairs = all_stats.len();
        if let Some(pb) = progress {
            pb.set_style(
                ProgressStyle::default_bar()
                    .template(
                        "[{elapsed_precise}] [{wide_bar} {percent}%{msg}] ({per_sec} - {eta})",
                    )
                    .unwrap(),
            );
            pb.set_message(format!(
                " of {} field pairs",
                HumanCount(num_field_pairs as u64)
            ));
            pb.set_length(num_field_pairs as u64);
            pb.set_position(0); // Reset position for Phase 2
            log::info!("Phase 2 started... {num_field_pairs} field pairs");
        }

        // Only compute marginal frequencies if we need mutual information
        if needs_freq_counts {
            // Compute marginal frequencies from joint frequencies to ensure consistency
            // This ensures x_counts and y_counts are computed from the same set of records
            // as xy_counts (only pairs where both fields are non-empty)
            // This is critical for correct mutual information calculation
            for chunk_stats in all_stats.values_mut() {
                // Compute marginal frequencies from joint frequencies
                // Sum over y to get x_counts, sum over x to get y_counts
                chunk_stats.x_counts.clear();
                chunk_stats.y_counts.clear();

                for ((x_val, y_val), &count) in &chunk_stats.xy_counts {
                    *chunk_stats.x_counts.entry(x_val.clone()).or_insert(0) += count;
                    *chunk_stats.y_counts.entry(y_val.clone()).or_insert(0) += count;
                }
            }
        }

        // Finalize statistics from aggregated chunk stats (parallelized)
        let final_stats: HashMap<(u16, u16), BivariateStats> = all_stats
            .into_par_iter()
            .map(|(pair_key, chunk_stats)| {
                if let Some(pb) = progress {
                    pb.inc(1);
                }
                let n_pairs = chunk_stats
                    .correlation_state
                    .count
                    .max(chunk_stats.total_pairs);

                // Get field info for this pair to check cardinality threshold
                let (field1_info, field2_info) = field_pairs
                    .get(&pair_key)
                    .unwrap_or_else(|| panic!("Field pair not found: {pair_key:?}"));

                // Early exit: skip all correlation/covariance computations if variance is zero
                let has_zero_variance = field1_info.stddev.is_some_and(|s| s.abs() < f64::EPSILON)
                    || field2_info.stddev.is_some_and(|s| s.abs() < f64::EPSILON)
                    || field1_info.variance.is_some_and(|v| v.abs() < f64::EPSILON)
                    || field2_info.variance.is_some_and(|v| v.abs() < f64::EPSILON);

                // Compute Pearson correlation if requested
                let pearson = if !stats_config.pearson
                    || has_zero_variance
                    || chunk_stats.correlation_state.count < 2
                {
                    None
                } else {
                    finalize_pearson_correlation(&chunk_stats.correlation_state)
                };

                // Compute covariance if requested
                let covariance_sample = if !stats_config.covariance
                    || has_zero_variance
                    || chunk_stats.correlation_state.count < 2
                {
                    None
                } else {
                    finalize_covariance(&chunk_stats.correlation_state, true)
                };
                let covariance_population = if !stats_config.covariance
                    || has_zero_variance
                    || chunk_stats.correlation_state.count < 2
                {
                    None
                } else {
                    finalize_covariance(&chunk_stats.correlation_state, false)
                };

                // Compute Spearman correlation if requested
                let spearman = if !stats_config.spearman
                    || has_zero_variance
                    || chunk_stats.x_values.len() < 2
                {
                    None
                } else {
                    compute_spearman_correlation(&chunk_stats.x_values, &chunk_stats.y_values)
                };

                // Compute Kendall's tau if requested
                let kendall =
                    if !stats_config.kendall || has_zero_variance || chunk_stats.x_values.len() < 2
                    {
                        None
                    } else {
                        compute_kendall_tau(&chunk_stats.x_values, &chunk_stats.y_values)
                    };

                // Compute mutual information if requested and apply cardinality threshold
                let mutual_information = if !stats_config.mi || chunk_stats.total_pairs == 0 {
                    None
                } else if let Some(threshold) = cardinality_threshold {
                    // Check if either field exceeds cardinality threshold
                    let exceeds_threshold = field1_info.cardinality.is_some_and(|c| c > threshold)
                        || field2_info.cardinality.is_some_and(|c| c > threshold);
                    if exceeds_threshold {
                        // Convert indices to names for logging (u16 -> usize for indexing)
                        let (idx1, idx2) = pair_key;
                        let field1_name = field_names
                            .get(idx1 as usize)
                            .map_or("?", std::string::String::as_str);
                        let field2_name = field_names
                            .get(idx2 as usize)
                            .map_or("?", std::string::String::as_str);
                        log::debug!(
                            "Skipping mutual information for pair ({field1_name}, {field2_name}) \
                             - cardinality exceeds threshold {threshold}"
                        );
                        None
                    } else {
                        compute_mutual_information_from_counts(
                            &chunk_stats.xy_counts,
                            &chunk_stats.x_counts,
                            &chunk_stats.y_counts,
                            chunk_stats.total_pairs,
                        )
                    }
                } else {
                    compute_mutual_information_from_counts(
                        &chunk_stats.xy_counts,
                        &chunk_stats.x_counts,
                        &chunk_stats.y_counts,
                        chunk_stats.total_pairs,
                    )
                };

                // Compute normalized mutual information if requested
                // NMI requires MI and entropies computed from the same frequency counts
                let normalized_mutual_information = if !stats_config.nmi
                    || chunk_stats.total_pairs == 0
                {
                    None
                } else if let Some(threshold) = cardinality_threshold {
                    // Check if either field exceeds cardinality threshold (same as MI)
                    let exceeds_threshold = field1_info.cardinality.is_some_and(|c| c > threshold)
                        || field2_info.cardinality.is_some_and(|c| c > threshold);
                    if exceeds_threshold {
                        // Convert indices to names for logging (u16 -> usize for indexing)
                        let (idx1, idx2) = pair_key;
                        let field1_name = field_names
                            .get(idx1 as usize)
                            .map_or("?", std::string::String::as_str);
                        let field2_name = field_names
                            .get(idx2 as usize)
                            .map_or("?", std::string::String::as_str);
                        log::debug!(
                            "Skipping normalized mutual information for pair ({field1_name}, \
                             {field2_name}) - cardinality exceeds threshold {threshold}"
                        );
                        None
                    } else {
                        // Compute entropies from marginal frequency counts
                        let h_x = compute_entropy_from_counts(
                            &chunk_stats.x_counts,
                            chunk_stats.total_pairs,
                        );
                        let h_y = compute_entropy_from_counts(
                            &chunk_stats.y_counts,
                            chunk_stats.total_pairs,
                        );
                        // Compute MI if not already computed (needed for NMI)
                        let mi = if mutual_information.is_some() {
                            mutual_information
                        } else {
                            compute_mutual_information_from_counts(
                                &chunk_stats.xy_counts,
                                &chunk_stats.x_counts,
                                &chunk_stats.y_counts,
                                chunk_stats.total_pairs,
                            )
                        };
                        compute_normalized_mutual_information(mi, h_x, h_y)
                    }
                } else {
                    // Compute entropies from marginal frequency counts
                    let h_x =
                        compute_entropy_from_counts(&chunk_stats.x_counts, chunk_stats.total_pairs);
                    let h_y =
                        compute_entropy_from_counts(&chunk_stats.y_counts, chunk_stats.total_pairs);
                    // Compute MI if not already computed (needed for NMI)
                    let mi = if mutual_information.is_some() {
                        mutual_information
                    } else {
                        compute_mutual_information_from_counts(
                            &chunk_stats.xy_counts,
                            &chunk_stats.x_counts,
                            &chunk_stats.y_counts,
                            chunk_stats.total_pairs,
                        )
                    };
                    compute_normalized_mutual_information(mi, h_x, h_y)
                };

                (
                    pair_key,
                    BivariateStats {
                        pearson,
                        spearman,
                        kendall,
                        covariance_sample,
                        covariance_population,
                        mutual_information,
                        normalized_mutual_information,
                        n_pairs,
                    },
                )
            })
            .collect();

        // Finish progress bar after final statistics computation
        if let Some(pb) = progress {
            util::finish_progress(pb);
        }

        Ok(final_stats)
    } else {
        // Sequential fallback when no index exists
        let mut rdr = rconfig.reader_file()?;
        let _headers = rdr.headers()?.clone();
        compute_all_bivariatestats_sequential(
            field_pairs,
            field_names,
            rdr,
            progress,
            cardinality_threshold,
            stats_config,
        )
    }
}

/// Sequential processing for small files (< 10k records) or when no index exists
fn compute_all_bivariatestats_sequential(
    field_pairs: &HashMap<(u16, u16), (BivariateFieldInfo, BivariateFieldInfo)>,
    field_names: &[String],
    mut rdr: csv::Reader<std::fs::File>,
    progress: Option<&ProgressBar>,
    cardinality_threshold: Option<u64>,
    stats_config: BivariateStatsConfig,
) -> CliResult<HashMap<(u16, u16), BivariateStats>> {
    if field_pairs.is_empty() {
        return Ok(HashMap::new());
    }

    // Check what we need based on config
    let needs_all_values = stats_config.needs_all_values();
    let needs_freq_counts = stats_config.needs_frequency_counts();

    // Collect all values for each field pair
    // Use frequency counts for strings instead of storing all strings
    let estimated_capacity = 5000; // Reasonable estimate for sequential processing
    let estimated_unique_strings = estimated_capacity.min(1000); // Estimate for string frequency maps
    let mut pair_values: HashMap<
        (u16, u16),
        (
            Vec<f64>,
            Vec<f64>,
            CorrelationState, // Always track correlation state for Pearson/covariance
            HashMap<(String, String), u64>,
            HashMap<String, u64>,
            HashMap<String, u64>,
            u64,
        ),
    > = field_pairs
        .keys()
        .map(|k| {
            let mut xy_counts = HashMap::new();
            let mut x_counts = HashMap::new();
            let mut y_counts = HashMap::new();
            // Only allocate if needed
            if needs_freq_counts {
                xy_counts.reserve(estimated_unique_strings);
                x_counts.reserve(estimated_unique_strings / 2);
                y_counts.reserve(estimated_unique_strings / 2);
            }
            (
                *k,
                (
                    if needs_all_values {
                        Vec::with_capacity(estimated_capacity)
                    } else {
                        Vec::new()
                    },
                    if needs_all_values {
                        Vec::with_capacity(estimated_capacity)
                    } else {
                        Vec::new()
                    },
                    CorrelationState::default(), // Always initialize for Pearson/covariance
                    xy_counts,
                    x_counts,
                    y_counts,
                    0,
                ),
            )
        })
        .collect();

    let prefer_dmy = util::get_envvar_flag("QSV_PREFER_DMY");

    // Optimization #1: Date parsing cache - Cache parsed dates to avoid re-parsing same strings
    let mut date_cache: HashMap<String, Option<f64>> = HashMap::with_capacity(estimated_capacity);

    // Optimization #6: String interning - Cache frequently used strings to reduce allocations
    // Only needed if we're computing mutual information
    let mut string_interner: HashMap<String, String> = if needs_freq_counts {
        HashMap::with_capacity(estimated_unique_strings)
    } else {
        HashMap::new()
    };

    // amortize allocations
    #[allow(unused_assignments)]
    let mut record: StringRecord = StringRecord::new();
    let mut value_str_x;
    let mut value_str_y;
    let mut numeric_value_x;
    let mut numeric_value_y;

    // Process each record once, collecting values for all field pairs
    let mut processed = 0u64;
    for result in rdr.records() {
        record = result?;
        processed += 1;

        // Update progress bar every 1000 records
        if let Some(pb) = progress {
            if processed == 1 {
                // Initialize progress bar on first record (unknown total)
                pb.set_style(
                    ProgressStyle::default_bar()
                        .template("[{elapsed_precise}] [{wide_bar}] {pos} records ({per_sec})")
                        .unwrap(),
                );
                pb.set_length(0); // Unknown length
            }
            if processed.is_multiple_of(1000) {
                pb.set_position(processed);
            }
        }

        // Optimization #4: Batch string conversions - record is already StringRecord, so strings
        // are available But we still need to cache date parsing results

        for ((idx1, idx2), (field1_info, field2_info)) in field_pairs {
            value_str_x = record.get(field1_info.col_idx).unwrap_or("");
            value_str_y = record.get(field2_info.col_idx).unwrap_or("");

            if value_str_x.is_empty() || value_str_y.is_empty() {
                continue;
            }

            if let Some((x_nums, y_nums, correlation_state, xy_counts, _, _, total_pairs)) =
                pair_values.get_mut(&(*idx1, *idx2))
            {
                // Optimization #1: Use date parsing cache
                // Optimization #5: Skip date parsing for non-date fields
                numeric_value_x = if field1_info.field_type.is_date_or_datetime() {
                    // Use cached parsed date or parse and cache
                    *date_cache
                        .entry(value_str_x.to_string())
                        .or_insert_with(|| parse_date_to_days(value_str_x, prefer_dmy))
                } else {
                    // Direct float parsing (much faster than date parsing)
                    parse_float_opt(value_str_x)
                };

                numeric_value_y = if field2_info.field_type.is_date_or_datetime() {
                    // Use cached parsed date or parse and cache
                    *date_cache
                        .entry(value_str_y.to_string())
                        .or_insert_with(|| parse_date_to_days(value_str_y, prefer_dmy))
                } else {
                    // Direct float parsing (much faster than date parsing)
                    parse_float_opt(value_str_y)
                };

                if let (Some(x_val), Some(y_val)) = (numeric_value_x, numeric_value_y) {
                    // Always update correlation state for Pearson/covariance
                    update_correlation_state(correlation_state, x_val, y_val);
                    // Only store values if needed for Spearman/Kendall
                    if needs_all_values {
                        x_nums.push(x_val);
                        y_nums.push(y_val);
                    }
                }

                // Only compute frequency counts if needed for mutual information
                if needs_freq_counts {
                    // Optimization #2 & #6: Reduce string allocations using string interning
                    // Intern strings to reuse allocations for frequently repeated values
                    let x_str = string_interner
                        .entry(value_str_x.to_string())
                        .or_insert_with(|| value_str_x.to_string())
                        .clone();
                    let y_str = string_interner
                        .entry(value_str_y.to_string())
                        .or_insert_with(|| value_str_y.to_string())
                        .clone();

                    // Accumulate joint frequency counts (xy_counts) - these are needed for mutual
                    // information. Marginal frequencies (x_counts, y_counts) are computed from
                    // xy_counts at finalization to ensure consistency.
                    *xy_counts.entry((x_str, y_str)).or_insert(0) += 1;
                    *total_pairs += 1;
                }
            }
        }
    }

    // Finish progress bar
    if let Some(pb) = progress {
        pb.set_position(processed);
        util::finish_progress(pb);
    }

    // Compute statistics for each field pair
    let mut final_stats: HashMap<(u16, u16), BivariateStats> =
        HashMap::with_capacity(field_pairs.len() * 2);

    for (pair_key, (x_nums, y_nums, correlation_state, xy_counts, _, _, total_pairs)) in pair_values
    {
        // Get field info for this pair to check variance and cardinality
        let (field1_info, field2_info) = field_pairs
            .get(&pair_key)
            .unwrap_or_else(|| panic!("Field pair not found: {pair_key:?}"));

        // Compute marginal frequencies from joint frequencies if needed for mutual information
        // This ensures x_counts and y_counts are computed from the same set of records
        // as xy_counts (only pairs where both fields are non-empty)
        let (x_counts, y_counts) = if needs_freq_counts && !xy_counts.is_empty() {
            let mut x_counts: HashMap<String, u64> = HashMap::new();
            let mut y_counts: HashMap<String, u64> = HashMap::new();
            for ((x_val, y_val), &count) in &xy_counts {
                *x_counts.entry(x_val.clone()).or_insert(0) += count;
                *y_counts.entry(y_val.clone()).or_insert(0) += count;
            }
            (x_counts, y_counts)
        } else {
            (HashMap::new(), HashMap::new())
        };

        // Early termination: check for zero variance
        let has_zero_variance = field1_info.stddev.is_some_and(|s| s.abs() < f64::EPSILON)
            || field2_info.stddev.is_some_and(|s| s.abs() < f64::EPSILON)
            || field1_info.variance.is_some_and(|v| v.abs() < f64::EPSILON)
            || field2_info.variance.is_some_and(|v| v.abs() < f64::EPSILON);

        let n_pairs = correlation_state.count.max(total_pairs);

        // Compute Pearson correlation if requested (use correlation_state)
        let pearson = if !stats_config.pearson || has_zero_variance || correlation_state.count < 2 {
            None
        } else {
            finalize_pearson_correlation(&correlation_state)
        };

        // Compute Spearman correlation if requested (requires arrays)
        let spearman = if !stats_config.spearman || has_zero_variance || x_nums.len() < 2 {
            None
        } else {
            compute_spearman_correlation(&x_nums, &y_nums)
        };

        // Compute Kendall's tau if requested (requires arrays)
        let kendall = if !stats_config.kendall || has_zero_variance || x_nums.len() < 2 {
            None
        } else {
            compute_kendall_tau(&x_nums, &y_nums)
        };

        // Compute covariance from correlation state (skip if not requested or variance is zero)
        let (covariance_sample, covariance_population) =
            if !stats_config.covariance || has_zero_variance || correlation_state.count < 2 {
                (None, None)
            } else {
                (
                    finalize_covariance(&correlation_state, true),
                    finalize_covariance(&correlation_state, false),
                )
            };

        // Compute mutual information if requested and apply cardinality threshold
        let mutual_information = if !stats_config.mi || total_pairs == 0 {
            None
        } else if let Some(threshold) = cardinality_threshold {
            // Check if either field exceeds cardinality threshold
            let exceeds_threshold = field1_info.cardinality.is_some_and(|c| c > threshold)
                || field2_info.cardinality.is_some_and(|c| c > threshold);
            if exceeds_threshold {
                // Convert indices to names for logging (u16 -> usize for indexing)
                let (idx1, idx2) = pair_key;
                let field1_name = field_names.get(idx1 as usize).map_or("?", |s| s.as_str());
                let field2_name = field_names.get(idx2 as usize).map_or("?", |s| s.as_str());
                log::debug!(
                    "Skipping mutual information for pair ({field1_name}, {field2_name}) - \
                     cardinality exceeds threshold {threshold}",
                );
                None
            } else {
                compute_mutual_information_from_counts(
                    &xy_counts,
                    &x_counts,
                    &y_counts,
                    total_pairs,
                )
            }
        } else {
            compute_mutual_information_from_counts(&xy_counts, &x_counts, &y_counts, total_pairs)
        };

        // Compute normalized mutual information if requested
        // NMI requires MI and entropies computed from the same frequency counts
        let normalized_mutual_information = if !stats_config.nmi || total_pairs == 0 {
            None
        } else if let Some(threshold) = cardinality_threshold {
            // Check if either field exceeds cardinality threshold (same as MI)
            let exceeds_threshold = field1_info.cardinality.is_some_and(|c| c > threshold)
                || field2_info.cardinality.is_some_and(|c| c > threshold);
            if exceeds_threshold {
                // Convert indices to names for logging (u16 -> usize for indexing)
                let (idx1, idx2) = pair_key;
                let field1_name = field_names.get(idx1 as usize).map_or("?", |s| s.as_str());
                let field2_name = field_names.get(idx2 as usize).map_or("?", |s| s.as_str());
                log::debug!(
                    "Skipping normalized mutual information for pair ({field1_name}, \
                     {field2_name}) - cardinality exceeds threshold {threshold}",
                );
                None
            } else {
                // Compute entropies from marginal frequency counts
                let h_x = compute_entropy_from_counts(&x_counts, total_pairs);
                let h_y = compute_entropy_from_counts(&y_counts, total_pairs);
                // Compute MI if not already computed (needed for NMI)
                let mi = if mutual_information.is_some() {
                    mutual_information
                } else {
                    compute_mutual_information_from_counts(
                        &xy_counts,
                        &x_counts,
                        &y_counts,
                        total_pairs,
                    )
                };
                compute_normalized_mutual_information(mi, h_x, h_y)
            }
        } else {
            // Compute entropies from marginal frequency counts
            let h_x = compute_entropy_from_counts(&x_counts, total_pairs);
            let h_y = compute_entropy_from_counts(&y_counts, total_pairs);
            // Compute MI if not already computed (needed for NMI)
            let mi = if mutual_information.is_some() {
                mutual_information
            } else {
                compute_mutual_information_from_counts(
                    &xy_counts,
                    &x_counts,
                    &y_counts,
                    total_pairs,
                )
            };
            compute_normalized_mutual_information(mi, h_x, h_y)
        };

        final_stats.insert(
            pair_key,
            BivariateStats {
                pearson,
                spearman,
                kendall,
                covariance_sample,
                covariance_population,
                mutual_information,
                normalized_mutual_information,
                n_pairs,
            },
        );
    }

    Ok(final_stats)
}

/// Compute Kurtosis, Gini coefficient, and Atkinson index for all fields.
/// Since Kurtosis, Gini & Atkinson Index require all values from the entire dataset, this always
/// uses sequential processing to read all values in a single pass.
/// Returns a HashMap mapping field names to their Kurtosis, Gini coefficient, and Atkinson index
/// statistics
fn compute_all_kga(
    fields_to_compute: &HashMap<String, KGAFieldInfo>,
    input_path: &Path,
    atkinson_epsilon: f64,
) -> CliResult<HashMap<String, KGAStats>> {
    if fields_to_compute.is_empty() {
        return Ok(HashMap::new());
    }

    let input_path_str = input_path
        .to_str()
        .ok_or_else(|| CliError::Other(format!("Invalid input path: {}", input_path.display())))?;
    let input_path_string = input_path_str.to_string();
    let rconfig = Config::new(Some(&input_path_string));
    let mut rdr = rconfig.reader_file()?;
    let _headers = rdr.headers()?.clone();
    compute_all_kga_from_reader(fields_to_compute, rdr, atkinson_epsilon)
}

/// Compute Kurtosis, Gini coefficient, and Atkinson index for all fields in a single pass through
/// the CSV (sequential) The CSV reader should already be positioned after the headers
/// Returns a HashMap mapping field names to their Kurtosis, Gini coefficient, and Atkinson index
/// statistics
fn compute_all_kga_from_reader(
    fields_to_compute: &HashMap<String, KGAFieldInfo>,
    mut rdr: csv::Reader<std::fs::File>,
    atkinson_epsilon: f64,
) -> CliResult<HashMap<String, KGAStats>> {
    if fields_to_compute.is_empty() {
        return Ok(HashMap::new());
    }

    // Collect all values for each field
    let mut field_values: HashMap<String, Vec<f64>> = fields_to_compute
        .keys()
        .map(|k| (k.clone(), Vec::new()))
        .collect();

    let prefer_dmy = util::get_envvar_flag("QSV_PREFER_DMY");

    // amortize allocations
    #[allow(unused_assignments)]
    let mut record: StringRecord = StringRecord::new();
    let mut value_str;
    let mut numeric_value;

    // Process each record once, collecting values for all fields
    for result in rdr.records() {
        record = result?;

        for (field_name, field_info) in fields_to_compute {
            value_str = record.get(field_info.col_idx).unwrap_or("");

            if value_str.is_empty() {
                continue; // Skip null/empty values
            }

            // Parse the value based on field type
            numeric_value = if field_info.field_type.is_date_or_datetime() {
                parse_date_to_days(value_str, prefer_dmy)
            } else {
                parse_float_opt(value_str)
            };

            if let Some(val) = numeric_value
                && let Some(values) = field_values.get_mut(field_name)
            {
                values.push(val);
            }
        }
    }

    // Compute statistics for each field
    let mut all_stats: HashMap<String, KGAStats> = HashMap::new();

    for (field_name, values) in field_values {
        if values.len() < 2 {
            // Need at least 2 values for meaningful statistics
            all_stats.insert(
                field_name,
                KGAStats {
                    kurtosis:         None,
                    gini_coefficient: None,
                    atkinson_index:   None,
                },
            );
            continue;
        }

        // Get precalculated stats for this field
        let (precalc_mean, precalc_variance, precalc_sum) = fields_to_compute
            .get(&field_name)
            .map_or((None, None, None), |info| {
                (info.mean, info.variance, info.sum)
            });

        // Compute kurtosis with precalculated mean and variance
        let kurtosis_val = kurtosis(values.iter().copied(), precalc_mean, precalc_variance);

        // Compute Gini coefficient with precalculated sum (not mean!)
        let gini_val = gini(values.iter().copied(), precalc_sum);

        // Compute Atkinson Index (epsilon parameter typically 0.5 or 1.0, configurable via
        // --epsilon) atkinson function signature: atkinson(iter, epsilon,
        // precalc_mean, precalc_geometric_sum) See: https://docs.rs/qsv-stats/latest/stats/fn.atkinson.html
        let atkinson_val = atkinson(
            values.iter().copied(),
            atkinson_epsilon,
            precalc_mean,
            None, // geometric sum not precalculated
        );

        all_stats.insert(
            field_name,
            KGAStats {
                kurtosis:         kurtosis_val,
                gini_coefficient: gini_val,
                atkinson_index:   atkinson_val,
            },
        );
    }

    Ok(all_stats)
}

/// Compute Shannon Entropy for all fields by calling the frequency command.
/// Uses run_qsv_cmd to call frequency command with --limit 0 to get all frequencies,
/// then parses the CSV output and computes entropy for each field.
/// Returns a HashMap mapping field names to their entropy statistics
fn compute_all_entropy(input_path: &Path) -> CliResult<HashMap<String, EntropyStats>> {
    let input_path_str = input_path
        .to_str()
        .ok_or_else(|| CliError::Other(format!("Invalid input path: {}", input_path.display())))?;

    // Call frequency command with --limit 0 to get all frequencies for all fields
    let (freq_output, _) = util::run_qsv_cmd(
        "frequency",
        &["--limit", "0"],
        input_path_str,
        "Computing frequency distributions for entropy...",
    )?;

    // Parse the frequency CSV output
    // Format: field,value,count,percentage,rank
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(freq_output.as_bytes());

    let headers = rdr.headers()?.clone();
    let field_idx = headers
        .iter()
        .position(|h| h == "field")
        .ok_or_else(|| CliError::Other("Frequency CSV missing 'field' column".to_string()))?;
    let value_idx = headers
        .iter()
        .position(|h| h == "value")
        .ok_or_else(|| CliError::Other("Frequency CSV missing 'value' column".to_string()))?;
    let count_idx = headers
        .iter()
        .position(|h| h == "count")
        .ok_or_else(|| CliError::Other("Frequency CSV missing 'count' column".to_string()))?;

    // Group frequencies by field name
    let mut field_frequencies: HashMap<String, HashMap<String, u64>> = HashMap::new();
    let mut field_totals: HashMap<String, u64> = HashMap::new();

    for result in rdr.records() {
        let record = result?;
        let field_name = record.get(field_idx).unwrap_or("").to_string();
        let value = record.get(value_idx).unwrap_or("").to_string();
        let count: u64 = record
            .get(count_idx)
            .ok_or_else(|| CliError::Other("Missing count in frequency CSV".to_string()))?
            .parse()
            .map_err(|e| CliError::Other(format!("Failed to parse count: {e}")))?;

        // Skip empty field names (shouldn't happen, but be safe)
        if field_name.is_empty() {
            continue;
        }

        // Initialize field entry if needed
        field_frequencies
            .entry(field_name.clone())
            .or_default()
            .insert(value, count);

        // Accumulate total count for this field
        *field_totals.entry(field_name).or_insert(0) += count;
    }

    // Compute entropy for each field
    let mut entropy_stats: HashMap<String, EntropyStats> = HashMap::new();

    #[allow(clippy::cast_precision_loss)]
    for (field_name, frequencies) in field_frequencies {
        let total_count = field_totals.get(&field_name).copied().unwrap_or(0);

        if total_count == 0 {
            entropy_stats.insert(field_name, EntropyStats { entropy: None });
            continue;
        }

        // Check if this is an all-unique field (frequency command outputs <ALL_UNIQUE> for these)
        // The default text is "<ALL_UNIQUE>" but it can be customized with --all-unique-text
        // We check for both the default and common variations
        let is_all_unique = frequencies.len() == 1
            && frequencies.keys().any(|v| {
                v == "<ALL_UNIQUE>"
                    || v == "<ALL UNIQUE>"
                    || (v.starts_with("<ALL") && v.contains("UNIQUE"))
            });

        let entropy = if is_all_unique {
            // For all-unique fields, each value appears exactly once
            // Entropy = log2(n) where n is the number of unique values (which equals total_count)
            // Formula: -Σ p_i * log2(p_i) where p_i = 1/n for each of n values
            // = -n * (1/n) * log2(1/n) = -log2(1/n) = log2(n)
            (total_count as f64).log2()
        } else {
            // Compute Shannon Entropy: H(X) = -Σ p_i * log2(p_i)
            let mut entropy = 0.0;
            let total = total_count as f64;

            for count in frequencies.values() {
                if *count > 0 {
                    let p = *count as f64 / total;
                    entropy -= p * p.log2();
                }
            }
            entropy
        };

        entropy_stats.insert(
            field_name,
            EntropyStats {
                entropy: Some(entropy),
            },
        );
    }

    Ok(entropy_stats)
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

    // Check atkinson epsilon is >= 0
    if args.flag_advanced && args.flag_epsilon < 0.0 {
        return fail_incorrectusage_clierror!(
            "Atkinson Index inequality aversion parameter must be >= 0. Got: {}",
            args.flag_epsilon
        );
    }

    // Handle multi-dataset join if requested
    let temp_joined_path: Option<PathBuf>;
    let actual_input_path = if let Some(ref join_inputs_str) = args.flag_join_inputs {
        let additional_inputs: Vec<String> = join_inputs_str
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();
        let join_keys_str = args.flag_join_keys.as_ref().ok_or_else(|| {
            CliError::IncorrectUsage(
                "--join-keys required when --join-inputs is specified".to_string(),
            )
        })?;
        let join_keys: Vec<String> = join_keys_str
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();
        let join_type = args.flag_join_type.as_deref().unwrap_or("inner");

        let joined_path =
            join_datasets_internal(input_path, &additional_inputs, &join_keys, join_type)?;
        temp_joined_path = Some(joined_path);
        temp_joined_path.as_ref().unwrap()
    } else {
        temp_joined_path = None;
        input_path
    };

    // Auto-create index if --advanced or --bivariate is set and index doesn't exist
    if args.flag_advanced || args.flag_bivariate {
        let actual_input_path_str = actual_input_path
            .to_str()
            .ok_or_else(|| CliError::Other("Invalid input path".to_string()))?
            .to_string();
        let rconfig = Config::new(Some(&actual_input_path_str));
        let indexed_result = rconfig.indexed()?;

        if indexed_result.is_none() && !rconfig.is_stdin() {
            let option_name = if args.flag_bivariate {
                "--bivariate"
            } else {
                "--advanced"
            };
            log::info!(
                "{option_name} option requires reading the entire CSV file. Auto-creating index \
                 to enable parallel processing..."
            );

            match util::create_index_for_file(actual_input_path, &rconfig) {
                Ok(()) => {
                    log::info!("Index created successfully for statistics computation.");
                },
                Err(index_err) => {
                    log::warn!("Failed to auto-create index: {index_err}");
                    // Continue anyway - the code will fall back to sequential processing
                },
            }
        }
    }

    // Determine stats CSV path
    // If we joined datasets, we need stats for the joined dataset, but write bivariate stats
    // based on the original input path
    let stats_csv_path = if temp_joined_path.is_some() {
        // For joined datasets, generate stats for the joined dataset
        // Use a temp stats CSV file
        let actual_input_path_str = actual_input_path
            .to_str()
            .ok_or_else(|| CliError::Other("Invalid joined path".to_string()))?
            .to_string();
        let temp_stats_file = tempfile::Builder::new().suffix(".stats.csv").tempfile_in(
            crate::config::TEMP_FILE_DIR.get_or_init(|| tempfile::TempDir::new().unwrap().keep()),
        )?;
        let temp_stats_path = temp_stats_file.path().to_path_buf();
        drop(temp_stats_file); // Close so stats can write to it

        // Generate stats for joined dataset
        let stats_args_vec: Vec<&str> = args.flag_stats_options.split_whitespace().collect();
        let mut stats_cmd_args = stats_args_vec.clone();
        stats_cmd_args.push("--output");
        stats_cmd_args.push(temp_stats_path.to_str().unwrap());

        let qsv_path = env::current_exe()
            .map_err(|e| CliError::Other(format!("Failed to get current executable path: {e:?}")))?
            .to_string_lossy()
            .to_string();
        let mut cmd = Command::new(&qsv_path);
        cmd.arg("stats")
            .args(&stats_cmd_args)
            .arg(&actual_input_path_str);
        let output = cmd
            .output()
            .map_err(|e| CliError::Other(format!("Error while executing stats command: {e:?}")))?;
        if !output.status.success() {
            return fail_clierror!(
                "Command stats failed: Output {{ status: {:?}, stdout: {:?}, stderr: {:?} }}",
                output.status,
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
        }

        temp_stats_path
    } else {
        // For single dataset, use normal stats CSV path
        let path = get_stats_csv_path(input_path)?;

        // Check if stats CSV exists, if not, run stats command
        if args.flag_force || !path.exists() {
            if args.flag_force {
                winfo!("Force flag set: recomputing stats...");
            } else {
                wwarn!(
                    "Stats CSV file not found: {}\nComputing baseline stats...",
                    path.display()
                );
            }

            // Parse stats options
            let stats_args_vec: Vec<&str> = args.flag_stats_options.split_whitespace().collect();
            let _ = util::run_qsv_cmd(
                "stats",
                &stats_args_vec,
                &input_path_str,
                "Ran stats command to generate baseline stats...",
            )?;
            if !path.exists() {
                return fail_clierror!("Stats CSV file was not created: {}", path.display());
            }
        }

        path
    };

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
    let variance_idx = headers.iter().position(|h| h == "variance");
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
    let sum_idx = headers.iter().position(|h| h == "sum");
    let skewness_idx = headers.iter().position(|h| h == "skewness");
    let cardinality_idx = headers.iter().position(|h| h == "cardinality");
    // let nullcount_idx = headers.iter().position(|h| h == "nullcount");
    let lower_outer_fence_idx = headers.iter().position(|h| h == "lower_outer_fence");
    let lower_inner_fence_idx = headers.iter().position(|h| h == "lower_inner_fence");
    let upper_inner_fence_idx = headers.iter().position(|h| h == "upper_inner_fence");
    let upper_outer_fence_idx = headers.iter().position(|h| h == "upper_outer_fence");
    let percentiles_idx = headers.iter().position(|h| h == "percentiles");

    // Parse and validate scan mode for Gregorian XSD date type detection
    let scan_mode = args.flag_xsd_gdate_scan.as_deref().unwrap_or("fast");
    if scan_mode != "fast" && scan_mode != "comprehensive" {
        return fail_clierror!(
            "Invalid scan mode: {}. Must be either 'fast' or 'comprehensive'",
            scan_mode
        );
    }

    // Parse and validate percentile thresholds if --use-percentiles is set
    let (lower_percentile, upper_percentile) = if args.flag_use_percentiles {
        let thresholds_str = args
            .flag_pct_thresholds
            .as_ref()
            .map_or("5,95", std::string::String::as_str);

        let parts: Vec<&str> = thresholds_str.split(',').map(str::trim).collect();
        if parts.len() != 2 {
            return fail_clierror!(
                "Invalid percentile thresholds: {}. Expected format: 'lower,upper' (e.g., '5,95')",
                thresholds_str
            );
        }

        let lower = fast_float2::parse::<f64, &[u8]>(parts[0].as_bytes()).map_err(|_| {
            CliError::IncorrectUsage(format!("Invalid lower percentile: {}", parts[0]))
        })?;
        let upper = fast_float2::parse::<f64, &[u8]>(parts[1].as_bytes()).map_err(|_| {
            CliError::IncorrectUsage(format!("Invalid upper percentile: {}", parts[1]))
        })?;

        if !(0.0..=100.0).contains(&lower) || !(0.0..=100.0).contains(&upper) {
            return fail_clierror!(
                "Percentile thresholds must be between 0 and 100. Got: {}, {}",
                lower,
                upper
            );
        }

        if lower >= upper {
            return fail_clierror!(
                "Lower percentile must be less than upper percentile. Got: {}, {}",
                lower,
                upper
            );
        }

        (Some(lower), Some(upper))
    } else {
        (None, None)
    };

    // Helper function to check if a column already exists in headers
    let column_exists = |col_name: &str| headers.iter().any(|h| h == col_name);

    // Generate Atkinson Index column name with epsilon parameter
    let atkinson_index_col_name = format!("atkinson_index_({})", args.flag_epsilon);

    // Check which new columns we can add (based on available base stats)
    // Skip columns that already exist to avoid duplicates
    let mut new_columns: Vec<String> = Vec::new();
    let mut new_column_indices = IndexMap::new();

    if mean_idx.is_some()
        && (median_idx.is_some() || q2_median_idx.is_some())
        && stddev_idx.is_some()
        && !column_exists("pearson_skewness")
    {
        new_columns.push("pearson_skewness".to_string());
        new_column_indices.insert("pearson_skewness".to_string(), new_columns.len() - 1);
    }

    if range_idx.is_some() && stddev_idx.is_some() && !column_exists("range_stddev_ratio") {
        new_columns.push("range_stddev_ratio".to_string());
        new_column_indices.insert("range_stddev_ratio".to_string(), new_columns.len() - 1);
    }

    if q1_idx.is_some() && q3_idx.is_some() && !column_exists("quartile_coefficient_dispersion") {
        new_columns.push("quartile_coefficient_dispersion".to_string());
        new_column_indices.insert(
            "quartile_coefficient_dispersion".to_string(),
            new_columns.len() - 1,
        );
    }

    if mode_idx.is_some()
        && mean_idx.is_some()
        && stddev_idx.is_some()
        && !column_exists("mode_zscore")
    {
        new_columns.push("mode_zscore".to_string());
        new_column_indices.insert("mode_zscore".to_string(), new_columns.len() - 1);
    }

    if sem_idx.is_some() && mean_idx.is_some() && !column_exists("relative_standard_error") {
        new_columns.push("relative_standard_error".to_string());
        new_column_indices.insert("relative_standard_error".to_string(), new_columns.len() - 1);
    }

    if min_idx.is_some()
        && mean_idx.is_some()
        && stddev_idx.is_some()
        && !column_exists("min_zscore")
    {
        new_columns.push("min_zscore".to_string());
        new_column_indices.insert("min_zscore".to_string(), new_columns.len() - 1);
    }

    if max_idx.is_some()
        && mean_idx.is_some()
        && stddev_idx.is_some()
        && !column_exists("max_zscore")
    {
        new_columns.push("max_zscore".to_string());
        new_column_indices.insert("max_zscore".to_string(), new_columns.len() - 1);
    }

    if (median_idx.is_some() || q2_median_idx.is_some())
        && mean_idx.is_some()
        && !column_exists("median_mean_ratio")
    {
        new_columns.push("median_mean_ratio".to_string());
        new_column_indices.insert("median_mean_ratio".to_string(), new_columns.len() - 1);
    }

    if iqr_idx.is_some() && range_idx.is_some() && !column_exists("iqr_range_ratio") {
        new_columns.push("iqr_range_ratio".to_string());
        new_column_indices.insert("iqr_range_ratio".to_string(), new_columns.len() - 1);
    }

    if mad_idx.is_some() && stddev_idx.is_some() && !column_exists("mad_stddev_ratio") {
        new_columns.push("mad_stddev_ratio".to_string());
        new_column_indices.insert("mad_stddev_ratio".to_string(), new_columns.len() - 1);
    }

    // Add kurtosis column (requires reading raw data, computed for numeric/date types)
    // Only add if --advanced flag is set
    if args.flag_advanced && !column_exists("kurtosis") {
        new_columns.push("kurtosis".to_string());
        new_column_indices.insert("kurtosis".to_string(), new_columns.len() - 1);
    }

    // Add bimodality coefficient (requires skewness from base stats and kurtosis from --advanced)
    // Only add if --advanced flag is set (since it requires kurtosis)
    if args.flag_advanced
        && skewness_idx.is_some()
        && new_column_indices.contains_key("kurtosis")
        && !column_exists("bimodality_coefficient")
    {
        new_columns.push("bimodality_coefficient".to_string());
        new_column_indices.insert("bimodality_coefficient".to_string(), new_columns.len() - 1);
    }

    // Add Gini coefficient column (requires reading raw data, computed for numeric/date types)
    // Only add if --advanced flag is set
    if args.flag_advanced && !column_exists("gini_coefficient") {
        new_columns.push("gini_coefficient".to_string());
        new_column_indices.insert("gini_coefficient".to_string(), new_columns.len() - 1);
    }

    // Add Atkinson Index column (requires reading raw data, computed for numeric/date types)
    // Only add if --advanced flag is set
    if args.flag_advanced && !column_exists(&atkinson_index_col_name) {
        new_columns.push(atkinson_index_col_name.clone());
        new_column_indices.insert(atkinson_index_col_name.clone(), new_columns.len() - 1);
    }

    // Add Shannon Entropy column (requires reading raw data, computed for all field types)
    // Only add if --advanced flag is set
    if args.flag_advanced && !column_exists("shannon_entropy") {
        new_columns.push("shannon_entropy".to_string());
        new_column_indices.insert("shannon_entropy".to_string(), new_columns.len() - 1);
    }

    if new_column_indices.contains_key("shannon_entropy")
        && cardinality_idx.is_some()
        && !column_exists("normalized_entropy")
    {
        new_columns.push("normalized_entropy".to_string());
        new_column_indices.insert("normalized_entropy".to_string(), new_columns.len() - 1);
    }

    // Add XSD type column (computed for all field types based on type and min/max)
    if !column_exists("xsd_type") {
        new_columns.push("xsd_type".to_string());
        new_column_indices.insert("xsd_type".to_string(), new_columns.len() - 1);
    }

    // Add outlier count columns if all fences are available
    // Only add if at least one outlier column doesn't exist (to avoid partial duplicates)
    if lower_outer_fence_idx.is_some()
        && lower_inner_fence_idx.is_some()
        && upper_inner_fence_idx.is_some()
        && upper_outer_fence_idx.is_some()
        && !column_exists("outliers_extreme_lower_cnt")
    {
        // Count columns (with _cnt suffix)
        new_columns.push("outliers_extreme_lower_cnt".to_string());
        new_column_indices.insert(
            "outliers_extreme_lower_cnt".to_string(),
            new_columns.len() - 1,
        );
        new_columns.push("outliers_mild_lower_cnt".to_string());
        new_column_indices.insert("outliers_mild_lower_cnt".to_string(), new_columns.len() - 1);
        new_columns.push("outliers_normal_cnt".to_string());
        new_column_indices.insert("outliers_normal_cnt".to_string(), new_columns.len() - 1);
        new_columns.push("outliers_mild_upper_cnt".to_string());
        new_column_indices.insert("outliers_mild_upper_cnt".to_string(), new_columns.len() - 1);
        new_columns.push("outliers_extreme_upper_cnt".to_string());
        new_column_indices.insert(
            "outliers_extreme_upper_cnt".to_string(),
            new_columns.len() - 1,
        );
        new_columns.push("outliers_total_cnt".to_string());
        new_column_indices.insert("outliers_total_cnt".to_string(), new_columns.len() - 1);
        // Additional outlier statistics computed during outlier scanning
        new_columns.push("outliers_mean".to_string());
        new_column_indices.insert("outliers_mean".to_string(), new_columns.len() - 1);
        new_columns.push("non_outliers_mean".to_string());
        new_column_indices.insert("non_outliers_mean".to_string(), new_columns.len() - 1);
        new_columns.push("outliers_to_normal_mean_ratio".to_string());
        new_column_indices.insert(
            "outliers_to_normal_mean_ratio".to_string(),
            new_columns.len() - 1,
        );
        new_columns.push("outliers_min".to_string());
        new_column_indices.insert("outliers_min".to_string(), new_columns.len() - 1);
        new_columns.push("outliers_max".to_string());
        new_column_indices.insert("outliers_max".to_string(), new_columns.len() - 1);
        new_columns.push("outliers_range".to_string());
        new_column_indices.insert("outliers_range".to_string(), new_columns.len() - 1);
        // Additional outlier statistics: variance/stddev
        new_columns.push("outliers_stddev".to_string());
        new_column_indices.insert("outliers_stddev".to_string(), new_columns.len() - 1);
        new_columns.push("outliers_variance".to_string());
        new_column_indices.insert("outliers_variance".to_string(), new_columns.len() - 1);
        new_columns.push("non_outliers_stddev".to_string());
        new_column_indices.insert("non_outliers_stddev".to_string(), new_columns.len() - 1);
        new_columns.push("non_outliers_variance".to_string());
        new_column_indices.insert("non_outliers_variance".to_string(), new_columns.len() - 1);
        // Coefficient of variation
        new_columns.push("outliers_cv".to_string());
        new_column_indices.insert("outliers_cv".to_string(), new_columns.len() - 1);
        new_columns.push("non_outliers_cv".to_string());
        new_column_indices.insert("non_outliers_cv".to_string(), new_columns.len() - 1);
        // Outlier percentage
        new_columns.push("outliers_percentage".to_string());
        new_column_indices.insert("outliers_percentage".to_string(), new_columns.len() - 1);
        // Outlier impact
        new_columns.push("outlier_impact".to_string());
        new_column_indices.insert("outlier_impact".to_string(), new_columns.len() - 1);
        new_columns.push("outlier_impact_ratio".to_string());
        new_column_indices.insert("outlier_impact_ratio".to_string(), new_columns.len() - 1);
        // Outlier-to-normal spread ratio
        new_columns.push("outliers_normal_stddev_ratio".to_string());
        new_column_indices.insert(
            "outliers_normal_stddev_ratio".to_string(),
            new_columns.len() - 1,
        );
        // Z-scores of outlier boundaries
        new_columns.push("lower_outer_fence_zscore".to_string());
        new_column_indices.insert(
            "lower_outer_fence_zscore".to_string(),
            new_columns.len() - 1,
        );
        new_columns.push("upper_outer_fence_zscore".to_string());
        new_column_indices.insert(
            "upper_outer_fence_zscore".to_string(),
            new_columns.len() - 1,
        );
    }

    // Add winsorized and trimmed mean columns
    // Check if we can add winsorized/trimmed means
    // Need either Q1/Q3 (default) or percentiles (with --use-percentiles)
    let can_add_winsorized_trimmed = if args.flag_use_percentiles {
        percentiles_idx.is_some()
    } else {
        q1_idx.is_some() && q3_idx.is_some()
    };

    // Determine column names for winsorized/trimmed means
    let (winsorized_col_name, trimmed_col_name) = if args.flag_use_percentiles {
        if let (Some(lower_pct), Some(_upper_pct)) = (lower_percentile, upper_percentile) {
            let pct_str = if lower_pct.fract() == 0.0 {
                format!("{}pct", lower_pct as u32)
            } else {
                format!("{lower_pct}pct")
            };
            (
                format!("winsorized_mean_{pct_str}"),
                format!("trimmed_mean_{pct_str}"),
            )
        } else {
            (
                "winsorized_mean_5pct".to_string(),
                "trimmed_mean_5pct".to_string(),
            )
        }
    } else {
        (
            "winsorized_mean_25pct".to_string(),
            "trimmed_mean_25pct".to_string(),
        )
    };

    if can_add_winsorized_trimmed && !column_exists(winsorized_col_name.as_str()) {
        new_columns.push(winsorized_col_name.clone());
        new_column_indices.insert(winsorized_col_name.clone(), new_columns.len() - 1);
        new_columns.push(trimmed_col_name.clone());
        new_column_indices.insert(trimmed_col_name.clone(), new_columns.len() - 1);
        // Add trimmed/winsorized variance and stddev columns
        let trimmed_stddev_name = trimmed_col_name.replace("mean", "stddev");
        let trimmed_variance_name = trimmed_col_name.replace("mean", "variance");
        let winsorized_stddev_name = winsorized_col_name.replace("mean", "stddev");
        let winsorized_variance_name = winsorized_col_name.replace("mean", "variance");
        new_columns.push(trimmed_stddev_name.clone());
        new_column_indices.insert(trimmed_stddev_name, new_columns.len() - 1);
        new_columns.push(trimmed_variance_name.clone());
        new_column_indices.insert(trimmed_variance_name, new_columns.len() - 1);
        new_columns.push(winsorized_stddev_name.clone());
        new_column_indices.insert(winsorized_stddev_name, new_columns.len() - 1);
        new_columns.push(winsorized_variance_name.clone());
        new_column_indices.insert(winsorized_variance_name, new_columns.len() - 1);
        // Add trimmed/winsorized coefficient of variation
        let trimmed_cv_name = trimmed_col_name.replace("mean", "cv");
        let winsorized_cv_name = winsorized_col_name.replace("mean", "cv");
        new_columns.push(trimmed_cv_name.clone());
        new_column_indices.insert(trimmed_cv_name, new_columns.len() - 1);
        new_columns.push(winsorized_cv_name.clone());
        new_column_indices.insert(winsorized_cv_name, new_columns.len() - 1);
        // Add robust spread ratios (replace "mean" with empty string and clean up double
        // underscores)
        let trimmed_base = trimmed_col_name.replace("mean", "").replace("__", "_");
        let winsorized_base = winsorized_col_name.replace("mean", "").replace("__", "_");
        let trimmed_stddev_ratio_name =
            format!("{}_stddev_ratio", trimmed_base.trim_end_matches('_'));
        let winsorized_stddev_ratio_name =
            format!("{}_stddev_ratio", winsorized_base.trim_end_matches('_'));
        new_columns.push(trimmed_stddev_ratio_name.clone());
        new_column_indices.insert(trimmed_stddev_ratio_name, new_columns.len() - 1);
        new_columns.push(winsorized_stddev_ratio_name.clone());
        new_column_indices.insert(winsorized_stddev_ratio_name, new_columns.len() - 1);
        // Add trimmed/winsorized range
        let trimmed_range_name = trimmed_col_name.replace("mean", "range");
        let winsorized_range_name = winsorized_col_name.replace("mean", "range");
        new_columns.push(trimmed_range_name.clone());
        new_column_indices.insert(trimmed_range_name, new_columns.len() - 1);
        new_columns.push(winsorized_range_name.clone());
        new_column_indices.insert(winsorized_range_name, new_columns.len() - 1);
    }

    if new_columns.is_empty() {
        // Check if any moarstats columns already exist to determine the reason
        let moarstats_columns = [
            "pearson_skewness",
            "range_stddev_ratio",
            "quartile_coefficient_dispersion",
            "mode_zscore",
            "relative_standard_error",
            "min_zscore",
            "max_zscore",
            "median_mean_ratio",
            "iqr_range_ratio",
            "mad_stddev_ratio",
            "kurtosis",
            "bimodality_coefficient",
            "gini_coefficient",
            "atkinson_index",
            "shannon_entropy",
            "normalized_entropy",
            "xsd_type",
            "outliers_extreme_lower_cnt",
        ];

        let any_exist = moarstats_columns.iter().any(|col| column_exists(col))
            || headers.iter().any(|h| h.starts_with("atkinson_index_"));

        if any_exist {
            wwarn!(
                "Warning: No additional stats can be computed. All available additional \
                 statistics have already been added to this stats CSV file."
            );
        } else {
            wwarn!(
                "Warning: No additional stats can be computed with the available base statistics."
            );
            wwarn!(
                "Consider running stats with --everything, or including --quartiles --median \
                 --mode in your --stats-options."
            );
        }
        // If bivariate statistics are not requested, we can return early
        if !args.flag_bivariate {
            return Ok(());
        }
    }

    // Read all records
    let mut records = Vec::new();
    for result in rdr.records() {
        let record = result?;
        records.push(record);
    }

    // Collect fields that need outlier counting and/or winsorized/trimmed means
    let mut fields_to_count: HashMap<String, OutlierFieldInfo> = HashMap::new();
    let needs_outlier_counting = new_column_indices.contains_key("outliers_extreme_lower");
    let needs_winsorized_trimmed = new_column_indices.contains_key(winsorized_col_name.as_str())
        || new_column_indices.contains_key(trimmed_col_name.as_str());

    // Collect fields that need Kurtosis, Gini & Atkinson Index computation
    // (with their precalculated stats)
    let needs_kga = new_column_indices.contains_key("kurtosis")
        || new_column_indices.contains_key("gini_coefficient")
        || new_column_indices.contains_key("atkinson_index");

    // First pass: collect field information from stats records
    if needs_outlier_counting || needs_winsorized_trimmed {
        for record in &records {
            let field_name = field_idx.and_then(|idx| record.get(idx)).unwrap_or("");
            let field_type_str = record.get(type_idx).unwrap_or("");

            // Convert string to enum for efficient comparisons
            let Some(field_type) = FieldType::from_str(field_type_str) else {
                continue;
            };

            if field_name.is_empty() || !field_type.is_numeric_or_date_type() {
                continue;
            }

            // Parse fence values (needed for outlier counting)
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

            // Parse threshold values for winsorization/trimming
            let (lower_threshold, upper_threshold) = if args.flag_use_percentiles {
                // Use percentiles
                if let (Some(percentiles_idx_val), Some(lower_pct), Some(upper_pct)) =
                    (percentiles_idx, lower_percentile, upper_percentile)
                {
                    let percentiles_str = record.get(percentiles_idx_val).unwrap_or("");
                    let lower_pct_str = if lower_pct.fract() == 0.0 {
                        format!("{}", lower_pct as u32)
                    } else {
                        format!("{lower_pct}")
                    };
                    let upper_pct_str = if upper_pct.fract() == 0.0 {
                        format!("{}", upper_pct as u32)
                    } else {
                        format!("{upper_pct}")
                    };

                    let lower_val =
                        parse_percentile_value(percentiles_str, &lower_pct_str, field_type);
                    let upper_val =
                        parse_percentile_value(percentiles_str, &upper_pct_str, field_type);
                    (lower_val, upper_val)
                } else {
                    (None, None)
                }
            } else {
                // Use Q1/Q3
                let q1_val = if field_type.is_date_or_datetime() {
                    q1_idx.and_then(|idx| record.get(idx)).and_then(|s| {
                        let prefer_dmy = util::get_envvar_flag("QSV_PREFER_DMY");
                        parse_date_to_days(s, prefer_dmy)
                    })
                } else {
                    q1_idx
                        .and_then(|idx| record.get(idx))
                        .and_then(parse_float_opt)
                };
                let q3_val = if field_type.is_date_or_datetime() {
                    q3_idx.and_then(|idx| record.get(idx)).and_then(|s| {
                        let prefer_dmy = util::get_envvar_flag("QSV_PREFER_DMY");
                        parse_date_to_days(s, prefer_dmy)
                    })
                } else {
                    q3_idx
                        .and_then(|idx| record.get(idx))
                        .and_then(parse_float_opt)
                };
                (q1_val, q3_val)
            };

            // Determine if we should include this field
            let include_for_outliers = needs_outlier_counting
                && lower_outer_fence.is_some()
                && lower_inner_fence.is_some()
                && upper_inner_fence.is_some()
                && upper_outer_fence.is_some();

            let include_for_winsorized_trimmed =
                needs_winsorized_trimmed && lower_threshold.is_some() && upper_threshold.is_some();

            if include_for_outliers || include_for_winsorized_trimmed {
                // Use default values for fences if not needed
                let lower_outer = lower_outer_fence.unwrap_or(0.0);
                let lower_inner = lower_inner_fence.unwrap_or(0.0);
                let upper_inner = upper_inner_fence.unwrap_or(0.0);
                let upper_outer = upper_outer_fence.unwrap_or(0.0);
                let lower_thresh = lower_threshold.unwrap_or(0.0);
                let upper_thresh = upper_threshold.unwrap_or(0.0);

                // We'll find the column index when we read the CSV
                fields_to_count.insert(
                    field_name.to_string(),
                    OutlierFieldInfo {
                        col_idx: 0, // Will be set when we read CSV headers
                        field_type, // Store enum directly
                        lower_outer,
                        lower_inner,
                        upper_inner,
                        upper_outer,
                        lower_threshold: lower_thresh,
                        upper_threshold: upper_thresh,
                    },
                );
            }
        }
    }

    // Collect fields for Kurtosis, Gini & Atkinson Index computation with their precalculated stats
    let mut fields_for_kga: HashMap<String, KGAFieldInfo> = HashMap::new();
    if needs_kga {
        for record in &records {
            let field_name = field_idx.and_then(|idx| record.get(idx)).unwrap_or("");
            let field_type_str = record.get(type_idx).unwrap_or("");

            // Convert string to enum for efficient comparisons
            let Some(field_type) = FieldType::from_str(field_type_str) else {
                continue;
            };

            if field_name.is_empty() || !field_type.is_numeric_or_date_type() {
                continue;
            }

            // Parse precalculated stats
            let mean_val = mean_idx
                .and_then(|idx| record.get(idx))
                .and_then(parse_float_opt);
            let stddev_val = stddev_idx
                .and_then(|idx| record.get(idx))
                .and_then(parse_float_opt);
            let variance_val = stddev_val.map(|s| s * s); // variance = stddev^2
            let sum_val = sum_idx
                .and_then(|idx| record.get(idx))
                .and_then(parse_float_opt);

            // We'll find the column index when we read the CSV
            fields_for_kga.insert(
                field_name.to_string(),
                KGAFieldInfo {
                    col_idx: 0, // Will be set when we read CSV headers
                    field_type,
                    mean: mean_val,
                    variance: variance_val,
                    sum: sum_val,
                },
            );
        }
    }

    // Count outliers for all fields in a single pass through the original CSV
    let outlier_counts = if fields_to_count.is_empty() {
        HashMap::new()
    } else {
        // Get headers to map field names to column indices
        let mut csv_rdr = ReaderBuilder::new()
            .has_headers(true)
            .from_path(actual_input_path)?;
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
        count_all_outliers(&fields_to_count, actual_input_path, args.flag_jobs)?
    };

    // Compute kurtosis, Gini coefficient & Atkinson Index for all fields
    let kga_stats = if fields_for_kga.is_empty() {
        HashMap::new()
    } else {
        // Get headers to map field names to column indices
        let mut csv_rdr = ReaderBuilder::new()
            .has_headers(true)
            .from_path(actual_input_path)?;
        let csv_headers = csv_rdr.headers()?.clone();

        // Update column indices in fields_for_kga and remove fields not found in CSV
        fields_for_kga.retain(|field_name, field_info| {
            if let Some(col_idx) = csv_headers.iter().position(|h| h == field_name) {
                field_info.col_idx = col_idx;
                true
            } else {
                false
            }
        });

        // Compute Kurtosis, Gini & Atkinson Index (will use sequential processing for correctness)
        compute_all_kga(&fields_for_kga, actual_input_path, args.flag_epsilon)?
    };

    // Compute Shannon Entropy for all fields
    let entropy_stats = if new_column_indices.contains_key("shannon_entropy") {
        compute_all_entropy(actual_input_path)?
    } else {
        HashMap::new()
    };

    let mut stats_config = BivariateStatsConfig::default();
    // Compute bivariate statistics if requested
    // Store field_names for output conversion (indices -> names)
    let mut bivariate_field_names: Option<Vec<String>> = None;
    let bivariate_stats = if args.flag_bivariate {
        // Validate bivariate stats config early
        stats_config = BivariateStatsConfig::from_flag(&args.flag_bivariate_stats)?;

        // Get record count to check for all-unique fields (cardinality == rowcount)
        let record_count: Option<u64> = {
            let actual_input_path_str = actual_input_path
                .to_str()
                .ok_or_else(|| CliError::Other("Invalid input path".to_string()))?
                .to_string();
            let rconfig = Config::new(Some(&actual_input_path_str));
            if let Ok(Some(idx)) = rconfig.indexed() {
                Some(idx.count())
            } else if !rconfig.is_stdin() {
                // Fall back to counting rows if no index
                util::count_rows(&rconfig).ok()
            } else {
                None // Can't get count from stdin
            }
        };

        // Get CSV headers to map field names to column indices
        let mut csv_rdr = ReaderBuilder::new()
            .has_headers(true)
            .from_path(actual_input_path)?;
        let csv_headers = csv_rdr.headers()?.clone();

        // Store field names for index-to-name lookups (used for output and frequency cache)
        let field_names: Vec<String> = csv_headers
            .iter()
            .map(std::string::ToString::to_string)
            .collect();
        bivariate_field_names = Some(field_names.clone());

        // Collect all field pairs for bivariate computation using column indices as keys
        // Using u16 for keys (2 bytes) instead of usize (8 bytes) for better memory efficiency
        // u16 supports up to 65,535 columns, which is more than sufficient for any CSV
        let mut field_pairs: HashMap<(u16, u16), (BivariateFieldInfo, BivariateFieldInfo)> =
            HashMap::new();

        // Collect all numeric/date/string field names from stats CSV
        let stats_field_names: Vec<String> = records
            .iter()
            .filter_map(|r| {
                field_idx
                    .and_then(|idx| r.get(idx))
                    .map(std::string::ToString::to_string)
            })
            .collect();

        for (i, field1_name) in stats_field_names.iter().enumerate() {
            let field1_type_str = records.get(i).and_then(|r| r.get(type_idx)).unwrap_or("");
            let Some(field1_type) = FieldType::from_str(field1_type_str) else {
                continue;
            };

            // Get column index for field1
            let Some(field1_col_idx) = csv_headers.iter().position(|h| h == field1_name) else {
                continue;
            };

            // Extract pre-computed statistics for field1 from stats CSV
            let field1_record = records.get(i);
            let field1_stddev = field1_record
                .and_then(|r| stddev_idx.and_then(|idx| r.get(idx)))
                .and_then(parse_float_opt);
            let field1_variance = field1_record
                .and_then(|r| variance_idx.and_then(|idx| r.get(idx)))
                .and_then(parse_float_opt);
            let field1_cardinality = field1_record
                .and_then(|r| cardinality_idx.and_then(|idx| r.get(idx)))
                .and_then(|s| s.parse::<u64>().ok());

            // Compare with all other fields
            for (j, field2_name) in stats_field_names.iter().enumerate().skip(i + 1) {
                let field2_type_str = records.get(j).and_then(|r| r.get(type_idx)).unwrap_or("");
                let Some(field2_type) = FieldType::from_str(field2_type_str) else {
                    continue;
                };

                // Get column index for field2
                let Some(field2_col_idx) = csv_headers.iter().position(|h| h == field2_name) else {
                    continue;
                };

                // Extract pre-computed statistics for field2 from stats CSV
                let field2_record = records.get(j);
                let field2_stddev = field2_record
                    .and_then(|r| stddev_idx.and_then(|idx| r.get(idx)))
                    .and_then(parse_float_opt);
                let field2_variance = field2_record
                    .and_then(|r| variance_idx.and_then(|idx| r.get(idx)))
                    .and_then(parse_float_opt);
                let field2_cardinality = field2_record
                    .and_then(|r| cardinality_idx.and_then(|idx| r.get(idx)))
                    .and_then(|s| s.parse::<u64>().ok());

                // Filter invalid pairs: skip constant fields (zero variance)
                if let (Some(stddev1), Some(stddev2)) = (field1_stddev, field2_stddev) {
                    if stddev1.abs() < f64::EPSILON || stddev2.abs() < f64::EPSILON {
                        continue; // Skip pairs with constant fields (correlation undefined)
                    }
                } else if let (Some(var1), Some(var2)) = (field1_variance, field2_variance)
                    && (var1.abs() < f64::EPSILON || var2.abs() < f64::EPSILON)
                {
                    continue; // Skip pairs with constant fields (correlation undefined)
                }

                // Filter invalid pairs: skip both-constant pairs (cardinality = 1 for both)
                if let (Some(card1), Some(card2)) = (field1_cardinality, field2_cardinality)
                    && card1 == 1
                    && card2 == 1
                {
                    continue; // Both constant, no meaningful correlation
                }

                // Filter invalid pairs: skip fields with all unique values (cardinality ==
                // rowcount)
                if let Some(rowcount) = record_count
                    && (field1_cardinality.is_some_and(|c| c == rowcount)
                        || field2_cardinality.is_some_and(|c| c == rowcount))
                {
                    continue; // All values are unique, correlations are not meaningful
                }

                // Include pairs where at least one field is numeric/date/string
                // (for mutual information, we want all types)
                if field1_type.is_numeric_or_date_type()
                    || field2_type.is_numeric_or_date_type()
                    || field1_type == FieldType::TString
                    || field2_type == FieldType::TString
                {
                    // Use column indices as keys (cast to u16 for memory efficiency)
                    // col_idx is usize but we store as u16 in the HashMap key
                    field_pairs.insert(
                        (field1_col_idx as u16, field2_col_idx as u16),
                        (
                            BivariateFieldInfo {
                                col_idx:     field1_col_idx,
                                field_type:  field1_type,
                                // mean:        field1_mean,
                                stddev:      field1_stddev,
                                variance:    field1_variance,
                                cardinality: field1_cardinality,
                                // nullcount:   field1_nullcount,
                            },
                            BivariateFieldInfo {
                                col_idx:     field2_col_idx,
                                field_type:  field2_type,
                                // mean:        field2_mean,
                                stddev:      field2_stddev,
                                variance:    field2_variance,
                                cardinality: field2_cardinality,
                                // nullcount:   field2_nullcount,
                            },
                        ),
                    );
                }
            }
        }

        if field_pairs.is_empty() {
            HashMap::new()
        } else {
            // Setup progress bar if requested and not reading from stdin
            let actual_input_path_str = actual_input_path
                .to_str()
                .ok_or_else(|| CliError::Other("Invalid input path".to_string()))?
                .to_string();
            let rconfig_bivariate = Config::new(Some(&actual_input_path_str));
            let show_progress = (args.flag_progressbar || util::get_envvar_flag("QSV_PROGRESSBAR"))
                && !rconfig_bivariate.is_stdin();
            let progress = if show_progress {
                Some(ProgressBar::with_draw_target(
                    Some(0),
                    ProgressDrawTarget::stderr_with_hz(5),
                ))
            } else {
                None
            };

            // Get cardinality threshold (default: 1,000,000)
            let cardinality_threshold = args.flag_cardinality_threshold.or(Some(1_000_000));

            // Log which stats are being computed
            let stats_list: Vec<&str> = [
                stats_config.pearson.then_some("pearson"),
                stats_config.spearman.then_some("spearman"),
                stats_config.kendall.then_some("kendall"),
                stats_config.covariance.then_some("covariance"),
                stats_config.mi.then_some("mi"),
                stats_config.nmi.then_some("nmi"),
            ]
            .into_iter()
            .flatten()
            .collect();
            winfo!(
                "Computing bivariate statistics: {}...",
                stats_list.join(", ")
            );

            let result = compute_all_bivariatestats(
                &field_pairs,
                &field_names,
                actual_input_path,
                progress.as_ref(),
                cardinality_threshold,
                stats_config,
                args.flag_jobs,
            );

            // Clean up progress bar if it was created
            if let Some(pb) = progress {
                pb.finish_and_clear();
            }

            result?
        }
    } else {
        HashMap::new()
    };

    // Write bivariate statistics CSV if computed
    // Always use the original input path for naming, even if we joined datasets
    if args.flag_bivariate && !bivariate_stats.is_empty() {
        let is_joined = temp_joined_path.is_some();
        let bivariate_csv_path = get_bivariate_csv_path(input_path, is_joined)?;
        let mut bivariate_wtr = WriterBuilder::new()
            .has_headers(true)
            .from_path(&bivariate_csv_path)?;

        // Build headers dynamically based on requested stats
        let mut headers = vec!["field1", "field2"];
        if stats_config.pearson {
            headers.push("pearson_correlation");
        }
        if stats_config.spearman {
            headers.push("spearman_correlation");
        }
        if stats_config.kendall {
            headers.push("kendall_tau");
        }
        if stats_config.covariance {
            headers.push("covariance_sample");
            headers.push("covariance_population");
        }
        if stats_config.mi {
            headers.push("mutual_information");
        }
        if stats_config.nmi {
            headers.push("normalized_mutual_information");
        }
        headers.push("n_pairs");

        // Write headers
        bivariate_wtr.write_record(&headers)?;

        // Write bivariate statistics
        // Convert indices to names for output
        let field_names_for_output = bivariate_field_names.as_ref().ok_or_else(|| {
            CliError::Other("Field names not available for bivariate output".to_string())
        })?;

        let mut sorted_pairs: Vec<_> = bivariate_stats.keys().collect();
        sorted_pairs.sort();

        for (idx1, idx2) in sorted_pairs {
            if let Some(stats) = bivariate_stats.get(&(*idx1, *idx2)) {
                // Convert indices to field names for output (u16 -> usize for indexing)
                let field1_name = field_names_for_output
                    .get(*idx1 as usize)
                    .map_or("?", |s| s.as_str());
                let field2_name = field_names_for_output
                    .get(*idx2 as usize)
                    .map_or("?", |s| s.as_str());

                // Build record dynamically based on requested stats
                let mut record: Vec<String> =
                    vec![field1_name.to_string(), field2_name.to_string()];
                if stats_config.pearson {
                    record.push(
                        stats
                            .pearson
                            .map_or(String::new(), |v| util::round_num(v, args.flag_round)),
                    );
                }
                if stats_config.spearman {
                    record.push(
                        stats
                            .spearman
                            .map_or(String::new(), |v| util::round_num(v, args.flag_round)),
                    );
                }
                if stats_config.kendall {
                    record.push(
                        stats
                            .kendall
                            .map_or(String::new(), |v| util::round_num(v, args.flag_round)),
                    );
                }
                if stats_config.covariance {
                    record.push(
                        stats
                            .covariance_sample
                            .map_or(String::new(), |v| util::round_num(v, args.flag_round)),
                    );
                    record.push(
                        stats
                            .covariance_population
                            .map_or(String::new(), |v| util::round_num(v, args.flag_round)),
                    );
                }
                if stats_config.mi {
                    record.push(
                        stats
                            .mutual_information
                            .map_or(String::new(), |v| util::round_num(v, args.flag_round)),
                    );
                }
                if stats_config.nmi {
                    record.push(
                        stats
                            .normalized_mutual_information
                            .map_or(String::new(), |v| util::round_num(v, args.flag_round)),
                    );
                }
                record.push(stats.n_pairs.to_string());

                bivariate_wtr.write_record(&record)?;
            }
        }

        bivariate_wtr.flush()?;
        wwarn!(
            "Wrote bivariate statistics to {}",
            bivariate_csv_path.display()
        );
    }

    // Prepare output
    let output_path: &Path = args.flag_output.as_ref().map_or(&stats_csv_path, Path::new);
    let mut wtr = WriterBuilder::new()
        .has_headers(true)
        .from_path(output_path)?;

    // Write headers with new columns appended
    let mut header_record = headers;
    for col in &new_columns {
        header_record.push_field(col.as_str());
    }
    wtr.write_record(&header_record)?;

    // Process each record
    #[allow(clippy::cast_precision_loss)]
    for record in &records {
        let mut output_record = record.clone();

        // Get field name and type (skip dataset stats rows that might not have proper type)
        let field_name = field_idx.and_then(|idx| record.get(idx)).unwrap_or("");
        let field_type_str = record.get(type_idx).unwrap_or("");

        // Convert string to enum for efficient comparisons
        let field_type_opt = FieldType::from_str(field_type_str);

        // Initialize new_values for all field types (needed for entropy which works for all types)
        let mut new_values = vec![String::new(); new_columns.len()];

        // Compute XSD type for all field types (needs type, min, max)
        if new_column_indices.contains_key("xsd_type") {
            // Extract min and max string values (needed for comprehensive mode and as fallback)
            let min_str = min_idx.and_then(|idx| {
                let s = record.get(idx)?;
                if s.is_empty() { None } else { Some(s) }
            });
            let max_str = max_idx.and_then(|idx| {
                let s = record.get(idx)?;
                if s.is_empty() { None } else { Some(s) }
            });

            // Extract percentile values for fast mode
            let (percentile_values, actual_scan_mode) = if scan_mode == "fast" {
                if let Some(idx) = percentiles_idx {
                    if let Some(percentiles_str) = record.get(idx) {
                        if percentiles_str.is_empty() {
                            // Empty percentile string, fall back to comprehensive
                            (None, "comprehensive")
                        } else {
                            let values = parse_all_percentile_string_values(percentiles_str);
                            if values.is_empty() {
                                // Empty percentile values, fall back to comprehensive
                                (None, "comprehensive")
                            } else {
                                (Some(values), "fast")
                            }
                        }
                    } else {
                        // No percentile string, fall back to comprehensive
                        (None, "comprehensive")
                    }
                } else {
                    // No percentiles column, fall back to comprehensive
                    (None, "comprehensive")
                }
            } else {
                (None, scan_mode)
            };

            // Parse min and max values - they may be strings (for dates) or numbers (for
            // integers/floats)
            let min_val = if let Some(min_idx_val) = min_idx {
                record.get(min_idx_val).and_then(|s| {
                    if s.is_empty() {
                        None
                    } else if field_type_opt.is_some_and(FieldType::is_date_or_datetime) {
                        // For dates, parse as date string
                        let prefer_dmy = util::get_envvar_flag("QSV_PREFER_DMY");
                        parse_date_to_days(s, prefer_dmy)
                    } else {
                        // For integers/floats, parse as number
                        parse_float_opt(s)
                    }
                })
            } else {
                None
            };

            let max_val = if let Some(max_idx_val) = max_idx {
                record.get(max_idx_val).and_then(|s| {
                    if s.is_empty() {
                        None
                    } else if field_type_opt.is_some_and(FieldType::is_date_or_datetime) {
                        // For dates, parse as date string
                        let prefer_dmy = util::get_envvar_flag("QSV_PREFER_DMY");
                        parse_date_to_days(s, prefer_dmy)
                    } else {
                        // For integers/floats, parse as number
                        parse_float_opt(s)
                    }
                })
            } else {
                None
            };

            // Infer XSD type (pass all parameters including scan_mode and percentile_values)
            // Use actual_scan_mode which may have fallen back to comprehensive if percentiles
            // unavailable
            let xsd_type = infer_xsd_type(
                field_type_str,
                min_val,
                max_val,
                field_type_opt,
                actual_scan_mode,
                min_str,
                max_str,
                percentile_values.as_deref(),
            );
            if let Some(idx) = new_column_indices.get("xsd_type") {
                new_values[*idx] = xsd_type;
            }
        }

        // Write Shannon Entropy from pre-computed results (works for all field types)
        if new_column_indices.contains_key("shannon_entropy")
            && !field_name.is_empty()
            && let Some(stats) = entropy_stats.get(field_name)
            && let Some(entropy_val) = stats.entropy
            && let Some(idx) = new_column_indices.get("shannon_entropy")
        {
            new_values[*idx] = util::round_num(entropy_val, args.flag_round);
        }

        // Write Normalized Entropy from pre-computed results (works for all field types)
        if let Some(idx) = new_column_indices.get("normalized_entropy")
            && !field_name.is_empty()
            && let Some(entropy_stats) = entropy_stats.get(field_name)
            && let Some(entropy_val) = entropy_stats.entropy
        {
            let cardinality_val = cardinality_idx
                .and_then(|idx| record.get(idx))
                .and_then(|s| s.parse::<u64>().ok());
            if let Some(val) = compute_normalized_entropy(Some(entropy_val), cardinality_val) {
                new_values[*idx] = util::round_num(val, args.flag_round);
            }
        }

        // Only compute other stats for numeric/date types
        let Some(field_type) = field_type_opt else {
            // For unrecognized types, append new values (entropy already set above)
            for val in new_values {
                output_record.push_field(&val);
            }
            wtr.write_record(&output_record)?;
            continue;
        };

        if field_type.is_numeric_or_date_type() {
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

            // Compute new stats (entropy already computed above for all field types)

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

            // Compute Bimodality Coefficient (requires skewness and kurtosis)
            if let Some(idx) = new_column_indices.get("bimodality_coefficient")
                && !field_name.is_empty()
                && let Some(kga_stats_val) = kga_stats.get(field_name)
                && let Some(kurtosis_val) = kga_stats_val.kurtosis
            {
                let skewness = skewness_idx
                    .and_then(|idx| record.get(idx))
                    .and_then(parse_float_opt);
                if let Some(val) = compute_bimodality_coefficient(skewness, Some(kurtosis_val)) {
                    new_values[*idx] = util::round_num(val, args.flag_round);
                }
            }

            // Get outlier statistics from pre-computed results
            if new_column_indices.contains_key("outliers_extreme_lower_cnt")
                && !field_name.is_empty()
                && let Some(stats) = outlier_counts.get(field_name)
            {
                // Write counts (with _cnt suffix)
                if let Some(idx) = new_column_indices.get("outliers_extreme_lower_cnt") {
                    new_values[*idx] = stats.counts[0].to_string();
                }
                if let Some(idx) = new_column_indices.get("outliers_mild_lower_cnt") {
                    new_values[*idx] = stats.counts[1].to_string();
                }
                if let Some(idx) = new_column_indices.get("outliers_normal_cnt") {
                    new_values[*idx] = stats.counts[2].to_string();
                }
                if let Some(idx) = new_column_indices.get("outliers_mild_upper_cnt") {
                    new_values[*idx] = stats.counts[3].to_string();
                }
                if let Some(idx) = new_column_indices.get("outliers_extreme_upper_cnt") {
                    new_values[*idx] = stats.counts[4].to_string();
                }
                if let Some(idx) = new_column_indices.get("outliers_total_cnt") {
                    new_values[*idx] = stats.counts[5].to_string();
                }

                // Compute means
                let mean_outliers = if stats.counts[5] > 0 {
                    Some(stats.sum_outliers / stats.counts[5] as f64)
                } else {
                    None
                };
                let mean_normal = if stats.counts[2] > 0 {
                    Some(stats.sum_normal / stats.counts[2] as f64)
                } else {
                    None
                };
                let mean_all = if stats.count_all > 0 {
                    Some(stats.sum_all / stats.count_all as f64)
                } else {
                    None
                };

                // Compute outliers variance and stddev once for reuse
                let (variance_outliers, stddev_outliers) = if stats.counts[5] > 1 {
                    let n = stats.counts[5] as f64;
                    let variance = (stats.sum_squares_outliers
                        - (stats.sum_outliers * stats.sum_outliers / n))
                        / (n - 1.0);
                    if variance >= 0.0 {
                        (Some(variance), Some(variance.sqrt()))
                    } else {
                        (None, None)
                    }
                } else {
                    (None, None)
                };

                // Compute and write additional statistics
                if let Some(mean_outliers_val) = mean_outliers {
                    // Mean of outliers
                    if let Some(idx) = new_column_indices.get("outliers_mean") {
                        new_values[*idx] = if field_type.is_date_or_datetime() {
                            days_to_rfc3339(mean_outliers_val, field_type)
                        } else {
                            util::round_num(mean_outliers_val, args.flag_round)
                        };
                    }

                    // Variance and stddev of outliers
                    if let (Some(variance_outliers_val), Some(stddev_outliers_val)) =
                        (variance_outliers, stddev_outliers)
                    {
                        if let Some(idx) = new_column_indices.get("outliers_stddev") {
                            new_values[*idx] =
                                util::round_num(stddev_outliers_val, args.flag_round);
                        }
                        if let Some(idx) = new_column_indices.get("outliers_variance") {
                            new_values[*idx] =
                                util::round_num(variance_outliers_val, args.flag_round);
                        }
                        // Coefficient of variation for outliers
                        if mean_outliers_val.abs() > f64::EPSILON
                            && let Some(idx) = new_column_indices.get("outliers_cv")
                        {
                            let cv = stddev_outliers_val / mean_outliers_val.abs();
                            new_values[*idx] = util::round_num(cv, args.flag_round);
                        }
                    }
                }

                if let Some(mean_normal_val) = mean_normal {
                    // Mean of non-outliers
                    if let Some(idx) = new_column_indices.get("non_outliers_mean") {
                        new_values[*idx] = if field_type.is_date_or_datetime() {
                            days_to_rfc3339(mean_normal_val, field_type)
                        } else {
                            util::round_num(mean_normal_val, args.flag_round)
                        };
                    }

                    // Variance and stddev of non-outliers
                    if stats.counts[2] > 1 {
                        let n = stats.counts[2] as f64;
                        let variance_normal = (stats.sum_squares_normal
                            - (stats.sum_normal * stats.sum_normal / n))
                            / (n - 1.0);
                        if variance_normal >= 0.0 {
                            let stddev_normal = variance_normal.sqrt();
                            if let Some(idx) = new_column_indices.get("non_outliers_stddev") {
                                new_values[*idx] = util::round_num(stddev_normal, args.flag_round);
                            }
                            if let Some(idx) = new_column_indices.get("non_outliers_variance") {
                                new_values[*idx] =
                                    util::round_num(variance_normal, args.flag_round);
                            }
                            // Coefficient of variation for non-outliers
                            if mean_normal_val.abs() > f64::EPSILON
                                && let Some(idx) = new_column_indices.get("non_outliers_cv")
                            {
                                let cv = stddev_normal / mean_normal_val.abs();
                                new_values[*idx] = util::round_num(cv, args.flag_round);
                            }

                            // Outlier-to-normal spread ratio
                            if let Some(stddev_outliers_val) = stddev_outliers
                                && stddev_normal.abs() > f64::EPSILON
                                && let Some(idx) =
                                    new_column_indices.get("outliers_normal_stddev_ratio")
                            {
                                let ratio = stddev_outliers_val / stddev_normal;
                                new_values[*idx] = util::round_num(ratio, args.flag_round);
                            }
                        }
                    }

                    // Outlier-to-normal mean ratio
                    if let Some(mean_outliers_val) = mean_outliers
                        && let Some(idx) = new_column_indices.get("outliers_to_normal_mean_ratio")
                        && mean_normal_val.abs() > f64::EPSILON
                    {
                        let ratio = mean_outliers_val / mean_normal_val;
                        new_values[*idx] = util::round_num(ratio, args.flag_round);
                    }
                }

                // Outlier percentage
                if stats.count_all > 0
                    && let Some(idx) = new_column_indices.get("outliers_percentage")
                {
                    let percentage = (stats.counts[5] as f64 / stats.count_all as f64) * 100.0;
                    new_values[*idx] = util::round_num(percentage, args.flag_round);
                }

                // Outlier impact
                if let (Some(mean_all_val), Some(mean_normal_val)) = (mean_all, mean_normal) {
                    if let Some(idx) = new_column_indices.get("outlier_impact") {
                        let impact = mean_all_val - mean_normal_val;
                        new_values[*idx] = util::round_num(impact, args.flag_round);
                    }
                    if let Some(idx) = new_column_indices.get("outlier_impact_ratio")
                        && mean_normal_val.abs() > f64::EPSILON
                    {
                        let impact = mean_all_val - mean_normal_val;
                        let ratio = impact / mean_normal_val.abs();
                        new_values[*idx] = util::round_num(ratio, args.flag_round);
                    }
                }

                // Z-scores of outlier boundaries
                if let (Some(mean_val), Some(stddev_val)) = (mean, stddev)
                    && stddev_val.abs() > f64::EPSILON
                {
                    if let (Some(lower_outer), Some(idx)) = (
                        lower_outer_fence_idx
                            .and_then(|idx| record.get(idx))
                            .and_then(parse_float_opt),
                        new_column_indices.get("lower_outer_fence_zscore"),
                    ) {
                        let zscore = (lower_outer - mean_val) / stddev_val;
                        new_values[*idx] = util::round_num(zscore, args.flag_round);
                    }
                    if let (Some(upper_outer), Some(idx)) = (
                        upper_outer_fence_idx
                            .and_then(|idx| record.get(idx))
                            .and_then(parse_float_opt),
                        new_column_indices.get("upper_outer_fence_zscore"),
                    ) {
                        let zscore = (upper_outer - mean_val) / stddev_val;
                        new_values[*idx] = util::round_num(zscore, args.flag_round);
                    }
                }

                // Min/Max/Range of outliers
                if let Some(min_outliers) = stats.min_outliers
                    && let Some(idx) = new_column_indices.get("outliers_min")
                {
                    new_values[*idx] = if field_type.is_date_or_datetime() {
                        days_to_rfc3339(min_outliers, field_type)
                    } else {
                        util::round_num(min_outliers, args.flag_round)
                    };
                }
                if let Some(max_outliers) = stats.max_outliers {
                    if let Some(idx) = new_column_indices.get("outliers_max") {
                        new_values[*idx] = if field_type.is_date_or_datetime() {
                            days_to_rfc3339(max_outliers, field_type)
                        } else {
                            util::round_num(max_outliers, args.flag_round)
                        };
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

            // Write winsorized and trimmed means and related statistics
            if (new_column_indices.contains_key(winsorized_col_name.as_str())
                || new_column_indices.contains_key(trimmed_col_name.as_str()))
                && !field_name.is_empty()
                && let Some(stats) = outlier_counts.get(field_name)
            {
                // Compute means
                let winsorized_mean = if stats.winsorized_count > 0 {
                    Some(stats.winsorized_sum / stats.winsorized_count as f64)
                } else {
                    None
                };
                let trimmed_mean = if stats.trimmed_count > 0 {
                    Some(stats.trimmed_sum / stats.trimmed_count as f64)
                } else {
                    None
                };

                // Winsorized mean
                if let Some(winsorized_mean_val) = winsorized_mean
                    && let Some(idx) = new_column_indices.get(winsorized_col_name.as_str())
                {
                    new_values[*idx] = if field_type.is_date_or_datetime() {
                        days_to_rfc3339(winsorized_mean_val, field_type)
                    } else {
                        util::round_num(winsorized_mean_val, args.flag_round)
                    };
                }

                // Winsorized variance and stddev
                if let Some(winsorized_mean_val) = winsorized_mean
                    && stats.winsorized_count > 1
                {
                    let n = stats.winsorized_count as f64;
                    let winsorized_variance = (stats.sum_squares_winsorized
                        - (stats.winsorized_sum * stats.winsorized_sum / n))
                        / (n - 1.0);
                    if winsorized_variance >= 0.0 {
                        let winsorized_stddev = winsorized_variance.sqrt();
                        let winsorized_stddev_name = winsorized_col_name.replace("mean", "stddev");
                        let winsorized_variance_name =
                            winsorized_col_name.replace("mean", "variance");
                        if let Some(idx) = new_column_indices.get(&winsorized_stddev_name) {
                            new_values[*idx] = util::round_num(winsorized_stddev, args.flag_round);
                        }
                        if let Some(idx) = new_column_indices.get(&winsorized_variance_name) {
                            new_values[*idx] =
                                util::round_num(winsorized_variance, args.flag_round);
                        }
                        // Winsorized coefficient of variation
                        if winsorized_mean_val.abs() > f64::EPSILON {
                            let winsorized_cv_name = winsorized_col_name.replace("mean", "cv");
                            if let Some(idx) = new_column_indices.get(&winsorized_cv_name) {
                                let cv = winsorized_stddev / winsorized_mean_val.abs();
                                new_values[*idx] = util::round_num(cv, args.flag_round);
                            }
                        }
                        // Winsorized stddev ratio
                        if let Some(stddev_val) = stddev
                            && stddev_val.abs() > f64::EPSILON
                        {
                            let winsorized_base =
                                winsorized_col_name.replace("mean", "").replace("__", "_");
                            let winsorized_stddev_ratio_name =
                                format!("{}_stddev_ratio", winsorized_base.trim_end_matches('_'));
                            if let Some(idx) = new_column_indices.get(&winsorized_stddev_ratio_name)
                            {
                                let ratio = winsorized_stddev / stddev_val;
                                new_values[*idx] = util::round_num(ratio, args.flag_round);
                            }
                        }
                    }
                }

                // Winsorized range
                if let (Some(min_winsorized), Some(max_winsorized)) =
                    (stats.min_winsorized, stats.max_winsorized)
                {
                    let winsorized_range_name = winsorized_col_name.replace("mean", "range");
                    if let Some(idx) = new_column_indices.get(&winsorized_range_name) {
                        let range = max_winsorized - min_winsorized;
                        new_values[*idx] = util::round_num(range, args.flag_round);
                    }
                }

                // Trimmed mean
                if let Some(trimmed_mean_val) = trimmed_mean
                    && let Some(idx) = new_column_indices.get(trimmed_col_name.as_str())
                {
                    new_values[*idx] = if field_type.is_date_or_datetime() {
                        days_to_rfc3339(trimmed_mean_val, field_type)
                    } else {
                        util::round_num(trimmed_mean_val, args.flag_round)
                    };
                }

                // Trimmed variance and stddev
                if let Some(trimmed_mean_val) = trimmed_mean
                    && stats.trimmed_count > 1
                {
                    let n = stats.trimmed_count as f64;
                    let trimmed_variance = (stats.sum_squares_trimmed
                        - (stats.trimmed_sum * stats.trimmed_sum / n))
                        / (n - 1.0);
                    if trimmed_variance >= 0.0 {
                        let trimmed_stddev = trimmed_variance.sqrt();
                        let trimmed_stddev_name = trimmed_col_name.replace("mean", "stddev");
                        let trimmed_variance_name = trimmed_col_name.replace("mean", "variance");
                        if let Some(idx) = new_column_indices.get(&trimmed_stddev_name) {
                            new_values[*idx] = util::round_num(trimmed_stddev, args.flag_round);
                        }
                        if let Some(idx) = new_column_indices.get(&trimmed_variance_name) {
                            new_values[*idx] = util::round_num(trimmed_variance, args.flag_round);
                        }
                        // Trimmed coefficient of variation
                        if trimmed_mean_val.abs() > f64::EPSILON {
                            let trimmed_cv_name = trimmed_col_name.replace("mean", "cv");
                            if let Some(idx) = new_column_indices.get(&trimmed_cv_name) {
                                let cv = trimmed_stddev / trimmed_mean_val.abs();
                                new_values[*idx] = util::round_num(cv, args.flag_round);
                            }
                        }
                        // Trimmed stddev ratio
                        if let Some(stddev_val) = stddev
                            && stddev_val.abs() > f64::EPSILON
                        {
                            let trimmed_base =
                                trimmed_col_name.replace("mean", "").replace("__", "_");
                            let trimmed_stddev_ratio_name =
                                format!("{}_stddev_ratio", trimmed_base.trim_end_matches('_'));
                            if let Some(idx) = new_column_indices.get(&trimmed_stddev_ratio_name) {
                                let ratio = trimmed_stddev / stddev_val;
                                new_values[*idx] = util::round_num(ratio, args.flag_round);
                            }
                        }
                    }
                }

                // Trimmed range
                if let (Some(min_trimmed), Some(max_trimmed)) =
                    (stats.min_trimmed, stats.max_trimmed)
                {
                    let trimmed_range_name = trimmed_col_name.replace("mean", "range");
                    if let Some(idx) = new_column_indices.get(&trimmed_range_name) {
                        let range = max_trimmed - min_trimmed;
                        new_values[*idx] = util::round_num(range, args.flag_round);
                    }
                }
            }

            // Write Kurtosis, Gini & Atkinson Index from pre-computed results
            if (new_column_indices.contains_key("kurtosis")
                || new_column_indices.contains_key("gini_coefficient")
                || new_column_indices.contains_key(&atkinson_index_col_name))
                && !field_name.is_empty()
                && let Some(stats) = kga_stats.get(field_name)
            {
                // Kurtosis
                if let Some(kurtosis_val) = stats.kurtosis
                    && let Some(idx) = new_column_indices.get("kurtosis")
                {
                    new_values[*idx] = util::round_num(kurtosis_val, args.flag_round);
                }

                // Gini coefficient
                if let Some(gini_val) = stats.gini_coefficient
                    && let Some(idx) = new_column_indices.get("gini_coefficient")
                {
                    new_values[*idx] = util::round_num(gini_val, args.flag_round);
                }

                // Atkinson Index
                if let Some(atkinson_val) = stats.atkinson_index
                    && let Some(idx) = new_column_indices.get(&atkinson_index_col_name)
                {
                    new_values[*idx] = util::round_num(atkinson_val, args.flag_round);
                }
            }
        }
        // Append all new values to record
        for val in new_values {
            output_record.push_field(&val);
        }

        wtr.write_record(&output_record)?;
    }

    wtr.flush()?;

    winfo!(
        "Added {} additional statistics columns to {}",
        new_columns.len(),
        output_path.display()
    );
    winfo!("Elapsed: {:.2}s", start_time.elapsed().as_secs_f64());

    // Clean up temporary joined file if it was created
    if let Some(ref temp_path) = temp_joined_path
        && temp_path.exists()
        && let Err(e) = fs::remove_file(temp_path)
    {
        wwarn!(
            "Failed to remove temporary joined file {}: {}",
            temp_path.display(),
            e
        );
    }

    Ok(())
}
