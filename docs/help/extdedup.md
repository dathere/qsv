# extdedup

👆

> Remove duplicate rows from an arbitrarily large CSV/text file using a memory-mapped, on-disk hash table. Unlike the `dedup` command, this command does not load the entire file into memory nor does it sort the deduped file.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/extdedup.rs](https://github.com/dathere/qsv/blob/master/src/cmd/extdedup.rs)**

<a name="nav"></a>
[Description](#description) | [Usage](#usage) | [Extdedup Options](#extdedup-options) | [Csv Mode Only Options](#csv-mode-only-options)

## Description [↩](#nav)

Remove duplicate rows from an arbitrarily large CSV/text file using a memory-mapped,
on-disk hash table.

Unlike the 'dedup' command, this command does not load the entire file into memory
to sort the CSV first before deduping it.

This allows it to run in constant memory and the output will retain the input sort order.

This command has TWO modes of operation.

* CSV MODE
when --select is set, it dedupes based on the given column/s. See `qsv select --help`
for select syntax details.
* LINE MODE
when --select is NOT set, it deduplicates any input text file (not just CSVs) on a
line-by-line basis.

A duplicate count will be sent to <stderr>.


## Usage [↩](#nav)

```console
qsv extdedup [options] [<input>] [<output>]
qsv extdedup --help
```

## Extdedup Options [↩](#nav)

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-s, --select` | string | Select a subset of columns to dedup. Note that the outputs will remain at the full width of the CSV. If --select is NOT set, extdedup will work in LINE MODE, deduping the input as a text file on a line-by-line basis. |  |
| `--no-output` | flag | Do not write deduplicated output to <output>. Use this if you only want to know the duplicate count. |  |
| `-D, --dupes-output` | string | Write duplicates to <file>. Note that the file will NOT be a valid CSV. It is a list of duplicate lines, with the row number of the duplicate separated by a tab from the duplicate line itself. |  |
| `-H, --human-readable` | flag | Comma separate duplicate count. |  |
| `--memory-limit` | string | The maximum amount of memory to buffer the on-disk hash table. If less than 50, this is a percentage of total memory. If more than 50, this is the memory in MB to allocate, capped at 90 percent of total memory. | `10` |
| `--temp-dir` | string | Directory to store temporary hash table file. If not specified, defaults to operating system temp directory. |  |

## Csv Mode Only Options [↩](#nav)

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-n, --no-headers` | flag | When set, the first row will not be interpreted as headers. That is, it will be deduped with the rest of the rows. Otherwise, the first row will always appear as the header row in the output. |  |
| `-d, --delimiter` | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |
| `-h, --help` | flag | Display this message |  |
| `-q, --quiet` | flag | Do not print duplicate count to stderr. |  |

---
**Source:** [`src/cmd/extdedup.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/extdedup.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
