# explode

> Explode rows into multiple ones by splitting a column value based on the given separator.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/explode.rs](https://github.com/dathere/qsv/blob/master/src/cmd/explode.rs)** | [🔣](TableOfContents.md#legend "requires UTF-8 encoded input.")[👆](TableOfContents.md#legend "has powerful column selector support. See `select` for syntax.")

<a name="nav"></a>
[Description](#description) | [Examples](#examples) | [Usage](#usage) | [Explode Options](#explode-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Explodes a row into multiple ones by splitting a column value based on the
given separator.


<a name="examples"></a>

## Examples [↩](#nav)

```csv
name,colors
John,blue|yellow
Mary,red
```

> Can be exploded on the "colors" <column> based on the "|" <separator>

```console
qsv explode colors "|" data.csv
```

```csv
name,colors
John,blue
John,yellow
Mary,red
```


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv explode [options] <column> <separator> [<input>]
qsv explode --help
```

<a name="explode-options"></a>

## Explode Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑r,`<br>`‑‑rename`&nbsp; | string | New name for the exploded column. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑h,`<br>`‑‑help`&nbsp; | flag | Display this message |  |
| &nbsp;`‑o,`<br>`‑‑output`&nbsp; | string | Write output to <file> instead of stdout. |  |
| &nbsp;`‑n,`<br>`‑‑no‑headers`&nbsp; | flag | When set, the first row will not be interpreted as headers. |  |
| &nbsp;`‑d,`<br>`‑‑delimiter`&nbsp; | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |

---
**Source:** [`src/cmd/explode.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/explode.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
