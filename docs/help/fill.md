# fill

> Fill empty values.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/fill.rs](https://github.com/dathere/qsv/blob/master/src/cmd/fill.rs)** | 👆

<a name="nav"></a>
[Description](#description) | [Usage](#usage) | [Fill Options](#fill-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Fill empty fields in selected columns of a CSV.

This command fills empty fields in the selected column
using the last seen non-empty field in the CSV. This is
useful to forward-fill values which may only be included
the first time they are encountered.

The option `--default <value>` fills all empty values
in the selected columns with the provided default value.

The option `--first` fills empty values using the first
seen non-empty value in that column, instead of the most
recent non-empty value in that column.

The option `--backfill` fills empty values at the start of
the CSV with the first valid value in that column. This
requires buffering rows with empty values in the target
column which appear before the first valid value.

The option `--groupby` groups the rows by the specified
columns before filling in the empty values. Using this
option, empty values are only filled with values which
belong to the same group of rows, as determined by the
columns selected in the `--groupby` option.

When both `--groupby` and `--backfill` are specified, and the
CSV is not sorted by the `--groupby` columns, rows may be
re-ordered during output due to the buffering of rows
collected before the first valid value.

For examples, see <https://github.com/dathere/qsv/blob/master/tests/test_fill.rs>.


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv fill [options] [--] <selection> [<input>]
qsv fill --help
```

<a name="fill-options"></a>

## Fill Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-g,`<br>`--groupby`&nbsp; | string | Group by specified columns. |  |
| &nbsp;`-f,`<br>`--first`&nbsp; | flag | Fill using the first valid value of a column, instead of the latest. |  |
| &nbsp;`-b,`<br>`--backfill`&nbsp; | flag | Fill initial empty values with the first valid value. |  |
| &nbsp;`-v,`<br>`--default`&nbsp; | string | Fill using this default value. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-h,`<br>`--help`&nbsp; | flag | Display this message |  |
| &nbsp;`-o,`<br>`--output`&nbsp; | string | Write output to <file> instead of stdout. |  |
| &nbsp;`-n,`<br>`--no-headers`&nbsp; | flag | When set, the first row will not be interpreted as headers. (i.e., They are not searched, analyzed, sliced, etc.) |  |
| &nbsp;`-d,`<br>`--delimiter`&nbsp; | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |

---
**Source:** [`src/cmd/fill.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/fill.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
