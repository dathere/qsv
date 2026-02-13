# joinp

> Inner, outer, right, cross, anti, semi, non-equi & asof joins using the Pola.rs engine. Unlike the `join` command, `joinp` can process files larger than RAM, is multithreaded, has join key validation, a maintain row order option, pre and post-join filtering, join keys unicode normalization, supports "special" non-equi joins and asof joins (which is particularly useful for time series data) & its output columns can be coalesced.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/joinp.rs](https://github.com/dathere/qsv/blob/master/src/cmd/joinp.rs)** | 🚀🐻‍❄️🪄

<a name="nav"></a>
[Description](#description) | [Usage](#usage) | [Joinp Options](#joinp-options) | [Join Options](#join-options) | [Polars CSV Parsing Options](#polars-csv-parsing-options) | [Asof Join Options](#asof-join-options) | [Output Format Options](#output-format-options) | [Join Key Transformation Options](#join-key-transformation-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Joins two sets of CSV data on the specified columns using the Polars engine.

The default join operation is an 'inner' join. This corresponds to the
intersection of rows on the keys specified.

Unlike the join command, joinp can process files larger than RAM, is multithreaded,
has join key validation, a maintain row order option, pre-join filtering, supports
non-equi & asof joins and its output columns can be coalesced (no duplicate columns).

Returns the shape of the join result (number of rows, number of columns) to stderr.

For examples, see <https://github.com/dathere/qsv/blob/master/tests/test_joinp.rs>.


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv joinp [options] <columns1> <input1> <columns2> <input2>
qsv joinp --cross [--validate <arg>] <input1> <input2> [--decimal-comma] [--delimiter <arg>] [--output <file>]
qsv joinp --non-equi <expr> <input1> <input2> [options] [--output <file>]
qsv joinp --help
```

<a name="joinp-options"></a>

## Joinp Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`--left`&nbsp; | flag | Do a 'left outer' join. This returns all rows in first CSV data set, including rows with no corresponding row in the second data set. When no corresponding row exists, it is padded out with empty fields. |  |
| &nbsp;`--left-anti`&nbsp; | flag | This returns only the rows in the first CSV data set that do not have a corresponding row in the second data set. The output schema is the same as the first dataset. |  |
| &nbsp;`--left-semi`&nbsp; | flag | This returns only the rows in the first CSV data set that have a corresponding row in the second data set. The output schema is the same as the first data set. |  |
| &nbsp;`--right`&nbsp; | flag | Do a 'right outer' join. This returns all rows in second CSV data set, including rows with no corresponding row in the first data set. When no corresponding row exists, it is padded out with empty fields. (This is the reverse of 'outer left'.) |  |
| &nbsp;`--right-anti`&nbsp; | flag | This returns only the rows in the second CSV data set that do not have a corresponding row in the first data set. The output schema is the same as the second dataset. |  |
| &nbsp;`--right-semi`&nbsp; | flag | This returns only the rows in the second CSV data set that have a corresponding row in the first data set. The output schema is the same as the second data set. |  |
| &nbsp;`--full`&nbsp; | flag | Do a 'full outer' join. This returns all rows in both data sets with matching records joined. If there is no match, the missing side will be padded out with empty fields. |  |
| &nbsp;`--cross`&nbsp; | flag | USE WITH CAUTION. This returns the cartesian product of the CSV data sets given. The number of rows return is equal to N * M, where N and M correspond to the number of rows in the given data sets, respectively. The columns1 and columns2 arguments are ignored. |  |
| &nbsp;`--non-equi`&nbsp; | string | Do a non-equi join. The given expression is evaluated for each row in the left dataset and can refer to columns in the left and right dataset. If the expression evaluates to true, the row is joined with the corresponding row in the right dataset. The expression is a valid Polars SQL where clause, with each column name followed by "_left" or "_right" suffixes to indicate which data set the column belongs to. (e.g. "salary_left >= min_salary_right AND \ salary_left <= max_salary_right AND \ experience_left >= min_exp_right") |  |
| &nbsp;`--coalesce`&nbsp; | flag | Force the join to coalesce columns with the same name. For inner joins, this is not necessary as the join columns are automatically coalesced. |  |
| &nbsp;`--filter-left`&nbsp; | string | Filter the left CSV data set by the given Polars SQL expression BEFORE the join. Only rows that evaluates to true are used in the join. |  |
| &nbsp;`--filter-right`&nbsp; | string | Filter the right CSV data set by the given Polars SQL expression BEFORE the join. Only rows that evaluates to true are used in the join. |  |
| &nbsp;`--validate`&nbsp; | string | Validate the join keys BEFORE performing the join. | `none` |

<a name="join-options"></a>

## Join Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`--maintain-order`&nbsp; | string | Which row order to preserve, if any. Valid values are: none, left, right, left_right, right_left Do not rely on any observed ordering without explicitly setting this parameter. Not specifying any order can improve performance. Supported for inner, left, right and full joins. | `none` |
| &nbsp;`--nulls`&nbsp; | flag | When set, joins will work on empty fields. Otherwise, empty fields are completely ignored. |  |
| &nbsp;`--streaming`&nbsp; | flag | When set, the join will be done in a streaming fashion. Only use this when you get out of memory errors. |  |

<a name="polars-csv-parsing-options"></a>

## Polars CSV Parsing Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`--try-parsedates`&nbsp; | flag | When set, will attempt to parse the columns as dates. If the parse fails, columns remain as strings. This is useful when the join keys are formatted as dates with differing date formats, as the date formats will be normalized. Note that this will be automatically enabled when using asof joins. |  |
| &nbsp;`--infer-len`&nbsp; | string | The number of rows to scan when inferring the schema of the CSV. Set to 0 to do a full table scan (warning: very slow). Only used when --cache-schema is 0 or 1 and no cached schema exists or when --infer-len is 0. | `10000` |
| &nbsp;`--cache-schema`&nbsp; | string | Create and cache Polars schema JSON files. Ignored when --infer-len is 0. ‎ -2: treat all columns as String. A Polars schema file is created & cached. ‎ -1: treat all columns as String. No Polars schema file is created. 0: do not cache Polars schema. Uses --infer-len to infer schema. 1: cache Polars schema with the following behavior: * If schema file exists and is newer than input: use cached schema * If schema file missing/outdated and stats cache exists: derive schema from stats and cache it * If no schema or stats cache: infer schema using --infer-len and cache the result Schema files use the same name as input with .pschema.json extension (e.g., data.csv -> data.pschema.json) NOTE: If the input files have pschema.json files that are newer or created at the same time as the input files, they will be used to inform the join operation regardless of the value of --cache-schema unless --infer-len is 0. | `0` |
| &nbsp;`--low-memory`&nbsp; | flag | Use low memory mode when parsing CSVs. This will use less memory but will be slower. It will also process the join in streaming mode. Only use this when you get out of memory errors. |  |
| &nbsp;`--no-optimizations`&nbsp; | flag | Disable non-default join optimizations. This will make joins slower. Only use this when you get join errors. |  |
| &nbsp;`--ignore-errors`&nbsp; | flag | Ignore errors when parsing CSVs. If set, rows with errors will be skipped. If not set, the query will fail. Only use this when debugging queries, as polars does batched parsing and will skip the entire batch where the error occurred. To get more detailed error messages, set the environment variable POLARS_BACKTRACE_IN_ERR=1 before running the join. |  |
| &nbsp;`--decimal-comma`&nbsp; | flag | Use comma as the decimal separator when parsing & writing CSVs. Otherwise, use period as the decimal separator. Note that you'll need to set --delimiter to an alternate delimiter other than the default comma if you are using this option. |  |

<a name="asof-join-options"></a>

## Asof Join Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`--asof`&nbsp; | flag | Do an 'asof' join. This is similar to a left inner join, except we match on nearest key rather than equal keys (see --allow-exact-matches). Particularly useful for time series data. Note that both CSV data sets will be SORTED on the join columns by default, unless --no-sort is set. |  |
| &nbsp;`--no-sort`&nbsp; | flag | Do not sort the CSV data sets on the join columns by default. Note that asof joins REQUIRE the join keys to be sorted, so this option should only be used as a performance optimization when you know the CSV join keys are already sorted. If the CSV join keys are not sorted, the asof join will fail or return incorrect results. |  |
| &nbsp;`--left_by`&nbsp; | string | Do an 'asof_by' join - a special implementation of the asof join that searches for the nearest keys within a subgroup set by the asof_by columns. This specifies the column/s for the left CSV. Columns are referenced by name. Specify multiple columns by separating them with a comma. |  |
| &nbsp;`--right_by`&nbsp; | string | Do an 'asof_by' join. This specifies the column/s for the right CSV. |  |
| &nbsp;`--strategy`&nbsp; | string | The strategy to use for the asof join: backward - For each row in the first CSV data set, we find the last row in the second data set whose key is less than or equal to the key in the first data set. forward -  For each row in the first CSV data set, we find the first row in the second data set whose key is greater than or equal to the key in the first data set. nearest -  selects the last row in the second data set whose value is nearest to the value in the first data set. | `backward` |
| &nbsp;`--tolerance`&nbsp; | string | The tolerance for the nearest asof join. This is only used when the nearest strategy is used. The tolerance is a positive integer that specifies the maximum number of rows to search for a match. |  |
| &nbsp;`-X,`<br>`--allow-exact-matches`&nbsp; | flag | When set, the asof join will allow exact matches. (i.e. less-than-or-equal-to or greater-than-or-equal-to) Otherwise, the asof join will only allow nearest matches (strictly less-than or greater-than) by default. |  |

<a name="output-format-options"></a>

## Output Format Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`--sql-filter`&nbsp; | string | The SQL expression to apply against the join result. Used to select columns and filter rows AFTER running the join. Be sure to select from the "join_result" table when formulating the SQL expression. (e.g. "select c1, c2 as colname from join_result where c2 > 20") |  |
| &nbsp;`--datetime-format`&nbsp; | string | The datetime format to use writing datetimes. See <https://docs.rs/chrono/latest/chrono/format/strftime/index.html> for the list of valid format specifiers. |  |
| &nbsp;`--date-format`&nbsp; | string | The date format to use writing dates. |  |
| &nbsp;`--time-format`&nbsp; | string | The time format to use writing times. |  |
| &nbsp;`--float-precision`&nbsp; | string | The number of digits of precision to use when writing floats. (default: 6) |  |
| &nbsp;`--null-value`&nbsp; | string | The string to use when writing null values. (default: <empty string>) |  |

<a name="join-key-transformation-options"></a>

## Join Key Transformation Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-i,`<br>`--ignore-case`&nbsp; | flag | When set, joins are done case insensitively. |  |
| &nbsp;`-z,`<br>`--ignore-leading-zeros`&nbsp; | flag | When set, joins are done ignoring leading zeros. Note that this is only applied to the join keys for both numeric and string columns. Also note that Polars will automatically remove leading zeros from numeric columns when it infers the schema. To force the schema to be all String types, set --cache-schema to -1 or -2. |  |
| &nbsp;`-N,`<br>`--norm-unicode`&nbsp; | string | When set, join keys are Unicode normalized. | `none` |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-h,`<br>`--help`&nbsp; | flag | Display this message |  |
| &nbsp;`-o,`<br>`--output`&nbsp; | string | Write output to <file> instead of stdout. |  |
| &nbsp;`-d,`<br>`--delimiter`&nbsp; | string | The field delimiter for reading/writing CSV data. Must be a single character. (default: ,) |  |
| &nbsp;`-q,`<br>`--quiet`&nbsp; | flag | Do not return join shape to stderr. |  |

---
**Source:** [`src/cmd/joinp.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/joinp.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
