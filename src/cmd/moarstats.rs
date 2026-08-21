static USAGE: &str = r#"
Add dozens of additional statistics, including extended outlier, robust & bivariate
statistics to an existing stats CSV file. It also maps the field type to the most specific
W3C XML Schema Definition (XSD) datatype (https://www.w3.org/TR/xmlschema-2/).

IMPORTANT:
  The `moarstats` command is designed to be run AFTER the `stats` command, as it relies
  on the baseline statistics computed by `stats` to calculate "moar" statistics.

The `moarstats` command extends an existing stats CSV file (created by the `stats` command)
by computing "moar" (https://www.dictionary.com/culture/slang/moar) statistics that can be
derived from existing stats columns and by scanning the original CSV file.

It looks for the `<FILESTEM>.stats.csv` file for a given CSV input. If the stats CSV file
does not exist, it will first run the `stats` command with configurable options to establish
the baseline stats, to which it will add more stats columns.

If the `.stats.csv` file is found, it will skip running stats and just append the additional
stats columns.

Currently computes the following 25 additional univariate statistics:
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
11. Trimean: (Q1 + 2*median + Q3) / 4
    Tukey's trimean - a robust estimator of central tendency combining the median
    with the midhinge. More robust than mean, more efficient than median alone.
    https://en.wikipedia.org/wiki/Trimean
12. Midhinge: (Q1 + Q3) / 2
    Midpoint of the middle 50% of data. A robust central tendency measure
    that complements the mean and median.
    https://en.wikipedia.org/wiki/Midhinge
13. Robust CV: MAD / |median|
    Robust Coefficient of Variation using MAD and the magnitude of the median.
    Always non-negative. Resistant to outliers, useful for comparing variability.
    https://en.wikipedia.org/wiki/Robust_measures_of_scale
14. Kurtosis: Measures the "tailedness" of the distribution (excess kurtosis).
    Positive values indicate heavy tails, negative values indicate light tails.
    Values near 0 indicate a normal distribution.
    Requires --advanced flag.
    https://en.wikipedia.org/wiki/Kurtosis
15. Bimodality Coefficient: Measures whether a distribution has two modes (peaks) or is unimodal.
    BC < 0.555 indicates unimodal, BC >= 0.555 indicates bimodal/multimodal.
    Computed as (skewness² + 1) / (kurtosis + 3).
    Requires --advanced flag (needs skewness from base stats and kurtosis from --advanced flag).
    https://en.wikipedia.org/wiki/Bimodality
16. Jarque-Bera Test: (n/6) * (S² + K²/4)
    Standard test for normality using skewness and kurtosis.
    Also computes jarque_bera_pvalue (from chi-squared distribution with 2 df).
    Low p-values (< 0.05) indicate the data is NOT normally distributed.
    Requires --advanced flag (needs kurtosis).
    https://en.wikipedia.org/wiki/Jarque%E2%80%93Bera_test
17. Gini Coefficient: Measures inequality/dispersion in the distribution.
    Values range from 0 (perfect equality) to 1 (maximum inequality).
    Requires --advanced flag.
    https://en.wikipedia.org/wiki/Gini_coefficient
18. Atkinson Index: Measures inequality in the distribution with a sensitivity parameter.
    Values range from 0 (perfect equality) to 1 (maximum inequality).
    The Atkinson Index is a more general form of the Gini coefficient that allows for
    different sensitivity to inequality. Sensitivity is configurable via --epsilon.
    Requires --advanced flag.
    https://en.wikipedia.org/wiki/Atkinson_index
19. Theil Index: (1/n) * Σ((x_i / mean) * ln(x_i / mean))
    Measures inequality/concentration. Unlike Gini, it is decomposable into
    within-group and between-group components. Only computed for positive values.
    Requires --advanced flag.
    https://en.wikipedia.org/wiki/Theil_index
20. Mean Absolute Deviation (from mean): (1/n) * Σ|x_i - mean|
    Average absolute distance from the mean. Different from MAD (which uses median).
    Less robust but more statistically efficient than MAD.
    Requires --advanced flag.
21. Shannon Entropy: Measures the information content/uncertainty in the distribution.
    Higher values indicate more diversity, lower values indicate more concentration.
    Values range from 0 (all values identical) to log2(n) where n is the number of unique values.
    Requires --advanced flag.
    https://en.wikipedia.org/wiki/Entropy_(information_theory)
22. Normalized Entropy: Normalized version of Shannon Entropy scaled to [0, 1].
    Values range from 0 (all values identical) to 1 (all values equally distributed).
    Computed as shannon_entropy / log2(cardinality).
    Requires shannon_entropy (from --advanced flag) and cardinality (from base stats).
23. Simpson's Diversity Index: 1 - Σ(p_i²)
    Probability that two randomly chosen values are different.
    Ranges from 0 (all identical) to 1 (all unique). More intuitive than entropy.
    Requires --advanced flag (computed alongside entropy from frequency data).
    https://en.wikipedia.org/wiki/Diversity_index#Simpson_index
24. Winsorized Mean: Replaces values below/above thresholds with threshold values, then computes mean.
    All values are included in the calculation, but extreme values are capped at thresholds.
    https://en.wikipedia.org/wiki/Winsorized_mean
    Also computes (<PCT> is the threshold suffix of the mean column, e.g. 25pct or 5pct):
    winsorized_stddev_<PCT>, winsorized_variance_<PCT>, winsorized_cv_<PCT>,
    winsorized_range_<PCT>, and winsorized_<PCT>_stddev_ratio
    (winsorized stddev / overall stddev). Note the ratio column interpolates <PCT>
    before _stddev_ratio, unlike the others.
25. Trimmed Mean: Excludes values outside thresholds, then computes mean.
    Only values within thresholds are included in the calculation.
    https://en.wikipedia.org/wiki/Truncated_mean
    Also computes (<PCT> is the threshold suffix of the mean column, e.g. 25pct or 5pct):
    trimmed_stddev_<PCT>, trimmed_variance_<PCT>, trimmed_cv_<PCT>, trimmed_range_<PCT>,
    and trimmed_<PCT>_stddev_ratio (trimmed stddev / overall stddev). Note the ratio
    column interpolates <PCT> before _stddev_ratio, unlike the others.
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

The `moarstats` command also computes the following 7 bivariate statistics:
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
 7. Theil's U (uncertainty coefficient)
    Directed measure of how much knowing one field reduces uncertainty about the other.
    Asymmetric, so two columns are emitted: u_field2_given_field1 and u_field1_given_field2.
    Values range from 0 (no reduction) to 1 (fully determined).
    Selected with `u` in --bivariate-stats (or via "all").
    https://en.wikipedia.org/wiki/Uncertainty_coefficient

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

Non-finite numeric tokens ("NaN", "Infinity", "-Infinity", and their case variants) are
excluded from moarstats computations — the parser in moarstats filters them out before they
reach correlation, variance and mean calculations, preventing a single bad cell from silently
poisoning the results. Note that the baseline `stats` command may still count these tokens
as Float observations, so the `type`/`null_count` columns in `<FILESTEM>.stats.csv` are not
affected by this filter.

Examples:

  # Add moar stats to existing stats file
  qsv moarstats data.csv

  # Generate baseline stats first with custom options, then add moar stats
  qsv moarstats data.csv --stats-options "--everything --infer-dates"

  # Compute bivariate statistics between fields
  qsv moarstats data.csv --bivariate

  # Compute even more bivariate statistics
  qsv moarstats data.csv --bivariate --bivariate-stats pearson,spearman,kendall,mi,nmi,covariance

  # Join multiple datasets and compute bivariate statistics
  qsv moarstats data.csv --bivariate --join-inputs customers.csv,products.csv --join-keys cust_id,prod_id

  # Join multiple datasets and compute bivariate statistics with different join type
  qsv moarstats data.csv --bivariate --join-inputs customers.csv,products.csv --join-keys cust_id,prod_id --join-type left

For more examples, see https://github.com/dathere/qsv/blob/master/tests/test_moarstats.rs.
See also https://github.com/dathere/qsv/wiki/Aggregation-and-Statistics#moarstats

Usage:
    qsv moarstats [options] [<input>]
    qsv moarstats --help

moarstats options:
    --advanced             Compute Kurtosis, Shannon Entropy, Bimodality Coefficient,
                           Jarque-Bera, Gini Coefficient, Atkinson Index, Theil Index,
                           Mean Absolute Deviation, and Simpson's Diversity Index.
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
                           [default: --infer-dates --infer-boolean --cardinality --mode --mad --quartiles --percentiles --force --stats-jsonl]
    --round <n>            Round statistics to <n> decimal places. Rounding follows
                           Midpoint Nearest Even (Bankers Rounding) rule.
                           [default: 4]
    --use-percentiles      Use percentiles instead of Q1/Q3 for winsorization/trimming.
                           Requires percentiles to be computed in the stats CSV.
   --pct-thresholds <arg>  Comma-separated percentile pair (e.g., "10,90") to use
                           for winsorization/trimming when --use-percentiles is set.
                           Both values must truncate to whole percentiles between
                           1 and 100, and lower < upper. The thresholds are
                           automatically merged into the --percentile-list of the
                           stats run, and an existing stats CSV computed with a
                           percentile list that lacks them is recomputed.
                           [default: 5,95]
 --xsd-gdate-scan <mode>   Gregorian XSD date type detection mode.
                           "quick": Fast detection using min/max values.
                                    Produces types with ?? suffix (less confident).
                           "thorough": Comprehensive detection checking all percentile values.
                                     Slower but ensures all values match the pattern.
                                     Produces types with ? suffix (more confident).
                           [default: quick]

                           BIVARIATE STATISTICS OPTIONS:
    -B, --bivariate        Enable bivariate statistics computation.
                           Requires indexed CSV file (index will be auto-created if missing).
                           Computes pairwise correlations, covariances, mutual information, and
                           normalized mutual information between columns. The bivariate statistics
                           are saved to a separate file in the same directory as the input:
                           <FILESTEM>.stats.bivariate.csv.
    -S, --bivariate-stats <stats>
                           Comma-separated list of bivariate statistics to compute.
                           Options: pearson, spearman, kendall, covariance, mi (mutual information),
                           nmi (normalized mutual information), u (Theil's directed uncertainty
                           coefficient; emits u_field2_given_field1 and u_field1_given_field2)
                           Use "all" to compute all statistics or "fast" to compute only
                           pearson & covariance, which is much faster as it doesn't require storing
                           all values and uses streaming algorithms.
                           [default: fast]
    -C, --cardinality-threshold <n>
                           Skip mutual information (mi/nmi/u) for field pairs where either
                           field's cardinality exceeds this threshold. Such pairs also skip
                           building their joint-frequency table, which is the dominant memory
                           cost of --bivariate-stats all.
                           Defaults to half the row count, floored at 1000, so it stays inert
                           on small inputs and scales with large ones. Mutual information
                           between near-unique columns saturates at log(n) and is noise
                           regardless of how efficiently it is computed.
    --bivariate-batch <n>  Process at most <n> field pairs per pass over the input,
                           bounding peak memory at the cost of extra passes.
                           Peak memory is otherwise O(columns^2) regardless of row
                           count - a 160-column, 100k-row (60 MB) input needs ~21 GiB
                           with mi/nmi/u enabled. Extra passes are cheap, so prefer
                           the largest <n> that fits.
                           Only applies to indexed input with >= 10,000 rows.
                           Set to 0 to process all pairs in one pass.
                           [default: 0]
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

use core::hint::cold_path;
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::Arc,
    time::Instant,
};

use crossbeam_channel;
use csv::{ReaderBuilder, StringRecord, WriterBuilder};
use foldhash::{HashMap, HashMapExt};
use indexmap::{IndexMap, IndexSet};
use indicatif::{HumanCount, ProgressBar, ProgressDrawTarget, ProgressStyle};
use qsv_dateparser::parse_with_preference;
use rayon::prelude::*;
use serde::Deserialize;
use simdutf8::basic::from_utf8;
use stats::{atkinson, gini, kurtosis};
use threadpool::ThreadPool;

use crate::{CliError, CliResult, config::Config, regex_oncelock, util};

/// Minimum record count before parallel processing is worthwhile for outliers and
/// bivariate stats. Below this, the scheduling overhead outweighs the speedup, so
/// fall back to the sequential path.
const PARALLEL_THRESHOLD: usize = 10_000;

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
    flag_bivariate_batch:       usize,
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
    u:          bool, // directed uncertainty coefficient (Theil's U), both directions
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
                "u" | "uncertainty" | "uncertainty_coefficient" | "uncertainty-coefficient" => {
                    config.u = true;
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
                 normalized_mutual_information or normalized-mutual-information), u (or \
                 uncertainty or uncertainty_coefficient), fast, all",
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
            && !config.u
        {
            return fail_clierror!(
                "No valid bivariate statistics specified. Valid options are: pearson, spearman, \
                 kendall, covariance (or cov), mi (or mutual_information or mutual-information), \
                 nmi (or normalized_mutual_information or normalized-mutual-information), u (or \
                 uncertainty or uncertainty_coefficient), fast, all"
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
            u:          true,
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
        self.mi || self.nmi || self.u
    }
}

/// Get the absolute stats CSV file path for a given input CSV path.
/// Delegates to the shared implementation in `util::get_stats_csv_path`.
fn get_stats_csv_path(input_path: &Path) -> CliResult<PathBuf> {
    util::get_stats_csv_path(input_path)
}

/// Get the absolute bivariate CSV file path for a given input CSV path
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
    let result = parent.join(bivariate_filename);
    if result.is_absolute() {
        Ok(result)
    } else {
        Ok(std::env::current_dir()?.join(result))
    }
}

fn join_datasets_internal(
    primary_input: &Path,
    additional_inputs: &[String],
    join_keys: &[String],
    join_type: &str,
) -> CliResult<(PathBuf, Vec<String>)> {
    use std::collections::HashSet;

    use tempfile::TempPath;

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

    // Create temporary file for joined output with .csv extension.
    //
    // We use `into_temp_path().keep()` instead of holding a `NamedTempFile`
    // and `drop`ping it. NamedTempFile's `Drop` deletes the file from disk,
    // which leaves a dangling reservation that the spawned `qsv join`
    // re-creates with O_CREAT. The previous "drop to close" pattern was
    // misleading — the path was free, not just closed. `keep()` persists the
    // path as a normal file so the caller owns its lifetime.
    let temp_dir =
        crate::config::TEMP_FILE_DIR.get_or_init(|| tempfile::TempDir::new().unwrap().keep());
    let temp_path = tempfile::Builder::new()
        .suffix(".csv")
        .tempfile_in(temp_dir)?
        .into_temp_path()
        .keep()
        .map_err(|e| CliError::Other(format!("Failed to persist join temp path: {e}")))?;

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

    // Resolve the executable path once before the loop
    let qsv_path = env::current_exe()
        .map_err(|e| CliError::Other(format!("Failed to get current executable path: {e:?}")))?
        .to_string_lossy()
        .to_string();

    // Drop-guard collection: this Vec is intentionally never read — its
    // ONLY purpose is to keep each intermediate `TempPath` alive until the
    // function returns, at which point each entry's `Drop` removes its
    // file from disk. `TempPath` holds no writable handle (so `qsv join`'s
    // `O_CREAT|O_TRUNC` open still works) but it does own the path's
    // lifetime. Without this Vec, intermediate temp files would either be
    // deleted mid-loop (if we let `TempPath` drop after each iteration) or
    // accumulate forever in `TEMP_FILE_DIR` (if we called `.keep()`).
    // DO NOT remove the Vec to silence `collection_is_never_read` — the
    // Drop side-effect is load-bearing.
    #[allow(clippy::collection_is_never_read)]
    let mut intermediate_temps: Vec<TempPath> =
        Vec::with_capacity(additional_inputs.len().saturating_sub(1));

    // Header of the final joined CSV, captured from the last join step's
    // already-validated read. Returned to the caller so downstream stats
    // coverage checks can validate against a trusted header instead of an
    // independent re-read of the joined temp file (which, under heavy
    // parallel load, has been observed to come back short).
    let mut final_joined_headers: Vec<String> = Vec::new();

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
            // Last join - use final temp path (kept; caller owns lifetime).
            temp_path_str.clone()
        } else {
            // Intermediate join - create another temp file with .csv
            // extension. We retain the TempPath (not .keep()) in
            // intermediate_temps so the file is auto-deleted when this
            // function returns, preventing accumulation in TEMP_FILE_DIR.
            let intermediate = tempfile::Builder::new()
                .suffix(".csv")
                .tempfile_in(temp_dir)?
                .into_temp_path();
            let s = intermediate
                .to_str()
                .ok_or_else(|| CliError::Other("Invalid intermediate temp path".to_string()))?
                .to_string();
            intermediate_temps.push(intermediate);
            s
        };
        // qsv `join` writes the joined CSV to stdout. Redirect that stdout
        // straight into a file the PARENT opens and owns, instead of the
        // old `--output <path>` round-trip where the child created and
        // closed the file and the parent then re-opened it blind by path.
        // With a parent-owned handle we can `sync_all()` a descriptor we
        // KNOW refers to the finished file before any follow-up subprocess
        // opens the path — closing the read-after-write window that
        // intermittently produced short reads / silent "primary-only"
        // joined output under heavy parallel CI load.
        let output_path = Path::new(&output_path_str);
        let join_out_file = fs::File::create(output_path).map_err(|e| {
            CliError::Other(format!(
                "Failed to create join output file ({}): {e}",
                output_path.display()
            ))
        })?;
        let join_out_for_child = join_out_file
            .try_clone()
            .map_err(|e| CliError::Other(format!("Failed to clone join output handle: {e}")))?;

        // Construct join command directly since it doesn't fit run_qsv_cmd pattern
        // (join takes two input files, not one)
        let mut cmd = Command::new(&qsv_path);
        cmd.arg("join")
            .args(&args)
            .stdout(Stdio::from(join_out_for_child))
            .stderr(Stdio::piped());

        let child = cmd
            .spawn()
            .map_err(|e| CliError::Other(format!("Error while spawning join command: {e:?}")))?;
        let output = child
            .wait_with_output()
            .map_err(|e| CliError::Other(format!("Error while executing join command: {e:?}")))?;

        if !output.status.success() {
            return fail_clierror!(
                "Command join failed: Output {{ status: {:?}, stderr: {:?} }}",
                output.status,
                String::from_utf8_lossy(&output.stderr)
            );
        }

        // The child has exited, so every byte it produced has gone through
        // our cloned handle. fsync the parent's own descriptor — guaranteed
        // to refer to the fully-written file — and close it before the
        // header is read or the next subprocess opens the path.
        join_out_file.sync_all().map_err(|e| {
            CliError::Other(format!(
                "Failed to sync join output ({}): {e}",
                output_path.display()
            ))
        })?;
        drop(join_out_file);

        // Validate that the joined output's header contains every column
        // from the secondary input. qsv's `join` (without --cross or merge
        // flags, which join_datasets_internal never passes — it only uses
        // --left/--right/--full or default inner) preserves the union of
        // both inputs' columns. If a column from the secondary is missing,
        // the join produced silently corrupt output and we must fail loudly
        // rather than feeding it to downstream stats/bivariate computation.
        let mut joined_rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_path(output_path)
            .map_err(|e| {
                CliError::Other(format!(
                    "Failed to open joined output to validate header ({}): {e}",
                    output_path.display()
                ))
            })?;
        let joined_headers: Vec<String> = joined_rdr
            .headers()
            .map_err(|e| CliError::Other(format!("Failed to read joined header: {e}")))?
            .iter()
            .map(std::string::ToString::to_string)
            .collect();
        drop(joined_rdr);

        // Read through `Config`, not a raw `from_path`: a special-format secondary
        // (.gz/.zip/.parquet/...) is only decompressed on Config's read path, so a raw
        // open would parse the compressed container and fail with "invalid utf-8 near
        // byte index 0". Config also picks up the inner file's real delimiter (e.g. a
        // .tsv inside a .zip). The `qsv join` subprocess above already handles these.
        let additional_input_owned = additional_input.to_string();
        let mut additional_rdr = Config::new(Some(&additional_input_owned))
            .reader()
            .map_err(|e| {
                CliError::Other(format!(
                    "Failed to open secondary input to validate header ({additional_input}): {e}"
                ))
            })?;
        let additional_headers: Vec<String> = additional_rdr
            .headers()
            .map_err(|e| CliError::Other(format!("Failed to read secondary header: {e}")))?
            .iter()
            .map(std::string::ToString::to_string)
            .collect();
        drop(additional_rdr);

        // O(1) membership check via HashSet — for wide CSVs and multi-join
        // chains this avoids an O(n*m) scan per iteration.
        let joined_set: HashSet<&str> = joined_headers.iter().map(String::as_str).collect();
        for h in &additional_headers {
            if !joined_set.contains(h.as_str()) {
                return fail_clierror!(
                    "Joined output header missing column from {additional_input}: expected to \
                     find {h:?} among {additional_headers:?}, got {joined_headers:?}"
                );
            }
        }

        let size = std::fs::metadata(output_path).map_or(0, |m| m.len());
        // Keep info-level output compact (column count + size). The full
        // header vector can be very large/noisy on wide CSVs and is also
        // slow to format — log it at debug only.
        log::info!(
            "Join step {i}: produced {} cols, {size} bytes",
            joined_headers.len()
        );
        log::debug!("Join step {i} header: {joined_headers:?}");

        // Update for next iteration
        final_joined_headers.clone_from(&joined_headers);
        current_input = output_path_str;
        current_key.clone_from(next_key);
    }

    Ok((temp_path, final_joined_headers))
}

/// Returns true if the given `qsv stats` option string contains an output
/// redirection flag (`-o`/`--output`).
///
/// A plain `token.starts_with("-o")` check misses docopt-style clustered
/// short options: `-Eo joined_stats.csv` expands to `-E -o joined_stats.csv`,
/// so the stats CSV would still be redirected away from the captured stdout.
///
/// The no-argument short flags accepted by `qsv stats` are taken from its
/// USAGE in `src/cmd/stats.rs` (`-E`, `-h`, `-n`). When scanning a clustered
/// short-option token, we stop at the first argument-taking option, since the
/// remainder of the token is that option's value (e.g. `-so` selects a column
/// literally named `o` — it is *not* `-s -o`). If `qsv stats` ever gains a new
/// no-argument short flag, add it here so clusters ending in `-o` stay caught.
fn stats_options_redirect_output(stats_options: &str) -> bool {
    // no-argument short flags accepted by `qsv stats`
    const STATS_SHORT_FLAGS: [char; 3] = ['E', 'h', 'n'];

    stats_options.split_whitespace().any(|token| {
        if let Some(long) = token.strip_prefix("--") {
            return long == "output" || long.starts_with("output=");
        }
        if let Some(short) = token.strip_prefix('-') {
            for ch in short.chars() {
                if ch == 'o' {
                    return true;
                }
                if !STATS_SHORT_FLAGS.contains(&ch) {
                    // argument-taking (or unknown) short option: the rest of
                    // the token is its value, so any `o` here is not -o/--output
                    break;
                }
            }
        }
        false
    })
}

/// Merges the required winsorization/trimming percentiles into an existing
/// `--percentile-list` value. Returns `Some(merged)` when the list had to be
/// extended, `None` when both percentiles are already covered (so a
/// caller-supplied list is left byte-for-byte untouched).
///
/// Membership is tested on the truncated integer percentile, matching what
/// `stats` actually computes and labels. The special values
/// `deciles`/`quintiles` are expanded exactly as `stats` expands them.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn merge_percentile_list(existing: &str, lower: u32, upper: u32) -> Option<String> {
    let expanded = match existing.to_lowercase().as_str() {
        "deciles" => "10,20,30,40,50,60,70,80,90",
        "quintiles" => "20,40,60,80",
        _ => existing,
    };

    let entries: Vec<&str> = expanded
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();
    let covered = |pct: u32| {
        entries.iter().any(|e| {
            matches!(
                fast_float2::parse::<f64, &[u8]>(e.as_bytes()),
                Ok(v) if v.is_finite() && v >= 0.0 && v.trunc() as u32 == pct
            )
        })
    };

    let mut additions: Vec<u32> = Vec::with_capacity(2);
    if !covered(lower) {
        additions.push(lower);
    }
    if !covered(upper) {
        additions.push(upper);
    }
    if additions.is_empty() {
        return None;
    }

    let mut merged: Vec<String> = entries.iter().map(ToString::to_string).collect();
    merged.extend(additions.iter().map(ToString::to_string));
    // sort numerically so the percentiles cell reads in order; entries that
    // don't parse (which `stats` will reject anyway) sort last
    merged.sort_by(|a, b| {
        let pa = fast_float2::parse::<f64, &[u8]>(a.as_bytes()).unwrap_or(f64::MAX);
        let pb = fast_float2::parse::<f64, &[u8]>(b.as_bytes()).unwrap_or(f64::MAX);
        pa.partial_cmp(&pb).unwrap_or(std::cmp::Ordering::Equal)
    });
    Some(merged.join(","))
}

/// Builds the argument vector for the `qsv stats` subprocess from
/// `--stats-options`, ensuring that when `--use-percentiles` is active the
/// percentile list `stats` computes includes the requested `--pct-thresholds`.
///
/// Without this, `stats` computed only its default percentile list
/// (5,10,40,60,90,95) and any requested threshold outside it was simply absent
/// from the `percentiles` cell - the winsorized/trimmed lookups then silently
/// came back 0/partial at exit 0 (issue #4455).
fn build_stats_args(stats_options: &str, pct_thresholds: Option<(f64, f64)>) -> Vec<String> {
    let mut tokens: Vec<String> = stats_options
        .split_whitespace()
        .map(ToString::to_string)
        .collect();

    let Some((lower, upper)) = pct_thresholds else {
        return tokens;
    };
    // thresholds reach here already validated and truncated to whole
    // percentiles in 1..=100
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let (lower, upper) = (lower as u32, upper as u32);

    // Merge into a caller-supplied --percentile-list (either
    // `--percentile-list <arg>` or `--percentile-list=<arg>`); otherwise merge
    // into the `stats` default list so the default percentiles stay available.
    for i in 0..tokens.len() {
        if tokens[i] == "--percentile-list" {
            if let Some(value) = tokens.get(i + 1).cloned()
                && let Some(merged) = merge_percentile_list(&value, lower, upper)
            {
                tokens[i + 1] = merged;
            }
            // a trailing `--percentile-list` with no value is left for
            // `stats` to reject with its own usage error
            return tokens;
        }
        if let Some(value) = tokens[i].strip_prefix("--percentile-list=") {
            if let Some(merged) = merge_percentile_list(value, lower, upper) {
                tokens[i] = format!("--percentile-list={merged}");
            }
            return tokens;
        }
    }

    // `stats`' default --percentile-list (src/cmd/stats.rs USAGE)
    const STATS_DEFAULT_PERCENTILE_LIST: &str = "5,10,40,60,90,95";
    if let Some(merged) = merge_percentile_list(STATS_DEFAULT_PERCENTILE_LIST, lower, upper) {
        tokens.push("--percentile-list".to_string());
        tokens.push(merged);
    }
    tokens
}

/// Returns true when the percentiles cell contains an entry whose label
/// matches `percentile_label` (regardless of whether its value parses).
fn percentile_entry_present(percentile_str: &str, percentile_label: &str, separator: &str) -> bool {
    percentile_str.split(separator).any(|entry| {
        entry
            .trim()
            .split_once(':')
            .is_some_and(|(label, _)| label.trim() == percentile_label)
    })
}

/// Returns true when the existing stats CSV at `path` can serve the
/// `--use-percentiles` thresholds: it has a `percentiles` column and every
/// non-empty percentiles cell contains entries for both threshold labels.
/// Any read/parse problem returns false so the caller recomputes.
fn stats_csv_covers_percentiles(
    path: &Path,
    lower_label: &str,
    upper_label: &str,
    separator: &str,
) -> bool {
    let Ok(mut rdr) = csv::ReaderBuilder::new().has_headers(true).from_path(path) else {
        return false;
    };
    let Ok(headers) = rdr.headers() else {
        return false;
    };
    let Some(pct_idx) = headers.iter().position(|h| h == "percentiles") else {
        return false;
    };
    for rec in rdr.records() {
        let Ok(rec) = rec else {
            return false;
        };
        let cell = rec.get(pct_idx).unwrap_or("");
        if cell.is_empty() {
            continue;
        }
        if !percentile_entry_present(cell, lower_label, separator)
            || !percentile_entry_present(cell, upper_label, separator)
        {
            return false;
        }
    }
    true
}

/// Compute Pearson's Second Skewness Coefficient: 3 * (mean - median) / stddev
#[inline]
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
#[inline]
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
#[inline]
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
#[inline]
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
#[inline]
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
#[inline]
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
#[inline]
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
#[inline]
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
#[inline]
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

/// Compute Trimean: (Q1 + 2*median + Q3) / 4
/// Tukey's trimean - a robust estimator of central tendency that
/// combines the median with the midhinge.
#[inline]
fn compute_trimean(q1: Option<f64>, median: Option<f64>, q3: Option<f64>) -> Option<f64> {
    if let (Some(q1_val), Some(median_val), Some(q3_val)) = (q1, median, q3) {
        Some((2.0f64.mul_add(median_val, q1_val) + q3_val) / 4.0)
    } else {
        None
    }
}

/// Compute Midhinge: (Q1 + Q3) / 2
/// Midpoint of the middle 50% of data, a robust central tendency measure.
#[inline]
const fn compute_midhinge(q1: Option<f64>, q3: Option<f64>) -> Option<f64> {
    if let (Some(q1_val), Some(q3_val)) = (q1, q3) {
        Some(f64::midpoint(q1_val, q3_val))
    } else {
        None
    }
}

/// Compute Robust Coefficient of Variation: MAD / |median|
/// Uses robust measures (MAD and median magnitude) instead of stddev and mean.
#[inline]
fn compute_robust_cv(mad: Option<f64>, median: Option<f64>) -> Option<f64> {
    if let (Some(mad_val), Some(median_val)) = (mad, median) {
        if median_val.abs() > f64::EPSILON {
            Some(mad_val / median_val.abs())
        } else {
            None
        }
    } else {
        None
    }
}

/// Compute Jarque-Bera test statistic: (n/6) * (S^2 + K^2/4)
/// Tests whether data follows a normal distribution.
/// Returns (`jb_statistic`, `p_value`) where `p_value` is from chi-squared(2) distribution.
#[inline]
fn compute_jarque_bera(skewness: Option<f64>, kurtosis: Option<f64>, n: u64) -> Option<(f64, f64)> {
    if n < 3 {
        return None;
    }
    if let (Some(skew_val), Some(kurt_val)) = (skewness, kurtosis) {
        #[allow(clippy::cast_precision_loss)]
        let n_f64 = n as f64;
        let jb = (n_f64 / 6.0) * skew_val.mul_add(skew_val, kurt_val * kurt_val / 4.0);
        // Upper-tail p-value from chi-squared distribution with 2 degrees of freedom
        // For chi-squared(2), the survival function (1 - CDF) is e^(-x/2)
        let p_value = (-jb / 2.0_f64).exp();
        Some((jb, p_value))
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

/// Compute Normalized Entropy: `shannon_entropy` / log2(cardinality)
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
    // Filter NaN/±Infinity: a single non-finite cell would poison Welford
    // correlation state and propagate silently through all downstream statistics.
    fast_float2::parse::<f64, &[u8]>(s.as_bytes())
        .ok()
        .filter(|v| v.is_finite())
}

/// Parse a numeric value from bytes, handling empty bytes and invalid values
#[inline]
fn parse_float_opt_from_bytes(bytes: &[u8]) -> Option<f64> {
    if bytes.is_empty() {
        return None;
    }
    fast_float2::parse::<f64, &[u8]>(bytes)
        .ok()
        .filter(|v| v.is_finite())
}

/// Format a percentile value compactly: integral percentiles render without a
/// fractional part (e.g. `5.0` -> `"5"`, `5.5` -> `"5.5"`). Used for both
/// constructing column names and for looking up keys in the percentiles string;
/// keeping a single formatter avoids drift between the two sites.
#[inline]
fn fmt_pct(p: f64) -> String {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    if p.fract() == 0.0 && p >= 0.0 {
        format!("{}", p as u32)
    } else {
        format!("{p}")
    }
}

/// Parse a percentile value from the percentiles column string
/// Format: "5: value1|10: value2|..." (separator from `QSV_STATS_SEPARATOR` env var, default "|")
/// For Date/DateTime types, values are RFC3339 date strings; for numeric types, they're numbers
/// Returns the numeric value (in days since epoch for dates) for the specified percentile label, or
/// None if not found
fn parse_percentile_value(
    percentile_str: &str,
    percentile_label: &str,
    field_type: FieldType,
    separator: &str,
    prefer_dmy: bool,
) -> Option<f64> {
    if percentile_str.is_empty() {
        return None;
    }

    // Split by separator and find matching percentile
    for entry in percentile_str.split(separator) {
        let entry = entry.trim();
        if let Some(colon_pos) = entry.find(':') {
            let label = entry[..colon_pos].trim();
            let value_str = entry[colon_pos + 1..].trim();

            if label == percentile_label {
                // For Date/DateTime types, parse as date string; for numeric types, parse as float
                return if field_type.is_date_or_datetime() {
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
/// Format: "5: value1|10: value2|25: value3|..." (separator from `QSV_STATS_SEPARATOR` env var,
/// default "|") Returns a vector of all percentile value strings (the values after colons)
/// Used for pattern matching all percentile values in fast mode
fn parse_all_percentile_string_values<'a>(
    percentile_str: &'a str,
    separator: &str,
) -> Vec<&'a str> {
    if percentile_str.is_empty() {
        return Vec::new();
    }

    // Split by separator and extract all values after colons
    percentile_str
        .split(separator)
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
/// Matches the `FieldType` enum from stats.rs but kept local for performance
#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, PartialEq, Debug)]
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
    /// Convert string representation to `FieldType` enum
    /// Returns None if the string doesn't match any known type
    #[inline]
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

    /// Check if this type is Date or `DateTime`
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
/// Quick mode checks min/max values only (fast), thorough mode checks all percentile values (slower
/// but more confident).
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
    // More question marks = less confidence
    let suffix = if scan_mode == "quick" { "??" } else { "?" };

    // Shared closure used for both quick and thorough modes
    // to check if a string matches a Gregorian date pattern
    let check_value = |s: &str| -> Option<&str> {
        // gYearMonth: "1999-05" (length 7, dash at position 4)
        if s.len() == 7
            && s.as_bytes().get(4) == Some(&b'-')
            && regex_oncelock!(r"^\d{4}-(0[1-9]|1[0-2])$").is_match(s)
        {
            // Validate that the month portion is within 01-12
            let month_str = &s[5..7];
            if let Ok(month) = month_str.parse::<u8>()
                && (1..=12).contains(&month)
            {
                return Some("gYearMonth");
            }
        }

        // gYear: "1999" (length 4)
        if s.len() == 4
            && regex_oncelock!(r"^\d{4}$").is_match(s)
            && let Ok(year) = s.parse::<i32>()
            && (1000..=3000).contains(&year)
        {
            return Some("gYear");
        }

        // gMonthDay: "--05-01" (length 7)
        if s.len() == 7 && regex_oncelock!(r"^--\d{2}-\d{2}$").is_match(s) {
            // validate numeric ranges: month 1-12, with month-specific day limits
            if let (Ok(month), Ok(day)) = (s[2..4].parse::<u32>(), s[5..7].parse::<u32>())
                && (1..=12).contains(&month)
                && match month {
                    // Months with 31 days
                    1 | 3 | 5 | 7 | 8 | 10 | 12 => (1..=31).contains(&day),
                    // Months with 30 days
                    4 | 6 | 9 | 11 => (1..=30).contains(&day),
                    // February: allow up to 29 to accommodate leap years (year is unknown)
                    2 => (1..=29).contains(&day),
                    _ => false,
                }
            {
                return Some("gMonthDay");
            }
        }

        // gDay: "---01" (length 5)
        if s.len() == 5 && regex_oncelock!(r"^---\d{2}$").is_match(s) &&
            // validate numeric range: day 1-31
            let Ok(day) = s[3..5].parse::<u32>()
            && (1..=31).contains(&day)
        {
            return Some("gDay");
        }

        // gMonth: "--05" (length 4)
        if s.len() == 4 && regex_oncelock!(r"^--\d{2}$").is_match(s) {
            // validate numeric range: month 1-12
            if let Ok(month) = s[2..4].parse::<u32>()
                && (1..=12).contains(&month)
            {
                return Some("gMonth");
            }
        }

        None
    };

    // Thorough mode: check all percentile values
    if scan_mode == "thorough" {
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
            // Check all percentile values - only return type if ALL match the same pattern
            let mut matched_type: Option<&str> = None;
            for &val_str in pct_values {
                if val_str.is_empty() {
                    continue; // Skip empty values
                }
                {
                    let pattern_type = check_value(val_str)?;
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
                }
            }

            // All values matched the same pattern
            if let Some(base_type) = matched_type {
                return Some(format!("{base_type}{suffix}"));
            }
        }
        return None;
    }

    // Quick mode: check min/max values
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
    // Check min_str first
    if let Some(min_s) = min_str
        && !min_s.is_empty()
        && let Some(greg_type) = check_value(min_s)
    {
        // If max_str is available, verify it matches the same pattern for confidence
        if let Some(max_s) = max_str {
            if !max_s.is_empty() {
                if let Some(max_type) = check_value(max_s) {
                    // Both match the same type, return it
                    if greg_type == max_type {
                        return Some(format!("{greg_type}{suffix}"));
                    }
                    // Different patterns, not confident - return None
                    return None;
                }
                // max_str does not match pattern, don't return based only on min_str
                return None;
            }
            // max_str is empty; treat as missing, don't return based only on min_str
            return None;
        }
        // max_str not present at all, rely on min_str alone (conservative)
        return Some(format!("{greg_type}{suffix}"));
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
/// Based on the analysis at <https://github.com/user-attachments/files/23841656/xsd_analysis.md>
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
/// For `DateTime` types, returns full RFC3339 format with time and timezone
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

// Indices into `OutlierStats::counts` (keep in sync with OUTLIER_COUNTS_LEN).
const OUTLIER_EXTREME_LOWER: usize = 0;
const OUTLIER_MILD_LOWER: usize = 1;
const OUTLIER_NORMAL: usize = 2;
const OUTLIER_MILD_UPPER: usize = 3;
const OUTLIER_EXTREME_UPPER: usize = 4;
const OUTLIER_TOTAL: usize = 5;
const OUTLIER_COUNTS_LEN: usize = 6;

/// Statistics tracked during outlier scanning
#[derive(Clone, Default)]
struct OutlierStats {
    // Counts indexed by OUTLIER_{EXTREME_LOWER, MILD_LOWER, NORMAL, MILD_UPPER, EXTREME_UPPER,
    // TOTAL}
    counts:                 [u64; OUTLIER_COUNTS_LEN],
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
    theil_index:      Option<f64>,
    mean_ad:          Option<f64>,
}

/// Statistics for Shannon Entropy and Simpson's Diversity Index
#[derive(Clone, Default)]
struct EntropyStats {
    entropy:            Option<f64>,
    simpsons_diversity: Option<f64>,
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
///
/// `xy_counts` is keyed on a packed pair of per-column dictionary symbols rather
/// than on the values themselves. Two heap `String`s per joint cell cost ~200 bytes
/// once `HashMap` slack is counted, and -- worse -- `entry()` takes its key by value,
/// so the common path (cell already present) allocated, hashed, compared and then
/// immediately dropped two `String`s for every pair of every row. See #4356.
///
/// Marginal frequencies are deliberately NOT stored here. They are a pure function
/// of `xy_counts` and are only consumed by MI/NMI/U, so they are derived inside
/// `finalize_bivariate_pair_stats` instead. Keeping them off this struct means only
/// the pairs currently being finalized hold marginals, rather than every field pair
/// holding a pair of maps for the whole run.
#[derive(Clone, Default)]
struct BivariateChunkStats {
    correlation_state: CorrelationState,
    x_values:          Vec<f64>, // For Spearman/Kendall (need ranks)
    y_values:          Vec<f64>, // For Spearman/Kendall (need ranks)
    /// Joint frequencies, keyed by `pack_joint_key(x_sym, y_sym)`.
    xy_counts:         HashMap<u64, u64>,
    total_pairs:       u64, // Total count of pairs
}

/// What one chunk produces: per-pair statistics plus the dictionaries needed to
/// interpret the symbols inside them.
///
/// Symbols are chunk-local -- each worker interns independently -- so the merge has
/// to translate them into a shared numbering before joint counts from different
/// chunks can be added together. `dicts` is empty when frequency counts are not
/// being computed, since nothing then needs translating.
struct BivariateChunkOutput {
    stats: Vec<BivariateChunkStats>,
    dicts: Vec<ValueDict>,
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
    // Theil's U, both directions. `x` = field1, `y` = field2 (see
    // `finalize_bivariate_pair_stats`).
    u_field2_given_field1: Option<f64>, // U(field2|field1) = MI / H(field2)
    u_field1_given_field2: Option<f64>, // U(field1|field2) = MI / H(field1)
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

/// Per-column value dictionary: distinct raw values in first-seen order, so a
/// value's index IS its symbol. foldhash matches the hasher used elsewhere in qsv
/// (see the same alias in `cat.rs`).
///
/// Keyed on `Box<[u8]>` rather than `Box<str>` deliberately. The values come off a
/// `ByteRecord` as raw bytes, and the accumulation loop must EXCLUDE values that are
/// not valid UTF-8 rather than lossily decode them -- `from_utf8_lossy` would both
/// count rows the scan is supposed to skip and merge distinct invalid byte sequences
/// into a single replacement-character symbol.
type ValueDict = IndexSet<Box<[u8]>, foldhash::fast::RandomState>;

/// Symbol meaning "this cell has no symbol": empty, not valid UTF-8, or a column
/// that is not being interned. Never appears in a joint key.
const NO_SYM: u32 = u32::MAX;

/// Floor for the default `-C/--cardinality-threshold`, which is otherwise half the
/// row count. Keeps the guard inert on small inputs, where no column is meaningfully
/// high-cardinality and half the row count would prune ordinary categorical columns.
const DEFAULT_CARDINALITY_THRESHOLD: u64 = 1_000;

/// Pack two per-column symbols into one joint-frequency key.
///
/// x occupies the high half and y the low half, so the two symbols can be recovered
/// independently when the marginals are derived at finalize time.
#[inline]
const fn pack_joint_key(x_sym: u32, y_sym: u32) -> u64 {
    ((x_sym as u64) << 32) | (y_sym as u64)
}

/// One field pair's place in the per-chunk statistics vector, with the two columns
/// it reads resolved to dense slot indices.
struct PairPlan {
    /// The `(col_idx, col_idx)` key this pair is reported under.
    key:             (u16, u16),
    /// Slot of the x (field1) column in `BivariatePlan::cols`.
    x_slot:          u32,
    /// Slot of the y (field2) column in `BivariatePlan::cols`.
    y_slot:          u32,
    /// Whether this pair should build a joint-frequency map at all.
    ///
    /// False when either side's cardinality exceeds `-C/--cardinality-threshold`.
    /// The gate used to be applied only at finalize, so an excluded pair still
    /// accumulated its full joint map and then threw it away; deciding here means it
    /// is never built. Row COUNTING is unaffected (see `total_pairs` in the scan), so
    /// the reported `n_pairs` is the same either way.
    accumulate_freq: bool,
}

/// Execution plan shared by every chunk of a bivariate run.
///
/// Two things it buys over iterating the `field_pairs` map directly:
///
///   * Pairs live in a `Vec` in a stable order, so the hot loop indexes into a
///     `Vec<BivariateChunkStats>` instead of doing a `HashMap` lookup per pair per record -- 780
///     lookups per row on the 41-column benchmark.
///   * The distinct COLUMNS taking part in at least one pair are enumerated once (~41 of them,
///     versus 780 pairs), which is what lets a record be decoded once per column rather than once
///     per pair. Each column appears in 40 pairs on that file, so the per-pair form re-fetched,
///     re-validated and re-parsed every value 40 times.
struct BivariatePlan {
    /// slot -> (`col_idx`, `field_type`). Order is stable across chunks.
    cols:  Vec<(usize, FieldType)>,
    /// Pairs in a stable order; index into this is the index into the per-chunk stats.
    pairs: Vec<PairPlan>,
}

/// Whether BOTH sides of a pair can ever yield a numeric value.
///
/// `x_values`/`y_values` are only pushed when both sides parse numeric, so this is
/// the gate for reserving them. It is an allocation hint only -- a `Vec` still grows
/// on demand, so a mistyped column costs a realloc, never a wrong answer.
fn pair_is_numeric(plan: &BivariatePlan, pair: &PairPlan) -> bool {
    let numeric = |slot: u32| {
        plan.cols
            .get(slot as usize)
            .is_some_and(|(_, t)| t.is_numeric_or_date_type())
    };
    numeric(pair.x_slot) && numeric(pair.y_slot)
}

/// The canonical pair ordering: every plan, full or partial, walks keys in this order.
///
/// `field_pairs.keys()` comes out in `HashMap` order, so sorting is load-bearing rather
/// than cosmetic -- per-chunk statistics vectors from different threads line up
/// positionally only because every worker walks the same plan in the same order.
fn sorted_pair_keys(
    field_pairs: &HashMap<(u16, u16), (BivariateFieldInfo, BivariateFieldInfo)>,
) -> Vec<(u16, u16)> {
    let mut keys: Vec<(u16, u16)> = field_pairs.keys().copied().collect();
    keys.sort_unstable();
    keys
}

/// The run-wide `col_idx -> FieldType` decision, made ONCE over the full key list.
///
/// A column's decode type must be a property of the COLUMN, not of whichever pair
/// happens to reach it first. Normally those coincide, but header lookup when building
/// `field_pairs` is first-match-wins, so duplicate header names can resolve two
/// differently-typed stats rows onto a single `col_idx` -- and then the two pairs
/// disagree about that column's type.
///
/// Resolving that per plan was a real bug under `--bivariate-batch`: the full plan
/// walks every key and takes the first type, while a batch walks only its own slice
/// and can take a different one. On a 3-column `a,b,a` file whose first `a` is Integer
/// and second is Date, `--bivariate-batch 1` put key (1,0) in a pass of its own, where
/// column 0 decoded as Date instead of Integer and the pair's covariance came out
/// 0.0009 instead of 78.0268. Deciding here, over `sorted_pair_keys`, makes every
/// sub-plan agree with the full plan by construction (roborev job 4110).
fn canonical_field_types(
    field_pairs: &HashMap<(u16, u16), (BivariateFieldInfo, BivariateFieldInfo)>,
    keys: &[(u16, u16)],
) -> HashMap<usize, FieldType> {
    let mut types: HashMap<usize, FieldType> = HashMap::new();
    for &key in keys {
        let Some((field1_info, field2_info)) = field_pairs.get(&key) else {
            cold_path();
            continue;
        };
        for info in [field1_info, field2_info] {
            types.entry(info.col_idx).or_insert(info.field_type);
        }
    }
    types
}

/// Build the shared plan for `keys`.
///
/// `keys` MUST be ascending (see `sorted_pair_keys`) and every key must be present in
/// `field_pairs`. `keys` MAY be a SUBSET of `field_pairs.keys()` -- one batch of a
/// multi-pass run (`--bivariate-batch`). `field_pairs` is still passed WHOLE, because
/// `accumulate_freq` needs each pair's cardinalities; only `keys` is partitioned.
///
/// `col_types` MUST come from `canonical_field_types` over the FULL key list, never
/// over `keys` -- that is what keeps a batch's decode types identical to the unbatched
/// run's. See that function for the bug this prevents.
///
/// A sub-plan's `cols` narrows to just the columns its own pairs touch, numbered
/// `0..m-1` in first-touch order. That renumbering needs no extra code because
/// `slot_of`/`cols` are call-local, and nothing outside a plan holds a slot index
/// across plans: `compute_chunk_bivariate` reads only `plan.cols`/`plan.pairs`, the
/// merge sizes `global_dicts`/`remap` from `plan.cols.len()`, and
/// `finalize_bivariate_pair_stats` takes `pair.key`, never a slot.
///
/// `cardinality_threshold` decides, per pair, whether a joint-frequency map is worth
/// building at all -- see `PairPlan::accumulate_freq`.
///
/// `report` controls the two summary log lines. Only the once-per-run full-plan build
/// passes `true`; per-batch builds pass `false`, because both lines are wrong or
/// misleading when computed over a subset -- see the comments at each site.
fn build_bivariate_plan(
    field_pairs: &HashMap<(u16, u16), (BivariateFieldInfo, BivariateFieldInfo)>,
    keys: &[(u16, u16)],
    col_types: &HashMap<usize, FieldType>,
    cardinality_threshold: Option<u64>,
    report: bool,
) -> BivariatePlan {
    debug_assert!(
        keys.is_sorted(),
        "build_bivariate_plan requires ascending keys; chunk stats align positionally"
    );

    // col_idx -> slot. The TYPE stored alongside comes from the run-wide `col_types`,
    // not from the pair being walked, so a batch cannot decode a column differently
    // from the full plan.
    let mut slot_of: HashMap<usize, u32> = HashMap::new();
    let mut cols: Vec<(usize, FieldType)> = Vec::new();
    let mut slot_for = |col_idx: usize, fallback: FieldType, cols: &mut Vec<(usize, FieldType)>| {
        *slot_of.entry(col_idx).or_insert_with(|| {
            let slot = u32::try_from(cols.len()).unwrap_or(u32::MAX);
            // The fallback cannot normally fire: `col_types` is built from the
            // full key list, which is a superset of `keys`.
            cols.push((
                col_idx,
                col_types.get(&col_idx).copied().unwrap_or(fallback),
            ));
            slot
        })
    };

    let mut pairs = Vec::with_capacity(keys.len());
    let mut skipped_high_cardinality = 0_usize;
    for &key in keys {
        // safety: keys came from field_pairs
        let Some((field1_info, field2_info)) = field_pairs.get(&key) else {
            cold_path();
            continue;
        };
        let x_slot = slot_for(field1_info.col_idx, field1_info.field_type, &mut cols);
        let y_slot = slot_for(field2_info.col_idx, field2_info.field_type, &mut cols);

        // Gated on `report` so the warning is emitted once per run rather than once per
        // pass. The comparison itself is now batch-independent -- it is against the
        // run-wide canonical type, not against whatever this plan happened to see first.
        if report {
            for (slot, info) in [(x_slot, field1_info), (y_slot, field2_info)] {
                if let Some((_, seen_type)) = cols.get(slot as usize)
                    && *seen_type != info.field_type
                {
                    cold_path();
                    log::warn!(
                        "bivariate plan: column index {} appears with conflicting field types \
                         ({seen_type:?} and {:?}); decoding it as {seen_type:?}. This means two \
                         stats rows resolved to the same CSV column, which duplicate header names \
                         can cause.",
                        info.col_idx,
                        info.field_type,
                    );
                }
            }
        }

        // Same predicate finalize uses, evaluated once here instead of after the
        // joint map has already been paid for.
        let exceeds_cardinality = cardinality_threshold.is_some_and(|threshold| {
            field1_info.cardinality.is_some_and(|c| c > threshold)
                || field2_info.cardinality.is_some_and(|c| c > threshold)
        });
        if exceeds_cardinality {
            skipped_high_cardinality += 1;
        }

        pairs.push(PairPlan {
            key,
            x_slot,
            y_slot,
            accumulate_freq: !exceeds_cardinality,
        });
    }

    // Gated on `report` because the denominator is `pairs.len()`, which under batching
    // is the BATCH size -- "32 of 32 pairs" once per pass is actively misleading, not
    // merely repetitive. The full-plan build reports the true run-wide figures.
    if report && skipped_high_cardinality > 0 {
        log::info!(
            "bivariate plan: {skipped_high_cardinality} of {} pairs will not accumulate joint \
             frequencies (cardinality exceeds threshold {:?}); mi/nmi/u are reported empty for \
             them",
            pairs.len(),
            cardinality_threshold,
        );
    }

    BivariatePlan { cols, pairs }
}

/// Update correlation state with a new pair of values using Welford's online algorithm
#[inline]
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

    state.m2_x = delta_x.mul_add(delta_x_new, state.m2_x);
    state.m2_y = delta_y.mul_add(delta_y_new, state.m2_y);
    state.cxy = delta_x.mul_add(delta_y_new, state.cxy);
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

    // Clamp factors to >= 0 to guard against tiny negative values from rounding
    // on tie-heavy data, which would otherwise produce sqrt(NaN).
    let denominator = ((n0 - n1).max(0.0) * (n0 - n2).max(0.0)).sqrt();

    if denominator.abs() < f64::EPSILON {
        return None;
    }

    let tau = ((concordant - discordant) as f64) / denominator;
    Some(tau)
}

/// Compute mutual information between two categorical/numeric fields from frequency counts
///
/// `xy_counts` is keyed by `pack_joint_key(x_sym, y_sym)`; the marginals are keyed by
/// the corresponding per-column symbol.
#[allow(clippy::cast_precision_loss)]
fn compute_mutual_information_from_counts(
    xy_counts: &HashMap<u64, u64>,
    x_counts: &HashMap<u32, u64>,
    y_counts: &HashMap<u32, u64>,
    total: u64,
) -> Option<f64> {
    if total == 0 {
        return None;
    }

    let total_f64 = total as f64;

    // Compute mutual information: MI(X,Y) = sum(p(x,y) * log2(p(x,y) / (p(x) * p(y))))
    let mut mi = 0.0;
    for (joint_key, &xy_count) in xy_counts {
        let x_sym = (joint_key >> 32) as u32;
        let y_sym = (joint_key & 0xFFFF_FFFF) as u32;
        let p_xy = xy_count as f64 / total_f64;
        let p_x = x_counts.get(&x_sym).copied().unwrap_or(0) as f64 / total_f64;
        let p_y = y_counts.get(&y_sym).copied().unwrap_or(0) as f64 / total_f64;

        if p_x > 0.0 && p_y > 0.0 && p_xy > 0.0 {
            mi = p_xy.mul_add((p_xy / (p_x * p_y)).log2(), mi);
        }
    }

    Some(mi)
}

/// Compute Shannon entropy from frequency counts
/// Uses the same formula as `compute_all_entropy()`: H(X) = -Σ `p_i` * `log2(p_i)`
/// where `p_i` = `count_i` / total
///
/// Only the counts are read, never the keys, so this is generic over the key type.
#[allow(clippy::cast_precision_loss)]
fn compute_entropy_from_counts<K>(counts: &HashMap<K, u64>, total: u64) -> Option<f64> {
    if total == 0 {
        return None;
    }

    let total_f64 = total as f64;
    let mut entropy = 0.0;

    for count in counts.values() {
        if *count > 0 {
            let p = *count as f64 / total_f64;
            entropy = p.mul_add(-p.log2(), entropy);
        }
    }

    Some(entropy)
}

/// Compute normalized mutual information from mutual information and entropies
/// NMI = MI / sqrt(H(X) * H(Y))
/// Returns None if either entropy is invalid (0, negative, or None) or if the denominator
/// is smaller than `f64::EPSILON` (guards against subnormal products from extremely low
/// entropies producing Inf/NaN).
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
    if denominator < f64::EPSILON {
        return None;
    }

    // NMI = MI / sqrt(H(X) * H(Y))
    Some(mi_val / denominator)
}

/// Compute Theil's directed uncertainty coefficient U(target|source) = MI / H(target).
/// This is the fraction of the target's entropy explained by the source: 1.0 when the source
/// fully determines the target (a functional mapping), 0.0 when they are independent. Unlike the
/// symmetric NMI, it is directional, which is what a directed source->target flow needs.
/// Returns None if H(target) is invalid (0, negative, or None), and clamps to [0, 1] to absorb
/// floating-point overshoot from MI and H being accumulated separately.
fn compute_uncertainty_coefficient(mi: Option<f64>, h_target: Option<f64>) -> Option<f64> {
    let (Some(mi_val), Some(h_target_val)) = (mi, h_target) else {
        return None;
    };

    // H(target) <= 0 means the target is constant; "explained fraction" is undefined.
    if h_target_val < f64::EPSILON {
        return None;
    }

    Some((mi_val / h_target_val).clamp(0.0, 1.0))
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

/// Combined per-chunk output from a single fused scan that BOTH counts outliers
/// and collects values for Kurtosis/Gini/Atkinson (KGA). Both vectors are
/// slot-indexed (parallel to the `outlier_fields` / `kga_fields` slices passed to
/// `count_and_collect_chunk`).
struct FusedChunkOutput {
    outlier_stats: Vec<OutlierStats>,
    kga_values:    Vec<Vec<f64>>,
}

/// Merge one chunk's outlier stats for a single field slot into the running total.
/// Order-independent (sums, counts, min/max), so chunks may be merged in any order.
fn merge_outlier_stats(total: &mut OutlierStats, stats: &OutlierStats) {
    for i in 0..OUTLIER_COUNTS_LEN {
        total.counts[i] += stats.counts[i];
    }
    total.sum_outliers += stats.sum_outliers;
    total.sum_normal += stats.sum_normal;
    total.sum_all += stats.sum_all;
    total.count_all += stats.count_all;
    total.winsorized_sum += stats.winsorized_sum;
    total.winsorized_count += stats.winsorized_count;
    total.trimmed_sum += stats.trimmed_sum;
    total.trimmed_count += stats.trimmed_count;
    total.sum_squares_outliers += stats.sum_squares_outliers;
    total.sum_squares_normal += stats.sum_squares_normal;
    total.sum_squares_trimmed += stats.sum_squares_trimmed;
    total.sum_squares_winsorized += stats.sum_squares_winsorized;
    if let Some(min) = stats.min_outliers {
        total.min_outliers = Some(total.min_outliers.map_or(min, |m| m.min(min)));
    }
    if let Some(max) = stats.max_outliers {
        total.max_outliers = Some(total.max_outliers.map_or(max, |m| m.max(max)));
    }
    if let Some(min) = stats.min_normal {
        total.min_normal = Some(total.min_normal.map_or(min, |m| m.min(min)));
    }
    if let Some(max) = stats.max_normal {
        total.max_normal = Some(total.max_normal.map_or(max, |m| m.max(max)));
    }
    if let Some(min) = stats.min_trimmed {
        total.min_trimmed = Some(total.min_trimmed.map_or(min, |m| m.min(min)));
    }
    if let Some(max) = stats.max_trimmed {
        total.max_trimmed = Some(total.max_trimmed.map_or(max, |m| m.max(max)));
    }
    if let Some(min) = stats.min_winsorized {
        total.min_winsorized = Some(total.min_winsorized.map_or(min, |m| m.min(min)));
    }
    if let Some(max) = stats.max_winsorized {
        total.max_winsorized = Some(total.max_winsorized.map_or(max, |m| m.max(max)));
    }
}

/// Finalize Kurtosis, Gini, Atkinson, Theil & mean absolute deviation for a single
/// field from its full (file-ordered) value vector. Kept bit-identical to the former
/// sequential `compute_all_kga_from_reader` finalize block.
fn finalize_kga(
    values: &[f64],
    precalc_mean: Option<f64>,
    precalc_variance: Option<f64>,
    precalc_sum: Option<f64>,
    atkinson_epsilon: f64,
) -> KGAStats {
    // Need at least 2 values for meaningful statistics
    if values.len() < 2 {
        return KGAStats::default();
    }

    // Compute kurtosis with precalculated mean and variance
    let kurtosis_val = kurtosis(values.iter().copied(), precalc_mean, precalc_variance);

    // Compute Gini coefficient with precalculated sum (not mean!)
    let gini_val = gini(values.iter().copied(), precalc_sum);

    // Compute Atkinson Index (epsilon parameter configurable via --epsilon)
    let atkinson_val = atkinson(values.iter().copied(), atkinson_epsilon, precalc_mean, None);

    // Compute Theil Index: (1/n) * Σ((x_i / mean) * ln(x_i / mean))
    // Only for positive values (Theil index is undefined for non-positive values)
    #[allow(clippy::cast_precision_loss)]
    let theil_val = {
        // First pass: compute sum and count of positive values
        let mut pos_sum = 0.0_f64;
        let mut pos_count: usize = 0;
        for &v in values {
            if v > 0.0 {
                pos_sum += v;
                pos_count += 1;
            }
        }

        if pos_count >= 2 {
            let n = pos_count as f64;
            let pos_mean = pos_sum / n;
            if pos_mean > f64::EPSILON {
                // Second pass: accumulate Theil sum over positive values
                let mut theil_sum = 0.0_f64;
                for &v in values {
                    if v > 0.0 {
                        let ratio = v / pos_mean;
                        // use CPU's Fused Multiply-Add (FMA) for better precision
                        theil_sum = ratio.mul_add(ratio.ln(), theil_sum);
                    }
                }
                Some(theil_sum / n)
            } else {
                None
            }
        } else {
            None
        }
    };

    // Compute Mean Absolute Deviation from mean: (1/n) * Σ|x_i - mean|
    #[allow(clippy::cast_precision_loss)]
    let mean_ad_val = if let Some(mean_val) = precalc_mean {
        let n = values.len() as f64;
        let sum_abs_dev: f64 = values.iter().map(|&x| (x - mean_val).abs()).sum();
        Some(sum_abs_dev / n)
    } else {
        None
    };

    KGAStats {
        kurtosis:         kurtosis_val,
        gini_coefficient: gini_val,
        atkinson_index:   atkinson_val,
        theil_index:      theil_val,
        mean_ad:          mean_ad_val,
    }
}

/// Single fused scan over a chunk of records that BOTH counts outliers (into
/// slot-indexed `OutlierStats`) and collects numeric values for KGA (into
/// slot-indexed `Vec<f64>`), replacing the two former independent full-file passes.
/// Uses index-based slot access (no per-cell String-keyed `HashMap` lookup) and a
/// single reused `csv::ByteRecord`.
fn count_and_collect_chunk<I>(
    outlier_fields: &[OutlierFieldInfo],
    kga_fields: &[KGAFieldInfo],
    prefer_dmy: bool,
    records: I,
) -> CliResult<FusedChunkOutput>
where
    I: Iterator<Item = csv::Result<csv::ByteRecord>>,
{
    let mut outlier_stats: Vec<OutlierStats> = vec![OutlierStats::default(); outlier_fields.len()];
    let mut kga_values: Vec<Vec<f64>> = vec![Vec::new(); kga_fields.len()];

    #[allow(unused_assignments)]
    let mut record: csv::ByteRecord = csv::ByteRecord::new();

    for result in records {
        record = result?;

        // --- Outlier slots: bucket each value against its IQR fences ---
        for (i, field_info) in outlier_fields.iter().enumerate() {
            let value_bytes = record.get(field_info.col_idx).unwrap_or(&[]);
            if value_bytes.is_empty() {
                continue; // Skip null/empty values
            }

            let numeric_value = if field_info.field_type.is_date_or_datetime() {
                if let Ok(value_str) = from_utf8(value_bytes) {
                    parse_date_to_days(value_str, prefer_dmy)
                } else {
                    cold_path();
                    None
                }
            } else {
                parse_float_opt_from_bytes(value_bytes)
            };

            let Some(val) = numeric_value else {
                continue; // Skip values that can't be parsed
            };

            // safety: outlier_stats is sized to outlier_fields.len()
            let stats = &mut outlier_stats[i];

            // Update sums and count
            stats.sum_all += val;
            stats.count_all += 1;

            // Compute winsorized and trimmed statistics
            let winsorized_val = val
                .max(field_info.lower_threshold)
                .min(field_info.upper_threshold);
            stats.winsorized_sum += winsorized_val;
            stats.winsorized_count += 1;
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
            stats.sum_squares_winsorized =
                winsorized_val.mul_add(winsorized_val, stats.sum_squares_winsorized);

            // For trimmed mean, only include values within thresholds
            if val >= field_info.lower_threshold && val <= field_info.upper_threshold {
                stats.trimmed_sum += val;
                stats.trimmed_count += 1;
                stats.min_trimmed = Some(stats.min_trimmed.map_or(val, |m| m.min(val)));
                stats.max_trimmed = Some(stats.max_trimmed.map_or(val, |m| m.max(val)));
                stats.sum_squares_trimmed = val.mul_add(val, stats.sum_squares_trimmed);
            }

            // Count outliers and track statistics based on fence comparisons
            if val < field_info.lower_outer {
                stats.counts[OUTLIER_EXTREME_LOWER] += 1;
                stats.counts[OUTLIER_TOTAL] += 1;
                stats.sum_outliers += val;
                stats.sum_squares_outliers = val.mul_add(val, stats.sum_squares_outliers);
                stats.min_outliers = Some(stats.min_outliers.map_or(val, |m| m.min(val)));
                stats.max_outliers = Some(stats.max_outliers.map_or(val, |m| m.max(val)));
            } else if val < field_info.lower_inner {
                stats.counts[OUTLIER_MILD_LOWER] += 1;
                stats.counts[OUTLIER_TOTAL] += 1;
                stats.sum_outliers += val;
                stats.sum_squares_outliers = val.mul_add(val, stats.sum_squares_outliers);
                stats.min_outliers = Some(stats.min_outliers.map_or(val, |m| m.min(val)));
                stats.max_outliers = Some(stats.max_outliers.map_or(val, |m| m.max(val)));
            } else if val <= field_info.upper_inner {
                stats.counts[OUTLIER_NORMAL] += 1;
                stats.sum_normal += val;
                stats.sum_squares_normal = val.mul_add(val, stats.sum_squares_normal);
                stats.min_normal = Some(stats.min_normal.map_or(val, |m| m.min(val)));
                stats.max_normal = Some(stats.max_normal.map_or(val, |m| m.max(val)));
            } else if val <= field_info.upper_outer {
                stats.counts[OUTLIER_MILD_UPPER] += 1;
                stats.counts[OUTLIER_TOTAL] += 1;
                stats.sum_outliers += val;
                stats.sum_squares_outliers = val.mul_add(val, stats.sum_squares_outliers);
                stats.min_outliers = Some(stats.min_outliers.map_or(val, |m| m.min(val)));
                stats.max_outliers = Some(stats.max_outliers.map_or(val, |m| m.max(val)));
            } else {
                stats.counts[OUTLIER_EXTREME_UPPER] += 1;
                stats.counts[OUTLIER_TOTAL] += 1;
                stats.sum_outliers += val;
                stats.sum_squares_outliers = val.mul_add(val, stats.sum_squares_outliers);
                stats.min_outliers = Some(stats.min_outliers.map_or(val, |m| m.min(val)));
                stats.max_outliers = Some(stats.max_outliers.map_or(val, |m| m.max(val)));
            }
        }

        // --- KGA slots: collect parsed values in file order for later finalize ---
        for (j, field_info) in kga_fields.iter().enumerate() {
            let value_bytes = record.get(field_info.col_idx).unwrap_or(&[]);
            if value_bytes.is_empty() {
                continue; // Skip null/empty values
            }

            let numeric_value = if field_info.field_type.is_date_or_datetime() {
                if let Ok(value_str) = from_utf8(value_bytes) {
                    parse_date_to_days(value_str, prefer_dmy)
                } else {
                    cold_path();
                    None
                }
            } else {
                parse_float_opt_from_bytes(value_bytes)
            };

            if let Some(val) = numeric_value {
                // safety: kga_values is sized to kga_fields.len()
                kga_values[j].push(val);
            }
        }
    }

    Ok(FusedChunkOutput {
        outlier_stats,
        kga_values,
    })
}

/// Fused replacement for the former `count_all_outliers` + `compute_all_kga`:
/// computes outlier statistics AND Kurtosis/Gini/Atkinson in a SINGLE scan of the
/// original CSV (chunked & parallel when an index exists and the file is large
/// enough, sequential otherwise). Outlier stats merge order-independently; KGA
/// value vectors are concatenated in strict chunk-index (file) order so results
/// are bit-identical to a sequential read.
///
/// `outlier_names`/`kga_names` are parallel to `outlier_fields`/`kga_fields` (slot
/// i <-> field name i) and map the slot-indexed results back to name-keyed maps.
#[allow(clippy::type_complexity)]
fn compute_outliers_and_kga(
    outlier_fields: Vec<OutlierFieldInfo>,
    outlier_names: Vec<String>,
    kga_fields: Vec<KGAFieldInfo>,
    kga_names: Vec<String>,
    input_path: &Path,
    flag_jobs: Option<usize>,
    atkinson_epsilon: f64,
) -> CliResult<(HashMap<String, OutlierStats>, HashMap<String, KGAStats>)> {
    if outlier_fields.is_empty() && kga_fields.is_empty() {
        return Ok((HashMap::new(), HashMap::new()));
    }

    // Precompute KGA per-field (mean, variance, sum) for finalize BEFORE the field
    // list is moved into an Arc for the parallel workers.
    let kga_precalc: Vec<(Option<f64>, Option<f64>, Option<f64>)> = kga_fields
        .iter()
        .map(|f| (f.mean, f.variance, f.sum))
        .collect();

    let input_path_str = input_path
        .to_str()
        .ok_or_else(|| CliError::Other(format!("Invalid input path: {}", input_path.display())))?;
    let input_path_string = input_path_str.to_string();
    let rconfig = Config::new(Some(&input_path_string));
    let indexed_result = rconfig.indexed()?;
    let prefer_dmy = util::get_envvar_flag("QSV_PREFER_DMY");

    let n_outlier = outlier_fields.len();
    let n_kga = kga_fields.len();

    // Resolve the job count up front (also sizes Rayon's global pool via
    // `util::njobs`). This must happen on EVERY path — including the sequential
    // scan fallbacks below — so the parallel KGA finalize (`into_par_iter`) honors
    // `--jobs` (e.g. `--jobs 1` stays single-threaded) instead of defaulting to
    // Rayon's all-cores global pool.
    let njobs = util::njobs(flag_jobs);

    // (merged_outliers, kga_concat) produced by whichever path runs.
    let (merged_outliers, kga_concat): (Vec<OutlierStats>, Vec<Vec<f64>>) = if let Some(idx) =
        indexed_result
    {
        let idx_count = idx.count() as usize;

        if idx_count == 0 {
            // Empty CSV: match the former behavior — outliers = empty map (the old
            // count_all_outliers early-returned an empty map), KGA = all-None
            // entries (finalize on empty vecs). Empty merged_outliers zips to an
            // empty outlier map below.
            (Vec::new(), vec![Vec::new(); n_kga])
        } else if idx_count < PARALLEL_THRESHOLD {
            // Sequential fallback for small files (test-covered golden path).
            let mut rdr = rconfig.reader_file()?;
            let _headers = rdr.headers()?.clone();
            let fused = count_and_collect_chunk(
                &outlier_fields,
                &kga_fields,
                prefer_dmy,
                rdr.byte_records(),
            )?;
            (fused.outlier_stats, fused.kga_values)
        } else {
            // Parallel path: chunk by index, one fused scan per chunk.
            let chunk_size = util::chunk_size(idx_count, njobs);
            let nchunks = util::num_of_chunks(idx_count, chunk_size);

            log::info!("Parallelizing outlier+KGA computation: {nchunks} chunks, {njobs} jobs");

            // Retain freed jemalloc pages for this parallel, allocation-heavy pass.
            util::retain_alloc_pages_for_aggregation();

            let pool = ThreadPool::new(njobs);
            let (send, recv) = crossbeam_channel::bounded(nchunks);

            // Share the read-only slot-ordered field lists via Arc (no clones).
            let outlier_arc = Arc::new(outlier_fields);
            let kga_arc = Arc::new(kga_fields);

            for i in 0..nchunks {
                let send = send.clone();
                let outlier_arc = Arc::clone(&outlier_arc);
                let kga_arc = Arc::clone(&kga_arc);
                let input_path_string_clone = input_path_string.clone();
                pool.execute(move || {
                    // Open index for this thread; propagate failures through the
                    // channel rather than silently dropping (would under-count).
                    let rconfig_chunk = Config::new(Some(&input_path_string_clone));
                    let mut idx_chunk = match rconfig_chunk.indexed() {
                        Ok(Some(idx)) => idx,
                        Ok(None) => {
                            let _ = send.send((
                                i,
                                Err(CliError::Other(format!(
                                    "Chunk {i}: index is not available for \
                                     {input_path_string_clone}"
                                ))),
                            ));
                            return;
                        },
                        Err(e) => {
                            let _ = send.send((
                                i,
                                Err(CliError::Other(format!(
                                    "Chunk {i}: failed to open index: {e}"
                                ))),
                            ));
                            return;
                        },
                    };

                    // Seek to chunk start position
                    if let Err(e) = idx_chunk.seek((i * chunk_size) as u64) {
                        let _ = send.send((
                            i,
                            Err(CliError::Other(format!("Chunk {i}: seek failed: {e}"))),
                        ));
                        return;
                    }

                    let it = idx_chunk.byte_records().take(chunk_size);
                    let result = count_and_collect_chunk(&outlier_arc, &kga_arc, prefer_dmy, it);
                    let _ = send.send((i, result));
                });
            }

            drop(send);

            // Collect chunks by index so the KGA merge can be file-ordered.
            let mut chunks: Vec<Option<FusedChunkOutput>> = (0..nchunks).map(|_| None).collect();
            for (i, chunk_result) in recv {
                chunks[i] = Some(chunk_result?);
            }

            let mut merged_outliers: Vec<OutlierStats> = vec![OutlierStats::default(); n_outlier];
            let mut kga_concat: Vec<Vec<f64>> = vec![Vec::new(); n_kga];

            // Merge in ASCENDING chunk index. Outliers are order-independent, but
            // KGA value vectors MUST be concatenated in file order to keep float
            // summation bit-identical to a sequential read.
            for chunk in &mut chunks {
                let Some(c) = chunk.take() else {
                    continue;
                };
                for (slot, stats) in c.outlier_stats.iter().enumerate() {
                    merge_outlier_stats(&mut merged_outliers[slot], stats);
                }
                for (slot, mut v) in c.kga_values.into_iter().enumerate() {
                    kga_concat[slot].append(&mut v);
                }
            }

            (merged_outliers, kga_concat)
        }
    } else {
        // No index: single sequential fused scan over the whole file.
        let mut rdr = rconfig.reader_file()?;
        let _headers = rdr.headers()?.clone();
        let fused =
            count_and_collect_chunk(&outlier_fields, &kga_fields, prefer_dmy, rdr.byte_records())?;
        (fused.outlier_stats, fused.kga_values)
    };

    // Build the outlier map (slot -> name). For the empty-CSV case merged_outliers
    // is empty and this yields an empty map (matching the former behavior).
    let outlier_counts: HashMap<String, OutlierStats> =
        outlier_names.into_iter().zip(merged_outliers).collect();

    // Finalize KGA per field (slot -> name) over its file-ordered value vector.
    // Each field's finalize is independent and dominated by a per-column sort
    // (Gini) plus kurtosis/Theil/mean_ad, so fan out across fields with rayon.
    // Results are keyed by name and each field's math is order-independent of the
    // others, so this stays bit-identical to a sequential finalize.
    let kga_stats: HashMap<String, KGAStats> = kga_concat
        .into_par_iter()
        .zip(kga_precalc)
        .zip(kga_names)
        .map(|((values, (mean, variance, sum)), name)| {
            (
                name,
                finalize_kga(&values, mean, variance, sum, atkinson_epsilon),
            )
        })
        .collect();

    Ok((outlier_counts, kga_stats))
}

/// Process a chunk of records and update bivariate statistics
/// Similar to `count_chunk_outliers` but for bivariate computation
///
/// Returns per-pair statistics positionally aligned with `plan.pairs`, plus the
/// chunk-local value dictionaries the joint-frequency symbols refer to.
fn compute_chunk_bivariate<I>(
    plan: &BivariatePlan,
    records: I,
    stats_config: BivariateStatsConfig,
) -> CliResult<BivariateChunkOutput>
where
    I: Iterator<Item = csv::Result<csv::ByteRecord>>,
{
    if plan.pairs.is_empty() {
        return Ok(BivariateChunkOutput {
            stats: Vec::new(),
            dicts: Vec::new(),
        });
    }

    // Check what we need to compute based on config
    let needs_all_values = stats_config.needs_all_values();
    let needs_freq_counts = stats_config.needs_frequency_counts();

    // Initialize statistics for all field pairs.
    //
    // These reservations are paid per pair PER CHUNK, so they are a fixed cost of
    // `plan.pairs.len() * nchunks` that does not shrink with the row count. On the
    // 1M-row/41-column benchmark that is 780 pairs x ~16 chunks, which is why the
    // measured peak RSS bottomed out around 2.3 GiB even on a 15k-row slice (see
    // scripts/pgo-train.sh). Over-reserving here is therefore expensive in a way that
    // is easy to miss. Reserve only what this chunk can actually fill:
    //
    //   * x_values/y_values are only pushed when BOTH sides parse numeric, so reserving for a pair
    //     with a non-numeric side is pure waste. On the benchmark only ~36 of 780 pairs are
    //     numeric/date on both sides, so the blanket reservation wasted ~95% of ~1 GB.
    //
    // The field_type gate is an allocation hint only: a Vec still grows on demand, so
    // a mistyped column costs a realloc, never a wrong answer.
    let estimated_capacity = 5000; // Reasonable estimate for chunk processing
    let estimated_unique_values = estimated_capacity.min(1000);
    let mut chunk_stats: Vec<BivariateChunkStats> = plan
        .pairs
        .iter()
        .map(|pair| {
            let mut stats = BivariateChunkStats::default();
            // Only allocate value vectors if needed for Spearman/Kendall AND both
            // sides can actually yield a numeric value to push.
            if needs_all_values && pair_is_numeric(plan, pair) {
                stats.x_values.reserve(estimated_capacity);
                stats.y_values.reserve(estimated_capacity);
            }
            // Only allocate the joint map if needed for mutual information AND this
            // pair is actually going to build one. A pair excluded by the cardinality
            // gate never writes a joint cell, so reserving for it is the same
            // reserve-what-never-fills waste this function already avoids for
            // x_values/y_values.
            if needs_freq_counts && pair.accumulate_freq {
                stats.xy_counts.reserve(estimated_unique_values);
            }
            stats
        })
        .collect();

    let prefer_dmy = util::get_envvar_flag("QSV_PREFER_DMY");

    let ncols = plan.cols.len();

    // A column is interned when its symbols are needed: for the joint keys, or --
    // even on the `fast` path -- to memoize date parsing. Interning a date column
    // replaces the old `HashMap<String, Option<f64>>` parse cache with a
    // `Vec<Option<f64>>` indexed by symbol, which removes the last String allocation
    // from the scan without making the cache any larger (it already held one entry
    // per distinct date string).
    // A column is interned when its symbols are actually used: by a pair that will
    // build a joint map, or -- even on the `fast` path -- to memoize date parsing.
    // A column whose every pair is excluded by the cardinality gate is not interned
    // at all, so an excluded high-cardinality column costs no dictionary either.
    let mut intern_slot: Vec<bool> = plan
        .cols
        .iter()
        .map(|(_, field_type)| field_type.is_date_or_datetime())
        .collect();
    if needs_freq_counts {
        for pair in &plan.pairs {
            if pair.accumulate_freq {
                if let Some(f) = intern_slot.get_mut(pair.x_slot as usize) {
                    *f = true;
                }
                if let Some(f) = intern_slot.get_mut(pair.y_slot as usize) {
                    *f = true;
                }
            }
        }
    }

    let mut dicts: Vec<ValueDict> = (0..ncols).map(|_| ValueDict::default()).collect();
    // Parsed-date memo, indexed by [slot][symbol]. Only date columns ever grow one.
    let mut date_by_sym: Vec<Vec<Option<f64>>> = (0..ncols).map(|_| Vec::new()).collect();

    // Per-record scratch, indexed by column slot and reused across records.
    //
    // Decoding happens ONCE per column per record here, instead of once per pair.
    // Each column takes part in 40 pairs on the benchmark file, so the per-pair form
    // re-fetched, re-validated and re-parsed every value 40 times.
    //
    //   skip[slot] -- this cell disqualifies every pair it appears in, for this
    //                 record: the value is empty, or the column is date-typed and the
    //                 bytes are not valid UTF-8. Both previously `continue`d the pair
    //                 BEFORE the correlation update, so neither reaches n_pairs.
    //   num[slot]  -- the parsed numeric value, if any.
    //   sym[slot]  -- the dictionary symbol, or NO_SYM when the column is not
    //                 interned. Only read for pairs that accumulate.
    //   utf8[slot] -- whether the bytes are valid UTF-8. Tracked separately from
    //                 `sym` because a cell still COUNTS toward total_pairs when its
    //                 pair is excluded by the cardinality gate (and so has no
    //                 symbol); invalid UTF-8 is excluded from the frequency counts
    //                 but still feeds correlation, which is the pre-existing
    //                 behavior for a non-date column.
    let mut skip: Vec<bool> = vec![true; ncols];
    let mut num: Vec<Option<f64>> = vec![None; ncols];
    let mut sym: Vec<u32> = vec![NO_SYM; ncols];
    let mut utf8: Vec<bool> = vec![false; ncols];

    #[allow(unused_assignments)]
    let mut record: csv::ByteRecord = csv::ByteRecord::new();

    // Process each record in the chunk
    for result in records {
        record = result?;

        // Decode pass: one entry per participating column.
        for (slot, (col_idx, field_type)) in plan.cols.iter().enumerate() {
            let value_bytes = record.get(*col_idx).unwrap_or(&[]);
            sym[slot] = NO_SYM;
            utf8[slot] = false;
            if value_bytes.is_empty() {
                skip[slot] = true;
                num[slot] = None;
                continue;
            }
            let is_date = field_type.is_date_or_datetime();

            // UTF-8 is validated only where it was before: always for a date column
            // (parsing needs &str), and otherwise only when frequency counts are
            // being built. The `fast` path must not pay for a check it never used.
            let utf8_ok = if is_date || needs_freq_counts {
                from_utf8(value_bytes).is_ok()
            } else {
                false
            };
            utf8[slot] = utf8_ok;

            if is_date && !utf8_ok {
                // Invalid UTF-8 in a date column skipped the whole pair before the
                // correlation update.
                cold_path();
                skip[slot] = true;
                num[slot] = None;
                continue;
            }
            skip[slot] = false;

            if intern_slot[slot] && utf8_ok {
                // Probe with the borrowed slice FIRST. `insert_full` takes its key by
                // value, so building the Box up front would allocate and copy on every
                // row even when the value is already interned -- exactly the per-row
                // allocation this encoding exists to remove. Only a miss allocates.
                let dict = &mut dicts[slot];
                let idx = if let Some(idx) = dict.get_index_of(value_bytes) {
                    idx
                } else {
                    dict.insert_full(Box::from(value_bytes)).0
                };
                // NO_SYM is u32::MAX, so u32::try_from alone is not enough: an index
                // equal to the sentinel would be read back as "no symbol", silently
                // dropping the cell from the frequency counts in release builds and,
                // for a date column, resizing the memo to 4 billion entries.
                let (Ok(idx_u32), true) = (u32::try_from(idx), idx != NO_SYM as usize) else {
                    cold_path();
                    return fail_incorrectusage_clierror!(
                        "Column {col_idx} has more than {} distinct values, which exceeds what \
                         the bivariate joint-frequency encoding can address. Use \
                         -C/--cardinality-threshold to skip mutual information for \
                         high-cardinality fields.",
                        NO_SYM
                    );
                };
                sym[slot] = idx_u32;
            }

            num[slot] = if is_date {
                // safety: a date column with invalid UTF-8 was skipped above, and
                // date columns are always interned, so the symbol is present.
                let s = sym[slot] as usize;
                let memo = &mut date_by_sym[slot];
                if s >= memo.len() {
                    memo.resize(s + 1, None);
                    // A fresh slot is indistinguishable from a cached `None`, so parse
                    // on first sight and store the result.
                    let parsed = from_utf8(value_bytes)
                        .ok()
                        .and_then(|v| parse_date_to_days(v, prefer_dmy));
                    memo[s] = parsed;
                    parsed
                } else {
                    memo[s]
                }
            } else {
                // Numeric parsing reads bytes directly, so it never allocates.
                parse_float_opt_from_bytes(value_bytes)
            };
        }

        for (pair_idx, pair) in plan.pairs.iter().enumerate() {
            let (x_slot, y_slot) = (pair.x_slot as usize, pair.y_slot as usize);
            if skip[x_slot] || skip[y_slot] {
                continue;
            }
            // safety: chunk_stats is built from plan.pairs, so indices line up
            let Some(stats) = chunk_stats.get_mut(pair_idx) else {
                cold_path();
                debug_assert!(false, "chunk_stats missing expected index: {pair_idx}");
                continue;
            };

            // For numeric/date types, update correlation state and collect values
            if let (Some(x_val), Some(y_val)) = (num[x_slot], num[y_slot]) {
                update_correlation_state(&mut stats.correlation_state, x_val, y_val);
                // Only store values if needed for Spearman/Kendall
                if needs_all_values {
                    stats.x_values.push(x_val);
                    stats.y_values.push(y_val);
                }
            }

            // Accumulate joint frequency counts - these are needed for mutual
            // information. Marginal frequencies are derived from xy_counts at
            // finalization to ensure consistency. Invalid UTF-8 is excluded here but
            // has already fed correlation above.
            if needs_freq_counts {
                if !utf8[x_slot] || !utf8[y_slot] {
                    cold_path();
                    continue;
                }
                // Counted even when the pair is excluded by the cardinality gate, so
                // the reported n_pairs does not depend on whether the joint map was
                // built. Only the map itself is skipped.
                stats.total_pairs += 1;
                if !pair.accumulate_freq {
                    continue;
                }
                let (x_sym, y_sym) = (sym[x_slot], sym[y_slot]);
                debug_assert!(
                    x_sym != NO_SYM && y_sym != NO_SYM,
                    "an accumulating pair must have both columns interned"
                );
                *stats
                    .xy_counts
                    .entry(pack_joint_key(x_sym, y_sym))
                    .or_insert(0) += 1;
            }
        }
    }

    // The dictionaries only need to outlive the chunk when the merge has symbols to
    // translate. On the `fast` path they are just date-parse memos -- dropping them
    // here keeps that path's footprint where it was.
    if !needs_freq_counts {
        dicts.clear();
    }

    Ok(BivariateChunkOutput {
        stats: chunk_stats,
        dicts,
    })
}

/// Finalize per-pair bivariate statistics from an aggregated `BivariateChunkStats`.
///
/// Marginal frequencies are derived here from `chunk_stats.xy_counts` rather than
/// being supplied by the caller, so they live only for the duration of this call.
/// This function only reads from `chunk_stats` — it does not mutate it.
fn finalize_bivariate_pair_stats(
    pair_key: (u16, u16),
    chunk_stats: &BivariateChunkStats,
    field_pairs: &HashMap<(u16, u16), (BivariateFieldInfo, BivariateFieldInfo)>,
    field_names: &[String],
    cardinality_threshold: Option<u64>,
    stats_config: BivariateStatsConfig,
) -> CliResult<((u16, u16), BivariateStats)> {
    let n_pairs = chunk_stats
        .correlation_state
        .count
        .max(chunk_stats.total_pairs);

    // chunk_stats keys mirror field_pairs keys; a miss indicates an invariant violation.
    let (field1_info, field2_info) = field_pairs.get(&pair_key).ok_or_else(|| {
        CliError::Other(format!(
            "Invariant violation: field pair not found: {pair_key:?}"
        ))
    })?;

    // Early exit: skip all correlation/covariance computations if variance is zero
    let has_zero_variance = field1_info.stddev.is_some_and(|s| s.abs() < f64::EPSILON)
        || field2_info.stddev.is_some_and(|s| s.abs() < f64::EPSILON)
        || field1_info.variance.is_some_and(|v| v.abs() < f64::EPSILON)
        || field2_info.variance.is_some_and(|v| v.abs() < f64::EPSILON);

    let pearson =
        if !stats_config.pearson || has_zero_variance || chunk_stats.correlation_state.count < 2 {
            None
        } else {
            finalize_pearson_correlation(&chunk_stats.correlation_state)
        };

    let (covariance_sample, covariance_population) =
        if !stats_config.covariance || has_zero_variance || chunk_stats.correlation_state.count < 2
        {
            (None, None)
        } else {
            (
                finalize_covariance(&chunk_stats.correlation_state, true),
                finalize_covariance(&chunk_stats.correlation_state, false),
            )
        };

    let spearman = if !stats_config.spearman || has_zero_variance || chunk_stats.x_values.len() < 2
    {
        None
    } else {
        compute_spearman_correlation(&chunk_stats.x_values, &chunk_stats.y_values)
    };

    let kendall = if !stats_config.kendall || has_zero_variance || chunk_stats.x_values.len() < 2 {
        None
    } else {
        compute_kendall_tau(&chunk_stats.x_values, &chunk_stats.y_values)
    };

    // MI / NMI share a cardinality-threshold gate; compute it once.
    // `exceeds_cardinality` is None when no threshold is configured (always proceed).
    let exceeds_cardinality = cardinality_threshold.map(|threshold| {
        field1_info.cardinality.is_some_and(|c| c > threshold)
            || field2_info.cardinality.is_some_and(|c| c > threshold)
    });

    let log_skip = |what: &str| {
        if let Some(threshold) = cardinality_threshold {
            let (idx1, idx2) = pair_key;
            let field1_name = field_names
                .get(idx1 as usize)
                .map_or("?", std::string::String::as_str);
            let field2_name = field_names
                .get(idx2 as usize)
                .map_or("?", std::string::String::as_str);
            log::debug!(
                "Skipping {what} for pair ({field1_name}, {field2_name}) - cardinality exceeds \
                 threshold {threshold}"
            );
        }
    };

    // Marginal frequencies, derived from the joint counts. Only MI/NMI/U consume
    // them, and each is skipped when total_pairs is 0 or the cardinality gate fires,
    // so deriving them under the same condition avoids building maps nothing reads.
    // Finalize runs under `into_par_iter`, so this bounds live marginals to roughly
    // the job count rather than one pair of maps per field pair.
    let needs_marginals = (stats_config.mi || stats_config.nmi || stats_config.u)
        && chunk_stats.total_pairs > 0
        && exceeds_cardinality != Some(true);
    // Symbol-keyed, not dense Vec<u64> indexed by symbol. A dense table would have to
    // be sized to the COLUMN's cardinality, and two of them are built per pair inside
    // a rayon finalize -- allocation proportional to something other than this pair's
    // actual occupancy, which is the same shape as the over-reservation bug this
    // series already fixed. Integer-keyed maps keep the win (no String hashing, no
    // per-cell clone) without that exposure.
    let (x_counts, y_counts) = if needs_marginals {
        let mut x_counts: HashMap<u32, u64> = HashMap::new();
        let mut y_counts: HashMap<u32, u64> = HashMap::new();
        for (joint_key, &count) in &chunk_stats.xy_counts {
            *x_counts.entry((joint_key >> 32) as u32).or_insert(0) += count;
            *y_counts
                .entry((joint_key & 0xFFFF_FFFF) as u32)
                .or_insert(0) += count;
        }
        (x_counts, y_counts)
    } else {
        (HashMap::new(), HashMap::new())
    };

    // The cardinality gate is checked BEFORE the empty-counts check: the scan now
    // declines to build a joint map for an excluded pair, so its xy_counts is empty
    // by construction. Testing emptiness first would take the silent branch and lose
    // the "skipped, cardinality exceeds threshold" log line.
    let mutual_information = if !stats_config.mi {
        None
    } else if exceeds_cardinality == Some(true) {
        log_skip("mutual information");
        None
    } else if chunk_stats.total_pairs == 0 {
        None
    } else {
        compute_mutual_information_from_counts(
            &chunk_stats.xy_counts,
            &x_counts,
            &y_counts,
            chunk_stats.total_pairs,
        )
    };

    // NMI and the directed uncertainty coefficients (Theil's U) all require MI and the marginal
    // entropies computed from the same frequency counts. Compute them once and share.
    let (normalized_mutual_information, u_field2_given_field1, u_field1_given_field2) =
        if !stats_config.nmi && !stats_config.u {
            (None, None, None)
        } else if exceeds_cardinality == Some(true) {
            if stats_config.nmi {
                log_skip("normalized mutual information");
            }
            if stats_config.u {
                log_skip("uncertainty coefficient");
            }
            (None, None, None)
        } else if chunk_stats.total_pairs == 0 {
            (None, None, None)
        } else {
            // x = field1, y = field2 (see the `value_bytes_x`/`value_bytes_y` assignment in the
            // pair accumulation).
            let h_x = compute_entropy_from_counts(&x_counts, chunk_stats.total_pairs);
            let h_y = compute_entropy_from_counts(&y_counts, chunk_stats.total_pairs);
            let mi = if mutual_information.is_some() {
                mutual_information
            } else {
                compute_mutual_information_from_counts(
                    &chunk_stats.xy_counts,
                    &x_counts,
                    &y_counts,
                    chunk_stats.total_pairs,
                )
            };
            let nmi = if stats_config.nmi {
                compute_normalized_mutual_information(mi, h_x, h_y)
            } else {
                None
            };
            let (u_2_given_1, u_1_given_2) = if stats_config.u {
                (
                    compute_uncertainty_coefficient(mi, h_y), // U(field2|field1) = MI / H(field2)
                    compute_uncertainty_coefficient(mi, h_x), // U(field1|field2) = MI / H(field1)
                )
            } else {
                (None, None)
            };
            (nmi, u_2_given_1, u_1_given_2)
        };

    Ok((
        pair_key,
        BivariateStats {
            pearson,
            spearman,
            kendall,
            covariance_sample,
            covariance_population,
            mutual_information,
            normalized_mutual_information,
            u_field2_given_field1,
            u_field1_given_field2,
            n_pairs,
        },
    ))
}

/// Fold one chunk's output into the run-wide accumulators.
///
/// Extracted from a closure so it can be called from inside the `--bivariate-batch`
/// pass loop, where the accumulators are re-created each iteration and a closure's
/// captures would have to be re-formed with them.
///
/// `pending` is deliberately NOT a parameter: the drain loop borrows
/// `pending.get_mut(next_chunk)` in its `while let` guard while this runs, which only
/// type-checks because the merge does not also hold it.
fn merge_bivariate_chunk(
    chunk: BivariateChunkOutput,
    plan: &BivariatePlan,
    all_stats: &mut [BivariateChunkStats],
    global_dicts: &mut [ValueDict],
    remap: &mut [Vec<u32>],
    needs_all_values: bool,
    needs_freq_counts: bool,
) -> CliResult<()> {
    let BivariateChunkOutput {
        stats: chunk_stats,
        dicts,
    } = chunk;

    if needs_freq_counts {
        // An IndexSet iterates in insertion order, and insertion order IS
        // symbol order, so pushing each value's global index builds a table
        // indexable by the chunk-local symbol. Anything that reordered this
        // iteration would silently corrupt every joint key.
        for (slot, dict) in dicts.into_iter().enumerate() {
            let Some(table) = remap.get_mut(slot) else {
                cold_path();
                continue;
            };
            table.clear();
            table.reserve(dict.len());
            for value in dict {
                // The merged dictionary can exceed any single chunk's, so the
                // NO_SYM sentinel has to be excluded here too, not just at
                // intern time.
                let idx = if let Some(idx) = global_dicts[slot].get_index_of(&value) {
                    idx
                } else {
                    global_dicts[slot].insert_full(value).0
                };
                let (Ok(idx_u32), true) = (u32::try_from(idx), idx != NO_SYM as usize) else {
                    cold_path();
                    return fail_incorrectusage_clierror!(
                        "Merged column {} has more than {} distinct values, which exceeds what \
                         the bivariate joint-frequency encoding can address. Use \
                         -C/--cardinality-threshold to skip mutual information for \
                         high-cardinality fields.",
                        plan.cols.get(slot).map_or(slot, |(c, _)| *c),
                        NO_SYM
                    );
                };
                table.push(idx_u32);
            }
        }
    }

    for (pair_idx, (total_stats, stats)) in all_stats.iter_mut().zip(chunk_stats).enumerate() {
        // Merge correlation states (always needed for Pearson/covariance)
        total_stats.correlation_state =
            merge_correlation_states(&total_stats.correlation_state, &stats.correlation_state);
        // Only merge values if needed for Spearman/Kendall
        if needs_all_values {
            total_stats.x_values.extend(stats.x_values);
            total_stats.y_values.extend(stats.y_values);
        }
        // Only merge frequency counts if needed for mutual information.
        // Marginal frequencies are derived from xy_counts at finalization.
        if needs_freq_counts {
            let Some(pair) = plan.pairs.get(pair_idx) else {
                cold_path();
                continue;
            };
            let (Some(x_remap), Some(y_remap)) = (
                remap.get(pair.x_slot as usize),
                remap.get(pair.y_slot as usize),
            ) else {
                cold_path();
                continue;
            };
            for (joint_key, count) in stats.xy_counts {
                let x_sym = (joint_key >> 32) as u32;
                let y_sym = (joint_key & 0xFFFF_FFFF) as u32;
                let (Some(&gx), Some(&gy)) =
                    (x_remap.get(x_sym as usize), y_remap.get(y_sym as usize))
                else {
                    cold_path();
                    debug_assert!(false, "joint key references an unknown chunk symbol");
                    continue;
                };
                *total_stats
                    .xy_counts
                    .entry(pack_joint_key(gx, gy))
                    .or_insert(0) += count;
            }
            total_stats.total_pairs += stats.total_pairs;
        }
    }
    Ok(())
}

/// Compute all bivariate statistics
/// Uses parallel chunked processing when an index is available and there
/// are more than 10,000 records.
/// Otherwise, uses sequential processing.
/// Returns a `HashMap` mapping field pairs to their bivariate statistics.
fn compute_all_bivariatestats(
    field_pairs: HashMap<(u16, u16), (BivariateFieldInfo, BivariateFieldInfo)>,
    field_names: &[String],
    input_path: &Path,
    progress: Option<&ProgressBar>,
    cardinality_threshold: Option<u64>,
    stats_config: BivariateStatsConfig,
    flag_jobs: Option<usize>,
    flag_bivariate_batch: usize,
) -> CliResult<HashMap<(u16, u16), BivariateStats>> {
    if field_pairs.is_empty() {
        return Ok(HashMap::new());
    }

    // Check what we need based on config
    let needs_all_values = stats_config.needs_all_values();
    let needs_freq_counts = stats_config.needs_frequency_counts();

    // Batching partitions the pairs across repeated passes of the CHUNKED scan, so it
    // only exists on the indexed parallel path. Say so rather than silently ignoring
    // the flag -- a user tuning it on a small file would otherwise get no feedback.
    let warn_batch_ignored = |why: &str| {
        if flag_bivariate_batch > 0 {
            log::info!(
                "--bivariate-batch {flag_bivariate_batch} ignored: {why}, so bivariate statistics \
                 are computed in a single sequential pass."
            );
        }
    };

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

        // Only parallelize if file is large enough
        if idx_count < PARALLEL_THRESHOLD {
            warn_batch_ignored(&format!(
                "{idx_count} rows is below the {PARALLEL_THRESHOLD}-row parallel threshold"
            ));
            // Fall back to sequential for small files
            let mut rdr = rconfig.reader_file()?;
            let _headers = rdr.headers()?.clone();
            winfo!("Computing bivariate statistics sequentially...");
            return compute_all_bivariatestats_sequential(
                &field_pairs,
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

        // The pair ordering is fixed ONCE here. Batches are contiguous slices of it, so
        // every pass walks its pairs in the same relative order a single pass would --
        // which is what lets a batched run reproduce an unbatched one.
        let sorted_keys = sorted_pair_keys(&field_pairs);
        let npairs = sorted_keys.len();

        // `--bivariate-batch 0` means "every pair in one pass". Resolve it to a concrete
        // slice width BEFORE calling `chunks()`, which PANICS on a width of 0.
        let batch_size = if flag_bivariate_batch == 0 {
            npairs
        } else {
            flag_bivariate_batch
        }
        .clamp(1, npairs.max(1));
        let nbatches = npairs.div_ceil(batch_size);

        if nbatches > 1 {
            winfo!(
                "Parallelizing bivariate computation: {npairs} pairs in {nbatches} passes of up \
                 to {batch_size} pairs; {nchunks} chunks, {njobs} jobs per pass"
            );
        } else {
            winfo!("Parallelizing bivariate computation: {nchunks} chunks, {njobs} jobs");
        }

        // ONE monotonic progress bar for the whole run, set up once. The old two-phase
        // bar called `set_position(0)` between scan and finalize, which under batching
        // would reset 2 x nbatches times -- and since {eta}/{per_sec} are derived from
        // elapsed-vs-position, every reset makes them nonsense for the rest of the run.
        // Length counts both units of work: nbatches x nchunks chunk merges, plus one
        // tick per pair finalized.
        if let Some(pb) = progress {
            pb.set_style(
                ProgressStyle::default_bar()
                    .template(
                        "[{elapsed_precise}] [{wide_bar} {percent}%{msg}] ({per_sec} - {eta})",
                    )
                    .unwrap(),
            );
            pb.set_message(format!(
                " of {} field pairs in {} pass(es)",
                HumanCount(npairs as u64),
                HumanCount(nbatches as u64)
            ));
            pb.set_length((nbatches * nchunks + npairs) as u64);
            log::info!("Progress started... {nbatches} x {nchunks} chunks, {npairs} pairs");
        }

        // Retain freed jemalloc pages for this parallel, hashmap-heavy bivariate
        // pass (no-op when background_thread is active or QSV_NO_ALLOC_TUNING is set).
        //
        // This is NOT in tension with batching: retained pages are reused by the next
        // pass rather than returned and re-faulted, so RSS plateaus at the high-water
        // pass instead of ratcheting up across passes.
        util::retain_alloc_pages_for_aggregation();

        // ONE pool for the whole run. Building it per pass would spawn `njobs` OS
        // threads every pass (199 x njobs on a 160-column file). Reuse is safe because
        // a pass fully drains before the next dispatches -- see the drain loop below --
        // so no two passes' chunk statistics are ever live at the same time.
        let pool = ThreadPool::new(njobs);

        // The two plan-build diagnostics are only correct run-wide, so when batching is
        // active they are emitted from one throwaway full-plan build here and suppressed
        // in the per-pass builds. A `BivariatePlan` is a `Vec<PairPlan>` plus a column
        // table -- ~16 bytes per pair, no per-row data -- so this costs ~200 KB even at
        // 12,720 pairs. See `build_bivariate_plan`'s `report` parameter for why each of
        // the two would otherwise be wrong rather than merely repeated.
        //
        // The canonical column types are decided here too, over the FULL key list, and
        // handed to every pass -- see `canonical_field_types` for why deciding them per
        // plan silently changed results under batching.
        let col_types = canonical_field_types(&field_pairs, &sorted_keys);
        if nbatches > 1 {
            drop(build_bivariate_plan(
                &field_pairs,
                &sorted_keys,
                &col_types,
                cardinality_threshold,
                true,
            ));
        }

        let mut final_stats: HashMap<(u16, u16), BivariateStats> = HashMap::with_capacity(npairs);

        for (batch_no, batch_keys) in sorted_keys.chunks(batch_size).enumerate() {
            // Only when batching: a multi-pass run is otherwise silent between the
            // dispatch message and completion -- minutes, on a large input, for anyone
            // not using --progressbar. A single pass stays exactly as quiet as before.
            if nbatches > 1 {
                winfo!(
                    "  pass {}/{nbatches} ({} pairs)...",
                    batch_no + 1,
                    batch_keys.len()
                );
            }

            // Only the KEY LIST is partitioned -- `field_pairs` is passed whole to both
            // the plan build and the finalize below. `finalize_bivariate_pair_stats`
            // looks its pair up in this map and hard-errors on a miss, so narrowing it
            // to the batch would turn into an "Invariant violation", not a wrong number.
            let plan_arc = Arc::new(build_bivariate_plan(
                &field_pairs,
                batch_keys,
                &col_types,
                cardinality_threshold,
                nbatches == 1,
            ));

            let (send, recv) = crossbeam_channel::bounded(nchunks);

            // Process each chunk in parallel. Share the read-only plan via Arc instead of
            // deep-cloning it into every worker. `input_path_string` from above is already
            // UTF-8-validated, so no need to re-validate here.
            for i in 0..nchunks {
                let send = send.clone();
                let plan_arc = Arc::clone(&plan_arc);
                let input_path_string_clone = input_path_string.clone();
                pool.execute(move || {
                    // Open index for this thread. If this fails, propagate an error
                    // through the channel — dropping the chunk silently would
                    // produce incorrect bivariate stats without any indication.
                    let rconfig_chunk = Config::new(Some(&input_path_string_clone));
                    let mut idx_chunk = match rconfig_chunk.indexed() {
                        Ok(Some(idx)) => idx,
                        Ok(None) => {
                            let _ = send.send((
                                i,
                                Err(CliError::Other(format!(
                                    "Chunk {i}: index is not available for \
                                     {input_path_string_clone}"
                                ))),
                            ));
                            return;
                        },
                        Err(e) => {
                            let _ = send.send((
                                i,
                                Err(CliError::Other(format!(
                                    "Chunk {i}: failed to open index: {e}"
                                ))),
                            ));
                            return;
                        },
                    };

                    // Seek to chunk start position
                    if let Err(e) = idx_chunk.seek((i * chunk_size) as u64) {
                        let _ = send.send((
                            i,
                            Err(CliError::Other(format!("Chunk {i}: seek failed: {e}"))),
                        ));
                        return;
                    }

                    // Process chunk records
                    let it = idx_chunk.byte_records().take(chunk_size);
                    let result = compute_chunk_bivariate(&plan_arc, it, stats_config);
                    let _ = send.send((i, result));
                });
            }

            drop(send);

            // Everything from here to the end of the pass is sized by THIS BATCH's pair
            // count, and is dropped when the pass ends. That drop is the memory bound:
            // peak is O(batch_size x nchunks) rather than O(pairs).
            let mut all_stats: Vec<BivariateChunkStats> = plan_arc
                .pairs
                .iter()
                .map(|pair| {
                    let mut stats = BivariateChunkStats::default();
                    // Pre-allocate value vectors with total capacity if needed.
                    //
                    // Gated on BOTH sides being numeric, mirroring `compute_chunk_bivariate`:
                    // x_values/y_values are only pushed when both sides parse numeric, so
                    // reserving idx_count for a pair with a non-numeric side reserves capacity
                    // that can never be filled. This gate was missing here (the closure ignored
                    // the pair entirely) while the per-chunk twin has always had it -- on the
                    // 41-column benchmark that reserved for all 780 pairs where only ~36 can
                    // ever fill, and it scales with pairs x rows (#4360).
                    if needs_all_values && pair_is_numeric(&plan_arc, pair) {
                        stats.x_values.reserve(idx_count);
                        stats.y_values.reserve(idx_count);
                    }
                    stats
                })
                .collect();

            // Merge chunk results in ASCENDING chunk order -- never in completion order.
            //
            // Merging as results arrive makes the output nondeterministic run-to-run:
            // `merge_correlation_states` is Welford, which is not associative in floating
            // point, so pearson/covariance drift in their last digits depending on which
            // worker finished first; and `x_values`/`y_values` are order-sensitive inputs
            // to the Spearman/Kendall rankings. Measured on the 1M-row NYC311 benchmark
            // before this fix: three consecutive runs of the SAME binary on the SAME
            // input produced three different covariance values.
            // `compute_outliers_and_kga` already orders its merge for exactly this
            // reason; this mirrors it.
            //
            // This is a reorder buffer rather than a collect-then-merge: a chunk that
            // arrives early is parked, and each arrival merges every consecutively
            // numbered chunk that is now ready, so a chunk is freed as soon as its
            // predecessors are in. Buffering ALL chunks first would be equally
            // deterministic but measurably costlier -- at 1M rows the chunks finish
            // seconds apart and holding all 16 sets of 780 maps until the last one landed
            // raised peak RSS by ~1 GiB. (At 50k rows chunks finish together and the two
            // are indistinguishable.)
            //
            // Deliberately NOT covered by a unit test: reproducing the bug needs chunk
            // completion order to diverge from dispatch order, which only happens when
            // chunks are big enough to contend. It reproduces on the 539 MB benchmark,
            // but synthetic fixtures up to 41 columns / 820 pairs / 50k rows all finish
            // in dispatch order and stay deterministic even with the bug present, so a
            // test would have to win a race to catch a regression. Correct by
            // construction instead -- do not delete it as "untested".
            //
            // Symbols are chunk-local: each worker interned the values it happened to see,
            // in the order it saw them, so the same symbol means different things in
            // different chunks. Before joint counts can be added together they are
            // translated into one shared numbering, built here in chunk order.
            //
            // Batching does NOT perturb any of this. A column's dictionary is built from
            // first-seen order over the same rows in the same chunks regardless of which
            // pairs share the pass, so a pair's joint counts -- and therefore mi/nmi/u --
            // are identical however the pairs are partitioned. Note the subtlety that
            // makes this hold: `intern_slot` in `compute_chunk_bivariate` is "date column
            // OR touched by an accumulating pair", NOT "present in plan.cols", so a column
            // that reaches a pass only via a cardinality-excluded pair goes uninterned
            // there -- and contributes no joint counts there either, so nothing is lost.
            let ncols = plan_arc.cols.len();
            let mut global_dicts: Vec<ValueDict> =
                (0..ncols).map(|_| ValueDict::default()).collect();
            // Scratch, reused per chunk: remap[slot][chunk_symbol] = global symbol.
            let mut remap: Vec<Vec<u32>> = (0..ncols).map(|_| Vec::new()).collect();

            let mut pending: Vec<Option<BivariateChunkOutput>> =
                (0..nchunks).map(|_| None).collect();
            let mut next_chunk = 0_usize;

            for (i, chunk_result) in &recv {
                let chunk_stats = chunk_result?;
                if let Some(slot) = pending.get_mut(i) {
                    *slot = Some(chunk_stats);
                }
                // Merge the run of chunks that is now contiguous from `next_chunk`.
                while let Some(slot) = pending.get_mut(next_chunk)
                    && let Some(stats) = slot.take()
                {
                    merge_bivariate_chunk(
                        stats,
                        &plan_arc,
                        &mut all_stats,
                        &mut global_dicts,
                        &mut remap,
                        needs_all_values,
                        needs_freq_counts,
                    )?;
                    next_chunk += 1;
                }

                // Update progress bar
                if let Some(pb) = progress {
                    pb.inc(1);
                }
            }

            // Marginal frequencies are derived per pair inside
            // `finalize_bivariate_pair_stats`, from the same `xy_counts` that MI reads --
            // so they stay consistent (counted only over rows where both fields are
            // non-empty) without every pair carrying a pair of maps until finalize runs.
            //
            // Finalize has to happen HERE, inside the pass: it consumes `all_stats` by
            // value, and hoisting it out of the loop would mean keeping every pass's
            // accumulators alive to the end -- exactly what batching exists to avoid.
            final_stats.extend(
                all_stats
                    .into_par_iter()
                    .zip(plan_arc.pairs.par_iter())
                    .map(|(chunk_stats, pair)| {
                        if let Some(pb) = progress {
                            pb.inc(1);
                        }
                        finalize_bivariate_pair_stats(
                            pair.key,
                            &chunk_stats,
                            &field_pairs,
                            field_names,
                            cardinality_threshold,
                            stats_config,
                        )
                    })
                    .collect::<CliResult<Vec<_>>>()?,
            );
        }

        // Finish progress bar after every pass has been finalized.
        if let Some(pb) = progress {
            util::finish_progress(pb);
        }

        Ok(final_stats)
    } else {
        warn_batch_ignored("the input has no index");
        // Sequential fallback when no index exists
        let mut rdr = rconfig.reader_file()?;
        let _headers = rdr.headers()?.clone();
        compute_all_bivariatestats_sequential(
            &field_pairs,
            field_names,
            rdr,
            progress,
            cardinality_threshold,
            stats_config,
        )
    }
}

/// Sequential processing for small files (< 10k records) or when no index exists.
///
/// Delegates to `compute_chunk_bivariate` for the per-record scan (treating the whole
/// file as a single chunk), then computes marginal frequencies and finalizes per pair
/// via `finalize_bivariate_pair_stats` — sharing all of that logic with the parallel path.
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

    // Set up progress bar once before iteration (unknown total, ticks per record).
    if let Some(pb) = progress {
        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] [{wide_bar}] {pos} records ({per_sec})")
                .unwrap(),
        );
        pb.set_length(0);
    }

    // Wrap byte_records() with inspect() so the progress bar ticks every 1000 records
    // without coupling compute_chunk_bivariate to a ProgressBar parameter.
    let mut processed = 0u64;
    let it = rdr.byte_records().inspect(|_| {
        processed += 1;
        if let Some(pb) = progress
            && processed.is_multiple_of(1000)
        {
            pb.set_position(processed);
        }
    });
    // The sort is load-bearing, not cosmetic: `field_pairs.keys()` comes out in
    // `HashMap` order, and `all_stats` below is zipped positionally against
    // `plan.pairs`.
    let sorted_keys = sorted_pair_keys(field_pairs);
    let col_types = canonical_field_types(field_pairs, &sorted_keys);
    let plan = build_bivariate_plan(
        field_pairs,
        &sorted_keys,
        &col_types,
        cardinality_threshold,
        true,
    );
    // One chunk, so its symbols are already the only numbering there is -- no remap,
    // and the dictionaries are dropped with the output.
    let all_stats = compute_chunk_bivariate(&plan, it, stats_config)?.stats;

    if let Some(pb) = progress {
        pb.set_position(processed);
        util::finish_progress(pb);
    }

    // Marginal frequencies are derived per pair inside
    // `finalize_bivariate_pair_stats` (same as the parallel path).

    all_stats
        .into_iter()
        .zip(plan.pairs.iter())
        .map(|(chunk_stats, pair)| {
            finalize_bivariate_pair_stats(
                pair.key,
                &chunk_stats,
                field_pairs,
                field_names,
                cardinality_threshold,
                stats_config,
            )
        })
        .collect()
}

/// Compute Shannon Entropy for all fields by calling the frequency command.
/// Uses `run_qsv_cmd` to call frequency command with --limit 0 to get all frequencies,
/// then parses the CSV output and computes entropy for each field.
/// Returns a `HashMap` mapping field names to their entropy statistics
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
            entropy_stats.insert(
                field_name,
                EntropyStats {
                    entropy:            None,
                    simpsons_diversity: None,
                },
            );
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

        let (entropy, simpsons) = if is_all_unique {
            // For all-unique fields, each value appears exactly once
            // Entropy = log2(n) where n is the number of unique values (which equals total_count)
            // Formula: -Σ p_i * log2(p_i) where p_i = 1/n for each of n values
            // = -n * (1/n) * log2(1/n) = -log2(1/n) = log2(n)
            let entropy = (total_count as f64).log2();
            // Simpson's: 1 - Σ(p_i²) = 1 - n*(1/n)² = 1 - 1/n
            let simpsons = 1.0 - 1.0 / total_count as f64;
            (entropy, simpsons)
        } else {
            // Compute Shannon Entropy: H(X) = -Σ p_i * log2(p_i)
            // and Simpson's Diversity: 1 - Σ(p_i²)
            let mut entropy = 0.0;
            let mut sum_p_squared = 0.0;
            let total = total_count as f64;

            for count in frequencies.values() {
                if *count > 0 {
                    let p = *count as f64 / total;
                    entropy -= p * p.log2();
                    sum_p_squared += p * p;
                }
            }
            (entropy, 1.0 - sum_p_squared)
        };

        entropy_stats.insert(
            field_name,
            EntropyStats {
                entropy:            Some(entropy),
                simpsons_diversity: Some(simpsons),
            },
        );
    }

    Ok(entropy_stats)
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let start_time = Instant::now();
    let args: Args = util::get_args(USAGE, argv)?;

    // Read environment variables once at the top to avoid repeated reads in hot loops
    let stats_separator = std::env::var("QSV_STATS_SEPARATOR").unwrap_or_else(|_| "|".to_string());
    let prefer_dmy = util::get_envvar_flag("QSV_PREFER_DMY");

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

    // Parse and validate percentile thresholds if --use-percentiles is set.
    // This is done BEFORE the `qsv stats` subprocess runs below, so the
    // thresholds can be forwarded into the percentile list `stats` computes -
    // otherwise `stats` computes only its default list (5,10,40,60,90,95) and
    // any threshold outside it is absent from the `percentiles` cell, which
    // used to silently degrade the winsorized/trimmed stats to 0/partial
    // values (issue #4455).
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

        // Truncate to the INTEGER percentile, matching what `stats` actually computes.
        // `stats --percentile-list` casts each entry `as u8`, so asking it for 33.3 yields p33
        // and (since the label reports the percentile computed) writes the key "33". These
        // values are used ONLY to build that lookup key and the winsorized_/trimmed_ column
        // names - never in a numeric computation - so truncating here keeps both in step.
        // Without it, `--pct-thresholds 33.3,66.6` searched the percentiles cell for "33.3",
        // found nothing, and silently emitted 0/empty winsorized and trimmed statistics.
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let (lower, upper) = (lower.trunc(), upper.trunc());

        // truncation can collapse a valid-looking pair (e.g. "33.3,33.6" -> 33, 33), so the
        // ordering has to be re-checked against the percentiles actually used
        if lower >= upper {
            return fail_clierror!(
                "Percentile thresholds {thresholds_str} both truncate to the same percentile \
                 ({lower}). `stats` computes whole percentiles, so give thresholds that differ by \
                 at least 1."
            );
        }

        // `stats` only accepts percentiles in 1..=100, so a lower bound that
        // truncates to 0 (e.g. "0.5") can never be computed or looked up.
        if lower < 1.0 {
            return fail_clierror!(
                "Lower percentile threshold {} truncates to {lower}. `stats` computes whole \
                 percentiles between 1 and 100, so the lower threshold must be at least 1.",
                parts[0]
            );
        }

        (Some(lower), Some(upper))
    } else {
        (None, None)
    };

    // Handle multi-dataset join if requested
    let temp_joined_path: Option<PathBuf>;
    // Header of the joined CSV (empty when not joining). Used to validate that
    // the joined-stats subprocess produced a record for every joined column.
    let mut joined_csv_header: Vec<String> = Vec::new();
    let actual_input_path = if let Some(ref join_inputs_str) = args.flag_join_inputs {
        let additional_inputs: Vec<String> = join_inputs_str
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        // moarstats manages the joined stats output internally — it captures
        // the `qsv stats` subprocess stdout. Reject a caller-supplied
        // -o/--output in --stats-options: it would silently redirect the
        // stats CSV to a file, leaving stdout empty and triggering a
        // confusing downstream "missing 'field' column" parse failure.
        // `stats_options_redirect_output` also catches clustered short
        // options (e.g. `-Eo file`), which a naive `starts_with("-o")` misses.
        if stats_options_redirect_output(&args.flag_stats_options) {
            return fail_incorrectusage_clierror!(
                "--stats-options may not contain -o/--output when --join-inputs is used; \
                 moarstats manages the joined stats output internally."
            );
        }

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

        let (joined_path, joined_header) =
            join_datasets_internal(input_path, &additional_inputs, &join_keys, join_type)?;
        joined_csv_header = joined_header;
        // Belt-and-suspenders: fsync the joined CSV's parent directory so the
        // directory entry is durable for the follow-up `qsv stats` subprocess.
        // `fsync(file)` on Linux doesn't flush parent dir metadata, and rare
        // FS/timing edge cases have correlated with the subprocess seeing a
        // partial view of the joined CSV (CI run 25545197594).
        if let Some(parent) = joined_path.parent() {
            util::sync_directory(parent);
        }
        temp_joined_path = Some(joined_path);
        temp_joined_path.as_ref().unwrap()
    } else {
        temp_joined_path = None;
        input_path
    };

    // A special-format input (.gz/.zip/.parquet/.jsonl/...) is only decompressed when it is
    // read through `Config`. Several passes below hand a path straight to
    // `csv::ReaderBuilder::from_path` (and to Configs built inside worker threads), which
    // would parse the COMPRESSED CONTAINER as CSV - erroring with "invalid utf-8 near byte
    // index 0", or worse, silently yielding empty headers where the error is swallowed.
    //
    // Bind ONE Config and resolve through it. `resolved_path()` caches in that Config's
    // `Arc<OnceLock>`, so every call returns the SAME converted temp; two throwaway
    // `Config::new(..).resolved_path()` calls would each convert to a temp of their own.
    // For an ordinary input this is just `path` unchanged and nothing is converted.
    let actual_input_path_str = actual_input_path
        .to_str()
        .ok_or_else(|| CliError::Other("Invalid input path".to_string()))?
        .to_string();
    let read_conf = Config::new(Some(&actual_input_path_str));
    let resolved_input = read_conf.resolved_path()?;
    // The path to READ DATA from. `actual_input_path` is deliberately left alone: the
    // `qsv stats` subprocess below must still receive the ORIGINAL path, both because
    // `stats` does its own conversion and because the stats cache is keyed on it.
    let read_input_path: &Path = resolved_input.as_deref().unwrap_or(actual_input_path);

    // The PRIMARY input's own Config. Differs from `read_conf` only under --join-inputs, where
    // `actual_input_path` is the JOINED temp: the bivariate guard below needs the primary's own
    // header to tell which joined columns are exclusively SECONDARY. Reading the joined header
    // there would mark every column "primary", leaving `pairable_secondary_only` empty and
    // silently disabling that guard.
    //
    // When not joining this IS the same input, so `read_conf` is reused rather than built afresh -
    // a second Config would resolve a SECOND converted temp (its own `Arc<OnceLock>`).
    let primary_conf = if temp_joined_path.is_some() {
        Config::new(Some(&input_path_str))
    } else {
        read_conf.clone()
    };

    // Auto-create index if --advanced or --bivariate is set and index doesn't exist
    if args.flag_advanced || args.flag_bivariate {
        // index the RESOLVED path - an index beside the compressed source is useless,
        // since the data actually read is the converted temp
        let rconfig = read_conf.clone();
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

            match util::create_index_for_file(read_input_path, &rconfig) {
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
    // based on the original input path.
    // For the joined path, the coverage-validated stats CSV content is
    // captured here so it does NOT have to be re-read (and possibly observed
    // short) further below.
    let mut prevalidated_stats_content: Option<String> = None;
    let stats_csv_path = if temp_joined_path.is_some() {
        // Joined datasets: compute stats on the joined CSV. Run `qsv stats`
        // and capture its CSV output straight from the child's stdout pipe
        // rather than routing it through a `--output <tmpfile>` round-trip.
        // `Command::output()` drains the pipe to EOF before returning, so
        // the bytes handed back are guaranteed complete — there is no
        // filesystem read-after-write window for a follow-up open() to
        // observe short. This replaces the old fsync-and-retry loop.
        let actual_input_path_str = actual_input_path
            .to_str()
            .ok_or_else(|| CliError::Other("Invalid joined path".to_string()))?
            .to_string();

        let qsv_path = env::current_exe()
            .map_err(|e| CliError::Other(format!("Failed to get current executable path: {e:?}")))?
            .to_string_lossy()
            .to_string();

        // `qsv stats` writes the stats CSV to stdout when no --output is
        // given; capture it in memory. A caller-supplied -o/--output in
        // --stats-options was already rejected up front (see the
        // --join-inputs guard above), so stdout is the stats CSV here.
        let stats_args_vec = build_stats_args(
            &args.flag_stats_options,
            lower_percentile.zip(upper_percentile),
        );
        let mut cmd = Command::new(&qsv_path);
        cmd.arg("stats")
            .args(&stats_args_vec)
            .arg(&actual_input_path_str);
        let output = cmd
            .output()
            .map_err(|e| CliError::Other(format!("Error while executing stats command: {e:?}")))?;
        if !output.status.success() {
            // Omit stdout: on the joined path it carries the (potentially
            // large) stats CSV, and qsv reports errors on stderr anyway.
            return fail_clierror!(
                "Command stats failed: Output {{ status: {:?}, stderr: {:?} }}",
                output.status,
                String::from_utf8_lossy(&output.stderr)
            );
        }
        let stats_content = String::from_utf8(output.stdout)
            .map_err(|e| CliError::Other(format!("Joined-stats output is not valid UTF-8: {e}")))?;

        // Verify the stats output has a record for every column of the
        // joined CSV. `joined_csv_header` was read and validated inside
        // `join_datasets_internal`. Because `stats_content` is the
        // complete, pipe-drained subprocess output (not a possibly-short
        // file re-read), a missing column here is a genuine join/stats
        // failure — fail loudly, no retry needed.
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(stats_content.as_bytes());
        let hdrs: Vec<String> = rdr
            .headers()
            .map_err(|e| CliError::Other(format!("Failed to read joined-stats header: {e}")))?
            .iter()
            .map(std::string::ToString::to_string)
            .collect();
        let field_idx = hdrs.iter().position(|h| h == "field").ok_or_else(|| {
            CliError::Other(format!(
                "Joined-stats output missing 'field' column: got headers {hdrs:?}"
            ))
        })?;
        // Propagate CSV parse errors instead of silently dropping them — a
        // malformed stats row must surface as "failed to parse row N"
        // rather than as a downstream "missing columns" assertion.
        //
        // Count OCCURRENCES per field name (not just set membership): qsv
        // `join` keeps the join key on both sides, so a self-join leaves
        // the joined CSV with duplicate column names (e.g. two `id`
        // columns). A set-based check accepts a stats output that's
        // missing one of those duplicates — but the field_pairs loop
        // downstream needs ONE stats record per joined-CSV column to
        // build all pairs. Using a multiset catches "off by one duplicate"
        // corruption that a HashSet silently masks.
        let mut stats_field_counts: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();
        for (row_idx, rec) in rdr.records().enumerate() {
            let rec = rec.map_err(|e| {
                CliError::Other(format!(
                    "Failed to parse joined-stats row {row}: {e}",
                    row = row_idx + 1, // +1 so numbering matches a human reader (header is row 0)
                ))
            })?;
            if let Some(v) = rec.get(field_idx) {
                *stats_field_counts.entry(v.to_string()).or_insert(0) += 1;
            }
        }
        let mut joined_header_counts: std::collections::HashMap<&str, usize> =
            std::collections::HashMap::new();
        for h in &joined_csv_header {
            *joined_header_counts.entry(h.as_str()).or_insert(0) += 1;
        }
        let mut undercounted: Vec<(String, usize, usize)> = joined_header_counts
            .iter()
            .filter_map(|(name, expected)| {
                let got = stats_field_counts.get(*name).copied().unwrap_or(0);
                (got < *expected).then(|| ((*name).to_string(), *expected, got))
            })
            .collect();
        if !undercounted.is_empty() {
            // Sort by (expected desc, name asc) for deterministic diagnostics.
            undercounted.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
            return fail_clierror!(
                "Joined-stats subprocess output is undercounted for columns {undercounted:?} \
                 (each entry is (name, expected, got)): the joined CSV has {} column(s) \
                 ({joined_csv_header:?}), but the stats output's `field` column distribution is \
                 {stats_field_counts:?}",
                joined_csv_header.len()
            );
        }

        prevalidated_stats_content = Some(stats_content);

        // Reserve a unique temp path for the augmented-stats main output
        // (written further below; it defaults here when the user gave no
        // --output). Nothing reads this path as input, so only the path —
        // not a populated file — matters.
        tempfile::Builder::new()
            .suffix(".stats.csv")
            .tempfile_in(
                crate::config::TEMP_FILE_DIR
                    .get_or_init(|| tempfile::TempDir::new().unwrap().keep()),
            )?
            .into_temp_path()
            .keep()
            .map_err(|e| CliError::Other(format!("Failed to persist temp stats path: {e}")))?
    } else {
        // For single dataset, use normal stats CSV path
        let path = get_stats_csv_path(input_path)?;

        // Check if the stats CSV exists AND is newer than the input; if not, run stats.
        // Existence alone is not enough: a stats CSV left over from an earlier version of
        // the input would otherwise be used as the baseline for every derived statistic.
        let stats_current = !args.flag_force && util::stats_csv_is_current(&path, input_path);
        // A current stats CSV can still be unusable for --use-percentiles: it
        // may have been computed with a --percentile-list that lacks the
        // requested --pct-thresholds (issue #4455). Treat that as stale so the
        // percentile labels the winsorized/trimmed lookups need are present.
        let covers_thresholds = !stats_current
            || if let (Some(lower), Some(upper)) = (lower_percentile, upper_percentile) {
                stats_csv_covers_percentiles(
                    &path,
                    &fmt_pct(lower),
                    &fmt_pct(upper),
                    &stats_separator,
                )
            } else {
                true
            };
        if !stats_current || !covers_thresholds {
            if args.flag_force {
                winfo!("Force flag set: recomputing stats...");
            } else if !stats_current {
                if path.exists() {
                    wwarn!(
                        "Stats CSV file is older than the input: {}\nRecomputing baseline stats...",
                        path.display()
                    );
                } else {
                    wwarn!(
                        "Stats CSV file not found: {}\nComputing baseline stats...",
                        path.display()
                    );
                }
            } else {
                wwarn!(
                    "Stats CSV file was computed with a --percentile-list that does not include \
                     the requested --pct-thresholds: {}\nRecomputing baseline stats...",
                    path.display()
                );
            }

            // Parse stats options, forwarding the --pct-thresholds percentiles
            let stats_args_vec = build_stats_args(
                &args.flag_stats_options,
                lower_percentile.zip(upper_percentile),
            );
            let stats_args_refs: Vec<&str> = stats_args_vec.iter().map(String::as_str).collect();
            let _ = util::run_qsv_cmd(
                "stats",
                &stats_args_refs,
                &input_path_str,
                "Ran stats command to generate baseline stats...",
            )?;
            if !path.exists() {
                return fail_clierror!("Stats CSV file was not created: {}", path.display());
            }
            // Force the freshly-written stats CSV's bytes to disk so a
            // subsequent read sees the full file (defensive fsync; same
            // rationale as the joined-stats path above).
            util::sync_subprocess_output(&path)?;
        }

        path
    };

    // Read the stats CSV file. For the joined path we already hold the
    // coverage-validated content captured above, so reuse it instead of
    // performing a second, unvalidated read of the temp stats file.
    let stats_csv_content = match prevalidated_stats_content {
        Some(content) => content,
        None => fs::read_to_string(&stats_csv_path)?,
    };

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
    let n_positive_idx = headers.iter().position(|h| h == "n_positive");
    let n_negative_idx = headers.iter().position(|h| h == "n_negative");
    let n_zero_idx = headers.iter().position(|h| h == "n_zero");
    let kurtosis_idx = headers.iter().position(|h| h == "kurtosis");
    let lower_outer_fence_idx = headers.iter().position(|h| h == "lower_outer_fence");
    let lower_inner_fence_idx = headers.iter().position(|h| h == "lower_inner_fence");
    let upper_inner_fence_idx = headers.iter().position(|h| h == "upper_inner_fence");
    let upper_outer_fence_idx = headers.iter().position(|h| h == "upper_outer_fence");
    let percentiles_idx = headers.iter().position(|h| h == "percentiles");

    // Parse and validate scan mode for Gregorian XSD date type detection
    let scan_mode = args.flag_xsd_gdate_scan.as_deref().unwrap_or("quick");
    if scan_mode != "quick" && scan_mode != "thorough" {
        return fail_clierror!(
            "Invalid scan mode: {}. Must be either 'quick' or 'thorough'",
            scan_mode
        );
    }

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

    // Trimean: (Q1 + 2*median + Q3) / 4 - Tukey's robust central tendency estimator
    if q1_idx.is_some()
        && (median_idx.is_some() || q2_median_idx.is_some())
        && q3_idx.is_some()
        && !column_exists("trimean")
    {
        new_columns.push("trimean".to_string());
        new_column_indices.insert("trimean".to_string(), new_columns.len() - 1);
    }

    // Midhinge: (Q1 + Q3) / 2 - midpoint of the middle 50%
    if q1_idx.is_some() && q3_idx.is_some() && !column_exists("midhinge") {
        new_columns.push("midhinge".to_string());
        new_column_indices.insert("midhinge".to_string(), new_columns.len() - 1);
    }

    // Robust CV: MAD / median - outlier-resistant coefficient of variation
    if mad_idx.is_some()
        && (median_idx.is_some() || q2_median_idx.is_some())
        && !column_exists("robust_cv")
    {
        new_columns.push("robust_cv".to_string());
        new_column_indices.insert("robust_cv".to_string(), new_columns.len() - 1);
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

    // Add Jarque-Bera test statistic (requires skewness and kurtosis)
    // Only add if --advanced flag is set. Kurtosis can come from this run (new column)
    // or from a previous run (existing column in stats CSV).
    if args.flag_advanced
        && skewness_idx.is_some()
        && (new_column_indices.contains_key("kurtosis") || kurtosis_idx.is_some())
        && n_positive_idx.is_some()
        && n_negative_idx.is_some()
        && n_zero_idx.is_some()
        && !column_exists("jarque_bera")
    {
        new_columns.push("jarque_bera".to_string());
        new_column_indices.insert("jarque_bera".to_string(), new_columns.len() - 1);
        new_columns.push("jarque_bera_pvalue".to_string());
        new_column_indices.insert("jarque_bera_pvalue".to_string(), new_columns.len() - 1);
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

    // Add Theil Index column (requires reading raw data, computed for numeric/date types)
    // Only add if --advanced flag is set
    if args.flag_advanced && !column_exists("theil_index") {
        new_columns.push("theil_index".to_string());
        new_column_indices.insert("theil_index".to_string(), new_columns.len() - 1);
    }

    // Add Mean Absolute Deviation from mean (requires reading raw data)
    // Only add if --advanced flag is set
    if args.flag_advanced && !column_exists("mean_ad") {
        new_columns.push("mean_ad".to_string());
        new_column_indices.insert("mean_ad".to_string(), new_columns.len() - 1);
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

    // Simpson's Diversity Index: 1 - Σ(p_i²)
    // Computed alongside entropy from frequency data, works for all field types
    if new_column_indices.contains_key("shannon_entropy")
        && !column_exists("simpsons_diversity_index")
    {
        new_columns.push("simpsons_diversity_index".to_string());
        new_column_indices.insert(
            "simpsons_diversity_index".to_string(),
            new_columns.len() - 1,
        );
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
            let pct_str = format!("{}pct", fmt_pct(lower_pct));
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
            "trimean",
            "midhinge",
            "robust_cv",
            "kurtosis",
            "bimodality_coefficient",
            "jarque_bera",
            "jarque_bera_pvalue",
            "gini_coefficient",
            // atkinson_index headers are parameterized (e.g. atkinson_index_(1)) and
            // are matched separately via the starts_with("atkinson_index_") check below.
            "theil_index",
            "mean_ad",
            "shannon_entropy",
            "normalized_entropy",
            "simpsons_diversity_index",
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
    let needs_outlier_counting = new_column_indices.contains_key("outliers_extreme_lower_cnt");
    let needs_winsorized_trimmed = new_column_indices.contains_key(winsorized_col_name.as_str())
        || new_column_indices.contains_key(trimmed_col_name.as_str());

    // Collect fields that need Kurtosis, Gini & Atkinson Index computation
    // (with their precalculated stats)
    let needs_kga = new_column_indices.contains_key("kurtosis")
        || new_column_indices.contains_key("gini_coefficient")
        || new_column_indices.contains_key(&atkinson_index_col_name)
        || new_column_indices.contains_key("theil_index")
        || new_column_indices.contains_key("mean_ad");

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
                    let lower_pct_str = fmt_pct(lower_pct);
                    let upper_pct_str = fmt_pct(upper_pct);

                    let lower_val = parse_percentile_value(
                        percentiles_str,
                        &lower_pct_str,
                        field_type,
                        &stats_separator,
                        prefer_dmy,
                    );
                    let upper_val = parse_percentile_value(
                        percentiles_str,
                        &upper_pct_str,
                        field_type,
                        &stats_separator,
                        prefer_dmy,
                    );

                    // Backstop for issue #4455: a label missing from a
                    // non-empty percentiles cell means the stats CSV was
                    // computed with a --percentile-list lacking the requested
                    // thresholds. This used to fall through silently - the
                    // field was still counted (its fences were present) but
                    // with 0.0 thresholds, so winsorized/trimmed statistics
                    // came out 0 or partially winsorized at exit 0. The
                    // percentile list is now forwarded to the stats run, so
                    // this should be unreachable; fail loudly if it isn't.
                    if needs_winsorized_trimmed && !percentiles_str.is_empty() {
                        for (bound, label, val) in [
                            ("lower", &lower_pct_str, lower_val),
                            ("upper", &upper_pct_str, upper_val),
                        ] {
                            if val.is_none()
                                && !percentile_entry_present(
                                    percentiles_str,
                                    label,
                                    &stats_separator,
                                )
                            {
                                return fail_clierror!(
                                    "Percentile {label} (the {bound} --pct-thresholds bound) is \
                                     not among the percentiles computed for field {field_name:?}: \
                                     {percentiles_str:?}. The stats CSV was computed with a \
                                     --percentile-list that does not include it. Re-run with \
                                     --force, or align --percentile-list in --stats-options with \
                                     --pct-thresholds."
                                );
                            }
                        }
                    }

                    (lower_val, upper_val)
                } else {
                    (None, None)
                }
            } else {
                // Use Q1/Q3
                let q1_val = if field_type.is_date_or_datetime() {
                    q1_idx
                        .and_then(|idx| record.get(idx))
                        .and_then(|s| parse_date_to_days(s, prefer_dmy))
                } else {
                    q1_idx
                        .and_then(|idx| record.get(idx))
                        .and_then(parse_float_opt)
                };
                let q3_val = if field_type.is_date_or_datetime() {
                    q3_idx
                        .and_then(|idx| record.get(idx))
                        .and_then(|s| parse_date_to_days(s, prefer_dmy))
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

    // Build slot-ordered field vectors for the FUSED outlier + KGA pass. Both
    // outliers and KGA scan the same original CSV over the same numeric/date
    // columns, so they are computed together in a SINGLE (chunked, parallel) pass
    // instead of two independent full-file reads. A single header read maps each
    // field name to its column index (dropping fields not present in the CSV).
    let (outlier_fields, outlier_names, kga_fields, kga_names) = {
        // through `read_conf`, NOT a bare `ReaderBuilder`: a raw builder defaults to a COMMA
        // delimiter, so a tab/semicolon-delimited input (including a `.tsv` extracted from a
        // `.zip`) parses its whole header as ONE field. No field name then matches, and every
        // outlier/KGA field is SILENTLY dropped - empty gini/atkinson/outlier columns with no
        // error. `read_conf` carries the resolved temp's real delimiter.
        let mut csv_rdr = read_conf.reader()?;
        let csv_headers = csv_rdr.headers()?.clone();
        // First occurrence wins for duplicate header names, matching the prior
        // `csv_headers.iter().position(|h| h == field_name)` (first-match) semantics
        // — a plain `.collect()` would keep the LAST duplicate instead.
        let mut header_pos: HashMap<&str, usize> = HashMap::with_capacity(csv_headers.len());
        for (idx, h) in csv_headers.iter().enumerate() {
            header_pos.entry(h).or_insert(idx);
        }

        let mut o_fields = Vec::with_capacity(fields_to_count.len());
        let mut o_names = Vec::with_capacity(fields_to_count.len());
        for (name, mut info) in fields_to_count {
            if let Some(&col_idx) = header_pos.get(name.as_str()) {
                info.col_idx = col_idx;
                o_fields.push(info);
                o_names.push(name);
            }
        }

        let mut k_fields = Vec::with_capacity(fields_for_kga.len());
        let mut k_names = Vec::with_capacity(fields_for_kga.len());
        for (name, mut info) in fields_for_kga {
            if let Some(&col_idx) = header_pos.get(name.as_str()) {
                info.col_idx = col_idx;
                k_fields.push(info);
                k_names.push(name);
            }
        }

        (o_fields, o_names, k_fields, k_names)
    };

    // Single fused pass: outlier counting + Kurtosis/Gini/Atkinson in one scan
    // (parallel when an index exists and the file is large enough).
    let (outlier_counts, kga_stats) = if outlier_fields.is_empty() && kga_fields.is_empty() {
        (HashMap::new(), HashMap::new())
    } else {
        compute_outliers_and_kga(
            outlier_fields,
            outlier_names,
            kga_fields,
            kga_names,
            read_input_path,
            args.flag_jobs,
            args.flag_epsilon,
        )?
    };

    // Compute Shannon Entropy for all fields
    let entropy_stats = if new_column_indices.contains_key("shannon_entropy") {
        compute_all_entropy(read_input_path)?
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
            // the resolved Config, so the count is of the DATA, not the compressed container
            let rconfig = read_conf.clone();
            if let Ok(Some(idx)) = rconfig.indexed() {
                Some(idx.count())
            } else if !rconfig.is_stdin() {
                // Fall back to counting rows if no index
                util::count_rows(&rconfig).ok()
            } else {
                None // Can't get count from stdin
            }
        };

        // Collect all field names from the stats CSV (the `field` column has
        // one row per column of the joined CSV, regardless of type).
        // Computed before the header read so the joined-input path can
        // verify the joined CSV's header covers every stats field.
        let stats_field_names: Vec<String> = records
            .iter()
            .filter_map(|r| {
                field_idx
                    .and_then(|idx| r.get(idx))
                    .map(std::string::ToString::to_string)
            })
            .collect();

        // Read the input/joined CSV header to map field names to column
        // indices. Wrapped in a closure so the joined-input path can retry
        // the read after an fsync.
        // via `read_conf` so the resolved temp's real delimiter is used (see the outlier/KGA
        // header read above); a bare ReaderBuilder would assume a comma.
        let read_csv_headers = || -> CliResult<StringRecord> {
            let mut csv_rdr = read_conf.reader()?;
            Ok(csv_rdr.headers()?.clone())
        };

        let csv_headers = if temp_joined_path.is_some() {
            // For joined inputs, the stats CSV was freshly computed FROM the
            // joined CSV, so every stats field MUST appear in the joined
            // CSV's header. A column missing here means this follow-up read
            // saw a short/stale view of the joined temp file — the same
            // page-cache race already guarded against in
            // `join_datasets_internal` and the joined-stats block. fsync and
            // retry once; if a column is still missing, fail loud rather
            // than silently dropping bivariate pairs (lines below `continue`
            // on a header miss) and emitting "primary-only" join-corrupt
            // output.
            util::sync_subprocess_output(read_input_path)?;
            let missing_cols = |hdrs: &StringRecord| -> Vec<String> {
                let header_set: std::collections::HashSet<&str> = hdrs.iter().collect();
                stats_field_names
                    .iter()
                    .filter(|f| !header_set.contains(f.as_str()))
                    .cloned()
                    .collect()
            };
            let mut headers = read_csv_headers()?;
            let mut missing = missing_cols(&headers);
            if !missing.is_empty() {
                log::warn!(
                    "Joined CSV header missing columns {missing:?} present in the stats output; \
                     re-syncing joined CSV and re-reading its header once"
                );
                util::sync_subprocess_output(read_input_path)?;
                headers = read_csv_headers()?;
                missing = missing_cols(&headers);
                if !missing.is_empty() {
                    return fail_clierror!(
                        "Joined CSV header is still missing columns {missing:?} after one retry: \
                         the stats output covers {} column(s) ({stats_field_names:?}), but the \
                         joined CSV header only has {:?}. This indicates silent join corruption — \
                         aborting instead of emitting primary-only bivariate output.",
                        stats_field_names.len(),
                        headers.iter().collect::<Vec<_>>()
                    );
                }
            }
            headers
        } else {
            read_csv_headers()?
        };

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

        // Diagnostic counters: surface WHY pairs get dropped. The
        // field_pairs construction loop has six silent `continue` branches.
        // When CI sees only primary-side pairs in the bivariate output
        // (recurring flake on joined-inputs tests), we have no signal as
        // to which filter fired. Track each rejection reason so a future
        // flake leaves an actionable trail.
        let mut skipped_field1_bad_type: u64 = 0;
        let mut skipped_field1_missing_in_csv: u64 = 0;
        let mut skipped_field2_bad_type: u64 = 0;
        let mut skipped_field2_missing_in_csv: u64 = 0;
        let mut skipped_zero_variance: u64 = 0;
        let mut skipped_both_constant: u64 = 0;
        let mut skipped_card_eq_rowcount: u64 = 0;
        let mut skipped_type_filter: u64 = 0;

        for (i, field1_name) in stats_field_names.iter().enumerate() {
            let field1_type_str = records.get(i).and_then(|r| r.get(type_idx)).unwrap_or("");
            let Some(field1_type) = FieldType::from_str(field1_type_str) else {
                skipped_field1_bad_type += 1;
                // log::warn! (not wwarn!): per-pair skip details are
                // gated behind QSV_LOG_LEVEL so they don't flood stderr
                // during routine bivariate runs — zero-variance,
                // both-constant, all-unique, and type-filter skips are
                // normal for many datasets and the loop visits O(n²)
                // pairs. The post-loop winfo! summary already carries
                // aggregate per-reason skip counts on stderr, which is
                // enough signal to spot the corruption mode without
                // dragging the full pair trail into every healthy run.
                // Set QSV_LOG_LEVEL=warn to surface these per-pair
                // diagnostics when actually debugging a flake.
                log::warn!(
                    "bivariate field_pairs: skipping field1={field1_name:?} (i={i}): unrecognized \
                     type {field1_type_str:?}"
                );
                continue;
            };

            // Get column index for field1
            let Some(field1_col_idx) = csv_headers.iter().position(|h| h == field1_name) else {
                skipped_field1_missing_in_csv += 1;
                log::warn!(
                    "bivariate field_pairs: skipping field1={field1_name:?} (i={i}): name not \
                     found in csv_headers (len={hdr_len})",
                    hdr_len = csv_headers.len()
                );
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
                    skipped_field2_bad_type += 1;
                    log::warn!(
                        "bivariate field_pairs: skipping field2={field2_name:?} (i={i}, j={j}) \
                         with field1={field1_name:?}: unrecognized type {field2_type_str:?}"
                    );
                    continue;
                };

                // Get column index for field2
                let Some(field2_col_idx) = csv_headers.iter().position(|h| h == field2_name) else {
                    skipped_field2_missing_in_csv += 1;
                    log::warn!(
                        "bivariate field_pairs: skipping field2={field2_name:?} (i={i}, j={j}) \
                         with field1={field1_name:?}: name not found in csv_headers \
                         (len={hdr_len})",
                        hdr_len = csv_headers.len()
                    );
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
                        skipped_zero_variance += 1;
                        log::warn!(
                            "bivariate field_pairs: skipping ({field1_name:?}, {field2_name:?}) \
                             (i={i}, j={j}): zero stddev (s1={stddev1}, s2={stddev2})"
                        );
                        continue; // Skip pairs with constant fields (correlation undefined)
                    }
                } else if let (Some(var1), Some(var2)) = (field1_variance, field2_variance)
                    && (var1.abs() < f64::EPSILON || var2.abs() < f64::EPSILON)
                {
                    skipped_zero_variance += 1;
                    log::warn!(
                        "bivariate field_pairs: skipping ({field1_name:?}, {field2_name:?}) \
                         (i={i}, j={j}): zero variance (v1={var1}, v2={var2})"
                    );
                    continue; // Skip pairs with constant fields (correlation undefined)
                }

                // Filter invalid pairs: skip both-constant pairs (cardinality = 1 for both)
                if let (Some(card1), Some(card2)) = (field1_cardinality, field2_cardinality)
                    && card1 == 1
                    && card2 == 1
                {
                    skipped_both_constant += 1;
                    log::warn!(
                        "bivariate field_pairs: skipping ({field1_name:?}, {field2_name:?}) \
                         (i={i}, j={j}): both cardinalities == 1"
                    );
                    continue; // Both constant, no meaningful correlation
                }

                // Filter invalid pairs: skip fields with all unique values (cardinality ==
                // rowcount)
                if let Some(rowcount) = record_count
                    && (field1_cardinality.is_some_and(|c| c == rowcount)
                        || field2_cardinality.is_some_and(|c| c == rowcount))
                {
                    skipped_card_eq_rowcount += 1;
                    log::warn!(
                        "bivariate field_pairs: skipping ({field1_name:?}, {field2_name:?}) \
                         (i={i}, j={j}): cardinality == rowcount ({rowcount}) \
                         (c1={field1_cardinality:?}, c2={field2_cardinality:?})",
                    );
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
                                stddev:      field1_stddev,
                                variance:    field1_variance,
                                cardinality: field1_cardinality,
                            },
                            BivariateFieldInfo {
                                col_idx:     field2_col_idx,
                                field_type:  field2_type,
                                stddev:      field2_stddev,
                                variance:    field2_variance,
                                cardinality: field2_cardinality,
                            },
                        ),
                    );
                } else {
                    skipped_type_filter += 1;
                    log::warn!(
                        "bivariate field_pairs: skipping ({field1_name:?}, {field2_name:?}) \
                         (i={i}, j={j}): neither field passes the numeric/date/string filter \
                         (t1={field1_type:?}, t2={field2_type:?})"
                    );
                }
            }
        }

        let total_skipped = skipped_field1_bad_type
            + skipped_field1_missing_in_csv
            + skipped_field2_bad_type
            + skipped_field2_missing_in_csv
            + skipped_zero_variance
            + skipped_both_constant
            + skipped_card_eq_rowcount
            + skipped_type_filter;
        if total_skipped > 0 || field_pairs.is_empty() {
            // Always log a summary when something was skipped or when no
            // pairs survived — this is the diagnostic trail for the
            // recurring "primary-only bivariate output" flake. winfo!
            // (not log::info!) writes to stderr unconditionally; qsv's
            // default log level is `off`, so a bare log::info! would
            // disappear in CI. This single line carries the aggregate
            // per-reason skip counts and the full csv_headers — enough
            // signal to spot the corruption mode without dragging the
            // per-pair trail into every healthy run. Per-pair detail is
            // emitted via log::warn! (gated by QSV_LOG_LEVEL) above.
            winfo!(
                "bivariate field_pairs: built {built} pair(s) from {nfields} stats fields \
                 (record_count={record_count:?}); skipped: \
                 field1_bad_type={skipped_field1_bad_type}, \
                 field1_missing_in_csv={skipped_field1_missing_in_csv}, \
                 field2_bad_type={skipped_field2_bad_type}, \
                 field2_missing_in_csv={skipped_field2_missing_in_csv}, \
                 zero_variance={skipped_zero_variance}, both_constant={skipped_both_constant}, \
                 card_eq_rowcount={skipped_card_eq_rowcount}, type_filter={skipped_type_filter}; \
                 csv_headers={csv_headers:?}",
                built = field_pairs.len(),
                nfields = stats_field_names.len(),
                csv_headers = csv_headers.iter().collect::<Vec<_>>()
            );
        }

        // In joined-inputs mode, fail loud when the corruption signature
        // fires: NO surviving pair touches a secondary-side column that
        // would actually be pairable under the loop's own filter rules.
        // This is the precise failure mode of the recurring CI flake
        // (e.g. moarstats_join_type_left_runs_and_writes_bivariate) —
        // the bivariate output silently contains only primary-side
        // pairs, which downstream produces a confusing "missing column"
        // assertion in the test rather than pointing at moarstats.
        //
        // The guard's "pairable" check mirrors the COLUMN-LEVEL filters
        // the loop applies, so a column that the loop legitimately
        // filtered does not count against the guard:
        //   - Type is recognized AND passes the bivariate type filter (numeric / date / string).
        //   - Stddev/variance is non-zero (when reported) — zero variance makes the column
        //     ineligible in ALL pairs.
        //   - Cardinality != record_count (when both known) — equal cardinality also makes the
        //     column ineligible in ALL pairs.
        // The both-constant (cardinality == 1) filter is intentionally
        // omitted: it's genuinely pair-level (only fires when BOTH columns
        // have cardinality 1), so a card==1 column may still legitimately
        // pair with a higher-cardinality partner.
        //
        // The guard also excludes:
        //   - Columns that share a csv_headers position with primary (e.g. the join key) — never
        //     exclusively secondary.
        //   - Aliased duplicate names whose first-match position is elsewhere — field_pairs cannot
        //     key on those indices anyway.
        //
        // It fires only when at least one pairable secondary-only column
        // exists AND none of them appears in any pair.
        if temp_joined_path.is_some() && !field_pairs.is_empty() {
            // Build the set of csv_headers indices covered by surviving
            // pairs.
            let mut covered_indices: std::collections::HashSet<u16> =
                std::collections::HashSet::new();
            for (a, b) in field_pairs.keys() {
                covered_indices.insert(*a);
                covered_indices.insert(*b);
            }

            // Read the primary's header ONCE. Used to compute both
            // primary_positions and primary_has_pairable below; the
            // earlier revision opened input_path twice.
            let primary_headers: Vec<String> = primary_conf
                .reader()
                .ok()
                .and_then(|mut r| r.headers().ok().cloned())
                .map(|h| h.iter().map(std::string::ToString::to_string).collect())
                .unwrap_or_default();

            // Name -> first-occurrence position in csv_headers. O(1)
            // lookups replace the linear `csv_headers.iter().position()`
            // scans the previous revision did inside nested loops.
            let csv_name_to_pos: std::collections::HashMap<&str, usize> = {
                let mut m = std::collections::HashMap::with_capacity(csv_headers.len());
                for (idx, name) in csv_headers.iter().enumerate() {
                    m.entry(name).or_insert(idx);
                }
                m
            };

            // Set of csv_headers indices that come from the primary's
            // header — used to identify which positions are exclusively
            // secondary. Computed via the cached name->pos map.
            let primary_positions: std::collections::HashSet<usize> = primary_headers
                .iter()
                .filter_map(|h| csv_name_to_pos.get(h.as_str()).copied())
                .collect();

            // Pair-level pairability: mirrors the construction loop's
            // exact filter rules so the guard does not diverge from the
            // loop's view of which secondary columns are eligible.
            //
            // Key subtlety the previous (column-level) check missed: the
            // loop's zero-stddev/variance filter fires only when BOTH
            // sides have stddev (or both have variance). A zero-stddev
            // numeric column paired with a TString partner (no
            // stddev/variance) is NOT skipped. Treating zero-stddev as
            // an unconditional column-level disqualifier would mask
            // corruption in mixed string/numeric joined datasets.
            //
            // pair_compatible mirrors:
            //   - Zero stddev/variance filter (the loop's two-branch form)
            //   - Both-constant (cardinality == 1) — only fires when BOTH
            //   - Cardinality == record_count — fires when EITHER side
            //   - Pair type filter — at least one side must be numeric/date/string
            #[allow(clippy::items_after_statements)]
            #[derive(Clone)]
            struct ColInfo {
                field_type:  FieldType,
                stddev:      Option<f64>,
                variance:    Option<f64>,
                cardinality: Option<u64>,
            }
            // Parse every stats record's column attributes ONCE. Indexed
            // by stats_field_names position. Avoids re-parsing the same
            // record many times across nested pairability checks.
            let col_infos: Vec<Option<ColInfo>> = (0..stats_field_names.len())
                .map(|si| {
                    let rec = records.get(si)?;
                    let type_str = rec.get(type_idx).unwrap_or("");
                    let field_type = FieldType::from_str(type_str)?;
                    let stddev = stddev_idx
                        .and_then(|i| rec.get(i))
                        .and_then(parse_float_opt);
                    let variance = variance_idx
                        .and_then(|i| rec.get(i))
                        .and_then(parse_float_opt);
                    let cardinality = cardinality_idx
                        .and_then(|i| rec.get(i))
                        .and_then(|s| s.parse::<u64>().ok());
                    Some(ColInfo {
                        field_type,
                        stddev,
                        variance,
                        cardinality,
                    })
                })
                .collect();
            // First-occurrence name -> stats_record_idx. Mirrors the
            // construction loop's `stats_field_names.iter().position()`
            // first-match semantics for duplicate names (e.g. two `id`
            // entries from a self-join).
            let stats_name_to_idx: std::collections::HashMap<&str, usize> = {
                let mut m = std::collections::HashMap::with_capacity(stats_field_names.len());
                for (idx, name) in stats_field_names.iter().enumerate() {
                    m.entry(name.as_str()).or_insert(idx);
                }
                m
            };

            let pair_compatible = |a: &ColInfo, b: &ColInfo| -> bool {
                if let (Some(s1), Some(s2)) = (a.stddev, b.stddev) {
                    if s1.abs() < f64::EPSILON || s2.abs() < f64::EPSILON {
                        return false;
                    }
                } else if let (Some(v1), Some(v2)) = (a.variance, b.variance)
                    && (v1.abs() < f64::EPSILON || v2.abs() < f64::EPSILON)
                {
                    return false;
                }
                if let (Some(c1), Some(c2)) = (a.cardinality, b.cardinality)
                    && c1 == 1
                    && c2 == 1
                {
                    return false;
                }
                if let Some(rc) = record_count
                    && (a.cardinality.is_some_and(|c| c == rc)
                        || b.cardinality.is_some_and(|c| c == rc))
                {
                    return false;
                }
                a.field_type.is_numeric_or_date_type()
                    || b.field_type.is_numeric_or_date_type()
                    || a.field_type == FieldType::TString
                    || b.field_type == FieldType::TString
            };
            // A column at `stats_idx` is "pairable" if at least one OTHER
            // entry in col_infos forms a compatible pair with it under
            // the construction loop's exact rules. One linear pass over
            // col_infos per call; the surrounding caller pays O(n) per
            // column tested, so overall O(n^2) — down from the previous
            // revision's superlinear behavior driven by repeated
            // `stats_field_names.iter().position()` scans.
            let is_pairable_at = |stats_idx: usize| -> bool {
                let Some(col) = col_infos.get(stats_idx).and_then(|c| c.as_ref()) else {
                    return false;
                };
                col_infos.iter().enumerate().any(|(other_idx, other)| {
                    other_idx != stats_idx
                        && other.as_ref().is_some_and(|p| pair_compatible(col, p))
                })
            };
            let is_pairable_by_name = |name: &str| -> bool {
                stats_name_to_idx
                    .get(name)
                    .copied()
                    .is_some_and(is_pairable_at)
            };

            // Collect pairable secondary-only positions from each
            // additional input. We DON'T re-check `position(|jh| jh ==
            // h) == Some(idx)` after the cached lookup — that compared
            // first-match against the same first-match and was always
            // true. The `primary_positions.contains(&idx)` check already
            // rejects columns whose csv_headers position is shared with
            // primary (the join-key alias case).
            let additional_inputs_for_guard: Vec<String> = args
                .flag_join_inputs
                .as_ref()
                .map(|s| s.split(',').map(|p| p.trim().to_string()).collect())
                .unwrap_or_default();
            let mut pairable_secondary_only: Vec<(String, String, usize)> = Vec::new();
            for add_path in &additional_inputs_for_guard {
                // via `Config` so a special-format secondary is decompressed first; a raw
                // open would fail here and be SWALLOWED by the `else { continue }`,
                // silently dropping that input's pairable columns from the guard.
                let Ok(mut add_rdr) = Config::new(Some(add_path)).reader() else {
                    continue;
                };
                let Ok(add_hdrs) = add_rdr.headers() else {
                    continue;
                };
                for h in add_hdrs {
                    let Some(&idx) = csv_name_to_pos.get(h) else {
                        continue;
                    };
                    if primary_positions.contains(&idx) {
                        continue;
                    }
                    if !is_pairable_by_name(h) {
                        continue;
                    }
                    pairable_secondary_only.push((add_path.clone(), h.to_string(), idx));
                }
            }

            // Skip the guard entirely when no primary column passes the
            // same pairability check: no pair would survive either, and
            // the empty/sparse output is then a property of the data,
            // not corruption.
            let primary_has_pairable = primary_headers
                .iter()
                .any(|h| is_pairable_by_name(h.as_str()));

            // Fire only when both sides have pairable columns AND no
            // pairable secondary-only column is covered. Both sides
            // having pairable columns is what the construction loop
            // would also need to produce a secondary-touching pair — so
            // if the loop didn't, that's the corruption.
            if primary_has_pairable
                && !pairable_secondary_only.is_empty()
                && !pairable_secondary_only
                    .iter()
                    .any(|(_, _, idx)| covered_indices.contains(&(*idx as u16)))
            {
                return fail_clierror!(
                    "Bivariate field_pairs built {} pair(s) but none touches any pairable \
                     secondary-side column: {pairable_secondary_only:?} (each entry is (input, \
                     column, csv_header_idx)). Primary input has at least one pairable column. \
                     This is the recurring primary-only-bivariate corruption mode; refusing to \
                     write a misleading bivariate output. \
                     stats_field_names={stats_field_names:?}, csv_headers={csv_headers:?}, \
                     record_count={record_count:?}.",
                    field_pairs.len()
                );
            }
        }

        if field_pairs.is_empty() {
            HashMap::new()
        } else {
            // Setup progress bar if requested and not reading from stdin
            let rconfig_bivariate = read_conf.clone();
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

            // Cardinality threshold for mi/nmi/u. The default is relative to the row
            // count because the old fixed 1,000,000 could never fire on anything
            // smaller -- on the 1M-row benchmark it sat exactly at the row count, so
            // the guard was inert on the very file it mattered for.
            //
            // Floored so it stays inert on small inputs: on an 8-row fixture nothing
            // is meaningfully "high cardinality", and half of 8 would prune ordinary
            // 5-value columns. The fully-unique case is already excluded earlier by
            // the cardinality == rowcount filter.
            let cardinality_threshold = args.flag_cardinality_threshold.or_else(|| {
                Some(
                    record_count
                        .map_or(DEFAULT_CARDINALITY_THRESHOLD, |rc| rc / 2)
                        .max(DEFAULT_CARDINALITY_THRESHOLD),
                )
            });

            // Log which stats are being computed
            let stats_list: Vec<&str> = [
                stats_config.pearson.then_some("pearson"),
                stats_config.spearman.then_some("spearman"),
                stats_config.kendall.then_some("kendall"),
                stats_config.covariance.then_some("covariance"),
                stats_config.mi.then_some("mi"),
                stats_config.nmi.then_some("nmi"),
                stats_config.u.then_some("u"),
            ]
            .into_iter()
            .flatten()
            .collect();
            winfo!(
                "Computing bivariate statistics: {}...",
                stats_list.join(", ")
            );

            let result = compute_all_bivariatestats(
                field_pairs,
                &field_names,
                read_input_path,
                progress.as_ref(),
                cardinality_threshold,
                stats_config,
                args.flag_jobs,
                args.flag_bivariate_batch,
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
        if stats_config.u {
            headers.push("u_field2_given_field1");
            headers.push("u_field1_given_field2");
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
                if stats_config.u {
                    record.push(
                        stats
                            .u_field2_given_field1
                            .map_or(String::new(), |v| util::round_num(v, args.flag_round)),
                    );
                    record.push(
                        stats
                            .u_field1_given_field2
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

    // Pre-compute derived column names for winsorized/trimmed stats
    // to avoid repeated String::replace() allocations in the per-record loop
    let winsorized_stddev_name = winsorized_col_name.replace("mean", "stddev");
    let winsorized_variance_name = winsorized_col_name.replace("mean", "variance");
    let winsorized_cv_name = winsorized_col_name.replace("mean", "cv");
    let winsorized_base = winsorized_col_name.replace("mean", "").replace("__", "_");
    let winsorized_stddev_ratio_name =
        format!("{}_stddev_ratio", winsorized_base.trim_end_matches('_'));
    let winsorized_range_name = winsorized_col_name.replace("mean", "range");
    let trimmed_stddev_name = trimmed_col_name.replace("mean", "stddev");
    let trimmed_variance_name = trimmed_col_name.replace("mean", "variance");
    let trimmed_cv_name = trimmed_col_name.replace("mean", "cv");
    let trimmed_base = trimmed_col_name.replace("mean", "").replace("__", "_");
    let trimmed_stddev_ratio_name = format!("{}_stddev_ratio", trimmed_base.trim_end_matches('_'));
    let trimmed_range_name = trimmed_col_name.replace("mean", "range");

    // Pre-allocate new_values outside the loop and reuse across iterations
    let new_columns_len = new_columns.len();
    let mut new_values: Vec<String> = vec![String::new(); new_columns_len];

    // Process each record
    #[allow(clippy::cast_precision_loss)]
    for record in &records {
        // Get field name and type (skip dataset stats rows that might not have proper type)
        let field_name = field_idx.and_then(|idx| record.get(idx)).unwrap_or("");
        let field_type_str = record.get(type_idx).unwrap_or("");

        // Convert string to enum for efficient comparisons
        let field_type_opt = FieldType::from_str(field_type_str);

        // Clear new_values for reuse (reset each string to empty without deallocating)
        for v in &mut new_values {
            v.clear();
        }

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

            // Extract percentile values for thorough mode
            let (percentile_values, actual_scan_mode) = if scan_mode == "thorough" {
                if let Some(idx) = percentiles_idx {
                    if let Some(percentiles_str) = record.get(idx) {
                        if percentiles_str.is_empty() {
                            // Empty percentile string, fall back to quick
                            (None, "quick")
                        } else {
                            let values = parse_all_percentile_string_values(
                                percentiles_str,
                                &stats_separator,
                            );
                            if values.is_empty() {
                                // Empty percentile values, fall back to quick
                                (None, "quick")
                            } else {
                                (Some(values), "thorough")
                            }
                        }
                    } else {
                        // No percentile string, fall back to quick
                        (None, "quick")
                    }
                } else {
                    // No percentiles column, fall back to quick
                    (None, "quick")
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
                        parse_date_to_days(s, prefer_dmy)
                    } else {
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
                        parse_date_to_days(s, prefer_dmy)
                    } else {
                        parse_float_opt(s)
                    }
                })
            } else {
                None
            };

            // Infer XSD type (pass all parameters including scan_mode and percentile_values)
            // Use actual_scan_mode which may have fallen back to quick if percentiles unavailable
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

        // Write Simpson's Diversity Index from pre-computed results (works for all field types)
        if let Some(idx) = new_column_indices.get("simpsons_diversity_index")
            && !field_name.is_empty()
            && let Some(entropy_stats_val) = entropy_stats.get(field_name)
            && let Some(simpsons_val) = entropy_stats_val.simpsons_diversity
        {
            new_values[*idx] = util::round_num(simpsons_val, args.flag_round);
        }

        // Only compute other stats for numeric/date types
        let Some(field_type) = field_type_opt else {
            // For unrecognized types, write existing fields + new values directly
            for field in record {
                wtr.write_field(field)?;
            }
            for val in &new_values {
                wtr.write_field(val)?;
            }
            wtr.write_record(None::<&[u8]>)?;
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

            if let Some(idx) = new_column_indices.get("trimean")
                && let Some(val) = compute_trimean(q1, median, q3)
            {
                new_values[*idx] = util::round_num(val, args.flag_round);
            }

            if let Some(idx) = new_column_indices.get("midhinge")
                && let Some(val) = compute_midhinge(q1, q3)
            {
                new_values[*idx] = util::round_num(val, args.flag_round);
            }

            if let Some(idx) = new_column_indices.get("robust_cv")
                && let Some(val) = compute_robust_cv(mad, median)
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

            // Compute Jarque-Bera test (requires skewness and kurtosis)
            // Prefer kurtosis from KGA stats when available, otherwise fall back
            // to the kurtosis value already present in the stats CSV record.
            if new_column_indices.contains_key("jarque_bera") && !field_name.is_empty() {
                let kurtosis_from_kga = kga_stats
                    .get(field_name)
                    .and_then(|kga_stats_val| kga_stats_val.kurtosis);
                let kurtosis_from_stats = kurtosis_idx
                    .and_then(|idx| record.get(idx))
                    .and_then(parse_float_opt);
                let kurtosis_val = kurtosis_from_kga.or(kurtosis_from_stats);

                if let Some(kurtosis_val) = kurtosis_val {
                    let skewness = skewness_idx
                        .and_then(|idx| record.get(idx))
                        .and_then(parse_float_opt);
                    // Compute n from n_positive + n_negative + n_zero
                    let n_pos = n_positive_idx
                        .and_then(|idx| record.get(idx))
                        .and_then(|s| s.parse::<u64>().ok())
                        .unwrap_or(0);
                    let n_neg = n_negative_idx
                        .and_then(|idx| record.get(idx))
                        .and_then(|s| s.parse::<u64>().ok())
                        .unwrap_or(0);
                    let n_z = n_zero_idx
                        .and_then(|idx| record.get(idx))
                        .and_then(|s| s.parse::<u64>().ok())
                        .unwrap_or(0);
                    let n = n_pos + n_neg + n_z;
                    if let Some((jb, pval)) = compute_jarque_bera(skewness, Some(kurtosis_val), n) {
                        if let Some(idx) = new_column_indices.get("jarque_bera") {
                            new_values[*idx] = util::round_num(jb, args.flag_round);
                        }
                        if let Some(idx) = new_column_indices.get("jarque_bera_pvalue") {
                            new_values[*idx] = util::round_num(pval, args.flag_round);
                        }
                    }
                }
            }

            // Get outlier statistics from pre-computed results
            if new_column_indices.contains_key("outliers_extreme_lower_cnt")
                && !field_name.is_empty()
                && let Some(stats) = outlier_counts.get(field_name)
            {
                // Write counts (with _cnt suffix)
                if let Some(idx) = new_column_indices.get("outliers_extreme_lower_cnt") {
                    new_values[*idx] = stats.counts[OUTLIER_EXTREME_LOWER].to_string();
                }
                if let Some(idx) = new_column_indices.get("outliers_mild_lower_cnt") {
                    new_values[*idx] = stats.counts[OUTLIER_MILD_LOWER].to_string();
                }
                if let Some(idx) = new_column_indices.get("outliers_normal_cnt") {
                    new_values[*idx] = stats.counts[OUTLIER_NORMAL].to_string();
                }
                if let Some(idx) = new_column_indices.get("outliers_mild_upper_cnt") {
                    new_values[*idx] = stats.counts[OUTLIER_MILD_UPPER].to_string();
                }
                if let Some(idx) = new_column_indices.get("outliers_extreme_upper_cnt") {
                    new_values[*idx] = stats.counts[OUTLIER_EXTREME_UPPER].to_string();
                }
                if let Some(idx) = new_column_indices.get("outliers_total_cnt") {
                    new_values[*idx] = stats.counts[OUTLIER_TOTAL].to_string();
                }

                // Compute means
                let mean_outliers = if stats.counts[OUTLIER_TOTAL] > 0 {
                    Some(stats.sum_outliers / stats.counts[OUTLIER_TOTAL] as f64)
                } else {
                    None
                };
                let mean_normal = if stats.counts[OUTLIER_NORMAL] > 0 {
                    Some(stats.sum_normal / stats.counts[OUTLIER_NORMAL] as f64)
                } else {
                    None
                };
                let mean_all = if stats.count_all > 0 {
                    Some(stats.sum_all / stats.count_all as f64)
                } else {
                    None
                };

                // Compute outliers variance and stddev once for reuse
                let (variance_outliers, stddev_outliers) = if stats.counts[OUTLIER_TOTAL] > 1 {
                    let n = stats.counts[OUTLIER_TOTAL] as f64;
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
                    if stats.counts[OUTLIER_NORMAL] > 1 {
                        let n = stats.counts[OUTLIER_NORMAL] as f64;
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
                    let percentage =
                        (stats.counts[OUTLIER_TOTAL] as f64 / stats.count_all as f64) * 100.0;
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
                        if let Some(idx) = new_column_indices.get(&winsorized_stddev_name) {
                            new_values[*idx] = util::round_num(winsorized_stddev, args.flag_round);
                        }
                        if let Some(idx) = new_column_indices.get(&winsorized_variance_name) {
                            new_values[*idx] =
                                util::round_num(winsorized_variance, args.flag_round);
                        }
                        // Winsorized coefficient of variation
                        if winsorized_mean_val.abs() > f64::EPSILON
                            && let Some(idx) = new_column_indices.get(&winsorized_cv_name)
                        {
                            let cv = winsorized_stddev / winsorized_mean_val.abs();
                            new_values[*idx] = util::round_num(cv, args.flag_round);
                        }
                        // Winsorized stddev ratio
                        if let Some(stddev_val) = stddev
                            && stddev_val.abs() > f64::EPSILON
                            && let Some(idx) = new_column_indices.get(&winsorized_stddev_ratio_name)
                        {
                            let ratio = winsorized_stddev / stddev_val;
                            new_values[*idx] = util::round_num(ratio, args.flag_round);
                        }
                    }
                }

                // Winsorized range
                if let (Some(min_winsorized), Some(max_winsorized)) =
                    (stats.min_winsorized, stats.max_winsorized)
                    && let Some(idx) = new_column_indices.get(&winsorized_range_name)
                {
                    let range = max_winsorized - min_winsorized;
                    new_values[*idx] = util::round_num(range, args.flag_round);
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
                        if let Some(idx) = new_column_indices.get(&trimmed_stddev_name) {
                            new_values[*idx] = util::round_num(trimmed_stddev, args.flag_round);
                        }
                        if let Some(idx) = new_column_indices.get(&trimmed_variance_name) {
                            new_values[*idx] = util::round_num(trimmed_variance, args.flag_round);
                        }
                        // Trimmed coefficient of variation
                        if trimmed_mean_val.abs() > f64::EPSILON
                            && let Some(idx) = new_column_indices.get(&trimmed_cv_name)
                        {
                            let cv = trimmed_stddev / trimmed_mean_val.abs();
                            new_values[*idx] = util::round_num(cv, args.flag_round);
                        }
                        // Trimmed stddev ratio
                        if let Some(stddev_val) = stddev
                            && stddev_val.abs() > f64::EPSILON
                            && let Some(idx) = new_column_indices.get(&trimmed_stddev_ratio_name)
                        {
                            let ratio = trimmed_stddev / stddev_val;
                            new_values[*idx] = util::round_num(ratio, args.flag_round);
                        }
                    }
                }

                // Trimmed range
                if let (Some(min_trimmed), Some(max_trimmed)) =
                    (stats.min_trimmed, stats.max_trimmed)
                    && let Some(idx) = new_column_indices.get(&trimmed_range_name)
                {
                    let range = max_trimmed - min_trimmed;
                    new_values[*idx] = util::round_num(range, args.flag_round);
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

                // Theil Index
                if let Some(theil_val) = stats.theil_index
                    && let Some(idx) = new_column_indices.get("theil_index")
                {
                    new_values[*idx] = util::round_num(theil_val, args.flag_round);
                }

                // Mean Absolute Deviation from mean
                if let Some(mean_ad_val) = stats.mean_ad
                    && let Some(idx) = new_column_indices.get("mean_ad")
                {
                    new_values[*idx] = util::round_num(mean_ad_val, args.flag_round);
                }
            }
        }
        // Write existing fields + new values directly (avoids record.clone())
        for field in record {
            wtr.write_field(field)?;
        }
        for val in &new_values {
            wtr.write_field(val)?;
        }
        wtr.write_record(None::<&[u8]>)?;
    }

    wtr.flush()?;

    winfo!(
        "Added {} additional statistics columns to {}",
        new_columns.len(),
        output_path.display()
    );
    winfo!("Elapsed: {:.2}s", start_time.elapsed().as_secs_f64());

    // Regenerate the .stats.csv.data.jsonl so downstream "smart" commands
    // (pivotp, schema, etc.) can see moarstats columns
    if let Some(output_path_str) = output_path.to_str() {
        let jsonl_path = PathBuf::from(format!("{output_path_str}.data.jsonl"));
        if let Err(e) = util::csv_to_jsonl(
            output_path_str,
            &crate::cmd::stats::STATSDATA_TYPES_MAP,
            &jsonl_path,
            b',',
        ) {
            wwarn!("Failed to regenerate stats JSONL cache: {e}");
        }
    } else {
        wwarn!(
            "Output path {:?} is not valid UTF-8, skipping JSONL cache regeneration",
            output_path
        );
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_float_opt_filters_nan_and_infinity() {
        assert_eq!(parse_float_opt(""), None);
        assert_eq!(parse_float_opt("1.5"), Some(1.5));
        assert_eq!(parse_float_opt("NaN"), None);
        assert_eq!(parse_float_opt("nan"), None);
        assert_eq!(parse_float_opt("Infinity"), None);
        assert_eq!(parse_float_opt("-Infinity"), None);
        assert_eq!(parse_float_opt("inf"), None);
        assert_eq!(parse_float_opt("not a number"), None);
    }

    #[test]
    fn parse_float_opt_from_bytes_filters_nan_and_infinity() {
        assert_eq!(parse_float_opt_from_bytes(b""), None);
        assert_eq!(parse_float_opt_from_bytes(b"42"), Some(42.0));
        assert_eq!(parse_float_opt_from_bytes(b"NaN"), None);
        assert_eq!(parse_float_opt_from_bytes(b"inf"), None);
    }

    #[test]
    fn compute_pearson_skewness_handles_zero_stddev() {
        // Zero stddev -> None (avoid divide-by-zero)
        assert_eq!(
            compute_pearson_skewness(Some(5.0), Some(5.0), Some(0.0)),
            None,
        );
        // Any None input -> None
        assert_eq!(compute_pearson_skewness(None, Some(5.0), Some(1.0)), None);
        // Normal case: 3 * (mean - median) / stddev
        assert_eq!(
            compute_pearson_skewness(Some(10.0), Some(8.0), Some(2.0)),
            Some(3.0),
        );
    }

    #[test]
    fn compute_kendall_tau_no_nan_on_all_ties() {
        // All identical values: every pair is a tie in both x and y.
        // Pre-fix this would produce sqrt(NaN) from rounding making
        // (n0 - n1) or (n0 - n2) slightly negative.
        let x = vec![1.0; 10];
        let y = vec![2.0; 10];
        let tau = compute_kendall_tau(&x, &y);
        // Denominator is zero -> None, never NaN.
        assert!(tau.is_none());
    }

    #[test]
    fn compute_kendall_tau_perfect_concordance() {
        let x: Vec<f64> = (1..=5).map(f64::from).collect();
        let y: Vec<f64> = (1..=5).map(f64::from).collect();
        assert_eq!(compute_kendall_tau(&x, &y), Some(1.0));
    }

    #[test]
    fn compute_normalized_mutual_information_epsilon_guard() {
        // h_x = 0 -> None (early guard)
        assert_eq!(
            compute_normalized_mutual_information(Some(0.5), Some(0.0), Some(1.0)),
            None,
        );
        // Very small positive entropies that would produce a subnormal
        // denominator -> None (epsilon guard, not float-equality).
        let tiny = 1e-200;
        assert_eq!(
            compute_normalized_mutual_information(Some(tiny), Some(tiny), Some(tiny)),
            None,
        );
    }

    #[test]
    fn outlier_count_indices_are_in_range() {
        // Guard against a refactor changing these without updating COUNTS_LEN.
        const {
            assert!(OUTLIER_EXTREME_LOWER < OUTLIER_COUNTS_LEN);
            assert!(OUTLIER_MILD_LOWER < OUTLIER_COUNTS_LEN);
            assert!(OUTLIER_NORMAL < OUTLIER_COUNTS_LEN);
            assert!(OUTLIER_MILD_UPPER < OUTLIER_COUNTS_LEN);
            assert!(OUTLIER_EXTREME_UPPER < OUTLIER_COUNTS_LEN);
            assert!(OUTLIER_TOTAL < OUTLIER_COUNTS_LEN);
        }
    }

    // ---------------------------------------------------------------------
    // Pure helper-function tests. These guard the math kernels exercised
    // indirectly by the integration suite — a regression in one of these
    // would otherwise surface only as a numeric drift in a large CSV diff.
    // ---------------------------------------------------------------------

    #[test]
    fn fmt_pct_drops_fraction_for_integral_values() {
        assert_eq!(fmt_pct(5.0), "5");
        assert_eq!(fmt_pct(25.0), "25");
        assert_eq!(fmt_pct(0.0), "0");
        // Non-integral values retain their fractional part via Display.
        assert_eq!(fmt_pct(5.5), "5.5");
        assert_eq!(fmt_pct(2.5), "2.5");
    }

    #[test]
    fn compute_quartile_coefficient_dispersion_basic_and_edges() {
        // Standard case: (3 - 1) / (3 + 1) = 0.5
        assert_eq!(
            compute_quartile_coefficient_dispersion(Some(1.0), Some(3.0)),
            Some(0.5),
        );
        // Q1 == Q3 -> invalid order -> None (would otherwise be 0/(2*Q)).
        assert_eq!(
            compute_quartile_coefficient_dispersion(Some(2.0), Some(2.0)),
            None,
        );
        // Q1 > Q3 -> invalid order -> None.
        assert_eq!(
            compute_quartile_coefficient_dispersion(Some(5.0), Some(3.0)),
            None,
        );
        // Q1 == -Q3 (sum near zero) -> None to avoid divide-by-near-zero.
        assert_eq!(
            compute_quartile_coefficient_dispersion(Some(-2.0), Some(2.0)),
            None,
        );
        // Either operand None -> None.
        assert_eq!(
            compute_quartile_coefficient_dispersion(None, Some(3.0)),
            None,
        );
        assert_eq!(
            compute_quartile_coefficient_dispersion(Some(1.0), None),
            None,
        );
    }

    #[test]
    fn compute_robust_cv_basic_and_zero_median() {
        assert_eq!(compute_robust_cv(Some(2.0), Some(4.0)), Some(0.5));
        // |median| < EPSILON -> None.
        assert_eq!(compute_robust_cv(Some(2.0), Some(0.0)), None);
        // Negative median uses |median|, so still positive.
        assert_eq!(compute_robust_cv(Some(2.0), Some(-4.0)), Some(0.5));
        assert_eq!(compute_robust_cv(None, Some(4.0)), None);
        assert_eq!(compute_robust_cv(Some(2.0), None), None);
    }

    #[test]
    fn compute_normalized_entropy_handles_low_cardinality() {
        // cardinality 0 or 1 -> always 0.0 (degenerate distribution).
        assert_eq!(compute_normalized_entropy(Some(0.0), Some(0)), Some(0.0));
        assert_eq!(compute_normalized_entropy(Some(0.0), Some(1)), Some(0.0));
        // Uniform distribution over k values: entropy == log2(k), so
        // normalized entropy == 1.
        let k: u64 = 8;
        let entropy = (k as f64).log2();
        let got = compute_normalized_entropy(Some(entropy), Some(k)).unwrap();
        assert!((got - 1.0).abs() < 1e-12, "expected ~1.0, got {got}");
        // Either input None -> None.
        assert_eq!(compute_normalized_entropy(None, Some(8)), None);
        assert_eq!(compute_normalized_entropy(Some(1.0), None), None);
    }

    #[test]
    fn compute_bimodality_coefficient_basic_and_singularity() {
        // (skew^2 + 1) / (kurt + 3) for skew=0, kurt=0 -> 1/3.
        let got = compute_bimodality_coefficient(Some(0.0), Some(0.0)).unwrap();
        assert!((got - 1.0 / 3.0).abs() < 1e-12);
        // kurt == -3 makes the denominator zero -> None.
        assert_eq!(compute_bimodality_coefficient(Some(0.0), Some(-3.0)), None,);
        assert_eq!(compute_bimodality_coefficient(None, Some(0.0)), None);
        assert_eq!(compute_bimodality_coefficient(Some(0.0), None), None);
    }

    #[test]
    fn compute_jarque_bera_small_n_and_known_values() {
        // The `kurtosis` parameter here is *excess* kurtosis (i.e. raw - 3),
        // matching the convention produced by qsv's stats command. The hand-
        // computed expectations below silently encode that — if the function
        // ever switches to raw kurtosis, the second case will break.
        // n < 3 -> None.
        assert_eq!(compute_jarque_bera(Some(0.0), Some(0.0), 0), None);
        assert_eq!(compute_jarque_bera(Some(0.0), Some(0.0), 2), None);
        // Skew = 0, excess kurt = 0 -> JB = 0, p = exp(0) = 1 (normal-looking moments).
        let (jb, p) = compute_jarque_bera(Some(0.0), Some(0.0), 100).unwrap();
        assert!(jb.abs() < 1e-12);
        assert!((p - 1.0).abs() < 1e-12);
        // Hand-computed (excess kurtosis convention):
        //   n = 60, skew = 1, excess kurt = 2 -> JB = (60/6) * (1 + 4/4) = 20, p = exp(-10).
        let (jb, p) = compute_jarque_bera(Some(1.0), Some(2.0), 60).unwrap();
        assert!((jb - 20.0).abs() < 1e-9);
        assert!((p - (-10.0_f64).exp()).abs() < 1e-12);
        // Either moment None -> None.
        assert_eq!(compute_jarque_bera(None, Some(0.0), 100), None);
        assert_eq!(compute_jarque_bera(Some(0.0), None, 100), None);
    }

    #[test]
    fn merge_correlation_states_zero_count_passthrough() {
        let empty = CorrelationState::default();
        let mut populated = CorrelationState::default();
        update_correlation_state(&mut populated, 1.0, 2.0);
        update_correlation_state(&mut populated, 3.0, 4.0);

        // Merging with an empty state must return the populated state untouched.
        let merged_a = merge_correlation_states(&empty, &populated);
        assert_eq!(merged_a.count, populated.count);
        assert!((merged_a.mean_x - populated.mean_x).abs() < 1e-12);
        assert!((merged_a.cxy - populated.cxy).abs() < 1e-12);

        let merged_b = merge_correlation_states(&populated, &empty);
        assert_eq!(merged_b.count, populated.count);
        assert!((merged_b.mean_x - populated.mean_x).abs() < 1e-12);
        assert!((merged_b.cxy - populated.cxy).abs() < 1e-12);
    }

    #[test]
    fn merge_correlation_states_matches_single_pass() {
        // Build the "ground truth" state by feeding all pairs sequentially.
        let xs = [1.0_f64, 2.0, 4.0, 7.0, 11.0, 16.0, 22.0, 29.0];
        let ys = [3.0_f64, 1.0, 4.0, 1.0, 5.0, 9.0, 2.0, 6.0];

        let mut full = CorrelationState::default();
        for i in 0..xs.len() {
            update_correlation_state(&mut full, xs[i], ys[i]);
        }

        // Build two partial states for a non-trivial split (3 + 5).
        let split = 3;
        let mut part1 = CorrelationState::default();
        for i in 0..split {
            update_correlation_state(&mut part1, xs[i], ys[i]);
        }
        let mut part2 = CorrelationState::default();
        for i in split..xs.len() {
            update_correlation_state(&mut part2, xs[i], ys[i]);
        }

        let merged = merge_correlation_states(&part1, &part2);

        // Counts must match exactly; floats within tight tolerance because
        // the Welford parallel formula is algebraically equivalent but
        // takes a different rounding path than single-pass.
        assert_eq!(merged.count, full.count);
        assert!((merged.mean_x - full.mean_x).abs() < 1e-9);
        assert!((merged.mean_y - full.mean_y).abs() < 1e-9);
        assert!((merged.m2_x - full.m2_x).abs() < 1e-9);
        assert!((merged.m2_y - full.m2_y).abs() < 1e-9);
        assert!((merged.cxy - full.cxy).abs() < 1e-9);
    }

    #[test]
    fn count_inversions_merge_known_cases() {
        // Helper that runs the function on a fresh buffer pair. Empty input
        // returns 0 directly so the helper itself can be exercised on `&[]`
        // without underflow when computing `last = data.len() - 1`.
        fn count(pairs: &[(f64, f64)]) -> i64 {
            if pairs.is_empty() {
                return 0;
            }
            let mut data = pairs.to_vec();
            let mut temp = vec![(0.0, 0.0); data.len()];
            let last = data.len() - 1;
            count_inversions_merge(&mut data, &mut temp, 0, last)
        }

        // Empty slice -> helper short-circuits to 0.
        assert_eq!(count(&[]), 0);
        // Already sorted by y -> 0 inversions.
        assert_eq!(count(&[(1.0, 1.0), (2.0, 2.0), (3.0, 3.0)]), 0);
        // Reverse-sorted by y -> n*(n-1)/2 inversions for n=3 -> 3.
        assert_eq!(count(&[(1.0, 3.0), (2.0, 2.0), (3.0, 1.0)]), 3);
        // Ties don't count as inversions (uses Ordering::Greater).
        assert_eq!(count(&[(1.0, 5.0), (2.0, 5.0), (3.0, 5.0)]), 0);
        // Mixed: only (3 > 1) and (3 > 2) are inversions for y = [3, 1, 2] -> 2.
        assert_eq!(count(&[(1.0, 3.0), (2.0, 1.0), (3.0, 2.0)]), 2);
    }

    #[test]
    fn parse_date_and_days_to_rfc3339_roundtrip() {
        // Date round-trip: parse "2022-01-15", format with TDate -> "2022-01-15".
        let days = parse_date_to_days("2022-01-15", false).unwrap();
        assert_eq!(days_to_rfc3339(days, FieldType::TDate), "2022-01-15");

        // DateTime round-trip with explicit UTC: converting through `f64` days can
        // introduce tiny rounding differences, so assert that the reconstructed
        // timestamp is within 1 ms rather than requiring exact RFC3339 string
        // equality.
        let dt = "2022-01-15T12:30:45+00:00";
        let dt_days = parse_date_to_days(dt, false).unwrap();
        let dt_rfc3339 = days_to_rfc3339(dt_days, FieldType::TDateTime);
        let reparsed_days = parse_date_to_days(&dt_rfc3339, false).unwrap();
        let one_millisecond_in_days = 1.0 / 86_400_000.0;
        assert!(
            (reparsed_days - dt_days).abs() <= one_millisecond_in_days,
            "expected `{dt_rfc3339}` to round-trip within 1 ms of `{dt}`"
        );

        // Empty input -> None.
        assert_eq!(parse_date_to_days("", false), None);
        // Garbage input -> None (no panic).
        assert_eq!(parse_date_to_days("not-a-date", false), None);
    }

    #[test]
    fn finalize_covariance_sample_and_population() {
        // Build a state for x = y = [1, 2, 3, 4, 5].
        // Centered: each (xi - mean) * (yi - mean) summed = m2 = 10.0
        let mut state = CorrelationState::default();
        for v in [1.0, 2.0, 3.0, 4.0, 5.0] {
            update_correlation_state(&mut state, v, v);
        }
        // Sample covariance: cxy / (n - 1) = 10 / 4 = 2.5.
        let sample = finalize_covariance(&state, true).unwrap();
        assert!((sample - 2.5).abs() < 1e-12, "sample covariance: {sample}");
        // Population covariance: cxy / n = 10 / 5 = 2.0.
        let pop = finalize_covariance(&state, false).unwrap();
        assert!((pop - 2.0).abs() < 1e-12, "population covariance: {pop}");

        // count < 2 -> None.
        let mut tiny = CorrelationState::default();
        update_correlation_state(&mut tiny, 1.0, 2.0);
        assert_eq!(finalize_covariance(&tiny, true), None);
        assert_eq!(finalize_covariance(&tiny, false), None);
    }

    #[test]
    fn finalize_pearson_correlation_guards() {
        // count < 2 -> None.
        let empty = CorrelationState::default();
        assert_eq!(finalize_pearson_correlation(&empty), None);

        // Constant y (variance_y == 0) -> None.
        let mut const_y = CorrelationState::default();
        for x in 1..=5 {
            update_correlation_state(&mut const_y, f64::from(x), 7.0);
        }
        assert_eq!(finalize_pearson_correlation(&const_y), None);
    }

    #[test]
    fn compute_pearson_correlation_known_values() {
        // Perfect positive linear -> +1.
        let r =
            compute_pearson_correlation(&[1.0, 2.0, 3.0, 4.0, 5.0], &[2.0, 4.0, 6.0, 8.0, 10.0])
                .unwrap();
        assert!((r - 1.0).abs() < 1e-12, "perfect positive: {r}");

        // Perfect negative linear -> -1.
        let r =
            compute_pearson_correlation(&[1.0, 2.0, 3.0, 4.0, 5.0], &[10.0, 8.0, 6.0, 4.0, 2.0])
                .unwrap();
        assert!((r + 1.0).abs() < 1e-12, "perfect negative: {r}");

        // Hand-computed: x=[1,2,3,4,5], y=[2,4,5,4,5]
        //   dx = [-2,-1,0,1,2], dy = [-2,0,1,0,1]
        //   Sxy = 6, Sxx = 10, Syy = 6 -> r = 6 / sqrt(60) = sqrt(0.6).
        let r = compute_pearson_correlation(&[1.0, 2.0, 3.0, 4.0, 5.0], &[2.0, 4.0, 5.0, 4.0, 5.0])
            .unwrap();
        assert!(
            (r - 0.6_f64.sqrt()).abs() < 1e-12,
            "fractional case: got {r}, expected {}",
            0.6_f64.sqrt()
        );

        // Mismatched lengths -> None.
        assert_eq!(
            compute_pearson_correlation(&[1.0, 2.0], &[1.0, 2.0, 3.0]),
            None,
        );
        // Length < 2 -> None.
        assert_eq!(compute_pearson_correlation(&[1.0], &[2.0]), None);
        // Constant input (zero variance) -> None, not NaN.
        assert_eq!(
            compute_pearson_correlation(&[5.0, 5.0, 5.0], &[1.0, 2.0, 3.0]),
            None,
        );
    }

    #[test]
    fn compute_spearman_correlation_handles_monotonic_and_ties() {
        // Strictly increasing non-linear (cubic) -> Spearman = +1
        // even though Pearson would be < 1. This is the whole point of
        // rank correlation.
        let r = compute_spearman_correlation(
            &[1.0, 2.0, 3.0, 4.0, 5.0],
            &[1.0, 8.0, 27.0, 64.0, 125.0],
        )
        .unwrap();
        assert!((r - 1.0).abs() < 1e-12, "cubic monotonic: {r}");

        // Strictly decreasing -> -1.
        let r =
            compute_spearman_correlation(&[1.0, 2.0, 3.0, 4.0, 5.0], &[5.0, 4.0, 3.0, 2.0, 1.0])
                .unwrap();
        assert!((r + 1.0).abs() < 1e-12, "decreasing: {r}");

        // Tie handling. y has two pairs of ties at 4 and 5:
        //   x=[1,2,3,4,5] -> ranks [1,2,3,4,5]
        //   y=[2,4,5,4,5] -> 2->rank 1, 4 (×2)->avg rank 2.5, 5 (×2)->avg rank 4.5
        //   y_ranks = [1, 2.5, 4.5, 2.5, 4.5]
        // Pearson on the ranks: dx = [-2,-1,0,1,2], dy = [-2,-0.5,1.5,-0.5,1.5]
        //   Sxy = 7, Sxx = 10, Syy = 9 -> r = 7 / sqrt(90).
        let r =
            compute_spearman_correlation(&[1.0, 2.0, 3.0, 4.0, 5.0], &[2.0, 4.0, 5.0, 4.0, 5.0])
                .unwrap();
        let expected = 7.0_f64 / 90.0_f64.sqrt();
        assert!(
            (r - expected).abs() < 1e-9,
            "tied Spearman: got {r}, expected {expected}",
        );

        // Length guards mirror Pearson.
        assert_eq!(
            compute_spearman_correlation(&[1.0, 2.0], &[1.0, 2.0, 3.0]),
            None,
        );
        assert_eq!(compute_spearman_correlation(&[1.0], &[2.0]), None);
    }

    #[test]
    fn stats_options_redirect_output_detects_output_flags() {
        // Plain long/short forms.
        assert!(stats_options_redirect_output("-o joined.csv"));
        assert!(stats_options_redirect_output("-ojoined.csv"));
        assert!(stats_options_redirect_output("--output joined.csv"));
        assert!(stats_options_redirect_output("--output=joined.csv"));
        assert!(stats_options_redirect_output("-E --output joined.csv"));

        // Clustered short options — the regression the original
        // `starts_with("-o")` guard missed.
        assert!(stats_options_redirect_output("-Eo joined.csv"));
        assert!(stats_options_redirect_output("-Eojoined.csv"));
        assert!(stats_options_redirect_output("-no joined.csv"));
        assert!(stats_options_redirect_output("-Eno joined.csv"));

        // No redirection.
        assert!(!stats_options_redirect_output(""));
        assert!(!stats_options_redirect_output("-E"));
        assert!(!stats_options_redirect_output("--everything --infer-dates"));
        assert!(!stats_options_redirect_output("-En"));

        // `-s` takes an argument, so `-so` selects a column named `o` —
        // it must NOT be mistaken for `-s -o`.
        assert!(!stats_options_redirect_output("-so"));
        assert!(!stats_options_redirect_output("-Eso"));
        assert!(!stats_options_redirect_output("--select output"));
    }

    #[test]
    fn merge_percentile_list_covers_and_extends() {
        // Already covered — untouched (returns None).
        assert_eq!(merge_percentile_list("5,10,40,60,90,95", 5, 95), None);
        assert_eq!(merge_percentile_list("5,10,40,60,90,95", 10, 90), None);
        // Fractional entries cover their truncated integer percentile.
        assert_eq!(merge_percentile_list("7.5,93.9", 7, 93), None);

        // Extensions come back sorted numerically.
        assert_eq!(
            merge_percentile_list("5,10,40,60,90,95", 7, 93).as_deref(),
            Some("5,7,10,40,60,90,93,95")
        );
        // One bound present, the other appended.
        assert_eq!(
            merge_percentile_list("5,10,40,60,90,95", 5, 93).as_deref(),
            Some("5,10,40,60,90,93,95")
        );

        // deciles/quintiles expand as `stats` expands them.
        assert_eq!(merge_percentile_list("deciles", 10, 90), None);
        assert_eq!(
            merge_percentile_list("quintiles", 7, 93).as_deref(),
            Some("7,20,40,60,80,93")
        );
    }

    #[test]
    fn build_stats_args_forwards_pct_thresholds() {
        // No thresholds: tokens pass through untouched.
        assert_eq!(
            build_stats_args("-E --infer-dates", None),
            vec!["-E", "--infer-dates"]
        );

        // Thresholds outside the stats default list get a merged
        // --percentile-list appended.
        assert_eq!(
            build_stats_args("--percentiles --force", Some((7.0, 93.0))),
            vec![
                "--percentiles",
                "--force",
                "--percentile-list",
                "5,7,10,40,60,90,93,95"
            ]
        );
        // Thresholds inside the default list: nothing to add.
        assert_eq!(
            build_stats_args("--percentiles", Some((5.0, 95.0))),
            vec!["--percentiles"]
        );

        // A caller-supplied --percentile-list is merged into, not replaced.
        assert_eq!(
            build_stats_args("--percentile-list 10,90 --force", Some((7.0, 93.0))),
            vec!["--percentile-list", "7,10,90,93", "--force"]
        );
        assert_eq!(
            build_stats_args("--percentile-list=10,90", Some((7.0, 93.0))),
            vec!["--percentile-list=7,10,90,93"]
        );
        // ...and left byte-for-byte alone when it already covers both bounds.
        assert_eq!(
            build_stats_args("--percentile-list 93,7", Some((7.0, 93.0))),
            vec!["--percentile-list", "93,7"]
        );
    }

    #[test]
    fn percentile_entry_present_matches_labels_only() {
        let cell = "5: 5|10: 10|93: 2026-08-21T00:00:00+00:00";
        assert!(percentile_entry_present(cell, "5", "|"));
        assert!(percentile_entry_present(cell, "10", "|"));
        // Date values contain colons; only the label before the FIRST colon
        // of an entry counts.
        assert!(percentile_entry_present(cell, "93", "|"));
        assert!(!percentile_entry_present(cell, "7", "|"));
        assert!(!percentile_entry_present("", "5", "|"));
    }

    /// Build a `field_pairs` map over four columns at `col_idx` 10/20/30/40, with every
    /// upper-triangle pair present (6 pairs).
    fn sample_field_pairs() -> HashMap<(u16, u16), (BivariateFieldInfo, BivariateFieldInfo)> {
        let info = |col_idx: usize| BivariateFieldInfo {
            col_idx,
            field_type: FieldType::TString,
            stddev: None,
            variance: None,
            cardinality: Some(5),
        };
        let cols = [10_usize, 20, 30, 40];
        let mut field_pairs = HashMap::new();
        for (i, &a) in cols.iter().enumerate() {
            for &b in &cols[i + 1..] {
                let (ka, kb) = (
                    u16::try_from(a).unwrap_or_default(),
                    u16::try_from(b).unwrap_or_default(),
                );
                field_pairs.insert((ka, kb), (info(a), info(b)));
            }
        }
        field_pairs
    }

    /// A sub-plan must enumerate ONLY the columns its own pairs touch, renumbering their
    /// slots densely from 0. This is the invariant `--bivariate-batch` rests on: slots
    /// index into `plan.cols`, and the merge sizes `global_dicts`/`remap` from
    /// `plan.cols.len()`, so a sub-plan that kept the full-plan numbering would index
    /// past the end -- or, worse, silently attribute one column's dictionary to another
    /// and corrupt every joint key.
    #[test]
    fn build_bivariate_plan_subset_renumbers_slots() {
        let field_pairs = sample_field_pairs();
        let keys = sorted_pair_keys(&field_pairs);
        let col_types = canonical_field_types(&field_pairs, &keys);
        assert_eq!(keys.len(), 6);

        let full = build_bivariate_plan(&field_pairs, &keys, &col_types, None, false);
        assert_eq!(full.pairs.len(), 6);
        assert_eq!(full.cols.len(), 4);

        // Take a 2-key slice and check the plan narrows to just those pairs' columns.
        // (10,20),(10,30) touches 3 of the 4 columns -- deliberately NOT a slice like
        // (10,40),(20,30), which touches all four and so could not detect narrowing.
        let batch = &keys[0..2];
        let sub = build_bivariate_plan(&field_pairs, batch, &col_types, None, false);
        assert_eq!(sub.pairs.len(), 2);

        let mut touched: Vec<usize> = batch
            .iter()
            .flat_map(|&(a, b)| [a as usize, b as usize])
            .collect();
        touched.sort_unstable();
        touched.dedup();
        assert_eq!(sub.cols.len(), touched.len());
        assert!(sub.cols.len() < full.cols.len());

        let mut seen: Vec<usize> = sub.cols.iter().map(|(c, _)| *c).collect();
        seen.sort_unstable();
        assert_eq!(seen, touched);

        // Every slot must resolve, within THIS plan, back to the pair's own columns.
        for (pair, &(ka, kb)) in sub.pairs.iter().zip(batch) {
            assert_eq!(pair.key, (ka, kb));
            assert_eq!(sub.cols[pair.x_slot as usize].0, ka as usize);
            assert_eq!(sub.cols[pair.y_slot as usize].0, kb as usize);
        }
    }

    /// Concatenating every batch of a `chunks(k)` partition must reproduce the full
    /// plan's pair order exactly -- that ordering is what makes a batched run reproduce
    /// an unbatched one.
    #[test]
    fn build_bivariate_plan_batches_cover_every_pair_in_order() {
        let field_pairs = sample_field_pairs();
        let keys = sorted_pair_keys(&field_pairs);
        let col_types = canonical_field_types(&field_pairs, &keys);
        let full = build_bivariate_plan(&field_pairs, &keys, &col_types, None, false);

        for k in 1..=keys.len() + 2 {
            let batched: Vec<(u16, u16)> = keys
                .chunks(k)
                .flat_map(|batch| {
                    build_bivariate_plan(&field_pairs, batch, &col_types, None, false)
                        .pairs
                        .into_iter()
                        .map(|p| p.key)
                })
                .collect();
            let expected: Vec<(u16, u16)> = full.pairs.iter().map(|p| p.key).collect();
            assert_eq!(batched, expected, "batch width {k} changed the pair order");
        }
    }

    /// A column's decode type must not depend on which batch reaches it first.
    ///
    /// Duplicate header names make `field_pairs` resolve two differently-typed stats
    /// rows onto one `col_idx` (header lookup is first-match-wins), so the two pairs
    /// disagree about that column's type. Before `canonical_field_types`, each plan
    /// resolved that independently: the full plan took the first type in the full key
    /// order, while a batch took the first type in ITS slice. On a real `a,b,a` file
    /// that flipped column 0 from Integer to Date under `--bivariate-batch 1` and
    /// changed the covariance from 78.0268 to 0.0009 (roborev job 4110).
    #[test]
    fn build_bivariate_plan_column_type_is_batch_independent() {
        let info = |col_idx: usize, field_type: FieldType| BivariateFieldInfo {
            col_idx,
            field_type,
            stddev: None,
            variance: None,
            cardinality: Some(5),
        };
        // Mirrors headers `a,b,a` where stats infers Integer for the first `a` and Date
        // for the second, and both resolve to col_idx 0. Keys are the (col_idx, col_idx)
        // pairs that enumeration produces: (0,0), (0,1) and (1,0).
        let mut field_pairs = HashMap::new();
        field_pairs.insert(
            (0_u16, 0_u16),
            (info(0, FieldType::TInteger), info(0, FieldType::TDate)),
        );
        field_pairs.insert(
            (0_u16, 1_u16),
            (info(0, FieldType::TInteger), info(1, FieldType::TInteger)),
        );
        // The pair that used to poison a batch: col 0 appears here as the DATE row.
        field_pairs.insert(
            (1_u16, 0_u16),
            (info(1, FieldType::TInteger), info(0, FieldType::TDate)),
        );

        let keys = sorted_pair_keys(&field_pairs);
        let col_types = canonical_field_types(&field_pairs, &keys);
        assert_eq!(col_types.get(&0), Some(&FieldType::TInteger));

        let type_of = |plan: &BivariatePlan, col_idx: usize| {
            plan.cols
                .iter()
                .find(|(c, _)| *c == col_idx)
                .map(|(_, t)| *t)
        };

        let full = build_bivariate_plan(&field_pairs, &keys, &col_types, None, false);
        assert_eq!(type_of(&full, 0), Some(FieldType::TInteger));

        // Every batch width must agree with the full plan about column 0 -- including
        // width 1, which isolates key (1,0), the slice that used to yield TDate.
        for k in 1..=keys.len() {
            for batch in keys.chunks(k) {
                let sub = build_bivariate_plan(&field_pairs, batch, &col_types, None, false);
                if let Some(t) = type_of(&sub, 0) {
                    assert_eq!(
                        t,
                        FieldType::TInteger,
                        "batch width {k}, batch {batch:?} decoded column 0 as {t:?}"
                    );
                }
            }
        }
    }

    /// `--bivariate-batch` must default to 0, which means "every pair in one pass".
    /// A non-zero default would silently impose extra passes over the input on every
    /// existing user.
    #[test]
    fn bivariate_batch_defaults_to_zero() {
        let args: Args = util::get_args(USAGE, &["qsv", "moarstats", "in.csv"]).unwrap();
        assert_eq!(args.flag_bivariate_batch, 0);
    }
}
