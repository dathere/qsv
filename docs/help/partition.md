# partition

> Partition a CSV based on a column value.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/partition.rs](https://github.com/dathere/qsv/blob/master/src/cmd/partition.rs)** | 👆

<a name="nav"></a>
[Description](#description) | [Usage](#usage) | [Arguments](#arguments) | [Partition Options](#partition-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Partitions the given CSV data into chunks based on the value of a column.

See `split` command to split a CSV data by row count, by number of chunks or
by kb-size.

The files are written to the output directory with filenames based on the
values in the partition column and the `--filename` flag.

Note: To account for case-insensitive file system collisions (e.g. macOS APFS
and Windows NTFS), the command will add a number suffix to the filename if the
value is already in use.

EXAMPLE:

Partition nyc311.csv file into separate files based on the value of the
"Borough" column in the current directory:
```console
$ qsv partition Borough . --filename "nyc311-{}.csv" nyc311.csv
```


will create the following files, each containing the data for each borough:
nyc311-Bronx.csv
nyc311-Brooklyn.csv
nyc311-Manhattan.csv
nyc311-Queens.csv
nyc311-Staten_Island.csv

For more examples, see <https://github.com/dathere/qsv/blob/master/tests/test_partition.rs>.


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv partition [options] <column> <outdir> [<input>]
qsv partition --help
```

<a name="arguments"></a>

## Arguments [↩](#nav)

| Argument | Description |
|----------|-------------|
| `<column>` | The column to use as a key for partitioning. You can use the `--select` option to select the column by name or index, but only one column can be used for partitioning. See `select` command for more details. |
| `<outdir>` | The directory to write the output files to. |
| `<input>` | The CSV file to read from. If not specified, then the input will be read from stdin. |

<a name="partition-options"></a>

## Partition Options [↩](#nav)

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `--filename` | string | A filename template to use when constructing the names of the output files.  The string '{}' will be replaced by a value based on the partition column, but sanitized for shell safety. | `{}.csv` |
| `-p,`<br>`--prefix-length` | string | Truncate the partition column after the specified number of bytes when creating the output file. |  |
| `--drop` | flag | Drop the partition column from results. |  |
| `--limit` | string | Limit the number of simultaneously open files. Useful for partitioning large datasets with many unique values to avoid "too many open files" errors. Data is processed in batches until all unique values are processed. If not set, it will be automatically set to the system limit with a 10% safety margin. If set to 0, it will process all data at once, regardless of the system's open files limit. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-h,`<br>`--help` | flag | Display this message |  |
| `-n,`<br>`--no-headers` | flag | When set, the first row will NOT be interpreted as column names. Otherwise, the first row will appear in all chunks as the header row. |  |
| `-d,`<br>`--delimiter` | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |

---
**Source:** [`src/cmd/partition.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/partition.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
