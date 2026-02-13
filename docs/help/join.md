# join

> Inner, outer, right, cross, anti & semi joins. Automatically creates a simple, in-memory hash index to make it fast.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/join.rs](https://github.com/dathere/qsv/blob/master/src/cmd/join.rs)** | 😣👆

<a name="nav"></a>
[Description](#description) | [Usage](#usage) | [Arguments](#arguments) | [Join Options](#join-options) | [Join Key Transformation Options](#join-key-transformation-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Joins two sets of CSV data on the specified columns.

The default join operation is an 'inner' join. This corresponds to the
intersection of rows on the keys specified.

Joins are always done by ignoring leading and trailing whitespace. By default,
joins are done case sensitively, but this can be disabled with the --ignore-case
flag.

For examples, see <https://github.com/dathere/qsv/blob/master/tests/test_join.rs>.


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv join [options] <columns1> <input1> <columns2> <input2>
qsv join --help
```

<a name="arguments"></a>

## Arguments [↩](#nav)

| Argument | Description |
|----------|-------------|
| `<input1>` | is the first CSV data set to join. |
| `<input2>` | is the second CSV data set to join. |
| `<columns1>` | & <columns2> are the columns to join on for each input. |

<a name="join-options"></a>

## Join Options [↩](#nav)

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `--left` | flag | Do a 'left outer' join. This returns all rows in first CSV data set, including rows with no corresponding row in the second data set. When no corresponding row exists, it is padded out with empty fields. |  |
| `--left-anti` | flag | Do a 'left anti' join. This returns all rows in first CSV data set that has no match with the second data set. |  |
| `--left-semi` | flag | Do a 'left semi' join. This returns all rows in first CSV data set that has a match with the second data set. |  |
| `--right` | flag | Do a 'right outer' join. This returns all rows in second CSV data set, including rows with no corresponding row in the first data set. When no corresponding row exists, it is padded out with empty fields. (This is the reverse of 'outer left'.) |  |
| `--right-anti` | flag | This returns only the rows in the second CSV data set that do not have a corresponding row in the first data set. The output schema is the same as the second dataset. |  |
| `--right-semi` | flag | This returns only the rows in the second CSV data set that have a corresponding row in the first data set. The output schema is the same as the second data set. |  |
| `--full` | flag | Do a 'full outer' join. This returns all rows in both data sets with matching records joined. If there is no match, the missing side will be padded out with empty fields. (This is the combination of 'outer left' and 'outer right'.) |  |
| `--cross` | flag | USE WITH CAUTION. This returns the cartesian product of the CSV data sets given. The number of rows return is equal to N * M, where N and M correspond to the number of rows in the given data sets, respectively. |  |
| `--nulls` | flag | When set, joins will work on empty fields. Otherwise, empty fields are completely ignored. (In fact, any row that has an empty field in the key specified is ignored.) |  |
| `--keys-output` | string | Write successfully joined keys to <file>. This means that the keys are written to the output file when a match is found, with the exception of anti joins, where keys are written when NO match is found. Cross joins do not write keys. |  |

<a name="join-key-transformation-options"></a>

## Join Key Transformation Options [↩](#nav)

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-i,`<br>`--ignore-case` | flag | When set, joins are done case insensitively. |  |
| `-z,`<br>`--ignore-leading-zeros` | flag | When set, leading zeros are ignored in join keys. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-h,`<br>`--help` | flag | Display this message |  |
| `-o,`<br>`--output` | string | Write output to <file> instead of stdout. |  |
| `-n,`<br>`--no-headers` | flag | When set, the first row will not be interpreted as headers. (i.e., They are not searched, analyzed, sliced, etc.) |  |
| `-d,`<br>`--delimiter` | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |

---
**Source:** [`src/cmd/join.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/join.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
