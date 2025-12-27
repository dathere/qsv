# QSV Stats Definitions

This document enumerates all the statistics produced by the `qsv stats` command, sourced from `src/cmd/stats.rs`.

Each statistic is categorized by its relevant section, with its identifier (column name), summary, computation method, and level (File or Variable).

> **Note**: "Streaming" statistics are computed in constant memory. "Non-Streaming" statistics require loading the column data into memory (or multiple passes) and may use approximation or exact calculation depending on configuration.

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
- Use `--dates-whitelist all` to inspect all fields (may cause false positives with numeric data)
- Use `--prefer-dmy` to parse dates in day/month/year format instead of month/day/year

## Metadata & Type Inference

| Identifier | Level | Summary | Computation |
|:---|:---:|:---|:---|
| `field` | Variable | The name of the column/header. | Extracted from the CSV header row. |
| `type` | Variable | Inferred data type of the column. | Inferred by checking values against: NULL, Integer, Float, Date, DateTime, Boolean (optional), and fallback to String. |
| `subtype_xsd` | Variable | Inferred XSD data subtype (if enabled). | Refined inference for Integers (Requesting `byte`, `short`, `int`, `long`) and Floats (`decimal`). |
| `is_ascii` | Variable | Indicates if all characters in the string column are ASCII. | Checked during UTF-8 validation; true if bytes are valid ASCII. |

**Note on Boolean Type Inference:** Boolean type is inferred when `--infer-boolean` is enabled. A column is inferred as Boolean when its cardinality is 2 and the two values match the boolean patterns specified by `--boolean-patterns` (default: `1:0,t*:f*,y*:n*`). Boolean inference automatically enables `--cardinality` computation. Patterns are case-insensitive and support prefix matching with `*` wildcards.

## Descriptive Statistics (Numerical & General)

| Identifier | Level | Summary | Computation |
|:---|:---:|:---|:---|
| `sum` | Variable | Sum of all values in the column. | Rolling sum. Integers sum to Integer until a Float is encountered, then switches to Float. Integer sums that overflow/underflow show `*OVERFLOW*` or `*UNDERFLOW*`. |
| `min` | Variable | Minimum value found. | Tracks minimum value during the scan. |
| `max` | Variable | Maximum value found. | Tracks maximum value during the scan. |
| `range` | Variable | Difference between Max and Min. | `max - min`. |
| `sort_order` | Variable | Sorting status of the column. | Checked during scan. Returns "Ascending", "Descending", or "Not Sorted". |
| `sortiness` | Variable | Measure of how sorted the column is. | Computed metric indicating the degree of sortedness (0.0 to 1.0 scale). |

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

Computed for columns inferred as `String` type.

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

| Identifier | Level | Summary | Computation |
|:---|:---:|:---|:---|
| `median` | Variable | Median value (50th percentile). | Middle value of sorted data (or average of two middle values). Weighted: uses weighted nearest-rank method. |
| `mad` | Variable | Median Absolute Deviation. | Median of the absolute deviations from the data's median. Weighted: weighted median of absolute deviations. For dates/datetimes, returned in days. |
| `q1` | Variable | First Quartile (25th percentile). | Value at 25% rank (Method 3). Weighted: value at which cumulative weight first reaches 25% of total weight. |
| `q2_median` | Variable | Second Quartile (Median). | Same as `median`. |
| `q3` | Variable | Third Quartile (75th percentile). | Value at 75% rank (Method 3). Weighted: value at which cumulative weight first reaches 75% of total weight. |
| `iqr` | Variable | Interquartile Range. | `q3 - q1`. For dates/datetimes, returned in days with at least 5 decimal places. |
| `lower_outer_fence` | Variable | Lower bound for extreme outliers. | `q1 - (3.0 * iqr)`. For dates/datetimes, returned in RFC3339 format. |
| `lower_inner_fence` | Variable | Lower bound for outliers. | `q1 - (1.5 * iqr)`. For dates/datetimes, returned in RFC3339 format. |
| `upper_inner_fence` | Variable | Upper bound for outliers. | `q3 + (1.5 * iqr)`. For dates/datetimes, returned in RFC3339 format. |
| `upper_outer_fence` | Variable | Upper bound for extreme outliers. | `q3 + (3.0 * iqr)`. For dates/datetimes, returned in RFC3339 format. |
| `skewness` | Variable | Measure of asymmetry of the distribution. | Quantile-based skewness: `((q3 - q2) - (q2 - q1)) / iqr` or `(q3 - (2.0 * q2) + q1) / iqr`. |

## Cardinality & Modes (Non-Streaming)

When `--weight <column>` is specified, weighted versions are computed. For weighted modes, `mode_occurrences` is the maximum weight (rounded). For weighted antimodes, `antimode_occurrences` is the minimum weight (rounded).

Multiple modes/antimodes are separated by the `QSV_STATS_SEPARATOR` environment variable (default: `|`).

| Identifier | Level | Summary | Computation |
|:---|:---:|:---|:---|
| `cardinality` | Variable | Count of unique values. | Count of distinct entries in the column. Weighted: count of unique values (weights are not considered for uniqueness). |
| `uniqueness_ratio` | Variable | Ratio of unique values to total records. | `cardinality / record_count`. |
| `mode` | Variable | The most frequent value(s). | Value(s) with the highest frequency count. Weighted: value(s) with the highest weight. Multimodal-aware. |
| `mode_count` | Variable | Number of modes found. | Count of values tied for highest frequency. |
| `mode_occurrences` | Variable | Frequency count of the mode. | Number of times the mode appears. Weighted: maximum weight (rounded). |
| `antimode` | Variable | The least frequent value(s). | Value(s) with the lowest frequency count (non-zero). Returns `*ALL` if all values are unique. Limited to first 10 values, truncating after 100 characters (configurable with `QSV_ANTIMODES_LEN`). Weighted: value(s) with the lowest weight. |
| `antimode_count` | Variable | Number of antimodes found. | Count of values tied for lowest frequency. |
| `antimode_occurrences` | Variable | Frequency count of the antimode. | Number of times the antimode appears. Weighted: minimum weight (rounded). |

## Percentiles (Non-Streaming)

Requires loading data into memory and sorting. When `--weight <column>` is specified, weighted percentiles are computed using weighted nearest-rank method.

| Identifier | Level | Summary | Computation |
|:---|:---:|:---|:---|
| `percentiles` | Variable | Custom percentiles. | Nearest rank method for user-defined list. Weighted: weighted nearest-rank method. Multiple percentiles are separated by `QSV_STATS_SEPARATOR` (default: `|`). Special values: "deciles" expands to "10,20,30,40,50,60,70,80,90" and "quintiles" expands to "20,40,60,80". Default: "5,10,40,60,90,95". For dates/datetimes, values are returned in RFC3339 format. |

## Dataset Statistics (File Level)

These statistics appear as additional rows with the prefix `qsv__` when `--dataset-stats` is enabled. The `qsv__value` column is added to hold the values for these dataset-level statistics.

| Identifier | Level | Summary | Computation |
|:---|:---:|:---|:---|
| `qsv__rowcount` | File | Total number of rows (records). | Count of records processed (excluding header). |
| `qsv__columncount` | File | Total number of columns. | Count of fields in the header/first record. |
| `qsv__filesize_bytes` | File | Total file size in bytes. | Filesystem metadata size. |
| `qsv__fingerprint_hash`| File | Cryptographic hash of the dataset's stats. | BLAKE3 hash of the first 26 columns ("streaming" stats) + dataset stats (rowcount, columncount, filesize_bytes). |
| `qsv__value` | File | The value column for dataset stats. | Holds the value for the `qsv__` row (e.g., the row count integer, column count, file size, or fingerprint hash). |
