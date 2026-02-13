# select

> Select, re-order, reverse, duplicate or drop columns.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/select.rs](https://github.com/dathere/qsv/blob/master/src/cmd/select.rs)** | 👆

<a name="nav"></a>
[Description](#description) | [Examples](#examples) | [Usage](#usage) | [Arguments](#arguments) | [Select Options](#select-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Select columns from CSV data efficiently.

This command lets you manipulate the columns in CSV data. You can re-order,
duplicate, reverse or drop them. Columns can be referenced by index or by
name if there is a header row (duplicate column names can be disambiguated with
more indexing). Column ranges can also be specified. Finally, columns can be
selected using regular expressions.


<a name="examples"></a>

## Examples [↩](#nav)

> Select the first and fourth columns

```console
qsv select 1,4
```

> Select the first 4 columns (by index)

```console
qsv select 1-4
```

> Select the first 4 columns (by name)

```console
qsv select Header1-Header4
```

> Ignore the first 2 columns (by range)

```console
qsv select 3-
```

> Ignore the first 2 columns (by index)

```console
qsv select '!1-2'
```

> Select the third column named 'Foo':

```console
qsv select 'Foo[2]'
```

> Select the first and last columns, _ is a special character for the last column:

```console
qsv select 1,_
```

> Reverse the order of columns:

```console
qsv select _-1
```

> select columns starting with 'a' (regex)

```console
qsv select /^a/
```

> select columns with a digit (regex)

```console
qsv select '/^.*\d.*$/'
```

> remove SSN, account_no and password columns (regex)

```console
qsv select '!/SSN|account_no|password/'
```

> Sort the columns lexicographically (i.e. by their byte values)

```console
qsv select 1- --sort
```

> Select some columns and then sort them

```console
qsv select 1,4,5-7 --sort
```

> Randomly shuffle the columns:

```console
qsv select 1- --random
```

> Randomly shuffle the columns with a seed

```console
qsv select 1- --random --seed 42
```

> Select some columns and then shuffle them with a seed:

```console
qsv select 1,4,5-7 --random --seed 42
```

> Re-order and duplicate columns arbitrarily using different types of selectors

```console
qsv select 3-1,Header3-Header1,Header1,Foo[2],Header1
```

> Quote column names that conflict with selector syntax:

```console
qsv select '\"Date - Opening\",\"Date - Actual Closing\"'
```

For more examples, see [tests](https://github.com/dathere/qsv/blob/master/tests/test_select.rs).


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv select [options] [--] <selection> [<input>]
qsv select --help
```

<a name="arguments"></a>

## Arguments [↩](#nav)

| &nbsp;&nbsp;Argument&nbsp;&nbsp;&nbsp; | Description |
|----------|-------------|
| &nbsp;`<selection>`&nbsp; | The columns to select. You can select columns by index, by name, by range, by regex and any combination of these. If the first character is '!', the selection will be inverted. If the selection contains embedded spaces or characters that conflict with selector syntax, it must be quoted. See examples above. |

<a name="select-options"></a>

## Select Options [↩](#nav)

| &nbsp;&nbsp;Option&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-R,`<br>`--random`&nbsp; | flag | Randomly shuffle the columns in the selection. |  |
| &nbsp;`--seed`&nbsp; | string | Seed for the random number generator. |  |
| &nbsp;`-S,`<br>`--sort`&nbsp; | flag | Sort the selected columns lexicographically, i.e. by their byte values. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-h,`<br>`--help`&nbsp; | flag | Display this message |  |
| &nbsp;`-o,`<br>`--output`&nbsp; | string | Write output to <file> instead of stdout. |  |
| &nbsp;`-n,`<br>`--no-headers`&nbsp; | flag | When set, the first row will not be interpreted as headers. (i.e., They are not searched, analyzed, sliced, etc.) |  |
| &nbsp;`-d,`<br>`--delimiter`&nbsp; | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |

---
**Source:** [`src/cmd/select.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/select.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
