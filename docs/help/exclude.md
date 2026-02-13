# exclude

> Removes a set of CSV data from another set based on the specified columns.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/exclude.rs](https://github.com/dathere/qsv/blob/master/src/cmd/exclude.rs)** | 📇👆

<a name="nav"></a>
[Description](#description) | [Examples](#examples) | [Usage](#usage) | [Arguments](#arguments) | [Exclude Options](#exclude-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Removes a set of CSV data from another set based on the specified columns.

Also can compute the intersection of two CSV sets with the -v flag.

Matching is always done by ignoring leading and trailing whitespace. By default,
matching is done case sensitively, but this can be disabled with the --ignore-case
flag.

The columns arguments specify the columns to match for each input. Columns can
be referenced by name or index, starting at 1. Specify multiple columns by
separating them with a comma. Specify a range of columns with `-`. Both
columns1 and columns2 must specify exactly the same number of columns.
(See 'qsv select --help' for the full syntax.)


<a name="examples"></a>

## Examples [↩](#nav)

> Remove all records in previously-processed.csv from records.csv

```console
qsv exclude id records.csv id previously-processed.csv
```

> Remove all records in previously-processed.csv matching on multiple columns

```console
qsv exclude col1,col2 records.csv col1,col2 previously-processed.csv
```

> Remove all records in previously-processed.csv matching on column ranges

```console
qsv exclude col1-col5 records.csv col1-col5 previously-processed.csv
```

> Remove all records in previously-processed.csv with the same id from records.csv

> and write to new-records.csv

```console
qsv exclude id records.csv id previously-processed.csv > new-records.csv
```

> Remove all records in previously-processed.csv with the same id from records.csv

> and write to new-records.csv

```console
qsv exclude id records.csv id previously-processed.csv --output new-records.csv
```

> Get the intersection of records.csv and previously-processed.csv on id column

> (i.e., only records present in both files)

```console
qsv exclude -v id records.csv id previously-processed.csv -o intersection.csv
```

> Do a case insensitive exclusion on the id column

```console
qsv exclude --ignore-case id records.csv id previously-processed.csv
```

> Chain exclude with sort to create a new sorted records file without previously processed records

```console
qsv exclude id records.csv id previously-processed.csv | \
qsv sort > new-sorted-records.csv
```

> Chain exclude with sort and dedup to create a new sorted deduped records file

```console
qsv exclude id records.csv id previously-processed.csv | qsv sort | \
qsv --sorted dedup > new-sorted-deduped-records.csv
```

For more examples, see [tests](https://github.com/dathere/qsv/blob/master/tests/test_exclude.rs).


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv exclude [options] <columns1> <input1> <columns2> <input2>
qsv exclude --help
```

<a name="arguments"></a>

## Arguments [↩](#nav)

| &nbsp;Argument&nbsp; | Description |
|----------|-------------|
| &nbsp;`<input1>`&nbsp; | is the file from which data will be removed. |
| &nbsp;`<input2>`&nbsp; | is the file containing the data to be removed from <input1> e.g. 'qsv exclude id records.csv id previously-processed.csv' |

<a name="exclude-options"></a>

## Exclude Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-i,`<br>`--ignore-case`&nbsp; | flag | When set, matching is done case insensitively. |  |
| &nbsp;`-v,`<br>`--invert`&nbsp; | flag | When set, matching rows will be the only ones included, forming set intersection, instead of the ones discarded. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-h,`<br>`--help`&nbsp; | flag | Display this message |  |
| &nbsp;`-o,`<br>`--output`&nbsp; | string | Write output to <file> instead of stdout. |  |
| &nbsp;`-n,`<br>`--no-headers`&nbsp; | flag | When set, the first row will not be interpreted as headers. (i.e., They are not searched, analyzed, sliced, etc.) |  |
| &nbsp;`-d,`<br>`--delimiter`&nbsp; | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |

---
**Source:** [`src/cmd/exclude.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/exclude.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
