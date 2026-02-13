# fmt

> Reformat a CSV with different delimiters, record terminators or quoting rules. (Supports ASCII delimited data.)

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/fmt.rs](https://github.com/dathere/qsv/blob/master/src/cmd/fmt.rs)**

<a name="nav"></a>
[Description](#description) | [Usage](#usage) | [Fmt Options](#fmt-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Formats CSV data with a custom delimiter or CRLF line endings.

Generally, all commands in qsv output CSV data in a default format, which is
the same as the default format for reading CSV data. This makes it easy to
pipe multiple qsv commands together. However, you may want the final result to
have a specific delimiter or record separator, and this is where 'qsv fmt' is
useful.

For examples, see <https://github.com/dathere/qsv/blob/master/tests/test_fmt.rs>.


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv fmt [options] [<input>]
qsv fmt --help
```

<a name="fmt-options"></a>

## Fmt Options [↩](#nav)

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-t,`<br>`--out-delimiter`&nbsp; | string | The field delimiter for writing CSV data. Must be a single character. If set to "T", uses tab as the delimiter. | `,` |
| &nbsp;`--crlf`&nbsp; | flag | Use '\r\n' line endings in the output. |  |
| &nbsp;`--ascii`&nbsp; | flag | Use ASCII field and record separators. Use Substitute (U+00A1) as the quote character. |  |
| &nbsp;`--quote`&nbsp; | string | The quote character to use. | `"` |
| &nbsp;`--quote-always`&nbsp; | flag | Put quotes around every value. |  |
| &nbsp;`--quote-never`&nbsp; | flag | Never put quotes around any value. |  |
| &nbsp;`--escape`&nbsp; | string | The escape character to use. When not specified, quotes are escaped by doubling them. |  |
| &nbsp;`--no-final-newline`&nbsp; | flag | Do not write a newline at the end of the output. This makes it easier to paste the output into Excel. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-h,`<br>`--help`&nbsp; | flag | Display this message |  |
| &nbsp;`-o,`<br>`--output`&nbsp; | string | Write output to <file> instead of stdout. |  |
| &nbsp;`-d,`<br>`--delimiter`&nbsp; | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |

---
**Source:** [`src/cmd/fmt.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/fmt.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
