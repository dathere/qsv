# readstat

> Convert SAS (`.sas7bdat`, `.xpt`), Stata (`.dta`) & SPSS (`.sav`, `.zsav`, `.por`) files to CSV, preserving the underlying codes of labelled values by default. Also dumps the rich variable metadata these formats carry - variable labels, value labels, missing-value codes, measure & display settings - with `--metadata`. Only SPSS portable (`.por`) files are read whole - every other format streams with constant memory.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/readstat.rs](https://github.com/dathere/qsv/blob/master/src/cmd/readstat.rs)** | [🤯](TableOfContents.md#legend "loads entire CSV into memory, though `dedup`, `stats` & `transpose` have \"streaming\" modes as well.")[🐻‍❄️](TableOfContents.md#legend "command powered/accelerated by  vectorized query engine.")[🚀](TableOfContents.md#legend "multithreaded even without an index.")

<a name="nav"></a>
[Description](#description) | [Usage](#usage) | [Readstat Options](#readstat-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Convert SAS, Stata & SPSS files to CSV.

EXPERIMENTAL: the readers this command is built on are young. Spot-check the
output against your source files before relying on a conversion, and please
report anything that looks wrong.

Supported input formats:

```text
SAS     .sas7bdat, .xpt, .xpt5, .xpt8
Stata   .dta
SPSS    .sav, .zsav, .por
```

Coded values are written as their underlying codes, not their labels, so the
conversion is lossless. Use --value-labels to decode them instead.

The variable metadata these formats carry - variable labels, value labels,
missing-value codes, measure & display settings - can be dumped instead of the
data with --metadata.

Convert a SAS dataset to CSV:  
```console
qsv readstat data.sas7bdat > data.csv
```


Convert an SPSS file, decoding coded values to their labels:  
```console
qsv readstat --value-labels survey.sav -o survey.csv
```


Dump the variable dictionary of a Stata file:  
```console
qsv readstat --metadata pretty-json panel.dta
```


Pipe straight into another qsv command:  
```console
qsv readstat data.sas7bdat | qsv stats
```


For examples, see <https://github.com/dathere/qsv/blob/master/tests/test_readstat.rs>.


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv readstat [options] [<input>]
qsv readstat --help
```

<a name="readstat-options"></a>

## Readstat Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑‑metadata`&nbsp; | string | Dump variable metadata instead of the data. Valid values: none, csv, json, pretty-json. | `none` |
| &nbsp;`‑‑value‑labels`&nbsp; | flag | Decode coded values to their label strings (e.g. 1 becomes "Male") instead of writing the underlying codes. Stata & SPSS only - SAS keeps its value labels in a separate .sas7bcat catalog, which this command does not read yet. |  |
| &nbsp;`‑j,`<br>`‑‑jobs`&nbsp; | integer | Number of reader threads. Raising it speeds up large uncompressed files at the cost of memory, as out-of-order chunks have to be buffered to keep the rows in source order. Row order is preserved either way. | `1` |
| &nbsp;`‑b,`<br>`‑‑batch`&nbsp; | integer | Number of rows to read into memory at a time. Does not apply to SPSS portable (.por) files - they have no chunked reader upstream, so they are read whole & memory scales with the file. | `50000` |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑h,`<br>`‑‑help`&nbsp; | flag | Display this message |  |
| &nbsp;`‑o,`<br>`‑‑output`&nbsp; | string | Write output to <file> instead of stdout. |  |
| &nbsp;`‑d,`<br>`‑‑delimiter`&nbsp; | string | The delimiter to use when writing CSV data. Must be a single character. | `,` |

---
**Source:** [`src/cmd/readstat.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/readstat.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
