# transpose

> Transpose rows/columns of a CSV.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/transpose.rs](https://github.com/dathere/qsv/blob/master/src/cmd/transpose.rs)** | [🤯](TableOfContents.md#legend "loads entire CSV into memory, though `dedup`, `stats` & `transpose` have \"streaming\" modes as well.")[👆](TableOfContents.md#legend "has powerful column selector support. See `select` for syntax.")

<a name="nav"></a>
[Description](#description) | [Examples](#examples) | [Usage](#usage) | [Transpose Options](#transpose-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Transpose the rows/columns of CSV data.


<a name="examples"></a>

## Examples [↩](#nav)

> Transpose data in-memory.

```console
qsv transpose data.csv
```

> Transpose data using multiple passes. For large datasets.

```console
qsv transpose data.csv --multipass
```

> Convert CSV to "long" format using the first column as the "field" identifier

```console
qsv transpose data.csv --long 1
```

> use the columns "name" & "age" as the "field" identifier

```console
qsv transpose --long "name,age" data.csv
```

> use the columns 1 & 3 as the "field" identifier

```console
qsv transpose --long 1,3 data.csv
```

> use the columns 1 to 3 as the "field" identifier

```console
qsv transpose --long 1-3 data.csv
```

> use all columns starting with "name" as the "field" identifier

```console
qsv transpose --long /^name/ data.csv
```

See <https://github.com/dathere/qsv/blob/master/tests/test_transpose.rs> for more examples.

<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv transpose [options] [<input>]
qsv transpose --help
```

<a name="transpose-options"></a>

## Transpose Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-m,`<br>`--multipass`&nbsp; | flag | Process the transpose by making multiple passes over the dataset. Consumes memory relative to the number of rows. Note that in general it is faster to process the transpose in memory. Useful for really big datasets as the default is to read the entire dataset into memory. |  |
| &nbsp;`-s,`<br>`--select`&nbsp; | string | Select a subset of columns to transpose. When used with --long, this filters which columns become attribute rows (the field columns are unaffected). See 'qsv select --help' for the full selection syntax. |  |
| &nbsp;`--long`&nbsp; | string | Convert wide-format CSV to "long" format. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-h,`<br>`--help`&nbsp; | flag | Display this message |  |
| &nbsp;`-o,`<br>`--output`&nbsp; | string | Write output to <file> instead of stdout. |  |
| &nbsp;`-d,`<br>`--delimiter`&nbsp; | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |
| &nbsp;`--memcheck`&nbsp; | flag | Check if there is enough memory to load the entire CSV into memory using CONSERVATIVE heuristics. Ignored when --multipass or --long option is enabled. |  |

---
**Source:** [`src/cmd/transpose.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/transpose.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
