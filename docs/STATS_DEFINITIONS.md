# QSV Stats Definitions

This document enumerates all the statistics produced by the `qsv stats` command, sourced from `src/cmd/stats.rs`.

Each statistic is categorized by its relevant section, with its identifier (column name), summary, computation method, and level (File or Variable).

> **Note**: "Streaming" statistics are computed in constant memory. "Non-Streaming" statistics require loading the column data into memory (or multiple passes) and may use approximation or exact calculation depending on configuration.

**Important:** Unlike the `sniff` command, `stats` data type inferences are **GUARANTEED**, as the entire file is scanned, not just sampled. This makes `stats` a central command in qsv that underpins other "smart" commands (`frequency`, `pivotp`, `sample`, `schema`, `validate`, `tojsonl`) which use cached statistical information to work smarter & faster.

The command supports various caching options to improve performance on subsequent runs. See `--stats-jsonl` and `--cache-threshold` options for details.

## Streaming vs Non-Streaming Statistics

**Streaming Statistics** (computed in constant memory, always included by default):
- Metadata: `field`, `type`, `is_ascii`
- Descriptive: `sum`, `min`, `max`, `range`, `sort_order`, `sortiness`
- String length: `min_length`, `max_length`, `sum_length`, `avg_length`, `stddev_length`, `variance_length`, `cv_length`
- Central tendency: `mean`, `sem`, `geometric_mean`, `harmonic_mean`, `stddev`, `variance`, `cv`
- Quality: `nullcount`, `n_negative`, `n_zero`, `n_positive`, `max_precision`, `sparsity`

**Non-Streaming Statistics** (require loading data into memory, must be enabled explicitly):
- `cardinality`, `uniqueness_ratio`
- `mode`, `mode_count`, `mode_occurrences`, `antimode`, `antimode_count`, `antimode_occurrences`
- `median`, `mad`
- `q1`, `q2_median`, `q3`, `iqr`, `lower_outer_fence`, `lower_inner_fence`, `upper_inner_fence`, `upper_outer_fence`, `skewness`
- `percentiles`

Non-streaming statistics use memory-aware chunking for large files, dynamically calculating chunk size based on available memory and record sampling.

## Weighted Statistics

When the `--weight <column>` option is specified, all statistics are computed using weighted algorithms. The weight column must be numeric and is automatically excluded from statistics computation. Missing or non-numeric weights default to 1.0. Zero and negative weights are ignored and do not contribute to the statistics.

Weighted statistics use weighted versions of the standard algorithms:
- **Weighted mean/variance/stddev**: Weighted Welford's algorithm (West, 1979)
- **Weighted geometric mean**: `exp(Σ(w_i * ln(x_i)) / Σ(w_i))` for positive values
- **Weighted harmonic mean**: `Σ(w_i) / Σ(w_i / x_i)` for non-zero values
- **Weighted median/quartiles/percentiles**: Weighted nearest-rank method
- **Weighted MAD**: Weighted median of absolute deviations
- **Weighted modes/antimodes**: Based on weight values rather than frequency counts

The output filename will be `<FILESTEM>.stats.weighted.csv` to distinguish from unweighted statistics.

## Date/DateTime Statistics

Date and DateTime statistics are only computed when `--infer-dates` is enabled. Date inference is an expensive operation that matches date candidates against 19 possible date formats with multiple variants.

**Formatting:**
- DateTime results are in RFC3339 format (e.g., "2023-01-15T10:30:00Z")
- Date results are in "yyyy-mm-dd" format (UTC timezone)
- If timezone is not specified in the data, it is set to UTC

**Units:**
- Date range, stddev, variance, MAD, and IQR are returned in **days** (not milliseconds)
- These values are rounded to at least 5 decimal places to provide millisecond precision
- Mean, geometric mean, and harmonic mean for dates/datetimes are returned in RFC3339 format

**Date Column Selection:**
- By default, only columns with names containing patterns from `--dates-whitelist` are checked (default: `date,time,due,open,close,created`)
- **Examples of column names that trigger date inference:** "start_date", "Observation Time", "timestamp", "Date Closed"
- **Examples that do NOT trigger:** "start_dt", "create_dt", "tmstmp", "close_dt" (unless added to whitelist)
- Use `--dates-whitelist all` to inspect all fields (may cause false positives with numeric data like Unix epoch timestamps)
- Use `--prefer-dmy` to parse dates in day/month/year format instead of month/day/year

## Metadata & Type Inference

| Identifier | Level | Summary | Computation |
|:---|:---:|:---|:---|
| `field` | Variable | The name of the column/header (or its index if `--no-headers` is used). | Extracted from the CSV header row. |
| `type` | Variable | Inferred data type of the column. | Inferred by checking values against: NULL, Integer, Float, Date, DateTime, Boolean (optional), and fallback to String. Data type inferences are **GUARANTEED** as `stats` scans the entire file. |
| `subtype_xsd` | Variable | Inferred XSD data subtype (if enabled). | Refined inference for Integers (Requesting `byte`, `short`, `int`, `long`) and Floats (`decimal`). Note: This may not be actively used in the current implementation. |
| `is_ascii` | Variable | Indicates if all characters in the string column are ASCII. | Checked during UTF-8 validation; true if bytes are valid ASCII. |

**Date and DateTime Type Inference:**
- Date and DateTime types are only inferred when `--infer-dates` is enabled
- Date inference matches candidates against **19 possible date formats** with multiple variants
- As date parsing is relatively expensive, it only attempts date inferencing for columns in the `--dates-whitelist` (default: `date,time,due,open,close,created`)
- **Examples of column names that trigger date inference:** "start_date", "Observation Time", "timestamp", "Date Closed"
- **Examples that do NOT trigger:** "start_dt", "create_dt", "tmstmp", "close_dt" (unless added to whitelist)
- Use `--dates-whitelist all` to inspect all fields (may cause false positives with numeric data like Unix epoch timestamps)
- The date formats recognized and their sub-variants can be found at: https://github.com/dathere/qsv-dateparser?tab=readme-ov-file#accepted-date-formats

**Boolean Type Inference:**
- Boolean type is inferred when `--infer-boolean` is enabled
- A column is inferred as Boolean when its cardinality is 2 and the two values match the boolean patterns specified by `--boolean-patterns` (default: `1:0,t*:f*,y*:n*`)
- Boolean inference automatically enables `--cardinality` computation
- Patterns are case-insensitive and support prefix matching with `*` wildcards
- **Example:** `"t*:f*,y*:n*"` will match "true", "truthy", "Truth" as boolean true values so long as the corresponding false pattern (e.g., False, f, etc.) is also matched and cardinality is 2

## Descriptive Statistics (Numerical & General)

| Identifier | Level | Summary | Computation |
|:---|:---:|:---|:---|
| `sum` | Variable | Sum of all values in the column. | Rolling sum. Integers sum to Integer until a Float is encountered, then switches to Float. Integer sums that overflow/underflow show `*OVERFLOW*` or `*UNDERFLOW*`. For Floats, returns NaN as the string "NaN", positive infinity as "inf", and negative infinity as "-inf". |
| `min` | Variable | Minimum value found. | Tracks minimum value during the scan. |
| `max` | Variable | Maximum value found. | Tracks maximum value during the scan. |
| `range` | Variable | Difference between Max and Min. | `max - min`. |
| `sort_order` | Variable | Sorting status of the column. | Checked during scan. Returns "ASCENDING", "DESCENDING", or "UNSORTED". |
| `sortiness` | Variable | Measure of how sorted the column is. | Returns a score between -1.0 and 1.0: 1.0 indicates perfectly ascending order, -1.0 indicates perfectly descending order, values in between indicate the general tendency towards ascending or descending order, and 0.0 indicates either no clear ordering or empty/single-element collections. |

## Central Tendency & Dispersion (Streaming)

Computed using Welford's online algorithm for single-pass accuracy. When `--weight <column>` is specified, weighted versions are computed using weighted Welford's algorithm (West, 1979).

| Identifier | Level | Summary | Computation |
|:---|:---:|:---|:---|
| `mean` | Variable | Arithmetic mean (average). | Welford's algorithm mean. Weighted: `Σ(w_i * x_i) / Σ(w_i)`. |
| `sem` | Variable | Standard Error of the Mean. | `stddev / sqrt(count)`. |
| `geometric_mean` | Variable | Geometric mean. | Online calculation using logarithms. Weighted: `exp(Σ(w_i * ln(x_i)) / Σ(w_i))` for positive values. |
| `harmonic_mean` | Variable | Harmonic mean. | Online calculation using reciprocals. Weighted: `Σ(w_i) / Σ(w_i / x_i)` for non-zero values. |
| `stddev` | Variable | Standard deviation (sample). | Welford's algorithm standard deviation. Weighted: uses frequency weight definition. |
| `variance` | Variable | Variance (sample). | Square of standard deviation. Weighted: `S_n / (W_n - 1)` where S_n is sum of squared differences. |
| `cv` | Variable | Coefficient of Variation. | `(stddev / mean) * 100`. Returns NaN when mean is 0. |

## String Statistics

> **NOTE:** Length statistics are **only computed for columns with a String data type**. Lengths are **byte lengths, not character lengths**, as some UTF-8 characters take more than one byte.

| Identifier | Level | Summary | Computation |
|:---|:---:|:---|:---|
| `min_length` | Variable | Length of the shortest string. | Tracks minimum length in bytes. |
| `max_length` | Variable | Length of the longest string. | Tracks maximum length in bytes. |
| `sum_length` | Variable | Sum of lengths of all strings. | Accumulates length of every value. Shows `*OVERFLOW*` when sum exceeds u64::MAX. |
| `avg_length` | Variable | Average string length. | `sum_length / count`. Shows `*OVERFLOW*` when `sum_length` overflowed. |
| `stddev_length` | Variable | Standard deviation of string lengths. | Welford's algorithm on lengths. Shows `*OVERFLOW*` when `sum_length` overflowed. |
| `variance_length` | Variable | Variance of string lengths. | Square of `stddev_length`. Shows `*OVERFLOW*` when `sum_length` overflowed. |
| `cv_length` | Variable | Coefficient of Variation of lengths. | `(stddev_length / avg_length) * 100`. Shows `*OVERFLOW*` when `sum_length` overflowed. |

## Quality & Distribution

| Identifier | Level | Summary | Computation |
|:---|:---:|:---|:---|
| `nullcount` | Variable | Count of NULL (empty) values. | Incremented when a field is empty (or matches custom NULL). |
| `n_negative` | Variable | Count of negative values. | Computed for Integer and Float types only. |
| `n_zero` | Variable | Count of zero values. | Computed for Integer and Float types only. |
| `n_positive` | Variable | Count of positive values. | Computed for Integer and Float types only. |
| `max_precision` | Variable | Maximum decimal precision found (Floats). | Tracks the maximum number of digits after the decimal point. |
| `sparsity` | Variable | Fraction of missing (NULL) values. | `nullcount / record_count`. |

## Median & Quartiles (Non-Streaming)

Requires loading data into memory and sorting. When `--weight <column>` is specified, weighted versions are computed using weighted nearest-rank method.

**Note on Date/DateTime types:** For Date and DateTime types, range, stddev, variance, MAD, and IQR are returned in days (not milliseconds). These values are rounded to at least 5 decimal places to provide millisecond precision.

**Requirements:**
- `median` requires `--median` or `--everything` (unless `--quartiles` is specified, in which case `median` is not returned separately as it's the same as `q2_median`)
- `mad` requires `--mad` or `--everything`
- Quartile statistics require `--quartiles` or `--everything`

| Identifier | Level | Summary | Computation |
|:---|:---:|:---|:---|
| `median` | Variable | Median value (50th percentile). | Middle value of sorted data (or average of two middle values). Weighted: uses weighted nearest-rank method. When `--quartiles` is specified, this is not returned separately as it's the same as `q2_median`. |
| `mad` | Variable | Median Absolute Deviation, a robust measure of variability. | Median of the absolute deviations from the data's median. Weighted: weighted median of absolute deviations. For dates/datetimes, returned in days. |
| `q1` | Variable | First Quartile (25th percentile). | Value at 25% rank using [Method 3](https://en.wikipedia.org/wiki/Quartile#Method_3). Weighted: value at which cumulative weight first reaches 25% of total weight. |
| `q2_median` | Variable | Second Quartile (Median). | Same as `median` (50th percentile). |
| `q3` | Variable | Third Quartile (75th percentile). | Value at 75% rank using [Method 3](https://en.wikipedia.org/wiki/Quartile#Method_3). Weighted: value at which cumulative weight first reaches 75% of total weight. |
| `iqr` | Variable | Interquartile Range. | `q3 - q1`. For dates/datetimes, returned in days with at least 5 decimal places. |
| `lower_outer_fence` | Variable | Lower bound for extreme outliers. | `q1 - (3.0 * iqr)`, used to identify extreme outliers. For dates/datetimes, returned in RFC3339 format. |
| `lower_inner_fence` | Variable | Lower bound for outliers. | `q1 - (1.5 * iqr)`, used to identify mild outliers. For dates/datetimes, returned in RFC3339 format. |
| `upper_inner_fence` | Variable | Upper bound for outliers. | `q3 + (1.5 * iqr)`, used to identify mild outliers. For dates/datetimes, returned in RFC3339 format. |
| `upper_outer_fence` | Variable | Upper bound for extreme outliers. | `q3 + (3.0 * iqr)`, used to identify extreme outliers. For dates/datetimes, returned in RFC3339 format. |
| `skewness` | Variable | Measure of asymmetry of the probability distribution. | Quantile-based skewness: `((q3 - q2) - (q2 - q1)) / iqr` or `(q3 - (2.0 * q2) + q1) / iqr`. |

## Cardinality & Modes (Non-Streaming)

**Requirements:**
- `cardinality` and `uniqueness_ratio` require `--cardinality` or `--everything`
- `mode`, `mode_count`, `mode_occurrences`, `antimode`, `antimode_count`, `antimode_occurrences` require `--mode` or `--everything`

When `--weight <column>` is specified, weighted versions are computed. For weighted modes, `mode_occurrences` is the maximum weight (rounded). For weighted antimodes, `antimode_occurrences` is the minimum weight (rounded).

Multiple modes/antimodes are separated by the `QSV_STATS_SEPARATOR` environment variable (default: `|`).

| Identifier | Level | Summary | Computation |
|:---|:---:|:---|:---|
| `cardinality` | Variable | Count of unique values. | Count of distinct entries in the column. Weighted: count of unique values (weights are not considered for uniqueness). |
| `uniqueness_ratio` | Variable | Ratio of unique values to total records. | `cardinality / record_count`. **Interpretation:** 1.0 = All unique values (e.g., primary keys). Close to 1.0 = Mostly unique values (e.g., user IDs, timestamps). Close to 0.0 = Many repeated values (e.g., categorical labels like "Male/Female" or "Yes/No"). |
| `mode` | Variable | The most frequent value(s) in the column. | Value(s) with the highest frequency count. Weighted: value(s) with the highest weight. Multimodal-aware. If there are multiple modes, they are separated by `QSV_STATS_SEPARATOR`. |
| `mode_count` | Variable | Number of modes found. | Count of values tied for highest frequency. |
| `mode_occurrences` | Variable | Frequency count of the mode. | Number of times the mode(s) appear. Weighted: maximum weight (rounded). |
| `antimode` | Variable | The least frequent non-zero/non-null value(s) in the column. | Value(s) with the lowest frequency count (non-zero). Returns `*ALL` if all values are unique. Limited to first 10 values, truncating after 100 characters (configurable with `QSV_ANTIMODES_LEN`). If truncated, includes `*PREVIEW:` prefix. Weighted: value(s) with the lowest weight. If there are multiple antimodes, they are separated by `QSV_STATS_SEPARATOR`. |
| `antimode_count` | Variable | Number of antimodes found. | Count of values tied for lowest frequency. |
| `antimode_occurrences` | Variable | Frequency count of the antimode. | Number of times the antimode(s) appear. Weighted: minimum weight (rounded). |

## Percentiles (Non-Streaming)

Requires loading data into memory and sorting. When `--weight <column>` is specified, weighted percentiles are computed using weighted nearest-rank method.

**Requirements:** `--percentiles` or `--everything`

Computed using the [nearest-rank method](https://en.wikipedia.org/wiki/Percentile#The_nearest-rank_method).

| Identifier | Level | Summary | Computation |
|:---|:---:|:---|:---|
| `percentiles` | Variable | Custom percentiles of sorted values. | Nearest rank method for user-defined list. Weighted: weighted nearest-rank method. Multiple percentiles are separated by `QSV_STATS_SEPARATOR` (default: `|`). Special values: "deciles" expands to "10,20,30,40,50,60,70,80,90" and "quintiles" expands to "20,40,60,80". Default: "5,10,40,60,90,95". For dates/datetimes, values are returned in RFC3339 format. |

## Dataset Statistics (File Level)

These statistics appear as additional rows with the prefix `qsv__` when `--dataset-stats` is enabled. The `qsv__value` column is added to hold the values for these dataset-level statistics.

**Note:** The `--everything` option does **NOT** enable `--dataset-stats`; it must be specified separately.

| Identifier | Level | Summary | Computation |
|:---|:---:|:---|:---|
| `qsv__rowcount` | File | Total number of rows (records). | Count of records processed (excluding header). |
| `qsv__columncount` | File | Total number of columns. | Count of fields in the header/first record. |
| `qsv__filesize_bytes` | File | Total file size in bytes. | Filesystem metadata size. |
| `qsv__fingerprint_hash`| File | Cryptographic hash of the dataset's stats. | BLAKE3 hash (formerly SHA-256) of the first 26 columns ("streaming" stats) + dataset stats (rowcount, columncount, filesize_bytes). This allows users to quickly detect duplicate files without having to load the entire file to compute the hash. Especially useful for detecting duplicates of very large files with pre-existing stats cache metadata. |
| `qsv__value` | File | The value column for dataset stats. | Holds the value for the `qsv__` row (e.g., the row count integer, column count, file size, or fingerprint hash). |

## Whitespace Visualization

The `--vis-whitespace` option visualizes whitespace characters in the output to make them visible. Note that spaces will only be visualized (using `《_》`) if the entire value is composed of spaces.

The following whitespace markers are used (as defined in the [Rust reference](https://doc.rust-lang.org/reference/whitespace.html)):

| Character | Visualization | Description |
|:---|:---|:---|
| `\t` | `《→》` | Tab |
| `\n` | `《¶》` | Newline |
| `\r` | `《⏎》` | Carriage return |
| `\u{000B}` | `《⋮》` | Vertical tab |
| `\u{000C}` | `《␌》` | Form feed |
| `\u{0009}` | `《↹》` | Horizontal tab |
| `\u{0085}` | `《␤》` | Next line |
| `\u{200E}` | `《␎》` | Left-to-right mark |
| `\u{200F}` | `《␏》` | Right-to-left mark |
| `\u{2028}` | `《␊》` | Line separator |
| `\u{2029}` | `《␍》` | Paragraph separator |
| `\u{00A0}` | `《⍽》` | Non-breaking space |
| `\u{2003}` | `《emsp》` | Em space |
| `\u{2007}` | `《figsp》` | Figure space |
| `\u{200B}` | `《zwsp》` | Zero width space |

## Performance & Caching

The `stats` command is central to qsv and underpins other "smart" commands (`frequency`, `pivotp`, `sample`, `schema`, `validate`, `tojsonl`) that use cached statistical information to work smarter & faster.

**Caching Behavior:**
- Statistics are cached in `<FILESTEM>.stats.csv` and optionally `<FILESTEM>.stats.csv.data.jsonl` (with `--stats-jsonl`)
- The arguments used to generate cached stats are saved in `<FILESTEM>.stats.csv.json`
- If stats have already been computed with similar arguments and the file hasn't changed, stats are loaded from cache instead of recomputing
- Use `--force` to force recomputing stats even if valid cache exists
- Use `--cache-threshold` to control caching behavior (default: 5000ms)

## Additional Statistics (moarstats Command)

The `moarstats` command extends an existing stats CSV file (created by the `stats` command) by computing additional statistics that can be derived from existing stats columns and/or by scanning the original CSV file.

**How it works:**
- Looks for `<FILESTEM>.stats.csv` for a given CSV input
- If the stats CSV file doesn't exist, it will first run the `stats` command with configurable options (via `--stats-options`) to establish baseline stats
- If the `.stats.csv` file is found, it skips running stats and just appends the additional stats columns
- Statistics are rounded using Bankers Rounding (Midpoint Nearest Even) to the specified number of decimal places (default: 4, configurable with `--round`)
- Uses parallel processing when an index is available for large files

**Requirements:**
- All statistics are computed only for numeric and date/datetime columns (except Shannon Entropy which works for all field types)
- Derived statistics require specific base statistics to be present in the stats CSV
- Advanced statistics require `--advanced` flag and reading the entire CSV file
- Outlier statistics require quartiles (and thus fences) to be computed in the baseline stats
- Winsorized/trimmed means require either Q1/Q3 or percentiles to be available

### Derived Statistics

These statistics are computed directly from existing stats columns without scanning the original CSV file. They require specific base statistics to be present in the stats CSV.

| Identifier | Level | Summary | Computation |
|:---|:---:|:---|:---|
| `pearson_skewness` | Variable | Pearson's Second Skewness Coefficient. Measures asymmetry of the distribution. Positive values indicate right skew, negative values indicate left skew. | `3 * (mean - median) / stddev`. Requires: `mean`, `median` (or `q2_median`), `stddev`. Returns `None` if stddev is zero. See: [Skewness](https://en.wikipedia.org/wiki/Skewness) |
| `range_stddev_ratio` | Variable | Range to Standard Deviation Ratio. Normalizes the spread of data. Higher values indicate more extreme outliers relative to the variability. | `range / stddev`. Requires: `range`, `stddev`. Returns `None` if stddev is zero. |
| `quartile_coefficient_dispersion` | Variable | Quartile Coefficient of Dispersion. Measures relative variability using quartiles. Useful for comparing dispersion across different scales. | `(Q3 - Q1) / (Q3 + Q1)`. Requires: `q1`, `q3`. Returns `None` if Q1 >= Q3 or if denominator is zero. See: [Quartile Coefficient of Dispersion](https://en.wikipedia.org/wiki/Quartile_coefficient_of_dispersion) |
| `mode_zscore` | Variable | Z-Score of Mode. Indicates how typical the mode is relative to the distribution. Values near 0 suggest the mode is near the mean. | `(mode - mean) / stddev`. Requires: `mode`, `mean`, `stddev`. If multiple modes exist, uses the first mode. Returns `None` if stddev is zero. |
| `relative_standard_error` | Variable | Relative Standard Error. Measures precision of the mean estimate relative to its magnitude. Lower values indicate more reliable estimates. | `sem / mean`. Requires: `sem`, `mean`. Returns `None` if mean is zero. |
| `min_zscore` | Variable | Z-Score of Min. Shows how extreme the minimum value is. Large negative values indicate outliers or heavy left tail. | `(min - mean) / stddev`. Requires: `min`, `mean`, `stddev`. Returns `None` if stddev is zero. |
| `max_zscore` | Variable | Z-Score of Max. Shows how extreme the maximum value is. Large positive values indicate outliers or heavy right tail. | `(max - mean) / stddev`. Requires: `max`, `mean`, `stddev`. Returns `None` if stddev is zero. |
| `median_mean_ratio` | Variable | Median-to-Mean Ratio. Indicates skewness direction. Ratio < 1 suggests right skew, > 1 suggests left skew, = 1 suggests symmetry. | `median / mean`. Requires: `median` (or `q2_median`), `mean`. Returns `None` if mean is zero. |
| `iqr_range_ratio` | Variable | IQR-to-Range Ratio. Measures concentration of data. Higher values (closer to 1) indicate more data concentrated in the middle 50%. | `iqr / range`. Requires: `iqr`, `range`. Returns `None` if range is zero. |
| `mad_stddev_ratio` | Variable | MAD-to-StdDev Ratio. Compares robust vs non-robust spread measures. Higher values suggest presence of outliers affecting stddev. | `mad / stddev`. Requires: `mad`, `stddev`. Returns `None` if stddev is zero. |

### Advanced Statistics

These statistics require the `--advanced` flag and reading the entire CSV file to collect all values for computation. They are computationally expensive.

| Identifier | Level | Summary | Computation |
|:---|:---:|:---|:---|
| `kurtosis` | Variable | Excess kurtosis. Measures the "tailedness" of the distribution. Positive values indicate heavy tails, negative values indicate light tails. Values near 0 indicate a normal distribution. | Computed from all values in the column. Uses precalculated mean and variance from baseline stats for efficiency. Requires: `mean`, `variance` (or `stddev`). See: [Kurtosis](https://en.wikipedia.org/wiki/Kurtosis) |
| `gini_coefficient` | Variable | Gini Coefficient. Measures inequality/dispersion in the distribution. Values range from 0 (perfect equality) to 1 (maximum inequality). | Computed from all values in the column. Uses precalculated sum from baseline stats for efficiency. Requires: `sum`. See: [Gini Coefficient](https://en.wikipedia.org/wiki/Gini_coefficient) |
| `shannon_entropy` | Variable | Shannon Entropy. Measures the information content/uncertainty in the distribution. Higher values indicate more diversity, lower values indicate more concentration. Values range from 0 (all values identical) to log2(n) where n is the number of unique values. | Computed using the `frequency` command with `--limit 0` to collect all frequencies, then calculates: `H(X) = -Σ p_i * log2(p_i)` where p_i is the probability of value i. Works for **all field types** (not just numeric). For all-unique fields, returns log2(n). See: [Entropy (Information Theory)](https://en.wikipedia.org/wiki/Entropy_(information_theory)) |

### Robust Statistics (Winsorized & Trimmed Means)

These statistics require scanning the original CSV file. They provide robust alternatives to the standard mean by handling extreme values differently.

**Winsorized Mean:** Replaces values below/above thresholds with threshold values, then computes mean. All values are included in the calculation, but extreme values are capped at thresholds.

**Trimmed Mean:** Excludes values outside thresholds, then computes mean. Only values within thresholds are included in the calculation.

**Threshold Options:**
- **Default:** Uses Q1 and Q3 as thresholds (25% winsorization/trimming)
- **With `--use-percentiles`:** Uses configurable percentiles (default: 5th/95th) as thresholds via `--pct-thresholds`

**Requirements:**
- Default mode: Requires `q1` and `q3` in baseline stats
- Percentile mode: Requires `percentiles` in baseline stats and `--use-percentiles` flag

| Identifier | Level | Summary | Computation |
|:---|:---:|:---|:---|
| `winsorized_mean_25pct` | Variable | Winsorized mean using Q1/Q3 thresholds (25% winsorization). | All values are included, but values below Q1 are set to Q1 and values above Q3 are set to Q3, then mean is computed. For dates/datetimes, returned in RFC3339 format. |
| `winsorized_mean_5pct` | Variable | Winsorized mean using percentile thresholds (5th/95th percentiles). | Only computed when `--use-percentiles` is set. Column name varies based on `--pct-thresholds` (e.g., `winsorized_mean_10pct` for 10th/90th percentiles). |
| `winsorized_stddev` | Variable | Standard deviation of winsorized values. | Sample standard deviation computed from winsorized values. |
| `winsorized_variance` | Variable | Variance of winsorized values. | Sample variance computed from winsorized values. |
| `winsorized_cv` | Variable | Coefficient of variation for winsorized values. | `winsorized_stddev / winsorized_mean`. Returns `None` if mean is zero. |
| `winsorized_range` | Variable | Range of winsorized values. | `max_winsorized - min_winsorized`. |
| `winsorized_stddev_ratio` | Variable | Ratio of winsorized stddev to overall stddev. | `winsorized_stddev / stddev`. Compares robust vs non-robust spread. Returns `None` if overall stddev is zero. |
| `trimmed_mean_25pct` | Variable | Trimmed mean using Q1/Q3 thresholds (25% trimming). | Only values within Q1 and Q3 are included in the mean calculation. For dates/datetimes, returned in RFC3339 format. |
| `trimmed_mean_5pct` | Variable | Trimmed mean using percentile thresholds (5th/95th percentiles). | Only computed when `--use-percentiles` is set. Column name varies based on `--pct-thresholds`. |
| `trimmed_stddev` | Variable | Standard deviation of trimmed values. | Sample standard deviation computed from trimmed values (only values within thresholds). |
| `trimmed_variance` | Variable | Variance of trimmed values. | Sample variance computed from trimmed values. |
| `trimmed_cv` | Variable | Coefficient of variation for trimmed values. | `trimmed_stddev / trimmed_mean`. Returns `None` if mean is zero. |
| `trimmed_range` | Variable | Range of trimmed values. | `max_trimmed - min_trimmed`. |
| `trimmed_stddev_ratio` | Variable | Ratio of trimmed stddev to overall stddev. | `trimmed_stddev / stddev`. Compares robust vs non-robust spread. Returns `None` if overall stddev is zero. |

See: [Winsorized Mean](https://en.wikipedia.org/wiki/Winsorized_mean), [Truncated Mean](https://en.wikipedia.org/wiki/Truncated_mean)

### Outlier Statistics

These statistics require scanning the original CSV file and comparing each value against fence thresholds. Fences are computed using the IQR method: inner fences at Q1/Q3 ± 1.5*IQR, outer fences at Q1/Q3 ± 3.0*IQR.

**Requirements:**
- Requires `--quartiles` or `--everything` in baseline stats (to compute fences)
- Requires: `lower_outer_fence`, `lower_inner_fence`, `upper_inner_fence`, `upper_outer_fence` in baseline stats

**Outlier Classification:**
- **Extreme Lower:** Values below the lower outer fence
- **Mild Lower:** Values between lower outer and inner fences
- **Normal:** Values between inner fences (non-outliers)
- **Mild Upper:** Values between upper inner and outer fences
- **Extreme Upper:** Values above the upper outer fence

See: [Outlier](https://en.wikipedia.org/wiki/Outlier)

#### Outlier Counts

| Identifier | Level | Summary | Computation |
|:---|:---:|:---|:---|
| `outliers_extreme_lower_cnt` | Variable | Count of values below the lower outer fence. | Count of extreme lower outliers. |
| `outliers_mild_lower_cnt` | Variable | Count of values between lower outer and inner fences. | Count of mild lower outliers. |
| `outliers_normal_cnt` | Variable | Count of values between inner fences (non-outliers). | Count of normal (non-outlier) values. |
| `outliers_mild_upper_cnt` | Variable | Count of values between upper inner and outer fences. | Count of mild upper outliers. |
| `outliers_extreme_upper_cnt` | Variable | Count of values above the upper outer fence. | Count of extreme upper outliers. |
| `outliers_total_cnt` | Variable | Total count of all outliers (sum of extreme and mild outliers). | Sum of all outlier counts (extreme + mild, both lower and upper). |
| `outliers_percentage` | Variable | Percentage of values that are outliers. | `(outliers_total_cnt / total_count) * 100`. |

#### Outlier Descriptive Statistics

| Identifier | Level | Summary | Computation |
|:---|:---:|:---|:---|
| `outliers_mean` | Variable | Mean value of outliers. | Mean of all outlier values (extreme and mild, lower and upper). For dates/datetimes, returned in RFC3339 format. |
| `non_outliers_mean` | Variable | Mean value of non-outliers. | Mean of all normal (non-outlier) values. For dates/datetimes, returned in RFC3339 format. |
| `outliers_to_normal_mean_ratio` | Variable | Ratio of outlier mean to non-outlier mean. | `outliers_mean / non_outliers_mean`. Returns `None` if non_outliers_mean is zero. |
| `outliers_min` | Variable | Minimum value among outliers. | Minimum value across all outliers. For dates/datetimes, returned in RFC3339 format. |
| `outliers_max` | Variable | Maximum value among outliers. | Maximum value across all outliers. For dates/datetimes, returned in RFC3339 format. |
| `outliers_range` | Variable | Range of outlier values. | `outliers_max - outliers_min`. |

#### Outlier Variance/Spread Statistics

| Identifier | Level | Summary | Computation |
|:---|:---:|:---|:---|
| `outliers_stddev` | Variable | Standard deviation of outlier values. | Sample standard deviation computed from outlier values. Requires at least 2 outliers. |
| `outliers_variance` | Variable | Variance of outlier values. | Sample variance computed from outlier values. Requires at least 2 outliers. |
| `non_outliers_stddev` | Variable | Standard deviation of non-outlier values. | Sample standard deviation computed from normal (non-outlier) values. Requires at least 2 non-outliers. |
| `non_outliers_variance` | Variable | Variance of non-outlier values. | Sample variance computed from normal (non-outlier) values. Requires at least 2 non-outliers. |
| `outliers_cv` | Variable | Coefficient of variation for outliers. | `outliers_stddev / outliers_mean`. Returns `None` if outliers_mean is zero or if stddev cannot be computed. |
| `non_outliers_cv` | Variable | Coefficient of variation for non-outliers. | `non_outliers_stddev / non_outliers_mean`. Returns `None` if non_outliers_mean is zero or if stddev cannot be computed. |
| `outliers_normal_stddev_ratio` | Variable | Ratio of outlier stddev to non-outlier stddev. | `outliers_stddev / non_outliers_stddev`. Compares spread of outliers vs non-outliers. Returns `None` if non_outliers_stddev is zero or if either stddev cannot be computed. |

#### Outlier Impact Statistics

| Identifier | Level | Summary | Computation |
|:---|:---:|:---|:---|
| `outlier_impact` | Variable | Difference between overall mean and non-outlier mean. | `overall_mean - non_outliers_mean`. Measures how much outliers affect the overall mean. |
| `outlier_impact_ratio` | Variable | Relative impact of outliers. | `outlier_impact / non_outliers_mean`. Normalized measure of outlier impact. Returns `None` if non_outliers_mean is zero. |

#### Outlier Boundary Statistics

| Identifier | Level | Summary | Computation |
|:---|:---:|:---|:---|
| `lower_outer_fence_zscore` | Variable | Z-score of the lower outer fence boundary. | `(lower_outer_fence - mean) / stddev`. Shows how extreme the lower outlier boundary is relative to the distribution. Returns `None` if stddev is zero. |
| `upper_outer_fence_zscore` | Variable | Z-score of the upper outer fence boundary. | `(upper_outer_fence - mean) / stddev`. Shows how extreme the upper outlier boundary is relative to the distribution. Returns `None` if stddev is zero. |
