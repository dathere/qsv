static USAGE: &str = r#"
Formats recognized date fields (19 formats recognized) to a specified date format
using strftime date format specifiers.

For recognized date formats, see
https://github.com/dathere/qsv-dateparser?tab=readme-ov-file#accepted-date-formats

See https://docs.rs/chrono/latest/chrono/format/strftime/ for 
accepted date format specifiers for --formatstr.
Defaults to ISO 8601/RFC 3339 format when --formatstr is not specified.
( "%Y-%m-%dT%H:%M:%S%z" - e.g. 2001-07-08T00:34:60.026490+09:30 )

Examples:

  # Format dates in Open Date column to ISO 8601/RFC 3339 format:
  qsv datefmt 'Open Date' file.csv

  # Format multiple date columns in file.csv to ISO 8601/RFC 3339 format:
  qsv datefmt 'Open Date,Modified Date,Closed Date' file.csv

  # Format all columns that end with "_date" case-insensitive in file.csv to ISO 8601/RFC 3339 format:
  qsv datefmt '/(?i)_date$/' file.csv

  # Format dates in OpenDate column using '%Y-%m-%d' format:
  qsv datefmt OpenDate --formatstr '%Y-%m-%d' file.csv

  # Format multiple date columns using '%Y-%m-%d' format:
  qsv datefmt OpenDate,CloseDate,ReopenDate --formatstr '%Y-%m-%d' file.csv

  # Get the week number for OpenDate and store it in the week_number column:
  qsv datefmt OpenDate --formatstr '%V' --new-column week_number file.csv

  # Get the day of the week for several date columns and store it in the corresponding weekday columns:
  qsv datefmt OpenDate,CloseDate --formatstr '%u' --rename Open_weekday,Close_weekday file.csv

For more extensive examples, see https://github.com/dathere/qsv/blob/master/tests/test_datefmt.rs.

Usage:
qsv datefmt [--formatstr=<string>] [options] <column> [<input>]
qsv datefmt --help

datefmt arguments:
    <column>                    The column/s to apply the date formats to.
                                Note that the <column> argument supports multiple columns.
                                See 'qsv select --help' for the format details.

    --formatstr=<string>        The date format to use for the datefmt operation.
                                The date format to use. For formats, see
                                https://docs.rs/chrono/latest/chrono/format/strftime/
                                Default to ISO 8601 / RFC 3339 date & time format -
                                "%Y-%m-%dT%H:%M:%S%z" - e.g. 2001-07-08T00:34:60.026490+09:30
                                [default: %+]
        
    <input>                     The input file to read from. If not specified, reads from stdin.

datefmt options:
    -c, --new-column <name>     Put the transformed values in new column(s) instead of replacing
                                the source column(s). When the selection has multiple columns,
                                pass a comma-separated list of new column names that match the
                                selection count (e.g. --new-column 'open_iso,close_iso' for
                                'OpenDate,CloseDate'). To rename in place instead, use --rename.
    -r, --rename <name>         New name for the transformed column.
    --prefer-dmy                Prefer to parse dates in dmy format. Otherwise, use mdy format.
    --keep-zero-time            If a formatted date ends with "T00:00:00+00:00", keep the time
                                instead of removing it.
    --input-tz=<string>         The timezone to use for the input date if the date does not have
                                timezone specified. The timezone must be a valid IANA timezone name or
                                the string "local" for the local timezone.
                                See https://en.wikipedia.org/wiki/List_of_tz_database_time_zones
                                for a list of valid timezone names.
                                [default: UTC]
    --output-tz=<string>        The timezone to use for the output date.
                                The timezone must be a valid IANA timezone name or the string "local".
                                [default: UTC]
    --default-tz=<string>       Fallback timezone consulted only when --input-tz or --output-tz
                                is set to "local" but local-timezone detection fails. Defaults
                                to UTC. Does NOT override the --input-tz / --output-tz defaults —
                                use --utc to force both input and output to UTC.
                                The timezone must be a valid IANA timezone name or the string "local".
    --utc                       Shortcut for --input-tz and --output-tz set to UTC.
    --zulu                      Shortcut for --output-tz set to UTC and --formatstr set to "%Y-%m-%dT%H:%M:%SZ".
    -R, --ts-resolution <res>   The resolution to use when parsing Unix timestamps.
                                Valid values are "sec", "milli", "micro", "nano".
                                [default: sec]
    -j, --jobs <arg>            The number of jobs to run in parallel.
                                When not set, the number of jobs is set to the number of CPUs detected.
    -b, --batch <size>          The number of rows per batch to load into memory, before running in parallel.
                                Automatically determined for CSV files with more than 50000 rows.
                                Set to 0 to load all rows in one batch. Set to 1 to force batch optimization
                                even for files with less than 50000 rows.
                                [default: 50000]

Common options:
    -h, --help                  Display this message
    -o, --output <file>         Write output to <file> instead of stdout.
    -n, --no-headers            When set, the first row will not be interpreted
                                as headers.
    -d, --delimiter <arg>       The field delimiter for reading CSV data.
                                Must be a single character. (default: ,)
    -p, --progressbar           Show progress bars. Not valid for stdin.
"#;

use std::str::FromStr;

use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::Tz;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
use indicatif::{ProgressBar, ProgressDrawTarget};
use qsv_dateparser::parse_with_preference_and_timezone;
use rayon::{
    iter::{IndexedParallelIterator, ParallelIterator},
    prelude::IntoParallelRefIterator,
};
use serde::Deserialize;

use crate::{
    CliResult,
    config::{Config, Delimiter},
    select::SelectColumns,
    util,
    util::replace_column_value,
};

#[allow(dead_code)]
#[derive(Deserialize)]
struct Args {
    arg_column:          SelectColumns,
    arg_input:           Option<String>,
    flag_rename:         Option<String>,
    flag_prefer_dmy:     bool,
    flag_keep_zero_time: bool,
    flag_ts_resolution:  String,
    flag_formatstr:      String,
    flag_input_tz:       String,
    flag_output_tz:      String,
    flag_default_tz:     Option<String>,
    flag_utc:            bool,
    flag_zulu:           bool,
    flag_batch:          usize,
    flag_jobs:           Option<usize>,
    flag_new_column:     Option<String>,
    flag_output:         Option<String>,
    flag_no_headers:     bool,
    flag_delimiter:      Option<Delimiter>,
    flag_progressbar:    bool,
}

#[derive(Default, Clone, Copy)]
enum TimestampResolution {
    #[default]
    Second,
    Millisecond,
    Microsecond,
    Nanosecond,
}

impl FromStr for TimestampResolution {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "sec" => Ok(TimestampResolution::Second),
            "milli" => Ok(TimestampResolution::Millisecond),
            "micro" => Ok(TimestampResolution::Microsecond),
            "nano" => Ok(TimestampResolution::Nanosecond),
            _ => Err(format!("Invalid timestamp resolution: {s}")),
        }
    }
}

// Default value for --formatstr. Must stay in sync with the `[default: %+]` literal
// in the USAGE string above; the --zulu conflict check at the bottom of `run()` uses
// this constant to detect "no explicit --formatstr passed".
const DEFAULT_FORMATSTR: &str = "%+";

#[inline]
fn unix_timestamp(input: &str, resolution: TimestampResolution) -> Option<DateTime<Utc>> {
    // atoi_simd::parse::<i64, NEG=false, CHK=false>: rejects negative integers, skips overflow
    // checks. We intentionally reject negatives in the fast path so columns of arbitrary
    // signed integers aren't unconditionally interpreted as pre-1970 unix timestamps.
    // Negative timestamps are still recognized at the call site by qsv-dateparser's
    // numeric-string handling, so legitimate pre-1970 inputs (e.g. "-770172300") still parse.
    let Ok(ts_input_val) = atoi_simd::parse::<i64, false, false>(input.as_bytes()) else {
        return None;
    };

    // these constructors already return DateTime<Utc>, so no with_timezone(&Utc) is needed.
    match resolution {
        TimestampResolution::Second => Utc.timestamp_opt(ts_input_val, 0).single(),
        TimestampResolution::Millisecond => Utc.timestamp_millis_opt(ts_input_val).single(),
        TimestampResolution::Microsecond => Utc.timestamp_micros(ts_input_val).single(),
        TimestampResolution::Nanosecond => Some(Utc.timestamp_nanos(ts_input_val)),
    }
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let rconfig = Config::new(args.arg_input.as_ref())
        .delimiter(args.flag_delimiter)
        .no_headers_flag(args.flag_no_headers)
        .select(args.arg_column);

    let mut rdr = rconfig.reader()?;
    let mut wtr = Config::new(args.flag_output.as_ref()).writer()?;

    let headers = rdr.byte_headers()?.clone();
    let sel = rconfig.selection(&headers)?;

    let tsres = args.flag_ts_resolution.parse::<TimestampResolution>()?;

    let mut headers = rdr.headers()?.clone();

    if let Some(new_name) = args.flag_rename {
        let new_col_names = util::ColumnNameParser::new(&new_name).parse()?;
        if new_col_names.len() != sel.len() {
            return fail_incorrectusage_clierror!(
                "Number of new columns does not match input column selection."
            );
        }
        for (i, col_index) in sel.iter().enumerate() {
            headers = replace_column_value(&headers, *col_index, &new_col_names[i]);
        }
    }

    // Parse --new-column up-front so its name count is validated against the selection
    // even when --no-headers is set. ColumnNameParser supports quoted/comma-separated
    // names just like --rename.
    let new_col_names: Option<Vec<String>> = if let Some(new_column) = &args.flag_new_column {
        let names = util::ColumnNameParser::new(new_column).parse()?;
        if names.len() != sel.len() {
            return fail_incorrectusage_clierror!(
                "Number of --new-column names ({}) does not match the selected column count ({}). \
                 Pass a comma-separated list of names that matches the selection (use --rename to \
                 rename in place).",
                names.len(),
                sel.len()
            );
        }
        Some(names)
    } else {
        None
    };

    if !rconfig.no_headers {
        if let Some(ref names) = new_col_names {
            for name in names {
                headers.push_field(name);
            }
        }
        wtr.write_record(&headers)?;
    }

    let mut flag_formatstr = args.flag_formatstr;
    let flag_new_column = args.flag_new_column;

    // prep progress bar
    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    let show_progress =
        (args.flag_progressbar || util::get_envvar_flag("QSV_PROGRESSBAR")) && !rconfig.is_stdin();

    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    let progress = ProgressBar::with_draw_target(None, ProgressDrawTarget::stderr_with_hz(5));

    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    if show_progress {
        util::prep_progress(&progress, util::count_rows(&rconfig)?);
    } else {
        progress.set_draw_target(ProgressDrawTarget::hidden());
    }

    let prefer_dmy = args.flag_prefer_dmy || rconfig.get_dmy_preference();
    let keep_zero_time = args.flag_keep_zero_time;

    // amortize memory allocation by reusing record
    #[allow(unused_assignments)]
    let mut batch_record = csv::StringRecord::new();

    let num_jobs = util::njobs(args.flag_jobs);

    // reuse batch buffers
    let batchsize = util::optimal_batch_size(&rconfig, args.flag_batch, num_jobs);
    let mut batch = Vec::with_capacity(batchsize);
    let mut batch_results = Vec::with_capacity(batchsize);

    // set timezone variables
    let default_tz = match args.flag_default_tz.as_deref() {
        Some(tz) => {
            if tz.eq_ignore_ascii_case("local") {
                if let Ok(tz) = iana_time_zone::get_timezone() {
                    log::info!("default-tz local timezone: {tz}");
                    tz.parse::<Tz>()?
                } else {
                    log::warn!("default-tz local timezone {tz} not found. Defaulting to UTC.");
                    chrono_tz::UTC
                }
            } else {
                tz.parse::<Tz>()?
            }
        },
        None => chrono_tz::UTC,
    };

    let mut input_tz = if args.flag_input_tz.eq_ignore_ascii_case("local") {
        if let Ok(tz) = iana_time_zone::get_timezone() {
            log::info!("input-tz local timezone: {tz}");
            tz.parse::<Tz>()?
        } else {
            log::warn!("input-tz local timezone not found. Falling back to default-tz.");
            default_tz
        }
    } else {
        args.flag_input_tz.parse::<Tz>().map_err(|_| {
            format!(
                "Invalid --input-tz: {}. Must be a valid IANA timezone name or \"local\".",
                args.flag_input_tz
            )
        })?
    };
    #[allow(clippy::useless_let_if_seq)] // more readable this way
    let mut output_tz = if args.flag_output_tz.eq_ignore_ascii_case("local") {
        if let Ok(tz) = iana_time_zone::get_timezone() {
            log::info!("output-tz local timezone: {tz}");
            tz.parse::<Tz>()?
        } else {
            log::warn!("output-tz local timezone not found. Falling back to default-tz.");
            default_tz
        }
    } else {
        args.flag_output_tz.parse::<Tz>().map_err(|_| {
            format!(
                "Invalid --output-tz: {}. Must be a valid IANA timezone name or \"local\".",
                args.flag_output_tz
            )
        })?
    };

    // --utc / --zulu are shortcuts that force specific tz/format values, so reject
    // explicit overrides that would silently lose. Defaults are "UTC" for the tz flags
    // and "%+" for --formatstr (set in USAGE above).
    if args.flag_utc {
        if !args.flag_input_tz.eq_ignore_ascii_case("UTC") {
            return fail_incorrectusage_clierror!(
                "--utc cannot be combined with --input-tz={}; --utc forces input timezone to UTC.",
                args.flag_input_tz
            );
        }
        if !args.flag_output_tz.eq_ignore_ascii_case("UTC") {
            return fail_incorrectusage_clierror!(
                "--utc cannot be combined with --output-tz={}; --utc forces output timezone to \
                 UTC.",
                args.flag_output_tz
            );
        }
        if let Some(ref dtz) = args.flag_default_tz
            && !dtz.eq_ignore_ascii_case("UTC")
        {
            return fail_incorrectusage_clierror!(
                "--utc cannot be combined with --default-tz={dtz}; --utc forces both input and \
                 output timezones to UTC.",
            );
        }
        input_tz = chrono_tz::UTC;
        output_tz = chrono_tz::UTC;
    }
    if args.flag_zulu {
        if !args.flag_output_tz.eq_ignore_ascii_case("UTC") {
            return fail_incorrectusage_clierror!(
                "--zulu cannot be combined with --output-tz={}; --zulu forces output timezone to \
                 UTC.",
                args.flag_output_tz
            );
        }
        if flag_formatstr != DEFAULT_FORMATSTR {
            return fail_incorrectusage_clierror!(
                "--zulu cannot be combined with --formatstr={flag_formatstr}; --zulu forces the \
                 output format.",
            );
        }
        output_tz = chrono_tz::UTC;
        flag_formatstr = "%Y-%m-%dT%H:%M:%SZ".to_string();
    }

    let is_output_utc = output_tz == chrono_tz::UTC;
    let new_column = flag_new_column.is_some();

    // main loop to read CSV and construct batches for parallel processing.
    // each batch is processed via Rayon parallel iterator.
    // loop exits when batch is empty.
    'batch_loop: loop {
        for _ in 0..batchsize {
            match rdr.read_record(&mut batch_record) {
                Ok(true) => batch.push(std::mem::take(&mut batch_record)),
                Ok(false) => break, // nothing else to add to batch
                Err(e) => {
                    return fail_clierror!("Error reading file: {e}");
                },
            }
        }

        if batch.is_empty() {
            // break out of infinite loop when at EOF
            break 'batch_loop;
        }

        // do actual datefmt via Rayon parallel iterator.
        //
        // For each row we:
        //   1. transform each selected cell into a Vec<Option<String>> indexed by column,
        //   2. emit the output StringRecord in a single pass — replace mode rebuilds the record
        //      once (instead of k times via replace_column_value), and new-column mode appends the
        //      transformed values after cloning the original record.
        batch
            .par_iter()
            .map(|record_item| {
                let mut cell = String::new();
                let mut transformed: Vec<Option<String>> = vec![None; record_item.len()];

                for col_index in &*sel {
                    record_item[*col_index].clone_into(&mut cell);
                    if !cell.is_empty() {
                        let parsed_date = if let Some(ts) = unix_timestamp(&cell, tsres) {
                            Ok(ts)
                        } else {
                            parse_with_preference_and_timezone(&cell, prefer_dmy, &input_tz)
                        };
                        if let Ok(format_date) = parsed_date {
                            // skip with_timezone() if output_tz is already UTC,
                            // as format_date is already in UTC
                            let formatted_date = if is_output_utc {
                                format_date.format(&flag_formatstr).to_string()
                            } else {
                                format_date
                                    .with_timezone(&output_tz)
                                    .format(&flag_formatstr)
                                    .to_string()
                            };
                            // Strip the time component when the formatted output is exactly
                            // midnight UTC. Both "+00:00" (default ISO format) and "Z"
                            // (--zulu) render midnight as a no-op time, so collapse to the
                            // YYYY-MM-DD date portion. --keep-zero-time disables this.
                            if !keep_zero_time
                                && (formatted_date.ends_with("T00:00:00+00:00")
                                    || formatted_date.ends_with("T00:00:00Z"))
                            {
                                formatted_date[..10].clone_into(&mut cell);
                            } else {
                                formatted_date.clone_into(&mut cell);
                            }
                        }
                    }
                    transformed[*col_index] = Some(std::mem::take(&mut cell));
                }

                if new_column {
                    let mut out = record_item.clone();
                    for col_index in &*sel {
                        let v = transformed[*col_index].as_deref().unwrap_or("");
                        out.push_field(v);
                    }
                    out
                } else {
                    let mut out = csv::StringRecord::new();
                    for (i, field) in record_item.iter().enumerate() {
                        match transformed[i].as_deref() {
                            Some(v) => out.push_field(v),
                            None => out.push_field(field),
                        }
                    }
                    out
                }
            })
            .collect_into_vec(&mut batch_results);

        // rayon collect() guarantees original order, so we can just append results each batch
        for result_record in &batch_results {
            wtr.write_record(result_record)?;
        }

        #[cfg(any(feature = "feature_capable", feature = "lite"))]
        if show_progress {
            progress.inc(batch.len() as u64);
        }

        batch.clear();
    } // end batch loop

    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    if show_progress {
        util::finish_progress(&progress);
    }
    Ok(wtr.flush()?)
}
