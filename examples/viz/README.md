# `qsv viz` examples

Sample datasets and ready-to-run commands that showcase every `qsv viz` chart
type. All commands assume you're in this directory and `qsv` is on your `PATH`
(built with the `viz` feature). Each writes a self-contained, interactive HTML
file you can open in any browser; swap the `.html` extension for `.png`/`.svg`/
`.pdf`/`.jpeg`/`.webp` to export a static image (requires a `viz_static` build
and a local Chrome/Firefox).

## Gallery

[`gallery.html`](gallery.html) is a single, lightweight page rendering every
chart type below from these datasets — handy for linking from the wiki. It
loads plotly from the CDN (so the file stays small), and is a generated
artifact: regenerate it (after changing viz output or the datasets) by building
qsv and running [`gen_gallery.py`](gen_gallery.py) from the repo root —
`python3 examples/viz/gen_gallery.py`. Individual `qsv viz` outputs are instead
fully self-contained (plotly embedded), so they work offline.

**▶ View it rendered** (via githack, which serves the file with the correct
`text/html` type — one-click "External Content Notice" the first time):

<https://raw.githack.com/dathere/qsv/master/examples/viz/gallery.html>

**Raw file** (downloads / shows source — `raw.githubusercontent.com` serves HTML
as `text/plain`, so a browser won't render it):

<https://raw.githubusercontent.com/dathere/qsv/master/examples/viz/gallery.html>

> Note: the GitHub `htmlpreview.github.io` wrapper does **not** work for this page
> — it drops external CDN scripts and can't handle Plotly, so charts render blank.
> Use the githack link above, or open the downloaded file in a browser.

## Datasets

| File | Shape | Used by |
|------|-------|---------|
| `sales_sample.csv` | 500 e-commerce orders: categoricals, a boolean, a rating, several correlated numerics, an ID and a high-cardinality text column | `smart`, `bar`, `line`, `scatter` (incl. bubble & 3D), `histogram`, `box`, `pie`, `heatmap`, `contour` |
| `stock_prices.csv` | 90 trading days of `date,open,high,low,close,volume` | `smart` (time-series), `candlestick`, `ohlc`, `line` |
| `web_flows.csv` | `source,target,sessions` funnel edges | `sankey` |
| `product_ratings.csv` | `brand` + 6 numeric score axes (multiple reviews per brand) | `radar` |
| `quakes.csv` | 40 world cities with `lat,lon,magnitude,depth_km,region` | `smart` (auto geo panel — global extent), `map` (points & density), `geo` (projection) |
| `customer_spend.csv` | 300 customers: a bimodal `monthly_spend`, a right-skewed `account_age_days`, plan/region categoricals, an ID | `smart --smarter` (moarstats-informed: histogram + box hints) |
| `seismic_events.csv` | 417 synthetic Japanese earthquakes: `timestamp`, `lat`/`lon`, a bimodal `depth_km`, a right-skewed `magnitude` correlated with `felt_reports`, a `tsunami` boolean, `region`, an ID | `smart --smarter` (the full geospatial dashboard: map + time-series + correlation + scatter + histogram + boxes + bars) |

## The smart dashboard

`viz smart` auto-profiles the dataset (from qsv's stats cache) and picks a panel
per column: a **correlation heatmap** over the numeric columns, **box plots** for
continuous numerics, and **frequency bars** for low-cardinality categoricals,
booleans, and ratings. The box plots overlay sample points based on the dataset
size — every point for small data, just the Tukey outliers for medium data, and
none for large data (where the box stays a fast cache-only quartile summary);
pass `--box-points <outliers|all|suspected|none>` to force a mode. When the
numeric columns have a strongly correlated pair,
a **scatter** of that pair is added next to the heatmap as a drill-down — or, for
large datasets where a scatter would overplot, a **2D density contour** of the
pair instead. With three or more numeric columns, a **3D scatter** of the
strongest-correlation triple is added as well. When the data has a date/datetime
column (auto-detected via stats date inference) plus a continuous numeric column,
a **time-series trend** panel of that column over time is added too. When a
latitude/longitude column pair is detected, a **geographic map** panel leads the
dashboard — drawn on mapbox tiles for local extents, or as an offline
**projection world-overview** (ScatterGeo, no tiles or token) when the
coordinates span a continental/global area. ID-like and high-cardinality text
columns are skipped.

On large datasets `viz smart` keeps the page light and interactive: each
data-heavy panel (map, time-series, correlated-pair scatter, 3D scatter) is
uniformly downsampled to at most 50,000 points (the correlated-pair density
contour instead embeds only a fixed bin grid, so it stays compact at any row
count), and the map view is framed to the bulk of
the coordinates (a 2.5% trim on each axis) so a few stray geocodes can't zoom it
out to nothing. The map panel also adapts to volume — at ~20,000+ mappable rows
it renders as a **density heatmap** (individual markers would overplot into a
solid blob), and below that as semi-transparent point markers. (These caps apply
only to the `smart` dashboard; the standalone chart commands below plot every
row and frame the full extent.)

### moarstats-informed dashboards

Pass **`--smarter`** to have `viz smart` run
[`qsv moarstats --advanced`](https://github.com/dathere/qsv/blob/master/docs/help/moarstats.md)
itself before building the dashboard (or run `qsv moarstats` first by hand) — either way
`viz smart` reads the extended statistics from the stats cache and makes better chart choices
(with neither, the behavior is unchanged):

- a **bimodal/multimodal** continuous column (high bimodality coefficient) renders
  as a **histogram** instead of a box plot, which would hide the separate peaks;
- **box panels are annotated** with the column's skew direction and outlier share
  (e.g. `account_age_days (right-skewed, 4.7% outliers)`), from the Pearson
  skewness and outlier-percentage stats;
- a **concentrated** high-cardinality categorical that would normally be skipped as
  ID-like noise is kept as a top-N bar (when its normalized entropy is low).

```bash
# one step: --smarter runs `qsv moarstats --advanced` itself, then builds the dashboard
qsv viz smart customer_spend.csv --smarter -o spend_dashboard.html
# monthly_spend (bimodal) -> histogram; account_age_days (skewed) -> annotated box

# or do it in two steps — extend the stats cache first, then let viz smart reuse it
qsv moarstats --advanced customer_spend.csv
qsv viz smart customer_spend.csv -o spend_dashboard.html
```

> `moarstats --advanced` (which `--smarter` runs for you) reads the whole file and auto-creates
> an `.idx` index; the bimodality test needs the advanced stats, while the skew/outlier box hints
> work from a plain `qsv moarstats` run. `--smarter` applies only with default parsing — inputs
> using `--no-headers` or a custom `--delimiter` fall back to the standard dashboard.

```bash
# 12 panels from sales_sample.csv (>8, so it renders as an inline-div grid)
qsv viz smart sales_sample.csv -o dashboard.html

# cap the panel count, lay out 3 columns, top-5 categories per bar
qsv viz smart sales_sample.csv --max-charts 6 --grid-cols 3 --limit 5 -o dashboard.html

# stock_prices has a date column, so the dashboard leads with a time-series trend
qsv viz smart stock_prices.csv -o stocks_dashboard.html

# quakes has lat/lon, so the dashboard leads with a geographic map panel
qsv viz smart quakes.csv -o quakes_dashboard.html

# the full geospatial dashboard: a map, a time-series, a correlation heatmap + drill-down
# scatter, a bimodal-depth histogram, annotated boxes and frequency bars — all auto-chosen.
# Recognized lat/lon columns are charted on the map only, not as redundant distribution panels.
# Rendered with the built-in plotly_dark theme (--theme works on every chart type, incl. smart).
qsv viz smart seismic_events.csv --smarter --theme plotly_dark --grid-cols 3 -o seismic_dashboard.html
```

## Individual chart types

```bash
# bar — revenue by region (aggregated)
qsv viz bar sales_sample.csv --x region --y revenue --agg sum -o bar.html

# line — closing price over time
qsv viz line stock_prices.csv --x date --y close -o line.html

# scatter — units sold vs revenue
qsv viz scatter sales_sample.csv --x units_sold --y revenue -o scatter.html

# scatter (bubble) — encode numeric columns as marker size & color (continuous colorscale)
qsv viz scatter sales_sample.csv --x units_sold --y revenue --size shipping_cost --color profit_margin_pct -o bubble.html

# scatter3d — three numeric columns in 3D; marker color by a fourth
qsv viz scatter3d sales_sample.csv --x units_sold --y revenue --z shipping_cost --color profit_margin_pct -o scatter3d.html

# histogram — distribution of unit price
qsv viz histogram sales_sample.csv --x unit_price -o histogram.html

# box — spread of revenue (true Tukey whiskers; points beyond the fences are outliers)
qsv viz box sales_sample.csv --y revenue -o box.html

# box (grouped) — revenue per region; --box-points all overlays every (jittered) point
qsv viz box sales_sample.csv --y revenue --x region --box-points all -o box_grouped.html

# pie (donut) — revenue share by product category
qsv viz pie sales_sample.csv --x product_category --y revenue --donut -o pie.html

# heatmap (correlation) — Pearson matrix over all numeric columns
qsv viz heatmap sales_sample.csv -o heatmap_corr.html

# scatter (correlated pair) — the strongest pair from the matrix; viz smart auto-adds this
qsv viz scatter sales_sample.csv --x discount_pct --y profit_margin_pct -o corr_pair.html

# contour — 2D density of two numeric columns (binned); viz smart uses this for big data
qsv viz contour sales_sample.csv --x units_sold --y revenue --bins 20 -o contour.html

# heatmap (pivot) — region x category grid of revenue (give --x, --y and --z)
qsv viz heatmap sales_sample.csv --x region --y product_category --z revenue -o heatmap_pivot.html

# candlestick — OHLC price action
qsv viz candlestick stock_prices.csv --x date --ohlc-open open --high high --low low --close close -o candlestick.html

# ohlc — open-high-low-close bars
qsv viz ohlc stock_prices.csv --x date --ohlc-open open --high high --low low --close close -o ohlc.html

# sankey — session funnel (duplicate source->target pairs are aggregated)
qsv viz sankey web_flows.csv --source source --target target --value sessions -o sankey.html

# radar — multi-axis brand comparison (one polygon per --series value, per-axis mean)
qsv viz radar product_ratings.csv --cols battery,camera,performance,display,value,design --series brand -o radar.html

# map — point map on token-free OpenStreetMap tiles; color by magnitude, size by depth
qsv viz map quakes.csv --lat lat --lon lon --color magnitude --size depth_km -o map.html

# map (density) — DensityMapbox heatmap of the same points, on a light Carto basemap
qsv viz map quakes.csv --lat lat --lon lon --density --style carto-positron -o map_density.html

# geo — offline projection map (no tiles/token); viz smart auto-uses this for global coordinates
qsv viz geo quakes.csv --lat lat --lon lon --color magnitude --projection natural-earth -o geo.html
```

> Note: `--ohlc-open` is spelled out (not `--open`) because `--open` already means
> "open the result in a browser".
