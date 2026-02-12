# reverse

📇🤯

> Reverse order of rows in a CSV. Unlike the `sort --reverse` command, it preserves the order of rows with the same key. If an index is present, it works with constant memory. Otherwise, it will load all the data into memory.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/reverse.rs](https://github.com/dathere/qsv/blob/master/src/cmd/reverse.rs)**

## Description

Reverses rows of CSV data.

Useful for cases when there is no column that can be used for sorting in reverse order,
or when keys are not unique and order of rows with the same key needs to be preserved.

Note that if the CSV is not indexed, this operation will require reading all of the
CSV data into memory


## Usage

```console
qsv reverse [options] [<input>]
qsv reverse --help
```

## Common Options

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-h, --help` | flag | Display this message |  |
| `-o, --output` | string | Write output to <file> instead of stdout. |  |
| `-n, --no-headers` | flag | When set, the first row will not be interpreted as headers. Namely, it will be reversed with the rest of the rows. Otherwise, the first row will always appear as the header row in the output. |  |
| `-d, --delimiter` | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |
| `--memcheck` | flag | Check if there is enough memory to load the entire CSV into memory using CONSERVATIVE heuristics. |  |

---
**Source:** [`src/cmd/reverse.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/reverse.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
