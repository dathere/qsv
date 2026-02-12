# stats

> Compute summary statistics (sum, min/max/range, sort order/sortiness, min/max/sum/avg length, mean, standard error of the mean (SEM), geometric/harmonic means, stddev, variance, Coefficient of Variation (CV), nullcount, max precision, sparsity, quartiles, Interquartile Range (IQR), lower/upper fences, skewness, median, mode/s, antimode/s, cardinality & uniqueness ratio) & make GUARANTEED data type inferences (Null, String, Float, Integer, Date, DateTime, Boolean) for each column in a CSV (Example - more info). Uses multithreading to go faster if an index is present (with an index, can compile "streaming" stats on NYC's 311 data (15gb, 28m rows) in less than 7.3 seconds!).

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/stats.rs](https://github.com/dathere/qsv/blob/master/src/cmd/stats.rs)**

## Description

Compute summary statistics & infers data types for each column in a CSV.

> IMPORTANT: `stats` is heavily optimized for speed. It ASSUMES the CSV is well-formed & UTF-8 encoded.
> This allows it to employ numerous performance optimizations (skip repetitive UTF-8 validation, skip
> bounds checks, cache results, etc.) that may result in undefined behavior if the CSV is not well-formed.
> All these optimizations are GUARANTEED to work with well-formed CSVs.
> If you encounter problems generating stats, use `qsv validate` FIRST to confirm the CSV is valid.

> For MAXIMUM PERFORMANCE, create an index for the CSV first with 'qsv index' to enable multithreading,
> or set --cache-threshold option or set the QSV_AUTOINDEX_SIZE environment variable to automatically
> create an index when the file size is greater than the specified size (in bytes).

Summary stats include sum, min/max/range, sort order/sortiness, min/max/sum/avg/stddev/variance/cv length,
mean, standard error of the mean (SEM), geometric mean, harmonic mean, stddev, variance, coefficient of
variation (CV), nullcount, n_negative, n_zero, n_positive, max_precision, sparsity,
Median Absolute Deviation (MAD), quartiles, lower/upper inner/outer fences, skewness, median,
cardinality/uniqueness ratio, mode/s & "antimode/s" & percentiles.

Note that some stats require loading the entire file into memory, so they must be enabled explicitly.

By default, the following "streaming" statistics are reported for *every* column:
sum, min/max/range values, sort order/"sortiness", min/max/sum/avg/stddev/variance/cv length, mean, sem,
geometric_mean, harmonic_mean,stddev, variance, cv, nullcount, n_negative, n_zero, n_positive,
max_precision & sparsity.

The default set of statistics corresponds to ones that can be computed efficiently on a stream of data
(i.e., constant memory) and works with arbitrarily large CSVs.

The following additional "non-streaming, advanced" statistics require loading the entire file into memory:
cardinality/uniqueness ratio, modes/antimodes, median, MAD, quartiles and its related measures
(q1, q2, q3, IQR, lower/upper fences & skewness) and percentiles.

When computing "non-streaming" statistics, a memory-aware chunking algorithm is used to dynamically
calculate chunk size based on available memory & record sampling. This SHOULD help process arbitrarily
large "real-world" files by creating smaller chunks that fit in available memory.
However, there is still a chance that the command will run out of memory if the cardinality of
several columns is very high.

Chunk size is dynamically calculated based on the number of logical CPUs detected.
You can override this behavior by setting the QSV_STATS_CHUNK_MEMORY_MB environment variable
(set to 0 for dynamic sizing, or a positive number for a fixed memory limit per chunk,
or -1 for CPU-based chunking (1 chunk = records/number of CPUs)).

"Antimode" is the least frequently occurring non-zero value and is the opposite of mode.
It returns "*ALL" if all the values are unique, and only returns a preview of the first
10 antimodes, truncating after 100 characters (configurable with QSV_ANTIMODES_LEN).

If you need all the antimode values of a column, run the `frequency` command with --limit set
to zero. The resulting frequency table will have all the "antimode" values.

Summary statistics for dates are also computed when --infer-dates is enabled, with DateTime
results in rfc3339 format and Date results in "yyyy-mm-dd" format in the UTC timezone.
Date range, stddev, variance, MAD & IQR are returned in days, not timestamp milliseconds.

Each column's data type is also inferred (NULL, Integer, String, Float, Date, DateTime and
Boolean with --infer-boolean option).
For String data types, it also determines if the column is all ASCII characters.
Unlike the sniff command, stats' data type inferences are GUARANTEED, as the entire file
is scanned, and not just sampled.

Note that the Date and DateTime data types are only inferred with the --infer-dates option
as its an expensive operation to match a date candidate against 19 possible date formats,
with each format, having several variants.

The date formats recognized and its sub-variants along with examples can be found at
<https://github.com/dathere/qsv-dateparser?tab=readme-ov-file#accepted-date-formats>.

Computing statistics on a large file can be made MUCH faster if you create an index for it
first with 'qsv index' to enable multithreading. With an index, the file is split into chunks
and each chunk is processed in parallel.

As stats is a central command in qsv, and can be expensive to compute, `stats` caches results
in <FILESTEM>.stats.csv & if the --stats-json option is used, <FILESTEM>.stats.csv.data.jsonl
(e.g., qsv stats nyc311.csv will create nyc311.stats.csv & nyc311.stats.csv.data.jsonl).
The arguments used to generate the cached stats are saved in <FILESTEM>.stats.csv.jsonl.

If stats have already been computed for the input file with similar arguments and the file
hasn't changed, the stats will be loaded from the cache instead of recomputing it.

These cached stats are also used by other qsv commands (currently `describegpt`, `frequency`,
`joinp`, `pivotp`, `schema`, `sqlp` & `tojsonl`) to work smarter & faster.
If the cached stats are not current (i.e., the input file is newer than the cached stats),
the cached stats will be ignored and recomputed. For example, see the "boston311" test files in
<https://github.com/dathere/qsv/blob/4529d51273218347fef6aca15ac24e22b85b2ec4/tests/test_stats.rs#L608>.


## Examples

> Compute "streaming" statistics for "nyc311.csv"

```console
qsv stats nyc311.csv
```

> Compute all statistics for "nyc311.csv"

```console
qsv stats --everything nyc311.csv
```

> Compute all statistics for "nyc311.tsv" (Tab-separated)

```console
qsv stats -E nyc311.tsv
```

> Compute all stats for "nyc311.tsv", inferring dates using sniff to auto-detect date columns

```console
qsv stats -E --infer-dates nyc311.tsv
```

> Compute all stats for "nyc311.tab", inferring dates only for columns

> with "_date" & "_dte" in the column names

```console
qsv stats -E --infer-dates --dates-whitelist _date,_dte nyc311.tab
```

> Compute all stats, infer dates and boolean data types for "nyc311.ssv" file

```console
qsv stats -E --infer-dates --infer-boolean nyc311.ssv
```

> In addition to basic "streaming" stats, also compute cardinality for "nyc311.csv"

```console
qsv stats --cardinality nyc311.csv
```

> Prefer DMY format when inferring dates for the "nyc311.csv"

```console
qsv stats -E --infer-dates --prefer-dmy nyc311.csv
```

> Infer data types only for the "nyc311.csv" file:

```console
qsv stats --typesonly nyc311.csv
```

> Infer data types only, including boolean and date types for "nyc311.csv"

```console
qsv stats --typesonly --infer-boolean --infer-dates nyc311.csv
```

> Automatically create an index for the "nyc311.csv" file to enable multithreading

> if it's larger than 5MB and there is no existing index file:

```console
qsv stats -E --cache-threshold -5000000 nyc311.csv
```

> Auto-create a TEMPORARY index for the "nyc311.csv" file to enable multithreading

> if it's larger than 5MB and delete the index and the stats cache file after the stats run:

```console
qsv stats -E --cache-threshold -5000005 nyc311.csv
```

For more examples, see [tests](https://github.com/dathere/qsv/tree/master/resources/test).

If the polars feature is enabled, support additional tabular file formats and
compression formats:
```console
qsv stats data.parquet // Parquet
```

```console
qsv stats data.avro // Avro
```

```console
qsv stats data.jsonl // JSON Lines
```

```console
qsv stats data.json (will only work with a JSON Array)
```

```console
qsv stats data.csv.gz // Gzipped CSV
```

```console
qsv stats data.tab.zlib // Zlib-compressed Tab-separated
```

```console
qsv stats data.ssv.zst // Zstd-compressed Semicolon-separated
```

For more info, see <https://github.com/dathere/qsv/blob/master/docs/STATS_DEFINITIONS.md>

## Usage

```console
qsv stats [options] [<input>]
qsv stats --help
```

## Stats Options

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-s, --select` | string | Select a subset of columns to compute stats for. See 'qsv select --help' for the format details. This is provided here because piping 'qsv select' into 'qsv stats' will prevent the use of indexing. |  |
| `-E, --everything` | flag | Compute all statistics available. |  |
| `--typesonly` | flag | Infer data types only and do not compute statistics. Note that if you want to infer dates and boolean types, you'll still need to use the --infer-dates & --infer-boolean options. |  |

## Boolean Inferencing Options

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `--infer-boolean` | flag | Infer boolean data type. This automatically enables the --cardinality option. When a column's cardinality is 2, and the 2 values' are in the true/false patterns specified by --boolean-patterns, the data type is inferred as boolean. |  |
| `--boolean-patterns` | string | Comma-separated list of boolean pattern pairs in the format "true_pattern:false_pattern". Each pattern can be a string of any length. The patterns are case-insensitive. If a pattern ends with a "*", it is treated as a prefix. For example, "t*:f*,y*:n*" will match "true", "truthy", "Truth" as boolean true values so long as the corresponding false pattern (e.g. False, f, etc.) is also matched & cardinality is 2. Ignored if --infer-boolean is false. | `1:0,t*:f*,y*:n*` |
| `--mode` | flag | Compute the mode/s & antimode/s. Multimodal-aware. If there are multiple modes/antimodes, they are separated by the QSV_STATS_SEPARATOR environment variable. If not set, the default separator is "\|". Uses memory proportional to the cardinality of each column. |  |
| `--cardinality` | flag | Compute the cardinality and the uniqueness ratio. This is automatically enabled if --infer-boolean is enabled. <https://en.wikipedia.org/wiki/Cardinality_(SQL_statements)> Uses memory proportional to the number of unique values in each column. |  |
| `--median` | flag | Compute the median. Loads & sorts all the selected columns' data in memory. <https://en.wikipedia.org/wiki/Median> |  |
| `--mad` | flag | Compute the median absolute deviation (MAD). <https://en.wikipedia.org/wiki/Median_absolute_deviation> |  |
| `--quartiles` | flag | Compute the quartiles (using method 3), the IQR, the lower/upper, inner/outer fences and skewness. <https://en.wikipedia.org/wiki/Quartile#Method_3> |  |
| `--percentiles` | flag | Compute custom percentiles using the nearest rank method. <https://en.wikipedia.org/wiki/Percentile#The_nearest-rank_method> |  |
| `--percentile-list` | string | Comma-separated list of percentiles to compute. For example, "5,10,40,60,90,95" will compute percentiles 5th, 10th, 40th, 60th, 90th, and 95th. Multiple percentiles are separated by the QSV_STATS_SEPARATOR environment variable. If not set, the default separator is "\|". It is ignored if --percentiles is not set. Special values "deciles" and "quintiles" are automatically expanded to "10,20,30,40,50,60,70,80,90" and "20,40,60,80" respectively. | `5,10,40,60,90,95` |
| `--round` | string | Round statistics to <decimal_places>. Rounding is done following Midpoint Nearest Even (aka "Bankers Rounding") rule. <https://docs.rs/rust_decimal/latest/rust_decimal/enum.RoundingStrategy.html> If set to the sentinel value 9999, no rounding is done. For dates - range, stddev & IQR are always at least 5 decimal places as they are reported in days, and 5 places gives us millisecond precision. | `4` |
| `--nulls` | flag | Include NULLs in the population size for computing mean and standard deviation. |  |
| `--weight` | string | Compute weighted statistics using the specified column as weights. The weight column must be numeric. When specified, all statistics (mean, stddev, variance, median, quartiles, mode, etc.) will be computed using weighted algorithms. The weight column is automatically excluded from statistics computation. Missing or non-numeric weights default to 1.0. Zero and negative weights are ignored and do not contribute to the statistics. The output filename will be <FILESTEM>.stats.weighted.csv to distinguish from unweighted statistics. |  |

## Date Inferencing Options

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `--infer-dates` | flag | Infer date/datetime data types. This is an expensive option and should only be used when you know there are date/datetime fields. Also, if timezone is not specified in the data, it'll be set to UTC. |  |
| `--dates-whitelist` | string | The comma-separated, case-insensitive patterns to look for when shortlisting fields for date inferencing. i.e. if the field's name has any of these patterns, it is shortlisted for date inferencing. | `sniff` |
| `--prefer-dmy` | flag | Parse dates in dmy format. Otherwise, use mdy format. Ignored if --infer-dates is false. |  |
| `--force` | flag | Force recomputing stats even if valid precomputed stats cache exists. |  |
| `-j, --jobs` | string | The number of jobs to run in parallel. This works only when the given CSV has an index. Note that a file handle is opened for each job. When not set, the number of jobs is set to the number of CPUs detected. |  |
| `--stats-jsonl` | flag | Also write the stats in JSONL format. If set, the stats will be written to <FILESTEM>.stats.csv.data.jsonl. Note that this option used internally by other qsv "smart" commands (see <https://github.com/dathere/qsv/blob/master/docs/PERFORMANCE.md#stats-cache>) to load cached stats to make them work smarter & faster. You can preemptively create the stats-jsonl file by using this option BEFORE running "smart" commands and they will automatically use it. |  |
| `-c, --cache-threshold` | string | Controls the creation of stats cache files. * when greater than 1, the threshold in milliseconds before caching stats results. If a stats run takes longer than this threshold, the stats results will be cached. * 0 to suppress caching. * 1 to force caching. * a negative number to automatically create an index when the input file size is greater than abs(arg) in bytes. If the negative number ends with 5, it will delete the index file and the stats cache file after the stats run. Otherwise, the index file and the cache files are kept. | `5000` |
| `--vis-whitespace` | flag | Visualize whitespace characters in the output. See <https://github.com/dathere/qsv/wiki/Supplemental#whitespace-markers> for the list of whitespace markers. |  |

## Common Options

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-h, --help` | flag | Display this message |  |
| `-o, --output` | string | Write output to <file> instead of stdout. |  |
| `-n, --no-headers` | flag | When set, the first row will NOT be interpreted as column names. i.e., They will be included in statistics. |  |
| `-d, --delimiter` | string | The field delimiter for READING CSV data. Must be a single character. (default: ,) |  |
| `--memcheck` | flag | Check if there is enough memory to load the entire CSV into memory using CONSERVATIVE heuristics. This option is ignored when computing default, streaming statistics, as it is not needed. |  |

---
**Source:** [`src/cmd/stats.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/stats.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
