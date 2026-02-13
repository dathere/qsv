# diff

> Find the difference between two CSVs with ludicrous speed! e.g. _compare two CSVs with 1M rows x 9 columns in under 600ms!_

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/diff.rs](https://github.com/dathere/qsv/blob/master/src/cmd/diff.rs)** | 🚀🪄

<a name="nav"></a>
[Description](#description) | [Examples](#examples) | [Usage](#usage) | [Diff Options](#diff-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Find the difference between two CSVs with ludicrous speed.

NOTE: diff does not support stdin. A file path is required for both arguments.
Further, PRIMARY KEY VALUES MUST BE UNIQUE WITHIN EACH CSV.

To check if a CSV has unique primary key values, use `qsv extdedup`
with the same key columns using the `--select` option:

```console
$ qsv extdedup --select keycol data.csv --no-output
```


The duplicate count will be printed to stderr.


<a name="examples"></a>

## Examples [↩](#nav)

> Find the difference between two CSVs

```console
qsv diff left.csv right.csv
```

> Find the difference between two CSVs when the right CSV has no headers

```console
qsv diff left.csv --no-headers-right right-noheaders.csv
```

> Find the difference between two CSVs when the left CSV uses a tab delimiter

```console
qsv diff --delimiter-left '\t' left.csv right-tab.tsv
```

> Find the difference between two CSVs when the left CSV uses a semicolon delimiter

```console
qsv diff --delimiter-left ';' left.csv right-semicolon.csv
```

> Find the difference between two CSVs and write output with tab delimiter to a file

```console
qsv diff -o diff-tab.tsv --delimiter-output '\t' left.csv right.csv
```

> Find the difference between two CSVs and write output with semicolon delimiter to a file

```console
qsv diff -o diff-semicolon.csv --delimiter-output ';' left.csv right.csv
```

> Find the difference comparing records with the same values in the first two columns

```console
qsv diff --key 0,1 left.csv right.csv
```

> Find the difference using first two columns as key and sort result by those columns

```console
qsv diff -k 0,1 --sort-columns 0,1 left.csv right.csv
```

> Find the difference but replace equal field values with empty string (key fields still appear)

```console
qsv diff --drop-equal-fields left.csv right.csv
```

> Find the difference but do not output headers in the result

```console
qsv diff --no-headers-output left.csv right.csv
```

> Find the difference when both CSVs have no headers (generic headers _col_1, _col_2, etc. are used)

```console
qsv diff --no-headers-left --no-headers-right left.csv right.csv
```

For more examples, see [tests](https://github.com/dathere/qsv/blob/master/tests/test_diff.rs).


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv diff [options] [<input-left>] [<input-right>]
qsv diff --help
```

<a name="diff-options"></a>

## Diff Options [↩](#nav)

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`--no-headers-left`&nbsp; | flag | When set, the first row will be considered as part of the left CSV to diff. (When not set, the first row is the header row and will be skipped during the diff. It will always appear in the output.) |  |
| &nbsp;`--no-headers-right`&nbsp; | flag | When set, the first row will be considered as part of the right CSV to diff. (When not set, the first row is the header row and will be skipped during the diff. It will always appear in the output.) |  |
| &nbsp;`--no-headers-output`&nbsp; | flag | When set, the diff result won't have a header row in its output. If not set and both CSVs have no headers, headers in the result will be: _col_1,_col_2, etc. |  |
| &nbsp;`--delimiter-left`&nbsp; | string | The field delimiter for reading CSV data on the left. Must be a single character. (default: ,) |  |
| &nbsp;`--delimiter-right`&nbsp; | string | The field delimiter for reading CSV data on the right. Must be a single character. (default: ,) |  |
| &nbsp;`--delimiter-output`&nbsp; | string | The field delimiter for writing the CSV diff result. Must be a single character. (default: ,) |  |
| &nbsp;`-k,`<br>`--key`&nbsp; | string | The column indices that uniquely identify a record as a comma separated list of 0-based indices, e.g. 0,1,2 or column names, e.g. name,age. Note that when selecting columns by name, only the left CSV's headers are used to match the column names and it is assumed that the right CSV has the same selected column names in the same order as the left CSV. (default: 0) |  |
| &nbsp;`--sort-columns`&nbsp; | string | The column indices by which the diff result should be sorted as a comma separated list of indices, e.g. 0,1,2 or column names, e.g. name,age. Records in the diff result that are marked as "modified" ("delete" and "add" records that have the same key, but have different content) will always be kept together in the sorted diff result and so won't be sorted independently from each other. Note that when selecting columns by name, only the left CSV's headers are used to match the column names and it is assumed that the right CSV has the same selected column names in the same order as the left CSV. |  |
| &nbsp;`--drop-equal-fields`&nbsp; | flag | Drop values of equal fields in modified rows of the CSV diff result (and replace them with the empty string). Key field values will not be dropped. |  |
| &nbsp;`-j,`<br>`--jobs`&nbsp; | string | The number of jobs to run in parallel. When not set, the number of jobs is set to the number of CPUs detected. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-h,`<br>`--help`&nbsp; | flag | Display this message |  |
| &nbsp;`-o,`<br>`--output`&nbsp; | string | Write output to <file> instead of stdout. |  |
| &nbsp;`-d,`<br>`--delimiter`&nbsp; | string | Set ALL delimiters to this character. Overrides --delimiter-right, --delimiter-left and --delimiter-output. |  |

---
**Source:** [`src/cmd/diff.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/diff.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
