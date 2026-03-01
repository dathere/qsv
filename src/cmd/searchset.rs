static USAGE: &str = r#"
Filters CSV data by whether the given regex set matches a row.

Unlike the search operation, this allows regex matching of multiple regexes 
in a single pass.

The regexset-file is a plain text file with multiple regexes, with a regex on 
each line. For an example scanning for common Personally Identifiable Information (PII) -
SSN, credit cards, email, bank account numbers & phones, see
https://github.com/dathere/qsv/blob/master/resources/examples/searchset/pii_regexes.txt

The regex set is applied to each field in each row, and if any field matches,
then the row is written to the output, and the number of matches to stderr.

The columns to search can be limited with the '--select' flag (but the full row
is still written to the output if there is a match).

Returns exitcode 0 when matches are found, returning number of matches to stderr.
Returns exitcode 1 when no match is found, unless the '--not-one' flag is used.

When --quick is enabled, no output is produced and exitcode 0 is returned on 
the first match.

When the CSV is indexed, a faster parallel search is used.

For examples, see https://github.com/dathere/qsv/blob/master/tests/test_searchset.rs.

Usage:
    qsv searchset [options] (<regexset-file>) [<input>]
    qsv searchset --help

searchset arguments:
    <regexset-file>            The file containing regular expressions to match, with a 
                               regular expression on each line.
                               See https://docs.rs/regex/latest/regex/index.html#syntax
                               or https://regex101.com with the Rust flavor for regex syntax.
    <input>                    The CSV file to read. If not given, reads from stdin.

searchset options:
    -i, --ignore-case          Case insensitive search. This is equivalent to
                               prefixing the regex with '(?i)'.
    --literal                  Treat the regex as a literal string. This allows you to
                               search for matches that contain regex special characters.
    --exact                    Match the ENTIRE field exactly. Treats the pattern
                               as a literal string (like --literal) and automatically
                               anchors it to match the complete field value (^pattern$).
    -s, --select <arg>         Select the columns to search. See 'qsv select -h'
                               for the full syntax.
    -v, --invert-match         Select only rows that did not match
    -u, --unicode              Enable unicode support. When enabled, character classes
                               will match all unicode word characters instead of only
                               ASCII word characters. Decreases performance.

    -f, --flag <column>        If given, the command will not filter rows
                               but will instead flag the found rows in a new
                               column named <column>. For each found row, <column>
                               is set to the row number of the row, followed by a
                               semicolon, then a list of the matching regexes.
    --flag-matches-only        When --flag is enabled, only rows that match are
                               sent to output. Rows that do not match are filtered.
    --unmatched-output <file>  When --flag-matches-only is enabled, output the rows
                               that did not match to <file>.

    -Q, --quick                Return on first match with an exitcode of 0, returning
                               the row number of the first match to stderr.
                               Return exit code 1 if no match is found.
                               No output is produced. Ignored if --json is enabled.
    -c, --count                Return number of matches to stderr.
                               Ignored if --json is enabled.
    -j, --json                 Return number of matches, number of rows with matches,
                               and number of rows to stderr in JSON format.
    --size-limit <mb>          Set the approximate size limit (MB) of the compiled
                               regular expression. If the compiled expression exceeds this 
                               number, then a compilation error is returned.
                               Modify this only if you're getting regular expression
                               compilation errors. [default: 50]
    --dfa-size-limit <mb>      Set the approximate size of the cache (MB) used by the regular
                               expression engine's Discrete Finite Automata.
                               Modify this only if you're getting regular expression
                               compilation errors. [default: 10]
    --not-one                  Use exit code 0 instead of 1 for no match found.
    --jobs <arg>               The number of jobs to run in parallel when the given CSV data has
                               an index. Note that a file handle is opened for each job.
                               When not set, defaults to the number of CPUs detected.

Common options:
    -h, --help                 Display this message
    -o, --output <file>        Write output to <file> instead of stdout.
    -n, --no-headers           When set, the first row will not be interpreted
                               as headers. (i.e., They are not searched, analyzed,
                               sliced, etc.)
    -d, --delimiter <arg>      The field delimiter for reading CSV data.
                               Must be a single character. (default: ,)
    -p, --progressbar          Show progress bars. Not valid for stdin.
    -q, --quiet                Do not return number of matches to stderr.
"#;

use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use crossbeam_channel;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
use indicatif::{HumanCount, ProgressBar, ProgressDrawTarget};
use log::{debug, info};
use regex::{Regex, bytes::RegexSetBuilder};
use serde::Deserialize;
use serde_json::json;
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
    arg_input:              Option<String>,
    arg_regexset_file:      String,
    flag_literal:           bool,
    flag_exact:             bool,
    flag_select:            SelectColumns,
    flag_output:            Option<String>,
    flag_no_headers:        bool,
    flag_delimiter:         Option<Delimiter>,
    flag_invert_match:      bool,
    flag_unicode:           bool,
    flag_ignore_case:       bool,
    flag_flag:              Option<String>,
    flag_flag_matches_only: bool,
    flag_unmatched_output:  Option<String>,
    flag_size_limit:        usize,
    flag_dfa_size_limit:    usize,
    flag_quick:             bool,
    flag_count:             bool,
    flag_json:              bool,
    flag_not_one:           bool,
    flag_progressbar:       bool,
    flag_quiet:             bool,
    flag_jobs:              Option<usize>,
}

// SearchSetResult holds information about a search result for parallel processing
struct SearchSetResult {
    row_number: u64,
    record:     csv::ByteRecord,
    matched:    bool,
    match_list: Vec<usize>, // indices of matched regexes (1-based)
}

fn read_regexset(filename: &str, literal: bool, exact: bool) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines = reader.lines();

    if literal {
        lines.map(|line| line.map(|s| regex::escape(&s))).collect()
    } else if exact {
        lines
            .map(|line| line.map(|s| format!("^{}$", regex::escape(&s))))
            .collect()
    } else {
        lines.collect()
    }
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    if args.flag_flag.is_none() && args.flag_flag_matches_only {
        return fail_incorrectusage_clierror!("Cannot use --flag-matches-only without --flag",);
    }
    if !args.flag_flag_matches_only && args.flag_unmatched_output.is_some() {
        return fail_incorrectusage_clierror!(
            "Cannot use --unmatched-output without --flag-matches-only",
        );
    }

    let regexset = read_regexset(&args.arg_regexset_file, args.flag_literal, args.flag_exact)?;

    let mut regex_labels: Vec<String> = Vec::with_capacity(regexset.len());
    let labels_re = Regex::new(r".?#(?P<label>.*)$").unwrap();

    // use regex comment labels if they exist, so matches are easier to understand
    for (i, regex) in regexset.iter().enumerate() {
        let label = labels_re
            .captures(regex)
            .and_then(|cap| cap.name("label"))
            .map_or_else(|| (i + 1).to_string(), |m| m.as_str().to_string());
        regex_labels.push(label);
    }

    let regex_unicode = if util::get_envvar_flag("QSV_REGEX_UNICODE") {
        true
    } else {
        args.flag_unicode
    };

    debug!("Compiling {} regex set expressions...", regexset.len());
    let pattern = RegexSetBuilder::new(&regexset)
        .case_insensitive(args.flag_ignore_case)
        .unicode(regex_unicode)
        .size_limit(args.flag_size_limit * (1 << 20))
        .dfa_size_limit(args.flag_dfa_size_limit * (1 << 20))
        .build()?;
    debug!("Successfully compiled regex set!");

    let rconfig = Config::new(args.arg_input.as_ref())
        .delimiter(args.flag_delimiter)
        .no_headers_flag(args.flag_no_headers)
        .select(args.flag_select.clone());

    // Route to parallel or sequential search
    // based on index availability and number of jobs
    if let Some(idx) = rconfig.indexed()?
        && util::njobs(args.flag_jobs) > 1
    {
        args.parallel_search(&idx, pattern, &rconfig, &regex_labels)
    } else {
        args.sequential_search(&pattern, &rconfig, &regex_labels)
    }
}

impl Args {
    fn rconfig(&self) -> Config {
        Config::new(self.arg_input.as_ref())
            .delimiter(self.flag_delimiter)
            .no_headers_flag(self.flag_no_headers)
            .select(self.flag_select.clone())
    }

    fn sequential_search(
        &self,
        pattern: &regex::bytes::RegexSet,
        rconfig: &Config,
        regex_labels: &[String],
    ) -> CliResult<()> {
        let flag_not_one = self.flag_not_one;

        let mut rdr = rconfig.reader()?;
        let mut wtr = Config::new(self.flag_output.as_ref()).writer()?;
        let mut unmatched_wtr = Config::new(self.flag_unmatched_output.as_ref()).writer()?;

        let mut headers = rdr.byte_headers()?.clone();
        let sel = rconfig.selection(&headers)?;

        let do_match_list = self.flag_flag.as_ref().is_some_and(|column_name| {
            headers.push_field(column_name.as_bytes());
            true
        });

        if !rconfig.no_headers && !self.flag_quick {
            wtr.write_record(&headers)?;
        }

        let record_count = util::count_rows(rconfig)?;
        // prep progress bar
        #[cfg(any(feature = "feature_capable", feature = "lite"))]
        let show_progress = (self.flag_progressbar || util::get_envvar_flag("QSV_PROGRESSBAR"))
            && !rconfig.is_stdin();
        #[cfg(any(feature = "feature_capable", feature = "lite"))]
        let progress = ProgressBar::with_draw_target(None, ProgressDrawTarget::stderr_with_hz(5));
        #[cfg(any(feature = "feature_capable", feature = "lite"))]
        if show_progress {
            util::prep_progress(&progress, record_count);
        } else {
            progress.set_draw_target(ProgressDrawTarget::hidden());
        }

        let mut record = csv::ByteRecord::new();
        let mut flag_rowi: u64 = 0;
        let mut match_row_ctr: u64 = 0;
        let mut total_matches: u64 = 0;
        let mut row_ctr: u64 = 0;

        // minimize allocs
        #[allow(unused_assignments)]
        let mut flag_column: Vec<u8> = Vec::with_capacity(20);
        let mut match_list_vec = Vec::with_capacity(20);
        #[allow(unused_assignments)]
        let mut match_list = String::with_capacity(20);
        let mut matched_rows = String::with_capacity(20);
        #[allow(unused_assignments)]
        let mut match_list_with_row = String::with_capacity(20);
        let mut m;
        let mut matched = false;
        let mut matches: Vec<usize> = Vec::with_capacity(20);

        while rdr.read_byte_record(&mut record)? {
            row_ctr += 1;
            #[cfg(any(feature = "feature_capable", feature = "lite"))]
            if show_progress {
                progress.inc(1);
            }
            m = sel.select(&record).any(|f| {
                matched = pattern.is_match(f);
                if matched && do_match_list {
                    matches = pattern.matches(f).into_iter().collect();
                    total_matches += matches.len() as u64;
                    for j in &mut matches {
                        *j += 1; // so the list is human readable - i.e. not zero-based
                    }
                    match_list_vec.clone_from(&matches);
                }
                matched
            });
            if self.flag_invert_match {
                m = !m;
            }
            if m {
                match_row_ctr += 1;
                if self.flag_quick {
                    break;
                }
            }

            if do_match_list {
                flag_rowi += 1;
                flag_column = if m {
                    itoa::Buffer::new()
                        .format(flag_rowi)
                        .clone_into(&mut matched_rows);
                    if self.flag_invert_match {
                        matched_rows.as_bytes().to_vec()
                    } else {
                        match_list = match_list_vec
                            .iter()
                            .map(|i| regex_labels[*i - 1].clone())
                            .collect::<Vec<String>>()
                            .join(",");
                        match_list_with_row = format!("{matched_rows};{match_list}");
                        match_list_with_row.as_bytes().to_vec()
                    }
                } else {
                    b"0".to_vec()
                };
                if self.flag_flag_matches_only && !m {
                    if self.flag_unmatched_output.is_some() {
                        unmatched_wtr.write_byte_record(&record)?;
                    }
                    continue;
                }
                record.push_field(&flag_column);
                wtr.write_byte_record(&record)?;
            } else if m {
                wtr.write_byte_record(&record)?;
            }
        }
        unmatched_wtr.flush()?;
        wtr.flush()?;

        #[cfg(any(feature = "feature_capable", feature = "lite"))]
        if show_progress {
            if do_match_list {
                progress.set_message(format!(
                    " - {} total matches in {} rows with matches found in {} records.",
                    HumanCount(total_matches),
                    HumanCount(match_row_ctr),
                    HumanCount(record_count),
                ));
            } else {
                progress.set_message(format!(
                    " - {} rows with matches found in {} records.",
                    HumanCount(match_row_ctr),
                    HumanCount(record_count),
                ));
            }
            util::finish_progress(&progress);
        }

        if self.flag_json {
            let json = json!({
                "rows_with_matches": match_row_ctr,
                "total_matches": total_matches,
                "record_count": record_count,
            });
            eprintln!("{json}");
        } else {
            if self.flag_count && !self.flag_quick {
                if !self.flag_quiet {
                    eprintln!("{match_row_ctr}");
                }
                info!("matches: {match_row_ctr}");
            }

            if match_row_ctr == 0 && !flag_not_one {
                return Err(CliError::NoMatch());
            } else if self.flag_quick {
                if !self.flag_quiet {
                    eprintln!("{row_ctr}");
                }
                info!("quick searchset first match at {row_ctr}");
            }
        }

        Ok(())
    }

    fn parallel_search(
        &self,
        idx: &Indexed<fs::File, fs::File>,
        pattern: regex::bytes::RegexSet,
        rconfig: &Config,
        regex_labels: &[String],
    ) -> CliResult<()> {
        let mut rdr = rconfig.reader()?;
        let mut headers = rdr.byte_headers()?.clone();
        let sel = rconfig.selection(&headers)?;

        let idx_count = idx.count() as usize;
        if idx_count == 0 {
            return Ok(());
        }

        let njobs = util::njobs(self.flag_jobs);
        let chunk_size = util::chunk_size(idx_count, njobs);
        let nchunks = util::num_of_chunks(idx_count, chunk_size);

        // Setup flag column if needed
        let do_match_list = self.flag_flag.as_ref().is_some_and(|column_name| {
            headers.push_field(column_name.as_bytes());
            true
        });

        // Wrap pattern and regex_labels in Arc for sharing across threads
        let pattern = Arc::new(pattern);
        let regex_labels = Arc::new(regex_labels.to_vec());
        let invert_match = self.flag_invert_match;
        let flag_quick = self.flag_quick;

        // Atomic flag for early termination in quick mode
        let match_found = Arc::new(AtomicBool::new(false));

        // Create thread pool and channel
        let pool = ThreadPool::new(njobs);
        let (send, recv) = crossbeam_channel::bounded(nchunks);

        // Spawn search jobs
        for i in 0..nchunks {
            let (send, args, sel, pattern, match_found_flag) = (
                send.clone(),
                self.clone(),
                sel.clone(),
                Arc::clone(&pattern),
                Arc::clone(&match_found),
            );
            pool.execute(move || {
                // safety: we know the file is indexed and seekable
                let mut idx = args.rconfig().indexed().unwrap().unwrap();
                idx.seek((i * chunk_size) as u64).unwrap();
                let it = idx.byte_records().take(chunk_size);

                let mut results = Vec::with_capacity(chunk_size);

                // 1-based row numbering
                for (row_number, record) in ((i * chunk_size) as u64 + 1..).zip(it.flatten()) {
                    // Early exit for quick mode if match already found by another thread
                    if flag_quick && match_found_flag.load(Ordering::Relaxed) {
                        break;
                    }

                    let mut match_list = Vec::with_capacity(pattern.len());

                    // Check if any field matches
                    let row_matched = sel.select(&record).any(|f| {
                        let is_match = pattern.is_match(f);
                        if is_match && do_match_list {
                            for m in pattern.matches(f) {
                                match_list.push(m + 1); // 1-based for human readability
                            }
                        }
                        is_match
                    });

                    let final_matched = if invert_match {
                        !row_matched
                    } else {
                        row_matched
                    };

                    // Set flag if we found a match in quick mode
                    if flag_quick && final_matched {
                        match_found_flag.store(true, Ordering::Relaxed);
                    }

                    results.push(SearchSetResult {
                        row_number,
                        record,
                        matched: final_matched,
                        match_list,
                    });
                }
                send.send(results).unwrap();
            });
        }
        drop(send);

        // Collect all results from all chunks
        let mut all_chunks: Vec<Vec<SearchSetResult>> = Vec::with_capacity(nchunks);
        for chunk_results in &recv {
            all_chunks.push(chunk_results);
        }

        // Sort chunks by first row_number to maintain original order
        all_chunks.sort_unstable_by_key(|chunk| chunk.first().map_or(0, |r| r.row_number));

        // Handle --quick mode: find earliest match
        if self.flag_quick {
            if let Some(first_match) = all_chunks.iter().flatten().find(|r| r.matched) {
                if !self.flag_quiet {
                    eprintln!("{}", first_match.row_number);
                }
                info!("quick searchset first match at {}", first_match.row_number);
                return Ok(());
            }
            // No match found
            if !self.flag_not_one {
                return Err(CliError::NoMatch());
            }
            return Ok(());
        }

        // Setup writers
        let mut wtr = Config::new(self.flag_output.as_ref()).writer()?;
        let mut unmatched_wtr = Config::new(self.flag_unmatched_output.as_ref()).writer()?;

        // Write headers
        if !rconfig.no_headers {
            wtr.write_record(&headers)?;
        }

        // Write results
        let mut match_row_ctr: u64 = 0;
        let mut total_matches: u64 = 0;
        let mut matched_rows = String::with_capacity(20);
        #[allow(unused_assignments)]
        let mut match_list_with_row = String::with_capacity(20);

        for chunk in all_chunks {
            for result in chunk {
                let mut record = result.record;
                let matched = result.matched;
                let match_list = result.match_list;

                if matched {
                    match_row_ctr += 1;
                }

                if do_match_list {
                    let flag_column = if matched {
                        itoa::Buffer::new()
                            .format(result.row_number)
                            .clone_into(&mut matched_rows);
                        if self.flag_invert_match {
                            matched_rows.as_bytes().to_vec()
                        } else {
                            total_matches += match_list.len() as u64;
                            // builds format!("{matched_rows};{match_list}")
                            // without intermediate Vec allocation
                            match_list_with_row.clear();
                            match_list_with_row.push_str(&matched_rows);
                            match_list_with_row.push(';');
                            for (idx, i) in match_list.iter().enumerate() {
                                if idx > 0 {
                                    match_list_with_row.push(',');
                                }
                                match_list_with_row.push_str(&regex_labels[*i - 1]);
                            }
                            match_list_with_row.as_bytes().to_vec()
                        }
                    } else {
                        b"0".to_vec()
                    };

                    if self.flag_flag_matches_only && !matched {
                        if self.flag_unmatched_output.is_some() {
                            unmatched_wtr.write_byte_record(&record)?;
                        }
                        continue;
                    }
                    record.push_field(&flag_column);
                    wtr.write_byte_record(&record)?;
                } else if matched {
                    wtr.write_byte_record(&record)?;
                }
            }
        }

        unmatched_wtr.flush()?;
        wtr.flush()?;

        let record_count = idx_count as u64;
        if self.flag_json {
            let json = json!({
                "rows_with_matches": match_row_ctr,
                "total_matches": total_matches,
                "record_count": record_count,
            });
            eprintln!("{json}");
        } else {
            if self.flag_count {
                if !self.flag_quiet {
                    eprintln!("{match_row_ctr}");
                }
                info!("matches: {match_row_ctr}");
            }

            if match_row_ctr == 0 && !self.flag_not_one {
                return Err(CliError::NoMatch());
            }
        }

        Ok(())
    }
}
