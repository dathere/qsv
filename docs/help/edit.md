# edit

> Replace the value of a cell specified by its row and column.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/edit.rs](https://github.com/dathere/qsv/blob/master/src/cmd/edit.rs)**

## Description

Replace the value of a cell specified by its row and column.

For example we have the following CSV file named items.csv:

item,color
shoes,blue
flashlight,gray

To output the data with the color of the shoes as green instead of blue, run:

```console
$ qsv edit items.csv color 0 green
```


The following is returned as output:

item,color
shoes,green
flashlight,gray

You may also choose to specify the column name by its index (in this case 1).
Specifying a column as a number is prioritized by index rather than name.
If there is no newline (\n) at the end of the input data, it may be added to the output.


## Usage

```console
qsv edit [options] <input> <column> <row> <value>
qsv edit --help
```

## Edit Options

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-i, --in-place` | flag | Overwrite the input file data with the output. The input file is renamed to a .bak file in the same directory. |  |

## Common Options

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-h, --help` | flag | Display this message |  |
| `-o, --output` | string | Write output to <file> instead of stdout. |  |
| `-n, --no-headers` | flag | Start row indices from the header row as 0 (allows editing the header row). |  |

---
**Source:** [`src/cmd/edit.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/edit.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
