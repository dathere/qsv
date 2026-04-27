static USAGE: &str = r#"
Check if a CSV is sorted. The check is done on a streaming basis (i.e. constant memory).
With the --json options, also retrieve record count, sort breaks & duplicate count.

This command can be used in tandem with other qsv commands that sort or require sorted data
to ensure that they also work on a stream of data - i.e. without loading an entire CSV into memory.

For instance, a naive `dedup` requires loading the entire CSV into memory to sort it
first before deduping. However, if you know a CSV is sorted beforehand, you can invoke
`dedup` with the --sorted option, and it will skip loading entire CSV into memory to sort
it first. It will just immediately dedupe on a streaming basis.

`sort` also requires loading the entire CSV into memory. For very large CSV files that will
not fit in memory, `extsort` - a multi-threaded streaming sort that can work with arbitrarily
large files - can be used instead.

Use --numeric or --natural to verify the file matches the order produced by `sort --numeric`
or `sort --natural` before piping into a downstream command (e.g. `dedup --numeric --sorted`).
When multiple comparison flags are set, --natural takes precedence over --numeric, which takes
precedence over --ignore-case (matching `sort` and `dedup` semantics).

Simply put, sortcheck allows you to make informed choices on how to compose pipelines that
require sorted data.

Returns exit code 0 if a CSV is sorted, and exit code 1 otherwise.

Examples:

  # Check if file.csv is lexicographically sorted on all columns:
  qsv sortcheck file.csv

  # Check column "name" only, ignoring case:
  qsv sortcheck --select name --ignore-case file.csv

  # Verify file.csv is sorted numerically before piping into `dedup --numeric --sorted`:
  qsv sortcheck --numeric file.csv && qsv dedup --numeric --sorted file.csv

  # Check natural order (e.g. item1, item2, item10) and emit JSON stats:
  qsv sortcheck --natural --json file.csv

For examples, see https://github.com/dathere/qsv/blob/master/tests/test_sortcheck.rs.

Usage:
    qsv sortcheck [options] [<input>]
    qsv sortcheck --help

sort options:
    -s, --select <arg>      Select a subset of columns to check for sort.
                            See 'qsv select --help' for the format details.
    -N, --numeric           Compare according to string numerical value.
    --natural               Compare using natural sort order (e.g. item1 < item2 < item10).
                            Takes precedence over --numeric. Composes with --ignore-case.
    -i, --ignore-case       Compare strings disregarding case. Ignored when --numeric is set
                            (numeric comparison is case-insensitive by definition).
    --all                   Check all records. Do not stop/short-circuit the check
                            on the first unsorted record.
    --json                  Return results in JSON format, scanning --all records.
                            The JSON result has the following properties -
                            sorted (boolean), record_count (number),
                            unsorted_breaks (number) & dupe_count (number).
                            Unsorted breaks count the number of times two consecutive
                            rows are unsorted (i.e. n row > n+1 row).
                            Dupe count is the number of times two consecutive
                            rows are equal. Note that dupe count does not apply
                            if the file is not sorted and is set to -1.
    --pretty-json           Same as --json but in pretty JSON format.

Common options:
    -h, --help              Display this message
    -n, --no-headers        When set, the first row will not be interpreted
                            as headers. That is, it will be sorted with the rest
                            of the rows. Otherwise, the first row will always
                            appear as the header row in the output.
    -d, --delimiter <arg>   The field delimiter for reading CSV data.
                            Must be a single character. (default: ,)
    -p, --progressbar       Show progress bars. Not valid for stdin.
"#;

use std::cmp::Ordering;

use csv::ByteRecord;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
use indicatif::{HumanCount, ProgressBar, ProgressDrawTarget};
use serde::{Deserialize, Serialize};

use crate::{
    CliResult,
    cmd::sort::{
        iter_cmp, iter_cmp_ignore_case, iter_cmp_natural, iter_cmp_natural_ignore_case,
        iter_cmp_num,
    },
    config::{Config, Delimiter},
    select::SelectColumns,
    util,
};

#[derive(Deserialize)]
struct Args {
    arg_input:        Option<String>,
    flag_select:      SelectColumns,
    flag_numeric:     bool,
    flag_natural:     bool,
    flag_ignore_case: bool,
    flag_all:         bool,
    flag_no_headers:  bool,
    flag_delimiter:   Option<Delimiter>,
    flag_progressbar: bool,
    flag_json:        bool,
    flag_pretty_json: bool,
}

#[derive(Serialize, Deserialize)]
struct SortCheckStruct {
    sorted:          bool,
    record_count:    u64,
    unsorted_breaks: u64,
    dupe_count:      i64,
}

// Mirrors `SortMode` in `cmd/sort.rs` so sortcheck verifies the same ordering
// the user would get from `sort` / `dedup`.
enum ComparisonMode {
    Lex,
    LexIgnoreCase,
    Numeric,
    Natural,
    NaturalIgnoreCase,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    // Resolution order matches `sort` and `dedup`: --natural beats --numeric
    // beats --ignore-case. Done once before the loop so the dispatch `match`
    // monomorphizes to a single comparator per row.
    let compare_mode = if args.flag_natural {
        if args.flag_ignore_case {
            ComparisonMode::NaturalIgnoreCase
        } else {
            ComparisonMode::Natural
        }
    } else if args.flag_numeric {
        ComparisonMode::Numeric
    } else if args.flag_ignore_case {
        ComparisonMode::LexIgnoreCase
    } else {
        ComparisonMode::Lex
    };

    let rconfig = Config::new(args.arg_input.as_ref())
        .delimiter(args.flag_delimiter)
        .no_headers_flag(args.flag_no_headers)
        .select(args.flag_select);

    let mut rdr = rconfig.reader()?;

    let headers = rdr.byte_headers()?.clone();
    let sel = rconfig.selection(&headers)?;
    let record_count;

    // prep progress bar
    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    let show_progress =
        (args.flag_progressbar || util::get_envvar_flag("QSV_PROGRESSBAR")) && !rconfig.is_stdin();
    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    let progress = ProgressBar::with_draw_target(None, ProgressDrawTarget::stderr_with_hz(5));
    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    {
        record_count = if show_progress {
            let count = util::count_rows(&rconfig)?;
            util::prep_progress(&progress, count);
            count
        } else {
            progress.set_draw_target(ProgressDrawTarget::hidden());
            0
        };
    }
    #[cfg(feature = "datapusher_plus")]
    {
        record_count = 0;
    }

    let do_json = args.flag_json || args.flag_pretty_json;

    let mut record = ByteRecord::new();
    let mut next_record = ByteRecord::new();
    let mut sorted = true;
    let mut scan_ctr: u64 = 0;
    let mut dupe_count: u64 = 0;
    let mut unsorted_breaks: u64 = 0;

    rdr.read_byte_record(&mut record)?;
    loop {
        #[cfg(any(feature = "feature_capable", feature = "lite"))]
        if show_progress {
            progress.inc(1);
        }
        scan_ctr += 1;
        let more_records = rdr.read_byte_record(&mut next_record)?;
        if !more_records {
            break;
        }
        let a = sel.select(&record);
        let b = sel.select(&next_record);
        let comparison = match compare_mode {
            ComparisonMode::Lex => iter_cmp(a, b),
            ComparisonMode::LexIgnoreCase => iter_cmp_ignore_case(a, b),
            ComparisonMode::Numeric => iter_cmp_num(a, b),
            ComparisonMode::Natural => iter_cmp_natural(a, b),
            ComparisonMode::NaturalIgnoreCase => iter_cmp_natural_ignore_case(a, b),
        };

        match comparison {
            Ordering::Equal => {
                dupe_count += 1;
            },
            Ordering::Less => {
                // Allocation-free buffer rotation: next_record will be
                // overwritten by the next read_byte_record, so swapping is
                // safe and avoids the clone_from copy.
                std::mem::swap(&mut record, &mut next_record);
            },
            Ordering::Greater => {
                sorted = false;
                if args.flag_all || do_json {
                    unsorted_breaks += 1;
                    std::mem::swap(&mut record, &mut next_record);
                } else {
                    break;
                }
            },
        }
    } // end loop

    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    if show_progress {
        if sorted {
            progress.set_message(format!(
                " - ALL {} records checked. {} duplicates found. Sorted.",
                HumanCount(record_count),
                HumanCount(dupe_count),
            ));
        } else if args.flag_all || do_json {
            progress.set_message(format!(
                " - ALL {} records checked. {} unsorted breaks. NOT Sorted.",
                HumanCount(record_count),
                HumanCount(unsorted_breaks),
            ));
        } else {
            progress.set_message(format!(
                " - {} of {} records checked before aborting. {} duplicates found so far. NOT \
                 sorted.",
                HumanCount(scan_ctr),
                HumanCount(record_count),
                HumanCount(dupe_count),
            ));
        }
        util::finish_progress(&progress);
    }

    if do_json {
        // `do_json` forces a full scan in the Greater arm above, so when
        // record_count was not pre-computed via count_rows (no --progressbar
        // / datapusher_plus build), scan_ctr equals the total record count.
        let sortcheck_struct = SortCheckStruct {
            sorted,
            record_count: if record_count == 0 {
                scan_ctr
            } else {
                record_count
            },
            unsorted_breaks,
            // -1 signals "not applicable" when the file is unsorted.
            // try_from is defensive; in practice u64 dupe counts never
            // exceed i64::MAX on real CSVs.
            dupe_count: if sorted {
                i64::try_from(dupe_count).unwrap_or(i64::MAX)
            } else {
                -1
            },
        };
        // it's OK to have unwrap here as we know sortcheck_struct is valid json
        if args.flag_pretty_json {
            println!(
                "{}",
                simd_json::to_string_pretty(&sortcheck_struct).unwrap()
            );
        } else {
            println!("{}", simd_json::to_string(&sortcheck_struct).unwrap());
        }
    }

    if !sorted {
        return fail!("not sorted");
    }

    Ok(())
}
