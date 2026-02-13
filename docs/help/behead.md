# behead

> Drop headers from a CSV.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/behead.rs](https://github.com/dathere/qsv/blob/master/src/cmd/behead.rs)**

<a name="nav"></a>
[Description](#description) | [Usage](#usage) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Drop a CSV file's header.


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv behead [options] [<input>]
qsv behead --help
```

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-h,`<br>`--help`&nbsp; | flag | Display this message |  |
| &nbsp;`-f,`<br>`--flexible`&nbsp; | flag | Do not validate if the CSV has different number of fields per record, increasing performance. |  |
| &nbsp;`-o,`<br>`--output`&nbsp; | string | Write output to <file> instead of stdout. |  |

---
**Source:** [`src/cmd/behead.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/behead.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
