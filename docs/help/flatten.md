# flatten

> A flattened view of CSV records. Useful for viewing one record at a time. e.g. `qsv slice -i 5 data.csv | qsv flatten`.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/flatten.rs](https://github.com/dathere/qsv/blob/master/src/cmd/flatten.rs)**

<a name="nav"></a>
[Description](#description) | [Usage](#usage) | [Flatten Options](#flatten-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Prints flattened records such that fields are labeled separated by a new line.
This mode is particularly useful for viewing one record at a time. Each
record is separated by a special '#' character (on a line by itself), which
can be changed with the --separator flag.

There is also a condensed view (-c or --condense) that will shorten the
contents of each field to provide a summary view.

For examples, see <https://github.com/dathere/qsv/blob/master/tests/test_flatten.rs>.


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv flatten [options] [<input>]
qsv flatten --help
```

<a name="flatten-options"></a>

## Flatten Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-c,`<br>`--condense`&nbsp; | string | Limits the length of each field to the value specified. If the field is UTF-8 encoded, then <arg> refers to the number of code points. Otherwise, it refers to the number of bytes. |  |
| &nbsp;`-f,`<br>`--field-separator`&nbsp; | string | A string of character to write between a column name and its value. |  |
| &nbsp;`-s,`<br>`--separator`&nbsp; | string | A string of characters to write after each record. When non-empty, a new line is automatically appended to the separator. | `#` |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-h,`<br>`--help`&nbsp; | flag | Display this message |  |
| &nbsp;`-n,`<br>`--no-headers`&nbsp; | flag | When set, the first row will not be interpreted as headers. When set, the name of each field will be its index. |  |
| &nbsp;`-d,`<br>`--delimiter`&nbsp; | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |

---
**Source:** [`src/cmd/flatten.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/flatten.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
