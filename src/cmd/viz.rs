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

  # Histogram of a numeric column with 30 bins
  qsv viz histogram data.csv --x value --bins 30 -o hist.html

  # Box plot of a value column grouped by a category, exported to PNG (needs viz_static)
  qsv viz box data.csv --y measurement --x group -o box.png

For more examples, see https://github.com/dathere/qsv/blob/master/tests/test_viz.rs.

Usage:
    qsv viz smart     [options] <input>
    qsv viz bar       [options] <input>
    qsv viz line      [options] <input>
    qsv viz scatter   [options] <input>
    qsv viz histogram [options] <input>
    qsv viz box       [options] <input>
    qsv viz --help

viz options:
    -x, --x <col>          Column for the x-axis / category / bin / group.
    -y, --y <col>          Column for the y-axis / value.
    --series <col>         Column to split into multiple series (one trace per
                           distinct value). Applies to bar/line/scatter.
    --bins <n>             Number of bins for the histogram. (default: auto)
    --agg <fn>             For bar/line, aggregate the y values when the x value
                           repeats. One of: sum, mean, count, min, max.

smart options:
    --max-charts <n>       Maximum number of panels in the dashboard. Capped at 8
                           (plotly's typed subplot-axis limit); extra eligible
                           columns are reported but not drawn. [default: 8]
    --grid-cols <n>        Number of columns in the dashboard grid. [default: 2]
    --limit <n>            Top-N categories per frequency bar chart. [default: 10]

    --title <s>            Chart title.
    --x-title <s>          X-axis title. (defaults to the x column name)
    --y-title <s>          Y-axis title. (defaults to the y column name)
    --width <n>            Image width in pixels (static export). [default: 1000]
    --height <n>           Image height in pixels (static export). [default: 600]
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
    Bar, BoxPlot, Histogram, Plot, Scatter, Trace,
    common::{Mode, Title},
    layout::{Axis, GridPattern, Layout, LayoutGrid},
};
use serde::Deserialize;

use crate::{
    CliResult,
    config::{Config, Delimiter},
    select::SelectColumns,
    util,
};

/// Plotly's `Layout` exposes typed axis fields only up to x_axis8/y_axis8, which caps a
/// typed subplot grid at 8 panels.
const MAX_SUBPLOTS: usize = 8;

/// A column whose cardinality is at or below this is treated as categorical (frequency
/// bar) rather than continuous (box plot).
const CATEGORICAL_MAX_CARDINALITY: u64 = 30;

#[derive(Deserialize)]
struct Args {
    cmd_smart:       bool,
    cmd_bar:         bool,
    cmd_line:        bool,
    cmd_scatter:     bool,
    cmd_histogram:   bool,
    cmd_box:         bool,
    arg_input:       Option<String>,
    flag_x:          Option<SelectColumns>,
    flag_y:          Option<SelectColumns>,
    flag_series:     Option<SelectColumns>,
    flag_bins:       Option<usize>,
    flag_agg:        Option<String>,
    flag_max_charts: usize,
    flag_grid_cols:  usize,
    flag_limit:      usize,
    flag_title:      Option<String>,
    flag_x_title:    Option<String>,
    flag_y_title:    Option<String>,
    // width/height/scale only affect static image export (the viz_static feature)
    #[cfg_attr(not(feature = "viz_static"), allow(dead_code))]
    flag_width:      usize,
    #[cfg_attr(not(feature = "viz_static"), allow(dead_code))]
    flag_height:     usize,
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

    let plot = if args.cmd_smart {
        build_smart(&args)?
    } else {
        build_plot(&args)?
    };

    output_plot(&plot, &args, out_format)
}

/// Build a `Plot` for the requested chart subcommand.
fn build_plot(args: &Args) -> CliResult<Plot> {
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
        // bar / line / scatter all consume (x, y) pairs, optionally split by --series
        kind => {
            let (traces, x_label, y_label) = build_xy_traces(args, kind)?;
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
                let mut t = Bar::new(xs, ys);
                if !name.is_empty() {
                    t = t.name(name);
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

    let y_label = col_label(&headers, y_idx, nh);
    let x_label = group_idx.map(|i| col_label(&headers, i, nh));
    let trace: Box<dyn Trace> = if group_idx.is_some() {
        BoxPlot::new_xy(groups, ys)
    } else {
        BoxPlot::new(ys).name(y_label.clone())
    };
    Ok((trace, y_label, x_label))
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

fn output_plot(plot: &Plot, args: &Args, fmt: OutFormat) -> CliResult<()> {
    if matches!(fmt, OutFormat::Html) {
        output_html(plot, args)
    } else {
        output_image(plot, args, fmt)
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
fn output_image(plot: &Plot, args: &Args, fmt: OutFormat) -> CliResult<()> {
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
    plot.write_image(
        path,
        image_format,
        args.flag_width,
        args.flag_height,
        args.flag_scale,
    )
    .map_err(|e| {
        crate::CliError::Other(format!(
            "Static image export failed: {e}. A Chromium or Firefox browser must be installed and \
             available for plotly's webdriver-based export."
        ))
    })?;
    if args.flag_open {
        open_path(path)?;
    }
    Ok(())
}

#[cfg(not(feature = "viz_static"))]
#[allow(clippy::unnecessary_wraps)]
fn output_image(_plot: &Plot, _args: &Args, _fmt: OutFormat) -> CliResult<()> {
    unreachable!("image export is rejected in run() without the viz_static feature");
}

fn open_path(path: &str) -> CliResult<()> {
    opener::open(path).map_err(|e| crate::CliError::Other(format!("Could not open '{path}': {e}")))
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

/// Build the `viz smart` auto-dashboard from the dataset's statistics + frequency data.
fn build_smart(args: &Args) -> CliResult<Plot> {
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
        flag_dates_whitelist: "all".to_string(),
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

    // cap to the typed-axis subplot limit
    let max_panels = args.flag_max_charts.clamp(1, MAX_SUBPLOTS);
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
            PanelKind::BoxStats { .. } => None,
        })
        .collect();
    let top_n = args.flag_limit.max(1);
    let freq = if bar_indices.is_empty() {
        HashMap::new()
    } else {
        count_values(args, &bar_indices, top_n)?
    };

    // assemble the subplot-grid dashboard
    let cols = args.flag_grid_cols.clamp(1, panels.len());
    let rows = panels.len().div_ceil(cols);

    let mut plot = Plot::new();
    let mut layout = Layout::new().show_legend(false).grid(
        LayoutGrid::new()
            .rows(rows)
            .columns(cols)
            .pattern(GridPattern::Independent),
    );
    if let Some(t) = &args.flag_title {
        layout = layout.title(Title::with_text(t));
    }

    for (n, panel) in panels.iter().enumerate() {
        let pos = n + 1;
        let xref = axis_ref('x', pos);
        let yref = axis_ref('y', pos);
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
                    .x_axis(xref)
                    .y_axis(yref);
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
                Bar::new(xs, ys)
                    .name(panel.name.clone())
                    .x_axis(xref)
                    .y_axis(yref)
            },
        };
        plot.add_trace(trace);
        layout = set_subplot_title(layout, pos, &panel.name);
    }

    plot.set_layout(layout);
    Ok(plot)
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

/// Set the x-axis title (the column name) for subplot `pos`. Typed Layout axis fields
/// only exist up to 8, which matches MAX_SUBPLOTS.
fn set_subplot_title(layout: Layout, pos: usize, title: &str) -> Layout {
    let axis = Axis::new().title(Title::with_text(title.to_string()));
    match pos {
        1 => layout.x_axis(axis),
        2 => layout.x_axis2(axis),
        3 => layout.x_axis3(axis),
        4 => layout.x_axis4(axis),
        5 => layout.x_axis5(axis),
        6 => layout.x_axis6(axis),
        7 => layout.x_axis7(axis),
        8 => layout.x_axis8(axis),
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
}
