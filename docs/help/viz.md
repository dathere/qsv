# viz

> Generate interactive charts (bar, line, scatter, histogram, box, pie, heatmap, candlestick/ohlc, sankey, radar, treemap, sunburst, geographic maps) and an auto-dashboard (`viz smart`) from CSV data using [plotly](https://plotly.com). `viz smart` "automagically" picks an appropriate chart per column from the dataset's statistics & frequency distributions (box plots for continuous columns from precomputed quartiles; frequency bars for low-cardinality/boolean columns; a correlation heatmap when there are 2+ eligible continuous numeric columns; a treemap/sunburst part-to-whole hierarchy when 2+ low-cardinality dimensions exist; a map panel when a lat/lon column pair is detected). Outputs self-contained, interactive HTML (charts work offline; map basemaps fetch their tiles over the network unless the `white-bg` style is used) - or static PNG/SVG/PDF/JPEG/WebP with the `viz_static` feature - and can `--open` the result in your browser.

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/viz.rs](https://github.com/dathere/qsv/blob/master/src/cmd/viz.rs)** | [🪄](TableOfContents.md#legend "\"automagical\" commands that uses stats and/or frequency tables to work \"smarter\" & \"faster\".")[👆](TableOfContents.md#legend "has powerful column selector support. See `select` for syntax.")

<a name="nav"></a>
[Description](#description) | [Examples](#examples) | [Usage](#usage) | [Viz Options](#viz-options) | [Map Options](#map-options) | [Geo Options](#geo-options) | [Smart Options](#smart-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Generate charts from CSV data using the plotly charting library.

Produces a self-contained, interactive HTML chart (the plotly.js runtime is embedded,
so charts work offline; map basemaps fetch their tiles over the network at view time
unless the `white-bg` style is used). With a qsv build that includes the `viz_static`
feature, charts can also be exported as static PNG/SVG/PDF/JPEG/WebP images (this
requires a Chromium/Firefox browser at runtime - a webdriver is auto-managed by plotly).

The output format is inferred from the --output file extension (.html is the default).
Interactive HTML is written to stdout when --output is not given; image formats always
require --output. Use --open to view the result in your default browser/viewer.

Chart types (subcommands):

```text
smart       Auto-dashboard. Picks an appropriate chart per column from the
            dataset's statistics & frequency distribution (no --x/--y needed).
bar         Bar chart.        --x = category column, --y = value column.
line        Line chart.       --x = x column, --y = y column.
scatter     Scatter plot.     --x = x column, --y = y column.
scatter3d   3D scatter plot.  --x, --y, --z = three numeric columns.
histogram   Distribution.     --x = numeric column to bin.
box         Box plot.         --y = value column, optional --x = group column.
pie         Proportions.      --x = label column, optional --y = value column.
heatmap     Color grid. Correlation matrix of numeric columns (default; an
            optional column subset via --cols), or a category x category pivot
            with --x/--y/--z.
contour     2D density contour of two numeric columns (--x and --y), binned
            into a grid (--bins controls the grid resolution).
candlestick Financial OHLC.   --x = date column, plus --ohlc-open/--high/--low/--close.
ohlc        Financial OHLC bars (same inputs as candlestick).
sankey      Flow diagram.     --source, --target, optional --value column.
radar       Polar/radar chart of numeric --cols, optional --series per trace.
map         Geographic point map (or --density heatmap) on tile basemaps.
            Pick the coordinate columns with the lat/lon options below.
geo         Geographic point map on a projection basemap (coastlines/land/
            countries; no tiles, no token). Uses the same lat/lon options
            as `map`, plus --projection. Good for global/country-scale data.
```

`qsv viz smart` builds a one-page dashboard of subplots by reusing qsv's stats and
frequency caches. Continuous numeric columns become box plots (quartiles from the stats
cache; sample points are overlaid by a size heuristic - see --box-points), and
low-cardinality / boolean columns become frequency bar charts. ID-like (near-unique) and
all-empty columns are skipped. When the dataset has two or more continuous numeric columns,
a correlation heatmap panel is added (one extra data pass to compute Pearson correlations),
and if the most strongly correlated pair is at least moderately correlated, a drill-down of
that pair is added beside it: a scatter, or a 2D density contour for large datasets where a
scatter would overplot; with three or more numeric columns, a 3D scatter of the
strongest-correlation triple is added too. When the dataset has a date/datetime column
(auto-detected via stats date inference) plus a continuous numeric column, a time-series
line panel over time is added. When a latitude/longitude column pair is detected, a
geographic panel leads the dashboard: for HTML, a Mapbox tile map for a local extent or an
offline ScatterGeo projection world-overview for continental/global data. For static image
export the map is rendered as an offline ScatterGeo projection fit to the data extent (the
Mapbox tile map can't be exported as it needs network tiles); US-spanning data uses an
albers-usa projection. The Mapbox tile map and 3D panels stay HTML-only. Points that lie
far from the cluster centroid (distance beyond the Tukey far-out fence of all points' distances) are
flagged as geographic outliers: they're drawn with a distinct marker, excluded from the spatial
extent (so a few strays don't inflate it), and the map zooms tightly to the core extent (outliers
may then sit off-screen until you zoom out). When those outliers fall within the same jurisdiction
as the core, the spatial-extent label's outlier call-out is suppressed (they're the cluster's far
edge, not strays elsewhere). When qsv is built with the `geocode` feature, the map's (core) spatial extent
(its 4 bounding-box corners + center) is reverse-geocoded against the local Geonames index and
drawn on the map as a bounding box with labeled points, plus a consolidated location summary below
it (e.g. "New York & New Jersey, United States"); any outliers are called out there too with their
count and jurisdiction (e.g. "... - 3 outliers (Pennsylvania)"). When there are outliers, a second
dotted box with no fill marks the full extent (core + outliers), so the strays' span is visible
alongside the core box, and the interactive HTML map gets "Core extent" / "Full extent" buttons to
jump between the two views (the map opens at the tight core view). In HTML the points reveal their
city/state/country on hover; static exports show the box without hover. The first such run may
download the Geonames index (~13MB, cached in ~/.qsv-cache); if it's unavailable (offline) the map
still renders without the overlay. Extents that span the antimeridian (>180 degrees of longitude)
are skipped. These overview
panels (map/geo, correlation heatmap and its drill-downs, time-series) each lead the dashboard
on their own full-width row; the per-column box/bar/histogram panels flow below in the
multi-column grid (see --grid-cols). The first run computes & caches stats; subsequent runs
are fast.


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

> Bubble scatter: marker size by population, marker color by a numeric score

```console
qsv viz scatter data.csv --x gdp --y life_exp --size population --color score -o bubble.html
```

> Histogram of a numeric column with 30 bins

```console
qsv viz histogram data.csv --x value --bins 30 -o hist.html
```

> Box plot of a value column grouped by a category, exported to PNG (needs viz_static)

```console
qsv viz box data.csv --y measurement --x group -o box.png
```

> Box plot with every sample point overlaid (jittered) instead of just the outliers

```console
qsv viz box data.csv --y measurement --box-points all -o box.html
```

> Pie chart of category proportions (counts), as a donut

```console
qsv viz pie data.csv --x category --donut -o pie.html
```

> Correlation heatmap over all numeric columns

```console
qsv viz heatmap data.csv -o corr.html
```

> Heatmap pivot: average value per (region x product)

```console
qsv viz heatmap sales.csv --x region --y product --z amount -o pivot.html
```

> Candlestick chart from a date column and OHLC price columns

```console
qsv viz candlestick prices.csv --x date --ohlc-open open --high high --low low --close close -o ohlc.html
```

> Sankey flow diagram of source -> target weighted by value

```console
qsv viz sankey flows.csv --source from --target to --value weight -o sankey.html
```

> Radar chart comparing numeric metrics, one trace per team

```console
qsv viz radar teams.csv --cols speed,power,range,accuracy --series team -o radar.html
```

> Point map of earthquakes, marker color by magnitude and size by depth

```console
qsv viz map quakes.csv --lat lat --lon lon --color magnitude --size depth -o map.html
```

> Density heatmap of the same points, on a light Carto basemap

```console
qsv viz map quakes.csv --lat lat --lon lon --density --style carto-positron -o heat.html
```

> 3D scatter of three numeric columns, colored by a fourth

```console
qsv viz scatter3d data.csv --x length --y width --z height --color weight -o scatter3d.html
```

> 2D density contour of two numeric columns with a 40x40 grid

```console
qsv viz contour data.csv --x height --y weight --bins 40 -o contour.html
```

> Projection map of earthquakes (token-free), marker color by magnitude

```console
qsv viz geo quakes.csv --lat lat --lon lon --color magnitude --projection natural-earth -o geo.html
```

> Treemap of part-to-whole sales by region then category, sized by amount

```console
qsv viz treemap sales.csv --cols region,category --value amount --agg sum -o treemap.html
```

> Sunburst of a deep 3-level web-traffic hierarchy, sized by row count

```console
qsv viz sunburst web.csv --cols source,campaign,landing_page -o sunburst.html
```

For more examples, see [tests](https://github.com/dathere/qsv/blob/master/tests/test_viz.rs).

See also <https://github.com/dathere/qsv/wiki/Visualization>

<a name="usage"></a>

## Usage [↩](#nav)

```console
qsv viz smart       [options] <input>
qsv viz bar         [options] <input>
qsv viz line        [options] <input>
qsv viz scatter     [options] <input>
qsv viz scatter3d   [options] <input>
qsv viz histogram   [options] <input>
qsv viz box         [options] <input>
qsv viz pie         [options] <input>
qsv viz heatmap     [options] <input>
qsv viz contour     [options] <input>
qsv viz candlestick [options] <input>
qsv viz ohlc        [options] <input>
qsv viz sankey      [options] <input>
qsv viz radar       [options] <input>
qsv viz map         [options] <input>
qsv viz geo         [options] <input>
qsv viz treemap     [options] <input>
qsv viz sunburst    [options] <input>
qsv viz --help
```

<a name="viz-options"></a>

## Viz Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑x,`<br>`‑‑x`&nbsp; | string | Column for the x-axis / category / bin / group. |  |
| &nbsp;`‑y,`<br>`‑‑y`&nbsp; | string | Column for the y-axis / value. |  |
| &nbsp;`‑z,`<br>`‑‑z`&nbsp; | string | The z column: a heatmap pivot value (with --x and --y), or the third numeric axis for scatter3d. |  |
| &nbsp;`‑‑cols`&nbsp; | string | Columns to use. For heatmap: numeric columns for the correlation matrix (default: all numeric). For radar: the numeric axes to plot. For treemap/sunburst: the categorical dimensions that form the hierarchy levels, outermost first (e.g. region,category,subcategory). |  |
| &nbsp;`‑‑series`&nbsp; | string | Column to split into multiple series (one trace per distinct value). Applies to bar, line, scatter, scatter3d, radar, map and geo. |  |
| &nbsp;`‑‑color`&nbsp; | string | For scatter/scatter3d/map/geo: a numeric column to encode as marker color (a continuous colorscale with a colorbar). For categorical coloring, use the --series option instead. Cannot be combined with --series. In map density mode, this column is the heatmap weight. |  |
| &nbsp;`‑‑size`&nbsp; | string | For scatter/scatter3d/map/geo: a numeric column to encode as marker size, producing a bubble chart (values are rescaled to a readable pixel range). Cannot be combined with --series. In map density mode, this column is the heatmap weight. |  |
| &nbsp;`‑‑donut`&nbsp; | flag | Render a pie chart as a donut (with a center hole). |  |
| &nbsp;`‑‑ohlc‑open`&nbsp; | string | Open-price column for candlestick/ohlc charts. |  |
| &nbsp;`‑‑high`&nbsp; | string | High-price column for candlestick/ohlc charts. |  |
| &nbsp;`‑‑low`&nbsp; | string | Low-price column for candlestick/ohlc charts. |  |
| &nbsp;`‑‑close`&nbsp; | string | Close-price column for candlestick/ohlc charts. |  |
| &nbsp;`‑‑source`&nbsp; | string | Source node column for a sankey diagram. |  |
| &nbsp;`‑‑target`&nbsp; | string | Target node column for a sankey diagram. |  |
| &nbsp;`‑‑value`&nbsp; | string | Flow value column for a sankey diagram. When omitted, each row counts as a flow of 1. For treemap/sunburst: a numeric measure summed per sector (when omitted, each row counts as 1). |  |
| &nbsp;`‑‑bins`&nbsp; | integer | Number of bins. For histogram: bins along the x-axis (default: auto). For contour: the per-axis resolution of the density grid (default: 20). |  |
| &nbsp;`‑‑agg`&nbsp; | string | For bar/line, aggregate the y values when the x value repeats. One of: sum, mean, count, min, max. For treemap/sunburst, only additive aggregations apply: count (default) or sum (requires --value). |  |
| &nbsp;`‑‑box‑points`&nbsp; | string | Which sample points to draw alongside a box. Reading the raw values lets plotly render true Tukey whiskers (1.5*IQR) with the points beyond the fences as outliers. One of: outliers (only the outliers), all (every point, jittered), suspected (mark suspected outliers), none (no points, but still real Tukey whiskers). For `viz box` the default is outliers. For `viz smart` this flag OVERRIDES the default size-based heuristic, which overlays all points for small data (<=1,000 rows) and only the outliers for medium data (<=10,000 rows). Above that, a column that HAS outliers shows them as points on a precomputed quartile box (a single pass collects only the out-of-fence values, capped); a column with no outliers stays a fast cache-only quartile summary with no data re-scan. An explicit mode is applied to every box panel (one batched pass to read the values), except `none`, which always keeps the cache-only box. |  |

<a name="map-options"></a>

## Map Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑‑lat`&nbsp; | string | Latitude column for a map (decimal degrees, -90 to 90). |  |
| &nbsp;`‑‑lon`&nbsp; | string | Longitude column for a map (decimal degrees, -180 to 180). |  |
| &nbsp;`‑‑text`&nbsp; | string | Column whose value labels each point on hover. |  |
| &nbsp;`‑‑density`&nbsp; | flag | Render a density heatmap (DensityMapbox) instead of points. Weighted by the --color or --size column when given, else by a uniform weight. Cannot be combined with --series. |  |
| &nbsp;`‑‑style`&nbsp; | string | Map basemap style. Token-free styles: open-street-map (the default), carto-positron, carto-darkmatter, stamen-terrain, stamen-toner, stamen-watercolor, white-bg. Mapbox-hosted styles (basic, streets, outdoors, light, dark, satellite, satellite-streets) require --mapbox-token. | `open-street-map` |
| &nbsp;`‑‑mapbox‑token`&nbsp; | string | Mapbox access token, required only for the mapbox-hosted basemap styles listed above. Can also be set with the QSV_MAPBOX_TOKEN environment variable (the --mapbox-token flag takes precedence). |  |

<a name="geo-options"></a>

## Geo Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑‑projection`&nbsp; | string | Map projection for `viz geo`. One of: natural-earth (the default), mercator, orthographic, equirectangular, albers-usa, robinson, winkel-tripel, mollweide, hammer, azimuthal-equal-area. `viz geo` also reuses the lat, lon, text, color, size and series options from `map`. | `natural-earth` |

<a name="smart-options"></a>

## Smart Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑‑max‑charts`&nbsp; | integer | Maximum number of panels in the dashboard. 0 (the default) means auto: draw every eligible column (up to 64), for both HTML and static image export (png/svg/pdf/...). Up to 8 cartesian panels render as one typed subplot grid; beyond 8, HTML switches to an inline-div grid of independent plots, and static image export uses domain-positioned axes to fit them in one image. Set a positive <n> to cap the panel count instead. Eligible columns beyond the cap are reported but not drawn. | `0` |
| &nbsp;`‑‑grid‑cols`&nbsp; | integer | Number of columns in the dashboard grid for the per-column distribution panels. Overview panels (map/geo, correlation, time-series) always span the full width. | `2` |
| &nbsp;`‑‑limit`&nbsp; | integer | Top-N categories per frequency bar chart. | `10` |
| &nbsp;`‑‑no‑nulls`&nbsp; | flag | Omit the "(NULL)" bar (empty cells) from frequency bar charts. By default `viz smart` shows a "(NULL)" bar, like `qsv frequency`. |  |
| &nbsp;`‑‑no‑other`&nbsp; | flag | Omit the "Other (N)" aggregate bar from frequency bar charts. It collects the categories beyond --limit (N = how many distinct categories were rolled up) and is shown by default. |  |
| &nbsp;`‑‑smarter`&nbsp; | flag | Before building the dashboard, run `qsv moarstats --advanced` to enrich the stats cache with distribution-shape statistics (bimodality, entropy, skewness, outlier share). This unlocks histograms for bimodal columns, frequency bars for concentrated high-cardinality columns, and skew/outlier hints on box panels. Costs one extra pass over the data and writes <stem>.stats.csv, its sidecars, and an .idx index (like running `qsv moarstats` manually). Only affects `smart`. Applied only with default parsing; inputs using --no-headers or a custom --delimiter fall back to the standard dashboard. |  |
| &nbsp;`‑‑hierarchy‑style`&nbsp; | string | For `smart`, the chart used for the categorical part-to-whole hierarchy panel (built when 2+ low-cardinality dimensions exist). One of: auto (default), treemap, sunburst. auto follows best practice — a treemap for a shallow 2-level hierarchy (accurate size comparison) and a sunburst for a deep 3-level one (parent child structure). Only affects `smart`. |  |
| &nbsp;`‑‑dictionary`&nbsp; | string | EXPERIMENTAL. Use a describegpt Data Dictionary to guide panel selection from each field's semantic role/concept (falling back to its content type) instead of relying on column statistics alone: dimensions and numeric codes (ward, census_tract, zone) become bars, measures get box/correlation/trend panels, date/datetime columns feed the time-series panel (not noisy frequency bars), identifiers / PII / free-text are skipped, and lat/lon feed the map. Field labels become panel titles. Columns the dictionary cannot classify still use the statistical heuristic. <src> is one of: "infer" to run describegpt on the input now (with infer-content-type, two-pass and jsonschema output; requires an LLM configured) and use its output; or a path to an existing describegpt dictionary file (jsonschema or json). Generation/read failures soft-fall back to the stats-only dashboard. Only affects `smart`. |  |
| &nbsp;`‑‑dictionary‑context`&nbsp; | string | Path to a file with extra context about the dataset (a glossary, README, data dictionary, PDF, etc.) forwarded to describegpt as --context-file when `--dictionary infer` generates the dictionary. Better context yields better role/concept/label/grain tags, hence a better dashboard. Ignored unless `--dictionary infer` is used (it does not apply when reading an existing dictionary file). Only affects `smart`. |  |
| &nbsp;`‑‑log‑scale`&nbsp; | string | Use a logarithmic y-axis for frequency bar panels whose tallest bar dwarfs the rest (e.g. a large "(NULL)" or "Other (N)" bucket), so the small categories stay visible. One of: auto, on, off. "auto" (the default) switches a panel to a log y-axis only when its dynamic range is high; "on" forces a log y-axis on every frequency panel; "off" keeps the linear axes. Only affects `smart`. | `auto` |
| &nbsp;`‑‑title`&nbsp; | string | Chart title. |  |
| &nbsp;`‑‑x‑title`&nbsp; | string | X-axis title. (defaults to the x column name) |  |
| &nbsp;`‑‑y‑title`&nbsp; | string | Y-axis title. (defaults to the y column name) |  |
| &nbsp;`‑‑theme`&nbsp; | string | Plotly theme that drives the chart's overall look (background, fonts, axis styling). One of: default, plotly_white, plotly_dark, seaborn, seaborn_whitegrid, seaborn_dark, matplotlib, plotnine (case-insensitive; hyphens accepted). When omitted, qsv's built-in look is used. Applies to all chart types, including `smart`. |  |
| &nbsp;`‑‑width`&nbsp; | integer | Image width in pixels for static export. Default 1000; for `smart`, auto-scaled to the grid's column count. |  |
| &nbsp;`‑‑height`&nbsp; | integer | Image height in pixels for static export. Default 600; for `smart`, auto-scaled to the number of panel rows. |  |
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
