static USAGE: &str = r#"
Generate charts from CSV data using the plotly charting library.

Produces a self-contained, interactive HTML chart (the plotly.js runtime is embedded,
so the output works offline). With a qsv build that includes the `viz_static` feature,
charts can also be exported as static PNG/SVG/PDF/JPEG/WebP images (this requires a
Chromium/Firefox browser at runtime - a webdriver is auto-managed by plotly).

The output format is inferred from the --output file extension (.html is the default).
Interactive HTML is written to stdout when --output is not given; image formats always
require --output. Use --open to view the result in your default browser/viewer.

Chart types (subcommands):
    smart       Auto-dashboard. Picks an appropriate chart per column from the
                dataset's statistics & frequency distribution (no --x/--y needed).
    bar         Bar chart.        --x = category column, --y = value column.
    line        Line chart.       --x = x column, --y = y column.
    scatter     Scatter plot.     --x = x column, --y = y column.
    histogram   Distribution.     --x = numeric column to bin.
    box         Box plot.         --y = value column, optional --x = group column.
    pie         Proportions.      --x = label column, optional --y = value column.
    heatmap     Color grid. Correlation matrix of numeric columns (default; an
                optional column subset via --cols), or a category x category pivot
                with --x/--y/--z.
    candlestick Financial OHLC.   --x = date column, plus --ohlc-open/--high/--low/--close.
    ohlc        Financial OHLC bars (same inputs as candlestick).
    sankey      Flow diagram.     --source, --target, optional --value column.
    radar       Polar/radar chart of numeric --cols, optional --series per trace.

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

For more examples, see https://github.com/dathere/qsv/blob/master/tests/test_viz.rs.

Usage:
    qsv viz smart       [options] <input>
    qsv viz bar         [options] <input>
    qsv viz line        [options] <input>
    qsv viz scatter     [options] <input>
    qsv viz histogram   [options] <input>
    qsv viz box         [options] <input>
    qsv viz pie         [options] <input>
    qsv viz heatmap     [options] <input>
    qsv viz candlestick [options] <input>
    qsv viz ohlc        [options] <input>
    qsv viz sankey      [options] <input>
    qsv viz radar       [options] <input>
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
    --color <col>          For scatter: a numeric column to encode as marker color
                           (a continuous colorscale with a colorbar). For categorical
                           coloring, use the --series option instead. Cannot be
                           combined with --series.
    --size <col>           For scatter: a numeric column to encode as marker size,
                           producing a bubble chart (values are rescaled to a readable
                           pixel range). Cannot be combined with --series.
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
    --box-points <mode>    For box plots, which sample points to draw alongside the
                           box. Explicit `viz box` reads the raw values, so plotly
                           renders true Tukey whiskers (1.5*IQR) and the points beyond
                           the fences are the outliers. One of: outliers (only the
                           outliers, the default), all (every point, jittered),
                           suspected (mark suspected outliers), none (no points).
                           [default: outliers]

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
    Bar, BoxPlot, Candlestick, Configuration, HeatMap, Histogram, Ohlc, Pie, Plot, Sankey, Scatter,
    ScatterPolar, Trace,
    box_plot::{BoxPoints, QuartileMethod},
    common::{
        Anchor, ColorBar, ColorScale, ColorScalePalette, Fill, Font, Line, Marker, Mode,
        TextPosition, Title,
    },
    layout::{Annotation, Axis, AxisType, Layout, Margin},
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

/// Max bars in a single-series `viz bar` chart that still get value labels; beyond this the
/// labels would overlap, so they're omitted.
const LABEL_MAX_BARS: usize = 40;

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
    cmd_smart:       bool,
    cmd_bar:         bool,
    cmd_line:        bool,
    cmd_scatter:     bool,
    cmd_histogram:   bool,
    cmd_box:         bool,
    cmd_pie:         bool,
    cmd_heatmap:     bool,
    cmd_candlestick: bool,
    cmd_ohlc:        bool,
    cmd_sankey:      bool,
    cmd_radar:       bool,
    arg_input:       Option<String>,
    flag_x:          Option<SelectColumns>,
    flag_y:          Option<SelectColumns>,
    flag_z:          Option<SelectColumns>,
    flag_cols:       Option<SelectColumns>,
    flag_series:     Option<SelectColumns>,
    flag_donut:      bool,
    // scatter encodings: map a numeric column to per-point marker color (continuous
    // colorscale) and/or marker size (bubble chart). Mutually exclusive with --series.
    flag_color:      Option<SelectColumns>,
    flag_size:       Option<SelectColumns>,
    // candlestick / ohlc columns (--open is already taken by the browser-open flag below,
    // so the open-price column is selected with --ohlc-open)
    flag_ohlc_open:  Option<SelectColumns>,
    flag_high:       Option<SelectColumns>,
    flag_low:        Option<SelectColumns>,
    flag_close:      Option<SelectColumns>,
    // sankey columns
    flag_source:     Option<SelectColumns>,
    flag_target:     Option<SelectColumns>,
    flag_value:      Option<SelectColumns>,
    flag_bins:       Option<usize>,
    flag_agg:        Option<String>,
    flag_box_points: Option<String>,
    flag_max_charts: usize,
    flag_grid_cols:  usize,
    flag_limit:      usize,
    flag_title:      Option<String>,
    flag_x_title:    Option<String>,
    flag_y_title:    Option<String>,
    // width/height/scale only affect static image export (the viz_static feature). width
    // and height are optional: when unset, `viz smart` derives them from its grid shape and
    // other charts fall back to the defaults below.
    flag_width:      Option<usize>,
    flag_height:     Option<usize>,
    #[cfg_attr(not(feature = "viz_static"), allow(dead_code))]
    flag_scale:      f64,
    flag_open:       bool,
    flag_output:     Option<String>,
    flag_delimiter:  Option<Delimiter>,
    flag_no_headers: bool,
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
    // --color/--size are per-point marker encodings that only apply to scatter, and need a
    // single trace, so they can't be combined with --series (which splits into traces).
    if encoded_scatter(args) {
        if !matches!(chart_kind(args), Chart::Scatter) {
            return fail_incorrectusage_clierror!("--color/--size only apply to `viz scatter`.");
        }
        if args.flag_series.is_some() {
            return fail_incorrectusage_clierror!(
                "--color/--size cannot be combined with --series. Use --series to split into \
                 colored traces by category, or --color/--size to encode numeric columns onto a \
                 single series."
            );
        }
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
    Histogram,
    Box,
    Pie,
    Heatmap,
    Candlestick,
    Ohlc,
    Sankey,
    Radar,
}

fn chart_kind(args: &Args) -> Chart {
    if args.cmd_bar {
        Chart::Bar
    } else if args.cmd_line {
        Chart::Line
    } else if args.cmd_scatter {
        Chart::Scatter
    } else if args.cmd_histogram {
        Chart::Histogram
    } else if args.cmd_box {
        Chart::Box
    } else if args.cmd_pie {
        Chart::Pie
    } else if args.cmd_heatmap {
        Chart::Heatmap
    } else if args.cmd_candlestick {
        Chart::Candlestick
    } else if args.cmd_ohlc {
        Chart::Ohlc
    } else if args.cmd_sankey {
        Chart::Sankey
    } else if args.cmd_radar {
        Chart::Radar
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
        Some(PanelKind::FreqBar { idx })
    } else {
        None // high-cardinality / ID-like text
    }
}

/// Build a time-series trend panel when the dataset has a date/datetime column and a
/// continuous numeric column: that numeric column plotted as a line over the first
/// date/datetime column, with rows sorted chronologically.
///
/// `viz smart` computes stats with `--infer-dates --dates-whitelist sniff` (see
/// `util::get_stats_records`, `StatsMode::ProfileSchema`), so date/datetime columns that
/// `qsv sniff` identifies are typed confidently rather than reported as plain strings. Like
/// the correlation panel, this does one extra data pass (timestamps are not in the stats
/// cache). Returns None when there is no date column, no continuous numeric column, or fewer
/// than two parseable (date, value) pairs.
fn build_timeseries_panel(
    args: &Args,
    stats: &[crate::cmd::stats::StatsData],
    prefer_dmy: bool,
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
    // near-unique integers as ID-like and skip them.
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
        .filter(|(_, s)| continuous_numeric(s))
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
    let xs = points.iter().map(|p| p.1.clone()).collect();
    let ys = points.iter().map(|p| p.2).collect();
    Ok(Some(Panel {
        name: format!("{y_label} over {date_label}"),
        kind: PanelKind::TimeSeries { y_label, xs, ys },
    }))
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

    let schema_args = util::SchemaArgs {
        flag_enum_threshold:  0,
        flag_ignore_case:     false,
        flag_strict_dates:    false,
        flag_strict_formats:  false,
        flag_pattern_columns: SelectColumns::parse("").expect("empty selection is valid"),
        flag_dates_whitelist: "sniff".to_string(),
        flag_prefer_dmy:      false,
        flag_force:           false,
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

    // classify each column into a dashboard panel
    let mut panels: Vec<Panel> = Vec::new();
    let mut skipped: Vec<String> = Vec::new();
    for (idx, s) in stats.iter().enumerate() {
        let name = if s.field.is_empty() {
            format!("col {}", idx + 1)
        } else {
            s.field.clone()
        };
        match classify(idx, s) {
            Some(kind) => panels.push(Panel { name, kind }),
            None => skipped.push(name),
        }
    }

    // when 2+ continuous numeric columns exist, prepend a correlation-heatmap panel. This is
    // the one panel that re-scans the data (a single extra pass), since Pearson correlations
    // are not in the stats cache; it's prepended so it survives the panel cap below.
    let numeric_indices: Vec<usize> = stats
        .iter()
        .enumerate()
        .filter(|(_, s)| {
            matches!(s.r#type.as_str(), "Integer" | "Float")
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
            // a scatter of the most strongly correlated pair drills into the heatmap's
            // headline relationship. Reuses the columns already read for the matrix (no extra
            // data pass) and is only added when that pair is at least moderately correlated.
            let scatter_panel = strongest_pair(&matrix).and_then(|(i, j, r)| {
                (r.abs() >= SCATTER_PAIR_MIN_ABS_R).then(|| Panel {
                    name: format!("{} vs {} (r={r:.2})", labels[i], labels[j]),
                    kind: PanelKind::ScatterPair {
                        xs: columns[i].clone(),
                        ys: columns[j].clone(),
                    },
                })
            });
            panels.insert(
                0,
                Panel {
                    name: "Correlation".to_string(),
                    kind: PanelKind::CorrHeatmap { labels, matrix },
                },
            );
            // place the drill-down scatter right after the heatmap
            if let Some(panel) = scatter_panel {
                panels.insert(1, panel);
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
    if let Some(panel) = build_timeseries_panel(args, &stats, prefer_dmy)? {
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
    let inline = is_html && want > MAX_SUBPLOTS;

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
            | PanelKind::CorrHeatmap { .. }
            | PanelKind::TimeSeries { .. }
            | PanelKind::ScatterPair { .. } => None,
        })
        .collect();
    let top_n = args.flag_limit.max(1);
    let freq = if bar_indices.is_empty() {
        HashMap::new()
    } else {
        count_values(args, &bar_indices, top_n)?
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
            &title_text,
        )))
    } else {
        render_smart_grid(args, &panels, &freq, &title_text)
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
            | PanelKind::FreqBar { .. }
            | PanelKind::TimeSeries { .. }
            | PanelKind::ScatterPair { .. } => None,
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
        let is_box = matches!(panel.kind, PanelKind::BoxStats { .. });
        let is_date = matches!(panel.kind, PanelKind::TimeSeries { .. });
        let (trace, bar_max) = panel_trace(panel, color, freq, Some((xref.clone(), yref.clone())));
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
) -> Plot {
    let is_box = matches!(panel.kind, PanelKind::BoxStats { .. });
    let is_corr = matches!(panel.kind, PanelKind::CorrHeatmap { .. });
    let is_date = matches!(panel.kind, PanelKind::TimeSeries { .. });
    let (trace, bar_max) = panel_trace(panel, color, freq, None);

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
    title_text: &str,
) -> String {
    let cols = args.flag_grid_cols.clamp(1, panels.len().max(1));

    let mut cells = String::new();
    for (n, panel) in panels.iter().enumerate() {
        let color = PALETTE[n % PALETTE.len()];
        let plot = smart_inline_panel_plot(panel, color, freq);
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
}
