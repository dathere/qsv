# table

> Align output of a CSV using elastic tabstops for viewing; or to create an "aligned TSV" file or Fixed Width Format file. To interactively view a CSV, use the `lens` command.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/table.rs](https://github.com/dathere/qsv/blob/master/src/cmd/table.rs)** | 🤯

<a name="nav"></a>
[Description](#description) | [Usage](#usage) | [Table Options](#table-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Outputs CSV data as a table with columns in alignment.

Though this command is primarily designed for DISPLAYING CSV data using
"elastic tabstops" so its more human-readable, it can also be used to convert
CSV data to other special machine-readable formats:
-  a more human-readable TSV format with the "leftendtab" alignment option
-  Fixed-Width format with the "leftfwf" alignment option - similar to "left",
but with the first line being a comment (prefixed with "#") that enumerates
the position (1-based, comma-separated) of each column (e.g. "#1,10,15").

This will not work well if the CSV data contains large fields.

Note that formatting a table requires buffering all CSV data into memory.
Therefore, you should use the 'sample' or 'slice' command to trim down large
CSV data before formatting it with this command.


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv table [options] [<input>]
qsv table --help
```

<a name="table-options"></a>

## Table Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-w,`<br>`--width`&nbsp; | string | The minimum width of each column. | `2` |
| &nbsp;`-p,`<br>`--pad`&nbsp; | string | The minimum number of spaces between each column. | `2` |
| &nbsp;`-a,`<br>`--align`&nbsp; | string | How entries should be aligned in a column. Options: "left", "right", "center". "leftendtab" & "leftfwf" "leftendtab" is a special alignment that similar to "left" but with whitespace padding ending with a tab character. The resulting output still validates as a valid TSV file, while also being more human-readable (aka "aligned" TSV). "leftfwf" is similar to "left" with Fixed Width Format allgnment. The first line is a comment (prefixed with "#") that enumerates the position (1-based, comma-separated) of each column. | `left` |
| &nbsp;`-c,`<br>`--condense`&nbsp; | string | Limits the length of each field to the value specified. If the field is UTF-8 encoded, then <arg> refers to the number of code points. Otherwise, it refers to the number of bytes. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-h,`<br>`--help`&nbsp; | flag | Display this message |  |
| &nbsp;`-o,`<br>`--output`&nbsp; | string | Write output to <file> instead of stdout. |  |
| &nbsp;`-d,`<br>`--delimiter`&nbsp; | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |
| &nbsp;`--memcheck`&nbsp; | flag | Check if there is enough memory to load the entire CSV into memory using CONSERVATIVE heuristics. |  |

---
**Source:** [`src/cmd/table.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/table.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
