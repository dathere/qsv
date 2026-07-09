# fixedwidth

> Convert fixed-width text (fields at fixed byte-column positions, no delimiters) to CSV.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/fixedwidth.rs](https://github.com/dathere/qsv/blob/master/src/cmd/fixedwidth.rs)**

<a name="nav"></a>
[Description](#description) | [Examples](#examples) | [Usage](#usage) | [Fixedwidth Options](#fixedwidth-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Converts fixed-width text (fields at fixed byte-column positions, no
delimiters) to CSV.

By default, this expects the input's first line to be a comment enumerating
the 1-based starting byte position of each column, comma-separated and
prefixed with "#" - the same format `qsv table --align leftfwf` produces
(e.g. "#1,10,15"). Every subsequent line is a data record; each field runs
from its starting position up to (but not including) the next column's
starting position, or to the end of the line for the last column. Trailing
whitespace in each field is trimmed.

If the input doesn't have such a header comment - e.g. it comes from an
external system - specify the column positions explicitly with --positions,
or column widths with --widths.


<a name="examples"></a>

## Examples [↩](#nav)

Convert output of `qsv table --align leftfwf` back to CSV:  
```console
qsv table --align leftfwf data.csv | qsv fixedwidth > roundtrip.csv
```

Convert a file with explicit 1-based column start positions:  
```console
qsv fixedwidth --positions 1,10,15 mainframe_extract.txt
```

Convert a file with explicit column widths instead of positions:  
```console
qsv fixedwidth --widths 9,5,20 mainframe_extract.txt
```

See also <https://github.com/dathere/qsv/wiki/Transform-and-Reshape#fixedwidth>

<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv fixedwidth [options] [<input>]
qsv fixedwidth --help
```

<a name="fixedwidth-options"></a>

## Fixedwidth Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑‑positions`&nbsp; | string | Comma-separated, 1-based starting byte position of each column (e.g. "1,10,15"). Overrides any "#..." header comment in the input. |  |
| &nbsp;`‑‑widths`&nbsp; | string | Comma-separated width, in bytes, of each column (e.g. "9,5,20"). An alternative to --positions; the two are mutually exclusive. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑h,`<br>`‑‑help`&nbsp; | flag | Display this message |  |
| &nbsp;`‑o,`<br>`‑‑output`&nbsp; | string | Write output to <file> instead of stdout. |  |

---
**Source:** [`src/cmd/fixedwidth.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/fixedwidth.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
