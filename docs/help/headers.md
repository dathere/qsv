# headers

> Show the headers of a CSV. Or show the intersection of all headers between many CSV files.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/headers.rs](https://github.com/dathere/qsv/blob/master/src/cmd/headers.rs)** | <abbr title="Extended input support.">🗄️</abbr>

<a name="nav"></a>
[Description](#description) | [Usage](#usage) | [Arguments](#arguments) | [Headers Options](#headers-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Prints the fields of the first row in the CSV data.

These names can be used in commands like 'select' to refer to columns in the
CSV data.

Note that multiple CSV files may be given to this command. This is useful with
the --intersect flag.

For examples, see <https://github.com/dathere/qsv/blob/master/tests/test_headers.rs>.


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv headers [options] [<input>...]
qsv headers --help
```

<a name="arguments"></a>

## Arguments [↩](#nav)

| Argument&nbsp; | Description |
|----------|-------------|
| &nbsp;`<input>`&nbsp; | ...             The CSV file(s) to read. Use '-' for standard input. If input is a directory, all files in the directory will be read as input. If the input is a file with a '.infile-list' extension, the file will be read as a list of input files. If the input are snappy-compressed files(s), it will be decompressed automatically. |

<a name="headers-options"></a>

## Headers Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-j,`<br>`--just-names`&nbsp; | flag | Only show the header names (hide column index). This is automatically enabled if more than one input is given. |  |
| &nbsp;`-J,`<br>`--just-count`&nbsp; | flag | Only show the number of headers. |  |
| &nbsp;`--intersect`&nbsp; | flag | Shows the intersection of all headers in all of the inputs given. |  |
| &nbsp;`--trim`&nbsp; | flag | Trim space & quote characters from header name. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-h,`<br>`--help`&nbsp; | flag | Display this message |  |
| &nbsp;`-d,`<br>`--delimiter`&nbsp; | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |

---
**Source:** [`src/cmd/headers.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/headers.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
