# tojsonl

> Smartly converts CSV to a newline-delimited JSON ([JSONL](https://jsonlines.org/)/[NDJSON](http://ndjson.org/)). By scanning the CSV first, it "smartly" infers the appropriate JSON data type for each column. See `jsonl` command to convert JSONL to CSV.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/tojsonl.rs](https://github.com/dathere/qsv/blob/master/src/cmd/tojsonl.rs)** | <abbr title="uses an index when available.">📇</abbr><abbr title="uses additional memory proportional to the cardinality of the columns in the CSV.">😣</abbr><abbr title="multithreaded even without an index.">🚀</abbr><abbr title="requires UTF-8 encoded input.">🔣</abbr><abbr title="&quot;automagical&quot; commands that uses stats and/or frequency tables to work &quot;smarter&quot; & &quot;faster&quot;.">🪄</abbr><abbr title="Limited Extended input support.">🗃️</abbr>

<a name="nav"></a>
[Description](#description) | [Usage](#usage) | [Tojsonl Options](#tojsonl-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Smartly converts CSV to a newline-delimited JSON (JSONL/NDJSON).

By computing stats on the CSV first, it "smartly" infers the appropriate JSON data type
for each column (string, number, boolean, null).

It will infer a column as boolean if its cardinality is 2, and the first character of
the values are one of the following case-insensitive combinations:
t/f; t/null; 1/0; 1/null; y/n & y/null are treated as true/false.

The `tojsonl` command will reuse a `stats.csv.data.jsonl` file if it exists and is
current (i.e. stats generated with --cardinality and --infer-dates options) and will
skip recomputing stats.

For examples, see <https://github.com/dathere/qsv/blob/master/tests/test_tojsonl.rs>.


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv tojsonl [options] [<input>]
qsv tojsonl --help
```

<a name="tojsonl-options"></a>

## Tojsonl Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`--trim`&nbsp; | flag | Trim leading and trailing whitespace from fields before converting to JSON. |  |
| &nbsp;`--no-boolean`&nbsp; | flag | Do not infer boolean fields. |  |
| &nbsp;`-j,`<br>`--jobs`&nbsp; | string | The number of jobs to run in parallel. When not set, the number of jobs is set to the number of CPUs detected. |  |
| &nbsp;`-b,`<br>`--batch`&nbsp; | string | The number of rows per batch to load into memory, before running in parallel. Automatically determined for CSV files with more than 50000 rows. Set to 0 to load all rows in one batch. Set to 1 to force batch optimization even for files with less than 50000 rows. | `50000` |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-h,`<br>`--help`&nbsp; | flag | Display this message |  |
| &nbsp;`-d,`<br>`--delimiter`&nbsp; | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |
| &nbsp;`-o,`<br>`--output`&nbsp; | string | Write output to <file> instead of stdout. |  |
| &nbsp;`--memcheck`&nbsp; | flag | Check if there is enough memory to load the entire CSV into memory using CONSERVATIVE heuristics. |  |
| &nbsp;`-q,`<br>`--quiet`&nbsp; | flag | Do not display enum/const list inferencing messages. |  |

---
**Source:** [`src/cmd/tojsonl.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/tojsonl.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
