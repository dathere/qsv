# pivotp

🚀🐻‍❄️🪄

> Pivot CSV data. Features "smart" aggregation auto-selection based on data type & stats.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/pivotp.rs](https://github.com/dathere/qsv/blob/master/src/cmd/pivotp.rs)**

<a name="nav"></a>
[Description](#description) | [Usage](#usage) | [Arguments](#arguments) | [Pivotp Options](#pivotp-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Pivots CSV data using the Polars engine.

The pivot operation consists of:
- One or more index columns (these will be the new rows)
- A column that will be pivoted (this will create the new columns)
- A values column that will be aggregated
- An aggregation function to apply. Features "smart" aggregation auto-selection.

For examples, see <https://github.com/dathere/qsv/blob/master/tests/test_pivotp.rs>.


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv pivotp [options] <on-cols> <input>
qsv pivotp --help
```

<a name="arguments"></a>

## Arguments [↩](#nav)

| Argument | Description |
|----------|-------------|
| `<on-cols>` | The column(s) to pivot on (creates new columns). |
| `<input>` | is the input CSV file. The file must have headers. If the file has a pschema.json file, it will be used to inform the pivot operation unless --infer-len is explicitly set to a value other than the default of 10,000 rows. Stdin is not supported. |

<a name="pivotp-options"></a>

## Pivotp Options [↩](#nav)

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-i, --index` | string | The column(s) to use as the index (row labels). Specify multiple columns by separating them with a comma. The output will have one row for each unique combination of the index's values. If None, all remaining columns not specified on --on and --values will be used. At least one of --index and --values must be specified. |  |
| `-v, --values` | string | The column(s) containing values to aggregate. If an aggregation is specified, these are the values on which the aggregation will be computed. If None, all remaining columns not specified on --on and --index will be used. At least one of --index and --values must be specified. |  |
| `-a, --agg` | string | The aggregation function to use: first - First value encountered last - Last value encountered sum - Sum of values min - Minimum value max - Maximum value mean - Average value median - Median value len - Count of values item - Get single value from group. Raises error if there are multiple values. smart - use value column data type & statistics to pick an aggregation. Will only work if there is one value column, otherwise it falls back to `first` | `smart` |
| `--sort-columns` | flag | Sort the transposed columns by name. |  |
| `--maintain-order` | flag | Maintain the order of the input columns. |  |
| `--col-separator` | string | The separator in generated column names in case of multiple --values columns. | `_` |
| `--validate` | flag | Validate a pivot by checking the pivot column(s)' cardinality. |  |
| `--try-parsedates` | flag | When set, will attempt to parse columns as dates. |  |
| `--infer-len` | string | Number of rows to scan when inferring schema. Set to 0 to scan entire file. | `10000` |
| `--decimal-comma` | flag | Use comma as decimal separator when READING the input. Note that you will need to specify an alternate --delimiter. |  |
| `--ignore-errors` | flag | Skip rows that can't be parsed. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| Option | Type | Description | Default |
|--------|------|-------------|--------|
| `-h, --help` | flag | Display this message |  |
| `-o, --output` | string | Write output to <file> instead of stdout. |  |
| `-d, --delimiter` | string | The field delimiter for reading/writing CSV data. Must be a single character. (default: ,) |  |
| `-q, --quiet` | flag | Do not return smart aggregation chosen nor pivot result shape to stderr. |  |

---
**Source:** [`src/cmd/pivotp.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/pivotp.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
