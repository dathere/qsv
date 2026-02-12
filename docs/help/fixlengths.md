# fixlengths

> Force a CSV to have same-length records by either padding or truncating them.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/fixlengths.rs](https://github.com/dathere/qsv/blob/master/src/cmd/fixlengths.rs)**

## Description

Transforms CSV data so that all records have the same length. The length is
the length of the longest record in the data (not counting trailing empty fields,
but at least 1). Records with smaller lengths are padded with empty fields.

This requires two complete scans of the CSV data: one for determining the
record size and one for the actual transform. Because of this, the input
given must be a file and not stdin.

Alternatively, if --length is set, then all records are forced to that length.
This requires a single pass and can be done with stdin.


## Usage

```console
qsv fixlengths [options] [<input>]
qsv fixlengths --help
```

## Fixlengths Options

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-l, --length` | string | Forcefully set the length of each record. If a record is not the size given, then it is truncated or expanded as appropriate. |  |
| `-r, --remove-empty` | flag | Remove empty columns. |  |
| `-i, --insert` | string | If empty fields need to be inserted, insert them at <pos>. If <pos> is zero, then it is inserted at the end of each record. If <pos> is negative, it is inserted from the END of each record going backwards. If <pos> is positive, it is inserted from the BEGINNING of each record going forward. | `0` |
| `--quote` | string | The quote character to use. | `"` |
| `--escape` | string | The escape character to use. When not specified, quotes are escaped by doubling them. |  |

## Common Options

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-h, --help` | flag | Display this message |  |
| `-o, --output` | string | Write output to <file> instead of stdout. |  |
| `-d, --delimiter` | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |
| `-q, --quiet` | flag | Don't print removed column information. |  |

---
**Source:** [`src/cmd/fixlengths.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/fixlengths.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
