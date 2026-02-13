# clipboard

🖥️

> Provide input from the clipboard or save output to the clipboard.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/clipboard.rs](https://github.com/dathere/qsv/blob/master/src/cmd/clipboard.rs)**

<a name="nav"></a>
[Description](#description) | [Examples](#examples) | [Usage](#usage) | [Clip Options](#clip-options) | [Common Options](#common-options)

## Description [↩](#nav)

Provide input from the clipboard or save output to the clipboard.

Note when saving to clipboard on Windows, line breaks may be represented as \r\n (CRLF).
Meanwhile on Linux and macOS, they may be represented as \n (LF).


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


## Usage [↩](#nav)

```console
qsv clipboard [options]
qsv clipboard --help
```

## Clip Options [↩](#nav)

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-s, --save` | flag | Save output to clipboard. |  |

## Common Options [↩](#nav)

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-h, --help` | flag | Display this message |  |

---
**Source:** [`src/cmd/clipboard.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/clipboard.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
