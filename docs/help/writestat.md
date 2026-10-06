# writestat

> The inverse of `readstat`: write a CSV as an SPSS (`.sav`, `.por`), Stata (`.dta`) or SAS transport (`.xpt`) file. With the JSON Schema data dictionary `readstat --dictionary` wrote, a converted file comes back with its variable labels, value labels, missing values & display settings - and decoded value labels & sentinel columns are turned back into codes & declared missing values. Metadata the output format can't hold is refused, not silently dropped, unless `--lossy` is given.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/writestat.rs](https://github.com/dathere/qsv/blob/master/src/cmd/writestat.rs)** | [🤯](TableOfContents.md#legend "loads entire CSV into memory, though `dedup`, `stats` & `transpose` have \"streaming\" modes as well.")[🐻‍❄️](TableOfContents.md#legend "command powered/accelerated by  vectorized query engine.")

<a name="nav"></a>
[Description](#description) | [Usage](#usage) | [Writestat Options](#writestat-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Write a CSV as an SPSS, Stata or SAS transport file.

EXPERIMENTAL: the writers this command is built on are young. Check the files
it writes in the package that will read them before relying on them, and please
report anything that looks wrong.

Output formats, chosen by the --output file's extension or by --format:

```text
SPSS    .sav, .por
Stata   .dta
SAS     .xpt (transport version 8, or 5 with --format xpt5)
```

Compressed SPSS .zsav files & SAS .sas7bdat datasets can't be written.

The variable metadata comes from a JSON Schema data dictionary (--dictionary).
One written by 'qsv readstat --dictionary' brings a converted file back with its
variable labels, value labels, missing values & display settings; one written
by 'qsv describegpt --dictionary' gives its labels as variable labels. Without
a dictionary, the column types are inferred from the CSV: numbers from its stats
cache (created beside it if missing, so later runs are faster), and dates, times
& true/false values as Polars recognises them. Set QSV_STATSCACHE_MODE to none
to have Polars infer every type itself instead, reading the whole file on a
single thread, which is much slower.

The dictionary also says how readstat wrote the CSV, and that is undone: the
values decoded by the option --value-labels go back to their codes, and the
SPSS sentinels of --sentinels-as become declared missing values again, whether
in <name>_null columns or embedded. A label that can't be turned back into one
code - shared by two codes, reading as a number that isn't its code, or also
an ordinary value - is an error: write the CSV again with readstat without
the label.

Metadata the output format can't hold is refused rather than dropped silently:  
value labels & missing values in SAS transport & SPSS portable files, missing
values & file labels in Stata files, labels too long for SAS transport files,
column names Stata would have to change, and so on. With --lossy, the file is
written anyway, and a warning names everything left out. Display settings
(measure, alignment, width & format) are written where the format has them and
left out where it doesn't.

Convert an SPSS file to CSV & back, with its metadata:  
```console
qsv readstat --dictionary survey.schema.json survey.sav -o survey.csv
```

```console
qsv writestat --dictionary survey.schema.json survey.csv -o survey2.sav
```


Write the same data as a Stata file:  
```console
qsv writestat --dictionary survey.schema.json survey.csv -o survey.dta
```


Write a CSV as a SAS transport file, inferring the types:  
```console
qsv writestat data.csv -o data.xpt
```


For examples, see <https://github.com/dathere/qsv/blob/master/tests/test_writestat.rs>.


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv writestat [options] [<input>]
qsv writestat --help
```

<a name="writestat-options"></a>

## Writestat Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑‑dictionary`&nbsp; | string | The JSON Schema data dictionary giving the variable metadata & types (see above). |  |
| &nbsp;`‑‑format`&nbsp; | string | The format to write, if not the one the --output extension names. Valid values: sav, por, dta, xpt, xpt5, xpt8. |  |
| &nbsp;`‑‑lossy`&nbsp; | flag | Write the file even if metadata the format can't hold has to be left out, warning about each. |  |
| &nbsp;`‑‑compress`&nbsp; | flag | Compress an SPSS .sav file (bytecode compression, which every SPSS version reads). |  |
| &nbsp;`‑‑table‑name`&nbsp; | string | The dataset name in a SAS transport file. Defaults to the dictionary's, else the --output file name. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑h,`<br>`‑‑help`&nbsp; | flag | Display this message |  |
| &nbsp;`‑o,`<br>`‑‑output`&nbsp; | string | The file to write. Required. |  |
| &nbsp;`‑d,`<br>`‑‑delimiter`&nbsp; | string | The field delimiter for reading the CSV. Must be a single character. (default: ,) |  |

---
**Source:** [`src/cmd/writestat.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/writestat.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
