use std::cmp;

use crate::cmd::dedup;
use crate::config::{Config, Delimiter};
use crate::select::SelectColumns;
use crate::util;
use crate::CliResult;
use csv::ByteRecord;
#[cfg(any(feature = "full", feature = "lite"))]
use indicatif::{HumanCount, ProgressBar, ProgressDrawTarget};
use serde::{Deserialize, Serialize};

use crate::cmd::sort::iter_cmp;

static USAGE: &str = r#"
Check if a CSV is sorted. The check is done on a streaming basis (i.e. constant memory).

This command can be used in tandem with other qsv commands that sort or require sorted data
to ensure that they also work on a stream of data - i.e. without loading an entire CSV into memory.

For instance, a naive `dedup` requires loading the entire CSV into memory to sort it
first before deduping. However, if you know a CSV is sorted beforehand, you can invoke
`dedup` with the --sorted option, and it will skip loading entire CSV into memory to sort
it first. It will just immediately dedupe on a streaming basis.

`sort` also requires loading the entire CSV into memory. For simple "sorts" (not numeric,
reverse & random sorts), particularly of very large CSV files that will not fit in memory,
`extsort` - a multi-threaded streaming sort that is exponentially faster and can work with 
arbitrarily large files, can be used instead.

Simply put, sortcheck allows you to make informed choices on how to compose pipelines that
require sorted data.

Apart from checking if a CSV is sorted, it also allows you to check a CSV's record count,
sort breaks & dupe count with its --json options.

Returns exit code 0 if a CSV is sorted, and exit code 1 otherwise.

Usage:
    qsv sortcheck [options] [<input>]

sort options:
    -s, --select <arg>      Select a subset of columns to check for sort.
                            See 'qsv select --help' for the format details.
    -C, --no-case           Compare strings disregarding case
    --all                   Check all records. Do not stop the check on the
                            first unsorted record.
    --json                  Return results in JSON format. The JSON result has
                            the following properties - sorted (boolean), 
                            record_count (number), unsorted_breaks (number) &
                            dupe_count (number).
    --pretty-json           Return results in pretty JSON format.

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

#[derive(Deserialize)]
struct Args {
    arg_input: Option<String>,
    flag_select: SelectColumns,
    flag_no_case: bool,
    flag_all: bool,
    flag_no_headers: bool,
    flag_delimiter: Option<Delimiter>,
    flag_progressbar: bool,
    flag_json: bool,
    flag_pretty_json: bool,
}

#[derive(Serialize, Deserialize)]
struct SortCheckStruct {
    sorted: bool,
    record_count: u64,
    unsorted_breaks: u64,
    dupe_count: u64,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let no_case = args.flag_no_case;
    let rconfig = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers)
        .checkutf8(false)
        .select(args.flag_select);

    let mut rdr = rconfig.reader()?;

    let headers = rdr.byte_headers()?.clone();
    let sel = rconfig.selection(&headers)?;

    // prep progress bar
    #[cfg(any(feature = "full", feature = "lite"))]
    let show_progress =
        (args.flag_progressbar || std::env::var("QSV_PROGRESSBAR").is_ok()) && !rconfig.is_stdin();
    #[cfg(any(feature = "full", feature = "lite"))]
    let progress = ProgressBar::with_draw_target(None, ProgressDrawTarget::stderr_with_hz(5));
    #[cfg(any(feature = "full", feature = "lite"))]
    let record_count = if show_progress {
        let count = util::count_rows(&rconfig)?;
        util::prep_progress(&progress, count);
        count
    } else {
        progress.set_draw_target(ProgressDrawTarget::hidden());
        0
    };
    #[cfg(feature = "datapusher_plus")]
    let mut record_count: u64 = 0;

    let mut record = ByteRecord::new();
    let mut next_record = ByteRecord::new();
    let mut sorted = true;
    let mut scan_ctr: u64 = 0;
    let mut dupe_count: u64 = 0;
    let mut unsorted_breaks: u64 = 0;

    rdr.read_byte_record(&mut record)?;
    loop {
        #[cfg(any(feature = "full", feature = "lite"))]
        if show_progress {
            progress.inc(1);
        }
        scan_ctr += 1;
        let more_records = rdr.read_byte_record(&mut next_record)?;
        if !more_records {
            break;
        };
        let a = sel.select(&record);
        let b = sel.select(&next_record);
        let comparison = if no_case {
            dedup::iter_cmp_no_case(a, b)
        } else {
            iter_cmp(a, b)
        };

        match comparison {
            cmp::Ordering::Equal => {
                dupe_count += 1;
            }
            cmp::Ordering::Less => {
                record.clone_from(&next_record);
            }
            cmp::Ordering::Greater => {
                sorted = false;
                if args.flag_all {
                    unsorted_breaks += 1;
                    record.clone_from(&next_record);
                } else {
                    break;
                }
            }
        }
    } // end loop

    #[cfg(feature = "datapusher_plus")]
    {
        record_count = scan_ctr;
    }

    #[cfg(any(feature = "full", feature = "lite"))]
    if show_progress {
        if sorted {
            progress.set_message(format!(
                " - ALL {} records checked. {} duplicates found. Sorted.",
                HumanCount(record_count),
                HumanCount(dupe_count),
            ));
        } else if args.flag_all {
            progress.set_message(format!(
                " - ALL {} records checked. {} unsorted breaks & {} duplicates found. NOT Sorted.",
                HumanCount(record_count),
                HumanCount(unsorted_breaks),
                HumanCount(dupe_count),
            ));
        } else {
            progress.set_message(format!(
                " - {} of {} records checked before aborting. {} duplicates found so far. NOT sorted.",
                HumanCount(scan_ctr),
                HumanCount(record_count),
                HumanCount(dupe_count),
            ));
        }
        util::finish_progress(&progress);
    }

    if args.flag_json || args.flag_pretty_json {
        let sortcheck_struct = SortCheckStruct {
            sorted,
            record_count: if record_count == 0 {
                scan_ctr
            } else {
                record_count
            },
            unsorted_breaks,
            dupe_count,
        };
        if args.flag_pretty_json {
            println!(
                "{}",
                serde_json::to_string_pretty(&sortcheck_struct).unwrap()
            );
        } else {
            let json_result = serde_json::to_string(&sortcheck_struct).unwrap();
            println!("{json_result}");
        };
    }

    if sorted {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }

    #[allow(unreachable_code)]
    Ok(())
}
