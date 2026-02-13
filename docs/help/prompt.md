# prompt

> Open a file dialog to either pick a file as input or save output to a file.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/prompt.rs](https://github.com/dathere/qsv/blob/master/src/cmd/prompt.rs)** | 🐻‍❄️🖥️

<a name="nav"></a>
[Description](#description) | [Examples](#examples) | [Usage](#usage) | [Prompt Options](#prompt-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Open a file dialog to pick a file as input or save to an output file.


<a name="examples"></a>

## Examples [↩](#nav)

Pick a single file as input to qsv stats using an INPUT file dialog,
pipe into qsv stats using qsv prompt, and browse the stats using qsv lens:
```console
qsv prompt | qsv stats | qsv lens
```

If you want to save the output of a command to a file using a save file OUTPUT dialog,
pipe into qsv prompt using the --fd-output flag:
```console
qsv prompt -m 'Pick a CSV file to summarize' | qsv stats -E | qsv prompt --fd-output
```

Prompt for a spreadsheet, and export to CSV using a save file dialog:
```console
qsv prompt -m 'Select a spreadsheet to export to CSV' -F xlsx,xls,ods | \
qsv excel - | qsv prompt -m 'Save exported CSV to...' --fd-output
```


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv prompt [options]
qsv prompt --help
```

<a name="prompt-options"></a>

## Prompt Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-m,`<br>`--msg`&nbsp; | string | The prompt message to display in the file dialog title. When not using --fd-output, the default is "Select a File". When using --fd-output, the default is "Save File As". |  |
| &nbsp;`-F,`<br>`--filters`&nbsp; | string | The filter to use for the INPUT file dialog. Set to "None" to disable filters. Filters are comma-delimited file extensions. Defaults to csv,tsv,tab,ssv,xls,xlsx,xlsm,xlsb,ods. If the polars feature is enabled, it adds avro,arrow,ipc,parquet, json,jsonl,ndjson & gz,zst,zlib compressed files to the filter. |  |
| &nbsp;`-d,`<br>`--workdir`&nbsp; | string | The directory to start the file dialog in. | `.` |
| &nbsp;`-f,`<br>`--fd-output`&nbsp; | flag | Write output to a file by using a save file dialog. Used when piping into qsv prompt. Mutually exclusive with --output. |  |
| &nbsp;`--save-fname`&nbsp; | string | The filename to save the output as when using --fd-output. | `output.csv` |
| &nbsp;`--base-delay-ms`&nbsp; | string | The base delay in milliseconds to use when opening INPUT dialog. This is to ensure that the INPUT dialog is shown before/over the OUTPUT dialog when using the prompt command is used in both INPUT and OUTPUT modes in a single pipeline. | `200` |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-h,`<br>`--help`&nbsp; | flag | Display this message |  |
| &nbsp;`-o,`<br>`--output`&nbsp; | string | Write output to <file> without showing a save dialog. Mutually exclusive with --fd-output. |  |
| &nbsp;`-q,`<br>`--quiet`&nbsp; | flag | Do not print --fd-output message to stderr. |  |

---
**Source:** [`src/cmd/prompt.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/prompt.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
