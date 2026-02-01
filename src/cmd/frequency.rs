static USAGE: &str = r#"
Compute a frequency distribution table on input data. It has CSV and JSON output modes.
https://en.wikipedia.org/wiki/Frequency_(statistics)#Frequency_distribution_table

In CSV output mode (default), the table is formatted as CSV data with the following
columns - field,value,count,percentage,rank.

The rank column is 1-based and is calculated based on the count of the values,
with the most frequent having a rank of 1. In case of ties, the rank is calculated
based on the rank-strategy option - "min" (default), "max", "dense", "ordinal", or "average".

Only the top N values (set by the --limit option) are computed, with the rest of the values
grouped into an "Other" category with a special rank of 0. The "Other" category includes
the count of remaining unique values that are not in the top N values.

In JSON output mode, the table is formatted as nested JSON data. In addition to
the columns above, the JSON output also includes the row count, field count, rank-strategy,
each field's data type, cardinality, nullcount, sparsity, uniqueness_ratio and its stats.

Since this command computes an exact frequency distribution table, memory proportional
to the cardinality of each column would be normally required.

However, this is problematic for columns with ALL unique values (e.g. an ID column),
as the command will need to allocate memory proportional to the column's cardinality.

To overcome this, the frequency command uses several mechanisms:

STATS CACHE:
If the stats cache exists for the input file, it is used to get column cardinality information.
This short-circuits frequency compilation for columns with all unique values (i.e. where
rowcount == cardinality), eliminating the need to maintain an in-memory hashmap for ID columns.
This allows `frequency` to handle larger-than-memory datasets with the added benefit of also
making it faster when working with datasets with ID columns.

That's why for MAXIMUM PERFORMANCE, it's HIGHLY RECOMMENDED to create an index (`qsv index data.csv`)
and pre-populate the stats cache (`qsv stats data.csv --cardinality --stats-jsonl`)
BEFORE running `frequency`.

MEMORY-AWARE CHUNKING:
When working with large datasets, memory-aware chunking is automatically enabled. Chunk size
is dynamically calculated based on available memory and record sampling.

You can override this behavior by setting the QSV_FREQ_CHUNK_MEMORY_MB environment variable.
(set to 0 for dynamic sizing, or a positive number for a fixed memory limit per chunk,
or -1 for CPU-based chunking (1 chunk = num records/number of CPUs)), or by setting the --jobs option.

NOTE: "Complete" Frequency Tables:

    By default, ID columns will have an "<ALL UNIQUE>" value with count equal to
    rowcount and percentage set to 100 with a rank of 0. This is done by using the
    stats cache to fetch each column's cardinality - allowing qsv to short-circuit
    frequency compilation and eliminate the need to maintain a hashmap for ID columns.

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
-r, --rank-strategy <arg>   The strategy to use when there are count-tied values in the frequency table.
                            See https://en.wikipedia.org/wiki/Ranking for more info.
                            Valid values are:
                              - dense: Assigns consecutive integers regardless of ties,
                                incrementing by 1 for each new count value (AKA "1223" ranking).
                              - min: Tied items receive the minimum rank position (AKA "1224" ranking).
                              - max: Tied items receive the maximum rank position (AKA "1334" ranking).
                              - ordinal: The next rank is the current rank plus 1 (AKA "1234" ranking).
                              - average: Tied items receive the average of their ordinal positions
                                (AKA "1 2.5 2.5 4" ranking).
                            Note that tied values with the same rank are sorted alphabetically.
                            [default: dense]
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
    --no-other              Don't include the "Other" category in the frequency table.
                            This is equivalent to --other-text "<NONE>".
    --null-sorted           By default, the NULL category (controlled by --null-text)
                            is placed at the end of the frequency table for a field,
                            after "Other" if present. If this is enabled, the NULL
                            category will be sorted with the rest of the values by count.
    -a, --asc               Sort the frequency tables in ascending order by count.
                            The default is descending order. Note that this option will
                            also reverse ranking - i.e. the LEAST frequent values will
                            have a rank of 1.
    --no-trim               Don't trim whitespace from values when computing frequencies.
                            The default is to trim leading and trailing whitespaces.
    --null-text <arg>       The text to use for NULL values. If set to "<NONE>",
                            NULLs will not be included in the frequency table
                            (equivalent to --no-nulls).
                            [default: (NULL)]
    --no-nulls              Don't include NULLs in the frequency table.
                            This is equivalent to --null-text "<NONE>".
    --pct-nulls             Include NULL values in percentage and rank calculations.
                            When disabled (default), percentages are "valid percentages"
                            calculated with NULLs excluded from the denominator, and
                            NULL entries display empty percentage and rank values.
                            When enabled, NULLs are included in the denominator
                            (original behavior).
                            Has no effect when --no-nulls is set.
    -i, --ignore-case       Ignore case when computing frequencies.
    --no-float <cols>       Exclude Float columns from frequency analysis.
                            Floats typically contain continuous values where
                            frequency tables are not meaningful.
                            To exclude ALL Float columns, use --no-float "*"
                            To exclude Floats except specific columns, specify
                            a comma-separated list of Float columns to INCLUDE.
                            e.g. "--no-float *" excludes all Floats
                                 "--no-float price,rate" excludes Floats
                                    except 'price' and 'rate'
                            Requires stats cache for type detection.
    --stats-filter <expr>   Filter columns based on their statistics using a Luau expression.
                            Columns where the expression evaluates to `true` are EXCLUDED.
                            Available fields: field, type, is_ascii, cardinality, nullcount,
                            sum, min, max, range, sort_order, min_length, max_length, mean,
                            stddev, variance, cv, sparsity, q1, q2_median, q3, iqr, mad,
                            skewness, mode, antimode, n_negative, n_zero, n_positive, etc.
                            e.g. "nullcount > 1000" - exclude columns with many nulls
                                 "type == 'Float'" - exclude Float columns
                                 "cardinality > 500 and nullcount > 0" - compound expression
                            Requires stats cache and the "luau" feature.
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
                            The JSON output includes additional metadata: row count, field count,
                            data type, cardinality, null count, sparsity, uniqueness_ratio and
                            17 additional stats (e.g. sum, min, max, range, sort_order, mean, sem, etc.).
    --pretty-json           Same as --json but pretty prints the JSON output.
    --toon                  Output frequency table and select stats in TOON format instead of CSV.
                            TOON is a compact, human-readable encoding of the JSON data model for LLM prompts.
                            See https://toonformat.dev/ for more info.
    --no-stats              When using the JSON or TOON output mode, do not include the additional stats.
    --weight <column>       Compute weighted frequencies using the specified column as weights.
                            The weight column must be numeric. When specified, frequency counts
                            are multiplied by the weight value for each row. The weight column is
                            automatically excluded from frequency computation. Missing or
                            unparsable weights default to 1.0. Zero, negative, NaN and infinite
                            weights are ignored and do not contribute to frequencies.

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

use std::{fs, io, str::FromStr, sync::OnceLock};

use crossbeam_channel;
use foldhash::{HashMap, HashMapExt, HashSet, HashSetExt};
use indicatif::HumanCount;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{self, Value as JsonValue};
use stats::{Frequencies, merge_all};
use threadpool::ThreadPool;
use toon_format::{EncodeOptions, encode};

use crate::{
    CliResult,
    cmd::stats::StatsData,
    config::{Config, Delimiter},
    index::Indexed,
    select::{SelectColumns, Selection},
    util::{self, ByteString, StatsMode, get_stats_records},
};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RankStrategy {
    Min,
    Max,
    Dense,
    Ordinal,
    Average,
}

impl FromStr for RankStrategy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "min" => Ok(RankStrategy::Min),
            "max" => Ok(RankStrategy::Max),
            "dense" => Ok(RankStrategy::Dense),
            "ordinal" => Ok(RankStrategy::Ordinal),
            "average" => Ok(RankStrategy::Average),
            _ => Err(format!(
                "Invalid rank-strategy: '{s}'. Valid values are: dense, min, max, ordinal, average"
            )),
        }
    }
}

#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Clone, Deserialize)]
pub struct Args {
    pub arg_input:            Option<String>,
    pub flag_select:          SelectColumns,
    pub flag_limit:           isize,
    pub flag_unq_limit:       usize,
    pub flag_lmt_threshold:   usize,
    pub flag_rank_strategy:   RankStrategy,
    pub flag_pct_dec_places:  isize,
    pub flag_other_sorted:    bool,
    pub flag_other_text:      String,
    pub flag_no_other:        bool,
    pub flag_null_sorted:     bool,
    pub flag_asc:             bool,
    pub flag_no_trim:         bool,
    pub flag_null_text:       String,
    pub flag_no_nulls:        bool,
    pub flag_pct_nulls:       bool,
    pub flag_ignore_case:     bool,
    pub flag_no_float:        Option<String>,
    #[cfg(feature = "luau")]
    pub flag_stats_filter:    Option<String>,
    pub flag_all_unique_text: String,
    pub flag_jobs:            Option<usize>,
    pub flag_output:          Option<String>,
    pub flag_no_headers:      bool,
    pub flag_delimiter:       Option<Delimiter>,
    pub flag_memcheck:        bool,
    pub flag_vis_whitespace:  bool,
    pub flag_json:            bool,
    pub flag_pretty_json:     bool,
    pub flag_toon:            bool,
    pub flag_no_stats:        bool,
    pub flag_weight:          Option<String>,
}

const NON_UTF8_ERR: &str = "<Non-UTF8 ERROR>";
const EMPTY_BYTE_VEC: Vec<u8> = Vec::new();

static STATS_RECORDS: OnceLock<HashMap<String, StatsData>> = OnceLock::new();
static NULL_VAL: OnceLock<Vec<u8>> = OnceLock::new();
static UNIQUE_COLUMNS_VEC: OnceLock<Vec<usize>> = OnceLock::new();
static COL_CARDINALITY_VEC: OnceLock<Vec<(String, u64)>> = OnceLock::new();
static FREQ_ROW_COUNT: OnceLock<u64> = OnceLock::new();
static FLOAT_COLUMNS_TO_SKIP: OnceLock<Vec<usize>> = OnceLock::new();
#[cfg(feature = "luau")]
static STATS_FILTER_COLUMNS_TO_SKIP: OnceLock<Vec<usize>> = OnceLock::new();
static EMPTY_VEC: Vec<(String, u64)> = Vec::new();
static ALL_UNIQUE_TEXT: OnceLock<Vec<u8>> = OnceLock::new();
// FrequencyEntry, FrequencyField and FrequencyOutput are
// structs for JSON output
#[derive(Serialize)]
struct FrequencyEntry {
    value:      String,
    count:      u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    percentage: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rank:       Option<f64>,
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
    input:         String,
    description:   String,
    rowcount:      u64,
    fieldcount:    usize,
    fields:        Vec<FrequencyField>,
    rank_strategy: RankStrategy,
}

// Shared frequency processing result
// used by both CSV and JSON output
#[derive(Clone)]
struct ProcessedFrequency {
    count:                u64,
    percentage:           f64,
    formatted_percentage: String,
    value:                Vec<u8>,
    rank:                 f64,
}

/// Estimates memory usage per record for frequency table computation.
///
/// This function calculates the approximate memory footprint of a single CSV record
/// when computing frequency tables. The estimate includes:
/// - Base record size (sum of field lengths)
/// - Hashmap overhead for storing unique values in Frequencies<Vec<u8>>
///
/// # Arguments
///
/// * `record` - The CSV record to estimate memory for
///
/// # Returns
///
/// Estimated memory in bytes per record
fn estimate_record_memory_for_frequency(record: &csv::ByteRecord) -> usize {
    // Base memory: sum of all field lengths
    let base_size: usize = record.iter().map(<[u8]>::len).sum();

    // Hashmap overhead: Frequencies<Vec<u8>> stores unique values
    // Each hashmap entry has overhead (~24 bytes) plus the value size
    // We estimate based on average field size
    let avg_field_size = if record.is_empty() {
        0
    } else {
        base_size / record.len()
    };

    // Estimate hashmap overhead: ~24 bytes per entry + value size
    // For frequency tables, we store each unique value once
    // Conservative estimate: assume we'll store all field values
    let hashmap_overhead = record.len() * (24 + avg_field_size);

    // Add overhead for Vec capacity
    let overhead = base_size / 4;

    base_size + hashmap_overhead + overhead
}

/// Helper function to calculate average record size from samples
fn calculate_avg_record_size_for_frequency(samples: &[csv::ByteRecord]) -> usize {
    if samples.is_empty() {
        1024 // Default: 1KB per record
    } else {
        let total_size: usize = samples
            .iter()
            .map(estimate_record_memory_for_frequency)
            .sum();
        (total_size / samples.len()).max(1024)
    }
}

/// Estimates total memory required for processing a chunk of records for frequency tables.
///
/// # Arguments
///
/// * `record_count` - Number of records in the chunk
/// * `avg_record_size` - Average size of a record in bytes
/// * `field_count` - Number of fields in the record
///
/// # Returns
///
/// Estimated total memory in bytes for the chunk
const fn estimate_chunk_memory_for_frequency(
    record_count: usize,
    avg_record_size: usize,
    field_count: usize,
) -> usize {
    // Base memory for records
    let base_memory = record_count.saturating_mul(avg_record_size);

    // Hashmap overhead: frequency tables store unique values
    // Estimate based on cardinality (assume 10% of records are unique per field)
    // Each hashmap entry: ~24 bytes overhead + value size
    let estimated_unique_per_field = if record_count / 10 > 0 {
        record_count / 10
    } else {
        1
    };
    let field_count_divisor = if field_count > 0 { field_count } else { 1 };
    let hashmap_overhead = estimated_unique_per_field
        .saturating_mul(field_count)
        .saturating_mul(avg_record_size / field_count_divisor + 24);

    // Add overhead for data structures (Frequencies objects, Vec capacity, etc.)
    // Estimate 20% overhead
    let overhead = (base_memory + hashmap_overhead) / 5;

    base_memory
        .saturating_add(hashmap_overhead)
        .saturating_add(overhead)
}

/// Calculates memory-aware chunk size for parallel frequency table processing.
///
/// This function determines an appropriate chunk size based on:
/// - Available memory per chunk (if configured)
/// - Dynamic estimation via sampling (if max_chunk_memory_mb is Some(0))
/// - CPU-based chunking (fallback)
///
/// # Arguments
///
/// * `idx_count` - Total number of records in the file
/// * `njobs` - Number of parallel jobs
/// * `max_chunk_memory_mb` - Maximum memory per chunk in MB ( None = dynamic sizing based on
///   available memory, Some(0) = dynamic sizing based on sampling, Some(n) = fixed limit)
/// * `field_count` - Number of fields in the record
/// * `sample_records` - Optional slice of sample records for dynamic sizing
///
/// # Returns
///
/// Calculated chunk size (number of records per chunk)
fn calculate_memory_aware_chunk_size_for_frequency(
    idx_count: u64,
    njobs: usize,
    max_chunk_memory_mb: Option<u64>,
    sample_records: Option<&[csv::ByteRecord]>,
) -> usize {
    // Frequency always uses memory-aware chunking since it builds hash tables.
    // This function has three configuration paths:
    //   - None: dynamic sizing based on sample records and estimated memory usage.
    //   - Some(0): dynamic sizing, also based on sample records and estimated memory usage.
    //   - Some(n): fixed memory limit per chunk.
    // In all cases, chunk size is calculated using memory-based estimates, not CPU-based chunking.
    match max_chunk_memory_mb {
        None => {
            // No memory limit configured, use dynamic sizing
            util::calculate_dynamic_chunk_size(
                idx_count,
                njobs,
                sample_records,
                estimate_record_memory_for_frequency,
            )
        },
        Some(0) => {
            // Dynamic sizing: sample records to estimate average size
            util::calculate_dynamic_chunk_size(
                idx_count,
                njobs,
                sample_records,
                estimate_record_memory_for_frequency,
            )
        },
        Some(limit_mb) => {
            // Fixed memory limit per chunk
            #[allow(clippy::cast_precision_loss)]
            let max_memory_bytes = (limit_mb as usize * 1024 * 1024) as f64 * util::SAFETY_MARGIN;

            // Estimate average record size
            // If we can't estimate, use a conservative default (1KB per record)
            let avg_record_size = if let Some(samples) = sample_records {
                if samples.is_empty() {
                    1024 // Default: 1KB per record
                } else {
                    let total_size: usize = samples
                        .iter()
                        .map(estimate_record_memory_for_frequency)
                        .sum();
                    debug_assert!(total_size > 0, "total_size should be positive here");
                    (total_size / samples.len()).max(1024) // ensure minimum 1KB estimate, samples.len() is guaranteed to be positive here
                }
            } else {
                1024 // Default: 1KB per record
            };

            // Calculate chunk size based on memory limit
            #[allow(clippy::cast_precision_loss)]
            let chunk_size = (max_memory_bytes / (avg_record_size as f64).max(1.0)) as usize;

            // Ensure chunk size is reasonable
            chunk_size.max(1).min(idx_count as usize)
        },
    }
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let mut args: Args = util::get_args(USAGE, argv)?;

    // Handle --no-other flag (alias for --other-text "<NONE>")
    if args.flag_no_other {
        args.flag_other_text = "<NONE>".to_string();
    }

    // Handle --null-text "<NONE>" (alias for --no-nulls)
    if args.flag_null_text == "<NONE>" {
        args.flag_no_nulls = true;
    }

    let mut rconfig = args.rconfig();

    let is_stdin = rconfig.is_stdin();

    // if stdin and args.flag_json is true, save stdin to tempfile
    // so we can derive stats
    let mut stdin_temp_file;
    let is_json = args.flag_json || args.flag_pretty_json || args.flag_toon;
    if is_stdin && is_json {
        let temp_dir = std::env::temp_dir();
        stdin_temp_file = tempfile::Builder::new()
            .suffix(".csv")
            .tempfile_in(&temp_dir)?;
        io::copy(&mut io::stdin(), &mut stdin_temp_file)?;
        args.arg_input = Some(stdin_temp_file.path().to_string_lossy().to_string());
        rconfig = args.rconfig();
    }

    // Check if we have an index and will use parallel processing
    // If so, skip mem_file_check since memory-aware chunking will handle it
    let mut indexed_result = args.rconfig().indexed()?;
    let will_use_parallel = match &indexed_result {
        Some(_) => {
            // We have an index, check if we'll use parallel processing
            match args.flag_jobs {
                Some(num_jobs) => num_jobs != 1,
                _ => true, // Default to parallel when index exists
            }
        },
        None => false, // No index, will use sequential
    };

    // we're loading the entire file into memory, we need to check avail mem
    // Skip this check for parallel processing since memory-aware chunking handles it
    if !will_use_parallel && let Some(path) = rconfig.path.clone() {
        // Try mem_file_check, and if it fails for an unindexed file, auto-create index
        match util::mem_file_check(&path, false, args.flag_memcheck) {
            Ok(_) => {
                // Memory check passed, proceed with sequential processing
            },
            Err(e) => {
                // Memory check failed - if we don't have an index, try creating one
                if indexed_result.is_none() && !rconfig.is_stdin() {
                    log::info!(
                        "File too large for sequential processing. Auto-creating index to enable \
                         parallel processing..."
                    );

                    // Create index and retry
                    match util::create_index_for_file(&path, &rconfig) {
                        Ok(()) => {
                            // Re-check for index after creation
                            indexed_result = args.rconfig().indexed()?;
                            if indexed_result.is_some() {
                                log::info!(
                                    "Index created successfully. Switching to parallel processing."
                                );
                                // Continue - the match statement below will use
                                // indexed_result to determine parallel/sequential
                            } else {
                                // Index creation succeeded but we still can't get it
                                // Return the original memory error
                                return Err(e);
                            }
                        },
                        Err(index_err) => {
                            // Index creation failed, return the original memory error
                            log::warn!("Failed to auto-create index: {index_err}");
                            return Err(e);
                        },
                    }
                } else {
                    // Either we already have an index or it's stdin - return the error
                    return Err(e);
                }
            },
        }
    }

    // Create NULL_VAL & ALL_UNIQUE_TEXT once at the start to avoid
    // repeated string & vec allocations in hot loops.
    // safety: we're initializing the OnceLocks at the start of the program
    NULL_VAL
        .set(args.flag_null_text.as_bytes().to_vec())
        .unwrap();

    ALL_UNIQUE_TEXT
        .set(args.flag_all_unique_text.as_bytes().to_vec())
        .unwrap();

    let (headers, tables, weighted_tables) = if let Some(idx) = indexed_result
        && util::njobs(args.flag_jobs) > 1
    {
        args.parallel_ftables(&idx)
    } else {
        args.sequential_ftables()
    }?;

    if is_json {
        return args.output_json(
            &headers,
            tables,
            weighted_tables.as_ref(),
            &rconfig,
            argv,
            is_stdin,
        );
    }

    // amortize allocations
    #[allow(unused_assignments)]
    let mut header_vec: Vec<u8> = Vec::with_capacity(tables.len());
    let mut itoa_buffer = itoa::Buffer::new();
    let mut zmij_buffer = zmij::Buffer::new();
    let mut rank_buffer = String::with_capacity(20);
    let mut row: Vec<&[u8]>;

    let head_ftables = headers.iter().zip(tables);
    let row_count = *FREQ_ROW_COUNT.get().unwrap_or(&0);
    let abs_dec_places = args.flag_pct_dec_places.unsigned_abs() as u32;

    #[allow(unused_assignments)]
    let mut processed_frequencies: Vec<ProcessedFrequency> = Vec::with_capacity(head_ftables.len());
    #[allow(unused_assignments)]
    let mut value_str = String::with_capacity(100);
    let vis_whitespace = args.flag_vis_whitespace;

    // safety: we know that UNIQUE_COLUMNS has been previously set
    // when compiling frequencies by sel_headers fn in either sequential or parallel mode
    let unique_headers_vec = UNIQUE_COLUMNS_VEC.get().unwrap();

    let mut wtr = Config::new(args.flag_output.as_ref()).writer()?;
    // write headers
    wtr.write_record(vec!["field", "value", "count", "percentage", "rank"])?;

    // Handle weighted vs unweighted frequencies
    if let Some(ref weighted) = weighted_tables {
        // Process weighted frequencies
        for (i, header) in headers.iter().enumerate() {
            header_vec = if rconfig.no_headers {
                (i + 1).to_string().into_bytes()
            } else {
                header.to_vec()
            };

            if i < weighted.len() {
                args.process_frequencies_weighted(
                    unique_headers_vec.contains(&i),
                    abs_dec_places,
                    row_count,
                    &weighted[i],
                    &mut processed_frequencies,
                );
            }

            for processed_freq in &processed_frequencies {
                // Format rank: show as integer if whole number, otherwise with decimals
                // Sentinel value -1.0 indicates NULL entry with --pct-nulls=false (empty rank)
                rank_buffer.clear();
                if processed_freq.rank >= 0.0 {
                    if processed_freq.rank.fract() == 0.0 {
                        rank_buffer.push_str(itoa_buffer.format(processed_freq.rank as u64));
                    } else {
                        rank_buffer.push_str(zmij_buffer.format(processed_freq.rank));
                    }
                }

                row = vec![
                    &*header_vec,
                    if vis_whitespace {
                        value_str = util::visualize_whitespace(&String::from_utf8_lossy(
                            &processed_freq.value,
                        ));
                        value_str.as_bytes()
                    } else {
                        &processed_freq.value
                    },
                    itoa_buffer.format(processed_freq.count).as_bytes(),
                    processed_freq.formatted_percentage.as_bytes(),
                    rank_buffer.as_bytes(),
                ];
                wtr.write_record(row)?;
            }
            processed_frequencies.clear();
        }
    } else {
        // Process unweighted frequencies (original code)
        for (i, (header, ftab)) in head_ftables.enumerate() {
            header_vec = if rconfig.no_headers {
                (i + 1).to_string().into_bytes()
            } else {
                header.to_vec()
            };

            args.process_frequencies(
                unique_headers_vec.contains(&i),
                abs_dec_places,
                row_count,
                &ftab,
                &mut processed_frequencies,
            );

            for processed_freq in &processed_frequencies {
                // Format rank: show as integer if whole number, otherwise with decimals
                // Sentinel value -1.0 indicates NULL entry with --pct-nulls=false (empty rank)
                rank_buffer.clear();
                if processed_freq.rank >= 0.0 {
                    if processed_freq.rank.fract() == 0.0 {
                        rank_buffer.push_str(itoa_buffer.format(processed_freq.rank as u64));
                    } else {
                        rank_buffer.push_str(zmij_buffer.format(processed_freq.rank));
                    }
                }

                row = vec![
                    &*header_vec,
                    if vis_whitespace {
                        value_str = util::visualize_whitespace(&String::from_utf8_lossy(
                            &processed_freq.value,
                        ));
                        value_str.as_bytes()
                    } else {
                        &processed_freq.value
                    },
                    itoa_buffer.format(processed_freq.count).as_bytes(),
                    processed_freq.formatted_percentage.as_bytes(),
                    rank_buffer.as_bytes(),
                ];
                wtr.write_record(row)?;
            }
            processed_frequencies.clear();
        }
    }
    Ok(wtr.flush()?)
}

type Headers = csv::ByteRecord;
type FTable = Frequencies<Vec<u8>>;
type FTables = Vec<Frequencies<Vec<u8>>>;
// Weighted frequency tables: HashMap for each column storing value -> weighted count
type WeightedFTables = Vec<HashMap<Vec<u8>, f64>>;

/// Apply ranking strategy to grouped unweighted frequency values (u64 counts)
///
/// # Arguments
/// * `groups` - A list of `(count, values)` pairs, where each `values` vector contains all distinct
///   values that share the same unweighted count.
/// * `strategy` - The ranking strategy to apply when assigning ranks to counts (for example, min,
///   max, dense, ordinal, or average).
/// * `pct_factor` - Multiplier used to convert counts into percentage values (typically derived
///   from the total row count).
/// * `null_val` - Byte representation used to identify or label null or missing values.
///
/// # Returns
/// A tuple `(counts_final, count_sum, pct_sum)` where:
/// * `counts_final` - The flattened list of `(value, count, percentage, rank)` tuples for each
///   grouped value after ranking.
/// * `count_sum` - The sum of all counts in `counts_final`.
/// * `pct_sum` - The sum of all percentage values in `counts_final`.
#[allow(clippy::cast_precision_loss)]
fn apply_ranking_strategy_unweighted(
    groups: Vec<(u64, Vec<Vec<u8>>)>,
    strategy: RankStrategy,
    pct_factor: f64,
    null_val: &[u8],
    pct_nulls: bool,
) -> (Vec<(Vec<u8>, u64, f64, f64)>, u64, f64) {
    let mut counts_final: Vec<(Vec<u8>, u64, f64, f64)> =
        Vec::with_capacity(groups.iter().map(|(_, group)| group.len()).sum::<usize>() + 1);
    let mut current_rank = 1.0_f64;
    let mut count_sum = 0_u64;
    let mut pct_sum = 0.0_f64;

    match strategy {
        RankStrategy::Dense => {
            // Dense ranking (1223)
            for (count, mut group) in groups {
                group.sort_unstable();
                for byte_string in group {
                    count_sum += count;

                    if byte_string.is_empty() {
                        if pct_nulls {
                            // Original behavior: include NULL in percentage
                            let pct = count as f64 * pct_factor;
                            pct_sum += pct;
                            counts_final.push((null_val.to_vec(), count, pct, current_rank));
                        } else {
                            // New behavior: exclude NULL from percentage/rank
                            // Use -1.0 as sentinel values for empty percentage and rank
                            counts_final.push((null_val.to_vec(), count, -1.0, -1.0));
                        }
                    } else {
                        let pct = count as f64 * pct_factor;
                        pct_sum += pct;
                        counts_final.push((byte_string, count, pct, current_rank));
                    }
                }
                current_rank += 1.0;
            }
        },
        RankStrategy::Min => {
            // Standard competition ranking (1224)
            for (count, mut group) in groups {
                group.sort_unstable();
                let group_len = group.len();
                for byte_string in group {
                    count_sum += count;

                    if byte_string.is_empty() {
                        if pct_nulls {
                            let pct = count as f64 * pct_factor;
                            pct_sum += pct;
                            counts_final.push((null_val.to_vec(), count, pct, current_rank));
                        } else {
                            counts_final.push((null_val.to_vec(), count, -1.0, -1.0));
                        }
                    } else {
                        let pct = count as f64 * pct_factor;
                        pct_sum += pct;
                        counts_final.push((byte_string, count, pct, current_rank));
                    }
                }
                current_rank += group_len as f64;
            }
        },
        RankStrategy::Max => {
            // Modified competition ranking (1334)
            for (count, mut group) in groups {
                group.sort_unstable();
                let group_len = group.len();
                let max_rank = current_rank + group_len as f64 - 1.0;
                for byte_string in group {
                    count_sum += count;

                    if byte_string.is_empty() {
                        if pct_nulls {
                            let pct = count as f64 * pct_factor;
                            pct_sum += pct;
                            counts_final.push((null_val.to_vec(), count, pct, max_rank));
                        } else {
                            counts_final.push((null_val.to_vec(), count, -1.0, -1.0));
                        }
                    } else {
                        let pct = count as f64 * pct_factor;
                        pct_sum += pct;
                        counts_final.push((byte_string, count, pct, max_rank));
                    }
                }
                current_rank += group_len as f64;
            }
        },
        RankStrategy::Ordinal => {
            // Ordinal ranking (1234)
            for (count, mut group) in groups {
                group.sort_unstable();
                for byte_string in group {
                    count_sum += count;

                    if byte_string.is_empty() {
                        if pct_nulls {
                            let pct = count as f64 * pct_factor;
                            pct_sum += pct;
                            counts_final.push((null_val.to_vec(), count, pct, current_rank));
                        } else {
                            counts_final.push((null_val.to_vec(), count, -1.0, -1.0));
                        }
                    } else {
                        let pct = count as f64 * pct_factor;
                        pct_sum += pct;
                        counts_final.push((byte_string, count, pct, current_rank));
                    }
                    current_rank += 1.0;
                }
            }
        },
        RankStrategy::Average => {
            // Fractional ranking (1 2.5 2.5 4)
            for (count, mut group) in groups {
                group.sort_unstable();
                let group_len = group.len();
                let avg_rank = current_rank + (group_len as f64 - 1.0) / 2.0;
                for byte_string in group {
                    count_sum += count;

                    if byte_string.is_empty() {
                        if pct_nulls {
                            let pct = count as f64 * pct_factor;
                            pct_sum += pct;
                            counts_final.push((null_val.to_vec(), count, pct, avg_rank));
                        } else {
                            counts_final.push((null_val.to_vec(), count, -1.0, -1.0));
                        }
                    } else {
                        let pct = count as f64 * pct_factor;
                        pct_sum += pct;
                        counts_final.push((byte_string, count, pct, avg_rank));
                    }
                }
                current_rank += group_len as f64;
            }
        },
    }

    (counts_final, count_sum, pct_sum)
}

/// Apply a ranking strategy to grouped weighted frequency values.
///
/// This function takes pre-aggregated weighted frequency groups and flattens them into a list
/// of individual values with their associated weight, percentage, and rank. The rank assigned
/// to each value depends on the provided `strategy`, and percentages are computed using
/// `pct_factor`. The `null_val` is treated specially as the null/other bucket when present.
///
/// # Arguments
///
/// * `groups` - A vector of tuples where each tuple contains:
///   * the total weight for the group of values (`f64`), and
///   * a vector of the grouped values (`Vec<u8>` for each value) that share that weight. Typically,
///     these groups represent distinct values with their aggregated weights after applying any
///     limiting or bucketing logic.
/// * `strategy` - The ranking strategy (`RankStrategy`) used to assign ranks to values based on
///   their weights. This controls how ties are handled (e.g., minimum, maximum, dense, ordinal, or
///   average ranks).
/// * `pct_factor` - A scaling factor used to convert weights into percentage values. For example,
///   this is often the reciprocal of the total weight so that the resulting percentages sum to
///   approximately 100.
/// * `null_val` - The byte representation of the value that should be treated as the null/other
///   bucket. This is used to identify and correctly label null/other values in the output.
///
/// # Returns
///
/// A tuple `(counts_final, count_sum, pct_sum)` where:
///
/// * `counts_final` - The flattened list of `(value, weight, percentage, rank)` tuples for each
///   grouped value after ranking has been applied.
/// * `count_sum` - The sum of all weights in `counts_final`.
/// * `pct_sum` - The sum of all percentage values in `counts_final`.
#[allow(clippy::cast_precision_loss)]
fn apply_ranking_strategy_weighted(
    groups: Vec<(f64, Vec<Vec<u8>>)>,
    strategy: RankStrategy,
    pct_factor: f64,
    null_val: &[u8],
    pct_nulls: bool,
) -> (Vec<(Vec<u8>, f64, f64, f64)>, f64, f64) {
    let mut counts_final: Vec<(Vec<u8>, f64, f64, f64)> =
        Vec::with_capacity(groups.iter().map(|(_, group)| group.len()).sum::<usize>() + 1);
    let mut current_rank = 1.0_f64;
    let mut count_sum = 0.0_f64;
    let mut pct_sum = 0.0_f64;

    match strategy {
        RankStrategy::Dense => {
            // Dense ranking (1223)
            for (weight, mut group) in groups {
                group.sort_unstable();
                for byte_string in group {
                    count_sum += weight;

                    if byte_string.is_empty() {
                        if pct_nulls {
                            let pct = weight * pct_factor;
                            pct_sum += pct;
                            counts_final.push((null_val.to_vec(), weight, pct, current_rank));
                        } else {
                            counts_final.push((null_val.to_vec(), weight, -1.0, -1.0));
                        }
                    } else {
                        let pct = weight * pct_factor;
                        pct_sum += pct;
                        counts_final.push((byte_string, weight, pct, current_rank));
                    }
                }
                current_rank += 1.0;
            }
        },
        RankStrategy::Min => {
            // Standard competition ranking (1224)
            for (weight, mut group) in groups {
                group.sort_unstable();
                let group_len = group.len();
                for byte_string in group {
                    count_sum += weight;

                    if byte_string.is_empty() {
                        if pct_nulls {
                            let pct = weight * pct_factor;
                            pct_sum += pct;
                            counts_final.push((null_val.to_vec(), weight, pct, current_rank));
                        } else {
                            counts_final.push((null_val.to_vec(), weight, -1.0, -1.0));
                        }
                    } else {
                        let pct = weight * pct_factor;
                        pct_sum += pct;
                        counts_final.push((byte_string, weight, pct, current_rank));
                    }
                }
                current_rank += group_len as f64;
            }
        },
        RankStrategy::Max => {
            // Modified competition ranking (1334)
            for (weight, mut group) in groups {
                group.sort_unstable();
                let group_len = group.len();
                let max_rank = current_rank + group_len as f64 - 1.0;
                for byte_string in group {
                    count_sum += weight;

                    if byte_string.is_empty() {
                        if pct_nulls {
                            let pct = weight * pct_factor;
                            pct_sum += pct;
                            counts_final.push((null_val.to_vec(), weight, pct, max_rank));
                        } else {
                            counts_final.push((null_val.to_vec(), weight, -1.0, -1.0));
                        }
                    } else {
                        let pct = weight * pct_factor;
                        pct_sum += pct;
                        counts_final.push((byte_string, weight, pct, max_rank));
                    }
                }
                current_rank += group_len as f64;
            }
        },
        RankStrategy::Ordinal => {
            // Ordinal ranking (1234)
            for (weight, mut group) in groups {
                group.sort_unstable();
                for byte_string in group {
                    count_sum += weight;

                    if byte_string.is_empty() {
                        if pct_nulls {
                            let pct = weight * pct_factor;
                            pct_sum += pct;
                            counts_final.push((null_val.to_vec(), weight, pct, current_rank));
                        } else {
                            counts_final.push((null_val.to_vec(), weight, -1.0, -1.0));
                        }
                    } else {
                        let pct = weight * pct_factor;
                        pct_sum += pct;
                        counts_final.push((byte_string, weight, pct, current_rank));
                    }
                    current_rank += 1.0;
                }
            }
        },
        RankStrategy::Average => {
            // Fractional ranking (1 2.5 2.5 4)
            for (weight, mut group) in groups {
                group.sort_unstable();
                let group_len = group.len();
                let avg_rank = current_rank + (group_len as f64 - 1.0) / 2.0;
                for byte_string in group {
                    count_sum += weight;

                    if byte_string.is_empty() {
                        if pct_nulls {
                            let pct = weight * pct_factor;
                            pct_sum += pct;
                            counts_final.push((null_val.to_vec(), weight, pct, avg_rank));
                        } else {
                            counts_final.push((null_val.to_vec(), weight, -1.0, -1.0));
                        }
                    } else {
                        let pct = weight * pct_factor;
                        pct_sum += pct;
                        counts_final.push((byte_string, weight, pct, avg_rank));
                    }
                }
                current_rank += group_len as f64;
            }
        },
    }

    (counts_final, count_sum, pct_sum)
}

/// Apply limits to weighted frequency counts
///
/// # Arguments
/// * `counts` - Mutable reference to vector of `(value, weight)` pairs
/// * `limit` - Limit value; if positive, keep only the top N weighted values; if negative, keep
///   only values with weight greater than or equal to the absolute value of this limit; if zero, no
///   limits are applied
/// * `lmt_threshold` - Threshold controlling when limits are applied. Limits are applied when this
///   is 0 or when it is greater than or equal to the number of unique values; when this is a
///   positive number less than the unique count, no limits are applied.
fn apply_limits_weighted(counts: &mut Vec<(Vec<u8>, f64)>, limit: isize, lmt_threshold: usize) {
    let unique_counts_len = counts.len();
    if lmt_threshold == 0 || lmt_threshold >= unique_counts_len {
        let abs_limit = limit.unsigned_abs();

        #[allow(clippy::cast_precision_loss)]
        if limit > 0 {
            counts.truncate(abs_limit);
        } else if limit < 0 {
            let count_limit = abs_limit as f64;
            counts.retain(|(_, weight)| *weight >= count_limit);
        }
    }
}

/// Apply limits to unweighted frequency counts
///
/// # Arguments
/// * `counts` - Mutable reference to counts vector
/// * `limit` - Limit value (positive = top N, negative = threshold)
/// * `unq_limit` - Unique limit for all-unique columns
/// * `lmt_threshold` - Threshold controlling when limits are applied. Limits are applied when this
///   is 0 or when it is >= the number of unique values. When this is a positive number less than
///   the unique count, no limits are applied.
/// * `all_unique` - Whether the column has all unique values
fn apply_limits_unweighted(
    counts: &mut Vec<(Vec<u8>, u64)>,
    limit: isize,
    unq_limit: usize,
    lmt_threshold: usize,
    all_unique: bool,
) {
    let unique_counts_len = counts.len();
    if lmt_threshold == 0 || lmt_threshold >= unique_counts_len {
        let abs_limit = limit.unsigned_abs();
        let unique_limited = if all_unique && limit > 0 && unq_limit != abs_limit && unq_limit > 0 {
            counts.truncate(unq_limit);
            true
        } else {
            false
        };

        if limit > 0 {
            counts.truncate(abs_limit);
        } else if limit < 0 && !unique_limited {
            // if limit < 0, only return values with an occurrence count >= abs value of limit
            // Only do this if we haven't already unique limited the values
            let count_limit = abs_limit as u64;
            counts.retain(|(_, count)| *count >= count_limit);
        }
    }
}

/// Group unweighted frequency values by count.
///
/// # Arguments
/// * `counts` - A vector of `(value, count)` pairs, where `value` is the byte-string representation
///   of the category and `count` is its frequency.
///
/// # Returns
/// A vector of `(count, values)` pairs, where `values` is the list of
/// byte-strings that share the same `count`.
fn group_by_count(counts: Vec<(Vec<u8>, u64)>) -> Vec<(u64, Vec<Vec<u8>>)> {
    let mut count_groups: Vec<(u64, Vec<Vec<u8>>)> = Vec::new();
    let mut current_count = None;
    let mut current_group = Vec::new();

    for (byte_string, count) in counts {
        if let Some(prev_count) = current_count
            && count != prev_count
            && !current_group.is_empty()
        {
            count_groups.push((prev_count, std::mem::take(&mut current_group)));
        }

        current_count = Some(count);
        current_group.push(byte_string);
    }
    if !current_group.is_empty() {
        // safety: we know that current_count is Some
        count_groups.push((current_count.unwrap(), current_group));
    }

    count_groups
}

/// Group weighted frequency values by weight (with tolerance).
///
/// # Arguments
///
/// * `counts` - A list of `(value, weight)` pairs, where `value` is a byte string and `weight` is
///   the numeric weight used for grouping. The vector is expected to be ordered by `weight` so that
///   equal (or near-equal) weights are adjacent.
/// * `tolerance` - The maximum absolute difference between consecutive weights for them to be
///   treated as belonging to the same group.
fn group_by_weight(counts: Vec<(Vec<u8>, f64)>, tolerance: f64) -> Vec<(f64, Vec<Vec<u8>>)> {
    let mut weight_groups: Vec<(f64, Vec<Vec<u8>>)> = Vec::new();
    let mut current_weight: Option<f64> = None;
    let mut current_group: Vec<Vec<u8>> = Vec::new();

    for (byte_string, weight) in counts {
        if let Some(prev_weight) = current_weight
            && (prev_weight - weight).abs() > tolerance
            && !current_group.is_empty()
        {
            weight_groups.push((prev_weight, std::mem::take(&mut current_group)));
        }

        current_weight = Some(weight);
        current_group.push(byte_string);
    }
    if !current_group.is_empty() {
        // safety: we know that current_weight is Some
        weight_groups.push((current_weight.unwrap(), current_group));
    }

    weight_groups
}

/// Implementation of helper methods for frequency command arguments.
/// Provides configuration helpers and post-processing utilities for results.
impl Args {
    pub fn rconfig(&self) -> Config {
        Config::new(self.arg_input.as_ref())
            .delimiter(self.flag_delimiter)
            .no_headers(self.flag_no_headers)
            .select(self.flag_select.clone())
    }

    /// Helper to move "Other" category to end if not sorted
    fn move_other_to_end_if_needed<T>(&self, counts: &mut [(Vec<u8>, T, f64, f64)]) {
        let other_prefix = format!("{} (", self.flag_other_text);
        let other_prefix_bytes = other_prefix.as_bytes();
        if !self.flag_other_sorted
            && counts
                .first()
                .is_some_and(|(value, _, _, _)| value.starts_with(other_prefix_bytes))
        {
            counts.rotate_left(1);
        }
    }

    /// Helper to move NULL category to end if not sorted.
    /// Unlike `move_other_to_end_if_needed()` which only checks position 0 (since "Other" always
    /// has rank 0 and appears first in ascending sort), NULL can appear anywhere based on its
    /// count, so we need to search the entire list.
    /// This function handles multiple NULL entries (e.g., when literal "(NULL)" values exist
    /// in the data alongside empty strings that are converted to "(NULL)").
    fn move_null_to_end_if_needed<T: Copy>(&self, counts: &mut Vec<(Vec<u8>, T, f64, f64)>) {
        if self.flag_null_sorted {
            return;
        }
        // safety: NULL_VAL is set in run()
        let null_val = NULL_VAL.get().unwrap();
        // Collect all NULL entries
        let mut null_entries = Vec::new();
        let mut i = 0;
        while i < counts.len() {
            if counts[i].0 == *null_val {
                null_entries.push(counts.remove(i));
            } else {
                i += 1;
            }
        }
        // Append all NULL entries at the end
        counts.extend(null_entries);
    }

    /// Process weighted frequencies
    fn process_frequencies_weighted(
        &self,
        all_unique_header: bool, /* Indicates the column has all-unique values (e.g., an ID
                                  * column). For all-unique columns, show a single <ALL_UNIQUE>
                                  * entry with the sum of all weights. */
        abs_dec_places: u32,
        _row_count: u64,
        weighted_map: &HashMap<Vec<u8>, f64>,
        processed_frequencies: &mut Vec<ProcessedFrequency>,
    ) {
        if all_unique_header {
            // For all-unique headers with weighted frequencies, create a single entry
            // with the sum of all weights
            let total_weight: f64 = weighted_map.values().sum();
            // Skip emitting an all-unique entry if the total weight is non-finite (NaN or infinity)
            // to avoid producing a misleading count, consistent with the non-all-unique path
            // where non-finite weights are skipped entirely.
            if !total_weight.is_finite() {
                return;
            }
            #[allow(clippy::cast_precision_loss)]
            let count = total_weight.clamp(0.0, u64::MAX as f64).round() as u64;
            processed_frequencies.push(ProcessedFrequency {
                value: ALL_UNIQUE_TEXT.get().unwrap().clone(),
                count,
                percentage: 100.0,
                formatted_percentage: self.format_percentage(100.0, abs_dec_places),
                rank: 0.0, // Rank 0 for all-unique headers
            });
            return;
        }

        // For non-all-unique columns, process individual weighted values
        let mut counts_to_process = self.counts_weighted(weighted_map);
        self.move_other_to_end_if_needed(&mut counts_to_process);
        self.move_null_to_end_if_needed(&mut counts_to_process);

        // Convert to processed frequencies (count is f64, convert to u64 for display)
        for (value, weight, percentage, rank) in counts_to_process {
            // Skip non-finite weights (NaN or infinity) to avoid emitting misleading zero-count
            // rows.
            if !weight.is_finite() {
                continue;
            }
            #[allow(clippy::cast_precision_loss)]
            let count = weight.clamp(0.0, u64::MAX as f64).round() as u64;
            processed_frequencies.push(ProcessedFrequency {
                value,
                count,
                percentage,
                formatted_percentage: self.format_percentage(percentage, abs_dec_places),
                rank,
            });
        }
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
            processed_frequencies.push(ProcessedFrequency {
                value:                ALL_UNIQUE_TEXT.get().unwrap().clone(),
                count:                row_count,
                percentage:           100.0,
                formatted_percentage: self.format_percentage(100.0, abs_dec_places),
                rank:                 0.0, // Rank 0 for all-unique headers
            });
        } else {
            // Process regular frequencies
            let mut counts_to_process = self.counts(ftab);
            self.move_other_to_end_if_needed(&mut counts_to_process);
            self.move_null_to_end_if_needed(&mut counts_to_process);

            // Convert to processed frequencies
            for (value, count, percentage, rank) in counts_to_process {
                processed_frequencies.push(ProcessedFrequency {
                    value,
                    count,
                    percentage,
                    formatted_percentage: self.format_percentage(percentage, abs_dec_places),
                    rank,
                });
            }
        }
    }

    /// Format percentage with proper decimal places
    fn format_percentage(&self, percentage: f64, abs_dec_places: u32) -> String {
        // Sentinel value -1.0 indicates NULL entry with --pct-nulls=false
        if percentage < 0.0 {
            return String::new();
        }
        let pct_decimal = Decimal::from_f64(percentage).unwrap_or_default();
        let pct_scale = if self.flag_pct_dec_places < 0 {
            let current_scale = pct_decimal.scale();
            current_scale.max(abs_dec_places)
        } else {
            abs_dec_places
        };
        let final_pct_decimal = pct_decimal
            .round_dp_with_strategy(
                pct_scale,
                rust_decimal::RoundingStrategy::MidpointAwayFromZero,
            )
            .normalize();
        // Optimize: Check scale directly instead of converting to string for length check
        if final_pct_decimal.scale() > abs_dec_places {
            final_pct_decimal
                .round_dp_with_strategy(abs_dec_places, RoundingStrategy::MidpointAwayFromZero)
                .normalize()
                .to_string()
        } else {
            final_pct_decimal.to_string()
        }
    }

    /// Process weighted frequencies from HashMap and return same format as counts()
    #[allow(clippy::cast_precision_loss)]
    fn counts_weighted(
        &self,
        weighted_map: &HashMap<Vec<u8>, f64>,
    ) -> Vec<(ByteString, f64, f64, f64)> {
        // Convert HashMap to Vec and sort
        let mut counts: Vec<(Vec<u8>, f64)> =
            weighted_map.iter().map(|(k, v)| (k.clone(), *v)).collect();

        // Sort by count (weight)
        if self.flag_asc {
            counts.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        } else {
            counts.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        }

        // Calculate total weight (sum of all weights)
        let total_weight: f64 = weighted_map.values().sum();

        // When --pct-nulls is false, extract NULL entries before ranking
        // so that non-NULL values get correct ranks (excluding NULLs from ranking)
        let null_entry = if self.flag_pct_nulls {
            None
        } else {
            counts
                .iter()
                .position(|(k, _)| k.is_empty())
                .map(|pos| counts.remove(pos))
        };

        // Calculate NULL weight for adjusted percentage
        let null_weight = null_entry.as_ref().map_or(0.0, |(_, w)| *w);

        // Apply limits
        let unique_counts_len = counts.len();
        apply_limits_weighted(&mut counts, self.flag_limit, self.flag_lmt_threshold);

        // Calculate pct_factor: when --pct-nulls is false, exclude NULLs from denominator
        let adjusted_total = total_weight - null_weight;
        let pct_factor = if adjusted_total > 0.0 {
            100.0_f64 / adjusted_total
        } else {
            0.0_f64
        };

        // Compute tolerance once before the loop (outside hot path)
        // Use stats cache if available to determine weight scale for more accurate tolerance
        let weight_tolerance = if let Some(ref weight_col) = self.flag_weight {
            STATS_RECORDS
                .get()
                .and_then(|records| records.get(weight_col))
                .and_then(|stats| {
                    // Prefer stddev as it represents the scale of variation
                    // Fall back to range or mean if stddev not available
                    stats
                        .stddev
                        .or(stats.range)
                        .or(stats.mean)
                        .filter(|&s| s > 0.0)
                })
                // Use a scale-aware tolerance with a minimum absolute epsilon to handle
                // both small and large weight scales robustly.
                .map_or(f64::EPSILON, |scale| (scale * 1e-6).max(1e-10))
        } else {
            f64::EPSILON
        };

        // Group by weight to handle ties
        let weight_groups = group_by_weight(counts, weight_tolerance);

        // safety: NULL_VAL is set in the main function
        let null_val = NULL_VAL.get().unwrap();

        // Apply ranking strategy (NULLs already removed when pct_nulls is false)
        let (mut counts_final, count_sum, pct_sum) = apply_ranking_strategy_weighted(
            weight_groups,
            self.flag_rank_strategy,
            pct_factor,
            null_val,
            self.flag_pct_nulls,
        );

        // Add NULL entry back with sentinel values when --pct-nulls is false
        // Insert at the correct sorted position based on weight to preserve sort order
        if let Some((_, null_weight_val)) = null_entry {
            let null_entry_final = (null_val.to_vec(), null_weight_val, -1.0, -1.0);
            // Find the correct insertion position based on weight (descending or ascending)
            let insert_pos = if self.flag_asc {
                // Ascending: find first position where weight > null_weight
                counts_final
                    .iter()
                    .position(|(_, w, _, _)| *w > null_weight_val)
                    .unwrap_or(counts_final.len())
            } else {
                // Descending: find first position where weight < null_weight
                counts_final
                    .iter()
                    .position(|(_, w, _, _)| *w < null_weight_val)
                    .unwrap_or(counts_final.len())
            };
            counts_final.insert(insert_pos, null_entry_final);
        }

        // Calculate "Other" category
        // When NULL was extracted, adjust calculations to exclude it
        let (adjusted_other_weight, adjusted_unique_len) = if null_weight > 0.0 {
            // Subtract null_weight from other_weight since NULL is handled separately
            (
                total_weight - count_sum - null_weight,
                unique_counts_len.saturating_sub(1), // NULL was removed from counts
            )
        } else {
            (total_weight - count_sum, unique_counts_len)
        };

        // When NULL was extracted and re-added, don't count it as a "shown" entry
        // because it's separate from the top-k values
        let shown_count = if null_weight > 0.0 {
            counts_final.len().saturating_sub(1)
        } else {
            counts_final.len()
        };
        let other_unique_count = adjusted_unique_len.saturating_sub(shown_count);
        // Only create Other entry if there are actually remaining unique values
        // and the weight is positive. This prevents "Other (0)" entries when
        // --limit 0 is used and all values are included.
        // Use 100.0 - pct_sum to ensure percentages sum exactly to 100%,
        // handling floating-point precision issues consistently with unweighted case.
        if adjusted_other_weight > 0.0 && other_unique_count > 0 && self.flag_other_text != "<NONE>"
        {
            counts_final.push((
                format!(
                    "{} ({})",
                    self.flag_other_text,
                    HumanCount(other_unique_count as u64)
                )
                .as_bytes()
                .to_vec(),
                adjusted_other_weight,
                100.0_f64 - pct_sum,
                0.0,
            ));
        }

        counts_final
    }

    #[inline]
    fn counts(&self, ftab: &FTable) -> Vec<(ByteString, u64, f64, f64)> {
        let (counts_ref, total_count) = if self.flag_asc {
            // parallel sort in ascending order - least frequent values first
            ftab.par_frequent(true)
        } else {
            // parallel sort in descending order - most frequent values first
            ftab.par_frequent(false)
        };

        // Convert references to owned values
        let mut counts: Vec<(Vec<u8>, u64)> = counts_ref
            .into_iter()
            .map(|(k, v)| (k.clone(), v))
            .collect();

        // check if we need to apply limits
        let unique_counts_len = counts.len();
        let all_unique = if unique_counts_len > 0 {
            counts[if self.flag_asc {
                unique_counts_len - 1
            } else {
                0
            }]
            .1 == 1
        } else {
            false
        };

        // When --pct-nulls is false, extract NULL entries before ranking
        // so that non-NULL values get correct ranks (excluding NULLs from ranking)
        let null_entry = if self.flag_pct_nulls {
            None
        } else {
            // Find and remove NULL entry from counts
            counts
                .iter()
                .position(|(k, _)| k.is_empty())
                .map(|pos| counts.remove(pos))
        };

        // Calculate NULL count for adjusted percentage
        let null_count = null_entry.as_ref().map_or(0, |(_, c)| *c);

        // Apply limits (including unique limit logic for all-unique columns)
        apply_limits_unweighted(
            &mut counts,
            self.flag_limit,
            self.flag_unq_limit,
            self.flag_lmt_threshold,
            all_unique,
        );

        // Calculate pct_factor: when --pct-nulls is false, exclude NULLs from denominator
        let adjusted_total = total_count.saturating_sub(null_count);
        let pct_factor = if adjusted_total > 0 {
            100.0_f64 / adjusted_total.to_f64().unwrap_or(1.0_f64)
        } else {
            0.0_f64
        };

        // Group by count to handle ties
        let count_groups = group_by_count(counts);

        // safety: NULL_VAL is set in the main function
        let null_val = NULL_VAL.get().unwrap();

        // Apply ranking strategy (NULLs already removed when pct_nulls is false)
        let (mut counts_final, count_sum, pct_sum) = apply_ranking_strategy_unweighted(
            count_groups,
            self.flag_rank_strategy,
            pct_factor,
            null_val,
            self.flag_pct_nulls,
        );

        // Add NULL entry back with sentinel values when --pct-nulls is false
        // Insert at the correct sorted position based on count to preserve sort order
        if let Some((_, null_count_val)) = null_entry {
            let null_entry_final = (null_val.to_vec(), null_count_val, -1.0, -1.0);
            // Find the correct insertion position based on count (descending or ascending)
            let insert_pos = if self.flag_asc {
                // Ascending: find first position where count > null_count
                counts_final
                    .iter()
                    .position(|(_, c, _, _)| *c > null_count_val)
                    .unwrap_or(counts_final.len())
            } else {
                // Descending: find first position where count < null_count
                counts_final
                    .iter()
                    .position(|(_, c, _, _)| *c < null_count_val)
                    .unwrap_or(counts_final.len())
            };
            counts_final.insert(insert_pos, null_entry_final);
        }

        // Calculate "Other" category
        // When NULL was extracted, adjust calculations to exclude it
        let (adjusted_other_count, adjusted_unique_len) = if null_count > 0 {
            // Subtract null_count from other_count since NULL is handled separately
            (
                total_count
                    .saturating_sub(count_sum)
                    .saturating_sub(null_count),
                unique_counts_len.saturating_sub(1), // NULL was removed from counts
            )
        } else {
            (total_count - count_sum, unique_counts_len)
        };

        if adjusted_other_count > 0 && self.flag_other_text != "<NONE>" {
            // When NULL was extracted and re-added, don't count it as a "shown" entry
            // because it's separate from the top-k values
            let shown_count = if null_count > 0 {
                counts_final.len().saturating_sub(1)
            } else {
                counts_final.len()
            };
            let other_unique_count = adjusted_unique_len.saturating_sub(shown_count);
            counts_final.push((
                format!(
                    "{} ({})",
                    self.flag_other_text,
                    HumanCount(other_unique_count as u64)
                )
                .as_bytes()
                .to_vec(),
                adjusted_other_count,
                100.0_f64 - pct_sum,
                0.0, // Special rank for "Other" category
            ));
        }

        counts_final
    }

    pub fn sequential_ftables(&self) -> CliResult<(Headers, FTables, Option<WeightedFTables>)> {
        let mut rdr = self.rconfig().reader()?;
        let (headers, sel, weight_col_idx) = self.sel_headers(&mut rdr)?;
        if weight_col_idx.is_some() {
            let weighted =
                self.ftables_weighted_internal(&sel, rdr.byte_records(), 1, weight_col_idx);
            Ok((headers, vec![], Some(weighted)))
        } else {
            Ok((
                headers,
                self.ftables_unweighted(&sel, rdr.byte_records(), 1),
                None,
            ))
        }
    }

    pub fn parallel_ftables(
        &self,
        idx: &Indexed<fs::File, fs::File>,
    ) -> CliResult<(Headers, FTables, Option<WeightedFTables>)> {
        let mut rdr = self.rconfig().reader()?;
        let (headers, sel, weight_col_idx) = self.sel_headers(&mut rdr)?;

        let idx_count = idx.count() as usize;
        if idx_count == 0 {
            return Ok((headers, vec![], None));
        }

        let njobs = util::njobs(self.flag_jobs);

        // Read memory limit from environment variable

        // Read memory limit from environment variable
        // If QSV_FREQ_CHUNK_MEMORY_MB is set & valid, set max chunk memory
        // If QSV_FREQ_CHUNK_MEMORY_MB is not set, use 0 (dynamic sizing)
        // If QSV_FREQ_CHUNK_MEMORY_MB is set to a value that cannot be parsed as u64 (e.g., -1 or
        // any invalid/non-positive value), use CPU-based chunking
        let max_chunk_memory_mb = if let Ok(val) = std::env::var("QSV_FREQ_CHUNK_MEMORY_MB") {
            // if valid, set max chunk memory
            // if invalid (cannot be parsed as u64), use CPU-based chunking
            atoi_simd::parse::<u64>(val.as_bytes()).ok()
        } else {
            Some(0) // default to dynamic sizing
        };

        // Sample first 1000 records for memory estimation
        // Always sample when QSV_FREQ_CHUNK_MEMORY_MB is set to ANY value (including 0)
        // or when not set (to enable dynamic sizing)
        let sample_records = if max_chunk_memory_mb.is_some() {
            util::sample_records(&self.rconfig(), 1000)
        } else {
            None
        };

        let (chunking_mode, chunk_size) = if let Some(limit_mb) = max_chunk_memory_mb {
            // Calculate memory-aware chunk size
            let chunk_size = calculate_memory_aware_chunk_size_for_frequency(
                idx_count as u64,
                njobs,
                max_chunk_memory_mb,
                sample_records.as_deref(),
            );

            // Log chunk size and memory estimates for debugging
            // Log when memory-aware chunking is active (either explicitly set or automatically
            // enabled) Estimate average record size from samples if available
            let avg_record_size = if let Some(samples) = sample_records {
                calculate_avg_record_size_for_frequency(&samples)
            } else {
                1024 // Default: 1KB per record
            };

            let estimated_memory_mb =
                estimate_chunk_memory_for_frequency(chunk_size, avg_record_size, headers.len())
                    / (1024 * 1024);
            let chunking_mode = if limit_mb == 0 {
                "dynamic (auto)"
            } else {
                "fixed limit"
            };
            (
                format!(
                    "Memory-aware chunking ({chunking_mode}): chunk_size={chunk_size}, \
                     estimated_memory_mb={estimated_memory_mb:.2}"
                ),
                chunk_size,
            )
        } else {
            let chunk_size = util::chunk_size(idx_count, njobs);
            (
                format!("CPU-based chunking: chunk_size={chunk_size}"),
                chunk_size,
            )
        };

        let nchunks = util::num_of_chunks(idx_count, chunk_size);
        log::info!("({chunking_mode}) nchunks={nchunks}");

        if weight_col_idx.is_some() {
            // Parallel weighted frequencies
            let pool = ThreadPool::new(njobs);
            let (send, recv) = crossbeam_channel::bounded(nchunks);
            for i in 0..nchunks {
                let (send, args, sel, weight_idx) =
                    (send.clone(), self.clone(), sel.clone(), weight_col_idx);
                pool.execute(move || {
                    let mut idx = args.rconfig().indexed().unwrap().unwrap();
                    idx.seek((i * chunk_size) as u64).unwrap();
                    let it = idx.byte_records().take(chunk_size);
                    send.send(args.ftables_weighted_internal(&sel, it, nchunks, weight_idx))
                        .unwrap();
                });
            }
            drop(send);

            // Merge weighted frequencies
            let mut merged: WeightedFTables = Vec::new();
            for weighted_chunk in &recv {
                if merged.is_empty() {
                    merged = weighted_chunk;
                } else {
                    // Merge HashMaps
                    for (col_idx, weighted_map) in weighted_chunk.into_iter().enumerate() {
                        if col_idx < merged.len() {
                            for (value, weight) in weighted_map {
                                *merged[col_idx].entry(value).or_insert(0.0) += weight;
                            }
                        } else {
                            merged.push(weighted_map);
                        }
                    }
                }
            }
            Ok((headers, vec![], Some(merged)))
        } else {
            // Parallel unweighted frequencies
            let pool = ThreadPool::new(njobs);
            let (send, recv) = crossbeam_channel::bounded(nchunks);
            for i in 0..nchunks {
                let (send, args, sel) = (send.clone(), self.clone(), sel.clone());
                pool.execute(move || {
                    let mut idx = args.rconfig().indexed().unwrap().unwrap();
                    idx.seek((i * chunk_size) as u64).unwrap();
                    let it = idx.byte_records().take(chunk_size);
                    send.send(args.ftables_unweighted(&sel, it, nchunks))
                        .unwrap();
                });
            }
            drop(send);
            Ok((headers, merge_all(recv.iter()).unwrap(), None))
        }
    }

    #[inline]
    fn ftables_weighted_internal<I>(
        &self,
        sel: &Selection,
        it: I,
        nchunks: usize,
        weight_col_idx: Option<usize>,
    ) -> WeightedFTables
    where
        I: Iterator<Item = csv::Result<csv::ByteRecord>>,
    {
        // Extract the weighted HashMap implementation from ftables_weighted
        // This is a duplicate of the weighted logic but returns WeightedFTables
        let sel_len = sel.len();

        #[allow(unused_assignments)]
        let mut field_buffer: Vec<u8> = Vec::with_capacity(1024);
        let mut row_buffer: csv::ByteRecord = csv::ByteRecord::with_capacity(200, sel_len);
        let mut string_buf = String::with_capacity(512);

        // For weighted frequencies, we process all columns including all-unique ones
        // because the weights provide meaningful information, so we don't need to track
        // which columns are all-unique here (unlike unweighted frequencies where all-unique
        // columns are skipped for memory efficiency)
        let flag_no_nulls = self.flag_no_nulls;
        let flag_ignore_case = self.flag_ignore_case;
        let flag_no_trim = self.flag_no_trim;

        let col_cardinality_vec = COL_CARDINALITY_VEC.get().unwrap_or(&EMPTY_VEC);
        let mut weighted_freq_tables: Vec<HashMap<Vec<u8>, f64>> = if col_cardinality_vec.is_empty()
        {
            (0..sel_len).map(|_| HashMap::with_capacity(1000)).collect()
        } else {
            (0..sel_len)
                .map(|i| {
                    // For weighted frequencies, we process all columns including all-unique ones
                    // so we use the actual cardinality (or a reasonable default) rather than 1
                    let capacity = if nchunks == 1 {
                        col_cardinality_vec
                            .get(i)
                            .map_or(1000, |(_, cardinality)| *cardinality as usize)
                    } else {
                        let cardinality = col_cardinality_vec
                            .get(i)
                            .map_or(1000, |(_, cardinality)| *cardinality as usize);
                        cardinality / nchunks
                    };
                    HashMap::with_capacity(capacity)
                })
                .collect()
        };

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
            #[inline]
            |field: &[u8], _buf: &mut String| trim_bs_whitespace(field).to_vec()
        };

        let mut row_result: csv::ByteRecord;
        for row in it {
            // safety: we know the row is valid because it comes from an iterator
            row_result = unsafe { row.unwrap_unchecked() };
            row_buffer.clone_from(&row_result);

            let weight = if let Some(widx) = weight_col_idx {
                if widx < row_result.len() {
                    // safety: widx < row_result.len() is checked above, so get(widx)
                    // will always return Some(&[u8])
                    fast_float2::parse::<f64, &[u8]>(row_result.get(widx).unwrap()).unwrap_or(1.0)
                } else {
                    1.0
                }
            } else {
                1.0
            };

            if !weight.is_finite() || weight <= 0.0 {
                continue;
            }

            for (i, field) in sel.select(&row_buffer).enumerate() {
                // For weighted frequencies, we process all columns including all-unique ones
                // because the weights provide meaningful information even when values are unique
                // (unlike unweighted frequencies where all-unique columns are skipped for memory
                // efficiency)

                // safety: weighted_freq_tables is pre-allocated with sel_len elements.
                // i will always be < sel_len as it comes from enumerate() over the selected cols
                if !field.is_empty() {
                    field_buffer = process_field(field, &mut string_buf);
                    unsafe {
                        *weighted_freq_tables
                            .get_unchecked_mut(i)
                            .entry(field_buffer)
                            .or_insert(0.0) += weight;
                    }
                } else if !flag_no_nulls {
                    unsafe {
                        *weighted_freq_tables
                            .get_unchecked_mut(i)
                            .entry(EMPTY_BYTE_VEC.clone())
                            .or_insert(0.0) += weight;
                    }
                }
            }
        }

        weighted_freq_tables
    }

    #[inline]
    fn ftables_unweighted<I>(&self, sel: &Selection, it: I, nchunks: usize) -> FTables
    where
        I: Iterator<Item = csv::Result<csv::ByteRecord>>,
    {
        let sel_len = sel.len();

        #[allow(unused_assignments)]
        // Optimize buffer allocations
        let mut field_buffer: Vec<u8> = Vec::with_capacity(1024);
        let mut row_buffer: csv::ByteRecord = csv::ByteRecord::with_capacity(200, sel_len);
        let mut string_buf = String::with_capacity(512);

        let unique_headers_vec = UNIQUE_COLUMNS_VEC.get().unwrap();

        // assign flags to local variables for faster access
        let flag_no_nulls = self.flag_no_nulls;
        let flag_ignore_case = self.flag_ignore_case;
        let flag_no_trim = self.flag_no_trim;

        // compile a vector of bool flags for all_unique_headers
        // so we can skip the contains check in the hot loop below
        let all_unique_flag_vec: Vec<bool> = (0..sel_len)
            .map(|i| unique_headers_vec.contains(&i))
            .collect();

        // optimize the capacity of the freq_tables based on the cardinality of the columns
        // if sequential, use the cardinality from the stats cache
        // if parallel, use a default capacity of 1000 for non-unique columns
        let col_cardinality_vec = COL_CARDINALITY_VEC.get().unwrap_or(&EMPTY_VEC);
        let mut freq_tables: Vec<_> = if col_cardinality_vec.is_empty() {
            (0..sel_len)
                .map(|_| Frequencies::with_capacity(1000))
                .collect()
        } else {
            (0..sel_len)
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
            for (i, field) in sel.select(&row_buffer).enumerate() {
                // safety: all_unique_flag_vec is pre-computed to have exactly sel_len elements,
                // which matches the number of selected columns that we iterate over.
                // i will always be < sel_len as it comes from enumerate() over the selected cols
                if unsafe { *all_unique_flag_vec.get_unchecked(i) } {
                    continue;
                }

                // safety: freq_tables is pre-allocated with sel_len elements.
                // i will always be < sel_len as it comes from enumerate() over the selected cols
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
        // if parallel (nchunks > 1), shrink the capacity to avoid over-allocating memory
        if nchunks > 1 {
            freq_tables.shrink_to_fit();
        }
        freq_tables
    }

    /// Compute indices of Float columns that should be skipped from frequency analysis.
    ///
    /// # Arguments
    /// * `headers` - The selected CSV headers
    /// * `col_type_map` - Map of column names to their data types from stats cache
    ///
    /// # Returns
    /// A vector of indices (into the selected headers) of Float columns to skip.
    /// Columns listed in the --no-float exception list are NOT included in the skip list.
    fn compute_float_columns_to_skip(
        &self,
        headers: &Headers,
        col_type_map: &HashMap<String, String>,
    ) -> Vec<usize> {
        // Parse exception columns from flag value
        // If value is "*", exclude ALL Float columns (empty exception list)
        // Otherwise, parse as comma-separated list of Float columns to INCLUDE
        let exception_cols: HashSet<String> = self
            .flag_no_float
            .as_ref()
            .map(|cols| {
                let trimmed = cols.trim();
                // "*" means exclude all floats (no exceptions)
                if trimmed == "*" {
                    HashSet::new()
                } else {
                    trimmed
                        .split(',')
                        .map(|s| s.trim().to_lowercase())
                        .filter(|s| !s.is_empty() && s != "*")
                        .collect()
                }
            })
            .unwrap_or_default();

        let mut float_columns_to_skip = Vec::new();

        for (i, header) in headers.iter().enumerate() {
            let header_str = simdutf8::basic::from_utf8(header)
                .unwrap_or(NON_UTF8_ERR)
                .to_string();

            // Check if column is Float type and not in exception list
            if let Some(col_type) = col_type_map.get(&header_str)
                && col_type == "Float"
                && !exception_cols.contains(&header_str.to_lowercase())
            {
                float_columns_to_skip.push(i);
            }
        }

        float_columns_to_skip
    }

    /// Compute indices of columns that should be skipped based on the --stats-filter expression.
    ///
    /// # Arguments
    /// * `headers` - The selected CSV headers
    /// * `stats_records` - Map of column names to their stats data from stats cache
    /// * `filter_expression` - The Luau expression to evaluate for each column
    ///
    /// # Returns
    /// A vector of indices (into the selected headers) of columns where the filter
    /// expression evaluated to `true` (meaning they should be excluded).
    #[allow(clippy::unused_self)]
    #[cfg(feature = "luau")]
    fn compute_stats_filter_columns_to_skip(
        &self,
        headers: &Headers,
        stats_records: &HashMap<String, StatsData>,
        filter_expression: &str,
    ) -> CliResult<Vec<usize>> {
        use mlua::Lua;

        // Create a single sandboxed Luau VM for all column evaluations
        let lua = Lua::new();
        lua.sandbox(true)
            .map_err(|e| format!("Failed to enable Luau sandbox: {e}"))?;

        let mut columns_to_skip = Vec::new();

        for (i, header) in headers.iter().enumerate() {
            let header_str = simdutf8::basic::from_utf8(header)
                .unwrap_or(NON_UTF8_ERR)
                .to_string();

            // Look up the stats record for this column
            if let Some(stats_data) = stats_records.get(&header_str) {
                // Evaluate the filter expression against this column's stats
                match evaluate_stats_filter(&lua, stats_data, filter_expression) {
                    Ok(should_exclude) => {
                        if should_exclude {
                            log::debug!(
                                "Column '{header_str}' excluded by --stats-filter expression"
                            );
                            columns_to_skip.push(i);
                        }
                    },
                    Err(e) => {
                        return fail_clierror!(
                            "Error evaluating --stats-filter expression for column \
                             '{header_str}': {e}"
                        );
                    },
                }
            } else {
                // No stats available for this column - skip filtering for it
                log::debug!(
                    "No stats available for column '{header_str}', skipping --stats-filter \
                     evaluation"
                );
            }
        }

        Ok(columns_to_skip)
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
            flag_strict_formats:  false,
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
            flag_output:          None,
        };
        let is_json = self.flag_json || self.flag_pretty_json || self.flag_toon;
        // Check if we need to populate stats_records_hashmap
        // We need it for JSON output and for --stats-filter
        #[cfg(feature = "luau")]
        let needs_stats_records = is_json || self.flag_stats_filter.is_some();
        #[cfg(not(feature = "luau"))]
        let needs_stats_records = is_json;

        // initialize the stats records hashmap
        let mut stats_records_hashmap = if needs_stats_records {
            HashMap::with_capacity(headers.len())
        } else {
            HashMap::new()
        };

        let (csv_fields, csv_stats) = get_stats_records(&schema_args, StatsMode::Frequency)?;

        if csv_fields.is_empty() || csv_stats.len() != csv_fields.len() {
            // the stats cache does not exist or the number of fields & stats records
            // do not match. Just return an empty vector.
            // we're not going to be able to get the cardinalities, so
            // this signals that we just compute frequencies for all columns
            return Ok(Vec::new());
        }

        // Build column name -> (cardinality, type) map for matching by name
        let mut col_type_map: HashMap<String, String> = HashMap::with_capacity(csv_stats.len());

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
                if needs_stats_records {
                    // Store the stats records hashmap for later use when producing JSON output
                    // or for --stats-filter evaluation
                    stats_records_hashmap.insert(col_name_str.clone(), stats_record.clone());
                }
                // Store type info for Float column detection
                col_type_map.insert(col_name_str.clone(), stats_record.r#type.clone());
                (col_name_str, stats_record.cardinality)
            })
            .collect();

        // now, get the unique headers, where cardinality == rowcount
        let row_count = util::count_rows(&self.rconfig()).unwrap_or_default();
        FREQ_ROW_COUNT.set(row_count).unwrap();

        // Most datasets have relatively few columns with all unique values (e.g. ID columns)
        // so pre-allocate space for 5 as a reasonable default capacity
        let mut all_unique_headers_vec: Vec<usize> = Vec::with_capacity(5);
        for (i, header) in headers.iter().enumerate() {
            // Look up cardinality by column name, not index, since headers may be a
            // user-selected subset in a different order than the original CSV columns
            let cardinality = col_cardinality_vec
                .iter()
                .find(|(name, _)| {
                    name == simdutf8::basic::from_utf8(header).unwrap_or(NON_UTF8_ERR)
                })
                .map_or(0, |(_, card)| *card);

            if cardinality == row_count {
                all_unique_headers_vec.push(i);
            }
        }

        // Compute Float columns to skip if --no-float is specified
        if self.flag_no_float.is_some() {
            let float_columns_to_skip = self.compute_float_columns_to_skip(headers, &col_type_map);
            // safety: we only set this once per invocation
            let _ = FLOAT_COLUMNS_TO_SKIP.set(float_columns_to_skip);
        }

        // Compute stats filter columns to skip if --stats-filter is specified
        #[cfg(feature = "luau")]
        if let Some(ref filter_expression) = self.flag_stats_filter {
            if stats_records_hashmap.is_empty() {
                log::warn!(
                    "Stats cache unavailable. Cannot apply --stats-filter. Run 'qsv stats \
                     --cardinality --stats-jsonl' first."
                );
            } else {
                let stats_filter_columns_to_skip = self.compute_stats_filter_columns_to_skip(
                    headers,
                    &stats_records_hashmap,
                    filter_expression,
                )?;
                // safety: we only set this once per invocation
                let _ = STATS_FILTER_COLUMNS_TO_SKIP.set(stats_filter_columns_to_skip);
            }
        }

        COL_CARDINALITY_VEC.get_or_init(|| col_cardinality_vec);

        if is_json {
            // Store the stats records hashmap for later use when producing JSON output
            STATS_RECORDS.set(stats_records_hashmap).unwrap();
        }

        Ok(all_unique_headers_vec)
    }

    #[allow(clippy::cast_precision_loss)]
    fn output_json(
        &self,
        headers: &Headers,
        tables: FTables,
        weighted_tables: Option<&WeightedFTables>,
        rconfig: &Config,
        argv: &[&str],
        is_stdin: bool,
    ) -> CliResult<()> {
        let fieldcount = headers.len();

        // init vars and amortize allocations
        let mut fields = Vec::with_capacity(fieldcount);
        let rowcount = *FREQ_ROW_COUNT.get().unwrap_or(&0);
        let unique_headers_vec = UNIQUE_COLUMNS_VEC.get().unwrap();
        let mut processed_frequencies = Vec::with_capacity(headers.len());
        let abs_dec_places = self.flag_pct_dec_places.unsigned_abs() as u32;
        // pre-allocate space for 17 field stats, see list below for details
        let mut field_stats: Vec<FieldStats> = Vec::with_capacity(17);

        // Helper function to build a frequency field for JSON output
        let build_frequency_field = |field_name: String,
                                     cardinality: u64,
                                     processed_frequencies: &mut Vec<ProcessedFrequency>,
                                     field_stats: &mut Vec<FieldStats>,
                                     skip_stats: bool| {
            // Sort frequencies by count if flag_other_sorted
            if self.flag_other_sorted {
                if self.flag_asc {
                    processed_frequencies.sort_unstable_by_key(|a| a.count);
                } else {
                    processed_frequencies.sort_unstable_by_key(|b| std::cmp::Reverse(b.count));
                }
            }

            // Get stats record for this field
            let stats_record = STATS_RECORDS
                .get()
                .and_then(|records| records.get(&field_name));

            // Get data type and nullcount from stats record
            let dtype = stats_record.map_or(String::new(), |sr| sr.r#type.clone());
            let nullcount = stats_record.map_or(0, |sr| sr.nullcount);
            let sparsity =
                fast_float2::parse(util::round_num(nullcount as f64 / rowcount as f64, 4))
                    .unwrap_or(0.0);
            let uniqueness_ratio =
                fast_float2::parse(util::round_num(cardinality as f64 / rowcount as f64, 4))
                    .unwrap_or(0.0);

            // Build stats vector from stats record if type is not empty and not NULL or Boolean
            // Skip stats when using weighted mode (stats would be misleading)
            if !self.flag_no_stats
                && !skip_stats
                && !dtype.is_empty()
                && dtype.as_str() != "NULL"
                && dtype.as_str() != "Boolean"
                && let Some(sr) = stats_record
            {
                // Add all available stats if some
                add_stat(field_stats, "sum", sr.sum);
                add_stat(field_stats, "min", sr.min.clone());
                add_stat(field_stats, "max", sr.max.clone());
                add_stat(field_stats, "range", sr.range);
                add_stat(field_stats, "sort_order", sr.sort_order.clone());

                // String-specific length stats
                add_stat(field_stats, "min_length", sr.min_length);
                add_stat(field_stats, "max_length", sr.max_length);
                add_stat(field_stats, "sum_length", sr.sum_length);
                add_stat(field_stats, "avg_length", sr.avg_length);
                add_stat(field_stats, "stddev_length", sr.stddev_length);
                add_stat(field_stats, "variance_length", sr.variance_length);
                add_stat(field_stats, "cv_length", sr.cv_length);

                // Numeric-specific stats
                add_stat(field_stats, "mean", sr.mean);
                add_stat(field_stats, "sem", sr.sem);
                add_stat(field_stats, "stddev", sr.stddev);
                add_stat(field_stats, "variance", sr.variance);
                add_stat(field_stats, "cv", sr.cv);
            }

            FrequencyField {
                field: field_name,
                r#type: dtype,
                cardinality,
                nullcount,
                sparsity,
                uniqueness_ratio,
                stats: std::mem::take(field_stats),
                frequencies: processed_frequencies
                    .iter()
                    .map(|pf| {
                        // Sentinel value -1.0 indicates NULL entry with --pct-nulls=false
                        let (pct_opt, rank_opt) = if pf.percentage < 0.0 {
                            (None, None)
                        } else {
                            (
                                Some(
                                    fast_float2::parse(&pf.formatted_percentage)
                                        .unwrap_or(pf.percentage),
                                ),
                                Some(pf.rank),
                            )
                        };
                        FrequencyEntry {
                            value:      if self.flag_vis_whitespace {
                                util::visualize_whitespace(&String::from_utf8_lossy(&pf.value))
                            } else {
                                String::from_utf8_lossy(&pf.value).into_owned()
                            },
                            count:      pf.count,
                            percentage: pct_opt,
                            rank:       rank_opt,
                        }
                    })
                    .collect(),
            }
        };

        if let Some(weighted) = weighted_tables {
            // Process weighted frequencies for JSON output
            for (i, header) in headers.iter().enumerate() {
                let field_name = if rconfig.no_headers {
                    (i + 1).to_string()
                } else {
                    String::from_utf8_lossy(header).to_string()
                };

                let all_unique_header = unique_headers_vec.contains(&i);
                if i < weighted.len() {
                    self.process_frequencies_weighted(
                        all_unique_header,
                        abs_dec_places,
                        rowcount,
                        &weighted[i],
                        &mut processed_frequencies,
                    );
                }

                // Calculate cardinality for this field
                let cardinality = if all_unique_header {
                    rowcount
                } else if i < weighted.len() {
                    weighted[i].len() as u64
                } else {
                    0
                };

                fields.push(build_frequency_field(
                    field_name,
                    cardinality,
                    &mut processed_frequencies,
                    &mut field_stats,
                    true, // Skip stats for weighted mode
                ));

                processed_frequencies.clear(); // clear for next field
            }
        } else {
            // Process unweighted frequencies for JSON output
            let head_ftables = headers.iter().zip(tables);
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

                // Calculate cardinality for this field
                let cardinality = if all_unique_header {
                    rowcount // For all-unique fields, cardinality == rowcount
                } else {
                    ftab.len() as u64 // otherwise, cardinality == number of unique values
                };

                fields.push(build_frequency_field(
                    field_name,
                    cardinality,
                    &mut processed_frequencies,
                    &mut field_stats,
                    false, // Include stats for non-weighted mode
                ));

                processed_frequencies.clear(); // clear for next field
            }
        }

        let output = FrequencyOutput {
            input: if is_stdin {
                "stdin".to_string()
            } else {
                // safety: we know arg_input is not None
                self.arg_input.clone().unwrap()
            },
            description: format!("Generated with `qsv {}`", argv[1..].join(" ")),
            rowcount: if rowcount == 0 {
                // if rowcount == 0, derive the rowcount from first field's frequencies
                // by summing the counts for the first field
                fields
                    .first()
                    .map_or(0, |field| field.frequencies.iter().map(|f| f.count).sum())
            } else {
                rowcount
            },
            fieldcount,
            fields,
            rank_strategy: self.flag_rank_strategy,
        };

        if self.flag_toon {
            // TOON output - encode the JSON structure to TOON format
            // First serialize to JSON Value, then remove empty stats, then encode to TOON
            let mut json_value = serde_json::to_value(&output)?;

            // Remove empty stats arrays from each field (same as JSON output)
            if let Some(fields) = json_value.get_mut("fields").and_then(|f| f.as_array_mut()) {
                for field in fields {
                    if let Some(field_obj) = field.as_object_mut() {
                        // Remove empty stats
                        if let Some(stats) = field_obj.get("stats")
                            && let Some(stats_array) = stats.as_array()
                            && stats_array.is_empty()
                        {
                            field_obj.remove("stats");
                        }
                    }
                }
            }

            let opts = EncodeOptions::new();
            let toon_output = encode(&json_value, &opts)
                .map_err(|e| crate::CliError::Other(format!("Failed to encode to TOON: {e}")))?;
            if let Some(output_path) = &self.flag_output {
                std::fs::write(output_path, toon_output)?;
            } else {
                println!("{toon_output}");
            }
        } else {
            // JSON output
            let mut json_output = if self.flag_pretty_json {
                // pretty, with more whitespace
                serde_json::to_string_pretty(&output)?
            } else {
                // still pretty, but more compact and faster
                simd_json::to_string_pretty(&output)?
            };

            // remove all empty stats properties from the JSON output using regex
            // safety: regex pattern is a valid static string
            let re = regex::Regex::new(r#""stats": \[\],\n\s*"#).unwrap();
            json_output = re.replace_all(&json_output, "").to_string();

            if let Some(output_path) = &self.flag_output {
                std::fs::write(output_path, json_output)?;
            } else {
                println!("{json_output}");
            }
        }

        Ok(())
    }

    /// Processes headers and handles weight column exclusion if needed.
    ///
    /// This function handles the logic for excluding the weight column from frequency
    /// computation. It finds the weight column index, creates a modified selection that
    /// excludes it, and returns the selected headers.
    ///
    /// # Arguments
    ///
    /// * `full_headers` - The full CSV headers as a ByteRecord
    ///
    /// # Returns
    ///
    /// * `Ok((Option<usize>, Selection, csv::ByteRecord))` - Tuple containing:
    ///   - Weight column index (None if no weight column specified)
    ///   - Modified selection (excluding weight column if present)
    ///   - Selected headers (excluding weight column if present)
    /// * `Err(CliError)` - If weight column is not found or no columns remain after exclusion
    fn process_headers_with_weight_exclusion(
        &self,
        full_headers: &csv::ByteRecord,
    ) -> CliResult<(Option<usize>, Selection, csv::ByteRecord)> {
        if let Some(ref weight_col) = self.flag_weight {
            // Find weight column index in full headers
            let weight_idx = full_headers
                .iter()
                .position(|h| {
                    let h_str = String::from_utf8_lossy(h);
                    h_str.trim().eq_ignore_ascii_case(weight_col.trim())
                })
                .ok_or_else(|| {
                    crate::CliError::Other(format!(
                        "Weight column '{weight_col}' not found in CSV headers"
                    ))
                })?;

            // Create selection excluding weight column
            let sel = self.rconfig().selection(full_headers)?;
            // Remove weight column index from selection if present
            let sel_vec: Vec<usize> = sel
                .iter()
                .copied()
                .filter(|&idx| idx != weight_idx)
                .collect();

            // Validate that we still have columns after excluding the weight column
            if sel_vec.is_empty() {
                return Err(crate::CliError::Other(format!(
                    "After excluding weight column '{weight_col}', no columns remain for \
                     frequency computation"
                )));
            }

            // safety: We know Selection is a tuple struct with a Vec<usize> field
            // This is safe because we're creating it with valid indices
            let modified_sel = unsafe { std::mem::transmute::<Vec<usize>, Selection>(sel_vec) };

            // Get selected headers (excluding weight column)
            let selected_headers: csv::ByteRecord = modified_sel.select(full_headers).collect();

            Ok((Some(weight_idx), modified_sel, selected_headers))
        } else {
            // No weight column specified, use normal selection
            let sel = self.rconfig().selection(full_headers)?;
            let headers: csv::ByteRecord = sel.select(full_headers).collect();
            Ok((None, sel, headers))
        }
    }

    fn sel_headers<R: io::Read>(
        &self,
        rdr: &mut csv::Reader<R>,
    ) -> CliResult<(csv::ByteRecord, Selection, Option<usize>)> {
        let full_headers = rdr.byte_headers()?.clone();
        let (weight_col_idx, mut sel, selected_headers) =
            self.process_headers_with_weight_exclusion(&full_headers)?;

        let all_unique_headers_vec = self.get_unique_headers(&selected_headers)?;

        // Filter out Float columns if --no-float is specified
        let (final_sel, final_headers) = if self.flag_no_float.is_some() {
            if let Some(float_cols_to_skip) = FLOAT_COLUMNS_TO_SKIP.get() {
                if float_cols_to_skip.is_empty() {
                    (sel, selected_headers)
                } else {
                    // Filter selection to exclude Float columns
                    // float_cols_to_skip contains indices into selected_headers
                    let sel_vec: Vec<usize> = sel
                        .iter()
                        .copied()
                        .enumerate()
                        .filter(|(i, _)| !float_cols_to_skip.contains(i))
                        .map(|(_, idx)| idx)
                        .collect();

                    if sel_vec.is_empty() {
                        return Err(crate::CliError::Other(
                            "No columns remain after excluding Float columns. Use --no-float with \
                             exception columns to include specific Float columns."
                                .to_string(),
                        ));
                    }

                    // safety: We know Selection is a tuple struct with a Vec<usize> field
                    sel = unsafe { std::mem::transmute::<Vec<usize>, Selection>(sel_vec) };
                    let headers: csv::ByteRecord = sel.select(&full_headers).collect();
                    (sel, headers)
                }
            } else {
                // Stats cache unavailable for Float type detection
                log::warn!(
                    "Stats cache unavailable. Cannot detect Float columns for --no-float. \
                     Processing all columns. Run 'qsv stats --cardinality --stats-jsonl' first."
                );
                (sel, selected_headers)
            }
        } else {
            (sel, selected_headers)
        };

        // Filter out stats-filtered columns if --stats-filter is specified
        #[cfg(feature = "luau")]
        let (final_sel, final_headers) = if self.flag_stats_filter.is_some() {
            if let Some(stats_filter_cols_to_skip) = STATS_FILTER_COLUMNS_TO_SKIP.get() {
                if stats_filter_cols_to_skip.is_empty() {
                    (final_sel, final_headers)
                } else {
                    // Filter selection to exclude stats-filtered columns
                    // stats_filter_cols_to_skip contains indices into selected_headers
                    let sel_vec: Vec<usize> = final_sel
                        .iter()
                        .copied()
                        .enumerate()
                        .filter(|(i, _)| !stats_filter_cols_to_skip.contains(i))
                        .map(|(_, idx)| idx)
                        .collect();

                    if sel_vec.is_empty() {
                        return Err(crate::CliError::Other(
                            "No columns remain after applying --stats-filter. Adjust your filter \
                             expression to be less restrictive."
                                .to_string(),
                        ));
                    }

                    // safety: We know Selection is a tuple struct with a Vec<usize> field
                    let new_sel = unsafe { std::mem::transmute::<Vec<usize>, Selection>(sel_vec) };
                    let headers: csv::ByteRecord = new_sel.select(&full_headers).collect();
                    (new_sel, headers)
                }
            } else {
                // Stats cache unavailable for --stats-filter
                log::warn!(
                    "Stats cache unavailable. Cannot apply --stats-filter. Processing all \
                     columns. Run 'qsv stats --cardinality --stats-jsonl' first."
                );
                (final_sel, final_headers)
            }
        } else {
            (final_sel, final_headers)
        };

        // Map original column indices to selected column indices
        let mapped_unique_headers: Vec<usize> = all_unique_headers_vec
            .iter()
            .filter_map(|&original_idx| {
                // Find the position of this original index in the selection
                final_sel
                    .iter()
                    .position(|&sel_idx| sel_idx == original_idx)
            })
            .collect();

        UNIQUE_COLUMNS_VEC
            .set(mapped_unique_headers)
            .map_err(|_| "Cannot set UNIQUE_COLUMNS")?;

        Ok((final_headers, final_sel, weight_col_idx))
    }
}

/// Helper function to add a field to field_stats if it exists
/// Automatically converts any type to appropriate JSON value
fn add_stat<T: ToString>(field_stats: &mut Vec<FieldStats>, name: &str, value: Option<T>) {
    if let Some(val) = value {
        let val_string = val.to_string();

        // Try to parse as integer first
        let json_value = if let Ok(int_val) = atoi_simd::parse::<i64>(val_string.as_bytes()) {
            JsonValue::Number(int_val.into())
        } else if let Ok(float_val) = fast_float2::parse(&val_string) {
            JsonValue::Number(
                serde_json::Number::from_f64(float_val)
                    .unwrap_or_else(|| serde_json::Number::from(0)),
            )
        } else {
            // Fall back to string
            JsonValue::String(val_string)
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

/// Evaluate a Luau expression against stats data for a column.
/// Returns `true` if the column should be EXCLUDED from frequency analysis.
/// The Lua VM is passed in to allow reuse across multiple column evaluations.
#[cfg(feature = "luau")]
fn evaluate_stats_filter(
    lua: &mlua::Lua,
    stats_data: &StatsData,
    filter_expression: &str,
) -> Result<bool, String> {
    use mlua::Value;

    // Set all StatsData fields as globals
    let globals = lua.globals();

    // Helper macros to reduce boilerplate
    macro_rules! set_string {
        ($name:ident) => {
            globals
                .set(stringify!($name), stats_data.$name.as_str())
                .map_err(|e| format!("Failed to set {}: {e}", stringify!($name)))?;
        };
    }

    macro_rules! set_u64 {
        ($name:ident) => {
            globals
                .set(stringify!($name), stats_data.$name)
                .map_err(|e| format!("Failed to set {}: {e}", stringify!($name)))?;
        };
    }

    macro_rules! set_bool {
        ($name:ident) => {
            globals
                .set(stringify!($name), stats_data.$name)
                .map_err(|e| format!("Failed to set {}: {e}", stringify!($name)))?;
        };
    }

    macro_rules! set_optional_f64 {
        ($name:ident) => {
            if let Some(val) = stats_data.$name {
                globals
                    .set(stringify!($name), val)
                    .map_err(|e| format!("Failed to set {}: {e}", stringify!($name)))?;
            } else {
                globals
                    .set(stringify!($name), Value::Nil)
                    .map_err(|e| format!("Failed to set {}: {e}", stringify!($name)))?;
            }
        };
    }

    macro_rules! set_optional_u64 {
        ($name:ident) => {
            if let Some(val) = stats_data.$name {
                globals
                    .set(stringify!($name), val)
                    .map_err(|e| format!("Failed to set {}: {e}", stringify!($name)))?;
            } else {
                globals
                    .set(stringify!($name), Value::Nil)
                    .map_err(|e| format!("Failed to set {}: {e}", stringify!($name)))?;
            }
        };
    }

    macro_rules! set_optional_usize {
        ($name:ident) => {
            if let Some(val) = stats_data.$name {
                globals
                    .set(stringify!($name), val)
                    .map_err(|e| format!("Failed to set {}: {e}", stringify!($name)))?;
            } else {
                globals
                    .set(stringify!($name), Value::Nil)
                    .map_err(|e| format!("Failed to set {}: {e}", stringify!($name)))?;
            }
        };
    }

    macro_rules! set_optional_u32 {
        ($name:ident) => {
            if let Some(val) = stats_data.$name {
                globals
                    .set(stringify!($name), val)
                    .map_err(|e| format!("Failed to set {}: {e}", stringify!($name)))?;
            } else {
                globals
                    .set(stringify!($name), Value::Nil)
                    .map_err(|e| format!("Failed to set {}: {e}", stringify!($name)))?;
            }
        };
    }

    macro_rules! set_optional_string {
        ($name:ident) => {
            if let Some(ref val) = stats_data.$name {
                globals
                    .set(stringify!($name), val.as_str())
                    .map_err(|e| format!("Failed to set {}: {e}", stringify!($name)))?;
            } else {
                globals
                    .set(stringify!($name), Value::Nil)
                    .map_err(|e| format!("Failed to set {}: {e}", stringify!($name)))?;
            }
        };
    }

    // Set all fields from StatsData
    // Basic fields
    set_string!(field);
    // 'type' is a reserved keyword in Rust, so we use r#type
    globals
        .set("type", stats_data.r#type.as_str())
        .map_err(|e| format!("Failed to set type: {e}"))?;
    set_bool!(is_ascii);

    // Counts (non-optional)
    set_u64!(cardinality);
    set_u64!(nullcount);

    // Optional numeric fields
    set_optional_f64!(sum);
    set_optional_string!(min);
    set_optional_string!(max);
    set_optional_f64!(range);
    set_optional_string!(sort_order);

    // String length stats
    set_optional_usize!(min_length);
    set_optional_usize!(max_length);
    set_optional_usize!(sum_length);
    set_optional_f64!(avg_length);
    set_optional_f64!(stddev_length);
    set_optional_f64!(variance_length);
    set_optional_f64!(cv_length);

    // Numeric stats
    set_optional_f64!(mean);
    set_optional_f64!(sem);
    set_optional_f64!(stddev);
    set_optional_f64!(variance);
    set_optional_f64!(cv);

    // Sign counts
    set_optional_u64!(n_negative);
    set_optional_u64!(n_zero);
    set_optional_u64!(n_positive);

    // Precision
    set_optional_u32!(max_precision);

    // Ratios
    set_optional_f64!(sparsity);
    set_optional_f64!(uniqueness_ratio);

    // Distribution stats
    set_optional_f64!(mad);
    set_optional_f64!(lower_outer_fence);
    set_optional_f64!(lower_inner_fence);
    set_optional_f64!(q1);
    set_optional_f64!(q2_median);
    set_optional_f64!(q3);
    set_optional_f64!(iqr);
    set_optional_f64!(upper_inner_fence);
    set_optional_f64!(upper_outer_fence);
    set_optional_f64!(skewness);

    // Mode/Antimode
    set_optional_string!(mode);
    set_optional_u64!(mode_count);
    set_optional_u64!(mode_occurrences);
    set_optional_string!(antimode);
    set_optional_u64!(antimode_count);
    set_optional_u64!(antimode_occurrences);

    // Wrap the expression in a return statement to get the result
    let wrapped_expr = format!("return {filter_expression}");

    // Evaluate the expression
    let result: Value = lua
        .load(&wrapped_expr)
        .eval()
        .map_err(|e| format!("Failed to evaluate filter expression: {e}"))?;

    // Convert result to boolean
    match result {
        Value::Boolean(b) => Ok(b),
        Value::Nil => Ok(false), // nil is treated as false (don't exclude)
        _ => Err(format!(
            "Filter expression must return a boolean, got: {result:?}"
        )),
    }
}
