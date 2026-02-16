# sort

> Sorts CSV data in [lexicographical](https://en.wikipedia.org/wiki/Lexicographic_order), [natural](https://en.wikipedia.org/wiki/Natural_sort_order), numerical, reverse, unique or random (with optional seed) order (Also see `extsort` & `sortcheck` commands).

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/sort.rs](https://github.com/dathere/qsv/blob/master/src/cmd/sort.rs)** | 🚀🤯👆

<a name="nav"></a>
[Description](#description) | [Usage](#usage) | [Sort Options](#sort-options) | [Random Sorting Options](#random-sorting-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Sorts CSV data in lexicographical, natural, numerical, reverse, unique or random order.

Note that this requires reading all of the CSV data into memory. If
you need to sort a large file that may not fit into memory, use the
extsort command instead.

For examples, see <https://github.com/dathere/qsv/blob/master/tests/test_sort.rs>.


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv sort [options] [<input>]
qsv sort --help
```

<a name="sort-options"></a>

## Sort Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-s,`<br>`--select`&nbsp; | string | Select a subset of columns to sort. See 'qsv select --help' for the format details. |  |
| &nbsp;`-N,`<br>`--numeric`&nbsp; | flag | Compare according to string numerical value |  |
| &nbsp;`--natural`&nbsp; | flag | Compare strings using natural sort order (treats numbers within strings as actual numbers, e.g. "data1.txt", "data2.txt", "data10.txt", as opposed to "data1.txt", "data10.txt", "data2.txt" when sorting lexicographically) <https://en.wikipedia.org/wiki/Natural_sort_order> |  |
| &nbsp;`-R,`<br>`--reverse`&nbsp; | flag | Reverse order |  |
| &nbsp;`-i,`<br>`--ignore-case`&nbsp; | flag | Compare strings disregarding case |  |
| &nbsp;`-u,`<br>`--unique`&nbsp; | flag | When set, identical consecutive lines will be dropped to keep only one line per sorted value. |  |

<a name="random-sorting-options"></a>

## Random Sorting Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`--random`&nbsp; | flag | Randomize (scramble) the data by row |  |
| &nbsp;`--seed`&nbsp; | string | Random Number Generator (RNG) seed to use if --random is set |  |
| &nbsp;`--rng`&nbsp; | string | The RNG algorithm to use if --random is set. | `standard` |
| &nbsp;`-j,`<br>`--jobs`&nbsp; | string | The number of jobs to run in parallel. When not set, the number of jobs is set to the number of CPUs detected. |  |
| &nbsp;`--faster`&nbsp; | flag | When set, the sort will be faster. This is done by using a faster sorting algorithm that is not "stable" (i.e. the order of identical values is not guaranteed to be preserved). It has the added side benefit that the sort will also be in-place (i.e. does not allocate), which is useful for sorting large files that will otherwise NOT fit in memory using the default allocating stable sort. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-h,`<br>`--help`&nbsp; | flag | Display this message |  |
| &nbsp;`-o,`<br>`--output`&nbsp; | string | Write output to <file> instead of stdout. |  |
| &nbsp;`-n,`<br>`--no-headers`&nbsp; | flag | When set, the first row will not be interpreted as headers. Namely, it will be sorted with the rest of the rows. Otherwise, the first row will always appear as the header row in the output. |  |
| &nbsp;`-d,`<br>`--delimiter`&nbsp; | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |
| &nbsp;`--memcheck`&nbsp; | flag | Check if there is enough memory to load the entire CSV into memory using CONSERVATIVE heuristics. Ignored if --random or --faster is set. |  |

---
**Source:** [`src/cmd/sort.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/sort.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
