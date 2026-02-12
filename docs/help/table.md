# table

> Align output of a CSV using elastic tabstops for viewing; or to create an "aligned TSV" file or Fixed Width Format file. To interactively view a CSV, use the `lens` command.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/table.rs](https://github.com/dathere/qsv/blob/master/src/cmd/table.rs)**

## Description

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


## Usage

```console
qsv table [options] [<input>]
qsv table --help
```

## Table Options

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-w, --width` | string | The minimum width of each column. | `2` |
| `-p, --pad` | string | The minimum number of spaces between each column. | `2` |
| `-a, --align` | string | How entries should be aligned in a column. Options: "left", "right", "center". "leftendtab" & "leftfwf" "leftendtab" is a special alignment that similar to "left" but with whitespace padding ending with a tab character. The resulting output still validates as a valid TSV file, while also being more human-readable (aka "aligned" TSV). "leftfwf" is similar to "left" with Fixed Width Format allgnment. The first line is a comment (prefixed with "#") that enumerates the position (1-based, comma-separated) of each column. | `left` |
| `-c, --condense` | string | Limits the length of each field to the value specified. If the field is UTF-8 encoded, then <arg> refers to the number of code points. Otherwise, it refers to the number of bytes. |  |

## Common Options

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-h, --help` | flag | Display this message |  |
| `-o, --output` | string | Write output to <file> instead of stdout. |  |
| `-d, --delimiter` | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |
| `--memcheck` | flag | Check if there is enough memory to load the entire CSV into memory using CONSERVATIVE heuristics. |  |

---
**Source:** [`src/cmd/table.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/table.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
