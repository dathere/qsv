# sort

> Sorts CSV data in lexicographical, natural, numerical, reverse, unique or random (with optional seed) order (Also see `extsort` & `sortcheck` commands).

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/sort.rs](https://github.com/dathere/qsv/blob/master/src/cmd/sort.rs)**

## Description

Sorts CSV data in lexicographical, natural, numerical, reverse, unique or random order.

Note that this requires reading all of the CSV data into memory. If
you need to sort a large file that may not fit into memory, use the
extsort command instead.

For examples, see https://github.com/dathere/qsv/blob/master/tests/test_sort.rs.


## Usage

```console
qsv sort [options] [<input>]
qsv sort --help
```

## Sort Options

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-s, --select` | string | Select a subset of columns to sort. See 'qsv select --help' for the format details. |  |
| `-N, --numeric` | flag | Compare according to string numerical value |  |
| `--natural` | flag | Compare strings using natural sort order (treats numbers within strings as actual numbers, e.g. "data1.txt", "data2.txt", "data10.txt", as opposed to "data1.txt", "data10.txt", "data2.txt" when sorting lexicographically) https://en.wikipedia.org/wiki/Natural_sort_order |  |
| `-R, --reverse` | flag | Reverse order |  |
| `-i, --ignore-case` | flag | Compare strings disregarding case |  |
| `-u, --unique` | flag | When set, identical consecutive lines will be dropped to keep only one line per sorted value. |  |

## Random Sorting Options

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `--random` | flag | Randomize (scramble) the data by row |  |
| `--seed` | string | Random Number Generator (RNG) seed to use if --random is set |  |
| `--rng` | string | The RNG algorithm to use if --random is set. | `standard` |
| `-, -` | flag | 1.5 GB/s throughput. |  |
| `-j, --jobs` | string | The number of jobs to run in parallel. When not set, the number of jobs is set to the number of CPUs detected. |  |
| `--faster` | flag | When set, the sort will be faster. This is done by using a faster sorting algorithm that is not "stable" (i.e. the order of identical values is not guaranteed to be preserved). It has the added side benefit that the sort will also be in-place (i.e. does not allocate), which is useful for sorting large files that will otherwise NOT fit in memory using the default allocating stable sort. |  |

## Common Options

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-h, --help` | flag | Display this message |  |
| `-o, --output` | string | Write output to <file> instead of stdout. |  |
| `-n, --no-headers` | flag | When set, the first row will not be interpreted as headers. Namely, it will be sorted with the rest of the rows. Otherwise, the first row will always appear as the header row in the output. |  |
| `-d, --delimiter` | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |
| `--memcheck` | flag | Check if there is enough memory to load the entire CSV into memory using CONSERVATIVE heuristics. Ignored if --random or --faster is set. |  |

---
**Source:** [`src/cmd/sort.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/sort.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
