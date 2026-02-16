# jsonl

> Convert newline-delimited JSON ([JSONL](https://jsonlines.org/)/[NDJSON](http://ndjson.org/)) to CSV. See `tojsonl` command to convert CSV to JSONL.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/jsonl.rs](https://github.com/dathere/qsv/blob/master/src/cmd/jsonl.rs)** | 🚀🔣

<a name="nav"></a>
[Description](#description) | [Usage](#usage) | [JSONL Options](#jsonl-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Convert newline-delimited JSON (JSONL/NDJSON) to CSV.

The command tries to do its best but since it is not possible to
straightforwardly convert JSON lines to CSV, the process might lose some complex
fields from the input.

Also, it will fail if the JSON documents are not consistent with one another,
as the first JSON line will be used to infer the headers of the CSV output.

For examples, see <https://github.com/dathere/qsv/blob/master/tests/test_jsonl.rs>.


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv jsonl [options] [<input>]
qsv jsonl --help
```

<a name="jsonl-options"></a>

## JSONL Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`--ignore-errors`&nbsp; | flag | Skip malformed input lines. |  |
| &nbsp;`-j,`<br>`--jobs`&nbsp; | string | The number of jobs to run in parallel. When not set, the number of jobs is set to the number of CPUs detected. |  |
| &nbsp;`-b,`<br>`--batch`&nbsp; | string | The number of rows per batch to load into memory, before running in parallel. Set to 0 to load all rows in one batch. | `50000` |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-h,`<br>`--help`&nbsp; | flag | Display this message |  |
| &nbsp;`-o,`<br>`--output`&nbsp; | string | Write output to <file> instead of stdout. |  |
| &nbsp;`-d,`<br>`--delimiter`&nbsp; | string | The delimiter to use when writing CSV data. Must be a single character. | `,` |

---
**Source:** [`src/cmd/jsonl.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/jsonl.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
