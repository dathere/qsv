static USAGE: &str = r#"
Pragmatic statistical toolkit.

Compute robust, median-of-pairwise statistics from the Pragmastat library.
Designed for messy, heavy-tailed, or outlier-prone data where mean/stddev can mislead.

Input handling
  * Only finite numeric values are used; non-numeric/NaN/Inf are ignored.
  * Each column is treated as its own sample (two-sample compares columns, not rows).
  * Non-numeric columns appear with n=0 and empty estimator cells.
  * NOTE: This command loads all numeric values into memory.

ONE-SAMPLE OUTPUT (default, per selected column)
  field, n, center, spread, rel_spread, center_lower, center_upper

  center       Robust location; median of pairwise averages (Hodges-Lehmann).
               Like the mean but stable with outliers; tolerates up to 29% corrupted data.
  spread       Robust dispersion; median of pairwise absolute differences (Shamos).
               Same units as data; also tolerates up to 29% corrupted data.
  rel_spread   Relative dispersion = spread / center (robust coefficient of variation).
               Dimensionless; compares variability across scales.

  center_lower/center_upper
               Bounds for center with error rate = misrate (exact under weak symmetry).
               Use 1e-3 for everyday analysis or 1e-6 for critical decisions.

TWO-SAMPLE OUTPUT (--twosample, per unordered column pair)
  field_x, field_y, n_x, n_y, shift, ratio, avg_spread, disparity,
  shift_lower, shift_upper, ratio_lower, ratio_upper

  shift        Robust difference in location; median of pairwise differences.
               Negative => first column tends to be lower.
  ratio        Robust multiplicative ratio; exp(shift(log x, log y)).
               Use for positive-valued quantities (latency, price, concentration).
  avg_spread   Pooled robust dispersion (weighted by sample sizes).
               Note: pooled scale, not Spread(x union y).
  disparity    Effect size = shift / avg_spread (robust Cohen's d).

  shift_lower/shift_upper, ratio_lower/ratio_upper
               Bounds for shift/ratio with error rate = misrate (ties may be conservative).
               If bounds exclude 0 (shift) or 1 (ratio), the difference is reliable.

When values are blank
  * Column has no numeric data (n=0).
  * Positivity required: rel_spread, ratio, ratio_* need all values > 0.
  * Sparity required: spread/avg_spread/disparity need real variability (not tie-dominant).
  * Bounds require enough data for requested misrate; try higher misrate or more data.

MISRATE PARAMETER
  misrate is the probability that bounds miss the true value (lower => wider bounds).
    1e-3    Everyday analysis [default]
    1e-6    Critical decisions

Examples:
  # Basic one-sample statistics
  qsv pragmastat data.csv

  # One-sample statistics with selected columns
  qsv pragmastat --select latency_ms,price data.csv

  # Two-sample statistics with selected columns
  qsv pragmastat --twosample --select latency_ms,price data.csv

  # One-sample statistics with very tight bounds (lower misrate)
  qsv pragmastat --misrate 1e-6 data.csv

Full Pragmastat manual:
https://github.com/AndreyAkinshin/pragmastat/releases/download/v8.0.0/pragmastat-v8.0.0.pdf
https://pragmastat.dev/ (latest version)

Usage:
    qsv pragmastat [options] [<input>]
    qsv pragmastat --help

pragmastat options:
    -t, --twosample        Compute two-sample estimators for all column pairs.
    -s, --select <cols>    Select columns for analysis. Uses qsv's column selection
                           syntax. Non-numeric columns appear with n=0.
                           In two-sample mode, all pairs of selected columns are computed.
    -m, --misrate <n>      Probability that bounds fail to contain the true parameter.
                           Lower values produce wider bounds.
                           Must be achievable for the given sample size.
                           [default: 0.001]

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -d, --delimiter <c>    The field delimiter for reading/writing CSV data.
                           Must be a single character. (default: ,)
    -n, --no-headers       When set, the first row will not be treated as headers.
    -j, --jobs <arg>       The number of jobs to run in parallel.
                           When not set, the number of jobs is set to the
                           number of CPUs detected.
    --memcheck             Check if there is enough memory to load the entire
                           CSV into memory using CONSERVATIVE heuristics. Not valid for stdin.
    -p, --progressbar      Show progress bars. Not valid for stdin.
"#;

use indicatif::{ProgressBar, ProgressDrawTarget};
use rayon::prelude::*;
use serde::Deserialize;

use crate::{
    CliResult,
    clitypes::CliError,
    config::{Config, Delimiter},
    select::SelectColumns,
    util,
};

#[derive(Deserialize)]
struct Args {
    arg_input:        Option<String>,
    flag_twosample:   bool,
    flag_select:      Option<SelectColumns>,
    flag_misrate:     f64,
    flag_output:      Option<String>,
    flag_delimiter:   Option<Delimiter>,
    flag_no_headers:  bool,
    flag_jobs:        Option<usize>,
    flag_memcheck:    bool,
    flag_progressbar: bool,
}

struct OneSampleResult {
    field:        String,
    n:            usize,
    center:       Option<f64>,
    spread:       Option<f64>,
    rel_spread:   Option<f64>,
    center_lower: Option<f64>,
    center_upper: Option<f64>,
}

struct TwoSampleResult {
    field_x:     String,
    field_y:     String,
    n_x:         usize,
    n_y:         usize,
    shift:       Option<f64>,
    ratio:       Option<f64>,
    avg_spread:  Option<f64>,
    disparity:   Option<f64>,
    shift_lower: Option<f64>,
    shift_upper: Option<f64>,
    ratio_lower: Option<f64>,
    ratio_upper: Option<f64>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    validate_misrate(args.flag_misrate)?;
    util::njobs(args.flag_jobs);
    let (col_names, col_values) = read_columns(&args)?;
    write_results(&args, &col_names, &col_values)?;
    Ok(())
}

fn validate_misrate(misrate: f64) -> CliResult<()> {
    if misrate.is_nan() || misrate <= 0.0 || misrate >= 1.0 {
        return Err(CliError::IncorrectUsage(
            "--misrate must be between 0 and 1 (exclusive).".to_string(),
        ));
    }
    Ok(())
}

fn read_columns(args: &Args) -> CliResult<(Vec<String>, Vec<Vec<f64>>)> {
    let rconfig = Config::new(args.arg_input.as_ref())
        .delimiter(args.flag_delimiter)
        .no_headers_flag(args.flag_no_headers);

    if let Some(ref path) = rconfig.path {
        util::mem_file_check(path, false, args.flag_memcheck)?;
    }

    let mut rdr = rconfig.reader()?;
    let headers = rdr.byte_headers()?.clone();
    let selected = resolve_columns(&rconfig, &headers, args.flag_select.as_ref())?;
    collect_numeric_values(
        &mut rdr,
        &headers,
        &selected,
        rconfig.no_headers,
        args.flag_progressbar,
        &rconfig,
    )
}

fn resolve_columns(
    rconfig: &Config,
    headers: &csv::ByteRecord,
    select: Option<&SelectColumns>,
) -> CliResult<Vec<usize>> {
    match select {
        Some(sel) => {
            let conf = rconfig.clone().select(sel.clone());
            Ok(conf
                .selection(headers)?
                .iter()
                .copied()
                .collect::<Vec<usize>>())
        },
        None => Ok((0..headers.len()).collect::<Vec<usize>>()),
    }
}

fn write_results(args: &Args, col_names: &[String], col_values: &[Vec<f64>]) -> CliResult<()> {
    let mut wtr = Config::new(args.flag_output.as_ref())
        .delimiter(args.flag_delimiter)
        .writer()?;

    if args.flag_twosample {
        write_twosample_results(&mut wtr, col_names, col_values, args.flag_misrate)?;
    } else {
        write_onesample_results(&mut wtr, col_names, col_values, args.flag_misrate)?;
    }

    wtr.flush()?;
    Ok(())
}

fn write_onesample_results(
    wtr: &mut csv::Writer<Box<dyn std::io::Write + 'static>>,
    col_names: &[String],
    col_values: &[Vec<f64>],
    misrate: f64,
) -> CliResult<()> {
    write_onesample_header(wtr)?;

    let results: Vec<OneSampleResult> = col_names
        .par_iter()
        .enumerate()
        .map(|(i, name)| compute_one_sample(name, &col_values[i], misrate))
        .collect();

    for result in &results {
        write_onesample_row(wtr, result)?;
    }
    Ok(())
}

fn write_twosample_results(
    wtr: &mut csv::Writer<Box<dyn std::io::Write + 'static>>,
    col_names: &[String],
    col_values: &[Vec<f64>],
    misrate: f64,
) -> CliResult<()> {
    write_twosample_header(wtr)?;

    let k = col_names.len();
    let num_pairs = k.saturating_mul(k - 1) / 2;
    if num_pairs > 100 {
        winfo!(
            "computing {num_pairs} column pairs from {k} columns. Use --select to limit columns \
             for faster results."
        );
    }

    let pairs: Vec<(usize, usize)> = (0..k)
        .flat_map(|i| ((i + 1)..k).map(move |j| (i, j)))
        .collect();

    let results: Vec<TwoSampleResult> = pairs
        .par_iter()
        .map(|&(i, j)| {
            compute_two_sample(
                &col_names[i],
                &col_names[j],
                &col_values[i],
                &col_values[j],
                misrate,
            )
        })
        .collect();

    for result in &results {
        write_twosample_row(wtr, result)?;
    }
    Ok(())
}

fn collect_numeric_values(
    rdr: &mut csv::Reader<Box<dyn std::io::Read + Send + 'static>>,
    headers: &csv::ByteRecord,
    selected: &[usize],
    no_headers: bool,
    show_progress: bool,
    rconfig: &Config,
) -> CliResult<(Vec<String>, Vec<Vec<f64>>)> {
    let col_names: Vec<String> = selected
        .iter()
        .map(|&i| {
            if no_headers {
                (i + 1).to_string()
            } else {
                String::from_utf8_lossy(&headers[i]).into_owned()
            }
        })
        .collect();

    let mut col_values: Vec<Vec<f64>> = vec![Vec::new(); selected.len()];

    let progress = ProgressBar::with_draw_target(None, ProgressDrawTarget::hidden());
    if show_progress {
        util::prep_progress(&progress, util::count_rows(rconfig)?);
        progress.set_draw_target(ProgressDrawTarget::stderr_with_hz(5));
    }

    for result in rdr.byte_records() {
        let record = result?;
        for (idx, &col_idx) in selected.iter().enumerate() {
            if let Some(field) = record.get(col_idx)
                && let Ok(val) = fast_float2::parse::<f64, _>(field)
                && val.is_finite()
            {
                col_values[idx].push(val);
            }
        }
        if show_progress {
            progress.inc(1);
        }
    }

    if show_progress {
        util::finish_progress(&progress);
    }

    Ok((col_names, col_values))
}

fn compute_one_sample(name: &str, values: &[f64], misrate: f64) -> OneSampleResult {
    let n = values.len();

    if n == 0 {
        return OneSampleResult {
            field: name.to_string(),
            n,
            center: None,
            spread: None,
            rel_spread: None,
            center_lower: None,
            center_upper: None,
        };
    }

    let center = pragmastat::center(values).ok();
    let spread = pragmastat::spread(values).ok();
    let rel_spread = pragmastat::rel_spread(values).ok();
    let bounds = pragmastat::center_bounds(values, misrate).ok();

    OneSampleResult {
        field: name.to_string(),
        n,
        center,
        spread,
        rel_spread,
        center_lower: bounds.map(|b| b.lower),
        center_upper: bounds.map(|b| b.upper),
    }
}

fn compute_two_sample(
    name_x: &str,
    name_y: &str,
    x: &[f64],
    y: &[f64],
    misrate: f64,
) -> TwoSampleResult {
    let n_x = x.len();
    let n_y = y.len();

    if n_x == 0 || n_y == 0 {
        return TwoSampleResult {
            field_x: name_x.to_string(),
            field_y: name_y.to_string(),
            n_x,
            n_y,
            shift: None,
            ratio: None,
            avg_spread: None,
            disparity: None,
            shift_lower: None,
            shift_upper: None,
            ratio_lower: None,
            ratio_upper: None,
        };
    }

    let shift = pragmastat::shift(x, y).ok();
    let ratio = pragmastat::ratio(x, y).ok();
    let avg_spread = pragmastat::avg_spread(x, y).ok();
    let disparity = pragmastat::disparity(x, y).ok();
    let shift_bounds = pragmastat::shift_bounds(x, y, misrate).ok();
    let ratio_bounds = pragmastat::ratio_bounds(x, y, misrate).ok();

    TwoSampleResult {
        field_x: name_x.to_string(),
        field_y: name_y.to_string(),
        n_x,
        n_y,
        shift,
        ratio,
        avg_spread,
        disparity,
        shift_lower: shift_bounds.map(|b| b.lower),
        shift_upper: shift_bounds.map(|b| b.upper),
        ratio_lower: ratio_bounds.map(|b| b.lower),
        ratio_upper: ratio_bounds.map(|b| b.upper),
    }
}

fn fmt_opt(val: Option<f64>) -> String {
    match val {
        Some(v) => util::round_num(v, 4),
        None => String::new(),
    }
}

fn write_onesample_header(
    wtr: &mut csv::Writer<Box<dyn std::io::Write + 'static>>,
) -> CliResult<()> {
    wtr.write_record([
        "field",
        "n",
        "center",
        "spread",
        "rel_spread",
        "center_lower",
        "center_upper",
    ])?;
    Ok(())
}

fn write_onesample_row(
    wtr: &mut csv::Writer<Box<dyn std::io::Write + 'static>>,
    r: &OneSampleResult,
) -> CliResult<()> {
    wtr.write_record([
        &r.field,
        &r.n.to_string(),
        &fmt_opt(r.center),
        &fmt_opt(r.spread),
        &fmt_opt(r.rel_spread),
        &fmt_opt(r.center_lower),
        &fmt_opt(r.center_upper),
    ])?;
    Ok(())
}

fn write_twosample_header(
    wtr: &mut csv::Writer<Box<dyn std::io::Write + 'static>>,
) -> CliResult<()> {
    wtr.write_record([
        "field_x",
        "field_y",
        "n_x",
        "n_y",
        "shift",
        "ratio",
        "avg_spread",
        "disparity",
        "shift_lower",
        "shift_upper",
        "ratio_lower",
        "ratio_upper",
    ])?;
    Ok(())
}

fn write_twosample_row(
    wtr: &mut csv::Writer<Box<dyn std::io::Write + 'static>>,
    r: &TwoSampleResult,
) -> CliResult<()> {
    wtr.write_record([
        &r.field_x,
        &r.field_y,
        &r.n_x.to_string(),
        &r.n_y.to_string(),
        &fmt_opt(r.shift),
        &fmt_opt(r.ratio),
        &fmt_opt(r.avg_spread),
        &fmt_opt(r.disparity),
        &fmt_opt(r.shift_lower),
        &fmt_opt(r.shift_upper),
        &fmt_opt(r.ratio_lower),
        &fmt_opt(r.ratio_upper),
    ])?;
    Ok(())
}
