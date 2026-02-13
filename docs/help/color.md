# color

> Outputs tabular data as a pretty, colorized table that always fits into the terminal. Apart from CSV and its dialects, Arrow, Avro/IPC, Parquet, JSON array & JSONL formats are supported with the "polars" feature.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/color.rs](https://github.com/dathere/qsv/blob/master/src/cmd/color.rs)** | 🐻‍❄️🖥️

<a name="nav"></a>
[Description](#description) | [Usage](#usage) | [Color Options](#color-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

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


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv color [options] [<input>]
qsv color --help
```

<a name="color-options"></a>

## Color Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-C,`<br>`--color`&nbsp; | flag | Force color on, even in situations where colors would normally be disabled. |  |
| &nbsp;`-n,`<br>`--row-numbers`&nbsp; | flag | Show row numbers. |  |
| &nbsp;`-t,`<br>`--title`&nbsp; | string | Add a title row above the headers. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-h,`<br>`--help`&nbsp; | flag | Display this message |  |
| &nbsp;`-o,`<br>`--output`&nbsp; | string | Write output to <file> instead of stdout. |  |
| &nbsp;`-d,`<br>`--delimiter`&nbsp; | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |
| &nbsp;`--memcheck`&nbsp; | flag | Check if there is enough memory to load the entire CSV into memory using CONSERVATIVE heuristics. |  |

---
**Source:** [`src/cmd/color.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/color.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
