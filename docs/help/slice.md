# slice

📇🏎️🗃️

> Slice rows from any part of a CSV. When an index is present, this only has to parse the rows in the slice (instead of all rows leading up to the start of the slice).

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/slice.rs](https://github.com/dathere/qsv/blob/master/src/cmd/slice.rs)**

[Description](#description) | [Examples](#examples) | [Usage](#usage) | [Slice Options](#slice-options) | [Common Options](#common-options)

## Description

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


## Examples

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


## Usage

```console
qsv slice [options] [<input>]
qsv slice --help
```

## Slice Options

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-s, --start` | string | The index of the record to slice from. If negative, starts from the last record. |  |
| `-e, --end` | string | The index of the record to slice to. |  |
| `-l, --len` | string | The length of the slice (can be used instead of --end). |  |
| `-i, --index` | string | Slice a single record (shortcut for -s N -l 1). If negative, starts from the last record. |  |
| `--json` | flag | Output the result as JSON. Fields are written as key-value pairs. The key is the column name. The value is the field value. The output is a JSON array. If --no-headers is set, then the keys are the column indices (zero-based). |  |
| `--invert` | flag | slice all records EXCEPT those in the specified range. |  |

## Common Options

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-h, --help` | flag | Display this message |  |
| `-o, --output` | string | Write output to <file> instead of stdout. |  |
| `-n, --no-headers` | flag | When set, the first row will not be interpreted as headers. Otherwise, the first row will always appear in the output as the header row. |  |
| `-d, --delimiter` | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |

---
**Source:** [`src/cmd/slice.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/slice.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
