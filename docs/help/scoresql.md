# scoresql

> Analyze a SQL query against CSV file caches (stats, moarstats, frequency) to produce a performance score with actionable optimization suggestions BEFORE running the query. Supports Polars (default) and DuckDB modes.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/scoresql.rs](https://github.com/dathere/qsv/blob/master/src/cmd/scoresql.rs)** | [🐻‍❄️](TableOfContents.md#legend "command powered/accelerated by  vectorized query engine.")[🪄](TableOfContents.md#legend "\"automagical\" commands that uses stats and/or frequency tables to work \"smarter\" & \"faster\".")

<a name="nav"></a>
[Description](#description) | [Examples](#examples) | [Usage](#usage) | [Scoresql Options](#scoresql-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Analyze a SQL query against CSV file caches (stats, moarstats, frequency) to produce a
performance score with actionable optimization suggestions BEFORE running the query.

Accepts the same input/SQL arguments as sqlp. Outputs a human-readable performance report
(default) or JSON (--json). Supports Polars mode (default) and DuckDB mode (--duckdb).

Scoring factors include:  
* Query plan analysis (EXPLAIN output from Polars or DuckDB)
* Type optimization (column types vs. usage in query)
* Join key cardinality and data distribution
* Filter selectivity from frequency cache
* Query anti-pattern detection (SELECT *, missing LIMIT, cartesian joins, etc.)
* Infrastructure checks (index files, cache freshness)

Caches are auto-generated when missing:  
* stats cache via `qsv stats --everything --stats-jsonl`
* frequency cache via `qsv frequency --frequency-jsonl`


<a name="examples"></a>

## Examples [↩](#nav)

> Score a simple filter query against a single CSV file

```console
qsv scoresql data.csv "SELECT * FROM data WHERE col1 > 10"
```

> Output the score report as JSON instead of the default human-readable format

```console
qsv scoresql --json data.csv "SELECT col1, col2 FROM data ORDER BY col1"
```

> Score a join query across two CSV files

```console
qsv scoresql data.csv data2.csv "SELECT * FROM data JOIN data2 ON data.id = data2.id"
```

> Use DuckDB for query plan analysis instead of Polars

```console
qsv scoresql --duckdb data.csv "SELECT * FROM data WHERE status = 'active'"
```

> Use _t_N aliases just like sqlp (see sqlp documentation)

```console
qsv scoresql data.csv data2.csv "SELECT * FROM _t_1 JOIN _t_2 ON _t_1.id = _t_2.id"
```

> Score a query from a SQL script file (only the last query is scored)

```console
qsv scoresql data.csv script.sql
```

For more examples, see [tests](https://github.com/dathere/qsv/blob/master/tests/test_scoresql.rs).


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv scoresql [options] <input>... <sql>
qsv scoresql --help
```

<a name="scoresql-options"></a>

## Scoresql Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑‑json`&nbsp; | flag | Output results as JSON instead of human-readable report. |  |
| &nbsp;`‑‑duckdb`&nbsp; | flag | Use DuckDB for query plan analysis instead of Polars. Uses the QSV_DUCKDB_PATH environment variable if set, otherwise looks for "duckdb" in PATH. |  |
| &nbsp;`‑‑try‑parsedates`&nbsp; | flag | Automatically try to parse dates/datetimes and time. |  |
| &nbsp;`‑‑infer‑len`&nbsp; | string | Number of rows to scan when inferring schema. | `10000` |
| &nbsp;`‑‑ignore‑errors`&nbsp; | flag | Ignore errors when parsing CSVs. |  |
| &nbsp;`‑‑truncate‑ragged‑lines`&nbsp; | flag | Truncate lines with more fields than the header. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑h,`<br>`‑‑help`&nbsp; | flag | Display this message |  |
| &nbsp;`‑o,`<br>`‑‑output`&nbsp; | string | Write output to <file> instead of stdout. |  |
| &nbsp;`‑d,`<br>`‑‑delimiter`&nbsp; | string | The field delimiter for reading CSV data. Must be a single character. | `,` |
| &nbsp;`‑q,`<br>`‑‑quiet`&nbsp; | flag | Do not print informational messages to stderr. |  |

---
**Source:** [`src/cmd/scoresql.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/scoresql.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
