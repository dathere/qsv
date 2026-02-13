# pro

> Interact with the qsv pro API.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/pro.rs](https://github.com/dathere/qsv/blob/master/src/cmd/pro.rs)**

<a name="nav"></a>
[Description](#description) | [Usage](#usage) | [Arguments](#arguments) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Interact with qsv pro API. Learn more about qsv pro at: <https://qsvpro.dathere.com>.

- qsv pro must be running for this command to work as described.
- Some features of this command require a paid plan of qsv pro and may require an Internet connection.

The qsv pro command has subcommands:
lens:     Run csvlens on a local file in a new Alacritty terminal emulator window (Windows only).
workflow: Import a local file into the qsv pro Workflow (Workflow must be open).


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv pro lens [options] [<input>]
qsv pro workflow [options] [<input>]
qsv pro --help
```

<a name="arguments"></a>

## Arguments [↩](#nav)

| Argument | Description |
|----------|-------------|
| `<input>` | The input file path to send to the qsv pro API. This must be a local file path, not stdin. Workflow supports: CSV, TSV, SSV, TAB, XLSX, XLS, XLSB, XLSM, ODS. |

<a name="common-options"></a>

## Common Options [↩](#nav)

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-h,`<br>`--help` | flag | Display this message |  |

---
**Source:** [`src/cmd/pro.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/pro.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
