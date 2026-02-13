# behead

> Drop headers from a CSV.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/behead.rs](https://github.com/dathere/qsv/blob/master/src/cmd/behead.rs)**

<a name="nav"></a>
[Description](#description) | [Usage](#usage) | [Common Options](#common-options)

## Description [↩](#nav)

Drop a CSV file's header.


## Usage [↩](#nav)

```console
qsv behead [options] [<input>]
qsv behead --help
```

## Common Options [↩](#nav)

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-h, --help` | flag | Display this message |  |
| `-f, --flexible` | flag | Do not validate if the CSV has different number of fields per record, increasing performance. |  |
| `-o, --output` | string | Write output to <file> instead of stdout. |  |

---
**Source:** [`src/cmd/behead.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/behead.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
