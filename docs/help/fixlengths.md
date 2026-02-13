# fixlengths

> Force a CSV to have same-length records by either padding or truncating them.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/fixlengths.rs](https://github.com/dathere/qsv/blob/master/src/cmd/fixlengths.rs)**

<a name="nav"></a>
[Description](#description) | [Usage](#usage) | [Fixlengths Options](#fixlengths-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Transforms CSV data so that all records have the same length. The length is
the length of the longest record in the data (not counting trailing empty fields,
but at least 1). Records with smaller lengths are padded with empty fields.

This requires two complete scans of the CSV data: one for determining the
record size and one for the actual transform. Because of this, the input
given must be a file and not stdin.

Alternatively, if --length is set, then all records are forced to that length.
This requires a single pass and can be done with stdin.


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv fixlengths [options] [<input>]
qsv fixlengths --help
```

<a name="fixlengths-options"></a>

## Fixlengths Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-l,`<br>`--length`&nbsp; | string | Forcefully set the length of each record. If a record is not the size given, then it is truncated or expanded as appropriate. |  |
| &nbsp;`-r,`<br>`--remove-empty`&nbsp; | flag | Remove empty columns. |  |
| &nbsp;`-i,`<br>`--insert`&nbsp; | string | If empty fields need to be inserted, insert them at <pos>. If <pos> is zero, then it is inserted at the end of each record. If <pos> is negative, it is inserted from the END of each record going backwards. If <pos> is positive, it is inserted from the BEGINNING of each record going forward. | `0` |
| &nbsp;`--quote`&nbsp; | string | The quote character to use. | `"` |
| &nbsp;`--escape`&nbsp; | string | The escape character to use. When not specified, quotes are escaped by doubling them. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-h,`<br>`--help`&nbsp; | flag | Display this message |  |
| &nbsp;`-o,`<br>`--output`&nbsp; | string | Write output to <file> instead of stdout. |  |
| &nbsp;`-d,`<br>`--delimiter`&nbsp; | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |
| &nbsp;`-q,`<br>`--quiet`&nbsp; | flag | Don't print removed column information. |  |

---
**Source:** [`src/cmd/fixlengths.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/fixlengths.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
