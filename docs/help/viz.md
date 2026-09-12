# viz

> Generate interactive charts & maps from CSV data using [plotly](https://plotly.com). `viz smart` creates a [Data Schematic](../DATA_SCHEMATIC.md) — a *"[neuro-symbolic](https://en.wikipedia.org/wiki/Neuro-symbolic_AI)"* interactive rendering of a dataset's schema & statistics — picking appropriate visualizations using the dataset's statistics, frequency distributions, data dictionary & optional LLM metadata inferencing/classification, with automatic geocoding enrichment. Outputs self-contained, interactive HTML or static PNG/SVG/PDF/JPEG/WebP with the `viz_static` feature. ([Gallery](https://dathere.github.io/qsv/gallery.html))

**[Table of Contents](TableOfContents.md)** | **Source: [src/cmd/viz.rs](https://github.com/dathere/qsv/blob/master/src/cmd/viz.rs)** | [🪄](TableOfContents.md#legend "\"automagical\" commands that uses stats and/or frequency tables to work \"smarter\" & \"faster\".")[📇](TableOfContents.md#legend "uses an index when available.")[🏎️](TableOfContents.md#legend "multithreaded and/or faster when an index (📇) is available.")[👆](TableOfContents.md#legend "has powerful column selector support. See `select` for syntax.")[🤖](TableOfContents.md#legend "command uses Natural Language Processing or Generative AI.")[🌐](TableOfContents.md#legend "has web-aware options.")[🌎](TableOfContents.md#legend "has geospatial capabilities.")

<a name="nav"></a>
[Description](#description) | [Examples](#examples) | [Usage](#usage) | [Viz Options](#viz-options) | [Map Options](#map-options) | [Geo Options](#geo-options) | [Choropleth Options](#choropleth-options) | [Smart Options](#smart-options) | [Common Options](#common-options)

<a name="description"></a>

## Description [↩](#nav)

Generate charts/maps from CSV data using the plotly charting library.

Produces a self-contained, interactive HTML chart - the plotly.js runtime is embedded, so
charts work offline. Tile basemaps (`viz map`, `viz choropleth --map`) are the exception:  
they fetch tiles over the network at view time. Titles and labels are plain text; LaTeX is
not typeset. With a build that includes the `viz_static` feature, charts can also be exported
as static PNG/SVG/PDF/JPEG/WebP (needs a Chromium/Firefox at runtime; plotly auto-manages the
webdriver).

Set QSV_VIZ_CDN to load plotly.js from its CDN (~1.9MB smaller, but the page then needs the
network to be VIEWED). Set QSV_VIZ_NO_COMPRESS for plain-text HTML that works on pre-2023
browsers and strict-CSP embeds. Progress is shown on stderr, auto-hidden when stderr is not a
terminal, and disabled by setting QSV_PROGRESSBAR to a falsy value. For all of the above, see
<https://github.com/dathere/qsv/wiki/Visualization#output-and-file-size>

The output format is inferred from the --output file extension (.html is the default).
Interactive HTML is written to stdout when --output is not given; image formats always
require --output. Use --open to view the result in your default browser/viewer.

Chart types (subcommands):

```text
smart       Auto-dashboard (Data Schematic) - picks a chart per column from the
            dataset's statistics & frequency distribution. No --x/--y needed.
bar         --x = category column, --y = value column.
line        --x = x column, --y = y column.
scatter     --x = x column, --y = y column.
scatter3d   --x, --y, --z = three numeric columns.
histogram   --x = numeric column to bin.
box         --y = value column, optional --x = group column.
violin      Box plot plus a KDE density curve revealing the distribution's shape
            (modes, shoulders). Same inputs as box.
pie         --x = label column, optional --y = value column.
funnel      Stage-by-stage drop-off. --x = stage column, optional --y = value column
            (counts stage occurrences when omitted). Stages keep the order they first
            appear in the file, so the rows define the pipeline.
heatmap     Correlation matrix of numeric columns (default; subset with --cols), or a
            category x category pivot with --x/--y/--z.
contour     2D density contour of --x and --y, binned into a --bins x --bins grid.
candlestick Financial OHLC. --x = date column, plus --ohlc-open/--high/--low/--close.
ohlc        Financial OHLC bars (same inputs as candlestick).
sankey      Flow diagram. --source, --target, optional --value column.
radar       Polar/radar chart of numeric --cols, optional --series per trace.
treemap     Part-to-whole hierarchy as nested tiles. --cols = 2+ dimension columns
            (levels, outermost first), optional --value and --agg.
sunburst    Same inputs as treemap, as concentric rings. Better for deep hierarchies.
icicle      Same inputs as treemap, as level-aligned stacked bars. Good for deep,
            wide hierarchies.
splom       Scatter-plot matrix: every pair of numeric --cols in a grid, with each
            column's distribution on the diagonal (default: all numeric).
parcats     Parallel categories: ribbons showing how rows flow across the
            categorical --cols (best with 3-4). Complements sankey, which takes
            2 columns.
map         Point map (or --density heatmap) on MapLibre tile basemaps. Pick the
            coordinate columns with the lat/lon options below.
geo         Point map on a projection basemap - coastlines/land/countries, no tiles
            and no token. Same lat/lon options as `map`, plus --projection.
choropleth  Fill whole regions (countries, US states, or custom GeoJSON areas) by a
            value. --locations names the region-code column, --value/--agg the
            measure (row counts if omitted). Defaults to a token-free projection
            basemap; --map switches to MapLibre tiles.
```

`qsv viz smart` builds a DATA SCHEMATIC - a single, self-contained rendering of a dataset's
schema and statistics in which every claim shown is checkable against the data it describes.
The format is specified in docs/DATA_SCHEMATIC.md and is tool-neutral. A DETERMINISTIC half
(statistics and heuristics - reproducible, offline, no tokens) picks the panels and scales the
axes; a SEMANTIC half (an optional LLM-inferred Data Dictionary, see the dictionary option)
supplies what the fields MEAN. Both halves show their work, and the dictionary is a
hand-editable sidecar, so a Data Steward can correct anything the model proposed.

It reuses qsv's stats and frequency caches (the first run computes & caches stats; later runs
are fast) and auto-picks panels, so no --x/--y is needed:  

Per-column panels flow in a grid below the overview rows (see --grid-cols):

```text
continuous numeric -> violin or box plot; low-cardinality/boolean -> frequency bar;
confirmed-bimodal -> histogram (needs --smarter). ID-like (near-unique) and all-empty
columns are skipped and reported.
```

Overview panels each lead the Data Schematic on their own full-width row: a KPI "big
number" strip, a declared pipeline funnel or bridge, a correlation heatmap with a
drill-down scatter/contour/3D triple, a time-series line, a part-to-whole hierarchy, and
a geographic map plus region choropleth. Each is drawn only when the data supports it;
which panel is chosen, and why, is documented at
<https://github.com/dathere/qsv/wiki/Visualization#smart-panels>


<a name="examples"></a>

## Examples [↩](#nav)

> Data Schematic for a dataset, opened in the browser

```console
qsv viz smart data.csv --open
```

> Data Schematic, at most 6 panels in a 3-column grid, top-5 categories per bar

```console
qsv viz smart data.csv --max-charts 6 --grid-cols 3 --limit 5 -o dashboard.html
```

> Semantics-guided Data Schematic with a browsable Data Dictionary drawer

```console
qsv viz smart data.csv --dictionary infer --dict-info -o dashboard.html
```

> Richer Data Schematic: extra distribution-shape stats plus pairwise associations

```console
qsv viz smart data.csv --smarter --bivariate -o dashboard.html
```

> Aggregate (sum) sales by region into a titled bar chart, opened in the browser

```console
qsv viz bar sales.csv --x region --y amount --agg sum --title "Sales by region" --open
```

> Scatter plot with a separate series (trace) per category

```console
qsv viz scatter data.csv --x age --y income --series gender -o scatter.html
```

> Bubble scatter: marker size by population, marker color by a numeric score

```console
qsv viz scatter data.csv --x gdp --y life_exp --size population --color score -o bubble.html
```

> Gapminder bubble animation: one bubble per region moving over time

```console
qsv viz scatter regions.csv --x gdp_index --y wellbeing_index --size population_m --series region --slider month_date -o gapminder.html
```

> 3D scatter of three numeric columns, colored by a fourth

```console
qsv viz scatter3d data.csv --x length --y width --z height --color weight -o scatter3d.html
```

> 2D density contour of two numeric columns with a 40x40 grid

```console
qsv viz contour data.csv --x height --y weight --bins 40 -o contour.html
```

> Histogram of a numeric column with 30 bins

```console
qsv viz histogram data.csv --x value --bins 30 -o hist.html
```

> Box plot grouped by a category, every sample point overlaid, exported to PNG

```console
qsv viz box data.csv --y measurement --x group --box-points all -o box.png
```

> Violin plot (KDE density curve + inner box) of a value column grouped by a category

```console
qsv viz violin data.csv --y measurement --x group -o violin.html
```

> Pie chart of category proportions (counts), as a donut

```console
qsv viz pie data.csv --x category --donut -o pie.html
```

> Funnel of a pipeline whose stages are ROWS (stage column + amount column)

```console
qsv viz funnel pipeline.csv --x stage --y amount -o funnel.html
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

> Treemap of part-to-whole sales by region then category, sized by amount

```console
qsv viz treemap sales.csv --cols region,category --value amount --agg sum -o treemap.html
```

> Sunburst of a deep 3-level web-traffic hierarchy, sized by row count

```console
qsv viz sunburst web.csv --cols source,campaign,landing_page -o sunburst.html
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

> Projection map of earthquakes (token-free, offline), marker color by magnitude

```console
qsv viz geo quakes.csv --lat lat --lon lon --color magnitude --projection natural-earth -o geo.html
```

> Choropleth coloring countries (ISO-3 codes) by a summed measure

```console
qsv viz choropleth gdp.csv --locations iso3 --value gdp --agg sum -o choropleth.html
```

> US-state choropleth of row counts per state (2-letter state codes)

```console
qsv viz choropleth orders.csv --locations state --location-mode usa-states -o states.html
```

> US counties with no GeoJSON supplied - boundaries fetched from Census TIGERweb

```console
qsv viz choropleth complaints.csv --locations fips --geojson auto -o counties.html
```

> ...as a per-capita RATE, with the population fetched from the Census Data API

```console
qsv viz choropleth complaints.csv --locations fips --geojson auto --denominator census -o rate.html
```

> Custom GeoJSON regions on a MapLibre tile basemap, matched by a feature id

```console
qsv viz choropleth counties.csv --locations fips --value pop --map --geojson counties.json --feature-id-key id -o counties.html
```

> Point-in-polygon: bin lat/lon points into custom GeoJSON regions by count (no geocode)

```console
qsv viz choropleth quakes.csv --lat lat --lon lon --geojson prefectures.geojson --feature-id-key properties.id -o by_pref.html
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
qsv viz violin      [options] <input>
qsv viz pie         [options] <input>
qsv viz funnel      [options] <input>
qsv viz heatmap     [options] <input>
qsv viz contour     [options] <input>
qsv viz candlestick [options] <input>
qsv viz ohlc        [options] <input>
qsv viz sankey      [options] <input>
qsv viz radar       [options] <input>
qsv viz map         [options] <input>
qsv viz geo         [options] <input>
qsv viz choropleth  [options] <input>
qsv viz treemap     [options] <input>
qsv viz sunburst    [options] <input>
qsv viz icicle      [options] <input>
qsv viz splom       [options] <input>
qsv viz parcats     [options] <input>
qsv viz --help
```

<a name="viz-options"></a>

## Viz Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑x,`<br>`‑‑x`&nbsp; | string | Column for the x-axis / category / bin / group. |  |
| &nbsp;`‑y,`<br>`‑‑y`&nbsp; | string | Column for the y-axis / value. |  |
| &nbsp;`‑z,`<br>`‑‑z`&nbsp; | string | The z column: a heatmap pivot value (with --x and --y), or the third numeric axis for scatter3d. |  |
| &nbsp;`‑‑cols`&nbsp; | string | Columns to use. For heatmap: numeric columns for the correlation matrix (default: all numeric). For radar: the numeric axes to plot. For treemap/sunburst/icicle: the categorical dimensions that form the hierarchy levels, outermost first (e.g. region,category,subcategory). For splom: the numeric columns to cross-plot (default: all numeric). For parcats: the categorical dimensions to flow across (best with 3-4). |  |
| &nbsp;`‑‑series`&nbsp; | string | Column to split into multiple series (one trace per distinct value). Applies to bar, line, scatter, scatter3d, radar, map and geo. |  |
| &nbsp;`‑‑color`&nbsp; | string | For scatter/scatter3d/map/geo: a numeric column to encode as marker color (a continuous colorscale with a colorbar). For categorical coloring, use the --series option instead. Cannot be combined with --series. In map density mode, this column is the heatmap weight. |  |
| &nbsp;`‑‑size`&nbsp; | string | For scatter/scatter3d/map/geo: a numeric column to encode as marker size, producing a bubble chart (values are rescaled to a readable pixel range). Cannot be combined with --series, EXCEPT on an animated scatter (see the --slider option), where the --series column names the entity and this column is the third data variable. In map density mode, this column is the heatmap weight. |  |
| &nbsp;`‑‑donut`&nbsp; | flag | Render a pie chart as a donut (with a center hole). |  |
| &nbsp;`‑‑ohlc‑open`&nbsp; | string | Open-price column for candlestick/ohlc charts. |  |
| &nbsp;`‑‑high`&nbsp; | string | High-price column for candlestick/ohlc charts. |  |
| &nbsp;`‑‑low`&nbsp; | string | Low-price column for candlestick/ohlc charts. |  |
| &nbsp;`‑‑close`&nbsp; | string | Close-price column for candlestick/ohlc charts. |  |
| &nbsp;`‑‑source`&nbsp; | string | Source node column for a sankey diagram. |  |
| &nbsp;`‑‑target`&nbsp; | string | Target node column for a sankey diagram. |  |
| &nbsp;`‑‑value`&nbsp; | string | Flow value column for a sankey diagram. When omitted, each row counts as a flow of 1. For treemap/sunburst/icicle: a numeric measure summed per sector (when omitted, each row counts as 1). |  |
| &nbsp;`‑‑sankey‑value‑order`&nbsp; | flag | Order sankey nodes by total flow (largest at the top of each column) instead of plotly's default crossing-minimizing "snap" order. Either way the chart carries an on-screen "node order" button to flip between the two layouts, and link ribbons are always colored by their source node. Applies to the `sankey` chart AND the `smart` Data Schematic flow panel. |  |
| &nbsp;`‑‑bins`&nbsp; | integer | Number of bins. For histogram: bins along the x-axis (default: auto). For contour: the per-axis resolution of the density grid (default: 20). |  |
| &nbsp;`‑‑agg`&nbsp; | string | How to combine values. For bar/line, aggregate the y values when the x value repeats: one of sum, mean, count, min, max. For treemap/sunburst/icicle, only the additive aggregations apply: count (the default) or sum (which requires --value). For choropleth, how rows sharing a region are combined: sum when --value is given, else count; pass mean for an intensive measure (a density, rate, index or per-capita figure), since summing those is meaningless and the count/sum hover's "share of total" line would be too. On a bubble animation (see the slider option), it collapses each entity x frame cell to a single bubble and defaults to mean, the cell centroid; count is rejected there, since a row count is not a position. |  |
| &nbsp;`‑‑box‑points`&nbsp; | string | Which sample points to draw alongside a box or violin. Reading the raw values lets plotly render true Tukey whiskers (1.5*IQR) with the points beyond the fences as outliers. One of: outliers (only the outliers), all (every point, jittered), suspected (mark suspected outliers), none (no points, but still real Tukey whiskers). Applies to `viz box`, `viz violin` AND `viz smart`. For `viz box` the default is outliers; for `viz smart` an explicit mode OVERRIDES a size-based heuristic and is applied to every panel. `none` only hides a violin's points, since its KDE needs the values anyway; the cache-only escape hatch is `--violin off` with this set to none. For the heuristic's thresholds, see <https://github.com/dathere/qsv/wiki/Visualization#distribution-panels> |  |

<a name="map-options"></a>

## Map Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑‑lat`&nbsp; | string | Latitude column for a map (decimal degrees, -90 to 90). |  |
| &nbsp;`‑‑lon`&nbsp; | string | Longitude column for a map (decimal degrees, -180 to 180). |  |
| &nbsp;`‑‑text`&nbsp; | string | Column whose value labels each point on hover. |  |
| &nbsp;`‑‑density`&nbsp; | flag | Render a density heatmap (DensityMap) instead of points. Weighted by the --color or --size column when given, else by a uniform weight. Cannot be combined with --series. |  |
| &nbsp;`‑‑style`&nbsp; | string | MapLibre basemap style (all render without an access token): open-street-map (the default), carto-positron, carto-darkmatter, carto-voyager, white-bg, basic, streets, outdoors, light, dark, satellite, satellite-streets. The three carto styles also have label-free "-nolabels" variants (e.g. carto-positron-nolabels) that keep basemap place names from competing with a dense data overlay. | `open-street-map` |

<a name="geo-options"></a>

## Geo Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑‑projection`&nbsp; | string | Map projection for `viz geo`. One of: natural-earth (the default), mercator, orthographic, equirectangular, albers-usa, robinson, winkel-tripel, mollweide, hammer, azimuthal-equal-area. `viz geo` also reuses the lat, lon, text, color, size and series options from `map`. | `natural-earth` |

<a name="choropleth-options"></a>

## Choropleth Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑‑locations`&nbsp; | string | Column holding the region key for each row (an ISO-3 country code, a 2-letter US state code, a country name, or a GeoJSON feature id, per --location-mode). With --geocode, this instead names a place-name column to forward-geocode into region codes. |  |
| &nbsp;`‑‑location‑mode`&nbsp; | string | How --locations values are matched to regions. One of: iso3 (the default, ISO-3166-1 alpha-3 country codes), usa-states (2-letter US state codes), country-names (full country names), geojson-id (match a --geojson feature id). | `iso3` |
| &nbsp;`‑‑color‑scale`&nbsp; | string | Colorscale for the region fill. One of: viridis (the default), cividis, greys, greens, blues, reds, ylgnbu, ylorrd, bluered, rdbu, portland, electric, jet, hot, blackbody, earth, picnic, rainbow. | `viridis` |
| &nbsp;`‑‑map`&nbsp; | flag | Render on a token-free MapLibre tile basemap (a ChoroplethMap) instead of the default projection basemap. Requires --geojson; feature-id-key defaults to id. Reuses --style for the basemap. |  |
| &nbsp;`‑‑geojson`&nbsp; | string | Custom region polygons: a local file path, an http(s) URL to a GeoJSON FeatureCollection, or a shortcut name from the QSV_GEOJSON_SHORTCUTS env var (whose id sets --feature-id-key when you don't pass one). Use `auto` (or `census`) to fetch US boundaries from Census TIGERweb automatically; force a layer with census:county, census:zcta, census:tract or census:place (incorporated places AND CDPs), and pin a vintage with @<year>, e.g. census:county@2021. That path sets feature-id-key to properties.GEOID and caches under ~/.qsv-cache for 30 days (QSV_VIZ_BOUNDARY_CACHE_TTL_DAYS); it reads the codes from the --locations column, so in `viz smart` it needs a --dictionary naming the region column. A column of city/place NAMES also needs --geocode. Required for --map and for the geojson-id location mode; with --lat/--lon it instead enables point-in-polygon binning. See <https://github.com/dathere/qsv/wiki/Visualization#census-boundaries> |  |
| &nbsp;`‑‑feature‑id‑key`&nbsp; | string | Property path in each GeoJSON feature whose value matches an entry in the locations column, or that labels each binned region (e.g. id, properties.fips). | `id` |
| &nbsp;`‑‑feature‑name‑key`&nbsp; | string | GeoJSON property path whose value is shown as the human-readable region label in choropleth hover (e.g. properties.name). When omitted, common name keys are auto-detected; falls back to the feature id when absent. |  |
| &nbsp;`‑‑check‑geojson‑key`&nbsp; | flag | Diagnostic mode. Instead of rendering, score EVERY candidate feature-id path in --geojson against the distinct values of the locations column and print them ranked by overlap - use it when a region map shades nothing while exiting 0. Requires --geojson and --locations, with a concrete source for the former (path, URL or shortcut); every automatic Census spec is refused, since automatic resolution picks the key itself. |  |
| &nbsp;`‑‑denominator‑key`&nbsp; | string | GeoJSON property path holding each region's DENOMINATOR (e.g. properties.POP2020), addressed like --feature-id-key. Turns a raw-count region map into a RATE map: in `viz choropleth` the map itself becomes the rate; in `viz smart` a rate panel is added BESIDE the count panel. Regions whose denominator is missing or non-positive are excluded and reported; the per-region scale (per 1,000 / 10,000 / 100,000 / 1,000,000 / 1,000,000,000) is chosen automatically. See <https://github.com/dathere/qsv/wiki/Visualization#rate-maps> |  |
| &nbsp;`‑‑denominator‑unit`&nbsp; | string | Unit the denominator values are IN, so qsv can convert and name them. One of: m2, km2 (also spelled m², km², sqm, sqkm). Values in m2 are converted to km², so the map reads "per km²". NEVER inferred from a field name - real boundary files carry land area in square METRES, which divided raw reads as "per 100,000 AREALAND" at approximately zero everywhere. Boundaries fetched by `--geojson auto` declare their own units and need no flag. |  |
| &nbsp;`‑‑denominator`&nbsp; | string | Column holding each region's DENOMINATOR, as an alternative to reading it from the GeoJSON. Its value must be constant within a region (it describes the region, not the row). `viz choropleth` only, and only with a --locations region column - not with the --geocode flag or lat/lon binning. Valid with the default count aggregation and with --agg sum; the other aggregations are already intensive, so a denominator is rejected there. In `viz smart`, declare it in the data dictionary instead, via x-qsv.denominator (see --dictionary). The value `census` is RESERVED (a column of that name cannot be used): it fetches US population from the Census Data API for a per-capita rate with nothing supplied, and being a source rather than a column it works on `viz smart` too. Region codes must be 5-digit county FIPS, 2-digit state FIPS, or 2-letter state codes. The newest published ACS 5-year release is used and is always stated beneath the map; pin one with @<year>, e.g. census@2019. Uncovered regions are excluded and reported. Cached for 30 days (QSV_VIZ_DENOMINATOR_CACHE_TTL_DAYS). REQUIRES a Census API key in QSV_CENSUS_API_KEY - free at <https://api.census.gov/data/key_signup.html> |  |
| &nbsp;`‑‑geocode`&nbsp; | flag | Derive the region codes with qsv's geocode engine (needs a build with the geocode feature): either reverse-geocode the lat/lon points, or forward-geocode the locations name column. Only valid with location modes iso3 or usa-states. `viz choropleth` also reuses --value, --agg, --style and the lat/lon options. Combined with `--geojson auto` it forward-geocodes a column of city/place NAMES to their containing US county FIPS and fetches those counties; lat/lon is REJECTED there, since reverse geocoding finds the nearest populated place, not the containing county. Each distinct name is a full engine lookup, so a column with thousands of them resolves slowly; unresolvable names are reported with a denser-index remedy. |  |
| &nbsp;`‑‑geocode‑country`&nbsp; | string | Column holding each row's expected country, as an ISO-3166-1 alpha-2 code (e.g. US, GB, BR). Without it a place-name search spans every country, so US city names have resolved to Pakistan and a point near a border takes the nearest FOREIGN place. Rows whose match falls outside the hint are counted and reported, never silently dropped. Country NAMES are not accepted. With location-mode usa-states the country defaults to `us`. Applies to both geocode sources. |  |
| &nbsp;`‑‑geocode‑admin1`&nbsp; | string | Column holding each row's expected admin1 (state/province), either as a qualified Geonames code (US.AL) or a bare subdivision code (AL) qualified by the country. Narrows a place-name search for names ambiguous on their own - there are dozens of US Springfields. Spelled-out names are REJECTED: admin1 names match by prefix, so `AL` would silently accept Alaska. Valid only with a locations name column, not with lat/lon points. |  |
| &nbsp;`‑‑region‑state`&nbsp; | string | Column holding each row's state, for a --locations column of county NAMES rather than codes. Accepts a USPS code (PA), a 2-digit state FIPS (42) or the full state name (Pennsylvania). 422 county names are carried by more than one county - Washington County exists in 30 states - so without this, a column containing any of them is refused rather than resolved to an arbitrary state. `viz choropleth --geojson auto` only; in `viz smart` the state column comes from the data dictionary. A county FIPS column needs none of this. |  |
| &nbsp;`‑‑no‑snap`&nbsp; | flag | For point-in-polygon binning: do not snap at all - drop every point that falls outside every region. By default an outside point snaps to its nearest region when within the snap-distance limit (see --snap-max-dist). Applies to both `viz choropleth` and the `viz smart` GeoJSON choropleth panel; coverage is reported on stderr and beneath the map either way. |  |
| &nbsp;`‑‑snap‑max‑dist`&nbsp; | float | For point-in-polygon binning: the farthest (in km, an equirectangular approximation) an outside point may snap to a region's boundary; points with no region within it are dropped. When omitted the cap is AUTO-DERIVED from the GeoJSON's median region size and the lat/lon coordinate precision, clamped to 0.1-100 km; a fixed 10 km is used only when neither signal is available. The coverage notes state the cap and how it was derived. Pass a large value for effectively unbounded snapping; cannot be combined with --no-snap. See <https://github.com/dathere/qsv/wiki/Visualization#point-in-polygon> |  |

<a name="smart-options"></a>

## Smart Options [↩](#nav)

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Type | Description | Default |
|--------|------|-------------|--------|
| &nbsp;`‑‑max‑charts`&nbsp; | integer | Maximum number of panels in the Data Schematic. 0 (the default) means auto: draw every eligible column (up to 64), for both HTML and static image export. Up to 8 cartesian panels render as one typed subplot grid; beyond 8, HTML switches to an inline-div grid and static export uses domain-positioned axes to fit one image. Set a positive <n> to cap the panel count instead; eligible columns beyond the cap are reported but not drawn. | `0` |
| &nbsp;`‑‑grid‑cols`&nbsp; | integer | Number of columns in the Data Schematic grid for the per-column distribution panels. Overview panels (map/geo, correlation, time-series) always span the full width. | `2` |
| &nbsp;`‑‑preview‑threshold`&nbsp; | integer | Row threshold for the interactive data viewer drawer of HTML dashboards. Next to the row count in the metadata table, an "(Explore)" link opens the underlying table in a bottom drawer (with global, per-column and builder search) when the dataset has at most <n> rows and ALL rows are embedded; above <n>, only the first <n> rows are embedded and the link reads "(Preview)". Embedded rows grow the HTML file, and browser memory, in proportion to rows x columns. Set to 0 to disable the data viewer entirely (no link, no embedded data). `smart` HTML only. | `50000` |
| &nbsp;`‑‑tour‑steps`&nbsp; | integer | How many chart panels the guided tour of a `smart` HTML dashboard spotlights. This budgets the per-panel steps only - the introduction, the Data Dictionary drawer chapter, the data viewer chapter and the conclusion are always included. Set to 0 to suppress the tour entirely - no tour, no "Tour" button, no first-visit hint beacons. Left at its default (8) the budget is SOFT: an all-overview dashboard gains one extra step so a frequency distribution is always spotlighted. An explicit value makes it a HARD cap, substituting rather than adding. A dictionary's dataset-level "x-qsv" "tour" object can refine the narration - see <https://github.com/dathere/qsv/wiki/Visualization#guided-tour> `smart` HTML only. |  |
| &nbsp;`‑‑heatmap‑density`&nbsp; | integer | For the `viz smart` map panel: at or above <n> mappable points, draw the core as a density heatmap (DensityMap) instead of markers, keeping per-point hover. Defaults to 0 (disabled): dense point maps instead open as individual points with an in-map "Clusters/Points" toggle rather than aggregating into a static heat surface. Set a positive <n> to opt back in. | `0` |
| &nbsp;`‑‑cluster`&nbsp; | string | For the `viz smart` map panel: whether to offer an in-map "Clusters/Points" toggle that collapses dense points into native MapLibre count bubbles (which expand back into individual, hoverable points on zoom-in). The map always OPENS as individual points. One of: auto, on, off. "auto" offers the toggle for a plain-marker core (no density heatmap, no bubble-size measure) once it reaches 1,000 points; "on" offers it whenever the core draws as markers; "off" omits it. Only affects `smart`. | `auto` |
| &nbsp;`‑‑photos`&nbsp; | flag | For the `viz smart` map panel: when a column holds image URLs (detected from the stats cache), embed each point's image URL so that dwelling on a map point for 2 seconds opens that row's photo in a preview card beside the pointer (arrow keys page through a row's photos; Escape dismisses). OFF by default, and deliberately so: images load from whatever third-party host the DATA names, so enabling this makes everyone who opens the Data Schematic request those URLs directly, revealing their IP to that host. Nothing is fetched until a viewer dwells. HTML output only, tile-basemap map panel only, `smart` only. |  |
| &nbsp;`‑‑limit`&nbsp; | integer | Top-N categories per frequency bar chart. | `10` |
| &nbsp;`‑‑no‑nulls`&nbsp; | flag | Omit the "(NULL)" bar (empty cells) from frequency bar charts. By default `viz smart` shows a "(NULL)" bar, like `qsv frequency`. |  |
| &nbsp;`‑‑no‑other`&nbsp; | flag | Omit the "Other (N)" aggregate bar from frequency bar charts. It collects the categories beyond --limit (N = how many distinct categories were rolled up) and is shown by default. When just ONE category is left over, it is charted under its own name instead of as an opaque "Other (1)" bar. |  |
| &nbsp;`‑‑smarter`&nbsp; | flag | Before building the Data Schematic, run `qsv moarstats --advanced` to enrich the stats cache with distribution-shape statistics (bimodality, entropy, skewness, outlier share, Gini). This unlocks histograms for bimodal columns, frequency bars for concentrated high-cardinality columns, skew/outlier hints on box panels, and Lorenz curves for the most unequal additive measures. Costs one extra pass and writes <stem>.stats.csv, its sidecars and an .idx index. On geocode-enabled builds it also adds the US FIPS code to map hovers. Applied only with default parsing; an input using the --no-headers or --delimiter flags falls back to the standard Data Schematic. Only affects `smart`. |  |
| &nbsp;`‑‑hierarchy‑style`&nbsp; | string | For `smart`, the chart used for the categorical part-to-whole hierarchy panel (built when 2+ low-cardinality dimensions exist). One of: auto (default), treemap, sunburst, icicle. auto follows best practice - a treemap for a shallow 2-level hierarchy (accurate size comparison) and a sunburst for a deep 3-level one (parent/child structure); icicle is an opt-in level-aligned alternative. An explicit style also bypasses the association screen that would otherwise skip an uninformative panel. Only affects `smart`. |  |
| &nbsp;`‑‑dictionary`&nbsp; | string | Use a describegpt Data Dictionary to guide panel selection from each field's semantic role/concept (falling back to its content type) instead of relying on column statistics alone: dimensions and numeric codes (ward, census_tract, zone) become bars, measures get box/correlation/trend panels, date/datetime columns feed the time-series panel, identifiers / PII / free-text are skipped, and lat/lon feed the map. Field labels are shown as panel subtitles. Columns the dictionary cannot classify still use the statistical heuristic. <src> is either "infer", to run describegpt on the input now (requires an LLM configured), or a path to an existing describegpt dictionary file (jsonschema or json). With "infer", the generated dictionary is saved beside the input as <stem>.schema.json; if that file already exists it is REUSED as-is, skipping the LLM. An inferred dictionary is a DRAFT, not an oracle: re-inferring the same data can assign a column a different role, and role drives panel selection, so a re-infer can hand you a structurally different Data Schematic. The saved sidecar is therefore the artifact of record - review it, correct it, and keep it beside the data. Set QSV_VIZ_DICT_FRESH=1 to ignore it and bypass describegpt's completion cache, forcing a fresh inference that overwrites the sidecar on success. Generation/read failures soft-fall back to the stats-only Data Schematic. qsv re-verifies every hint below on read, so a hand-edit can correct the model but cannot smuggle in a claim the data does not support. Per-field hints live in a property's "x-qsv" object: gauge_range [min,max] - render the KPI tile as a GAUGE on that canonical scale; kept only when the observed value is inside it target <number> - add a "vs target" DELTA (value minus target). A goal you supply, so "infer" never emits one currency <ISO-4217> - prefix the KPI tile with that currency's symbol ($192B); an unknown code renders verbatim ("XOF 1.2B") unit <UCUM code> - suffix a NON-monetary measure with the unit symbol ("18.4 °C"), in KPI tiles, axis titles and hovers. Checked CASE-SENSITIVELY against ~44 common units (UCUM distinguishes "m" from the "M" mega prefix); an off-table code is dropped. Mutually exclusive with currency aggregation sum\|mean - declare how the measure combines across a group, overriding qsv's extensive-vs-intensive guess either way denominator {column, unit?, level?} - on a REGION-CODE column, name the column holding each region's population to add a RATE panel beside the raw-count region map. "unit" converts an area given in m2 or km2; "level" names the AREAL unit those values describe as a "geo." concept token, and a level naming a COARSER geography than the region column is refused The dictionary is also the ONLY source of the pipeline panel, declared in the DATASET-level "x-qsv" object as a "relationships" entry with "kind": "pipeline". Stages may be held in separate columns ("members", widest/upstream first) or as values of one category column ("stage_column" plus "stages", with an optional "value_column"). Declared order is authoritative and is never re-sorted by size: it draws as a funnel while the stage totals never grow, and as a BRIDGE when one outruns its predecessor. For the hint semantics, the pipeline encodings and worked JSON, see <https://github.com/dathere/qsv/wiki/Visualization#data-dictionary> Only affects `smart`. |  |
| &nbsp;`‑‑dictionary‑context`&nbsp; | string | Path to a file with extra context about the dataset (a glossary, README, data dictionary, PDF, etc.) forwarded to describegpt as --context-file when `--dictionary infer` generates the dictionary. Better context yields better role/concept/label tags, hence a better Data Schematic. Ignored unless `--dictionary infer` is used. Only affects `smart`. |  |
| &nbsp;`‑‑tour‑audience`&nbsp; | string | Who the guided Tour should talk to. Forwarded to describegpt when `--dictionary infer` generates the dictionary, so the inferred dictionary carries an "x-qsv.tour" narration written for that audience (e.g. "Explain like I'm 10" - the default - or "a board of directors", "data journalists"). The audience shapes ONLY the tour prose; field labels and descriptions keep their normal register. Ignored unless a FRESH `--dictionary infer` actually runs: when an existing <stem>.schema.json sidecar is reused, delete it (or set QSV_VIZ_DICT_FRESH=1) to re-infer with a new audience. Only affects `smart`. |  |
| &nbsp;`‑‑dict‑info`&nbsp; | flag | When a usable Data Dictionary is available (per --dictionary), add a "Data Schematic" link beneath the page title and an info icon on each panel title: hovering shows that column's description, clicking opens a human-friendly rendering of the dictionary in a side drawer NEXT TO the plots, scrolled to that column's entry. The drawer carries a table of contents, per-column "View chart" links, download buttons bundling every sidecar this run READ (each under 4 MB), and a record of what the Data Schematic LEFT OUT. Everything is embedded in the HTML, so no extra file is written. HTML output only; ignored with a note when no dictionary is available or when exporting an image. Only affects `smart`. See <https://github.com/dathere/qsv/wiki/Visualization#dict-info> |  |
| &nbsp;`‑‑dataset‑pid`&nbsp; | string | A persistent identifier (PID) for the dataset - typically a full URL such as a DOI (<https://doi.org/10.1234/abc>) or other citable link. When set, a "PID" row is added to the metadata table at the top of the Data Schematic. http(s) and mailto values become a clickable link; any other scheme is shown as plain text. HTML output only. Only affects `smart`. |  |
| &nbsp;`‑‑bivariate`&nbsp; | flag | Add two pairwise-association overview panels driven by `qsv moarstats --bivariate`: a normalized mutual information (NMI) heatmap over every column pair (numeric AND categorical, unlike the Pearson-only correlation heatmap), plus a ranked "top relationships" bar of the strongest pairs when there are more than 8 chartable columns. Identifier / PII / free-text columns (per the --dictionary option) and map lat/lon columns are excluded. It TURNS ON --smarter, plus --dictionary infer when --dictionary isn't already set. Capped at 50 columns (1,225 pairs); wider datasets skip these panels with a warning. Only affects `smart`. See <https://github.com/dathere/qsv/wiki/Visualization#bivariate> |  |
| &nbsp;`‑‑log‑scale`&nbsp; | string | Use a logarithmic y-axis for panels with a high dynamic range: frequency bar panels whose tallest bar dwarfs the rest (e.g. a large "(NULL)" or "Other (N)" bucket), so the small categories stay visible; and box panels of an all-positive column whose observed max/min ratio is huge. One of: auto, on, off. "auto" switches a panel to a log y-axis only when its dynamic range is high; "on" forces it on every frequency panel and every all-positive box panel; "off" keeps the linear axes. Only affects `smart`. | `auto` |
| &nbsp;`‑‑violin`&nbsp; | string | Draw distribution panels as violins (a box plot wrapped in a KDE density silhouette - a strict superset of a plain box) instead of boxes. One of: auto, on, off. "auto" draws a violin for every continuous column EXCEPT those on a log value axis (the KDE is estimated in linear space, which a log axis would distort) and those with too few distinct values or observations for a meaningful KDE; both keep an honest box. "on" also violins those; "off" never draws violins. KDEs are clamped to each column's observed range, so a bounded or discrete measure can't show density past a natural limit. Above a size threshold a violin is drawn from a deterministic strided sample, carries no point overlay and is titled "(sampled)"; that budget scales with QSV_VIZ_MAX_POINTS. For the thresholds and how this interacts with the box-points option, see <https://github.com/dathere/qsv/wiki/Visualization#distribution-panels> Only affects `smart`. | `auto` |
| &nbsp;`‑‑title`&nbsp; | string | Chart title. |  |
| &nbsp;`‑‑x‑title`&nbsp; | string | X-axis title. (defaults to the x column name) |  |
| &nbsp;`‑‑y‑title`&nbsp; | string | Y-axis title. (defaults to the y column name) |  |
| &nbsp;`‑‑y‑range`&nbsp; | string | Fix the y-axis to an explicit min:max range (two colon- separated numbers, e.g. -10:55) instead of autoscaling; points outside are clipped from view. Handy when one extreme outlier squashes the rest. Applies to cartesian charts (bar/line/scatter/histogram/box/violin/heatmap/ contour). Pass a negative min with `=`: --y-range=-10:55. |  |
| &nbsp;`‑‑rangeslider`&nbsp; | flag | Add a draggable range-slider (a navigator strip) under the x-axis for panning and zooming. Best for an ordered x-axis (time series, line, candlestick/ohlc). Applies to cartesian charts (bar/line/scatter/histogram/box/violin/candlestick/ ohlc); off by default. |  |
| &nbsp;`‑‑slider`&nbsp; | string | Animate the chart over a column: each distinct value of the given column becomes an animation frame, with a Play/Pause button and a slider to scrub through the frames (Gapminder- style). Supported for bar/line/scatter and geo, and may be split into animated traces with --series. NOT supported for `viz map`, whose MapLibre basemap can't animate - use `viz geo` for an animated point map. A slider column that parses as dates is auto-bucketed to a readable number of frames; any other column frames on its distinct values. Axis ranges are pinned across frames so nothing jumps. On `viz scatter`, a slider with --series AND --size builds a Gapminder BUBBLE ANIMATION: --series is the entity, --x/--y the measure pair and --size the third variable (bubble area). Each entity x frame cell collapses to one bubble via --agg (mean by default), scaled once across all frames so sizes stay comparable. For `smart`, this instead takes auto, on or off, controlling whether ONE animated relationship-over-time panel is added - a drifting numeric pair, an animated geo map of dated points, or a Gapminder entity-bubble chart (at most one; the bubble wins, then the geo map, then the pair). auto requires a strong signal (a date column and enough time buckets); on lowers the bar to two buckets but still requires a genuinely drifting relationship; off never does. |  |
| &nbsp;`‑‑slider‑speed`&nbsp; | integer | Milliseconds each animation frame is shown while playing. | `800` |
| &nbsp;`‑‑slider‑cumulative`&nbsp; | flag | Accumulate rows across frames: frame N includes every row whose --slider value is at or below the Nth value, so the animation builds up cumulatively instead of replacing each step. Applies to bar/line/scatter and geo. On a bubble animation there is only ever one bubble per entity per frame, so nothing piles up on screen; instead each bubble shows a RUNNING aggregate over all frames up to that point, and so drifts less and less as the animation plays. |  |
| &nbsp;`‑‑annotation`&nbsp; | string | Caption note drawn at the bottom of the plot (e.g. to note a clipped outlier). Cartesian charts only. |  |
| &nbsp;`‑‑theme`&nbsp; | string | Plotly theme that drives the chart's overall look (background, fonts, axis styling). One of: default, plotly_white, plotly_dark, seaborn, seaborn_whitegrid, seaborn_dark, matplotlib, plotnine (case-insensitive; hyphens accepted). When omitted, qsv's built-in look is used. Applies to all chart types, including `smart`. |  |
| &nbsp;`‑‑language`&nbsp; | string | Render the Data Schematic UI in this language. Accepts a BCP-47/ISO 639 code (es, spa) or an English language name (Spanish). Overrides the language detected in a data dictionary, and is also forwarded to describegpt when inferring one. Curated: en, es, fr, de, it, pt-BR, ja, zh-CN. | `auto` |
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
