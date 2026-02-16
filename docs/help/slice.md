# slice

> Slice rows from any part of a CSV. When an index is present, this only has to parse the rows in the slice (instead of all rows leading up to the start of the slice).

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/slice.rs](https://github.com/dathere/qsv/blob/master/src/cmd/slice.rs)** | <abbr title="uses an index when available.">📇</abbr><abbr title="multithreaded and/or faster when an index (📇) is available.">🏎️</abbr><abbr title="Limited Extended input support.">🗃️</abbr>

<a name="nav"></a>
[Description](#description) | [Examples](#examples) | [Usage](#usage) | [Slice Options](#slice-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Returns the rows in the range specified (starting at 0, half-open interval).
The range does not include headers.

If the start of the range isn't specified, then the slice starts from the first
record in the CSV data.

If the end of the range isn't specified, then the slice continues to the last
record in the CSV data.

This operation can be made much faster by creating an index with 'qsv index'
first. With an index, the command requires parsing just the rows that are
sliced. Without an index, all rows up to the first row in the slice must be
parsed.


<a name="examples"></a>

## Examples [↩](#nav)

> Slice from the 3rd record to the end

```console
qsv slice --start 2 data.csv
```

> Slice the first three records

```console
qsv slice --start 0 --end 2 data.csv
```

> Slice the first three records (using --len)

```console
qsv slice --len 3 data.csv
```

> Slice the last record

```console
qsv slice -s -1 data.csv
```

> Slice the last 10 records

```console
qsv slice -s -10 data.csv
```

> Get everything except the last 10 records

```console
qsv slice -s -10 --invert data.csv
```

> Slice the first three records of the last 10 records

```console
qsv slice -s -10 -l 3 data.csv
```

> Slice the second record

```console
qsv slice --index 1 data.csv
```

> Slice from the second record, two records

```console
qsv slice -s 1 --len 2 data.csv
```

> Slice records 10 to 20 as JSON

```console
qsv slice --start 9 --end 19 --json data.csv
```

> Slice records 1 to 9 and 21 to the end as JSON

```console
qsv slice --start 9 --len 10 --invert --json data.csv
```

For more examples, see [tests](https://github.com/dathere/qsv/blob/master/tests/test_slice.rs).


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv slice [options] [<input>]
qsv slice --help
```

<a name="slice-options"></a>

## Slice Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-s,`<br>`--start`&nbsp; | string | The index of the record to slice from. If negative, starts from the last record. |  |
| &nbsp;`-e,`<br>`--end`&nbsp; | string | The index of the record to slice to. |  |
| &nbsp;`-l,`<br>`--len`&nbsp; | string | The length of the slice (can be used instead of --end). |  |
| &nbsp;`-i,`<br>`--index`&nbsp; | string | Slice a single record (shortcut for -s N -l 1). If negative, starts from the last record. |  |
| &nbsp;`--json`&nbsp; | flag | Output the result as JSON. Fields are written as key-value pairs. The key is the column name. The value is the field value. The output is a JSON array. If --no-headers is set, then the keys are the column indices (zero-based). |  |
| &nbsp;`--invert`&nbsp; | flag | slice all records EXCEPT those in the specified range. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-h,`<br>`--help`&nbsp; | flag | Display this message |  |
| &nbsp;`-o,`<br>`--output`&nbsp; | string | Write output to <file> instead of stdout. |  |
| &nbsp;`-n,`<br>`--no-headers`&nbsp; | flag | When set, the first row will not be interpreted as headers. Otherwise, the first row will always appear in the output as the header row. |  |
| &nbsp;`-d,`<br>`--delimiter`&nbsp; | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |

---
**Source:** [`src/cmd/slice.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/slice.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
