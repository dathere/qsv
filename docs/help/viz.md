# viz

> Generate interactive charts (bar, line, scatter, histogram, box) and an auto-dashboard (`viz smart`) from CSV data using [plotly](https://plotly.com). `viz smart` "automagically" picks an appropriate chart per column from the dataset's statistics & frequency distributions (box plots for continuous columns from precomputed quartiles; frequency bars for low-cardinality/boolean columns). Outputs self-contained, interactive HTML (works offline) - or static PNG/SVG/PDF/JPEG/WebP with the `viz_static` feature - and can `--open` the result in your browser.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/viz.rs](https://github.com/dathere/qsv/blob/master/src/cmd/viz.rs)** | [🪄](TableOfContents.md#legend "\"automagical\" commands that uses stats and/or frequency tables to work \"smarter\" & \"faster\".")[👆](TableOfContents.md#legend "has powerful column selector support. See `select` for syntax.")

<a name="nav"></a>
[Description](#description) | [Examples](#examples) | [Usage](#usage) | [Viz Options](#viz-options) | [Smart Options](#smart-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Generate charts from CSV data using the plotly charting library.

Produces a self-contained, interactive HTML chart (the plotly.js runtime is embedded,
so the output works offline). With a qsv build that includes the `viz_static` feature,
charts can also be exported as static PNG/SVG/PDF/JPEG/WebP images (this requires a
Chromium/Firefox browser at runtime - a webdriver is auto-managed by plotly).

The output format is inferred from the --output file extension (.html is the default).
Interactive HTML is written to stdout when --output is not given; image formats always
require --output. Use --open to view the result in your default browser/viewer.

Chart types (subcommands):  
smart      Auto-dashboard. Picks an appropriate chart per column from the
dataset's statistics & frequency distribution (no --x/--y needed).
bar        Bar chart.        --x = category column, --y = value column.
line       Line chart.       --x = x column, --y = y column.
scatter    Scatter plot.     --x = x column, --y = y column.
histogram  Distribution.     --x = numeric column to bin.
box        Box plot.         --y = value column, optional --x = group column.

`qsv viz smart` builds a one-page dashboard of subplots by reusing qsv's stats and
frequency caches: continuous numeric columns become box plots (drawn from precomputed
quartiles, so no data is re-scanned), and low-cardinality / boolean columns become
frequency bar charts. ID-like (near-unique) and all-empty columns are skipped. The
first run computes & caches stats; subsequent runs are fast.


<a name="examples"></a>

## Examples [↩](#nav)

> Auto-dashboard for a dataset, opened in the browser

```console
qsv viz smart data.csv --open
```

> Auto-dashboard, at most 6 panels in a 3-column grid, top-5 categories per bar

```console
qsv viz smart data.csv --max-charts 6 --grid-cols 3 --limit 5 -o dashboard.html
```

> Bar chart of fruit prices, opened in the browser

```console
qsv viz bar fruits.csv --x Fruit --y Price --title "Fruit prices" --open
```

> Aggregate (sum) sales by region into a bar chart

```console
qsv viz bar sales.csv --x region --y amount --agg sum -o sales.html
```

> Scatter plot with a separate series (trace) per category

```console
qsv viz scatter data.csv --x age --y income --series gender -o scatter.html
```

> Histogram of a numeric column with 30 bins

```console
qsv viz histogram data.csv --x value --bins 30 -o hist.html
```

> Box plot of a value column grouped by a category, exported to PNG (needs viz_static)

```console
qsv viz box data.csv --y measurement --x group -o box.png
```

For more examples, see [tests](https://github.com/dathere/qsv/blob/master/tests/test_viz.rs).


<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv viz smart     [options] <input>
qsv viz bar       [options] <input>
qsv viz line      [options] <input>
qsv viz scatter   [options] <input>
qsv viz histogram [options] <input>
qsv viz box       [options] <input>
qsv viz --help
```

<a name="viz-options"></a>

## Viz Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑x,`<br>`‑‑x`&nbsp; | string | Column for the x-axis / category / bin / group. |  |
| &nbsp;`‑y,`<br>`‑‑y`&nbsp; | string | Column for the y-axis / value. |  |
| &nbsp;`‑‑series`&nbsp; | string | Column to split into multiple series (one trace per distinct value). Applies to bar/line/scatter. |  |
| &nbsp;`‑‑bins`&nbsp; | integer | Number of bins for the histogram. (default: auto) |  |
| &nbsp;`‑‑agg`&nbsp; | string | For bar/line, aggregate the y values when the x value repeats. One of: sum, mean, count, min, max. |  |

<a name="smart-options"></a>

## Smart Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑‑max‑charts`&nbsp; | integer | Maximum number of panels in the dashboard. Capped at 8 (plotly's typed subplot-axis limit); extra eligible columns are reported but not drawn. | `8` |
| &nbsp;`‑‑grid‑cols`&nbsp; | integer | Number of columns in the dashboard grid. | `2` |
| &nbsp;`‑‑limit`&nbsp; | integer | Top-N categories per frequency bar chart. | `10` |
| &nbsp;`‑‑title`&nbsp; | string | Chart title. |  |
| &nbsp;`‑‑x‑title`&nbsp; | string | X-axis title. (defaults to the x column name) |  |
| &nbsp;`‑‑y‑title`&nbsp; | string | Y-axis title. (defaults to the y column name) |  |
| &nbsp;`‑‑width`&nbsp; | integer | Image width in pixels (static export). | `1000` |
| &nbsp;`‑‑height`&nbsp; | integer | Image height in pixels (static export). | `600` |
| &nbsp;`‑‑scale`&nbsp; | float | Image scale factor (static export). | `1.0` |
| &nbsp;`‑‑open`&nbsp; | flag | Open the generated chart in the default browser/viewer. |  |

<a name="common-options"></a>

## Common Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑h,`<br>`‑‑help`&nbsp; | flag | Display this message |  |
| &nbsp;`‑o,`<br>`‑‑output`&nbsp; | string | Write output to <file> instead of stdout. The chart format is inferred from the extension: .html (default), .png, .svg, .pdf, .jpeg, .webp. |  |
| &nbsp;`‑d,`<br>`‑‑delimiter`&nbsp; | string | The field delimiter for reading CSV data. Must be a single character. (default: ,) |  |
| &nbsp;`‑n,`<br>`‑‑no‑headers`&nbsp; | flag | When set, the first row will not be interpreted as headers. Columns can then only be selected by index. |  |

---
**Source:** [`src/cmd/viz.rs`](https://github.com/dathere/qsv/blob/master/src/cmd/viz.rs)
| **[Table of Contents](TableOfContents.md)** | **[README](../../README.md)**
