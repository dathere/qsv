static USAGE: &str = r#"
Replace occurrences of a pattern across a CSV file.

You can of course match groups using parentheses and use those in
the replacement string. But don't forget to escape your $ in bash by using a
backslash or by wrapping the replacement string into single quotes:

  $ qsv replace 'hel(lo)' 'hal$1' file.csv
  $ qsv replace "hel(lo)" "hal\$1" file.csv

Returns exitcode 0 when replacements are done, returning number of replacements to stderr.
Returns exitcode 1 when no replacements are done, unless the '--not-one' flag is used.

When the CSV is indexed, a faster parallel replace is used.
If there were any replacements, the index will be refreshed.

Examples:

Replace all occurrences of 'hello' with 'world' in the file.csv file.

  $ qsv replace 'hello' 'world' file.csv

Replace all occurrences of 'hello' with 'world' in the file.csv file
and save the output to the file.out file.

  $ qsv replace 'hello' 'world' file.csv -o file.out

Replace all occurrences of 'hello' case insensitive with 'world'
in the file.csv file.

  $ qsv replace 'hello' 'world' file.csv -i

Replace all valid email addresses (using a regex)
with '<EMAIL>' in the file.csv file.

  $ qsv replace '([a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,})' \
   '<EMAIL>' file.csv


For more examples, see https://github.com/dathere/qsv/blob/master/tests/test_replace.rs.

Usage:
    qsv replace [options] <pattern> <replacement> [<input>]
    qsv replace --help

replace arguments:
    <pattern>              Regular expression pattern to match. Uses Rust regex syntax.
                           See https://docs.rs/regex/latest/regex/index.html#syntax
                           or https://regex101.com with the Rust flavor for more info.
    <input>                The CSV file to read. If not given, reads from stdin.
    <replacement>          Replacement string. Set to '<NULL>' if you want to
                           replace matches with ''.
replace options:
    -i, --ignore-case      Case insensitive search. This is equivalent to
                           prefixing the regex with '(?i)'.
    --literal              Treat the regex pattern as a literal string. This allows you
                           to search for matches that contain regex special characters.
    --exact                Match the ENTIRE field exactly. Treats the pattern
                           as a literal string (like --literal) and automatically
                           anchors it to match the complete field value (^pattern$).
    -s, --select <arg>     Select the columns to search. See 'qsv select -h'
                           for the full syntax.
    -u, --unicode          Enable unicode support. When enabled, character classes
                           will match all unicode word characters instead of only
                           ASCII word characters. Decreases performance.
    --size-limit <mb>      Set the approximate size limit (MB) of the compiled
                           regular expression. If the compiled expression exceeds this
                           number, then a compilation error is returned.
                           [default: 50]
    --dfa-size-limit <mb>  Set the approximate size of the cache (MB) used by the regular
                           expression engine's Discrete Finite Automata.
                           [default: 10]
    --not-one              Use exit code 0 instead of 1 for no replacement found.
    -j, --jobs <arg>       The number of jobs to run in parallel when the given CSV data has
                           an index. Note that a file handle is opened for each job.
                           When not set, defaults to the number of CPUs detected.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -n, --no-headers       When set, the first row will not be interpreted
                           as headers. (i.e., They are not searched, analyzed,
                           sliced, etc.)
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
    -p, --progressbar      Show progress bars. Not valid for stdin.
    -q, --quiet            Do not print number of replacements to stderr.

"#;

use std::{borrow::Cow, fs, sync::Arc};

use crossbeam_channel;
use foldhash::HashSet;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
use indicatif::{HumanCount, ProgressBar, ProgressDrawTarget};
use regex::bytes::RegexBuilder;
use serde::Deserialize;
use threadpool::ThreadPool;

use crate::{
    CliError, CliResult,
    config::{Config, Delimiter},
    index::Indexed,
    select::SelectColumns,
    util,
};

#[allow(dead_code)]
#[derive(Deserialize, Clone)]
struct Args {
    arg_input:           Option<String>,
    arg_pattern:         String,
    arg_replacement:     String,
    flag_select:         SelectColumns,
    flag_unicode:        bool,
    flag_output:         Option<String>,
    flag_no_headers:     bool,
    flag_delimiter:      Option<Delimiter>,
    flag_ignore_case:    bool,
    flag_literal:        bool,
    flag_exact:          bool,
    flag_size_limit:     usize,
    flag_dfa_size_limit: usize,
    flag_not_one:        bool,
    flag_progressbar:    bool,
    flag_quiet:          bool,
    flag_jobs:           Option<usize>,
}

const NULL_VALUE: &str = "<null>";

// ReplaceResult holds information about a replacement result for parallel processing
struct ReplaceResult {
    row_number:  u64,
    record:      csv::ByteRecord,
    match_count: u64,  // matches in this record
    had_match:   bool, // did this record have any matches
}

/// Process a single record, applying the regex pattern and replacement to selected fields.
/// Returns (processed_record, match_count, had_any_match)
#[inline]
fn process_record(
    record: &csv::ByteRecord,
    sel_indices: &HashSet<usize>,
    pattern: &regex::bytes::Regex,
    replacement: &[u8],
) -> (csv::ByteRecord, u64, bool) {
    let mut match_count = 0;
    let mut had_match = false;

    let processed_record = record
        .into_iter()
        .enumerate()
        .map(|(i, v)| {
            if sel_indices.contains(&i) {
                if pattern.is_match(v) {
                    match_count += 1;
                    had_match = true;
                    pattern.replace_all(v, replacement)
                } else {
                    Cow::Borrowed(v)
                }
            } else {
                Cow::Borrowed(v)
            }
        })
        .collect();

    (processed_record, match_count, had_match)
}

/// Handle the final results of a replace operation.
/// Prints match count to stderr (unless quiet) and returns error if no matches found
/// (unless not_one flag is set).
fn handle_replace_results(
    total_match_ctr: u64,
    flag_quiet: bool,
    flag_not_one: bool,
) -> CliResult<()> {
    if !flag_quiet {
        eprintln!("{total_match_ctr}");
    }
    if total_match_ctr == 0 && !flag_not_one {
        return Err(CliError::NoMatch());
    }
    Ok(())
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    let regex_unicode = if util::get_envvar_flag("QSV_REGEX_UNICODE") {
        true
    } else {
        args.flag_unicode
    };

    let arg_pattern = if args.flag_literal {
        regex::escape(&args.arg_pattern)
    } else if args.flag_exact {
        format!("^{}$", regex::escape(&args.arg_pattern))
    } else {
        args.arg_pattern.clone()
    };

    let pattern = RegexBuilder::new(&arg_pattern)
        .case_insensitive(args.flag_ignore_case)
        .unicode(regex_unicode)
        .size_limit(args.flag_size_limit * (1 << 20))
        .dfa_size_limit(args.flag_dfa_size_limit * (1 << 20))
        .build()?;
    let replacement = if args.arg_replacement.to_lowercase() == NULL_VALUE {
        b""
    } else {
        args.arg_replacement.as_bytes()
    };
    let rconfig = Config::new(args.arg_input.as_ref())
        .delimiter(args.flag_delimiter)
        .no_headers_flag(args.flag_no_headers)
        .select(args.flag_select.clone());

    // Route to parallel or sequential replace
    // based on index availability and number of jobs
    if let Some(idx) = rconfig.indexed()?
        && util::njobs(args.flag_jobs) > 1
    {
        args.parallel_replace(&idx, pattern, &rconfig, replacement)
    } else {
        args.sequential_replace(&pattern, &rconfig, replacement)
    }
}

impl Args {
    fn sequential_replace(
        &self,
        pattern: &regex::bytes::Regex,
        rconfig: &Config,
        replacement: &[u8],
    ) -> CliResult<()> {
        let mut rdr = rconfig.reader()?;
        let mut wtr = Config::new(self.flag_output.as_ref()).writer()?;

        let headers = rdr.byte_headers()?.clone();
        let sel = rconfig.selection(&headers)?;

        // use a hash set for O(1) time complexity
        // instead of O(n) with the previous vector lookup
        let sel_indices: HashSet<usize> = sel.iter().copied().collect();

        if !rconfig.no_headers {
            wtr.write_record(&headers)?;
        }

        // prep progress bar
        #[cfg(any(feature = "feature_capable", feature = "lite"))]
        let show_progress = (self.flag_progressbar || util::get_envvar_flag("QSV_PROGRESSBAR"))
            && !rconfig.is_stdin();
        #[cfg(any(feature = "feature_capable", feature = "lite"))]
        let progress = ProgressBar::with_draw_target(None, ProgressDrawTarget::stderr_with_hz(5));
        #[cfg(any(feature = "feature_capable", feature = "lite"))]
        if show_progress {
            util::prep_progress(&progress, util::count_rows(rconfig)?);
        } else {
            progress.set_draw_target(ProgressDrawTarget::hidden());
        }

        let mut record = csv::ByteRecord::new();
        let mut total_match_ctr: u64 = 0;
        #[cfg(any(feature = "feature_capable", feature = "lite"))]
        let mut rows_with_matches_ctr: u64 = 0;

        while rdr.read_byte_record(&mut record)? {
            #[cfg(any(feature = "feature_capable", feature = "lite"))]
            if show_progress {
                progress.inc(1);
            }

            let (processed_record, match_count, had_match) =
                process_record(&record, &sel_indices, pattern, replacement);

            total_match_ctr += match_count;
            #[cfg(any(feature = "feature_capable", feature = "lite"))]
            if had_match {
                rows_with_matches_ctr += 1;
            }

            wtr.write_byte_record(&processed_record)?;
            record = processed_record;
        }

        wtr.flush()?;

        #[cfg(any(feature = "feature_capable", feature = "lite"))]
        if show_progress {
            progress.set_message(format!(
                r#" - {} total matches replaced with "{}" in {} out of {} records."#,
                HumanCount(total_match_ctr),
                self.arg_replacement,
                HumanCount(rows_with_matches_ctr),
                HumanCount(progress.length().unwrap()),
            ));
            util::finish_progress(&progress);
        }

        handle_replace_results(total_match_ctr, self.flag_quiet, self.flag_not_one)
    }

    fn parallel_replace(
        &self,
        idx: &Indexed<fs::File, fs::File>,
        pattern: regex::bytes::Regex,
        rconfig: &Config,
        replacement: &[u8],
    ) -> CliResult<()> {
        let mut rdr = rconfig.reader()?;
        let headers = rdr.byte_headers()?.clone();
        let sel = rconfig.selection(&headers)?;

        let idx_count = idx.count() as usize;
        if idx_count == 0 {
            return Ok(());
        }

        let njobs = util::njobs(self.flag_jobs);
        let chunk_size = util::chunk_size(idx_count, njobs);
        let nchunks = util::num_of_chunks(idx_count, chunk_size);

        // Convert sel_indices to owned HashSet and wrap in Arc
        let sel_indices: Arc<HashSet<usize>> = Arc::new(sel.iter().copied().collect());

        // Wrap pattern in Arc for sharing across threads
        let pattern = Arc::new(pattern);
        let replacement = Arc::new(replacement.to_vec());

        // Create thread pool and channel
        let pool = ThreadPool::new(njobs);
        let (send, recv) = crossbeam_channel::bounded(nchunks);

        // Before the loop, prepare what each thread needs
        let rconfig_template = rconfig.clone();

        // Spawn replacement jobs
        for i in 0..nchunks {
            let (send, sel_indices, pattern, replacement) = (
                send.clone(),
                // self.clone(),
                Arc::clone(&sel_indices),
                Arc::clone(&pattern),
                Arc::clone(&replacement),
            );
            let rconfig = rconfig_template.clone();
            pool.execute(move || {
                // safety: we know the file is indexed and seekable
                let mut idx = rconfig.indexed().unwrap().unwrap();
                idx.seek((i * chunk_size) as u64).unwrap();
                let it = idx.byte_records().take(chunk_size);

                let mut results = Vec::with_capacity(chunk_size);
                let mut row_number = (i * chunk_size) as u64 + 1; // 1-based row numbering

                for record in it.flatten() {
                    let (processed_record, match_count_local, had_match_local) =
                        process_record(&record, &sel_indices, &pattern, replacement.as_slice());

                    results.push(ReplaceResult {
                        row_number,
                        record: processed_record,
                        match_count: match_count_local,
                        had_match: had_match_local,
                    });
                    row_number += 1;
                }
                send.send(results).unwrap();
            });
        }
        drop(send);

        // Collect all results from all chunks
        let mut all_chunks: Vec<Vec<ReplaceResult>> = Vec::with_capacity(nchunks);
        for chunk_results in &recv {
            all_chunks.push(chunk_results);
        }

        // Sort chunks by first row_number to maintain original order
        all_chunks.sort_unstable_by_key(|chunk| chunk.first().map_or(0, |r| r.row_number));

        // Setup writer
        let mut wtr = Config::new(self.flag_output.as_ref()).writer()?;

        // Write headers
        if !rconfig.no_headers {
            wtr.write_record(&headers)?;
        }

        // Write results and aggregate match counters
        let mut total_match_ctr: u64 = 0;
        #[cfg(any(feature = "feature_capable", feature = "lite"))]
        let mut _rows_with_matches_ctr: u64 = 0;

        for chunk in all_chunks {
            for result in chunk {
                total_match_ctr += result.match_count;
                #[cfg(any(feature = "feature_capable", feature = "lite"))]
                if result.had_match {
                    _rows_with_matches_ctr += 1;
                }
                wtr.write_byte_record(&result.record)?;
            }
        }

        wtr.flush()?;

        handle_replace_results(total_match_ctr, self.flag_quiet, self.flag_not_one)
    }
}
