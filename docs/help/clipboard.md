# clipboard

> Provide input from the clipboard or save output to the clipboard.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/clipboard.rs](https://github.com/dathere/qsv/blob/master/src/cmd/clipboard.rs)** | [🖥️](TableOfContents.md#legend "part of the User Interface (UI) feature group")

<a name="nav"></a>
[Description](#description) | [Examples](#examples) | [Usage](#usage) | [Clip Options](#clip-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Provide input from the clipboard or save output to the clipboard.

Note when saving to clipboard on Windows, line breaks may be represented as \r\n (CRLF).
Meanwhile on Linux and macOS, they may be represented as \n (LF).


<a name="examples"></a>

## Examples [↩](#nav)

Pipe into qsv stats using qsv clipboard and render it as a table:
```console
qsv clipboard | qsv stats | qsv table
```

If you want to save the output of a command to the clipboard,
pipe into qsv clipboard using the --save or -s flag:
```console
qsv clipboard | qsv stats | qsv clipboard -s
```


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv clipboard [options]
qsv clipboard --help
```

<a name="clip-options"></a>

## Clip Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-s,`<br>`--save`&nbsp; | flag | Save output to clipboard. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`-h,`<br>`--help`&nbsp; | flag | Display this message |  |

---
**Source:** [`src/cmd/clipboard.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/clipboard.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
