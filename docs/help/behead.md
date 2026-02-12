# behead

> Drop headers from a CSV.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/behead.rs](https://github.com/dathere/qsv/blob/master/src/cmd/behead.rs)**

## Description

Drop a CSV file's header.


## Usage

```console
qsv behead [options] [<input>]
qsv behead --help
```

## Common Options

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-h, --help` | flag | Display this message |  |
| `-f, --flexible` | flag | Do not validate if the CSV has different number of fields per record, increasing performance. |  |
| `-o, --output` | string | Write output to <file> instead of stdout. |  |

---
**Source:** [`src/cmd/behead.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/behead.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
