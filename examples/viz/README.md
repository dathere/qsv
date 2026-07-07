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

The eleven **smart dashboards** are embedded as `<iframe>`s of their genuine
`qsv viz smart` HTML output (`smart_*.html`) rather than reconstructed inline, so
the full-width overview panels (map, choropleth, correlation heatmap, time-series,
treemap/sunburst hierarchy), themes and map zoom buttons render exactly as the CLI produces
them. Those iframe sources are the real output with the inline plotly bundle swapped for the
same CDN tag (so they stay a few KB each); they need a network connection to render. Four of
them (`smart_dict_treemap.html`, `smart_dict_sunburst.html`, `smart_world_choropleth.html`,
`smart_nyc311.html`) are `--dictionary infer` examples that need a local LLM to regenerate, so
`gen_gallery.py` reuses the committed copies instead of re-running the LLM.

**▶ View it rendered** (GitHub Pages, served with the correct `text/html` type):

<https://dathere.github.io/qsv/gallery.html>

This directory is published to GitHub Pages by
[`.github/workflows/viz-gallery-pages.yml`](../../.github/workflows/viz-gallery-pages.yml)
on every push to `master`, so the gallery — and its embedded `smart_*.html`
iframes — render directly, no third-party proxy needed.

**Raw file** (downloads / shows source — `raw.githubusercontent.com` serves HTML
as `text/plain`, so a browser won't render it):

<https://raw.githubusercontent.com/dathere/qsv/master/examples/viz/gallery.html>

## Datasets

| File | Shape | Used by |
|------|-------|---------|
| `sales_sample.csv` | 500 e-commerce orders: categoricals, a boolean, a rating, several correlated numerics, an ID and a high-cardinality text column | `smart`, `bar`, `line`, `scatter` (incl. bubble & 3D), `histogram`, `box`, `pie`, `heatmap`, `contour` |
| `stock_prices.csv` | 90 trading days of `date,open,high,low,close,volume` | `smart` (time-series), `candlestick`, `ohlc`, `line` |
| `web_flows.csv` | `source,target,sessions` funnel edges | `sankey` |
| `product_ratings.csv` | `brand` + 6 numeric score axes (multiple reviews per brand) | `radar` |
| `quakes.csv` | 40 world cities with `lat,lon,magnitude,depth_km,region` | `smart` (auto geo panel — global extent), `map` (points & density), `geo` (projection) |
| `country_stats.csv` | 20 countries with `iso3,country,gdp_usd_tn` | `choropleth` (fill countries by GDP, matched by ISO-3 code) |
| `us_state_stats.csv` | 20 US states with `state,renewable_electricity_pct` | `choropleth --location-mode usa-states` (built-in state geometry, albers-usa) |
| `western_states.csv` + `western_states.geojson` | 7 near-rectangular western states with `state,wind_capacity_gw`, plus a tiny custom GeoJSON keyed by 2-letter `id` | `choropleth --map --geojson … --feature-id-key id` (filled regions on a MapLibre tile basemap) |
| `world_cities.csv` | **1,179 cities** with population **over 500,000** across **six inhabited continents** (GeoNames-derived): `country`, `continent`, `lat`/`lon`, `metro_population_m`, `elevation_m` (real), `avg_annual_temp_c` (synthesized from latitude + elevation). `continent` uses the [plotly.js geo `scope`](https://plotly.com/javascript/reference/layout/geo/#layout-geo-scope) vocabulary (`Oceania`, `North America`, …) | `smart --dictionary infer` (dense global geo map + per-COUNTRY choropleth via `fitbounds` with `geocode` + a six-continent bar + box panels) |
| `us_cities.csv` | 54 US cities across ~35 states: `lat`/`lon`, `census_region`, `population_m`, `median_age` | `smart` (US point map + per-US-STATE choropleth with `geocode` + box/bar/correlation panels) |
| `customer_spend.csv` | 300 customers: a bimodal `monthly_spend`, a right-skewed `account_age_days`, plan/region categoricals, an ID | `smart --smarter` (moarstats-informed: histogram + box hints) |
| `seismic_events.csv` + `japan_prefectures.geojson` | 417 synthetic Japanese earthquakes (`timestamp`, `lat`/`lon`, a bimodal `depth_km`, a right-skewed `magnitude` correlated with `felt_reports`, a `tsunami` boolean, `region`, an ID), plus a GeoJSON of the 47 prefectures keyed by `properties.id` (ISO&nbsp;3166-2), with a top-level `id` too so no `--feature-id-key` is needed | `smart --smarter --geojson japan_prefectures.geojson` (the full geospatial dashboard: map + **prefecture choropleth via point-in-polygon binning** + time-series + correlation + scatter + histogram + boxes + bars) |
| `delivery_stops.csv` | 90 delivery stops clustered in metro Denver + 4 bad-geocode strays in neighboring states, with `zone`/`vehicle` categoricals, `packages`, and correlated `weight_kg`/`distance_km`/`delivery_minutes` numerics over a `delivered_date` | `smart` (geographic outlier markers + core/full extent boxes, Core/Full zoom buttons & spatial-extent call-out with `geocode`; plus boxes, bars, correlation heatmap, strongest-pair scatter & a time-series — no `--smarter` needed) |
| `nyc_311.csv` + `nyc_neighborhoods.geojson` | **10,000-row** sample of NYC 311 service requests (2010–2020): 41 columns incl. `Latitude`/`Longitude`, `Borough`, `Agency`, `Complaint Type`, `Status`, several date columns, and `X`/`Y Coordinate (State Plane)`, plus a custom GeoJSON of **188 NYC neighborhoods** keyed by top-level `id` | `smart --smarter --dictionary infer --geojson nyc_neighborhoods.geojson` (full municipal dashboard: dense point map + **neighborhood choropleth on a MapLibre tile basemap via point-in-polygon binning** + correlation + time-series + boxes + bars, with LLM-inferred field labels) |
| `allegheny_dog_licenses.csv` + `allegheny_zip_boundaries.geojson` + `allegheny_dogs_dict.schema.json` | All **50,013** Allegheny County lifetime dog licenses (`LicenseType`, `Breed`, `Color`, `DogName`, `OwnerZip`, `ExpYear`, `ValidDate`) — **no lat/lon**, only the `OwnerZip` region code — plus a GeoJSON of **125 county zip boundaries** keyed by `properties.ZIP` and a curated dictionary tagging `OwnerZip` as `geo.zip_code` | `smart --smarter --bivariate --dict-info --dictionary allegheny_dogs_dict.schema.json --geojson allegheny_zip_boundaries.geojson --feature-id-key properties.ZIP` (**summary choropleth keyed off a region-code COLUMN**, not coordinates: licenses-per-zip filling the boundary polygons on a MapLibre tile basemap, + Breed/Color/LicenseType bars + NMI association heatmap) |

## The smart dashboard

`viz smart` auto-profiles the dataset (from qsv's stats cache) and picks a panel
per column: a **correlation heatmap** over the numeric columns (shown as the
lower triangle only — the mirror half and the trivial 1.0 diagonal are dropped),
**box plots** for continuous numerics, and **frequency bars** for low-cardinality
categoricals, booleans, and ratings. The box plots overlay sample points based on
the dataset size — every point for small data, just the Tukey outliers for medium
data, and none for large data (where the box stays a fast cache-only quartile
summary); pass `--box-points <outliers|all|suspected|none>` to force a mode. When
the numeric columns have a strongly correlated pair,
a **scatter** of that pair is added next to the heatmap as a drill-down — or, for
large datasets where a scatter would overplot, a **2D density contour** of the
pair instead. The drill-down title also reports Spearman's rho when it diverges
from Pearson's r enough to mean the relationship is monotonic but **nonlinear**
(so the single r isn't read as proof of linearity). With three or more numeric
columns, a **3D scatter** of the strongest pair plus the *least-redundant* third
axis is added as well — unless the strongest pair is itself near-collinear, in
which case the 3D would collapse to a plane and is skipped. When the data has a date/datetime
column (auto-detected via stats date inference) plus a continuous numeric column,
a **time-series trend** panel of that column over time is added too. When a
latitude/longitude column pair is detected, a **geographic map** panel leads the
dashboard — drawn on mapbox tiles for local extents, or as an offline
**projection world-overview** (ScatterGeo, no tiles or token) when the
coordinates span a continental/global area. Points far from the cluster centroid
(beyond the Tukey far-out fence of their distances) are flagged as **geographic
outliers**: they're drawn with a distinct marker, excluded from the spatial
extent (so a few bad geocodes can't inflate it), and excluded from the auto-zoom
(so the default view stays tight on the core cluster). When qsv is built with the
`geocode` feature, the (core) extent is reverse-geocoded into a one-line location
summary and outlined with a filled box; when there are outliers, a second dashed
no-fill box marks the full extent (core + outliers) so the strays' span stays
legible, the interactive HTML map gains **Core extent** / **Full extent** buttons
to jump between the two views, and any outliers in a *different* jurisdiction are
named in the summary
(e.g. `Colorado, United States — 4 outliers (Wyoming, Kansas & Nebraska)`) while
outliers within the core's own jurisdiction are folded in silently. (The extent
boxes, buttons, and summary need `geocode`; the outlier markers render in any `viz` build.) ID-like and
high-cardinality text columns are skipped.

On large datasets `viz smart` keeps the page light and interactive: each
data-heavy panel (map, time-series, correlated-pair scatter, 3D scatter) is
uniformly downsampled to at most 50,000 points (the correlated-pair density
contour instead embeds only a fixed bin grid, so it stays compact at any row
count), and the map view is framed to the core extent (geographic outliers
excluded) so a few stray geocodes can't zoom it out to nothing. The map panel
also adapts to volume — at ~20,000+ mappable rows
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

# delivery_stops clusters in metro Denver with a few bad-geocode strays: the map flags them as
# distinct outlier markers and keeps the auto-zoom tight on the core. With the geocode feature it
# also draws a filled core extent box, a dashed-magenta no-fill box around the full extent (core +
# strays), adds Core extent / Full extent zoom buttons to switch between the two views, and calls
# them out in the spatial-extent label, e.g. "... — 4 outliers (Wyoming, Kansas & Nebraska)"
qsv viz smart delivery_stops.csv -o delivery_dashboard.html

# the full geospatial dashboard: a map, a prefecture choropleth, a time-series, a correlation
# heatmap + drill-down scatter, a bimodal-depth histogram, annotated boxes and frequency bars —
# all auto-chosen. --geojson adds a point-in-polygon prefecture choropleth (the feature id key
# defaults to id): each quake is binned into the GeoJSON region that contains it (no geocoding).
# This catalog is mostly
# offshore, so under the default 10 km snap cap the far-offshore quakes are dropped (the panel title
# reports the count) and on-land/near-coast prefectures are colored; raise --snap-max-dist to snap
# distant quakes to the nearest prefecture, or --no-snap to drop every offshore point. A stderr note
# reports coverage either way.
# Recognized lat/lon columns are charted on the map only, not as redundant distribution panels.
# Rendered with the built-in plotly_dark theme (--theme works on every chart type, incl. smart).
qsv viz smart seismic_events.csv --smarter --theme plotly_dark --grid-cols 3 \
    --geojson japan_prefectures.geojson -o seismic_dashboard.html

# the same point-in-polygon choropleth, but on a CITY-scale dataset: 10k NYC 311 requests binned
# into 188 neighborhood polygons. Because the matched regions span a metro extent (under ~8° in
# both lat and lon), viz smart draws the filled regions on an interactive MapLibre TILE basemap
# (token-free carto tiles, fine street/coastline detail) instead of the coarse projection basemap
# it uses for country/continental choropleths — see the metro-choropleth gallery figure. (The
# --feature-id-key defaults to the GeoJSON's top-level `id`, so it's omitted here.) --dictionary
# infer adds LLM-inferred field labels; it needs a reachable local LLM (set QSV_TIMEOUT generously
# for slower local models).
qsv viz smart nyc_311.csv --smarter --dictionary infer \
    --geojson nyc_neighborhoods.geojson -o nyc311_dashboard.html

# A summary choropleth can also be keyed off a region-code COLUMN (a zip/county/state/country
# dimension), with NO lat/lon at all. Here the dog-license log has only an OwnerZip column: viz
# smart aggregates licenses-per-zip and fills the matching --geojson boundary polygons. The key
# column is auto-chosen by matching each geo-dimension column's values against the boundary ids;
# the curated --dictionary tags OwnerZip as geo.zip_code (the signal that makes a numeric zip a
# choropleth key instead of a frequency bar). Only the per-zip COUNT map is drawn here because the
# data carries no numeric measure; a dataset that also tags a measure column gets a per-region
# MEDIAN-of-measure choropleth beside it.
qsv viz smart allegheny_dog_licenses.csv --smarter --bivariate --dict-info \
    --dictionary allegheny_dogs_dict.schema.json \
    --geojson allegheny_zip_boundaries.geojson --feature-id-key properties.ZIP \
    -o allegheny_dogs_dashboard.html
```

### dictionary-guided hierarchy panels (treemap / sunburst)

When the dataset has **2+ low-cardinality categorical dimensions**, `viz smart` adds a
part-to-whole **hierarchy** panel nesting them (the chosen dimensions still keep their own
frequency bars). The chart type is auto-selected by depth, following visualization best practice:
a **treemap** for a shallow 2-level hierarchy (area encodes size, for accurate comparison) and a
**sunburst** for a deeper 3-level one (concentric rings emphasize parent-child structure). Override
with `--hierarchy-style auto|treemap|sunburst`.

The auto path also checks that the candidate dimensions are **statistically associated** (bias-
corrected Cramér's V): nesting *independent* categoricals just replicates each level's marginal at
every branch and tells you nothing the separate frequency bars don't, so that hierarchy is skipped
(with a note on stderr). Pass an explicit `--hierarchy-style treemap|sunburst` to force the panel
anyway.

Pairing this with **`--dictionary infer`** lets a local LLM (via
[`describegpt`](https://github.com/dathere/qsv/blob/master/docs/help/describegpt.md), default
endpoint LM Studio at `http://localhost:1234/v1`) infer each field's semantic role and a friendly
label first, so the dimensions are picked from semantics (not just statistics) and the panels are
nicely titled:

```bash
# two associated dimensions (plan, region) -> a TREEMAP, with LLM-inferred field labels
qsv viz smart customer_spend.csv --dictionary infer -o spend_dashboard.html

# sales_sample's three dimensions are independent, so the auto hierarchy is skipped; force a
# SUNBURST to showcase the deeper-hierarchy chart anyway
qsv viz smart sales_sample.csv --dictionary infer --hierarchy-style sunburst -o sales_dashboard.html
```

> `--dictionary infer` needs a reachable LLM endpoint; set `QSV_LLM_MODEL` (and
> `QSV_LLM_BASE_URL` / `QSV_LLM_APIKEY` as needed). Without a dictionary, the same hierarchy is
> still built from column statistics — only the dimension labels and semantic routing differ.

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

# violin — revenue per region as a KDE density silhouette around an inner quartile box
qsv viz violin sales_sample.csv --y revenue --x region -o violin.html

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

# treemap — part-to-whole hierarchy (--cols are the levels, outer first); sized by a --value sum
qsv viz treemap customer_spend.csv --cols plan,region --value monthly_spend --agg sum -o treemap.html

# sunburst — deeper hierarchy as concentric rings; sized by row count when --value is omitted
qsv viz sunburst sales_sample.csv --cols region,product_category,payment_method -o sunburst.html

# map — point map on token-free OpenStreetMap tiles; color by magnitude, size by depth
qsv viz map quakes.csv --lat lat --lon lon --color magnitude --size depth_km -o map.html

# map (density) — DensityMapbox heatmap of the same points, on a light Carto basemap
qsv viz map quakes.csv --lat lat --lon lon --density --style carto-positron -o map_density.html

# geo — offline projection map (no tiles/token); viz smart auto-uses this for global coordinates
qsv viz geo quakes.csv --lat lat --lon lon --color magnitude --projection natural-earth -o geo.html

# choropleth — fill countries by a value, matched by ISO-3 code (also: usa-states, country-names,
# geojson-id; --map for a MapLibre basemap; --geocode to derive codes from lat/lon or place names)
qsv viz choropleth country_stats.csv --locations iso3 --value gdp_usd_tn --color-scale viridis -o choropleth.html

# choropleth (US states) — built-in state geometry on the albers-usa projection, no GeoJSON needed
qsv viz choropleth us_state_stats.csv --locations state --value renewable_electricity_pct \
    --location-mode usa-states -o choropleth_states.html

# choropleth (--map) — filled regions on a MapLibre tile basemap from a custom GeoJSON, matched by
# --feature-id-key; the view auto-centers/zooms to the GeoJSON extent
qsv viz choropleth western_states.csv --locations state --value wind_capacity_gw \
    --geojson western_states.geojson --feature-id-key id --map --style carto-positron \
    -o choropleth_map.html
```

`viz smart` also adds a choropleth panel on its own whenever it detects lat/lon columns that
reverse-geocode to **two or more regions** — a per-US-**state** fill when every point resolves to
the United States, otherwise a per-**country** (ISO-3) fill framed to the filled-region geometries
via Plotly `fitbounds` (so the regions are never clipped). This auto-panel needs the **`geocode`**
feature (included in the prebuilt `qsv`/`qsvpy` binaries and any `all_features` build); a minimal
`viz`-only build shows just the point map. Pairing it with `--dictionary infer` adds LLM-inferred
field labels (see the [smart dashboard](#the-smart-dashboard) section):

```bash
# US point map + per-US-STATE choropleth, derived purely from the lat/lon columns (no flags)
qsv viz smart us_cities.csv -o us_dashboard.html

# global geo map + per-COUNTRY choropleth, with describegpt-inferred field labels (needs a local LLM)
qsv viz smart world_cities.csv --dictionary infer -o world_dashboard.html
```

> The smart per-country choropleth is **reverse-geocoded from coordinates**: each of the 1,179
> cities is resolved to its sovereign country (ISO-3) and counted, so the densest countries (China,
> India, the US, Brazil, …) fill darkest. The six-continent bar comes from the dataset's own
> `continent` column, whose values follow the
> [plotly.js geo `scope`](https://plotly.com/javascript/reference/layout/geo/#layout-geo-scope)
> continent vocabulary (`Oceania`, `North America`, …). `world_cities.csv` is GeoNames-derived
> (cities with population over 500,000); `elevation_m` is real, while `avg_annual_temp_c` is a rough
> synthetic proxy (a latitude + elevation-lapse model — no coastal, monsoon, or ocean-current
> effects), so treat it as illustrative, not measured.

> Note: `--ohlc-open` is spelled out (not `--open`) because `--open` already means
> "open the result in a browser".
