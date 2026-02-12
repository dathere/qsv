# count

> Count the rows and optionally compile record width statistics of a CSV file. (11.87 seconds for a 15gb, 27m row NYC 311 dataset without an index. Instantaneous with an index.) If the `polars` feature is enabled, uses Polars' multithreaded, mem-mapped CSV reader for fast counts even without an index

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/count.rs](https://github.com/dathere/qsv/blob/master/src/cmd/count.rs)**

## Description

Returns a count of the number of records in the CSV data.

It has three modes of operation:
1. If a valid index is present, it will use it to lookup the count and
return instantaneously. (fastest)

If no index is present, it will read the CSV and count the number
of records by scanning the file.

2. If the polars feature is enabled, it will use the multithreaded,
mem-mapped Polars CSV reader. (faster - not available on qsvlite)

3. If the polars feature is not enabled, it will use the "regular",
single-threaded CSV reader.

Note that the count will not include the header row (unless --no-headers is
given).


## Examples

> Basic count of records in data.csv:

```console
qsv count data.csv
```

> Count records in data.csv without headers:

```console
qsv count --no-headers data.csv
```

> Count records in data.csv with human-readable output:

```console
qsv count --human-readable data.csv
```

> Count records in data.csv with width statistics:

```console
qsv count --width data.csv
```

> Count records in data.csv with width statistics (excluding delimiters):

```console
qsv count --width-no-delims data.csv
```

> Count records in data.csv with width statistics in JSON format:

```console
qsv count --width --json data.csv
```

For more examples, see [tests](https://github.com/dathere/qsv/blob/master/tests/test_count.rs).


## Usage

```console
qsv count [options] [<input>]
qsv count --help
```

## Count Options

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-H, --human-readable` | flag | Comma separate counts. |  |

## Width Options

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `--width` | flag | Also return the estimated widths of each record. Its an estimate as it doesn't count quotes, and will be an undercount if the record has quoted fields. The count and width are separated by a semicolon. It will return the max, avg, median, min, variance, stddev & MAD widths, separated by hyphens. If --human-readable is set, the widths will be labeled as "max", "avg", "median", "min", "stddev" & "mad" respectively, separated by spaces. Note that this option will require scanning the entire file using the "regular", single-threaded, streaming CSV reader, using the index if available for the count. If the file is very large, it may not be able to compile some stats - particularly avg, variance, stddev & MAD. In this case, it will return 0.0 for those stats. |  |
| `--width-no-delims` | flag | Same as --width but does not count the delimiters in the width. |  |
| `--json` | flag | Output the width stats in JSON format. |  |

## When The Polars Feature Is Enabled Options

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `--no-polars` | flag | Use the "regular", single-threaded, streaming CSV reader instead of the much faster multithreaded, mem-mapped Polars CSV reader. Use this when you encounter memory issues when counting with the Polars CSV reader. The streaming reader is slower but can read any valid CSV file of any size. |  |
| `--low-memory` | flag | Use the Polars CSV Reader's low-memory mode. This mode is slower but uses less memory. If counting still fails, use --no-polars instead to use the streaming CSV reader. |  |

## Common Options

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-h, --help` | flag | Display this message |  |
| `-f, --flexible` | flag | Do not validate if the CSV has different number of fields per record, increasing performance when counting without an index. |  |
| `-n, --no-headers` | flag | When set, the first row will be included in the count. |  |
| `-d, --delimiter` | string | The delimiter to use when reading CSV data. Must be a single character. | `,` |

---
**Source:** [`src/cmd/count.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/count.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
