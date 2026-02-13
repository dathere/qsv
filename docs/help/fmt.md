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
| `-t,`<br>`--out-delimiter` | string | The field delimiter for writing CSV data. Must be a single character. If set to "T", uses tab as the delimiter. | `,` |
| `--crlf` | flag | Use '\r\n' line endings in the output. |  |
| `--ascii` | flag | Use ASCII field and record separators. Use Substitute (U+00A1) as the quote character. |  |
| `--quote` | string | The quote character to use. | `"` |
| `--quote-always` | flag | Put quotes around every value. |  |
| `--quote-never` | flag | Never put quotes around any value. |  |
| `--escape` | string | The escape character to use. When not specified, quotes are escaped by doubling them. |  |
| `--no-final-newline` | flag | Do not write a newline at the end of the output. This makes it easier to paste the output into Excel. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-h,`<br>`--help` | flag | Display this message |  |
| `-o,`<br>`--output` | string | Write output to <file> instead of stdout. |  |
| `-d,`<br>`--delimiter` | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |

---
**Source:** [`src/cmd/fmt.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/fmt.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
