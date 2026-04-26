static USAGE: &str = r#"
Deduplicates CSV rows. 

This requires reading all of the CSV data into memory because because the rows need
to be sorted first.

That is, unless the --sorted option is used to indicate the CSV is already sorted -
typically, with the sort cmd for more sorting options or the extsort cmd for larger
than memory CSV files. This will make dedup run in streaming mode with constant memory.

Either way, the output will not only be deduplicated, it will also be sorted.

A duplicate count will also be sent to <stderr>.

Examples:

  # Deduplicate an unsorted CSV file:
  qsv dedup unsorted.csv -o deduped.csv

  # Deduplicate a sorted CSV file:
  qsv sort unsorted.csv | qsv dedup --sorted -o deduped.csv

  # Deduplicate based on specific columns:
  qsv dedup --select col1,col2 unsorted.csv -o deduped.csv

  # Deduplicate based on numeric comparison of col1 and col2 columns:
  qsv dedup -s col1,col2 --numeric unsorted.csv -o deduped.csv
  
  # Deduplicate ignoring case of col1 and col2 columns:
  qsv dedup -s col1,col2 --ignore-case unsorted.csv -o deduped.csv

  # Write duplicates to a separate file:
  qsv dedup -s col1,col2 --dupes-output dupes.csv unsorted.csv -o deduped.csv

For examples, see https://github.com/dathere/qsv/blob/master/tests/test_dedup.rs.

Usage:
    qsv dedup [options] [<input>]
    qsv dedup --help

dedup options:
    -s, --select <arg>         Select a subset of columns to dedup.
                               Note that the outputs will remain at the full width
                               of the CSV.
                               See 'qsv select --help' for the format details.
    -N, --numeric              Compare according to string numerical value
    -i, --ignore-case          Compare strings disregarding case.
    --sorted                   The input is already sorted. Do not load the CSV into
                               memory to sort it first. Meant to be used in tandem and
                               after an extsort.
    -D, --dupes-output <file>  Write duplicates to <file>.
    -H, --human-readable       Comma separate duplicate count.
    -j, --jobs <arg>           The number of jobs to run in parallel when sorting
                               an unsorted CSV, before deduping.
                               When not set, the number of jobs is set to the
                               number of CPUs detected.
                               Does not work with --sorted option as its not
                               multithreaded.

Common options:
    -h, --help                 Display this message
    -o, --output <file>        Write output to <file> instead of stdout.
    -n, --no-headers           When set, the first row will not be interpreted
                               as headers. That is, it will be sorted with the rest
                               of the rows. Otherwise, the first row will always
                               appear as the header row in the output.
    -d, --delimiter <arg>      The field delimiter for reading CSV data.
                               Must be a single character. (default: ,)
    -q, --quiet                Do not print duplicate count to stderr.
    --memcheck                 Check if there is enough memory to load the entire
                               CSV into memory using CONSERVATIVE heuristics.
                               Has no effect when --sorted is set, as that path
                               streams the input and never loads it into memory.
"#;

use std::cmp::Ordering;

use csv::ByteRecord;
use rayon::slice::ParallelSliceMut;
use serde::Deserialize;

use crate::{
    CliResult,
    cmd::sort::{iter_cmp, iter_cmp_num},
    config::{Config, Delimiter},
    select::SelectColumns,
    util,
};
#[derive(Deserialize)]
struct Args {
    arg_input:           Option<String>,
    flag_select:         SelectColumns,
    flag_numeric:        bool,
    flag_ignore_case:    bool,
    flag_sorted:         bool,
    flag_dupes_output:   Option<String>,
    flag_output:         Option<String>,
    flag_no_headers:     bool,
    flag_delimiter:      Option<Delimiter>,
    flag_human_readable: bool,
    flag_jobs:           Option<usize>,
    flag_quiet:          bool,
    flag_memcheck:       bool,
}

#[derive(Debug)]
enum ComparisonMode {
    Numeric,
    IgnoreCase,
    Normal,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    let compare_mode = if args.flag_numeric {
        ComparisonMode::Numeric
    } else if args.flag_ignore_case {
        ComparisonMode::IgnoreCase
    } else {
        ComparisonMode::Normal
    };

    let rconfig = Config::new(args.arg_input.as_ref())
        .delimiter(args.flag_delimiter)
        .no_headers_flag(args.flag_no_headers)
        .select(args.flag_select);

    let mut rdr = rconfig.reader()?;
    let mut wtr = Config::new(args.flag_output.as_ref()).writer()?;
    // Use rconfig.write_headers so the dupes writer correctly skips header
    // emission under --no-headers (where byte_headers() returns the first
    // data row) and avoids writing an empty record when there are no headers.
    let mut dupewtr = if args.flag_dupes_output.is_some() {
        let mut w = Config::new(args.flag_dupes_output.as_ref()).writer()?;
        rconfig.write_headers(&mut rdr, &mut w)?;
        Some(w)
    } else {
        None
    };

    let headers = rdr.byte_headers()?;
    let sel = rconfig.selection(headers)?;

    rconfig.write_headers(&mut rdr, &mut wtr)?;
    let mut dupe_count = 0_usize;

    if args.flag_sorted {
        let mut record = ByteRecord::new();
        let mut next_record = ByteRecord::new();

        // Only enter the streaming loop if there is at least one data row;
        // otherwise fall through to the flush + duplicate-count print block
        // so empty input behaves identically to the in-memory path.
        if rdr.read_byte_record(&mut record)? {
            loop {
                let more_records = rdr.read_byte_record(&mut next_record)?;
                if !more_records {
                    wtr.write_byte_record(&record)?;
                    break;
                }
                let a = sel.select(&record);
                let b = sel.select(&next_record);
                let comparison = match compare_mode {
                    ComparisonMode::Normal => iter_cmp(a, b),
                    ComparisonMode::Numeric => iter_cmp_num(a, b),
                    ComparisonMode::IgnoreCase => iter_cmp_ignore_case(a, b),
                };
                match comparison {
                    Ordering::Equal => {
                        dupe_count += 1;
                        // Write the row being DROPPED to dupes (next_record),
                        // not the survivor (record). The streaming path keeps
                        // the first occurrence of a run, so for runs longer
                        // than 2 the dropped rows are next_record on each
                        // iteration. Writing &record here instead would emit
                        // the survivor N-1 times — and with --select, miss
                        // the actual dropped rows entirely.
                        if let Some(ref mut w) = dupewtr {
                            w.write_byte_record(&next_record)?;
                        }
                    },
                    Ordering::Less => {
                        wtr.write_byte_record(&record)?;
                        std::mem::swap(&mut record, &mut next_record);
                    },
                    Ordering::Greater => {
                        return fail_clierror!(
                            r#"Aborting! Input not sorted! Current record is greater than Next record.
  Compare mode: {compare_mode:?};  Select columns index/es (0-based): {sel:?}
  Current: {record:?}
     Next: {next_record:?}
"#
                        );
                    },
                }
            }
        }
    } else {
        // we're loading the entire file into memory, we need to check avail mem
        if let Some(path) = rconfig.path.clone() {
            util::mem_file_check(&path, false, args.flag_memcheck)?;
        }

        util::njobs(args.flag_jobs);

        let mut all = rdr.byte_records().collect::<Result<Vec<_>, _>>()?;
        match compare_mode {
            ComparisonMode::Normal => {
                all.par_sort_by(|r1, r2| {
                    let a = sel.select(r1);
                    let b = sel.select(r2);
                    iter_cmp(a, b)
                });
            },
            ComparisonMode::Numeric => {
                all.par_sort_by(|r1, r2| {
                    let a = sel.select(r1);
                    let b = sel.select(r2);
                    iter_cmp_num(a, b)
                });
            },
            ComparisonMode::IgnoreCase => {
                all.par_sort_by(|r1, r2| {
                    let a = sel.select(r1);
                    let b = sel.select(r2);
                    iter_cmp_ignore_case(a, b)
                });
            },
        }

        // Hoist comparison dispatch out of the row loop: pick the cmp once,
        // then run a single tight loop. Each branch monomorphizes the body.
        macro_rules! scan_dedup {
            ($cmp:expr) => {{
                let mut iter = all.iter();
                if let Some(mut prev) = iter.next() {
                    for current in iter {
                        if $cmp(sel.select(prev), sel.select(current)) == Ordering::Equal {
                            dupe_count += 1;
                            if let Some(ref mut w) = dupewtr {
                                w.write_byte_record(prev)?;
                            }
                        } else {
                            wtr.write_byte_record(prev)?;
                        }
                        prev = current;
                    }
                    wtr.write_byte_record(prev)?;
                }
            }};
        }
        match compare_mode {
            ComparisonMode::Normal => scan_dedup!(iter_cmp),
            ComparisonMode::Numeric => scan_dedup!(iter_cmp_num),
            ComparisonMode::IgnoreCase => scan_dedup!(iter_cmp_ignore_case),
        }
    }

    if let Some(mut w) = dupewtr {
        w.flush()?;
    }
    wtr.flush()?;

    if args.flag_quiet {
        return Ok(());
    }

    if args.flag_human_readable {
        use indicatif::HumanCount;

        eprintln!("{}", HumanCount(dupe_count as u64));
    } else {
        eprintln!("{dupe_count}");
    }

    Ok(())
}

/// Try comparing `a` and `b` ignoring the case.
///
/// Cells that are pure ASCII compare via a zero-allocation byte-wise lowercase
/// fold (the common case). Non-ASCII cells fall back to allocating a
/// lowercased `String`. Cells that are not valid UTF-8 fall back to a raw
/// byte comparison so a deterministic order is still produced.
#[inline]
pub fn iter_cmp_ignore_case<'a, L, R>(mut a: L, mut b: R) -> Ordering
where
    L: Iterator<Item = &'a [u8]>,
    R: Iterator<Item = &'a [u8]>,
{
    loop {
        match (a.next(), b.next()) {
            (None, None) => return Ordering::Equal,
            (None, _) => return Ordering::Less,
            (_, None) => return Ordering::Greater,
            (Some(x), Some(y)) => match cmp_ignore_case(x, y) {
                Ordering::Equal => (),
                non_eq => return non_eq,
            },
        }
    }
}

#[inline]
fn cmp_ignore_case(a: &[u8], b: &[u8]) -> Ordering {
    // ASCII fast path: zero-allocation byte-wise lowercase compare.
    if a.is_ascii() && b.is_ascii() {
        return a
            .iter()
            .map(u8::to_ascii_lowercase)
            .cmp(b.iter().map(u8::to_ascii_lowercase));
    }
    // Unicode slow path: allocate lowercased Strings.
    match (
        simdutf8::basic::from_utf8(a).ok(),
        simdutf8::basic::from_utf8(b).ok(),
    ) {
        (Some(sa), Some(sb)) => sa.to_lowercase().cmp(&sb.to_lowercase()),
        // Invalid UTF-8 on either side: fall back to raw byte comparison.
        _ => a.cmp(b),
    }
}
