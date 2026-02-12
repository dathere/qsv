# tojsonl

📇😣🚀🔣🪄🗃️

> Smartly converts CSV to a newline-delimited JSON (JSONL/NDJSON). By scanning the CSV first, it "smartly" infers the appropriate JSON data type for each column. See `jsonl` command to convert JSONL to CSV.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/tojsonl.rs](https://github.com/dathere/qsv/blob/master/src/cmd/tojsonl.rs)**

## Description

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


## Usage

```console
qsv tojsonl [options] [<input>]
qsv tojsonl --help
```

## Tojsonl Options

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `--trim` | flag | Trim leading and trailing whitespace from fields before converting to JSON. |  |
| `--no-boolean` | flag | Do not infer boolean fields. |  |
| `-j, --jobs` | string | The number of jobs to run in parallel. When not set, the number of jobs is set to the number of CPUs detected. |  |
| `-b, --batch` | string | The number of rows per batch to load into memory, before running in parallel. Automatically determined for CSV files with more than 50000 rows. Set to 0 to load all rows in one batch. Set to 1 to force batch optimization even for files with less than 50000 rows. | `50000` |

## Common Options

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-h, --help` | flag | Display this message |  |
| `-d, --delimiter` | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |
| `-o, --output` | string | Write output to <file> instead of stdout. |  |
| `--memcheck` | flag | Check if there is enough memory to load the entire CSV into memory using CONSERVATIVE heuristics. |  |
| `-q, --quiet` | flag | Do not display enum/const list inferencing messages. |  |

---
**Source:** [`src/cmd/tojsonl.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/tojsonl.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
