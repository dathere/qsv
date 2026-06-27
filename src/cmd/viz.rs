static USAGE: &str = r#"
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
    treemap     Part-to-whole hierarchy as nested tiles. --cols = 2+ dimension
                columns (levels), optional --value and --agg.
    sunburst    Part-to-whole hierarchy as concentric rings (same inputs as
                treemap). Better for deeper hierarchies.
    map         Geographic point map (or --density heatmap) on tile basemaps.
                Pick the coordinate columns with the lat/lon options below.
    geo         Geographic point map on a projection basemap (coastlines/land/
                countries; no tiles, no token). Uses the same lat/lon options
                as `map`, plus --projection. Good for global/country-scale data.
    choropleth  Filled-region map: color whole regions (countries, US states, or
                custom GeoJSON areas) by a value. --locations names the region-code
                column, --value/--agg the measure (row counts if omitted). Defaults
                to a token-free projection basemap; --map switches to MapLibre tiles.

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

Examples:
  # Auto-dashboard for a dataset, opened in the browser
  qsv viz smart data.csv --open

  # Auto-dashboard, at most 6 panels in a 3-column grid, top-5 categories per bar
  qsv viz smart data.csv --max-charts 6 --grid-cols 3 --limit 5 -o dashboard.html

  # Bar chart of fruit prices, opened in the browser
  qsv viz bar fruits.csv --x Fruit --y Price --title "Fruit prices" --open

  # Aggregate (sum) sales by region into a bar chart
  qsv viz bar sales.csv --x region --y amount --agg sum -o sales.html

  # Scatter plot with a separate series (trace) per category
  qsv viz scatter data.csv --x age --y income --series gender -o scatter.html

  # Bubble scatter: marker size by population, marker color by a numeric score
  qsv viz scatter data.csv --x gdp --y life_exp --size population --color score -o bubble.html

  # Histogram of a numeric column with 30 bins
  qsv viz histogram data.csv --x value --bins 30 -o hist.html

  # Box plot of a value column grouped by a category, exported to PNG (needs viz_static)
  qsv viz box data.csv --y measurement --x group -o box.png

  # Box plot with every sample point overlaid (jittered) instead of just the outliers
  qsv viz box data.csv --y measurement --box-points all -o box.html

  # Pie chart of category proportions (counts), as a donut
  qsv viz pie data.csv --x category --donut -o pie.html

  # Correlation heatmap over all numeric columns
  qsv viz heatmap data.csv -o corr.html

  # Heatmap pivot: average value per (region x product)
  qsv viz heatmap sales.csv --x region --y product --z amount -o pivot.html

  # Candlestick chart from a date column and OHLC price columns
  qsv viz candlestick prices.csv --x date --ohlc-open open --high high --low low --close close -o ohlc.html

  # Sankey flow diagram of source -> target weighted by value
  qsv viz sankey flows.csv --source from --target to --value weight -o sankey.html

  # Radar chart comparing numeric metrics, one trace per team
  qsv viz radar teams.csv --cols speed,power,range,accuracy --series team -o radar.html

  # Point map of earthquakes, marker color by magnitude and size by depth
  qsv viz map quakes.csv --lat lat --lon lon --color magnitude --size depth -o map.html

  # Density heatmap of the same points, on a light Carto basemap
  qsv viz map quakes.csv --lat lat --lon lon --density --style carto-positron -o heat.html

  # 3D scatter of three numeric columns, colored by a fourth
  qsv viz scatter3d data.csv --x length --y width --z height --color weight -o scatter3d.html

  # 2D density contour of two numeric columns with a 40x40 grid
  qsv viz contour data.csv --x height --y weight --bins 40 -o contour.html

  # Projection map of earthquakes (token-free), marker color by magnitude
  qsv viz geo quakes.csv --lat lat --lon lon --color magnitude --projection natural-earth -o geo.html

  # Treemap of part-to-whole sales by region then category, sized by amount
  qsv viz treemap sales.csv --cols region,category --value amount --agg sum -o treemap.html

  # Sunburst of a deep 3-level web-traffic hierarchy, sized by row count
  qsv viz sunburst web.csv --cols source,campaign,landing_page -o sunburst.html

  # Choropleth coloring countries (ISO-3 codes) by a summed measure
  qsv viz choropleth gdp.csv --locations iso3 --value gdp --agg sum -o choropleth.html

  # US-state choropleth of row counts per state (2-letter state codes)
  qsv viz choropleth orders.csv --locations state --location-mode usa-states -o states.html

  # Custom GeoJSON regions on a MapLibre basemap, matched by a feature id
  qsv viz choropleth counties.csv --locations fips --value pop --map --geojson counties.json --feature-id-key id -o counties.html

  # Reverse-geocode lat/lon points to ISO-3 codes, then count per country (needs geocode feature)
  qsv viz choropleth stops.csv --geocode --lat lat --lon lon -o by_country.html

For more examples, see https://github.com/dathere/qsv/blob/master/tests/test_viz.rs.
See also https://github.com/dathere/qsv/wiki/Visualization

Usage:
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
    qsv viz choropleth  [options] <input>
    qsv viz treemap     [options] <input>
    qsv viz sunburst    [options] <input>
    qsv viz --help

viz options:
    -x, --x <col>          Column for the x-axis / category / bin / group.
    -y, --y <col>          Column for the y-axis / value.
    -z, --z <col>          The z column: a heatmap pivot value (with --x and --y), or
                           the third numeric axis for scatter3d.
    --cols <cols>          Columns to use. For heatmap: numeric columns for the
                           correlation matrix (default: all numeric). For radar:
                           the numeric axes to plot. For treemap/sunburst: the
                           categorical dimensions that form the hierarchy levels,
                           outermost first (e.g. region,category,subcategory).
    --series <col>         Column to split into multiple series (one trace per
                           distinct value). Applies to bar, line, scatter, scatter3d,
                           radar, map and geo.
    --color <col>          For scatter/scatter3d/map/geo: a numeric column to encode as
                           marker color (a continuous colorscale with a colorbar). For
                           categorical coloring, use the --series option instead. Cannot
                           be combined with --series. In map density mode, this column is
                           the heatmap weight.
    --size <col>           For scatter/scatter3d/map/geo: a numeric column to encode as
                           marker size, producing a bubble chart (values are rescaled to
                           a readable pixel range). Cannot be combined with --series. In
                           map density mode, this column is the heatmap weight.
    --donut                Render a pie chart as a donut (with a center hole).
    --ohlc-open <col>      Open-price column for candlestick/ohlc charts.
    --high <col>           High-price column for candlestick/ohlc charts.
    --low <col>            Low-price column for candlestick/ohlc charts.
    --close <col>          Close-price column for candlestick/ohlc charts.
    --source <col>         Source node column for a sankey diagram.
    --target <col>         Target node column for a sankey diagram.
    --value <col>          Flow value column for a sankey diagram. When omitted,
                           each row counts as a flow of 1. For treemap/sunburst:
                           a numeric measure summed per sector (when omitted, each
                           row counts as 1).
    --bins <n>             Number of bins. For histogram: bins along the x-axis
                           (default: auto). For contour: the per-axis resolution of
                           the density grid (default: 20).
    --agg <fn>             For bar/line, aggregate the y values when the x value
                           repeats. One of: sum, mean, count, min, max.
                           For treemap/sunburst, only additive aggregations apply:
                           count (default) or sum (requires --value).
    --box-points <mode>    Which sample points to draw alongside a box. Reading the
                           raw values lets plotly render true Tukey whiskers (1.5*IQR)
                           with the points beyond the fences as outliers. One of:
                           outliers (only the outliers), all (every point, jittered),
                           suspected (mark suspected outliers), none (no points, but
                           still real Tukey whiskers). For `viz box` the default is
                           outliers. For `viz smart` this flag OVERRIDES the default
                           size-based heuristic, which overlays all points for small
                           data (<=1,000 rows) and only the outliers for medium data
                           (<=10,000 rows). Above that, a column that HAS outliers shows
                           them as points on a precomputed quartile box (a single pass
                           collects only the out-of-fence values, capped); a column with
                           no outliers stays a fast cache-only quartile summary with no
                           data re-scan. An explicit mode is applied to every box panel
                           (one batched pass to read the values), except `none`, which
                           always keeps the cache-only box.

map options:
    --lat <col>            Latitude column for a map (decimal degrees, -90 to 90).
    --lon <col>            Longitude column for a map (decimal degrees, -180 to 180).
    --text <col>           Column whose value labels each point on hover.
    --density              Render a density heatmap (DensityMapbox) instead of points.
                           Weighted by the --color or --size column when given, else by
                           a uniform weight. Cannot be combined with --series.
    --style <name>         Map basemap style. Token-free styles: open-street-map (the
                           default), carto-positron, carto-darkmatter, stamen-terrain,
                           stamen-toner, stamen-watercolor, white-bg. Mapbox-hosted
                           styles (basic, streets, outdoors, light, dark, satellite,
                           satellite-streets) require --mapbox-token.
                           [default: open-street-map]
    --mapbox-token <tok>   Mapbox access token, required only for the mapbox-hosted
                           basemap styles listed above.
                           Can also be set with the QSV_MAPBOX_TOKEN environment
                           variable (the --mapbox-token flag takes precedence).

geo options:
    --projection <name>    Map projection for `viz geo`. One of: natural-earth (the
                           default), mercator, orthographic, equirectangular,
                           albers-usa, robinson, winkel-tripel, mollweide, hammer,
                           azimuthal-equal-area. `viz geo` also reuses the lat, lon,
                           text, color, size and series options from `map`.
                           [default: natural-earth]

choropleth options:
    --locations <col>      Column holding the region key for each row (an ISO-3 country
                           code, a 2-letter US state code, a country name, or a GeoJSON
                           feature id, per --location-mode). With --geocode, this instead
                           names a place-name column to forward-geocode into region codes.
    --location-mode <m>    How --locations values are matched to regions. One of: iso3
                           (the default, ISO-3166-1 alpha-3 country codes), usa-states
                           (2-letter US state codes), country-names (full country names),
                           geojson-id (match a --geojson feature id). [default: iso3]
    --color-scale <name>   Colorscale for the region fill. One of: viridis (the default),
                           cividis, greys, greens, blues, reds, ylgnbu, ylorrd, bluered,
                           rdbu, portland, electric, jet, hot, blackbody, earth, picnic,
                           rainbow. [default: viridis]
    --map                  Render on a token-free MapLibre tile basemap (a ChoroplethMap)
                           instead of the default projection basemap. Requires --geojson
                           and --feature-id-key. Reuses --style for the basemap.
    --geojson <src>        Custom region polygons as a local file path or an http(s) URL
                           to a GeoJSON FeatureCollection. Required for --map, and for
                           the geojson-id location mode.
    --feature-id-key <k>   Property path in each GeoJSON feature whose value matches an
                           entry in the locations column (e.g. id, properties.fips).
                           [default: id]
    --geocode              Derive the region codes by reusing qsv's geocode engine
                           (needs a build with the geocode feature). Either reverse-geocode
                           the lat/lon points, or forward-geocode the locations name
                           column. Only valid with location modes iso3 or usa-states.
                           `viz choropleth` also reuses --value, --agg, --style and the
                           lat/lon options.

smart options:
    --max-charts <n>       Maximum number of panels in the dashboard. 0 (the default)
                           means auto: draw every eligible column (up to 64), for both
                           HTML and static image export (png/svg/pdf/...). Up to 8
                           cartesian panels render as one typed subplot grid; beyond 8,
                           HTML switches to an inline-div grid of independent plots, and
                           static image export uses domain-positioned axes to fit them in
                           one image. Set a positive <n> to cap the panel count instead.
                           Eligible columns beyond the cap are reported but not drawn.
                           [default: 0]
    --grid-cols <n>        Number of columns in the dashboard grid for the per-column
                           distribution panels. Overview panels (map/geo, correlation,
                           time-series) always span the full width. [default: 2]
    --limit <n>            Top-N categories per frequency bar chart. [default: 10]
    --no-nulls             Omit the "(NULL)" bar (empty cells) from frequency bar charts.
                           By default `viz smart` shows a "(NULL)" bar, like `qsv frequency`.
    --no-other             Omit the "Other (N)" aggregate bar from frequency bar charts. It
                           collects the categories beyond --limit (N = how many distinct
                           categories were rolled up) and is shown by default.
    --smarter              Before building the dashboard, run `qsv moarstats --advanced`
                           to enrich the stats cache with distribution-shape statistics
                           (bimodality, entropy, skewness, outlier share). This unlocks
                           histograms for bimodal columns, frequency bars for concentrated
                           high-cardinality columns, and skew/outlier hints on box panels.
                           Costs one extra pass over the data and writes <stem>.stats.csv,
                           its sidecars, and an .idx index (like running `qsv moarstats`
                           manually). Only affects `smart`. Applied only with default
                           parsing; inputs using --no-headers or a custom --delimiter
                           fall back to the standard dashboard.
    --hierarchy-style <k>  For `smart`, the chart used for the categorical part-to-whole
                           hierarchy panel (built when 2+ low-cardinality dimensions exist).
                           One of: auto (default), treemap, sunburst. auto follows best
                           practice — a treemap for a shallow 2-level hierarchy (accurate
                           size comparison) and a sunburst for a deep 3-level one (parent
                           child structure). Only affects `smart`.
    --dictionary <src>     EXPERIMENTAL. Use a describegpt Data Dictionary to guide panel
                           selection from each field's semantic role/concept (falling back to
                           its content type) instead of relying on column statistics alone:
                           dimensions and numeric codes (ward, census_tract, zone) become bars,
                           measures get box/correlation/trend panels, date/datetime columns feed
                           the time-series panel (not noisy frequency bars), identifiers / PII /
                           free-text are skipped, and lat/lon feed the map. Field labels become
                           panel titles. Columns the dictionary cannot classify still use the
                           statistical heuristic. <src> is one of:
                           "infer" to run describegpt on the input now (with infer-content-type,
                           two-pass and jsonschema output; requires an LLM configured) and use
                           its output; or a path to an existing describegpt dictionary file
                           (jsonschema or json). Generation/read failures soft-fall back to the
                           stats-only dashboard. Only affects `smart`.
    --dictionary-context <file>  Path to a file with extra context about the dataset
                           (a glossary, README, data dictionary, PDF, etc.) forwarded to
                           describegpt as --context-file when `--dictionary infer` generates the
                           dictionary. Better context yields better role/concept/label/grain
                           tags, hence a better dashboard. Ignored unless `--dictionary infer`
                           is used (it does not apply when reading an existing dictionary file).
                           Only affects `smart`.
    --log-scale <mode>     Use a logarithmic y-axis for frequency bar panels whose
                           tallest bar dwarfs the rest (e.g. a large "(NULL)" or
                           "Other (N)" bucket), so the small categories stay visible.
                           One of: auto, on, off. "auto" (the default) switches a panel
                           to a log y-axis only when its dynamic range is high; "on"
                           forces a log y-axis on every frequency panel; "off" keeps the
                           linear axes. Only affects `smart`. [default: auto]

    --title <s>            Chart title.
    --x-title <s>          X-axis title. (defaults to the x column name)
    --y-title <s>          Y-axis title. (defaults to the y column name)
    --theme <name>         Plotly theme that drives the chart's overall look
                           (background, fonts, axis styling). One of: default,
                           plotly_white, plotly_dark, seaborn, seaborn_whitegrid,
                           seaborn_dark, matplotlib, plotnine (case-insensitive;
                           hyphens accepted). When omitted, qsv's built-in look
                           is used. Applies to all chart types, including `smart`.
    --width <n>            Image width in pixels for static export. Default 1000;
                           for `smart`, auto-scaled to the grid's column count.
    --height <n>           Image height in pixels for static export. Default 600;
                           for `smart`, auto-scaled to the number of panel rows.
    --scale <f>            Image scale factor (static export). [default: 1.0]
    --open                 Open the generated chart in the default browser/viewer.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout. The chart
                           format is inferred from the extension: .html (default),
                           .png, .svg, .pdf, .jpeg, .webp.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
    -n, --no-headers       When set, the first row will not be interpreted
                           as headers. Columns can then only be selected by index.
"#;

use std::{
    collections::{BTreeMap, HashMap},
    io::Write,
};

use indicatif::HumanCount;
// the Core/Full extent zoom buttons are part of the geocode-gated spatial-extent overlay
#[cfg(feature = "geocode")]
use plotly::layout::update_menu::{
    Button, ButtonMethod, UpdateMenu, UpdateMenuDirection, UpdateMenuType,
};
use plotly::{
    Bar, BoxPlot, Candlestick, Choropleth, ChoroplethMap, Configuration, Contour, DensityMapbox,
    HeatMap, Histogram, Ohlc, Pie, Plot, Sankey, Scatter, Scatter3D, ScatterGeo, ScatterMapbox,
    ScatterPolar, Sunburst, Trace, Treemap,
    box_plot::{BoxPoints, QuartileMethod},
    choropleth::{LocationMode, Marker as ChoroplethMarker},
    color::NamedColor,
    common::{
        Anchor, ColorBar, ColorScale, ColorScalePalette, Fill, Font, HoverInfo, Line, Marker, Mode,
        Pattern, PatternShape, TextPosition, TickMode, Title,
    },
    layout::{
        Annotation, Axis, AxisType, Center, HoverMode, Layout, LayoutGeo, LayoutMap, LayoutScene,
        MapStyle, Mapbox, MapboxStyle, Margin, Projection, ProjectionType, themes::BuiltinTheme,
    },
    sankey::{Link, Node},
    sunburst::InsideTextOrientation,
    treemap::{BranchValues, Marker as TreemapMarker, Pad},
};
use serde::Deserialize;

use crate::{
    CliResult,
    config::{Config, Delimiter},
    select::SelectColumns,
    util,
};

/// Plotly's `Layout` exposes typed axis fields only up to x_axis8/y_axis8, which caps a
/// single-`Plot` typed subplot grid at 8 panels. HTML dashboards that need more panels are
/// rendered as an inline-div grid of independent plots instead (see `render_smart_inline`).
const MAX_SUBPLOTS: usize = 8;

/// Hard ceiling on panels for the inline-div HTML dashboard (used when `--max-charts` exceeds
/// `MAX_SUBPLOTS` and the output is HTML). Bounds the page size for very wide datasets.
const MAX_PANELS_INLINE: usize = 64;

/// A column whose cardinality is at or below this is treated as categorical (frequency
/// bar) rather than continuous (box plot).
const CATEGORICAL_MAX_CARDINALITY: u64 = 30;

/// `viz smart` categorical part-to-whole (treemap/sunburst) panel tuning.
/// Maximum nesting levels (root excluded): with 3+ eligible low-cardinality dimensions a
/// 3-level hierarchy is built (rendered as a sunburst per best practice for deep paths),
/// otherwise a 2-level hierarchy (rendered as a treemap for shallow size comparison).
const HIER_MAX_DEPTH: usize = 3;
/// Per parent, only the top-N children (by value desc, label asc) are kept; the remainder
/// collapse into a single "Other (k)" sibling (mirrors `finalize_freq_bars`).
const HIER_TOP_N_PER_LEVEL: usize = 12;
/// Global safety cap on emitted hierarchy nodes; once reached, deeper expansion stops
/// (capped nodes render as leaves). Keeps a pathological column combo from exploding.
const HIER_MAX_NODES: usize = 200;
/// Minimum eligible categorical dimensions required to build a hierarchy panel at all.
const HIER_MIN_DIMS: usize = 2;
/// A dimension needs at least this many distinct values to be worth a hierarchy level: a 2-value
/// (boolean-like) column is a trivial split better shown as a single bar, and nesting by it would
/// needlessly force the whole dashboard onto the inline (non-typed-grid) render path.
const HIER_MIN_DIM_CARDINALITY: u64 = 3;
/// Path separator used to build collision-free plotly node `ids` (US control char, which
/// cannot occur in trimmed cell text).
const HIER_PATH_SEP: char = '\u{001F}';
/// Synthetic root node id for the hierarchy (parent of the level-1 categories).
const HIER_ROOT_ID: &str = "\u{001F}root";
/// Inline render height (px) for a hierarchy panel — taller than a normal overview cell so
/// nested treemap rectangles / sunburst rings stay legible.
const HIER_ROW_HEIGHT_PX: usize = 520;
/// Minimum (bias-corrected) Cramér's V — across every pair of the candidate hierarchy dimensions —
/// for `viz smart` to AUTO-build a treemap/sunburst. Nesting statistically independent categoricals
/// just replicates each level's marginal at every branch, conveying no structure that the separate
/// per-column frequency bars don't already show more legibly; below this association the panel is
/// skipped (a note is printed). 0.10 is Cohen's "small effect" floor for V. An explicit
/// `--hierarchy-style treemap|sunburst` bypasses the screen (the user asked for it deliberately).
const HIER_MIN_ASSOCIATION_CRAMERS_V: f64 = 0.10;

/// Plotly `maxdepth` for sunburst panels: the number of levels rendered at once from the current
/// root, counting the center as one. 3 = center + 2 data rings. A 3-level sunburst fanned out to
/// ~100 tiny outer sectors whose labels were unreadable; capping the initial view to two rings
/// keeps every visible label legible, and clicking a sector rescales its subtree to the full ring
/// so the deeper level (with its now-large labels) is revealed on drill-down. For 2-level
/// hierarchies this is a no-op (both rings already fit within the cap).
const SUNBURST_MAXDEPTH: i32 = 3;

/// `viz smart --log-scale auto`: a frequency bar panel switches to a logarithmic y-axis when
/// its tallest bar is at least this many times the shortest positive bar. A dominating
/// "(NULL)"/"Other (N)" bucket flattens the real categories to invisible slivers on a linear
/// axis; a log scale keeps them legible. Requires at least 3 positive bars.
const LOG_SCALE_MIN_RATIO: f64 = 50.0;

/// Minimum absolute Pearson correlation for `viz smart` to add a scatter panel of the most
/// strongly correlated numeric pair. Below this (a weak relationship) the scatter is just a
/// noise cloud, so it's skipped — the correlation heatmap already conveys weak correlations.
const SCATTER_PAIR_MIN_ABS_R: f64 = 0.5;

/// `viz smart` skips the 3D scatter drill-down when the strongest numeric pair is at or above this
/// absolute correlation. The 3D is built ON that pair plus a third axis; when the pair is
/// near-collinear (|r| ~ 1) the two axes are effectively the same variable, so the cloud collapses
/// onto a plane and the "3D" adds nothing the 2D pair drill-down doesn't already show. The third
/// axis is otherwise chosen to be the LEAST redundant with the pair so the panel uses all three
/// dimensions.
const SMART_3D_COLLINEAR_MAX_ABS_R: f64 = 0.97;

/// `viz smart` flags the correlation drill-down pair as nonlinear when its Spearman |rho| exceeds
/// its Pearson |r| by at least this much. A large gap means the relationship is monotonic but
/// curved, so the single Pearson number understates it — the panel title notes the rho so the
/// reader doesn't read the cloud as merely linear. Pearson alone can't see this.
const SMART_NONLINEAR_MIN_GAP: f64 = 0.15;

/// `viz pie` prints a non-fatal advisory (suggesting a bar chart) when it has at least
/// `PIE_NEAR_EQUAL_MIN_SLICES` slices whose coefficient of variation is below this: near-equal
/// slices are the worst case for a pie (the eye can't compare similar angles/areas), and a bar
/// chart is strictly easier to read. The explicit `viz pie` request is still honored.
const PIE_NEAR_EQUAL_MAX_CV: f64 = 0.35;
const PIE_NEAR_EQUAL_MIN_SLICES: usize = 4;

/// At or above this many complete rows, `viz smart` draws the strongest-correlated numeric pair as
/// a 2D density contour instead of a scatter: past this point a scatter overplots into a solid
/// mass, while a binned contour embeds only a fixed grid (so it's both more readable and far
/// smaller).
const SMART_CONTOUR_MIN_POINTS: usize = 5_000;

/// Bin resolution (per axis) for `viz smart`'s correlated-pair density contour panel.
const SMART_CONTOUR_BINS: usize = 30;

/// `viz smart` row-count thresholds for the default (no explicit `--box-points`) box-overlay
/// heuristic: at or below ALL, every sample point is overlaid on a box (via `BoxRaw`); at or below
/// OUTLIERS, only the Tukey outliers (via `BoxRaw`). ABOVE OUTLIERS, embedding the full column is
/// an unreadable smear and a large payload, so the box stays a precomputed quartile box — but if
/// the column actually HAS outliers (cached min/max fall outside the Tukey fences), they're
/// overlaid as native box points via a single fence-filtered pass (`BoxOutliers`); a column with
/// no outliers stays a pure cache-only `BoxStats` with no pass at all.
const SMART_BOX_ALL_MAX: u64 = 1_000;
const SMART_BOX_OUTLIERS_MAX: u64 = 10_000;

/// Max number of outlier points embedded per `BoxOutliers` panel, keeping the HTML bounded for
/// heavy-tailed columns. When a column has more outliers than this, the rest are dropped (the
/// overflow is logged); far below `MAX_SMART_POINTS` since these all render as discrete dots.
const SMART_BOX_OUTLIERS_CAP: usize = 5_000;

/// `viz smart` renders its geographic panel as a `ScatterGeo` projection world-overview (offline,
/// no tiles) instead of a zoomed mapbox tile map when the coordinates span at least this many
/// degrees of longitude OR latitude — i.e. continental/global data, where a whole-world projection
/// gives better context than tiles framed to a wide bounding box. Local extents keep the tile map.
const SMART_GEO_MIN_LON_SPAN_DEG: f64 = 90.0;
const SMART_GEO_MIN_LAT_SPAN_DEG: f64 = 45.0;

/// Bimodality-coefficient threshold (Sarle's BC). A continuous numeric column whose
/// `bimodality_coefficient` reaches this AND is platykurtic (see `classify_measure`) is treated as
/// bimodal/multimodal, so `viz smart` draws a histogram (which shows the separate peaks) instead of
/// a box plot (which hides them). The coefficient is supplied by moarstats under `--smarter`, or
/// computed in one streaming pass by `enrich_bimodality` for plain `viz smart`.
///
/// Sarle's textbook cutoff is 5/9 (~0.5556) — the value for a UNIFORM distribution. But finite
/// samples of a uniform (or near-uniform) column scatter just ABOVE 5/9, so a strict cutoff
/// over-flags flat-but-unimodal data as "bimodal". A small margin (0.60) keeps genuinely two-peaked
/// columns (BC typically 0.7-1.0) while letting near-uniform columns stay box plots.
const BIMODALITY_COEFFICIENT_THRESHOLD: f64 = 0.60;

/// `viz smart` charts a high-cardinality categorical column (one that would otherwise be skipped)
/// only when moarstats says its distribution is concentrated rather than near-uniform: a
/// `normalized_entropy` at/above this is treated as noise (every value about equally frequent),
/// while a lower value means a few dominant categories worth a top-N bar.
const HIGH_CARD_ENTROPY_NOISE_THRESHOLD: f64 = 0.95;

/// Max bars in a single-series `viz bar` chart that still get value labels; beyond this the
/// labels would overlap, so they're omitted.
const LABEL_MAX_BARS: usize = 40;

/// `viz smart` caps how many data points it embeds in a single panel (map, time-series, and
/// correlated-pair scatter). Beyond this, the points are uniformly downsampled. The motivating
/// case: a 1M-row dataset's map embedded 745K opaque markers — an unreadable blob, a ~25 MB
/// payload, and a browser that froze on the first pan/zoom. Uniform stride sampling preserves the
/// overall shape/distribution of the data.
const MAX_SMART_POINTS: usize = 50_000;

/// At or above this many mappable rows, `viz smart` draws its map panel as a density heatmap
/// (DensityMapbox) rather than individual markers, which overplot into a solid mass at scale.
const MAP_DENSITY_MIN_POINTS: usize = 20_000;

/// Max geographic outlier points embedded per `viz smart` map panel. Outliers are the whole point
/// of the call-out, so this is set generously, but still bounds the embedded payload; if there are
/// more, they're uniformly stride-sampled (like the core points) rather than dropped wholesale.
const SMART_GEO_OUTLIER_CAP: usize = 5_000;

/// Marker opacity for the `viz smart` map panel when it's drawn as discrete points (i.e. fewer
/// than `MAP_DENSITY_MIN_POINTS` rows). Mild transparency reveals overlapping points instead of a
/// flat blob.
const MAP_POINT_OPACITY: f64 = 0.4;

/// Fraction trimmed from each end of the latitude/longitude distributions when framing a map, so a
/// few outlier coordinates (bad geocodes, sentinel values) can't blow up the center/zoom and push
/// the bulk of the data off-screen. 2.5% off each end — only the initial view is trimmed; every
/// point is still plotted, so a slightly tight default frame just means panning out to see the
/// long tail.
const MAP_FRAME_TRIM_FRAC: f64 = 0.025;

/// Marker diameter (pixels) the smallest and largest `--size` values map to in a bubble
/// scatter. Raw size values are linearly rescaled into this range so the plot stays
/// readable regardless of the column's magnitude.
const BUBBLE_MIN_PX: f64 = 6.0;
const BUBBLE_MAX_PX: f64 = 40.0;

/// Vertical space (in pixels) allotted to each row of subplots in the `viz smart`
/// dashboard. The total plot height scales with the number of rows so panels stay
/// readable instead of being crammed into plotly's ~450px default.
const ROW_HEIGHT_PX: usize = 320;

/// Taller height (in pixels) for full-width "overview" panels (map/geo, correlation heatmap and
/// its drill-downs, time-series) in the inline HTML dashboard, so they get more room than the
/// per-column box/bar/histogram panels — and, for a map, so the spatial-extent caption below it
/// has space.
const OVERVIEW_ROW_HEIGHT_PX: usize = 420;

/// Horizontal space (in pixels) per dashboard grid column, used to auto-size the `viz
/// smart` static image export width.
const SMART_COL_WIDTH_PX: usize = 500;

/// Fallback static-export image dimensions (pixels) when --width/--height are not given
/// and no auto-scaling applies (i.e. the non-`smart` chart types).
const DEFAULT_IMG_WIDTH: usize = 1000;
const DEFAULT_IMG_HEIGHT: usize = 600;

/// Web-Mercator tile size (px): mapbox zoom z fits 360° of longitude across `512 * 2^z` px.
/// The unit the slippy-tile zoom math in `fitbounds_zoom` is defined against.
const MAPBOX_TILE_SIZE_PX: f64 = 512.0;

/// Anti-clip margin for the `fitbounds_zoom` framing (a 1.15× span pad ≈ a 0.20 zoom reduction).
const MAP_FIT_PAD: f64 = 1.15;

/// Usable mapbox draw height (px) for a `viz smart` MAP overview panel: `OVERVIEW_ROW_HEIGHT_PX`
/// (420) minus the map layout's top (48) + bottom (20) margins = 352. This is the *reliable*
/// dimension at HTML-generation time (width is `responsive`/unknown) and, for the wide-short map
/// panel, the binding constraint — so the latitude fit is computed against it.
const MAP_PANEL_USABLE_HEIGHT_PX: f64 = OVERVIEW_ROW_HEIGHT_PX as f64 - 48.0 - 20.0;

/// Assumed mapbox draw width (px) for the smart MAP panel longitude fit. The panel spans the full
/// grid (`grid-column: 1 / -1`) and its real width is responsive/unknown when the HTML is
/// generated. The reliable `MAP_PANEL_USABLE_HEIGHT_PX` term binds the common case (a ~square or
/// tall extent is latitude-bound, so this width is irrelevant to it); the width term only matters
/// for a genuinely WIDE extent (a far east/west outlier stretching longitude). For those, over-
/// assuming the width would *over*-zoom and clip the extent on a narrower viewport — and silently
/// hiding points (especially the strays the Full extent exists to reveal) is worse than leaving
/// horizontal map context. So this is tied to a conservative minimum desktop content width (~960px,
/// safe for windows down to ~1024px): no over-zoom/clipping on supported viewports, at the cost of
/// some horizontal margin for wide extents on larger screens. A truly exact fit would recompute
/// from the live plot-div width in the browser (a possible follow-up).
const MAP_PANEL_ASSUMED_WIDTH_PX: f64 = 960.0;

/// Soft qualitative palette (Vega/Tableau-10) for coloring dashboard panels — distinct
/// but harmonious, and friendlier than plotly's saturated defaults.
const PALETTE: [&str; 8] = [
    "#4C78A8", "#F58518", "#54A24B", "#E45756", "#72B7B2", "#EECA3B", "#B279A2", "#FF9DA6",
];

/// Shared UI font stack and "ink" color for all dashboard text.
const FONT_FAMILY: &str = "Helvetica Neue, Helvetica, Arial, sans-serif";
const INK: &str = "#2A3F5F";
const PAPER_BG: &str = "#FFFFFF";
const GRID_COLOR: &str = "#ECECEC";
const AXIS_LINE: &str = "#BCC4CE";

/// Dashboard plot margins (pixels). Kept as named constants because the title-band math
/// below needs the plot-area height (total height minus these margins).
const TOP_MARGIN_PX: usize = 80;
const BOTTOM_MARGIN_PX: usize = 60;

/// Pixels reserved above the top row of panels for their titles, and the gap (in pixels)
/// between a cell's top edge and its title. Both are kept in *pixels* (converted to paper
/// fractions via the plot-area height) so neither the band nor the offset scales with the
/// dashboard height — otherwise a tall dashboard's title would drift past `y=1` and overlap
/// the dashboard title. The band must comfortably exceed `TITLE_OFFSET_PX` plus the title's
/// rendered glyph height (~17px for the 13px font).
const TITLE_BAND_PX: usize = 32;
const TITLE_OFFSET_PX: usize = 6;

/// Default dashboard left margin (pixels), widened when a correlation-heatmap panel is present
/// so its (long) numeric-column tick labels aren't clipped.
const DEFAULT_LEFT_MARGIN_PX: usize = 60;

/// Y-axis title shown ONLY on frequency panels rendered with a logarithmic y-axis (see
/// `--log-scale`). It's the visual cue that the axis is log, not linear; linear panels stay
/// title-less to keep the cells compact. The rotated title needs a little extra left margin
/// (`LOG_AXIS_TITLE_MARGIN_PX`) so it isn't clipped against the page edge.
const LOG_AXIS_TITLE: &str = "count (log)";
const LOG_AXIS_TITLE_MARGIN_PX: usize = 20;

/// `viz smart` correlation-heatmap panel tuning. Long numeric-column names would clip against
/// the dashboard's left margin, so its axis tick labels are truncated to this many characters
/// and the left margin is widened (≈ this many pixels per character) to fit them. In-cell `r`
/// value labels are only drawn when the matrix is small enough to stay legible in one cell.
const CORR_LABEL_MAX_CHARS: usize = 16;
const CORR_LABEL_PX_PER_CHAR: usize = 7;
const CORR_INCELL_MAX_N: usize = 8;

/// `viz smart` frequency-bar panel tuning. Very long category names (e.g. full agency names or
/// datetime strings) rotate into tall x-axis tick labels that squeeze the plot area, clipping
/// the outside value labels at the top of the cell. Displayed tick labels are truncated to this
/// many characters (the full value is preserved on hover) to keep the plot area tall enough.
const BAR_LABEL_MAX_CHARS: usize = 20;

/// Default labels for the aggregate frequency-bar buckets in `viz smart`, mirroring `qsv
/// frequency`'s defaults: the `(NULL)` bar collects empty cells and `Other (N)` collects the
/// categories beyond `--limit` (N = the count of those distinct categories). Suppressed by
/// `--no-nulls` / `--no-other` respectively.
const NULL_TEXT: &str = "(NULL)";
const OTHER_TEXT: &str = "Other";

/// Muted grey for the aggregate `(NULL)` / `Other (N)` frequency bars so they read as summary
/// buckets, visually distinct from the palette-colored real categories.
const MUTED_COLOR: &str = "#999999";

/// Zero-width space suffixed onto the plotly category-axis key of the synthetic aggregate
/// frequency bars. It keeps their `x_key` distinct from a real category that happens to share
/// their display label (so the two never collapse onto the same plotly bar) while staying
/// invisible in the hovered key.
const AGG_KEY_SENTINEL: char = '\u{200B}';

/// One bar in a `viz smart` frequency panel.
#[derive(Clone)]
struct FreqBar {
    /// The plotly category-axis key — distinct within the panel so bars never collapse onto
    /// the same category slot. Real categories use their (trimmed) value; the synthetic
    /// aggregate buckets append `AGG_KEY_SENTINEL` so they can't collide with a real category
    /// that shares their display label.
    x_key: String,
    /// Friendly display label, used for the x-axis tick text (and the hovered category for
    /// real categories).
    label: String,
    count: u64,
    /// Whether this is a real category or a synthetic aggregate (NULL / Other) bucket. This —
    /// not the label — drives the bar color, so a real category literally named "(NULL)" or
    /// "Other (5)" is never mis-colored as an aggregate.
    kind:  FreqBarKind,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum FreqBarKind {
    Category,
    Aggregate,
}

/// Per-column frequency bars for the `viz smart` panels, keyed by column index.
type FreqMap = HashMap<usize, Vec<FreqBar>>;

#[derive(Deserialize)]
struct Args {
    cmd_smart:               bool,
    cmd_bar:                 bool,
    cmd_line:                bool,
    cmd_scatter:             bool,
    cmd_scatter3d:           bool,
    cmd_histogram:           bool,
    cmd_box:                 bool,
    cmd_pie:                 bool,
    cmd_heatmap:             bool,
    cmd_contour:             bool,
    cmd_candlestick:         bool,
    cmd_ohlc:                bool,
    cmd_sankey:              bool,
    cmd_radar:               bool,
    cmd_map:                 bool,
    cmd_geo:                 bool,
    cmd_treemap:             bool,
    cmd_sunburst:            bool,
    cmd_choropleth:          bool,
    arg_input:               Option<String>,
    flag_x:                  Option<SelectColumns>,
    flag_y:                  Option<SelectColumns>,
    flag_z:                  Option<SelectColumns>,
    flag_cols:               Option<SelectColumns>,
    flag_series:             Option<SelectColumns>,
    flag_donut:              bool,
    // scatter encodings: map a numeric column to per-point marker color (continuous
    // colorscale) and/or marker size (bubble chart). Mutually exclusive with --series.
    flag_color:              Option<SelectColumns>,
    flag_size:               Option<SelectColumns>,
    // candlestick / ohlc columns (--open is already taken by the browser-open flag below,
    // so the open-price column is selected with --ohlc-open)
    flag_ohlc_open:          Option<SelectColumns>,
    flag_high:               Option<SelectColumns>,
    flag_low:                Option<SelectColumns>,
    flag_close:              Option<SelectColumns>,
    // sankey columns
    flag_source:             Option<SelectColumns>,
    flag_target:             Option<SelectColumns>,
    flag_value:              Option<SelectColumns>,
    // map columns/options
    flag_lat:                Option<SelectColumns>,
    flag_lon:                Option<SelectColumns>,
    flag_text:               Option<SelectColumns>,
    flag_density:            bool,
    flag_style:              Option<String>,
    flag_mapbox_token:       Option<String>,
    flag_projection:         Option<String>,
    // choropleth columns/options
    flag_locations:          Option<SelectColumns>,
    flag_location_mode:      Option<String>,
    flag_color_scale:        Option<String>,
    flag_map:                bool,
    flag_geojson:            Option<String>,
    flag_feature_id_key:     Option<String>,
    flag_geocode:            bool,
    flag_bins:               Option<usize>,
    flag_agg:                Option<String>,
    flag_box_points:         Option<String>,
    flag_max_charts:         usize,
    flag_grid_cols:          usize,
    flag_limit:              usize,
    flag_no_nulls:           bool,
    flag_no_other:           bool,
    flag_smarter:            bool,
    flag_hierarchy_style:    Option<String>,
    flag_dictionary:         Option<String>,
    flag_dictionary_context: Option<String>,
    flag_log_scale:          String,
    flag_title:              Option<String>,
    flag_x_title:            Option<String>,
    flag_y_title:            Option<String>,
    flag_theme:              Option<String>,
    // width/height/scale only affect static image export (the viz_static feature). width
    // and height are optional: when unset, `viz smart` derives them from its grid shape and
    // other charts fall back to the defaults below.
    flag_width:              Option<usize>,
    flag_height:             Option<usize>,
    #[cfg_attr(not(feature = "viz_static"), allow(dead_code))]
    flag_scale:              f64,
    flag_open:               bool,
    flag_output:             Option<String>,
    flag_delimiter:          Option<Delimiter>,
    flag_no_headers:         bool,
}

/// The chart image format, derived from the --output extension.
#[derive(Clone, Copy, PartialEq, Eq)]
enum OutFormat {
    Html,
    Png,
    Jpeg,
    Webp,
    Svg,
    Pdf,
}

impl OutFormat {
    fn from_output(path: &str) -> Option<Self> {
        let ext = std::path::Path::new(path)
            .extension()
            .and_then(|e| e.to_str())
            .map(str::to_ascii_lowercase)
            .unwrap_or_default();
        match ext.as_str() {
            "html" | "htm" => Some(Self::Html),
            "png" => Some(Self::Png),
            "jpeg" | "jpg" => Some(Self::Jpeg),
            "webp" => Some(Self::Webp),
            "svg" => Some(Self::Svg),
            "pdf" => Some(Self::Pdf),
            _ => None,
        }
    }

    const fn is_image(self) -> bool {
        !matches!(self, Self::Html)
    }
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let mut args: Args = util::get_args(USAGE, argv)?;

    if let Some(name) = &args.flag_theme
        && parse_theme(name).is_none()
    {
        return fail_incorrectusage_clierror!(
            "Unknown --theme '{name}'. Valid themes: {VALID_THEMES}."
        );
    }

    // --mapbox-token falls back to the QSV_MAPBOX_TOKEN env var when not passed explicitly.
    if args.flag_mapbox_token.is_none()
        && let Ok(token) = std::env::var("QSV_MAPBOX_TOKEN")
        && !token.is_empty()
    {
        args.flag_mapbox_token = Some(token);
    }

    let out_format = match args.flag_output.as_deref() {
        Some(path) => match OutFormat::from_output(path) {
            Some(fmt) => fmt,
            None => {
                return fail_incorrectusage_clierror!(
                    "Unsupported output extension for '{path}'. Use one of: html, png, jpeg, \
                     webp, svg, pdf."
                );
            },
        },
        None => OutFormat::Html,
    };

    if out_format.is_image() && args.flag_output.is_none() {
        return fail_incorrectusage_clierror!(
            "Image formats require an --output file (image data cannot be written to stdout)."
        );
    }
    #[cfg(not(feature = "viz_static"))]
    if out_format.is_image() {
        return fail_clierror!(
            "Static image export requires a qsv build with the `viz_static` feature (or a \
             prebuilt qsv binary). Output an .html file instead, or rebuild with `--features \
             viz_static`."
        );
    }

    let (mut plot, smart_dims) = if args.cmd_smart {
        match build_smart(&args, out_format)? {
            // >8-panel HTML dashboards are assembled as an inline-div grid that bypasses the
            // single-`Plot` output path entirely.
            SmartRender::Inline(html) => return output_inline_html(&html, &args),
            // >8-panel static image export is a raw Plotly JSON value (with xaxis9+/yaxis9+ that
            // the typed `Plot` can't hold), rendered directly via the static exporter.
            SmartRender::GridJson { value, dims } => {
                let img_width = args.flag_width.unwrap_or(dims.0);
                let img_height = args.flag_height.unwrap_or(dims.1);
                return output_image_json(
                    &value,
                    &args,
                    out_format,
                    img_width,
                    img_height,
                    args.flag_scale,
                );
            },
            SmartRender::Grid {
                plot,
                dims,
                title,
                theme,
            } => {
                // HTML smart-grid is wrapped in qsv's own page so it gets the light/dark toggle;
                // plotly's `to_html()` (used by the generic single-`Plot` path) has no injection
                // point. Static image export keeps the typed `Plot` path below.
                if matches!(out_format, OutFormat::Html) {
                    let html = render_smart_grid_page(*plot, theme, &title);
                    return output_inline_html(&html, &args);
                }
                (plot, Some(dims))
            },
        }
    } else {
        (Box::new(build_plot(&args, out_format)?), None)
    };

    // make the interactive HTML re-fit its width to the window/container on resize; this
    // is ignored by static image export (which sizes via --width/--height).
    plot.set_configuration(Configuration::new().responsive(true));

    // resolve static-export dimensions: an explicit --width/--height always wins, else the
    // `smart` auto-scaled size, else the fixed fallback.
    let (smart_w, smart_h) = smart_dims.unzip();
    let img_width = args.flag_width.or(smart_w).unwrap_or(DEFAULT_IMG_WIDTH);
    let img_height = args.flag_height.or(smart_h).unwrap_or(DEFAULT_IMG_HEIGHT);

    output_plot(
        &plot,
        &args,
        out_format,
        img_width,
        img_height,
        args.flag_scale,
    )
}

/// The ways `viz smart` can render: a single-`Plot` typed subplot grid (up to `MAX_SUBPLOTS`
/// panels; supports static image export), a raw-JSON subplot grid with domain-positioned axes
/// (for static image export of >`MAX_SUBPLOTS` panels, where plotly's typed `Layout` runs out
/// of axis fields), or a self-contained inline-div HTML page (for >8-panel HTML dashboards).
enum SmartRender {
    // `Plot` is large; box it so the enum isn't bloated by the rarely-larger variant.
    Grid {
        plot:  Box<Plot>,
        dims:  (usize, usize),
        // carried out so the HTML path can wrap the plot in qsv's own toggle-enabled page
        // (`render_smart_grid_page`); the image-export path ignores both.
        title: String,
        theme: Option<BuiltinTheme>,
    },
    /// A fully-assembled Plotly JSON value (data + layout). Used only for static image export of
    /// more than `MAX_SUBPLOTS` panels: the layout carries `xaxis9+`/`yaxis9+`, which the typed
    /// `Layout` can't express, so it's rendered via `StaticExporter::write_fig` (raw JSON).
    GridJson {
        value: Box<serde_json::Value>,
        dims:  (usize, usize),
    },
    Inline(String),
}

/// Write a pre-assembled inline-div dashboard HTML string to `--output` (or stdout), honoring
/// `--open`. When `--open` is set without `--output`, the HTML is also written to a securely
/// created temporary file which is then opened — mirroring plotly's own `Plot::show()` for the
/// single-`Plot` path.
fn output_inline_html(html: &str, args: &Args) -> CliResult<()> {
    match args.flag_output.as_deref() {
        Some(path) => {
            std::fs::write(path, html)?;
            if args.flag_open {
                open_path(path)?;
            }
        },
        None => {
            std::io::stdout().write_all(html.as_bytes())?;
            if args.flag_open {
                // Create the temp file via tempfile (random name, O_EXCL) to avoid a symlink
                // attack on a predictable path, then persist it so the browser can read it
                // after qsv exits (a NamedTempFile would otherwise delete on drop).
                let mut tmp = tempfile::Builder::new()
                    .prefix("qsv-viz-smart-")
                    .suffix(".html")
                    .tempfile()?;
                tmp.write_all(html.as_bytes())?;
                let (_file, path) = tmp.keep().map_err(|e| {
                    crate::CliError::Other(format!("Could not persist temp dashboard file: {e}"))
                })?;
                open_path(&path.to_string_lossy())?;
            }
        },
    }
    Ok(())
}

/// Build a `Plot` for the requested chart subcommand.
fn build_plot(args: &Args, out_format: OutFormat) -> CliResult<Plot> {
    // --color/--size are per-point marker encodings that apply to scatter and map only, and
    // need a single trace, so they can't be combined with --series (which splits into traces).
    if encoded_scatter(args) {
        if !matches!(
            chart_kind(args),
            Chart::Scatter | Chart::Scatter3D | Chart::Map | Chart::Geo
        ) {
            return fail_incorrectusage_clierror!(
                "--color/--size only apply to `viz scatter`, `viz scatter3d`, `viz map` and `viz \
                 geo`."
            );
        }
        if args.flag_series.is_some() {
            return fail_incorrectusage_clierror!(
                "--color/--size cannot be combined with --series. Use --series to split into \
                 colored traces by category, or --color/--size to encode numeric columns onto a \
                 single series."
            );
        }
    }

    // maps use a `mapbox` layout (tile basemap, center, zoom) rather than cartesian x/y axes,
    // so they own their whole `Plot` and bypass the cartesian `build_layout` below.
    if matches!(chart_kind(args), Chart::Map) {
        return build_map_plot(args, out_format);
    }

    // `geo` uses a `geo` layout (projection basemap) and `scatter3d` a `scene` layout (3D
    // axes); neither is cartesian, so each owns its whole `Plot` like the map above.
    if matches!(chart_kind(args), Chart::Geo) {
        return build_geo_plot(args);
    }

    // choropleth fills whole regions on a `geo` (projection) or `map` (MapLibre) subplot — never
    // cartesian — so it owns its whole `Plot` like the maps above.
    if matches!(chart_kind(args), Chart::Choropleth) {
        return build_choropleth_plot(args, out_format);
    }
    if matches!(chart_kind(args), Chart::Scatter3D) {
        return build_scatter3d_plot(args);
    }

    // treemap / sunburst are domain-based (no cartesian x/y axes), like pie — they own their whole
    // `Plot` as well.
    if matches!(chart_kind(args), Chart::Treemap | Chart::Sunburst) {
        return build_hierarchy_plot(args);
    }

    let mut plot = Plot::new();

    // default axis titles, derived from the selected column names
    let (default_x, default_y): (Option<String>, Option<String>) = match chart_kind(args) {
        Chart::Histogram => {
            let (trace, x_label) = build_histogram(args)?;
            plot.add_trace(trace);
            (Some(x_label), Some("count".to_string()))
        },
        Chart::Box => {
            let (trace, y_label, x_label) = build_box(args)?;
            plot.add_trace(trace);
            (x_label, Some(y_label))
        },
        // pie / sankey / radar are not cartesian, so they get no x/y axis titles
        Chart::Pie => {
            plot.add_trace(build_pie(args)?);
            (None, None)
        },
        Chart::Sankey => {
            plot.add_trace(build_sankey(args)?);
            (None, None)
        },
        Chart::Radar => {
            for trace in build_radar(args)? {
                plot.add_trace(trace);
            }
            (None, None)
        },
        Chart::Heatmap => {
            let (trace, x_label, y_label) = build_heatmap(args)?;
            plot.add_trace(trace);
            (x_label, y_label)
        },
        Chart::Contour => {
            let (trace, x_label, y_label) = build_contour(args)?;
            plot.add_trace(trace);
            (Some(x_label), Some(y_label))
        },
        kind @ (Chart::Candlestick | Chart::Ohlc) => {
            let (trace, x_label, y_label) = build_candlestick(args, matches!(kind, Chart::Ohlc))?;
            plot.add_trace(trace);
            (Some(x_label), Some(y_label))
        },
        // bar / line / scatter all consume (x, y) pairs, optionally split by --series
        kind => {
            let (traces, x_label, y_label) =
                if matches!(kind, Chart::Scatter) && encoded_scatter(args) {
                    build_scatter_encoded(args)?
                } else {
                    build_xy_traces(args, kind)?
                };
            for trace in traces {
                plot.add_trace(trace);
            }
            (Some(x_label), Some(y_label))
        },
    };

    let mut layout = build_layout(args, default_x, default_y);
    // `x unified` (plotly.js) shows one tooltip per x across every trace — ideal for an ordered
    // x-axis (line trends, OHLC sessions) but wrong for unordered clouds, so it's scoped to those
    // chart kinds. The smart dashboard builds its own layout and never calls build_layout, so this
    // can't leak onto its heterogeneous panels.
    if matches!(
        chart_kind(args),
        Chart::Line | Chart::Candlestick | Chart::Ohlc
    ) {
        layout = layout.hover_mode(HoverMode::XUnified);
    }
    plot.set_layout(layout);
    Ok(plot)
}

/// The requested chart subcommand.
#[derive(Clone, Copy)]
enum Chart {
    Bar,
    Line,
    Scatter,
    Scatter3D,
    Histogram,
    Box,
    Pie,
    Heatmap,
    Contour,
    Candlestick,
    Ohlc,
    Sankey,
    Radar,
    Map,
    Geo,
    Choropleth,
    Treemap,
    Sunburst,
}

fn chart_kind(args: &Args) -> Chart {
    if args.cmd_bar {
        Chart::Bar
    } else if args.cmd_line {
        Chart::Line
    } else if args.cmd_scatter {
        Chart::Scatter
    } else if args.cmd_scatter3d {
        Chart::Scatter3D
    } else if args.cmd_histogram {
        Chart::Histogram
    } else if args.cmd_box {
        Chart::Box
    } else if args.cmd_pie {
        Chart::Pie
    } else if args.cmd_heatmap {
        Chart::Heatmap
    } else if args.cmd_contour {
        Chart::Contour
    } else if args.cmd_candlestick {
        Chart::Candlestick
    } else if args.cmd_ohlc {
        Chart::Ohlc
    } else if args.cmd_sankey {
        Chart::Sankey
    } else if args.cmd_radar {
        Chart::Radar
    } else if args.cmd_map {
        Chart::Map
    } else if args.cmd_geo {
        Chart::Geo
    } else if args.cmd_choropleth {
        Chart::Choropleth
    } else if args.cmd_treemap {
        Chart::Treemap
    } else if args.cmd_sunburst {
        Chart::Sunburst
    } else {
        unreachable!("docopt guarantees exactly one chart subcommand")
    }
}

/// A reader bundle: the configured reader, the resolved header record, and the *effective*
/// no-headers flag (which also honors the `QSV_NO_HEADERS` / `QSV_TOGGLE_HEADERS` env vars,
/// so selection and labeling stay consistent with how the reader treats the first row).
fn reader_and_headers(
    args: &Args,
) -> CliResult<(
    csv::Reader<Box<dyn std::io::Read + Send>>,
    csv::ByteRecord,
    bool,
)> {
    let rconfig = Config::new(args.arg_input.as_ref())
        .delimiter(args.flag_delimiter)
        .no_headers_flag(args.flag_no_headers);
    let no_headers = rconfig.no_headers;
    let mut rdr = rconfig.reader()?;
    let headers = rdr.byte_headers()?.clone();
    Ok((rdr, headers, no_headers))
}

/// Resolve a required single-column selector to its column index.
fn resolve_one(
    sel: Option<&SelectColumns>,
    headers: &csv::ByteRecord,
    no_headers: bool,
    flag: &str,
) -> CliResult<usize> {
    let Some(sel) = sel else {
        return fail_incorrectusage_clierror!("--{flag} is required for this chart type.");
    };
    let selection = sel
        .selection(headers, !no_headers)
        .map_err(crate::CliError::Other)?;
    if selection.len() != 1 {
        return fail_incorrectusage_clierror!(
            "--{flag} must select exactly one column (it selected {}).",
            selection.len()
        );
    }
    Ok(selection[0])
}

/// The label to use for a column index (its header name, or 1-based index when --no-headers).
fn col_label(headers: &csv::ByteRecord, idx: usize, no_headers: bool) -> String {
    if no_headers {
        format!("col {}", idx + 1)
    } else {
        String::from_utf8_lossy(&headers[idx]).into_owned()
    }
}

/// Read (series, x, y) rows. `y` is parsed as f64; rows whose y is empty or non-numeric
/// are skipped. Returns the rows grouped into series traces (one unnamed group when
/// `series_idx` is None), preserving column labels for axis titles.
struct XyData {
    x_label: String,
    y_label: String,
    /// (series name, xs, ys) — series name is empty for the no-series case.
    groups:  Vec<(String, Vec<String>, Vec<f64>)>,
}

fn read_xy(args: &Args) -> CliResult<XyData> {
    let (mut rdr, headers, nh) = reader_and_headers(args)?;
    let x_idx = resolve_one(args.flag_x.as_ref(), &headers, nh, "x")?;
    let y_idx = resolve_one(args.flag_y.as_ref(), &headers, nh, "y")?;
    let series_idx = match args.flag_series.as_ref() {
        Some(s) => Some(resolve_one(Some(s), &headers, nh, "series")?),
        None => None,
    };

    // preserve series insertion order
    let mut order: Vec<String> = Vec::new();
    let mut grouped: BTreeMap<String, (Vec<String>, Vec<f64>)> = BTreeMap::new();

    let mut record = csv::ByteRecord::new();
    while rdr.read_byte_record(&mut record)? {
        let Some(y) = parse_f64(record.get(y_idx)) else {
            continue;
        };
        let x = cell_to_string(record.get(x_idx));
        let series = match series_idx {
            Some(i) => cell_to_string(record.get(i)),
            None => String::new(),
        };
        let entry = grouped.entry(series.clone()).or_insert_with(|| {
            order.push(series);
            (Vec::new(), Vec::new())
        });
        entry.0.push(x);
        entry.1.push(y);
    }

    let groups = order
        .into_iter()
        .map(|name| {
            let (xs, ys) = grouped.remove(&name).unwrap_or_default();
            (name, xs, ys)
        })
        .collect();

    Ok(XyData {
        x_label: col_label(&headers, x_idx, nh),
        y_label: col_label(&headers, y_idx, nh),
        groups,
    })
}

fn build_xy_traces(args: &Args, kind: Chart) -> CliResult<(Vec<Box<dyn Trace>>, String, String)> {
    let data = read_xy(args)?;
    let agg = parse_agg(args.flag_agg.as_deref())?;

    // value labels only make sense for a single-series bar chart with a modest bar count
    let single_series = data.groups.len() == 1;

    let mut traces: Vec<Box<dyn Trace>> = Vec::with_capacity(data.groups.len());
    for (name, xs, ys) in data.groups {
        let (xs, ys) = match agg {
            Some(agg) if matches!(kind, Chart::Bar | Chart::Line) => aggregate(xs, ys, agg),
            _ => (xs, ys),
        };
        // line charts connect points in order, so sort numeric x ascending
        let (xs, ys) = if matches!(kind, Chart::Line) {
            sort_line_xy(xs, ys)
        } else {
            (xs, ys)
        };
        match kind {
            Chart::Bar => {
                let show_labels = single_series && ys.len() <= LABEL_MAX_BARS;
                let mut t = Bar::new(xs, ys);
                if !name.is_empty() {
                    t = t.name(name);
                }
                if show_labels {
                    // SI-formatted value labels above each bar ("258k", "1.05M");
                    // clip_on_axis(false) keeps the tallest bar's label from being clipped
                    t = t
                        .text_template("%{y:.3s}")
                        .text_position(TextPosition::Outside)
                        .text_font(Font::new().size(10))
                        .clip_on_axis(false);
                }
                traces.push(t);
            },
            _ => {
                let mode = if matches!(kind, Chart::Line) {
                    Mode::Lines
                } else {
                    Mode::Markers
                };
                traces.push(scatter_trace(&name, xs, ys, mode));
            },
        }
    }
    if traces.is_empty() {
        return fail_clierror!("No plottable rows found (is --y numeric?).");
    }
    Ok((traces, data.x_label, data.y_label))
}

/// Build a Scatter trace, using a numeric x-axis when every x value parses as a number,
/// otherwise a categorical (string) x-axis.
fn scatter_trace(name: &str, xs: Vec<String>, ys: Vec<f64>, mode: Mode) -> Box<dyn Trace> {
    if !xs.is_empty() && xs.iter().all(|s| s.trim().parse::<f64>().is_ok()) {
        let xn: Vec<f64> = xs
            .iter()
            .map(|s| s.trim().parse::<f64>().unwrap_or(f64::NAN))
            .collect();
        let mut t = Scatter::new(xn, ys).mode(mode);
        if !name.is_empty() {
            t = t.name(name);
        }
        t
    } else {
        let mut t = Scatter::new(xs, ys).mode(mode);
        if !name.is_empty() {
            t = t.name(name);
        }
        t
    }
}

/// True when the user requested a per-point marker encoding (--color and/or --size).
fn encoded_scatter(args: &Args) -> bool {
    args.flag_color.is_some() || args.flag_size.is_some()
}

/// Build a single scatter trace that encodes numeric columns onto marker color (continuous
/// colorscale + colorbar) and/or marker size (bubble chart). Rows missing a numeric value
/// for y or any requested encoding are skipped.
fn build_scatter_encoded(args: &Args) -> CliResult<(Vec<Box<dyn Trace>>, String, String)> {
    let (mut rdr, headers, nh) = reader_and_headers(args)?;
    let x_idx = resolve_one(args.flag_x.as_ref(), &headers, nh, "x")?;
    let y_idx = resolve_one(args.flag_y.as_ref(), &headers, nh, "y")?;
    let color_idx = match args.flag_color.as_ref() {
        Some(s) => Some(resolve_one(Some(s), &headers, nh, "color")?),
        None => None,
    };
    let size_idx = match args.flag_size.as_ref() {
        Some(s) => Some(resolve_one(Some(s), &headers, nh, "size")?),
        None => None,
    };

    let mut xs: Vec<String> = Vec::new();
    let mut ys: Vec<f64> = Vec::new();
    let mut colors: Vec<f64> = Vec::new();
    let mut sizes: Vec<f64> = Vec::new();
    let mut record = csv::ByteRecord::new();
    while rdr.read_byte_record(&mut record)? {
        let Some(y) = parse_f64(record.get(y_idx)) else {
            continue;
        };
        // a point is only plottable if every requested encoding has a numeric value
        let color = match color_idx {
            Some(i) => match parse_f64(record.get(i)) {
                Some(v) => Some(v),
                None => continue,
            },
            None => None,
        };
        let size = match size_idx {
            Some(i) => match parse_f64(record.get(i)) {
                Some(v) => Some(v),
                None => continue,
            },
            None => None,
        };
        xs.push(cell_to_string(record.get(x_idx)));
        ys.push(y);
        if let Some(v) = color {
            colors.push(v);
        }
        if let Some(v) = size {
            sizes.push(v);
        }
    }
    if ys.is_empty() {
        return fail_clierror!(
            "No plottable rows found (are --y and the --color/--size columns numeric?)."
        );
    }

    let mut marker = Marker::new();
    if !sizes.is_empty() {
        marker = marker.size_array(scale_bubble_sizes(&sizes));
    }
    if !colors.is_empty() {
        let color_label = col_label(&headers, color_idx.unwrap(), nh);
        marker = marker
            .color_array(colors)
            .color_scale(ColorScale::Palette(ColorScalePalette::Viridis))
            .show_scale(true)
            .color_bar(ColorBar::new().title(color_label));
    }

    let trace = scatter_with_marker(xs, ys, marker);
    Ok((
        vec![trace],
        col_label(&headers, x_idx, nh),
        col_label(&headers, y_idx, nh),
    ))
}

/// Build a markers-mode Scatter with the given marker, using a numeric x-axis when every x
/// value parses as a number (otherwise a categorical x-axis), mirroring `scatter_trace`.
fn scatter_with_marker(xs: Vec<String>, ys: Vec<f64>, marker: Marker) -> Box<dyn Trace> {
    if !xs.is_empty() && xs.iter().all(|s| s.trim().parse::<f64>().is_ok()) {
        let xn: Vec<f64> = xs
            .iter()
            .map(|s| s.trim().parse::<f64>().unwrap_or(f64::NAN))
            .collect();
        Scatter::new(xn, ys).mode(Mode::Markers).marker(marker)
    } else {
        Scatter::new(xs, ys).mode(Mode::Markers).marker(marker)
    }
}

/// Linearly rescale raw `--size` values into the [`BUBBLE_MIN_PX`, `BUBBLE_MAX_PX`] pixel
/// range so bubbles stay readable regardless of the column's magnitude. When all values are
/// equal, every bubble gets the midpoint size.
fn scale_bubble_sizes(values: &[f64]) -> Vec<usize> {
    let (min, max) = values
        .iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), &v| {
            (lo.min(v), hi.max(v))
        });
    let span = max - min;
    values
        .iter()
        .map(|&v| {
            let t = if span > 0.0 { (v - min) / span } else { 0.5 };
            (BUBBLE_MIN_PX + t * (BUBBLE_MAX_PX - BUBBLE_MIN_PX)).round() as usize
        })
        .collect()
}

/// Radius (pixels) of each point's influence in a `--density` heatmap. A moderate default that
/// reads as a smooth surface for city-to-continent scales without over-blurring dense clusters.
const MAP_DENSITY_RADIUS_PX: u8 = 20;

/// Density radius for the `viz smart` auto map panel. Smaller than the standalone `viz map`
/// default because the dashboard panel is small: a large radius saturates the whole built-up area
/// into one flat blob, hiding the internal hotspots a smaller radius reveals.
const MAP_SMART_DENSITY_RADIUS_PX: u8 = 8;

/// Resolve a `--style` name to its plotly `MapboxStyle` plus whether it is a Mapbox-hosted style
/// (which needs an access token). The token-free styles render from public OSM/Carto/Stamen tile
/// servers; the rest are served by Mapbox and require `--mapbox-token`.
fn parse_map_style(name: &str) -> CliResult<(MapboxStyle, bool)> {
    let resolved = match name.to_ascii_lowercase().as_str() {
        "open-street-map" | "osm" => (MapboxStyle::OpenStreetMap, false),
        "carto-positron" => (MapboxStyle::CartoPositron, false),
        "carto-darkmatter" => (MapboxStyle::CartoDarkMatter, false),
        "stamen-terrain" => (MapboxStyle::StamenTerrain, false),
        "stamen-toner" => (MapboxStyle::StamenToner, false),
        "stamen-watercolor" => (MapboxStyle::StamenWatercolor, false),
        "white-bg" => (MapboxStyle::WhiteBg, false),
        "basic" => (MapboxStyle::Basic, true),
        "streets" => (MapboxStyle::Streets, true),
        "outdoors" => (MapboxStyle::Outdoors, true),
        "light" => (MapboxStyle::Light, true),
        "dark" => (MapboxStyle::Dark, true),
        "satellite" => (MapboxStyle::Satellite, true),
        "satellite-streets" => (MapboxStyle::SatelliteStreets, true),
        other => {
            return fail_incorrectusage_clierror!(
                "Unknown --style '{other}'. Token-free styles: open-street-map, carto-positron, \
                 carto-darkmatter, stamen-terrain, stamen-toner, stamen-watercolor, white-bg. \
                 Mapbox-hosted (need --mapbox-token): basic, streets, outdoors, light, dark, \
                 satellite, satellite-streets."
            );
        },
    };
    Ok(resolved)
}

/// Value at quantile `q` (0.0..=1.0) of an already-sorted slice, via nearest-rank. Empty → 0.0.
fn sorted_quantile(sorted: &[f64], q: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    let idx = (((sorted.len() - 1) as f64) * q).round() as usize;
    sorted[idx.min(sorted.len() - 1)]
}

/// Uniformly downsample two row-aligned vectors to at most `cap` elements via evenly spaced stride
/// sampling, preserving the overall distribution/shape. Returns clones unchanged when already
/// within `cap` (or when `cap == 0`).
fn downsample_pair<X: Clone, Y: Clone>(xs: &[X], ys: &[Y], cap: usize) -> (Vec<X>, Vec<Y>) {
    let n = xs.len();
    if cap == 0 || n <= cap {
        return (xs.to_vec(), ys.to_vec());
    }
    let mut out_x = Vec::with_capacity(cap);
    let mut out_y = Vec::with_capacity(cap);
    for i in 0..cap {
        // endpoint-inclusive: first sample is index 0, last is index n-1, so a chronologically
        // sorted series (e.g. a time-series panel) keeps both its earliest and latest observation
        let idx = if cap == 1 { 0 } else { i * (n - 1) / (cap - 1) };
        out_x.push(xs[idx].clone());
        out_y.push(ys[idx].clone());
    }
    (out_x, out_y)
}

/// Largest-Triangle-Three-Buckets downsampling. Returns the indices (into `xs`/`ys`) of at most
/// `cap` points that best preserve the visual shape of a `y = f(x)` series. Unlike uniform stride
/// sampling (`downsample_pair`), this selects by triangle area in (x, y) space, so spikes/peaks
/// between strides survive instead of being stepped over. Requires `xs` sorted ascending
/// (monotonic) and finite `ys`. First and last points are always retained.
fn lttb_indices(xs: &[f64], ys: &[f64], cap: usize) -> Vec<usize> {
    let n = xs.len();
    if cap == 0 || n <= cap {
        return (0..n).collect();
    }
    if cap < 3 {
        return if cap == 1 { vec![0] } else { vec![0, n - 1] };
    }
    let mut out = Vec::with_capacity(cap);
    out.push(0); // always keep the first point
    let every = (n - 2) as f64 / (cap - 2) as f64; // bucket width over the interior points
    let mut a = 0usize; // last selected point (the triangle's first vertex)
    for i in 0..cap - 2 {
        // average of the *next* bucket is the triangle's third vertex
        let avg_start = ((i + 1) as f64 * every) as usize + 1;
        let avg_end = (((i + 2) as f64 * every) as usize + 1).min(n);
        let avg_len = (avg_end - avg_start).max(1) as f64;
        let (mut avg_x, mut avg_y) = (0.0_f64, 0.0_f64);
        for j in avg_start..avg_end {
            avg_x += xs[j];
            avg_y += ys[j];
        }
        avg_x /= avg_len;
        avg_y /= avg_len;
        // pick the point in the current bucket forming the largest triangle with (a, next-avg)
        let range_start = (i as f64 * every) as usize + 1;
        let range_end = (((i + 1) as f64 * every) as usize + 1).min(n);
        let (ax, ay) = (xs[a], ys[a]);
        let (mut max_area, mut chosen) = (-1.0_f64, range_start);
        for j in range_start..range_end {
            let area = ((ax - avg_x) * (ys[j] - ay) - (ax - xs[j]) * (avg_y - ay)).abs();
            if area > max_area {
                max_area = area;
                chosen = j;
            }
        }
        out.push(chosen);
        a = chosen;
    }
    out.push(n - 1); // always keep the last point
    out
}

/// Web-Mercator normalized Y for a latitude (0 at +85.05°N, 1 at 85.05°S) — the vertical analogue
/// of `lon / 360`. Latitude is clamped to the Web-Mercator limit (±85.05°) so `tan`/`ln` can't blow
/// up. Used by `fitbounds_zoom` to size a latitude span in the same projected units mapbox zoom is
/// defined in.
fn mercator_y(lat: f64) -> f64 {
    const MERCATOR_MAX_LAT: f64 = 85.05;
    let lat_rad = lat.clamp(-MERCATOR_MAX_LAT, MERCATOR_MAX_LAT).to_radians();
    (1.0 - (std::f64::consts::FRAC_PI_4 + lat_rad / 2.0).tan().ln() / std::f64::consts::PI) / 2.0
}

/// Aspect-aware Web-Mercator `fitBounds` zoom: fits the longitude and latitude spans SEPARATELY
/// against the panel's pixel width/height and takes the tighter (smaller) zoom, so a wide-short map
/// panel frames a square core box by its (binding) height and a wide full-extent box by its width —
/// instead of the naive `floor(log2(360 / max(latSpan, lonSpan)))` that compared the larger span to
/// the 360°-wide world and ignored the panel aspect (over-zooming square boxes, under-zooming wide
/// ones).
///   zoom_lon = log2( (width_px  / 512) / (lon_span / 360) )
///   zoom_lat = log2( (height_px / 512) / (mercator_y(min_lat) - mercator_y(max_lat)) )
///   zoom     = floor( min(zoom_lon, zoom_lat) - log2(MAP_FIT_PAD) ).clamp(1, 16)
/// A degenerate (single-point) box returns a street-level zoom of 10. Callers never pass an
/// antimeridian-crossing extent, so `lon_span` is non-wrapping and < 180.
fn fitbounds_zoom(min_lat: f64, max_lat: f64, lon_span: f64, width_px: f64, height_px: f64) -> u8 {
    let lon_frac = lon_span / 360.0;
    let lat_frac = mercator_y(min_lat) - mercator_y(max_lat);
    if lon_frac <= 0.0 && lat_frac <= 0.0 {
        // degenerate (single point) box: sensible street-level default
        return 10;
    }
    let zoom_lon = if lon_frac > 0.0 {
        ((width_px / MAPBOX_TILE_SIZE_PX) / lon_frac).log2()
    } else {
        f64::INFINITY
    };
    let zoom_lat = if lat_frac > 0.0 {
        ((height_px / MAPBOX_TILE_SIZE_PX) / lat_frac).log2()
    } else {
        f64::INFINITY
    };
    (zoom_lon.min(zoom_lat) - MAP_FIT_PAD.log2())
        .floor()
        .clamp(1.0, 16.0) as u8
}

/// Compute a map center and a zoom level that frames the data, so the basemap doesn't default to
/// plotly's whole-world view centered at (0, 0). Longitude is handled with antimeridian wrap so a
/// cluster straddling the 180° line (e.g. 179 and -179) frames as the small arc across the date
/// line rather than spanning almost the whole globe and centering near 0.
///
/// `trim_frac` is the fraction trimmed off each end of the lat/lon distributions before framing,
/// so a few outlier coordinates can't dominate the center/zoom. `viz smart`'s auto panel passes
/// `MAP_FRAME_TRIM_FRAC`; the standalone `viz map` command passes `0.0` to frame the full extent
/// of every valid coordinate (its edge points are intentional, not noise).
///
/// `width_px`/`height_px` are the panel's pixel dimensions, used by the aspect-aware
/// `fitbounds_zoom`: the smart inline panel is wide and short (`MAP_PANEL_ASSUMED_WIDTH_PX` ×
/// `MAP_PANEL_USABLE_HEIGHT_PX`), the standalone `viz map` export is `DEFAULT_IMG_WIDTH` ×
/// `DEFAULT_IMG_HEIGHT`.
fn map_center_zoom(
    lats: &[f64],
    lons: &[f64],
    trim_frac: f64,
    width_px: f64,
    height_px: f64,
) -> (Center, u8) {
    let mut sorted_lats = lats.to_vec();
    sorted_lats.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let min_lat = sorted_quantile(&sorted_lats, trim_frac);
    let max_lat = sorted_quantile(&sorted_lats, 1.0 - trim_frac);
    let lat_center = (min_lat + max_lat) / 2.0;

    let (lon_center, lon_span) = lon_center_and_span(lons, trim_frac);
    let center = Center::new(lat_center, lon_center);

    let zoom = fitbounds_zoom(min_lat, max_lat, lon_span, width_px, height_px);
    (center, zoom)
}

/// Antimeridian-aware longitude center + span. The data occupies all of the 360° circle except
/// its single largest empty gap; the cluster is the complementary arc, so the span is
/// `360 - largest_gap` and the center is that arc's midpoint, normalized to [-180, 180]. When the
/// largest gap is the wrap between max and min (the common, non-crossing case), this reduces to a
/// `trim_frac`-trimmed midpoint and span (the same robust framing the latitude axis uses; pass
/// `0.0` for the plain full-extent `(min+max)/2` midpoint and `max-min` span).
fn lon_center_and_span(lons: &[f64], trim_frac: f64) -> (f64, f64) {
    let mut sorted: Vec<f64> = lons.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let n = sorted.len();
    // largest gap between adjacent longitudes (the empty wedge the data does NOT cover)
    let mut max_gap = 0.0_f64;
    let mut gap_idx = 0; // gap lies between sorted[gap_idx] and sorted[gap_idx + 1]
    for i in 0..n.saturating_sub(1) {
        let gap = sorted[i + 1] - sorted[i];
        if gap > max_gap {
            max_gap = gap;
            gap_idx = i;
        }
    }
    // the wrap gap closes the circle from the easternmost point back to the westernmost
    let wrap_gap = (sorted[0] + 360.0) - sorted[n - 1];
    if wrap_gap >= max_gap {
        // data doesn't cross the antimeridian: trimmed (or full-extent when trim_frac == 0)
        // bounding-box midpoint and span
        let lo = sorted_quantile(&sorted, trim_frac);
        let hi = sorted_quantile(&sorted, 1.0 - trim_frac);
        ((lo + hi) / 2.0, hi - lo)
    } else {
        // data crosses the antimeridian: unwrap the cluster into a contiguous ascending longitude
        // range — the arc east of the gap (sorted[gap_idx + 1..]), then the points west of it
        // shifted +360 — and apply the same quantile trimming as the non-crossing branch, so a lone
        // far in-range outlier can't inflate the span. The cluster runs from sorted[gap_idx + 1]
        // eastward, wrapping past 180, to sorted[gap_idx] (+360 keeps the arc contiguous). With
        // trim_frac == 0 this reduces exactly to the full `360 - max_gap` span and the arc-midpoint
        // center.
        let mut unwrapped: Vec<f64> = Vec::with_capacity(n);
        unwrapped.extend_from_slice(&sorted[gap_idx + 1..]);
        unwrapped.extend(sorted[..=gap_idx].iter().map(|v| v + 360.0));
        let lo = sorted_quantile(&unwrapped, trim_frac);
        let hi = sorted_quantile(&unwrapped, 1.0 - trim_frac);
        let mut center = (lo + hi) / 2.0;
        if center > 180.0 {
            center -= 360.0;
        }
        (center, hi - lo)
    }
}

/// Build a markers-mode `ScatterMapbox` point trace with the given marker (and optional per-point
/// hover text), mirroring `scatter_with_marker` for the cartesian scatter path.
fn scatter_mapbox_with_marker(
    lats: Vec<f64>,
    lons: Vec<f64>,
    marker: Marker,
    text: Option<Vec<String>>,
) -> Box<dyn Trace> {
    let mut t = ScatterMapbox::new(lats, lons)
        .mode(Mode::Markers)
        .marker(marker);
    if let Some(text) = text {
        t = t.text_array(text);
    }
    t
}

/// Split row-aligned coordinates into one `ScatterMapbox` trace per `--series` category,
/// preserving first-seen category order. `texts` is applied as per-point hover text when present.
fn map_series_traces(
    lats: Vec<f64>,
    lons: Vec<f64>,
    series: Vec<String>,
    texts: Vec<String>,
) -> Vec<Box<dyn Trace>> {
    let has_text = texts.len() == lats.len();
    let mut order: Vec<String> = Vec::new();
    let mut groups: BTreeMap<String, (Vec<f64>, Vec<f64>, Vec<String>)> = BTreeMap::new();
    for i in 0..lats.len() {
        let name = series[i].clone();
        let entry = groups.entry(name.clone()).or_insert_with(|| {
            order.push(name);
            (Vec::new(), Vec::new(), Vec::new())
        });
        entry.0.push(lats[i]);
        entry.1.push(lons[i]);
        if has_text {
            entry.2.push(texts[i].clone());
        }
    }
    order
        .into_iter()
        .map(|name| {
            let (la, lo, tx) = groups.remove(&name).unwrap_or_default();
            let mut t = ScatterMapbox::new(la, lo).mode(Mode::Markers).name(name);
            if !tx.is_empty() {
                t = t.text_array(tx);
            }
            let trace: Box<dyn Trace> = t;
            trace
        })
        .collect()
}

/// Pixel dimensions to frame a map / choropleth basemap against (fed to `map_center_zoom`'s
/// aspect-aware fit). `--width`/`--height` size the static image export but are NOT applied to the
/// responsive HTML layout, so honor them only when exporting an image — so a narrow/tall or
/// wide/short export frames its own aspect instead of cropping the extent — while HTML frames for
/// the representative default aspect, matching how `run` sizes each output.
fn fit_dims(
    flag_width: Option<usize>,
    flag_height: Option<usize>,
    out_format: OutFormat,
) -> (usize, usize) {
    if out_format.is_image() {
        (
            flag_width.unwrap_or(DEFAULT_IMG_WIDTH),
            flag_height.unwrap_or(DEFAULT_IMG_HEIGHT),
        )
    } else {
        (DEFAULT_IMG_WIDTH, DEFAULT_IMG_HEIGHT)
    }
}

/// Build the complete `Plot` for `viz map`: a `ScatterMapbox` point map (optionally with
/// `--color`/`--size` marker encodings or `--series` per-category traces) or a `--density`
/// `DensityMapbox` heatmap, on a tile basemap framed to the data's bounding box.
fn build_map_plot(args: &Args, out_format: OutFormat) -> CliResult<Plot> {
    let style_name = args.flag_style.as_deref().unwrap_or("open-street-map");
    let (style, needs_token) = parse_map_style(style_name)?;
    if needs_token && args.flag_mapbox_token.is_none() {
        return fail_incorrectusage_clierror!(
            "--style '{style_name}' is a Mapbox-hosted style that requires --mapbox-token. Use a \
             token-free style (e.g. open-street-map, carto-positron) or pass --mapbox-token."
        );
    }

    let (mut rdr, headers, nh) = reader_and_headers(args)?;
    let lat_idx = resolve_one(args.flag_lat.as_ref(), &headers, nh, "lat")?;
    let lon_idx = resolve_one(args.flag_lon.as_ref(), &headers, nh, "lon")?;
    let color_idx = match args.flag_color.as_ref() {
        Some(s) => Some(resolve_one(Some(s), &headers, nh, "color")?),
        None => None,
    };
    let size_idx = match args.flag_size.as_ref() {
        Some(s) => Some(resolve_one(Some(s), &headers, nh, "size")?),
        None => None,
    };
    let series_idx = match args.flag_series.as_ref() {
        Some(s) => Some(resolve_one(Some(s), &headers, nh, "series")?),
        None => None,
    };
    let text_idx = match args.flag_text.as_ref() {
        Some(s) => Some(resolve_one(Some(s), &headers, nh, "text")?),
        None => None,
    };

    if args.flag_density && series_idx.is_some() {
        return fail_incorrectusage_clierror!(
            "--density renders a single heatmap layer and cannot be combined with --series."
        );
    }

    // accumulate row-aligned columns. A row is kept only when lat/lon are valid coordinates AND
    // every requested encoding column has a numeric value, so all per-point arrays stay aligned.
    let mut lats: Vec<f64> = Vec::new();
    let mut lons: Vec<f64> = Vec::new();
    let mut colors: Vec<f64> = Vec::new();
    let mut sizes: Vec<f64> = Vec::new();
    let mut series: Vec<String> = Vec::new();
    let mut texts: Vec<String> = Vec::new();
    let mut record = csv::ByteRecord::new();
    while rdr.read_byte_record(&mut record)? {
        let (Some(lat), Some(lon)) = (
            parse_f64(record.get(lat_idx)),
            parse_f64(record.get(lon_idx)),
        ) else {
            continue;
        };
        if !(-90.0..=90.0).contains(&lat) || !(-180.0..=180.0).contains(&lon) {
            continue;
        }
        let color = match color_idx {
            Some(i) => match parse_f64(record.get(i)) {
                Some(v) => Some(v),
                None => continue,
            },
            None => None,
        };
        let size = match size_idx {
            Some(i) => match parse_f64(record.get(i)) {
                Some(v) => Some(v),
                None => continue,
            },
            None => None,
        };
        lats.push(lat);
        lons.push(lon);
        if let Some(v) = color {
            colors.push(v);
        }
        if let Some(v) = size {
            sizes.push(v);
        }
        if let Some(i) = series_idx {
            series.push(cell_to_string(record.get(i)));
        }
        if let Some(i) = text_idx {
            texts.push(cell_to_string(record.get(i)));
        }
    }
    if lats.is_empty() {
        return fail_clierror!(
            "No mappable rows found (are --lat/--lon numeric and within valid coordinate ranges?)."
        );
    }

    // standalone `viz map`: frame the full extent — its edge coordinates are intentional.
    let (fit_w, fit_h) = fit_dims(args.flag_width, args.flag_height, out_format);
    let (center, zoom) = map_center_zoom(&lats, &lons, 0.0, fit_w as f64, fit_h as f64);

    let mut plot = Plot::new();
    if args.flag_density {
        // density weight: the --color or --size column when given, else a uniform 1.0
        let z = if !colors.is_empty() {
            colors
        } else if !sizes.is_empty() {
            sizes
        } else {
            vec![1.0_f64; lats.len()]
        };
        plot.add_trace(DensityMapbox::new(lats, lons, z).radius(MAP_DENSITY_RADIUS_PX));
    } else if series_idx.is_some() {
        for trace in map_series_traces(lats, lons, series, texts) {
            plot.add_trace(trace);
        }
    } else {
        let mut marker = Marker::new();
        if !sizes.is_empty() {
            marker = marker.size_array(scale_bubble_sizes(&sizes));
        }
        if !colors.is_empty() {
            let color_label = col_label(&headers, color_idx.unwrap(), nh);
            marker = marker
                .color_array(colors)
                .color_scale(ColorScale::Palette(ColorScalePalette::Viridis))
                .show_scale(true)
                .color_bar(ColorBar::new().title(color_label));
        }
        let text = (!texts.is_empty()).then_some(texts);
        plot.add_trace(scatter_mapbox_with_marker(lats, lons, marker, text));
    }

    let mut mapbox = Mapbox::new().style(style).center(center).zoom(zoom);
    // only embed the token when the resolved style actually needs it — otherwise it would leak
    // into stdout / saved HTML for token-free styles (e.g. the default open-street-map)
    if needs_token && let Some(token) = args.flag_mapbox_token.clone() {
        mapbox = mapbox.access_token(token);
    }
    let mut layout = Layout::new()
        .mapbox(mapbox)
        .show_legend(series_idx.is_some());
    if let Some(title) = &args.flag_title {
        layout = layout.title(Title::with_text(title));
    }
    plot.set_layout(apply_theme(layout, args.theme()));
    Ok(plot)
}

/// Parse a `--projection` name into a plotly geo `ProjectionType`. A curated, token-free subset
/// of plotly's projections; names are case-insensitive and accept spaces or underscores.
fn parse_projection(name: &str) -> CliResult<ProjectionType> {
    let normalized = name.to_ascii_lowercase().replace([' ', '_'], "-");
    let projection = match normalized.as_str() {
        "natural-earth" => ProjectionType::NaturalEarth,
        "mercator" => ProjectionType::Mercator,
        "orthographic" => ProjectionType::Orthographic,
        "equirectangular" => ProjectionType::Equirectangular,
        "albers-usa" => ProjectionType::AlbersUsa,
        "robinson" => ProjectionType::Robinson,
        "winkel-tripel" => ProjectionType::WinkelTripel,
        "mollweide" => ProjectionType::Mollweide,
        "hammer" => ProjectionType::Hammer,
        "azimuthal-equal-area" => ProjectionType::AzimuthalEqualArea,
        _ => {
            return fail_incorrectusage_clierror!(
                "Unknown --projection '{name}'. One of: natural-earth, mercator, orthographic, \
                 equirectangular, albers-usa, robinson, winkel-tripel, mollweide, hammer, \
                 azimuthal-equal-area."
            );
        },
    };
    Ok(projection)
}

/// Build a markers-mode `ScatterGeo` point trace with the given marker (and optional per-point
/// hover text), mirroring `scatter_mapbox_with_marker` for the projection-basemap path.
fn scatter_geo_with_marker(
    lats: Vec<f64>,
    lons: Vec<f64>,
    marker: Marker,
    text: Option<Vec<String>>,
) -> Box<dyn Trace> {
    let mut t = ScatterGeo::new(lats, lons)
        .mode(Mode::Markers)
        .marker(marker);
    if let Some(text) = text {
        t = t.text_array(text);
    }
    t
}

/// Split row-aligned coordinates into one `ScatterGeo` trace per `--series` category, preserving
/// first-seen category order. `texts` is applied as per-point hover text when present.
fn geo_series_traces(
    lats: Vec<f64>,
    lons: Vec<f64>,
    series: Vec<String>,
    texts: Vec<String>,
) -> Vec<Box<dyn Trace>> {
    let has_text = texts.len() == lats.len();
    let mut order: Vec<String> = Vec::new();
    let mut groups: BTreeMap<String, (Vec<f64>, Vec<f64>, Vec<String>)> = BTreeMap::new();
    for i in 0..lats.len() {
        let name = series[i].clone();
        let entry = groups.entry(name.clone()).or_insert_with(|| {
            order.push(name);
            (Vec::new(), Vec::new(), Vec::new())
        });
        entry.0.push(lats[i]);
        entry.1.push(lons[i]);
        if has_text {
            entry.2.push(texts[i].clone());
        }
    }
    order
        .into_iter()
        .map(|name| {
            let (la, lo, tx) = groups.remove(&name).unwrap_or_default();
            let mut t = ScatterGeo::new(la, lo).mode(Mode::Markers).name(name);
            if !tx.is_empty() {
                t = t.text_array(tx);
            }
            let trace: Box<dyn Trace> = t;
            trace
        })
        .collect()
}

/// Build the complete `Plot` for `viz geo`: a `ScatterGeo` point map on a projection basemap
/// (coastlines/land/countries, no tiles or token), with optional `--color`/`--size` marker
/// encodings or `--series` per-category traces. Mirrors `build_map_plot` minus the tile basemap.
fn build_geo_plot(args: &Args) -> CliResult<Plot> {
    let projection = parse_projection(args.flag_projection.as_deref().unwrap_or("natural-earth"))?;

    let (mut rdr, headers, nh) = reader_and_headers(args)?;
    let lat_idx = resolve_one(args.flag_lat.as_ref(), &headers, nh, "lat")?;
    let lon_idx = resolve_one(args.flag_lon.as_ref(), &headers, nh, "lon")?;
    let color_idx = match args.flag_color.as_ref() {
        Some(s) => Some(resolve_one(Some(s), &headers, nh, "color")?),
        None => None,
    };
    let size_idx = match args.flag_size.as_ref() {
        Some(s) => Some(resolve_one(Some(s), &headers, nh, "size")?),
        None => None,
    };
    let series_idx = match args.flag_series.as_ref() {
        Some(s) => Some(resolve_one(Some(s), &headers, nh, "series")?),
        None => None,
    };
    let text_idx = match args.flag_text.as_ref() {
        Some(s) => Some(resolve_one(Some(s), &headers, nh, "text")?),
        None => None,
    };

    // accumulate row-aligned columns; a row is kept only when lat/lon are valid coordinates AND
    // every requested encoding column has a numeric value, so all per-point arrays stay aligned.
    let mut lats: Vec<f64> = Vec::new();
    let mut lons: Vec<f64> = Vec::new();
    let mut colors: Vec<f64> = Vec::new();
    let mut sizes: Vec<f64> = Vec::new();
    let mut series: Vec<String> = Vec::new();
    let mut texts: Vec<String> = Vec::new();
    let mut record = csv::ByteRecord::new();
    while rdr.read_byte_record(&mut record)? {
        let (Some(lat), Some(lon)) = (
            parse_f64(record.get(lat_idx)),
            parse_f64(record.get(lon_idx)),
        ) else {
            continue;
        };
        if !(-90.0..=90.0).contains(&lat) || !(-180.0..=180.0).contains(&lon) {
            continue;
        }
        let color = match color_idx {
            Some(i) => match parse_f64(record.get(i)) {
                Some(v) => Some(v),
                None => continue,
            },
            None => None,
        };
        let size = match size_idx {
            Some(i) => match parse_f64(record.get(i)) {
                Some(v) => Some(v),
                None => continue,
            },
            None => None,
        };
        lats.push(lat);
        lons.push(lon);
        if let Some(v) = color {
            colors.push(v);
        }
        if let Some(v) = size {
            sizes.push(v);
        }
        if let Some(i) = series_idx {
            series.push(cell_to_string(record.get(i)));
        }
        if let Some(i) = text_idx {
            texts.push(cell_to_string(record.get(i)));
        }
    }
    if lats.is_empty() {
        return fail_clierror!(
            "No mappable rows found (are --lat/--lon numeric and within valid coordinate ranges?)."
        );
    }

    let mut plot = Plot::new();
    if series_idx.is_some() {
        for trace in geo_series_traces(lats, lons, series, texts) {
            plot.add_trace(trace);
        }
    } else {
        let mut marker = Marker::new();
        if !sizes.is_empty() {
            marker = marker.size_array(scale_bubble_sizes(&sizes));
        }
        if !colors.is_empty() {
            let color_label = col_label(&headers, color_idx.unwrap(), nh);
            marker = marker
                .color_array(colors)
                .color_scale(ColorScale::Palette(ColorScalePalette::Viridis))
                .show_scale(true)
                .color_bar(ColorBar::new().title(color_label));
        }
        let text = (!texts.is_empty()).then_some(texts);
        plot.add_trace(scatter_geo_with_marker(lats, lons, marker, text));
    }

    let geo = LayoutGeo::new()
        .projection(Projection::new().projection_type(projection))
        .showland(true)
        .landcolor(NamedColor::LightGray)
        .showocean(true)
        .oceancolor(NamedColor::LightBlue)
        .showlakes(true)
        .lakecolor(NamedColor::LightBlue)
        .showcountries(true);
    let mut layout = Layout::new().geo(geo).show_legend(series_idx.is_some());
    if let Some(title) = &args.flag_title {
        layout = layout.title(Title::with_text(title));
    }
    plot.set_layout(apply_theme(layout, args.theme()));
    Ok(plot)
}

/// Map a `--location-mode` name to a plotly [`LocationMode`].
fn parse_location_mode(name: &str) -> CliResult<LocationMode> {
    match name.to_ascii_lowercase().as_str() {
        "iso3" | "iso-3" => Ok(LocationMode::Iso3),
        "usa-states" | "usa_states" | "us-states" => Ok(LocationMode::UsaStates),
        "country-names" | "country_names" | "names" => Ok(LocationMode::CountryNames),
        "geojson-id" | "geojson_id" | "geojson" => Ok(LocationMode::GeoJsonId),
        other => fail_incorrectusage_clierror!(
            "Unknown --location-mode '{other}'. Use iso3, usa-states, country-names, or \
             geojson-id."
        ),
    }
}

/// Map a `--color-scale` name to a plotly [`ColorScalePalette`].
fn parse_color_scale(name: &str) -> CliResult<ColorScalePalette> {
    match name.to_ascii_lowercase().as_str() {
        "viridis" => Ok(ColorScalePalette::Viridis),
        "cividis" => Ok(ColorScalePalette::Cividis),
        "greys" | "grays" => Ok(ColorScalePalette::Greys),
        "greens" => Ok(ColorScalePalette::Greens),
        "blues" => Ok(ColorScalePalette::Blues),
        "reds" => Ok(ColorScalePalette::Reds),
        "ylgnbu" => Ok(ColorScalePalette::YlGnBu),
        "ylorrd" => Ok(ColorScalePalette::YlOrRd),
        "bluered" => Ok(ColorScalePalette::Bluered),
        "rdbu" => Ok(ColorScalePalette::RdBu),
        "portland" => Ok(ColorScalePalette::Portland),
        "electric" => Ok(ColorScalePalette::Electric),
        "jet" => Ok(ColorScalePalette::Jet),
        "hot" => Ok(ColorScalePalette::Hot),
        "blackbody" => Ok(ColorScalePalette::Blackbody),
        "earth" => Ok(ColorScalePalette::Earth),
        "picnic" => Ok(ColorScalePalette::Picnic),
        "rainbow" => Ok(ColorScalePalette::Rainbow),
        other => fail_incorrectusage_clierror!(
            "Unknown --color-scale '{other}'. Use viridis, cividis, greys, greens, blues, reds, \
             ylgnbu, ylorrd, bluered, rdbu, portland, electric, jet, hot, blackbody, earth, \
             picnic, or rainbow."
        ),
    }
}

/// Map a `--style` name to a token-free MapLibre [`MapStyle`] for `viz choropleth --map`.
fn parse_choropleth_map_style(name: &str) -> CliResult<MapStyle> {
    match name.to_ascii_lowercase().as_str() {
        "carto-positron" => Ok(MapStyle::CartoPositron),
        "carto-darkmatter" | "carto-dark-matter" => Ok(MapStyle::CartoDarkMatter),
        "carto-voyager" => Ok(MapStyle::CartoVoyager),
        "open-street-map" | "osm" => Ok(MapStyle::OpenStreetMap),
        "dark" => Ok(MapStyle::Dark),
        "light" => Ok(MapStyle::Light),
        "white-bg" => Ok(MapStyle::WhiteBg),
        "satellite" => Ok(MapStyle::Satellite),
        "basic" => Ok(MapStyle::Basic),
        other => fail_incorrectusage_clierror!(
            "Unknown --style '{other}' for `viz choropleth --map`. Use carto-positron, \
             carto-darkmatter, carto-voyager, open-street-map, dark, light, white-bg, satellite, \
             or basic."
        ),
    }
}

/// Load a GeoJSON FeatureCollection from a local file path or an http(s) URL into a JSON value.
fn load_geojson(spec: &str) -> CliResult<serde_json::Value> {
    let bytes = if spec.starts_with("http://") || spec.starts_with("https://") {
        let resp = reqwest::blocking::get(spec)
            .and_then(reqwest::blocking::Response::error_for_status)
            .map_err(|e| {
                crate::CliError::Other(format!("Failed to fetch --geojson URL '{spec}': {e}"))
            })?;
        resp.bytes()
            .map_err(|e| {
                crate::CliError::Other(format!("Failed to read --geojson body from '{spec}': {e}"))
            })?
            .to_vec()
    } else {
        std::fs::read(spec).map_err(|e| {
            crate::CliError::Other(format!("Failed to read --geojson '{spec}': {e}"))
        })?
    };
    serde_json::from_slice(&bytes)
        .map_err(|e| crate::CliError::Other(format!("--geojson '{spec}' is not valid JSON: {e}")))
}

/// Collect every `[lon, lat]` vertex from a GeoJSON value into parallel `(lats, lons)` vectors, so
/// a `--map` choropleth can frame the MapLibre basemap to its regions instead of opening at
/// plotly's whole-world default (where county/city polygons are effectively invisible). Descends
/// only through GeoJSON's geometry-bearing keys (`features`/`geometry`/`geometries`/`coordinates`)
/// so numeric arrays inside feature `properties` (e.g. a stray `bbox` or data array) can't be
/// mistaken for coordinates. Out-of-range pairs are dropped. GeoJSON positions are `[lon, lat]`.
fn geojson_lat_lons(geojson: &serde_json::Value) -> (Vec<f64>, Vec<f64>) {
    fn walk(v: &serde_json::Value, lats: &mut Vec<f64>, lons: &mut Vec<f64>) {
        match v {
            serde_json::Value::Array(arr) => {
                // a leaf position is `[lon, lat, ...]`: the first two entries are bare numbers
                // (nested coordinate arrays have arrays, not numbers, in those slots).
                if arr.len() >= 2 && arr[0].is_number() && arr[1].is_number() {
                    if let (Some(lon), Some(lat)) = (arr[0].as_f64(), arr[1].as_f64())
                        && (-180.0..=180.0).contains(&lon)
                        && (-90.0..=90.0).contains(&lat)
                    {
                        lons.push(lon);
                        lats.push(lat);
                    }
                } else {
                    for item in arr {
                        walk(item, lats, lons);
                    }
                }
            },
            serde_json::Value::Object(map) => {
                for (k, val) in map {
                    if matches!(
                        k.as_str(),
                        "features" | "geometry" | "geometries" | "coordinates"
                    ) {
                        walk(val, lats, lons);
                    }
                }
            },
            _ => {},
        }
    }
    let mut lats = Vec::new();
    let mut lons = Vec::new();
    walk(geojson, &mut lats, &mut lons);
    (lats, lons)
}

/// Build the complete `Plot` for `viz choropleth`: fill whole geographic regions colored by an
/// aggregated value. Defaults to a token-free `Choropleth` on the projection `geo` subplot; `--map`
/// switches to a MapLibre `ChoroplethMap` (GeoJSON-only). Region keys come from `--locations`, or
/// — with `--geocode` — are derived from `--lat`/`--lon` (reverse) or a `--locations` name column
/// (forward) by reusing qsv's geocode engine.
fn build_choropleth_plot(args: &Args, out_format: OutFormat) -> CliResult<Plot> {
    let mode = parse_location_mode(args.flag_location_mode.as_deref().unwrap_or("iso3"))?;
    let palette = parse_color_scale(args.flag_color_scale.as_deref().unwrap_or("viridis"))?;

    // --value drives the colored measure; aggregation defaults to sum when a --value is given,
    // else per-region row counts. Non-count aggs require a --value column.
    let agg = match (
        parse_agg(args.flag_agg.as_deref())?,
        args.flag_value.is_some(),
    ) {
        (Some(a), _) => a,
        (None, true) => Agg::Sum,
        (None, false) => Agg::Count,
    };
    if agg != Agg::Count && args.flag_value.is_none() {
        return fail_incorrectusage_clierror!("--agg sum/mean/min/max requires a --value column.");
    }

    // --map (ChoroplethMap) is MapLibre + GeoJSON-only; the default geo Choropleth has built-in
    // country/state geometries and needs a GeoJSON only for the geojson-id location mode.
    if args.flag_map && (args.flag_geojson.is_none() || args.flag_feature_id_key.is_none()) {
        return fail_incorrectusage_clierror!(
            "--map (ChoroplethMap) requires both --geojson and --feature-id-key."
        );
    }
    if matches!(mode, LocationMode::GeoJsonId) && args.flag_geojson.is_none() {
        return fail_incorrectusage_clierror!(
            "--location-mode geojson-id requires a --geojson source."
        );
    }
    if args.flag_map && args.flag_geocode {
        return fail_incorrectusage_clierror!(
            "--map cannot be combined with --geocode: geocode yields ISO-3/US-state codes, which \
             won't match a GeoJSON feature id. Use the default geo basemap with --geocode."
        );
    }

    let (locations, z, measure_label) = if args.flag_geocode {
        choropleth_geocoded_locations(args, mode.clone(), agg)?
    } else {
        choropleth_literal_locations(args, agg)?
    };

    if locations.is_empty() {
        return fail_clierror!(
            "No choropleth regions resolved (check --locations / --geocode inputs and \
             --location-mode)."
        );
    }

    let mut plot = Plot::new();
    if args.flag_map {
        // ChoroplethMap on the MapLibre `map` subplot (GeoJSON-only).
        let geojson = load_geojson(args.flag_geojson.as_deref().unwrap())?;
        // collect the GeoJSON extent BEFORE the value is moved into the trace below.
        let (g_lats, g_lons) = geojson_lat_lons(&geojson);
        let trace = ChoroplethMap::new(locations, z)
            .geojson(geojson)
            .feature_id_key(args.flag_feature_id_key.as_deref().unwrap())
            .color_scale(ColorScale::Palette(palette))
            .show_scale(true)
            .color_bar(ColorBar::new().title(measure_label))
            .marker(ChoroplethMarker::new().line(Line::new().width(0.5)));
        plot.add_trace(trace);
        // --style carries a global docopt default of open-street-map (a token-free MapLibre style).
        let style =
            parse_choropleth_map_style(args.flag_style.as_deref().unwrap_or("open-street-map"))?;
        let mut layout_map = LayoutMap::new().style(style);
        // frame the basemap to the GeoJSON extent so local/custom regions (counties, cities) are
        // visible instead of being lost at plotly's default whole-world view centered at (0, 0).
        // Falls back to that default only when the GeoJSON has no usable coordinates. `fit_dims`
        // honors --width/--height for image exports (so a non-default static aspect frames its
        // extent instead of cropping) and the default aspect for HTML — matching `build_map_plot`.
        if !g_lats.is_empty() {
            let (fit_w, fit_h) = fit_dims(args.flag_width, args.flag_height, out_format);
            let (center, zoom) = map_center_zoom(&g_lats, &g_lons, 0.0, fit_w as f64, fit_h as f64);
            layout_map = layout_map.center(center).zoom(f64::from(zoom));
        }
        let mut layout = Layout::new().map(layout_map);
        if let Some(title) = &args.flag_title {
            layout = layout.title(Title::with_text(title));
        }
        plot.set_layout(apply_theme(layout, args.theme()));
    } else {
        // Choropleth on the projection `geo` subplot (token-free built-in geometries).
        let mut trace = Choropleth::new(locations, z)
            .location_mode(mode)
            .color_scale(ColorScale::Palette(palette))
            .show_scale(true)
            .color_bar(ColorBar::new().title(measure_label))
            .marker(ChoroplethMarker::new().line(Line::new().width(0.5)));
        if let Some(spec) = args.flag_geojson.as_deref() {
            trace = trace
                .geojson(load_geojson(spec)?)
                .feature_id_key(args.flag_feature_id_key.as_deref().unwrap_or("id"));
        }
        plot.add_trace(trace);
        let geo = LayoutGeo::new()
            .showland(true)
            .landcolor(NamedColor::LightGray)
            .showcountries(true);
        let mut layout = Layout::new().geo(geo);
        if let Some(title) = &args.flag_title {
            layout = layout.title(Title::with_text(title));
        }
        plot.set_layout(apply_theme(layout, args.theme()));
    }
    Ok(plot)
}

/// Resolve choropleth `(locations, z, measure_label)` from a literal `--locations` region-key
/// column, aggregating the `--value` measure (or row counts) per region.
fn choropleth_literal_locations(
    args: &Args,
    agg: Agg,
) -> CliResult<(Vec<String>, Vec<f64>, String)> {
    let (mut rdr, headers, nh) = reader_and_headers(args)?;
    let loc_idx = resolve_one(args.flag_locations.as_ref(), &headers, nh, "locations")?;
    let value_idx = match args.flag_value.as_ref() {
        Some(s) => Some(resolve_one(Some(s), &headers, nh, "value")?),
        None => None,
    };
    let measure_label = match value_idx {
        Some(i) => col_label(&headers, i, nh),
        None => "count".to_string(),
    };

    let mut raw_locs: Vec<String> = Vec::new();
    let mut values: Vec<f64> = Vec::new();
    let mut record = csv::ByteRecord::new();
    while rdr.read_byte_record(&mut record)? {
        let loc = cell_to_string(record.get(loc_idx));
        if loc.is_empty() {
            continue;
        }
        let value = match value_idx {
            Some(i) => match parse_f64(record.get(i)) {
                Some(v) => v,
                None => continue,
            },
            None => 1.0,
        };
        raw_locs.push(loc);
        values.push(value);
    }
    let (locs, z) = aggregate(raw_locs, values, agg);
    Ok((locs, z, measure_label))
}

/// Resolve choropleth `(locations, z)` via qsv's geocode engine: reverse-geocode `--lat`/`--lon`
/// points, or forward-geocode a `--locations` name column, into ISO-3 / US-state codes per
/// `--location-mode`, then aggregate the `--value` measure (or row counts) per region.
#[cfg(feature = "geocode")]
fn choropleth_geocoded_locations(
    args: &Args,
    mode: LocationMode,
    agg: Agg,
) -> CliResult<(Vec<String>, Vec<f64>, String)> {
    if !matches!(mode, LocationMode::Iso3 | LocationMode::UsaStates) {
        return fail_incorrectusage_clierror!(
            "--geocode only resolves --location-mode iso3 or usa-states (the codes geocode can \
             produce)."
        );
    }
    let has_latlon = args.flag_lat.is_some() && args.flag_lon.is_some();
    let has_names = args.flag_locations.is_some();
    if has_latlon == has_names {
        return fail_incorrectusage_clierror!(
            "--geocode needs exactly one source: --lat/--lon points (reverse) OR a --locations \
             name column (forward)."
        );
    }

    let (mut rdr, headers, nh) = reader_and_headers(args)?;
    let value_idx = match args.flag_value.as_ref() {
        Some(s) => Some(resolve_one(Some(s), &headers, nh, "value")?),
        None => None,
    };
    let measure_label = match value_idx {
        Some(i) => col_label(&headers, i, nh),
        None => "count".to_string(),
    };

    // Collect the per-row geocode query + aligned measure, skipping rows missing inputs.
    let mut values: Vec<f64> = Vec::new();
    let mut record = csv::ByteRecord::new();
    let regions = if has_latlon {
        let lat_idx = resolve_one(args.flag_lat.as_ref(), &headers, nh, "lat")?;
        let lon_idx = resolve_one(args.flag_lon.as_ref(), &headers, nh, "lon")?;
        let mut points: Vec<(f64, f64)> = Vec::new();
        while rdr.read_byte_record(&mut record)? {
            let (Some(lat), Some(lon)) = (
                parse_f64(record.get(lat_idx)),
                parse_f64(record.get(lon_idx)),
            ) else {
                continue;
            };
            let value = match value_idx {
                Some(i) => match parse_f64(record.get(i)) {
                    Some(v) => v,
                    None => continue,
                },
                None => 1.0,
            };
            points.push((lat, lon));
            values.push(value);
        }
        crate::cmd::geocode::reverse_geocode_regions(&points, None)?
    } else {
        let name_idx = resolve_one(args.flag_locations.as_ref(), &headers, nh, "locations")?;
        let mut names: Vec<String> = Vec::new();
        while rdr.read_byte_record(&mut record)? {
            let name = cell_to_string(record.get(name_idx));
            if name.is_empty() {
                continue;
            }
            let value = match value_idx {
                Some(i) => match parse_f64(record.get(i)) {
                    Some(v) => v,
                    None => continue,
                },
                None => 1.0,
            };
            names.push(name);
            values.push(value);
        }
        let name_refs: Vec<&str> = names.iter().map(String::as_str).collect();
        crate::cmd::geocode::forward_geocode_regions(&name_refs, None)?
    };

    // Map each resolved region to the code for the requested mode; drop rows that didn't resolve.
    let mut locations: Vec<String> = Vec::with_capacity(regions.len());
    let mut kept_values: Vec<f64> = Vec::with_capacity(regions.len());
    for (region, value) in regions.into_iter().zip(values) {
        let code = region.and_then(|r| match mode {
            LocationMode::Iso3 => (!r.iso3.is_empty()).then_some(r.iso3),
            LocationMode::UsaStates => r.us_state_code,
            _ => None,
        });
        if let Some(code) = code {
            locations.push(code);
            kept_values.push(value);
        }
    }
    let (locs, z) = aggregate(locations, kept_values, agg);
    Ok((locs, z, measure_label))
}

/// Non-geocode build: `--geocode` is unsupported, so reject it with an actionable message.
#[cfg(not(feature = "geocode"))]
fn choropleth_geocoded_locations(
    _args: &Args,
    _mode: LocationMode,
    _agg: Agg,
) -> CliResult<(Vec<String>, Vec<f64>, String)> {
    fail_incorrectusage_clierror!(
        "--geocode requires a qsv build with the `geocode` feature (or a prebuilt qsv binary). \
         Supply ready-made region codes via --locations instead."
    )
}

/// Split row-aligned (x, y, z) numeric triples into one `Scatter3D` trace per `--series` category,
/// preserving first-seen category order.
fn scatter3d_series_traces(
    xs: Vec<f64>,
    ys: Vec<f64>,
    zs: Vec<f64>,
    series: Vec<String>,
) -> Vec<Box<dyn Trace>> {
    let mut order: Vec<String> = Vec::new();
    let mut groups: BTreeMap<String, (Vec<f64>, Vec<f64>, Vec<f64>)> = BTreeMap::new();
    for i in 0..xs.len() {
        let name = series[i].clone();
        let entry = groups.entry(name.clone()).or_insert_with(|| {
            order.push(name);
            (Vec::new(), Vec::new(), Vec::new())
        });
        entry.0.push(xs[i]);
        entry.1.push(ys[i]);
        entry.2.push(zs[i]);
    }
    order
        .into_iter()
        .map(|name| {
            let (a, b, c) = groups.remove(&name).unwrap_or_default();
            let trace: Box<dyn Trace> = Scatter3D::new(a, b, c).mode(Mode::Markers).name(name);
            trace
        })
        .collect()
}

/// Build the complete `Plot` for `viz scatter3d`: a `Scatter3D` markers trace over three numeric
/// columns (--x/--y/--z), with optional `--color`/`--size` marker encodings or `--series` traces.
/// Uses a 3D `scene` layout (not cartesian axes), so it owns its whole `Plot`.
fn build_scatter3d_plot(args: &Args) -> CliResult<Plot> {
    let (mut rdr, headers, nh) = reader_and_headers(args)?;
    let x_idx = resolve_one(args.flag_x.as_ref(), &headers, nh, "x")?;
    let y_idx = resolve_one(args.flag_y.as_ref(), &headers, nh, "y")?;
    let z_idx = resolve_one(args.flag_z.as_ref(), &headers, nh, "z")?;
    let color_idx = match args.flag_color.as_ref() {
        Some(s) => Some(resolve_one(Some(s), &headers, nh, "color")?),
        None => None,
    };
    let size_idx = match args.flag_size.as_ref() {
        Some(s) => Some(resolve_one(Some(s), &headers, nh, "size")?),
        None => None,
    };
    let series_idx = match args.flag_series.as_ref() {
        Some(s) => Some(resolve_one(Some(s), &headers, nh, "series")?),
        None => None,
    };

    // a row is kept only when x/y/z and every requested encoding parse as numbers, so all
    // per-point arrays stay aligned.
    let mut xs: Vec<f64> = Vec::new();
    let mut ys: Vec<f64> = Vec::new();
    let mut zs: Vec<f64> = Vec::new();
    let mut colors: Vec<f64> = Vec::new();
    let mut sizes: Vec<f64> = Vec::new();
    let mut series: Vec<String> = Vec::new();
    let mut record = csv::ByteRecord::new();
    while rdr.read_byte_record(&mut record)? {
        let (Some(x), Some(y), Some(z)) = (
            parse_f64(record.get(x_idx)),
            parse_f64(record.get(y_idx)),
            parse_f64(record.get(z_idx)),
        ) else {
            continue;
        };
        let color = match color_idx {
            Some(i) => match parse_f64(record.get(i)) {
                Some(v) => Some(v),
                None => continue,
            },
            None => None,
        };
        let size = match size_idx {
            Some(i) => match parse_f64(record.get(i)) {
                Some(v) => Some(v),
                None => continue,
            },
            None => None,
        };
        xs.push(x);
        ys.push(y);
        zs.push(z);
        if let Some(v) = color {
            colors.push(v);
        }
        if let Some(v) = size {
            sizes.push(v);
        }
        if let Some(i) = series_idx {
            series.push(cell_to_string(record.get(i)));
        }
    }
    if xs.is_empty() {
        return fail_clierror!(
            "No plottable rows found (are --x/--y/--z and the --color/--size columns numeric?)."
        );
    }

    let mut plot = Plot::new();
    if series_idx.is_some() {
        for trace in scatter3d_series_traces(xs, ys, zs, series) {
            plot.add_trace(trace);
        }
    } else {
        let mut marker = Marker::new();
        if !sizes.is_empty() {
            marker = marker.size_array(scale_bubble_sizes(&sizes));
        }
        if !colors.is_empty() {
            let color_label = col_label(&headers, color_idx.unwrap(), nh);
            marker = marker
                .color_array(colors)
                .color_scale(ColorScale::Palette(ColorScalePalette::Viridis))
                .show_scale(true)
                .color_bar(ColorBar::new().title(color_label));
        }
        plot.add_trace(
            Scatter3D::new(xs, ys, zs)
                .mode(Mode::Markers)
                .marker(marker),
        );
    }

    let x_title = args
        .flag_x_title
        .clone()
        .unwrap_or_else(|| col_label(&headers, x_idx, nh));
    let y_title = args
        .flag_y_title
        .clone()
        .unwrap_or_else(|| col_label(&headers, y_idx, nh));
    let z_title = col_label(&headers, z_idx, nh);
    let scene = LayoutScene::new()
        .x_axis(Axis::new().title(Title::with_text(x_title)))
        .y_axis(Axis::new().title(Title::with_text(y_title)))
        .z_axis(Axis::new().title(Title::with_text(z_title)));
    let mut layout = Layout::new().scene(scene).show_legend(series_idx.is_some());
    if let Some(title) = &args.flag_title {
        layout = layout.title(Title::with_text(title));
    }
    plot.set_layout(apply_theme(layout, args.theme()));
    Ok(plot)
}

/// Build the `Plot` for `viz treemap` / `viz sunburst`: a domain-based hierarchical part-to-whole
/// chart over the `--cols` dimensions (outer level first), sized by row count (default) or by a
/// summed `--value` measure. Owns its whole `Plot` (no cartesian axes), like `pie`.
fn build_hierarchy_plot(args: &Args) -> CliResult<Plot> {
    let (mut rdr, headers, nh) = reader_and_headers(args)?;
    let dims = resolve_many(args.flag_cols.as_ref(), &headers, nh, "cols")?;
    if dims.len() < HIER_MIN_DIMS {
        return fail_incorrectusage_clierror!(
            "treemap/sunburst needs at least {HIER_MIN_DIMS} --cols (hierarchy levels, outer \
             first)."
        );
    }

    // hierarchy areas must be ADDITIVE so a parent equals the sum of its children: only `count`
    // (default; no --value) and `sum` (with --value) are valid. mean/min/max don't roll up a tree.
    let value_idx = match args
        .flag_agg
        .as_deref()
        .map(str::to_ascii_lowercase)
        .as_deref()
    {
        // a --value with no explicit --agg sums it (matches `viz bar`'s --value semantics)
        None => match args.flag_value.as_ref() {
            Some(s) => Some(resolve_one(Some(s), &headers, nh, "value")?),
            None => None,
        },
        Some("count") => None,
        Some("sum") => {
            let Some(s) = args.flag_value.as_ref() else {
                return fail_incorrectusage_clierror!("--agg sum requires a --value column.");
            };
            Some(resolve_one(Some(s), &headers, nh, "value")?)
        },
        Some(other) => {
            return fail_incorrectusage_clierror!(
                "treemap/sunburst only supports additive --agg (count or sum); got '{other}'."
            );
        },
    };

    let leaves = accumulate_hierarchy_counts(&mut rdr, &dims, value_idx)?;
    let depth = dims.len();
    let top_n = args.flag_limit.max(1);
    let style = if args.cmd_sunburst {
        HierStyle::Sunburst
    } else {
        HierStyle::Treemap
    };
    let Some((labels, parents, values, ids)) =
        hierarchy_arrays(&leaves, depth, top_n, HIER_MAX_NODES, "All")
    else {
        return fail_clierror!(
            "No hierarchy to chart from --cols (need at least one level that splits into 2+ \
             groups)."
        );
    };

    let mut plot = Plot::new();
    plot.add_trace(hierarchy_trace(style, &labels, &parents, &values, &ids));
    let mut layout = Layout::new().show_legend(false);
    if let Some(title) = &args.flag_title {
        layout = layout.title(Title::with_text(title));
    }
    plot.set_layout(apply_theme(layout, args.theme()));
    Ok(plot)
}

fn build_histogram(args: &Args) -> CliResult<(Box<dyn Trace>, String)> {
    let (mut rdr, headers, nh) = reader_and_headers(args)?;
    let x_idx = resolve_one(args.flag_x.as_ref(), &headers, nh, "x")?;

    let mut values: Vec<f64> = Vec::new();
    let mut record = csv::ByteRecord::new();
    while rdr.read_byte_record(&mut record)? {
        if let Some(v) = parse_f64(record.get(x_idx)) {
            values.push(v);
        }
    }
    if values.is_empty() {
        return fail_clierror!("No numeric values found in the --x column for the histogram.");
    }

    let label = col_label(&headers, x_idx, nh);
    let mut hist = Histogram::new(values).name(label.clone());
    if let Some(bins) = args.flag_bins {
        hist = hist.n_bins_x(bins);
    }
    Ok((hist, label))
}

fn build_box(args: &Args) -> CliResult<(Box<dyn Trace>, String, Option<String>)> {
    let (mut rdr, headers, nh) = reader_and_headers(args)?;
    let y_idx = resolve_one(args.flag_y.as_ref(), &headers, nh, "y")?;
    let group_idx = match args.flag_x.as_ref() {
        Some(s) => Some(resolve_one(Some(s), &headers, nh, "x")?),
        None => None,
    };

    let mut ys: Vec<f64> = Vec::new();
    let mut groups: Vec<String> = Vec::new();
    let mut record = csv::ByteRecord::new();
    while rdr.read_byte_record(&mut record)? {
        let Some(y) = parse_f64(record.get(y_idx)) else {
            continue;
        };
        ys.push(y);
        if let Some(i) = group_idx {
            groups.push(cell_to_string(record.get(i)));
        }
    }
    if ys.is_empty() {
        return fail_clierror!("No numeric values found in the --y column for the box plot.");
    }

    // Unlike `viz smart` (which draws boxes from precomputed quartiles with observed
    // min/max whiskers, since the stats cache has no per-row data), explicit `viz box`
    // reads the raw values, so plotly can render true Tukey whiskers (extending to the
    // most extreme point within 1.5*IQR of the quartiles) and plot the points beyond
    // those fences as individual outliers. `QuartileMethod::Linear` is the standard
    // (linear-interpolation) quartile definition Tukey fences are built on.
    let box_points = parse_box_points(args.flag_box_points.as_deref())?;

    let y_label = col_label(&headers, y_idx, nh);
    let x_label = group_idx.map(|i| col_label(&headers, i, nh));
    let trace: Box<dyn Trace> = if group_idx.is_some() {
        BoxPlot::new_xy(groups, ys)
            .quartile_method(QuartileMethod::Linear)
            .box_points(box_points)
    } else {
        BoxPlot::new(ys)
            .name(y_label.clone())
            .quartile_method(QuartileMethod::Linear)
            .box_points(box_points)
    };
    Ok((trace, y_label, x_label))
}

/// Resolve a required multi-column selector to its column indices (one or more).
fn resolve_many(
    sel: Option<&SelectColumns>,
    headers: &csv::ByteRecord,
    no_headers: bool,
    flag: &str,
) -> CliResult<Vec<usize>> {
    let Some(sel) = sel else {
        return fail_incorrectusage_clierror!("--{flag} is required for this chart type.");
    };
    let selection = sel
        .selection(headers, !no_headers)
        .map_err(crate::CliError::Other)?;
    if selection.is_empty() {
        return fail_incorrectusage_clierror!("--{flag} selected no columns.");
    }
    Ok(selection.iter().copied().collect())
}

/// From an already-open reader, keep the candidate columns that are numeric (a majority of
/// non-empty cells parse as f64), and return their labels plus listwise-complete value vectors
/// (rows where any kept column is non-numeric/empty are dropped). Takes the reader + headers
/// from the caller (rather than opening its own) so the input is read exactly once — opening a
/// second time would consume/corrupt a streamed stdin. Shared by the standalone correlation
/// heatmap and the `viz smart` correlation panel.
fn read_numeric_columns(
    rdr: &mut csv::Reader<Box<dyn std::io::Read + Send>>,
    headers: &csv::ByteRecord,
    nh: bool,
    candidates: &[usize],
) -> CliResult<(Vec<String>, Vec<Vec<f64>>)> {
    let mut raw: Vec<Vec<Option<f64>>> = vec![Vec::new(); candidates.len()];
    let mut nonempty = vec![0_usize; candidates.len()];
    let mut parsed = vec![0_usize; candidates.len()];
    let mut record = csv::ByteRecord::new();
    while rdr.read_byte_record(&mut record)? {
        for (k, &idx) in candidates.iter().enumerate() {
            let cell = record.get(idx);
            let is_nonempty = cell.is_some_and(|c| !c.iter().all(u8::is_ascii_whitespace));
            let v = parse_f64(cell);
            if is_nonempty {
                nonempty[k] += 1;
                if v.is_some() {
                    parsed[k] += 1;
                }
            }
            raw[k].push(v);
        }
    }
    // keep majority-numeric columns (drops text/ID/date columns from an all-columns default)
    let keep: Vec<usize> = (0..candidates.len())
        .filter(|&k| nonempty[k] > 0 && parsed[k] * 2 >= nonempty[k])
        .collect();
    if keep.is_empty() {
        return Ok((Vec::new(), Vec::new()));
    }
    let labels: Vec<String> = keep
        .iter()
        .map(|&k| col_label(headers, candidates[k], nh))
        .collect();
    let n_rows = raw[0].len();
    let kept: Vec<&Vec<Option<f64>>> = keep.iter().map(|&k| &raw[k]).collect();
    let mut columns: Vec<Vec<f64>> = vec![Vec::new(); kept.len()];
    // transpose the kept columns, dropping any row where a kept column is non-numeric/empty
    for r in 0..n_rows {
        if kept.iter().all(|col| col[r].is_some()) {
            for (out, col) in columns.iter_mut().zip(&kept) {
                out.push(col[r].expect("checked is_some above"));
            }
        }
    }
    Ok((labels, columns))
}

/// Pearson correlation of two equal-length numeric slices via a numerically stable, centered
/// two-pass algorithm (raw-sums formulas suffer catastrophic cancellation for large values
/// with small variance). The denominator is `var_x.sqrt() * var_y.sqrt()` rather than
/// `(var_x * var_y).sqrt()` so it stays finite for large-but-valid variances (the product
/// would overflow to infinity and spuriously yield `NaN`). Returns `NaN` when the correlation
/// is undefined — fewer than two points, or zero variance in either input — so callers can
/// render it as a gap rather than a fabricated 0.0. The result is clamped to [-1, 1] to absorb
/// floating-point overshoot.
fn pearson(x: &[f64], y: &[f64]) -> f64 {
    let len = x.len().min(y.len());
    if len < 2 {
        return f64::NAN;
    }
    let n = len as f64;
    let mean_x = x[..len].iter().sum::<f64>() / n;
    let mean_y = y[..len].iter().sum::<f64>() / n;
    let (mut cov, mut var_x, mut var_y) = (0.0, 0.0, 0.0);
    for k in 0..len {
        let dx = x[k] - mean_x;
        let dy = y[k] - mean_y;
        cov += dx * dy;
        var_x += dx * dx;
        var_y += dy * dy;
    }
    let den = var_x.sqrt() * var_y.sqrt();
    if den == 0.0 || !den.is_finite() {
        f64::NAN
    } else {
        (cov / den).clamp(-1.0, 1.0)
    }
}

/// Tie-averaged (fractional) ranks of `v`, 1-based, for Spearman's rho. Equal values share the
/// mean of the ranks they span, so tied data isn't biased by an arbitrary ordering.
fn average_ranks(v: &[f64]) -> Vec<f64> {
    let n = v.len();
    let mut idx: Vec<usize> = (0..n).collect();
    idx.sort_by(|&a, &b| v[a].partial_cmp(&v[b]).unwrap_or(std::cmp::Ordering::Equal));
    let mut ranks = vec![0.0_f64; n];
    let mut i = 0;
    while i < n {
        let mut j = i + 1;
        while j < n && v[idx[j]] == v[idx[i]] {
            j += 1;
        }
        // mean of the 1-based ranks (i+1)..=j spanned by this tie group.
        #[allow(clippy::cast_precision_loss)]
        let avg = (i + 1 + j) as f64 / 2.0;
        for &k in &idx[i..j] {
            ranks[k] = avg;
        }
        i = j;
    }
    ranks
}

/// Spearman's rank correlation: Pearson on the (tie-averaged) ranks of each column. It measures
/// MONOTONIC association, so a large `|spearman| - |pearson|` gap signals a monotonic-but-curved
/// (nonlinear) relationship — one a single Pearson r understates. Returns `NaN` when undefined
/// (propagated from `pearson`).
fn spearman_rho(x: &[f64], y: &[f64]) -> f64 {
    pearson(&average_ranks(x), &average_ranks(y))
}

/// Blank out the upper triangle AND the diagonal of a symmetric correlation matrix (set to `NaN`,
/// which serializes to JSON `null` so plotly draws no cell and `corr_incell_annotations` skips it),
/// leaving only the lower triangle. A correlation matrix mirrors across the diagonal and the
/// diagonal is a trivial 1.0, so the full square wastes half the panel on redundant cells; the
/// lower triangle is the standard, less-cluttered presentation.
fn mask_to_lower_triangle(mut matrix: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    for (r, row) in matrix.iter_mut().enumerate() {
        for (c, cell) in row.iter_mut().enumerate() {
            if c >= r {
                *cell = f64::NAN;
            }
        }
    }
    matrix
}

/// Pick the least-redundant third axis for a 3D scatter, given the two columns `i`/`j` already
/// chosen as the strongest-correlated pair. "Least redundant" = the candidate `k` whose strongest
/// correlation to either chosen axis (`max(|r_ik|, |r_jk|)`) is smallest, so the third dimension
/// adds the most new information.
///
/// Candidates are restricted to those with DEFINED correlations to both chosen axes. A numeric
/// column can become constant (zero-variance) on the listwise-complete rows even though column
/// stats admitted it, leaving NaN correlations; `NaN.partial_cmp(..)` falls back to `Equal`, so an
/// unfiltered `min_by` could select such a degenerate column as "least redundant" and render a
/// flat, meaningless 3D axis. Returns `None` when no finite candidate remains (caller skips 3D).
fn least_redundant_third(matrix: &[Vec<f64>], i: usize, j: usize) -> Option<usize> {
    (0..matrix.len())
        .filter(|&k| k != i && k != j && matrix[i][k].is_finite() && matrix[j][k].is_finite())
        .min_by(|&a, &b| {
            let ra = matrix[i][a].abs().max(matrix[j][a].abs());
            let rb = matrix[i][b].abs().max(matrix[j][b].abs());
            ra.partial_cmp(&rb).unwrap_or(std::cmp::Ordering::Equal)
        })
}

/// Pearson correlation matrix for equal-length numeric columns: NxN symmetric. The diagonal
/// is computed from `pearson` (rather than hard-coded to 1.0) so a degenerate column — zero
/// variance or too few observations — surfaces as `NaN` instead of a fabricated 1.0. Cells for
/// undefined correlations are likewise `NaN` (serialized to `null` → rendered as a heatmap gap).
fn pearson_matrix(columns: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let n = columns.len();
    let mut m = vec![vec![f64::NAN; n]; n];
    for i in 0..n {
        m[i][i] = pearson(&columns[i], &columns[i]);
        for j in (i + 1)..n {
            let r = pearson(&columns[i], &columns[j]);
            m[i][j] = r;
            m[j][i] = r;
        }
    }
    m
}

/// Index pair (i, j) with the largest absolute Pearson correlation in the off-diagonal of a
/// symmetric matrix, returned with its signed r. Skips NaN cells (undefined correlations, e.g.
/// a constant column). Returns None when the matrix has fewer than two columns or every
/// off-diagonal cell is NaN.
fn strongest_pair(matrix: &[Vec<f64>]) -> Option<(usize, usize, f64)> {
    let mut best: Option<(usize, usize, f64)> = None;
    for (i, row) in matrix.iter().enumerate() {
        for (j, &r) in row.iter().enumerate().skip(i + 1) {
            if r.is_nan() {
                continue;
            }
            if best.is_none_or(|(_, _, b)| r.abs() > b.abs()) {
                best = Some((i, j, r));
            }
        }
    }
    best
}

/// A diverging (RdBu) correlation heatmap trace fixed to the [-1, 1] scale. `axes` assigns
/// the subplot axis refs when used as a `viz smart` panel (None for the standalone chart).
/// A `hovertemplate` (and trace name) gives a clean `y vs x: r` tooltip instead of plotly's
/// default "trace 0".
fn corr_heatmap_trace(
    labels: Vec<String>,
    matrix: Vec<Vec<f64>>,
    axes: Option<(String, String)>,
    show_scale: bool,
) -> Box<dyn Trace> {
    let mut h = HeatMap::new(labels.clone(), labels, matrix)
        .color_scale(ColorScale::Palette(ColorScalePalette::RdBu))
        .zmin(-1.0)
        .zmax(1.0)
        .zmid(0.0)
        .show_scale(show_scale)
        .name("correlation")
        .hover_template("%{y} \u{2194} %{x}<br>r = %{z:.3f}<extra></extra>");
    if let Some((x, y)) = axes {
        h = h.x_axis(x).y_axis(y);
    }
    h
}

/// Truncate a label to at most `max` characters (Unicode-aware), appending an ellipsis when
/// shortened. Used for `viz smart` correlation-heatmap axis ticks so long numeric-column names
/// don't clip against the dashboard's left margin (full names remain visible on hover).
fn truncate_label(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        return s.to_string();
    }
    let keep = max.saturating_sub(1);
    let mut out: String = s.chars().take(keep).collect();
    out.push('\u{2026}');
    out
}

/// Pie chart of label proportions: sums --y per --x label, or counts label occurrences when
/// --y is omitted.
fn build_pie(args: &Args) -> CliResult<Box<dyn Trace>> {
    let (mut rdr, headers, nh) = reader_and_headers(args)?;
    let label_idx = resolve_one(args.flag_x.as_ref(), &headers, nh, "x")?;
    let value_idx = match args.flag_y.as_ref() {
        Some(s) => Some(resolve_one(Some(s), &headers, nh, "y")?),
        None => None,
    };

    let mut order: Vec<String> = Vec::new();
    let mut acc: HashMap<String, f64> = HashMap::new();
    let mut record = csv::ByteRecord::new();
    while rdr.read_byte_record(&mut record)? {
        let label = cell_to_string(record.get(label_idx));
        if label.is_empty() {
            continue;
        }
        let inc = match value_idx {
            Some(i) => match parse_f64(record.get(i)) {
                Some(v) => v,
                None => continue,
            },
            None => 1.0,
        };
        if let Some(v) = acc.get_mut(&label) {
            *v += inc;
        } else {
            order.push(label.clone());
            acc.insert(label, inc);
        }
    }
    if order.is_empty() {
        return fail_clierror!("No data found for the pie chart.");
    }
    let values: Vec<f64> = order.iter().map(|l| acc[l]).collect();
    advise_if_pie_hard_to_read(&values);
    let mut pie = Pie::new(values).labels(order).text_info("label+percent");
    if args.flag_donut {
        pie = pie.hole(0.4);
    }
    Ok(pie)
}

/// Non-fatal advisory: a pie of many NEAR-EQUAL slices is the worst case for a pie chart — humans
/// compare angles/areas poorly, so similar slices are nearly indistinguishable and a bar chart is
/// strictly easier to read. Measured by the coefficient of variation (stddev / mean) of the slice
/// values: low CV across enough slices means no slice dominates. Only nudges (prints to stderr);
/// the explicit `viz pie` request is still rendered.
fn advise_if_pie_hard_to_read(values: &[f64]) {
    if values.len() < PIE_NEAR_EQUAL_MIN_SLICES {
        return;
    }
    #[allow(clippy::cast_precision_loss)]
    let n = values.len() as f64;
    let mean = values.iter().sum::<f64>() / n;
    if mean <= 0.0 {
        return;
    }
    let variance = values.iter().map(|v| (v - mean) * (v - mean)).sum::<f64>() / n;
    let cv = variance.sqrt() / mean;
    if cv < PIE_NEAR_EQUAL_MAX_CV {
        eprintln!(
            "viz pie: {} slices are near-equal (coefficient of variation {cv:.2} < \
             {PIE_NEAR_EQUAL_MAX_CV:.2}); a pie makes near-equal slices hard to compare. Consider \
             `viz bar` for an easier read.",
            values.len()
        );
    }
}

/// Heatmap: a correlation matrix of numeric columns (default), or a category x category
/// pivot when --x, --y and --z are all given.
fn build_heatmap(args: &Args) -> CliResult<(Box<dyn Trace>, Option<String>, Option<String>)> {
    if args.flag_x.is_some() && args.flag_y.is_some() && args.flag_z.is_some() {
        return build_heatmap_pivot(args);
    }
    if args.flag_z.is_some() {
        return fail_incorrectusage_clierror!(
            "heatmap pivot mode needs --x, --y and --z together. Omit --z for a correlation \
             heatmap."
        );
    }
    build_heatmap_correlation(args)
}

fn build_heatmap_correlation(
    args: &Args,
) -> CliResult<(Box<dyn Trace>, Option<String>, Option<String>)> {
    let (mut rdr, headers, nh) = reader_and_headers(args)?;
    let candidates: Vec<usize> = match args.flag_cols.as_ref() {
        Some(s) => resolve_many(Some(s), &headers, nh, "cols")?,
        None => (0..headers.len()).collect(),
    };
    let (labels, columns) = read_numeric_columns(&mut rdr, &headers, nh, &candidates)?;
    if labels.len() < 2 {
        return fail_clierror!(
            "A correlation heatmap needs at least 2 numeric columns (found {}). Use --cols to \
             select them.",
            labels.len()
        );
    }
    let n_obs = columns.first().map_or(0, Vec::len);
    if n_obs < 2 {
        return fail_clierror!(
            "A correlation heatmap needs at least 2 rows where all selected numeric columns are \
             present (found {n_obs})."
        );
    }
    let matrix = pearson_matrix(&columns);
    Ok((corr_heatmap_trace(labels, matrix, None, true), None, None))
}

fn build_heatmap_pivot(args: &Args) -> CliResult<(Box<dyn Trace>, Option<String>, Option<String>)> {
    let (mut rdr, headers, nh) = reader_and_headers(args)?;
    let x_idx = resolve_one(args.flag_x.as_ref(), &headers, nh, "x")?;
    let y_idx = resolve_one(args.flag_y.as_ref(), &headers, nh, "y")?;
    let z_idx = resolve_one(args.flag_z.as_ref(), &headers, nh, "z")?;

    let mut x_cats: Vec<String> = Vec::new();
    let mut x_pos: HashMap<String, usize> = HashMap::new();
    let mut y_cats: Vec<String> = Vec::new();
    let mut y_pos: HashMap<String, usize> = HashMap::new();
    // (y, x) -> (sum, count) so duplicate cells are averaged
    let mut cells: HashMap<(usize, usize), (f64, f64)> = HashMap::new();
    let mut record = csv::ByteRecord::new();
    while rdr.read_byte_record(&mut record)? {
        let Some(z) = parse_f64(record.get(z_idx)) else {
            continue;
        };
        let xv = cell_to_string(record.get(x_idx));
        let yv = cell_to_string(record.get(y_idx));
        let xi = *x_pos.entry(xv.clone()).or_insert_with(|| {
            x_cats.push(xv);
            x_cats.len() - 1
        });
        let yi = *y_pos.entry(yv.clone()).or_insert_with(|| {
            y_cats.push(yv);
            y_cats.len() - 1
        });
        let e = cells.entry((yi, xi)).or_insert((0.0, 0.0));
        e.0 += z;
        e.1 += 1.0;
    }
    if x_cats.is_empty() || y_cats.is_empty() {
        return fail_clierror!("No data found for the heatmap pivot (is --z numeric?).");
    }
    // z matrix indexed [y][x]; missing cells are NaN (serialized as null -> rendered as a gap)
    let mut z: Vec<Vec<f64>> = vec![vec![f64::NAN; x_cats.len()]; y_cats.len()];
    for ((yi, xi), (sum, cnt)) in cells {
        if cnt > 0.0 {
            z[yi][xi] = sum / cnt;
        }
    }
    let x_label = col_label(&headers, x_idx, nh);
    let y_label = col_label(&headers, y_idx, nh);
    let trace = HeatMap::new(x_cats, y_cats, z)
        .color_scale(ColorScale::Palette(ColorScalePalette::Viridis));
    Ok((trace, Some(x_label), Some(y_label)))
}

/// Bin two row-aligned numeric vectors into a `bins` x `bins` grid of point counts, returning the
/// per-axis bin-center coordinates and the `z[yi][xi]` count matrix. Shared by `viz contour` and
/// `viz smart`'s correlated-pair density panel.
fn bin_2d(xs: &[f64], ys: &[f64], bins: usize) -> (Vec<f64>, Vec<f64>, Vec<Vec<f64>>) {
    let x_min = xs.iter().copied().fold(f64::INFINITY, f64::min);
    let x_max = xs.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let y_min = ys.iter().copied().fold(f64::INFINITY, f64::min);
    let y_max = ys.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    // guard against a degenerate (single-value) axis so the bin math never divides by zero
    let x_span = if x_max > x_min { x_max - x_min } else { 1.0 };
    let y_span = if y_max > y_min { y_max - y_min } else { 1.0 };

    // z[yi][xi] = number of points falling in that cell
    let mut z = vec![vec![0.0_f64; bins]; bins];
    for (&x, &y) in xs.iter().zip(ys.iter()) {
        let xi = (((x - x_min) / x_span) * bins as f64) as usize;
        let yi = (((y - y_min) / y_span) * bins as f64) as usize;
        z[yi.min(bins - 1)][xi.min(bins - 1)] += 1.0;
    }

    // bin-center coordinates for the axes
    let x_centers: Vec<f64> = (0..bins)
        .map(|i| x_min + (i as f64 + 0.5) * x_span / bins as f64)
        .collect();
    let y_centers: Vec<f64> = (0..bins)
        .map(|i| y_min + (i as f64 + 0.5) * y_span / bins as f64)
        .collect();
    (x_centers, y_centers, z)
}

/// Build a `Contour` trace: the 2D density of two numeric columns (--x and --y), binned into a
/// `--bins` x `--bins` grid of point counts. Unlike `heatmap` (categorical/correlation), this
/// shows the smooth joint distribution of two continuous variables.
fn build_contour(args: &Args) -> CliResult<(Box<dyn Trace>, String, String)> {
    let (mut rdr, headers, nh) = reader_and_headers(args)?;
    let x_idx = resolve_one(args.flag_x.as_ref(), &headers, nh, "x")?;
    let y_idx = resolve_one(args.flag_y.as_ref(), &headers, nh, "y")?;

    let mut xs: Vec<f64> = Vec::new();
    let mut ys: Vec<f64> = Vec::new();
    let mut record = csv::ByteRecord::new();
    while rdr.read_byte_record(&mut record)? {
        let (Some(x), Some(y)) = (parse_f64(record.get(x_idx)), parse_f64(record.get(y_idx)))
        else {
            continue;
        };
        xs.push(x);
        ys.push(y);
    }
    if xs.is_empty() {
        return fail_clierror!("No rows with numeric --x and --y values found for the contour.");
    }

    // grid resolution: --bins per axis (clamped to a sane range), else a default
    let bins = args.flag_bins.unwrap_or(20).clamp(2, 200);
    let (x_centers, y_centers, z) = bin_2d(&xs, &ys, bins);

    let trace = Contour::new(x_centers, y_centers, z)
        .color_scale(ColorScale::Palette(ColorScalePalette::Viridis));
    Ok((
        trace,
        col_label(&headers, x_idx, nh),
        col_label(&headers, y_idx, nh),
    ))
}

/// Candlestick (or OHLC bar) chart from a date/x column and four numeric price columns.
fn build_candlestick(args: &Args, ohlc: bool) -> CliResult<(Box<dyn Trace>, String, String)> {
    let (mut rdr, headers, nh) = reader_and_headers(args)?;
    let x_idx = resolve_one(args.flag_x.as_ref(), &headers, nh, "x")?;
    let o_idx = resolve_one(args.flag_ohlc_open.as_ref(), &headers, nh, "ohlc-open")?;
    let h_idx = resolve_one(args.flag_high.as_ref(), &headers, nh, "high")?;
    let l_idx = resolve_one(args.flag_low.as_ref(), &headers, nh, "low")?;
    let c_idx = resolve_one(args.flag_close.as_ref(), &headers, nh, "close")?;

    let (mut xs, mut open, mut high, mut low, mut close) =
        (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new());
    let mut record = csv::ByteRecord::new();
    while rdr.read_byte_record(&mut record)? {
        let (Some(ov), Some(hv), Some(lv), Some(cv)) = (
            parse_f64(record.get(o_idx)),
            parse_f64(record.get(h_idx)),
            parse_f64(record.get(l_idx)),
            parse_f64(record.get(c_idx)),
        ) else {
            continue;
        };
        // x is passed through as a string; plotly auto-detects ISO dates client-side and
        // renders anything else as a category (no Rust-side date parsing).
        xs.push(cell_to_string(record.get(x_idx)));
        open.push(ov);
        high.push(hv);
        low.push(lv);
        close.push(cv);
    }
    if xs.is_empty() {
        return fail_clierror!(
            "No complete rows found (need numeric --ohlc-open/--high/--low/--close)."
        );
    }
    let x_label = col_label(&headers, x_idx, nh);
    // Plotly's default OHLC/candlestick hover label leads with the trace name; a custom
    // template gives a clean O/H/L/C readout. The x (date) is omitted on purpose — the
    // `x unified` hover mode (build_layout) already renders x as the tooltip header.
    // `hover_template_fallback` is defensive: financial traces have known gaps resolving
    // the per-point %{open|high|low|close} variables.
    let hover = "Open: %{open}<br>High: %{high}<br>Low: %{low}<br>Close: %{close}<extra></extra>";
    let trace: Box<dyn Trace> = if ohlc {
        Box::new(
            Ohlc::new(xs, open, high, low, close)
                .hover_template(hover)
                .hover_template_fallback("-"),
        )
    } else {
        Box::new(
            Candlestick::new(xs, open, high, low, close)
                .hover_template(hover)
                .hover_template_fallback("-"),
        )
    };
    Ok((trace, x_label, "price".to_string()))
}

/// Sankey flow diagram. Builds a unified node index across source & target labels and
/// aggregates duplicate source->target pairs into a single weighted link.
fn build_sankey(args: &Args) -> CliResult<Box<dyn Trace>> {
    let (mut rdr, headers, nh) = reader_and_headers(args)?;
    let s_idx = resolve_one(args.flag_source.as_ref(), &headers, nh, "source")?;
    let t_idx = resolve_one(args.flag_target.as_ref(), &headers, nh, "target")?;
    let v_idx = match args.flag_value.as_ref() {
        Some(s) => Some(resolve_one(Some(s), &headers, nh, "value")?),
        None => None,
    };

    let mut node_labels: Vec<String> = Vec::new();
    let mut node_pos: HashMap<String, usize> = HashMap::new();
    let mut links: HashMap<(usize, usize), f64> = HashMap::new();
    let mut link_order: Vec<(usize, usize)> = Vec::new();
    let mut record = csv::ByteRecord::new();
    while rdr.read_byte_record(&mut record)? {
        let s = cell_to_string(record.get(s_idx));
        let t = cell_to_string(record.get(t_idx));
        if s.is_empty() || t.is_empty() {
            continue;
        }
        let val = match v_idx {
            Some(i) => parse_f64(record.get(i)).unwrap_or(0.0),
            None => 1.0,
        };
        let si = *node_pos.entry(s.clone()).or_insert_with(|| {
            node_labels.push(s);
            node_labels.len() - 1
        });
        let ti = *node_pos.entry(t.clone()).or_insert_with(|| {
            node_labels.push(t);
            node_labels.len() - 1
        });
        links
            .entry((si, ti))
            .and_modify(|w| *w += val)
            .or_insert_with(|| {
                link_order.push((si, ti));
                val
            });
    }
    if link_order.is_empty() {
        return fail_clierror!(
            "No flows found for the sankey diagram (need --source and --target)."
        );
    }

    let label_refs: Vec<&str> = node_labels.iter().map(String::as_str).collect();
    let node = Node::new().label(label_refs).pad(15).thickness(20);
    let (mut sources, mut targets, mut values) = (Vec::new(), Vec::new(), Vec::new());
    for &(si, ti) in &link_order {
        sources.push(si);
        targets.push(ti);
        values.push(links[&(si, ti)]);
    }
    let link = Link::new().source(sources).target(targets).value(values);
    Ok(Sankey::new().node(node).link(link))
}

/// Radar (polar) chart of numeric --cols. Each axis is min-max normalized to 0..1 across all
/// rows (axes typically differ in scale); --series produces one polygon per distinct value,
/// each the per-axis mean of its rows.
fn build_radar(args: &Args) -> CliResult<Vec<Box<dyn Trace>>> {
    let (mut rdr, headers, nh) = reader_and_headers(args)?;
    let axis_idx = resolve_many(args.flag_cols.as_ref(), &headers, nh, "cols")?;
    if axis_idx.len() < 2 {
        return fail_incorrectusage_clierror!(
            "radar needs at least 2 columns via --cols (got {}).",
            axis_idx.len()
        );
    }
    let series_idx = match args.flag_series.as_ref() {
        Some(s) => Some(resolve_one(Some(s), &headers, nh, "series")?),
        None => None,
    };
    let axis_labels: Vec<String> = axis_idx
        .iter()
        .map(|&i| col_label(&headers, i, nh))
        .collect();

    let mut order: Vec<String> = Vec::new();
    let mut grouped: HashMap<String, Vec<Vec<f64>>> = HashMap::new();
    let mut record = csv::ByteRecord::new();
    while rdr.read_byte_record(&mut record)? {
        let mut vals = Vec::with_capacity(axis_idx.len());
        let mut complete = true;
        for &i in &axis_idx {
            match parse_f64(record.get(i)) {
                Some(v) => vals.push(v),
                None => {
                    complete = false;
                    break;
                },
            }
        }
        if !complete {
            continue;
        }
        let series = match series_idx {
            Some(i) => cell_to_string(record.get(i)),
            None => String::new(),
        };
        grouped
            .entry(series.clone())
            .or_insert_with(|| {
                order.push(series);
                Vec::new()
            })
            .push(vals);
    }
    if order.is_empty() {
        return fail_clierror!("No numeric rows found for the radar chart.");
    }

    let n_axes = axis_idx.len();
    let mut mins = vec![f64::INFINITY; n_axes];
    let mut maxs = vec![f64::NEG_INFINITY; n_axes];
    for rows in grouped.values() {
        for row in rows {
            for a in 0..n_axes {
                mins[a] = mins[a].min(row[a]);
                maxs[a] = maxs[a].max(row[a]);
            }
        }
    }
    let normalize = |a: usize, v: f64| -> f64 {
        let (lo, hi) = (mins[a], maxs[a]);
        if (hi - lo).abs() < f64::EPSILON {
            0.5
        } else {
            (v - lo) / (hi - lo)
        }
    };

    // close the polygon by repeating the first axis at the end
    let mut theta_closed = axis_labels.clone();
    theta_closed.push(axis_labels[0].clone());

    let mut traces: Vec<Box<dyn Trace>> = Vec::with_capacity(order.len());
    for series in order {
        let rows = &grouped[&series];
        let mut means = vec![0.0; n_axes];
        for row in rows {
            for a in 0..n_axes {
                means[a] += row[a];
            }
        }
        for m in &mut means {
            *m /= rows.len() as f64;
        }
        let mut r: Vec<f64> = (0..n_axes).map(|a| normalize(a, means[a])).collect();
        r.push(r[0]);
        let mut t = ScatterPolar::new(theta_closed.clone(), r)
            .mode(Mode::Lines)
            .fill(Fill::ToSelf);
        if !series.is_empty() {
            t = t.name(series);
        }
        traces.push(t);
    }
    Ok(traces)
}

/// Map a `--theme` name to a plotly built-in theme. Case-insensitive; hyphens are
/// accepted as separators. Returns `None` for an unrecognized name so callers can
/// surface a usage error.
fn parse_theme(name: &str) -> Option<BuiltinTheme> {
    match name.trim().to_ascii_lowercase().replace('-', "_").as_str() {
        "default" => Some(BuiltinTheme::Default),
        "plotly_white" | "white" => Some(BuiltinTheme::PlotlyWhite),
        "plotly_dark" | "dark" => Some(BuiltinTheme::PlotlyDark),
        "seaborn" => Some(BuiltinTheme::Seaborn),
        "seaborn_whitegrid" => Some(BuiltinTheme::SeabornWhitegrid),
        "seaborn_dark" => Some(BuiltinTheme::SeabornDark),
        "matplotlib" => Some(BuiltinTheme::Matplotlib),
        "plotnine" => Some(BuiltinTheme::Plotnine),
        _ => None,
    }
}

/// The comma-separated list of accepted `--theme` names, for usage errors.
const VALID_THEMES: &str = "default, plotly_white, plotly_dark, seaborn, seaborn_whitegrid, \
                            seaborn_dark, matplotlib, plotnine";

impl Args {
    /// The resolved plotly theme for this invocation. `flag_theme` is validated in
    /// `run()`, so an unrecognized name never reaches here (returns `None`).
    fn theme(&self) -> Option<BuiltinTheme> {
        self.flag_theme.as_deref().and_then(parse_theme)
    }
}

/// Apply the chosen built-in theme to a layout as a plotly template. When no theme
/// is set the layout is returned unchanged (qsv's built-in look). Explicit color
/// overrides at the call sites are gated on `theme.is_none()` so the template can
/// drive backgrounds, fonts and axis chrome.
fn apply_theme(layout: Layout, theme: Option<BuiltinTheme>) -> Layout {
    match theme {
        Some(t) => layout.template(t.build()),
        None => layout,
    }
}

/// The `(page background, text color)` to use for the inline `viz smart` HTML page chrome
/// (the `<body>` around the panel grid), matching each built-in theme's paper/font colors so
/// a dark theme gets a dark page rather than dark plots floating on white. Mirrors the values
/// in plotly's `layout::themes` templates. `None` keeps qsv's built-in look.
fn theme_page_chrome(theme: Option<BuiltinTheme>) -> (&'static str, &'static str) {
    match theme {
        None => (PAPER_BG, INK),
        Some(BuiltinTheme::Default | BuiltinTheme::PlotlyWhite) => ("#FFFFFF", "#2a3f5f"),
        Some(BuiltinTheme::PlotlyDark) => ("#111111", "#f2f5fa"),
        Some(BuiltinTheme::Seaborn) => ("#EAEAF2", "#333333"),
        Some(BuiltinTheme::SeabornWhitegrid) => ("#FFFFFF", "#333333"),
        Some(BuiltinTheme::SeabornDark) => ("#222222", "#eaeaf2"),
        Some(BuiltinTheme::Matplotlib) => ("#FFFFFF", "black"),
        Some(BuiltinTheme::Plotnine) => ("#EBEBEB", "#525252"),
    }
}

/// The CSS, button markup, and re-theming `<script>` for the `viz smart` light/dark toggle,
/// shared by both HTML render paths (the inline-div grid and the single typed-`Plot` grid).
/// Page chrome is driven by CSS variables (`--qsv-page-bg`/`--qsv-page-ink`/`--qsv-geo-meta`)
/// so flipping `body.qsv-dark` recolors the page instantly, while the script calls
/// `Plotly.relayout` on every live graph div to recolor the plots themselves. `theme` only
/// decides the *default* mode when the viewer has no saved/OS preference (dark built-in themes
/// open dark). Known limitation: geo/scene/polar/pie panels flip their background, font, and
/// container color, but basemap fills, mapbox tiles, and trace marker colors do NOT re-theme
/// (that would need a trace-type-aware `Plotly.restyle`, which is out of scope here).
struct ToggleChrome {
    /// CSS for the `:root`/`body.qsv-dark` variables and the toggle button; goes in `<style>`.
    style:  String,
    /// The fixed-position toggle button; placed right after `<body>`.
    button: String,
    /// The toggle `<script>`; placed just before `</body>` (after all `Plotly.newPlot` calls).
    script: String,
}

fn toggle_chrome(theme: Option<BuiltinTheme>) -> ToggleChrome {
    let (light_bg, light_ink) = theme_page_chrome(theme);
    // Three-state initial mode (overridable by a saved localStorage choice). An EXPLICIT
    // `--theme` opens in that theme's implied mode — dark for the dark built-ins, light for every
    // other (light) built-in — so e.g. `--theme plotly_white` is NOT overridden by a dark-mode OS.
    // Only with no `--theme` at all do we defer to the viewer's `prefers-color-scheme`.
    let default_mode = match theme {
        None => "system",
        Some(BuiltinTheme::PlotlyDark | BuiltinTheme::SeabornDark) => "dark",
        Some(_) => "light",
    };

    // Raw-string templates with token placeholders, so the brace-heavy JS needs no `{{`/`}}`
    // escaping and rustfmt's `format_strings` (regular-string-only) won't reflow/mangle them.
    let style = STYLE_TEMPLATE
        .replace("__LIGHT_BG__", light_bg)
        .replace("__LIGHT_INK__", light_ink)
        .replace("__FONT_FAMILY__", FONT_FAMILY);

    let button = "<button id=\"qsv-theme-toggle\" type=\"button\" aria-label=\"Toggle light/dark \
                  mode\">\u{1F313} Theme</button>"
        .to_string();

    // The light palette mirrors qsv's built-in look (INK/PAPER_BG/GRID_COLOR/AXIS_LINE); the dark
    // palette is a fixed dark set. `buildUpdate` only sets keys present on a given graph's layout,
    // so axis-less plots (pie) and arbitrary subplot counts both work without knowing div ids.
    let script = SCRIPT_TEMPLATE
        .replace("__DEFAULT_MODE__", default_mode)
        .replace("__PAPER_BG__", PAPER_BG)
        .replace("__INK__", INK)
        .replace("__GRID_COLOR__", GRID_COLOR)
        .replace("__AXIS_LINE__", AXIS_LINE);

    ToggleChrome {
        style,
        button,
        script,
    }
}

/// The fixed-position qsv/datHere logo (bottom-right) for `viz smart` HTML pages, linking to
/// the qsv site. Two theme variants are embedded as base64 PNG data URIs (so the page stays
/// self-contained) and CSS-swapped via the `body.qsv-dark` class the toggle already manages: the
/// dark-text logo shows in light mode, the light-text (dark-background) logo in dark mode. The
/// accompanying CSS lives inline in `smart_html_page`'s `<style>`.
fn logo_markup() -> String {
    use base64_simd::STANDARD as BASE64;
    let light = BASE64.encode_to_string(include_bytes!("assets/qsv_logo_light.png"));
    let dark = BASE64.encode_to_string(include_bytes!("assets/qsv_logo_dark.png"));
    format!(
        r#"<a id="qsv-logo" href="https://qsv.dathere.com/" target="_blank" rel="noopener" aria-label="qsv by datHere"><img class="qsv-logo-light" alt="qsv by datHere" src="data:image/png;base64,{light}" /><img class="qsv-logo-dark" alt="qsv by datHere" src="data:image/png;base64,{dark}" /></a>"#
    )
}

/// CSS variables + toggle-button rule for `toggle_chrome`. Token placeholders are substituted at
/// runtime; kept as a raw string so the literal CSS braces stay intact.
const STYLE_TEMPLATE: &str = r#"  :root { --qsv-page-bg: __LIGHT_BG__; --qsv-page-ink: __LIGHT_INK__; --qsv-geo-meta: #4b5563; }
  body.qsv-dark { --qsv-page-bg: #111111; --qsv-page-ink: #f2f5fa; --qsv-geo-meta: #9aa4b2; }
  #qsv-theme-toggle { position: fixed; top: 12px; right: 12px; z-index: 1000; font: 13px __FONT_FAMILY__; padding: 6px 12px; border-radius: 6px; border: 1px solid var(--qsv-page-ink); background: var(--qsv-page-bg); color: var(--qsv-page-ink); cursor: pointer; opacity: 0.85; }
  #qsv-theme-toggle:hover { opacity: 1; }"#;

/// The light/dark toggle `<script>` for `toggle_chrome`. Token placeholders are substituted at
/// runtime; kept as a raw string so the JS braces and regex backslashes need no escaping.
const SCRIPT_TEMPLATE: &str = r##"<script>
(function () {
  var themeDefaultMode = "__DEFAULT_MODE__";
  var DARK = { paper: "#111111", plot: "#111111", font: "#f2f5fa", grid: "#283442", line: "#506784", zero: "#283442", bg: "#111111" };
  var LIGHT = { paper: "__PAPER_BG__", plot: "__PAPER_BG__", font: "__INK__", grid: "__GRID_COLOR__", line: "__AXIS_LINE__", zero: "__GRID_COLOR__", bg: "__PAPER_BG__" };
  function isDark() {
    try {
      var saved = localStorage.getItem("qsv-viz-theme");
      if (saved === "dark") return true;
      if (saved === "light") return false;
    } catch (e) {}
    if (themeDefaultMode === "dark") return true;
    if (themeDefaultMode === "light") return false;
    return window.matchMedia && window.matchMedia("(prefers-color-scheme: dark)").matches;
  }
  function buildUpdate(gd, p) {
    var u = { "paper_bgcolor": p.paper, "font.color": p.font };
    var lay = gd.layout || {};
    var hasAxis = false;
    Object.keys(lay).forEach(function (k) {
      if (/^xaxis\d*$/.test(k) || /^yaxis\d*$/.test(k)) {
        hasAxis = true;
        u[k + ".gridcolor"] = p.grid;
        u[k + ".linecolor"] = p.line;
        u[k + ".zerolinecolor"] = p.zero;
        u[k + ".tickcolor"] = p.line;
      }
      if (/^geo\d*$/.test(k) || /^polar\d*$/.test(k) || /^scene\d*$/.test(k)) u[k + ".bgcolor"] = p.bg;
    });
    if (hasAxis) u["plot_bgcolor"] = p.plot;
    return u;
  }
  function apply(dark) {
    document.body.classList.toggle("qsv-dark", dark);
    var p = dark ? DARK : LIGHT;
    document.querySelectorAll(".js-plotly-plot").forEach(function (gd) {
      try { Plotly.relayout(gd, buildUpdate(gd, p)); } catch (e) {}
    });
  }
  function init() {
    var dark = isDark();
    apply(dark);
    var btn = document.getElementById("qsv-theme-toggle");
    if (btn) btn.addEventListener("click", function () {
      var nowDark = !document.body.classList.contains("qsv-dark");
      try { localStorage.setItem("qsv-viz-theme", nowDark ? "dark" : "light"); } catch (e) {}
      apply(nowDark);
    });
  }
  if (document.readyState === "loading")
    document.addEventListener("DOMContentLoaded", function () { setTimeout(init, 0); });
  else setTimeout(init, 0);
})();
</script>"##;

fn smart_html_page(
    title_text: &str,
    theme: Option<BuiltinTheme>,
    extra_style: &str,
    body: &str,
    show_heading: bool,
) -> String {
    let js = plotly_js_only();
    let title = html_escape(title_text);
    let ToggleChrome {
        style: toggle_style,
        button,
        script,
    } = toggle_chrome(theme);
    let logo = logo_markup();
    // The inline-div grid has no overall plot title (panels carry only their own), so it shows the
    // dashboard title as a page `<h1>`. The typed-`Plot` grid already bakes the dashboard title
    // into its layout (needed for static image export), so its wrapper suppresses the `<h1>` to
    // avoid rendering the title twice. The `<title>` (browser tab) is always set either way.
    let heading = if show_heading {
        format!("<h1 class=\"qsv-viz-title\">{title}</h1>")
    } else {
        String::new()
    };
    // A RAW-string template (actual newlines, not `\n` escapes) so rustfmt's `format_strings` can't
    // split an escape across a line wrap and corrupt the output — it once mangled `\n{script}` into
    // a stray `\` + `n` in every page. `format!` still doubles the literal CSS braces (`{{`/`}}`)
    // and substitutes each placeholder in a single pass. The `#qsv-logo` rules mirror the toggle's
    // fixed-position pattern (bottom-right) and CSS-swap the two logo variants off `body.qsv-dark`.
    format!(
        r#"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8" />
<meta name="viewport" content="width=device-width, initial-scale=1" />
<title>{title}</title>
{js}
<style>
  body {{ font-family: {FONT_FAMILY}; color: var(--qsv-page-ink); background: var(--qsv-page-bg); margin: 0; padding: 16px; }}
  h1.qsv-viz-title {{ font-size: 20px; font-weight: 600; text-align: center; margin: 8px 0 20px; }}
  .qsv-viz-geo-meta {{ font-size: 13px; color: var(--qsv-geo-meta); text-align: center; padding: 8px 4px 4px; }}
  #qsv-logo {{ position: fixed; bottom: 12px; right: 12px; z-index: 999; opacity: 0.85; line-height: 0; }}
  #qsv-logo:hover {{ opacity: 1; }}
  #qsv-logo img {{ height: 28px; width: auto; display: block; }}
  #qsv-logo .qsv-logo-dark {{ display: none; }}
  body.qsv-dark #qsv-logo .qsv-logo-light {{ display: none; }}
  body.qsv-dark #qsv-logo .qsv-logo-dark {{ display: block; }}
{toggle_style}
{extra_style}
</style>
</head>
<body>
{button}
{logo}
{heading}
{body}
{script}
</body>
</html>
"#
    )
}

/// The embedded plotly.js `<script>` tag for `viz smart` HTML pages, WITHOUT the MathJax
/// (tex-svg) bundle that `Plot::offline_js_sources` also embeds.
///
/// `offline_js_sources` emits two consecutive `<script>` tags — plotly.js (~4.4MB) first, then
/// the tex-svg MathJax bundle (~2.0MB). Smart dashboards only ever render plain-text titles and
/// labels (column names + stat-derived strings), never LaTeX `$...$`, and plotly guards its only
/// MathJax use behind `typeof MathJax != "undefined"`, so the bundle is dead weight here —
/// dropping it shrinks each self-contained dashboard by ~31% with no visual change.
///
/// Neither minified payload contains the literal `</script>`, so cutting after the first close
/// tag cleanly isolates the plotly.js tag. Guarded by a `Plotly` marker check (present only in
/// plotly.js, never in tex-svg): if a future plotly release reorders the two tags, we fall back
/// to the full bundle rather than risk stripping plotly.js itself.
fn plotly_js_only() -> String {
    const CLOSE: &str = "</script>";
    let full = Plot::offline_js_sources();
    if let Some(idx) = full.find(CLOSE) {
        let first = &full[..idx + CLOSE.len()];
        if first.contains("Plotly") {
            return first.to_string();
        }
    }
    // layout changed unexpectedly — keep the full (correct, if heavier) bundle.
    full
}

fn build_layout(
    args: &Args,
    default_x: Option<String>,
    default_y: Option<String>,
) -> plotly::Layout {
    let mut layout = plotly::Layout::new();
    if let Some(title) = &args.flag_title {
        layout = layout.title(Title::with_text(title));
    }
    // axis titles: prefer the explicit flag, else the resolved column label
    if let Some(t) = args.flag_x_title.clone().or(default_x) {
        layout = layout.x_axis(Axis::new().title(Title::with_text(t)));
    }
    if let Some(t) = args.flag_y_title.clone().or(default_y) {
        layout = layout.y_axis(Axis::new().title(Title::with_text(t)));
    }
    apply_theme(layout, args.theme())
}

fn output_plot(
    plot: &Plot,
    args: &Args,
    fmt: OutFormat,
    width: usize,
    height: usize,
    scale: f64,
) -> CliResult<()> {
    if matches!(fmt, OutFormat::Html) {
        output_html(plot, args)
    } else {
        output_image(plot, args, fmt, width, height, scale)
    }
}

fn output_html(plot: &Plot, args: &Args) -> CliResult<()> {
    match args.flag_output.as_deref() {
        Some(path) => plot.write_html(path),
        None => {
            let html = plot.to_html();
            std::io::stdout().write_all(html.as_bytes())?;
        },
    }
    if args.flag_open {
        match args.flag_output.as_deref() {
            Some(path) => open_path(path)?,
            None => plot.show(),
        }
    }
    Ok(())
}

// image formats (only reachable with the viz_static feature; guarded in run())
#[cfg(feature = "viz_static")]
fn output_image(
    plot: &Plot,
    args: &Args,
    fmt: OutFormat,
    width: usize,
    height: usize,
    scale: f64,
) -> CliResult<()> {
    let path = args
        .flag_output
        .as_deref()
        .expect("image format without --output should have been rejected");
    plot.write_image(path, image_format(fmt), width, height, scale)
        .map_err(image_export_err)?;
    if args.flag_open {
        open_path(path)?;
    }
    Ok(())
}

#[cfg(not(feature = "viz_static"))]
#[allow(clippy::unnecessary_wraps)]
fn output_image(
    _plot: &Plot,
    _args: &Args,
    _fmt: OutFormat,
    _width: usize,
    _height: usize,
    _scale: f64,
) -> CliResult<()> {
    unreachable!("image export is rejected in run() without the viz_static feature");
}

/// Render a pre-assembled Plotly JSON value (data + layout) to a static image, for `viz smart`
/// dashboards with > `MAX_SUBPLOTS` panels — whose layout carries `xaxis9+`/`yaxis9+` that the
/// typed `Plot` can't express. `StaticExporter::write_fig` accepts an arbitrary JSON value and
/// renders it through the same headless-browser backend as `Plot::write_image`.
#[cfg(feature = "viz_static")]
fn output_image_json(
    value: &serde_json::Value,
    args: &Args,
    fmt: OutFormat,
    width: usize,
    height: usize,
    scale: f64,
) -> CliResult<()> {
    use std::path::Path;

    use plotly::plotly_static::StaticExporterBuilder;

    let path = args
        .flag_output
        .as_deref()
        .expect("image format without --output should have been rejected");
    let mut exporter = StaticExporterBuilder::default()
        .build()
        .map_err(image_export_err)?;
    let result = exporter.write_fig(
        Path::new(path),
        value,
        image_format(fmt),
        width,
        height,
        scale,
    );
    // release the webdriver even if the render failed (a leaked session worsens later exports),
    // mirroring plotly's own `Plot::write_image`.
    exporter.close();
    result.map_err(image_export_err)?;
    if args.flag_open {
        open_path(path)?;
    }
    Ok(())
}

#[cfg(not(feature = "viz_static"))]
#[allow(clippy::unnecessary_wraps)]
fn output_image_json(
    _value: &serde_json::Value,
    _args: &Args,
    _fmt: OutFormat,
    _width: usize,
    _height: usize,
    _scale: f64,
) -> CliResult<()> {
    unreachable!("image export is rejected in run() without the viz_static feature");
}

/// Map a `viz` output format to plotly's `ImageFormat`. `Html` never reaches here (it's routed to
/// the HTML writers before any image export).
#[cfg(feature = "viz_static")]
fn image_format(fmt: OutFormat) -> plotly::ImageFormat {
    use plotly::ImageFormat;
    match fmt {
        OutFormat::Png => ImageFormat::PNG,
        OutFormat::Jpeg => ImageFormat::JPEG,
        OutFormat::Webp => ImageFormat::WEBP,
        OutFormat::Svg => ImageFormat::SVG,
        OutFormat::Pdf => ImageFormat::PDF,
        OutFormat::Html => unreachable!(),
    }
}

/// Shared error mapper for static image export failures, pointing users at the browser/webdriver
/// requirement (the most common cause).
#[cfg(feature = "viz_static")]
fn image_export_err(e: impl std::fmt::Display) -> crate::CliError {
    crate::CliError::Other(format!(
        "Static image export failed: {e}. A Chromium or Firefox browser must be installed and \
         available for plotly's webdriver-based export."
    ))
}

/// Open `path` in the user's default application, honoring the `BROWSER` environment variable
/// when set (via `opener::open_browser`).
fn open_path(path: &str) -> CliResult<()> {
    opener::open_browser(path)
        .map_err(|e| crate::CliError::Other(format!("Could not open '{path}': {e}")))
}

// ----- small helpers -----

fn cell_to_string(cell: Option<&[u8]>) -> String {
    cell.map(|b| String::from_utf8_lossy(b).into_owned())
        .unwrap_or_default()
}

fn parse_f64(cell: Option<&[u8]>) -> Option<f64> {
    let s = std::str::from_utf8(cell?).ok()?.trim();
    if s.is_empty() {
        return None;
    }
    s.parse::<f64>().ok()
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Agg {
    Sum,
    Mean,
    Count,
    Min,
    Max,
}

fn parse_agg(agg: Option<&str>) -> CliResult<Option<Agg>> {
    match agg {
        None => Ok(None),
        Some(s) => match s.to_ascii_lowercase().as_str() {
            "sum" => Ok(Some(Agg::Sum)),
            "mean" | "avg" | "average" => Ok(Some(Agg::Mean)),
            "count" => Ok(Some(Agg::Count)),
            "min" => Ok(Some(Agg::Min)),
            "max" => Ok(Some(Agg::Max)),
            other => {
                fail_incorrectusage_clierror!(
                    "Unknown --agg '{other}'. Use sum, mean, count, min, or max."
                )
            },
        },
    }
}

/// Best-practice auto-selection for a categorical hierarchy of `depth` levels: a **treemap** for
/// shallow hierarchies (≤2 levels), where area encodes value for accurate size comparison; a
/// **sunburst** for deeper ones (3+ levels), which emphasizes parent-child structure / path
/// tracing and keeps many small leaves legible as ring segments.
fn auto_hierarchy_style(depth: usize) -> HierStyle {
    if depth >= 3 {
        HierStyle::Sunburst
    } else {
        HierStyle::Treemap
    }
}

/// Resolve the `--hierarchy-style` flag (`auto` | `treemap` | `sunburst`; default `auto`) to a
/// concrete chart for a `depth`-level hierarchy. `auto` applies `auto_hierarchy_style`.
fn resolve_hierarchy_style(flag: Option<&str>, depth: usize) -> CliResult<HierStyle> {
    match flag.map(|s| s.to_ascii_lowercase()).as_deref() {
        None | Some("auto") => Ok(auto_hierarchy_style(depth)),
        Some("treemap") => Ok(HierStyle::Treemap),
        Some("sunburst") => Ok(HierStyle::Sunburst),
        Some(other) => fail_incorrectusage_clierror!(
            "Unknown --hierarchy-style '{other}'. Use auto, treemap, or sunburst."
        ),
    }
}

/// Resolved `--log-scale` mode for `viz smart` frequency bar panels.
#[derive(Clone, Copy, PartialEq, Eq)]
enum LogScale {
    /// Per-panel: log y-axis only when the bar distribution's dynamic range is high.
    Auto,
    /// Force a log y-axis on every frequency bar panel.
    On,
    /// Never use a log y-axis (linear axes only).
    Off,
}

/// Parse the `--log-scale` flag (defaults to `auto`).
fn parse_log_scale(mode: &str) -> CliResult<LogScale> {
    match mode.to_ascii_lowercase().as_str() {
        "auto" => Ok(LogScale::Auto),
        "on" | "true" => Ok(LogScale::On),
        "off" | "false" => Ok(LogScale::Off),
        other => {
            fail_incorrectusage_clierror!("Unknown --log-scale '{other}'. Use auto, on, or off.")
        },
    }
}

/// Decide whether a frequency bar panel should use a logarithmic y-axis, given its resolved
/// `--log-scale` mode and the panel's bar counts. `On` always logs (when there are 2+ positive
/// bars to compare), `Off` never does, and `Auto` logs only when the tallest positive bar is at
/// least `LOG_SCALE_MIN_RATIO`x the shortest positive bar (the high-dynamic-range case where a
/// dominating "(NULL)"/"Other" bucket would otherwise flatten the real categories).
fn freq_panel_logs(mode: LogScale, counts: &[u64]) -> bool {
    if mode == LogScale::Off {
        return false;
    }
    let mut min_pos = u64::MAX;
    let mut max_pos = 0_u64;
    let mut n_pos = 0_usize;
    for &c in counts {
        if c > 0 {
            n_pos += 1;
            min_pos = min_pos.min(c);
            max_pos = max_pos.max(c);
        }
    }
    match mode {
        // forcing log still needs 2+ positive bars to be meaningful (and a positive max)
        LogScale::On => n_pos >= 2 && max_pos > 0,
        LogScale::Auto => n_pos >= 3 && (max_pos as f64) >= (min_pos as f64) * LOG_SCALE_MIN_RATIO,
        LogScale::Off => false,
    }
}

/// Whether a panel will render with a logarithmic y-axis under the resolved `--log-scale` mode.
/// Only frequency bar panels can be log; every other panel kind is always linear. Used both to
/// gate the panel's y-axis title cue and to size the dashboard's left margin to fit it.
fn panel_is_log(panel: &Panel, freq: &FreqMap, log_scale: LogScale) -> bool {
    match panel.kind {
        PanelKind::FreqBar { idx } => {
            let counts: Vec<u64> = freq
                .get(&idx)
                .map(|bars| bars.iter().map(|b| b.count).collect())
                .unwrap_or_default();
            freq_panel_logs(log_scale, &counts)
        },
        _ => false,
    }
}

/// Parse the `--box-points` flag controlling which sample points are drawn alongside a
/// box plot. Defaults to `outliers` (only the points beyond the Tukey 1.5*IQR fences).
fn parse_box_points(points: Option<&str>) -> CliResult<BoxPoints> {
    match points {
        None => Ok(BoxPoints::Outliers),
        Some(s) => match s.to_ascii_lowercase().as_str() {
            "outliers" => Ok(BoxPoints::Outliers),
            "all" => Ok(BoxPoints::All),
            "suspected" | "suspectedoutliers" => Ok(BoxPoints::SuspectedOutliers),
            "none" | "false" => Ok(BoxPoints::False),
            other => {
                fail_incorrectusage_clierror!(
                    "Unknown --box-points '{other}'. Use outliers, all, suspected, or none."
                )
            },
        },
    }
}

/// Resolve the box-overlay mode for a `viz smart` box panel. An explicit user `--box-points` mode
/// always wins (an explicit `none` means "no overlay" — keep the cheap cache-only quartile box, so
/// this returns `None`). Without an explicit mode, a size-based heuristic on the dataset row count
/// picks the mode: all points for small data, just outliers for medium, and none for large data
/// (returning `None` so no raw-values pass is done and the box stays a cache-only summary).
/// A `None` return means "don't build a raw box for this column".
fn smart_box_points(explicit: Option<&BoxPoints>, nrows: u64) -> Option<BoxPoints> {
    if let Some(mode) = explicit {
        return if matches!(mode, BoxPoints::False) {
            None
        } else {
            Some(mode.clone())
        };
    }
    if nrows <= SMART_BOX_ALL_MAX {
        Some(BoxPoints::All)
    } else if nrows <= SMART_BOX_OUTLIERS_MAX {
        Some(BoxPoints::Outliers)
    } else {
        None
    }
}

/// Aggregate y values by x category, preserving first-seen (input) order so that
/// downstream numeric ordering is not lost to lexicographic sorting (e.g. 1, 10, 2).
fn aggregate(xs: Vec<String>, ys: Vec<f64>, agg: Agg) -> (Vec<String>, Vec<f64>) {
    let mut order: Vec<String> = Vec::new();
    let mut acc: HashMap<String, (f64, f64)> = HashMap::new(); // x -> (running, count)
    for (x, y) in xs.into_iter().zip(ys) {
        let e = acc.entry(x.clone()).or_insert_with(|| {
            order.push(x);
            match agg {
                Agg::Min => (f64::INFINITY, 0.0),
                Agg::Max => (f64::NEG_INFINITY, 0.0),
                _ => (0.0, 0.0),
            }
        });
        e.1 += 1.0;
        match agg {
            Agg::Sum | Agg::Mean => e.0 += y,
            Agg::Count => {},
            Agg::Min => e.0 = e.0.min(y),
            Agg::Max => e.0 = e.0.max(y),
        }
    }
    let mut out_x = Vec::with_capacity(order.len());
    let mut out_y = Vec::with_capacity(order.len());
    for x in order {
        let (running, count) = acc[&x];
        out_x.push(x);
        out_y.push(match agg {
            Agg::Sum | Agg::Min | Agg::Max => running,
            Agg::Mean => {
                if count == 0.0 {
                    0.0
                } else {
                    running / count
                }
            },
            Agg::Count => count,
        });
    }
    (out_x, out_y)
}

/// For line charts, order points by their x value so the connecting line is drawn in the
/// correct sequence: numeric x is sorted numerically (fixing 1, 10, 2), while categorical x
/// preserves input order.
fn sort_line_xy(xs: Vec<String>, ys: Vec<f64>) -> (Vec<String>, Vec<f64>) {
    if xs.is_empty() || !xs.iter().all(|s| s.trim().parse::<f64>().is_ok()) {
        return (xs, ys);
    }
    let mut pairs: Vec<(f64, String, f64)> = xs
        .into_iter()
        .zip(ys)
        .map(|(x, y)| (x.trim().parse::<f64>().unwrap_or(f64::NAN), x, y))
        .collect();
    pairs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    let mut out_x = Vec::with_capacity(pairs.len());
    let mut out_y = Vec::with_capacity(pairs.len());
    for (_, x, y) in pairs {
        out_x.push(x);
        out_y.push(y);
    }
    (out_x, out_y)
}

// ===== viz smart: cache-driven auto-dashboard =====

/// A single dashboard panel: a column and the chart chosen for it.
struct Panel {
    name:     String,
    kind:     PanelKind,
    /// Reverse-geocoded spatial-extent metadata for a `Map`/`Geo` panel (the 4 bounding-box
    /// corners + center, plus a consolidated jurisdiction summary). `None` for non-map panels,
    /// when geocoding was skipped (e.g. antimeridian-spanning data), or when the lookup failed
    /// (offline/missing index) — the map then renders without the overlay.
    #[cfg(feature = "geocode")]
    geo_meta: Option<GeoMeta>,
}

impl Panel {
    /// Construct a panel with no geo metadata (the common case for non-map panels).
    fn new(name: String, kind: PanelKind) -> Self {
        Panel {
            name,
            kind,
            #[cfg(feature = "geocode")]
            geo_meta: None,
        }
    }
}

/// An observed min/max lat/lon bounding box. For a `viz smart` map this is the CORE extent — the
/// box of the non-outlier points (geographic outliers are excluded so a few strays don't inflate
/// it); `map_extent` itself just computes min/max over whatever coordinates it's given.
#[cfg(feature = "geocode")]
#[derive(Clone, Copy)]
struct MapExtent {
    min_lat: f64,
    max_lat: f64,
    min_lon: f64,
    max_lon: f64,
}

/// One reverse-geocoded point of a map's spatial extent (a corner or the center).
#[cfg(feature = "geocode")]
struct GeoPoint {
    /// Short positional tag: "NW", "NE", "SW", "SE", or "Center".
    tag:   &'static str,
    lat:   f64,
    lon:   f64,
    /// The reverse-geocoded location, or `None` when no city matched (e.g. over open water).
    label: Option<crate::cmd::geocode::GeoLabel>,
}

/// Reverse-geocoded spatial-extent metadata attached to a `Map`/`Geo` panel.
#[cfg(feature = "geocode")]
struct GeoMeta {
    /// The CORE extent (non-outlier bounding box) — drives the (filled) overlay box, corner
    /// markers, and the tight default framing.
    extent:      MapExtent,
    /// The FULL extent (core + all geographic outliers). `Some` only when there are outliers (and
    /// the box doesn't wrap the antimeridian); drawn as a second, no-fill dotted box so the
    /// strays' span is legible alongside the core box.
    full_extent: Option<MapExtent>,
    /// The 4 core-extent corners + center, in order NW, NE, SW, SE, Center.
    points:      Vec<GeoPoint>,
    /// Consolidated one-line jurisdiction summary (e.g. "New York & New Jersey, United States"),
    /// with an outlier call-out appended when there are geographic outliers (e.g.
    /// "… — 3 outliers (Pennsylvania)"). Empty when no core point resolved to a city.
    summary:     String,
}

/// Which hierarchical part-to-whole chart a `Hierarchy` panel renders as. Auto-selected by
/// depth (shallow → `Treemap` for accurate size comparison, deep → `Sunburst` for tracing
/// structural paths), or forced via the standalone subcommands / `--hierarchy-style`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum HierStyle {
    Treemap,
    Sunburst,
}

enum PanelKind {
    /// Box plot drawn from precomputed quartiles (no raw data).
    BoxStats {
        q1:     f64,
        median: f64,
        q3:     f64,
        lower:  Option<f64>,
        upper:  Option<f64>,
        mean:   Option<f64>,
    },
    /// Box plot drawn from the raw column values (via `--box-points`, or the size-based heuristic),
    /// so plotly can render true Tukey whiskers and overlay the sample points. `idx` is the source
    /// column index; the (downsampled) values are gathered in the same batched pass as `Histogram`
    /// and looked up by `idx` at render time. `points` is this panel's resolved overlay mode.
    BoxRaw { idx: usize, points: BoxPoints },
    /// Cache-quartile box for a large (> `SMART_BOX_OUTLIERS_MAX` row) column that HAS Tukey
    /// outliers. The box is drawn from the precomputed q1/median/q3 (like `BoxStats`), but its
    /// whiskers end at the observed in-fence extremes and the out-of-fence outlier values are
    /// overlaid as native box points (plotly does NOT recompute the box). `idx` keys the
    /// post-pass `OutlierStats` side table (whisker ends + the capped outlier values); the box
    /// stats are carried on the panel and `fence_low`/`fence_high` are the Tukey inner fences used
    /// to filter the single collection pass.
    BoxOutliers {
        idx:        usize,
        q1:         f64,
        median:     f64,
        q3:         f64,
        mean:       Option<f64>,
        fence_low:  f64,
        fence_high: f64,
    },
    /// Frequency bar chart; `idx` is the source column index.
    FreqBar { idx: usize },
    /// Line chart of a numeric column over a date/datetime column, sorted chronologically.
    /// Carries the precomputed (already date-sorted) x date strings and y values so the render
    /// loop stays a pure assembly step.
    TimeSeries {
        y_label: String,
        xs:      Vec<String>,
        ys:      Vec<f64>,
    },
    /// Pearson correlation heatmap over the dataset's numeric columns. Carries precomputed
    /// data (labels + matrix) so the render loop stays a pure assembly step.
    CorrHeatmap {
        labels: Vec<String>,
        matrix: Vec<Vec<f64>>,
    },
    /// Scatter of the most strongly correlated numeric pair — a drill-down for the correlation
    /// heatmap. Carries the two columns' precomputed, row-aligned values.
    ScatterPair { xs: Vec<f64>, ys: Vec<f64> },
    /// 2D density contour of the most strongly correlated numeric pair — used INSTEAD of
    /// `ScatterPair` for large datasets (>= `SMART_CONTOUR_MIN_POINTS`), where a scatter overplots.
    /// Carries the precomputed bin-center axes and count grid so the render loop stays pure.
    ContourPair {
        x: Vec<f64>,
        y: Vec<f64>,
        z: Vec<Vec<f64>>,
    },
    /// 3D scatter of the three numeric columns that form the strongest-correlation triple — a
    /// spatial drill-down for the correlation heatmap. Carries the three columns' precomputed,
    /// row-aligned values plus their axis labels. Like `Map`, a 3D `scene` doesn't compose with the
    /// typed x/y subplot grid, so a dashboard containing this panel always renders via the inline
    /// path.
    Scatter3D {
        xs:     Vec<f64>,
        ys:     Vec<f64>,
        zs:     Vec<f64>,
        labels: (String, String, String),
    },
    /// Histogram of a continuous numeric column, chosen INSTEAD of a box plot when moarstats
    /// flagged the column as bimodal/multimodal (a box plot would hide the multiple peaks).
    /// `idx` is the source column index; the (downsampled) values are gathered in a single
    /// batched pass and looked up by `idx` at render time — same side-table pattern as `FreqBar`.
    Histogram { idx: usize },
    /// Geographic point map over an auto-detected latitude/longitude column pair. Carries the
    /// precomputed, row-aligned coordinates (already downsampled to `MAX_SMART_POINTS`). `density`
    /// requests a heatmap render instead of discrete markers when the source had many rows. Mapbox
    /// subplots don't compose with the typed x/y subplot grid, so a dashboard containing this panel
    /// always renders via the inline path.
    /// `outlier_lats`/`outlier_lons` carry the geographic outliers (far from the cluster centroid),
    /// drawn as a distinct marker trace on top of the core points.
    Map {
        lats:         Vec<f64>,
        lons:         Vec<f64>,
        density:      bool,
        outlier_lats: Vec<f64>,
        outlier_lons: Vec<f64>,
    },
    /// Geographic point map drawn on a `ScatterGeo` projection basemap (coastlines/land/countries,
    /// no network tiles) instead of mapbox — used for `viz smart` when the coordinates span a
    /// continental/global extent. Like `Map`, the `geo` subplot doesn't compose with the typed x/y
    /// grid, so a dashboard containing this panel always renders via the inline path.
    /// `outlier_lats`/`outlier_lons` carry the geographic outliers (see `Map`).
    Geo {
        lats:         Vec<f64>,
        lons:         Vec<f64>,
        outlier_lats: Vec<f64>,
        outlier_lons: Vec<f64>,
    },
    /// Filled-region `Choropleth` aggregate drawn on the projection `geo` subplot — added beside
    /// the point Map/Geo panel when geocode resolves the coordinates to 2+ distinct countries,
    /// coloring each country by its row count. Carries precomputed `locations` (ISO-3 codes)
    /// and `z` (counts). Like `Geo`, the `geo` subplot doesn't compose with the typed x/y grid,
    /// so a dashboard containing this panel always renders via the inline path.
    Choropleth {
        locations:     Vec<String>,
        z:             Vec<f64>,
        location_mode: LocationMode,
    },
    /// Categorical part-to-whole hierarchy (`Treemap` or `Sunburst`, per `style`) over 2–3
    /// nested low-cardinality dimensions. Carries the fully precomputed flat plotly arrays
    /// (`labels`/`parents`/`values` keyed by path-joined `ids`) so the render loop stays a pure
    /// assembly step. Like `Map`/`Scatter3D`, a domain-based trace doesn't compose with the typed
    /// x/y subplot grid, so a dashboard containing this panel always renders via the inline path.
    Hierarchy {
        style:   HierStyle,
        labels:  Vec<String>,
        parents: Vec<String>,
        values:  Vec<f64>,
        ids:     Vec<String>,
    },
}

/// Tukey inner fences (lower = q1 - 1.5*IQR, upper = q3 + 1.5*IQR) for a numeric column, used to
/// identify outliers for a large-dataset box panel. Prefers the values precomputed in the stats
/// cache; falls back to computing them from q1/q3/IQR. Returns `None` when the quartiles are
/// unavailable or the IQR is non-positive (a constant / near-constant column, where every value
/// would falsely flag as an outlier).
fn box_fences(s: &crate::cmd::stats::StatsData) -> Option<(f64, f64)> {
    let (q1, q3) = (s.q1?, s.q3?);
    let iqr = s.iqr.unwrap_or(q3 - q1);
    if iqr <= 0.0 {
        return None;
    }
    let lo = s.lower_inner_fence.unwrap_or(q1 - 1.5 * iqr);
    let hi = s.upper_inner_fence.unwrap_or(q3 + 1.5 * iqr);
    Some((lo, hi))
}

/// Tukey IQR multiplier applied to the distance-from-centroid distribution when flagging
/// geographic outliers. 3.0 is the "far out" (extreme-outlier) fence, deliberately stricter than
/// the 1.5 inner fence so only points GENUINELY far from the cluster flag — a broad single-region
/// distribution's edge points stay core rather than swamping the map.
const GEO_OUTLIER_DIST_IQR_MULT: f64 = 3.0;

/// Partition row-aligned (lat, lon) pairs into a core set and a geographic-outlier set by distance
/// from the cluster centroid: a point is an outlier when its distance from the (robust, per-axis
/// median) centroid exceeds the Tukey far-out fence `q3 + 3*IQR` of all points' distances. Distance
/// is an equirectangular approximation with longitude scaled by `cos(centroid_lat)` so it's roughly
/// isotropic in degrees. Unlike a per-axis percentile, this flags only points genuinely far from
/// the bulk (true strays), not the distribution's tails. When the distance IQR is zero (a
/// point-mass core, e.g. many duplicate coordinates) the fence falls back to the point-mass band
/// itself (`d > q3`), so a lone far stray is flagged even with very few duplicates; with too few
/// points (< 4) or a fully degenerate (all-identical) distance spread, nothing is flagged and the
/// whole set is core. Returns `(core_lats, core_lons, outlier_lats, outlier_lons)`.
fn partition_geo_outliers(lats: &[f64], lons: &[f64]) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let n = lats.len();
    if n < 4 {
        // too few points to characterize a cluster -> everything is core, zero outliers
        return (lats.to_vec(), lons.to_vec(), Vec::new(), Vec::new());
    }
    // robust centroid: per-axis medians (so a few strays don't drag the center toward them)
    let mut sorted_lats = lats.to_vec();
    let mut sorted_lons = lons.to_vec();
    sorted_lats.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    sorted_lons.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let c_lat = sorted_quantile(&sorted_lats, 0.5);
    let c_lon = sorted_quantile(&sorted_lons, 0.5);
    // scale longitude so a degree of lon counts like a degree of lat at this latitude
    let lon_scale = c_lat.to_radians().cos().abs().max(0.01);

    let dist = |lat: f64, lon: f64| -> f64 {
        let dlat = lat - c_lat;
        let dlon = (lon - c_lon) * lon_scale;
        (dlat * dlat + dlon * dlon).sqrt()
    };

    let dists: Vec<f64> = lats
        .iter()
        .zip(lons)
        .map(|(&la, &lo)| dist(la, lo))
        .collect();
    let mut sorted_dists = dists.clone();
    sorted_dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let q1 = sorted_quantile(&sorted_dists, 0.25);
    let q3 = sorted_quantile(&sorted_dists, 0.75);
    let iqr = q3 - q1;
    // a zero IQR means the middle 50% of distances are identical — a point-mass core, common when
    // many rows share the exact same coordinate (duplicate geocodes). The Tukey fence would then
    // collapse and a scale-based fallback (mean/std) is itself inflated by the candidate stray, so
    // it misses a lone stray when there are few duplicates. Instead flag anything beyond the
    // point-mass band itself (`d > q3`): `q3` is robust (unmoved by the stray), so a single far
    // stray is caught regardless of how few duplicates form the mass. A truly degenerate
    // all-identical set has `q3 == 0` and nothing exceeds it, so nothing is flagged.
    let fence = if iqr > 0.0 {
        q3 + GEO_OUTLIER_DIST_IQR_MULT * iqr
    } else {
        q3
    };

    let (mut core_lats, mut core_lons) = (Vec::with_capacity(n), Vec::with_capacity(n));
    let (mut out_lats, mut out_lons) = (Vec::new(), Vec::new());
    for ((&lat, &lon), &d) in lats.iter().zip(lons).zip(&dists) {
        if d > fence {
            out_lats.push(lat);
            out_lons.push(lon);
        } else {
            core_lats.push(lat);
            core_lons.push(lon);
        }
    }
    (core_lats, core_lons, out_lats, out_lons)
}

/// The panel a column is routed to once a describegpt Data Dictionary's semantic signals are
/// folded into `viz smart`. Distilled from `role` / `concept` / `content_type` by
/// `derive_semantics`. `Defer` (no usable signal, or any column absent from the dictionary) falls
/// back to the statistical `classify`, so a dashboard built without `--dictionary` is unchanged.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
enum Route {
    /// No semantic signal -> defer to the statistical heuristic (`classify`).
    #[default]
    Defer,
    /// A categorical/code field -> frequency bar, even when stored as a numeric code or when its
    /// cardinality exceeds the usual bar threshold. This is what lets administrative codes stored
    /// as numbers (ward, police_zone, census_tract, ...) become bars instead of being misread as
    /// continuous measures.
    Dimension,
    /// A genuine continuous measure -> box / histogram / correlation / time-series y. This is the
    /// positive "this IS a measure" signal the statistical heuristic cannot express.
    Measure,
    /// A date/datetime field -> handled by the time-series overview panel; no per-column
    /// distribution panel. Stops high-cardinality timestamps from becoming top-N frequency bars.
    Temporal,
    /// A latitude/longitude coordinate -> consumed by the map panel; never boxed/barred.
    MapCoord,
    /// An identifier / PII / free-text field -> not a meaningful distribution to chart; skipped.
    Skip,
}

/// A column's resolved charting verdict: where it goes (`route`), how to aggregate it over time
/// (`agg`: `Some(Sum)` for additive counts/amounts, `Some(Mean)` for rates/ratios, `None` for the
/// trend panel's default raw value), its dictionary `concept` (kept for map-pairing / dedup / the
/// guardrail), and its human `label` (kept for panel titles). Built per column by
/// `derive_semantics`; `Agg` is the existing chart-aggregation enum, reused here.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
struct ColSemantics {
    route:   Route,
    agg:     Option<Agg>,
    concept: String,
    label:   String,
}

/// One column's semantic signals parsed from a describegpt Data Dictionary.
#[derive(Clone, Debug, Default)]
struct DictRow {
    content_type: String,
    role:         String,
    concept:      String,
    label:        String,
}

/// A parsed describegpt Data Dictionary: per-column semantic rows keyed by field name, plus the
/// dataset `grain` ("one row = one X"). Drives `viz smart` semantic routing.
#[derive(Clone, Debug, Default)]
struct DictData {
    rows:  HashMap<String, DictRow>,
    grain: Option<String>,
}

/// Map a `concept` token (the most specific, highest-confidence signal) to a route. Returns `None`
/// for an unrecognized namespace so the caller falls through to `role`. Concepts are namespaced
/// `ns.leaf` (see `describegpt::dictionary::CONCEPT_VOCAB`).
fn route_from_concept(concept: &str) -> Option<(Route, Option<Agg>)> {
    let (ns, leaf) = concept.split_once('.').unwrap_or((concept, ""));
    let routed = match ns {
        "geo" => match leaf {
            "latitude" | "longitude" | "coordinate_pair" => (Route::MapCoord, None),
            // geo *keys* (zip, census_tract, city, state, country, street_address) name a place;
            // they are dimensions to bar, never continuous measures. This is the signal that fixes
            // census_tract even when describegpt defaulted its numeric `role` to `measure`.
            _ => (Route::Dimension, None),
        },
        "time" => (Route::Temporal, None),
        "id" | "pii" => (Route::Skip, None),
        "org" | "category" | "nyc" => (Route::Dimension, None),
        "measure" => match leaf {
            // ratios/percentages average over time; counts/amounts are additive.
            "ratio" => (Route::Measure, Some(Agg::Mean)),
            _ => (Route::Measure, Some(Agg::Sum)),
        },
        // unknown namespace (or the bare "unknown" token) -> defer to role
        _ => return None,
    };
    Some(routed)
}

/// Map a `role` token (the coarse fallback when no concept resolves) to a route.
fn route_from_role(role: &str) -> Option<(Route, Option<Agg>)> {
    let routed = match role {
        "timestamp" => (Route::Temporal, None),
        "identifier" => (Route::Skip, None),
        "dimension" => (Route::Dimension, None),
        // additive by default; a measure.* concept refines this to Mean for ratios.
        "measure" => (Route::Measure, Some(Agg::Sum)),
        _ => return None,
    };
    Some(routed)
}

/// Map a describegpt `content_type` token to a route (the legacy fallback). Date/datetime/duration
/// tokens may carry a `:<suffix>`, so match on the base token before the first `:`. Unknown/empty
/// yields `Defer` so the column falls back to statistics.
fn route_from_content_type(content_type: &str) -> (Route, Option<Agg>) {
    let base = content_type
        .split_once(':')
        .map_or(content_type, |(b, _)| b)
        .trim();
    let route = match base {
        "category" | "state" | "state_abbr" | "country" | "country_code" | "currency_code"
        | "mime_type" | "color_hex" | "industry" | "job_title" | "profession" | "time_zone" => {
            Route::Dimension
        },
        "latitude" | "longitude" => Route::MapCoord,
        "date" | "datetime" | "time" | "duration" => Route::Temporal,
        "unknown" | "" => Route::Defer,
        // identifier / PII / address / technical / free-text -> not a meaningful distribution
        _ => Route::Skip,
    };
    (route, None)
}

/// Defend against describegpt's numeric `role` defaulting to `measure` (see
/// `describegpt::dictionary::coerce_role_concept`): downgrade a `Measure` verdict to a `Dimension`
/// (bar) when the column looks like an integer *code* — few distinct values spread over many rows
/// — rather than a quantity. Only ever touches `Measure`; an explicit `measure.*` concept, a
/// non-integer, or a (near-)unique column is trusted as-is. Reuses `CATEGORICAL_MAX_CARDINALITY`
/// so "is it categorical?" means the same here as in `classify`.
fn guardrail(mut sem: ColSemantics, s: &crate::cmd::stats::StatsData) -> ColSemantics {
    if sem.route != Route::Measure
        || sem.concept.starts_with("measure.")
        || s.r#type.as_str() != "Integer"
    {
        return sem;
    }
    let ratio = s.uniqueness_ratio;
    if ratio.is_some_and(|r| r > 0.95) {
        return sem; // genuinely (near-)continuous, e.g. a monetary integer
    }
    if s.cardinality <= CATEGORICAL_MAX_CARDINALITY && ratio.is_some_and(|r| r < 0.05) {
        sem.route = Route::Dimension;
    }
    sem
}

/// Distill a column's `StatsData` + optional dictionary `row` into one charting verdict.
///
/// Precedence: **concept -> role -> content_type -> statistics**. `concept` is the most specific,
/// highest-confidence signal (it fixes a numeric admin code that describegpt defaulted to
/// `role: measure`); `role` is the coarse fallback; `content_type` is the legacy mapping; and a
/// column with no usable signal `Defer`s to `classify`. A guardrail wraps every `Measure` verdict.
/// With no dictionary `row`, returns the default (`Defer`) so stats-only behavior is unchanged.
fn derive_semantics(s: &crate::cmd::stats::StatsData, row: Option<&DictRow>) -> ColSemantics {
    let Some(row) = row else {
        return ColSemantics::default();
    };
    let label = row.label.trim().to_string();
    let concept = row.concept.trim();
    let make = |route: Route, agg: Option<Agg>| {
        guardrail(
            ColSemantics {
                route,
                agg,
                concept: concept.to_string(),
                label: label.clone(),
            },
            s,
        )
    };

    // 1. concept — most specific
    if !concept.is_empty()
        && concept != "unknown"
        && let Some((route, agg)) = route_from_concept(concept)
    {
        return make(route, agg);
    }
    // 2. role — coarse fallback
    if let Some((route, agg)) = route_from_role(row.role.trim()) {
        return make(route, agg);
    }
    // 3. content_type — legacy mapping
    let ct = row.content_type.trim();
    if !ct.is_empty() && ct != "unknown" {
        let (route, agg) = route_from_content_type(ct);
        return make(route, agg);
    }
    // 4. statistics floor — carry the label so panel titles still benefit
    ColSemantics {
        route: Route::Defer,
        agg: None,
        concept: String::new(),
        label,
    }
}

/// The continuous-measure arm of `classify`: a box plot from precomputed quartiles, or a histogram
/// when moarstats flagged the column bimodal/multimodal. Shared by `classify` (stats path) and
/// `classify_with_semantics` (a dictionary `measure` verdict) so a measure is charted the same way
/// however it was identified. Returns `None` when the column lacks quartiles.
fn classify_measure(idx: usize, s: &crate::cmd::stats::StatsData) -> Option<PanelKind> {
    let (Some(q1), Some(median), Some(q3)) = (s.q1, s.q2_median, s.q3) else {
        return None;
    };
    if s.cardinality <= 1 {
        return None;
    }
    // a box plot hides multiple peaks; a histogram tells the truth — but only flag bimodal when
    // Sarle's BC clears the threshold AND the distribution is PLATYKURTIC (negative excess
    // kurtosis). The kurtosis guard rejects BC's well-known false positive on heavily skewed
    // UNIMODAL data: a long tail inflates BC through skewness, yet such columns are leptokurtic and
    // are far better shown as a box with its outlier points than as a one-tall-bar histogram. A
    // genuine two-peaked column is flat-topped (negative excess kurtosis), so it still becomes a
    // histogram. `bimodality_coefficient`/`kurtosis` come from moarstats (`--smarter`) or
    // `enrich_bimodality` (plain smart); when either is absent (e.g. too few rows) it's a box plot.
    if s.bimodality_coefficient
        .is_some_and(|bc| bc >= BIMODALITY_COEFFICIENT_THRESHOLD)
        && s.kurtosis.is_some_and(|k| k < 0.0)
    {
        return Some(PanelKind::Histogram { idx });
    }
    // Observed min/max as whisker endpoints (NOT Tukey fences, which are computed thresholds that
    // need not be observed values) — honest for a precomputed, no-rescan box.
    let lower = s.min.as_deref().and_then(|v| v.trim().parse::<f64>().ok());
    let upper = s.max.as_deref().and_then(|v| v.trim().parse::<f64>().ok());
    Some(PanelKind::BoxStats {
        q1,
        median,
        q3,
        lower,
        upper,
        mean: s.mean,
    })
}

/// Populate `bimodality_coefficient` for continuous-numeric columns that don't already have one —
/// the plain `viz smart` path, where the stats cache carries `skewness` but not the moarstats-only
/// `kurtosis`/`bimodality_coefficient`. Without it a bimodal column (two separated peaks) renders
/// as a box plot whose median can sit in the empty gap BETWEEN the peaks — actively misleading;
/// `classify_measure` upgrades a flagged column to a histogram instead, which shows the peaks.
///
/// Sarle's bimodality coefficient is computed EXACTLY as moarstats does — `BC = (skewness² + 1) /
/// (kurtosis + 3)` with the same qsv-stats sample excess-kurtosis definition and the SAME cached
/// `mean`/`variance`/`skewness` inputs — so a column classifies identically with or without
/// `--smarter`. The 4th central moment is accumulated in a single streaming pass (O(1) memory per
/// column, no value buffering) using each column's cached mean. Columns moarstats already enriched
/// (BC present) are skipped, so this is a no-op — with no data pass at all — under `--smarter` or
/// when no column qualifies.
fn enrich_bimodality(args: &Args, stats: &mut [crate::cmd::stats::StatsData]) -> CliResult<()> {
    // candidates: columns that would become a box plot (continuous numeric with quartiles) and lack
    // a bimodality coefficient. Mirrors classify()/classify_measure()'s box-vs-bar conditions so we
    // don't pay for columns that chart as frequency bars or are skipped as ID-like.
    let candidates: Vec<usize> = stats
        .iter()
        .enumerate()
        .filter(|(_, s)| {
            let near_unique = s.uniqueness_ratio.is_some_and(|r| r > 0.95);
            let low_card = s.cardinality <= CATEGORICAL_MAX_CARDINALITY && !near_unique;
            s.bimodality_coefficient.is_none()
                && matches!(s.r#type.as_str(), "Integer" | "Float")
                && s.cardinality > 1
                && !low_card
                && s.q1.is_some()
                && s.q2_median.is_some()
                && s.q3.is_some()
                && s.skewness.is_some()
                && s.mean.is_some()
                && s.variance.is_some_and(|v| v > 0.0)
        })
        .map(|(i, _)| i)
        .collect();
    if candidates.is_empty() {
        return Ok(());
    }

    let means: Vec<f64> = candidates
        .iter()
        .map(|&i| stats[i].mean.unwrap_or_default())
        .collect();
    let mut counts = vec![0_u64; candidates.len()];
    let mut sum4 = vec![0.0_f64; candidates.len()]; // Σ(x - mean)⁴ per candidate column

    let rconfig = Config::new(args.arg_input.as_ref())
        .delimiter(args.flag_delimiter)
        .no_headers_flag(args.flag_no_headers);
    let mut rdr = rconfig.reader()?;
    let mut record = csv::ByteRecord::new();
    while rdr.read_byte_record(&mut record)? {
        for (k, &col) in candidates.iter().enumerate() {
            let Some(cell) = record.get(col) else {
                continue;
            };
            let cell = crate::cmd::frequency::trim_bs_whitespace(cell);
            if cell.is_empty() {
                continue;
            }
            if let Some(x) = std::str::from_utf8(cell)
                .ok()
                .and_then(|s| s.parse::<f64>().ok())
                && x.is_finite()
            {
                let d2 = (x - means[k]) * (x - means[k]);
                sum4[k] = d2.mul_add(d2, sum4[k]);
                counts[k] += 1;
            }
        }
    }

    for (k, &col) in candidates.iter().enumerate() {
        let n = counts[k];
        // qsv-stats requires >= 4 observations for a defined sample excess kurtosis.
        if n < 4 {
            continue;
        }
        let (Some(skew), Some(variance)) = (stats[col].skewness, stats[col].variance) else {
            continue;
        };
        let variance_sq = variance * variance;
        if variance_sq == 0.0 {
            continue;
        }
        #[allow(clippy::cast_precision_loss)]
        let nf = n as f64;
        // sample excess kurtosis, matching qsv-stats `kurtosis` with precalc mean+variance:
        // (n(n+1) Σ(x-mean)⁴) / ((n-1)(n-2)(n-3) variance²) - 3(n-1)²/((n-2)(n-3))
        let denominator = (nf - 1.0) * (nf - 2.0) * (nf - 3.0);
        let adjustment = 3.0 * (nf - 1.0) * (nf - 1.0) / ((nf - 2.0) * (nf - 3.0));
        let kurtosis =
            (nf * (nf + 1.0) * sum4[k]).mul_add(1.0 / (denominator * variance_sq), -adjustment);
        // Sarle's bimodality coefficient (moarstats `compute_bimodality_coefficient`).
        let bc = skew.mul_add(skew, 1.0) / (kurtosis + 3.0);
        if bc.is_finite() && kurtosis.is_finite() {
            // store both so classify_measure's platykurtic guard (BC high AND excess kurtosis < 0)
            // sees the same pair moarstats would have written under `--smarter`.
            stats[col].kurtosis = Some(kurtosis);
            stats[col].bimodality_coefficient = Some(bc);
        }
    }
    Ok(())
}

/// Combine a column's `ColSemantics` verdict with the statistical `classify`. The dictionary
/// verdict wins for the routes it speaks to; `Defer` falls back to `classify` — also the path for
/// every column when no `--dictionary` is given, so stats-only behavior is unchanged.
///
/// Note on `MapCoord`: the caller already skips coordinates that the map panel ACTUALLY consumed
/// (via `is_map_col`) before reaching here, so a `MapCoord` column that arrives was NOT mapped
/// (e.g. only one of lat/lon present, out-of-range values, or no map rendered). Rather than let it
/// vanish, fall back to `classify` so it's still charted as a distribution — matching the
/// no-dictionary behavior for named-but-unmappable coordinates.
fn classify_with_semantics(
    idx: usize,
    s: &crate::cmd::stats::StatsData,
    sem: &ColSemantics,
) -> Option<PanelKind> {
    match sem.route {
        Route::Defer | Route::MapCoord => classify(idx, s),
        // a categorical/code with no observed values is still nothing to chart
        Route::Dimension => (s.r#type.as_str() != "NULL" && s.cardinality >= 1)
            .then_some(PanelKind::FreqBar { idx }),
        Route::Measure => classify_measure(idx, s),
        Route::Temporal | Route::Skip => None,
    }
}

/// Parse a describegpt Data Dictionary into per-column semantic rows + the dataset grain.
///
/// Accepts BOTH shapes so `--dictionary <path>` works with either:
///   * a `--format jsonschema` document — `properties.<col>` carries the human label as `title` and
///     the semantic tokens in `x-qsv.{content_type,role,concept}`; the dataset grain is the
///     top-level `x-qsv.grain`. (This is what `--dictionary infer` produces.)
///   * a legacy `--format json` dictionary — `{"Dictionary":{"response":{"fields":[...]}}}` (or a
///     bare `{"fields":[...]}`); fields carry `name`/`content_type`/`label` (and role/concept if a
///     future emitter adds them). No grain.
///
/// Returns `None` when neither shape yields a usable column, so the caller degrades to the
/// statistical heuristic rather than erroring.
fn parse_dictionary_semantics(json_text: &str) -> Option<DictData> {
    let v: serde_json::Value = serde_json::from_str(json_text).ok()?;

    // JSON Schema shape: a top-level `properties` object keyed by column name.
    if let Some(props) = v.get("properties").and_then(serde_json::Value::as_object) {
        let mut rows = HashMap::with_capacity(props.len());
        for (name, prop) in props {
            if name.is_empty() {
                continue;
            }
            let xq = prop.get("x-qsv");
            let from_xq = |k: &str| {
                xq.and_then(|x| x.get(k))
                    .and_then(serde_json::Value::as_str)
                    .unwrap_or_default()
                    .to_string()
            };
            let label = prop
                .get("title")
                .and_then(serde_json::Value::as_str)
                .unwrap_or_default()
                .to_string();
            rows.insert(
                name.clone(),
                DictRow {
                    content_type: from_xq("content_type"),
                    role: from_xq("role"),
                    concept: from_xq("concept"),
                    label,
                },
            );
        }
        if rows.is_empty() {
            return None;
        }
        let grain = v
            .get("x-qsv")
            .and_then(|x| x.get("grain"))
            .and_then(serde_json::Value::as_str)
            .filter(|g| !g.is_empty())
            .map(ToString::to_string);
        return Some(DictData { rows, grain });
    }

    // Legacy plain-json dictionary shape.
    let fields = v
        .get("Dictionary")
        .and_then(|d| d.get("response"))
        .and_then(|r| r.get("fields"))
        .or_else(|| v.get("fields"))
        .and_then(serde_json::Value::as_array)?;
    let mut rows = HashMap::with_capacity(fields.len());
    for f in fields {
        let Some(name) = f.get("name").and_then(serde_json::Value::as_str) else {
            continue;
        };
        if name.is_empty() {
            continue;
        }
        let from_field = |k: &str| {
            f.get(k)
                .and_then(serde_json::Value::as_str)
                .unwrap_or_default()
                .to_string()
        };
        rows.insert(
            name.to_string(),
            DictRow {
                content_type: from_field("content_type"),
                role:         from_field("role"),
                concept:      from_field("concept"),
                label:        from_field("label"),
            },
        );
    }
    (!rows.is_empty()).then_some(DictData { rows, grain: None })
}

/// Resolve the `--dictionary` source into a parsed `DictData` for `viz smart`:
///   * `infer` -> run `qsv describegpt --dictionary --infer-content-type --two-pass --format
///     jsonschema` on the input now (requires an LLM configured) and parse its stdout. The
///     jsonschema format carries role/concept (in each property's `x-qsv`) and the dataset grain,
///     and — unlike `--format json` — does not perturb describegpt's two-pass refine cache.
///     `--dictionary-context <file>` is forwarded as describegpt's `--context-file` so a glossary /
///     README / data dictionary conditions the role/concept/label/grain inference.
///   * any other value -> a path to an existing describegpt dictionary file (jsonschema or json).
///     `--dictionary-context` does not apply here (the dictionary is already built) and is ignored
///     with a warning.
///
/// Soft-fails (warns, returns `Ok(None)`) when generation or reading fails, so a missing LLM or a
/// stray path degrades to the plain stats-driven dashboard instead of aborting.
fn load_dictionary_semantics(args: &Args) -> CliResult<Option<DictData>> {
    let Some(spec) = args.flag_dictionary.as_deref() else {
        if args.flag_dictionary_context.is_some() {
            eprintln!("viz smart --dictionary-context: ignored without --dictionary infer.");
        }
        return Ok(None);
    };
    let Some(input) = args.arg_input.as_deref() else {
        return Ok(None);
    };

    let is_infer = spec.eq_ignore_ascii_case("infer");
    if args.flag_dictionary_context.is_some() && !is_infer {
        eprintln!(
            "viz smart --dictionary-context: only applies to `--dictionary infer` (a generated \
             dictionary); ignored when reading an existing dictionary file."
        );
    }

    let json_text = if is_infer {
        // forward --dictionary-context to describegpt as --context-file when present, so the LLM's
        // role/concept/label/grain inference is conditioned on the user's domain context.
        let mut dg_args: Vec<&str> = vec![
            "--dictionary",
            "--infer-content-type",
            "--two-pass",
            "--format",
            "jsonschema",
        ];
        if let Some(ctx) = args.flag_dictionary_context.as_deref() {
            dg_args.push("--context-file");
            dg_args.push(ctx);
        }
        match util::run_qsv_cmd(
            "describegpt",
            &dg_args,
            input,
            "Generated a Data Dictionary via describegpt for `viz smart --dictionary infer`",
        ) {
            Ok((stdout, _stderr)) => stdout,
            Err(e) => {
                eprintln!(
                    "viz smart --dictionary infer: describegpt failed ({e}); building the \
                     dashboard from statistics alone (no semantic hints)."
                );
                return Ok(None);
            },
        }
    } else {
        match std::fs::read_to_string(spec) {
            Ok(text) => text,
            Err(e) => {
                eprintln!(
                    "viz smart --dictionary: could not read dictionary file '{spec}' ({e}); \
                     building the dashboard from statistics alone (no semantic hints)."
                );
                return Ok(None);
            },
        }
    };

    let data = parse_dictionary_semantics(&json_text);
    if data.is_none() {
        eprintln!(
            "viz smart --dictionary: '{spec}' is not a recognizable describegpt dictionary (JSON \
             Schema or JSON); building the dashboard from statistics alone."
        );
    }
    Ok(data)
}

/// Decide which chart (if any) suits a column, from its computed statistics.
fn classify(idx: usize, s: &crate::cmd::stats::StatsData) -> Option<PanelKind> {
    let ty = s.r#type.as_str();
    if ty == "NULL" {
        return None; // all-empty column
    }
    let near_unique = s.uniqueness_ratio.is_some_and(|r| r > 0.95);
    let low_cardinality =
        s.cardinality >= 1 && s.cardinality <= CATEGORICAL_MAX_CARDINALITY && !near_unique;

    if ty == "Boolean" {
        return Some(PanelKind::FreqBar { idx });
    }

    if matches!(ty, "Integer" | "Float") {
        // near-unique integers are almost certainly IDs/keys - not meaningful to chart
        if ty == "Integer" && near_unique {
            return None;
        }
        // low-cardinality numeric (codes/ratings) -> frequency bar, NOT a box plot
        if low_cardinality {
            return Some(PanelKind::FreqBar { idx });
        }
        // continuous numeric -> box plot / histogram from precomputed quartiles. Shared with the
        // dictionary `measure` verdict (see `classify_measure`) so a measure is charted the same
        // way however it was identified; returns None when the column lacks quartiles.
        return classify_measure(idx, s);
    }

    // String / Date / DateTime -> frequency bar when low-cardinality
    if low_cardinality {
        return Some(PanelKind::FreqBar { idx });
    }
    // moarstats refinement: a high-cardinality categorical is normally skipped as ID-like noise.
    // But normalized_entropy distinguishes "near-uniform (truly noise)" from "concentrated (a few
    // dominant categories)". When moarstats says the distribution is concentrated, a top-N
    // frequency bar is still informative, so chart it. Absent moarstats (None), keep today's
    // behavior and skip.
    if !near_unique
        && s.normalized_entropy
            .is_some_and(|e| e < HIGH_CARD_ENTROPY_NOISE_THRESHOLD)
    {
        return Some(PanelKind::FreqBar { idx });
    }
    None // high-cardinality / ID-like text
}

/// Build a short parenthetical title hint for a box-plot panel from moarstats shape statistics:
/// skew direction (from `pearson_skewness`) and the outlier share (from `outliers_percentage`).
/// Returns None when neither extended stat is present (moarstats hasn't been run) or the column
/// is roughly symmetric with no notable outliers — so the title is left unchanged.
fn box_shape_hint(s: &crate::cmd::stats::StatsData) -> Option<String> {
    // |Pearson skewness| below this reads as ~symmetric; outlier shares below 1% are negligible.
    const SKEW_MIN_ABS: f64 = 0.5;
    const OUTLIER_MIN_PCT: f64 = 1.0;

    let mut parts: Vec<String> = Vec::new();
    if let Some(skew) = s.pearson_skewness
        && skew.abs() >= SKEW_MIN_ABS
    {
        parts.push(
            if skew > 0.0 {
                "right-skewed"
            } else {
                "left-skewed"
            }
            .to_string(),
        );
    }
    if let Some(pct) = s.outliers_percentage
        && pct >= OUTLIER_MIN_PCT
    {
        parts.push(format!("{pct:.1}% outliers"));
    }
    if parts.is_empty() {
        None
    } else {
        Some(format!("({})", parts.join(", ")))
    }
}

/// If `name` (already lower-cased) is a key/code twin of a sibling — ending in `_code`/`_id`/`_key`
/// — return the base name it twins (e.g. "subject_code" -> "subject"); otherwise None.
fn code_twin_base(name: &str) -> Option<&str> {
    for suffix in ["_code", "_id", "_key"] {
        if let Some(base) = name.strip_suffix(suffix)
            && !base.is_empty()
        {
            return Some(base);
        }
    }
    None
}

/// Identify "key twin" columns to suppress so a code/label pair (subject + subject_code, street +
/// street_id) charts only the human-readable member. A column is suppressed when it routes to a
/// frequency bar (`Dimension`) AND its name is `<base>_code`/`_id`/`_key` AND a sibling column
/// named `<base>` ALSO routes to a Dimension bar. Conservative on purpose: it only fires between
/// two charted dimensions, so a lone `*_code` (whose label isn't itself charted) is kept; and the
/// caller applies it only when a dictionary is present, so a stats-only dashboard is unchanged.
fn dimension_code_twins(
    stats: &[crate::cmd::stats::StatsData],
    sems: &[ColSemantics],
) -> std::collections::HashSet<usize> {
    use std::collections::{HashMap, HashSet};

    let dim_names: HashMap<String, usize> = stats
        .iter()
        .enumerate()
        .filter(|(i, _)| sems.get(*i).is_some_and(|s| s.route == Route::Dimension))
        .map(|(i, s)| (s.field.to_lowercase(), i))
        .collect();

    let mut suppress = HashSet::new();
    for (lname, &i) in &dim_names {
        if let Some(base) = code_twin_base(lname)
            && dim_names.contains_key(base)
        {
            suppress.insert(i);
        }
    }
    suppress
}

/// Canonical-timestamp priority for a date column's dictionary concept: a smaller rank wins as the
/// time-series x-axis. Event/created timestamps lead; closed/updated/due are secondary; a bare date
/// or no concept is last. Lets `viz smart` trend "requests over created_date" rather than over a
/// modified timestamp. Columns without a dictionary all rank 3, so selection falls back to the
/// first date column, exactly as before.
fn timestamp_rank(concept: &str) -> u8 {
    match concept {
        "time.event_timestamp" => 0,
        "time.created_at" => 1,
        "time.closed_at" | "time.updated_at" | "time.due_at" => 2,
        _ => 3,
    }
}

/// Time bucket granularity for the trend panel, widened as the date span grows so a multi-year
/// dataset doesn't render thousands of daily points.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum TsBucket {
    Day,
    Week,
    Month,
}

/// Pick a bucket granularity from the span (in days) between the date column's observed min and
/// max.
fn ts_bucket_for_span(span_days: i64) -> TsBucket {
    if span_days <= 370 {
        TsBucket::Day
    } else if span_days <= 1825 {
        TsBucket::Week
    } else {
        TsBucket::Month
    }
}

/// Truncate a date to its bucket's representative day (the day itself, its ISO-week Monday, or the
/// first of its month) so rows in the same period share one key.
fn ts_bucket_key(date: chrono::NaiveDate, bucket: TsBucket) -> chrono::NaiveDate {
    use chrono::{Datelike, Duration};
    match bucket {
        TsBucket::Day => date,
        TsBucket::Week => date - Duration::days(i64::from(date.weekday().num_days_from_monday())),
        TsBucket::Month => date.with_day(1).unwrap_or(date),
    }
}

/// Display label for a bucket key: ISO date for Day/Week, `YYYY-MM` for Month.
fn ts_bucket_label(key: chrono::NaiveDate, bucket: TsBucket) -> String {
    match bucket {
        TsBucket::Day | TsBucket::Week => key.format("%Y-%m-%d").to_string(),
        TsBucket::Month => key.format("%Y-%m").to_string(),
    }
}

/// A short word for the bucket granularity, used in axis titles ("records per day").
fn ts_bucket_word(bucket: TsBucket) -> &'static str {
    match bucket {
        TsBucket::Day => "day",
        TsBucket::Week => "week",
        TsBucket::Month => "month",
    }
}

/// Best-effort entity name for a count-over-time panel, distilled from the dataset `grain`
/// ("one row = one 311 service request" -> "311 service request"). Falls back to "records" when
/// grain is absent or doesn't fit the "... one <X>" shape, so the label is always sensible.
fn count_unit_from_grain(grain: Option<&str>) -> String {
    const FALLBACK: &str = "records";
    let Some(g) = grain else {
        return FALLBACK.to_string();
    };
    // describegpt phrases grain as "one row = one <X>" / "each row is one <X>"; take the text after
    // the last " one ".
    let entity = g
        .rsplit_once(" one ")
        .map(|(_, tail)| tail)
        .unwrap_or("")
        .trim()
        .trim_end_matches('.')
        .trim();
    if entity.is_empty() || entity.len() > 40 {
        FALLBACK.to_string()
    } else {
        entity.to_string()
    }
}

/// Parse a row's date cell into a `NaiveDateTime`, honoring the DMY preference stats used to infer
/// the column. Returns `None` for a missing/blank/unparseable cell.
fn parse_record_date(
    record: &csv::ByteRecord,
    idx: usize,
    prefer_dmy: bool,
) -> Option<chrono::DateTime<chrono::Utc>> {
    let text = std::str::from_utf8(record.get(idx)?).ok()?.trim();
    if text.is_empty() {
        return None;
    }
    qsv_dateparser::parse_with_preference(text, prefer_dmy).ok()
}

fn build_timeseries_panel(
    args: &Args,
    stats: &[crate::cmd::stats::StatsData],
    prefer_dmy: bool,
    map_cols: Option<(usize, usize)>,
    sems: &[ColSemantics],
    grain: Option<&str>,
) -> CliResult<Option<Panel>> {
    use std::collections::BTreeMap;

    use qsv_dateparser::parse_with_preference;

    // pick the canonical date/datetime column: when a dictionary tagged timestamps, prefer the
    // event/created one (timestamp_rank); otherwise the first date column (rank ties break on
    // column order). stats emits "Date"/"DateTime" once dates are inferred.
    let Some((date_idx, is_datetime)) = stats
        .iter()
        .enumerate()
        .filter_map(|(i, s)| match s.r#type.as_str() {
            "Date" => Some((i, false)),
            "DateTime" => Some((i, true)),
            _ => None,
        })
        .min_by_key(|&(i, _)| {
            (
                timestamp_rank(sems.get(i).map_or("", |s| s.concept.as_str())),
                i,
            )
        })
    else {
        return Ok(None);
    };

    // y-axis candidate: the first continuous numeric column (preferring Float), excluding map
    // coordinates and any dictionary-tagged non-measure. Near-unique columns are deliberately
    // allowed — a measurement like revenue is often near-unique yet makes the most meaningful
    // trend. A confirmed Measure or an un-tagged (Defer) column qualifies; without a dictionary
    // every column is Defer, exactly as before.
    let is_map_col = |idx: usize| map_cols.is_some_and(|(la, lo)| idx == la || idx == lo);
    let continuous_numeric = |s: &crate::cmd::stats::StatsData| {
        if !matches!(s.r#type.as_str(), "Integer" | "Float") || s.cardinality <= 1 {
            return false;
        }
        let near_unique = s.uniqueness_ratio.is_some_and(|r| r > 0.95);
        let low_cardinality = s.cardinality <= CATEGORICAL_MAX_CARDINALITY && !near_unique;
        !low_cardinality
    };
    let y_eligible = |i: usize| {
        sems.get(i)
            .is_none_or(|s| matches!(s.route, Route::Defer | Route::Measure))
    };
    let y_idx = stats
        .iter()
        .enumerate()
        .filter(|(i, s)| {
            *i != date_idx && !is_map_col(*i) && y_eligible(*i) && continuous_numeric(s)
        })
        .min_by_key(|(_, s)| usize::from(s.r#type != "Float"))
        .map(|(i, _)| i);

    // Aggregation mode:
    //   * a dictionary `measure` (route == Measure) carries an additive/mean `agg`: bucket the
    //     value by calendar period and sum (counts/amounts) or average (ratios) it.
    //   * an un-tagged (Defer) numeric: plot the raw values over time (today's behavior, LTTB).
    //   * no eligible numeric at all: count records per period — the "volume over time" overview,
    //     the single most valuable view for event datasets (e.g. 311 requests per day).
    enum Mode {
        Raw(usize),
        AggValue(usize, Agg),
        Count,
    }
    let mode = match y_idx {
        Some(i) => match sems.get(i).and_then(|s| s.agg) {
            Some(agg) => Mode::AggValue(i, agg),
            None => Mode::Raw(i),
        },
        None => Mode::Count,
    };

    let (mut rdr, headers, nh) = reader_and_headers(args)?;
    // prefer the dictionary's human label for axis/panel titles (like the per-column panels),
    // falling back to the raw header.
    let label_for = |idx: usize| {
        sems.get(idx)
            .map(|s| s.label.as_str())
            .filter(|l| !l.is_empty())
            .map_or_else(|| col_label(&headers, idx, nh), ToString::to_string)
    };
    let date_label = label_for(date_idx);
    let mut record = csv::ByteRecord::new();

    // The raw path plots individual points (chronologically sorted) over time, exactly as before.
    if let Mode::Raw(y_idx) = mode {
        let y_label = label_for(y_idx);
        let mut points: Vec<(i64, String, f64)> = Vec::new();
        while rdr.read_byte_record(&mut record)? {
            // skip non-finite y (NaN/inf): parse_f64 accepts "NaN"/"inf", but a single non-finite
            // value would poison LTTB's bucket averages and area comparisons (and render as a gap)
            let Some(y) = parse_f64(record.get(y_idx)).filter(|v| v.is_finite()) else {
                continue;
            };
            let Some(dt) = parse_record_date(&record, date_idx, prefer_dmy) else {
                continue;
            };
            let label = if is_datetime {
                dt.format("%Y-%m-%dT%H:%M:%S").to_string()
            } else {
                dt.format("%Y-%m-%d").to_string()
            };
            points.push((dt.timestamp_millis(), label, y));
        }
        // a line needs at least two points
        if points.len() < 2 {
            return Ok(None);
        }
        points.sort_by_key(|p| p.0);
        // points are chronologically sorted (monotonic x), so LTTB can downsample by triangle area,
        // preserving spikes/peaks a uniform stride would step over.
        let (xs, ys) = if points.len() > MAX_SMART_POINTS {
            let ts: Vec<f64> = points.iter().map(|p| p.0 as f64).collect();
            let yv: Vec<f64> = points.iter().map(|p| p.2).collect();
            let keep = lttb_indices(&ts, &yv, MAX_SMART_POINTS);
            let xs = keep.iter().map(|&i| points[i].1.clone()).collect();
            let ys = keep.iter().map(|&i| points[i].2).collect();
            (xs, ys)
        } else {
            (
                points.iter().map(|p| p.1.clone()).collect(),
                points.iter().map(|p| p.2).collect(),
            )
        };
        return Ok(Some(Panel::new(
            format!("{y_label} over {date_label}"),
            PanelKind::TimeSeries { y_label, xs, ys },
        )));
    }

    // Bucketed paths (AggValue / Count): group rows by calendar period. The granularity widens
    // with the date column's observed span (from the stats cache, no extra scan) so a multi-year
    // dataset stays readable.
    let bucket = {
        let parse_bound = |o: &Option<String>| {
            o.as_deref()
                .map(str::trim)
                .filter(|t| !t.is_empty())
                .and_then(|t| parse_with_preference(t, prefer_dmy).ok())
        };
        match (
            parse_bound(&stats[date_idx].min),
            parse_bound(&stats[date_idx].max),
        ) {
            (Some(lo), Some(hi)) => ts_bucket_for_span((hi - lo).num_days().max(0)),
            _ => TsBucket::Day,
        }
    };

    // accumulate (sum, n) per period; Count ignores the sum and uses n.
    let value_idx = if let Mode::AggValue(i, _) = mode {
        Some(i)
    } else {
        None
    };
    let mut buckets: BTreeMap<chrono::NaiveDate, (f64, u64)> = BTreeMap::new();
    while rdr.read_byte_record(&mut record)? {
        let Some(dt) = parse_record_date(&record, date_idx, prefer_dmy) else {
            continue;
        };
        let y = match value_idx {
            Some(i) => match parse_f64(record.get(i)).filter(|v| v.is_finite()) {
                Some(v) => v,
                // a row whose value is missing/non-finite contributes to neither sum nor count
                None => continue,
            },
            None => 0.0,
        };
        let entry = buckets
            .entry(ts_bucket_key(dt.date_naive(), bucket))
            .or_insert((0.0, 0));
        entry.0 += y;
        entry.1 += 1;
    }
    // a line needs at least two periods
    if buckets.len() < 2 {
        return Ok(None);
    }
    let word = ts_bucket_word(bucket);
    let xs: Vec<String> = buckets
        .keys()
        .map(|k| ts_bucket_label(*k, bucket))
        .collect();

    match mode {
        Mode::AggValue(y_idx, agg) => {
            let y_label = label_for(y_idx);
            let (agg_word, ys): (&str, Vec<f64>) = if agg == Agg::Mean {
                (
                    "mean",
                    buckets
                        .values()
                        .map(|&(s, n)| if n > 0 { s / n as f64 } else { 0.0 })
                        .collect(),
                )
            } else {
                // counts/amounts are additive -> sum per period
                ("sum", buckets.values().map(|&(s, _)| s).collect())
            };
            Ok(Some(Panel::new(
                format!("{y_label} ({agg_word}) over {date_label}"),
                PanelKind::TimeSeries {
                    y_label: format!("{y_label} ({agg_word}/{word})"),
                    xs,
                    ys,
                },
            )))
        },
        // Count: one point per period = number of records (rows with a parseable date).
        _ => {
            let unit = count_unit_from_grain(grain);
            let ys: Vec<f64> = buckets.values().map(|&(_, n)| n as f64).collect();
            Ok(Some(Panel::new(
                format!("{unit} over {date_label}"),
                PanelKind::TimeSeries {
                    y_label: format!("{unit} per {word}"),
                    xs,
                    ys,
                },
            )))
        },
    }
}

/// Detect the latitude/longitude column index pair by header name + numeric stats type. Shared by
/// the map-panel builder and the `viz smart` classifier so that columns recognized as geographic
/// coordinates are charted on the map panel only — not redundantly as per-column distribution
/// panels, a correlation-matrix axis, or the time-series y. Name detection needs headers, so this
/// returns None under `--no-headers` (field names are then "0","1",... and won't match).
fn latlon_indices(stats: &[crate::cmd::stats::StatsData]) -> Option<(usize, usize)> {
    let find = |names: &[&str]| {
        stats
            .iter()
            .enumerate()
            .find(|(_, s)| {
                matches!(s.r#type.as_str(), "Integer" | "Float")
                    && names.contains(&s.field.to_ascii_lowercase().as_str())
            })
            .map(|(i, _)| i)
    };
    match (
        find(&["lat", "latitude"]),
        find(&["lon", "long", "lng", "longitude"]),
    ) {
        (Some(lat), Some(lon)) => Some((lat, lon)),
        _ => None,
    }
}

// Rough bounding box of the United States (including Alaska and Hawaii) for the `viz smart` static
// geo map's `albers usa` heuristic.
const US_LON_MIN: f64 = -170.0;
const US_LON_MAX: f64 = -66.0;
const US_LAT_MIN: f64 = 18.0;
const US_LAT_MAX: f64 = 72.0;
// `albers usa` is only chosen when the data spans a continental fraction of the US, so a single
// US city renders as a fitted Mercator view rather than a US-wide composite.
const US_SPAN_MIN_LON_DEG: f64 = 15.0;
const US_SPAN_MIN_LAT_DEG: f64 = 8.0;
// padding (as a fraction of the span, with a small floor) added around a fitted local geo map so
// points aren't flush against the frame.
const GEO_FIT_PAD_FRAC: f64 = 0.1;
const GEO_FIT_PAD_MIN_DEG: f64 = 0.5;

/// Choose the projection and (for local extents) the fitted lon/lat axis ranges for a `viz smart`
/// static geo map. Data that spans a continental fraction of the US uses `albers usa` (a fixed
/// composite that frames the US — CONUS plus Alaska/Hawaii insets — so no axis ranges are set);
/// other continental/global extents use the world `NaturalEarth` projection; everything else uses
/// `Mercator` fit to the data's trimmed extent so a city-scale dataset renders as a zoomed-in map
/// instead of a dot on the world. Returns the longitude and latitude ranges separately so a local
/// extent that wraps the antimeridian can still fit latitude while leaving longitude full-width.
/// Mirrors the antimeridian-aware, trimmed framing semantics that `build_map_panel`/the map view
/// use.
fn geo_framing(lats: &[f64], lons: &[f64]) -> (ProjectionType, Option<Axis>, Option<Axis>) {
    let (lon_center, lon_span) = lon_center_and_span(lons, MAP_FRAME_TRIM_FRAC);
    let mut lats_sorted = lats.to_vec();
    lats_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let lat_lo = sorted_quantile(&lats_sorted, MAP_FRAME_TRIM_FRAC);
    let lat_hi = sorted_quantile(&lats_sorted, 1.0 - MAP_FRAME_TRIM_FRAC);
    let lat_span = lat_hi - lat_lo;

    let lon_lo = lon_center - lon_span / 2.0;
    let lon_hi = lon_center + lon_span / 2.0;

    // US-spanning extent: `albers usa` frames the US itself (CONUS + Alaska/Hawaii insets) and
    // ignores lon/lat axis ranges, so don't set them. Checked BEFORE the global fallback so a
    // coast-to-coast dataset that includes Alaska/Hawaii — and therefore exceeds the global span
    // thresholds — still gets albers usa rather than a NaturalEarth world overview.
    let within_us = lon_lo >= US_LON_MIN
        && lon_hi <= US_LON_MAX
        && lat_lo >= US_LAT_MIN
        && lat_hi <= US_LAT_MAX;
    if within_us && lon_span >= US_SPAN_MIN_LON_DEG && lat_span >= US_SPAN_MIN_LAT_DEG {
        return (ProjectionType::AlbersUsa, None, None);
    }

    // other continental/global extent: a fitted view would be unwieldy, so use the world
    // projection.
    if lon_span >= SMART_GEO_MIN_LON_SPAN_DEG || lat_span >= SMART_GEO_MIN_LAT_SPAN_DEG {
        return (ProjectionType::NaturalEarth, None, None);
    }

    // local extent: Mercator fit to the padded data bounds, clamped to valid coordinate ranges.
    let lat_pad = (lat_span * GEO_FIT_PAD_FRAC).max(GEO_FIT_PAD_MIN_DEG);
    let lataxis = Axis::new().range(vec![
        (lat_lo - lat_pad).max(-90.0),
        (lat_hi + lat_pad).min(90.0),
    ]);

    // a cluster that wraps the antimeridian yields fitted longitudes outside [-180, 180]; clamping
    // them would crop the points on the far side of +/-180, so skip the fitted longitude range
    // (let the projection show the full width) and fit latitude only.
    if lon_lo < -180.0 || lon_hi > 180.0 {
        return (ProjectionType::Mercator, None, Some(lataxis));
    }

    let lon_pad = (lon_span * GEO_FIT_PAD_FRAC).max(GEO_FIT_PAD_MIN_DEG);
    let lonaxis = Axis::new().range(vec![
        (lon_lo - lon_pad).max(-180.0),
        (lon_hi + lon_pad).min(180.0),
    ]);
    (ProjectionType::Mercator, Some(lonaxis), Some(lataxis))
}

/// Detect the latitude/longitude column pair from describegpt dictionary signals — concept
/// `geo.latitude`/`geo.longitude`, or `content_type` `latitude`/`longitude` — so coordinates with
/// non-standard headers (e.g. `X Coordinate`/`Y Coordinate`) still drive the map panel. Each must
/// be a numeric stats type. Returns None unless BOTH a latitude and a longitude column are found,
/// so the caller falls back to the header-name heuristic (`latlon_indices`).
fn semantic_latlon(
    stats: &[crate::cmd::stats::StatsData],
    dict: &DictData,
) -> Option<(usize, usize)> {
    // Some(true) = latitude, Some(false) = longitude, None = not a coordinate
    let axis_of = |row: &DictRow| -> Option<bool> {
        let concept = row.concept.trim();
        let ct = row
            .content_type
            .split_once(':')
            .map_or(row.content_type.as_str(), |(b, _)| b)
            .trim();
        if concept == "geo.latitude" || ct == "latitude" {
            Some(true)
        } else if concept == "geo.longitude" || ct == "longitude" {
            Some(false)
        } else {
            None
        }
    };
    let mut lat = None;
    let mut lon = None;
    for (i, s) in stats.iter().enumerate() {
        if !matches!(s.r#type.as_str(), "Integer" | "Float") {
            continue;
        }
        let Some(row) = dict.rows.get(&s.field) else {
            continue;
        };
        match axis_of(row) {
            Some(true) if lat.is_none() => lat = Some(i),
            Some(false) if lon.is_none() => lon = Some(i),
            _ => {},
        }
    }
    Some((lat?, lon?))
}

/// Build a `viz smart` choropleth overview panel from already-collected map coordinates: reverse-
/// geocode the points to ISO-3 country codes (reusing qsv's geocode engine) and color each country
/// by its point count. Returns `None` unless at least 2 distinct countries resolve (a
/// single-country or metro dataset stays point-only). Geocode-gated; the engine load is
/// shared/cached.
#[cfg(feature = "geocode")]
fn build_smart_choropleth_panel(lats: &[f64], lons: &[f64]) -> Option<Panel> {
    let points: Vec<(f64, f64)> = lats.iter().copied().zip(lons.iter().copied()).collect();
    let regions = crate::cmd::geocode::reverse_geocode_regions(&points, None).ok()?;

    // count points per ISO-3 country, preserving first-seen order (matches `aggregate` semantics).
    let mut order: Vec<String> = Vec::new();
    let mut counts: HashMap<String, f64> = HashMap::new();
    for region in regions.into_iter().flatten() {
        if region.iso3.is_empty() {
            continue;
        }
        counts
            .entry(region.iso3.clone())
            .and_modify(|c| *c += 1.0)
            .or_insert_with(|| {
                order.push(region.iso3.clone());
                1.0
            });
    }
    // a choropleth of one filled country tells you nothing a point map doesn't; require 2+.
    if order.len() < 2 {
        return None;
    }
    let z: Vec<f64> = order.iter().map(|iso3| counts[iso3]).collect();
    Some(Panel::new(
        "Countries".to_string(),
        PanelKind::Choropleth {
            locations: order,
            z,
            location_mode: LocationMode::Iso3,
        },
    ))
}

/// Detect a latitude/longitude column pair and, if a usable pair exists, build a `viz smart` map
/// panel. The pair comes from `coord_hint` (dictionary `geo.latitude`/`geo.longitude` signals) when
/// supplied, else the header-name heuristic (`latlon_indices`). Does one extra data pass to collect
/// the in-range coordinates (the stats cache holds no geometry). Returns `None` when no pair is
/// found, the columns aren't numeric, or no row has valid coordinates. On success returns the panel
/// together with the (lat, lon) column indices it consumed, so the caller can exclude exactly those
/// columns from the other panels — and only when a map is actually rendered. Without a dictionary
/// hint, name detection needs headers, so this is a no-op under `--no-headers`.
fn build_map_panel(
    args: &Args,
    stats: &[crate::cmd::stats::StatsData],
    coord_hint: Option<(usize, usize)>,
) -> CliResult<Option<(Panel, Option<Panel>, (usize, usize))>> {
    let Some((lat_idx, lon_idx)) = coord_hint.or_else(|| latlon_indices(stats)) else {
        return Ok(None);
    };

    let (mut rdr, _headers, _nh) = reader_and_headers(args)?;
    let mut lats: Vec<f64> = Vec::new();
    let mut lons: Vec<f64> = Vec::new();
    let mut record = csv::ByteRecord::new();
    while rdr.read_byte_record(&mut record)? {
        let (Some(lat), Some(lon)) = (
            parse_f64(record.get(lat_idx)),
            parse_f64(record.get(lon_idx)),
        ) else {
            continue;
        };
        if (-90.0..=90.0).contains(&lat) && (-180.0..=180.0).contains(&lon) {
            lats.push(lat);
            lons.push(lon);
        }
    }
    if lats.is_empty() {
        return Ok(None);
    }
    // decide density vs. markers from the full row count, then cap the embedded points so a huge
    // dataset doesn't bloat the HTML or freeze the browser on pan/zoom
    let density = lats.len() >= MAP_DENSITY_MIN_POINTS;

    // continental/global extents render as an offline `ScatterGeo` projection world-overview
    // (no network tiles, better whole-world context) rather than a zoomed mapbox tile map.
    // Use the same robust framing semantics as the map view (computed on the full in-range set,
    // before downsampling): an antimeridian-aware, trimmed longitude span and a trimmed latitude
    // span, so a cluster straddling +/-180 or a single far in-range outlier doesn't misclassify a
    // local dataset as global.
    let (_, lon_span) = lon_center_and_span(&lons, MAP_FRAME_TRIM_FRAC);
    let mut lats_sorted = lats.clone();
    lats_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let lat_span = sorted_quantile(&lats_sorted, 1.0 - MAP_FRAME_TRIM_FRAC)
        - sorted_quantile(&lats_sorted, MAP_FRAME_TRIM_FRAC);
    // density/global classification stays on the FULL in-range set (above), so removing outliers
    // doesn't perturb which basemap or render path is chosen.
    let global = lon_span >= SMART_GEO_MIN_LON_SPAN_DEG || lat_span >= SMART_GEO_MIN_LAT_SPAN_DEG;

    // split into the core cluster vs. geographic outliers (points far from the cluster centroid)
    // BEFORE downsampling, so the centroid/distances see the full population and true strays are
    // never stride-sampled away first. Not geocode-gated: the distinct outlier markers render in
    // every `viz` build; only the jurisdiction naming/suppression below needs geocode.
    let (core_lats, core_lons, out_lats, out_lons) = partition_geo_outliers(&lats, &lons);

    // reverse-geocode the CORE bounding box into the spatial-extent overlay + summary, plus a
    // representative sample of the outliers for the call-out (one batched engine load). Degrades to
    // None when the geocode feature is off, the lookup fails, or the extent spans the antimeridian.
    #[cfg(feature = "geocode")]
    let geo_meta = build_geo_meta(
        map_extent(&core_lats, &core_lons),
        &out_lats,
        &out_lons,
        out_lats.len(),
    );

    let (lats, lons) = downsample_pair(&core_lats, &core_lons, MAX_SMART_POINTS);
    let (outlier_lats, outlier_lons) = downsample_pair(&out_lats, &out_lons, SMART_GEO_OUTLIER_CAP);

    // geocode-gated companion: aggregate the FULL core set (pre-downsampling) into a per-country
    // choropleth overview drawn beside the point map. Counting the full population — not the
    // downsampled `lats`/`lons` — keeps the per-country tallies accurate for datasets above
    // `MAX_SMART_POINTS` (the panel labels them "count", so sampled tallies would mislabel). The
    // panel embeds only the per-country aggregates, never the raw points, so the full-set pass adds
    // no HTML weight. This is a SECOND reverse-geocode pass (build_geo_meta above only geocodes the
    // 5 bounding-box corners); the engine is already loaded/cached, so it's one extra batched
    // lookup loop, not a second index load. None when the points resolve to fewer than 2
    // countries, or the geocode feature is off.
    #[cfg(feature = "geocode")]
    let choropleth_panel = build_smart_choropleth_panel(&core_lats, &core_lons);
    #[cfg(not(feature = "geocode"))]
    let choropleth_panel: Option<Panel> = None;

    let kind = if global {
        PanelKind::Geo {
            lats,
            lons,
            outlier_lats,
            outlier_lons,
        }
    } else {
        PanelKind::Map {
            lats,
            lons,
            density,
            outlier_lats,
            outlier_lons,
        }
    };
    Ok(Some((
        Panel {
            name: "Map".to_string(),
            kind,
            #[cfg(feature = "geocode")]
            geo_meta,
        },
        choropleth_panel,
        (lat_idx, lon_idx),
    )))
}

/// Vertical paper-coordinate offset below a map subplot's plotting area at which the
/// consolidated location-summary annotation is anchored (typed-`Plot` grid + GridJson paths).
#[cfg(feature = "geocode")]
const GEO_META_OFFSET: f64 = 0.02;

/// Compute the observed bounding box (min/max lat/lon, no trimming) of a map panel's
/// in-range coordinates.
#[cfg(feature = "geocode")]
fn map_extent(lats: &[f64], lons: &[f64]) -> MapExtent {
    let mut e = MapExtent {
        min_lat: f64::INFINITY,
        max_lat: f64::NEG_INFINITY,
        min_lon: f64::INFINITY,
        max_lon: f64::NEG_INFINITY,
    };
    for (&lat, &lon) in lats.iter().zip(lons) {
        e.min_lat = e.min_lat.min(lat);
        e.max_lat = e.max_lat.max(lat);
        e.min_lon = e.min_lon.min(lon);
        e.max_lon = e.max_lon.max(lon);
    }
    e
}

/// Expand a bounding box to also include the given coordinates (used to grow the core extent into
/// the full extent that covers the geographic outliers too).
#[cfg(feature = "geocode")]
fn extent_including(core: MapExtent, lats: &[f64], lons: &[f64]) -> MapExtent {
    let mut e = core;
    for (&lat, &lon) in lats.iter().zip(lons) {
        e.min_lat = e.min_lat.min(lat);
        e.max_lat = e.max_lat.max(lat);
        e.min_lon = e.min_lon.min(lon);
        e.max_lon = e.max_lon.max(lon);
    }
    e
}

/// Whether a map extent spans the antimeridian (>180 degrees of longitude), in which case the
/// box corners + center are meaningless and reverse-geocoding is skipped.
#[cfg(feature = "geocode")]
fn extent_spans_antimeridian(e: &MapExtent) -> bool {
    e.max_lon - e.min_lon > 180.0
}

/// Build the closed-loop (lat, lon) polyline tracing a bounding box, in order
/// NW -> NE -> SE -> SW -> NW, for drawing the spatial extent on the map.
#[cfg(feature = "geocode")]
fn extent_box_latlon(e: &MapExtent) -> (Vec<f64>, Vec<f64>) {
    let lats = vec![e.max_lat, e.max_lat, e.min_lat, e.min_lat, e.max_lat];
    let lons = vec![e.min_lon, e.max_lon, e.max_lon, e.min_lon, e.min_lon];
    (lats, lons)
}

/// Build a *dashed* bounding-box outline as a single gapped polyline: each edge is split into short
/// dash segments separated by `NaN` breaks (which serialize to JSON `null`, rendering as gaps).
/// `scattermapbox` ignores `line.dash` (Mapbox GL can't render dashed lines), so the dashes have to
/// be drawn as geometry; this gives the full-extent box a dashed look on the tile map. Dash spacing
/// is proportional to the box size so it reads as dashes at any zoom.
#[cfg(feature = "geocode")]
fn dashed_box_latlon(e: &MapExtent) -> (Vec<f64>, Vec<f64>) {
    let span = (e.max_lat - e.min_lat).max(e.max_lon - e.min_lon);
    // target dash+gap cell length (~40 cells across the longest edge -> ~20 dashes)
    let cell = (span / 40.0).max(f64::MIN_POSITIVE);
    let corners = [
        (e.max_lat, e.min_lon),
        (e.max_lat, e.max_lon),
        (e.min_lat, e.max_lon),
        (e.min_lat, e.min_lon),
        (e.max_lat, e.min_lon),
    ];
    let (mut lats, mut lons) = (Vec::new(), Vec::new());
    for edge in corners.windows(2) {
        let (la0, lo0) = edge[0];
        let (la1, lo1) = edge[1];
        let len = ((la1 - la0).powi(2) + (lo1 - lo0).powi(2)).sqrt();
        // even cell count so each edge ends on a gap, not a dash bleeding into the corner
        let mut cells = ((len / cell).round() as usize).max(2);
        if cells % 2 == 1 {
            cells += 1;
        }
        // draw the even cells (dashes), skip the odd ones (gaps)
        for c in (0..cells).step_by(2) {
            let (t0, t1) = (c as f64 / cells as f64, (c + 1) as f64 / cells as f64);
            lats.push(la0 + (la1 - la0) * t0);
            lons.push(lo0 + (lo1 - lo0) * t0);
            lats.push(la0 + (la1 - la0) * t1);
            lons.push(lo0 + (lo1 - lo0) * t1);
            lats.push(f64::NAN); // break -> gap between dashes
            lons.push(f64::NAN);
        }
    }
    (lats, lons)
}

/// Styled line for the spatial-extent bounding box: a thick dotted purple frame, so it reads as a
/// boundary annotation rather than a data series.
#[cfg(feature = "geocode")]
fn extent_box_line() -> Line {
    Line::new()
        .color(GEO_EXTENT_LINE_COLOR)
        .width(GEO_EXTENT_LINE_WIDTH)
        .dash(plotly::common::DashType::Dot)
}

/// Styled line for the FULL-extent box (core + outliers): a dashed vivid-magenta frame. Magenta
/// stays high-contrast on every basemap layer (tan land, green, blue water, white) — unlike the
/// amber it replaced, which washed out over light terrain — and is distinct from the purple core
/// box, the warm data points, and the amber outlier markers.
#[cfg(feature = "geocode")]
fn full_extent_box_line() -> Line {
    Line::new()
        .color(GEO_FULL_EXTENT_LINE_COLOR)
        .width(GEO_FULL_EXTENT_LINE_WIDTH)
        .dash(plotly::common::DashType::Dot)
}

/// Marker for the extent corner/center points on a mapbox tile map: a large, fully opaque purple
/// circle with a white halo. Mapbox raster basemaps can't render custom marker symbols (no sprite)
/// or reliably show text glyphs (label collision culls them), so a haloed circle is used here.
#[cfg(feature = "geocode")]
fn extent_marker_mapbox() -> Marker {
    Marker::new()
        .color(GEO_EXTENT_LINE_COLOR)
        .size(GEO_EXTENT_MARKER_SIZE)
        .opacity(1.0)
        .line(Line::new().color(GEO_EXTENT_MARKER_BORDER).width(2.5))
}

/// Like `extent_marker_mapbox`, but a diamond — `ScatterGeo` (offline projection / static export)
/// renders the standard plotly marker symbols, so the points get a distinctive shape there.
#[cfg(feature = "geocode")]
fn extent_marker_geo() -> Marker {
    extent_marker_mapbox().symbol(plotly::common::MarkerSymbol::Diamond)
}

/// Mapbox (center lat, center lon, zoom) that frames an extent bounding box as tightly as possible
/// while keeping the whole box inside the panel, via the aspect-aware `fitbounds_zoom`. A `viz
/// smart` MAP panel is full grid width but a FIXED short height (`MAP_PANEL_USABLE_HEIGHT_PX`),
/// i.e. wide and short, so the latitude (height) fit and longitude (width) fit are computed
/// separately and the tighter one wins — replacing the old `floor(log2(360/max(latSpan, lonSpan)))`
/// that ignored the panel aspect (over-zooming square core boxes so they clipped, under-zooming
/// wide full extents). Width is `responsive`/unknown at generation time, so a conservative-small
/// `MAP_PANEL_ASSUMED_WIDTH_PX` is assumed; verify visually if the panel sizing changes.
#[cfg(feature = "geocode")]
fn extent_center_zoom_raw(e: &MapExtent) -> (f64, f64, u8) {
    let lat_center = (e.min_lat + e.max_lat) / 2.0;
    let lon_center = (e.min_lon + e.max_lon) / 2.0;
    let zoom = fitbounds_zoom(
        e.min_lat,
        e.max_lat,
        e.max_lon - e.min_lon,
        MAP_PANEL_ASSUMED_WIDTH_PX,
        MAP_PANEL_USABLE_HEIGHT_PX,
    );
    (lat_center, lon_center, zoom)
}

/// Mapbox center + zoom framing the (core) extent — so the default view fills with the core cluster
/// rather than zooming out to include far-flung outliers (those are drawn as distinct markers,
/// named in the label, and reachable via the "Full extent" zoom button). Thin wrapper over
/// `extent_center_zoom_raw`.
#[cfg(feature = "geocode")]
fn extent_center_zoom(e: &MapExtent) -> (Center, u8) {
    let (lat, lon, zoom) = extent_center_zoom_raw(e);
    (Center::new(lat, lon), zoom)
}

/// Build the "Core extent" / "Full extent" zoom buttons for a `viz smart` mapbox map. Each button
/// relayouts the mapbox center+zoom to frame the respective extent. Only used when the panel has
/// geographic outliers (so the two views actually differ). The map opens at the core view, so
/// "Core extent" is the active button.
#[cfg(feature = "geocode")]
fn extent_zoom_menu(core: &MapExtent, full: &MapExtent) -> UpdateMenu {
    let button = |label: &str, e: &MapExtent| {
        let (lat, lon, zoom) = extent_center_zoom_raw(e);
        Button::new()
            .label(label)
            .method(ButtonMethod::Relayout)
            .args(serde_json::json!([{
                "mapbox.center": { "lat": lat, "lon": lon },
                "mapbox.zoom": zoom,
            }]))
    };
    UpdateMenu::new()
        .ty(UpdateMenuType::Buttons)
        .direction(UpdateMenuDirection::Right)
        .buttons(vec![
            button("Core extent", core),
            button("Full extent", full),
        ])
        .x(0.02)
        .x_anchor(Anchor::Left)
        .y(0.98)
        .y_anchor(Anchor::Top)
        .show_active(true)
        .active(0)
        .background_color(NamedColor::White)
        .border_color(NamedColor::Gray)
        .border_width(1)
        // the button pill is always white (over the light map tiles), so pin the label color to
        // ink rather than inheriting `layout.font.color` — otherwise the dark-mode toggle (or a
        // dark `--theme`) flips the label to a light color and it vanishes on the white pill.
        .font(Font::new().family(FONT_FAMILY).color(INK).size(11))
}

/// Padded longitude + latitude `geo` axis ranges that frame the (core) extent box as tightly as
/// possible, for the offline `ScatterGeo` (local Mercator) path — the analogue of
/// `extent_center_zoom` for projection maps. Unlike mapbox, these are exact ranges, so the small
/// `GEO_EXTENT_FIT_PAD_*` padding is the only slack.
#[cfg(feature = "geocode")]
fn extent_geo_axes(e: &MapExtent) -> (Axis, Axis) {
    let lat_pad =
        ((e.max_lat - e.min_lat) * GEO_EXTENT_FIT_PAD_FRAC).max(GEO_EXTENT_FIT_PAD_MIN_DEG);
    let lon_pad =
        ((e.max_lon - e.min_lon) * GEO_EXTENT_FIT_PAD_FRAC).max(GEO_EXTENT_FIT_PAD_MIN_DEG);
    let lon = Axis::new().range(vec![
        (e.min_lon - lon_pad).max(-180.0),
        (e.max_lon + lon_pad).min(180.0),
    ]);
    let lat = Axis::new().range(vec![
        (e.min_lat - lat_pad).max(-90.0),
        (e.max_lat + lat_pad).min(90.0),
    ]);
    (lon, lat)
}

/// Join the non-empty components of a reverse-geocoded label into "City, Admin1, Country".
#[cfg(feature = "geocode")]
fn format_geo_label(label: &crate::cmd::geocode::GeoLabel) -> String {
    [
        label.city.as_str(),
        label.admin1.as_str(),
        label.country.as_str(),
    ]
    .iter()
    .filter(|s| !s.trim().is_empty())
    .copied()
    .collect::<Vec<_>>()
    .join(", ")
}

/// Hover text for a single extent point marker, e.g. "NW: Newark, New Jersey, United States"
/// (or "NW: no nearby city" when the coordinate didn't resolve).
#[cfg(feature = "geocode")]
fn point_hover_text(p: &GeoPoint) -> String {
    match &p.label {
        Some(label) => format!("{}: {}", p.tag, format_geo_label(label)),
        None => format!("{}: no nearby city", p.tag),
    }
}

/// Distinct, first-seen order, case-insensitive dedup of non-empty trimmed values.
#[cfg(feature = "geocode")]
fn distinct_jurisdictions<'a>(vals: impl Iterator<Item = &'a str>) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    let mut out = Vec::new();
    for v in vals {
        let v = v.trim();
        if v.is_empty() {
            continue;
        }
        if seen.insert(v.to_lowercase()) {
            out.push(v.to_string());
        }
    }
    out
}

/// Join a list of names as "A" | "A & B" | "A, B & C".
#[cfg(feature = "geocode")]
fn join_jurisdictions(items: &[String]) -> String {
    match items {
        [] => String::new(),
        [a] => a.clone(),
        [a, b] => format!("{a} & {b}"),
        [rest @ .., last] => format!("{} & {last}", rest.join(", ")),
    }
}

/// Concise jurisdiction list for the outlier call-out: distinct admin1s (states/regions) if any
/// resolved, else distinct countries, joined "A" / "A & B" / "A, B & C"; more than 3 collapses to a
/// count. Returns "" when no outlier point resolved (the call-out then omits the parenthetical).
#[cfg(feature = "geocode")]
fn outlier_jurisdictions(labels: &[Option<crate::cmd::geocode::GeoLabel>]) -> String {
    let resolved: Vec<&crate::cmd::geocode::GeoLabel> =
        labels.iter().filter_map(|l| l.as_ref()).collect();
    if resolved.is_empty() {
        return String::new();
    }
    let admin1s = distinct_jurisdictions(resolved.iter().map(|l| l.admin1.as_str()));
    let names = if admin1s.is_empty() {
        distinct_jurisdictions(resolved.iter().map(|l| l.country.as_str()))
    } else {
        admin1s
    };
    match names.len() {
        0 => String::new(),
        n if n > 3 => format!("{n} areas"),
        _ => join_jurisdictions(&names),
    }
}

/// Whether the geographic outliers fall within the SAME jurisdiction(s) as the core extent — in
/// which case they're not meaningful "elsewhere" strays (just the cluster's far edge), so the
/// outlier call-out is suppressed. True when every resolved outlier admin1 is already among the
/// core's admin1s AND every resolved outlier country is among the core's countries; also true when
/// no outlier point resolved to a jurisdiction (we can't claim a different place). False as soon as
/// an outlier introduces a new admin1/country (e.g. a Pennsylvania stray beside a NY/NJ core).
#[cfg(feature = "geocode")]
fn outliers_share_core_region(
    core: &[GeoPoint],
    outlier_labels: &[Option<crate::cmd::geocode::GeoLabel>],
) -> bool {
    use std::collections::HashSet;
    let lower_set = |it: &mut dyn Iterator<Item = &str>| -> HashSet<String> {
        it.map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_lowercase)
            .collect()
    };
    let core_admin1 = lower_set(
        &mut core
            .iter()
            .filter_map(|p| p.label.as_ref())
            .map(|l| l.admin1.as_str()),
    );
    let core_country = lower_set(
        &mut core
            .iter()
            .filter_map(|p| p.label.as_ref())
            .map(|l| l.country.as_str()),
    );
    let out_admin1 = lower_set(
        &mut outlier_labels
            .iter()
            .filter_map(|l| l.as_ref())
            .map(|l| l.admin1.as_str()),
    );
    let out_country = lower_set(
        &mut outlier_labels
            .iter()
            .filter_map(|l| l.as_ref())
            .map(|l| l.country.as_str()),
    );

    if out_admin1.is_empty() && out_country.is_empty() {
        // outliers didn't resolve to any place -> can't claim they're elsewhere; suppress
        return true;
    }
    out_admin1.is_subset(&core_admin1) && out_country.is_subset(&core_country)
}

/// Assemble the spatial-extent summary string from the core jurisdiction summary, the (true)
/// outlier count, and their consolidated jurisdiction. With zero outliers the core summary is
/// returned unchanged (byte-identical to the pre-feature behavior). Otherwise an em-dash call-out
/// is appended, e.g. "New York & New Jersey, United States — 3 outliers (Pennsylvania)"; the
/// parenthetical is omitted when no outlier point resolved to a jurisdiction.
#[cfg(feature = "geocode")]
fn outlier_summary(core: &str, n_outliers: usize, jurisdictions: &str) -> String {
    if n_outliers == 0 {
        return core.to_string();
    }
    let noun = if n_outliers == 1 {
        "outlier"
    } else {
        "outliers"
    };
    let n_outliers = HumanCount(n_outliers as u64);
    if jurisdictions.is_empty() {
        format!("{core} \u{2014} {n_outliers} {noun}")
    } else {
        format!("{core} \u{2014} {n_outliers} {noun} ({jurisdictions})")
    }
}

/// Post-process the up-to-5 reverse-geocoded extent points into one concise jurisdiction
/// line. Collapses shared country/state instead of repeating it; lists up to 3 distinct
/// regions/countries, else falls back to a count. Returns "" when no point resolved.
#[cfg(feature = "geocode")]
fn consolidate_geo(points: &[GeoPoint]) -> String {
    let labels: Vec<&crate::cmd::geocode::GeoLabel> =
        points.iter().filter_map(|p| p.label.as_ref()).collect();
    if labels.is_empty() {
        return String::new();
    }

    let countries = distinct_jurisdictions(labels.iter().map(|l| l.country.as_str()));
    if countries.len() > 3 {
        return format!("{} countries", countries.len());
    }
    if countries.len() >= 2 {
        return join_jurisdictions(&countries);
    }
    // 0 or 1 distinct country
    let country = countries.first();
    let suffix = country.map(|c| format!(", {c}")).unwrap_or_default();

    let admin1s = distinct_jurisdictions(labels.iter().map(|l| l.admin1.as_str()));
    if admin1s.is_empty() {
        return country.cloned().unwrap_or_default();
    }
    if admin1s.len() > 3 {
        return format!("{} regions{suffix}", admin1s.len());
    }
    if admin1s.len() >= 2 {
        return format!("{}{suffix}", join_jurisdictions(&admin1s));
    }
    // single admin1: name the city only when the whole extent resolved to one city
    let admin1 = &admin1s[0];
    let cities = distinct_jurisdictions(labels.iter().map(|l| l.city.as_str()));
    if cities.len() == 1 {
        return format!("{}, {admin1}{suffix}", cities[0]);
    }
    format!("{admin1}{suffix}")
}

/// Max outlier points reverse-geocoded for the call-out's jurisdiction list. The TRUE outlier count
/// is reported separately (it's just the partition size); this only bounds how many we look up to
/// NAME the jurisdictions, so a far-flung set is still represented without ballooning the lookup.
#[cfg(feature = "geocode")]
const GEO_OUTLIER_GEOCODE_SAMPLE: usize = 12;

/// Reverse-geocode the CORE bounding box (4 corners + center) into the `GeoMeta` overlay + summary,
/// plus a capped, stride-sampled set of the outlier points — all in ONE batched lookup (the engine
/// loads once). The summary names the core jurisdictions and, when there are outliers that resolve
/// to a DIFFERENT jurisdiction than the core, appends a call-out with the count + the outliers'
/// jurisdiction; outliers within the core's own jurisdiction(s) are suppressed (just the cluster's
/// far edge, not strays elsewhere), and with zero outliers the result is byte-identical to the
/// pre-feature behavior. Returns `None` (so the map renders without the
/// overlay) when the core extent spans the antimeridian (corner/center labels would be
/// meaningless), the geocode lookup fails (offline/missing index), or no core point resolves to a
/// city.
#[cfg(feature = "geocode")]
fn build_geo_meta(
    core_extent: MapExtent,
    outlier_lats: &[f64],
    outlier_lons: &[f64],
    n_outliers: usize,
) -> Option<GeoMeta> {
    // a dataset straddling +/-180 yields a near-global box whose center lands mid-ocean; skip.
    if extent_spans_antimeridian(&core_extent) {
        return None;
    }
    let c_lat = (core_extent.min_lat + core_extent.max_lat) / 2.0;
    let c_lon = (core_extent.min_lon + core_extent.max_lon) / 2.0;
    let coords = [
        ("NW", core_extent.max_lat, core_extent.min_lon),
        ("NE", core_extent.max_lat, core_extent.max_lon),
        ("SW", core_extent.min_lat, core_extent.min_lon),
        ("SE", core_extent.min_lat, core_extent.max_lon),
        ("Center", c_lat, c_lon),
    ];

    // batch the 5 core corner/center lookups with a stride-sampled set of the outlier points so the
    // whole call-out resolves in a single engine load.
    let (sample_lats, sample_lons) =
        downsample_pair(outlier_lats, outlier_lons, GEO_OUTLIER_GEOCODE_SAMPLE);
    let mut query: Vec<(f64, f64)> = coords.iter().map(|&(_, lat, lon)| (lat, lon)).collect();
    query.extend(
        sample_lats
            .iter()
            .zip(&sample_lons)
            .map(|(&lat, &lon)| (lat, lon)),
    );

    // a lookup failure (e.g. offline first-use) degrades to no metadata rather than failing viz.
    let mut labels = crate::cmd::geocode::reverse_geocode_points(&query, None).ok()?;
    // split: first 5 -> core corners/center, remainder -> the outlier sample
    let outlier_labels = labels.split_off(coords.len());

    let points: Vec<GeoPoint> = coords
        .iter()
        .zip(labels)
        .map(|(&(tag, lat, lon), label)| GeoPoint {
            tag,
            lat,
            lon,
            label,
        })
        .collect();

    let core_summary = consolidate_geo(&points);
    if core_summary.is_empty() {
        return None;
    }
    // suppress the call-out when the outliers fall within the core's own jurisdiction(s) — they're
    // the cluster's far edge, not strays "elsewhere", so naming them would just be redundant noise.
    let summary = if n_outliers == 0 || outliers_share_core_region(&points, &outlier_labels) {
        core_summary
    } else {
        outlier_summary(
            &core_summary,
            n_outliers,
            &outlier_jurisdictions(&outlier_labels),
        )
    };
    // the full extent (core + all outliers) is drawn as a second, no-fill box. Only meaningful when
    // there ARE outliers; skip it if a stray pushes the box across the antimeridian (it would
    // wrap).
    let full_extent = (n_outliers > 0)
        .then(|| extent_including(core_extent, outlier_lats, outlier_lons))
        .filter(|fe| !extent_spans_antimeridian(fe));
    Some(GeoMeta {
        extent: core_extent,
        full_extent,
        points,
        summary,
    })
}

/// Outline/symbol color for the spatial-extent overlay drawn on `viz smart` maps. A deep purple,
/// deliberately distinct from the warm (red/orange) palette the data points/density use.
#[cfg(feature = "geocode")]
const GEO_EXTENT_LINE_COLOR: &str = "#6a1b9a";
/// Translucent fill for the spatial-extent bounding box (kept faint so it doesn't obscure data).
#[cfg(feature = "geocode")]
const GEO_EXTENT_FILL_COLOR: &str = "rgba(106, 27, 154, 0.12)";
/// White halo around extent markers, for contrast against both the basemap and the data points.
#[cfg(feature = "geocode")]
const GEO_EXTENT_MARKER_BORDER: &str = "#ffffff";
/// Marker size (px) for the reverse-geocoded extent corner/center points — deliberately large so
/// they read as annotations, not data.
#[cfg(feature = "geocode")]
const GEO_EXTENT_MARKER_SIZE: usize = 16;
/// Bounding-box line width (px) for the spatial-extent frame.
#[cfg(feature = "geocode")]
const GEO_EXTENT_LINE_WIDTH: f64 = 3.0;
/// Line width (px) for the full-extent box.
#[cfg(feature = "geocode")]
const GEO_FULL_EXTENT_LINE_WIDTH: f64 = 2.5;
/// Line color for the full-extent box: a vivid magenta/fuchsia that stays visible on every basemap
/// layer (light land, green, blue water) and is distinct from the purple core box and amber
/// outliers.
#[cfg(feature = "geocode")]
const GEO_FULL_EXTENT_LINE_COLOR: &str = "#e6007e";
/// Fractional padding added to each side of the (core) extent box for the `ScatterGeo` exact-range
/// fit. Small, so the projection map frames the core cluster tightly.
#[cfg(feature = "geocode")]
const GEO_EXTENT_FIT_PAD_FRAC: f64 = 0.04;
/// Minimum absolute padding (degrees) for the `ScatterGeo` extent fit, so a near-degenerate box
/// still gets a sliver of margin.
#[cfg(feature = "geocode")]
const GEO_EXTENT_FIT_PAD_MIN_DEG: f64 = 0.1;

/// Add the spatial-extent bounding box (dotted filled outline) plus reverse-geocoded corner/center
/// points (drawn as hover-labeled diamond glyphs) to a mapbox map `Plot`. When the panel has
/// geographic outliers, a second no-fill dotted amber box marking the full extent (core + outliers)
/// is drawn underneath.
#[cfg(feature = "geocode")]
fn add_extent_overlay_mapbox(plot: &mut Plot, meta: &GeoMeta) {
    // the full-extent box (core + outliers) is drawn first, underneath, with no fill. Mapbox
    // ignores line.dash, so the dashed look is drawn as a gapped polyline.
    if let Some(full) = &meta.full_extent {
        let (flat, flon) = dashed_box_latlon(full);
        plot.add_trace(
            ScatterMapbox::new(flat, flon)
                .name("full extent (incl. outliers)")
                .mode(Mode::Lines)
                .line(full_extent_box_line())
                .hover_info(HoverInfo::Skip)
                .show_legend(false),
        );
    }
    let (blat, blon) = extent_box_latlon(&meta.extent);
    plot.add_trace(
        ScatterMapbox::new(blat, blon)
            .name("spatial extent")
            .mode(Mode::Lines)
            .line(extent_box_line())
            .fill(plotly::traces::scatter_mapbox::Fill::ToSelf)
            .fill_color(GEO_EXTENT_FILL_COLOR)
            .hover_info(HoverInfo::Skip)
            .show_legend(false),
    );
    let mlat: Vec<f64> = meta.points.iter().map(|p| p.lat).collect();
    let mlon: Vec<f64> = meta.points.iter().map(|p| p.lon).collect();
    let htext: Vec<String> = meta.points.iter().map(point_hover_text).collect();
    plot.add_trace(
        ScatterMapbox::new(mlat, mlon)
            .name("extent points")
            .mode(Mode::Markers)
            .marker(extent_marker_mapbox())
            .hover_text_array(htext)
            .hover_info(HoverInfo::Text)
            .show_legend(false),
    );
}

/// Like `add_extent_overlay_mapbox` (including the optional full-extent box), but for an offline
/// `ScatterGeo` projection `Plot` (used for continental/global extents and static image export).
/// Hover labels are inert in static images, so the static export simply shows the box + diamond
/// glyphs, as intended.
#[cfg(feature = "geocode")]
fn add_extent_overlay_geo(plot: &mut Plot, meta: &GeoMeta) {
    // the full-extent box (core + outliers) is drawn first, underneath, with no fill
    if let Some(full) = &meta.full_extent {
        let (flat, flon) = extent_box_latlon(full);
        plot.add_trace(
            ScatterGeo::new(flat, flon)
                .name("full extent (incl. outliers)")
                .mode(Mode::Lines)
                .line(full_extent_box_line())
                .hover_info(HoverInfo::Skip)
                .show_legend(false),
        );
    }
    let (blat, blon) = extent_box_latlon(&meta.extent);
    plot.add_trace(
        ScatterGeo::new(blat, blon)
            .name("spatial extent")
            .mode(Mode::Lines)
            .line(extent_box_line())
            .fill(plotly::traces::scatter_geo::Fill::ToSelf)
            .fill_color(GEO_EXTENT_FILL_COLOR)
            .hover_info(HoverInfo::Skip)
            .show_legend(false),
    );
    let mlat: Vec<f64> = meta.points.iter().map(|p| p.lat).collect();
    let mlon: Vec<f64> = meta.points.iter().map(|p| p.lon).collect();
    let htext: Vec<String> = meta.points.iter().map(point_hover_text).collect();
    plot.add_trace(
        ScatterGeo::new(mlat, mlon)
            .name("extent points")
            .mode(Mode::Markers)
            .marker(extent_marker_geo())
            .hover_text_array(htext)
            .hover_info(HoverInfo::Text)
            .show_legend(false),
    );
}

/// Marker color for geographic outlier points on `viz smart` maps. A vivid amber, deliberately
/// distinct from BOTH the warm (red/orange) data points AND the deep-purple extent overlay, so all
/// three layers read as separate. Not geocode-gated — outlier styling works in every `viz` build.
const GEO_OUTLIER_COLOR: &str = "#f9a825";
/// Marker size (px) for outlier points — larger than the data points so the handful of strays stand
/// out at a glance, even when they sit outside the default (core-fit) viewport.
const GEO_OUTLIER_MARKER_SIZE: usize = 11;
/// White halo around outlier markers, for contrast against both tile and projection basemaps.
const GEO_OUTLIER_MARKER_BORDER: &str = "#ffffff";

/// Outlier marker for mapbox tile maps: a haloed amber circle. Mapbox can't render custom symbols,
/// so the distinction comes from size + color versus the data points.
fn outlier_marker_mapbox() -> Marker {
    Marker::new()
        .color(GEO_OUTLIER_COLOR)
        .size(GEO_OUTLIER_MARKER_SIZE)
        .opacity(0.95)
        .line(Line::new().color(GEO_OUTLIER_MARKER_BORDER).width(1.5))
}

/// Like `outlier_marker_mapbox`, but an X glyph — `ScatterGeo` renders plotly symbols, giving the
/// outliers a distinct SHAPE in addition to color (and a readable glyph in inert static images).
fn outlier_marker_geo() -> Marker {
    outlier_marker_mapbox().symbol(plotly::common::MarkerSymbol::X)
}

/// Build the `viz smart` auto-dashboard from the dataset's statistics + frequency data.
/// Classifies columns into panels, then renders either a single-`Plot` subplot grid (≤8 panels,
/// or any image export) or a self-contained inline-div HTML page (>8 panels, HTML output).
fn build_smart(args: &Args, out_format: OutFormat) -> CliResult<SmartRender> {
    let Some(input) = args.arg_input.clone() else {
        return fail_incorrectusage_clierror!(
            "`viz smart` requires a file input (it derives charts from the dataset's statistics)."
        );
    };
    if input == "-" {
        return fail_incorrectusage_clierror!(
            "`viz smart` cannot read from stdin; pass a CSV/TSV file path."
        );
    }

    // `--smarter` with non-default parsing (--no-headers / custom --delimiter) skips moarstats
    // enrichment (see below) AND forces the standard stats path to regenerate. get_stats_records
    // keys its `.stats.csv.data.jsonl` cache only by mtime + stat sufficiency, NOT by parsing
    // options, so without this a current default-parsing cache (e.g. from an earlier headered run)
    // would be reused and the "standard dashboard" would render from incorrectly parsed stats.
    let force_stats_regen =
        args.flag_smarter && (args.flag_no_headers || args.flag_delimiter.is_some());

    if args.flag_smarter {
        // moarstats --advanced computes its advanced stats by RE-READING the input itself: the
        // KGA pass via `Config::new(...).reader_file()` and the entropy pass via a `frequency`
        // subprocess, both using extension-based delimiter detection and assuming a header row
        // (moarstats has no --delimiter/--no-headers flags). So when viz smart is given a custom
        // --delimiter or --no-headers, moarstats would parse the data differently than viz does:
        // for --no-headers the base stats name columns 1,2,... while the KGA reader treats row 1
        // as headers, so the advanced columns never attach; for a custom delimiter the advanced
        // readers mis-split the rows entirely. In either case skip enrichment and let the standard
        // get_stats_records path below build a correct, freshly-regenerated cache
        // (force_stats_regen above) that honors the parsing flags.
        if args.flag_no_headers || args.flag_delimiter.is_some() {
            eprintln!(
                "viz smart --smarter: moarstats enrichment is only applied with default parsing; \
                 --no-headers / --delimiter inputs use the standard dashboard instead."
            );
        } else {
            // Enrich the stats cache with moarstats advanced distribution-shape stats so
            // classify()/box_shape_hint() can upgrade box->histogram, surface concentrated
            // high-cardinality FreqBars, and annotate box titles. moarstats rewrites the
            // .stats.csv.data.jsonl sidecar that get_stats_records reads below.
            //
            // moarstats' top-level --force guarantees the base `qsv stats` subprocess re-runs
            // with the full default --stats-options (cardinality/quartiles/skewness); without it
            // a stale, lean stats cache would be reused and the advanced columns viz needs
            // (bimodality_coefficient, normalized_entropy) would silently not be added.
            //
            // --stats-options mirrors moarstats' own default plus --dates-whitelist sniff, so the
            // regenerated cache infers dates the same way viz smart does (for the time-series
            // panel). Soft-fail: a moarstats error degrades to the plain ProfileSchema dashboard.
            let stats_opts = "--infer-dates --infer-boolean --cardinality --mode --mad \
                              --quartiles --percentiles --force --stats-jsonl --dates-whitelist \
                              sniff";
            let moar_argv = ["--advanced", "--force", "--stats-options", stats_opts];
            if let Err(e) = util::run_qsv_cmd(
                "moarstats",
                &moar_argv,
                &input,
                "Enriched stats cache via moarstats --advanced for `viz smart --smarter`",
            ) {
                eprintln!(
                    "viz smart --smarter: moarstats enrichment failed ({e}); falling back to \
                     standard stats. Dashboard will omit advanced refinements."
                );
            }
        }
    }

    let schema_args = util::SchemaArgs {
        flag_enum_threshold:  0,
        flag_ignore_case:     false,
        flag_strict_dates:    false,
        flag_strict_formats:  false,
        flag_pattern_columns: SelectColumns::parse("").expect("empty selection is valid"),
        flag_dates_whitelist: "sniff".to_string(),
        flag_prefer_dmy:      false,
        flag_force:           force_stats_regen,
        flag_stdout:          false,
        flag_jobs:            None,
        flag_polars:          false,
        flag_no_headers:      args.flag_no_headers,
        flag_delimiter:       args.flag_delimiter,
        arg_input:            Some(input),
        flag_memcheck:        false,
        flag_output:          None,
    };

    let (_headers, mut stats) =
        util::get_stats_records(&schema_args, util::StatsMode::ProfileSchema)?;
    if stats.is_empty() {
        return fail_clierror!(
            "Could not compute statistics for `viz smart`. The input must be a regular CSV/TSV \
             file (not stdin or a compressed/special format)."
        );
    }

    // Plain `viz smart` (no `--smarter`): the stats cache has `skewness` but not the moarstats-only
    // `kurtosis`/`bimodality_coefficient`, so a bimodal column would render as a box plot that
    // hides its peaks. Compute Sarle's bimodality coefficient in one streaming pass so
    // `classify_measure` can upgrade such a column to a histogram WITHOUT requiring
    // `--smarter`. No-op (and no pass) when moarstats already enriched the cache or no column
    // qualifies. Soft-fail to a box plot.
    if let Err(e) = enrich_bimodality(args, &mut stats) {
        eprintln!(
            "viz smart: bimodality detection failed ({e}); bimodal columns may render as box \
             plots. Use --smarter for moarstats-based enrichment."
        );
    }

    // Optional describegpt Data Dictionary (--dictionary): per-column semantic verdicts (concept ->
    // role -> content_type -> stats) that override the statistical guess for the cases they speak
    // to. When --dictionary is absent, dict_data is None and every column's route resolves to
    // Defer, so classification is identical to today's stats-only behavior.
    let dict_data = load_dictionary_semantics(args)?;
    let col_sems: Vec<ColSemantics> = stats
        .iter()
        .map(|s| derive_semantics(s, dict_data.as_ref().and_then(|d| d.rows.get(&s.field))))
        .collect();

    // De-duplicate code/label twins (subject + subject_code -> chart only "subject"). Gated on a
    // dictionary being present so a stats-only dashboard is byte-identical; timezone twins and IDs
    // are already de-duplicated by the Temporal/Skip routing above.
    let twin_suppress = if dict_data.is_some() {
        dimension_code_twins(&stats, &col_sems)
    } else {
        std::collections::HashSet::new()
    };

    // Build the geographic map panel up front (one data pass) so we can learn which lat/lon columns
    // it ACTUALLY consumed. Those columns are charted on the map only — excluded from per-column
    // distribution panels, the correlation matrix, and the time-series y so a map dashboard doesn't
    // redundantly box/histogram its coordinates or plot e.g. latitude vs time.
    //
    // The map panel is built for BOTH HTML and static image output. The mapbox tile basemap
    // (ScatterMapbox/DensityMapbox) needs a live browser + network tiles, so it can't be statically
    // exported — but the offline `ScatterGeo` projection basemap CAN (it's how the standalone
    // `viz geo` command exports images). So for image output a local-extent `Map` panel is coerced
    // to the `Geo` form, fit to the data's extent (see `geo_framing`). When build_map_panel returns
    // None (no usable lat/lon pair), map_cols stays None and those columns are charted normally.
    // (map_panel, choropleth_panel): the point map plus an optional geocode-derived per-country
    // choropleth overview. The choropleth is non-cartesian and HTML-only, so it's dropped for image
    // export; the point map is coerced from Map -> Geo for image export (only the offline
    // ScatterGeo basemap can be statically exported).
    let (map_panel, choropleth_panel) = {
        // prefer dictionary geo.latitude/geo.longitude (handles non-standard coord names like
        // `X Coordinate`/`Y Coordinate`), falling back to the header-name heuristic.
        let coord_hint = dict_data.as_ref().and_then(|d| semantic_latlon(&stats, d));
        match build_map_panel(args, &stats, coord_hint)? {
            None => (None, None),
            Some((p, choro, cols)) => {
                if out_format.is_image() {
                    let kind = match p.kind {
                        PanelKind::Map {
                            lats,
                            lons,
                            outlier_lats,
                            outlier_lons,
                            ..
                        } => PanelKind::Geo {
                            lats,
                            lons,
                            outlier_lats,
                            outlier_lons,
                        },
                        other => other,
                    };
                    let p = Panel {
                        name: p.name,
                        kind,
                        #[cfg(feature = "geocode")]
                        geo_meta: p.geo_meta,
                    };
                    (Some((p, cols)), None)
                } else {
                    (Some((p, cols)), choro)
                }
            },
        }
    };
    let map_cols = map_panel.as_ref().map(|(_, cols)| *cols);
    let is_map_col = |idx: usize| map_cols.is_some_and(|(la, lo)| idx == la || idx == lo);

    // Box-points handling for `viz smart`. A continuous-numeric column is normally a cache-only
    // quartile box (no data re-scan). When points should be overlaid, it instead becomes a raw box
    // (one extra batched pass, below) so plotly can draw true Tukey whiskers and the sample points.
    // The overlay mode is the user's explicit `--box-points` when given, otherwise a size-based
    // heuristic decides per the dataset's row count (see `smart_box_points`). The row count is
    // pulled once from the stats/index cache, and only when the first box panel actually needs it.
    let explicit_box_points: Option<BoxPoints> = match args.flag_box_points.as_deref() {
        Some(s) => Some(parse_box_points(Some(s))?),
        None => None,
    };
    let count_conf = Config::new(args.arg_input.as_ref())
        .delimiter(args.flag_delimiter)
        .no_headers_flag(args.flag_no_headers);
    let mut nrows: Option<u64> = None;

    // classify each column into a dashboard panel
    let mut panels: Vec<Panel> = Vec::new();
    let mut skipped: Vec<String> = Vec::new();
    for (idx, s) in stats.iter().enumerate() {
        if is_map_col(idx) {
            continue;
        }
        let sem = &col_sems[idx];
        // prefer the dictionary's human label for the panel title, falling back to the header
        // (or a positional name when headerless).
        let name = if !sem.label.is_empty() {
            sem.label.clone()
        } else if s.field.is_empty() {
            format!("col {}", idx + 1)
        } else {
            s.field.clone()
        };
        // a code/key twin (e.g. subject_code beside subject) is redundant with its label sibling
        if twin_suppress.contains(&idx) {
            skipped.push(name);
            continue;
        }
        match classify_with_semantics(idx, s, sem) {
            Some(mut kind) => {
                // a cache-only quartile box becomes a raw box (with an overlay mode) when the
                // explicit flag or the size heuristic calls for points (<= SMART_BOX_OUTLIERS_MAX
                // rows). Above that, a column that HAS Tukey outliers becomes a `BoxOutliers`
                // (precomputed box + native outlier-point overlay via a single fence-filtered
                // pass); a column with no outliers stays a cheap cache-only `BoxStats` (no pass).
                if let PanelKind::BoxStats {
                    q1,
                    median,
                    q3,
                    lower,
                    upper,
                    mean,
                } = kind
                {
                    let n =
                        *nrows.get_or_insert_with(|| util::count_rows(&count_conf).unwrap_or(0));
                    if let Some(points) = smart_box_points(explicit_box_points.as_ref(), n) {
                        kind = PanelKind::BoxRaw { idx, points };
                    } else if explicit_box_points.is_none()
                        && n > SMART_BOX_OUTLIERS_MAX
                        && let Some((fence_low, fence_high)) = box_fences(s)
                        && (lower.is_some_and(|lo| lo < fence_low)
                            || upper.is_some_and(|hi| hi > fence_high))
                    {
                        // `lower`/`upper` are the column's observed min/max — outside the Tukey
                        // fences means real outliers exist, so overlay them.
                        kind = PanelKind::BoxOutliers {
                            idx,
                            q1,
                            median,
                            q3,
                            mean,
                            fence_low,
                            fence_high,
                        };
                    }
                }
                // for box panels, append moarstats shape hints (skew direction, outlier share)
                // to the panel title when those extended stats are present. Cache-only, no cost;
                // without moarstats the hint is None and the title is unchanged.
                let name = match &kind {
                    PanelKind::BoxStats { .. }
                    | PanelKind::BoxRaw { .. }
                    | PanelKind::BoxOutliers { .. } => match box_shape_hint(s) {
                        Some(hint) => format!("{name} {hint}"),
                        None => name,
                    },
                    _ => name,
                };
                panels.push(Panel::new(name, kind));
            },
            None => skipped.push(name),
        }
    }

    // when 2+ continuous numeric columns exist, prepend a correlation-heatmap panel. This is
    // the one panel that re-scans the data (a single extra pass), since Pearson correlations
    // are not in the stats cache; it's prepended so it survives the panel cap below.
    let numeric_indices: Vec<usize> = stats
        .iter()
        .enumerate()
        .filter(|(i, s)| {
            !is_map_col(*i)
                // a dictionary-tagged dimension/geo/temporal/id numeric is NOT a continuous
                // measure, so exclude it from the Pearson matrix (and thus from the corr drill-down
                // and the implicit box-plot pool). A confirmed Measure, or an un-tagged (Defer)
                // numeric, still qualifies — without a dictionary every column is Defer, exactly
                // as before.
                && matches!(col_sems[*i].route, Route::Defer | Route::Measure)
                && matches!(s.r#type.as_str(), "Integer" | "Float")
                && s.cardinality > 1
                && !s.uniqueness_ratio.is_some_and(|r| r > 0.95)
        })
        .map(|(i, _)| i)
        .collect();
    if numeric_indices.len() >= 2 {
        let (mut rdr, headers, nh) = reader_and_headers(args)?;
        let (labels, columns) = read_numeric_columns(&mut rdr, &headers, nh, &numeric_indices)?;
        // need 2+ numeric columns AND 2+ complete rows for a meaningful correlation matrix
        if labels.len() >= 2 && columns.first().is_some_and(|c| c.len() >= 2) {
            let matrix = pearson_matrix(&columns);
            // the most strongly correlated pair drills into the heatmap's headline relationship,
            // but only when it's at least moderately correlated (else it's a noise cloud). All the
            // drill-downs below reuse the columns already read for the matrix (no extra data pass).
            let pair =
                strongest_pair(&matrix).filter(|&(_, _, r)| r.abs() >= SCATTER_PAIR_MIN_ABS_R);

            // pair drill-down beside the heatmap: a 2D density contour for large datasets (a
            // scatter would overplot into a solid mass, and the contour embeds only a fixed grid),
            // or a scatter otherwise. The title carries Pearson r, plus Spearman rho when the two
            // diverge enough to mean the relationship is monotonic-but-curved (nonlinear) — so the
            // reader doesn't take the single r as proof of a linear relationship.
            let pair_panel = pair.map(|(i, j, r)| {
                let rho = spearman_rho(&columns[i], &columns[j]);
                let name = if rho.abs() - r.abs() >= SMART_NONLINEAR_MIN_GAP {
                    format!(
                        "{} vs {} (r={r:.2}, \u{3c1}={rho:.2} \u{2014} nonlinear)",
                        labels[i], labels[j]
                    )
                } else {
                    format!("{} vs {} (r={r:.2})", labels[i], labels[j])
                };
                if columns[i].len() >= SMART_CONTOUR_MIN_POINTS {
                    let (x, y, z) = bin_2d(&columns[i], &columns[j], SMART_CONTOUR_BINS);
                    Panel::new(name, PanelKind::ContourPair { x, y, z })
                } else {
                    let (xs, ys) = downsample_pair(&columns[i], &columns[j], MAX_SMART_POINTS);
                    Panel::new(name, PanelKind::ScatterPair { xs, ys })
                }
            });

            // 3D drill-down: with 3+ numeric columns, a Scatter3D of the strongest pair plus a
            // third axis chosen to be the LEAST redundant with that pair (minimizing
            // max(|r_ik|, |r_jk|)), so the cloud genuinely uses all three dimensions instead of
            // collapsing onto the pair's plane. Skipped entirely when the strongest pair is itself
            // near-collinear (|r| >= SMART_3D_COLLINEAR_MAX_ABS_R): two near-identical axes can't
            // form a non-degenerate 3D, and the 2D pair drill-down already shows that relationship.
            // A 3D scene can't share the typed x/y subplot grid, so it's built ONLY for HTML output
            // (which uses the inline render path, like the map panel). Static image export goes
            // through the typed grid, where a 3D panel would hit `panel_trace`'s unreachable arm.
            let scatter3d_panel = pair
                .filter(|&(_, _, r)| r.abs() < SMART_3D_COLLINEAR_MAX_ABS_R)
                .filter(|_| columns.len() >= 3 && !out_format.is_image())
                .and_then(|(i, j, _)| {
                    let third = least_redundant_third(&matrix, i, j);
                    third.map(|k| {
                        // both downsamples share (n, cap) so they pick the same row indices ->
                        // aligned
                        let (xs, ys) = downsample_pair(&columns[i], &columns[j], MAX_SMART_POINTS);
                        let (_, zs) = downsample_pair(&columns[i], &columns[k], MAX_SMART_POINTS);
                        Panel::new(
                            format!("{} / {} / {} (3D)", labels[i], labels[j], labels[k]),
                            PanelKind::Scatter3D {
                                xs,
                                ys,
                                zs,
                                labels: (labels[i].clone(), labels[j].clone(), labels[k].clone()),
                            },
                        )
                    })
                });

            // Show only the lower triangle: a correlation matrix mirrors across the diagonal (and
            // the diagonal is a trivial 1.0), so masking the upper half + diagonal drops redundant
            // cells. `matrix` was already consumed by `strongest_pair`/the 3D third-axis pick above
            // (which need the full square), so masking here only affects the rendered panel.
            panels.insert(
                0,
                Panel::new(
                    "Correlation".to_string(),
                    PanelKind::CorrHeatmap {
                        labels,
                        matrix: mask_to_lower_triangle(matrix),
                    },
                ),
            );
            // place the drill-down panels right after the heatmap
            let mut at = 1;
            if let Some(panel) = pair_panel {
                panels.insert(at, panel);
                at += 1;
            }
            if let Some(panel) = scatter3d_panel {
                panels.insert(at, panel);
            }
        }
    }

    // prepend a categorical part-to-whole hierarchy (treemap/sunburst) when 2+ low-cardinality
    // dimensions exist. HTML-only: like Scatter3D/Map, a domain-based trace can't compose with the
    // typed x/y subplot grid, so it forces the inline render path. The chosen dimensions also keep
    // their individual frequency-bar panels — the hierarchy is a cross-dimensional overview, not a
    // replacement. The chart type is auto-selected by depth (treemap for shallow, sunburst for
    // deep) unless --hierarchy-style forces one. Prepended so it survives the panel cap.
    if !out_format.is_image() {
        // eligible dims = genuine categorical (String) freq-bar columns with enough distinct
        // values to be worth nesting (HIER_MIN_DIM_CARDINALITY..=CATEGORICAL_MAX_CARDINALITY).
        // Restricting to String type excludes numeric codes (low-card Integer/Float also become
        // freq bars) and booleans, which make poor — and surprising — hierarchy levels. Sort
        // ascending by cardinality so the coarsest grouping is the outermost level/ring.
        let mut dims: Vec<(usize, u64, String)> = panels
            .iter()
            .filter_map(|p| match p.kind {
                PanelKind::FreqBar { idx } => {
                    let s = &stats[idx];
                    let card = s.cardinality;
                    (s.r#type == "String"
                        && (HIER_MIN_DIM_CARDINALITY..=CATEGORICAL_MAX_CARDINALITY).contains(&card))
                    .then(|| (idx, card, p.name.clone()))
                },
                _ => None,
            })
            .collect();
        if dims.len() >= HIER_MIN_DIMS {
            dims.sort_by_key(|&(idx, card, _)| (card, idx));
            dims.truncate(HIER_MAX_DEPTH);
            let depth = dims.len();
            let style = resolve_hierarchy_style(args.flag_hierarchy_style.as_deref(), depth)?;
            let dim_idxs: Vec<usize> = dims.iter().map(|&(idx, ..)| idx).collect();
            let leaves = collect_hierarchy_counts(args, &dim_idxs, None)?;
            let title = dims
                .iter()
                .map(|(.., name)| name.as_str())
                .collect::<Vec<_>>()
                .join(" › ");
            // Don't AUTO-nest statistically independent dimensions: a treemap/sunburst of
            // independent categoricals just replicates each level's marginal at every branch,
            // conveying nothing the separate frequency bars don't already show more legibly. An
            // explicit `--hierarchy-style treemap|sunburst` is a deliberate request, so it bypasses
            // the screen. The association is read off the joint counts already in hand (no
            // re-scan).
            let explicit_style = matches!(
                args.flag_hierarchy_style
                    .as_deref()
                    .map(str::to_ascii_lowercase)
                    .as_deref(),
                Some("treemap" | "sunburst")
            );
            let assoc = max_pairwise_cramers_v(&leaves, depth);
            if !explicit_style && assoc < HIER_MIN_ASSOCIATION_CRAMERS_V {
                eprintln!(
                    "viz smart: skipping the '{title}' hierarchy panel — its dimensions are \
                     statistically independent (max Cramér's V={assoc:.2} < \
                     {HIER_MIN_ASSOCIATION_CRAMERS_V:.2}); the per-column frequency bars convey \
                     the same information. Use --hierarchy-style treemap|sunburst to force it."
                );
            } else if let Some((labels, parents, values, ids)) =
                hierarchy_arrays(&leaves, depth, HIER_TOP_N_PER_LEVEL, HIER_MAX_NODES, "All")
            {
                panels.insert(
                    0,
                    Panel::new(
                        title,
                        PanelKind::Hierarchy {
                            style,
                            labels,
                            parents,
                            values,
                            ids,
                        },
                    ),
                );
            }
        }
    }

    // prepend a time-series trend panel when the data has a date/datetime column and a
    // continuous numeric column. Like the correlation panel, it does one extra data pass and
    // is prepended so it survives the panel cap.
    // mirror the DMY preference stats used to infer dates: `viz smart` builds stats with
    // flag_prefer_dmy = false, and stats itself ORs in QSV_PREFER_DMY (see `cmd::stats`), so the
    // effective preference is the env flag. Parse dates the same way here so DMY-formatted dates
    // are ordered correctly rather than misparsed/dropped.
    let prefer_dmy = util::get_envvar_flag("QSV_PREFER_DMY");
    let grain = dict_data.as_ref().and_then(|d| d.grain.as_deref());
    if let Some(panel) =
        build_timeseries_panel(args, &stats, prefer_dmy, map_cols, &col_sems, grain)?
    {
        panels.insert(0, panel);
    }

    // prepend the geographic overview panels (built up front, above) so they lead the dashboard and
    // survive the panel cap. Insert the per-country choropleth first, then the point map at index
    // 0, yielding [map, choropleth, ...] — the point map for spatial detail, the choropleth
    // beside it for the per-jurisdiction aggregate.
    if let Some(panel) = choropleth_panel {
        panels.insert(0, panel);
    }
    if let Some((panel, _)) = map_panel {
        panels.insert(0, panel);
    }

    // decide the rendering path. Up to MAX_SUBPLOTS panels always use the typed subplot grid
    // (the only form that supports static image export). For HTML output, more than that
    // switches to an inline-div grid (up to MAX_PANELS_INLINE). Image export can't assemble
    // multiple plots, so it stays capped at MAX_SUBPLOTS (with a warning if more were eligible).
    //
    // `--max-charts 0` (the default) means "auto": fit as many panels as the data warrants —
    // every eligible column for HTML (bounded by MAX_PANELS_INLINE), or MAX_SUBPLOTS for image
    // export. An explicit `--max-charts N` caps the panel count to N instead.
    let is_html = matches!(out_format, OutFormat::Html);

    // non-cartesian panels (mapbox map, geo projection, or 3D scatter) can't share the typed x/y
    // subplot grid, so any of them forces the inline render path. All are HTML-only (never built
    // for image export above).
    let has_noncartesian = panels.iter().any(|p| {
        matches!(
            p.kind,
            PanelKind::Map { .. }
                | PanelKind::Geo { .. }
                | PanelKind::Choropleth { .. }
                | PanelKind::Scatter3D { .. }
                | PanelKind::Hierarchy { .. }
        )
    });

    let eligible = panels.len();
    // default (--max-charts 0) draws every eligible panel up to MAX_PANELS_INLINE, for both HTML
    // and static image export. Static export of >MAX_SUBPLOTS panels is rendered via raw-JSON
    // domain-positioned axes (see render_smart_grid_json), so it's no longer capped at 8.
    let requested = if args.flag_max_charts == 0 {
        MAX_PANELS_INLINE
    } else {
        args.flag_max_charts
    };
    let want = requested.min(eligible);
    // HTML with >MAX_SUBPLOTS panels (or any non-cartesian panel) uses the inline-div grid.
    // Static image export always uses a single composition (typed grid for ≤MAX_SUBPLOTS, raw
    // JSON beyond), so it never takes the inline path; non-cartesian panels stay HTML-only.
    let inline = is_html && (want > MAX_SUBPLOTS || has_noncartesian);

    let max_panels = requested.min(MAX_PANELS_INLINE);

    if panels.len() > max_panels {
        for p in panels.drain(max_panels..) {
            skipped.push(p.name);
        }
    }
    if panels.is_empty() {
        return fail_clierror!(
            "No chartable columns found for `viz smart` (all columns were empty or too \
             high-cardinality to summarize)."
        );
    }
    if !skipped.is_empty() {
        eprintln!(
            "viz smart: charting {} column(s); skipped {}: {}",
            panels.len(),
            skipped.len(),
            skipped.join(", ")
        );
    }

    // gather frequency counts for the bar panels in a single pass
    let bar_indices: Vec<usize> = panels
        .iter()
        .filter_map(|p| match p.kind {
            PanelKind::FreqBar { idx } => Some(idx),
            PanelKind::BoxStats { .. }
            | PanelKind::BoxRaw { .. }
            | PanelKind::BoxOutliers { .. }
            | PanelKind::CorrHeatmap { .. }
            | PanelKind::TimeSeries { .. }
            | PanelKind::ScatterPair { .. }
            | PanelKind::ContourPair { .. }
            | PanelKind::Scatter3D { .. }
            | PanelKind::Histogram { .. }
            | PanelKind::Map { .. }
            | PanelKind::Geo { .. }
            | PanelKind::Choropleth { .. }
            | PanelKind::Hierarchy { .. } => None,
        })
        .collect();
    let top_n = args.flag_limit.max(1);
    let freq = if bar_indices.is_empty() {
        HashMap::new()
    } else {
        // Prefer a pre-existing `frequency` cache (no data pass); fall back to a
        // single-pass recompute when it's absent/stale/incompatible.
        match freq_from_cache(args, &stats, &bar_indices, top_n) {
            Some(cached) => cached,
            None => count_values(args, &bar_indices, top_n)?,
        }
    };

    // gather raw values for the panels that need them — histogram panels (moarstats-flagged
    // bimodal columns) and opt-in raw box panels (`--box-points`) — in a single batched pass.
    // Taken only when at least one such panel exists.
    let raw_indices: Vec<usize> = panels
        .iter()
        .filter_map(|p| match p.kind {
            PanelKind::Histogram { idx } | PanelKind::BoxRaw { idx, .. } => Some(idx),
            _ => None,
        })
        .collect();
    // large box panels that have outliers: collect ONLY their out-of-fence values, keyed by the
    // Tukey fences resolved during classification (folded into the same pass as `raw_indices`).
    let fence_bounds: HashMap<usize, (f64, f64)> = panels
        .iter()
        .filter_map(|p| match p.kind {
            PanelKind::BoxOutliers {
                idx,
                fence_low,
                fence_high,
                ..
            } => Some((idx, (fence_low, fence_high))),
            _ => None,
        })
        .collect();
    let (raw_values, outlier_stats) = if raw_indices.is_empty() && fence_bounds.is_empty() {
        (HashMap::new(), HashMap::new())
    } else {
        collect_smart_values(args, &raw_indices, &fence_bounds)?
    };

    // dashboard title: the user's --title, else the dataset's file name
    let title_text = args.flag_title.clone().unwrap_or_else(|| {
        let dataset = std::path::Path::new(args.arg_input.as_deref().unwrap_or("data"))
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("data");
        format!("{dataset} \u{2014} data overview")
    });

    let log_scale = parse_log_scale(&args.flag_log_scale)?;
    // a Geo panel in image mode must use the raw-JSON grid: it injects a domain-positioned `geo`
    // subplot, which the typed `Layout` (a single `.geo()`, no per-cell domain) can't express.
    let has_geo_image = out_format.is_image()
        && panels
            .iter()
            .any(|p| matches!(p.kind, PanelKind::Geo { .. }));
    if inline {
        Ok(SmartRender::Inline(render_smart_inline(
            args,
            &panels,
            &freq,
            &raw_values,
            &outlier_stats,
            &title_text,
            log_scale,
        )))
    } else if panels.len() > MAX_SUBPLOTS || has_geo_image {
        // static image export of >MAX_SUBPLOTS panels (or any panel count with a geo subplot):
        // plotly's typed Layout only has xaxis1..xaxis8 and a single typed `geo`, so assemble the
        // grid as raw JSON with domain-positioned xaxis9+ and (for geo panels) geo/geo2+ subplots.
        render_smart_grid_json(
            args,
            &panels,
            &freq,
            &raw_values,
            &outlier_stats,
            &title_text,
            log_scale,
        )
    } else {
        render_smart_grid(
            args,
            &panels,
            &freq,
            &raw_values,
            &outlier_stats,
            &title_text,
            log_scale,
        )
    }
}

/// Build the plotly trace for one smart-dashboard panel. `axes` carries the subplot axis refs
/// when rendering into the typed grid; pass `None` for a standalone inline-div plot (which uses
/// the default x/y axes). Returns the trace plus, for bar panels, the tallest bar value (used
/// to add vertical headroom for the outside value labels) and whether that panel's y-axis
/// should be logarithmic (per `--log-scale`; always `false` for non-bar panels).
fn panel_trace(
    panel: &Panel,
    color: &'static str,
    freq: &FreqMap,
    hist: &HashMap<usize, Vec<f64>>,
    outliers: &HashMap<usize, OutlierStats>,
    axes: Option<(String, String)>,
    theme: Option<BuiltinTheme>,
    log_scale: LogScale,
) -> (Box<dyn Trace>, Option<f64>, bool) {
    let mut bar_max: Option<f64> = None;
    let mut log_y = false;
    // bar value-label font: omit the explicit color so the label inherits the layout font color
    // (qsv's ink in the unthemed look, the template's font when themed) -- this lets the dark/light
    // toggle's `font.color` relayout flip the labels instead of leaving them dark on a dark page.
    let label_font = {
        let f = Font::new().size(9);
        if theme.is_some() {
            f
        } else {
            f.family(FONT_FAMILY)
        }
    };
    let trace: Box<dyn Trace> = match &panel.kind {
        PanelKind::BoxStats {
            q1,
            median,
            q3,
            lower,
            upper,
            mean,
        } => {
            let mut b = BoxPlot::new(Vec::<f64>::new())
                .name(panel.name.clone())
                .q1(vec![*q1])
                .median(vec![*median])
                .q3(vec![*q3])
                .marker(Marker::new().color(color))
                // show only the y stats in the hover ("median: 202.771k"), not plotly's default
                // "(<trace name>, median: ...)" which repeats the (long) column name on every
                // statistic line — the column name is already the panel title.
                .hover_info(HoverInfo::Y);
            if let Some((x, y)) = &axes {
                b = b.x_axis(x.clone()).y_axis(y.clone());
            }
            if let Some(l) = lower {
                b = b.lower_fence(vec![*l]);
            }
            if let Some(u) = upper {
                b = b.upper_fence(vec![*u]);
            }
            if let Some(m) = mean {
                b = b.mean(vec![*m]);
            }
            b
        },
        PanelKind::BoxRaw { idx, points } => {
            // opt-in / heuristic raw box: plotly computes the quartiles + true Tukey whiskers from
            // the values and overlays the sample points per this panel's chosen --box-points mode
            let values = hist.get(idx).cloned().unwrap_or_default();
            let mut b = BoxPlot::new(values)
                .name(panel.name.clone())
                .quartile_method(QuartileMethod::Linear)
                .box_points(points.clone())
                .marker(Marker::new().color(color))
                // show only the y stats in the hover ("median: 202.771k"), not plotly's default
                // "(<trace name>, median: ...)" which repeats the (long) column name on every
                // statistic line — the column name is already the panel title.
                .hover_info(HoverInfo::Y);
            if let Some((x, y)) = &axes {
                b = b.x_axis(x.clone()).y_axis(y.clone());
            }
            b
        },
        PanelKind::BoxOutliers {
            idx,
            q1,
            median,
            q3,
            mean,
            ..
        } => {
            // large column WITH outliers: a precomputed quartile box whose whiskers end at the
            // observed in-fence extremes, with the out-of-fence values overlaid as NATIVE box
            // points. The outliers are passed as a 2D `y` (`[[...]]`, via `Y = Vec<f64>`), which
            // makes plotly draw them as points WITHOUT recomputing the box from them — a 1D `y`
            // renders the box but drops the points.
            let stats = outliers.get(idx);
            let pts = stats.map(|o| o.outliers.clone()).unwrap_or_default();
            let whisker_low = stats.map_or(*q1, |o| o.whisker_low);
            let whisker_high = stats.map_or(*q3, |o| o.whisker_high);
            let mut b = BoxPlot::<f64, Vec<f64>>::new(vec![pts])
                .name(panel.name.clone())
                .q1(vec![*q1])
                .median(vec![*median])
                .q3(vec![*q3])
                .lower_fence(vec![whisker_low])
                .upper_fence(vec![whisker_high])
                .box_points(BoxPoints::All)
                .point_pos(0.0)
                .jitter(0.0)
                .marker(Marker::new().color(color).size(4))
                .hover_info(HoverInfo::Y);
            if let Some(m) = mean {
                b = b.mean(vec![*m]);
            }
            if let Some((x, y)) = &axes {
                b = b.x_axis(x.clone()).y_axis(y.clone());
            }
            b
        },
        PanelKind::FreqBar { idx } => {
            let bars = freq.get(idx).cloned().unwrap_or_default();
            // x = each bar's distinct category-axis key (real value, or aggregate sentinel) so
            // distinct categories never collapse onto the same plotly category; the friendly,
            // truncated tick labels are applied separately via freq_bar_tick_text.
            let xs: Vec<String> = bars.iter().map(|b| b.x_key.clone()).collect();
            let ys: Vec<f64> = bars.iter().map(|b| b.count as f64).collect();
            // muted-grey the aggregate "(NULL)" / "Other (N)" buckets so they read as summary
            // bars, visually distinct from the palette-colored real categories. Color is driven
            // by the bar's kind (not its label), so a real category named like an aggregate is
            // never mis-colored.
            let bar_colors: Vec<&'static str> = bars
                .iter()
                .map(|b| match b.kind {
                    FreqBarKind::Aggregate => MUTED_COLOR,
                    FreqBarKind::Category => color,
                })
                .collect();
            bar_max = Some(ys.iter().copied().fold(0.0_f64, f64::max));
            // high dynamic range (a dominating "(NULL)"/"Other" bucket) -> log y-axis so the
            // small real categories stay visible; gated by the resolved --log-scale mode.
            log_y = panel_is_log(panel, freq, log_scale);
            // on a log y-axis, hatch the muted-grey aggregate bars as a redundant cue
            // (alongside the "count (log)" axis title) that the axis is non-linear
            let mut marker = Marker::new().color_array(bar_colors);
            if let Some(shapes) = freq_bar_pattern_shapes(&bars, log_y) {
                marker = marker.pattern(Pattern::new().shape_array(shapes));
            }
            let mut bar = Bar::new(xs, ys)
                .name(panel.name.clone())
                .marker(marker)
                // value labels above each bar, SI-formatted ("258k", "1.05M") to match
                // the axis ticks
                .text_template("%{y:.3s}")
                .text_position(TextPosition::Outside)
                // don't clip the outside label of the tallest bar at the cell's top edge. On a
                // log axis the fixed top headroom shrinks to a few pixels when counts span many
                // decades (a dominant "(NULL)"/"Other" bucket), so its value label would otherwise
                // be cut off; let it draw into the inter-row gap instead.
                .clip_on_axis(false)
                .text_font(label_font);
            if let Some((x, y)) = &axes {
                bar = bar.x_axis(x.clone()).y_axis(y.clone());
            }
            bar
        },
        PanelKind::Histogram { idx } => {
            let values = hist.get(idx).cloned().unwrap_or_default();
            let mut h = Histogram::new(values)
                .name(panel.name.clone())
                .marker(Marker::new().color(color));
            if let Some((x, y)) = &axes {
                h = h.x_axis(x.clone()).y_axis(y.clone());
            }
            h
        },
        PanelKind::TimeSeries { y_label, xs, ys } => {
            let mut t = Scatter::new(xs.clone(), ys.clone())
                .mode(Mode::Lines)
                .name(y_label.clone())
                .line(Line::new().color(color));
            if let Some((x, y)) = &axes {
                t = t.x_axis(x.clone()).y_axis(y.clone());
            }
            t
        },
        PanelKind::ScatterPair { xs, ys } => {
            let mut t = Scatter::new(xs.clone(), ys.clone())
                .mode(Mode::Markers)
                .name(panel.name.clone())
                .marker(Marker::new().color(color));
            if let Some((x, y)) = &axes {
                t = t.x_axis(x.clone()).y_axis(y.clone());
            }
            t
        },
        PanelKind::ContourPair { x, y, z } => {
            // standalone (inline) panels show the colorbar; grid cells hide it to avoid clutter
            let mut c = Contour::new(x.clone(), y.clone(), z.clone())
                .color_scale(ColorScale::Palette(ColorScalePalette::Viridis))
                .show_scale(axes.is_none());
            if let Some((xa, ya)) = &axes {
                c = c.x_axis(xa.as_str()).y_axis(ya.as_str());
            }
            c
        },
        PanelKind::CorrHeatmap { labels, matrix } => corr_heatmap_trace(
            labels
                .iter()
                .map(|l| truncate_label(l, CORR_LABEL_MAX_CHARS))
                .collect(),
            matrix.clone(),
            axes.clone(),
            // standalone (inline) panels show the colorbar; grid panels use in-cell labels
            axes.is_none(),
        ),
        // map / geo / 3D / hierarchy panels use a non-cartesian layout (mapbox, geo projection,
        // 3D scene, or domain-based treemap/sunburst) that can't share the typed x/y subplot grid,
        // so they are rendered entirely by `smart_inline_panel_plot` and never reach this
        // assembler.
        PanelKind::Map { .. }
        | PanelKind::Geo { .. }
        | PanelKind::Choropleth { .. }
        | PanelKind::Scatter3D { .. }
        | PanelKind::Hierarchy { .. } => {
            unreachable!(
                "map/geo/choropleth/3D/hierarchy panels are rendered via the inline path, not \
                 panel_trace"
            )
        },
    };
    (trace, bar_max, log_y)
}

/// The reusable building blocks of a smart-dashboard subplot grid, shared by the typed-`Layout`
/// renderer (`render_smart_grid`, ≤ `MAX_SUBPLOTS` panels) and the raw-JSON renderer
/// (`render_smart_grid_json`, used for static image export of more panels). Each axis pair in
/// `axes` already has its paper-domain and cross-anchor applied; `base_layout` carries the
/// page chrome (size/margins/fonts/background) but neither the per-subplot axes nor the
/// annotations, so each assembler can stamp those on in its own way.
struct SmartGridParts {
    traces:      Vec<Box<dyn Trace>>,
    axes:        Vec<(usize, Axis, Axis)>,
    // geo panels don't use cartesian x/y axes: each carries a domain-positioned `geo{pos}` layout
    // object (as raw JSON, since the typed `Layout` has only one `.geo()`), injected by
    // `render_smart_grid_json`. Empty for the typed-`Layout` grid (geo dashboards never use it).
    geos:        Vec<(usize, serde_json::Value)>,
    annotations: Vec<Annotation>,
    base_layout: Layout,
    theme:       Option<BuiltinTheme>,
    dims:        (usize, usize),
}

/// Build the shared pieces of a smart-dashboard subplot grid: one trace per panel (each
/// referencing its own `x{n}`/`y{n}` axes), the domain-positioned + cross-anchored axis pairs,
/// the dashboard + per-panel title annotations, and the base page layout. We lay out each cell's
/// axes with explicit paper-domains (rather than a plotly `grid`) so we can (a) scale the plot
/// height with the row count, (b) reserve a band at the top for the dashboard title, and (c)
/// place each column's name as a title *above* its panel.
fn smart_grid_parts(
    args: &Args,
    panels: &[Panel],
    freq: &FreqMap,
    hist: &HashMap<usize, Vec<f64>>,
    outliers: &HashMap<usize, OutlierStats>,
    title_text: &str,
    log_scale: LogScale,
) -> SmartGridParts {
    let cols = args.flag_grid_cols.clamp(1, panels.len());
    // each leading overview panel takes a full-width row; the rest pack into `cols`-wide rows.
    let (geoms, rows) = smart_grid_layout(panels, cols);

    // widen the left margin when a correlation panel is present so its (long) numeric-column
    // tick labels — truncated to CORR_LABEL_MAX_CHARS — aren't clipped against the page edge.
    let left_margin = panels
        .iter()
        .find_map(|p| match &p.kind {
            PanelKind::CorrHeatmap { labels, .. } => Some(labels),
            PanelKind::BoxStats { .. }
            | PanelKind::BoxRaw { .. }
            | PanelKind::BoxOutliers { .. }
            | PanelKind::FreqBar { .. }
            | PanelKind::TimeSeries { .. }
            | PanelKind::ScatterPair { .. }
            | PanelKind::ContourPair { .. }
            | PanelKind::Scatter3D { .. }
            | PanelKind::Histogram { .. }
            | PanelKind::Map { .. }
            | PanelKind::Geo { .. }
            | PanelKind::Choropleth { .. }
            | PanelKind::Hierarchy { .. } => None,
        })
        .map_or(DEFAULT_LEFT_MARGIN_PX, |labels| {
            let longest = labels
                .iter()
                .map(|l| l.chars().count().min(CORR_LABEL_MAX_CHARS))
                .max()
                .unwrap_or(0);
            (longest * CORR_LABEL_PX_PER_CHAR + 24).max(DEFAULT_LEFT_MARGIN_PX)
        });
    // log freq panels carry a rotated y-axis title cue; reserve extra left room (shared across
    // the single-Plot grid) so it isn't clipped against the page edge.
    let left_margin = if panels.iter().any(|p| panel_is_log(p, freq, log_scale)) {
        left_margin + LOG_AXIS_TITLE_MARGIN_PX
    } else {
        left_margin
    };

    // when a theme is set, let its plotly template drive backgrounds/fonts/axis chrome;
    // otherwise apply qsv's built-in look. `themed` gates the explicit overrides below.
    let theme = args.theme();
    let themed = theme.is_some();
    // a dashboard text font: set the family but never an explicit color, so the text inherits the
    // layout font color (qsv's ink unthemed, the template's font when themed). Inheriting -- rather
    // than baking in ink -- lets the dark/light toggle's `font.color` relayout flip these labels.
    let ann_font = |size: usize| {
        let f = Font::new().size(size);
        if themed { f } else { f.family(FONT_FAMILY) }
    };

    let mut base_layout = Layout::new()
        .show_legend(false)
        .height(rows * ROW_HEIGHT_PX)
        .margin(
            Margin::new()
                .top(TOP_MARGIN_PX)
                .bottom(BOTTOM_MARGIN_PX)
                .left(left_margin)
                .right(40)
                .pad(4),
        );
    if !themed {
        base_layout = base_layout
            .font(Font::new().family(FONT_FAMILY).color(INK).size(12))
            .paper_background_color(PAPER_BG)
            .plot_background_color(PAPER_BG);
    }

    // annotations: the dashboard title (in the reserved top strip) plus one title per panel
    let mut annotations: Vec<Annotation> = Vec::with_capacity(panels.len() + 1);
    annotations.push(
        Annotation::new()
            .text(title_text)
            .x(0.5)
            .y(1.0)
            .x_ref("paper")
            .y_ref("paper")
            .x_anchor(Anchor::Center)
            .y_anchor(Anchor::Bottom)
            .show_arrow(false)
            .font(ann_font(20)),
    );

    let mut traces: Vec<Box<dyn Trace>> = Vec::with_capacity(panels.len());
    let mut axes: Vec<(usize, Axis, Axis)> = Vec::with_capacity(panels.len());
    let mut geos: Vec<(usize, serde_json::Value)> = Vec::new();

    for (n, panel) in panels.iter().enumerate() {
        let pos = n + 1;
        let xref = axis_ref('x', pos);
        let yref = axis_ref('y', pos);
        let color = PALETTE[n % PALETTE.len()];

        // geo panels use a `geo{pos}` projection subplot (no cartesian x/y axes), only reachable on
        // the raw-JSON static-export path. Build the ScatterGeo trace, fit the projection to the
        // data extent, and emit the domain-positioned geo layout object as raw JSON (the typed
        // `Layout` has only a single `.geo()` with no per-cell domain).
        if let PanelKind::Geo {
            lats,
            lons,
            outlier_lats,
            outlier_lons,
        } = &panel.kind
        {
            let geom = geoms[n].clone();
            // a static image can't be panned/zoomed, so when there are outliers the projection AND
            // framing must cover the FULL plotted extent (core + outliers), not the core alone —
            // otherwise a stray outside the US could leave the projection on `albers-usa` (a fixed
            // US composite, no fitted axes) and clip it. Derive the frame from the actual plotted
            // coords so this holds even without geocode metadata (the outlier markers render either
            // way). Pass the bounding-box CORNERS (not the raw points) so geo_framing's 2.5% trim
            // can't drop the handful of outliers. Without outliers, frame from the core points.
            let (frame_lats, frame_lons) = if outlier_lats.is_empty() {
                (lats.clone(), lons.clone())
            } else {
                let lat_iter = || lats.iter().chain(outlier_lats.iter()).copied();
                let lon_iter = || lons.iter().chain(outlier_lons.iter()).copied();
                let min_lat = lat_iter().fold(f64::INFINITY, f64::min);
                let max_lat = lat_iter().fold(f64::NEG_INFINITY, f64::max);
                let min_lon = lon_iter().fold(f64::INFINITY, f64::min);
                let max_lon = lon_iter().fold(f64::NEG_INFINITY, f64::max);
                (
                    vec![max_lat, max_lat, min_lat, min_lat, max_lat],
                    vec![min_lon, max_lon, max_lon, min_lon, min_lon],
                )
            };
            let (projection, lonaxis, lataxis) = geo_framing(&frame_lats, &frame_lons);
            // for a fitted (Mercator) extent, refine the axis ranges with the tighter extent-fit
            // padding — but only from a concrete extent that already covers everything plotted: the
            // full extent if we have it, else the core extent ONLY when there are no outliers. With
            // outliers but no full extent (e.g. it was antimeridian-filtered), keep the
            // plotted-coords frame so the outlier markers aren't cropped back to the core. Whole-
            // region projections (albers-usa / natural-earth) return no axes and need no refining.
            #[cfg(feature = "geocode")]
            let (lonaxis, lataxis) = match &panel.geo_meta {
                Some(meta) if lonaxis.is_some() || lataxis.is_some() => {
                    match meta
                        .full_extent
                        .as_ref()
                        .or_else(|| outlier_lats.is_empty().then_some(&meta.extent))
                    {
                        Some(ext) => {
                            let (lon, lat) = extent_geo_axes(ext);
                            (Some(lon), Some(lat))
                        },
                        None => (lonaxis, lataxis),
                    }
                },
                _ => (lonaxis, lataxis),
            };
            let mut geo = LayoutGeo::new()
                .projection(Projection::new().projection_type(projection))
                .showland(true)
                .landcolor(NamedColor::LightGray)
                .showocean(true)
                .oceancolor(NamedColor::LightBlue)
                .showlakes(true)
                .lakecolor(NamedColor::LightBlue)
                .showcountries(true);
            if let Some(lonaxis) = lonaxis {
                geo = geo.lonaxis(lonaxis);
            }
            if let Some(lataxis) = lataxis {
                geo = geo.lataxis(lataxis);
            }
            traces.push(
                ScatterGeo::new(lats.clone(), lons.clone())
                    .mode(Mode::Markers)
                    .marker(Marker::new().color(color).opacity(MAP_POINT_OPACITY))
                    .subplot(geo_ref(pos)),
            );
            // geographic outliers as a distinct amber/X marker trace on top of the core points
            if !outlier_lats.is_empty() {
                traces.push(
                    ScatterGeo::new(outlier_lats.clone(), outlier_lons.clone())
                        .name("geographic outliers")
                        .mode(Mode::Markers)
                        .marker(outlier_marker_geo())
                        .show_legend(false)
                        .subplot(geo_ref(pos)),
                );
            }
            // spatial-extent overlay (bounding box + reverse-geocoded corner/center markers),
            // both bound to this cell's `geo{pos}` subplot, plus a consolidated summary
            // annotation below the cell. Hover labels are inert in static images.
            #[cfg(feature = "geocode")]
            if let Some(meta) = &panel.geo_meta {
                // full-extent box (core + outliers), no fill, drawn first/underneath
                if let Some(full) = &meta.full_extent {
                    let (flat, flon) = extent_box_latlon(full);
                    traces.push(
                        ScatterGeo::new(flat, flon)
                            .name("full extent (incl. outliers)")
                            .mode(Mode::Lines)
                            .line(full_extent_box_line())
                            .hover_info(HoverInfo::Skip)
                            .show_legend(false)
                            .subplot(geo_ref(pos)),
                    );
                }
                let (blat, blon) = extent_box_latlon(&meta.extent);
                traces.push(
                    ScatterGeo::new(blat, blon)
                        .name("spatial extent")
                        .mode(Mode::Lines)
                        .line(extent_box_line())
                        .fill(plotly::traces::scatter_geo::Fill::ToSelf)
                        .fill_color(GEO_EXTENT_FILL_COLOR)
                        .hover_info(HoverInfo::Skip)
                        .show_legend(false)
                        .subplot(geo_ref(pos)),
                );
                let mlat: Vec<f64> = meta.points.iter().map(|p| p.lat).collect();
                let mlon: Vec<f64> = meta.points.iter().map(|p| p.lon).collect();
                let htext: Vec<String> = meta.points.iter().map(point_hover_text).collect();
                traces.push(
                    ScatterGeo::new(mlat, mlon)
                        .name("extent points")
                        .mode(Mode::Markers)
                        .marker(extent_marker_geo())
                        .hover_text_array(htext)
                        .hover_info(HoverInfo::Text)
                        .show_legend(false)
                        .subplot(geo_ref(pos)),
                );
                if !meta.summary.is_empty() {
                    annotations.push(
                        Annotation::new()
                            .text(format!("Spatial extent: {}", meta.summary))
                            .x(geom.title_x)
                            .y((geom.y_domain[0] - GEO_META_OFFSET).max(0.0))
                            .x_ref("paper")
                            .y_ref("paper")
                            .x_anchor(Anchor::Center)
                            .y_anchor(Anchor::Top)
                            .show_arrow(false)
                            .font(ann_font(11)),
                    );
                }
            }
            let mut geo_json = serde_json::to_value(&geo).unwrap_or(serde_json::Value::Null);
            if let Some(obj) = geo_json.as_object_mut() {
                obj.insert(
                    "domain".to_string(),
                    serde_json::json!({ "x": geom.x_domain, "y": geom.y_domain }),
                );
            }
            geos.push((pos, geo_json));
            annotations.push(
                Annotation::new()
                    .text(panel.name.clone())
                    .x(geom.title_x)
                    .y(geom.title_y)
                    .x_ref("paper")
                    .y_ref("paper")
                    .x_anchor(Anchor::Center)
                    .y_anchor(Anchor::Bottom)
                    .show_arrow(false)
                    .font(ann_font(13)),
            );
            continue;
        }

        let is_box = matches!(
            panel.kind,
            PanelKind::BoxStats { .. } | PanelKind::BoxRaw { .. } | PanelKind::BoxOutliers { .. }
        );
        let is_date = matches!(panel.kind, PanelKind::TimeSeries { .. });
        let (trace, bar_max, log_y) = panel_trace(
            panel,
            color,
            freq,
            hist,
            outliers,
            Some((xref.clone(), yref.clone())),
            theme,
            log_scale,
        );
        traces.push(trace);

        // build this subplot's styled, domain-positioned, cross-anchored axes and add its title
        // above the cell. x-axis anchors to its paired y-axis and vice versa.
        let geom = geoms[n].clone();
        let x_axis = styled_x_axis(is_box, is_date, theme, freq_bar_tick_text(panel, freq))
            .domain(&geom.x_domain)
            .anchor(yref.clone());
        let y_axis = styled_y_axis(bar_max, log_y, theme)
            .domain(&geom.y_domain)
            .anchor(xref.clone());
        axes.push((pos, x_axis, y_axis));
        annotations.push(
            Annotation::new()
                .text(panel.name.clone())
                .x(geom.title_x)
                .y(geom.title_y)
                .x_ref("paper")
                .y_ref("paper")
                .x_anchor(Anchor::Center)
                .y_anchor(Anchor::Bottom)
                .show_arrow(false)
                .font(ann_font(13)),
        );

        // in-cell `r` value labels for the correlation panel, drawn only when the matrix is
        // small enough to stay legible in one dashboard cell. Category axes index annotations
        // by serial number (0-based), and the text flips to white on the dark high-|r| cells
        // for contrast against the RdBu scale.
        if let PanelKind::CorrHeatmap { matrix, .. } = &panel.kind
            && matrix.len() <= CORR_INCELL_MAX_N
        {
            for ann in corr_incell_annotations(matrix, &xref, &yref) {
                annotations.push(ann);
            }
        }
    }

    SmartGridParts {
        traces,
        axes,
        geos,
        annotations,
        base_layout,
        theme,
        dims: (cols * SMART_COL_WIDTH_PX, rows * ROW_HEIGHT_PX),
    }
}

fn render_smart_grid(
    args: &Args,
    panels: &[Panel],
    freq: &FreqMap,
    hist: &HashMap<usize, Vec<f64>>,
    outliers: &HashMap<usize, OutlierStats>,
    title_text: &str,
    log_scale: LogScale,
) -> CliResult<SmartRender> {
    let SmartGridParts {
        traces,
        axes,
        // geo dashboards are routed to `render_smart_grid_json` (a geo subplot can't live in the
        // typed `Layout`), so `geos` is always empty here.
        geos: _,
        annotations,
        mut base_layout,
        theme,
        dims,
    } = smart_grid_parts(args, panels, freq, hist, outliers, title_text, log_scale);

    let mut plot = Plot::new();
    for trace in traces {
        plot.add_trace(trace);
    }
    for (pos, x_axis, y_axis) in axes {
        base_layout = assign_typed_axis(base_layout, pos, x_axis, y_axis);
    }
    base_layout = base_layout.annotations(annotations);

    plot.set_layout(apply_theme(base_layout, theme));
    Ok(SmartRender::Grid {
        plot: Box::new(plot),
        dims,
        title: title_text.to_string(),
        theme,
    })
}

fn render_smart_grid_page(mut plot: Plot, theme: Option<BuiltinTheme>, title_text: &str) -> String {
    // match the responsiveness the single-`Plot` HTML path applies in `run`.
    plot.set_configuration(Configuration::new().responsive(true));
    // raw strings (actual newlines, real quotes) so rustfmt's `format_strings` can't split a `\n`
    // or `\"` escape across a line wrap and corrupt the markup (see `smart_html_page`).
    let extra_style = r#"  .qsv-viz-grid { width: 100%; }
  .qsv-viz-plot { width: 100%; }"#;
    let inner = plot.to_inline_html(Some("qsv-viz-smart-grid"));
    let body = format!(
        r#"<div class="qsv-viz-grid">
      <div class="qsv-viz-plot">
{inner}
      </div>
</div>"#
    );
    // the typed plot already carries the dashboard title in its layout, so suppress the page <h1>.
    smart_html_page(title_text, theme, extra_style, &body, false)
}

/// Render the dashboard as a raw Plotly JSON value with domain-positioned axes, for static image
/// export of > `MAX_SUBPLOTS` panels. plotly's typed `Layout` only has `x_axis1..x_axis8`, but
/// plotly.js itself supports any number of axes — so we serialize the plot, then inject each
/// cell's `xaxis{n}`/`yaxis{n}` (the traces already reference them by name). The result is fed to
/// `StaticExporter::write_fig`, which renders an arbitrary JSON value through the same
/// headless-browser backend as `Plot::write_image`.
fn render_smart_grid_json(
    args: &Args,
    panels: &[Panel],
    freq: &FreqMap,
    hist: &HashMap<usize, Vec<f64>>,
    outliers: &HashMap<usize, OutlierStats>,
    title_text: &str,
    log_scale: LogScale,
) -> CliResult<SmartRender> {
    let SmartGridParts {
        traces,
        axes,
        geos,
        annotations,
        base_layout,
        theme,
        dims,
    } = smart_grid_parts(args, panels, freq, hist, outliers, title_text, log_scale);

    let mut plot = Plot::new();
    for trace in traces {
        plot.add_trace(trace);
    }
    plot.set_layout(apply_theme(base_layout.annotations(annotations), theme));

    let mut value: serde_json::Value = serde_json::from_str(&plot.to_json()).map_err(|e| {
        crate::CliError::Other(format!("viz smart: could not assemble dashboard JSON: {e}"))
    })?;
    let Some(layout_obj) = value
        .get_mut("layout")
        .and_then(serde_json::Value::as_object_mut)
    else {
        return fail_clierror!("viz smart: assembled dashboard JSON has no layout object");
    };
    for (pos, x_axis, y_axis) in &axes {
        layout_obj.insert(
            axis_json_key('x', *pos),
            serde_json::to_value(x_axis).map_err(|e| {
                crate::CliError::Other(format!("viz smart: could not serialize x-axis {pos}: {e}"))
            })?,
        );
        layout_obj.insert(
            axis_json_key('y', *pos),
            serde_json::to_value(y_axis).map_err(|e| {
                crate::CliError::Other(format!("viz smart: could not serialize y-axis {pos}: {e}"))
            })?,
        );
    }
    // geo panels: inject each domain-positioned `geo{pos}` layout object (its trace already carries
    // a matching `subplot` reference). Pre-serialized as JSON because the typed `Layout` can't hold
    // more than one geo subplot nor a per-cell domain.
    for (pos, geo) in geos {
        layout_obj.insert(geo_ref(pos), geo);
    }

    Ok(SmartRender::GridJson {
        value: Box::new(value),
        dims,
    })
}

/// In-cell `r` value annotations for a correlation matrix, referenced to the given axes (use
/// "x"/"y" for a standalone plot). Undefined correlations (heatmap gaps) get no label; the text
/// flips to white on dark high-|r| cells for contrast against the RdBu scale.
fn corr_incell_annotations(matrix: &[Vec<f64>], xref: &str, yref: &str) -> Vec<Annotation> {
    let mut out = Vec::new();
    for (i, row_vals) in matrix.iter().enumerate() {
        for (j, &r) in row_vals.iter().enumerate() {
            if !r.is_finite() {
                continue;
            }
            let text_color = if r.abs() >= 0.5 { "#FFFFFF" } else { INK };
            out.push(
                Annotation::new()
                    .text(format!("{r:.2}"))
                    .x(j as f64)
                    .y(i as f64)
                    .x_ref(xref)
                    .y_ref(yref)
                    .x_anchor(Anchor::Center)
                    .y_anchor(Anchor::Middle)
                    .show_arrow(false)
                    .font(Font::new().family(FONT_FAMILY).color(text_color).size(9)),
            );
        }
    }
    out
}

/// Build a standalone themed `Plot` for one panel, used as a cell in the inline-div dashboard.
fn smart_inline_panel_plot(
    panel: &Panel,
    color: &'static str,
    freq: &FreqMap,
    hist: &HashMap<usize, Vec<f64>>,
    outliers: &HashMap<usize, OutlierStats>,
    theme: Option<BuiltinTheme>,
    log_scale: LogScale,
) -> Plot {
    // when a theme is set, its template drives backgrounds/fonts; otherwise apply qsv's look.
    let themed = theme.is_some();
    // overview panels (map/geo, correlation, time-series, …) render a little taller than the
    // per-column box/bar/histogram panels.
    let row_height = panel_render_height(&panel.kind);
    // map panels use a mapbox layout (tile basemap, framed to the points) instead of cartesian
    // x/y axes, so they're assembled here rather than through the shared `panel_trace`/axis path.
    if let PanelKind::Map {
        lats,
        lons,
        density,
        outlier_lats,
        outlier_lons,
    } = &panel.kind
    {
        // smart auto panel: trim outliers so a few bad geocodes don't blow up the default view
        #[cfg_attr(not(feature = "geocode"), expect(unused_mut))]
        let (mut center, mut zoom) = map_center_zoom(
            lats,
            lons,
            MAP_FRAME_TRIM_FRAC,
            MAP_PANEL_ASSUMED_WIDTH_PX,
            MAP_PANEL_USABLE_HEIGHT_PX,
        );
        // frame the tight CORE extent (outliers are drawn distinctly and reachable via the "Full
        // extent" zoom button below).
        #[cfg(feature = "geocode")]
        let mut extent_menu: Option<UpdateMenu> = None;
        #[cfg(feature = "geocode")]
        if let Some(meta) = &panel.geo_meta {
            let (c, z) = extent_center_zoom(&meta.extent);
            center = c;
            zoom = z;
            // with geographic outliers, offer Core/Full extent zoom buttons: the map opens tight on
            // the core, and "Full extent" reveals the strays without manual panning/zooming.
            if let Some(full) = &meta.full_extent {
                extent_menu = Some(extent_zoom_menu(&meta.extent, full));
            }
        }
        let mut plot = Plot::new();
        if *density {
            // many points overplot into a solid mass as markers, so aggregate into a heatmap
            plot.add_trace(
                DensityMapbox::new(lats.clone(), lons.clone(), vec![1.0_f64; lats.len()])
                    .radius(MAP_SMART_DENSITY_RADIUS_PX),
            );
        } else {
            plot.add_trace(
                ScatterMapbox::new(lats.clone(), lons.clone())
                    .mode(Mode::Markers)
                    .marker(Marker::new().color(color).opacity(MAP_POINT_OPACITY)),
            );
        }
        // geographic outliers as a distinct amber marker trace on top of the core points/heatmap
        if !outlier_lats.is_empty() {
            plot.add_trace(
                ScatterMapbox::new(outlier_lats.clone(), outlier_lons.clone())
                    .name("geographic outliers")
                    .mode(Mode::Markers)
                    .marker(outlier_marker_mapbox())
                    .show_legend(false),
            );
        }
        #[cfg(feature = "geocode")]
        if let Some(meta) = &panel.geo_meta {
            add_extent_overlay_mapbox(&mut plot, meta);
        }
        let mut layout = Layout::new()
            .show_legend(false)
            .height(row_height)
            .title(Title::with_text(panel.name.clone()))
            .margin(Margin::new().top(48).bottom(20).left(20).right(20).pad(4))
            .mapbox(
                Mapbox::new()
                    .style(MapboxStyle::OpenStreetMap)
                    .center(center)
                    .zoom(zoom),
            );
        #[cfg(feature = "geocode")]
        if let Some(menu) = extent_menu {
            layout = layout.update_menus(vec![menu]);
        }
        if !themed {
            layout = layout
                .font(Font::new().family(FONT_FAMILY).color(INK).size(12))
                .paper_background_color(PAPER_BG);
        }
        plot.set_layout(apply_theme(layout, theme));
        plot.set_configuration(Configuration::new().responsive(true));
        return plot;
    }

    // geo panels use a `geo` projection layout (no tiles, fully offline) instead of cartesian
    // x/y axes, so they're assembled here like the mapbox map panel above.
    if let PanelKind::Geo {
        lats,
        lons,
        outlier_lats,
        outlier_lons,
    } = &panel.kind
    {
        let mut plot = Plot::new();
        plot.add_trace(
            ScatterGeo::new(lats.clone(), lons.clone())
                .mode(Mode::Markers)
                .marker(Marker::new().color(color).opacity(MAP_POINT_OPACITY)),
        );
        // geographic outliers as a distinct amber/X marker trace on top of the core points
        if !outlier_lats.is_empty() {
            plot.add_trace(
                ScatterGeo::new(outlier_lats.clone(), outlier_lons.clone())
                    .name("geographic outliers")
                    .mode(Mode::Markers)
                    .marker(outlier_marker_geo())
                    .show_legend(false),
            );
        }
        #[cfg(feature = "geocode")]
        if let Some(meta) = &panel.geo_meta {
            add_extent_overlay_geo(&mut plot, meta);
        }
        let geo = LayoutGeo::new()
            .projection(Projection::new().projection_type(ProjectionType::NaturalEarth))
            .showland(true)
            .landcolor(NamedColor::LightGray)
            .showocean(true)
            .oceancolor(NamedColor::LightBlue)
            .showlakes(true)
            .lakecolor(NamedColor::LightBlue)
            .showcountries(true);
        let mut layout = Layout::new()
            .show_legend(false)
            .height(row_height)
            .title(Title::with_text(panel.name.clone()))
            .margin(Margin::new().top(48).bottom(20).left(20).right(20).pad(4))
            .geo(geo);
        if !themed {
            layout = layout
                .font(Font::new().family(FONT_FAMILY).color(INK).size(12))
                .paper_background_color(PAPER_BG);
        }
        plot.set_layout(apply_theme(layout, theme));
        plot.set_configuration(Configuration::new().responsive(true));
        return plot;
    }

    // choropleth panels fill whole countries on a `geo` projection layout (no tiles), colored by
    // the per-country point count; assembled here like the geo point map above.
    if let PanelKind::Choropleth {
        locations,
        z,
        location_mode,
    } = &panel.kind
    {
        let mut plot = Plot::new();
        plot.add_trace(
            Choropleth::new(locations.clone(), z.clone())
                .location_mode(location_mode.clone())
                .color_scale(ColorScale::Palette(ColorScalePalette::Viridis))
                .show_scale(true)
                .color_bar(ColorBar::new().title("count"))
                .marker(ChoroplethMarker::new().line(Line::new().width(0.5))),
        );
        let geo = LayoutGeo::new()
            .projection(Projection::new().projection_type(ProjectionType::NaturalEarth))
            .showland(true)
            .landcolor(NamedColor::LightGray)
            .showcountries(true);
        let mut layout = Layout::new()
            .show_legend(false)
            .height(row_height)
            .title(Title::with_text(panel.name.clone()))
            .margin(Margin::new().top(48).bottom(20).left(20).right(20).pad(4))
            .geo(geo);
        if !themed {
            layout = layout
                .font(Font::new().family(FONT_FAMILY).color(INK).size(12))
                .paper_background_color(PAPER_BG);
        }
        plot.set_layout(apply_theme(layout, theme));
        plot.set_configuration(Configuration::new().responsive(true));
        return plot;
    }

    // 3D scatter panels use a `scene` (3D) layout instead of cartesian x/y axes, so they're
    // assembled here as well.
    if let PanelKind::Scatter3D { xs, ys, zs, labels } = &panel.kind {
        let (x_label, y_label, z_label) = labels;
        let mut plot = Plot::new();
        plot.add_trace(
            Scatter3D::new(xs.clone(), ys.clone(), zs.clone())
                .mode(Mode::Markers)
                .marker(Marker::new().color(color).opacity(MAP_POINT_OPACITY)),
        );
        let scene = LayoutScene::new()
            .x_axis(Axis::new().title(Title::with_text(x_label.clone())))
            .y_axis(Axis::new().title(Title::with_text(y_label.clone())))
            .z_axis(Axis::new().title(Title::with_text(z_label.clone())));
        let mut layout = Layout::new()
            .show_legend(false)
            .height(row_height)
            .title(Title::with_text(panel.name.clone()))
            .margin(Margin::new().top(48).bottom(20).left(20).right(20).pad(4))
            .scene(scene);
        if !themed {
            layout = layout
                .font(Font::new().family(FONT_FAMILY).color(INK).size(12))
                .paper_background_color(PAPER_BG);
        }
        plot.set_layout(apply_theme(layout, theme));
        plot.set_configuration(Configuration::new().responsive(true));
        return plot;
    }

    // hierarchy (treemap/sunburst) panels are domain-based — no cartesian x/y axes — so, like the
    // map/geo/3D panels above, they own their whole Plot here.
    if let PanelKind::Hierarchy {
        style,
        labels,
        parents,
        values,
        ids,
    } = &panel.kind
    {
        let mut plot = Plot::new();
        plot.add_trace(hierarchy_trace(*style, labels, parents, values, ids));
        let mut layout = Layout::new()
            .show_legend(false)
            .height(row_height)
            .title(Title::with_text(panel.name.clone()))
            .margin(Margin::new().top(48).bottom(20).left(20).right(20).pad(4));
        if !themed {
            layout = layout
                .font(Font::new().family(FONT_FAMILY).color(INK).size(12))
                .paper_background_color(PAPER_BG);
        }
        plot.set_layout(apply_theme(layout, theme));
        plot.set_configuration(Configuration::new().responsive(true));
        return plot;
    }

    let is_box = matches!(
        panel.kind,
        PanelKind::BoxStats { .. } | PanelKind::BoxRaw { .. } | PanelKind::BoxOutliers { .. }
    );
    let is_corr = matches!(panel.kind, PanelKind::CorrHeatmap { .. });
    let is_date = matches!(panel.kind, PanelKind::TimeSeries { .. });
    let (trace, bar_max, log_y) =
        panel_trace(panel, color, freq, hist, outliers, None, theme, log_scale);

    let mut plot = Plot::new();
    plot.add_trace(trace);

    // correlation cells need extra left room for tick labels and right room for the colorbar;
    // log freq cells need a little extra left room for the rotated y-axis title cue.
    let (left, right) = if is_corr {
        (110, 90)
    } else if log_y {
        (60 + LOG_AXIS_TITLE_MARGIN_PX, 30)
    } else {
        (60, 30)
    };
    let mut layout = Layout::new()
        .show_legend(false)
        .height(row_height)
        .title(Title::with_text(panel.name.clone()))
        .margin(
            Margin::new()
                .top(48)
                .bottom(60)
                .left(left)
                .right(right)
                .pad(4),
        )
        .x_axis(styled_x_axis(
            is_box,
            is_date,
            theme,
            freq_bar_tick_text(panel, freq),
        ))
        .y_axis(styled_y_axis(bar_max, log_y, theme));
    if !themed {
        layout = layout
            .font(Font::new().family(FONT_FAMILY).color(INK).size(12))
            .paper_background_color(PAPER_BG)
            .plot_background_color(PAPER_BG);
    }

    if let PanelKind::CorrHeatmap { matrix, .. } = &panel.kind
        && matrix.len() <= CORR_INCELL_MAX_N
    {
        layout = layout.annotations(corr_incell_annotations(matrix, "x", "y"));
    }

    plot.set_layout(apply_theme(layout, theme));
    plot.set_configuration(Configuration::new().responsive(true));
    plot
}

fn render_smart_inline(
    args: &Args,
    panels: &[Panel],
    freq: &FreqMap,
    hist: &HashMap<usize, Vec<f64>>,
    outliers: &HashMap<usize, OutlierStats>,
    title_text: &str,
    log_scale: LogScale,
) -> String {
    let cols = args.flag_grid_cols.clamp(1, panels.len().max(1));
    let theme = args.theme();

    // raw strings (actual newlines, real quotes) throughout so rustfmt's `format_strings` can't
    // split a `\n`/`\"` escape across a line wrap and corrupt the markup (see `smart_html_page`).
    let mut cells = String::new();
    for (n, panel) in panels.iter().enumerate() {
        let color = PALETTE[n % PALETTE.len()];
        let plot = smart_inline_panel_plot(panel, color, freq, hist, outliers, theme, log_scale);
        let div_id = format!("qsv-viz-panel-{n}");
        // leading overview panels span the full page width (their own grid row).
        if is_overview_panel(&panel.kind) {
            cells.push_str("    <div class=\"qsv-viz-cell full-width\">\n");
        } else {
            cells.push_str("    <div class=\"qsv-viz-cell\">\n");
        }
        // wrap the plot in a fixed-height box so plotly's responsive `height:100%` div resolves to
        // a concrete height instead of expanding to fill the whole cell (which would otherwise
        // overlap/clip any caption rendered below it).
        let plot_height = panel_render_height(&panel.kind);
        cells.push_str(&format!(
            r#"      <div class="qsv-viz-plot" style="height:{plot_height}px">
"#
        ));
        cells.push_str(&plot.to_inline_html(Some(&div_id)));
        cells.push_str("\n      </div>\n");
        // reverse-geocoded spatial-extent summary caption, shown below a map panel.
        #[cfg(feature = "geocode")]
        if let Some(meta) = &panel.geo_meta
            && !meta.summary.is_empty()
        {
            cells.push_str(&format!(
                r#"      <div class="qsv-viz-geo-meta">Spatial extent: {}</div>
"#,
                html_escape(&meta.summary)
            ));
        }
        cells.push_str("    </div>\n");
    }

    let extra_style = format!(
        r#"  .qsv-viz-grid {{ display: grid; grid-template-columns: repeat({cols}, minmax(0, 1fr)); gap: 16px; }}
  .qsv-viz-cell {{ min-width: 0; }}
  .qsv-viz-cell.full-width {{ grid-column: 1 / -1; }}
  .qsv-viz-plot {{ width: 100%; }}"#
    );
    let body = format!(
        r#"<div class="qsv-viz-grid">
{cells}</div>"#
    );
    // panels carry no overall title, so the dashboard title is shown as the page <h1>.
    smart_html_page(title_text, theme, &extra_style, &body, true)
}

/// Minimal HTML-escaping for text interpolated into the inline dashboard page.
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// Try to satisfy the bar-panel frequency counts from a pre-existing `frequency`
/// JSONL cache (`qsv frequency --frequency-jsonl`), avoiding the extra full-data
/// pass `count_values` does. Returns `None` — so the caller falls back to
/// `count_values` — when no compatible cache exists or any requested bar column
/// is absent from it (e.g. a high-cardinality/all-unique sentinel column).
///
/// On a hit, each column's non-null pairs are re-sorted (count desc, then value asc),
/// truncated to `top_n`, and given the same aggregate `(NULL)` / `Other (N)` buckets
/// (via `finalize_freq_bars`) as the `count_values` path, so cached and recomputed bars
/// are identical. The cache stores complete per-value data including the empty/null bucket,
/// which `read_frequency_cache_view` surfaces as `FreqCacheColumn::null_count`. (The cache is
/// always whitespace-trimmed — `--frequency-jsonl` is incompatible with `--no-trim` — matching
/// the trim `count_values` applies.)
fn freq_from_cache(
    args: &Args,
    stats: &[crate::cmd::stats::StatsData],
    bar_indices: &[usize],
    top_n: usize,
) -> Option<FreqMap> {
    let path = args.arg_input.as_ref()?;
    let rconfig = Config::new(Some(path))
        .delimiter(args.flag_delimiter)
        .no_headers_flag(args.flag_no_headers);
    let no_headers = rconfig.no_headers;

    // `viz` never sets --no-nulls on the underlying cache read, so it can only reuse a
    // default (nulls-kept) cache; the null bucket is then surfaced as the "(NULL)" bar
    // (unless the user passed --no-nulls, which `finalize_freq_bars` honors).
    let view = crate::cmd::frequency::read_frequency_cache_view(
        std::path::Path::new(path),
        false,
        no_headers,
        args.flag_delimiter,
    )?;

    let mut out = HashMap::with_capacity(bar_indices.len());
    for &idx in bar_indices {
        // In --no-headers mode the cache keys columns positionally ("1", "2", …),
        // matching how `frequency` (and stats) name them; otherwise use the field
        // name. Any column missing from the cache abandons the fast path entirely
        // (return None) so output never silently mixes cached and recomputed bars.
        let key = if no_headers {
            (idx + 1).to_string()
        } else {
            stats.get(idx)?.field.clone()
        };
        let col = view.columns.get(&key)?;
        let mut pairs = col.pairs.clone();
        pairs.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        out.insert(
            idx,
            finalize_freq_bars(
                pairs,
                col.null_count,
                top_n,
                args.flag_no_nulls,
                args.flag_no_other,
            ),
        );
    }
    Some(out)
}

/// Per-bar pattern (hatch) shapes for a frequency panel, or `None` when no hatching applies.
/// On a log y-axis, the dominant `(NULL)`/`Other` aggregate bars are diagonally hatched as a
/// redundant cue that the axis is non-linear; real categories stay flat-filled. Returns `None`
/// on linear panels so the marker is built exactly as before. Hatching is keyed on the bar's
/// `kind` (not its label), so a real category literally named "Other" is never hatched.
fn freq_bar_pattern_shapes(bars: &[FreqBar], log_y: bool) -> Option<Vec<PatternShape>> {
    if !log_y {
        return None;
    }
    Some(
        bars.iter()
            .map(|b| match b.kind {
                FreqBarKind::Aggregate => PatternShape::RightDiagonalLine,
                FreqBarKind::Category => PatternShape::None,
            })
            .collect(),
    )
}

/// Turn a column's non-null `(value, count)` pairs into the panel's frequency bars, appending
/// the aggregate `(NULL)` and `Other (N)` buckets to match `qsv frequency`'s default output.
/// `counts` is the full set of non-null pairs (already sorted: count desc, then value asc);
/// `null_count` is the number of empty cells. The top `top_n` non-null categories are kept,
/// and the rest are summarized as a single `Other (N)` bar where N = the count of distinct
/// categories dropped. Both aggregate buckets are appended AFTER the real categories (`(NULL)`
/// first, then `Other`) and can be suppressed with `no_nulls` / `no_other`.
fn finalize_freq_bars(
    counts: Vec<(String, u64)>,
    null_count: u64,
    top_n: usize,
    no_nulls: bool,
    no_other: bool,
) -> Vec<FreqBar> {
    let other_unique = counts.len().saturating_sub(top_n);
    let other_count: u64 = counts.iter().skip(top_n).map(|(_, c)| *c).sum();
    let mut bars: Vec<FreqBar> = counts
        .into_iter()
        .take(top_n)
        .map(|(label, count)| FreqBar {
            x_key: label.clone(),
            label,
            count,
            kind: FreqBarKind::Category,
        })
        .collect();

    // derive the aggregate bars' axis keys against the real category keys (and each other) so
    // they're provably unique — see `push_aggregate_bar`.
    let mut used: std::collections::HashSet<String> =
        bars.iter().map(|b| b.x_key.clone()).collect();
    if !no_nulls && null_count > 0 {
        push_aggregate_bar(&mut bars, &mut used, NULL_TEXT.to_string(), null_count);
    }
    if !no_other && other_count > 0 {
        push_aggregate_bar(
            &mut bars,
            &mut used,
            format!("{OTHER_TEXT} ({})", HumanCount(other_unique as u64)),
            other_count,
        );
    }
    bars
}

/// Append a synthetic aggregate frequency bar (a `(NULL)` or `Other (N)` bucket) to `bars`.
/// Its category-axis key is derived from the display label plus an `AGG_KEY_SENTINEL` suffix,
/// extended (sentinel repeated) until it is provably distinct from every key already in `used`
/// — the real category keys plus any earlier aggregate. This guarantees the aggregate can never
/// collapse onto a real category's bar on the plotly axis, even if a real value happens to equal
/// the sentinel-suffixed form. `used` is updated with the chosen key.
fn push_aggregate_bar(
    bars: &mut Vec<FreqBar>,
    used: &mut std::collections::HashSet<String>,
    label: String,
    count: u64,
) {
    let mut x_key = format!("{label}{AGG_KEY_SENTINEL}");
    while used.contains(&x_key) {
        x_key.push(AGG_KEY_SENTINEL);
    }
    used.insert(x_key.clone());
    bars.push(FreqBar {
        x_key,
        label,
        count,
        kind: FreqBarKind::Aggregate,
    });
}

/// Count value occurrences for the given column indices in a single pass, returning the
/// top-N `(value, count)` pairs per column (sorted by count desc, then value asc), plus the
/// aggregate `(NULL)` / `Other (N)` buckets appended by `finalize_freq_bars` (honoring
/// `--no-nulls` / `--no-other`). Values are ASCII-whitespace-trimmed before counting, matching
/// `qsv frequency`'s default, so whitespace-only cells count as NULL and the raw-scan path
/// stays consistent with the always-trimmed frequency cache.
fn count_values(args: &Args, indices: &[usize], top_n: usize) -> CliResult<FreqMap> {
    let rconfig = Config::new(args.arg_input.as_ref())
        .delimiter(args.flag_delimiter)
        .no_headers_flag(args.flag_no_headers);
    let mut rdr = rconfig.reader()?;

    let mut maps: HashMap<usize, HashMap<Vec<u8>, u64>> =
        indices.iter().map(|&i| (i, HashMap::new())).collect();
    // empty cells are tallied here (not in `maps`, so they're never labeled "") and surfaced
    // as a "(NULL)" bar by `finalize_freq_bars`.
    let mut null_counts: HashMap<usize, u64> = indices.iter().map(|&i| (i, 0_u64)).collect();

    let mut record = csv::ByteRecord::new();
    while rdr.read_byte_record(&mut record)? {
        for &i in indices {
            if let Some(cell) = record.get(i) {
                // trim ASCII whitespace before the empty/null check (and before counting),
                // matching `qsv frequency`'s default trim — otherwise whitespace-only cells
                // would become a literal blank category here but "(NULL)" from the cache.
                let cell = crate::cmd::frequency::trim_bs_whitespace(cell);
                if cell.is_empty() {
                    if let Some(n) = null_counts.get_mut(&i) {
                        *n += 1;
                    }
                    continue;
                }
                if let Some(m) = maps.get_mut(&i) {
                    *m.entry(cell.to_vec()).or_insert(0) += 1;
                }
            }
        }
    }

    let mut out = HashMap::with_capacity(maps.len());
    for (i, m) in maps {
        let mut counts: Vec<(String, u64)> = m
            .into_iter()
            .map(|(k, c)| (String::from_utf8_lossy(&k).into_owned(), c))
            .collect();
        counts.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        let null_count = null_counts.get(&i).copied().unwrap_or(0);
        out.insert(
            i,
            finalize_freq_bars(
                counts,
                null_count,
                top_n,
                args.flag_no_nulls,
                args.flag_no_other,
            ),
        );
    }
    Ok(out)
}

/// Single-pass accumulation of leaf-path values for a categorical hierarchy (treemap/sunburst).
///
/// For each row, the `dims` columns (outer level first) are read, ASCII-whitespace-trimmed, and
/// empty cells mapped to `(NULL)` — matching the freq-bar panels — to form a `dims.len()`-segment
/// path. The path's accumulator is incremented by `1.0` when `value_idx` is `None` (count of rows)
/// or by the parsed numeric `value_idx` cell otherwise (sum; unparseable/empty cells contribute 0).
/// Returns a map of full-depth path → aggregated value, which `hierarchy_arrays` turns into the
/// flat plotly arrays.
///
/// Hierarchy values must be ADDITIVE (count or sum) so a parent equals the sum of its children
/// under plotly's `branchvalues="total"`; mean/min/max don't roll up a tree and are rejected by the
/// caller. Memory is bounded by the product of the `dims` cardinalities — safe for `viz smart`
/// (each dim ≤ `CATEGORICAL_MAX_CARDINALITY`); the standalone subcommands trust the user's column
/// choice.
fn collect_hierarchy_counts(
    args: &Args,
    dims: &[usize],
    value_idx: Option<usize>,
) -> CliResult<HashMap<Vec<String>, f64>> {
    let rconfig = Config::new(args.arg_input.as_ref())
        .delimiter(args.flag_delimiter)
        .no_headers_flag(args.flag_no_headers);
    let mut rdr = rconfig.reader()?;
    accumulate_hierarchy_counts(&mut rdr, dims, value_idx)
}

/// Accumulate leaf-path values from an already-open reader (positioned past the header). Split out
/// from `collect_hierarchy_counts` so the standalone `viz treemap`/`viz sunburst` subcommands can
/// reuse the single reader from `reader_and_headers` (the header is needed first to resolve
/// `--cols`/`--value`), reading streamed stdin exactly once.
fn accumulate_hierarchy_counts(
    rdr: &mut csv::Reader<Box<dyn std::io::Read + Send>>,
    dims: &[usize],
    value_idx: Option<usize>,
) -> CliResult<HashMap<Vec<String>, f64>> {
    let mut leaves: HashMap<Vec<String>, f64> = HashMap::new();
    // value mode (--value): track usable vs unusable measure cells so a typo'd / non-numeric
    // measure column fails loudly instead of silently producing a blank or misleading chart.
    let mut valid_values = 0_usize;
    let mut invalid_values = 0_usize;
    let mut record = csv::ByteRecord::new();
    while rdr.read_byte_record(&mut record)? {
        let mut path = Vec::with_capacity(dims.len());
        for &d in dims {
            let seg = match record.get(d) {
                Some(cell) => {
                    let cell = crate::cmd::frequency::trim_bs_whitespace(cell);
                    if cell.is_empty() {
                        NULL_TEXT.to_string()
                    } else {
                        String::from_utf8_lossy(cell).into_owned()
                    }
                },
                None => NULL_TEXT.to_string(),
            };
            path.push(seg);
        }
        match value_idx {
            // count mode: every row contributes 1 to its leaf.
            None => *leaves.entry(path).or_insert(0.0) += 1.0,
            // value mode: sum a finite, non-negative measure. An empty cell is a benign missing
            // measure (skipped, no leaf created); a non-numeric / negative / non-finite cell can't
            // size an area/angle, so it's tallied and the pass errors afterwards. This avoids
            // silently dropping rows (which would misstate every part-to-whole proportion) or
            // coercing bad data to 0 and rendering a blank/misleading sector.
            Some(vi) => {
                let cell = record
                    .get(vi)
                    .map(crate::cmd::frequency::trim_bs_whitespace)
                    .unwrap_or_default();
                if cell.is_empty() {
                    continue;
                }
                match std::str::from_utf8(cell)
                    .ok()
                    .and_then(|s| s.parse::<f64>().ok())
                {
                    Some(v) if v.is_finite() && v >= 0.0 => {
                        valid_values += 1;
                        *leaves.entry(path).or_insert(0.0) += v;
                    },
                    _ => invalid_values += 1,
                }
            },
        }
    }
    if value_idx.is_some() {
        if invalid_values > 0 {
            // A part-to-whole chart that silently dropped these rows would misstate every
            // proportion, so any unusable measure cell is a hard error (not a warning).
            return fail_clierror!(
                "the --value column has {invalid_values} cell(s) that are non-numeric, negative, \
                 or non-finite; a treemap/sunburst would drop them and misstate the totals. Clean \
                 or filter the data first."
            );
        }
        if valid_values == 0 {
            return fail_clierror!(
                "the --value column has no usable values (need finite, non-negative numbers) — \
                 cannot size the chart."
            );
        }
    }
    Ok(leaves)
}

/// Maximum bias-corrected (Bergsma 2013) Cramér's V across every pair of the `ndims` hierarchy
/// dimensions, computed by marginalizing the joint leaf counts to each 2-way contingency table — so
/// it needs NO extra data pass (the counts are already in hand). `viz smart` uses this to avoid
/// auto-nesting statistically independent dimensions: when no pair of levels is associated, a
/// treemap/sunburst merely repeats each level's marginal at every branch and a set of separate bars
/// says the same thing far more legibly.
///
/// The Bergsma bias correction matters here precisely because the joint table can be sparse (the
/// product of cardinalities): the naive V is upward-biased for large, sparse tables, so the
/// correction subtracts the expected-under-independence inflation and shrinks the effective table
/// size. When the correction is undefined for a tiny table (n close to the table size), the pair
/// falls back to the uncorrected Cramér's V so a small-but-associated hierarchy isn't wrongly read
/// as independent. Returns 0.0 only when there are fewer than two dimensions or no pair has a
/// 2x2-or-larger table with n > 1.
fn max_pairwise_cramers_v(leaves: &HashMap<Vec<String>, f64>, ndims: usize) -> f64 {
    if ndims < 2 {
        return 0.0;
    }
    let mut max_v = 0.0_f64;
    for a in 0..ndims {
        for b in (a + 1)..ndims {
            // marginalize the joint counts down to the (a, b) contingency table.
            let mut cell: HashMap<(&str, &str), f64> = HashMap::new();
            let mut row_tot: HashMap<&str, f64> = HashMap::new();
            let mut col_tot: HashMap<&str, f64> = HashMap::new();
            let mut n = 0.0_f64;
            for (path, &v) in leaves {
                if path.len() != ndims {
                    continue; // defensive: ignore malformed paths
                }
                let ra = path[a].as_str();
                let cb = path[b].as_str();
                *cell.entry((ra, cb)).or_insert(0.0) += v;
                *row_tot.entry(ra).or_insert(0.0) += v;
                *col_tot.entry(cb).or_insert(0.0) += v;
                n += v;
            }
            let (r, c) = (row_tot.len(), col_tot.len());
            // need a 2x2 table and at least as many observations as df+1 for a defined corrected V.
            if r < 2 || c < 2 || n <= 1.0 {
                continue;
            }
            // Pearson chi-square — summed over ALL r*c cells, including structural zeros (an
            // observed-0 cell still contributes (0 - e)^2 / e = e), so iterate the label cross
            // product rather than just the observed cells.
            let mut chi2 = 0.0_f64;
            for (&ra, &rt) in &row_tot {
                for (&cb, &ct) in &col_tot {
                    let e = rt * ct / n;
                    if e > 0.0 {
                        let o = cell.get(&(ra, cb)).copied().unwrap_or(0.0);
                        let d = o - e;
                        chi2 += d * d / e;
                    }
                }
            }
            // Bergsma (2013) bias-corrected Cramér's V.
            #[allow(clippy::cast_precision_loss)]
            let (rf, cf) = (r as f64, c as f64);
            let phi2 = chi2 / n;
            let phi2_corr = (phi2 - (rf - 1.0) * (cf - 1.0) / (n - 1.0)).max(0.0);
            let r_corr = rf - (rf - 1.0) * (rf - 1.0) / (n - 1.0);
            let c_corr = cf - (cf - 1.0) * (cf - 1.0) / (n - 1.0);
            let denom = (r_corr - 1.0).min(c_corr - 1.0);
            let v = if denom > 0.0 {
                (phi2_corr / denom).sqrt()
            } else {
                // The bias correction is undefined for a tiny table (n close to the table size
                // shrinks the effective dimensions to <= 1). Falling through here would leave a
                // perfectly-associated small hierarchy reading as V=0 (independent) and wrongly
                // skip it, so fall back to the UNCORRECTED Cramér's V, which is
                // always defined for r,c >= 2 and n > 1 (denom_raw >= 1.0).
                let denom_raw = (rf - 1.0).min(cf - 1.0);
                (phi2 / denom_raw).sqrt()
            };
            max_v = max_v.max(v);
        }
    }
    max_v
}

/// Turn a map of full-depth `leaf path → value` into the flat
/// `(labels, parents, values, ids)` arrays plotly's `Treemap`/`Sunburst` consume.
///
/// A synthetic root (`HIER_ROOT_ID`, label `root_label`, value = grand total) parents the level-1
/// categories. Subtree totals are rolled up to every ancestor (so parent = sum of children, valid
/// under `branchvalues="total"`). Per parent, only the top `top_n` children (value desc, then label
/// asc — same comparator as `count_values`) are kept; any remainder collapses into a single
/// `Other (k)` leaf so the kept children plus `Other` still sum to the parent. `ids` are
/// path-joined with `HIER_PATH_SEP`, so the same child label under two different parents never
/// collides (plotly keys `parents` on `ids`). Expansion stops once `max_nodes` nodes are emitted
/// (capped nodes render as leaves). Returns `None` for degenerate input (no real split anywhere).
fn hierarchy_arrays(
    leaves: &HashMap<Vec<String>, f64>,
    depth: usize,
    top_n: usize,
    max_nodes: usize,
    root_label: &str,
) -> Option<(Vec<String>, Vec<String>, Vec<f64>, Vec<String>)> {
    if leaves.is_empty() || depth == 0 {
        return None;
    }

    // tree[parent_prefix][child_segment] = subtree total under that child.
    let mut tree: HashMap<Vec<String>, HashMap<String, f64>> = HashMap::new();
    for (path, &v) in leaves {
        if path.len() != depth {
            continue; // defensive: ignore malformed paths
        }
        for l in 0..path.len() {
            *tree
                .entry(path[..l].to_vec())
                .or_default()
                .entry(path[l].clone())
                .or_insert(0.0) += v;
        }
    }

    // a meaningful breakdown needs at least one node that actually splits into 2+ children.
    if !tree.values().any(|children| children.len() >= 2) {
        return None;
    }

    let root_children = tree.get(&Vec::<String>::new())?;
    let total: f64 = root_children.values().sum();

    let mut labels = vec![root_label.to_string()];
    let mut parents = vec![String::new()];
    let mut values = vec![total];
    let mut ids = vec![HIER_ROOT_ID.to_string()];
    let mut node_count = 1_usize;

    let mut queue: std::collections::VecDeque<(Vec<String>, String, usize)> =
        std::collections::VecDeque::new();
    queue.push_back((Vec::new(), HIER_ROOT_ID.to_string(), 0));

    while let Some((prefix, prefix_id, level)) = queue.pop_front() {
        if level >= depth || node_count >= max_nodes {
            continue; // leaf level reached, or global cap hit → leave as a leaf
        }
        let Some(children) = tree.get(&prefix) else {
            continue;
        };

        let mut ranked: Vec<(&String, f64)> = children.iter().map(|(k, &v)| (k, v)).collect();
        ranked.sort_by(|a, b| {
            b.1.partial_cmp(&a.1)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.0.cmp(b.0))
        });

        let keep = top_n.min(ranked.len());
        let mut other_value = 0.0;
        let mut other_drops = 0_usize;
        for (i, (seg, v)) in ranked.into_iter().enumerate() {
            if i < keep {
                let child_id = format!("{prefix_id}{HIER_PATH_SEP}{seg}");
                let mut child_path = prefix.clone();
                child_path.push(seg.clone());
                labels.push(seg.clone());
                parents.push(prefix_id.clone());
                values.push(v);
                ids.push(child_id.clone());
                node_count += 1;
                queue.push_back((child_path, child_id, level + 1));
            } else {
                other_value += v;
                other_drops += 1;
            }
        }
        // always emit the Other bucket when dropping, so kept children + Other == parent
        // (keeps `branchvalues="total"` consistent; otherwise plotly shows a phantom gap).
        if other_drops > 0 {
            labels.push(format!("{OTHER_TEXT} ({})", HumanCount(other_drops as u64)));
            parents.push(prefix_id.clone());
            values.push(other_value);
            // prefix_id is unique per parent, so a sentinel suffix is collision-free.
            ids.push(format!("{prefix_id}{HIER_PATH_SEP}{AGG_KEY_SENTINEL}other"));
            node_count += 1;
        }
    }

    // root + at least two real nodes, else it's not worth a panel.
    if node_count < 3 {
        return None;
    }
    Some((labels, parents, values, ids))
}

/// Construct the plotly domain-based trace for a hierarchy panel/chart from its precomputed flat
/// arrays. A treemap is sorted largest-first (best practice for size comparison via area); a
/// sunburst relies on plotly's lineage colorway for deep paths. Both use `branchvalues="total"`,
/// matching the rolled-up subtree totals `hierarchy_arrays` emits. Both label their tiles/rings
/// with the richer `label+value+percent parent`; the sunburst additionally caps the initial view to
/// two rings (see the per-branch comment below for why), with deeper levels reachable via
/// click-to-zoom.
fn hierarchy_trace(
    style: HierStyle,
    labels: &[String],
    parents: &[String],
    values: &[f64],
    ids: &[String],
) -> Box<dyn Trace> {
    match style {
        HierStyle::Treemap => Treemap::new(labels.to_vec(), parents.to_vec())
            .ids(ids.to_vec())
            .values(values.to_vec())
            .branch_values(BranchValues::Total)
            // treemap-specific marker (plotly.rs#406): rounded corners + a thin white tile
            // outline and inner padding so nested rectangles read as distinct, legible tiles.
            // NOTE: the top pad is intentionally left UNSET. plotly draws each parent's label in
            // the tile's top padding band; pinning `top` to a few px collapses that band so the
            // top-level grouping (e.g. plan: Basic/Plus/Pro) renders as bare color with no header.
            // Omitting `top` lets plotly auto-size a header band that fits the label.
            .marker(
                TreemapMarker::new()
                    .corner_radius(4.0)
                    .pad(Pad::new().left(3.0).right(3.0).bottom(3.0))
                    .line(Line::new().width(1.0).color(NamedColor::White)),
            )
            .sort(true)
            .text_info("label+value+percent parent"),
        // the sunburst's initial view is capped to two rings (the first ring is the root, the
        // second ring is the top-level categories) so it isn't overwhelming; deeper levels are
        // still accessible via click-to-zoom.
        HierStyle::Sunburst => Sunburst::new(labels.to_vec(), parents.to_vec())
            .ids(ids.to_vec())
            .values(values.to_vec())
            .branch_values(BranchValues::Total)
            .text_info("label+value+percent parent")
            // radial in-sector text (plotly.js 3.6) lets the label+value+percent run along each
            // ring's spoke, so deep-path sectors stay legible instead of clipping tangential text.
            .inside_text_orientation(InsideTextOrientation::Radial)
            .max_depth(SUNBURST_MAXDEPTH),
    }
}

/// Per-column result for a `BoxOutliers` panel, produced by the single `collect_smart_values`
/// pass: the out-of-fence outlier values (capped, rendered as native box points), the most
/// extreme IN-fence values (the honest Tukey whisker endpoints), and the true uncapped outlier
/// count (for the overflow log).
struct OutlierStats {
    outliers:      Vec<f64>,
    whisker_low:   f64,
    whisker_high:  f64,
    outlier_count: usize,
}

/// One streaming pass serving both `viz smart` raw-value panels and outlier-box panels:
/// - `full_indices` (histograms / small raw boxes) collect EVERY numeric value, each downsampled to
///   at most `MAX_SMART_POINTS` via uniform stride (preserving the distribution shape).
/// - `fence_bounds` (large boxes that have outliers) collect ONLY the out-of-fence values (capped
///   at `SMART_BOX_OUTLIERS_CAP`; never uniform-downsampled, which would drop the extremes) and
///   track the in-fence min/max for honest whisker ends.
///
/// Empty and non-numeric cells are skipped. Folding both into one reader loop means a dashboard
/// mixing histograms and outlier boxes still scans the data only once.
fn collect_smart_values(
    args: &Args,
    full_indices: &[usize],
    fence_bounds: &HashMap<usize, (f64, f64)>,
) -> CliResult<(HashMap<usize, Vec<f64>>, HashMap<usize, OutlierStats>)> {
    let rconfig = Config::new(args.arg_input.as_ref())
        .delimiter(args.flag_delimiter)
        .no_headers_flag(args.flag_no_headers);
    let mut rdr = rconfig.reader()?;

    let mut full: HashMap<usize, Vec<f64>> =
        full_indices.iter().map(|&i| (i, Vec::new())).collect();
    let mut outliers: HashMap<usize, OutlierStats> = fence_bounds
        .keys()
        .map(|&i| {
            (
                i,
                OutlierStats {
                    outliers:      Vec::new(),
                    whisker_low:   f64::INFINITY,
                    whisker_high:  f64::NEG_INFINITY,
                    outlier_count: 0,
                },
            )
        })
        .collect();

    let mut record = csv::ByteRecord::new();
    while rdr.read_byte_record(&mut record)? {
        for &i in full_indices {
            if let Some(v) = parse_f64(record.get(i))
                && let Some(col) = full.get_mut(&i)
            {
                col.push(v);
            }
        }
        for (&i, &(lo, hi)) in fence_bounds {
            if let Some(v) = parse_f64(record.get(i))
                && let Some(os) = outliers.get_mut(&i)
            {
                if v < lo || v > hi {
                    os.outlier_count += 1;
                    if os.outliers.len() < SMART_BOX_OUTLIERS_CAP {
                        os.outliers.push(v);
                    }
                } else {
                    os.whisker_low = os.whisker_low.min(v);
                    os.whisker_high = os.whisker_high.max(v);
                }
            }
        }
    }

    // downsample full columns so the embedded HTML stays small (outlier columns are already capped)
    for col in full.values_mut() {
        if col.len() > MAX_SMART_POINTS {
            let (sampled, _) = downsample_pair(col, col, MAX_SMART_POINTS);
            *col = sampled;
        }
    }

    // degenerate guard: if a column had NO in-fence value (essentially impossible — q1..q3 is
    // within the fences by construction — but be safe), fall the whisker back to the fence.
    for (&i, os) in &mut outliers {
        if !os.whisker_low.is_finite() {
            os.whisker_low = fence_bounds[&i].0;
        }
        if !os.whisker_high.is_finite() {
            os.whisker_high = fence_bounds[&i].1;
        }
        if os.outlier_count > SMART_BOX_OUTLIERS_CAP {
            log::info!(
                "viz smart: column index {i} has {} outliers; showing the first {}",
                os.outlier_count,
                SMART_BOX_OUTLIERS_CAP
            );
        }
    }

    Ok((full, outliers))
}

/// The plotly axis reference string for subplot `pos` (1-based): "x"/"y" for the first,
/// "x2"/"y2", … thereafter.
fn axis_ref(prefix: char, pos: usize) -> String {
    if pos <= 1 {
        prefix.to_string()
    } else {
        format!("{prefix}{pos}")
    }
}

/// Plot-area height (pixels) of a `rows`-row dashboard: total height minus the top/bottom
/// margins. Floored at 1 to avoid division by zero.
fn smart_plot_area_h(rows: usize) -> f64 {
    (rows * ROW_HEIGHT_PX)
        .saturating_sub(TOP_MARGIN_PX + BOTTOM_MARGIN_PX)
        .max(1) as f64
}

/// Top of the subplot band (paper coords) for a `rows`-row dashboard. The strip above it
/// reserves a fixed `TITLE_BAND_PX` pixels (converted to a paper fraction via the plot-area
/// height) for the top row's panel titles, so they always clear the dashboard title — for
/// both short (one-row) and tall (eight-row) dashboards. Capped at half the area.
fn smart_grid_top(rows: usize) -> f64 {
    let band = (TITLE_BAND_PX as f64 / smart_plot_area_h(rows)).min(0.5);
    1.0 - band
}

/// Paper-fraction gap between a cell's top and its title, fixed at `TITLE_OFFSET_PX` pixels
/// so it doesn't scale with dashboard height (which would consume the reserved band).
fn smart_title_offset(rows: usize) -> f64 {
    (TITLE_OFFSET_PX as f64 / smart_plot_area_h(rows)).min(0.05)
}

/// Geometry (in paper coordinates, 0..1) for one subplot cell in the dashboard grid.
#[derive(Clone)]
struct SubplotGeometry {
    x_domain: Vec<f64>,
    y_domain: Vec<f64>,
    title_x:  f64,
    title_y:  f64,
}

/// Whether a panel is a leading "overview" summary — map/geo, correlation heatmap and its
/// scatter/contour/3D drill-downs, or the time-series trend — that should span the full dashboard
/// width, as opposed to a per-column distribution panel (box/bar/histogram) that flows in the
/// multi-column grid below.
fn is_overview_panel(kind: &PanelKind) -> bool {
    match kind {
        PanelKind::CorrHeatmap { .. }
        | PanelKind::ScatterPair { .. }
        | PanelKind::ContourPair { .. }
        | PanelKind::Scatter3D { .. }
        | PanelKind::TimeSeries { .. }
        | PanelKind::Map { .. }
        | PanelKind::Geo { .. }
        | PanelKind::Choropleth { .. }
        | PanelKind::Hierarchy { .. } => true,
        PanelKind::BoxStats { .. }
        | PanelKind::BoxRaw { .. }
        | PanelKind::BoxOutliers { .. }
        | PanelKind::FreqBar { .. }
        | PanelKind::Histogram { .. } => false,
    }
}

/// Inline-dashboard render height (px) for a panel: hierarchy panels get the tallest
/// `HIER_ROW_HEIGHT_PX` (nested rectangles / rings need the room), other overview panels get
/// `OVERVIEW_ROW_HEIGHT_PX`, and everything else the standard `ROW_HEIGHT_PX`.
fn panel_render_height(kind: &PanelKind) -> usize {
    if matches!(kind, PanelKind::Hierarchy { .. }) {
        HIER_ROW_HEIGHT_PX
    } else if is_overview_panel(kind) {
        OVERVIEW_ROW_HEIGHT_PX
    } else {
        ROW_HEIGHT_PX
    }
}

/// Compute the paper-space domains for a cell at grid position (`row`, `col`) in a `rows`×`cols`
/// grid that occupies the vertical band `0.0..=top` (the strip above `top` is left for the
/// dashboard title), plus the (x, y) anchor for the panel's own title, placed `title_offset` (a
/// paper fraction) above the cell. When `full_width` is set the cell spans the entire page width
/// (`x_domain == [0.0, 1.0]`, used for the leading overview panels) rather than a single column.
///
/// The inter-cell gaps are fixed paper fractions for typical dashboards, but the *total* gap is
/// capped (`MAX_TOTAL_HGAP`/`MAX_TOTAL_VGAP`) so cells never collapse to a negative size: a fixed
/// per-gap fraction overflows the `0..=top` band once there are enough rows (e.g. a 42-panel,
/// 21-row dashboard would need 1.8 of vertical gap alone).
fn cell_geometry(
    row: usize,
    col: usize,
    rows: usize,
    cols: usize,
    full_width: bool,
    top: f64,
    title_offset: f64,
) -> SubplotGeometry {
    const HGAP_BASE: f64 = 0.08;
    const VGAP_BASE: f64 = 0.09;
    const MAX_TOTAL_HGAP: f64 = 0.5;
    const MAX_TOTAL_VGAP: f64 = 0.4;

    let cols_f = cols as f64;
    let rows_f = rows as f64;

    let hgap = if cols > 1 {
        HGAP_BASE.min(MAX_TOTAL_HGAP / (cols_f - 1.0))
    } else {
        0.0
    };
    let vgap = if rows > 1 {
        VGAP_BASE.min(MAX_TOTAL_VGAP / (rows_f - 1.0))
    } else {
        0.0
    };

    let cell_w = (1.0 - hgap * (cols_f - 1.0)) / cols_f;
    let cell_h = (top - vgap * (rows_f - 1.0)) / rows_f;

    // a full-width overview cell spans the whole page; otherwise it occupies its single column.
    let (x0, x1) = if full_width {
        (0.0, 1.0)
    } else {
        let x0 = col as f64 * (cell_w + hgap);
        (x0, x0 + cell_w)
    };
    let y1 = top - row as f64 * (cell_h + vgap); // top edge of the cell
    let y0 = y1 - cell_h;

    SubplotGeometry {
        x_domain: vec![x0, x1],
        y_domain: vec![y0, y1],
        title_x:  (x0 + x1) / 2.0,
        title_y:  (y1 + title_offset).min(1.0),
    }
}

/// Lay out the dashboard panels: each leading overview panel (see `is_overview_panel`) takes its
/// own full-width row, while the remaining per-column distribution panels pack into `cols`-wide
/// rows below. Returns the per-panel geometry (indexed like `panels`) and the total row count.
fn smart_grid_layout(panels: &[Panel], cols: usize) -> (Vec<SubplotGeometry>, usize) {
    // assign each panel a (row, col, full_width) placement
    let mut placements: Vec<(usize, usize, bool)> = Vec::with_capacity(panels.len());
    let mut row = 0;
    let mut col_in_row = 0;
    for panel in panels {
        if is_overview_panel(&panel.kind) {
            // close any partially-filled grid row, then take a full-width row of its own
            if col_in_row > 0 {
                row += 1;
                col_in_row = 0;
            }
            placements.push((row, 0, true));
            row += 1;
        } else {
            if col_in_row == cols {
                row += 1;
                col_in_row = 0;
            }
            placements.push((row, col_in_row, false));
            col_in_row += 1;
        }
    }
    let rows = (if col_in_row > 0 { row + 1 } else { row }).max(1);

    let top = smart_grid_top(rows);
    let title_offset = smart_title_offset(rows);
    let geoms = placements
        .into_iter()
        .map(|(r, c, full)| cell_geometry(r, c, rows, cols, full, top, title_offset))
        .collect();
    (geoms, rows)
}

/// For a frequency-bar panel, the display-only truncated x-axis tick labels (in the same order
/// as the bar's x data). Very long category names (full agency names, datetime strings) would
/// otherwise rotate into tall labels that squeeze the plot area and clip the top value labels.
/// Returns `None` for non-bar panels (their axes are left untouched).
fn freq_bar_tick_text(panel: &Panel, freq: &FreqMap) -> Option<Vec<String>> {
    if let PanelKind::FreqBar { idx } = &panel.kind {
        let labels = freq
            .get(idx)?
            .iter()
            .map(|b| truncate_label(&b.label, BAR_LABEL_MAX_CHARS))
            .collect();
        Some(labels)
    } else {
        None
    }
}

/// A styled x-axis for a dashboard panel: no vertical gridlines, a light baseline, and
/// small tick labels. For single-box panels (`is_box`), the lone "0" category tick is
/// meaningless, so its labels and baseline are hidden. For time-series panels (`is_date`),
/// the axis is typed as a date axis so plotly spaces ticks chronologically. `tick_text`, when
/// present (frequency-bar panels), supplies display-only truncated category labels — the bar's
/// x data keeps the full names, so distinct categories never collapse onto one tick.
fn styled_x_axis(
    is_box: bool,
    is_date: bool,
    theme: Option<BuiltinTheme>,
    tick_text: Option<Vec<String>>,
) -> Axis {
    let mut a = Axis::new()
        .show_grid(false)
        .zero_line(false)
        .show_line(true);
    // when themed, let the template style the axis lines/ticks/fonts
    if theme.is_none() {
        a = a
            .line_color(AXIS_LINE)
            .tick_color(AXIS_LINE)
            // no explicit font color: inherit the layout font (ink) so the toggle can flip it.
            .tick_font(Font::new().family(FONT_FAMILY).size(10));
    }
    if is_box {
        a = a.show_tick_labels(false).show_line(false);
    }
    if is_date {
        a = a.type_(AxisType::Date);
    }
    // display-only truncated labels for frequency-bar panels. Force the axis to category mode:
    // a category axis positions categories at integer indices 0..n in x-data order regardless
    // of how the category strings look, so our integer `tickvals` line up with the bars. Without
    // this, numeric ("10", "2") or date-like ("2026-06-21") category values could be inferred as
    // a linear/date axis, leaving the ticks misaligned. The full x category values are unchanged.
    if let Some(labels) = tick_text {
        let positions: Vec<f64> = (0..labels.len()).map(|i| i as f64).collect();
        a = a
            .type_(AxisType::Category)
            .tick_mode(TickMode::Array)
            .tick_values(positions)
            .tick_text(labels);
    }
    a
}

/// A styled y-axis for a dashboard panel: light horizontal gridlines only, small ticks.
/// When `headroom_max` is given (bar panels), the range is fixed so the tallest bar's outside
/// value label has room and isn't clipped at the cell top: `0..=max*1.15` on a linear axis, or
/// (when `log`) a log10 range from just under 1 up to `log10(max)` plus a margin. A `log` axis
/// also carries a `LOG_AXIS_TITLE` ("count (log)") as the visual cue that it's logarithmic.
fn styled_y_axis(headroom_max: Option<f64>, log: bool, theme: Option<BuiltinTheme>) -> Axis {
    let mut a = Axis::new()
        .show_grid(true)
        .grid_width(1)
        .zero_line(false)
        .show_line(false);
    // when themed, let the template style the gridlines/ticks/fonts
    if theme.is_none() {
        a = a
            .grid_color(GRID_COLOR)
            .tick_color(AXIS_LINE)
            // no explicit font color: inherit the layout font (ink) so the toggle can flip it.
            .tick_font(Font::new().family(FONT_FAMILY).size(10));
    }
    match (log, headroom_max) {
        // log axis: plotly ranges are in log10 units. Start a hair below 1 (counts are >= 1) so
        // a count-of-1 bar still draws a visible sliver, and add ~0.15 in log10 (~1.4x) of top
        // headroom for the outside value labels.
        (true, Some(m)) if m > 0.0 => {
            a = a.type_(AxisType::Log).range(vec![-0.05, m.log10() + 0.15]);
        },
        // log axis without a known max (shouldn't happen for bar panels): let plotly autorange.
        (true, _) => {
            a = a.type_(AxisType::Log);
        },
        (false, Some(m)) if m > 0.0 => {
            a = a.range(vec![0.0, m * 1.15]);
        },
        (false, _) => {},
    }
    if log {
        // the visual cue: only log panels get a y-axis title, so a log scale is never mistaken
        // for linear. Linear panels stay title-less to keep the cells compact.
        let mut title_font = Font::new().size(11);
        if theme.is_none() {
            // no explicit color: inherit the layout font (ink) so the toggle can flip it.
            title_font = title_font.family(FONT_FAMILY);
        }
        a = a.title(Title::with_text(LOG_AXIS_TITLE).font(title_font));
    }
    a
}

/// Assign subplot `pos`'s prebuilt axis pair (domains + cross anchors already applied by
/// `smart_grid_parts`) to the typed `Layout` axis fields, which only exist up to 8 (matching
/// `MAX_SUBPLOTS`). Positions beyond 8 can't be expressed by the typed `Layout`; those grids
/// go through `render_smart_grid_json` instead, so this silently drops them as a safeguard.
fn assign_typed_axis(layout: Layout, pos: usize, x_axis: Axis, y_axis: Axis) -> Layout {
    let (x, y) = (x_axis, y_axis);
    match pos {
        1 => layout.x_axis(x).y_axis(y),
        2 => layout.x_axis2(x).y_axis2(y),
        3 => layout.x_axis3(x).y_axis3(y),
        4 => layout.x_axis4(x).y_axis4(y),
        5 => layout.x_axis5(x).y_axis5(y),
        6 => layout.x_axis6(x).y_axis6(y),
        7 => layout.x_axis7(x).y_axis7(y),
        8 => layout.x_axis8(x).y_axis8(y),
        _ => layout,
    }
}

/// The plotly.js layout key for axis `pos` (1-based): `xaxis`/`yaxis` for the first cell,
/// `xaxis{pos}`/`yaxis{pos}` thereafter — matching the `serde(rename = ...)` the typed `Layout`
/// uses, and the `x{pos}`/`y{pos}` refs `axis_ref` stamps on each trace.
fn axis_json_key(prefix: char, pos: usize) -> String {
    if pos <= 1 {
        format!("{prefix}axis")
    } else {
        format!("{prefix}axis{pos}")
    }
}

/// The plotly.js subplot id / layout key for geo panel `pos` (1-based): `geo` for the first geo
/// subplot, `geo{pos}` thereafter. The same string is used both as the trace's `subplot` value
/// and as the layout object key (plotly.js uses identical names for the two).
fn geo_ref(pos: usize) -> String {
    if pos <= 1 {
        "geo".to_string()
    } else {
        format!("geo{pos}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stat(ty: &str, cardinality: u64, uniqueness: Option<f64>) -> crate::cmd::stats::StatsData {
        crate::cmd::stats::StatsData {
            r#type: ty.to_string(),
            cardinality,
            uniqueness_ratio: uniqueness,
            ..Default::default()
        }
    }

    // map / choropleth basemap framing must honor --width/--height for static image exports but use
    // the default aspect for responsive HTML, so a non-default static aspect doesn't crop the
    // GeoJSON extent (the wiring shared by build_map_plot and build_choropleth_plot).
    #[test]
    fn fit_dims_honors_image_aspect_only() {
        // HTML ignores --width/--height and always frames for the representative default aspect
        assert_eq!(
            fit_dims(Some(400), Some(900), OutFormat::Html),
            (DEFAULT_IMG_WIDTH, DEFAULT_IMG_HEIGHT)
        );
        // image exports honor an explicit (here narrow/tall) aspect ...
        assert_eq!(fit_dims(Some(400), Some(900), OutFormat::Png), (400, 900));
        // ... and a wide/short one ...
        assert_eq!(fit_dims(Some(1600), Some(300), OutFormat::Svg), (1600, 300));
        // ... falling back to the defaults for whichever dimension is omitted
        assert_eq!(
            fit_dims(None, Some(900), OutFormat::Jpeg),
            (DEFAULT_IMG_WIDTH, 900)
        );

        // the aspect actually changes the fit zoom: a wide extent frames tighter (higher zoom) in a
        // wide-short panel than in a tall-narrow one, so passing the wrong dims would mis-zoom.
        let lats = [30.0, 45.0];
        let lons = [-124.0, -72.0];
        let (_, zoom_wide) = map_center_zoom(&lats, &lons, 0.0, 1600.0, 300.0);
        let (_, zoom_tall) = map_center_zoom(&lats, &lons, 0.0, 300.0, 1600.0);
        assert!(
            zoom_wide > zoom_tall,
            "wide-short panel should frame a wide extent tighter than a tall-narrow one \
             (zoom_wide={zoom_wide}, zoom_tall={zoom_tall})"
        );
    }

    #[cfg(feature = "geocode")]
    fn gp(tag: &'static str, city: &str, admin1: &str, country: &str) -> GeoPoint {
        GeoPoint {
            tag,
            lat: 0.0,
            lon: 0.0,
            label: Some(crate::cmd::geocode::GeoLabel {
                city:    city.to_string(),
                admin1:  admin1.to_string(),
                country: country.to_string(),
            }),
        }
    }

    #[cfg(feature = "geocode")]
    #[test]
    fn consolidate_geo_single_city() {
        let pts = vec![
            gp("NW", "New York City", "New York", "United States"),
            gp("NE", "New York City", "New York", "United States"),
            gp("Center", "New York City", "New York", "United States"),
        ];
        assert_eq!(
            consolidate_geo(&pts),
            "New York City, New York, United States"
        );
    }

    #[cfg(feature = "geocode")]
    #[test]
    fn consolidate_geo_two_states_one_country() {
        let pts = vec![
            gp("NW", "Newark", "New Jersey", "United States"),
            gp("NE", "Brooklyn", "New York", "United States"),
        ];
        // multiple distinct states within a single country collapse to "A & B, Country"
        assert_eq!(
            consolidate_geo(&pts),
            "New Jersey & New York, United States"
        );
    }

    #[cfg(feature = "geocode")]
    #[test]
    fn consolidate_geo_multi_country() {
        let pts = vec![
            gp("NW", "Seattle", "Washington", "United States"),
            gp("NE", "Vancouver", "British Columbia", "Canada"),
        ];
        assert_eq!(consolidate_geo(&pts), "United States & Canada");
    }

    #[cfg(feature = "geocode")]
    #[test]
    fn consolidate_geo_all_unresolved_is_empty() {
        let pts = vec![
            GeoPoint {
                tag:   "NW",
                lat:   0.0,
                lon:   0.0,
                label: None,
            },
            GeoPoint {
                tag:   "Center",
                lat:   0.0,
                lon:   0.0,
                label: None,
            },
        ];
        assert!(consolidate_geo(&pts).is_empty());
    }

    #[cfg(feature = "geocode")]
    #[test]
    fn map_extent_min_max() {
        let e = map_extent(&[1.0, 3.0, 2.0], &[-5.0, 10.0, 2.0]);
        assert_eq!(
            (e.min_lat, e.max_lat, e.min_lon, e.max_lon),
            (1.0, 3.0, -5.0, 10.0)
        );
    }

    #[cfg(feature = "geocode")]
    #[test]
    fn extent_including_grows_to_cover_points() {
        let core = MapExtent {
            min_lat: 40.0,
            max_lat: 41.0,
            min_lon: -75.0,
            max_lon: -74.0,
        };
        // a far north-west stray and a far south-east stray expand the box on all four sides
        let full = extent_including(core, &[44.0, 38.0], &[-78.0, -70.0]);
        assert_eq!(
            (full.min_lat, full.max_lat, full.min_lon, full.max_lon),
            (38.0, 44.0, -78.0, -70.0)
        );
        // no points -> unchanged
        let same = extent_including(core, &[], &[]);
        assert_eq!(
            (same.min_lat, same.max_lat, same.min_lon, same.max_lon),
            (40.0, 41.0, -75.0, -74.0)
        );
    }

    #[test]
    fn partition_geo_outliers_flags_far_from_centroid() {
        // a small spread cluster (10x10 grid in a ~0.09deg box, so the distance IQR is non-zero),
        // plus two points far from the centroid — only the two strays exceed the far-out fence.
        let mut lats: Vec<f64> = Vec::new();
        let mut lons: Vec<f64> = Vec::new();
        for i in 0..10 {
            for j in 0..10 {
                lats.push(40.70 + f64::from(i) * 0.01);
                lons.push(-74.00 + f64::from(j) * 0.01);
            }
        }
        lats.push(45.0); // far north
        lons.push(-74.0);
        lats.push(40.7); // far east
        lons.push(-69.0);
        let (clat, clon, olat, olon) = partition_geo_outliers(&lats, &lons);
        assert_eq!(olat.len(), 2, "only the two far strays flagged");
        assert_eq!(olon.len(), 2);
        assert_eq!(
            clat.len() + olat.len(),
            lats.len(),
            "every point accounted for"
        );
        assert_eq!(clon.len(), clat.len());
        assert!(olat.contains(&45.0));
        assert!(olon.contains(&-69.0));
    }

    #[test]
    fn partition_geo_outliers_too_few_all_core() {
        // too few points to characterize a cluster -> everything is core, zero outliers
        let lats = vec![1.0, 2.0, 3.0];
        let lons = vec![1.0, 2.0, 3.0];
        let (clat, _clon, olat, olon) = partition_geo_outliers(&lats, &lons);
        assert_eq!(clat.len(), 3);
        assert!(olat.is_empty());
        assert!(olon.is_empty());
    }

    #[test]
    fn partition_geo_outliers_duplicates_plus_stray() {
        // a very common shape: a few duplicate coordinates (zero distance IQR) plus one bad
        // geocode. the point-mass (d > q3) fallback must flag the lone far stray even with
        // < 10 duplicates, where a mean/std fence (inflated by the stray itself) would not.
        let mut lats: Vec<f64> = vec![40.70; 5];
        let mut lons: Vec<f64> = vec![-74.00; 5];
        lats.push(60.0); // a distant stray
        lons.push(-74.0);
        let (clat, _clon, olat, olon) = partition_geo_outliers(&lats, &lons);
        assert_eq!(
            olat.len(),
            1,
            "the lone far stray is flagged despite zero IQR"
        );
        assert!(olat.contains(&60.0));
        assert_eq!(clat.len(), 5, "the duplicate core points stay core");
        assert_eq!(olon, vec![-74.0]);
    }

    #[test]
    fn partition_geo_outliers_constant_flags_nothing() {
        // a fully constant (zero-spread) distribution has zero distance IQR -> nothing flagged
        let lats = vec![10.0; 50];
        let lons = vec![20.0; 50];
        let (_clat, _clon, olat, olon) = partition_geo_outliers(&lats, &lons);
        assert!(olat.is_empty());
        assert!(olon.is_empty());
    }

    #[cfg(feature = "geocode")]
    #[test]
    fn outlier_summary_count_and_jurisdiction() {
        let core = "New York & New Jersey, United States";
        // zero outliers -> core summary unchanged
        assert_eq!(outlier_summary(core, 0, "Pennsylvania"), core);
        // singular / plural
        assert_eq!(
            outlier_summary(core, 1, "Pennsylvania"),
            "New York & New Jersey, United States \u{2014} 1 outlier (Pennsylvania)"
        );
        assert_eq!(
            outlier_summary(core, 3, "Pennsylvania"),
            "New York & New Jersey, United States \u{2014} 3 outliers (Pennsylvania)"
        );
        // empty jurisdiction -> no parenthetical
        assert_eq!(
            outlier_summary(core, 2, ""),
            "New York & New Jersey, United States \u{2014} 2 outliers"
        );
        // large counts are humanized with thousands separators
        assert_eq!(
            outlier_summary(core, 1_234, "Pennsylvania"),
            "New York & New Jersey, United States \u{2014} 1,234 outliers (Pennsylvania)"
        );
        assert_eq!(
            outlier_summary(core, 12_345, ""),
            "New York & New Jersey, United States \u{2014} 12,345 outliers"
        );
    }

    #[cfg(feature = "geocode")]
    #[test]
    fn outlier_jurisdictions_lists_admin1_then_countries() {
        let label = |admin1: &str, country: &str| {
            Some(crate::cmd::geocode::GeoLabel {
                city:    String::new(),
                admin1:  admin1.to_string(),
                country: country.to_string(),
            })
        };
        // single admin1
        assert_eq!(
            outlier_jurisdictions(&[label("Pennsylvania", "United States")]),
            "Pennsylvania"
        );
        // two distinct admin1s
        assert_eq!(
            outlier_jurisdictions(&[
                label("Pennsylvania", "United States"),
                label("Ohio", "United States"),
            ]),
            "Pennsylvania & Ohio"
        );
        // no admin1 -> falls back to countries
        assert_eq!(
            outlier_jurisdictions(&[label("", "Canada"), label("", "Mexico")]),
            "Canada & Mexico"
        );
        // nothing resolved -> empty
        assert_eq!(outlier_jurisdictions(&[None, None]), "");
    }

    #[cfg(feature = "geocode")]
    #[test]
    fn outliers_share_core_region_suppression() {
        let label = |admin1: &str, country: &str| {
            Some(crate::cmd::geocode::GeoLabel {
                city:    String::new(),
                admin1:  admin1.to_string(),
                country: country.to_string(),
            })
        };
        let core = vec![
            gp("NW", "Newark", "New Jersey", "United States"),
            gp("NE", "Brooklyn", "New York", "United States"),
        ];
        // outliers in a core admin1 -> share region (suppress)
        assert!(outliers_share_core_region(
            &core,
            &[label("New York", "United States")]
        ));
        // outliers in a NEW admin1 (Pennsylvania) -> real strays (keep call-out)
        assert!(!outliers_share_core_region(
            &core,
            &[label("Pennsylvania", "United States")]
        ));
        // outliers in a new country -> real strays
        assert!(!outliers_share_core_region(
            &core,
            &[label("Ontario", "Canada")]
        ));
        // unresolved outliers -> can't claim elsewhere -> suppress
        assert!(outliers_share_core_region(&core, &[None]));
    }

    #[cfg(feature = "geocode")]
    #[test]
    fn extent_box_is_closed_loop_nw_ne_se_sw() {
        let e = MapExtent {
            min_lat: 1.0,
            max_lat: 3.0,
            min_lon: -5.0,
            max_lon: 10.0,
        };
        let (lats, lons) = extent_box_latlon(&e);
        assert_eq!(lats, vec![3.0, 3.0, 1.0, 1.0, 3.0]);
        assert_eq!(lons, vec![-5.0, 10.0, 10.0, -5.0, -5.0]);
    }

    #[cfg(feature = "geocode")]
    #[test]
    fn extent_zoom_menu_core_and_full_buttons() {
        let core = MapExtent {
            min_lat: 40.70,
            max_lat: 40.80,
            min_lon: -74.05,
            max_lon: -73.95,
        };
        // full extent reaches well beyond the core (a far stray)
        let full = MapExtent {
            min_lat: 40.20,
            max_lat: 41.20,
            min_lon: -76.90,
            max_lon: -73.90,
        };
        let v = serde_json::to_value(extent_zoom_menu(&core, &full)).unwrap();
        let btns = v["buttons"].as_array().expect("buttons array");
        assert_eq!(btns.len(), 2);
        assert_eq!(btns[0]["label"], "Core extent");
        assert_eq!(btns[1]["label"], "Full extent");
        // each button is a relayout of the mapbox center + zoom
        let core_args = &btns[0]["args"][0];
        assert!(core_args["mapbox.center"]["lat"].is_number());
        assert!(core_args["mapbox.center"]["lon"].is_number());
        let cz = core_args["mapbox.zoom"].as_f64().expect("core zoom");
        let fz = btns[1]["args"][0]["mapbox.zoom"]
            .as_f64()
            .expect("full zoom");
        // the full extent is larger, so its fit zoom is further out (<=) than the core's
        assert!(
            fz <= cz,
            "full-extent zoom {fz} should be <= core-extent zoom {cz}"
        );
    }

    #[cfg(feature = "geocode")]
    #[test]
    fn dashed_box_has_gaps_within_bounds() {
        let e = MapExtent {
            min_lat: 40.0,
            max_lat: 42.0,
            min_lon: -76.0,
            max_lon: -74.0,
        };
        let (lats, lons) = dashed_box_latlon(&e);
        assert_eq!(lats.len(), lons.len());
        // NaN breaks produce the dash gaps
        assert!(lats.iter().any(|v| v.is_nan()), "expected gap (NaN) breaks");
        // every finite dash vertex lies on the box bounds (a small epsilon for float math)
        for (&la, &lo) in lats.iter().zip(&lons) {
            if la.is_nan() {
                assert!(lo.is_nan(), "lat/lon gaps must be paired");
                continue;
            }
            assert!((40.0 - 1e-9..=42.0 + 1e-9).contains(&la));
            assert!((-76.0 - 1e-9..=-74.0 + 1e-9).contains(&lo));
        }
    }

    #[cfg(feature = "geocode")]
    #[test]
    fn antimeridian_guard() {
        let crossing = MapExtent {
            min_lat: -10.0,
            max_lat: 10.0,
            min_lon: -179.0,
            max_lon: 179.0,
        };
        let local = MapExtent {
            min_lat: 40.0,
            max_lat: 41.0,
            min_lon: -74.5,
            max_lon: -73.5,
        };
        assert!(extent_spans_antimeridian(&crossing));
        assert!(!extent_spans_antimeridian(&local));
    }

    #[test]
    fn classify_null_skipped() {
        assert!(classify(0, &stat("NULL", 0, None)).is_none());
    }

    #[test]
    fn classify_boolean_is_bar() {
        assert!(matches!(
            classify(0, &stat("Boolean", 2, Some(0.001))),
            Some(PanelKind::FreqBar { idx: 0 })
        ));
    }

    #[test]
    fn classify_near_unique_integer_id_skipped() {
        // a sequential id column (Integer, ~100% unique) is not meaningful to chart
        assert!(classify(0, &stat("Integer", 1000, Some(1.0))).is_none());
    }

    #[test]
    fn classify_low_card_numeric_is_bar_not_box() {
        // a 1-5 rating has quartiles but should be a frequency bar, not a box plot
        let mut s = stat("Integer", 5, Some(0.05));
        s.q1 = Some(2.0);
        s.q2_median = Some(3.0);
        s.q3 = Some(4.0);
        assert!(matches!(
            classify(2, &s),
            Some(PanelKind::FreqBar { idx: 2 })
        ));
    }

    #[test]
    fn classify_continuous_float_is_box() {
        // a near-unique continuous float (e.g. measurements) is a box plot, not skipped
        let mut s = stat("Float", 500, Some(0.99));
        s.q1 = Some(1.0);
        s.q2_median = Some(2.0);
        s.q3 = Some(3.0);
        s.min = Some("0.5".to_string());
        s.max = Some("3.5".to_string());
        assert!(matches!(classify(1, &s), Some(PanelKind::BoxStats { .. })));
    }

    fn dict_row(content_type: &str, role: &str, concept: &str, label: &str) -> DictRow {
        DictRow {
            content_type: content_type.to_string(),
            role:         role.to_string(),
            concept:      concept.to_string(),
            label:        label.to_string(),
        }
    }

    #[test]
    fn route_from_concept_maps_namespaces() {
        // coordinates -> map; other geo keys (census_tract, zip, ...) -> dimension bar
        assert_eq!(
            route_from_concept("geo.latitude"),
            Some((Route::MapCoord, None))
        );
        assert_eq!(
            route_from_concept("geo.coordinate_pair"),
            Some((Route::MapCoord, None))
        );
        assert_eq!(
            route_from_concept("geo.census_tract"),
            Some((Route::Dimension, None))
        );
        assert_eq!(
            route_from_concept("nyc.bbl"),
            Some((Route::Dimension, None))
        );
        // temporal
        assert_eq!(
            route_from_concept("time.created_at"),
            Some((Route::Temporal, None))
        );
        // identifiers / PII -> skip
        assert_eq!(
            route_from_concept("id.surrogate_key"),
            Some((Route::Skip, None))
        );
        assert_eq!(route_from_concept("pii.email"), Some((Route::Skip, None)));
        // measures: amount/count additive (Sum), ratio averaged (Mean)
        assert_eq!(
            route_from_concept("measure.amount"),
            Some((Route::Measure, Some(Agg::Sum)))
        );
        assert_eq!(
            route_from_concept("measure.count"),
            Some((Route::Measure, Some(Agg::Sum)))
        );
        assert_eq!(
            route_from_concept("measure.ratio"),
            Some((Route::Measure, Some(Agg::Mean)))
        );
        // unknown namespace / bare unknown -> None (fall through to role)
        assert_eq!(route_from_concept("weather.temp"), None);
        assert_eq!(route_from_concept("unknown"), None);
    }

    #[test]
    fn route_from_content_type_legacy_mapping() {
        assert_eq!(route_from_content_type("category").0, Route::Dimension);
        assert_eq!(route_from_content_type("state_abbr").0, Route::Dimension);
        assert_eq!(route_from_content_type("latitude").0, Route::MapCoord);
        // temporal tokens, with/without an strftime or length suffix
        assert_eq!(route_from_content_type("date").0, Route::Temporal);
        assert_eq!(
            route_from_content_type("datetime:%Y-%m-%dT%H:%M:%S").0,
            Route::Temporal
        );
        assert_eq!(route_from_content_type("duration:3600").0, Route::Temporal);
        // identifiers / PII / free-text -> skip
        assert_eq!(route_from_content_type("unique_id").0, Route::Skip);
        assert_eq!(route_from_content_type("email").0, Route::Skip);
        assert_eq!(route_from_content_type("free_text").0, Route::Skip);
        // explicit / empty -> defer to stats
        assert_eq!(route_from_content_type("unknown").0, Route::Defer);
        assert_eq!(route_from_content_type("").0, Route::Defer);
    }

    #[test]
    fn derive_semantics_precedence_and_label() {
        // no dictionary row -> Defer (stats-only behavior unchanged)
        assert_eq!(
            derive_semantics(&stat("Integer", 5, None), None).route,
            Route::Defer
        );

        // concept beats a defaulted `measure` role: a numeric census_tract that describegpt left
        // role=measure is still routed to a Dimension bar by its geo.census_tract concept.
        let tract = stat("Integer", 33, Some(0.00003));
        let row = dict_row("", "measure", "geo.census_tract", "Census Tract");
        let sem = derive_semantics(&tract, Some(&row));
        assert_eq!(sem.route, Route::Dimension);
        assert_eq!(sem.label, "Census Tract"); // label carried for the panel title

        // role used when concept is absent/unknown
        let r = derive_semantics(
            &stat("String", 8, Some(0.01)),
            Some(&dict_row("", "dimension", "", "")),
        );
        assert_eq!(r.route, Route::Dimension);

        // content_type used when neither concept nor role resolve
        let c = derive_semantics(
            &stat("Float", 99, Some(0.5)),
            Some(&dict_row("latitude", "", "", "")),
        );
        assert_eq!(c.route, Route::MapCoord);

        // all-empty dictionary row -> Defer, but the label still rides along
        let d = derive_semantics(
            &stat("String", 9, Some(0.02)),
            Some(&dict_row("", "", "", "Notes")),
        );
        assert_eq!(d.route, Route::Defer);
        assert_eq!(d.label, "Notes");
    }

    #[test]
    fn guardrail_downgrades_only_code_like_measures() {
        // a low-cardinality integer over many rows, role-defaulted to measure with no concept ->
        // downgraded to a Dimension bar (the fact-#2 defense).
        let code = stat("Integer", 6, Some(0.00001));
        assert_eq!(
            derive_semantics(&code, Some(&dict_row("", "measure", "", ""))).route,
            Route::Dimension
        );
        // an EXPLICIT measure.* concept is trusted even when low-card -> stays Measure
        assert_eq!(
            derive_semantics(&code, Some(&dict_row("", "measure", "measure.count", ""))).route,
            Route::Measure
        );
        // a float is ~never a code -> stays Measure
        assert_eq!(
            derive_semantics(
                &stat("Float", 6, Some(0.00001)),
                Some(&dict_row("", "measure", "", ""))
            )
            .route,
            Route::Measure
        );
        // a near-unique integer (genuine continuous) -> stays Measure
        assert_eq!(
            derive_semantics(
                &stat("Integer", 100_000, Some(0.99)),
                Some(&dict_row("", "measure", "", ""))
            )
            .route,
            Route::Measure
        );
        // residual gap (documented): cardinality above CATEGORICAL_MAX_CARDINALITY is left to the
        // concept layer, NOT the guardrail -> stays Measure.
        assert_eq!(
            derive_semantics(
                &stat("Integer", 50, Some(0.00001)),
                Some(&dict_row("", "measure", "", ""))
            )
            .route,
            Route::Measure
        );
    }

    #[test]
    fn classify_with_semantics_routes() {
        // Defer falls back to classify (low-card string -> frequency bar)
        let sem_defer = ColSemantics::default();
        assert!(matches!(
            classify_with_semantics(0, &stat("String", 5, Some(0.001)), &sem_defer),
            Some(PanelKind::FreqBar { idx: 0 })
        ));
        // Dimension -> frequency bar even for a many-distinct integer code
        let sem_dim = ColSemantics {
            route: Route::Dimension,
            ..Default::default()
        };
        assert!(matches!(
            classify_with_semantics(2, &stat("Integer", 500, Some(0.4)), &sem_dim),
            Some(PanelKind::FreqBar { idx: 2 })
        ));
        // Measure -> box plot from quartiles (via classify_measure)
        let mut m = stat("Integer", 500, Some(0.4));
        m.q1 = Some(10.0);
        m.q2_median = Some(20.0);
        m.q3 = Some(30.0);
        let sem_meas = ColSemantics {
            route: Route::Measure,
            ..Default::default()
        };
        assert!(matches!(
            classify_with_semantics(3, &m, &sem_meas),
            Some(PanelKind::BoxStats { .. })
        ));
        // Temporal / Skip draw no per-column panel
        for route in [Route::Temporal, Route::Skip] {
            let sem = ColSemantics {
                route,
                ..Default::default()
            };
            assert!(classify_with_semantics(0, &stat("DateTime", 9, Some(0.6)), &sem).is_none());
        }
        // MapCoord falls back to classify: a coordinate the map did NOT consume (the caller already
        // skips consumed ones via is_map_col) must still be charted, not vanish. A near-unique
        // float with quartiles -> box.
        let mut coord = stat("Float", 5000, Some(0.99));
        coord.q1 = Some(1.0);
        coord.q2_median = Some(2.0);
        coord.q3 = Some(3.0);
        let sem_coord = ColSemantics {
            route: Route::MapCoord,
            ..Default::default()
        };
        assert!(matches!(
            classify_with_semantics(7, &coord, &sem_coord),
            Some(PanelKind::BoxStats { .. })
        ));
    }

    #[test]
    fn classify_measure_box_histogram_none() {
        // quartiles present -> box plot
        let mut s = stat("Float", 500, Some(0.9));
        s.q1 = Some(1.0);
        s.q2_median = Some(2.0);
        s.q3 = Some(3.0);
        assert!(matches!(
            classify_measure(1, &s),
            Some(PanelKind::BoxStats { .. })
        ));
        // flagged bimodal AND platykurtic (negative excess kurtosis) -> histogram
        s.bimodality_coefficient = Some(BIMODALITY_COEFFICIENT_THRESHOLD + 0.01);
        s.kurtosis = Some(-1.0);
        assert!(matches!(
            classify_measure(1, &s),
            Some(PanelKind::Histogram { .. })
        ));
        // high BC but LEPTOKURTIC (skewed unimodal, e.g. a long-tailed/outlier column) stays a box
        s.kurtosis = Some(5.0);
        assert!(matches!(
            classify_measure(1, &s),
            Some(PanelKind::BoxStats { .. })
        ));
        // no quartiles -> nothing to chart
        assert!(classify_measure(1, &stat("Integer", 500, Some(0.9))).is_none());
    }

    #[test]
    fn parse_dictionary_semantics_jsonschema_shape() {
        // a describegpt --format jsonschema doc: label rides as `title`, semantic tokens in
        // x-qsv, dataset grain at the top-level x-qsv.
        let schema = r#"{
          "$schema": "https://json-schema.org/draft/2020-12/schema",
          "type": "object",
          "properties": {
            "census_tract": { "type": ["integer","null"], "title": "Census Tract",
              "x-qsv": { "qsv_type": "Integer", "content_type": "category",
                         "role": "dimension", "concept": "geo.census_tract" } },
            "amount": { "type": "number", "title": "Amount",
              "x-qsv": { "qsv_type": "Float", "role": "measure", "concept": "measure.amount" } },
            "notes": { "type": "string", "x-qsv": { "qsv_type": "String" } }
          },
          "x-qsv": { "grain": "one row = one 311 service request" }
        }"#;
        let data = parse_dictionary_semantics(schema).expect("schema should parse");
        let tract = data.rows.get("census_tract").expect("census_tract row");
        assert_eq!(tract.role, "dimension");
        assert_eq!(tract.concept, "geo.census_tract");
        assert_eq!(tract.content_type, "category");
        assert_eq!(tract.label, "Census Tract");
        let amount = data.rows.get("amount").expect("amount row");
        assert_eq!(amount.concept, "measure.amount");
        // a property with a bare x-qsv (no role/concept) still parses with empty signals
        let notes = data.rows.get("notes").expect("notes row");
        assert!(notes.role.is_empty() && notes.concept.is_empty());
        assert_eq!(
            data.grain.as_deref(),
            Some("one row = one 311 service request")
        );

        // end-to-end: the parsed census_tract row routes to a Dimension bar.
        assert_eq!(
            derive_semantics(&stat("Integer", 33, Some(0.00003)), Some(tract)).route,
            Route::Dimension
        );
    }

    #[test]
    fn parse_dictionary_semantics_legacy_and_garbage() {
        // legacy --format json dictionary (content_type only, no role/concept/grain)
        let json = r#"{
          "Dictionary": { "response": { "fields": [
            { "name": "status", "type": "String", "content_type": "category", "label": "Status" },
            { "name": "id", "type": "String" }
          ] } }
        }"#;
        let data = parse_dictionary_semantics(json).expect("legacy json should parse");
        let status = data.rows.get("status").expect("status row");
        assert_eq!(status.content_type, "category");
        assert_eq!(status.label, "Status");
        assert!(status.role.is_empty() && status.concept.is_empty());
        // a field with no content_type is still present (carries empty signals)
        assert!(data.rows.contains_key("id"));
        assert!(data.grain.is_none());

        // a bare {"fields":[...]} envelope also works
        let bare = r#"{"fields":[{"name":"x","content_type":"category"}]}"#;
        assert!(parse_dictionary_semantics(bare).is_some());

        // garbage / wrong-shaped JSON yields None (caller degrades gracefully)
        assert!(parse_dictionary_semantics("not json").is_none());
        assert!(parse_dictionary_semantics(r#"{"foo":1}"#).is_none());
    }

    #[test]
    fn timestamp_rank_prefers_event_and_created() {
        // event/created lead; closed/updated/due secondary; bare date or no concept last
        assert!(timestamp_rank("time.event_timestamp") < timestamp_rank("time.created_at"));
        assert!(timestamp_rank("time.created_at") < timestamp_rank("time.closed_at"));
        assert!(timestamp_rank("time.closed_at") < timestamp_rank("time.date"));
        assert_eq!(
            timestamp_rank("time.updated_at"),
            timestamp_rank("time.due_at")
        );
        assert_eq!(timestamp_rank(""), timestamp_rank("time.date")); // no concept == bare date
    }

    #[test]
    fn ts_bucket_span_thresholds_and_keys() {
        use chrono::NaiveDate;
        assert_eq!(ts_bucket_for_span(0), TsBucket::Day);
        assert_eq!(ts_bucket_for_span(370), TsBucket::Day);
        assert_eq!(ts_bucket_for_span(371), TsBucket::Week);
        assert_eq!(ts_bucket_for_span(1825), TsBucket::Week);
        assert_eq!(ts_bucket_for_span(1826), TsBucket::Month);

        // a Wednesday truncates to its ISO-week Monday, and to the 1st of its month
        let wed = NaiveDate::from_ymd_opt(2024, 5, 1).unwrap(); // 2024-05-01 is a Wednesday
        assert_eq!(ts_bucket_key(wed, TsBucket::Day), wed);
        assert_eq!(
            ts_bucket_key(wed, TsBucket::Week),
            NaiveDate::from_ymd_opt(2024, 4, 29).unwrap() // Monday
        );
        assert_eq!(
            ts_bucket_key(wed, TsBucket::Month),
            NaiveDate::from_ymd_opt(2024, 5, 1).unwrap()
        );
        // labels: ISO date for day/week, YYYY-MM for month
        assert_eq!(ts_bucket_label(wed, TsBucket::Day), "2024-05-01");
        assert_eq!(ts_bucket_label(wed, TsBucket::Month), "2024-05");
    }

    #[test]
    fn count_unit_from_grain_extracts_or_falls_back() {
        assert_eq!(
            count_unit_from_grain(Some("one row = one 311 service request")),
            "311 service request"
        );
        assert_eq!(
            count_unit_from_grain(Some("each row is one order.")),
            "order"
        );
        // no " one " pattern -> fallback
        assert_eq!(
            count_unit_from_grain(Some("each row represents a person")),
            "records"
        );
        // absent grain -> fallback
        assert_eq!(count_unit_from_grain(None), "records");
        // implausibly long tail -> fallback (guards against a runaway grain sentence)
        let long = format!("one row = one {}", "x".repeat(60));
        assert_eq!(count_unit_from_grain(Some(&long)), "records");
    }

    #[test]
    fn code_twin_base_strips_key_suffixes() {
        assert_eq!(code_twin_base("subject_code"), Some("subject"));
        assert_eq!(code_twin_base("street_id"), Some("street"));
        assert_eq!(code_twin_base("region_key"), Some("region"));
        // no key suffix, or suffix-only -> None
        assert_eq!(code_twin_base("subject"), None);
        assert_eq!(code_twin_base("_code"), None);
        // bare "id"/"code" (no underscore) is NOT a twin suffix
        assert_eq!(code_twin_base("zipcode"), None);
    }

    #[test]
    fn dimension_code_twins_suppresses_only_paired_keys() {
        // build a 4-column frame: subject (dim) + subject_code (dim) + street (dim) + lonely_id
        // (dim)
        let stats = [
            {
                let mut s = stat("String", 20, Some(0.01));
                s.field = "subject".to_string();
                s
            },
            {
                let mut s = stat("Integer", 20, Some(0.01));
                s.field = "subject_code".to_string();
                s
            },
            {
                let mut s = stat("String", 20, Some(0.01));
                s.field = "street".to_string();
                s
            },
            {
                let mut s = stat("Integer", 20, Some(0.01));
                s.field = "lonely_id".to_string();
                s
            },
        ];
        let dim = ColSemantics {
            route: Route::Dimension,
            ..Default::default()
        };
        let sems = [dim.clone(), dim.clone(), dim.clone(), dim.clone()];
        let suppress = dimension_code_twins(&stats, &sems);
        // subject_code is suppressed (its label sibling `subject` is charted) ...
        assert!(suppress.contains(&1));
        // ... but subject, street, and lonely_id (no `lonely` sibling) are kept
        assert!(!suppress.contains(&0));
        assert!(!suppress.contains(&2));
        assert!(!suppress.contains(&3));

        // a *_code with no charted label sibling is NOT suppressed
        let orphan_stats = [{
            let mut s = stat("Integer", 20, Some(0.01));
            s.field = "ward_code".to_string();
            s
        }];
        let orphan_sems = [dim];
        assert!(dimension_code_twins(&orphan_stats, &orphan_sems).is_empty());
    }

    #[test]
    fn log_scale_parse() {
        assert!(matches!(parse_log_scale("auto"), Ok(LogScale::Auto)));
        assert!(matches!(parse_log_scale("AUTO"), Ok(LogScale::Auto)));
        assert!(matches!(parse_log_scale("on"), Ok(LogScale::On)));
        assert!(matches!(parse_log_scale("true"), Ok(LogScale::On)));
        assert!(matches!(parse_log_scale("off"), Ok(LogScale::Off)));
        assert!(matches!(parse_log_scale("false"), Ok(LogScale::Off)));
        assert!(parse_log_scale("bogus").is_err());
    }

    #[test]
    fn freq_panel_logs_auto_triggers_on_high_dynamic_range() {
        // a dominating bucket (10000) dwarfs the small categories (>= 50x) -> log
        let dominated = [10_000, 120, 90, 30];
        assert!(freq_panel_logs(LogScale::Auto, &dominated));
        // a balanced distribution (max/min < 50x) stays linear under auto
        let balanced = [100, 90, 80, 70];
        assert!(!freq_panel_logs(LogScale::Auto, &balanced));
        // auto needs at least 3 positive bars (a lone tall bar isn't "dominating" anything)
        let two_bars = [10_000, 1];
        assert!(!freq_panel_logs(LogScale::Auto, &two_bars));
    }

    #[test]
    fn freq_panel_logs_on_and_off_modes() {
        let balanced = [100, 90, 80];
        // `on` forces log even when the range is low (needs 2+ positive bars)
        assert!(freq_panel_logs(LogScale::On, &balanced));
        assert!(!freq_panel_logs(LogScale::On, &[5]));
        // `off` never logs, even on a wildly skewed distribution
        assert!(!freq_panel_logs(LogScale::Off, &[10_000, 100, 10, 1]));
    }

    #[test]
    fn classify_low_card_string_is_bar() {
        assert!(matches!(
            classify(0, &stat("String", 3, Some(0.1))),
            Some(PanelKind::FreqBar { idx: 0 })
        ));
    }

    #[test]
    fn classify_high_card_string_skipped() {
        assert!(classify(0, &stat("String", 9999, Some(0.99))).is_none());
    }

    #[test]
    fn box_whiskers_use_observed_min_max_not_fences() {
        // Whiskers must be the observed min/max, never the Tukey fences - even when a fence
        // falls INSIDE the data range (a fence value need not exist in the dataset).
        let mut s = stat("Float", 100, Some(0.8));
        s.q1 = Some(10.0);
        s.q2_median = Some(15.0);
        s.q3 = Some(20.0);
        s.lower_inner_fence = Some(12.0); // inside [8, 25] - must be ignored
        s.upper_inner_fence = Some(22.0); // inside [8, 25] - must be ignored
        s.min = Some("8.0".to_string());
        s.max = Some("25.0".to_string());
        match classify(0, &s) {
            Some(PanelKind::BoxStats { lower, upper, .. }) => {
                assert_eq!(lower, Some(8.0)); // observed minimum
                assert_eq!(upper, Some(25.0)); // observed maximum
            },
            _ => panic!("expected BoxStats"),
        }
    }

    #[test]
    fn classify_bimodal_continuous_is_histogram_not_box() {
        // a continuous column flagged bimodal (BC >= 0.555) AND platykurtic (excess kurtosis < 0)
        // should become a histogram (which shows the two peaks), not a box plot (which hides them).
        let mut s = stat("Float", 500, Some(0.9));
        s.q1 = Some(1.0);
        s.q2_median = Some(2.0);
        s.q3 = Some(3.0);
        s.min = Some("0.0".to_string());
        s.max = Some("4.0".to_string());
        s.bimodality_coefficient = Some(0.7); // >= 0.555 threshold
        s.kurtosis = Some(-1.5); // platykurtic (two-peaked / flat-topped)
        assert!(matches!(
            classify(4, &s),
            Some(PanelKind::Histogram { idx: 4 })
        ));
    }

    #[test]
    fn classify_skewed_high_bc_leptokurtic_stays_box() {
        // a heavily-skewed UNIMODAL column (long tail / outliers) can have a high BC purely from
        // skewness, but it's leptokurtic — the platykurtic guard keeps it a box (with outlier
        // points) rather than a misleading one-tall-bar histogram.
        let mut s = stat("Float", 500, Some(0.9));
        s.q1 = Some(1.0);
        s.q2_median = Some(2.0);
        s.q3 = Some(3.0);
        s.min = Some("0.0".to_string());
        s.max = Some("9999.0".to_string());
        s.bimodality_coefficient = Some(0.98); // high, but driven by skew
        s.kurtosis = Some(10.0); // leptokurtic (heavy tail)
        assert!(matches!(classify(0, &s), Some(PanelKind::BoxStats { .. })));
    }

    #[test]
    fn classify_unimodal_continuous_stays_box() {
        // bimodality below the threshold (or absent) keeps the cache-only box plot.
        let mut s = stat("Float", 500, Some(0.9));
        s.q1 = Some(1.0);
        s.q2_median = Some(2.0);
        s.q3 = Some(3.0);
        s.min = Some("0.0".to_string());
        s.max = Some("4.0".to_string());
        s.bimodality_coefficient = Some(0.40); // unimodal
        assert!(matches!(classify(0, &s), Some(PanelKind::BoxStats { .. })));
    }

    #[test]
    fn classify_high_card_concentrated_string_is_bar() {
        // high-cardinality text is normally skipped, but a low normalized_entropy (concentrated
        // distribution, a few dominant categories) makes a top-N bar worthwhile.
        let mut s = stat("String", 9999, Some(0.6));
        s.normalized_entropy = Some(0.4); // concentrated, below the 0.95 noise cutoff
        assert!(matches!(
            classify(7, &s),
            Some(PanelKind::FreqBar { idx: 7 })
        ));
    }

    #[test]
    fn classify_high_card_uniform_string_still_skipped() {
        // high cardinality AND near-uniform entropy => genuinely noise, still skipped.
        let mut s = stat("String", 9999, Some(0.6));
        s.normalized_entropy = Some(0.99); // near-uniform
        assert!(classify(0, &s).is_none());
    }

    #[test]
    fn finalize_freq_bars_appends_null_and_other() {
        // 12 distinct categories (a..l), descending counts, plus 7 empty cells. With top_n=10,
        // the top 10 are kept and the remaining 2 distinct roll up into "Other (2)"; the null
        // bucket becomes "(NULL)". Both are appended after the real categories, tagged Aggregate.
        let counts: Vec<(String, u64)> = (0..12)
            .map(|i| (((b'a' + i) as char).to_string(), (100 - i as u64)))
            .collect();
        let out = finalize_freq_bars(counts, 7, 10, false, false);
        assert_eq!(out.len(), 12); // 10 real + (NULL) + Other
        assert_eq!(out[9].label, "j"); // last real category kept
        assert_eq!(out[9].kind, FreqBarKind::Category);

        assert_eq!(out[10].label, "(NULL)");
        assert_eq!(out[10].count, 7);
        assert_eq!(out[10].kind, FreqBarKind::Aggregate);

        // the two dropped categories ("k"=90, "l"=89) aggregate to 179, labeled by distinct count
        assert_eq!(out[11].label, "Other (2)");
        assert_eq!(out[11].count, 179);
        assert_eq!(out[11].kind, FreqBarKind::Aggregate);

        // aggregate bars get a sentinel-suffixed x_key so they can't collide with a real
        // category sharing their display label; real categories key on their value
        assert_eq!(out[9].x_key, "j");
        assert_eq!(out[10].x_key, format!("(NULL){AGG_KEY_SENTINEL}"));
        assert_ne!(out[10].x_key, out[10].label);
    }

    #[test]
    fn freq_bar_pattern_shapes_hatches_aggregates_only_on_log() {
        // 10 real categories + (NULL) + Other, the last two tagged Aggregate.
        let counts: Vec<(String, u64)> = (0..12)
            .map(|i| (((b'a' + i) as char).to_string(), (100 - i as u64)))
            .collect();
        let bars = finalize_freq_bars(counts, 7, 10, false, false);

        // linear panel -> no hatching at all
        assert!(freq_bar_pattern_shapes(&bars, false).is_none());

        // log panel -> diagonal hatch on aggregates, none on real categories
        let shapes = freq_bar_pattern_shapes(&bars, true).unwrap();
        assert_eq!(shapes.len(), bars.len());
        // PatternShape has no PartialEq; compare via serde (empty "" = no pattern, "/" = hatch)
        for (bar, shape) in bars.iter().zip(shapes.iter()) {
            let want = match bar.kind {
                FreqBarKind::Aggregate => "/",
                FreqBarKind::Category => "",
            };
            assert_eq!(
                serde_json::to_value(shape).unwrap(),
                serde_json::json!(want)
            );
        }
    }

    #[test]
    fn finalize_freq_bars_respects_opt_out_flags() {
        let counts: Vec<(String, u64)> = (0..12)
            .map(|i| (((b'a' + i) as char).to_string(), (100 - i as u64)))
            .collect();
        // suppress both buckets -> just the top 10 real categories remain
        let out = finalize_freq_bars(counts, 7, 10, true, true);
        assert_eq!(out.len(), 10);
        assert!(out.iter().all(|b| b.kind == FreqBarKind::Category));
    }

    #[test]
    fn finalize_freq_bars_omits_empty_buckets() {
        // no nulls and fewer distinct than top_n -> no aggregate bars even with flags off
        let counts = vec![("a".to_string(), 5_u64), ("b".to_string(), 3)];
        let out = finalize_freq_bars(counts, 0, 10, false, false);
        let labels: Vec<(String, u64)> = out.iter().map(|b| (b.label.clone(), b.count)).collect();
        assert_eq!(labels, vec![("a".to_string(), 5), ("b".to_string(), 3)]);
        assert!(out.iter().all(|b| b.kind == FreqBarKind::Category));
    }

    #[test]
    fn finalize_freq_bars_aggregate_key_never_collides() {
        // pathological data: a real category whose value already equals the sentinel-suffixed
        // form of the NULL bucket's key. The aggregate key must extend its sentinel until it is
        // distinct, so the synthetic (NULL) bar can never collapse onto that real category.
        let collider = format!("(NULL){AGG_KEY_SENTINEL}");
        let counts = vec![("apple".to_string(), 10_u64), (collider.clone(), 4)];
        let out = finalize_freq_bars(counts, 3, 10, false, false);

        // real category keeps its value as the key; the NULL aggregate is forced to a longer key
        let real = out.iter().find(|b| b.label == collider).unwrap();
        assert_eq!(real.x_key, collider);
        assert_eq!(real.kind, FreqBarKind::Category);

        let null_bar = out.iter().find(|b| b.label == "(NULL)").unwrap();
        assert_eq!(null_bar.kind, FreqBarKind::Aggregate);
        assert_ne!(null_bar.x_key, collider);
        // all x_keys across the panel are unique
        let keys: std::collections::HashSet<&String> = out.iter().map(|b| &b.x_key).collect();
        assert_eq!(keys.len(), out.len());
    }

    #[test]
    fn box_shape_hint_reports_skew_and_outliers() {
        let mut s = stat("Float", 100, Some(0.8));
        s.pearson_skewness = Some(1.2); // right skew
        s.outliers_percentage = Some(4.2);
        assert_eq!(
            box_shape_hint(&s).as_deref(),
            Some("(right-skewed, 4.2% outliers)")
        );

        // symmetric, no notable outliers => no hint
        let mut s2 = stat("Float", 100, Some(0.8));
        s2.pearson_skewness = Some(0.1);
        s2.outliers_percentage = Some(0.2);
        assert_eq!(box_shape_hint(&s2), None);

        // no moarstats stats at all => no hint
        assert_eq!(box_shape_hint(&stat("Float", 100, Some(0.8))), None);
    }

    #[test]
    fn strongest_pair_picks_max_abs_r_and_skips_nan() {
        // off-diagonal magnitudes: |(0,1)|=0.2, |(0,2)|=0.9, (1,2)=NaN (skipped).
        // strongest by |r| is the (0,2) pair, returned with its signed r.
        let m = vec![
            vec![1.0, 0.2, -0.9],
            vec![0.2, 1.0, f64::NAN],
            vec![-0.9, f64::NAN, 1.0],
        ];
        assert_eq!(strongest_pair(&m), Some((0, 2, -0.9)));

        // every off-diagonal cell undefined => no pair
        let all_nan = vec![vec![f64::NAN, f64::NAN], vec![f64::NAN, f64::NAN]];
        assert_eq!(strongest_pair(&all_nan), None);

        // fewer than two columns => no pair
        assert_eq!(strongest_pair(&[vec![1.0]]), None);
    }

    #[test]
    fn least_redundant_third_skips_undefined_correlations() {
        // Pair (0,1) is chosen. Candidates are cols 2 and 3.
        // col 2: max(|r02|,|r12|) = max(0.8, 0.7) = 0.8
        // col 3: max(|r03|,|r13|) = max(0.1, 0.2) = 0.2  <- least redundant
        let m = vec![
            vec![1.0, 0.9, 0.8, 0.1],
            vec![0.9, 1.0, 0.7, 0.2],
            vec![0.8, 0.7, 1.0, 0.3],
            vec![0.1, 0.2, 0.3, 1.0],
        ];
        assert_eq!(least_redundant_third(&m, 0, 1), Some(3));

        // col 3 became constant on the complete rows => NaN correlations.
        // Even though it would score "lowest" via the Equal fallback, it must be
        // excluded; the only finite candidate (col 2) is picked instead.
        let m_nan = vec![
            vec![1.0, 0.9, 0.8, f64::NAN],
            vec![0.9, 1.0, 0.7, f64::NAN],
            vec![0.8, 0.7, 1.0, f64::NAN],
            vec![f64::NAN, f64::NAN, f64::NAN, 1.0],
        ];
        assert_eq!(least_redundant_third(&m_nan, 0, 1), Some(2));

        // Every third-axis candidate is undefined => no usable third axis.
        let m_all_nan = vec![
            vec![1.0, 0.9, f64::NAN],
            vec![0.9, 1.0, f64::NAN],
            vec![f64::NAN, f64::NAN, 1.0],
        ];
        assert_eq!(least_redundant_third(&m_all_nan, 0, 1), None);
    }

    #[test]
    fn mask_to_lower_triangle_blanks_upper_and_diagonal() {
        let m = vec![
            vec![1.0, 0.5, 0.2],
            vec![0.5, 1.0, 0.9],
            vec![0.2, 0.9, 1.0],
        ];
        let masked = mask_to_lower_triangle(m);
        for r in 0..3 {
            for c in 0..3 {
                if c >= r {
                    assert!(
                        masked[r][c].is_nan(),
                        "upper/diagonal cell ({r},{c}) should be NaN"
                    );
                } else {
                    assert!(
                        masked[r][c].is_finite(),
                        "lower cell ({r},{c}) should be kept"
                    );
                }
            }
        }
        // a kept lower-triangle value is preserved unchanged
        assert!((masked[2][1] - 0.9).abs() < 1e-9);
    }

    #[test]
    fn spearman_rho_sees_monotonic_curve_pearson_misses() {
        // y = x^5 is perfectly monotonic but strongly curved: Spearman = 1.0 while Pearson ~0.82,
        // so the |rho| - |r| gap flags the nonlinearity a single Pearson number hides.
        let x: Vec<f64> = (0..20).map(f64::from).collect();
        let y: Vec<f64> = x.iter().map(|v| v.powi(5)).collect();
        let rho = spearman_rho(&x, &y);
        let r = pearson(&x, &y);
        assert!(
            (rho - 1.0).abs() < 1e-9,
            "spearman should be 1.0, got {rho}"
        );
        assert!(
            r < 0.95,
            "pearson should trail spearman for a curve, got {r}"
        );
        assert!(rho.abs() - r.abs() >= SMART_NONLINEAR_MIN_GAP);

        // tie-averaging: equal values share the mean of the ranks they span.
        let ranks = average_ranks(&[10.0, 10.0, 20.0]);
        assert!((ranks[0] - 1.5).abs() < 1e-9 && (ranks[1] - 1.5).abs() < 1e-9);
        assert!((ranks[2] - 3.0).abs() < 1e-9);
    }

    #[test]
    fn geo_framing_picks_projection_by_extent() {
        // a tight local cluster (LA area, ~1 deg span) -> Mercator fit to padded bounds
        let lats: Vec<f64> = (0..20).map(|i| 34.0 + i as f64 * 0.05).collect();
        let lons: Vec<f64> = (0..20).map(|i| -118.0 + i as f64 * 0.05).collect();
        let (proj, lonaxis, lataxis) = geo_framing(&lats, &lons);
        assert!(matches!(proj, ProjectionType::Mercator));
        assert!(
            lonaxis.is_some() && lataxis.is_some(),
            "local extent should be fit to bounds on both axes"
        );

        // coordinates spanning the continental US -> albers usa, no fitted ranges
        let us_lats = [40.7_f64, 34.0, 41.9, 29.8, 47.6, 25.8, 39.7];
        let us_lons = [-74.0_f64, -118.2, -87.6, -95.4, -122.3, -80.2, -105.0];
        let (proj, lonaxis, lataxis) = geo_framing(&us_lats, &us_lons);
        assert!(matches!(proj, ProjectionType::AlbersUsa));
        assert!(
            lonaxis.is_none() && lataxis.is_none(),
            "albers usa frames the US, so no axis ranges"
        );

        // a US extent that includes Alaska/Hawaii spans >90 deg of longitude, exceeding the global
        // threshold; the US heuristic must still win (albers usa), not fall through to
        // NaturalEarth.
        let akhi_lats = [21.3_f64, 61.2, 34.0, 41.9, 40.7, 42.4];
        let akhi_lons = [-157.8_f64, -149.9, -118.2, -87.6, -74.0, -168.0];
        let (proj, ..) = geo_framing(&akhi_lats, &akhi_lons);
        assert!(
            matches!(proj, ProjectionType::AlbersUsa),
            "a US extent with Alaska/Hawaii should still use albers usa"
        );

        // a global spread -> NaturalEarth world overview, no fitted ranges
        let g_lats = [-40.0_f64, 51.5, 35.7, -33.9, 1.3, 64.1];
        let g_lons = [174.8_f64, -0.1, 139.7, 151.2, 103.8, -21.9];
        let (proj, lonaxis, lataxis) = geo_framing(&g_lats, &g_lons);
        assert!(matches!(proj, ProjectionType::NaturalEarth));
        assert!(lonaxis.is_none() && lataxis.is_none());

        // a local cluster straddling the +/-180 antimeridian: the fitted longitude range would land
        // outside [-180, 180] and clamping it would crop the far-side points, so longitude is left
        // unfit (full width) while latitude is still fit.
        let am_lats = [10.0_f64, 10.5, 11.0, 11.5];
        let am_lons = [179.0_f64, 179.5, -179.5, -179.0];
        let (proj, lonaxis, lataxis) = geo_framing(&am_lats, &am_lons);
        assert!(matches!(proj, ProjectionType::Mercator));
        assert!(
            lonaxis.is_none(),
            "antimeridian-wrapped longitude must not be clamped (would crop points)"
        );
        assert!(
            lataxis.is_some(),
            "latitude is still fit for a local extent"
        );
    }

    #[test]
    fn geo_framing_full_extent_avoids_albers_usa_outside_us() {
        // a US-spanning core alone picks albers-usa (a fixed US composite with no fitted axes). The
        // static-export path feeds geo_framing the FULL-extent box corners (core + outliers)
        // instead, so once a stray outside the US (here far south, lat 10) expands the box, the
        // projection must NOT stay albers-usa — otherwise the full-extent box and the stray would
        // be clipped in a non-pannable image. Corners built the same way the static-export
        // path does.
        let (min_lat, max_lat, min_lon, max_lon) = (10.0, 47.6, -122.3, -74.0);
        let flat = vec![max_lat, max_lat, min_lat, min_lat, max_lat];
        let flon = vec![min_lon, max_lon, max_lon, min_lon, min_lon];
        let (proj, ..) = geo_framing(&flat, &flon);
        assert!(
            !matches!(proj, ProjectionType::AlbersUsa),
            "a full extent reaching outside the US must not use albers-usa"
        );
    }

    #[test]
    fn parse_theme_accepts_names_case_and_hyphens() {
        // all 8 canonical names resolve
        assert!(matches!(
            parse_theme("default"),
            Some(BuiltinTheme::Default)
        ));
        assert!(matches!(
            parse_theme("plotly_white"),
            Some(BuiltinTheme::PlotlyWhite)
        ));
        assert!(matches!(
            parse_theme("plotly_dark"),
            Some(BuiltinTheme::PlotlyDark)
        ));
        assert!(matches!(
            parse_theme("seaborn"),
            Some(BuiltinTheme::Seaborn)
        ));
        assert!(matches!(
            parse_theme("seaborn_whitegrid"),
            Some(BuiltinTheme::SeabornWhitegrid)
        ));
        assert!(matches!(
            parse_theme("seaborn_dark"),
            Some(BuiltinTheme::SeabornDark)
        ));
        assert!(matches!(
            parse_theme("matplotlib"),
            Some(BuiltinTheme::Matplotlib)
        ));
        assert!(matches!(
            parse_theme("plotnine"),
            Some(BuiltinTheme::Plotnine)
        ));

        // case-insensitive, hyphens accepted as separators, surrounding whitespace trimmed
        assert!(matches!(
            parse_theme("Plotly-Dark"),
            Some(BuiltinTheme::PlotlyDark)
        ));
        assert!(matches!(
            parse_theme("  SEABORN_WHITEGRID  "),
            Some(BuiltinTheme::SeabornWhitegrid)
        ));
        // short aliases
        assert!(matches!(
            parse_theme("dark"),
            Some(BuiltinTheme::PlotlyDark)
        ));
        assert!(matches!(
            parse_theme("white"),
            Some(BuiltinTheme::PlotlyWhite)
        ));

        // unknown names are rejected
        assert!(parse_theme("bogus").is_none());
        assert!(parse_theme("").is_none());
    }

    #[test]
    fn aggregate_preserves_input_order() {
        // first-seen order is preserved (not lexicographic 1, 10, 2)
        let xs = vec!["1".to_string(), "10".to_string(), "2".to_string()];
        let ys = vec![1.0, 2.0, 3.0];
        let (out_x, _out_y) = aggregate(xs, ys, Agg::Sum);
        assert_eq!(out_x, vec!["1", "10", "2"]);
    }

    #[test]
    fn sort_line_xy_orders_numeric_x() {
        let xs = vec!["1".to_string(), "10".to_string(), "2".to_string()];
        let ys = vec![1.0, 2.0, 3.0];
        let (out_x, out_y) = sort_line_xy(xs, ys);
        assert_eq!(out_x, vec!["1", "2", "10"]);
        assert_eq!(out_y, vec![1.0, 3.0, 2.0]);
    }

    #[test]
    fn sort_line_xy_preserves_categorical_order() {
        let xs = vec!["b".to_string(), "a".to_string(), "c".to_string()];
        let ys = vec![1.0, 2.0, 3.0];
        let (out_x, _) = sort_line_xy(xs, ys);
        assert_eq!(out_x, vec!["b", "a", "c"]);
    }

    #[test]
    fn cell_geometry_cells_are_within_bounds_and_titled_above() {
        // a 4-panel, 2-column dashboard -> 2 rows, occupying the full band (top = 1.0)
        let (rows, cols, top, offset) = (2, 2, 1.0, 0.01);
        for n in 0..4 {
            let g = cell_geometry(n / cols, n % cols, rows, cols, false, top, offset);
            // domains stay inside the paper area
            assert!(g.x_domain[0] >= 0.0 && g.x_domain[1] <= 1.0 + 1e-9);
            assert!(g.y_domain[0] >= -1e-9 && g.y_domain[1] <= 1.0 + 1e-9);
            // each title is horizontally centered on its cell and sits at/above the cell top
            assert!((g.title_x - (g.x_domain[0] + g.x_domain[1]) / 2.0).abs() < 1e-9);
            assert!(g.title_y >= g.y_domain[1] - 1e-9);
        }
        // top row is higher on the page than the bottom row
        let upper = cell_geometry(0, 0, rows, cols, false, top, offset);
        let lower = cell_geometry(1, 0, rows, cols, false, top, offset);
        assert!(upper.y_domain[0] > lower.y_domain[1]);
        // the two columns don't overlap horizontally
        let left = cell_geometry(0, 0, rows, cols, false, top, offset);
        let right = cell_geometry(0, 1, rows, cols, false, top, offset);
        assert!(left.x_domain[1] <= right.x_domain[0] + 1e-9);
        // a full-width cell spans the whole page regardless of column
        let full = cell_geometry(0, 0, rows, cols, true, top, offset);
        assert_eq!(full.x_domain, vec![0.0, 1.0]);
        assert!((full.title_x - 0.5).abs() < 1e-9);
    }

    #[test]
    fn smart_title_band_fits_every_row_count() {
        // For every dashboard size up to the 8-panel max (including a tall single-column
        // 8-row layout), the top-row panel title — offset above the cell plus its rendered
        // glyph height — must stay at/below y=1 so it never overlaps the dashboard title.
        const GLYPH_PX: f64 = 20.0; // generous estimate of the 13px title's rendered height
        for rows in 1..=MAX_SUBPLOTS {
            let area = smart_plot_area_h(rows);
            let g = cell_geometry(
                0,
                0,
                rows,
                1,
                false,
                smart_grid_top(rows),
                smart_title_offset(rows),
            );
            let title_top = g.title_y + GLYPH_PX / area;
            assert!(
                title_top <= 1.0 + 1e-9,
                "rows={rows}: title_top={title_top} crosses y=1 (would overlap dashboard title)"
            );
            // the title still sits above its own cell
            assert!(g.title_y >= g.y_domain[1] - 1e-9);
        }

        // the reserved band is a real pixel size even for a short one-row dashboard
        let band_px = (1.0 - smart_grid_top(1)) * smart_plot_area_h(1);
        assert!(band_px >= 30.0, "one-row title band too thin: {band_px}px");
    }

    #[test]
    fn cell_geometry_stays_positive_for_many_rows() {
        // Regression: a fixed per-gap paper fraction (VGAP=0.09) overflowed the 0..=top band once
        // there were ~6+ rows, yielding negative cell heights and a garbled dashboard. Now that
        // static image export draws >MAX_SUBPLOTS panels, verify every cell stays positive and
        // in-bounds for tall grids (e.g. a 42-panel, 21-row dashboard), with rows not overlapping.
        for panels in [9usize, 16, 30, 42, 64] {
            let cols = 2;
            let rows = panels.div_ceil(cols);
            let top = smart_grid_top(rows);
            let offset = smart_title_offset(rows);
            for n in 0..panels {
                let g = cell_geometry(n / cols, n % cols, rows, cols, false, top, offset);
                assert!(
                    g.y_domain[1] > g.y_domain[0],
                    "panels={panels} n={n}: non-positive cell height {:?}",
                    g.y_domain
                );
                assert!(
                    g.y_domain[0] >= -1e-9 && g.y_domain[1] <= 1.0 + 1e-9,
                    "panels={panels} n={n}: y-domain out of bounds {:?}",
                    g.y_domain
                );
                assert!(
                    g.x_domain[1] > g.x_domain[0] && g.x_domain[1] <= 1.0 + 1e-9,
                    "panels={panels} n={n}: bad x-domain {:?}",
                    g.x_domain
                );
                // each cell sits strictly below the one directly above it (n - cols)
                if n >= cols {
                    let m = n - cols;
                    let above = cell_geometry(m / cols, m % cols, rows, cols, false, top, offset);
                    assert!(
                        g.y_domain[1] <= above.y_domain[0] + 1e-9,
                        "panels={panels} n={n}: row overlaps the one above"
                    );
                }
            }
        }
    }

    #[test]
    fn smart_grid_layout_gives_overview_panels_full_width_rows() {
        // one overview panel (correlation heatmap) followed by three distribution bars, 2 columns:
        // the heatmap takes a full-width row, the three bars pack into 2 rows below (2 + 1).
        let panels = vec![
            Panel::new(
                "corr".to_string(),
                PanelKind::CorrHeatmap {
                    labels: vec!["a".to_string(), "b".to_string()],
                    matrix: vec![vec![1.0, 0.5], vec![0.5, 1.0]],
                },
            ),
            Panel::new("c1".to_string(), PanelKind::FreqBar { idx: 0 }),
            Panel::new("c2".to_string(), PanelKind::FreqBar { idx: 1 }),
            Panel::new("c3".to_string(), PanelKind::FreqBar { idx: 2 }),
        ];
        let (geoms, rows) = smart_grid_layout(&panels, 2);
        // 1 full-width overview row + ceil(3/2) = 2 grid rows
        assert_eq!(rows, 3);
        // the overview panel spans the full page width and leads the dashboard (top band)
        assert_eq!(geoms[0].x_domain, vec![0.0, 1.0]);
        // the distribution bars are NOT full width (two columns)
        assert!(geoms[1].x_domain[1] < 1.0 - 1e-9);
        assert!(geoms[2].x_domain[0] > 1e-9);
        // the first bar row sits below the overview row
        assert!(geoms[1].y_domain[1] <= geoms[0].y_domain[0] + 1e-9);
        // the two bars on the same row share a y-band but occupy different columns
        assert_eq!(geoms[1].y_domain, geoms[2].y_domain);
        assert!(geoms[1].x_domain[1] <= geoms[2].x_domain[0] + 1e-9);
        // the third bar wraps to its own (last) row, below the first bar row
        assert!(geoms[3].y_domain[1] <= geoms[1].y_domain[0] + 1e-9);
    }

    #[test]
    fn axis_layout_keys_align_with_trace_refs_beyond_eight() {
        // render_smart_grid_json lets static export exceed the typed Layout's 8-axis limit only if
        // each cell's injected layout key (xaxis{n}) matches the axis its trace references (x{n}).
        // Verify the two naming schemes agree for the first dozen positions — including the pos-1
        // special case ("xaxis"/"x", not "xaxis1"/"x1") — so panels 9+ aren't silently orphaned.
        for (prefix, axis_word) in [('x', "xaxis"), ('y', "yaxis")] {
            assert_eq!(axis_json_key(prefix, 1), axis_word);
            assert_eq!(axis_ref(prefix, 1), prefix.to_string());
            for pos in 2..=12usize {
                let key = axis_json_key(prefix, pos);
                let reference = axis_ref(prefix, pos);
                assert_eq!(key, format!("{axis_word}{pos}"));
                assert_eq!(reference, format!("{prefix}{pos}"));
                // a layout key like "xaxis9" is referenced by its trace as "x9"
                assert_eq!(reference.trim_start_matches(prefix), pos.to_string());
            }
        }

        // a domain-positioned, cross-anchored axis serializes to the JSON shape plotly.js
        // expects under the injected key: a 2-element `domain` array and an `anchor` string.
        let axis = styled_y_axis(None, false, None)
            .domain(&[0.0, 0.5])
            .anchor("x9");
        let v = serde_json::to_value(&axis).unwrap();
        assert_eq!(
            v.get("domain").and_then(|d| d.as_array()).map(Vec::len),
            Some(2)
        );
        assert_eq!(
            v.get("anchor").and_then(serde_json::Value::as_str),
            Some("x9")
        );
    }

    #[test]
    fn parse_map_style_token_free_and_aliases() {
        // token-free styles resolve without requiring a token
        for name in [
            "open-street-map",
            "osm",
            "carto-positron",
            "carto-darkmatter",
            "stamen-terrain",
            "stamen-toner",
            "stamen-watercolor",
            "white-bg",
        ] {
            let (_, needs_token) = parse_map_style(name).unwrap();
            assert!(!needs_token, "{name} should be token-free");
        }
        // matching is case-insensitive
        assert!(parse_map_style("Carto-Positron").is_ok());
    }

    #[test]
    fn parse_map_style_mapbox_hosted_needs_token() {
        for name in [
            "basic",
            "streets",
            "outdoors",
            "light",
            "dark",
            "satellite",
            "satellite-streets",
        ] {
            let (_, needs_token) = parse_map_style(name).unwrap();
            assert!(needs_token, "{name} should require a token");
        }
    }

    #[test]
    fn parse_map_style_unknown_errors() {
        assert!(parse_map_style("not-a-style").is_err());
    }

    #[test]
    fn map_center_zoom_centers_on_bounding_box() {
        // a tight cluster around (40, -75) centers there and zooms in
        let lats = [39.9, 40.1, 40.0];
        let lons = [-75.1, -74.9, -75.0];
        let (center, zoom) = map_center_zoom(&lats, &lons, 0.0, 1000.0, 600.0);
        let v = serde_json::to_value(&center).unwrap();
        assert!((v["lat"].as_f64().unwrap() - 40.0).abs() < 1e-9);
        assert!((v["lon"].as_f64().unwrap() - (-75.0)).abs() < 1e-9);
        // small span -> high zoom; large span -> low zoom. Use a genuinely wide (non-wrapping)
        // longitude range so this exercises the plain bounding-box path, not antimeridian wrap.
        let (_, world_zoom) = map_center_zoom(&[-60.0, 70.0], &[-80.0, 80.0], 0.0, 1000.0, 600.0);
        assert!(
            zoom > world_zoom,
            "tight cluster should zoom in more than world"
        );
    }

    #[test]
    fn map_center_zoom_single_point() {
        // a zero-span (single point) dataset gets a sensible non-extreme zoom
        let (_, zoom) = map_center_zoom(&[51.5], &[-0.12], 0.0, 1000.0, 600.0);
        assert!((1..=16).contains(&zoom));
    }

    #[test]
    fn map_center_zoom_handles_antimeridian() {
        // a tight cluster straddling the 180° line (179 and -179) is ~2° wide, not ~358°, so it
        // centers near the date line and zooms in rather than framing the whole globe at lon 0.
        let lats = [18.0, 16.0];
        let lons = [179.0, -179.0];
        let (center, zoom) = map_center_zoom(&lats, &lons, 0.0, 1000.0, 600.0);
        let lon = serde_json::to_value(&center).unwrap()["lon"]
            .as_f64()
            .unwrap();
        assert!(
            lon.abs() > 170.0,
            "center lon should be near 180, got {lon}"
        );
        assert!(
            zoom >= 5,
            "tight antimeridian cluster should zoom in, got {zoom}"
        );
    }

    #[test]
    fn lon_center_and_span_non_wrapping() {
        // ordinary western-hemisphere cluster: plain midpoint + span, no wrap
        let (center, span) = lon_center_and_span(&[-75.1, -74.9, -75.0], 0.0);
        assert!((center - (-75.0)).abs() < 1e-9);
        assert!((span - 0.2).abs() < 1e-9);
    }

    #[test]
    fn lon_center_and_span_crossing_trims_outlier() {
        // a tight cluster straddling +/-180 plus one far in-range longitude outlier (80). The
        // crossing branch must trim like the non-crossing one: untrimmed, the unwrapped span runs
        // from the outlier (~80) to the cluster (~182) and reads as global; trimming the outlier
        // away keeps the span local.
        let mut lons = vec![80.0_f64]; // single far outlier
        for k in 0..100 {
            lons.push(178.0 + (k % 2) as f64); // 178 / 179
            lons.push(-179.0 + (k % 2) as f64); // -179 / -178
        }
        let (_, untrimmed) = lon_center_and_span(&lons, 0.0);
        let (_, trimmed) = lon_center_and_span(&lons, MAP_FRAME_TRIM_FRAC);
        assert!(
            untrimmed >= 90.0,
            "untrimmed crossing span should include the outlier, got {untrimmed}"
        );
        assert!(
            trimmed < 10.0,
            "trimmed crossing span should exclude the outlier, got {trimmed}"
        );
    }

    #[test]
    fn sorted_quantile_nearest_rank() {
        let s: Vec<f64> = (0..=100).map(f64::from).collect(); // 0..=100
        assert_eq!(sorted_quantile(&s, 0.0), 0.0);
        assert_eq!(sorted_quantile(&s, 1.0), 100.0);
        assert_eq!(sorted_quantile(&s, 0.5), 50.0);
        assert_eq!(sorted_quantile(&s, 0.01), 1.0);
        assert_eq!(sorted_quantile(&s, 0.99), 99.0);
        assert_eq!(sorted_quantile(&[], 0.5), 0.0); // empty -> 0.0
    }

    #[test]
    fn downsample_pair_caps_and_preserves_shape() {
        let xs: Vec<f64> = (0..1000).map(f64::from).collect();
        let ys: Vec<f64> = xs.iter().map(|v| v * 2.0).collect();
        let (dx, dy) = downsample_pair(&xs, &ys, 100);
        assert_eq!(dx.len(), 100);
        assert_eq!(dy.len(), 100);
        // row alignment is preserved (ys == 2*xs throughout)
        assert!(dx.iter().zip(&dy).all(|(x, y)| (y - x * 2.0).abs() < 1e-9));
        // endpoint-inclusive: both the first AND the last observation are retained, so a
        // chronologically sorted time-series keeps its latest point
        assert_eq!(dx[0], 0.0);
        assert_eq!(*dx.last().unwrap(), 999.0);
        assert_eq!(*dy.last().unwrap(), 1998.0);
        // single-element cap doesn't divide by zero and returns the first point
        assert_eq!(downsample_pair(&xs, &ys, 1), (vec![0.0], vec![0.0]));
        // already within cap -> returned unchanged
        let (sx, sy) = downsample_pair(&xs[..50], &ys[..50], 100);
        assert_eq!(sx.len(), 50);
        assert_eq!(sy.len(), 50);
        // cap == 0 -> unchanged (no divide-by-zero)
        assert_eq!(downsample_pair(&xs, &ys, 0).0.len(), 1000);
    }

    #[test]
    fn lttb_indices_keeps_endpoints_and_spike() {
        // a flat baseline with a single tall spike that sits BETWEEN uniform-stride samples
        let n = 1000;
        let cap = 50;
        let spike = 333usize; // 333 * (n-1) / (cap-1) is not an integer -> stride would miss it
        let xs: Vec<f64> = (0..n).map(|i| i as f64).collect();
        let ys: Vec<f64> = (0..n)
            .map(|i| if i == spike { 1000.0 } else { 0.0 })
            .collect();

        // baseline: uniform stride steps right over the spike
        let stride: Vec<usize> = (0..cap).map(|i| i * (n - 1) / (cap - 1)).collect();
        assert!(
            !stride.contains(&spike),
            "test precondition: stride must miss the spike"
        );
        let (_, stride_y) = downsample_pair(&xs, &ys, cap);
        assert!(
            stride_y.iter().all(|&y| y < 1.0),
            "uniform stride drops the spike"
        );

        // LTTB selects by triangle area, so the spike survives
        let keep = lttb_indices(&xs, &ys, cap);
        assert_eq!(keep.len(), cap);
        assert_eq!(keep[0], 0, "first point always kept");
        assert_eq!(*keep.last().unwrap(), n - 1, "last point always kept");
        assert!(
            keep.contains(&spike),
            "LTTB preserves the peak a uniform stride misses"
        );
        // indices are strictly increasing (monotonic selection)
        assert!(keep.windows(2).all(|w| w[0] < w[1]));

        // edge caps: no panic, sensible output
        assert_eq!(lttb_indices(&xs, &ys, 0), (0..n).collect::<Vec<_>>());
        assert_eq!(lttb_indices(&xs, &ys, 1), vec![0]);
        assert_eq!(lttb_indices(&xs, &ys, 2), vec![0, n - 1]);
        // already within cap -> all indices unchanged
        assert_eq!(
            lttb_indices(&xs[..10], &ys[..10], 50),
            (0..10).collect::<Vec<_>>()
        );
    }

    /// A tight NYC cluster plus a handful of far-west bad coordinates, used to contrast the
    /// trimmed (`viz smart`) framing against the full-extent (`viz map`) framing.
    fn nyc_cluster_with_outliers() -> (Vec<f64>, Vec<f64>) {
        let mut lats = vec![40.7_f64; 200];
        let mut lons = vec![-73.95_f64; 200];
        // jitter so the cluster has a small but non-zero span
        for (i, (la, lo)) in lats.iter_mut().zip(lons.iter_mut()).enumerate() {
            *la += (i % 10) as f64 * 0.005;
            *lo += (i % 10) as f64 * 0.005;
        }
        // ~2.5% outliers dragging the raw bounding box out to Pennsylvania
        for _ in 0..5 {
            lats.push(40.5);
            lons.push(-77.5);
        }
        (lats, lons)
    }

    #[test]
    fn map_center_zoom_trimmed_ignores_outliers() {
        // robust (percentile-trimmed) framing must center on the cluster, not the outliers.
        let (lats, lons) = nyc_cluster_with_outliers();
        let (center, _zoom) = map_center_zoom(&lats, &lons, MAP_FRAME_TRIM_FRAC, 1000.0, 600.0);
        let lon = serde_json::to_value(&center).unwrap()["lon"]
            .as_f64()
            .unwrap();
        // raw min/max midpoint would be ~(-73.95 + -77.5)/2 = -75.7 (in Pennsylvania); the trimmed
        // framing must stay over the NYC cluster instead.
        assert!(
            lon > -74.3,
            "trimmed framing should not be pulled west of NYC, got lon {lon}"
        );
    }

    #[test]
    fn map_center_zoom_full_extent_includes_outliers() {
        // standalone `viz map` passes trim_frac == 0.0, so every coordinate frames the view; the
        // far-west outliers pull the center toward the raw midpoint (regression guard for the fix
        // that kept the trimming scoped to `viz smart`).
        let (lats, lons) = nyc_cluster_with_outliers();
        let (center, _zoom) = map_center_zoom(&lats, &lons, 0.0, 1000.0, 600.0);
        let lon = serde_json::to_value(&center).unwrap()["lon"]
            .as_f64()
            .unwrap();
        assert!(
            lon < -75.0,
            "full-extent framing must include the western outliers, got lon {lon}"
        );
    }

    #[test]
    fn auto_hierarchy_style_by_depth() {
        // best-practice rule: shallow (≤2 levels) → treemap, deep (3+ levels) → sunburst.
        assert_eq!(auto_hierarchy_style(2), HierStyle::Treemap);
        assert_eq!(auto_hierarchy_style(3), HierStyle::Sunburst);
        assert_eq!(auto_hierarchy_style(4), HierStyle::Sunburst);
    }

    #[test]
    fn resolve_hierarchy_style_flag() {
        assert_eq!(
            resolve_hierarchy_style(None, 2).unwrap(),
            HierStyle::Treemap
        );
        assert_eq!(
            resolve_hierarchy_style(Some("auto"), 3).unwrap(),
            HierStyle::Sunburst
        );
        // case-insensitive explicit override wins over the depth rule
        assert_eq!(
            resolve_hierarchy_style(Some("Treemap"), 3).unwrap(),
            HierStyle::Treemap
        );
        assert_eq!(
            resolve_hierarchy_style(Some("sunburst"), 2).unwrap(),
            HierStyle::Sunburst
        );
        assert!(resolve_hierarchy_style(Some("pie"), 2).is_err());
    }

    #[test]
    fn hierarchy_arrays_rolls_up_and_keeps_ids_unique() {
        // 2-level hierarchy; "Widgets"/"Gadgets" repeat under two regions to exercise id
        // uniqueness.
        let mut leaves: HashMap<Vec<String>, f64> = HashMap::new();
        leaves.insert(vec!["East".into(), "Widgets".into()], 150.0);
        leaves.insert(vec!["East".into(), "Gadgets".into()], 30.0);
        leaves.insert(vec!["West".into(), "Widgets".into()], 80.0);
        leaves.insert(vec!["West".into(), "Gadgets".into()], 20.0);
        leaves.insert(vec!["West".into(), "Gizmos".into()], 10.0);

        let (labels, parents, values, ids) =
            hierarchy_arrays(&leaves, 2, 12, 200, "All").expect("hierarchy");

        // synthetic root: parentless, labeled, value == grand total.
        assert_eq!(ids[0], HIER_ROOT_ID);
        assert_eq!(parents[0], "");
        assert_eq!(labels[0], "All");
        let total: f64 = leaves.values().sum();
        assert!((values[0] - total).abs() < 1e-9);

        // every non-root parent reference resolves to a real id.
        let id_set: std::collections::HashSet<&String> = ids.iter().collect();
        for (i, p) in parents.iter().enumerate().skip(1) {
            assert!(
                id_set.contains(p),
                "parent {p} of node {} not in ids",
                ids[i]
            );
        }
        // path-joined ids are unique despite repeated child labels.
        assert_eq!(id_set.len(), ids.len(), "ids must be unique");

        // a level-1 node's value == sum of its level-2 children (East = 150 + 30).
        let east_id = format!("{HIER_ROOT_ID}{HIER_PATH_SEP}East");
        let east_val = values[ids.iter().position(|x| *x == east_id).unwrap()];
        assert!((east_val - 180.0).abs() < 1e-9);
    }

    #[test]
    fn hierarchy_arrays_folds_remainder_into_other() {
        // parent "R" has 4 children; top_n = 2 keeps a,b and folds c,d into "Other (2)".
        let mut leaves: HashMap<Vec<String>, f64> = HashMap::new();
        for (seg, v) in [("a", 40.0), ("b", 30.0), ("c", 20.0), ("d", 10.0)] {
            leaves.insert(vec!["R".into(), seg.into()], v);
        }
        // a second top-level category so the root itself splits (required for a real hierarchy).
        leaves.insert(vec!["S".into(), "x".into()], 5.0);

        let (labels, _parents, values, _ids) =
            hierarchy_arrays(&leaves, 2, 2, 200, "All").expect("hierarchy");

        let other_pos = labels
            .iter()
            .position(|l| l == "Other (2)")
            .expect("Other bucket");
        assert!((values[other_pos] - 30.0).abs() < 1e-9); // 20 + 10 dropped

        // kept children (40 + 30) + Other (30) == R's rolled-up value (100).
        let r_pos = labels.iter().position(|l| l == "R").unwrap();
        assert!((values[r_pos] - 100.0).abs() < 1e-9);
    }

    #[test]
    fn hierarchy_arrays_degenerate_returns_none() {
        // a single chain (no node splits into 2+) isn't worth a panel.
        let mut leaves: HashMap<Vec<String>, f64> = HashMap::new();
        leaves.insert(vec!["only".into(), "one".into()], 5.0);
        assert!(hierarchy_arrays(&leaves, 2, 12, 200, "All").is_none());
    }

    #[test]
    fn cramers_v_zero_for_independent_dims() {
        // Perfectly independent 2x2 joint (cell = row_tot*col_tot/n): chi-square is 0, so the
        // (bias-corrected) V is 0 — below HIER_MIN_ASSOCIATION_CRAMERS_V, so `viz smart` would
        // SKIP the hierarchy (this is exactly the sales_sample region/payment/category case).
        let mut leaves: HashMap<Vec<String>, f64> = HashMap::new();
        leaves.insert(vec!["a1".into(), "b1".into()], 30.0);
        leaves.insert(vec!["a1".into(), "b2".into()], 30.0);
        leaves.insert(vec!["a2".into(), "b1".into()], 20.0);
        leaves.insert(vec!["a2".into(), "b2".into()], 20.0);
        let v = max_pairwise_cramers_v(&leaves, 2);
        assert!(
            v < HIER_MIN_ASSOCIATION_CRAMERS_V,
            "independent V={v} should be ~0"
        );
    }

    #[test]
    fn cramers_v_high_for_nested_dims() {
        // Perfect nesting: each child category lives under exactly one parent (a real hierarchy),
        // so association is near-total and V clears the threshold — `viz smart` builds the panel.
        let mut leaves: HashMap<Vec<String>, f64> = HashMap::new();
        leaves.insert(vec!["a1".into(), "b1".into()], 25.0);
        leaves.insert(vec!["a1".into(), "b2".into()], 25.0);
        leaves.insert(vec!["a2".into(), "b3".into()], 25.0);
        leaves.insert(vec!["a2".into(), "b4".into()], 25.0);
        let v = max_pairwise_cramers_v(&leaves, 2);
        assert!(
            v >= HIER_MIN_ASSOCIATION_CRAMERS_V,
            "nested V={v} should be high"
        );
    }

    #[test]
    fn cramers_v_tiny_table_falls_back_to_uncorrected() {
        // 3x3 perfectly one-to-one table with only 3 rows: the Bergsma correction is undefined
        // (r_corr/c_corr collapse to 1.0, denom 0), but the dims ARE perfectly associated. The
        // uncorrected fallback must report a high V so this small hierarchy isn't wrongly skipped.
        let mut leaves: HashMap<Vec<String>, f64> = HashMap::new();
        leaves.insert(vec!["a1".into(), "b1".into()], 1.0);
        leaves.insert(vec!["a2".into(), "b2".into()], 1.0);
        leaves.insert(vec!["a3".into(), "b3".into()], 1.0);
        let v = max_pairwise_cramers_v(&leaves, 2);
        assert!(
            v >= HIER_MIN_ASSOCIATION_CRAMERS_V,
            "tiny perfectly-associated table V={v} should not read as independent"
        );
    }

    #[test]
    fn cramers_v_max_over_pairs_and_guards() {
        // fewer than two dims is undefined -> 0.0
        let mut single: HashMap<Vec<String>, f64> = HashMap::new();
        single.insert(vec!["x".into()], 10.0);
        assert_eq!(max_pairwise_cramers_v(&single, 1), 0.0);

        // 3 dims: levels 0&1 independent, but level 2 is perfectly nested under level 0; the MAX
        // over all pairs must surface that association so the panel isn't wrongly skipped.
        let mut leaves: HashMap<Vec<String>, f64> = HashMap::new();
        leaves.insert(vec!["a1".into(), "b1".into(), "c1".into()], 25.0);
        leaves.insert(vec!["a1".into(), "b2".into(), "c1".into()], 25.0);
        leaves.insert(vec!["a2".into(), "b1".into(), "c2".into()], 25.0);
        leaves.insert(vec!["a2".into(), "b2".into(), "c2".into()], 25.0);
        let v = max_pairwise_cramers_v(&leaves, 3);
        assert!(
            v >= HIER_MIN_ASSOCIATION_CRAMERS_V,
            "max-pair V={v} should catch nested level"
        );
    }
}
