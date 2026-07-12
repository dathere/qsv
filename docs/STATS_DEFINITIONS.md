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
  - [Approximate Algorithms (Opt-In)](#approximate-algorithms-opt-in)
- [moarstats](#moarstats)
  - [Count Reference](#count-reference)
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
  - [Performance Characteristics](#performance-characteristics)
  - [When Values Are Blank](#when-values-are-blank)
- [frequency](#frequency)
  - [Frequency Table Output](#frequency-table-output)
  - [Ranking Strategies](#ranking-strategies)
  - [NULL Handling](#null-handling)
  - [Column Filtering](#column-filtering)
  - [Weighted Frequencies](#weighted-frequencies)
  - [Stats Cache Integration](#stats-cache-integration)
  - [JSON/TOON Output](#jsontoon-output)
  - [Frequent Items Sketch (Approximate Top-K)](#frequent-items-sketch-approximate-top-k)
  - [Memory-Aware Processing](#memory-aware-processing)
- [Processing Very Large Files](#processing-very-large-files)
  - [When to Worry](#when-to-worry)
  - [Memory Pressure Points](#memory-pressure-points)
  - [Recipe: stats on Very Large Files](#recipe-stats-on-very-large-files)
  - [Recipe: frequency on Very Large Files](#recipe-frequency-on-very-large-files)
  - [Indexing for Parallelism](#indexing-for-parallelism)
  - [Environment Variable Cheat Sheet](#environment-variable-cheat-sheet)
  - [Worked Example: a Multi-GB CSV](#worked-example-a-multi-gb-csv)
  - [Hard Limits (What Will Still OOM)](#hard-limits-what-will-still-oom)
  - [Platform Note: Big-Endian Targets](#platform-note-big-endian-targets)
  - [Notes for moarstats and pragmastat](#notes-for-moarstats-and-pragmastat)

---

## `stats`
Here are all the statistics produced by the `qsv stats` command, sourced from `src/cmd/stats.rs`.

Each statistic is categorized by its relevant section, with its identifier (column name), summary, computation method, and level (File or Variable).

> **Note**: "Streaming" statistics are computed in constant memory. "Non-Streaming" statistics require loading the column data into memory (or multiple passes) and may use approximation or exact calculation depending on configuration.

**Important:** Unlike the `sniff` command, `stats` data type inferences are **GUARANTEED**, as the entire file is scanned, not just sampled. This makes `stats` a central command in qsv that underpins other "smart" commands (`describegpt`, `frequency`, `joinp`, `pivotp`, `schema`, `sqlp`, `tojsonl`) which use cached statistical information to work smarter & faster.

The command supports various caching options to improve performance on subsequent runs. See `--stats-jsonl` and `--cache-threshold` options for details.

### Streaming vs Non-Streaming Statistics

**Streaming Statistics** (computed in constant memory, always emitted alongside the `field` and `type` identifier columns; those two identifier columns are always present in the output but are *not* counted as statistics here) — **27 stats**:

| #  | Identifier | Group |
|---:|:---|:---|
|  1 | `is_ascii` | Metadata |
|  2 | `sum` | Descriptive |
|  3 | `min` | Descriptive |
|  4 | `max` | Descriptive |
|  5 | `range` | Descriptive |
|  6 | `sort_order` | Descriptive |
|  7 | `sortiness` | Descriptive |
|  8 | `min_length` | String length |
|  9 | `max_length` | String length |
| 10 | `sum_length` | String length |
| 11 | `avg_length` | String length |
| 12 | `stddev_length` | String length |
| 13 | `variance_length` | String length |
| 14 | `cv_length` | String length |
| 15 | `mean` | Central tendency |
| 16 | `sem` | Central tendency |
| 17 | `geometric_mean` | Central tendency |
| 18 | `harmonic_mean` | Central tendency |
| 19 | `stddev` | Central tendency |
| 20 | `variance` | Central tendency |
| 21 | `cv` | Central tendency |
| 22 | `nullcount` | Quality |
| 23 | `n_negative` | Quality |
| 24 | `n_zero` | Quality |
| 25 | `n_positive` | Quality |
| 26 | `max_precision` | Quality |
| 27 | `sparsity` | Quality |

**Non-Streaming Statistics** (opt-in via flag or `--everything`; all but `zero_padded_numeric` load/retain per-column data in memory) — **21 stats**:

> `zero_padded_numeric` is the exception in this group: it is computed during the normal streaming scan in constant memory (two bookkeeping flags per column) and is listed here only because it is opt-in, not because it requires in-memory processing.

| #  | Identifier | Flag |
|---:|:---|:---|
| 28 | `median` | `--median` (suppressed when `--quartiles` is also on; supplied by `q2_median` instead) |
| 29 | `mad` | `--mad` |
| 30 | `lower_outer_fence` | `--quartiles` |
| 31 | `lower_inner_fence` | `--quartiles` |
| 32 | `q1` | `--quartiles` |
| 33 | `q2_median` | `--quartiles` |
| 34 | `q3` | `--quartiles` |
| 35 | `iqr` | `--quartiles` |
| 36 | `upper_inner_fence` | `--quartiles` |
| 37 | `upper_outer_fence` | `--quartiles` |
| 38 | `skewness` | `--quartiles` |
| 39 | `cardinality` | `--cardinality` |
| 40 | `uniqueness_ratio` | `--cardinality` |
| 41 | `mode` | `--mode` |
| 42 | `mode_count` | `--mode` |
| 43 | `mode_occurrences` | `--mode` |
| 44 | `antimode` | `--mode` |
| 45 | `antimode_count` | `--mode` |
| 46 | `antimode_occurrences` | `--mode` |
| 47 | `percentiles` | `--percentiles` (single column containing the comma-separated values listed in `--percentile-list`) |
| 48 | `zero_padded_numeric` | `--zero-padded-numeric` (`true` when leading/padding zeros would be lost if the column were cast to a number: the inferred `type` is `String` — a leading zero is exactly what forces an otherwise-numeric column to infer as `String` — and every non-null value is numeric-shaped: an all-digit integer (zip codes, barcodes, padded IDs), a zero-padded decimal code such as `007.1`/`05.10` (ICD-9, Dewey Decimal, Harmonized System codes), or a plain number mixed in; empty otherwise. Only `String`-typed columns are ever flagged) |

**Total: 48 statistics** (27 streaming + 21 non-streaming, beyond the `field`/`type` identifiers). 48 is the *maximum* — it's the size of the union across all flag combinations. The actual emitted column count for any particular run depends on which flags are set: any single run emits at most 48 stats columns because `median` (#28) and `q2_median` (#33) are mutually exclusive — `median` is only emitted under `--median` alone, while `q2_median` replaces it whenever `--quartiles` or `--everything` is set. Runs without `--everything` emit fewer columns (only the streaming 27 plus whichever opt-in groups are enabled). The non-streaming statistics that retain per-column samples (every stat in this group except `zero_padded_numeric`, which is streaming/constant-memory) use memory-aware chunking for large files, dynamically calculating chunk size based on available memory and record sampling. The enumeration above is the source-of-truth for the "48 summary statistics" count quoted in `README.md` and `docs/help/stats.md`; it is sourced from the `Stats::stat_headers` builder in [`src/cmd/stats.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/stats.rs).

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
- These values are rounded to a precision of 1e-5 days (sub-second), with trailing zeros trimmed in the displayed output
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
| `zero_padded_numeric` | Variable | `true` when leading/padding zeros would be lost if the column were cast to a number: a `String`-typed column whose every non-null value is numeric-shaped — zero-padded integers/codes (zip codes, barcodes, padded IDs) or zero-padded decimal codes (e.g. `007.1`, `05.10`), possibly mixed with plain numbers; empty otherwise. Only `String`-typed columns are ever flagged (a leading zero is exactly what forces an otherwise-numeric column to infer as `String`). Opt-in via `--zero-padded-numeric` or `--everything`. | While scanning, a column stays "qualified" only if every non-null value is numeric-shaped; the first non-numeric value disqualifies it (the disqualification is sticky). A value counts as numeric-shaped if it is (a) an all-ASCII-digit integer; (b) a zero-padded decimal code — a leading `0` in the integer part followed by another digit, with a single decimal point (`007.1`, `05.10`); or (c) a plain number — either freshly inferred as `Float`, or (once the column has already widened to `String`) re-parsed as a float, so a `3.5` arriving after a `007.1` does not disqualify the column. At output, `true` is emitted when the column qualified, saw at least one value, AND its final `type` is `String` — which, for an all-numeric-shaped column, implies at least one value carried a leading zero (a plain all-digit column infers as `Integer`, a plain decimal column as `Float`; both are never flagged). |

**Zero-Padded Numeric Detection:**
- qsv deliberately keeps zero-padded numerics — integers with leading zeros (e.g. `07306`) AND zero-padded decimal codes (e.g. `007.1`) — as `String` rather than `Integer`/`Float`, to avoid silently dropping the leading zeros. The flag is therefore only ever emitted for `String`-typed columns; what it adds over the `String` type alone is the guarantee that every value is numeric-shaped, i.e. the column is a numeric *code*, not free text.
- `zero_padded_numeric` surfaces both so such columns are not mistakenly re-typed/cast as numeric when loaded into SQL, SPSS, SAS, Stata, or other tools.
- Detection is strict: a column with any non-numeric value (e.g. `"N/A"`, `"Main St"`) is **not** flagged, matching the "only if all values are numeric" intent.
- The zero-padded decimal-code rule mirrors the integer one: a `0` in the integer part immediately followed by another digit (`007.1`, `05.10`, `0601.10`). This deliberately **excludes** ordinary fractions like `0.5`/`0.25` (a single `0` before the decimal point) — those infer as `Float` and are never flagged. Pure trailing-zero codes (`7.10` → `7.1`) are likewise **not** flagged: with no leading zero the column infers as `Float`, and trailing padding is indistinguishable from a rounded measurement without the original string anyway. Multi-dot codes like `0601.10.00` are neither all-digit nor parseable floats, so they disqualify the column (they count as non-numeric values) and it is never flagged — even though qsv does keep such columns as `String`.

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
| `sort_order` | Variable | Sorting status of the column. | Checked during scan. Returns "Ascending", "Descending", or "Unsorted". |
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
| `cv_length` | Variable | Coefficient of Variation of lengths. | `stddev_length / avg_length` (unitless ratio, **not** multiplied by 100 unlike the numeric `cv` above). Shows `*OVERFLOW*` when `sum_length` overflowed. |

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

**Note on Date/DateTime types:** For Date and DateTime types, range, stddev, variance, MAD, and IQR are returned in days (not milliseconds). These values are rounded to a precision of 1e-5 days (sub-second precision); trailing zeros may be omitted in the output.

**Requirements:**
- `median` requires `--median` or `--everything` (unless `--quartiles` is specified, in which case `median` is not returned separately as it's the same as `q2_median`)
- `mad` requires `--mad` or `--everything`
- Quartile statistics require `--quartiles` or `--everything`
- When `--quantile-method approx` is set, `median`, `q1`, `q2_median`, `q3`, `iqr`, the four fences, `skewness`, and `percentiles` are computed from a t-digest sketch (Apache DataSketches port of Dunning's MergingDigest, ~200 centroids, ~1% rank error — more accurate at the tails). See [Approximate Algorithms (Opt-In)](#approximate-algorithms-opt-in). Under that mode, `--mad` is auto-disabled with a warning, `--weight` is rejected, and results may differ slightly across runs with different `--jobs` values (pin `--jobs 1` for run-to-run determinism).

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
- By default, `cardinality` is computed exactly (via the same HashMap that backs mode tracking). Pass `--cardinality-method approx` to swap in a HyperLogLog sketch (Apache DataSketches port, lg_k=12, ~5KB/column, ~1.5% relative standard error) — useful on very-high-cardinality columns where exact counting is wasted work. See [Approximate Algorithms (Opt-In)](#approximate-algorithms-opt-in). `--infer-boolean` forces exact (boolean inference needs `cardinality == 2` exactness); a one-time warning is emitted.
- The `--mode-cardinality-cap <n>` option (default `0` = unbounded) bounds the per-column memory used to track modes/antimodes on high-cardinality columns. When the cap fires, `mode`/`antimode` columns emit the sentinel `*HIGH_CARDINALITY`, and (under `--cardinality-method exact` only) the `cardinality` column emits `>=<n>`. The `>=` prefix DOES break downstream parsers expecting a plain integer, so the cap is opt-in. Under `--cardinality-method approx`, the cap does **not** affect the `cardinality` column (HLL emits its own ~1.5%-RSE estimate at fixed memory); only mode/antimode tracking is gated. The cap measures total samples added under unweighted mode (~ row count) and number of unique values under `--weight` (HashMap `len()`, == true cardinality).

When `--weight <column>` is specified, weighted versions are computed. For weighted modes, `mode_occurrences` is the maximum weight (rounded). For weighted antimodes, `antimode_occurrences` is the minimum weight (rounded).

Multiple modes/antimodes are separated by the `QSV_STATS_SEPARATOR` environment variable (default: `|`).

| Identifier | Level | Summary | Computation |
|:---|:---:|:---|:---|
| `cardinality` | Variable | Count of unique values. | Count of distinct entries in the column. Weighted: count of unique values (weights are not considered for uniqueness). Use `--cardinality-method approx` for a HyperLogLog estimate (~1.5% RSE, fixed ~5KB memory) — see [Approximate Algorithms (Opt-In)](#approximate-algorithms-opt-in). When `--mode-cardinality-cap <n>` fires under exact mode, this column emits the sentinel `>=<n>`. |
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
| `percentiles` | Variable | Custom percentiles of sorted values. | Nearest rank method for user-defined list. Weighted: weighted nearest-rank method. Multiple percentiles separated by `QSV_STATS_SEPARATOR` (default: `\|`). Special values: "deciles" → "10,20,30,40,50,60,70,80,90", "quintiles" → "20,40,60,80". Default: "5,10,40,60,90,95". For dates/datetimes, values in RFC3339 format. |

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
| `hash.blake3` | BLAKE3 fingerprint hash of the dataset's stats. | BLAKE3 hash of the cached stats record's streaming-stats portion up to the `FINGERPRINT_HASH_COLUMNS` limit (29 columns in the default/non-`--typesonly` output; effectively `min(FINGERPRINT_HASH_COLUMNS, record.len())` columns in reduced-column modes such as `--typesonly`), plus dataset metadata (`record_count`, `field_count`, `filesize_bytes`). The limit is controlled by the `FINGERPRINT_HASH_COLUMNS` constant in `src/cmd/stats.rs`, which is kept in sync with the streaming-column count in `stats_headers()`. This allows users to quickly detect duplicate files without having to load the entire file to compute the hash. Especially useful for detecting duplicates of very large files with pre-existing stats cache metadata. |

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

### Approximate Algorithms (Opt-In)

By default, `stats` produces **exact, deterministic** results. Three opt-in flags swap exact accumulators for [Apache DataSketches](https://datasketches.apache.org/) ports — Rust ports of [streaming sketches](https://datasketches.apache.org/docs/Community/Research.html) — that trade a small, bounded error for **constant (or near-constant) memory** and faster compute on very-large columns.

| Flag | Default | Sketch | Memory | Error | Restrictions / Notes |
|:---|:---|:---|:---|:---|:---|
| `--quantile-method approx` | `exact` | t-digest (Apache DataSketches port of Dunning's [MergingDigest](https://arxiv.org/abs/1902.04023), ~200 centroids) | O(K) per numeric column | ~1% rank error (more accurate at the tails) | Replaces the sort-based `median`/`q1`/`q2_median`/`q3`/`iqr`/fences/`skewness`/`percentiles` pipeline. `--mad` is auto-disabled with a warning (MAD requires a second pass that t-digest does not support). `--weight` is rejected (the upstream `datasketches` crate does not expose a weighted-update API). Results may differ ~1% across runs with different `--jobs` values (`TDigestMut::merge` is associative but not chunk-count-invariant); pin `--jobs 1` for run-to-run determinism. |
| `--cardinality-method approx` | `exact` | [HyperLogLog](https://en.wikipedia.org/wiki/HyperLogLog) (Apache DataSketches port, lg_k=12) | ~5KB per column | ~1.5% relative standard error | Replaces exact `cardinality`/`uniqueness_ratio`. Reproducible across `--jobs` values (the HLL union used at merge time is associative and order-invariant, so chunk completion order does not affect the final estimate). `--infer-boolean` forces `exact` (boolean inference needs `cardinality == 2` exactness); a one-time warning is emitted. The `--mode-cardinality-cap` `>=<n>` sentinel is **never** emitted under `approx` — only mode/antimode columns remain gated by the cap. |
| `--mode-cardinality-cap <n>` | `0` (unbounded) | bounds the mode/antimode tracker | per-column cap on tracker entries | exact when ≤ cap; sentinels otherwise | When the tracker grows past `<n>`, qsv drops it and emits `*HIGH_CARDINALITY` for `mode`/`antimode` columns. Under `--cardinality-method exact`, the `cardinality` column emits `>=<n>` (the `>=` prefix breaks downstream integer parsers — that's why the cap is opt-in). Under `--cardinality-method approx`, the cap does not affect the cardinality column (HLL emits its estimate at fixed memory). The cap measures total samples added under unweighted mode (~ row count) and number of unique values under `--weight` (HashMap `len()`, == true cardinality). |

**Output validation:** `stats` uses [simdutf8](https://crates.io/crates/simdutf8) for SIMD-accelerated UTF-8 validation on the output path — a perf detail with no behavioral change.

**OOM auto-fallback:** Whenever `stats` takes the non-parallel path with non-streaming columns, it runs an in-memory load check via `util::mem_file_check`. By default the check is **NORMAL mode** (file size vs. total memory − headroom). Passing `--memcheck` (or setting the `QSV_MEMORY_CHECK` env var) switches to **CONSERVATIVE mode** (file size vs. available + free_swap × platform_factor − headroom), which is stricter and trips OOM far more readily. If the check fails in either mode, `stats` layers two fallbacks before propagating the OOM error:

1. **Auto-create an index** (when no index exists and input is not stdin) to switch to parallel/indexed processing.
2. **Auto-enable approx DataSketches estimators** — flips `--quantile-method` and `--cardinality-method` from `exact` to `approx` where the explicit-validation guards would have accepted them. Specifically:
   - `--quantile-method` auto-enables unless `--weight` is set; if `--mad` or `--everything` is also set, MAD is auto-disabled (mirroring the existing `--quantile-method approx` guard).
   - `--cardinality-method` auto-enables unless `--infer-boolean` is set.

A `wwarn!` is emitted listing each auto-enabled estimator. The original OOM error is only propagated when **neither** fallback engages. The sketch fallback can fire even when an index is already present and the OOM check still trips (e.g., with `--jobs 1` on a pre-indexed file) — that is a behavior change from the previous "error out" path in this narrow case. Users can disable the auto-enable by passing `--quantile-method exact` or `--cardinality-method exact` explicitly; the OOM arm scans `argv` for these flag names (since docopt fills in the default `exact` value either way) and skips the auto-enable when either flag was explicitly provided.

**See also:** [t-digest paper (Dunning, 2019)](https://arxiv.org/abs/1902.04023), [HyperLogLog (Flajolet et al., 2007)](https://en.wikipedia.org/wiki/HyperLogLog), [Apache DataSketches](https://datasketches.apache.org/).

## `moarstats`
Here are all the additional statistics produced by the `qsv moarstats` command, sourced from `src/cmd/moarstats.rs`.


The `moarstats` command extends an existing stats CSV file (created by the `stats` command) by computing additional statistics that can be derived from existing stats columns and/or by scanning the original CSV file.

**How it works:**
- Looks for `<FILESTEM>.stats.csv` for a given CSV input
- If the stats CSV file doesn't exist, it will first run the `stats` command with configurable options (via `--stats-options`, default: `--infer-dates --infer-boolean --cardinality --mode --mad --quartiles --percentiles --force --stats-jsonl`) to establish baseline stats
- If the `.stats.csv` file is found, it skips running stats and just appends the additional stats columns
- Statistics are rounded using Bankers Rounding (Midpoint Nearest Even) to the specified number of decimal places (default: 4, configurable with `--round`)
- Uses parallel processing when an index is available for large files

**Requirements:**
- All statistics are computed only for numeric and date/datetime columns (except Shannon Entropy which works for all field types)
- Derived statistics require specific base statistics to be present in the stats CSV
- Advanced statistics require `--advanced` flag and reading the entire CSV file
- Outlier statistics require quartiles (and thus fences) to be computed in the baseline stats
- Winsorized/trimmed means require either Q1/Q3 or percentiles to be available

### Count Reference

`moarstats` documentation cites "up to an additional 55 statistical measures." That figure is
the union of the three groups below; each is enumerated explicitly in this document so the
total can be audited against the source-of-truth in [`src/cmd/moarstats.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/moarstats.rs).

**Counting convention.** Each conceptual statistical *measure* counts once even when it
emits multiple companion columns; the count is therefore over distinct concepts, not over
output column names. Three grouping rules are applied below:

- **Jarque-Bera** (#17) counts as one measure even though it emits two columns
  (`jarque_bera` plus its `jarque_bera_pvalue`); the p-value is a derived companion of the
  test statistic, not an independent measure.
- **Winsorized Mean + Trimmed Mean** (#25) count together as a single robust-mean *pair*
  (one measure entry) even though they emit 12 columns combined (each mean plus 5
  companion stddev/variance/cv/range/stddev_ratio columns). They share a single robust-mean
  pipeline driven by `--use-percentiles`/`--pct-thresholds` and are conceptually the same
  measure under two different boundary policies.
- **Covariance** (bivariate #4) counts as one measure even though it emits two columns
  (`covariance_sample` and `covariance_population`); they differ only in the divisor.

A reader regenerating the count by tallying named output columns in `src/cmd/moarstats.rs`
will arrive at a higher number; arrive at 55 by collapsing the three groups above.

**Univariate measures (25)** — see [Derived Statistics](#derived-statistics), [Advanced Statistics](#advanced-statistics) and [Robust Statistics (Winsorized & Trimmed Means)](#robust-statistics-winsorized--trimmed-means):

|  # | Measure | Section / Flag |
|---:|:---|:---|
|  1 | Pearson's Second Skewness Coefficient (`pearson_skewness`) | Derived |
|  2 | Range to StdDev Ratio (`range_stddev_ratio`) | Derived |
|  3 | Quartile Coefficient of Dispersion (`quartile_coefficient_dispersion`) | Derived |
|  4 | Z-Score of Mode (`mode_zscore`) | Derived |
|  5 | Relative Standard Error (`relative_standard_error`) | Derived |
|  6 | Z-Score of Min (`min_zscore`) | Derived |
|  7 | Z-Score of Max (`max_zscore`) | Derived |
|  8 | Median-to-Mean Ratio (`median_mean_ratio`) | Derived |
|  9 | IQR-to-Range Ratio (`iqr_range_ratio`) | Derived |
| 10 | MAD-to-StdDev Ratio (`mad_stddev_ratio`) | Derived |
| 11 | Trimean (`trimean`) | Derived |
| 12 | Midhinge (`midhinge`) | Derived |
| 13 | Robust CV (`robust_cv`) | Derived |
| 14 | XSD type (`xsd_type`) | Derived |
| 15 | Kurtosis (`kurtosis`) | `--advanced` |
| 16 | Bimodality Coefficient (`bimodality_coefficient`) | `--advanced` |
| 17 | Jarque-Bera test (`jarque_bera` + `jarque_bera_pvalue`) | `--advanced` (emits 2 columns) |
| 18 | Gini Coefficient (`gini_coefficient`) | `--advanced` |
| 19 | Atkinson Index (`atkinson_index_(<ε>)`, e.g. `atkinson_index_(1)` with the default `--epsilon 1.0`) | `--advanced --epsilon` |
| 20 | Theil Index (`theil_index`) | `--advanced` |
| 21 | Mean Absolute Deviation from mean (`mean_ad`) | `--advanced` |
| 22 | Shannon Entropy (`shannon_entropy`) | `--advanced` |
| 23 | Normalized Entropy (`normalized_entropy`) | `--advanced` (when `cardinality` is present) |
| 24 | Simpson's Diversity Index (`simpsons_diversity_index`) | `--advanced` |
| 25 | Winsorized Mean (`winsorized_mean` + 5 companion columns) and Trimmed Mean (`trimmed_mean` + 5 companion columns) | Robust (counted as one measure pair, emits 12 columns) |

**Outlier measures (24)** — see [Outlier Statistics](#outlier-statistics):

| # | Group | Identifiers |
|---:|:---|:---|
| 1–7 | Outlier counts | `outliers_extreme_lower_cnt`, `outliers_mild_lower_cnt`, `outliers_normal_cnt`, `outliers_mild_upper_cnt`, `outliers_extreme_upper_cnt`, `outliers_total_cnt`, `outliers_percentage` |
| 8–13 | Outlier descriptive | `outliers_mean`, `non_outliers_mean`, `outliers_to_normal_mean_ratio`, `outliers_min`, `outliers_max`, `outliers_range` |
| 14–20 | Outlier variance / spread | `outliers_stddev`, `outliers_variance`, `non_outliers_stddev`, `non_outliers_variance`, `outliers_cv`, `non_outliers_cv`, `outliers_normal_stddev_ratio` |
| 21–22 | Outlier impact | `outlier_impact`, `outlier_impact_ratio` |
| 23–24 | Outlier boundary | `lower_outer_fence_zscore`, `upper_outer_fence_zscore` |

**Bivariate measures (6, written to `<FILESTEM>.stats.bivariate.csv` under `--bivariate`)** — see [Bivariate Statistics](#bivariate-statistics):

| # | Measure |
|---:|:---|
| 1 | Pearson's correlation (`pearson_correlation`) |
| 2 | Spearman's rank correlation (`spearman_correlation`) |
| 3 | Kendall's tau (`kendall_tau`) |
| 4 | Covariance (`covariance_sample` + `covariance_population` — counted as one measure, emits 2 columns) |
| 5 | Mutual Information (`mutual_information`) |
| 6 | Normalized Mutual Information (`normalized_mutual_information`) |

**Total: 25 + 24 + 6 = 55 statistical measures.** Note that several measures expand into more than one output column (e.g. Jarque-Bera → 2 columns, Winsorized/Trimmed Means → 12 columns combined, Covariance → 2 columns), so the actual column count in a `<FILESTEM>.stats.csv` extended by `moarstats --advanced` plus its bivariate sidecar is higher than 55.

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
| <a id="trimean"></a>`trimean` | Variable | Tukey's Trimean. Robust estimator of central tendency combining median with the midhinge. More robust than mean, more efficient than median alone. | `(Q1 + 2*median + Q3) / 4`. Requires: `q1`, `median` (or `q2_median`), `q3`. See: [Trimean](https://en.wikipedia.org/wiki/Trimean) |
| <a id="midhinge"></a>`midhinge` | Variable | Midhinge. Midpoint of the middle 50% of data. A robust central tendency measure that complements the mean and median. | `(Q1 + Q3) / 2`. Requires: `q1`, `q3`. See: [Midhinge](https://en.wikipedia.org/wiki/Midhinge) |
| <a id="robust_cv"></a>`robust_cv` | Variable | Robust Coefficient of Variation. Non-negative, outlier-resistant alternative to CV using MAD and the magnitude of the median instead of stddev and mean. | `MAD / abs(median)`. Requires: `mad`, `median` (or `q2_median`). Returns `None` if median is zero. See: [Robust measures of scale](https://en.wikipedia.org/wiki/Robust_measures_of_scale) |
| `xsd_type` | Variable | Inferred W3C XML Schema datatype. Infers the most specific XSD type based on field type and min/max values. Works for **all field types**. | Computed from `type`, `min`, and `max` columns. For Integer types, refines to most specific type (e.g., `byte`, `short`, `int`, `long`, `unsignedByte`, `unsignedShort`, `unsignedInt`, `unsignedLong`, `positiveInteger`, `nonNegativeInteger`, `negativeInteger`, `nonPositiveInteger`, or `integer`) based on min/max ranges. Also detects Gregorian date types (`gYear`, `gYearMonth`, `gMonthDay`, `gDay`, `gMonth`) with confidence markers (`?` = more confident from thorough scan, `??` = less confident from quick scan). For other types: Float → `decimal`, String → `string`, Date → `date`, DateTime → `dateTime`, Boolean → `boolean`, NULL → empty string. If min/max are not available for Integer types, defaults to `integer`. See: [XML Schema Part 2: Datatypes](https://www.w3.org/TR/xmlschema-2/) |

### Advanced Statistics

These statistics require the `--advanced` flag and reading the entire CSV file to collect all values for computation. They are computationally expensive.

| Identifier | Level | Summary | Computation |
|:---|:---:|:---|:---|
| `kurtosis` | Variable | Excess kurtosis. Measures the "tailedness" of the distribution. Positive values indicate heavy tails, negative values indicate light tails. Values near 0 indicate a normal distribution. | Computed from all values in the column. Uses precalculated mean and variance from baseline stats for efficiency. Requires: `mean`, `variance` (or `stddev`). See: [Kurtosis](https://en.wikipedia.org/wiki/Kurtosis) |
| `bimodality_coefficient` | Variable | Bimodality Coefficient. Measures whether a distribution has two modes (peaks) or is unimodal. BC < 0.555 indicates unimodal, BC >= 0.555 indicates bimodal/multimodal. | Computed as `(skewness² + 1) / (kurtosis + 3)`. Requires: `skewness` (from base stats) and `kurtosis` (from `--advanced` flag). See: [Bimodality](https://en.wikipedia.org/wiki/Bimodality) |
| <a id="jarque_bera"></a>`jarque_bera` | Variable | Jarque-Bera test statistic. Standard test for normality using skewness and kurtosis. Higher values indicate greater departure from normality. | Computed as `(n/6) * (S² + K²/4)` where S is skewness and K is excess kurtosis. Requires: `skewness` (from base stats), `kurtosis` (from `--advanced` flag), and sample size n (from `n_positive + n_negative + n_zero`). See: [Jarque-Bera test](https://en.wikipedia.org/wiki/Jarque%E2%80%93Bera_test) |
| `jarque_bera_pvalue` | Variable | P-value for the Jarque-Bera test. Low values (< 0.05) indicate the data is NOT normally distributed. | Computed from the chi-squared distribution with 2 degrees of freedom: `p = e^(-JB/2)`. Requires: `jarque_bera`. |
| `gini_coefficient` | Variable | Gini Coefficient. Measures inequality/dispersion in the distribution. Values range from 0 (perfect equality) to 1 (maximum inequality). | Computed from all values in the column. Uses precalculated sum from baseline stats for efficiency. Requires: `sum`. See: [Gini Coefficient](https://en.wikipedia.org/wiki/Gini_coefficient) |
| `atkinson_index_(<ε>)` | Variable | Atkinson Index. Measures inequality in the distribution with a sensitivity parameter. The column name interpolates the epsilon value (e.g. `atkinson_index_(1)` with the default `--epsilon 1.0`). Values range from 0 (perfect equality) to 1 (maximum inequality). The Atkinson Index is a more general form of the Gini coefficient that allows for different sensitivity to inequality. | Computed from all values in the column. Uses precalculated mean from baseline stats for efficiency. The epsilon (ε) parameter controls sensitivity to inequality (configurable via `--epsilon`, default: 1.0). Higher epsilon values indicate greater sensitivity to inequality. Requires: `mean`. See: [Atkinson Index](https://en.wikipedia.org/wiki/Atkinson_index) |
| <a id="theil_index"></a>`theil_index` | Variable | Theil Index (Generalized Entropy GE(1)). Measures inequality/concentration in the distribution. Unlike Gini, it is decomposable into within-group and between-group components. Only computed for positive values. | Computed as `(1/n) * Σ((x_i / mean) * ln(x_i / mean))` for positive values. Computes mean from positive values only (not the overall precalculated mean). Requires positive values in the column. See: [Theil Index](https://en.wikipedia.org/wiki/Theil_index) |
| <a id="mean_ad"></a>`mean_ad` | Variable | Mean Absolute Deviation from mean. Average absolute distance of values from the arithmetic mean. Less robust than MAD (which uses median) but more statistically efficient. | Computed as `(1/n) * Σ|x_i - mean|`. Uses precalculated mean from baseline stats. Requires: `mean`. |
| `shannon_entropy` | Variable | Shannon Entropy. Measures the information content/uncertainty in the distribution. Higher values indicate more diversity, lower values indicate more concentration. Values range from 0 (all values identical) to log2(n) where n is the number of unique values. | Computed using the `frequency` command with `--limit 0` to collect all frequencies, then calculates: `H(X) = -Σ p_i * log2(p_i)` where p_i is the probability of value i. Works for **all field types** (not just numeric). For all-unique fields, returns log2(n). See: [Entropy (Information Theory)](https://en.wikipedia.org/wiki/Entropy_(information_theory)) |
| `normalized_entropy` | Variable | Normalized Entropy. Normalized version of Shannon Entropy scaled to [0, 1]. Values range from 0 (all values identical) to 1 (all values equally distributed). | Computed as `shannon_entropy / log2(cardinality)`. Requires: `shannon_entropy` (from `--advanced` flag) and `cardinality` (from base stats). If cardinality is 0 or 1, returns 0. |
| <a id="simpsons_diversity_index"></a>`simpsons_diversity_index` | Variable | Simpson's Diversity Index. Probability that two randomly chosen values are different. More intuitive than entropy for many users. Ranges from 0 (all identical) to 1 (all unique). | Computed as `1 - Σ(p_i²)` where p_i are value proportions from frequency data. Computed alongside Shannon Entropy. Works for **all field types**. For all-unique fields, returns `1 - 1/n`. See: [Simpson's Diversity Index](https://en.wikipedia.org/wiki/Diversity_index#Simpson_index) |

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
- `--bivariate-stats`: Select specific statistics (pearson, spearman, kendall, covariance, mi, nmi) or use "all" or "fast" (pearson + covariance). Default: `fast`.
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

The `pragmastat` command computes robust, median-of-pairwise statistics using the [Pragmastat library](https://pragmastat.dev/) (v13.0.1). Designed for messy, heavy-tailed, or outlier-prone data where mean/stddev can mislead.

Sourced from `src/cmd/pragmastat.rs`.

**Key Features:**
- Only finite numeric values are used; non-numeric/NaN/Inf values are ignored
- Date/DateTime columns are supported when a stats cache is available (run `qsv stats -E --infer-dates --stats-jsonl` first); dates are converted to epoch milliseconds for analysis, then `center`/bounds are formatted as dates and `spread`/`shift` as days
- Each column is treated as its own sample (two-sample compares columns, not rows)
- Non-numeric columns appear with n=0 and empty estimator cells
- Loads all numeric values into memory

### Modes

`pragmastat` has four mutually exclusive output modes. The **default** (no mode flag) **extends the existing stats cache** the way `moarstats` does; the other modes always produce a standalone CSV.

| Mode flag | Behavior | Output |
|:---|:---|:---|
| *(none)* | **Default.** Appends 7 `ps_*` columns to the existing `.stats.csv` cache file. If no cache exists, runs `stats` first using `--stats-options`. | Extended stats CSV |
| `--standalone` | One-sample point/bound estimates as a fresh CSV, without touching the stats cache. | Standalone CSV |
| `-t` / `--twosample` | Two-sample estimators for every unordered column pair. | Standalone CSV |
| `--compare1 <spec>` | One-sample confirmatory analysis — tests `center` / `spread` against user-defined thresholds. | Standalone CSV |
| `--compare2 <spec>` | Two-sample confirmatory analysis — tests `shift` / `ratio` / `disparity` against user-defined thresholds. | Standalone CSV |

### Default Mode (Stats Cache Append)

Adds 7 `ps_*` columns to each row of the existing stats CSV (the same row-per-column layout `stats` and `moarstats` use). If no stats cache is present, one is generated first using `--stats-options` (default: `--infer-dates --infer-boolean --mad --quartiles --force --stats-jsonl` — note: **no `--percentiles`**, unlike `moarstats`).

`ps_*` columns that already exist in the cache are left untouched unless `--force` is set.

| Identifier | Level | Summary | Computation |
|:---|:---:|:---|:---|
| `ps_n` | Variable | Count of values used by pragmastat estimators. | Count after filtering: finite numerics for numeric columns, or parsed epoch-ms values for Date/DateTime columns (when supported via the stats cache). Non-numeric / NaN / Inf / unparsable values are excluded. |
| `ps_center` | Variable | Hodges-Lehmann estimator — robust location. | Median of pairwise averages. Tolerates up to 29% corrupted data. |
| `ps_spread` | Variable | Shamos estimator — robust dispersion. | Median of pairwise absolute differences. Same units as data; also tolerates up to 29% corrupted data. |
| `ps_center_lower` | Variable | Lower confidence bound for `ps_center`. | Exact under weak symmetry, with error rate = misrate. |
| `ps_center_upper` | Variable | Upper confidence bound for `ps_center`. | Exact under weak symmetry, with error rate = misrate. |
| `ps_spread_lower` | Variable | Lower confidence bound for `ps_spread`. | Randomized (bootstrap); error rate = misrate. |
| `ps_spread_upper` | Variable | Upper confidence bound for `ps_spread`. | Randomized (bootstrap); error rate = misrate. |

### Standalone Mode (`--standalone`)

One-sample mode that produces a fresh standalone CSV instead of extending the stats cache. (This is the legacy default behavior preserved behind a flag.)

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

### Two-Sample Mode (`-t` / `--twosample`)

Computes statistics for all unordered column pairs. Always produces a standalone CSV.

Output columns: `field_x, field_y, n_x, n_y, shift, ratio, disparity, shift_lower, shift_upper, ratio_lower, ratio_upper, disparity_lower, disparity_upper`

| Identifier | Level | Summary | Computation |
|:---|:---:|:---|:---|
| `field_x`, `field_y` | Pairwise | Column names being compared. | From CSV header. |
| `n_x`, `n_y` | Pairwise | Counts of finite numeric values. | Per-column counts after filtering non-numeric/NaN/Inf. |
| `shift` | Pairwise | Hodges-Lehmann difference — robust location difference. | Median of pairwise differences between columns. Negative means first column tends to be lower. |
| `ratio` | Pairwise | Robust multiplicative ratio. | `exp(shift(log x, log y))`. Use for positive-valued quantities (latency, price, concentration). Requires all values > 0. Suppressed for Date/DateTime pairs (depends on the arbitrary 1970 epoch origin). |
| `disparity` | Pairwise | Robust effect size. | `shift / (average spread of x and y)`. |
| `shift_lower`, `shift_upper` | Pairwise | Confidence bounds for shift. | Exact; error rate = misrate. If bounds exclude 0, the difference is reliable. Ties may be conservative. |
| `ratio_lower`, `ratio_upper` | Pairwise | Confidence bounds for ratio. | Exact; error rate = misrate. If bounds exclude 1, the difference is reliable. Requires all values > 0. |
| `disparity_lower`, `disparity_upper` | Pairwise | Confidence bounds for disparity. | Randomized (Bonferroni combination); error rate = misrate. If bounds exclude 0, the disparity is reliable. |

### Compare1 Mode (`--compare1 <spec>`)

One-sample confirmatory analysis. Tests one-sample estimates (`center` / `spread`) against user-supplied thresholds and renders a verdict per (column, threshold) pair. Always produces a standalone CSV.

**Threshold format:** comma-separated `metric:value` pairs, e.g. `center:42.0` or `center:42.0,spread:0.5`. Valid metrics: `center`, `spread`.

Output columns: `field, n, metric, threshold, estimate, lower, upper, verdict`

| Identifier | Level | Summary |
|:---|:---:|:---|
| `field` | Variable | Column name (or 1-based index if `--no-headers`). |
| `n` | Variable | Count of finite numeric values. |
| `metric` | Variable | The metric being tested (`center` or `spread`). |
| `threshold` | Variable | The user-supplied threshold from `--compare1 metric:value`. |
| `estimate` | Variable | Point estimate of the chosen metric for this column. |
| `lower`, `upper` | Variable | Confidence bounds for the estimate (error rate = misrate). |
| `verdict` | Variable | One of `less` (estimate statistically below threshold), `greater` (statistically above), or `inconclusive` (interval contains threshold). |

Incompatible with `--no-bounds` (the verdict requires bounds).

### Compare2 Mode (`--compare2 <spec>`)

Two-sample confirmatory analysis. Tests two-sample estimates (`shift` / `ratio` / `disparity`) against user-supplied thresholds and renders a verdict per (column pair, threshold). Always produces a standalone CSV.

**Threshold format:** comma-separated `metric:value` pairs, e.g. `shift:0` or `shift:0,disparity:0.8`. Valid metrics: `shift`, `ratio`, `disparity`.

Output columns: `field_x, field_y, n_x, n_y, metric, threshold, estimate, lower, upper, verdict`

| Identifier | Level | Summary |
|:---|:---:|:---|
| `field_x`, `field_y` | Pairwise | Column names being compared. |
| `n_x`, `n_y` | Pairwise | Per-column counts of finite numeric values. |
| `metric` | Pairwise | The metric being tested (`shift`, `ratio`, or `disparity`). |
| `threshold` | Pairwise | The user-supplied threshold from `--compare2 metric:value`. |
| `estimate` | Pairwise | Point estimate of the chosen metric for this column pair. |
| `lower`, `upper` | Pairwise | Confidence bounds for the estimate (error rate = misrate). |
| `verdict` | Pairwise | One of `less`, `greater`, or `inconclusive` (same semantics as compare1). |

`ratio` rows are suppressed for Date/DateTime pairs (see Two-Sample Mode above). Incompatible with `--no-bounds`.

### Options

| Option | Default | Description |
|:---|:---|:---|
| `--twosample` / `-t` | off | Compute two-sample estimators for all column pairs. Mutually exclusive with `--compare1` / `--compare2`. |
| `--compare1 <spec>` | — | One-sample confirmatory analysis. Format: `metric:value[,metric:value,...]`. Valid metrics: `center`, `spread`. Mutually exclusive with `--twosample` / `--compare2`. |
| `--compare2 <spec>` | — | Two-sample confirmatory analysis. Format: `metric:value[,metric:value,...]`. Valid metrics: `shift`, `ratio`, `disparity`. Mutually exclusive with `--twosample` / `--compare1`. |
| `--select <cols>` / `-s` | all numeric columns (when stats cache fresh) | Column selection using qsv's column-selection syntax. Non-numeric columns appear with n=0. In two-sample mode, all pairs of selected columns are computed. |
| `--misrate <n>` / `-m` | `0.001` | Probability that bounds fail to contain the true parameter. Lower values produce wider bounds. Must be achievable for the given sample size. Use `1e-3` for everyday analysis or `1e-6` for critical decisions. |
| `--standalone` | off | Force one-sample mode to emit a standalone CSV instead of extending the stats cache. No effect with `--twosample` / `--compare1` / `--compare2` (which are always standalone). |
| `--stats-options <arg>` | `--infer-dates --infer-boolean --mad --quartiles --force --stats-jsonl` | Options passed to the `stats` command when baseline stats need to be generated. Note: this default differs from `moarstats` by omitting `--percentiles`. |
| `--round <n>` | `4` | Round statistics to `<n>` decimal places. Uses Midpoint Nearest Even (Bankers Rounding). |
| `--force` | off | Force recomputing `ps_*` columns in the stats cache even if they already exist. |
| `--subsample <N>` | off | Partial Fisher-Yates shuffle keeping only N values per column before computing. ~100× speedup on large datasets while preserving statistical robustness. Recommended: 10,000–50,000 for exploratory analysis. Incompatible with the default cache-append mode (approximate results would be silently reused as if computed from the full dataset) — must be combined with `--standalone` or one of the other non-cache modes. |
| `--seed <N>` | `42` (when `--subsample` is set) | Seed for reproducible subsampling. |
| `--no-bounds` | off | Skip confidence bound computation (~2× speedup) when only point estimates are needed. Incompatible with `--compare1` / `--compare2`, and with the default cache-append mode (the cache would store empty bounds that a subsequent run would silently reuse) — must be combined with `--standalone`. |
| `--output <file>` / `-o` | stdout | Write output to file instead of stdout. |
| `--delimiter <c>` / `-d` | `,` | Field delimiter for reading/writing CSV data. |
| `--no-headers` / `-n` | off | When set, the first row will not be treated as headers. |
| `--jobs <arg>` / `-j` | number of CPUs | The number of jobs to run in parallel. When not set, defaults to the number of CPUs detected. |
| `--memcheck` | off | Check if there is enough memory to load the entire CSV into memory using CONSERVATIVE heuristics. Not valid for stdin. |

### Performance Characteristics

**Algorithmic Complexity**

All Pragmastat estimators avoid naïve O(n²) pairwise enumeration by using implicit‑matrix selection and binary‑search techniques (see [pragmastat.dev/synopsis](https://pragmastat.dev/synopsis/)):

| Estimator(s) | Complexity | Technique |
|:---|:---|:---|
| `center`, `center_bounds` | O(n log n) | Monahan's implicit‑matrix selection + SignedRankMargin |
| `spread`, `spread_bounds` | O(n log n) | Monahan's selection for differences + disjoint‑pair sign‑test inversion |
| `shift`, `shift_bounds`, `ratio`, `ratio_bounds` | O((n+m) log L) | Value‑space binary search over pairwise differences; L = value range |
| `disparity`, `disparity_bounds` | O((n+m) log L + n log n + m log m) | Bonferroni split combining shift bounds + average spread bounds |

These are per‑column (one‑sample) or per‑pair (two‑sample) complexities. Randomization primitives (xoshiro256++) are O(1) per draw.

**qsv Implementation Optimizations**

- **Parallel computation** — Columns (one‑sample) and column pairs (two‑sample) are processed in parallel via Rayon; controlled by `--jobs`.
- **Parallel indexed CSV reading** — Files with ≥ 10,000 rows and a `.csv.idx` index are read in parallel chunks using a ThreadPool with crossbeam channels and deterministic seeking.
- **`--subsample N`** — Partial Fisher‑Yates shuffle keeps only N values per column before computing, with deterministic per‑column seeding (`--seed` defaults to 42). Provides ~100× speedup on large datasets while preserving statistical robustness. Recommended: 10,000–50,000 for exploratory analysis.
- **`--no-bounds`** — Skips confidence bound computation for ~2× speedup when only point estimates are needed.
- **Combined `--subsample` + `--no-bounds`** — ~200× speedup for quick exploratory analysis on large datasets.
- **Stats cache integration** — When a fresh `.stats.csv.data.jsonl` cache exists, non‑numeric columns are automatically filtered out before computation, and Date/DateTime type detection is read from the cache rather than re‑inferred.
- **Pre‑computed log arrays** — In two‑sample mode, `ln()` transformations are computed once per column in parallel and shared across all pairs, avoiding redundant O(n) passes.
- **SIMD‑accelerated parsing** — Uses `simdutf8` for UTF‑8 validation, `fast_float2` for float parsing, and `simd_json` (little‑endian) for cache deserialization.
- **Pre‑allocated buffers** — Column vectors are sized to estimated row counts, and chunk buffers are pre‑allocated before parallel reads to minimize reallocations.

### When Values Are Blank

Cells are empty (blank) when:
- **No numeric data (n=0):** The column contains no finite numeric values
- **Positivity required:** `ratio`, `ratio_lower`, and `ratio_upper` require all values > 0
- **Date/DateTime pairs:** `ratio` is suppressed for `--twosample` and `--compare2` because it depends on the arbitrary 1970 epoch origin and isn't meaningful for dates; `shift`, `disparity`, and their bounds remain populated
- **Sparsity required:** `spread`, `spread_lower`, `spread_upper`, `disparity`, `disparity_lower`, and `disparity_upper` need real variability (not tie-dominant data)
- **Insufficient data for bounds:** All bounds columns need enough data for the requested misrate; try a higher misrate or more data

See: [Pragmastat manual (PDF)](https://github.com/AndreyAkinshin/pragmastat/releases/download/v13.0.1/pragmastat-v13.0.1.pdf), [pragmastat.dev](https://pragmastat.dev/)

## `frequency`

The `frequency` command computes frequency distribution tables for CSV columns (exact by default, with an opt-in Frequent Items sketch for bounded-memory top-K), with support for multiple output formats, ranking strategies, and weighted frequencies.

**Key Features:**
- Computes exact frequency counts by default; opt into a Misra-Gries heavy-hitters sketch via `--sketch-method frequent_items` for bounded-memory top-K on very-high-cardinality streams (see [Frequent Items Sketch (Approximate Top-K)](#frequent-items-sketch-approximate-top-k))
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

### Frequency Cache

The `--frequency-jsonl` flag writes a complete frequency distribution as a JSONL cache file (`FILESTEM.freq.csv.data.jsonl`). When a valid (fresh) cache exists, subsequent `frequency` runs automatically reuse it instead of recomputing from the CSV.

**Cache Options:**

| Option | Default | Description |
|:---|:---|:---|
| `--frequency-jsonl` | off | Write the frequency distribution as a JSONL cache. Requires a file input (not stdin). |
| `--high-card-threshold <arg>` | `100` | Absolute cardinality threshold for `<HIGH_CARDINALITY>` classification. Can also be set with `QSV_FREQ_HIGH_CARD_THRESHOLD` env var (env var takes precedence when CLI value equals the default). |
| `--high-card-pct <arg>` | `90` | Percentage of rowcount threshold for `<HIGH_CARDINALITY>` classification. Must be between 1 and 100. Can also be set with `QSV_FREQ_HIGH_CARD_PCT` env var (env var takes precedence when CLI value equals the default). |
| `--force` | off | Force recomputation and cache regeneration even when a valid frequency cache exists. |

**HIGH_CARDINALITY Sentinel:**
Columns whose cardinality exceeds the smaller of `--high-card-threshold` and `--high-card-pct` percent of rowcount are classified as HIGH_CARDINALITY. These get a single `<HIGH_CARDINALITY>` sentinel entry (count = rowcount, percentage = 100%, rank = 0), analogous to the `<ALL_UNIQUE>` sentinel for ID columns.

**Cache Validation:**
- The cache is considered valid when the CSV file's mtime is older than the cache file's mtime
- Metadata compatibility is checked: `--no-nulls`, `--no-headers`, and `--delimiter` must match the cached settings

**Incompatibilities:**
The `--frequency-jsonl` flag produces an error when combined with:
- `--ignore-case` — case folding changes computed values
- `--no-trim` — whitespace handling changes computed values
- `--weight` — weighted frequencies change computed values

**Partial Cache Hits:**
When the cache is valid, columns with full cached data are served directly from the cache. HIGH_CARDINALITY columns (which store only a sentinel) are recomputed via parallel processing against the original CSV.

### Frequent Items Sketch (Approximate Top-K)

By default, `frequency` computes **exact** counts by tracking every distinct value in a HashMap. For columns with very high cardinality, this can be memory-prohibitive. The `--sketch-method frequent_items` flag swaps the HashMap for the [Misra-Gries heavy-hitters sketch](https://en.wikipedia.org/wiki/Misra%E2%80%93Gries_summary) (Apache DataSketches port), which tracks the top-K most frequent values in **constant memory** with bounded additive error.

| Option | Default | Description |
|:---|:---|:---|
| `--sketch-method <m>` | `exact` | Algorithm for the frequency table. Choices: `exact` (HashMap, exact counts) or `frequent_items` (Misra-Gries sketch, approximate top-K). |
| `--sketch-map-size <n>` | `4096` | Maximum map size for the Frequent Items sketch. **Must be a power of two and ≥ 8.** Larger values tighten the error bound at the cost of more memory. Only used when `--sketch-method frequent_items`. |

**Counts are estimates.** The sketch reports each item's upper-bound frequency estimate; tail items not retained in the sketch are aggregated into a single "Other" row (no unique-count suffix, since the sketch cannot recover the true number of distinct tail items). The sketch's natural ordering is top-K by estimate descending; tied counts use the sketch's hash-table iteration order. The frequency cache is bypassed under this mode.

**Rejected flags** (the command errors out if any of these are combined with `--sketch-method frequent_items`):

| Flag | Reason |
|:---|:---|
| `--asc` | The sketch tracks heavy hitters only — least-frequent items are not recoverable. |
| `--weight` | The Apache DataSketches Frequent Items sketch operates on unit-weight streams. |
| `--ignore-case` | Case folding changes computed values; not supported in streaming sketch mode. |
| `--no-trim` | Whitespace handling changes computed values; not supported in streaming sketch mode. |
| `--other-sorted` | The sketch always emits the "Other" row at the end. |
| `--null-sorted` | The sketch ranks NULL alongside other values by estimate; no reordering support. |
| `--frequency-jsonl` | The frequency cache is bypassed under sketch mode. |
| `--stats-filter` | Incompatible with sketch-mode dispatch. |
| `--json` / `--pretty-json` / `--toon` | Only CSV output is supported under sketch mode. |

**Silently ignored flags** under `frequent_items` (no error, no effect):

- `--rank-strategy` — the sketch's natural top-K-by-estimate ordering is used.
- `--lmt-threshold` — the sketch always tracks at most `--sketch-map-size` candidates.
- `--unq-limit` — the sketch's bounded map is itself the unique-limit.

**"Other" row divergence:** Under `frequent_items`, the "Other" row label is the bare `--other-text` (no `(N)` unique-count suffix, since the sketch cannot recover the true count of items not in the top-K), and `rank` is `0` to match the existing convention for the exact mode's "Other" row.

**When to use:** prefer `frequent_items` for streaming over wide tables with many high-cardinality string columns where you only care about the heavy hitters and want predictable, fixed memory. Prefer `exact` (default) for small/medium cardinality, weighted streams, or whenever you need a complete frequency distribution.

**See also:** [Misra-Gries summary (Wikipedia)](https://en.wikipedia.org/wiki/Misra%E2%80%93Gries_summary), [Apache DataSketches Frequent Items](https://datasketches.apache.org/docs/Frequency/FrequentItemsOverview.html).

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
        {"name": "min_length", "value": 1},
        {"name": "max_length", "value": 1},
        {"name": "avg_length", "value": 1},
        {"name": "mean", "value": null},
        {"name": "stddev", "value": null},
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

When `--no-stats` is NOT set and the column type is not empty, NULL, or Boolean, the per-field `stats` array contains up to 17 statistics (only those present in the underlying stats record are emitted):

1. `sum` — Sum of numeric values
2. `min` — Minimum value
3. `max` — Maximum value
4. `range` — Range (max - min)
5. `sort_order` — ASCENDING, DESCENDING, or UNSORTED
6. `min_length` — Shortest string length (String types)
7. `max_length` — Longest string length (String types)
8. `sum_length` — Total of all string lengths (String types)
9. `avg_length` — Average string length (String types)
10. `stddev_length` — Standard deviation of string lengths (String types)
11. `variance_length` — Variance of string lengths (String types)
12. `cv_length` — Coefficient of variation of string lengths (String types)
13. `mean` — Arithmetic mean
14. `sem` — Standard error of the mean
15. `stddev` — Standard deviation
16. `variance` — Variance
17. `cv` — Coefficient of variation

> **Note:** `cardinality`, `nullcount`, `sparsity`, and `uniqueness_ratio` are emitted as **top-level properties of each `FrequencyField`** (see the JSON Structure example above), not inside the per-field `stats` array. `geometric_mean`, `harmonic_mean`, and `max_precision` are *not* included in `frequency` JSON output even when present in the stats cache — use `qsv stats` directly if you need them.

### Memory-Aware Processing

The frequency command defaults to dynamic, memory-aware chunking for large datasets to avoid out-of-memory errors. Both `stats` and `frequency` default to memory-aware sizing; `frequency` is documented separately here because it builds hash tables and therefore benefits from predictable per-chunk memory budgeting. CPU-based chunking can be requested explicitly via `QSV_FREQ_CHUNK_MEMORY_MB = -1` (the same convention `stats` uses with `QSV_STATS_CHUNK_MEMORY_MB`).

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

---

## Processing Very Large Files

This section consolidates guidance for running `stats` and `frequency` on files that are large relative to available RAM (rule of thumb: any file whose CSV size approaches or exceeds 50% of free memory, or any file with columns whose true cardinality could approach the row count). For configuration knobs referenced below, see [ENVIRONMENT_VARIABLES.md](https://github.com/dathere/qsv/blob/master/docs/ENVIRONMENT_VARIABLES.md).

### When to Worry

Most `stats`/`frequency` invocations on multi-GB files **do not** need special handling, because the defaults already cover the common cases:

- **Streaming stats are constant-memory.** All 27 streaming statistics ([list above](#streaming-vs-non-streaming-statistics)) — `sum`, `min`, `max`, `min_length`/`max_length`/`avg_length`, `mean`/`stddev`/`variance`/`cv`, `sem`, `geometric_mean`/`harmonic_mean`, `nullcount`/`sparsity`, type-counts, `sort_order`/`sortiness`, `is_ascii`, `max_precision`, and `range` — run in O(1) memory regardless of row count. A plain `qsv stats huge.csv` on a 1 TB file with no non-streaming flags will not OOM.
- **`frequency` uses memory-aware chunking by default.** With an index, it samples the first 1000 records, estimates per-record + HashMap overhead, and sizes chunks to fit available memory.
- **OOM auto-fallback is on by default.** When non-streaming `stats` or `frequency` would exceed the memory-check budget, qsv first tries to auto-create an index (for non-stdin inputs), then auto-enables DataSketches estimators where the flags allow. A `wwarn!` line is printed listing each auto-enabled estimator.

You only need the recipes in this section when:

1. You are requesting **non-streaming stats** (any of `--mode`, `--cardinality`, `--quartiles`, `--median`, `--mad`, `--percentiles`, or `--everything`) on a file too large to materialize the per-column state in memory, **or**
2. You are running `frequency` on columns whose distinct-value count could blow up the HashMap (UUIDs, free-text, timestamps with sub-second precision), **or**
3. You are reading from **stdin** (no index can be auto-created — the OOM fallback can still engage the sketch path, but cannot use indexed parallelism), **or**
4. You are on a **big-endian** target (see [Platform Note](#platform-note-big-endian-targets)).

### Memory Pressure Points

| Command | Pressure point | Scales with | Mitigation |
|:---|:---|:---|:---|
| `stats` streaming | none | O(1) | — |
| `stats --cardinality` (exact) | distinct-value HashMap per column | per-column cardinality | `--cardinality-method approx` (HLL, ~5 KB/col) or `--mode-cardinality-cap <n>` |
| `stats --mode` / `--everything` (unweighted) | mode tracker Vec | **row count** (every cell pushed) | `--mode-cardinality-cap <n>` (this is the only knob; mode is **not** sketched even under `--cardinality-method approx`) |
| `stats --mode --weight=…` | mode tracker HashMap | per-column cardinality | `--mode-cardinality-cap <n>` |
| `stats --median` / `--quartiles` / `--mad` / `--percentiles` | sort buffer per column | column row count | `--quantile-method approx` (t-digest, ~200 centroids/col). Note: `--mad` is **auto-disabled** under approx (needs a second pass that t-digest cannot serve). |
| `frequency` (exact) | per-column HashMap of distinct values | per-column cardinality | `--sketch-method frequent_items` (Misra-Gries, fixed `--sketch-map-size` slots) |

Two things are easy to miss:

- **`mode`/`antimode` is the most common surprise.** Under unweighted mode tracking, every cell in the column is pushed onto an underlying Vec, so the tracker grows with **row count, not cardinality**. The `--cardinality-method approx` HLL replaces only the `cardinality` column — mode/antimode is **not** sketched. The fix is `--mode-cardinality-cap <n>`: when the tracker grows past `n`, qsv drops it and emits `*HIGH_CARDINALITY` for the mode/antimode columns.
- **`--mode-cardinality-cap 0` is the default** (no cap). It is opt-in because, under `--cardinality-method exact`, an exceeded cap emits `>=<n>` in the `cardinality` column — and the `>=` prefix breaks downstream integer parsers. Under `--cardinality-method approx`, the cap does not affect the cardinality column (HLL emits its estimate regardless), so combining the two is safe.

### Recipe: stats on Very Large Files

Maximum-safety invocation on a multi-GB CSV when you need the full non-streaming stat set:

```bash
# one-time: index enables parallel chunking
qsv index huge.csv

qsv stats huge.csv \
  --everything \
  --quantile-method approx \
  --cardinality-method approx \
  --mode-cardinality-cap 1000000 \
  --stats-jsonl \
  -o huge.stats.csv
```

What each non-default flag contributes:

- `--quantile-method approx` — t-digest for median/quartiles/percentiles/skewness.
- `--cardinality-method approx` — HyperLogLog for `cardinality`/`uniqueness_ratio`.
- `--mode-cardinality-cap 1000000` — bound the mode/antimode trackers.
- `--stats-jsonl` — also write the stats cache for downstream "smart" commands.

What this gives you, in order of memory savings:

1. **Indexed parallel processing.** Without an index, `stats` runs sequentially; with an index, work is split into memory-aware chunks (sized by `QSV_STATS_CHUNK_MEMORY_MB`) processed in parallel and merged.
2. **t-digest for quantiles.** Median, q1/q2/q3, IQR, fences, skewness, and `--percentiles` all read from a ~200-centroid t-digest per column instead of sorting the full column. Error is ~1% rank error, more accurate at the tails. **Caveat:** `TDigestMut::merge` is associative but not chunk-count-invariant, so different `--jobs` values can yield ~1% differences across runs. Pin `--jobs 1` for run-to-run determinism. `--mad` is auto-disabled with a warning.
3. **HLL for cardinality.** `cardinality` and `uniqueness_ratio` come from a ~5 KB HyperLogLog per column. ~1.5% RSE. The HLL union is associative and order-invariant, so the estimate **is** reproducible across `--jobs` values.
4. **Cap on mode/antimode tracker.** Without this, the unweighted mode tracker grows linearly with row count. With it set to, e.g., `1000000`, columns whose tracker exceeds that drop to `*HIGH_CARDINALITY` for mode/antimode while every other statistic remains valid.

**Flags that block this recipe** (you must drop them or fall back to exact mode):

- `--weight <col>` — t-digest has no weighted-update API upstream, so `--quantile-method approx` is rejected with `--weight`.
- `--infer-boolean` — needs `cardinality == 2` exactness, so `--cardinality-method approx` is rejected (or, under OOM auto-enable, suppressed) with `--infer-boolean`.

If neither of those applies, you can omit the explicit method flags and rely on the OOM auto-fallback: qsv will flip them on automatically when `util::mem_file_check` trips. You can disable the auto-enable by passing `--quantile-method exact` or `--cardinality-method exact` explicitly (the OOM arm scans `argv` for these flag names, so docopt's default-fill does not count as an explicit opt-out).

### Recipe: frequency on Very Large Files

If you only care about the **top-K most frequent values** (a common analyst case), use the Misra-Gries sketch:

```bash
qsv index huge.csv
qsv frequency huge.csv \
  --sketch-method frequent_items \
  --sketch-map-size 4096 \
  --limit 100 \
  -o huge.freq.csv
```

`--sketch-map-size` must be a power of two and ≥ 8; larger values tighten the error bound at the cost of more memory. `--limit 100` emits the top 100 values per column.

`--sketch-map-size` sets the upper bound on map slots; the sketch's worst-case additive error is bounded by the stream length minus the active map total, so doubling the map size roughly halves the error bound at the cost of doubling memory. 4096 is a reasonable starting point; bump to 16384 or 65536 for tighter bounds.

**Flags that are rejected** under `--sketch-method frequent_items` (the full list is in the [Frequent Items Sketch section](#frequent-items-sketch-approximate-top-k)): `--asc`, `--weight`, `--ignore-case`, `--no-trim`, `--other-sorted`, `--null-sorted`, `--frequency-jsonl`, `--stats-filter`, `--json`/`--pretty-json`/`--toon`. If you need any of these, you must run in exact mode and rely on memory-aware chunking (and possibly the OOM auto-enable, which is itself blocked by the same flag set).

**Silently ignored under FI mode:** `--rank-strategy`, `--lmt-threshold`, `--unq-limit` (the sketch's bounded map is itself the unique-limit, and ordering is fixed at top-K by estimate descending).

**"Other" row divergence:** the `Other` label has no `(N)` unique-count suffix and `rank` is `0`, since the sketch cannot recover the true tail count.

### Indexing for Parallelism

For both `stats` and `frequency`, an index is the single highest-leverage prerequisite for large-file processing:

```bash
qsv index huge.csv      # creates huge.csv.idx; updated automatically when stale
```

What an index unlocks:

- **Parallel chunking.** Work is split across cores (`-j N` or auto-detected). Each chunk is processed independently and merged.
- **Memory-aware chunk sizing.** With `QSV_STATS_CHUNK_MEMORY_MB` / `QSV_FREQ_CHUNK_MEMORY_MB` unset (the default), qsv samples the first 1000 records, estimates per-record memory, and picks a chunk size that fits available memory.
- **OOM fallback for `stats`.** When `util::mem_file_check` trips and no index exists, qsv attempts to auto-create one before falling back to sketches. Auto-creation is skipped for stdin (not seekable), so `cat huge.csv | qsv stats …` cannot benefit from indexed parallelism — pipe to a file first if you can.

You can also auto-build the index by setting `QSV_AUTOINDEX_SIZE=<bytes>` — any CSV larger than that threshold gets an index created on first use.

### Environment Variable Cheat Sheet

Most relevant for large-file work (see [ENVIRONMENT_VARIABLES.md](https://github.com/dathere/qsv/blob/master/docs/ENVIRONMENT_VARIABLES.md) for the full list):

| Variable | Effect |
|:---|:---|
| `QSV_AUTOINDEX_SIZE` | Minimum file size (bytes) for automatic index creation. Set this so big inputs always get indexed. |
| `QSV_MEMORY_CHECK` | Switches `util::mem_file_check` from NORMAL (`file size vs. total memory − headroom`) to CONSERVATIVE (`file size vs. available + free_swap × platform_factor − headroom`). Trips OOM far more readily, so the auto-fallback engages sooner. |
| `QSV_FREEMEMORY_HEADROOM_PCT` | Free-memory headroom for the memory check (default 20%). Set to `0` to skip the check entirely (use at your own risk). |
| `QSV_STATS_CHUNK_MEMORY_MB` | Per-chunk memory cap for `stats` (positive integer in MB). `0` = dynamic sizing. `-1` = CPU-based chunking (chunks = rows/cores; ignores memory). |
| `QSV_FREQ_CHUNK_MEMORY_MB` | Same semantics as above, for `frequency`. |
| `QSV_ANTIMODES_LEN` | Truncation length for the antimodes preview (default 100 chars). `0` disables truncation. |
| `QSV_STATS_STRING_MAX_LENGTH` | Truncate `min`/`max` for String columns at this length (useful when a column contains GeoJSON / Shapefile geometry blobs that would otherwise blow up downstream parsers). |
| `QSV_MAX_JOBS` | Cap on parallel workers across all multithreaded qsv commands. Useful when each chunk's in-memory state is large (lower `QSV_MAX_JOBS` to leave headroom). |
| `QSV_FREQ_HIGH_CARD_THRESHOLD` / `QSV_FREQ_HIGH_CARD_PCT` | Cardinality cutoffs for the `--frequency-jsonl` cache to emit a `HIGH_CARDINALITY` sentinel instead of a full frequency entry. Useful for keeping the cache compact on wide tables with ID-like columns. |

### Worked Example: a Multi-GB CSV

For a 30 GB CSV with ~200 columns on a 32 GB host, where some columns are UUIDs:

```bash
# 1. Index up front so all subsequent passes are parallel + chunked.
qsv index big.csv

# 2. Stats: full non-streaming set, but bound mode tracking and use sketches.
QSV_STATS_CHUNK_MEMORY_MB=512 \
qsv stats big.csv \
  --everything \
  --quantile-method approx \
  --cardinality-method approx \
  --mode-cardinality-cap 1000000 \
  --stats-jsonl \
  -o big.stats.csv

# 3. Frequency: top-100 per column, sketch-mode for fixed memory.
qsv frequency big.csv \
  --sketch-method frequent_items \
  --sketch-map-size 16384 \
  --limit 100 \
  -o big.freq.csv

# 4. (Optional) Tighten the memory check if you're sharing the host:
QSV_MEMORY_CHECK=1 QSV_FREEMEMORY_HEADROOM_PCT=40 qsv stats big.csv …
```

If you forgot any of the sketch flags and `stats` hits the memory check, the OOM auto-fallback will print a `wwarn!` line such as:

```
OOM during memory check: auto-enabling DataSketches estimators
(--quantile-method approx, --cardinality-method approx).
Re-run with explicit --quantile-method exact / --cardinality-method exact
to disable the auto-enable.
```

The exact estimators auto-enabled depend on which incompatible flags are set (`--weight` blocks t-digest; `--infer-boolean` blocks HLL; `--mad`/`--everything` causes MAD to be auto-disabled under approx). The corresponding line for `frequency` mentions `--sketch-method frequent_items` and reports the map size.

### Hard Limits (What Will Still OOM)

The DataSketches integration is a major step toward unbounded inputs, but it does **not** make `stats`/`frequency` truly unconditional. Cases where you can still hit memory exhaustion:

1. **Big-endian targets** (s390x, PowerPC BE). DataSketches is unavailable — all `--quantile-method approx`, `--cardinality-method approx`, and `--sketch-method frequent_items` paths are rejected, and the OOM auto-enable compiles to a no-op stub. On these targets, fall back to `--mode-cardinality-cap`, smaller `QSV_STATS_CHUNK_MEMORY_MB`, and `--limit` / `--unq-limit` on `frequency`.
2. **Unweighted mode/antimode without a cap.** The tracker grows with row count regardless of `--cardinality-method`. Solution: set `--mode-cardinality-cap` to a value you can afford, or drop `--mode`/`--everything`.
3. **Frequency in exact mode with unbounded distinct values.** If the column is truly unique-per-row (a UUID column on a 1 B-row CSV), exact mode needs ~1 B HashMap entries. Solution: switch to `--sketch-method frequent_items`, or pre-bucket the column.
4. **`--weight` blocks t-digest** and **`--infer-boolean` blocks HLL** for `stats`. If both flags are set, neither auto-enable engages, and the memory check will simply fail. Solution: drop the blocking flag, run a separate boolean-inference pass with `stats` alone (no `--weight`), or accept exact mode with adequate RAM.
5. **`frequency` flag combinations that reject Frequent Items.** If you need `--asc`, `--ignore-case`, `--no-trim`, `--weight`, `--other-sorted`, `--null-sorted`, `--frequency-jsonl`, `--stats-filter`, or `--json`/`--pretty-json`/`--toon`, the sketch path is unavailable. Solution: exact mode with sufficient RAM, or do without that flag.
6. **Stdin input for `stats`.** Stdin is not seekable, so the auto-index path is skipped. The sketch auto-enable still runs, but you lose parallelism. Solution: tee to a file first (`tee /tmp/in.csv | qsv stats …` or `qsv stats /tmp/in.csv`).
7. **Explicit `--*-method exact` opt-out.** The OOM auto-enable scans `argv` for `--quantile-method` / `--cardinality-method` / `--sketch-method`; if you passed any of those (even `exact`), auto-enable is suppressed for that method. Drop the explicit opt-out to re-enable the fallback.

### Platform Note: Big-Endian Targets

Apache DataSketches' Rust port is gated to little-endian targets (verified upfront in `stats::run`, `frequency::run`, and the OOM fallback paths). On big-endian targets:

- `--quantile-method approx`, `--cardinality-method approx`, and `--sketch-method frequent_items` are all rejected with a clear error.
- `try_enable_approx_sketches` (stats) and `can_enable_frequent_items` (frequency) compile to no-op stubs, so the OOM path falls through to error rather than silently degrading.

If you maintain qsv on a big-endian platform, the practical large-file toolkit is:

- `--mode-cardinality-cap` for `stats` (bounds mode/antimode tracking only).
- Smaller `QSV_STATS_CHUNK_MEMORY_MB` / `QSV_FREQ_CHUNK_MEMORY_MB` to keep per-chunk state small.
- Pre-bucketing high-cardinality columns (e.g., truncate timestamps to the hour) before running `frequency`.
- For sort/dedup adjacencies, use `extsort`/`extdedup` (external on-disk variants) instead of `sort`/`dedup`.

### Notes for moarstats and pragmastat

The DataSketches fallback applies to `stats` and `frequency` only. Two adjacent commands have their own characteristics:

- **`moarstats`** computes the [Advanced](#advanced-statistics), [Bivariate](#bivariate-statistics), [Robust](#robust-statistics-winsorized--trimmed-means), and [Outlier](#outlier-statistics) statistic families. Most require either two passes or a full in-memory column (e.g., outlier detection needs the IQR + every value; correlation needs paired columns held together). There is no sketch fallback — for very large inputs, sample first with `qsv sample` and run `moarstats` on the sample, or pre-filter columns to the ones you actually need.
- **`pragmastat`** ([one-sample mode](#one-sample-mode-default) and [two-sample mode](#two-sample-mode)) computes deterministic robust estimators that require full-sample residuals. It is designed for inputs that fit comfortably in memory; for very large inputs, sample down first.
