static USAGE: &str = r#"
Filters CSV data by whether the given regex matches a row.

The regex is applied to selected field in each row, and if any field matches,
then the row is written to the output, and the number of matches to stderr.

The columns to search can be limited with the '--select' flag (but the full row
is still written to the output if there is a match).

Returns exitcode 0 when matches are found, returning number of matches to stderr.
Returns exitcode 1 when no match is found, unless the '--not-one' flag is used.

When --quick is enabled, no output is produced and exitcode 0 is returned on 
the first match.

When the CSV is indexed, a faster parallel search is used.

Examples:

  # Search for rows where any field contains the regex 'foo.*bar' (case sensitive)
  qsv search 'foo.*bar' data.csv

  # Case insensitive search for 'error' in the 'message' column
  qsv search -i 'error' -s message data.csv

  # Search for exact matches of 'completed' in the 'status' column
  qsv search --exact 'completed' -s status data.csv

  # Search for literal string 'a.b*c' in all columns
  qsv search --literal 'a.b*c' data.csv

  # Invert match: select rows that do NOT match the regex 'test'
  qsv search --invert-match 'test' data.csv

  # Flag matched rows in a new column named 'match_flag'
  qsv search --flag match_flag 'pattern' data.csv

  # Quick search: return on first match of 'urgent' in the 'subject' column
  qsv search --quick 'urgent' -s subject data.csv

  # Preview the first 5 matches of 'warning' in all columns
  qsv search --preview-match 5 'warning' data.csv

For examples, see https://github.com/dathere/qsv/blob/master/tests/test_search.rs.

Usage:
    qsv search [options] <regex> [<input>]
    qsv search --help

search arguments:
    <regex>                Regular expression to match. Uses Rust regex syntax.
                           See https://docs.rs/regex/latest/regex/index.html#syntax
                           or https://regex101.com with the Rust flavor for more info.
    <input>                The CSV file to read. If not given, reads from stdin.

search options:
    -i, --ignore-case      Case insensitive search. This is equivalent to
                           prefixing the regex with '(?i)'.
    --literal              Treat the regex as a literal string. This allows you to
                           search for matches that contain regex special characters.
    --exact                Match the ENTIRE field exactly. Treats the pattern
                           as a literal string (like --literal) and automatically
                           anchors it to match the complete field value (^pattern$).
    -s, --select <arg>     Select the columns to search. See 'qsv select -h'
                           for the full syntax.
    -v, --invert-match     Select only rows that did not match
    -u, --unicode          Enable unicode support. When enabled, character classes
                           will match all unicode word characters instead of only
                           ASCII word characters. Decreases performance.
    -f, --flag <column>    If given, the command will not filter rows
                           but will instead flag the found rows in a new
                           column named <column>, with the row numbers
                           of the matched rows and 0 for the non-matched rows.
                           If column is named M, only the M column will be written
                           to the output, and only matched rows are returned.
    -Q, --quick            Return on first match with an exitcode of 0, returning
                           the row number of the first match to stderr.
                           Return exit code 1 if no match is found.
                           No output is produced.
    --preview-match <arg>  Preview the first N matches or all the matches found in
                           N milliseconds, whichever occurs first. Returns the preview to
                           stderr. Output is still written to stdout or --output as usual.
                           Only applicable when CSV is NOT indexed, as it's read sequentially.
                           Forces a sequential search, even if the CSV is indexed.
    -c, --count            Return number of matches to stderr.
    --size-limit <mb>      Set the approximate size limit (MB) of the compiled
                           regular expression. If the compiled expression exceeds this 
                           number, then a compilation error is returned.
                           Modify this only if you're getting regular expression
                           compilation errors. [default: 50]
    --dfa-size-limit <mb>  Set the approximate size of the cache (MB) used by the regular
                           expression engine's Discrete Finite Automata.
                           Modify this only if you're getting regular expression
                           compilation errors. [default: 10]
    --json                 Output the result as JSON. Fields are written
                           as key-value pairs. The key is the column name.
                           The value is the field value. The output is a
                           JSON array. If --no-headers is set, then
                           the keys are the column indices (zero-based).
                           Automatically sets --quiet.
    --not-one              Use exit code 0 instead of 1 for no match found.
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
                           Only applicable when CSV is NOT indexed.
    -q, --quiet            Do not return number of matches to stderr.
"#;

use std::{
    fs,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use crossbeam_channel;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
use indicatif::{HumanCount, ProgressBar, ProgressDrawTarget};
use log::info;
use regex::bytes::RegexBuilder;
use serde::Deserialize;
use threadpool::ThreadPool;

use crate::{
    CliError, CliResult,
    config::{Config, DEFAULT_WTR_BUFFER_CAPACITY, Delimiter},
    index::Indexed,
    select::SelectColumns,
    util,
};

#[allow(dead_code)]
#[derive(Deserialize, Clone)]
struct Args {
    arg_input:           Option<String>,
    arg_regex:           String,
    flag_exact:          bool,
    flag_literal:        bool,
    flag_select:         SelectColumns,
    flag_output:         Option<String>,
    flag_no_headers:     bool,
    flag_delimiter:      Option<Delimiter>,
    flag_invert_match:   bool,
    flag_unicode:        bool,
    flag_ignore_case:    bool,
    flag_flag:           Option<String>,
    flag_size_limit:     usize,
    flag_dfa_size_limit: usize,
    flag_json:           bool,
    flag_not_one:        bool,
    flag_preview_match:  Option<usize>,
    flag_quick:          bool,
    flag_count:          bool,
    flag_progressbar:    bool,
    flag_quiet:          bool,
    flag_jobs:           Option<usize>,
}

// SearchResult holds information about a search result for parallel processing
struct SearchResult {
    row_number: u64,
    record:     csv::ByteRecord,
    matched:    bool,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let regex_unicode = if util::get_envvar_flag("QSV_REGEX_UNICODE") {
        true
    } else {
        args.flag_unicode
    };

    let arg_regex = if args.flag_literal {
        regex::escape(&args.arg_regex)
    } else if args.flag_exact {
        format!("^{}$", regex::escape(&args.arg_regex))
    } else {
        args.arg_regex.clone()
    };

    let pattern = RegexBuilder::new(&arg_regex)
        .case_insensitive(args.flag_ignore_case)
        .unicode(regex_unicode)
        .size_limit(args.flag_size_limit * (1 << 20))
        .dfa_size_limit(args.flag_dfa_size_limit * (1 << 20))
        .build()?;

    let rconfig = Config::new(args.arg_input.as_ref())
        .delimiter(args.flag_delimiter)
        .no_headers_flag(args.flag_no_headers)
        .select(args.flag_select.clone());

    // Route to parallel or sequential search
    // based on index availability, number of jobs, and --preview-match option
    if let Some(idx) = rconfig.indexed()?
        && util::njobs(args.flag_jobs) > 1
        && args.flag_preview_match.is_none()
    {
        args.parallel_search(&idx, pattern, &rconfig)
    } else {
        args.sequential_search(&pattern, &rconfig)
    }
}

/// Check if preview collection should continue
/// Returns true if still within both N matches and N milliseconds
#[inline]
fn should_collect_preview(
    preview_count: usize,
    start_time: std::time::Instant,
    preview_limit: usize,
) -> bool {
    if preview_limit == 0 {
        return false;
    }
    preview_count < preview_limit && start_time.elapsed().as_millis() < preview_limit as u128
}

/// Write a single result record to output
/// Returns true if the record was written (for match counting)
#[allow(clippy::too_many_arguments)]
#[allow(clippy::fn_params_excessive_bools)]
#[allow(clippy::inline_always)]
#[inline(always)]
fn write_result_record(
    record: &mut csv::ByteRecord,
    row_number: u64,
    matched: bool,
    flag_flag: bool,
    flag_json: bool,
    flag_no_headers: bool,
    matches_only: bool,
    headers: &csv::ByteRecord,
    wtr: &mut csv::Writer<Box<dyn std::io::Write>>,
    json_wtr: &mut Box<dyn std::io::Write>,
    is_first: &mut bool,
    matched_rows: &mut String,
) -> CliResult<bool> {
    if flag_flag {
        let match_row = if matched {
            itoa::Buffer::new()
                .format(row_number)
                .clone_into(matched_rows);
            matched_rows.as_bytes()
        } else {
            b"0"
        };

        if matches_only && match_row == b"0" {
            return Ok(false);
        }

        if matches_only {
            record.clear();
        }
        record.push_field(match_row);

        if flag_json {
            util::write_json_record(json_wtr, flag_no_headers, headers, record, is_first)?;
        } else {
            wtr.write_byte_record(record)?;
        }
        Ok(true)
    } else if matched {
        if flag_json {
            util::write_json_record(json_wtr, flag_no_headers, headers, record, is_first)?;
        } else {
            wtr.write_byte_record(record)?;
        }
        Ok(true)
    } else {
        Ok(false)
    }
}

impl Args {
    fn rconfig(&self) -> Config {
        Config::new(self.arg_input.as_ref())
            .delimiter(self.flag_delimiter)
            .no_headers_flag(self.flag_no_headers)
            .select(self.flag_select.clone())
    }

    /// Setup flag column in headers if --flag option is used
    /// Returns (flag_flag: bool, matches_only: bool)
    fn setup_flag_column(&self, headers: &mut csv::ByteRecord) -> (bool, bool) {
        let mut matches_only = false;
        let flag_flag = self.flag_flag.as_ref().is_some_and(|column_name| {
            if column_name == "M" {
                headers.clear();
                matches_only = true;
            }
            headers.push_field(column_name.as_bytes());
            true
        });
        (flag_flag, matches_only)
    }

    /// Create CSV and JSON writers
    fn create_writers(
        &self,
    ) -> CliResult<(
        csv::Writer<Box<dyn std::io::Write>>,
        Box<dyn std::io::Write>,
    )> {
        let wtr = Config::new(self.flag_output.as_ref()).writer()?;
        let json_wtr = if self.flag_json {
            util::create_json_writer(self.flag_output.as_ref(), DEFAULT_WTR_BUFFER_CAPACITY * 4)?
        } else {
            Box::new(std::io::sink())
        };
        Ok((wtr, json_wtr))
    }

    /// Finalize output, write match count, and check for errors
    fn finalize_output(
        &self,
        match_ctr: u64,
        mut wtr: csv::Writer<Box<dyn std::io::Write>>,
        mut json_wtr: Box<dyn std::io::Write>,
    ) -> CliResult<()> {
        let flag_json = self.flag_json;

        if flag_json {
            json_wtr.write_all(b"]")?;
            json_wtr.flush()?;
        } else {
            wtr.flush()?;
        }

        if self.flag_count && !self.flag_quick {
            let flag_quiet = self.flag_quiet || self.flag_json;
            if !flag_quiet {
                eprintln!("{match_ctr}");
            }
            info!("matches: {match_ctr}");
        }

        if match_ctr == 0 && !self.flag_not_one {
            return Err(CliError::NoMatch());
        }

        Ok(())
    }

    /// Write preview records to stderr
    /// If --json is used, output as JSON array; otherwise output as CSV with summary line
    fn write_preview(
        &self,
        preview_records: &[csv::ByteRecord],
        headers: &csv::ByteRecord,
        records_processed: u64,
        elapsed_ms: u128,
    ) -> CliResult<()> {
        if preview_records.is_empty() {
            return Ok(());
        }

        if self.flag_json {
            // Output as JSON
            let mut json_array = Vec::with_capacity(preview_records.len());
            for record in preview_records {
                let mut obj = serde_json::Map::new();
                for (i, field) in record.iter().enumerate() {
                    let key = if self.flag_no_headers {
                        i.to_string()
                    } else {
                        String::from_utf8_lossy(&headers[i]).to_string()
                    };
                    let value = String::from_utf8_lossy(field);
                    let json_value = if value.is_empty() {
                        serde_json::Value::Null
                    } else {
                        serde_json::Value::String(value.to_string())
                    };
                    obj.insert(key, json_value);
                }
                json_array.push(serde_json::Value::Object(obj));
            }
            let json_output = serde_json::to_string(&json_array)?;
            eprint!("{json_output}");
        } else {
            // Output as CSV with summary
            let mut preview_wtr = csv::WriterBuilder::new()
                .flexible(true)
                .from_writer(std::io::stderr());

            // Write headers
            preview_wtr.write_record(headers)?;

            // Write preview records
            for record in preview_records {
                preview_wtr.write_byte_record(record)?;
            }

            preview_wtr.flush()?;

            // Write summary line
            eprintln!(
                "Previewed {} matches in {} initial records in {} ms",
                preview_records.len(),
                records_processed,
                elapsed_ms
            );
        }
        Ok(())
    }

    fn sequential_search(&self, pattern: &regex::bytes::Regex, rconfig: &Config) -> CliResult<()> {
        // args struct booleans in hot loop assigned to local variables
        // to help the compiler optimize the code & hopefully use registers
        let flag_quick = self.flag_quick;
        let flag_json = self.flag_json;
        let flag_no_headers = self.flag_no_headers;

        let mut rdr = rconfig.reader()?;
        let (mut wtr, mut json_wtr) = self.create_writers()?;

        let mut headers = rdr.byte_headers()?.clone();
        let sel = rconfig.selection(&headers)?;

        let (flag_flag, matches_only) = self.setup_flag_column(&mut headers);

        if !rconfig.no_headers && !flag_quick && !flag_json {
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
        let mut match_ctr: u64 = 0;
        let mut row_ctr: u64 = 0;
        let mut m;
        let invert_match = self.flag_invert_match;

        #[allow(unused_assignments)]
        let mut matched_rows = String::with_capacity(20); // to save on allocs

        let mut is_first = true;

        // Preview collection setup
        let preview_limit = self.flag_preview_match.unwrap_or(0);
        let mut preview_records: Vec<csv::ByteRecord> = if preview_limit > 0 {
            Vec::with_capacity(preview_limit)
        } else {
            Vec::new()
        };
        let preview_start = std::time::Instant::now();
        let mut collecting_preview = preview_limit > 0;

        if flag_json {
            json_wtr.write_all(b"[")?;
        }

        while rdr.read_byte_record(&mut record)? {
            row_ctr += 1;

            #[cfg(any(feature = "feature_capable", feature = "lite"))]
            if show_progress {
                progress.inc(1);
            }
            m = sel.select(&record).any(|f| pattern.is_match(f));
            if invert_match {
                m = !m;
            }
            if m {
                match_ctr += 1;

                // Collect for preview if still within limits
                if collecting_preview {
                    preview_records.push(record.clone());
                    collecting_preview =
                        should_collect_preview(preview_records.len(), preview_start, preview_limit);
                }

                if flag_quick {
                    break;
                }
            }

            // Use helper to write record if needed
            write_result_record(
                &mut record,
                row_ctr,
                m,
                flag_flag,
                flag_json,
                flag_no_headers,
                matches_only,
                &headers,
                &mut wtr,
                &mut json_wtr,
                &mut is_first,
                &mut matched_rows,
            )?;
        }

        #[cfg(any(feature = "feature_capable", feature = "lite"))]
        if show_progress {
            progress.set_message(format!(
                " - {} matches found in {} records.",
                HumanCount(match_ctr),
                HumanCount(progress.length().unwrap()),
            ));
            util::finish_progress(&progress);
        }

        // Write preview to stderr if collected
        if preview_limit > 0 {
            let elapsed_ms = preview_start.elapsed().as_millis();
            self.write_preview(&preview_records, &headers, row_ctr, elapsed_ms)?;
        }

        // Handle quick mode separately
        if self.flag_quick {
            if !self.flag_quiet {
                eprintln!("{row_ctr}");
            }
            info!("quick search first match at {row_ctr}");
            if match_ctr == 0 && !self.flag_not_one {
                return Err(CliError::NoMatch());
            }
            return Ok(());
        }

        // Use helper to finalize output
        self.finalize_output(match_ctr, wtr, json_wtr)
    }

    fn parallel_search(
        &self,
        idx: &Indexed<fs::File, fs::File>,
        pattern: regex::bytes::Regex,
        rconfig: &Config,
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
        let (flag_flag, matches_only) = self.setup_flag_column(&mut headers);

        // Wrap pattern in Arc for sharing across threads
        let pattern = Arc::new(pattern);
        let invert_match = self.flag_invert_match;
        let flag_quick = self.flag_quick;
        let flag_no_headers = self.flag_no_headers;

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
                let mut row_number = (i * chunk_size) as u64 + 1; // 1-based row numbering

                for record in it.flatten() {
                    // Early exit for quick mode if match already found by another thread
                    if flag_quick && match_found_flag.load(Ordering::Relaxed) {
                        break;
                    }

                    let matched = if invert_match {
                        !sel.select(&record).any(|f| pattern.is_match(f))
                    } else {
                        sel.select(&record).any(|f| pattern.is_match(f))
                    };

                    // Set flag if we found a match in quick mode
                    if flag_quick && matched {
                        match_found_flag.store(true, Ordering::Relaxed);
                    }

                    results.push(SearchResult {
                        row_number,
                        record,
                        matched,
                    });
                    row_number += 1;
                }
                send.send(results).unwrap();
            });
        }
        drop(send);

        // Collect all results from all chunks
        let mut all_chunks: Vec<Vec<SearchResult>> = Vec::with_capacity(nchunks);
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
                info!("quick search first match at {}", first_match.row_number);
                return Ok(());
            }
            // No match found
            if !self.flag_not_one {
                return Err(CliError::NoMatch());
            }
            return Ok(());
        }

        // Setup writers
        let flag_json = self.flag_json;
        let (mut wtr, mut json_wtr) = self.create_writers()?;

        // Write headers
        if !rconfig.no_headers && !flag_json {
            wtr.write_record(&headers)?;
        }

        // Write results
        let mut match_ctr: u64 = 0;
        let mut is_first = true;
        let mut matched_rows = String::with_capacity(20);

        if flag_json {
            json_wtr.write_all(b"[")?;
        }

        for chunk in all_chunks {
            for result in chunk {
                let mut record = result.record;
                let matched = result.matched;

                if matched {
                    match_ctr += 1;
                }

                // Use helper to write record if needed
                write_result_record(
                    &mut record,
                    result.row_number,
                    matched,
                    flag_flag,
                    flag_json,
                    flag_no_headers,
                    matches_only,
                    &headers,
                    &mut wtr,
                    &mut json_wtr,
                    &mut is_first,
                    &mut matched_rows,
                )?;
            }
        }

        // Use helper to finalize output
        self.finalize_output(match_ctr, wtr, json_wtr)
    }
}
