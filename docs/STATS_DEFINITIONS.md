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
