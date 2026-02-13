# sortcheck

📇👆

> Check if a CSV is sorted. With the --json options, also retrieve record count, sort breaks & duplicate count.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/sortcheck.rs](https://github.com/dathere/qsv/blob/master/src/cmd/sortcheck.rs)**

<a name="nav"></a>
[Description](#description) | [Usage](#usage) | [Sort Options](#sort-options) | [Common Options](#common-options)

## Description [↩](#nav)

Check if a CSV is sorted. The check is done on a streaming basis (i.e. constant memory).
With the --json options, also retrieve record count, sort breaks & duplicate count.

This command can be used in tandem with other qsv commands that sort or require sorted data
to ensure that they also work on a stream of data - i.e. without loading an entire CSV into memory.

For instance, a naive `dedup` requires loading the entire CSV into memory to sort it
first before deduping. However, if you know a CSV is sorted beforehand, you can invoke
`dedup` with the --sorted option, and it will skip loading entire CSV into memory to sort
it first. It will just immediately dedupe on a streaming basis.

`sort` also requires loading the entire CSV into memory. For simple "sorts" (not numeric,
reverse, unique & random sorts), particularly of very large CSV files that will not fit in memory,
`extsort` - a multi-threaded streaming sort that is exponentially faster and can work with
arbitrarily large files, can be used instead.

Simply put, sortcheck allows you to make informed choices on how to compose pipelines that
require sorted data.

Returns exit code 0 if a CSV is sorted, and exit code 1 otherwise.

For examples, see <https://github.com/dathere/qsv/blob/master/tests/test_sortcheck.rs>.


## Usage [↩](#nav)

```console
qsv sortcheck [options] [<input>]
qsv sortcheck --help
```

## Sort Options [↩](#nav)

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-s, --select` | string | Select a subset of columns to check for sort. See 'qsv select --help' for the format details. |  |
| `-i, --ignore-case` | flag | Compare strings disregarding case |  |
| `--all` | flag | Check all records. Do not stop/short-circuit the check on the first unsorted record. |  |
| `--json` | flag | Return results in JSON format, scanning --all records. The JSON result has the following properties - sorted (boolean), record_count (number), unsorted_breaks (number) & dupe_count (number). Unsorted breaks count the number of times two consecutive rows are unsorted (i.e. n row > n+1 row). Dupe count is the number of times two consecutive rows are equal. Note that dupe count does not apply if the file is not sorted and is set to -1. |  |
| `--pretty-json` | flag | Same as --json but in pretty JSON format. |  |

## Common Options [↩](#nav)

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-h, --help` | flag | Display this message |  |
| `-n, --no-headers` | flag | When set, the first row will not be interpreted as headers. That is, it will be sorted with the rest of the rows. Otherwise, the first row will always appear as the header row in the output. |  |
| `-d, --delimiter` | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |
| `-p, --progressbar` | flag | Show progress bars. Not valid for stdin. |  |

---
**Source:** [`src/cmd/sortcheck.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/sortcheck.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
