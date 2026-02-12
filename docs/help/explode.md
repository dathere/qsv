# explode

🔣👆

> Explode rows into multiple ones by splitting a column value based on the given separator.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/explode.rs](https://github.com/dathere/qsv/blob/master/src/cmd/explode.rs)**

## Description

Explodes a row into multiple ones by splitting a column value based on the
given separator.

For instance the following CSV:

name,colors
John,blue|yellow
Mary,red

Can be exploded on the "colors" <column> based on the "|" <separator> to:

name,colors
John,blue
John,yellow
Mary,red


## Usage

```console
qsv explode [options] <column> <separator> [<input>]
qsv explode --help
```

## Explode Options

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-r, --rename` | string | New name for the exploded column. |  |

## Common Options

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-h, --help` | flag | Display this message |  |
| `-o, --output` | string | Write output to <file> instead of stdout. |  |
| `-n, --no-headers` | flag | When set, the first row will not be interpreted as headers. |  |
| `-d, --delimiter` | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |

---
**Source:** [`src/cmd/explode.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/explode.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
