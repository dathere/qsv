# index

> Create an index for a CSV. This is very quick (even the 15gb, 28m row NYC 311 dataset takes all of 14 seconds to index) & provides constant time indexing/random access into the CSV. With an index, `count`, `sample` & `slice` work instantaneously; random access mode is enabled in `luau`; and multithreading is enabled for the `frequency`, `split`, `stats`, `schema` & `tojsonl` commands.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/index.rs](https://github.com/dathere/qsv/blob/master/src/cmd/index.rs)**

<a name="nav"></a>
[Description](#description) | [Usage](#usage) | [Index Options](#index-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Creates an index of the given CSV data, which can make other operations like
slicing, splitting and gathering statistics much faster.

Note that this does not accept CSV data on stdin. You must give a file
path. The index is created at 'path/to/input.csv.idx'. The index will be
automatically used by commands that can benefit from it. If the original CSV
data changes after the index is made, commands that try to use it will result
in an error (you have to regenerate the index before it can be used again).

However, if the environment variable QSV_AUTOINDEX_SIZE is set, qsv will
automatically create an index when the input file size >= specified size (bytes).
It will also automatically update stale indices as well.


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv index [options] <input>
qsv index --help
```

<a name="index-options"></a>

## Index Options [↩](#nav)

| &nbsp;&nbsp;Option&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-o,`<br>`--output`&nbsp; | string | Write index to <file> instead of <input>.idx. Generally, this is not currently useful because the only way to use an index is if it is specially named <input>.idx. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;Option&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-h,`<br>`--help`&nbsp; | flag | Display this message |  |

---
**Source:** [`src/cmd/index.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/index.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
