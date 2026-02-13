# split

> Split one CSV file into many CSV files. It can split by number of rows, number of chunks or file size. Uses multithreading to go faster if an index is present when splitting by rows or chunks.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/split.rs](https://github.com/dathere/qsv/blob/master/src/cmd/split.rs)** | 📇🏎️

<a name="nav"></a>
[Description](#description) | [Examples](#examples) | [Usage](#usage) | [Arguments](#arguments) | [Split Options](#split-options) | [Filter Options](#filter-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Splits the given CSV data into chunks. It has three modes: by size (rowcount),
by number of chunks and by kb-size.

See `partition` command for splitting by a column value.

When splitting by size, the CSV data is split into chunks of the given number of
rows. The last chunk may have fewer rows if the number of records is not evenly
divisible by the given rowcount.

When splitting by number of chunks, the CSV data is split into the given number of
chunks. The number of rows in each chunk is determined by the number of records in
the CSV data and the number of desired chunks. If the number of records is not evenly
divisible by the number of chunks, the last chunk will have fewer records.

When splitting by kb-size, the CSV data is split into chunks of the given size in kilobytes.
The number of rows in each chunk may vary, but the size of each chunk will not exceed the
desired size.

Uses multithreading to go faster if the CSV has an index when splitting by size or
by number of chunks. Splitting by kb-size is always done sequentially with a single thread.

The default is to split by size with a chunk size of 500.

The files are written to the directory given with the name '{start}.csv',
where {start} is the index of the first record of the chunk (starting at 0).


<a name="examples"></a>

## Examples [↩](#nav)

> Create files with names like chunk_0.csv, chunk_100.csv, etc.

> in the directory 'outdir', creating the directory if it does not exist.

```console
qsv split outdir --size 100 --filename chunk_{}.csv input.csv
```

> Create files with names like chunk_00000.csv, chunk_00100.csv, etc.

> in the directory 'outdir/subdir', creating the directories if they do not exist.

```console
qsv split outdir/subdir -s 100 --filename chunk_{}.csv --pad 5 input.csv
```

> Create files like 0.csv, 100.csv, etc. in the current directory.

```console
qsv split . -s 100 input.csv
```

> Create files with names like 0.csv, 994.csv, etc. in the directory

> 'outdir', creating the directory if it does not exist. Each file will be close

> to 1000KB in size.

```console
qsv split outdir --kb-size 1000 input.csv
```

> Read from stdin and create files like mysplitoutput_0.csv, mysplitoutput_1000.csv, etc.

```console
cat in.csv | qsv split mysplitoutput -s 1000
```

> Create 10 files with names like 0.csv, 1000.csv, etc. in the directory 'outdir',

```console
qsv split outdir --chunks 10 input.csv
```

> Create 10 files with names like splitoutdir_0.csv, splitoutdir_1000.csv, etc.

> using 4 parallel jobs. Note that the input CSV must have an index

```console
qsv split splitoutdir -c 10 -j 4 input.csv
```

> This will create files with names like 0.csv, 100.csv, etc. in the directory

> 'outdir', and then run the command "gzip" on each chunk.

```console
qsv split outdir -s 100 --filter "gzip $FILE" input.csv
```

> WINDOWS: This will create files with names like 0.zip, 100.zip, etc. in the directory

> 'outdir', and then run the command "Compress-Archive" on each chunk.

```console
qsv split outdir --filter "powershell Compress-Archive -Path $FILE -Destination {}.zip" input.csv
```

For more examples, see [tests](https://github.com/dathere/qsv/blob/master/tests/test_split.rs).


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv split [options] (--size <arg> | --chunks <arg> | --kb-size <arg>) <outdir> [<input>]
qsv split --help
```

<a name="arguments"></a>

## Arguments [↩](#nav)

| Argument | Description |
|----------|-------------|
| &nbsp;`<outdir>`&nbsp; | The directory where the output files will be written. If it does not exist, it will be created. |
| &nbsp;`<input>`&nbsp; | The CSV file to read. If not given, input is read from STDIN. |

<a name="split-options"></a>

## Split Options [↩](#nav)

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-s,`<br>`--size`&nbsp; | string | The number of records to write into each chunk. | `500` |
| &nbsp;`-c,`<br>`--chunks`&nbsp; | string | The number of chunks to split the data into. This option is mutually exclusive with --size. The number of rows in each chunk is determined by the number of records in the CSV data and the number of desired chunks. If the number of records is not evenly divisible by the number of chunks, the last chunk will have fewer records. |  |
| &nbsp;`-k,`<br>`--kb-size`&nbsp; | string | The size of each chunk in kilobytes. The number of rows in each chunk may vary, but the size of each chunk will not exceed the desired size. This option is mutually exclusive with --size and --chunks. |  |
| &nbsp;`-j,`<br>`--jobs`&nbsp; | string | The number of splitting jobs to run in parallel. This only works when the given CSV data has an index already created. Note that a file handle is opened for each job. When not set, the number of jobs is set to the number of CPUs detected. |  |
| &nbsp;`--filename`&nbsp; | string | A filename template to use when constructing the names of the output files.  The string '{}' will be replaced by the zero-based row number of the first row in the chunk. | `{}.csv` |
| &nbsp;`--pad`&nbsp; | string | The zero padding width that is used in the generated filename. | `0` |

<a name="filter-options"></a>

## Filter Options [↩](#nav)

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`--filter`&nbsp; | string | Run the specified command on each chunk after it is written. The command should use the FILE environment variable ($FILE on Linux/macOS, %FILE% on Windows), which is set to the path of the output file for each chunk. The string '{}' in the command will be replaced by the zero-based row number of the first row in the chunk. |  |
| &nbsp;`--filter-cleanup`&nbsp; | flag | Cleanup the original output filename AFTER the filter command is run successfully for EACH chunk. If the filter command is not successful, the original filename is not removed. Only valid when --filter is used. |  |
| &nbsp;`--filter-ignore-errors`&nbsp; | flag | Ignore errors when running the filter command. Only valid when --filter is used. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-h,`<br>`--help`&nbsp; | flag | Display this message |  |
| &nbsp;`-n,`<br>`--no-headers`&nbsp; | flag | When set, the first row will NOT be interpreted as column names. Otherwise, the first row will appear in all chunks as the header row. |  |
| &nbsp;`-d,`<br>`--delimiter`&nbsp; | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |
| &nbsp;`-q,`<br>`--quiet`&nbsp; | flag | Do not display an output summary to stderr. |  |

---
**Source:** [`src/cmd/split.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/split.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
