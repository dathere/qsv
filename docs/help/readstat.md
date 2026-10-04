# readstat

> Convert SAS (`.sas7bdat`, `.xpt`), Stata (`.dta`) & SPSS (`.sav`, `.zsav`, `.por`) files to CSV, preserving the underlying codes of labelled values by default. Also dumps the rich variable metadata these formats carry - variable labels, value labels, missing-value codes, measure & display settings - with `--metadata`. Can read just the variables you need (`--select`) and a window or reproducible random sample of rows (`--offset`, `--limit`, `--sample`). Only SPSS portable (`.por`) files are read whole - every other format streams with constant memory, except SPSS `.sav`/`.zsav` files whose user-defined missing values are tracked (`--sentinels-as`, or the default check that warns when they are dropped).

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
conversion is lossless. Use --value-labels to decode them instead. SAS keeps
its value labels in a separate .sas7bcat format catalog, which --value-labels
finds next to the data file, or --sas7bcat names.

User-defined missing values ("sentinels") - SAS's .A to .Z & ._, Stata's .a
to .z, SPSS's declared missing codes - become empty cells by default, like any
other missing value. Use --sentinels-as to keep them. Without it, a warning
says how many sentinels were dropped & where. Counting them reads files on a
single thread, and SPSS files whole (see --batch), so it is skipped when the
option --jobs is above 1, and for SPSS files holding over about 128 MB of
data. As SPSS files declare their missing values, the warning then names the
variables that do instead. SAS & Stata files don't, so they get no warning.

The variable metadata these formats carry - variable labels, value labels,
missing-value codes, measure & display settings - can be dumped instead of the
data with --metadata.

Only part of a file can be read: --select picks variables, which the readers
skip over without decoding, and --offset, --limit & --sample pick rows. Rows
before the --offset are still read through, so skipping far into a large file
takes time, but stops early once --limit is reached.

Convert a SAS dataset to CSV:  
```console
qsv readstat data.sas7bdat > data.csv
```


Convert an SPSS file, decoding coded values to their labels:  
```console
qsv readstat --value-labels survey.sav -o survey.csv
```


Keep the sentinel codes of a SAS dataset, in a <name>_null column after each
numeric variable:  
```console
qsv readstat --sentinels-as value data.sas7bdat
```


Keep them in the variables' own columns instead:  
```console
qsv readstat --sentinels-as value --sentinels-embedded data.sas7bdat
```


Keep the sentinels of two SPSS variables as their labels, leaving the other
values as codes:  
```console
qsv readstat --sentinels-as label --sentinels-columns q1,q2 survey.sav
```


Read three variables & the first 1000 rows of a wide SPSS file:  
```console
qsv readstat --select id,age,income --limit 1000 survey.sav
```


Read every variable from "q1" to "q20", except "q7", by name range:  
```console
qsv readstat --select 'q1-q20,!q7' survey.sav
```


Take a reproducible random sample of 500 rows, in file order:  
```console
qsv readstat --sample 500 --seed 42 data.sas7bdat
```


Dump the variable dictionary of a Stata file:  
```console
qsv readstat --metadata pretty-json panel.dta
```


Dump the metadata of just two variables:  
```console
qsv readstat --metadata json --select age,income panel.dta
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

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑‑metadata`&nbsp; | string | Dump variable metadata instead of the data. Valid values: none, csv, json, pretty-json. For SAS, the value labels are included when a format catalog is used (see --value-labels). | `none` |
| &nbsp;`‑‑value‑labels`&nbsp; | flag | Decode coded values to their label strings (e.g. 1 becomes "Male") instead of writing the underlying codes. For SAS .sas7bdat files, the labels come from the format catalog: the file named by --sas7bcat, else <name>.sas7bcat or formats.sas7bcat next to the data file. Not supported for .xpt files. |  |
| &nbsp;`‑‑sas7bcat`&nbsp; | string | The SAS format catalog (.sas7bcat) holding the value labels of a .sas7bdat file, for use by the options --value-labels, --sentinels-as label & --metadata. It only names the catalog, so the data needs one of the first two. |  |
| &nbsp;`‑‑compress‑numeric`&nbsp; | flag | Write float variables that only ever hold whole numbers as integers, without the ".0" (e.g. 3.0 becomes 3). SPSS stores every number as a float, so this matters most for SPSS files. It is decided per variable, over the rows written: one value like 2.5 keeps the ".0" on every row of that variable. The file is read twice - once to check the values. With sentinel labels embedded, a variable with a label that reads as a number (e.g. "1.0") keeps its ".0", so the label is not rewritten. |  |
| &nbsp;`‑‑sentinels‑as`&nbsp; | string | Keep sentinels instead of writing them as empty cells. Each eligible variable gets a <name>_null column right after it, holding the sentinel of each row that has one & empty otherwise. Valid values: none, value, label. none  - write them as empty cells, without the check & its warning. value - the sentinel's code (e.g. .A or 99). label - the sentinel's value label if it has one, else its code. label labels only the sentinels: other values stay codes unless --value-labels is also given. SAS takes its sentinel labels from the format catalog, found as for --value-labels. For SPSS, value cannot be combined with --value-labels. Not supported for .xpt & .por files. Tracking sentinels reads files on a single thread, so the option --jobs has no effect, and reads SPSS .sav & .zsav files whole (see --batch). |  |
| &nbsp;`‑‑sentinels‑embedded`&nbsp; | flag | Write each sentinel into its variable's own column instead of a <name>_null column. Those columns then mix numbers & sentinels. Requires --sentinels-as. |  |
| &nbsp;`‑‑sentinels‑columns`&nbsp; | string | Comma-separated variables to keep sentinels for. Requires --sentinels-as. By default, every eligible variable: the numeric ones for SAS & Stata, those with declared missing values for SPSS. |  |
| &nbsp;`‑‑select`&nbsp; | string | The variables to read, in the order given, using qsv's select syntax: names, 1-based indices, ranges (q1-q20), /regex/ & ! to exclude. See 'qsv select --help' for the full syntax. Variables left out are skipped by the reader. Also applies to the option --metadata, which then lists only these variables. |  |
| &nbsp;`‑‑offset`&nbsp; | integer | Skip the first <n> rows. | `0` |
| &nbsp;`‑‑limit`&nbsp; | integer | Write at most <n> rows, counted after the rows skipped by --offset. |  |
| &nbsp;`‑‑sample`&nbsp; | integer | Write a random sample of <n> rows, in file order, drawn from the rows --offset & --limit select. If there are no more than <n> such rows, all of them are written. |  |
| &nbsp;`‑‑seed`&nbsp; | integer | Seed the random number generator of --sample, so the same sample is drawn every time. |  |
| &nbsp;`‑j,`<br>`‑‑jobs`&nbsp; | integer | Number of reader threads. Raising it speeds up large uncompressed files at the cost of memory, as out-of-order chunks have to be buffered to keep the rows in source order. Row order is preserved either way. | `1` |
| &nbsp;`‑b,`<br>`‑‑batch`&nbsp; | integer | Number of rows to read into memory at a time. Does not apply to SPSS portable (.por) files - they have no chunked reader upstream, so they are read whole & memory scales with the file. Nor does it apply to .sav & .zsav files while their sentinels are tracked, which also reads them whole: with the option --sentinels-as value or label, or by the sentinel check when a variable declares missing values & the file holds at most about 128 MB of data (see --sentinels-as none). | `50000` |

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
