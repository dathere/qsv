# rename

> Rename the columns of a CSV efficiently.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/rename.rs](https://github.com/dathere/qsv/blob/master/src/cmd/rename.rs)**

<a name="nav"></a>
[Description](#description) | [Examples](#examples) | [Usage](#usage) | [Arguments](#arguments) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Rename the columns of a CSV efficiently. It has two modes of operation:

Positional mode (default):
The new column names are given as a comma-separated list of names.
The number of column names given MUST match the number of columns in the
CSV unless "_all_generic" is used.

Pairwise mode:
The new column names are given as a comma-separated list of pairs of old and new
column names. The format is "old1,new1,old2,new2,...".


<a name="examples"></a>

## Examples [↩](#nav)

Change the column names of a CSV with three columns:
```console
qsv rename id,name,title
```

Rename only specific columns using pairs:
```console
qsv rename --pairwise oldname,newname,oldcol,newcol
```

Replace the column names with generic ones (_col_N):
```console
qsv rename _all_generic
```

Add generic column names to a CSV with no headers:
```console
qsv rename _all_generic --no-headers
```

Use column names that contains commas and conflict with the separator:
```console
qsv rename '"Date - Opening","Date - Actual Closing"'
```

For more examples, see [tests](https://github.com/dathere/qsv/blob/master/tests/test_rename.rs).


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv rename [options] [--] <headers> [<input>]
qsv rename --help
```

<a name="arguments"></a>

## Arguments [↩](#nav)

| &nbsp;Argument&nbsp;&nbsp; | Description |
|----------|-------------|
| &nbsp;`<headers>`&nbsp; | The new headers to use for the CSV. Separate multiple headers with a comma. If "_all_generic" is given, the headers will be renamed to generic column names, where the column name uses the format "_col_N" where N is the 1-based column index. Alternatively, specify pairs of old,new column names to rename only specific columns. |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-h,`<br>`--help`&nbsp; | flag | Display this message |  |
| &nbsp;`-o,`<br>`--output`&nbsp; | string | Write output to <file> instead of stdout. |  |
| &nbsp;`-n,`<br>`--no-headers`&nbsp; | flag | When set, the header will be inserted on top. |  |
| &nbsp;`-d,`<br>`--delimiter`&nbsp; | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |

---
**Source:** [`src/cmd/rename.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/rename.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
