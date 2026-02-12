# flatten

> A flattened view of CSV records. Useful for viewing one record at a time. e.g. `qsv slice -i 5 data.csv | qsv flatten`.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/flatten.rs](https://github.com/dathere/qsv/blob/master/src/cmd/flatten.rs)**

## Description

Prints flattened records such that fields are labeled separated by a new line.
This mode is particularly useful for viewing one record at a time. Each
record is separated by a special '#' character (on a line by itself), which
can be changed with the --separator flag.

There is also a condensed view (-c or --condense) that will shorten the
contents of each field to provide a summary view.

For examples, see https://github.com/dathere/qsv/blob/master/tests/test_flatten.rs.


## Usage

```console
qsv flatten [options] [<input>]
qsv flatten --help
```

## Flatten Options

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-c, --condense` | string | Limits the length of each field to the value specified. If the field is UTF-8 encoded, then <arg> refers to the number of code points. Otherwise, it refers to the number of bytes. |  |
| `-f, --field-separator` | string | A string of character to write between a column name and its value. |  |
| `-s, --separator` | string | A string of characters to write after each record. When non-empty, a new line is automatically appended to the separator. | `#` |

## Common Options

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-h, --help` | flag | Display this message |  |
| `-n, --no-headers` | flag | When set, the first row will not be interpreted as headers. When set, the name of each field will be its index. |  |
| `-d, --delimiter` | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |

---
**Source:** [`src/cmd/flatten.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/flatten.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
