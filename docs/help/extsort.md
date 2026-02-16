# extsort

> Sort an arbitrarily large CSV/text file using a multithreaded [external merge sort](https://en.wikipedia.org/wiki/External_sorting) algorithm.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/extsort.rs](https://github.com/dathere/qsv/blob/master/src/cmd/extsort.rs)** | 🚀📇👆

<a name="nav"></a>
[Description](#description) | [Usage](#usage) | [External Sort Option](#external-sort-option) | [CSV Mode Only Options](#csv-mode-only-options)

<a name="description"></a>

## Description [↩](#nav)

Sort an arbitrarily large CSV/text file using a multithreaded external sort algorithm.

This command has TWO modes of operation.

* CSV MODE
when --select is set, it sorts based on the given column/s. Requires an index.
See `qsv select --help` for select syntax details.
* LINE MODE
when --select is NOT set, it sorts any input text file (not just CSVs) on a
line-by-line basis. If sorting a non-CSV file, be sure to set --no-headers,
otherwise, the first line will not be included in the external sort.


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv extsort [options] [<input>] [<output>]
qsv extsort --help
```

<a name="external-sort-option"></a>

## External Sort Option [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-s,`<br>`--select`&nbsp; | string | Select a subset of columns to sort (CSV MODE). Note that the outputs will remain at the full width of the CSV. If --select is NOT set, extsort will work in LINE MODE, sorting the input as a text file on a line-by-line basis. |  |
| &nbsp;`-R,`<br>`--reverse`&nbsp; | flag | Reverse order |  |
| &nbsp;`--memory-limit`&nbsp; | string | The maximum amount of memory to buffer the external merge sort. If less than 50, this is a percentage of total memory. If more than 50, this is the memory in MB to allocate, capped at 90 percent of total memory. | `20` |
| &nbsp;`--tmp-dir`&nbsp; | string | The directory to use for externally sorting file segments. | `./` |
| &nbsp;`-j,`<br>`--jobs`&nbsp; | string | The number of jobs to run in parallel. When not set, the number of jobs is set to the number of CPUs detected. |  |

<a name="csv-mode-only-options"></a>

## CSV Mode Only Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-d,`<br>`--delimiter`&nbsp; | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |
| &nbsp;`-h,`<br>`--help`&nbsp; | flag | Display this message |  |
| &nbsp;`-n,`<br>`--no-headers`&nbsp; | flag | When set, the first row will not be interpreted as headers and will be sorted with the rest of the rows. Otherwise, the first row will always appear as the header row in the output. |  |

---
**Source:** [`src/cmd/extsort.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/extsort.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
