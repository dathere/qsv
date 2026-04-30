# pivotp

> Pivot CSV data. Features "smart" aggregation auto-selection based on data type & stats.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/pivotp.rs](https://github.com/dathere/qsv/blob/master/src/cmd/pivotp.rs)** | [🚀](TableOfContents.md#legend "multithreaded even without an index.")[🐻‍❄️](TableOfContents.md#legend "command powered/accelerated by  vectorized query engine.")[🪄](TableOfContents.md#legend "\"automagical\" commands that uses stats and/or frequency tables to work \"smarter\" & \"faster\".")

<a name="nav"></a>
[Description](#description) | [Usage](#usage) | [Arguments](#arguments) | [Pivotp Options](#pivotp-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Pivots or groups CSV data using the Polars engine.

PIVOT MODE (with <on-cols>):  
The pivot operation consists of:  
- One or more index columns (these will be the new rows)
- A column that will be pivoted (this will create the new columns)
- A values column that will be aggregated
- An aggregation function to apply. Features "smart" aggregation auto-selection.

GROUP-BY MODE (without <on-cols>):  
When <on-cols> is omitted, performs a group-by aggregation instead of a pivot.
This is useful for simple aggregations like counting rows per group.
In group-by mode, --index is required and --agg smart resolves to len (count).
The none aggregation is not supported in group-by mode.
If --values is omitted, a single "count" column is produced.

For examples, see <https://github.com/dathere/qsv/blob/master/tests/test_pivotp.rs>.


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv pivotp [options] <on-cols> <input>
qsv pivotp [options] <input>
qsv pivotp --help
```

<a name="arguments"></a>

## Arguments [↩](#nav)

| &nbsp;Argument&nbsp;&nbsp; | Description |
|----------|-------------|
| &nbsp;`<on-cols>`&nbsp; | The column(s) to pivot on (creates new columns). When omitted, pivotp runs in group-by mode. |
| &nbsp;`<input>`&nbsp; | The input CSV file. The file must have headers. If the file has a pschema.json file, it will be used to inform the pivot operation unless --infer-len is explicitly set to a value other than the default of 10,000 rows. Stdin is not supported. |

<a name="pivotp-options"></a>

## Pivotp Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑i,`<br>`‑‑index`&nbsp; | string | The column(s) to use as the index (row labels). Specify multiple columns by separating them with a comma. The output will have one row for each unique combination of the index's values. In pivot mode, if None, all remaining columns not specified on --on and --values will be used; at least one of --index and --values must be specified. Required in group-by mode. |  |
| &nbsp;`‑v,`<br>`‑‑values`&nbsp; | string | The column(s) containing values to aggregate. If an aggregation is specified, these are the values on which the aggregation will be computed. In pivot mode, if None, all remaining columns not specified on --on and --index will be used; at least one of --index and --values must be specified. In group-by mode, if omitted, a single "count" column is produced. |  |
| &nbsp;`‑a,`<br>`‑‑agg`&nbsp; | string | The aggregation function to use: first - First value encountered last - Last value encountered sum - Sum of values min - Minimum value max - Maximum value mean - Average value median - Median value len - Count of values item - Get single value from group. Raises error if there are multiple values. smart - use value column data type & statistics to pick an aggregation. Always uses type, cardinality, sparsity, CV, sign distribution (n_negative/n_positive), and sort_order from streaming stats. When the stats cache includes non-streaming stats (from a prior `stats --everything` or `stats --mode --quartiles`), also uses skewness and mode_count. When moarstats has been run, also leverages outlier profile, Pearson skewness, MAD/stddev ratio, median/mean ratio, and quartile coefficient of dispersion for smarter selection. With moarstats --advanced, also uses kurtosis, bimodality, entropy and Gini coefficient. For Date/DateTime values, checks sparsity and sort order. Will only work if there is one value column, otherwise it falls back to `first` | `smart` |
| &nbsp;`‑‑sort‑columns`&nbsp; | flag | Sort the transposed columns by name. (pivot mode only) |  |
| &nbsp;`‑‑maintain‑order`&nbsp; | flag | Maintain output order: preserve input column order in pivot mode, and preserve group/row order in group-by mode. |  |
| &nbsp;`‑‑col‑separator`&nbsp; | string | The separator in generated column names in case of multiple --values columns. (pivot mode only; ignored in group-by mode) | `_` |
| &nbsp;`‑‑validate`&nbsp; | flag | Validate a pivot by checking the pivot column(s)' cardinality. (pivot mode only) |  |
| &nbsp;`‑‑try‑parsedates`&nbsp; | flag | When set, will attempt to parse columns as dates. |  |
| &nbsp;`‑‑infer‑len`&nbsp; | string | Number of rows to scan when inferring schema. Set to 0 to scan entire file. | `10000` |
| &nbsp;`‑‑decimal‑comma`&nbsp; | flag | Use comma as decimal separator when READING the input. Note that you will need to specify an alternate --delimiter. |  |
| &nbsp;`‑‑ignore‑errors`&nbsp; | flag | Skip rows that can't be parsed. |  |
| &nbsp;`‑‑grand‑total`&nbsp; | flag | Append a grand total row summing all numeric non-index columns. The first index column will contain "Grand <total-label>". |  |
| &nbsp;`‑‑subtotal`&nbsp; | flag | Insert subtotal rows after each group in the first index column. The second index column will contain the total label. Requires 2+ index columns. (pivot mode only) |  |
| &nbsp;`‑‑total‑label`&nbsp; | string | Custom label for total rows. | `Total` |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑h,`<br>`‑‑help`&nbsp; | flag | Display this message |  |
| &nbsp;`‑o,`<br>`‑‑output`&nbsp; | string | Write output to <file> instead of stdout. |  |
| &nbsp;`‑d,`<br>`‑‑delimiter`&nbsp; | string | The field delimiter for reading/writing CSV data. Must be a single character. (default: ,) |  |
| &nbsp;`‑q,`<br>`‑‑quiet`&nbsp; | flag | Do not return smart aggregation chosen nor pivot result shape to stderr. |  |

---
**Source:** [`src/cmd/pivotp.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/pivotp.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
