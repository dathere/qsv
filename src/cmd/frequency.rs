static USAGE: &str = r#"
Compute a frequency table on input data. It has CSV and JSON output modes.
https://en.wikipedia.org/wiki/Frequency_(statistics)#Frequency_distribution_table

In CSV output mode (default), the table is formatted as CSV data with the following
columns - field,value,count,percentage.

In JSON output mode, the table is formatted as nested JSON data. In addition to
the columns above, the JSON output also includes the row count, field count, each
field's data type, cardinality, nullcount, sparsity, uniqueness_ratio and its stats.

Since this command computes an exact frequency distribution table, memory proportional
to the cardinality of each column would be normally required.

However, this is problematic for columns with ALL unique values (e.g. an ID column),
as the command will need to allocate memory proportional to the column's cardinality,
potentially causing Out-of-Memory (OOM) errors for larger-than-memory datasets.

To overcome this, the frequency command uses the stats cache if it exists to get
column cardinality information. This short-circuits frequency compilation for columns
with all unique values (i.e. where rowcount == cardinality), eliminating the need to
maintain an in-memory hashmap for ID columns. This allows `frequency` to handle
larger-than-memory datasets with the added benefit of also making it faster when
working with datasets with ID columns.

This is also why it is highly recommended to index the CSV and run the stats command
first before running the frequency command.

When using the JSON output mode, note that boolean and date type inference are
disabled by default. If you want to infer dates and boolean types, you can
"prime" the stats cache by running the stats command with the `--infer-dates`
or `--infer-boolean` options with the `--stats-jsonl` option
(e.g. `qsv stats --infer-dates --infer-boolean --stats-jsonl <input>`).
This will allow the frequency command to use the "primed" stats cache to inherit
the already inferred dates and boolean types.

NOTE: "Complete" Frequency Tables:

    By default, ID columns will have an "<ALL UNIQUE>" value with count equal to
    rowcount and percentage set to 100. This is done by using the stats cache to
    fetch each column's cardinality - allowing qsv to short-circuit frequency
    compilation and eliminate the need to maintain a hashmap for ID columns.

    If you wish to compile a "complete" frequency table even for ID columns, set
    QSV_STATSCACHE_MODE to "none". This will force the frequency command to compute
    frequencies for all columns regardless of cardinality, even for ID columns.

    In this case, the unique limit (--unq-limit) option is particularly useful when
    a column has all unique values  and --limit is set to 0.
    Without a unique limit, the frequency table for that column will be the same as
    the number of rows in the data.
    With a unique limit, the frequency table will be a sample of N unique values,
    all with a count of 1.

    The --lmt-threshold option also allows you to apply the --limit and --unq-limit
    options only when the number of unique items in a column >= threshold.
    This is useful when you want to apply limits only to columns with a large number
    of unique items and not to columns with a small number of unique items.

For examples, see https://github.com/dathere/qsv/blob/master/tests/test_frequency.rs.

Usage:
    qsv frequency [options] [<input>]
    qsv frequency --help

frequency options:
    -s, --select <arg>      Select a subset of columns to compute frequencies
                            for. See 'qsv select --help' for the format
                            details. This is provided here because piping 'qsv
                            select' into 'qsv frequency' will disable the use
                            of indexing.
    -l, --limit <arg>       Limit the frequency table to the N most common
                            items. Set to '0' to disable a limit.
                            If negative, only return values with an occurrence
                            count >= absolute value of the negative limit.
                            e.g. --limit -2 will only return values with an
                            occurrence count >= 2.
                            [default: 10]
    -u, --unq-limit <arg>   If a column has all unique values, limit the
                            frequency table to a sample of N unique items.
                            Set to '0' to disable a unique_limit.
                            [default: 10]
    --lmt-threshold <arg>   The threshold for which --limit and --unq-limit
                            will be applied. If the number of unique items
                            in a column >= threshold, the limits will be applied.
                            Set to '0' to disable the threshold and always apply limits.
                            [default: 0]
    --pct-dec-places <arg>  The number of decimal places to round the percentage to.
                            If negative, the number of decimal places will be set
                            automatically to the minimum number of decimal places needed
                            to represent the percentage accurately, up to the absolute
                            value of the negative number.
                            [default: -5]
    --other-sorted          By default, the "Other" category is placed at the
                            end of the frequency table for a field. If this is enabled, the
                            "Other" category will be sorted with the rest of the
                            values by count.
    --other-text <arg>      The text to use for the "Other" category. If set to "<NONE>",
                            the "Other" category will not be included in the frequency table.
                            [default: Other]
    -a, --asc               Sort the frequency tables in ascending order by count.
                            The default is descending order.
    --no-trim               Don't trim whitespace from values when computing frequencies.
                            The default is to trim leading and trailing whitespaces.
    --no-nulls              Don't include NULLs in the frequency table.
    -i, --ignore-case       Ignore case when computing frequencies.
   --all-unique-text <arg>  The text to use for the "<ALL_UNIQUE>" category.
                            [default: <ALL_UNIQUE>]
    --vis-whitespace        Visualize whitespace characters in the output. See
                            https://github.com/dathere/qsv/wiki/Supplemental#whitespace-markers
                            for the list of whitespace markers.
    -j, --jobs <arg>        The number of jobs to run in parallel when the given CSV data has
                            an index. Note that a file handle is opened for each job.
                            When not set, defaults to the number of CPUs detected.

                            JSON OUTPUT OPTIONS:
    --json                  Output frequency table as nested JSON instead of CSV.
                            The JSON output includes row count, field count & each field's
                            data type, cardinality, null count, sparsity, uniqueness_ratio
                            and its stats.
    --no-stats              When using the JSON output mode, do not include stats.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -n, --no-headers       When set, the first row will NOT be included
                           in the frequency table. Additionally, the 'field'
                           column will be 1-based indices instead of header
                           names.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
    --memcheck             Check if there is enough memory to load the entire
                           CSV into memory using CONSERVATIVE heuristics.
"#;

use std::{fs, io, sync::OnceLock};

use crossbeam_channel;
use foldhash::{HashMap, HashMapExt};
use indicatif::HumanCount;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{self, Value as JsonValue};
use stats::{Frequencies, merge_all};
use threadpool::ThreadPool;

use crate::{
    CliResult,
    cmd::stats::StatsData,
    config::{Config, Delimiter},
    index::Indexed,
    select::{SelectColumns, Selection},
    util::{self, ByteString, StatsMode, get_stats_records},
};

#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Clone, Deserialize)]
pub struct Args {
    pub arg_input:            Option<String>,
    pub flag_select:          SelectColumns,
    pub flag_limit:           isize,
    pub flag_unq_limit:       usize,
    pub flag_lmt_threshold:   usize,
    pub flag_pct_dec_places:  isize,
    pub flag_other_sorted:    bool,
    pub flag_other_text:      String,
    pub flag_asc:             bool,
    pub flag_no_trim:         bool,
    pub flag_no_nulls:        bool,
    pub flag_ignore_case:     bool,
    pub flag_all_unique_text: String,
    pub flag_jobs:            Option<usize>,
    pub flag_output:          Option<String>,
    pub flag_no_headers:      bool,
    pub flag_delimiter:       Option<Delimiter>,
    pub flag_memcheck:        bool,
    pub flag_vis_whitespace:  bool,
    pub flag_json:            bool,
    pub flag_no_stats:        bool,
}

const NULL_VAL: &[u8] = b"(NULL)";
const NON_UTF8_ERR: &str = "<Non-UTF8 ERROR>";
const EMPTY_BYTE_VEC: Vec<u8> = Vec::new();
static STATS_RECORDS: OnceLock<HashMap<String, StatsData>> = OnceLock::new();

// FrequencyEntry, FrequencyField and FrequencyOutput are
// structs for JSON output
#[derive(Serialize)]
struct FrequencyEntry {
    value:      String,
    count:      u64,
    percentage: f64,
    rank:       u32,
}

#[derive(Serialize)]
struct FrequencyField {
    field:            String,
    r#type:           String,
    cardinality:      u64,
    nullcount:        u64,
    sparsity:         f64,
    uniqueness_ratio: f64,
    stats:            Vec<FieldStats>,
    frequencies:      Vec<FrequencyEntry>,
}

#[derive(Serialize, Clone)]
struct FieldStats {
    name:  String,
    value: JsonValue,
}

#[derive(Serialize)]
struct FrequencyOutput {
    input:       String,
    description: String,
    rowcount:    u64,
    fieldcount:  usize,
    fields:      Vec<FrequencyField>,
}

// Shared frequency processing result
// used by both CSV and JSON output
#[derive(Clone)]
struct ProcessedFrequency {
    count:                u64,
    percentage:           f64,
    formatted_percentage: String,
    value:                Vec<u8>,
    rank:                 u32,
}

static UNIQUE_COLUMNS_VEC: OnceLock<Vec<usize>> = OnceLock::new();
static COL_CARDINALITY_VEC: OnceLock<Vec<(String, u64)>> = OnceLock::new();
static FREQ_ROW_COUNT: OnceLock<u64> = OnceLock::new();

pub fn run(argv: &[&str]) -> CliResult<()> {
    let mut args: Args = util::get_args(USAGE, argv)?;
    let mut rconfig = args.rconfig();

    let is_stdin = rconfig.is_stdin();

    // if stdin and args.flag_json is true, save stdin to tempfile
    // so we can derive stats
    let mut stdin_temp_file;
    if is_stdin && args.flag_json {
        let temp_dir = std::env::temp_dir();
        stdin_temp_file = tempfile::Builder::new()
            .suffix(".csv")
            .tempfile_in(&temp_dir)?;
        io::copy(&mut io::stdin(), &mut stdin_temp_file)?;
        args.arg_input = Some(stdin_temp_file.path().to_string_lossy().to_string());
        rconfig = args.rconfig();
    }

    // we're loading the entire file into memory, we need to check avail mem
    if let Some(path) = rconfig.path.clone() {
        util::mem_file_check(&path, false, args.flag_memcheck)?;
    }

    let (headers, tables) = match args.rconfig().indexed()? {
        Some(ref mut idx) if util::njobs(args.flag_jobs) > 1 => args.parallel_ftables(idx),
        _ => args.sequential_ftables(),
    }?;

    if args.flag_json {
        return args.output_json(&headers, tables, &rconfig, argv, is_stdin);
    }

    // amortize allocations
    #[allow(unused_assignments)]
    let mut header_vec: Vec<u8> = Vec::with_capacity(tables.len());
    let mut itoa_buffer = itoa::Buffer::new();
    let mut itoa_buffer_rank = itoa::Buffer::new();
    let mut row: Vec<&[u8]>;

    let head_ftables = headers.iter().zip(tables);
    let row_count = *FREQ_ROW_COUNT.get().unwrap_or(&0);
    let abs_dec_places = args.flag_pct_dec_places.unsigned_abs() as u32;

    #[allow(unused_assignments)]
    let mut processed_frequencies: Vec<ProcessedFrequency> = Vec::with_capacity(head_ftables.len());
    #[allow(unused_assignments)]
    let mut value_str = String::with_capacity(100);

    // safety: we know that UNIQUE_COLUMNS has been previously set
    // when compiling frequencies by sel_headers fn
    let unique_headers_vec = UNIQUE_COLUMNS_VEC.get().unwrap();

    let mut wtr = Config::new(args.flag_output.as_ref()).writer()?;
    wtr.write_record(vec!["field", "value", "count", "percentage", "rank"])?;

    for (i, (header, ftab)) in head_ftables.enumerate() {
        header_vec = if rconfig.no_headers {
            (i + 1).to_string().into_bytes()
        } else {
            header.to_vec()
        };

        let all_unique_header = unique_headers_vec.contains(&i);
        args.process_frequencies(
            all_unique_header,
            abs_dec_places,
            row_count,
            &ftab,
            &mut processed_frequencies,
        );

        for processed_freq in &processed_frequencies {
            row = vec![
                &*header_vec,
                if args.flag_vis_whitespace {
                    value_str =
                        util::visualize_whitespace(&String::from_utf8_lossy(&processed_freq.value));
                    value_str.as_bytes()
                } else {
                    &processed_freq.value
                },
                itoa_buffer.format(processed_freq.count).as_bytes(),
                processed_freq.formatted_percentage.as_bytes(),
                itoa_buffer_rank.format(processed_freq.rank).as_bytes(),
            ];
            wtr.write_record(row)?;
        }
        // Clear the vector for the next iteration
        processed_frequencies.clear();
    }
    Ok(wtr.flush()?)
}

type Headers = csv::ByteRecord;
type FTable = Frequencies<Vec<u8>>;
type FTables = Vec<Frequencies<Vec<u8>>>;

impl Args {
    pub fn rconfig(&self) -> Config {
        Config::new(self.arg_input.as_ref())
            .delimiter(self.flag_delimiter)
            .no_headers(self.flag_no_headers)
            .select(self.flag_select.clone())
    }

    /// Shared frequency processing function used by both CSV and JSON output
    fn process_frequencies(
        &self,
        all_unique_header: bool,
        abs_dec_places: u32,
        row_count: u64,
        ftab: &FTable,
        processed_frequencies: &mut Vec<ProcessedFrequency>,
    ) {
        if all_unique_header {
            // For all-unique headers, create a single entry
            let all_unique_text = self.flag_all_unique_text.as_bytes().to_vec();
            let formatted_pct = self.format_percentage(100.0, abs_dec_places);
            processed_frequencies.push(ProcessedFrequency {
                value:                all_unique_text,
                count:                row_count,
                percentage:           100.0,
                formatted_percentage: formatted_pct,
                rank:                 1, // Rank 1 for all-unique headers
            });
        } else {
            // Process regular frequencies
            let mut counts_to_process = self.counts(ftab);
            if !self.flag_other_sorted
                && counts_to_process.first().is_some_and(|(value, _, _, _)| {
                    value.starts_with(format!("{} (", self.flag_other_text).as_bytes())
                })
            {
                counts_to_process.rotate_left(1);
            }

            // Convert to processed frequencies
            for (value, count, percentage, rank) in counts_to_process {
                let formatted_pct = self.format_percentage(percentage, abs_dec_places);
                processed_frequencies.push(ProcessedFrequency {
                    value,
                    count,
                    percentage,
                    formatted_percentage: formatted_pct,
                    rank,
                });
            }
        }
    }

    /// Format percentage with proper decimal places
    fn format_percentage(&self, percentage: f64, abs_dec_places: u32) -> String {
        let pct_decimal = Decimal::from_f64(percentage).unwrap_or_default();
        let pct_scale = if self.flag_pct_dec_places < 0 {
            let current_scale = pct_decimal.scale();
            if current_scale > abs_dec_places {
                current_scale
            } else {
                abs_dec_places
            }
        } else {
            abs_dec_places
        };
        let final_pct_decimal = pct_decimal
            .round_dp_with_strategy(
                pct_scale,
                rust_decimal::RoundingStrategy::MidpointAwayFromZero,
            )
            .normalize();
        if final_pct_decimal.fract().to_string().len() > abs_dec_places as usize {
            final_pct_decimal
                .round_dp_with_strategy(abs_dec_places, RoundingStrategy::MidpointAwayFromZero)
                .normalize()
                .to_string()
        } else {
            final_pct_decimal.to_string()
        }
    }

    #[inline]
    fn counts(&self, ftab: &FTable) -> Vec<(ByteString, u64, f64, u32)> {
        let (mut counts, total_count) = if self.flag_asc {
            // parallel sort in ascending order - least frequent values first
            ftab.par_frequent(true)
        } else {
            // parallel sort in descending order - most frequent values first
            ftab.par_frequent(false)
        };

        // check if we need to apply limits
        let unique_counts_len = counts.len();
        if self.flag_lmt_threshold == 0 || self.flag_lmt_threshold >= unique_counts_len {
            // check if the column has all unique values
            // do this by looking at the counts vec
            // and see if it has a count of 1, indicating all unique values
            let all_unique = counts[if self.flag_asc {
                unique_counts_len - 1
            } else {
                0
            }]
            .1 == 1;

            let abs_limit = self.flag_limit.unsigned_abs();
            let unique_limited = if all_unique
                && self.flag_limit > 0
                && self.flag_unq_limit != abs_limit
                && self.flag_unq_limit > 0
            {
                counts.truncate(self.flag_unq_limit);
                true
            } else {
                false
            };

            // check if we need to limit the number of values
            if self.flag_limit > 0 {
                counts.truncate(abs_limit);
            } else if self.flag_limit < 0 && !unique_limited {
                // if limit is negative, only return values with an occurrence count >= absolute
                // value of the negative limit. We only do this if we haven't
                // already unique limited the values
                let count_limit = abs_limit as u64;
                counts.retain(|(_, count)| *count >= count_limit);
            }
        }

        let mut pct_sum = 0.0_f64;
        let mut pct: f64;
        let mut count_sum = 0_u64;
        let pct_factor = if total_count > 0 {
            100.0_f64 / total_count.to_f64().unwrap_or(1.0_f64)
        } else {
            0.0_f64
        };

        // Pre-allocate the result vector with known capacity
        // We might add an "Other" entry, so add 1 to capacity
        let mut counts_final: Vec<(Vec<u8>, u64, f64, u32)> = Vec::with_capacity(counts.len() + 1);

        // Create NULL value once to avoid repeated to_vec allocations
        let null_val = NULL_VAL.to_vec();

        // Group by count to handle ties
        let mut count_groups: Vec<(u64, Vec<Vec<u8>>)> = Vec::new();
        let mut current_count = None;
        let mut current_group = Vec::new();

        for (byte_string, count) in counts {
            if let Some(prev_count) = current_count
                && count != prev_count
                && !current_group.is_empty()
            {
                count_groups.push((prev_count, current_group));
                current_group = Vec::new();
            }

            current_count = Some(count);
            current_group.push(byte_string.clone());
        }
        if !current_group.is_empty() {
            count_groups.push((current_count.unwrap(), current_group));
        }

        // Sort each group alphabetically and assign ranks
        let mut current_rank = 1u32;
        for (count, mut group) in count_groups {
            // Sort the group alphabetically
            group.sort_by(|a, b| {
                let a_str = String::from_utf8_lossy(a);
                let b_str = String::from_utf8_lossy(b);
                a_str.cmp(&b_str)
            });

            for byte_string in group {
                count_sum += count;
                pct = count as f64 * pct_factor;
                pct_sum += pct;

                if byte_string.is_empty() {
                    counts_final.push((null_val.clone(), count, pct, current_rank));
                } else {
                    counts_final.push((byte_string, count, pct, current_rank));
                }
            }
            current_rank += 1;
        }

        let other_count = total_count - count_sum;
        if other_count > 0 && self.flag_other_text != "<NONE>" {
            let other_unique_count = unique_counts_len - counts_final.len();
            counts_final.push((
                format!(
                    "{} ({})",
                    self.flag_other_text,
                    HumanCount(other_unique_count as u64)
                )
                .as_bytes()
                .to_vec(),
                other_count,
                100.0_f64 - pct_sum,
                0, // Special rank for "Other" category
            ));
        }
        counts_final
    }

    pub fn sequential_ftables(&self) -> CliResult<(Headers, FTables)> {
        let mut rdr = self.rconfig().reader()?;
        let (headers, sel) = self.sel_headers(&mut rdr)?;
        Ok((headers, self.ftables(&sel, rdr.byte_records(), 1)))
    }

    pub fn parallel_ftables(
        &self,
        idx: &Indexed<fs::File, fs::File>,
    ) -> CliResult<(Headers, FTables)> {
        let mut rdr = self.rconfig().reader()?;
        let (headers, sel) = self.sel_headers(&mut rdr)?;

        let idx_count = idx.count() as usize;
        if idx_count == 0 {
            return Ok((headers, vec![]));
        }

        let njobs = util::njobs(self.flag_jobs);
        let chunk_size = util::chunk_size(idx_count, njobs);
        let nchunks = util::num_of_chunks(idx_count, chunk_size);

        let pool = ThreadPool::new(njobs);
        let (send, recv) = crossbeam_channel::bounded(nchunks);
        for i in 0..nchunks {
            let (send, args, sel) = (send.clone(), self.clone(), sel.clone());
            pool.execute(move || {
                // safety: we know the file is indexed and seekable
                let mut idx = args.rconfig().indexed().unwrap().unwrap();
                idx.seek((i * chunk_size) as u64).unwrap();
                let it = idx.byte_records().take(chunk_size);
                send.send(args.ftables(&sel, it, nchunks)).unwrap();
            });
        }
        drop(send);
        Ok((headers, merge_all(recv.iter()).unwrap()))
    }

    #[inline]
    fn ftables<I>(&self, sel: &Selection, it: I, nchunks: usize) -> FTables
    where
        I: Iterator<Item = csv::Result<csv::ByteRecord>>,
    {
        let nsel = sel.normal();
        let nsel_len = nsel.len();

        #[allow(unused_assignments)]
        // Optimize buffer allocations
        let mut field_buffer: Vec<u8> = Vec::with_capacity(1024);
        let mut row_buffer: csv::ByteRecord = csv::ByteRecord::with_capacity(200, nsel_len);
        let mut string_buf = String::with_capacity(512);

        let unique_headers_vec = UNIQUE_COLUMNS_VEC.get().unwrap();

        // assign flags to local variables for faster access
        let flag_no_nulls = self.flag_no_nulls;
        let flag_ignore_case = self.flag_ignore_case;
        let flag_no_trim = self.flag_no_trim;

        // compile a vector of bool flags for all_unique_headers
        // so we can skip the contains check in the hot loop below
        let all_unique_flag_vec: Vec<bool> = (0..nsel_len)
            .map(|i| unique_headers_vec.contains(&i))
            .collect();

        // optimize the capacity of the freq_tables based on the cardinality of the columns
        // if sequential, use the cardinality from the stats cache
        // if parallel, use a default capacity of 1000 for non-unique columns
        let empty_vec = Vec::new();
        let col_cardinality_vec = COL_CARDINALITY_VEC.get().unwrap_or(&empty_vec);
        let mut freq_tables: Vec<_> = if col_cardinality_vec.is_empty() {
            (0..nsel_len)
                .map(|_| Frequencies::with_capacity(1000))
                .collect()
        } else {
            (0..nsel_len)
                .map(|i| {
                    let capacity = if all_unique_flag_vec[i] {
                        1
                    } else if nchunks == 1 {
                        col_cardinality_vec
                            .get(i)
                            .map_or(1000, |(_, cardinality)| *cardinality as usize)
                    } else {
                        // use cardinality and number of jobs to set the capacity
                        let cardinality = col_cardinality_vec
                            .get(i)
                            .map_or(1000, |(_, cardinality)| *cardinality as usize);
                        cardinality / nchunks
                    };
                    Frequencies::with_capacity(capacity)
                })
                .collect()
        };

        // Pre-compute function pointers for the hot path
        // instead of doing if chains repeatedly in the hot loop
        let process_field = if flag_ignore_case {
            if flag_no_trim {
                |field: &[u8], buf: &mut String| {
                    if let Ok(s) = simdutf8::basic::from_utf8(field) {
                        util::to_lowercase_into(s, buf);
                        buf.as_bytes().to_vec()
                    } else {
                        field.to_vec()
                    }
                }
            } else {
                |field: &[u8], buf: &mut String| {
                    if let Ok(s) = simdutf8::basic::from_utf8(field) {
                        util::to_lowercase_into(s.trim(), buf);
                        buf.as_bytes().to_vec()
                    } else {
                        trim_bs_whitespace(field).to_vec()
                    }
                }
            }
        } else if flag_no_trim {
            |field: &[u8], _buf: &mut String| field.to_vec()
        } else {
            // this is the default hot path, so inline it
            #[inline]
            |field: &[u8], _buf: &mut String| trim_bs_whitespace(field).to_vec()
        };

        for row in it {
            // safety: we know the row is valid
            row_buffer.clone_from(&unsafe { row.unwrap_unchecked() });
            for (i, field) in nsel.select(row_buffer.into_iter()).enumerate() {
                // safety: all_unique_flag_vec is pre-computed to have exactly nsel_len elements,
                // which matches the number of selected columns that we iterate over.
                // i will always be < nsel_len as it comes from enumerate() over the selected cols
                if unsafe { *all_unique_flag_vec.get_unchecked(i) } {
                    continue;
                }

                // safety: freq_tables is pre-allocated with nsel_len elements.
                // i will always be < nsel_len as it comes from enumerate() over the selected cols
                if !field.is_empty() {
                    // Reuse buffers instead of creating new ones
                    field_buffer = process_field(field, &mut string_buf);
                    unsafe {
                        freq_tables.get_unchecked_mut(i).add(field_buffer);
                    }
                } else if !flag_no_nulls {
                    // set to null (EMPTY_BYTES) as flag_no_nulls is false
                    unsafe {
                        freq_tables.get_unchecked_mut(i).add(EMPTY_BYTE_VEC);
                    }
                }
            }
        }
        // shrink the capacity of the freq_tables to the actual number of elements.
        // if sequential (nchunks == 1), we don't need to shrink the capacity as we
        // use cardinality to set the capacity of the freq_tables
        // if parallel (nchunks > 1), we need to shrink the capacity to avoid
        // over-allocating memory
        if nchunks > 1 {
            freq_tables.shrink_to_fit();
        }
        freq_tables
    }

    /// return the names of headers/columns that are unique identifiers
    /// (i.e. where cardinality == rowcount)
    /// Also stores the stats records in a hashmap for use when producing JSON output
    fn get_unique_headers(&self, headers: &Headers) -> CliResult<Vec<usize>> {
        // get the stats records for the entire CSV
        let schema_args = util::SchemaArgs {
            flag_enum_threshold:  0,
            flag_ignore_case:     self.flag_ignore_case,
            flag_strict_dates:    false,
            // we still get all the stats columns so we can use the stats cache
            flag_pattern_columns: crate::select::SelectColumns::parse("").unwrap(),
            flag_dates_whitelist: String::new(),
            flag_prefer_dmy:      false,
            flag_force:           false,
            flag_stdout:          false,
            flag_jobs:            Some(util::njobs(self.flag_jobs)),
            flag_polars:          false,
            flag_no_headers:      self.flag_no_headers,
            flag_delimiter:       self.flag_delimiter,
            arg_input:            self.arg_input.clone(),
            flag_memcheck:        false,
        };
        // initialize the stats records hashmap
        let mut stats_records_hashmap = if self.flag_json {
            HashMap::with_capacity(headers.len())
        } else {
            HashMap::new()
        };

        let (csv_fields, csv_stats, dataset_stats) =
            get_stats_records(&schema_args, StatsMode::Frequency)?;

        if csv_fields.is_empty() || csv_stats.len() != csv_fields.len() {
            // the stats cache does not exist or the number of fields & stats records
            // do not match. Just return an empty vector.
            // we're not going to be able to get the cardinalities, so
            // this signals that we just compute frequencies for all columns
            return Ok(Vec::new());
        }

        let col_cardinality_vec: Vec<(String, u64)> = csv_stats
            .iter()
            .enumerate()
            .map(|(i, stats_record)| {
                // get the column name and stats record
                // safety: we know that csv_fields and csv_stats have the same length
                let col_name = csv_fields.get(i).unwrap();
                let col_name_str = simdutf8::basic::from_utf8(col_name)
                    .unwrap_or(NON_UTF8_ERR)
                    .to_string();
                if self.flag_json {
                    // Store the stats record in the hashmap for later use
                    // when we're producing JSON output
                    stats_records_hashmap.insert(col_name_str.clone(), stats_record.clone());
                }
                (col_name_str, stats_record.cardinality)
            })
            .collect();

        // now, get the unique headers, where cardinality == rowcount
        let row_count = dataset_stats
            .get("qsv__rowcount")
            .and_then(|count| count.parse::<u64>().ok())
            .unwrap_or_else(|| util::count_rows(&self.rconfig()).unwrap_or_default());
        FREQ_ROW_COUNT.set(row_count).unwrap();

        // Most datasets have relatively few columns with all unique values (e.g. ID columns)
        // so pre-allocate space for 5 as a reasonable default capacity
        let mut all_unique_headers_vec: Vec<usize> = Vec::with_capacity(5);
        for (i, _header) in headers.iter().enumerate() {
            // safety: we know that col_cardinality_vec has the same length as headers
            // as it was constructed from csv_fields which has the same length as headers
            let cardinality = unsafe { col_cardinality_vec.get_unchecked(i).1 };

            if cardinality == row_count {
                all_unique_headers_vec.push(i);
            }
        }

        COL_CARDINALITY_VEC.get_or_init(|| col_cardinality_vec);

        if self.flag_json {
            // Store the stats records hashmap for later use
            // when we're producing JSON output
            STATS_RECORDS.set(stats_records_hashmap).unwrap();
        }

        Ok(all_unique_headers_vec)
    }

    fn output_json(
        &self,
        headers: &Headers,
        tables: FTables,
        rconfig: &Config,
        argv: &[&str],
        is_stdin: bool,
    ) -> CliResult<()> {
        let fieldcount = headers.len();

        // init vars and amortize allocations
        let mut fields = Vec::with_capacity(fieldcount);
        let head_ftables = headers.iter().zip(tables);
        let rowcount = *FREQ_ROW_COUNT.get().unwrap_or(&0);
        let unique_headers_vec = UNIQUE_COLUMNS_VEC.get().unwrap();
        let mut processed_frequencies = Vec::with_capacity(head_ftables.len());
        let abs_dec_places = self.flag_pct_dec_places.unsigned_abs() as u32;
        let stats_records = STATS_RECORDS.get();
        let mut field_stats: Vec<FieldStats> = Vec::with_capacity(17);

        for (i, (header, ftab)) in head_ftables.enumerate() {
            let field_name = if rconfig.no_headers {
                (i + 1).to_string()
            } else {
                String::from_utf8_lossy(header).to_string()
            };

            let all_unique_header = unique_headers_vec.contains(&i);
            self.process_frequencies(
                all_unique_header,
                abs_dec_places,
                rowcount,
                &ftab,
                &mut processed_frequencies,
            );

            // Sort frequencies by count if flag_other_sorted
            if self.flag_other_sorted {
                if self.flag_asc {
                    // ascending order
                    processed_frequencies.sort_by(|a, b| a.count.cmp(&b.count));
                } else {
                    // descending order
                    processed_frequencies.sort_by(|a, b| b.count.cmp(&a.count));
                }
            }

            // Calculate cardinality for this field
            // we do this instead of using the stats record's cardinality
            // so we can handle stdin which doesn't have a stats record
            let cardinality = if all_unique_header {
                // For all-unique fields, cardinality equals rowcount
                rowcount
            } else {
                // For regular fields, cardinality is the number of unique values in the original
                // table before any limits are applied
                ftab.len() as u64
            };

            // Get stats record for this field
            let stats_record = stats_records.and_then(|records| records.get(&field_name));

            // Get data type and nullcount from stats record
            let dtype = stats_record.map_or(String::new(), |sr| sr.r#type.clone());
            let nullcount = stats_record.map_or(0, |sr| sr.nullcount);
            let sparsity = util::round_num(nullcount as f64 / rowcount as f64, 4)
                .parse::<f64>()
                .unwrap_or(0.0);
            let uniqueness_ratio = util::round_num(cardinality as f64 / rowcount as f64, 4)
                .parse::<f64>()
                .unwrap_or(0.0);

            // Build stats vector from stats record if type is not empty and not NULL or Boolean
            if !self.flag_no_stats
                && !dtype.is_empty()
                && dtype.as_str() != "NULL"
                && dtype.as_str() != "Boolean"
                && let Some(sr) = stats_record
            {
                // Add all available stats if some
                add_stat(&mut field_stats, "sum", sr.sum);
                add_stat(&mut field_stats, "min", sr.min.clone());
                add_stat(&mut field_stats, "max", sr.max.clone());
                add_stat(&mut field_stats, "range", sr.range);
                add_stat(&mut field_stats, "sort_order", sr.sort_order.clone());

                // String-specific stats
                add_stat(&mut field_stats, "min_length", sr.min_length);
                add_stat(&mut field_stats, "max_length", sr.max_length);
                add_stat(&mut field_stats, "sum_length", sr.sum_length);
                add_stat(&mut field_stats, "avg_length", sr.avg_length);
                add_stat(&mut field_stats, "stddev_length", sr.stddev_length);
                add_stat(&mut field_stats, "variance_length", sr.variance_length);
                add_stat(&mut field_stats, "cv_length", sr.cv_length);

                // Numeric-specific stats
                add_stat(&mut field_stats, "mean", sr.mean);
                add_stat(&mut field_stats, "sem", sr.sem);
                add_stat(&mut field_stats, "stddev", sr.stddev);
                add_stat(&mut field_stats, "variance", sr.variance);
                add_stat(&mut field_stats, "cv", sr.cv);
            }

            fields.push(FrequencyField {
                field: field_name,
                r#type: dtype,
                cardinality,
                nullcount,
                sparsity,
                uniqueness_ratio,
                stats: field_stats.clone(),
                frequencies: processed_frequencies
                    .iter()
                    .map(|pf| FrequencyEntry {
                        value:      if self.flag_vis_whitespace {
                            util::visualize_whitespace(&String::from_utf8_lossy(&pf.value))
                        } else {
                            String::from_utf8_lossy(&pf.value).to_string()
                        },
                        count:      pf.count,
                        percentage: pf
                            .formatted_percentage
                            .parse::<f64>()
                            .unwrap_or(pf.percentage),
                        rank:       pf.rank,
                    })
                    .collect(),
            });

            // Clear the vectors for the next iteration
            field_stats.clear();
            processed_frequencies.clear();
        } // end for loop

        let output = FrequencyOutput {
            input: if is_stdin {
                "stdin".to_string()
            } else {
                // safety: we know arg_input is not None
                self.arg_input.clone().unwrap()
            },
            description: format!("Generated with `qsv {}`", argv[1..].join(" ")),
            rowcount: if rowcount == 0 {
                // if rowcount == 0 (most probably, coz the input is STDIN),
                // derive the rowcount from first json_fields vec
                // by summing the counts for the first field
                fields
                    .first()
                    .map_or(0, |field| field.frequencies.iter().map(|f| f.count).sum())
            } else {
                rowcount
            },
            fieldcount,
            fields,
        };
        let mut json_output = serde_json::to_string_pretty(&output)?;

        // remove all empty stats properties from the JSON output using regex
        let re = regex::Regex::new(r#""stats": \[\],\n\s*"#).unwrap();
        json_output = re.replace_all(&json_output, "").to_string();

        if let Some(output_path) = &self.flag_output {
            std::fs::write(output_path, json_output)?;
        } else {
            println!("{json_output}");
        }

        Ok(())
    }

    fn sel_headers<R: io::Read>(
        &self,
        rdr: &mut csv::Reader<R>,
    ) -> CliResult<(csv::ByteRecord, Selection)> {
        let headers = rdr.byte_headers()?;
        let all_unique_headers_vec = self.get_unique_headers(headers)?;

        UNIQUE_COLUMNS_VEC
            .set(all_unique_headers_vec)
            .map_err(|_| "Cannot set UNIQUE_COLUMNS")?;

        let sel = self.rconfig().selection(headers)?;
        Ok((sel.select(headers).map(<[u8]>::to_vec).collect(), sel))
    }
}

/// Helper function to add a field to field_stats if it exists
/// Automatically converts any type to appropriate JSON value
fn add_stat<T: ToString>(field_stats: &mut Vec<FieldStats>, name: &str, value: Option<T>) {
    if let Some(val) = value {
        let value_str = val.to_string();

        // Try to parse as integer first
        let json_value = if let Ok(int_val) = value_str.parse::<i64>() {
            JsonValue::Number(int_val.into())
        } else if let Ok(float_val) = value_str.parse::<f64>() {
            JsonValue::Number(
                serde_json::Number::from_f64(float_val)
                    .unwrap_or_else(|| serde_json::Number::from(0)),
            )
        } else {
            // Fall back to string
            JsonValue::String(value_str)
        };

        field_stats.push(FieldStats {
            name:  name.to_string(),
            value: json_value,
        });
    }
}

/// trim leading and trailing whitespace from a byte slice
#[allow(clippy::inline_always)]
#[inline(always)]
fn trim_bs_whitespace(bytes: &[u8]) -> &[u8] {
    let mut start = 0;
    let mut end = bytes.len();

    // safety: use unchecked indexing since we're bounds checking with the while condition
    // Find start by scanning forward
    while start < end {
        let b = unsafe { *bytes.get_unchecked(start) };
        if !b.is_ascii_whitespace() {
            break;
        }
        start += 1;
    }

    // Find end by scanning backward
    while end > start {
        let b = unsafe { *bytes.get_unchecked(end - 1) };
        if !b.is_ascii_whitespace() {
            break;
        }
        end -= 1;
    }

    // safety: This slice is guaranteed to be in bounds due to our index calculations
    unsafe { bytes.get_unchecked(start..end) }
}
