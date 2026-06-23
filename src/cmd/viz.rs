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
    map         Geographic point map (or --density heatmap) on tile basemaps.
                Pick the coordinate columns with the lat/lon options below.
    geo         Geographic point map on a projection basemap (coastlines/land/
                countries; no tiles, no token). Uses the same lat/lon options
                as `map`, plus --projection. Good for global/country-scale data.

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
    qsv viz --help

viz options:
    -x, --x <col>          Column for the x-axis / category / bin / group.
    -y, --y <col>          Column for the y-axis / value.
    -z, --z <col>          The z column: a heatmap pivot value (with --x and --y), or
                           the third numeric axis for scatter3d.
    --cols <cols>          Columns to use. For heatmap: numeric columns for the
                           correlation matrix (default: all numeric). For radar:
                           the numeric axes to plot.
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
                           each row counts as a flow of 1.
    --bins <n>             Number of bins. For histogram: bins along the x-axis
                           (default: auto). For contour: the per-axis resolution of
                           the density grid (default: 20).
    --agg <fn>             For bar/line, aggregate the y values when the x value
                           repeats. One of: sum, mean, count, min, max.
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
    Bar, BoxPlot, Candlestick, Configuration, Contour, DensityMapbox, HeatMap, Histogram, Ohlc,
    Pie, Plot, Sankey, Scatter, Scatter3D, ScatterGeo, ScatterMapbox, ScatterPolar, Trace,
    box_plot::{BoxPoints, QuartileMethod},
    color::NamedColor,
    common::{
        Anchor, ColorBar, ColorScale, ColorScalePalette, Fill, Font, HoverInfo, Line, Marker, Mode,
        Pattern, PatternShape, TextPosition, TickMode, Title,
    },
    layout::{
        Annotation, Axis, AxisType, Center, Layout, LayoutGeo, LayoutScene, Mapbox, MapboxStyle,
        Margin, Projection, ProjectionType, themes::BuiltinTheme,
    },
    sankey::{Link, Node},
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

/// `viz smart --log-scale auto`: a frequency bar panel switches to a logarithmic y-axis when
/// its tallest bar is at least this many times the shortest positive bar. A dominating
/// "(NULL)"/"Other (N)" bucket flattens the real categories to invisible slivers on a linear
/// axis; a log scale keeps them legible. Requires at least 3 positive bars.
const LOG_SCALE_MIN_RATIO: f64 = 50.0;

/// Minimum absolute Pearson correlation for `viz smart` to add a scatter panel of the most
/// strongly correlated numeric pair. Below this (a weak relationship) the scatter is just a
/// noise cloud, so it's skipped — the correlation heatmap already conveys weak correlations.
const SCATTER_PAIR_MIN_ABS_R: f64 = 0.5;

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

/// Bimodality-coefficient threshold (Sarle's BC). A continuous numeric column whose moarstats
/// `bimodality_coefficient` reaches this is treated as bimodal/multimodal, so `viz smart` draws
/// a histogram (which shows the separate peaks) instead of a box plot (which hides them).
/// The 0.555 cutoff is the standard uniform-distribution reference value.
const BIMODALITY_COEFFICIENT_THRESHOLD: f64 = 0.555;

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
    cmd_smart:         bool,
    cmd_bar:           bool,
    cmd_line:          bool,
    cmd_scatter:       bool,
    cmd_scatter3d:     bool,
    cmd_histogram:     bool,
    cmd_box:           bool,
    cmd_pie:           bool,
    cmd_heatmap:       bool,
    cmd_contour:       bool,
    cmd_candlestick:   bool,
    cmd_ohlc:          bool,
    cmd_sankey:        bool,
    cmd_radar:         bool,
    cmd_map:           bool,
    cmd_geo:           bool,
    arg_input:         Option<String>,
    flag_x:            Option<SelectColumns>,
    flag_y:            Option<SelectColumns>,
    flag_z:            Option<SelectColumns>,
    flag_cols:         Option<SelectColumns>,
    flag_series:       Option<SelectColumns>,
    flag_donut:        bool,
    // scatter encodings: map a numeric column to per-point marker color (continuous
    // colorscale) and/or marker size (bubble chart). Mutually exclusive with --series.
    flag_color:        Option<SelectColumns>,
    flag_size:         Option<SelectColumns>,
    // candlestick / ohlc columns (--open is already taken by the browser-open flag below,
    // so the open-price column is selected with --ohlc-open)
    flag_ohlc_open:    Option<SelectColumns>,
    flag_high:         Option<SelectColumns>,
    flag_low:          Option<SelectColumns>,
    flag_close:        Option<SelectColumns>,
    // sankey columns
    flag_source:       Option<SelectColumns>,
    flag_target:       Option<SelectColumns>,
    flag_value:        Option<SelectColumns>,
    // map columns/options
    flag_lat:          Option<SelectColumns>,
    flag_lon:          Option<SelectColumns>,
    flag_text:         Option<SelectColumns>,
    flag_density:      bool,
    flag_style:        Option<String>,
    flag_mapbox_token: Option<String>,
    flag_projection:   Option<String>,
    flag_bins:         Option<usize>,
    flag_agg:          Option<String>,
    flag_box_points:   Option<String>,
    flag_max_charts:   usize,
    flag_grid_cols:    usize,
    flag_limit:        usize,
    flag_no_nulls:     bool,
    flag_no_other:     bool,
    flag_smarter:      bool,
    flag_log_scale:    String,
    flag_title:        Option<String>,
    flag_x_title:      Option<String>,
    flag_y_title:      Option<String>,
    flag_theme:        Option<String>,
    // width/height/scale only affect static image export (the viz_static feature). width
    // and height are optional: when unset, `viz smart` derives them from its grid shape and
    // other charts fall back to the defaults below.
    flag_width:        Option<usize>,
    flag_height:       Option<usize>,
    #[cfg_attr(not(feature = "viz_static"), allow(dead_code))]
    flag_scale:        f64,
    flag_open:         bool,
    flag_output:       Option<String>,
    flag_delimiter:    Option<Delimiter>,
    flag_no_headers:   bool,
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
            SmartRender::Grid { plot, dims } => (plot, Some(dims)),
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
        plot: Box<Plot>,
        dims: (usize, usize),
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
    if matches!(chart_kind(args), Chart::Scatter3D) {
        return build_scatter3d_plot(args);
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

    plot.set_layout(build_layout(args, default_x, default_y));
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

    // standalone `viz map`: frame the full extent — its edge coordinates are intentional. --width/
    // --height size the static image export but are NOT applied to the responsive HTML layout, so
    // only honor them when exporting an image (fit the actual export aspect instead of clipping);
    // HTML frames for the representative default aspect, matching how `run` sizes each output.
    let (fit_w, fit_h) = if out_format.is_image() {
        (
            args.flag_width.unwrap_or(DEFAULT_IMG_WIDTH),
            args.flag_height.unwrap_or(DEFAULT_IMG_HEIGHT),
        )
    } else {
        (DEFAULT_IMG_WIDTH, DEFAULT_IMG_HEIGHT)
    };
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
    let mut pie = Pie::new(values).labels(order).text_info("label+percent");
    if args.flag_donut {
        pie = pie.hole(0.4);
    }
    Ok(pie)
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
    let trace: Box<dyn Trace> = if ohlc {
        Ohlc::new(xs, open, high, low, close)
    } else {
        Candlestick::new(xs, open, high, low, close)
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

#[derive(Clone, Copy)]
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
        // continuous numeric -> box plot from precomputed quartiles
        if let (Some(q1), Some(median), Some(q3)) = (s.q1, s.q2_median, s.q3)
            && s.cardinality > 1
        {
            // moarstats refinement: a box plot summarizes a column by its quartiles, which hides
            // multiple peaks. When moarstats has flagged this column as bimodal/multimodal, a
            // histogram tells the truth instead. This is the one smart panel that needs the raw
            // values (gathered later in a single batched pass), taken only for the few columns
            // actually flagged bimodal — the same "selective extra pass" cost as the correlation
            // and time-series panels. Absent moarstats, `bimodality_coefficient` is None and we
            // fall through to the cache-only box plot (today's behavior).
            if s.bimodality_coefficient
                .is_some_and(|bc| bc >= BIMODALITY_COEFFICIENT_THRESHOLD)
            {
                return Some(PanelKind::Histogram { idx });
            }
            // Use the actual observed min/max as the whisker endpoints. We intentionally do
            // NOT use the Tukey inner fences: a fence is a computed threshold (Q1-1.5*IQR /
            // Q3+1.5*IQR) that need not coincide with any value in the dataset, so plotting
            // it as a whisker endpoint would fabricate a data point. min/max ARE observed
            // values, and since this is a precomputed box (no raw points / outlier markers),
            // min-to-max whiskers honestly convey the column's full range without re-scanning.
            let lower = s.min.as_deref().and_then(|v| v.trim().parse::<f64>().ok());
            let upper = s.max.as_deref().and_then(|v| v.trim().parse::<f64>().ok());
            return Some(PanelKind::BoxStats {
                q1,
                median,
                q3,
                lower,
                upper,
                mean: s.mean,
            });
        }
        return None;
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

fn build_timeseries_panel(
    args: &Args,
    stats: &[crate::cmd::stats::StatsData],
    prefer_dmy: bool,
    map_cols: Option<(usize, usize)>,
) -> CliResult<Option<Panel>> {
    use qsv_dateparser::parse_with_preference;

    // first date/datetime column (stats emits "Date" / "DateTime" once dates are inferred)
    let Some((date_idx, is_datetime)) =
        stats
            .iter()
            .enumerate()
            .find_map(|(i, s)| match s.r#type.as_str() {
                "Date" => Some((i, false)),
                "DateTime" => Some((i, true)),
                _ => None,
            })
    else {
        return Ok(None);
    };

    // y-axis: the first continuous numeric column (high-cardinality or near-unique, i.e. NOT a
    // low-cardinality categorical), preferring a Float over an Integer. Near-unique columns are
    // deliberately allowed here — a measurement like revenue or temperature is often near-unique
    // yet makes the most meaningful trend — unlike the box/correlation panels, which treat
    // near-unique integers as ID-like and skip them. Coordinate columns claimed by the map panel
    // are excluded so the trend doesn't end up plotting e.g. latitude over time.
    let is_map_col = |idx: usize| map_cols.is_some_and(|(la, lo)| idx == la || idx == lo);
    let continuous_numeric = |s: &crate::cmd::stats::StatsData| {
        if !matches!(s.r#type.as_str(), "Integer" | "Float") || s.cardinality <= 1 {
            return false;
        }
        let near_unique = s.uniqueness_ratio.is_some_and(|r| r > 0.95);
        let low_cardinality = s.cardinality <= CATEGORICAL_MAX_CARDINALITY && !near_unique;
        !low_cardinality
    };
    let Some(y_idx) = stats
        .iter()
        .enumerate()
        .filter(|(i, s)| !is_map_col(*i) && continuous_numeric(s))
        .min_by_key(|(_, s)| usize::from(s.r#type != "Float"))
        .map(|(i, _)| i)
    else {
        return Ok(None);
    };

    let (mut rdr, headers, nh) = reader_and_headers(args)?;

    // collect (timestamp_ms, formatted_date, y), skipping rows missing either field. dates are
    // parsed with the same DMY preference stats used to infer the column, so DMY-formatted
    // dates (e.g. with QSV_PREFER_DMY set) sort and render correctly.
    let mut points: Vec<(i64, String, f64)> = Vec::new();
    let mut record = csv::ByteRecord::new();
    while rdr.read_byte_record(&mut record)? {
        // skip non-finite y (NaN/inf): parse_f64 accepts "NaN"/"inf", but a single non-finite
        // value would poison LTTB's bucket averages and area comparisons (and render as a gap)
        let Some(y) = parse_f64(record.get(y_idx)).filter(|v| v.is_finite()) else {
            continue;
        };
        let Some(raw) = record.get(date_idx) else {
            continue;
        };
        let Ok(text) = std::str::from_utf8(raw) else {
            continue;
        };
        let text = text.trim();
        if text.is_empty() {
            continue;
        }
        let Ok(dt) = parse_with_preference(text, prefer_dmy) else {
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

    let y_label = col_label(&headers, y_idx, nh);
    let date_label = col_label(&headers, date_idx, nh);
    // points are chronologically sorted (monotonic x), so LTTB can downsample by triangle area,
    // preserving spikes/peaks a uniform stride would step over. Timestamps give LTTB its numeric
    // x-axis; the display labels are gathered alongside the selected indices.
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
    Ok(Some(Panel::new(
        format!("{y_label} over {date_label}"),
        PanelKind::TimeSeries { y_label, xs, ys },
    )))
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

/// Detect a latitude/longitude column pair (by header name + numeric stats type) and, if a usable
/// pair exists, build a `viz smart` map panel. Does one extra data pass to collect the in-range
/// coordinates (the stats cache holds no geometry). Returns `None` when no pair is found, the
/// columns aren't numeric, or no row has valid coordinates. On success returns the panel together
/// with the (lat, lon) column indices it consumed, so the caller can exclude exactly those columns
/// from the other panels — and only when a map is actually rendered. Name detection needs headers,
/// so this is a no-op under `--no-headers`.
fn build_map_panel(
    args: &Args,
    stats: &[crate::cmd::stats::StatsData],
) -> CliResult<Option<(Panel, (usize, usize))>> {
    let Some((lat_idx, lon_idx)) = latlon_indices(stats) else {
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
        .font(Font::new().size(11))
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

    let (_headers, stats) = util::get_stats_records(&schema_args, util::StatsMode::ProfileSchema)?;
    if stats.is_empty() {
        return fail_clierror!(
            "Could not compute statistics for `viz smart`. The input must be a regular CSV/TSV \
             file (not stdin or a compressed/special format)."
        );
    }

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
    let map_panel = {
        let panel = build_map_panel(args, &stats)?;
        if out_format.is_image() {
            panel.map(|(p, cols)| {
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
                (
                    Panel {
                        name: p.name,
                        kind,
                        #[cfg(feature = "geocode")]
                        geo_meta: p.geo_meta,
                    },
                    cols,
                )
            })
        } else {
            panel
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
        let name = if s.field.is_empty() {
            format!("col {}", idx + 1)
        } else {
            s.field.clone()
        };
        match classify(idx, s) {
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
            // or a scatter otherwise.
            let pair_panel = pair.map(|(i, j, r)| {
                let name = format!("{} vs {} (r={r:.2})", labels[i], labels[j]);
                if columns[i].len() >= SMART_CONTOUR_MIN_POINTS {
                    let (x, y, z) = bin_2d(&columns[i], &columns[j], SMART_CONTOUR_BINS);
                    Panel::new(name, PanelKind::ContourPair { x, y, z })
                } else {
                    let (xs, ys) = downsample_pair(&columns[i], &columns[j], MAX_SMART_POINTS);
                    Panel::new(name, PanelKind::ScatterPair { xs, ys })
                }
            });

            // 3D drill-down: with 3+ numeric columns, a Scatter3D of the strongest-correlation
            // triple — the strongest pair plus the third column most correlated with that pair.
            // A 3D scene can't share the typed x/y subplot grid, so it's built ONLY for HTML output
            // (which uses the inline render path, like the map panel). Static image export goes
            // through the typed grid, where a 3D panel would hit `panel_trace`'s unreachable arm.
            let scatter3d_panel = pair
                .filter(|_| columns.len() >= 3 && !out_format.is_image())
                .and_then(|(i, j, _)| {
                    let third =
                        (0..columns.len())
                            .filter(|&k| k != i && k != j)
                            .max_by(|&a, &b| {
                                let sa = matrix[i][a].abs() + matrix[j][a].abs();
                                let sb = matrix[i][b].abs() + matrix[j][b].abs();
                                sa.partial_cmp(&sb).unwrap_or(std::cmp::Ordering::Equal)
                            });
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

            panels.insert(
                0,
                Panel::new(
                    "Correlation".to_string(),
                    PanelKind::CorrHeatmap { labels, matrix },
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

    // prepend a time-series trend panel when the data has a date/datetime column and a
    // continuous numeric column. Like the correlation panel, it does one extra data pass and
    // is prepended so it survives the panel cap.
    // mirror the DMY preference stats used to infer dates: `viz smart` builds stats with
    // flag_prefer_dmy = false, and stats itself ORs in QSV_PREFER_DMY (see `cmd::stats`), so the
    // effective preference is the env flag. Parse dates the same way here so DMY-formatted dates
    // are ordered correctly rather than misparsed/dropped.
    let prefer_dmy = util::get_envvar_flag("QSV_PREFER_DMY");
    if let Some(panel) = build_timeseries_panel(args, &stats, prefer_dmy, map_cols)? {
        panels.insert(0, panel);
    }

    // prepend the geographic map panel (built up front, above) so it leads the dashboard as a
    // geographic overview and survives the panel cap.
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
            PanelKind::Map { .. } | PanelKind::Geo { .. } | PanelKind::Scatter3D { .. }
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
            | PanelKind::Geo { .. } => None,
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
    // bar value-label font: in the unthemed look use qsv's ink color; when themed, omit the
    // color so the label inherits the template's font color (legible on dark backgrounds).
    let label_font = {
        let f = Font::new().size(9);
        if theme.is_some() {
            f
        } else {
            f.family(FONT_FAMILY).color(INK)
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
        // map / geo / 3D panels use a non-cartesian layout (mapbox, geo projection, or 3D scene)
        // that can't share the typed x/y subplot grid, so they are rendered entirely by
        // `smart_inline_panel_plot` and never reach this assembler.
        PanelKind::Map { .. } | PanelKind::Geo { .. } | PanelKind::Scatter3D { .. } => {
            unreachable!("map/geo/3D panels are rendered via the inline path, not panel_trace")
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
            | PanelKind::Geo { .. } => None,
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
    // a dashboard text font: only set family/ink color in the unthemed look, so themed
    // text inherits the template's font (legible on dark backgrounds).
    let ann_font = |size: usize| {
        let f = Font::new().size(size);
        if themed {
            f
        } else {
            f.family(FONT_FAMILY).color(INK)
        }
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

/// Render the dashboard as a single `Plot` with a typed subplot grid (≤ `MAX_SUBPLOTS` panels).
/// Static image export of more panels goes through `render_smart_grid_json` instead, because
/// plotly's typed `Layout` only exposes axis fields up to `x_axis8`/`y_axis8`.
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
    })
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

/// Assemble the dashboard as a self-contained HTML page: a responsive CSS grid of independent
/// plotly plots, one per panel. This sidesteps plotly's 8-axis typed-subplot limit so HTML
/// dashboards can show many more panels than the single-`Plot` grid. The plotly.js bundle is
/// embedded once in `<head>` (via `Plot::offline_js_sources`); each panel is emitted as an
/// inline `<div>` + `<script>` that draws into the shared global `Plotly`.
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
    let (page_bg, page_ink) = theme_page_chrome(theme);

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
            "      <div class=\"qsv-viz-plot\" style=\"height:{plot_height}px\">\n"
        ));
        cells.push_str(&plot.to_inline_html(Some(&div_id)));
        cells.push_str("\n      </div>\n");
        // reverse-geocoded spatial-extent summary caption, shown below a map panel.
        #[cfg(feature = "geocode")]
        if let Some(meta) = &panel.geo_meta
            && !meta.summary.is_empty()
        {
            cells.push_str(&format!(
                "      <div class=\"qsv-viz-geo-meta\">Spatial extent: {}</div>\n",
                html_escape(&meta.summary)
            ));
        }
        cells.push_str("    </div>\n");
    }

    let js = Plot::offline_js_sources();
    let title = html_escape(title_text);
    format!(
        "<!doctype html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\" />\n<meta \
         name=\"viewport\" content=\"width=device-width, initial-scale=1\" \
         />\n<title>{title}</title>\n{js}\n<style>\n  body {{ font-family: {FONT_FAMILY}; color: \
         {page_ink}; background: {page_bg}; margin: 0; padding: 16px; }}\n  h1.qsv-viz-title {{ \
         font-size: 20px; font-weight: 600; text-align: center; margin: 8px 0 20px; }}\n  \
         .qsv-viz-grid {{ display: grid; grid-template-columns: repeat({cols}, minmax(0, 1fr)); \
         gap: 16px; }}\n  .qsv-viz-cell {{ min-width: 0; }}\n  .qsv-viz-cell.full-width {{ \
         grid-column: 1 / -1; }}\n  .qsv-viz-plot {{ width: 100%; }}\n  .qsv-viz-geo-meta {{ \
         font-size: 13px; color: #4b5563; text-align: center; padding: 8px 4px 4px; \
         }}\n</style>\n</head>\n<body>\n<h1 class=\"qsv-viz-title\">{title}</h1>\n<div \
         class=\"qsv-viz-grid\">\n{cells}</div>\n</body>\n</html>\n"
    )
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
        | PanelKind::Geo { .. } => true,
        PanelKind::BoxStats { .. }
        | PanelKind::BoxRaw { .. }
        | PanelKind::BoxOutliers { .. }
        | PanelKind::FreqBar { .. }
        | PanelKind::Histogram { .. } => false,
    }
}

/// Inline-dashboard render height (px) for a panel: overview panels get the taller
/// `OVERVIEW_ROW_HEIGHT_PX`, everything else the standard `ROW_HEIGHT_PX`.
fn panel_render_height(kind: &PanelKind) -> usize {
    if is_overview_panel(kind) {
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
            .tick_font(Font::new().family(FONT_FAMILY).color(INK).size(10));
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
            .tick_font(Font::new().family(FONT_FAMILY).color(INK).size(10));
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
            title_font = title_font.family(FONT_FAMILY).color(INK);
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
        // a continuous column moarstats flagged as bimodal (BC >= 0.555) should become a
        // histogram (which shows the two peaks), not a box plot (which hides them).
        let mut s = stat("Float", 500, Some(0.9));
        s.q1 = Some(1.0);
        s.q2_median = Some(2.0);
        s.q3 = Some(3.0);
        s.min = Some("0.0".to_string());
        s.max = Some("4.0".to_string());
        s.bimodality_coefficient = Some(0.7); // >= 0.555 threshold
        assert!(matches!(
            classify(4, &s),
            Some(PanelKind::Histogram { idx: 4 })
        ));
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
}
