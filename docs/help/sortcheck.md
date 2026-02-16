# sortcheck

> Check if a CSV is sorted. With the --json options, also retrieve record count, sort breaks & duplicate count.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/sortcheck.rs](https://github.com/dathere/qsv/blob/master/src/cmd/sortcheck.rs)** | <abbr title="uses an index when available.">📇</abbr><abbr title="has powerful column selector support. See `select` for syntax.">👆</abbr>

<a name="nav"></a>
[Description](#description) | [Usage](#usage) | [Sort Options](#sort-options) | [Common Options](#common-options)

<a name="description"></a>

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


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv sortcheck [options] [<input>]
qsv sortcheck --help
```

<a name="sort-options"></a>

## Sort Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-s,`<br>`--select`&nbsp; | string | Select a subset of columns to check for sort. See 'qsv select --help' for the format details. |  |
| &nbsp;`-i,`<br>`--ignore-case`&nbsp; | flag | Compare strings disregarding case |  |
| &nbsp;`--all`&nbsp; | flag | Check all records. Do not stop/short-circuit the check on the first unsorted record. |  |
| &nbsp;`--json`&nbsp; | flag | Return results in JSON format, scanning --all records. The JSON result has the following properties - sorted (boolean), record_count (number), unsorted_breaks (number) & dupe_count (number). Unsorted breaks count the number of times two consecutive rows are unsorted (i.e. n row > n+1 row). Dupe count is the number of times two consecutive rows are equal. Note that dupe count does not apply if the file is not sorted and is set to -1. |  |
| &nbsp;`--pretty-json`&nbsp; | flag | Same as --json but in pretty JSON format. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-h,`<br>`--help`&nbsp; | flag | Display this message |  |
| &nbsp;`-n,`<br>`--no-headers`&nbsp; | flag | When set, the first row will not be interpreted as headers. That is, it will be sorted with the rest of the rows. Otherwise, the first row will always appear as the header row in the output. |  |
| &nbsp;`-d,`<br>`--delimiter`&nbsp; | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |
| &nbsp;`-p,`<br>`--progressbar`&nbsp; | flag | Show progress bars. Not valid for stdin. |  |

---
**Source:** [`src/cmd/sortcheck.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/sortcheck.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
