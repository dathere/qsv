# lens

> Interactively view, search & filter tabular data files using the [csvlens](https://github.com/YS-L/csvlens#csvlens) engine. Apart from CSV and its dialects, Arrow, Avro/IPC, Parquet, JSON array & JSONL formats are supported with the "polars" feature.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/lens.rs](https://github.com/dathere/qsv/blob/master/src/cmd/lens.rs)** | [🐻‍❄️](TableOfContents.md#legend "command powered/accelerated by  vectorized query engine.")[🖥️](TableOfContents.md#legend "part of the User Interface (UI) feature group")

<a name="nav"></a>
[Description](#description) | [Examples](#examples) | [Usage](#usage) | [Lens Options](#lens-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Explore tabular data files interactively using the csvlens (<https://github.com/YS-L/csvlens>) engine.

If the polars feature is enabled, lens can browse tabular data in Arrow, Avro/IPC, Parquet, JSON (JSON Array)
and JSONL files. It also automatically decompresses csv/tsv/tab/ssv files using the gz,zlib & zst
compression formats (e.g. data.csv.gz, data.tsv.zlib, data.tab.gz & data.ssv.zst).

If the polars feature is not enabled, lens can only browse CSV dialects (CSV, TSV, Tab, SSV) and
its snappy-compressed variants (CSV.sz, TSV.sz, Tab.sz & SSV.sz).

Press 'q' to exit. Press '?' for help.


<a name="examples"></a>

## Examples [↩](#nav)

Automatically choose delimiter based on the file extension
```console
qsv lens data.csv // comma-separated
```

```console
qsv lens data.tsv // Tab-separated
```

```console
qsv lens data.tab // Tab-separated
```

```console
qsv lens data.ssv // Semicolon-separated
```

> custom delimiter

```console
qsv lens --delimiter '|' data.csv
```

Auto-decompresses several compression formats:
```console
qsv lens data.csv.sz // Snappy-compressed CSV
```

```console
qsv lens data.tsv.sz // Snappy-compressed Tab-separated
```

> additional compression formats below require polars feature

```console
qsv lens data.csv.gz // Gzipped CSV
```

```console
qsv lens data.tsv.zlib // Zlib-compressed Tab-separated
```

```console
qsv lens data.tab.zst // Zstd-compressed Tab-separated
```

```console
qsv lens data.ssv.zst // Zstd-compressed Semicolon-separated
```

Explore tabular data in other formats (if polars feature is enabled)
```console
qsv lens data.parquet // Parquet
```

```console
qsv lens data.jsonl // JSON Lines
```

```console
qsv lens data.json // JSON - will only work with a JSON Array
```

```console
qsv lens data.avro // Avro
```

Prompt the user to select a column to display. Once selected,
exit with the value of the City column for the selected row sent to stdout
```console
qsv lens --prompt 'Select City:' --echo-column 'City' data.csv
```

Only show rows that contain "NYPD"
```console
qsv lens --filter NYPD data.csv
```

> Show rows that contain "nois" case insensitive (for noise, noisy, noisier, etc.)

```console
qsv lens --filter nois --ignore-case data.csv
```

Find and highlight matches in the data
```console
qsv lens --find 'New York' data.csv
```

Find and highlight cells that have all numeric values in a column.
```console
qsv lens --find '^\d+$' data.csv
```


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv lens [options] [<input>]
qsv lens --help
```

<a name="lens-options"></a>

## Lens Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-d,`<br>`--delimiter`&nbsp; | string | Delimiter character (comma by default) "auto" to auto-detect the delimiter |  |
| &nbsp;`-t,`<br>`--tab-separated`&nbsp; | flag | Use tab separation. Shortcut for -d '\t' |  |
| &nbsp;`--no-headers`&nbsp; | flag | Do not interpret the first row as headers |  |
| &nbsp;`--columns`&nbsp; | string | Use this regex to select columns to display by default. Example: "col1\|col2\|col3" to select columns "col1", "col2" and "col3" and also columns like "col1_1", "col22" and "col3-more". |  |
| &nbsp;`--filter`&nbsp; | string | Use this regex to filter rows to display by default. The regex is matched against each cell in every column. Example: "val1\|val2" filters rows with any cells containing "val1", "val2" or text like "my_val1" or "val234". |  |
| &nbsp;`--find`&nbsp; | string | Use this regex to find and highlight matches by default. Automatically sets --monochrome to true so the matches are easier to see. The regex is matched against each cell in every column. Example: "val1\|val2" highlights text containing "val1", "val2" or longer text like "val1_ok" or "val2_error". |  |
| &nbsp;`-i,`<br>`--ignore-case`&nbsp; | flag | Searches ignore case. Ignored if any uppercase letters are present in the search string |  |
| &nbsp;`-f,`<br>`--freeze-columns`&nbsp; | string | Freeze the first N columns | `1` |
| &nbsp;`-m,`<br>`--monochrome`&nbsp; | flag | Disable color output |  |
| &nbsp;`-W,`<br>`--wrap-mode`&nbsp; | string | Set the wrap mode for the output. | `disabled` |
| &nbsp;`-A,`<br>`--auto-reload`&nbsp; | flag | Automatically reload the data when the file changes. |  |
| &nbsp;`-S,`<br>`--streaming-stdin`&nbsp; | flag | Enable streaming stdin (load input as it's being piped in) NOTE: This option only applies to stdin input. |  |
| &nbsp;`-P,`<br>`--prompt`&nbsp; | string | Set a custom prompt in the status bar. Normally paired w/ --echo-column: qsv lens --prompt 'Select City:' --echo-column 'City' Supports ANSI escape codes for colored or styled text. When using escape codes, ensure it's properly escaped. For example, in bash/zsh, the $'...' syntax is used to do so: qsv lens --prompt $'\033[1;5;31mBlinking red, bold text\033[0m' see <https://en.wikipedia.org/wiki/ANSI_escape_code#Colors> or <https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797> for more info on ANSI escape codes. Typing a complicated prompt on the command line can be tricky. If the prompt starts with "file:", it's interpreted as a filepath from which to load the prompt, e.g. qsv lens --prompt "file:prompt.txt" |  |
| &nbsp;`--echo-column`&nbsp; | string | Print the value of this column to stdout for the selected row |  |
| &nbsp;`--debug`&nbsp; | flag | Show stats for debugging |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-h,`<br>`--help`&nbsp; | flag | Display this message |  |

---
**Source:** [`src/cmd/lens.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/lens.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
