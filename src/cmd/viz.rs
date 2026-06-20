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
frequency caches: continuous numeric columns become box plots (drawn from precomputed
quartiles, so no data is re-scanned), and low-cardinality / boolean columns become
frequency bar charts. ID-like (near-unique) and all-empty columns are skipped. When the
dataset has two or more continuous numeric columns, a correlation heatmap panel is added
(this one panel does a single extra data pass to compute Pearson correlations), and if the
most strongly correlated pair is at least moderately correlated, a scatter of that pair is
added next to it. When the
dataset has a date/datetime column (auto-detected via stats date inference) plus a
continuous numeric column, a time-series line panel of that column over time is added too.
The first run computes & caches stats; subsequent runs are fast.

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
    -z, --z <col>          Value column for a heatmap pivot (with --x and --y).
    --cols <cols>          Columns to use. For heatmap: numeric columns for the
                           correlation matrix (default: all numeric). For radar:
                           the numeric axes to plot.
    --series <col>         Column to split into multiple series (one trace per
                           distinct value). Applies to bar/line/scatter/radar.
    --color <col>          For scatter/map: a numeric column to encode as marker color
                           (a continuous colorscale with a colorbar). For categorical
                           coloring, use the --series option instead. Cannot be
                           combined with --series. In density mode, this column is the
                           heatmap weight.
    --size <col>           For scatter/map: a numeric column to encode as marker size,
                           producing a bubble chart (values are rescaled to a readable
                           pixel range). Cannot be combined with --series. In density
                           mode, this column is the heatmap weight.
    --donut                Render a pie chart as a donut (with a center hole).
    --ohlc-open <col>      Open-price column for candlestick/ohlc charts.
    --high <col>           High-price column for candlestick/ohlc charts.
    --low <col>            Low-price column for candlestick/ohlc charts.
    --close <col>          Close-price column for candlestick/ohlc charts.
    --source <col>         Source node column for a sankey diagram.
    --target <col>         Target node column for a sankey diagram.
    --value <col>          Flow value column for a sankey diagram. When omitted,
                           each row counts as a flow of 1.
    --bins <n>             Number of bins for the histogram. (default: auto)
    --agg <fn>             For bar/line, aggregate the y values when the x value
                           repeats. One of: sum, mean, count, min, max.
    --box-points <mode>    Which sample points to draw alongside a box. Reading the
                           raw values lets plotly render true Tukey whiskers (1.5*IQR)
                           with the points beyond the fences as outliers. One of:
                           outliers (only the outliers), all (every point, jittered),
                           suspected (mark suspected outliers), none (no points, but
                           still real Tukey whiskers). For `viz box` the default is
                           outliers. For `viz smart` this flag is OPT-IN: without it,
                           box panels are drawn from the precomputed quartiles (no data
                           re-scan, observed min/max whiskers); passing it makes smart
                           do one extra batched pass to overlay the points.

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

geo options:
    --projection <name>    Map projection for `viz geo`. One of: natural-earth (the
                           default), mercator, orthographic, equirectangular,
                           albers-usa, robinson, winkel-tripel, mollweide, hammer,
                           azimuthal-equal-area. `viz geo` also reuses the lat, lon,
                           text, color, size and series options from `map`.
                           [default: natural-earth]

smart options:
    --max-charts <n>       Maximum number of panels in the dashboard. 0 (the default)
                           means auto: draw as many panels as the data warrants. For
                           HTML that's every eligible column (up to 64); for static
                           image export (png/svg/pdf/...) it's 8. Up to 8 panels render
                           as one subplot grid (plotly's typed subplot-axis limit);
                           HTML beyond 8 switches to an inline-div grid of independent
                           plots. Set a positive <n> to cap the panel count instead.
                           Eligible columns beyond the cap are reported but not drawn.
                           [default: 0]
    --grid-cols <n>        Number of columns in the dashboard grid. [default: 2]
    --limit <n>            Top-N categories per frequency bar chart. [default: 10]
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

    --title <s>            Chart title.
    --x-title <s>          X-axis title. (defaults to the x column name)
    --y-title <s>          Y-axis title. (defaults to the y column name)
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

use plotly::{
    Bar, BoxPlot, Candlestick, Configuration, Contour, DensityMapbox, HeatMap, Histogram, Ohlc,
    Pie, Plot, Sankey, Scatter, Scatter3D, ScatterGeo, ScatterMapbox, ScatterPolar, Trace,
    box_plot::{BoxPoints, QuartileMethod},
    color::NamedColor,
    common::{
        Anchor, ColorBar, ColorScale, ColorScalePalette, Fill, Font, Line, Marker, Mode,
        TextPosition, Title,
    },
    layout::{
        Annotation, Axis, AxisType, Center, Layout, LayoutGeo, LayoutScene, Mapbox, MapboxStyle,
        Margin, Projection, ProjectionType,
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
/// heuristic: at or below ALL, every sample point is overlaid on a box; at or below OUTLIERS, only
/// the Tukey outliers; above that, no points are drawn and the box stays a cache-only quartile
/// summary (overlaying tens of thousands of points is an unreadable smear, and skipping them avoids
/// the extra data pass).
const SMART_BOX_ALL_MAX: u64 = 1_000;
const SMART_BOX_OUTLIERS_MAX: u64 = 10_000;

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

/// Horizontal space (in pixels) per dashboard grid column, used to auto-size the `viz
/// smart` static image export width.
const SMART_COL_WIDTH_PX: usize = 500;

/// Fallback static-export image dimensions (pixels) when --width/--height are not given
/// and no auto-scaling applies (i.e. the non-`smart` chart types).
const DEFAULT_IMG_WIDTH: usize = 1000;
const DEFAULT_IMG_HEIGHT: usize = 600;

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

/// `viz smart` correlation-heatmap panel tuning. Long numeric-column names would clip against
/// the dashboard's left margin, so its axis tick labels are truncated to this many characters
/// and the left margin is widened (≈ this many pixels per character) to fit them. In-cell `r`
/// value labels are only drawn when the matrix is small enough to stay legible in one cell.
const CORR_LABEL_MAX_CHARS: usize = 16;
const CORR_LABEL_PX_PER_CHAR: usize = 7;
const CORR_INCELL_MAX_N: usize = 8;

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
    flag_smarter:      bool,
    flag_title:        Option<String>,
    flag_x_title:      Option<String>,
    flag_y_title:      Option<String>,
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
    let args: Args = util::get_args(USAGE, argv)?;

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
            SmartRender::Grid { plot, dims } => (plot, Some(dims)),
        }
    } else {
        (Box::new(build_plot(&args)?), None)
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

/// The two ways `viz smart` can render: a single-`Plot` typed subplot grid (up to
/// `MAX_SUBPLOTS` panels; the only form that supports static image export), or a
/// self-contained inline-div HTML page (for >8-panel HTML dashboards).
enum SmartRender {
    // `Plot` is large; box it so the enum isn't bloated by the rarely-larger variant.
    Grid {
        plot: Box<Plot>,
        dims: (usize, usize),
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
fn build_plot(args: &Args) -> CliResult<Plot> {
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
        return build_map_plot(args);
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

/// Compute a map center and a zoom level that frames the data, so the basemap doesn't default to
/// plotly's whole-world view centered at (0, 0). Longitude is handled with antimeridian wrap so a
/// cluster straddling the 180° line (e.g. 179 and -179) frames as the small arc across the date
/// line rather than spanning almost the whole globe and centering near 0.
///
/// `trim_frac` is the fraction trimmed off each end of the lat/lon distributions before framing,
/// so a few outlier coordinates can't dominate the center/zoom. `viz smart`'s auto panel passes
/// `MAP_FRAME_TRIM_FRAC`; the standalone `viz map` command passes `0.0` to frame the full extent
/// of every valid coordinate (its edge points are intentional, not noise).
fn map_center_zoom(lats: &[f64], lons: &[f64], trim_frac: f64) -> (Center, u8) {
    let mut sorted_lats = lats.to_vec();
    sorted_lats.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let min_lat = sorted_quantile(&sorted_lats, trim_frac);
    let max_lat = sorted_quantile(&sorted_lats, 1.0 - trim_frac);
    let lat_center = (min_lat + max_lat) / 2.0;
    let lat_span = max_lat - min_lat;

    let (lon_center, lon_span) = lon_center_and_span(lons, trim_frac);
    let center = Center::new(lat_center, lon_center);

    // the larger of the two degree-spans drives the zoom; halving the visible span ≈ +1 zoom.
    // single-point (zero span) datasets get a sensible street-level zoom.
    let span = lat_span.max(lon_span);
    let zoom = if span <= 0.0 {
        10
    } else {
        ((360.0 / span).log2().floor() as i32 - 1).clamp(1, 16) as u8
    };
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
        // data crosses the antimeridian: the cluster runs from sorted[gap_idx + 1] eastward,
        // wrapping past 180, to sorted[gap_idx] (+360 to keep the arc contiguous)
        let span = 360.0 - max_gap;
        let mut center = (sorted[gap_idx + 1] + sorted[gap_idx] + 360.0) / 2.0;
        if center > 180.0 {
            center -= 360.0;
        }
        (center, span)
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
fn build_map_plot(args: &Args) -> CliResult<Plot> {
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

    // standalone `viz map`: frame the full extent — its edge coordinates are intentional
    let (center, zoom) = map_center_zoom(&lats, &lons, 0.0);

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
    plot.set_layout(layout);
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
    plot.set_layout(layout);
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
    plot.set_layout(layout);
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
    layout
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
    use plotly::ImageFormat;
    let path = args
        .flag_output
        .as_deref()
        .expect("image format without --output should have been rejected");
    let image_format = match fmt {
        OutFormat::Png => ImageFormat::PNG,
        OutFormat::Jpeg => ImageFormat::JPEG,
        OutFormat::Webp => ImageFormat::WEBP,
        OutFormat::Svg => ImageFormat::SVG,
        OutFormat::Pdf => ImageFormat::PDF,
        OutFormat::Html => unreachable!(),
    };
    plot.write_image(path, image_format, width, height, scale)
        .map_err(|e| {
            crate::CliError::Other(format!(
                "Static image export failed: {e}. A Chromium or Firefox browser must be installed \
                 and available for plotly's webdriver-based export."
            ))
        })?;
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
    name: String,
    kind: PanelKind,
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
    Map {
        lats:    Vec<f64>,
        lons:    Vec<f64>,
        density: bool,
    },
    /// Geographic point map drawn on a `ScatterGeo` projection basemap (coastlines/land/countries,
    /// no network tiles) instead of mapbox — used for `viz smart` when the coordinates span a
    /// continental/global extent. Like `Map`, the `geo` subplot doesn't compose with the typed x/y
    /// grid, so a dashboard containing this panel always renders via the inline path.
    Geo { lats: Vec<f64>, lons: Vec<f64> },
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
        let Some(y) = parse_f64(record.get(y_idx)) else {
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
    // points are chronologically sorted, so stride downsampling preserves the trend's shape
    let xs_full: Vec<String> = points.iter().map(|p| p.1.clone()).collect();
    let ys_full: Vec<f64> = points.iter().map(|p| p.2).collect();
    let (xs, ys) = downsample_pair(&xs_full, &ys_full, MAX_SMART_POINTS);
    Ok(Some(Panel {
        name: format!("{y_label} over {date_label}"),
        kind: PanelKind::TimeSeries { y_label, xs, ys },
    }))
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
    // Compute the spans before downsampling, on the full in-range coordinate set.
    let span = |v: &[f64]| {
        v.iter().copied().fold(f64::NEG_INFINITY, f64::max)
            - v.iter().copied().fold(f64::INFINITY, f64::min)
    };
    let global =
        span(&lons) >= SMART_GEO_MIN_LON_SPAN_DEG || span(&lats) >= SMART_GEO_MIN_LAT_SPAN_DEG;

    let (lats, lons) = downsample_pair(&lats, &lons, MAX_SMART_POINTS);
    let kind = if global {
        PanelKind::Geo { lats, lons }
    } else {
        PanelKind::Map {
            lats,
            lons,
            density,
        }
    };
    Ok(Some((
        Panel {
            name: "Map".to_string(),
            kind,
        },
        (lat_idx, lon_idx),
    )))
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
    // The map panel is mapbox-based and HTML-only, so build it only for HTML output. Excluding the
    // coordinate columns is tied to a map that will actually render:
    //   - image export (PNG/SVG/PDF/...): the map can't be embedded, so skip the build entirely —
    //     map_cols stays None and the coordinates are charted as normal distributions (rather than
    //     being excluded AND having their only panel dropped, which would hide them from the
    //     image);
    //   - HTML with named+numeric lat/lon but no in-range value: build_map_panel returns None, so
    //     map_cols is None and those columns are charted normally.
    let map_panel = if out_format.is_image() {
        if latlon_indices(&stats).is_some() {
            eprintln!(
                "viz smart: map panels are HTML-only (mapbox can't be exported in the subplot \
                 grid); the map was skipped for image export."
            );
        }
        None
    } else {
        build_map_panel(args, &stats)?
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
                // explicit flag or the size heuristic calls for points; large datasets keep the
                // cheap cache-only box (the heuristic returns None) to avoid the extra pass.
                if matches!(kind, PanelKind::BoxStats { .. }) {
                    let n =
                        *nrows.get_or_insert_with(|| util::count_rows(&count_conf).unwrap_or(0));
                    if let Some(points) = smart_box_points(explicit_box_points.as_ref(), n) {
                        kind = PanelKind::BoxRaw { idx, points };
                    }
                }
                // for box panels, append moarstats shape hints (skew direction, outlier share)
                // to the panel title when those extended stats are present. Cache-only, no cost;
                // without moarstats the hint is None and the title is unchanged.
                let name = match &kind {
                    PanelKind::BoxStats { .. } | PanelKind::BoxRaw { .. } => {
                        match box_shape_hint(s) {
                            Some(hint) => format!("{name} {hint}"),
                            None => name,
                        }
                    },
                    _ => name,
                };
                panels.push(Panel { name, kind });
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
                    Panel {
                        name,
                        kind: PanelKind::ContourPair { x, y, z },
                    }
                } else {
                    let (xs, ys) = downsample_pair(&columns[i], &columns[j], MAX_SMART_POINTS);
                    Panel {
                        name,
                        kind: PanelKind::ScatterPair { xs, ys },
                    }
                }
            });

            // 3D drill-down: with 3+ numeric columns, a Scatter3D of the strongest-correlation
            // triple — the strongest pair plus the third column most correlated with that pair.
            // A 3D scene forces the inline (HTML) render path, like the map panel.
            let scatter3d_panel = pair.filter(|_| columns.len() >= 3).and_then(|(i, j, _)| {
                let third = (0..columns.len())
                    .filter(|&k| k != i && k != j)
                    .max_by(|&a, &b| {
                        let sa = matrix[i][a].abs() + matrix[j][a].abs();
                        let sb = matrix[i][b].abs() + matrix[j][b].abs();
                        sa.partial_cmp(&sb).unwrap_or(std::cmp::Ordering::Equal)
                    });
                third.map(|k| {
                    // both downsamples share (n, cap) so they pick the same row indices -> aligned
                    let (xs, ys) = downsample_pair(&columns[i], &columns[j], MAX_SMART_POINTS);
                    let (_, zs) = downsample_pair(&columns[i], &columns[k], MAX_SMART_POINTS);
                    Panel {
                        name: format!("{} / {} / {} (3D)", labels[i], labels[j], labels[k]),
                        kind: PanelKind::Scatter3D {
                            xs,
                            ys,
                            zs,
                            labels: (labels[i].clone(), labels[j].clone(), labels[k].clone()),
                        },
                    }
                })
            });

            panels.insert(
                0,
                Panel {
                    name: "Correlation".to_string(),
                    kind: PanelKind::CorrHeatmap { labels, matrix },
                },
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
    let requested = if args.flag_max_charts == 0 {
        if is_html {
            MAX_PANELS_INLINE
        } else {
            MAX_SUBPLOTS
        }
    } else {
        args.flag_max_charts
    };
    let want = requested.min(eligible);
    let inline = is_html && (want > MAX_SUBPLOTS || has_noncartesian);

    let max_panels = if inline {
        requested.min(MAX_PANELS_INLINE)
    } else {
        requested.min(MAX_SUBPLOTS)
    };

    // for image export, flag when the 8-panel limit (not an explicit smaller --max-charts) is
    // what's dropping panels, pointing users to HTML for the full picture.
    if out_format.is_image() && eligible > MAX_SUBPLOTS && max_panels == MAX_SUBPLOTS {
        eprintln!(
            "viz smart: static image export is limited to {MAX_SUBPLOTS} panels; output an .html \
             file to render up to {MAX_PANELS_INLINE} panels."
        );
    }

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
    let raw_values = if raw_indices.is_empty() {
        HashMap::new()
    } else {
        collect_numeric_values(args, &raw_indices)?
    };

    // dashboard title: the user's --title, else the dataset's file name
    let title_text = args.flag_title.clone().unwrap_or_else(|| {
        let dataset = std::path::Path::new(args.arg_input.as_deref().unwrap_or("data"))
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("data");
        format!("{dataset} \u{2014} data overview")
    });

    if inline {
        Ok(SmartRender::Inline(render_smart_inline(
            args,
            &panels,
            &freq,
            &raw_values,
            &title_text,
        )))
    } else {
        render_smart_grid(args, &panels, &freq, &raw_values, &title_text)
    }
}

/// Build the plotly trace for one smart-dashboard panel. `axes` carries the subplot axis refs
/// when rendering into the typed grid; pass `None` for a standalone inline-div plot (which uses
/// the default x/y axes). Returns the trace plus, for bar panels, the tallest bar value (used
/// to add vertical headroom for the outside value labels).
fn panel_trace(
    panel: &Panel,
    color: &'static str,
    freq: &HashMap<usize, Vec<(String, u64)>>,
    hist: &HashMap<usize, Vec<f64>>,
    axes: Option<(String, String)>,
) -> (Box<dyn Trace>, Option<f64>) {
    let mut bar_max: Option<f64> = None;
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
                .marker(Marker::new().color(color));
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
                .marker(Marker::new().color(color));
            if let Some((x, y)) = &axes {
                b = b.x_axis(x.clone()).y_axis(y.clone());
            }
            b
        },
        PanelKind::FreqBar { idx } => {
            let counts = freq.get(idx).cloned().unwrap_or_default();
            let xs: Vec<String> = counts.iter().map(|(v, _)| v.clone()).collect();
            let ys: Vec<f64> = counts.iter().map(|(_, c)| *c as f64).collect();
            bar_max = Some(ys.iter().copied().fold(0.0_f64, f64::max));
            let mut bar = Bar::new(xs, ys)
                .name(panel.name.clone())
                .marker(Marker::new().color(color))
                // value labels above each bar, SI-formatted ("258k", "1.05M") to match
                // the axis ticks
                .text_template("%{y:.3s}")
                .text_position(TextPosition::Outside)
                .text_font(Font::new().family(FONT_FAMILY).color(INK).size(9));
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
    (trace, bar_max)
}

/// Render the dashboard as a single `Plot` with a typed subplot grid (≤ `MAX_SUBPLOTS` panels).
/// This is the only form that supports static image export. We lay out each cell's axes with
/// explicit paper-domains (rather than a plotly `grid`) so we can (a) scale the plot height with
/// the row count, (b) reserve a band at the top for the dashboard title, and (c) place each
/// column's name as a title *above* its panel.
fn render_smart_grid(
    args: &Args,
    panels: &[Panel],
    freq: &HashMap<usize, Vec<(String, u64)>>,
    hist: &HashMap<usize, Vec<f64>>,
    title_text: &str,
) -> CliResult<SmartRender> {
    let cols = args.flag_grid_cols.clamp(1, panels.len());
    let rows = panels.len().div_ceil(cols);
    let grid_top = smart_grid_top(rows);
    let title_offset = smart_title_offset(rows);

    // widen the left margin when a correlation panel is present so its (long) numeric-column
    // tick labels — truncated to CORR_LABEL_MAX_CHARS — aren't clipped against the page edge.
    let left_margin = panels
        .iter()
        .find_map(|p| match &p.kind {
            PanelKind::CorrHeatmap { labels, .. } => Some(labels),
            PanelKind::BoxStats { .. }
            | PanelKind::BoxRaw { .. }
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

    let mut plot = Plot::new();
    let mut layout = Layout::new()
        .show_legend(false)
        .height(rows * ROW_HEIGHT_PX)
        .margin(
            Margin::new()
                .top(TOP_MARGIN_PX)
                .bottom(BOTTOM_MARGIN_PX)
                .left(left_margin)
                .right(40)
                .pad(4),
        )
        .font(Font::new().family(FONT_FAMILY).color(INK).size(12))
        .paper_background_color(PAPER_BG)
        .plot_background_color(PAPER_BG);

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
            .font(Font::new().family(FONT_FAMILY).color(INK).size(20)),
    );

    for (n, panel) in panels.iter().enumerate() {
        let pos = n + 1;
        let xref = axis_ref('x', pos);
        let yref = axis_ref('y', pos);
        let color = PALETTE[n % PALETTE.len()];
        let is_box = matches!(
            panel.kind,
            PanelKind::BoxStats { .. } | PanelKind::BoxRaw { .. }
        );
        let is_date = matches!(panel.kind, PanelKind::TimeSeries { .. });
        let (trace, bar_max) =
            panel_trace(panel, color, freq, hist, Some((xref.clone(), yref.clone())));
        plot.add_trace(trace);

        // position this subplot's styled axes and add its title above the cell
        let geom = subplot_geometry(n, rows, cols, grid_top, title_offset);
        layout = place_subplot_axes(
            layout,
            pos,
            styled_x_axis(is_box, is_date),
            styled_y_axis(bar_max),
            geom.x_domain,
            geom.y_domain,
            &xref,
            &yref,
        );
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
                .font(Font::new().family(FONT_FAMILY).color(INK).size(13)),
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
    layout = layout.annotations(annotations);

    plot.set_layout(layout);
    Ok(SmartRender::Grid {
        plot: Box::new(plot),
        dims: (cols * SMART_COL_WIDTH_PX, rows * ROW_HEIGHT_PX),
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
    freq: &HashMap<usize, Vec<(String, u64)>>,
    hist: &HashMap<usize, Vec<f64>>,
) -> Plot {
    // map panels use a mapbox layout (tile basemap, framed to the points) instead of cartesian
    // x/y axes, so they're assembled here rather than through the shared `panel_trace`/axis path.
    if let PanelKind::Map {
        lats,
        lons,
        density,
    } = &panel.kind
    {
        // smart auto panel: trim outliers so a few bad geocodes don't blow up the default view
        let (center, zoom) = map_center_zoom(lats, lons, MAP_FRAME_TRIM_FRAC);
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
        let layout = Layout::new()
            .show_legend(false)
            .height(ROW_HEIGHT_PX)
            .title(Title::with_text(panel.name.clone()))
            .margin(Margin::new().top(48).bottom(20).left(20).right(20).pad(4))
            .font(Font::new().family(FONT_FAMILY).color(INK).size(12))
            .paper_background_color(PAPER_BG)
            .mapbox(
                Mapbox::new()
                    .style(MapboxStyle::OpenStreetMap)
                    .center(center)
                    .zoom(zoom),
            );
        plot.set_layout(layout);
        plot.set_configuration(Configuration::new().responsive(true));
        return plot;
    }

    // geo panels use a `geo` projection layout (no tiles, fully offline) instead of cartesian
    // x/y axes, so they're assembled here like the mapbox map panel above.
    if let PanelKind::Geo { lats, lons } = &panel.kind {
        let mut plot = Plot::new();
        plot.add_trace(
            ScatterGeo::new(lats.clone(), lons.clone())
                .mode(Mode::Markers)
                .marker(Marker::new().color(color).opacity(MAP_POINT_OPACITY)),
        );
        let geo = LayoutGeo::new()
            .projection(Projection::new().projection_type(ProjectionType::NaturalEarth))
            .showland(true)
            .landcolor(NamedColor::LightGray)
            .showocean(true)
            .oceancolor(NamedColor::LightBlue)
            .showlakes(true)
            .lakecolor(NamedColor::LightBlue)
            .showcountries(true);
        let layout = Layout::new()
            .show_legend(false)
            .height(ROW_HEIGHT_PX)
            .title(Title::with_text(panel.name.clone()))
            .margin(Margin::new().top(48).bottom(20).left(20).right(20).pad(4))
            .font(Font::new().family(FONT_FAMILY).color(INK).size(12))
            .paper_background_color(PAPER_BG)
            .geo(geo);
        plot.set_layout(layout);
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
        let layout = Layout::new()
            .show_legend(false)
            .height(ROW_HEIGHT_PX)
            .title(Title::with_text(panel.name.clone()))
            .margin(Margin::new().top(48).bottom(20).left(20).right(20).pad(4))
            .font(Font::new().family(FONT_FAMILY).color(INK).size(12))
            .paper_background_color(PAPER_BG)
            .scene(scene);
        plot.set_layout(layout);
        plot.set_configuration(Configuration::new().responsive(true));
        return plot;
    }

    let is_box = matches!(
        panel.kind,
        PanelKind::BoxStats { .. } | PanelKind::BoxRaw { .. }
    );
    let is_corr = matches!(panel.kind, PanelKind::CorrHeatmap { .. });
    let is_date = matches!(panel.kind, PanelKind::TimeSeries { .. });
    let (trace, bar_max) = panel_trace(panel, color, freq, hist, None);

    let mut plot = Plot::new();
    plot.add_trace(trace);

    // correlation cells need extra left room for tick labels and right room for the colorbar
    let (left, right) = if is_corr { (110, 90) } else { (60, 30) };
    let mut layout = Layout::new()
        .show_legend(false)
        .height(ROW_HEIGHT_PX)
        .title(Title::with_text(panel.name.clone()))
        .margin(
            Margin::new()
                .top(48)
                .bottom(60)
                .left(left)
                .right(right)
                .pad(4),
        )
        .font(Font::new().family(FONT_FAMILY).color(INK).size(12))
        .paper_background_color(PAPER_BG)
        .plot_background_color(PAPER_BG)
        .x_axis(styled_x_axis(is_box, is_date))
        .y_axis(styled_y_axis(bar_max));

    if let PanelKind::CorrHeatmap { matrix, .. } = &panel.kind
        && matrix.len() <= CORR_INCELL_MAX_N
    {
        layout = layout.annotations(corr_incell_annotations(matrix, "x", "y"));
    }

    plot.set_layout(layout);
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
    freq: &HashMap<usize, Vec<(String, u64)>>,
    hist: &HashMap<usize, Vec<f64>>,
    title_text: &str,
) -> String {
    let cols = args.flag_grid_cols.clamp(1, panels.len().max(1));

    let mut cells = String::new();
    for (n, panel) in panels.iter().enumerate() {
        let color = PALETTE[n % PALETTE.len()];
        let plot = smart_inline_panel_plot(panel, color, freq, hist);
        let div_id = format!("qsv-viz-panel-{n}");
        cells.push_str("    <div class=\"qsv-viz-cell\">\n");
        cells.push_str(&plot.to_inline_html(Some(&div_id)));
        cells.push_str("\n    </div>\n");
    }

    let js = Plot::offline_js_sources();
    let title = html_escape(title_text);
    format!(
        "<!doctype html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\" />\n<meta \
         name=\"viewport\" content=\"width=device-width, initial-scale=1\" \
         />\n<title>{title}</title>\n{js}\n<style>\n  body {{ font-family: {FONT_FAMILY}; color: \
         {INK}; background: {PAPER_BG}; margin: 0; padding: 16px; }}\n  h1.qsv-viz-title {{ \
         font-size: 20px; font-weight: 600; text-align: center; margin: 8px 0 20px; }}\n  \
         .qsv-viz-grid {{ display: grid; grid-template-columns: repeat({cols}, minmax(0, 1fr)); \
         gap: 16px; }}\n  .qsv-viz-cell {{ min-width: 0; }}\n</style>\n</head>\n<body>\n<h1 \
         class=\"qsv-viz-title\">{title}</h1>\n<div \
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
/// On a hit, each column's pairs are re-sorted (count desc, then value asc) and
/// truncated to `top_n` so the result is identical to `count_values`. The cache
/// already had its null/empty bucket filtered out by `read_frequency_cache_view`,
/// matching `count_values`, which skips empty cells.
fn freq_from_cache(
    args: &Args,
    stats: &[crate::cmd::stats::StatsData],
    bar_indices: &[usize],
    top_n: usize,
) -> Option<HashMap<usize, Vec<(String, u64)>>> {
    let path = args.arg_input.as_ref()?;
    let rconfig = Config::new(Some(path))
        .delimiter(args.flag_delimiter)
        .no_headers_flag(args.flag_no_headers);
    let no_headers = rconfig.no_headers;

    // `viz` never sets --no-nulls, so it can only reuse a default (nulls-kept)
    // cache; the null bucket is then filtered to match count_values.
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
        let mut pairs = view.columns.get(&key)?.clone();
        pairs.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        pairs.truncate(top_n);
        out.insert(idx, pairs);
    }
    Some(out)
}

/// Count value occurrences for the given column indices in a single pass, returning the
/// top-N (value, count) pairs per column (sorted by count desc, then value asc).
fn count_values(
    args: &Args,
    indices: &[usize],
    top_n: usize,
) -> CliResult<HashMap<usize, Vec<(String, u64)>>> {
    let rconfig = Config::new(args.arg_input.as_ref())
        .delimiter(args.flag_delimiter)
        .no_headers_flag(args.flag_no_headers);
    let mut rdr = rconfig.reader()?;

    let mut maps: HashMap<usize, HashMap<Vec<u8>, u64>> =
        indices.iter().map(|&i| (i, HashMap::new())).collect();

    let mut record = csv::ByteRecord::new();
    while rdr.read_byte_record(&mut record)? {
        for &i in indices {
            if let Some(cell) = record.get(i) {
                if cell.is_empty() {
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
        counts.truncate(top_n);
        out.insert(i, counts);
    }
    Ok(out)
}

/// Collect raw numeric values for the given column indices in a single pass, each downsampled to
/// at most `MAX_SMART_POINTS` via uniform stride (preserving the distribution shape). Feeds
/// `viz smart`'s histogram panels — the only smart panel that needs raw data. Empty and
/// non-numeric cells are skipped.
fn collect_numeric_values(args: &Args, indices: &[usize]) -> CliResult<HashMap<usize, Vec<f64>>> {
    let rconfig = Config::new(args.arg_input.as_ref())
        .delimiter(args.flag_delimiter)
        .no_headers_flag(args.flag_no_headers);
    let mut rdr = rconfig.reader()?;

    let mut maps: HashMap<usize, Vec<f64>> = indices.iter().map(|&i| (i, Vec::new())).collect();

    let mut record = csv::ByteRecord::new();
    while rdr.read_byte_record(&mut record)? {
        for &i in indices {
            if let Some(v) = parse_f64(record.get(i))
                && let Some(col) = maps.get_mut(&i)
            {
                col.push(v);
            }
        }
    }

    // downsample each column so the embedded HTML stays small while keeping the distribution shape
    for col in maps.values_mut() {
        if col.len() > MAX_SMART_POINTS {
            let (sampled, _) = downsample_pair(col, col, MAX_SMART_POINTS);
            *col = sampled;
        }
    }
    Ok(maps)
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
struct SubplotGeometry {
    x_domain: Vec<f64>,
    y_domain: Vec<f64>,
    title_x:  f64,
    title_y:  f64,
}

/// Compute the paper-space domains for subplot `n` (0-based) in a `rows`×`cols` grid that
/// occupies the vertical band `0.0..=top` (the strip above `top` is left for the dashboard
/// title), plus the (x, y) anchor for the panel's own title, placed `title_offset` (a paper
/// fraction) above the cell. Cells are laid out left-to-right, top-to-bottom with fixed
/// gaps; the vertical gap leaves room for each panel's title and the row below it's tick
/// labels.
fn subplot_geometry(
    n: usize,
    rows: usize,
    cols: usize,
    top: f64,
    title_offset: f64,
) -> SubplotGeometry {
    const HGAP: f64 = 0.08;
    const VGAP: f64 = 0.09;

    let cols_f = cols as f64;
    let rows_f = rows as f64;
    let cell_w = (1.0 - HGAP * (cols_f - 1.0)) / cols_f;
    let cell_h = (top - VGAP * (rows_f - 1.0)) / rows_f;

    let col = (n % cols) as f64;
    let row = (n / cols) as f64; // row 0 is the top of the band

    let x0 = col * (cell_w + HGAP);
    let x1 = x0 + cell_w;
    let y1 = top - row * (cell_h + VGAP); // top edge of the cell
    let y0 = y1 - cell_h;

    SubplotGeometry {
        x_domain: vec![x0, x1],
        y_domain: vec![y0, y1],
        title_x:  (x0 + x1) / 2.0,
        title_y:  (y1 + title_offset).min(1.0),
    }
}

/// A styled x-axis for a dashboard panel: no vertical gridlines, a light baseline, and
/// small tick labels. For single-box panels (`is_box`), the lone "0" category tick is
/// meaningless, so its labels and baseline are hidden. For time-series panels (`is_date`),
/// the axis is typed as a date axis so plotly spaces ticks chronologically.
fn styled_x_axis(is_box: bool, is_date: bool) -> Axis {
    let mut a = Axis::new()
        .show_grid(false)
        .zero_line(false)
        .show_line(true)
        .line_color(AXIS_LINE)
        .tick_color(AXIS_LINE)
        .tick_font(Font::new().family(FONT_FAMILY).color(INK).size(10));
    if is_box {
        a = a.show_tick_labels(false).show_line(false);
    }
    if is_date {
        a = a.type_(AxisType::Date);
    }
    a
}

/// A styled y-axis for a dashboard panel: light horizontal gridlines only, small ticks.
/// When `headroom_max` is given (bar panels), the range is fixed to `0..=max*1.15` so the
/// tallest bar's outside value label has room and isn't clipped at the cell top.
fn styled_y_axis(headroom_max: Option<f64>) -> Axis {
    let mut a = Axis::new()
        .show_grid(true)
        .grid_color(GRID_COLOR)
        .grid_width(1)
        .zero_line(false)
        .show_line(false)
        .tick_color(AXIS_LINE)
        .tick_font(Font::new().family(FONT_FAMILY).color(INK).size(10));
    if let Some(m) = headroom_max
        && m > 0.0
    {
        a = a.range(vec![0.0, m * 1.15]);
    }
    a
}

/// Place subplot `pos`'s prebuilt axis pair: stamp on the cell's domains + cross anchors
/// and assign them to the typed Layout axis fields (which only exist up to 8, matching
/// MAX_SUBPLOTS).
fn place_subplot_axes(
    layout: Layout,
    pos: usize,
    x_axis: Axis,
    y_axis: Axis,
    x_domain: Vec<f64>,
    y_domain: Vec<f64>,
    xref: &str,
    yref: &str,
) -> Layout {
    // x-axis anchors to its paired y-axis and vice versa.
    let x = x_axis.domain(&x_domain).anchor(yref);
    let y = y_axis.domain(&y_domain).anchor(xref);
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
    fn subplot_geometry_cells_are_within_bounds_and_titled_above() {
        // a 4-panel, 2-column dashboard -> 2 rows, occupying the full band (top = 1.0)
        let (rows, cols, top, offset) = (2, 2, 1.0, 0.01);
        for n in 0..4 {
            let g = subplot_geometry(n, rows, cols, top, offset);
            // domains stay inside the paper area
            assert!(g.x_domain[0] >= 0.0 && g.x_domain[1] <= 1.0 + 1e-9);
            assert!(g.y_domain[0] >= -1e-9 && g.y_domain[1] <= 1.0 + 1e-9);
            // each title is horizontally centered on its cell and sits at/above the cell top
            assert!((g.title_x - (g.x_domain[0] + g.x_domain[1]) / 2.0).abs() < 1e-9);
            assert!(g.title_y >= g.y_domain[1] - 1e-9);
        }
        // top row is higher on the page than the bottom row
        let upper = subplot_geometry(0, rows, cols, top, offset);
        let lower = subplot_geometry(2, rows, cols, top, offset);
        assert!(upper.y_domain[0] > lower.y_domain[1]);
        // the two columns don't overlap horizontally
        let left = subplot_geometry(0, rows, cols, top, offset);
        let right = subplot_geometry(1, rows, cols, top, offset);
        assert!(left.x_domain[1] <= right.x_domain[0] + 1e-9);
    }

    #[test]
    fn smart_title_band_fits_every_row_count() {
        // For every dashboard size up to the 8-panel max (including a tall single-column
        // 8-row layout), the top-row panel title — offset above the cell plus its rendered
        // glyph height — must stay at/below y=1 so it never overlaps the dashboard title.
        const GLYPH_PX: f64 = 20.0; // generous estimate of the 13px title's rendered height
        for rows in 1..=MAX_SUBPLOTS {
            let area = smart_plot_area_h(rows);
            let g = subplot_geometry(0, rows, 1, smart_grid_top(rows), smart_title_offset(rows));
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
        let (center, zoom) = map_center_zoom(&lats, &lons, 0.0);
        let v = serde_json::to_value(&center).unwrap();
        assert!((v["lat"].as_f64().unwrap() - 40.0).abs() < 1e-9);
        assert!((v["lon"].as_f64().unwrap() - (-75.0)).abs() < 1e-9);
        // small span -> high zoom; large span -> low zoom. Use a genuinely wide (non-wrapping)
        // longitude range so this exercises the plain bounding-box path, not antimeridian wrap.
        let (_, world_zoom) = map_center_zoom(&[-60.0, 70.0], &[-80.0, 80.0], 0.0);
        assert!(
            zoom > world_zoom,
            "tight cluster should zoom in more than world"
        );
    }

    #[test]
    fn map_center_zoom_single_point() {
        // a zero-span (single point) dataset gets a sensible non-extreme zoom
        let (_, zoom) = map_center_zoom(&[51.5], &[-0.12], 0.0);
        assert!((1..=16).contains(&zoom));
    }

    #[test]
    fn map_center_zoom_handles_antimeridian() {
        // a tight cluster straddling the 180° line (179 and -179) is ~2° wide, not ~358°, so it
        // centers near the date line and zooms in rather than framing the whole globe at lon 0.
        let lats = [18.0, 16.0];
        let lons = [179.0, -179.0];
        let (center, zoom) = map_center_zoom(&lats, &lons, 0.0);
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
        let (center, _zoom) = map_center_zoom(&lats, &lons, MAP_FRAME_TRIM_FRAC);
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
        let (center, _zoom) = map_center_zoom(&lats, &lons, 0.0);
        let lon = serde_json::to_value(&center).unwrap()["lon"]
            .as_f64()
            .unwrap();
        assert!(
            lon < -75.0,
            "full-extent framing must include the western outliers, got lon {lon}"
        );
    }
}
