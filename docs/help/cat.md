# cat

🗄️

> Concatenate CSV files by row or by column.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/cat.rs](https://github.com/dathere/qsv/blob/master/src/cmd/cat.rs)**

[Description](#description) | [Examples](#examples) | [Usage](#usage) | [Arguments](#arguments) | [Columns Option](#columns-option) | [Rows Option](#rows-option) | [Rowskey Options](#rowskey-options) | [Common Options](#common-options)

## Description

Concatenate CSV files by row or by column.

When concatenating by column, the columns will be written in the same order as
the inputs given. The number of rows in the result is always equivalent to
the minimum number of rows across all given CSV data. (This behavior can be
reversed with the '--pad' flag.)

Concatenating by rows can be done in two ways:

'rows' subcommand:
All CSV data must have the same number of columns (unless --flexible is enabled)
and in the same order.
If you need to rearrange the columns or fix the lengths of records, use the
'select' or 'fixlengths' commands. Also, only the headers of the *first* CSV
data given are used. Headers in subsequent inputs are ignored. (This behavior
can be disabled with --no-headers.)

'rowskey' subcommand:
CSV data can have different numbers of columns and in different orders. All
columns are written in insertion order. If a column is missing in a row, an
empty field is written. If a column is missing in the header, an empty field
is written for all rows.


## Examples

> Concatenate CSV files by rows:

```console
qsv cat rows file1.csv file2.csv -o combined.csv
```

> Concatenate CSV files by rows, adding a grouping column with the filename:

```console
qsv cat rowskey --group fname --group-name source_file file1.csv file2.csv -o combined_with_keys.csv
```

> Concatenate CSV files by columns:

```console
qsv cat columns file1.csv file2.csv -o combined_columns.csv
```

> Concatenate all CSV files in a directory by rows:

```console
qsv cat rows path/to/csv_directory -o combined.csv
```

> Concatenate all CSV files listed in a .infile-list file by rows:

```console
qsv cat rows path/to/files_to_combine.infile-list -o combined.csv
```

For more examples, see [tests](https://github.com/dathere/qsv/blob/master/tests/test_cat.rs).


## Usage

```console
qsv cat rows    [options] [<input>...]
qsv cat rowskey [options] [<input>...]
qsv cat columns [options] [<input>...]
qsv cat --help
```

## Arguments

| Argument | Description |
|----------|-------------|
| `<input>` | ...              The CSV file(s) to read. Use '-' for standard input. If input is a directory, all files in the directory will be read as input. If the input is a file with a '.infile-list' extension, the file will be read as a list of input files. If the input are snappy-compressed files(s), it will be decompressed automatically. |

## Columns Option

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-p, --pad` | flag | When concatenating columns, this flag will cause all records to appear. It will pad each row if other CSV data isn't long enough. |  |

## Rows Option

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `--flexible` | flag | When concatenating rows, this flag turns off validation that the input and output CSVs have the same number of columns. This is faster, but may result in invalid CSV data. |  |

## Rowskey Options

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-g, --group` | string | When concatenating with rowskey, you can specify a grouping value which will be used as the first column in the output. This is useful when you want to know which file a row came from. Valid values are 'fullpath', 'parentdirfname', 'parentdirfstem', 'fname', 'fstem' and 'none'. A new column will be added to the beginning of each row using --group-name. If 'none' is specified, no grouping column will be added. | `none` |
| `-N, --group-name` | string | When concatenating with rowskey, this flag provides the name for the new grouping column. | `file` |

## Common Options

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-h, --help` | flag | Display this message |  |
| `-o, --output` | string | Write output to <file> instead of stdout. |  |
| `-n, --no-headers` | flag | When set, the first row will NOT be interpreted as column names. Note that this has no effect when concatenating columns. |  |
| `-d, --delimiter` | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |

---
**Source:** [`src/cmd/cat.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/cat.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
