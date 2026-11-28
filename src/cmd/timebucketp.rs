pub(crate) static USAGE: &str = r#"
Buckets time-series CSV data into time windows using the Polars engine.

Groups rows into windows over a date, datetime or integer index column and
aggregates them. Unlike sqlp's DATE_TRUNC, windows need not be calendar units
(e.g. 15m), can overlap (a --period longer than --every) and can be shifted
(--offset).

WINDOW MODE (default):
  A new window starts every --every and spans --period (default: --every).
  Each output row is one window. Windows with no input rows are skipped
  unless you set --fill-gaps.

ROLLING MODE (--rolling):
  One window per input row, looking back --period from that row's index
  value, e.g. "the average over the preceding 2 hours". By default a row's
  window is (index - period, index], so it includes the row itself. The
  output has one row per input row.

DURATIONS are an integer followed by a unit, and units combine (e.g. 1h30m):
  ns, us, ms, s, m (minute), h, d, w, mo (calendar month), q (quarter),
  y (year), i (index units, for integer index columns only: 3i spans 3).
  d, w, mo, q and y are calendar-aware: they respect month lengths.
  A leading minus makes a duration negative, e.g. --offset -30m.

The index column is parsed as dates or datetimes. Like the rest of qsv,
ambiguous dates such as 01/03/2024 are read month-first (January 3). Set
QSV_PREFER_DMY to read them day-first, or give the exact format with
the --date-format option. A text column whose every value is a date is
parsed the same way for min, max, mean, median and quantile, so it compares
as dates; first, last and --by keep values as written (unless a pschema.json
types them). Dates need 4-digit years unless --date-format is given. An index
that does not parse as dates is an error, as is an empty index value. An
index that mixes dates and datetimes, as `qsv datefmt` writes midnight
values, is read as datetimes, its dates as midnight.

A pschema.json made by `qsv schema --polars` while QSV_PREFER_DMY was set
marks day/month dates as already parsed, so they stay day-first in later
runs even with QSV_PREFER_DMY unset. After changing QSV_PREFER_DMY, delete
or regenerate the pschema.json. --date-format overrides it for the index
column only.

Examples:

  # hourly sales totals
  $ qsv timebucketp --index ts --every 1h --agg sum:sales sales.csv

  # 1-hour windows starting every 15 minutes, computed per station
  $ qsv timebucketp -i ts --every 15m --period 1h --by station --agg mean:temp,max:temp readings.csv

  # weekly row counts, weeks starting on Monday, with empty weeks as 0
  $ qsv timebucketp -i date --every 1w --start-by monday --fill-gaps events.csv

  # trailing 2-hour average price at each trade
  $ qsv timebucketp -i ts --rolling --period 2h --agg mean:price trades.csv

For more examples, see https://github.com/dathere/qsv/blob/master/tests/test_timebucketp.rs.
See also https://github.com/dathere/qsv/wiki/SQL-and-Polars#timebucketp

Usage:
    qsv timebucketp [options] <input>
    qsv timebucketp --help

timebucketp arguments:
    <input>                The input CSV file. The file must have headers.
                           If the file has a pschema.json file, it is used to
                           type the columns unless --infer-len is set to a value
                           other than the default. Stdin is not supported.
                           A snappy-compressed (.sz) input is decompressed
                           first, and an --output ending in .sz is
                           snappy-compressed. Lines starting with
                           QSV_COMMENT_CHAR are skipped.
                           QSV_POLARS_FLOAT_PRECISION and QSV_OUTPUT_BOM apply
                           to the output, as in sqlp and pivotp.

timebucketp options:
    -i, --index <col>      The date, datetime or integer column that defines
                           the windows. Required. The output keeps its name.
    --every <dur>          Start a new window at this interval.
                           Required in window mode.
    --period <dur>         The length of each window. Defaults to --every in
                           window mode. Required in rolling mode.
    --offset <dur>         Shift the window boundaries by this duration.
                           Defaults to 0 in window mode, and to minus --period
                           in rolling mode (a look-back window).
    --date-format <fmt>    The strptime format of a text --index column, e.g.
                           %m/%d/%Y or "%d.%m.%Y %H:%M". Without time fields
                           the index is read as dates, otherwise as datetimes.
    -b, --by <cols>        Also group by these columns (comma-separated).
                           Windows are computed separately for each group.
    -a, --agg <specs>      Comma-separated aggregations, as func:column pairs,
                           e.g. sum:sales,mean:temp. The functions are:
                             first, last, sum, min, max, mean, median,
                             quantile@<p> (alias q@<p>, p in [0, 1]), len.
                           Each is output as <column>_<func>, e.g. sales_sum,
                           or temp_quantile_0.95 for quantile@0.95.
                           len takes no column: it counts the rows in each
                           window and is output as "count".
                           [default: len]
    --label <how>          The value written to the index column for each
                           window. Window mode only. One of:
                             left      - the window's start (the default)
                             right     - the window's end
                             datapoint - the window's first index value
    --closed <side>        Which window boundaries are inclusive: left, right,
                           both or none. Defaults to left in window mode and
                           to right in rolling mode.
    --start-by <how>       Where the first window starts. Window mode only.
                           One of:
                             window    - the --every boundary at or before
                                         the first index value (the default)
                             datapoint - the first index value itself
                             monday ... sunday - that weekday (weekly --every)
    --boundaries           Add _lower_boundary and _upper_boundary columns
                           before the index column. Window mode only.
    --fill-gaps            Also emit the empty windows between each group's
                           first and last window: count, sum and len are 0
                           there, and every other aggregation is empty.
                           Window mode only. Requires the default left labels,
                           and cannot be combined with --start-by datapoint
                           or --boundaries.
    --rolling              Use rolling mode (see above).
    --sorted               The input is already sorted by --index, within
                           each group when using --by, so skip timebucketp's
                           sort. Unsorted input is then an error, never a
                           wrong result.
    --infer-len <arg>      Number of rows to scan when inferring the schema.
                           Set to 0 to scan the entire file. [default: 10000]
    --decimal-comma        Use comma as decimal separator when READING the input.
                           Note that you will need to specify an alternate --delimiter.
    --ignore-errors        Read CSV values that can't be parsed as their
                           column's type as empty (null) instead of failing.
                           Their rows are kept. This does not relax the
                           index: its values must still parse as dates (or
                           integers), with none empty.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -d, --delimiter <arg>  The field delimiter for reading/writing CSV data.
                           Must be a single character. If not set, QSV_DEFAULT_DELIMITER
                           is used, else ','. A .tsv/.tab (tab) or .ssv (semicolon)
                           extension on the input or --output file sets that file's
                           delimiter. (default: ,)
    -q, --quiet            Do not print the result's shape to stderr.
"#;

use std::{
    fs::File,
    io,
    io::Write,
    path::{Path, PathBuf},
};

use foldhash::HashSet;
use polars::prelude::*;
use serde::Deserialize;

use crate::{
    CliResult,
    cmd::{joinp::path_delim, pivotp::AggKind, sqlp::compress_output_if_needed},
    config::Delimiter,
    util,
};

const DEFAULT_INFER_LEN: usize = 10000;

#[derive(Deserialize)]
struct Args {
    arg_input:          String,
    flag_index:         Option<String>,
    flag_every:         Option<String>,
    flag_period:        Option<String>,
    flag_offset:        Option<String>,
    flag_date_format:   Option<String>,
    flag_by:            Option<String>,
    flag_agg:           String,
    flag_label:         Option<String>,
    flag_closed:        Option<String>,
    flag_start_by:      Option<String>,
    flag_boundaries:    bool,
    flag_fill_gaps:     bool,
    flag_rolling:       bool,
    flag_sorted:        bool,
    flag_infer_len:     usize,
    flag_decimal_comma: bool,
    flag_ignore_errors: bool,
    flag_output:        Option<String>,
    flag_delimiter:     Option<Delimiter>,
    flag_quiet:         bool,
}

/// One `--agg` entry: the aggregation, the column it reads (None for `len`) and its output name.
struct AggSpec {
    kind:   AggKind,
    column: Option<String>,
    output: String,
}

trait ComparesValues {
    fn compares_values(self) -> bool;
}

impl ComparesValues for AggKind {
    /// Aggregations whose result depends on comparing or averaging values, so a text date
    /// column must be parsed for them.
    fn compares_values(self) -> bool {
        matches!(
            self,
            Self::Min | Self::Max | Self::Mean | Self::Median | Self::Quantile(_)
        )
    }
}

impl AggSpec {
    fn expr(&self) -> Expr {
        match &self.column {
            Some(c) => self.kind.on(col(c.as_str())).alias(self.output.as_str()),
            None => len().alias(self.output.as_str()),
        }
    }
}

/// Parse `--agg`, e.g. `sum:sales,mean:temp,len`, rejecting duplicate output columns.
fn parse_aggs(spec: &str) -> CliResult<Vec<AggSpec>> {
    let mut aggs = Vec::new();
    let mut seen = HashSet::default();
    for item in spec.split(',').map(str::trim).filter(|s| !s.is_empty()) {
        let (func, column) = match item.split_once(':') {
            Some((f, c)) => (f.trim().to_lowercase(), Some(c.trim().to_string())),
            None => (item.to_lowercase(), None),
        };
        let agg = match (func.as_str(), column) {
            ("len", None) => AggSpec {
                kind:   AggKind::Len,
                column: None,
                output: "count".to_string(),
            },
            ("len", Some(_)) => {
                return fail_incorrectusage_clierror!(
                    "--agg len takes no column: it counts the rows in each window."
                );
            },
            ("item", _) => {
                return fail_incorrectusage_clierror!(
                    "--agg item is not supported by timebucketp."
                );
            },
            (f, Some(c)) if !c.is_empty() => {
                let Some(kind) = AggKind::from_name(f) else {
                    return fail_incorrectusage_clierror!(
                        "Invalid --agg function \"{f}\" in \"{item}\"."
                    );
                };
                let suffix = match kind {
                    AggKind::Quantile(p) => format!("quantile_{p}"),
                    _ => f.to_string(),
                };
                AggSpec {
                    kind,
                    output: format!("{c}_{suffix}"),
                    column: Some(c),
                }
            },
            (f, _) => {
                return fail_incorrectusage_clierror!(
                    "--agg \"{item}\" needs a column, as {f}:<column>."
                );
            },
        };
        if !seen.insert(agg.output.clone()) {
            return fail_incorrectusage_clierror!(
                "--agg produces the column \"{}\" more than once.",
                agg.output
            );
        }
        aggs.push(agg);
    }
    if aggs.is_empty() {
        return fail_incorrectusage_clierror!("--agg needs at least one aggregation.");
    }
    Ok(aggs)
}

fn parse_duration(flag: &str, value: &str) -> CliResult<Duration> {
    match Duration::try_parse(value) {
        Ok(d) => Ok(d),
        Err(e) => fail_incorrectusage_clierror!("Invalid {flag} duration \"{value}\": {e}"),
    }
}

fn parse_closed(value: &str) -> CliResult<ClosedWindow> {
    Ok(match value.to_lowercase().as_str() {
        "left" => ClosedWindow::Left,
        "right" => ClosedWindow::Right,
        "both" => ClosedWindow::Both,
        "none" => ClosedWindow::None,
        _ => {
            return fail_incorrectusage_clierror!(
                "Invalid --closed \"{value}\": use left, right, both or none."
            );
        },
    })
}

fn parse_label(value: &str) -> CliResult<Label> {
    Ok(match value.to_lowercase().as_str() {
        "left" => Label::Left,
        "right" => Label::Right,
        "datapoint" => Label::DataPoint,
        _ => {
            return fail_incorrectusage_clierror!(
                "Invalid --label \"{value}\": use left, right or datapoint."
            );
        },
    })
}

fn parse_start_by(value: &str) -> CliResult<StartBy> {
    Ok(match value.to_lowercase().as_str() {
        "window" => StartBy::WindowBound,
        "datapoint" => StartBy::DataPoint,
        "monday" => StartBy::Monday,
        "tuesday" => StartBy::Tuesday,
        "wednesday" => StartBy::Wednesday,
        "thursday" => StartBy::Thursday,
        "friday" => StartBy::Friday,
        "saturday" => StartBy::Saturday,
        "sunday" => StartBy::Sunday,
        _ => {
            return fail_incorrectusage_clierror!(
                "Invalid --start-by \"{value}\": use window, datapoint or a weekday name."
            );
        },
    })
}

/// Polars' own day-first and year-first date patterns (`polars-core`'s `patterns.rs`, which keeps
/// them private). Polars has no month-first patterns, and reads `01/03/2024` as 1 March.
const DATE_Y_M_D: &[&str] = &["%Y-%m-%d", "%Y/%m/%d", "%Y.%m.%d"];
const DATE_D_M_Y: &[&str] = &["%d-%m-%Y", "%d/%m/%Y", "%d.%m.%Y"];
const DATETIME_D_M_Y: &[&str] = &[
    "%d-%m-%YT%H:%M:%S%.f",
    "%d-%m-%YT%H%M%S%.f",
    "%d-%m-%YT%H:%M",
    "%d-%m-%YT%H%M",
    "%d-%m-%Y %H:%M:%S%.f",
    "%d-%m-%Y %H%M%S%.f",
    "%d-%m-%Y %H:%M",
    "%d-%m-%Y %H%M",
    "%d/%m/%YT%H:%M:%S%.f",
    "%d/%m/%YT%H%M%S%.f",
    "%d/%m/%YT%H:%M",
    "%d/%m/%YT%H%M",
    "%d/%m/%Y %H:%M:%S%.f",
    "%d/%m/%Y %H%M%S%.f",
    "%d/%m/%Y %H:%M",
    "%d/%m/%Y %H%M",
    "%d.%m.%YT%H:%M:%S%.f",
    "%d.%m.%YT%H%M%S%.f",
    "%d.%m.%YT%H:%M",
    "%d.%m.%YT%H%M",
    "%d.%m.%Y %H:%M:%S%.f",
    "%d.%m.%Y %H%M%S%.f",
    "%d.%m.%Y %H:%M",
    "%d.%m.%Y %H%M",
];

/// The format to parse a text index with, chosen from its first value: year-first dates as
/// such, and day/month dates month-first like the rest of qsv, or day-first with
/// `QSV_PREFER_DMY`. None (anything else, e.g. ISO datetimes) leaves the format to Polars.
fn index_date_format(sample: &str, prefer_dmy: bool) -> Option<String> {
    use chrono::{NaiveDate, NaiveDateTime};

    // chrono's %Y also takes a 2-digit year (10/05/24 would be read as the year 10 or 24), so
    // only pick a format for a 4-digit year where that format expects it
    let fields: Vec<&str> = sample
        .split([' ', 'T'])
        .next()
        .unwrap_or_default()
        .split(['-', '/', '.'])
        .collect();
    let four_digits = |i: usize| {
        fields
            .get(i)
            .is_some_and(|f| f.len() == 4 && f.bytes().all(|b| b.is_ascii_digit()))
    };
    if fields.len() != 3 {
        return None;
    }
    if four_digits(0)
        && let Some(f) = DATE_Y_M_D
            .iter()
            .find(|f| NaiveDate::parse_from_str(sample, f).is_ok())
    {
        return Some((*f).to_string());
    }
    if !four_digits(2) {
        return None;
    }
    let order = |f: &str, day_first: bool| {
        if day_first {
            f.to_string()
        } else {
            f.replacen("%d", "%DAY", 1)
                .replacen("%m", "%d", 1)
                .replacen("%DAY", "%m", 1)
        }
    };
    // The preferred order first; a first value that only parses the other way (25/12/2024
    // when month-first is preferred) settles the column's order.
    [prefer_dmy, !prefer_dmy].into_iter().find_map(|day_first| {
        DATE_D_M_Y
            .iter()
            .map(|f| order(f, day_first))
            .find(|f| NaiveDate::parse_from_str(sample, f).is_ok())
            .or_else(|| {
                DATETIME_D_M_Y
                    .iter()
                    .map(|f| order(f, day_first))
                    .find(|f| NaiveDateTime::parse_from_str(sample, f).is_ok())
            })
    })
}

/// The first non-empty value of a text column, trimmed.
fn first_text_value(lf: &LazyFrame, name: &str) -> CliResult<Option<String>> {
    let first = lf
        .clone()
        .select([col(name)])
        .filter(col(name).is_not_null())
        .limit(1)
        .collect()?;
    Ok(first
        .column(name)?
        .str()?
        .get(0)
        .map(|v| v.trim().to_string()))
}

/// Whether Polars recognizes a value as a year-first datetime, with or without a zone.
fn looks_like_datetime(value: &str) -> bool {
    use polars::chunked_array::temporal::string::{infer::infer_pattern_single, patterns::Pattern};
    matches!(
        infer_pattern_single(value),
        Some(Pattern::DatetimeYMD | Pattern::DatetimeYMDZ)
    )
}

fn text_date_expr(
    name: &str,
    format: Option<&str>,
    first: Option<&str>,
    strict: bool,
    guessed: bool,
    nanos: bool,
) -> Expr {
    text_date_expr_of(col(name), name, format, first, strict, guessed, nanos)
}

/// `text_date_expr` of any text expression, aliased to `name`.
fn text_date_expr_of(
    input: Expr,
    name: &str,
    format: Option<&str>,
    first: Option<&str>,
    strict: bool,
    guessed: bool,
    nanos: bool,
) -> Expr {
    use polars::chunked_array::temporal::string::{infer::infer_pattern_single, patterns::Pattern};
    let options = StrptimeOptions {
        format: format.map(PlSmallStr::from_str),
        strict,
        ..Default::default()
    };
    let parsed = if format.is_some_and(|f| !format_has_time(f)) {
        input.str().to_date(options)
    } else {
        let zoned =
            format.is_none() && first.and_then(infer_pattern_single) == Some(Pattern::DatetimeYMDZ);
        input.str().to_datetime(
            nanos.then_some(TimeUnit::Nanoseconds),
            zoned.then_some(TimeZone::UTC),
            options,
            lit("raise"),
        )
    };
    if !guessed {
        return parsed.alias(name);
    }
    // A guessed or inferred format's %Y also takes 2-digit years (10/06/24, 10/06/ 24 or
    // 24-10-06T10:15 as the year 24), and only the first value is checked, so years before
    // 1000 become nulls: an error for the index, text comparison for an aggregated column.
    when(parsed.clone().dt().year().lt(lit(1000)))
        .then(lit(NULL))
        .otherwise(parsed)
        .alias(name)
}

/// A text index that mixes dates and datetimes, e.g. `qsv datefmt` output, which drops midnight
/// times. Parses it as datetimes, its dates as midnight: values in the first value's `format`
/// as such, the rest strictly in the format of the first value that does not match it. None when
/// every value matches `format`, or the first that does not is no date or datetime of the other
/// kind.
fn mixed_index_expr(
    lf: &LazyFrame,
    name: &str,
    format: Option<&str>,
    first: &str,
    prefer_dmy: bool,
    nanos: bool,
) -> CliResult<Option<Expr>> {
    use polars::chunked_array::temporal::string::{infer::infer_pattern_single, patterns::Pattern};
    let first_is_date = format.is_some_and(|f| !format_has_time(f));
    let primary = text_date_expr_of(col(name), name, format, Some(first), false, false, nanos);
    let unmatched = lf
        .clone()
        .select([col(name)])
        .filter(col(name).is_not_null().and(primary.clone().is_null()))
        .limit(1)
        .collect()?;
    // ChunkedArray::get panics out of bounds; no row means every value matches `format`
    if unmatched.height() == 0 {
        return Ok(None);
    }
    let Some(value) = unmatched
        .column(name)?
        .str()?
        .get(0)
        .map(|v| v.trim().to_string())
    else {
        return Ok(None);
    };
    // a day/month first value settles the order for the rest too
    // Some(true) for day-first, Some(false) for month-first, None for year-first or inferred
    let day_first = |f: Option<&str>| match f.and_then(|f| f.get(..2)) {
        Some("%d") => Some(true),
        Some("%m") => Some(false),
        _ => None,
    };
    let first_order = day_first(format);
    let other_format = index_date_format(&value, first_order.unwrap_or(prefer_dmy));
    let other_is_kind = match other_format.as_deref() {
        Some(f) => {
            format_has_time(f) == first_is_date
                && first_order.is_none_or(|o| day_first(Some(f)) == Some(o))
        },
        None => first_is_date && looks_like_datetime(&value),
    };
    if !other_is_kind {
        return Ok(None);
    }
    // parse the rest on its own, so a format inferred by Polars comes from one of its values
    let others = text_date_expr_of(
        when(primary.clone().is_null())
            .then(col(name))
            .otherwise(lit(NULL)),
        name,
        other_format.as_deref(),
        Some(&value),
        true,
        false,
        nanos,
    );
    let zoned = |f: Option<&str>, v: &str| {
        f.is_none() && infer_pattern_single(v) == Some(Pattern::DatetimeYMDZ)
    };
    let dtype = DataType::Datetime(
        if nanos {
            TimeUnit::Nanoseconds
        } else {
            TimeUnit::Microseconds
        },
        (zoned(format, first) || zoned(other_format.as_deref(), &value)).then_some(TimeZone::UTC),
    );
    let parsed = polars::lazy::dsl::coalesce(&[primary.cast(dtype.clone()), others.cast(dtype)]);
    // as in text_date_expr: a guessed %Y also takes 2-digit years
    Ok(Some(
        when(parsed.clone().dt().year().lt(lit(1000)))
            .then(lit(NULL))
            .otherwise(parsed)
            .alias(name),
    ))
}

/// Whether a chrono/strptime format has any time-of-day field, skipping `%%` literals and the
/// padding/width modifiers (`%-H`, `%_M`, `%3f`, ...).
fn format_has_time(fmt: &str) -> bool {
    let mut chars = fmt.chars();
    while let Some(c) = chars.next() {
        if c != '%' {
            continue;
        }
        let mut spec = chars.next();
        while let Some(m) = spec
            && (matches!(m, '-' | '_' | '0' | '^' | '#' | '.' | ':') || m.is_ascii_digit())
        {
            spec = chars.next();
        }
        if matches!(
            spec,
            Some(
                'H' | 'I'
                    | 'k'
                    | 'l'
                    | 'M'
                    | 'S'
                    | 'T'
                    | 'R'
                    | 'r'
                    | 'c'
                    | 's'
                    | 'f'
                    | 'p'
                    | 'P'
                    | 'X'
                    | '+'
            )
        ) {
            return true;
        }
    }
    false
}

fn explain(
    err: PolarsError,
    sorted: bool,
    date_format_given: bool,
    text_index: Option<&(LazyFrame, String, Expr)>,
) -> crate::clitypes::CliError {
    let msg = err.to_string();
    if msg.contains("not sorted") {
        if sorted {
            return crate::clitypes::CliError::Other(format!(
                "The input is not sorted by --index (within each --by group). Drop --sorted to \
                 let timebucketp sort it. ({msg})"
            ));
        }
    } else if msg.contains("could not find an appropriate format") {
        return crate::clitypes::CliError::Other(format!(
            "Could not parse the --index column as dates. Set its format with --date-format, e.g. \
             --date-format %m/%d/%Y. ({msg})"
        ));
    } else if let Some(pos) = msg.find("conversion from `str` to") {
        // keep Polars' first line (which values failed), not its nested expression context
        let detail = msg[pos..].lines().next().unwrap_or_default();
        // Polars lists the failing values as `["v1", "v2"]`
        let datetime_value = detail
            .starts_with("conversion from `str` to `date`")
            .then(|| detail.split_once("[\"")?.1.split('"').next())
            .flatten()
            .filter(|v| {
                looks_like_datetime(v)
                    || index_date_format(v, false).is_some_and(|f| format_has_time(&f))
            });
        if let Some(value) = datetime_value.filter(|_| date_format_given) {
            return crate::clitypes::CliError::Other(format!(
                "The --index column mixes dates and datetimes (e.g. \"{value}\"), which a \
                 --date-format without time fields cannot parse. Drop --date-format to read the \
                 dates as midnight, or make every value a full datetime, e.g. with `qsv datefmt \
                 <col> --keep-zero-time`. ({detail})"
            ));
        }
        return crate::clitypes::CliError::Other(format!(
            "Some values do not match their date format: {detail}. Day-first dates need \
             QSV_PREFER_DMY, or set the index format with --date-format."
        ));
    } else if msg.contains("null values in") {
        let empty = format!(
            "The --index column has empty values. Remove those rows first, e.g. with `qsv search \
             --select <index> --invert-match '^$'`. ({msg})"
        );
        // A text index is null where a cell is empty or the year check rejected the date;
        // name the first such value. Only runs on this failure path.
        let Some((raw, index, parsed)) = text_index else {
            return crate::clitypes::CliError::Other(empty);
        };
        let first_null = raw
            .clone()
            .filter(parsed.clone().is_null())
            .select([col(index.as_str())])
            .limit(1)
            .collect()
            .ok()
            .and_then(|df| {
                df.column(index.as_str())
                    .ok()
                    .map(|c| c.str().ok().map(|s| s.get(0).map(str::to_string)))
            })
            .flatten();
        return crate::clitypes::CliError::Other(match first_null {
            Some(Some(value)) => format!(
                "The --index value \"{value}\" has a year before 1000. 2-digit years need \
                 --date-format, e.g. --date-format %m/%d/%y."
            ),
            _ => empty,
        });
    }
    err.into()
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    bucket(&args, false)
}

/// Whether a failed run should be retried with `mixed_index_expr`: a guessed text index whose
/// strict parse failed. Checking for a mixed index up front would cost a pass over the index
/// on every run; this costs one failed attempt, and only on input that used to be an error.
fn retry_mixed(err: &PolarsError, guessed_text_index: bool) -> bool {
    guessed_text_index && err.to_string().contains("conversion from `str` to")
}

fn bucket(args: &Args, try_mixed: bool) -> CliResult<()> {
    let Some(index) = args.flag_index.clone() else {
        return fail_incorrectusage_clierror!("--index <col> is required.");
    };
    let by_cols: Vec<String> = args
        .flag_by
        .as_deref()
        .map(|s| {
            s.split(',')
                .map(|c| c.trim().to_string())
                .filter(|c| !c.is_empty())
                .collect()
        })
        .unwrap_or_default();
    if by_cols.contains(&index) {
        return fail_incorrectusage_clierror!("--by cannot include the --index column.");
    }
    let aggs = parse_aggs(&args.flag_agg)?;
    let boundary_cols = if args.flag_boundaries {
        &["_lower_boundary", "_upper_boundary"][..]
    } else {
        &[]
    };
    for a in &aggs {
        if a.output == index
            || by_cols.contains(&a.output)
            || boundary_cols.contains(&a.output.as_str())
        {
            return fail_incorrectusage_clierror!(
                "--agg output \"{}\" clashes with an input or boundary column of the same name. \
                 Rename that column first, e.g. with `qsv rename`.",
                a.output
            );
        }
    }

    // Validate the mode-specific options before reading any data.
    if args.flag_rolling {
        for (given, flag) in [
            (args.flag_every.is_some(), "--every"),
            (args.flag_label.is_some(), "--label"),
            (args.flag_start_by.is_some(), "--start-by"),
            (args.flag_boundaries, "--boundaries"),
            (args.flag_fill_gaps, "--fill-gaps"),
        ] {
            if given {
                return fail_incorrectusage_clierror!("{flag} does not apply to --rolling.");
            }
        }
        if args.flag_period.is_none() {
            return fail_incorrectusage_clierror!("--rolling requires --period <dur>.");
        }
    } else if args.flag_every.is_none() {
        return fail_incorrectusage_clierror!(
            "--every <dur> is required (or use --rolling with --period)."
        );
    }

    let label = args
        .flag_label
        .as_deref()
        .map_or(Ok(Label::Left), parse_label)?;
    let start_by = args
        .flag_start_by
        .as_deref()
        .map_or(Ok(StartBy::WindowBound), parse_start_by)?;
    if args.flag_fill_gaps {
        // Inserted windows are labelled on the --every grid, which only matches labels that sit
        // on it, and carry no boundaries.
        // Window ends (--label right) are not evenly spaced when --period differs from --every
        // or the duration is calendar-based, so only the starts form a grid to fill.
        if label != Label::Left {
            return fail_incorrectusage_clierror!(
                "--fill-gaps requires --label left (the default)."
            );
        }
        if start_by == StartBy::DataPoint {
            return fail_incorrectusage_clierror!(
                "--fill-gaps cannot be combined with --start-by datapoint."
            );
        }
        if args.flag_boundaries {
            return fail_incorrectusage_clierror!(
                "--fill-gaps cannot be combined with --boundaries."
            );
        }
    }

    let every = args
        .flag_every
        .as_deref()
        .map(|v| parse_duration("--every", v))
        .transpose()?;
    let period_str = args
        .flag_period
        .clone()
        .or_else(|| args.flag_every.clone())
        .unwrap_or_default();
    let period = parse_duration("--period", &period_str)?;
    if period.negative() || period.is_zero() {
        return fail_incorrectusage_clierror!("--period must be positive.");
    }
    if let Some(e) = every
        && (e.negative() || e.is_zero())
    {
        return fail_incorrectusage_clierror!("--every must be positive.");
    }
    let offset = match args.flag_offset.as_deref() {
        Some(v) => Some(parse_duration("--offset", v)?),
        // a trailing window, as in Polars' Python rolling()
        None if args.flag_rolling => Some(parse_duration("--period", &format!("-{period_str}"))?),
        // zero, in the index's units: resolved once the schema is known
        None => None,
    };
    let closed = match args.flag_closed.as_deref() {
        Some(v) => parse_closed(v)?,
        None if args.flag_rolling => ClosedWindow::Right,
        None => ClosedWindow::Left,
    };

    // Delimiters, .sz inputs, comment lines, preambles and pschemas are handled as in pivotp.
    let fallback_delim = if let Some(delimiter) = args.flag_delimiter {
        delimiter.as_byte()
    } else if let Ok(delim) = std::env::var("QSV_DEFAULT_DELIMITER") {
        Delimiter::decode_delimiter(&delim)?.as_byte()
    } else {
        b','
    };
    let delim = path_delim(&args.arg_input, args.flag_delimiter, fallback_delim);
    let output_delim = args.flag_output.as_ref().map_or(fallback_delim, |out| {
        path_delim(out, args.flag_delimiter, fallback_delim)
    });
    if args.flag_decimal_comma && delim == b',' {
        return fail_incorrectusage_clierror!(
            "You need to specify an alternate --delimiter when using --decimal-comma."
        );
    }

    let snappy_tmpdir;
    let reader_path = if Path::new(&args.arg_input)
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("sz"))
    {
        snappy_tmpdir = tempfile::tempdir()?;
        util::decompress_snappy_file(&PathBuf::from(&args.arg_input), &snappy_tmpdir)?
    } else {
        args.arg_input.clone()
    };

    let comment_prefix = std::env::var("QSV_COMMENT_CHAR")
        .ok()
        .map(PlSmallStr::from_string);
    let mut csv_reader = LazyCsvReader::new(PlRefPath::new(&reader_path))
        .with_skip_lines(crate::config::sniffed_preamble_lines(Path::new(
            &reader_path,
        )))
        .with_has_header(true)
        .with_missing_is_null(true)
        .with_comment_prefix(comment_prefix)
        .with_try_parse_dates(false)
        .with_decimal_comma(args.flag_decimal_comma)
        .with_separator(delim)
        .with_ignore_errors(args.flag_ignore_errors);
    let pschema = if args.flag_infer_len == DEFAULT_INFER_LEN {
        util::load_valid_pschema(Path::new(&args.arg_input))?
    } else {
        None
    };
    if let Some(schema) = pschema {
        let mut schema = (*schema).clone();
        if args.flag_date_format.is_some()
            && let Some(dt) = schema.get_mut(index.as_str())
        {
            *dt = DataType::String;
        }
        csv_reader = csv_reader.with_schema(Some(Arc::new(schema)));
    } else {
        // With date guessing off, a date index is read as text anyway. --date-format also
        // needs digit-only dates (e.g. %Y%m%d) kept as text instead of read as integers.
        if args.flag_date_format.is_some() {
            let mut overwrite = Schema::default();
            overwrite.insert(index.as_str().into(), DataType::String);
            csv_reader = csv_reader.with_dtype_overwrite(Some(Arc::new(overwrite)));
        }
        csv_reader = csv_reader.with_infer_schema_length(if args.flag_infer_len == 0 {
            None
        } else {
            Some(args.flag_infer_len)
        });
    }
    let mut lf = csv_reader.finish()?;

    // Polars windows need a Date, Datetime or Int32/Int64 index.
    let schema = lf.collect_schema()?;
    let Some(index_dtype) = schema.get(index.as_str()).cloned() else {
        return fail_incorrectusage_clierror!("--index column \"{index}\" not found.");
    };
    for b in &by_cols {
        if schema.get(b.as_str()).is_none() {
            return fail_incorrectusage_clierror!("--by column \"{b}\" not found.");
        }
    }
    for c in aggs.iter().filter_map(|a| a.column.as_deref()) {
        if schema.get(c).is_none() {
            return fail_incorrectusage_clierror!("--agg column \"{c}\" not found.");
        }
    }
    // A header-only input has nothing to bucket, and its untyped (text) columns would fail the
    // date parsing and the aggregations: write the output header only.
    if lf
        .clone()
        .select([col(index.as_str())])
        .limit(1)
        .collect()?
        .height()
        == 0
    {
        let names = by_cols
            .iter()
            .cloned()
            .chain(
                args.flag_boundaries
                    .then(|| ["_lower_boundary".to_string(), "_upper_boundary".to_string()])
                    .into_iter()
                    .flatten(),
            )
            .chain(std::iter::once(index.clone()))
            .chain(aggs.iter().map(|a| a.output.clone()));
        let mut empty = DataFrame::new_infer_height(
            names
                .map(|n| Column::new_empty(n.as_str().into(), &DataType::String))
                .collect(),
        )?;
        return write_output(&mut empty, args, output_delim);
    }

    // Polars rejects an integer-index duration (e.g. 0i) on a temporal index, and vice versa.
    let offset = offset.unwrap_or_else(|| {
        Duration::parse(if index_dtype.is_integer() {
            "0i"
        } else {
            "0ns"
        })
    });
    // Text datetimes parse to microseconds; a window finer than that would select nothing
    let nanos = every
        .iter()
        .chain([&period, &offset])
        .any(|d| d.nanoseconds() % 1000 != 0);
    let prefer_dmy = util::get_envvar_flag("QSV_PREFER_DMY");
    // the unparsed input and the index parse, to name a bad value if the windows fail
    let mut text_index: Option<(LazyFrame, String, Expr)> = None;
    let mut retryable = false;
    match index_dtype {
        DataType::Datetime(unit, tz) if nanos && unit != TimeUnit::Nanoseconds => {
            lf = lf.with_column(
                col(index.as_str()).cast(DataType::Datetime(TimeUnit::Nanoseconds, tz.clone())),
            );
        },
        DataType::Date | DataType::Datetime(..) => {},
        // pschemas may type small integers as (U)Int8/16
        dt if dt.is_integer() => {
            lf = lf.with_column(col(index.as_str()).cast(DataType::Int64));
        },
        DataType::String => {
            // pick the format from the first index value, so ambiguous dates follow qsv's
            // month-first default (or QSV_PREFER_DMY) instead of Polars' day-first guess
            let first = first_text_value(&lf, &index)?;
            let format = args.flag_date_format.clone().or_else(|| {
                first
                    .as_deref()
                    .and_then(|v| index_date_format(v, prefer_dmy))
            });
            let guessed = args.flag_date_format.is_none();
            let mixed = match first.as_deref() {
                Some(v) if guessed && try_mixed => {
                    mixed_index_expr(&lf, &index, format.as_deref(), v, prefer_dmy, nanos)?
                },
                _ => None,
            };
            let parsed = mixed.unwrap_or_else(|| {
                text_date_expr(
                    &index,
                    format.as_deref(),
                    first.as_deref(),
                    true,
                    guessed,
                    nanos,
                )
            });
            text_index = Some((lf.clone(), index.clone(), parsed.clone()));
            lf = lf.with_column(parsed);
            retryable = guessed && !try_mixed;
        },
        dt => {
            return fail_incorrectusage_clierror!(
                "--index column \"{index}\" must hold dates, datetimes or integers, not {dt}."
            );
        },
    }

    // Text date columns compared by min/max/mean/median/quantile are parsed the same way as
    // the index (month-first unless QSV_PREFER_DMY); compared as text, "12/31/2023" would
    // rank above "01/05/2024". first/last keep the values as written.
    let mut parsed_cols: Vec<(String, Expr)> = Vec::new();
    for a in &aggs {
        let Some(c) = a.column.as_deref() else {
            continue;
        };
        if c == index
            || by_cols.iter().any(|b| b == c)
            || parsed_cols.iter().any(|(p, _)| p == c)
            || !a.kind.compares_values()
            || schema.get(c) != Some(&DataType::String)
        {
            continue;
        }
        let Some(first) = first_text_value(&lf, c)? else {
            continue;
        };
        let format = index_date_format(&first, prefer_dmy);
        if format.is_none() && !looks_like_datetime(&first) {
            continue;
        }
        // only a column whose every value parses is dates; mixed text keeps text comparison
        let parsed = text_date_expr(c, format.as_deref(), Some(&first), false, true, false);
        let unparsed = lf
            .clone()
            .select([col(c)
                .is_not_null()
                .and(parsed.clone().is_null())
                .any(true)
                .alias("unparsed")])
            .collect()?;
        if unparsed.column("unparsed")?.bool()?.get(0) != Some(true) {
            parsed_cols.push((c.to_string(), parsed));
        }
    }

    if args.flag_sorted {
        // Polars' lazy group_by_dynamic/rolling does not check that the index is sorted when no
        // --by is given, and silently builds wrong windows from unsorted input. A single pass
        // looking for a value smaller than its predecessor (within each --by group) is still
        // far cheaper than sorting.
        let idx = col(index.as_str());
        let descending_step = idx.clone().lt(idx.shift(lit(1)));
        let check = if by_cols.is_empty() {
            descending_step
        } else {
            descending_step.over(by_cols.iter().map(|c| col(c.as_str())).collect::<Vec<_>>())?
        };
        let unsorted = match lf
            .clone()
            .select([check.any(true).alias("unsorted")])
            .collect()
        {
            Err(e) if retry_mixed(&e, retryable) => return bucket(args, true),
            r => r.map_err(|e| {
                explain(
                    e,
                    true,
                    args.flag_date_format.is_some(),
                    text_index.as_ref(),
                )
            })?,
        };
        if unsorted.column("unsorted")?.bool()?.get(0) == Some(true) {
            return fail_clierror!(
                "--sorted was given, but the input is not sorted by --index{}. Drop --sorted to \
                 let timebucketp sort it.",
                if by_cols.is_empty() {
                    ""
                } else {
                    " within each --by group"
                }
            );
        }
    } else {
        let sort_cols: Vec<&str> = by_cols
            .iter()
            .map(String::as_str)
            .chain(std::iter::once(index.as_str()))
            .collect();
        lf = lf.sort(
            sort_cols,
            SortMultipleOptions::default().with_maintain_order(true),
        );
    }

    let by_exprs: Vec<Expr> = by_cols.iter().map(|c| col(c.as_str())).collect();
    // a parsed date column feeds only the aggregations that compare values; first/last and
    // the group keys keep the values as written
    let agg_exprs: Vec<Expr> = aggs
        .iter()
        .map(|a| {
            match parsed_cols
                .iter()
                .find(|(c, _)| a.kind.compares_values() && a.column.as_deref() == Some(c))
            {
                Some((_, parsed)) => a.kind.on(parsed.clone()).alias(a.output.as_str()),
                None => a.expr(),
            }
        })
        .collect();
    let grouped = if args.flag_rolling {
        lf.rolling(
            col(index.as_str()),
            by_exprs,
            RollingGroupOptions {
                index_column: index.as_str().into(),
                period,
                offset,
                closed_window: closed,
            },
        )
    } else {
        lf.group_by_dynamic(
            col(index.as_str()),
            by_exprs,
            DynamicGroupOptions {
                index_column: index.as_str().into(),
                every: every.unwrap_or(period),
                period,
                offset,
                label,
                include_boundaries: args.flag_boundaries,
                closed_window: closed,
                start_by,
            },
        )
    };
    let mut result = match grouped.agg(agg_exprs).collect() {
        Err(e) if retry_mixed(&e, retryable) => return bucket(args, true),
        r => r.map_err(|e| {
            explain(
                e,
                args.flag_sorted,
                args.flag_date_format.is_some(),
                text_index.as_ref(),
            )
        })?,
    };

    if args.flag_fill_gaps
        && let Some(every) = every
    {
        let by: Vec<PlSmallStr> = by_cols.iter().map(|c| c.as_str().into()).collect();
        result = result
            .sort(
                by_cols
                    .iter()
                    .map(String::as_str)
                    .chain(std::iter::once(index.as_str()))
                    .collect::<Vec<_>>(),
                SortMultipleOptions::default(),
            )?
            .upsample_stable(by, index.as_str(), every)?;
        let fills: Vec<Expr> = aggs
            .iter()
            .filter(|a| a.kind.empty_cell_is_zero())
            .map(|a| {
                let dtype = result.schema().get(a.output.as_str()).cloned();
                let zero = lit(0);
                col(a.output.as_str()).fill_null(match dtype {
                    Some(dt) => zero.cast(dt),
                    None => zero,
                })
            })
            .collect();
        if !fills.is_empty() {
            result = result.lazy().with_columns(fills).collect()?;
        }
    }

    // Group order is not guaranteed, so sort the output by --by, then --index.
    if !by_cols.is_empty() {
        result = result.sort(
            by_cols
                .iter()
                .map(String::as_str)
                .chain(std::iter::once(index.as_str()))
                .collect::<Vec<_>>(),
            SortMultipleOptions::default().with_maintain_order(true),
        )?;
    }

    write_output(&mut result, args, output_delim)
}

fn write_output(result: &mut DataFrame, args: &Args, output_delim: u8) -> CliResult<()> {
    let mut writer = match args.flag_output {
        Some(ref output_file) => Box::new(File::create(Path::new(output_file))?) as Box<dyn Write>,
        None => Box::new(io::stdout()) as Box<dyn Write>,
    };
    let float_precision = std::env::var("QSV_POLARS_FLOAT_PRECISION")
        .ok()
        .and_then(|s| s.parse().ok());
    CsvWriter::new(&mut writer)
        .include_header(true)
        .include_bom(util::get_envvar_flag("QSV_OUTPUT_BOM"))
        .with_float_precision(float_precision)
        .with_datetime_format(Some(PlSmallStr::from_static("%Y-%m-%d %H:%M:%S%.f")))
        .with_separator(output_delim)
        .finish(result)?;
    drop(writer);

    compress_output_if_needed(args.flag_output.clone())?;

    if !args.flag_quiet {
        eprintln!("{:?}", result.shape());
    }

    Ok(())
}
