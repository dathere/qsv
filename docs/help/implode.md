# implode

> Implode rows by grouping on key column(s) and joining a value column with a given separator. The inverse of `explode`.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/implode.rs](https://github.com/dathere/qsv/blob/master/src/cmd/implode.rs)** | [😣](TableOfContents.md#legend "uses additional memory proportional to the cardinality of the columns in the CSV.")[👆](TableOfContents.md#legend "has powerful column selector support. See `select` for syntax.")

<a name="nav"></a>
[Description](#description) | [Examples](#examples) | [Usage](#usage) | [Implode Options](#implode-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Implodes multiple rows into one by grouping on key column(s) and joining the
values of another column with the given separator. The inverse of `explode`.


<a name="examples"></a>

## Examples [↩](#nav)

```csv
name,color
John,blue
John,yellow
John,light red
Mary,red
```

> Can be imploded by key column "name", joining the "color" column with "; "

```console
qsv implode -k name -v color "; " data.csv
```

```csv
name,color
John,blue; yellow; light red
Mary,red
```

> With `-r colors` the value column is renamed

```console
qsv implode -k name -v color -r colors "; " data.csv
```

```csv
name,colors
John,blue; yellow; light red
Mary,red
```

Only the key column(s) and the value column appear in the output; any other
columns are dropped.
By default, all input rows are buffered in memory and groups are emitted in the
order keys are first seen. If the input is already sorted by the key column(s),
use --sorted to stream groups as they are seen (memory proportional to the
largest group, not the whole input).

<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv implode [options] -k <keys> -v <value> <separator> [<input>]
qsv implode --help
```

<a name="implode-options"></a>

## Implode Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑k,`<br>`‑‑keys`&nbsp; | string | Key column(s) to group by. Supports the usual selector syntax (e.g. "name", "1", "1-3", "a,c"). **(required)** |  |
| &nbsp;`‑v,`<br>`‑‑value`&nbsp; | string | The column whose values will be joined per group. Must resolve to exactly one column. **(required)** |  |
| &nbsp;`‑r,`<br>`‑‑rename`&nbsp; | string | New name for the imploded value column. |  |
| &nbsp;`‑‑sorted`&nbsp; | flag | Assume input is pre-sorted by the key column(s). Streams groups as they are seen; memory is bounded by the size of the largest group. |  |
| &nbsp;`‑‑skip‑empty`&nbsp; | flag | Skip empty values when joining. By default, empty values are included as empty tokens so that round-tripping with `explode` is lossless. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑h,`<br>`‑‑help`&nbsp; | flag | Display this message |  |
| &nbsp;`‑o,`<br>`‑‑output`&nbsp; | string | Write output to <file> instead of stdout. |  |
| &nbsp;`‑n,`<br>`‑‑no‑headers`&nbsp; | flag | When set, the first row will not be interpreted as headers. |  |
| &nbsp;`‑d,`<br>`‑‑delimiter`&nbsp; | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |

---
**Source:** [`src/cmd/implode.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/implode.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
