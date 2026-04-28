static USAGE: &str = r#"
Filters CSV data by whether the given regex matches a row.

The regex is applied to selected field in each row, and if any field matches,
then the row is written to the output, and the number of matches to stderr.

The columns to search can be limited with the '--select' flag (but the full row
is still written to the output if there is a match).

Returns exitcode 0 when matches are found.
Returns exitcode 1 when no match is found, unless the '--not-one' flag is used.
Use --count to also write the number of matches to stderr (suppressed by --quiet and --json).

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
                           but will instead flag every row in a new column
                           named <column>, set to the row number for matched
                           rows and "0" for non-matched rows.
                           SPECIAL: if <column> is exactly "M", only matched
                           rows are returned AND only the M column is written
                           (all other columns are dropped). To use a literal
                           column name "M" without this behavior, rename it
                           afterward (e.g., with `qsv rename`).
    -Q, --quick            Return on first match with an exitcode of 0, returning
                           the row number of the first match to stderr.
                           Return exit code 1 if no match is found.
                           No output is produced.
    --preview-match <arg>  Preview the first N matches OR all matches found
                           within N milliseconds, whichever occurs first.
                           NOTE: the same numeric value is used for BOTH the
                           match count AND the millisecond timeout - choose a
                           value where one bound effectively dominates (e.g.,
                           a small count for "first N" preview, or a large
                           count for "all within N ms").
                           Returns the preview to stderr; output is still
                           written to stdout or --output as usual.
                           Forces a sequential search, even if the CSV is indexed.
    -c, --count            Write the number of matches to stderr.
                           Suppressed by --quiet and --json.
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
                           Automatically sets --quiet (also suppresses --count).
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
                           Disabled when running parallel search (i.e., when
                           the CSV is indexed and --jobs > 1). Sequential
                           search on an indexed CSV (--jobs 1) still shows
                           the progress bar.
    -q, --quiet            Do not write the match count (--count) or the
                           first match row number reported by --quick to stderr.
"#;

use std::{
    collections::BTreeMap,
    fs,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
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

// SearchResult holds a record that needs to be written to output.
// In filter mode (no --flag), only matched records are produced.
// In flag mode (--flag), every record is produced so the flag column can be set.
struct SearchResult {
    row_number: u64,
    record:     csv::ByteRecord,
    matched:    bool,
}

// ChunkOutput is what each parallel worker sends back over the channel.
// In --quick mode, `records` is empty and only `first_match_row` is populated.
// In normal mode, `first_match_row` is None; `records` holds the rows the worker
// has decided need to be written, and `match_count` is the worker's tally.
struct ChunkOutput {
    chunk_index:     usize,
    records:         Vec<SearchResult>,
    match_count:     u64,
    first_match_row: Option<u64>,
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

/// Check if preview collection should continue.
/// Returns true if still within both N matches and N milliseconds.
/// Caller is responsible for gating on `preview_limit > 0`.
#[inline]
fn should_collect_preview(
    preview_count: usize,
    start_time: std::time::Instant,
    preview_limit: usize,
) -> bool {
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

        // skip the opening '[' in quick mode since we return early
        // without writing any records and would never write the closing ']'
        if flag_json && !flag_quick {
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

        // Handle quick mode separately. --json implies --quiet per USAGE,
        // so the row-number print is also suppressed when --json is set.
        if self.flag_quick {
            if !(self.flag_quiet || self.flag_json) {
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

        // Lowest chunk_index that has reported a match in --quick mode (or
        // usize::MAX if none yet). A chunk can stop scanning only when a
        // STRICTLY LOWER-indexed chunk has matched - if its own or a
        // higher-indexed chunk has matched, this chunk may still contain an
        // earlier match in row order and must keep going. This preserves the
        // sequential "first match in row order" semantic under parallelism.
        let lowest_match_chunk = Arc::new(AtomicUsize::new(usize::MAX));

        // Create thread pool and channel
        let pool = ThreadPool::new(njobs);
        let (send, recv) = crossbeam_channel::bounded::<CliResult<ChunkOutput>>(nchunks);

        // Share Args across workers via Arc to avoid a per-worker clone
        // of SelectColumns and other inner allocations.
        let args = Arc::new(self.clone());

        // Spawn search jobs
        for chunk_index in 0..nchunks {
            let (send, args, sel, pattern, lowest_match) = (
                send.clone(),
                Arc::clone(&args),
                sel.clone(),
                Arc::clone(&pattern),
                Arc::clone(&lowest_match_chunk),
            );
            pool.execute(move || {
                let result: CliResult<ChunkOutput> = (|| {
                    let mut idx = args
                        .rconfig()
                        .indexed()?
                        .ok_or_else(|| CliError::Other("CSV index unavailable".to_string()))?;
                    idx.seek((chunk_index * chunk_size) as u64)?;
                    let it = idx.byte_records().take(chunk_size);
                    let start_row = (chunk_index * chunk_size) as u64 + 1;

                    if flag_quick {
                        // --quick: only track the earliest match in this chunk.
                        // No record allocation; stop as soon as we find one.
                        // Skip the rest of the chunk if a strictly lower-indexed
                        // chunk has already matched (no earlier row possible here).
                        for (row_number, record_result) in (start_row..).zip(it) {
                            if lowest_match.load(Ordering::Relaxed) < chunk_index {
                                break;
                            }
                            let record = record_result?;
                            let matched = if invert_match {
                                !sel.select(&record).any(|f| pattern.is_match(f))
                            } else {
                                sel.select(&record).any(|f| pattern.is_match(f))
                            };
                            if matched {
                                // Publish this chunk as the lowest matching chunk
                                // (only if it's strictly lower than the current value).
                                let mut current = lowest_match.load(Ordering::Relaxed);
                                while chunk_index < current {
                                    match lowest_match.compare_exchange_weak(
                                        current,
                                        chunk_index,
                                        Ordering::Relaxed,
                                        Ordering::Relaxed,
                                    ) {
                                        Ok(_) => break,
                                        Err(c) => current = c,
                                    }
                                }
                                return Ok(ChunkOutput {
                                    chunk_index,
                                    records: Vec::new(),
                                    match_count: 1,
                                    first_match_row: Some(row_number),
                                });
                            }
                        }
                        return Ok(ChunkOutput {
                            chunk_index,
                            records: Vec::new(),
                            match_count: 0,
                            first_match_row: None,
                        });
                    }

                    // Normal mode: only retain records we actually need to write.
                    // In flag mode (flag_flag), every row is needed so the flag column
                    // can be populated. In filter mode, only matched rows are needed.
                    let mut records: Vec<SearchResult> = Vec::new();
                    let mut match_count: u64 = 0;
                    for (row_number, record_result) in (start_row..).zip(it) {
                        let record = record_result?;
                        let matched = if invert_match {
                            !sel.select(&record).any(|f| pattern.is_match(f))
                        } else {
                            sel.select(&record).any(|f| pattern.is_match(f))
                        };
                        if matched {
                            match_count += 1;
                        }
                        if flag_flag || matched {
                            records.push(SearchResult {
                                row_number,
                                record,
                                matched,
                            });
                        }
                    }
                    Ok(ChunkOutput {
                        chunk_index,
                        records,
                        match_count,
                        first_match_row: None,
                    })
                })();
                // If the receiver has already been dropped (e.g., main thread
                // returned early on another worker's error), discard quietly.
                let _ = send.send(result);
            });
        }
        drop(send);

        // --quick mode: collect each worker's earliest-match-in-chunk and
        // pick the lowest chunk_index that reported Some(row).
        if self.flag_quick {
            let mut earliest: Option<(usize, u64)> = None;
            for chunk_msg in &recv {
                let chunk = chunk_msg?;
                if let Some(row) = chunk.first_match_row {
                    match earliest {
                        None => earliest = Some((chunk.chunk_index, row)),
                        Some((idx, _)) if chunk.chunk_index < idx => {
                            earliest = Some((chunk.chunk_index, row));
                        },
                        _ => {},
                    }
                }
            }
            if let Some((_, row)) = earliest {
                // --json implies --quiet per USAGE, so suppress the row print
                // when --json is set.
                if !(self.flag_quiet || self.flag_json) {
                    eprintln!("{row}");
                }
                info!("quick search first match at {row}");
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

        // Stream chunks in row order as they arrive. Out-of-order chunks
        // are buffered in `pending` until their predecessor lands; this caps
        // memory at roughly the number of in-flight workers worth of rows
        // rather than the whole file.
        let mut pending: BTreeMap<usize, ChunkOutput> = BTreeMap::new();
        let mut next_chunk: usize = 0;

        for chunk_msg in &recv {
            let chunk = chunk_msg?;
            pending.insert(chunk.chunk_index, chunk);

            while let Some(chunk) = pending.remove(&next_chunk) {
                match_ctr += chunk.match_count;
                for result in chunk.records {
                    let mut record = result.record;
                    write_result_record(
                        &mut record,
                        result.row_number,
                        result.matched,
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
                next_chunk += 1;
            }
        }

        // Use helper to finalize output
        self.finalize_output(match_ctr, wtr, json_wtr)
    }
}
