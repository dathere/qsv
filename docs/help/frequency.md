# frequency

> Build frequency distribution tables of each column. Uses multithreading to go faster if an index is present (Examples: CSV JSON TOON).

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/frequency.rs](https://github.com/dathere/qsv/blob/master/src/cmd/frequency.rs)** | 📇😣🏎️👆🪄![Luau](../images/luau.png)

<a name="nav"></a>
[Description](#description) | [Usage](#usage) | [Frequency Options](#frequency-options) | [Json Output Options](#json-output-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Compute a frequency distribution table on input data. It has CSV and JSON output modes.
<https://en.wikipedia.org/wiki/Frequency_(statistics)#Frequency_distribution_table>

In CSV output mode (default), the table is formatted as CSV data with the following
columns - field,value,count,percentage,rank.

The rank column is 1-based and is calculated based on the count of the values,
with the most frequent having a rank of 1. In case of ties, the rank is calculated
based on the rank-strategy option - "min" (default), "max", "dense", "ordinal", or "average".

Only the top N values (set by the --limit option) are computed, with the rest of the values
grouped into an "Other" category with a special rank of 0. The "Other" category includes
the count of remaining unique values that are not in the top N values.

In JSON output mode, the table is formatted as nested JSON data. In addition to
the columns above, the JSON output also includes the row count, field count, rank-strategy,
each field's data type, cardinality, nullcount, sparsity, uniqueness_ratio and its stats.

Since this command computes an exact frequency distribution table, memory proportional
to the cardinality of each column would be normally required.

However, this is problematic for columns with ALL unique values (e.g. an ID column),
as the command will need to allocate memory proportional to the column's cardinality.

To overcome this, the frequency command uses several mechanisms:

STATS CACHE:
If the stats cache exists for the input file, it is used to get column cardinality information.
This short-circuits frequency compilation for columns with all unique values (i.e. where
rowcount == cardinality), eliminating the need to maintain an in-memory hashmap for ID columns.
This allows `frequency` to handle larger-than-memory datasets with the added benefit of also
making it faster when working with datasets with ID columns.

That's why for MAXIMUM PERFORMANCE, it's HIGHLY RECOMMENDED to create an index (`qsv index data.csv`)
and pre-populate the stats cache (`qsv stats data.csv --cardinality --stats-jsonl`)
BEFORE running `frequency`.

MEMORY-AWARE CHUNKING:
When working with large datasets, memory-aware chunking is automatically enabled. Chunk size
is dynamically calculated based on available memory and record sampling.

You can override this behavior by setting the QSV_FREQ_CHUNK_MEMORY_MB environment variable.
(set to 0 for dynamic sizing, or a positive number for a fixed memory limit per chunk,
or -1 for CPU-based chunking (1 chunk = num records/number of CPUs)), or by setting the --jobs option.

NOTE: "Complete" Frequency Tables:

By default, ID columns will have an "<ALL UNIQUE>" value with count equal to
rowcount and percentage set to 100 with a rank of 0. This is done by using the
stats cache to fetch each column's cardinality - allowing qsv to short-circuit
frequency compilation and eliminate the need to maintain a hashmap for ID columns.

If you wish to compile a "complete" frequency table even for ID columns, set
QSV_STATSCACHE_MODE to "none". This will force the frequency command to compute
frequencies for all columns regardless of cardinality, even for ID columns.

In this case, the unique limit (--unq-limit) option is particularly useful when
a column has all unique values  and --limit is set to 0.
Without a unique limit, the frequency table for that column will be the same as
the number of rows in the data.
With a unique limit, the frequency table will be a sample of N unique values,
all with a count of 1.

The --lmt-threshold option also allows you to apply the --limit and --unq-limit
options only when the number of unique items in a column >= threshold.
This is useful when you want to apply limits only to columns with a large number
of unique items and not to columns with a small number of unique items.

For examples, see <https://github.com/dathere/qsv/blob/master/tests/test_frequency.rs>.


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv frequency [options] [<input>]
qsv frequency --help
```

<a name="frequency-options"></a>

## Frequency Options [↩](#nav)

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-s, --select` | string | Select a subset of columns to compute frequencies for. See 'qsv select --help' for the format details. This is provided here because piping 'qsv select' into 'qsv frequency' will disable the use of indexing. |  |
| `-l, --limit` | string | Limit the frequency table to the N most common items. Set to '0' to disable a limit. If negative, only return values with an occurrence count >= absolute value of the negative limit. e.g. --limit -2 will only return values with an occurrence count >= 2. | `10` |
| `-u, --unq-limit` | string | If a column has all unique values, limit the frequency table to a sample of N unique items. Set to '0' to disable a unique_limit. | `10` |
| `--lmt-threshold` | string | The threshold for which --limit and --unq-limit will be applied. If the number of unique items in a column >= threshold, the limits will be applied. Set to '0' to disable the threshold and always apply limits. | `0` |
| `-r, --rank-strategy` | string | The strategy to use when there are count-tied values in the frequency table. See <https://en.wikipedia.org/wiki/Ranking> for more info. | `dense` |
| `--pct-dec-places` | string | The number of decimal places to round the percentage to. If negative, the number of decimal places will be set automatically to the minimum number of decimal places needed to represent the percentage accurately, up to the absolute value of the negative number. | `-5` |
| `--other-sorted` | flag | By default, the "Other" category is placed at the end of the frequency table for a field. If this is enabled, the "Other" category will be sorted with the rest of the values by count. |  |
| `--other-text` | string | The text to use for the "Other" category. If set to "<NONE>", the "Other" category will not be included in the frequency table. | `Other` |
| `--no-other` | flag | Don't include the "Other" category in the frequency table. This is equivalent to --other-text "<NONE>". |  |
| `--null-sorted` | flag | By default, the NULL category (controlled by --null-text) is placed at the end of the frequency table for a field, after "Other" if present. If this is enabled, the NULL category will be sorted with the rest of the values by count. |  |
| `-a, --asc` | flag | Sort the frequency tables in ascending order by count. The default is descending order. Note that this option will also reverse ranking - i.e. the LEAST frequent values will have a rank of 1. |  |
| `--no-trim` | flag | Don't trim whitespace from values when computing frequencies. The default is to trim leading and trailing whitespaces. |  |
| `--null-text` | string | The text to use for NULL values. If set to "<NONE>", NULLs will not be included in the frequency table (equivalent to --no-nulls). | `(NULL)` |
| `--no-nulls` | flag | Don't include NULLs in the frequency table. This is equivalent to --null-text "<NONE>". |  |
| `--pct-nulls` | flag | Include NULL values in percentage and rank calculations. When disabled (default), percentages are "valid percentages" calculated with NULLs excluded from the denominator, and NULL entries display empty percentage and rank values. When enabled, NULLs are included in the denominator (original behavior). Has no effect when --no-nulls is set. |  |
| `-i, --ignore-case` | flag | Ignore case when computing frequencies. |  |
| `--no-float` | string | Exclude Float columns from frequency analysis. Floats typically contain continuous values where frequency tables are not meaningful. To exclude ALL Float columns, use --no-float "*" To exclude Floats except specific columns, specify a comma-separated list of Float columns to INCLUDE. e.g. "--no-float *" excludes all Floats "--no-float price,rate" excludes Floats except 'price' and 'rate' Requires stats cache for type detection. |  |
| `--stats-filter` | string | Filter columns based on their statistics using a Luau expression. Columns where the expression evaluates to `true` are EXCLUDED. Available fields: field, type, is_ascii, cardinality, nullcount, sum, min, max, range, sort_order, min_length, max_length, mean, stddev, variance, cv, sparsity, q1, q2_median, q3, iqr, mad, skewness, mode, antimode, n_negative, n_zero, n_positive, etc. e.g. "nullcount > 1000" - exclude columns with many nulls "type == 'Float'" - exclude Float columns "cardinality > 500 and nullcount > 0" - compound expression Requires stats cache and the "luau" feature. |  |
| `--all-unique-text` | string | The text to use for the "<ALL_UNIQUE>" category. | `<ALL_UNIQUE>` |
| `--vis-whitespace` | flag | Visualize whitespace characters in the output. See <https://github.com/dathere/qsv/wiki/Supplemental#whitespace-markers> for the list of whitespace markers. |  |
| `-j, --jobs` | string | The number of jobs to run in parallel when the given CSV data has an index. Note that a file handle is opened for each job. When not set, defaults to the number of CPUs detected. |  |

<a name="json-output-options"></a>

## Json Output Options [↩](#nav)

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `--json` | flag | Output frequency table as nested JSON instead of CSV. The JSON output includes additional metadata: row count, field count, data type, cardinality, null count, sparsity, uniqueness_ratio and 17 additional stats (e.g. sum, min, max, range, sort_order, mean, sem, etc.). |  |
| `--pretty-json` | flag | Same as --json but pretty prints the JSON output. |  |
| `--toon` | flag | Output frequency table and select stats in TOON format instead of CSV. TOON is a compact, human-readable encoding of the JSON data model for LLM prompts. See <https://toonformat.dev/> for more info. |  |
| `--no-stats` | flag | When using the JSON or TOON output mode, do not include the additional stats. |  |
| `--weight` | string | Compute weighted frequencies using the specified column as weights. The weight column must be numeric. When specified, frequency counts are multiplied by the weight value for each row. The weight column is automatically excluded from frequency computation. Missing or unparsable weights default to 1.0. Zero, negative, NaN and infinite weights are ignored and do not contribute to frequencies. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-h, --help` | flag | Display this message |  |
| `-o, --output` | string | Write output to <file> instead of stdout. |  |
| `-n, --no-headers` | flag | When set, the first row will NOT be included in the frequency table. Additionally, the 'field' column will be 1-based indices instead of header names. |  |
| `-d, --delimiter` | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |
| `--memcheck` | flag | Check if there is enough memory to load the entire CSV into memory using CONSERVATIVE heuristics. |  |

---
**Source:** [`src/cmd/frequency.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/frequency.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
