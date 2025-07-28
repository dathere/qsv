static USAGE: &str = r#"
Compute a frequency table on CSV data.

The frequency table is formatted as CSV data with the following columns:

    field,value,count,percentage

With a row for the N most frequent values (default:10) for each column in the CSV.

Since this command computes an exact frequency table, memory proportional to the
cardinality of each column would be normally required.

However, this is problematic for columns with ALL unique values (e.g. an ID column),
as the command will need to allocate memory proportional to the column's cardinality,
potentially causing Out-of-Memory (OOM) errors for larger-than-memory datasets.

To overcome this, the frequency command can use the stats cache if it exists to get
column cardinalities. This short-circuits frequency compilation for columns with
all unique values (i.e. where rowcount == cardinality), eliminating the need to
maintain an in-memory hashmap for ID columns. This allows `frequency` to handle
larger-than-memory datasets with the added benefit of also making it faster when
working with datasets with ID columns.

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
    --vis-whitespace        Visualize whitespace characters in the output.
                            See https://github.com/dathere/qsv/wiki/Supplemental#whitespace-markers
                            for the list of whitespace markers.
    -j, --jobs <arg>        The number of jobs to run in parallel.
                            This works much faster when the given CSV data has
                            an index already created. Note that a file handle
                            is opened for each job.
                            When not set, the number of jobs is set to the
                            number of CPUs detected.

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
use indicatif::HumanCount;
use rust_decimal::prelude::*;
use serde::Deserialize;
use stats::{Frequencies, merge_all};
use threadpool::ThreadPool;

use crate::{
    CliResult,
    config::{Config, Delimiter},
    index::Indexed,
    select::{SelectColumns, Selection},
    util,
    util::{ByteString, StatsMode, get_stats_records},
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
}

const NULL_VAL: &[u8] = b"(NULL)";
const NON_UTF8_ERR: &str = "<Non-UTF8 ERROR>";
const EMPTY_BYTE_VEC: Vec<u8> = Vec::new();

static UNIQUE_COLUMNS_VEC: OnceLock<Vec<usize>> = OnceLock::new();
static COL_CARDINALITY_VEC: OnceLock<Vec<(String, u64)>> = OnceLock::new();
static FREQ_ROW_COUNT: OnceLock<u64> = OnceLock::new();

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let rconfig = args.rconfig();

    // we're loading the entire file into memory, we need to check avail mem
    if let Some(path) = rconfig.path.clone() {
        util::mem_file_check(&path, false, args.flag_memcheck)?;
    }

    let mut wtr = Config::new(args.flag_output.as_ref()).writer()?;
    let (headers, tables) = match args.rconfig().indexed()? {
        Some(ref mut idx) if util::njobs(args.flag_jobs) > 1 => args.parallel_ftables(idx),
        _ => args.sequential_ftables(),
    }?;

    #[allow(unused_assignments)]
    let mut header_vec: Vec<u8> = Vec::with_capacity(tables.len());
    let mut itoa_buffer = itoa::Buffer::new();
    let mut pct_decimal: Decimal;
    let mut final_pct_decimal: Decimal;
    // most percentages are less than 10 characters, so pre-allocate 10 characters
    #[allow(unused_assignments)]
    let mut pct_string = String::with_capacity(10);
    let mut pct_scale: u32;
    let mut current_scale: u32;
    let abs_dec_places = args.flag_pct_dec_places.unsigned_abs() as u32;
    let mut row: Vec<&[u8]>;
    let mut all_unique_header: bool;

    // safety: we know that UNIQUE_COLUMNS has been previously set when compiling frequencies
    // by sel_headers fn
    let unique_headers_vec = UNIQUE_COLUMNS_VEC.get().unwrap();

    wtr.write_record(vec!["field", "value", "count", "percentage"])?;
    let head_ftables = headers.iter().zip(tables);
    let row_count = *FREQ_ROW_COUNT.get().unwrap_or(&0);

    let all_unique_text = args.flag_all_unique_text.as_bytes();

    for (i, (header, ftab)) in head_ftables.enumerate() {
        header_vec = if rconfig.no_headers {
            (i + 1).to_string().into_bytes()
        } else {
            header.to_vec()
        };

        let mut sorted_counts: Vec<(Vec<u8>, u64, f64)>;
        all_unique_header = unique_headers_vec.contains(&i);

        if all_unique_header {
            // if the column has all unique values, we don't need to sort the counts
            sorted_counts = vec![(all_unique_text.to_vec(), row_count, 100.0_f64)];
        } else {
            sorted_counts = args.counts(&ftab);

            // if not --other_sorted and the first value is "Other (", rotate it to the end
            if !args.flag_other_sorted
                && sorted_counts.first().is_some_and(|(value, _, _)| {
                    value.starts_with(format!("{} (", args.flag_other_text).as_bytes())
                })
            {
                sorted_counts.rotate_left(1);
            }
        }

        #[allow(unused_assignments)]
        let mut value_str = String::with_capacity(100);
        for (value, count, percentage) in sorted_counts {
            pct_decimal = Decimal::from_f64(percentage).unwrap_or_default();
            pct_scale = if args.flag_pct_dec_places < 0 {
                current_scale = pct_decimal.scale();
                if current_scale > abs_dec_places {
                    current_scale
                } else {
                    abs_dec_places
                }
            } else {
                abs_dec_places
            };
            final_pct_decimal = pct_decimal
                .round_dp_with_strategy(
                    pct_scale,
                    rust_decimal::RoundingStrategy::MidpointAwayFromZero,
                )
                .normalize();
            pct_string = if final_pct_decimal.fract().to_string().len() > abs_dec_places as usize {
                final_pct_decimal
                    .round_dp_with_strategy(abs_dec_places, RoundingStrategy::MidpointAwayFromZero)
                    .normalize()
                    .to_string()
            } else {
                final_pct_decimal.to_string()
            };
            row = vec![
                &*header_vec,
                if args.flag_vis_whitespace {
                    value_str = util::visualize_whitespace(&String::from_utf8_lossy(&value));
                    value_str.as_bytes()
                } else {
                    &value
                },
                itoa_buffer.format(count).as_bytes(),
                pct_string.as_bytes(),
            ];
            wtr.write_record(row)?;
        }
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

    #[inline]
    fn counts(&self, ftab: &FTable) -> Vec<(ByteString, u64, f64)> {
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
        let mut counts_final: Vec<(Vec<u8>, u64, f64)> = Vec::with_capacity(counts.len() + 1);

        // Create NULL value once to avoid repeated to_vec allocations
        let null_val = NULL_VAL.to_vec();

        #[allow(clippy::cast_precision_loss)]
        for (byte_string, count) in counts {
            count_sum += count;
            pct = count as f64 * pct_factor;
            pct_sum += pct;
            if *b"" == **byte_string {
                counts_final.push((null_val.clone(), count, pct));
            } else {
                counts_final.push((byte_string.to_owned(), count, pct));
            }
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
        // amortize allocations
        let mut field_buffer: Vec<u8> = Vec::with_capacity(1024);
        let mut row_buffer: csv::ByteRecord = csv::ByteRecord::with_capacity(200, nsel_len);

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
        let newvec = Vec::new();
        let col_cardinality_vec = COL_CARDINALITY_VEC.get().unwrap_or(&newvec);
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

        let mut string_buf = String::with_capacity(100);
        for row in it {
            // safety: we know the row is valid
            row_buffer.clone_from(&unsafe { row.unwrap_unchecked() });
            for (i, field) in nsel.select(row_buffer.into_iter()).enumerate() {
                // safety: all_unique_flag_vec is pre-computed to have exactly nsel_len elements,
                // which matches the number of selected columns that we iterate over.
                // i will always be < nsel_len since it comes from enumerate() over the selected
                // columns
                if unsafe { *all_unique_flag_vec.get_unchecked(i) } {
                    continue;
                }

                // safety: freq_tables is pre-allocated with nsel_len elements.
                // i will always be < nsel_len since it comes from enumerate() over the selected
                // columns
                if !field.is_empty() {
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
                (
                    simdutf8::basic::from_utf8(col_name)
                        .unwrap_or(NON_UTF8_ERR)
                        .to_string(),
                    stats_record.cardinality,
                )
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

        Ok(all_unique_headers_vec)
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
