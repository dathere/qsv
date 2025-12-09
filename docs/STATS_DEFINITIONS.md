# QSV Stats Definitions

This document enumerates all the statistics produced by the `qsv stats` command, sourced from `src/cmd/stats.rs`.

Each statistic is categorized by its relevant section, with its identifier (column name), summary, computation method, and level (File or Variable).

> **Note**: "Streaming" statistics are computed in constant memory. "Non-Streaming" statistics require loading the column data into memory (or multiple passes) and may use approximation or exact calculation depending on configuration.

## Metadata & Type Inference

| Identifier | Level | Summary | Computation |
|:---|:---:|:---|:---|
| `field` | Variable | The name of the column/header. | Extracted from the CSV header row. |
| `type` | Variable | Inferred data type of the column. | Inferred by checking values against: NULL, Integer, Float, Date, DateTime, Boolean (optional), and fallback to String. |
| `subtype_xsd` | Variable | Inferred XSD data subtype (if enabled). | Refined inference for Integers (Requesting `byte`, `short`, `int`, `long`) and Floats (`decimal`). |
| `is_ascii` | Variable | Indicates if all characters in the string column are ASCII. | Checked during UTF-8 validation; true if bytes are valid ASCII. |

## Descriptive Statistics (Numerical & General)

| Identifier | Level | Summary | Computation |
|:---|:---:|:---|:---|
| `sum` | Variable | Sum of all values in the column. | Rolling sum. Integers sum to Integer until a Float is encountered, then switches to Float. |
| `min` | Variable | Minimum value found. | Tracks minimum value during the scan. |
| `max` | Variable | Maximum value found. | Tracks maximum value during the scan. |
| `range` | Variable | Difference between Max and Min. | `max - min`. |
| `sort_order` | Variable | Sorting status of the column. | Checked during scan (Ascending, Descending, Not Sorted). |
| `sortiness` | Variable | Measure of how sorted the column is. | (Not explicitly in all output modes, but internal metric). |

## Central Tendency & Dispersion (Streaming)

Computed using Welford's online algorithm for single-pass accuracy.

| Identifier | Level | Summary | Computation |
|:---|:---:|:---|:---|
| `mean` | Variable | Arithmetic mean (average). | Welford's algorithm mean. |
| `sem` | Variable | Standard Error of the Mean. | `stddev / sqrt(count)`. |
| `geometric_mean` | Variable | Geometric mean. | Online calculation using logarithms. |
| `harmonic_mean` | Variable | Harmonic mean. | Online calculation using reciprocals. |
| `stddev` | Variable | Standard deviation (sample). | Welford's algorithm standard deviation. |
| `variance` | Variable | Variance (sample). | Square of standard deviation. |
| `cv` | Variable | Coefficient of Variation. | `(stddev / mean) * 100`. |

## String Statistics

Computed for columns inferred as `String` type.

| Identifier | Level | Summary | Computation |
|:---|:---:|:---|:---|
| `min_length` | Variable | Length of the shortest string. | Tracks minimum length in bytes. |
| `max_length` | Variable | Length of the longest string. | Tracks maximum length in bytes. |
| `sum_length` | Variable | Sum of lengths of all strings. | Accumulates length of every value. |
| `avg_length` | Variable | Average string length. | `sum_length / count`. |
| `stddev_length` | Variable | Standard deviation of string lengths. | Welford's algorithm on lengths. |
| `variance_length` | Variable | Variance of string lengths. | Square of `stddev_length`. |
| `cv_length` | Variable | Coefficient of Variation of lengths. | `(stddev_length / avg_length) * 100`. |

## Quality & Distribution

| Identifier | Level | Summary | Computation |
|:---|:---:|:---|:---|
| `nullcount` | Variable | Count of NULL (empty) values. | Incremented when a field is empty (or matches custom NULL). |
| `max_precision` | Variable | Maximum decimal precision found (Floats). | Tracks the maximum number of digits after the decimal point. |
| `sparsity` | Variable | Fraction of missing (NULL) values. | `nullcount / record_count`. |
| `skewness` | Variable | Measure of asymmetry of the distribution. | Quantile-based skewness (Bowley's): `((q3 - q2) - (q2 - q1)) / iqr`. |

## Median & Quartiles (Non-Streaming)

Requires loading data into memory and sorting.

| Identifier | Level | Summary | Computation |
|:---|:---:|:---|:---|
| `median` | Variable | Median value (50th percentile). | Middle value of sorted data (or average of two middle values). |
| `mad` | Variable | Median Absolute Deviation. | Median of the absolute deviations from the data's median. |
| `q1` | Variable | First Quartile (25th percentile). | Value at 25% rank (Method 3). |
| `q2_median` | Variable | Second Quartile (Median). | Same as `median`. |
| `q3` | Variable | Third Quartile (75th percentile). | Value at 75% rank (Method 3). |
| `iqr` | Variable | Interquartile Range. | `q3 - q1`. |
| `lower_outer_fence` | Variable | Lower bound for extreme outliers. | `q1 - (3.0 * iqr)`. |
| `lower_inner_fence` | Variable | Lower bound for outliers. | `q1 - (1.5 * iqr)`. |
| `upper_inner_fence` | Variable | Upper bound for outliers. | `q3 + (1.5 * iqr)`. |
| `upper_outer_fence` | Variable | Upper bound for extreme outliers. | `q3 + (3.0 * iqr)`. |
| `percentiles` | Variable | Custom percentiles. | Nearest rank method for user-defined list. |

## Cardinality & Modes (Non-Streaming)

| Identifier | Level | Summary | Computation |
|:---|:---:|:---|:---|
| `cardinality` | Variable | Count of unique values. | Count of distinct entries in the column. |
| `uniqueness_ratio` | Variable | Ratio of unique values to total records. | `cardinality / record_count`. |
| `mode` | Variable | The most frequent value(s). | Value(s) with the highest frequency count. |
| `mode_count` | Variable | Number of modes found. | Count of values tied for highest frequency. |
| `mode_occurrences` | Variable | Frequency count of the mode. | Number of times the mode appears. |
| `antimode` | Variable | The least frequent value(s). | Value(s) with the lowest frequency count (non-zero). |
| `antimode_count` | Variable | Number of antimodes found. | Count of values tied for lowest frequency. |
| `antimode_occurrences` | Variable | Frequency count of the antimode. | Number of times the antimode appears. |

## Dataset Statistics (File Level)

These statistics appear as additional rows with the prefix `qsv__` when `--dataset-stats` is enabled.

| Identifier | Level | Summary | Computation |
|:---|:---:|:---|:---|
| `qsv__rowcount` | File | Total number of rows (records). | Count of records processed (excluding header). |
| `qsv__columncount` | File | Total number of columns. | Count of fields in the header/first record. |
| `qsv__filesize_bytes` | File | Total file size in bytes. | Filesystem metadata size. |
| `qsv__fingerprint_hash`| File | Cryptographic hash of the dataset's stats. | BLAKE3 hash of the first 26 columns ("streaming" stats) + dataset stats. |
| `qsv__value` | File | The value column for dataset stats. | Holds the value for the `qsv__` row (e.g., the row count integer). |
