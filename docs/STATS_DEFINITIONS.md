# qsv Stats Definitions


## Table of Contents

- [stats](#stats)
  - [Streaming vs Non-Streaming Statistics](#streaming-vs-non-streaming-statistics)
  - [Weighted Statistics](#weighted-statistics)
  - [Date/DateTime Statistics](#datedatetime-statistics)
  - [Metadata & Type Inference](#metadata--type-inference)
  - [Descriptive Statistics (Numerical & General)](#descriptive-statistics-numerical--general)
  - [Central Tendency & Dispersion](#central-tendency--dispersion-streaming)
  - [String Statistics](#string-statistics)
  - [Quality & Distribution](#quality--distribution)
  - [Median & Quartiles (Non-Streaming)](#median--quartiles-non-streaming)
  - [Cardinality & Modes (Non-Streaming)](#cardinality--modes-non-streaming)
  - [Percentiles (Non-Streaming)](#percentiles-non-streaming)
  - [File-Level Metadata (JSON Cache)](#file-level-metadata-json-cache)
  - [Whitespace Visualization](#whitespace-visualization)
  - [Performance & Caching](#performance--caching)
- [moarstats](#moarstats)
  - [Derived Statistics](#derived-statistics)
  - [Advanced Statistics](#advanced-statistics)
  - [Bivariate Statistics](#bivariate-statistics)
  - [Robust Statistics (Winsorized & Trimmed Means)](#robust-statistics-winsorized--trimmed-means)
  - [Outlier Statistics](#outlier-statistics)
    - [Outlier Counts](#outlier-counts)
    - [Outlier Descriptive Statistics](#outlier-descriptive-statistics)
    - [Outlier Variance/Spread Statistics](#outlier-variancespread-statistics)
    - [Outlier Impact Statistics](#outlier-impact-statistics)
    - [Outlier Boundary Statistics](#outlier-boundary-statistics)
- [pragmastat](#pragmastat)
  - [One-Sample Mode (Default)](#one-sample-mode-default)
  - [Two-Sample Mode](#two-sample-mode)
  - [Options](#options)
  - [When Values Are Blank](#when-values-are-blank)
- [frequency](#frequency)
  - [Frequency Table Output](#frequency-table-output)
  - [Ranking Strategies](#ranking-strategies)
  - [NULL Handling](#null-handling)
  - [Column Filtering](#column-filtering)
  - [Weighted Frequencies](#weighted-frequencies)
  - [Stats Cache Integration](#stats-cache-integration)
  - [JSON/TOON Output](#jsontoon-output)
  - [Memory-Aware Processing](#memory-aware-processing)

---

## `stats`
Here are all the statistics produced by the `qsv stats` command, sourced from `src/cmd/stats.rs`.

Each statistic is categorized by its relevant section, with its identifier (column name), summary, computation method, and level (File or Variable).

> **Note**: "Streaming" statistics are computed in constant memory. "Non-Streaming" statistics require loading the column data into memory (or multiple passes) and may use approximation or exact calculation depending on configuration.

**Important:** Unlike the `sniff` command, `stats` data type inferences are **GUARANTEED**, as the entire file is scanned, not just sampled. This makes `stats` a central command in qsv that underpins other "smart" commands (`describegpt`, `frequency`, `joinp`, `pivotp`, `schema`, `sqlp`, `tojsonl`) which use cached statistical information to work smarter & faster.

The command supports various caching options to improve performance on subsequent runs. See `--stats-jsonl` and `--cache-threshold` options for details.

### Streaming vs Non-Streaming Statistics

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

### Weighted Statistics

When the `--weight <column>` option is specified, all statistics are computed using weighted algorithms. The weight column must be numeric and is automatically excluded from statistics computation. Missing or non-numeric weights default to 1.0. Zero and negative weights are ignored and do not contribute to the statistics.

Weighted statistics use weighted versions of the standard algorithms:
- **Weighted mean/variance/stddev**: Weighted Welford's algorithm (West, 1979)
- **Weighted geometric mean**: `exp(Σ(w_i * ln(x_i)) / Σ(w_i))` for positive values
- **Weighted harmonic mean**: `Σ(w_i) / Σ(w_i / x_i)` for non-zero values
- **Weighted median/quartiles/percentiles**: Weighted nearest-rank method
- **Weighted MAD**: Weighted median of absolute deviations
- **Weighted modes/antimodes**: Based on weight values rather than frequency counts

The output filename will be `<FILESTEM>.stats.weighted.csv` to distinguish from unweighted statistics.

### Date/DateTime Statistics

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
- By default, `--dates-whitelist` is set to `sniff`, which uses two-stage date inferencing: first runs `qsv sniff` on the input file, then only infers dates for the columns that sniff identifies as date/datetime candidates. This is much faster than `all`, and more convenient than manually specifying patterns in the whitelist
- Alternatively, set `--dates-whitelist` to a comma-separated, case-insensitive list of patterns to match against column names (e.g., `date,time,due,open,close,created`). Only columns whose names contain one of the patterns will be checked
- **Examples of column names that trigger date inference** (with a manual whitelist like `date,time,due,open,close,created`): "start_date", "Observation Time", "timestamp", "Date Closed"
- **Examples that do NOT trigger:** "start_dt", "create_dt", "tmstmp", "close_dt" (unless added to whitelist)
- Use `--dates-whitelist all` to inspect all fields (may cause false positives with numeric data like Unix epoch timestamps)
- Use `--prefer-dmy` to parse dates in day/month/year format instead of month/day/year

### Metadata & Type Inference

| Identifier | Level | Summary | Computation |
|:---|:---:|:---|:---|
| `field` | Variable | The name of the column/header (or its index if `--no-headers` is used). | Extracted from the CSV header row. |
| `type` | Variable | Inferred data type of the column. | Inferred by checking values against: NULL, Integer, Float, Date, DateTime, Boolean (optional), and fallback to String. Data type inferences are **GUARANTEED** as `stats` scans the entire file. |
| `is_ascii` | Variable | Indicates if all characters in the string column are ASCII. | Checked during UTF-8 validation; true if bytes are valid ASCII. |

**Date and DateTime Type Inference:**
- See the **Date/DateTime Statistics** section for full details on how date columns are selected, the default `--dates-whitelist`, example column names that do and do not trigger inference, and the list of supported date formats.
- In summary, Date and DateTime types are only inferred when `--infer-dates` is enabled, and inference relies on matching candidate values against the supported date formats.

**Boolean Type Inference:**
- Boolean type is inferred when `--infer-boolean` is enabled
- A column is inferred as Boolean when its cardinality is 2 and the two values match the boolean patterns specified by `--boolean-patterns` (default: `1:0,t*:f*,y*:n*`)
- Boolean inference automatically enables `--cardinality` computation
- Patterns are case-insensitive and support prefix matching with `*` wildcards
- **Example:** With the default patterns `"t*:f*,y*:n*"`, a column is inferred as Boolean only when it contains exactly two distinct values—one matching a "true" pattern (for example, "true", "truthy", "Truth") and one matching a "false" pattern (for example, "false", "f", "no"); any additional distinct values (such as "falsified" or "falseness") would increase the cardinality above 2 and therefore prevent Boolean inference.

### Descriptive Statistics (Numerical & General)

| Identifier | Level | Summary | Computation |
|:---|:---:|:---|:---|
| `sum` | Variable | Sum of all values in the column. | Rolling sum. Integers sum to Integer until a Float is encountered, then switches to Float. Integer sums that overflow/underflow show `*OVERFLOW*` or `*UNDERFLOW*`. For Floats, returns NaN as the string "NaN", positive infinity as "inf", and negative infinity as "-inf". |
| `min` | Variable | Minimum value found. | Tracks minimum value during the scan. |
| `max` | Variable | Maximum value found. | Tracks maximum value during the scan. |
| `range` | Variable | Difference between Max and Min. | `max - min`. |
| `sort_order` | Variable | Sorting status of the column. | Checked during scan. Returns "ASCENDING", "DESCENDING", or "UNSORTED". |
| `sortiness` | Variable | Measure of how sorted the column is. | Returns a score between -1.0 and 1.0: 1.0 indicates perfectly ascending order, -1.0 indicates perfectly descending order, values in between indicate the general tendency towards ascending or descending order, and 0.0 indicates either no clear ordering or empty/single-element collections. |

### Central Tendency & Dispersion (Streaming)

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

### String Statistics

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

### Quality & Distribution

| Identifier | Level | Summary | Computation |
|:---|:---:|:---|:---|
| `nullcount` | Variable | Count of NULL (empty) values. | Incremented when a field is empty (or matches custom NULL). |
| `n_negative` | Variable | Count of negative values. | Computed for Integer and Float types only. |
| `n_zero` | Variable | Count of zero values. | Computed for Integer and Float types only. |
| `n_positive` | Variable | Count of positive values. | Computed for Integer and Float types only. |
| `max_precision` | Variable | Maximum decimal precision found (Floats). | Tracks the maximum number of digits after the decimal point. |
| `sparsity` | Variable | Fraction of missing (NULL) values. | `nullcount / record_count`. |

### Median & Quartiles (Non-Streaming)

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
| `skewness` | Variable | Measure of asymmetry of the probability distribution. | Quantile-based skewness: `(q3 - (2.0 * q2) + q1) / iqr`. |

### Cardinality & Modes (Non-Streaming)

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

### Percentiles (Non-Streaming)

Requires loading data into memory and sorting. When `--weight <column>` is specified, weighted percentiles are computed using weighted nearest-rank method.

**Requirements:** `--percentiles` or `--everything`

Computed using the [nearest-rank method](https://en.wikipedia.org/wiki/Percentile#The_nearest-rank_method).

| Identifier | Level | Summary | Computation |
|:---|:---:|:---|:---|
| `percentiles` | Variable | Custom percentiles of sorted values. | Nearest rank method for user-defined list. Weighted: weighted nearest-rank method. Multiple percentiles are separated by `QSV_STATS_SEPARATOR` (default: `|`). Special values: "deciles" expands to "10,20,30,40,50,60,70,80,90" and "quintiles" expands to "20,40,60,80". Default: "5,10,40,60,90,95". For dates/datetimes, values are returned in RFC3339 format. |

### File-Level Metadata (JSON Cache)

When stats are cached, the `.stats.csv.json` file includes file-level metadata that enables data fingerprinting and cache validation:

| Field | Description | Computation |
|:---|:---|:---|
| `canonical_input_path` | Canonical path to the input file. | Filesystem canonical (absolute) path. |
| `canonical_stats_path` | Canonical path to the stats output file. | Filesystem canonical (absolute) path. |
| `record_count` | Total number of rows (records). | Count of records processed (excluding header). |
| `field_count` | Total number of columns. | Count of fields in the header/first record. |
| `filesize_bytes` | Total file size in bytes. | Filesystem metadata size. |
| `date_generated` | When the stats were generated. | RFC3339 timestamp (UTC). |
| `compute_duration_ms` | Time taken to compute stats. | Elapsed wall-clock time in milliseconds. |
| `qsv_version` | Version of qsv used to generate stats. | `CARGO_PKG_VERSION` at compile time. Used for cache invalidation when qsv is upgraded. |
| `hash.blake3` | BLAKE3 fingerprint hash of the dataset's stats. | BLAKE3 hash of the first 26 columns ("streaming" stats) + dataset stats (record_count, field_count, filesize_bytes). This allows users to quickly detect duplicate files without having to load the entire file to compute the hash. Especially useful for detecting duplicates of very large files with pre-existing stats cache metadata. |

### Whitespace Visualization

The `--vis-whitespace` option visualizes whitespace characters in the output to make them visible. Note that spaces will only be visualized (using `《_》`) if the entire value is composed of spaces.

The following whitespace markers are used (as defined in the [Rust reference](https://doc.rust-lang.org/reference/whitespace.html)):

| Character | Visualization | Description |
|:---|:---|:---|
| `\t` | `《→》` | Tab |
| `\n` | `《¶》` | Newline |
| `\r` | `《⏎》` | Carriage return |
| `\u{000B}` | `《⋮》` | Vertical tab |
| `\u{000C}` | `《␌》` | Form feed |
| `\u{0085}` | `《␤》` | Next line |
| `\u{200E}` | `《␎》` | Left-to-right mark |
| `\u{200F}` | `《␏》` | Right-to-left mark |
| `\u{2028}` | `《␊》` | Line separator |
| `\u{2029}` | `《␍》` | Paragraph separator |
| `\u{00A0}` | `《⍽》` | Non-breaking space |
| `\u{2003}` | `《emsp》` | Em space |
| `\u{2007}` | `《figsp》` | Figure space |
| `\u{200B}` | `《zwsp》` | Zero width space |

### Performance & Caching

The `stats` command is central to qsv and underpins other "smart" commands (`describegpt`, `frequency`, `joinp`, `pivotp`, `schema`, `sqlp`, `tojsonl`) that use cached statistical information to work smarter & faster.

**Caching Behavior:**
- Statistics are cached in `<FILESTEM>.stats.csv` and optionally `<FILESTEM>.stats.csv.data.jsonl` (with `--stats-jsonl`)
- The arguments and file-level metadata used to generate cached stats are saved in `<FILESTEM>.stats.csv.json`
- If stats have already been computed with similar arguments and the file hasn't changed, stats are loaded from cache instead of recomputing
- Use `--force` to force recomputing stats even if valid cache exists
- Use `--cache-threshold` to control caching behavior (default: 5000ms)

**Memory-Aware Chunking:**
- For non-streaming statistics, dynamically calculate chunk size based on available memory and record sampling
- Override with `QSV_STATS_CHUNK_MEMORY_MB` environment variable (0 for dynamic sizing, positive for fixed limit, -1 for CPU-based chunking)
- Enables processing of arbitrarily large "real-world" files

## `moarstats`
Here are all the additional statistics produced by the `qsv moarstats` command, sourced from `src/cmd/moarstats.rs`.


The `moarstats` command extends an existing stats CSV file (created by the `stats` command) by computing additional statistics that can be derived from existing stats columns and/or by scanning the original CSV file.

**How it works:**
- Looks for `<FILESTEM>.stats.csv` for a given CSV input
- If the stats CSV file doesn't exist, it will first run the `stats` command with configurable options (via `--stats-options`, default: `--infer-dates --infer-boolean --mad --quartiles --percentiles --force --stats-jsonl`) to establish baseline stats
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
| `xsd_type` | Variable | Inferred W3C XML Schema datatype. Infers the most specific XSD type based on field type and min/max values. Works for **all field types**. | Computed from `type`, `min`, and `max` columns. For Integer types, refines to most specific type (e.g., `byte`, `short`, `int`, `long`, `unsignedByte`, `unsignedShort`, `unsignedInt`, `unsignedLong`, `positiveInteger`, `nonNegativeInteger`, `negativeInteger`, `nonPositiveInteger`, or `integer`) based on min/max ranges. Also detects Gregorian date types (`gYear`, `gYearMonth`, `gMonthDay`, `gDay`, `gMonth`) with confidence markers (`?` = more confident from thorough scan, `??` = less confident from quick scan). For other types: Float → `decimal`, String → `string`, Date → `date`, DateTime → `dateTime`, Boolean → `boolean`, NULL → empty string. If min/max are not available for Integer types, defaults to `integer`. See: [XML Schema Part 2: Datatypes](https://www.w3.org/TR/xmlschema-2/) |

### Advanced Statistics

These statistics require the `--advanced` flag and reading the entire CSV file to collect all values for computation. They are computationally expensive.

| Identifier | Level | Summary | Computation |
|:---|:---:|:---|:---|
| `kurtosis` | Variable | Excess kurtosis. Measures the "tailedness" of the distribution. Positive values indicate heavy tails, negative values indicate light tails. Values near 0 indicate a normal distribution. | Computed from all values in the column. Uses precalculated mean and variance from baseline stats for efficiency. Requires: `mean`, `variance` (or `stddev`). See: [Kurtosis](https://en.wikipedia.org/wiki/Kurtosis) |
| `bimodality_coefficient` | Variable | Bimodality Coefficient. Measures whether a distribution has two modes (peaks) or is unimodal. BC < 0.555 indicates unimodal, BC >= 0.555 indicates bimodal/multimodal. | Computed as `(skewness² + 1) / (kurtosis + 3)`. Requires: `skewness` (from base stats) and `kurtosis` (from `--advanced` flag). See: [Bimodality](https://en.wikipedia.org/wiki/Bimodality) |
| `gini_coefficient` | Variable | Gini Coefficient. Measures inequality/dispersion in the distribution. Values range from 0 (perfect equality) to 1 (maximum inequality). | Computed from all values in the column. Uses precalculated sum from baseline stats for efficiency. Requires: `sum`. See: [Gini Coefficient](https://en.wikipedia.org/wiki/Gini_coefficient) |
| `atkinson_index_(ε)` | Variable | Atkinson Index. Measures inequality in the distribution with a sensitivity parameter. Values range from 0 (perfect equality) to 1 (maximum inequality). The Atkinson Index is a more general form of the Gini coefficient that allows for different sensitivity to inequality. | Computed from all values in the column. Uses precalculated mean from baseline stats for efficiency. The epsilon (ε) parameter controls sensitivity to inequality (configurable via `--epsilon`, default: 1.0). Higher epsilon values indicate greater sensitivity to inequality. Requires: `mean`. See: [Atkinson Index](https://en.wikipedia.org/wiki/Atkinson_index) |
| `shannon_entropy` | Variable | Shannon Entropy. Measures the information content/uncertainty in the distribution. Higher values indicate more diversity, lower values indicate more concentration. Values range from 0 (all values identical) to log2(n) where n is the number of unique values. | Computed using the `frequency` command with `--limit 0` to collect all frequencies, then calculates: `H(X) = -Σ p_i * log2(p_i)` where p_i is the probability of value i. Works for **all field types** (not just numeric). For all-unique fields, returns log2(n). See: [Entropy (Information Theory)](https://en.wikipedia.org/wiki/Entropy_(information_theory)) |
| `normalized_entropy` | Variable | Normalized Entropy. Normalized version of Shannon Entropy scaled to [0, 1]. Values range from 0 (all values identical) to 1 (all values equally distributed). | Computed as `shannon_entropy / log2(cardinality)`. Requires: `shannon_entropy` (from `--advanced` flag) and `cardinality` (from base stats). If cardinality is 0 or 1, returns 0. |

### Bivariate Statistics

These statistics examine relationships between pairs of columns in a dataset. They are computed when the `--bivariate` flag is used and require an indexed CSV file (index will be auto-created if missing). Bivariate statistics are output to a separate file: `<FILESTEM>.stats.bivariate.csv`.

**Note**: Bivariate statistics require reading the entire CSV file and are computationally expensive. For large files (>= 10k records), parallel chunked processing is used when an index is available. For smaller files or when no index exists, sequential processing is used.

**Performance Optimizations:**
- Date parsing cache to avoid re-parsing same date strings
- String interning to reduce allocations for repeated values
- Batch string conversions to process multiple field pairs efficiently
- Early termination for zero-variance fields (skip all correlation computations)
- Streaming algorithms (Welford's online) for Pearson correlation and covariance
- Lazy value collection (only store values if needed for Spearman/Kendall)

**Multi-Dataset Bivariate Statistics**: When using `--join-inputs`, multiple datasets can be joined internally before computing relationships. This allows analyzing relationships across datasets that share common join keys. The joined dataset is automatically indexed before bivariate statistics computation. Output file: `<FILESTEM>.stats.bivariate.joined.csv`.

| Identifier | Level | Summary | Computation |
|:---|:---:|:---|:---|
| `pearson_correlation` | Pairwise | Pearson product-moment correlation coefficient. Measures linear correlation between two numeric/date fields. Values range from -1 (perfect negative correlation) to +1 (perfect positive correlation). 0 indicates no linear correlation. | Computed using Welford's online algorithm for efficient streaming computation across chunks. Requires both fields to be numeric or date types. Formula: `covariance / (stddev_x * stddev_y)`. See: [Pearson Correlation](https://en.wikipedia.org/wiki/Pearson_correlation_coefficient) |
| `spearman_correlation` | Pairwise | Spearman's rank correlation coefficient. Measures monotonic relationship between two numeric/date fields (not just linear). Values range from -1 to +1. More robust to outliers than Pearson correlation. | Computed by ranking both fields and then computing Pearson correlation on the ranks. Handles ties by averaging ranks. Requires both fields to be numeric or date types. See: [Spearman's Rank Correlation](https://en.wikipedia.org/wiki/Spearman%27s_rank_correlation_coefficient) |
| `kendall_tau` | Pairwise | Kendall's tau rank correlation coefficient. Measures ordinal association between two numeric/date fields. Values range from -1 to +1. More robust to outliers and handles ties better than Spearman. | Computed by counting concordant and discordant pairs using efficient O(n log n) merge sort algorithm. Formula accounts for ties in both variables. Requires both fields to be numeric or date types. See: [Kendall's Tau](https://en.wikipedia.org/wiki/Kendall_rank_correlation_coefficient) |
| `covariance_sample` | Pairwise | Sample covariance. Measures how two numeric/date fields vary together. Positive values indicate positive relationship, negative values indicate inverse relationship. | Computed using Welford's online algorithm. Formula: `sum((x - mean_x) * (y - mean_y)) / (n - 1)`. Requires both fields to be numeric or date types. |
| `covariance_population` | Pairwise | Population covariance. Same as sample covariance but uses population formula (divides by n instead of n-1). | Computed using Welford's online algorithm. Formula: `sum((x - mean_x) * (y - mean_y)) / n`. Requires both fields to be numeric or date types. |
| `mutual_information` | Pairwise | Mutual Information. Measures the amount of information obtained about one field by observing another. Values range from 0 (independent) to positive infinity. Works for **all field types** (numeric, date, string). | Computed from joint and marginal probability distributions. Formula: `MI(X,Y) = sum(p(x,y) * log2(p(x,y) / (p(x) * p(y))))`. Higher values indicate stronger relationship. Can be expensive for high-cardinality fields (use `--cardinality-threshold` to skip). See: [Mutual Information](https://en.wikipedia.org/wiki/Mutual_information) |
| `normalized_mutual_information` | Pairwise | Normalized Mutual Information. Normalized version of mutual information, scaled by the geometric mean of individual entropies. Values range from 0 (independent) to 1 (perfectly dependent). | Computed as `MI(X,Y) / sqrt(H(X) * H(Y))` where H(X) and H(Y) are Shannon entropies of individual fields. Requires mutual information computation. See: [Normalized Mutual Information](https://en.wikipedia.org/wiki/Mutual_information#Normalized_variants) |
| `n_pairs` | Pairwise | Number of valid pairs used in computation. Indicates how many non-null value pairs were available for computing the relationship statistics. | Count of records where both fields have non-empty values. |

**Configuration Options:**
- `--bivariate-stats`: Select specific statistics (pearson, spearman, kendall, covariance, mi, nmi) or use "all" or "fast" (pearson + covariance)
- `--cardinality-threshold`: Skip mutual information for field pairs where either field exceeds cardinality threshold (default: 1,000,000)
- `--join-inputs`: Join multiple datasets before computing bivariate statistics
- `--join-keys`: Specify join keys for each dataset
- `--join-type`: Specify join type (inner, left, right, full; default: inner)

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

## `pragmastat`

The `pragmastat` command computes robust, median-of-pairwise statistics using the [Pragmastat library](https://pragmastat.dev/) (v10.0.0). Designed for messy, heavy-tailed, or outlier-prone data where mean/stddev can mislead.

Sourced from `src/cmd/pragmastat.rs`.

**Key Features:**
- Only finite numeric values are used; non-numeric/NaN/Inf values are ignored
- Each column is treated as its own sample (two-sample compares columns, not rows)
- Non-numeric columns appear with n=0 and empty estimator cells
- Loads all numeric values into memory

### One-Sample Mode (Default)

Output columns: `field, n, center, spread, center_lower, center_upper, spread_lower, spread_upper`

| Identifier | Level | Summary | Computation |
|:---|:---:|:---|:---|
| `field` | Variable | Column name (or 1-based index if `--no-headers`). | From CSV header. |
| `n` | Variable | Count of finite numeric values. | Count after filtering non-numeric, NaN, Inf. |
| `center` | Variable | Hodges-Lehmann estimator — robust location. | Median of pairwise averages. Tolerates up to 29% corrupted data. Like the mean but stable with outliers. |
| `spread` | Variable | Shamos estimator — robust dispersion. | Median of pairwise absolute differences. Same units as data. Also tolerates up to 29% corrupted data. |
| `center_lower` | Variable | Lower confidence bound for center. | Exact under weak symmetry, with error rate = misrate. |
| `center_upper` | Variable | Upper confidence bound for center. | Exact under weak symmetry, with error rate = misrate. |
| `spread_lower` | Variable | Lower confidence bound for spread. | Randomized (bootstrap); error rate = misrate. |
| `spread_upper` | Variable | Upper confidence bound for spread. | Randomized (bootstrap); error rate = misrate. |

### Two-Sample Mode

Enabled with the `--twosample` option. Computes statistics for all unordered column pairs.

Output columns: `field_x, field_y, n_x, n_y, shift, ratio, disparity, shift_lower, shift_upper, ratio_lower, ratio_upper, disparity_lower, disparity_upper`

| Identifier | Level | Summary | Computation |
|:---|:---:|:---|:---|
| `field_x`, `field_y` | Pairwise | Column names being compared. | From CSV header. |
| `n_x`, `n_y` | Pairwise | Counts of finite numeric values. | Per-column counts after filtering non-numeric/NaN/Inf. |
| `shift` | Pairwise | Hodges-Lehmann difference — robust location difference. | Median of pairwise differences between columns. Negative means first column tends to be lower. |
| `ratio` | Pairwise | Robust multiplicative ratio. | `exp(shift(log x, log y))`. Use for positive-valued quantities (latency, price, concentration). Requires all values > 0. |
| `disparity` | Pairwise | Robust effect size. | `shift / (average spread of x and y)`. |
| `shift_lower`, `shift_upper` | Pairwise | Confidence bounds for shift. | Exact; error rate = misrate. If bounds exclude 0, the difference is reliable. Ties may be conservative. |
| `ratio_lower`, `ratio_upper` | Pairwise | Confidence bounds for ratio. | Exact; error rate = misrate. If bounds exclude 1, the difference is reliable. Requires all values > 0. |
| `disparity_lower`, `disparity_upper` | Pairwise | Confidence bounds for disparity. | Randomized (Bonferroni combination); error rate = misrate. If bounds exclude 0, the disparity is reliable. |

### Options

| Option | Default | Description |
|:---|:---|:---|
| `--twosample` / `-t` | off | Compute two-sample estimators for all column pairs. |
| `--select <cols>` / `-s` | all columns | Select columns for analysis using qsv's column selection syntax. Non-numeric columns appear with n=0. In two-sample mode, all pairs of selected columns are computed. |
| `--misrate <n>` / `-m` | `0.001` | Probability that bounds fail to contain the true parameter. Lower values produce wider bounds. Must be achievable for the given sample size. Use `1e-3` for everyday analysis or `1e-6` for critical decisions. |
| `--output <file>` / `-o` | stdout | Write output to file instead of stdout. |
| `--delimiter <c>` / `-d` | `,` | Field delimiter for reading/writing CSV data. |
| `--no-headers` / `-n` | off | When set, the first row will not be treated as headers. |

### When Values Are Blank

Cells are empty (blank) when:
- **No numeric data (n=0):** The column contains no finite numeric values
- **Positivity required:** `ratio`, `ratio_lower`, and `ratio_upper` require all values > 0
- **Sparity required:** `spread`, `spread_lower`, `spread_upper`, `disparity`, `disparity_lower`, and `disparity_upper` need real variability (not tie-dominant data)
- **Insufficient data for bounds:** All bounds columns need enough data for the requested misrate; try a higher misrate or more data

See: [Pragmastat manual (PDF)](https://github.com/AndreyAkinshin/pragmastat/releases/download/v10.0.0/pragmastat-v10.0.0.pdf), [pragmastat.dev](https://pragmastat.dev/)

## `frequency`

The `frequency` command computes exact frequency distribution tables for CSV columns, with support for multiple output formats, ranking strategies, and weighted frequencies.

**Key Features:**
- Computes exact frequency counts (unlike approximate sketches)
- Multiple ranking strategies for handling tied values
- Weighted frequency support using a specified weight column
- CSV and JSON/TOON output modes
- Integration with stats cache for memory optimization
- Memory-aware chunking for large datasets
- Parallel processing support with indexing

**Stats Cache Integration:**
When the stats cache exists (created by `qsv stats --stats-jsonl`), the frequency command uses it to:
- Detect ID columns (cardinality == rowcount) and short-circuit frequency compilation
- Pre-allocate appropriate hashmap capacity based on cardinality
- Avoid building hashmaps for all-unique columns

Without stats cache, frequency will compute frequencies for ALL columns, even ID columns, which can use significant memory.

For more examples, see https://github.com/dathere/qsv/blob/master/tests/test_frequency.rs.

### Frequency Table Output

In CSV output mode (default), the table is formatted as CSV data with the following columns:

| Column | Description |
|:---|:---|
| `field` | Column name (or 1-based index if `--no-headers` is used) |
| `value` | The value from the column |
| `count` | Frequency count (or weighted sum if `--weight` is used) |
| `percentage` | Percentage of total (count/total * 100) |
| `rank` | Rank based on count (1 = most frequent, or least frequent if `--asc` is used) |

**Formatting Options:**
- `--pct-dec-places <arg>` — decimal places for percentage rounding (default: -5). When negative, the number of decimal places is automatically determined to the minimum needed to represent the percentage accurately, up to the absolute value of the negative number.
- `--no-trim` — don't trim whitespace from values when computing frequencies. By default, leading and trailing whitespace is trimmed.
- `--vis-whitespace` — visualize whitespace characters in the output using the same markers as `stats` (see [Whitespace Visualization](#whitespace-visualization)).

**Special Values:**
- `<ALL_UNIQUE>` (configurable via `--all-unique-text`): For ID columns detected via stats cache, indicates all values are unique. Count equals row count, percentage is 100%, rank is 0.
- `Other (N)` (configurable via `--other-text`, default: `Other`): When `--limit` is used, remaining values are grouped into this category. N indicates the count of unique values grouped. Rank is 0. Use `--no-other` (alias for `--other-text "<NONE>"`) to exclude the "Other" category entirely.
- `(NULL)` (configurable via `--null-text`, default: `(NULL)`): Represents empty/missing values. Can be excluded with `--no-nulls` (alias for `--null-text "<NONE>"`).

**Limit Behavior:**
- `--limit N` (positive): Keep only top N most frequent values
- `--limit -N` (negative): Keep only values with count >= N
- `--limit 0`: No limit, return all values
- `--unq-limit N`: For all-unique columns, limit to N sample values (default: 10)
- `--lmt-threshold N`: Only apply limits when unique count >= N (default: 0 = always apply)

**Sorting:**
- Default: Descending order by count (most frequent first)
- `--asc`: Ascending order by count (least frequent first). Note: This also reverses ranking - least frequent values get rank 1.
- `--other-sorted`: Include "Other" category in sorted order instead of at the end

### Ranking Strategies

The `--rank-strategy` option controls how ranks are assigned when multiple values have the same count. See https://en.wikipedia.org/wiki/Ranking for more info.

| Strategy | Description | Example (counts: 4, 3, 3, 2) |
|:---|:---|:---|
| `dense` | Consecutive integers regardless of ties (1223 ranking) | 1, 2, 2, 3 |
| `min` | Tied items receive minimum rank position (1224 ranking) | 1, 2, 2, 4 |
| `max` | Tied items receive maximum rank position (1334 ranking) | 1, 3, 3, 4 |
| `ordinal` | Next rank is current rank plus 1 (1234 ranking) | 1, 2, 3, 4 |
| `average` | Tied items receive average of positions (1 2.5 2.5 4 ranking) | 1, 2.5, 2.5, 4 |

**Note:** Tied values with the same rank are sorted alphabetically within their rank group.

### NULL Handling

The frequency command provides several options for controlling how NULL (empty) values are handled:

| Option | Default | Description |
|:---|:---|:---|
| `--null-text <arg>` | `(NULL)` | Customize the display text for NULL values. Set to `<NONE>` to exclude NULLs entirely. |
| `--no-nulls` | off | Don't include NULLs in the frequency table. Alias for `--null-text "<NONE>"`. |
| `--null-sorted` | off | Sort NULL entries with other values by count instead of placing them at the end of the frequency table (after "Other" if present). |
| `--pct-nulls` | off | Include NULL values in percentage and rank calculations. When disabled (default), percentages are "valid percentages" with NULLs excluded from the denominator, and NULL entries display empty percentage and rank values. When enabled, NULLs are included in the denominator (original behavior). Has no effect when `--no-nulls` is set. |

### Column Filtering

The frequency command supports filtering columns from the frequency analysis:

| Option | Description |
|:---|:---|
| `--no-float <cols>` | Exclude Float columns from frequency analysis. Floats typically contain continuous values where frequency tables are not meaningful. Use `--no-float "*"` to exclude ALL Float columns, or specify a comma-separated list of Float columns to INCLUDE as exceptions (e.g., `--no-float price,rate` excludes all Floats except "price" and "rate"). Requires stats cache for type detection. |
| `--stats-filter <expr>` | Filter columns based on their statistics using a Luau expression. Columns where the expression evaluates to `true` are EXCLUDED. Available fields include: `field`, `type`, `is_ascii`, `cardinality`, `nullcount`, `sum`, `min`, `max`, `range`, `sort_order`, `min_length`, `max_length`, `mean`, `stddev`, `variance`, `cv`, `sparsity`, `q1`, `q2_median`, `q3`, `iqr`, `mad`, `skewness`, `mode`, `antimode`, `n_negative`, `n_zero`, `n_positive`, etc. Examples: `"nullcount > 1000"`, `"type == 'Float'"`, `"cardinality > 500 and nullcount > 0"`. Requires stats cache and the `luau` feature. |

### Weighted Frequencies

When the `--weight <column>` option is specified, frequency counts are multiplied by the weight value for each row.

**Weight Handling:**
- Weight column must be numeric
- Weight column is automatically excluded from frequency computation
- Missing or unparsable weights default to 1.0
- Zero, negative, NaN, and infinite weights are ignored and do not contribute to frequencies
- Weight tolerance calculation uses stats cache (stddev/range/mean) for scale-aware tolerance when available

**Output:**
- Count column shows weighted sum (displayed as rounded integer)
- Percentage calculated as: `weight / total_weight * 100`
- Ranking based on weighted sums

### Stats Cache Integration

The `frequency` command leverages the stats cache (created by `qsv stats --stats-jsonl`) to optimize memory usage and performance:

**ID Column Detection:**
When stats cache exists, columns where `cardinality == rowcount` are detected as ID columns. For these columns:
- Frequency compilation is short-circuited (no hashmap built)
- Output shows single `<ALL_UNIQUE>` entry with count = rowcount, percentage = 100%, rank = 0
- Saves significant memory for large datasets with ID columns

**Memory Optimization:**
- Hashmap capacity pre-allocated based on cardinality from stats cache
- For parallel processing, capacity divided by number of chunks
- Reduces allocations and improves performance

**Disabling Stats Cache:**
Set `QSV_STATSCACHE_MODE=none` to force computing frequencies for ALL columns including ID columns. Useful when you need a "complete" frequency table even for ID columns. In this case, use `--unq-limit` to avoid memory issues with large cardinality columns.

**Creating Stats Cache:**
```bash
# Create stats cache with cardinality for frequency optimization
qsv stats --cardinality --stats-jsonl data.csv

# Or create with all stats
qsv stats --everything --stats-jsonl data.csv
```

### JSON/TOON Output

The `--json` or `--pretty-json` flags output frequency tables as nested JSON. The `--toon` flag outputs in TOON format (compact, human-readable encoding for LLM prompts).

**JSON Structure:**
```json
{
  "input": "filename.csv",
  "description": "command arguments",
  "rowcount": 1000,
  "fieldcount": 5,
  "rank_strategy": "dense",
  "fields": [
    {
      "field": "column_name",
      "type": "String",
      "cardinality": 10,
      "nullcount": 0,
      "sparsity": 0.0,
      "uniqueness_ratio": 0.01,
      "stats": [
        {"name": "sum", "value": 1000},
        {"name": "min", "value": "A"},
        {"name": "max", "value": "Z"},
        {"name": "range", "value": null},
        {"name": "sort_order", "value": "UNSORTED"},
        {"name": "mean", "value": null},
        ...
      ],
      "frequencies": [
        {"value": "A", "count": 500, "percentage": 50.0, "rank": 1},
        {"value": "B", "count": 300, "percentage": 30.0, "rank": 2},
        ...
      ]
    }
  ]
}
```

**Additional Stats in JSON Output:**
When `--no-stats` is NOT set, JSON output includes 17 additional statistics per field:
1. `sum` - Sum of numeric values
2. `min` - Minimum value
3. `max` - Maximum value
4. `range` - Range (max - min)
5. `sort_order` - ASCENDING, DESCENDING, or UNSORTED
6. `mean` - Arithmetic mean
7. `sem` - Standard error of the mean
8. `geometric_mean` - Geometric mean
9. `harmonic_mean` - Harmonic mean
10. `stddev` - Standard deviation
11. `variance` - Variance
12. `cv` - Coefficient of variation
13. String length stats (for String types): `min_length`, `max_length`, `avg_length`, `stddev_length`
14. `nullcount` - Count of NULL values
15. `sparsity` - Fraction of NULL values
16. `max_precision` - Maximum decimal precision (for Float types)

### Memory-Aware Processing

The frequency command defaults to memory-aware chunking for large datasets to avoid out-of-memory errors. Unlike `stats` (which may default to CPU-based chunking), frequency’s default strategy is memory-aware because it builds hash tables that benefit from predictable memory usage. However, it can be explicitly configured to use CPU-based chunking via `QSV_FREQ_CHUNK_MEMORY_MB = -1`.

**Chunking Behavior:**
- Automatically enabled for indexed files
- Chunk size calculated based on:
  - Available memory
  - Record sampling (samples first 1000 records)
  - Estimated hashmap overhead for frequency tables
- Controlled by `QSV_FREQ_CHUNK_MEMORY_MB` environment variable:
  - Not set or `0`: Dynamic sizing based on available memory and sampling
  - Positive N: Fixed memory limit of N MB per chunk
  - `-1`: CPU-based chunking (num_records / num_CPUs)

**Parallel Processing:**
- Requires an index file (`qsv index data.csv`)
- Automatically enabled when index exists (disable with `--jobs 1`)
- Each chunk processed independently, then merged
- For unindexed files, falls back to sequential processing

**Memory Estimation:**
The command estimates memory per record as:
- Base record size (sum of field lengths)
- Hashmap overhead (~24 bytes per entry + value size)
- Additional overhead for Vec capacity (~25%)

**Auto-Index Creation:**
If memory check fails and file is not indexed:
- Attempts to auto-create index
- Switches to parallel processing if successful
- Falls back to sequential if index creation fails

For configuration details, see https://github.com/dathere/qsv/blob/master/docs/ENVIRONMENT_VARIABLES.md
