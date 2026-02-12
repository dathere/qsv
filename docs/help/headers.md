# headers

> Show the headers of a CSV. Or show the intersection of all headers between many CSV files.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/headers.rs](https://github.com/dathere/qsv/blob/master/src/cmd/headers.rs)**

## Description

Prints the fields of the first row in the CSV data.

These names can be used in commands like 'select' to refer to columns in the
CSV data.

Note that multiple CSV files may be given to this command. This is useful with
the --intersect flag.

For examples, see <https://github.com/dathere/qsv/blob/master/tests/test_headers.rs>.


## Usage

```console
qsv headers [options] [<input>...]
qsv headers --help
```

## Arguments

| Argument | Description |
|----------|-------------|
| `<input>` | ...             The CSV file(s) to read. Use '-' for standard input. If input is a directory, all files in the directory will be read as input. If the input is a file with a '.infile-list' extension, the file will be read as a list of input files. If the input are snappy-compressed files(s), it will be decompressed automatically. |

## Headers Options

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-j, --just-names` | flag | Only show the header names (hide column index). This is automatically enabled if more than one input is given. |  |
| `-J, --just-count` | flag | Only show the number of headers. |  |
| `--intersect` | flag | Shows the intersection of all headers in all of the inputs given. |  |
| `--trim` | flag | Trim space & quote characters from header name. |  |

## Common Options

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-h, --help` | flag | Display this message |  |
| `-d, --delimiter` | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |

---
**Source:** [`src/cmd/headers.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/headers.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
