# input

> Read CSV data with special commenting, quoting, trimming, line-skipping & non-UTF8 encoding handling rules. Typically used to "normalize" a CSV for further processing with other qsv commands.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/input.rs](https://github.com/dathere/qsv/blob/master/src/cmd/input.rs)**

<a name="nav"></a>
[Description](#description) | [Usage](#usage) | [Input Options](#input-options) | [Common Options](#common-options)

## Description [↩](#nav)

Read CSV data with special commenting, quoting, trimming, line-skipping &
non UTF-8 encoding rules and transforms it to a "normalized", UTF-8 encoded CSV.

Generally, all qsv commands support basic options like specifying the delimiter
used in CSV data. However, this does not cover all possible types of CSV data. For
example, some CSV files don't use '"' for quotes or use different escaping styles.

Also, CSVs with preamble lines can have them skipped with the --skip-lines & --auto-skip
options. Similarly, --skip-lastlines allows epilogue lines to be skipped.

Finally, non UTF-8 encoded files are "lossy" saved to UTF-8 by default, replacing all
invalid UTF-8 sequences with �. Note though that this is not true transcoding.

If you need to properly transcode non UTF-8 files, you'll need to use a tool like `iconv`
before processing it with qsv - e.g. to convert an ISO-8859-1 encoded file to UTF-8:
`iconv -f ISO-8859-1 -t UTF-8 input.csv -o utf8_output.csv`.

You can change this behavior with the --encoding-errors option.

See <https://github.com/dathere/qsv#utf-8-encoding> for more details.

This command is typically used at the beginning of a data pipeline (thus the name `input`)
to normalize & prepare CSVs for further processing with other qsv commands.

For examples, see <https://github.com/dathere/qsv/blob/master/tests/test_input.rs>.


## Usage [↩](#nav)

```console
qsv input [options] [<input>]
qsv input --help
```

## Input Options [↩](#nav)

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `--quote` | string | The quote character to use. | `"` |
| `--escape` | string | The escape character to use. When not specified, quotes are escaped by doubling them. |  |
| `--no-quoting` | flag | Disable quoting completely when reading CSV data. |  |
| `--quote-style` | string | The quoting style to use when writing CSV data. Possible values: all, necessary, nonnumeric and never. All: Quotes all fields. Necessary: Quotes fields only when necessary - when fields contain a quote, delimiter or record terminator. Quotes are also necessary when writing an empty record (which is indistinguishable from a record with one empty field). NonNumeric: Quotes all fields that are non-numeric. Never: Never write quotes. Even if it produces invalid CSV. | `necessary` |
| `--skip-lines` | string | The number of preamble lines to skip. |  |
| `--auto-skip` | flag | Sniffs a CSV for preamble lines and automatically skips them. Takes precedence over --skip-lines option. Does not work with <stdin>. |  |
| `--skip-lastlines` | string | The number of epilogue lines to skip. |  |
| `--trim-headers` | flag | Trim leading & trailing whitespace & quotes from header values. |  |
| `--trim-fields` | flag | Trim leading & trailing whitespace from field values. |  |
| `--comment` | string | The comment character to use. When set, lines starting with this character will be skipped. |  |
| `--encoding-errors` | string | How to handle UTF-8 encoding errors. Possible values: replace, skip, strict. replace: Replace invalid UTF-8 sequences with �. skip: Fields with encoding errors are "<SKIPPED>". strict: Fail on any encoding errors. | `replace` |

## Common Options [↩](#nav)

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-h, --help` | flag | Display this message |  |
| `-o, --output` | string | Write output to <file> instead of stdout. |  |
| `-d, --delimiter` | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |

---
**Source:** [`src/cmd/input.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/input.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
