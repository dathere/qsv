# dedup

> Remove duplicate rows (See also `extdedup`, `extsort`, `sort` & `sortcheck` commands).

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/dedup.rs](https://github.com/dathere/qsv/blob/master/src/cmd/dedup.rs)** | 🤯🚀👆

<a name="nav"></a>
[Description](#description) | [Examples](#examples) | [Usage](#usage) | [Dedup Options](#dedup-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Deduplicates CSV rows.

This requires reading all of the CSV data into memory because because the rows need
to be sorted first.

That is, unless the --sorted option is used to indicate the CSV is already sorted -
typically, with the sort cmd for more sorting options or the extsort cmd for larger
than memory CSV files. This will make dedup run in streaming mode with constant memory.

Either way, the output will not only be deduplicated, it will also be sorted.

A duplicate count will also be sent to <stderr>.


<a name="examples"></a>

## Examples [↩](#nav)

> Deduplicate an unsorted CSV file:

```console
qsv dedup unsorted.csv -o deduped.csv
```

> Deduplicate a sorted CSV file:

```console
qsv sort unsorted.csv | qsv dedup --sorted -o deduped.csv
```

> Deduplicate based on specific columns:

```console
qsv dedup --select col1,col2 unsorted.csv -o deduped.csv
```

> Deduplicate based on numeric comparison of col1 and col2 columns:

```console
qsv dedup -s col1,col2 --numeric unsorted.csv -o deduped.csv
```

> Deduplicate ignoring case of col1 and col2 columns:

```console
qsv dedup -s col1,col2 --ignore-case unsorted.csv -o deduped.csv
```

> Write duplicates to a separate file:

```console
qsv dedup -s col1,col2 --dupes-output dupes.csv unsorted.csv -o deduped.csv
```

For more examples, see [tests](https://github.com/dathere/qsv/blob/master/tests/test_dedup.rs).


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv dedup [options] [<input>]
qsv dedup --help
```

<a name="dedup-options"></a>

## Dedup Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-s,`<br>`--select`&nbsp; | string | Select a subset of columns to dedup. Note that the outputs will remain at the full width of the CSV. See 'qsv select --help' for the format details. |  |
| &nbsp;`-N,`<br>`--numeric`&nbsp; | flag | Compare according to string numerical value |  |
| &nbsp;`-i,`<br>`--ignore-case`&nbsp; | flag | Compare strings disregarding case. |  |
| &nbsp;`--sorted`&nbsp; | flag | The input is already sorted. Do not load the CSV into memory to sort it first. Meant to be used in tandem and after an extsort. |  |
| &nbsp;`-D,`<br>`--dupes-output`&nbsp; | string | Write duplicates to <file>. |  |
| &nbsp;`-H,`<br>`--human-readable`&nbsp; | flag | Comma separate duplicate count. |  |
| &nbsp;`-j,`<br>`--jobs`&nbsp; | string | The number of jobs to run in parallel when sorting an unsorted CSV, before deduping. When not set, the number of jobs is set to the number of CPUs detected. Does not work with --sorted option as its not multithreaded. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-h,`<br>`--help`&nbsp; | flag | Display this message |  |
| &nbsp;`-o,`<br>`--output`&nbsp; | string | Write output to <file> instead of stdout. |  |
| &nbsp;`-n,`<br>`--no-headers`&nbsp; | flag | When set, the first row will not be interpreted as headers. That is, it will be sorted with the rest of the rows. Otherwise, the first row will always appear as the header row in the output. |  |
| &nbsp;`-d,`<br>`--delimiter`&nbsp; | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |
| &nbsp;`-q,`<br>`--quiet`&nbsp; | flag | Do not print duplicate count to stderr. |  |
| &nbsp;`--memcheck`&nbsp; | flag | Check if there is enough memory to load the entire CSV into memory using CONSERVATIVE heuristics. |  |

---
**Source:** [`src/cmd/dedup.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/dedup.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
