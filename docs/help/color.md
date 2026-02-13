# color

🐻‍❄️🖥️

> Outputs tabular data as a pretty, colorized table that always fits into the terminal. Apart from CSV and its dialects, Arrow, Avro/IPC, Parquet, JSON array & JSONL formats are supported with the "polars" feature.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/color.rs](https://github.com/dathere/qsv/blob/master/src/cmd/color.rs)**

[Description](#description) | [Usage](#usage) | [Color Options](#color-options) | [Common Options](#common-options)

## Description

Outputs tabular data as a pretty, colorized table that always fits into the
terminal.

Tabular data formats include CSV and its dialects, Arrow, Avro/IPC, Parquet,
JSON Array & JSONL. Note that non-CSV formats require the "polars" feature.

Requires buffering all tabular data into memory. Therefore, you should use the
'sample' or 'slice' command to trim down large CSV data before formatting
it with this command.

Color is turned off when redirecting or running CI. Set QSV_FORCE_COLOR=1
to override this behavior.

The color theme is detected based on the current terminal background color
if possible. Set QSV_THEME to DARK or LIGHT to skip detection. QSV_TERMWIDTH
can be used to override terminal size.


## Usage

```console
qsv color [options] [<input>]
qsv color --help
```

## Color Options

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-C, --color` | flag | Force color on, even in situations where colors would normally be disabled. |  |
| `-n, --row-numbers` | flag | Show row numbers. |  |
| `-t, --title` | string | Add a title row above the headers. |  |

## Common Options

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-h, --help` | flag | Display this message |  |
| `-o, --output` | string | Write output to <file> instead of stdout. |  |
| `-d, --delimiter` | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |
| `--memcheck` | flag | Check if there is enough memory to load the entire CSV into memory using CONSERVATIVE heuristics. |  |

---
**Source:** [`src/cmd/color.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/color.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
