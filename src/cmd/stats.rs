static USAGE: &str = r#"
Compute summary statistics & infers data types for each column in a CSV.

IMPORTANT: `stats` is heavily optimized for speed. It ASSUMES the CSV is well-formed & UTF-8 encoded.
This allows it to employ numerous performance optimizations (skip repetitive UTF-8 validation, skip
bounds checks, cache results, etc.) that may result in undefined behavior if the CSV is not well-formed.
All these optimizations are GUARANTEED to work with well-formed CSVs.
If you encounter problems generating stats, use `qsv validate` FIRST to confirm the CSV is valid.

NOTE: For MAXIMUM PERFORMANCE, create an index for the CSV first with 'qsv index' to enable multithreading,
or set --cache-threshold option or set the QSV_AUTOINDEX_SIZE environment variable to automatically
create an index when the file size is greater than the specified size (in bytes).

Summary stats include sum, min/max/range, sort order/sortiness, min/max/sum/avg/stddev/variance/cv length,
mean, standard error of the mean (SEM), geometric mean, harmonic mean, stddev, variance, coefficient of
variation (CV), nullcount, n_negative, n_zero, n_positive, max_precision, sparsity,
Median Absolute Deviation (MAD), quartiles, lower/upper inner/outer fences, skewness, median,
cardinality/uniqueness ratio, mode/s & "antimode/s" & percentiles.

Note that some stats require loading the entire file into memory, so they must be enabled explicitly.

By default, the following "streaming" statistics are reported for *every* column:
  sum, min/max/range values, sort order/"sortiness", min/max/sum/avg/stddev/variance/cv length, mean, sem,
  geometric_mean, harmonic_mean,stddev, variance, cv, nullcount, n_negative, n_zero, n_positive,
  max_precision & sparsity.

The default set of statistics corresponds to ones that can be computed efficiently on a stream of data
(i.e., constant memory) and works with arbitrarily large CSVs.

The following additional "non-streaming, advanced" statistics require loading the entire file into memory:
cardinality/uniqueness ratio, modes/antimodes, median, MAD, quartiles and its related measures
(q1, q2, q3, IQR, lower/upper fences & skewness) and percentiles.

When computing "non-streaming" statistics, a memory-aware chunking algorithm is used to dynamically
calculate chunk size based on available memory & record sampling. This SHOULD help process arbitrarily
large "real-world" files by creating smaller chunks that fit in available memory.
However, there is still a chance that the command will run out of memory if the cardinality of
several columns is very high.

Chunk size is dynamically calculated based on the number of logical CPUs detected.
You can override this behavior by setting the QSV_STATS_CHUNK_MEMORY_MB environment variable
(set to 0 for dynamic sizing, or a positive number for a fixed memory limit per chunk,
or -1 for CPU-based chunking (1 chunk = records/number of CPUs)).

"Antimode" is the least frequently occurring non-zero value and is the opposite of mode.
It returns "*ALL" if all the values are unique, and only returns a preview of the first
10 antimodes, truncating after 100 characters (configurable with QSV_ANTIMODES_LEN).

If you need all the antimode values of a column, run the `frequency` command with --limit set
to zero. The resulting frequency table will have all the "antimode" values.

Summary statistics for dates are also computed when --infer-dates is enabled, with DateTime
results in rfc3339 format and Date results in "yyyy-mm-dd" format in the UTC timezone.
Date range, stddev, variance, MAD & IQR are returned in days, not timestamp milliseconds.

Each column's data type is also inferred (NULL, Integer, String, Float, Date, DateTime and
Boolean with --infer-boolean option).
For String data types, it also determines if the column is all ASCII characters.
Unlike the sniff command, stats' data type inferences are GUARANTEED, as the entire file
is scanned, and not just sampled.

Note that the Date and DateTime data types are only inferred with the --infer-dates option
as its an expensive operation to match a date candidate against 19 possible date formats,
with each format, having several variants.

The date formats recognized and its sub-variants along with examples can be found at
https://github.com/dathere/qsv-dateparser?tab=readme-ov-file#accepted-date-formats.

Computing statistics on a large file can be made MUCH faster if you create an index for it
first with 'qsv index' to enable multithreading. With an index, the file is split into chunks
and each chunk is processed in parallel.

As stats is a central command in qsv, and can be expensive to compute, `stats` caches results
in <FILESTEM>.stats.csv & if the --stats-jsonl option is used, <FILESTEM>.stats.csv.data.jsonl
(e.g., qsv stats nyc311.csv will create nyc311.stats.csv; adding --stats-jsonl also creates
nyc311.stats.csv.data.jsonl).
The arguments used to generate the cached stats are saved in <FILESTEM>.stats.csv.json.

If stats have already been computed for the input file with similar arguments and the file
hasn't changed, the stats will be loaded from the cache instead of recomputing it.

These cached stats are also used by other qsv commands (currently `describegpt`, `frequency`,
`joinp`, `pivotp`, `schema`, `sqlp` & `tojsonl`) to work smarter & faster.
If the cached stats are not current (i.e., the input file is newer than the cached stats),
the cached stats will be ignored and recomputed.

Examples:

  # Compute "streaming" statistics for "nyc311.csv"
  qsv stats nyc311.csv

  # Compute all statistics for "nyc311.csv"
  qsv stats --everything nyc311.csv

  # Compute all statistics for "nyc311.tsv" (Tab-separated)
  qsv stats -E nyc311.tsv

  # Compute all stats for "nyc311.tsv", inferring dates using sniff to auto-detect date columns
  qsv stats -E --infer-dates nyc311.tsv

  # Compute all stats for "nyc311.tab", inferring dates only for columns
  #  with "_date" & "_dte" in the column names
  qsv stats -E --infer-dates --dates-whitelist _date,_dte nyc311.tab

  # Compute all stats, infer dates and boolean data types for "nyc311.ssv" file
  qsv stats -E --infer-dates --infer-boolean nyc311.ssv

  # In addition to basic "streaming" stats, also compute cardinality for "nyc311.csv"
  qsv stats --cardinality nyc311.csv

  # Prefer DMY format when inferring dates for the "nyc311.csv"
  qsv stats -E --infer-dates --prefer-dmy nyc311.csv

  # Infer data types only for the "nyc311.csv" file:
  qsv stats --typesonly nyc311.csv

  # Infer data types only, including boolean and date types for "nyc311.csv"
  $ qsv stats --typesonly --infer-boolean --infer-dates nyc311.csv

  # Automatically create an index for the "nyc311.csv" file to enable multithreading
  # if it's larger than 5MB and there is no existing index file:
  qsv stats -E --cache-threshold -5000000 nyc311.csv

  # Auto-create a TEMPORARY index for the "nyc311.csv" file to enable multithreading
  # if it's larger than 5MB and delete the index and the stats cache file after the stats run:
  qsv stats -E --cache-threshold -5000005 nyc311.csv

For more examples, see https://github.com/dathere/qsv/tree/master/resources/test
See also https://github.com/dathere/qsv/wiki/Aggregation-and-Statistics#stats

If the polars feature is enabled, support additional tabular file formats and
compression formats:
  $ qsv stats data.parquet // Parquet
  $ qsv stats data.avro // Avro
  $ qsv stats data.jsonl // JSON Lines
  $ qsv stats data.json (will only work with a JSON Array)
  $ qsv stats data.csv.gz // Gzipped CSV
  $ qsv stats data.tab.zlib // Zlib-compressed Tab-separated
  $ qsv stats data.ssv.zst // Zstd-compressed Semicolon-separated

For more info, see https://github.com/dathere/qsv/blob/master/docs/STATS_DEFINITIONS.md

Usage:
    qsv stats [options] [<input>]
    qsv stats --help

stats options:
    -s, --select <arg>        Select a subset of columns to compute stats for.
                              See 'qsv select --help' for the format details.
                              This is provided here because piping 'qsv select'
                              into 'qsv stats' will prevent the use of indexing.
    -E, --everything          Compute all statistics available.
    --typesonly               Infer data types only and do not compute statistics.
                              Note that if you want to infer dates and boolean types, you'll
                              still need to use the --infer-dates & --infer-boolean options.

                              BOOLEAN INFERENCING:
    --infer-boolean           Infer boolean data type. This automatically enables
                              the --cardinality option. When a column's cardinality is 2,
                              and the 2 values' are in the true/false patterns specified
                              by --boolean-patterns, the data type is inferred as boolean.
    --boolean-patterns <arg>  Comma-separated list of boolean pattern pairs in the format
                              "true_pattern:false_pattern". Each pattern can be a string
                              of any length. The patterns are case-insensitive. If a pattern
                              ends with a "*", it is treated as a prefix. For example,
                              "t*:f*,y*:n*" will match "true", "truthy", "Truth" as boolean true
                              values so long as the corresponding false pattern (e.g. False, f, etc.)
                              is also matched & cardinality is 2. Ignored if --infer-boolean is false.
                              [default: 1:0,t*:f*,y*:n*]

    --mode                    Compute the mode/s & antimode/s. Multimodal-aware.
                              If there are multiple modes/antimodes, they are separated by the
                              QSV_STATS_SEPARATOR environment variable. If not set, the default
                              separator is "|".
                              Uses memory proportional to the cardinality of each column.
    --cardinality             Compute the cardinality and the uniqueness ratio.
                              This is automatically enabled if --infer-boolean is enabled.
                              https://en.wikipedia.org/wiki/Cardinality_(SQL_statements)
                              Uses memory proportional to the number of unique values in each column.
    --zero-padded-numeric     Add a "zero_padded_numeric" column that is "true" when a column's
                              leading/padding zeros would be lost if it were cast to a number.
                              Qualifies when the inferred type is String AND every non-null value is
                              numeric (an all-digit integer or a zero-padded decimal code), with at
                              least one carrying a leading zero — e.g. US zip codes, barcodes,
                              zero-padded IDs, and zero-padded decimal codes like 007.1, 05.10 (as in
                              ICD-9, Dewey Decimal & Harmonized System classification codes).
                              qsv keeps all of these as text to preserve their leading zeros, so they
                              are not mistakenly loaded/cast as integer/float. Note that ordinary
                              fractions (0.5) and pure trailing-zero codes (7.10) are not flagged.
                              The cell is empty when the column is not zero-padded numeric.
                              Automatically enabled with --everything.

                              NUMERIC & DATE/DATETIME STATS THAT REQUIRE IN-MEMORY SORTING:
                              The following statistics are only computed for numeric & date/datetime
                              columns & require loading & sorting ALL the selected columns' data
                              in memory FIRST before computing the statistics.

    --median                  Compute the median.
                              Loads & sorts all the selected columns' data in memory.
                              https://en.wikipedia.org/wiki/Median
    --mad                     Compute the median absolute deviation (MAD).
                              https://en.wikipedia.org/wiki/Median_absolute_deviation
    --quartiles               Compute the quartiles (using method 3), the IQR, the lower/upper,
                              inner/outer fences and skewness.
                              https://en.wikipedia.org/wiki/Quartile#Method_3
    --percentiles             Compute custom percentiles using the nearest rank method.
                              https://en.wikipedia.org/wiki/Percentile#The_nearest-rank_method
    --percentile-list <arg>   Comma-separated list of percentiles to compute.
                              For example, "5,10,40,60,90,95" will compute percentiles
                              5th, 10th, 40th, 60th, 90th, and 95th.
                              Multiple percentiles are separated by the QSV_STATS_SEPARATOR
                              environment variable. If not set, the default separator is "|".
                              It is ignored if --percentiles is not set.
                              Special values "deciles" and "quintiles" are automatically expanded
                              to "10,20,30,40,50,60,70,80,90" and "20,40,60,80" respectively.
                              [default: 5,10,40,60,90,95]
    --quantile-method <m>     Algorithm used to compute the median, quartiles and custom
                              percentiles. Choices:
                                exact  - load all values into memory and sort (current behavior).
                                         O(N) memory per numeric column, exact deterministic
                                         results.
                                approx - use t-digest (Apache DataSketches port, based on
                                         Dunning's MergingDigest). O(K) memory per numeric column
                                         (K~200 centroids), O(1) quantile reads. Approximate
                                         (~1% rank error, more accurate at the tails).
                                         Restrictions:
                                           * --mad is disabled with a warning under approx.
                                           * --weight is rejected; the upstream datasketches
                                             crate does not expose weighted-update.
                                           * Results may differ slightly across runs with
                                             different --jobs values.
                                           * Requires a little-endian target. Apache
                                             DataSketches does not support big-endian
                                             platforms (e.g., s390x); on those builds,
                                             this choice is rejected.
                              [default: exact]
    --cardinality-method <m>  Algorithm used to compute the --cardinality column. Choices:
                                exact  - track every unique value in a HashMap/Unsorted
                                         (current behavior). O(cardinality) memory per
                                         column. Subject to --mode-cardinality-cap, which
                                         emits the ">=<n>" sentinel on overflow.
                                approx - use HyperLogLog (Apache DataSketches port,
                                         lg_k=12). O(1) memory per column (~5KB),
                                         ~1.5% relative standard error.
                                         Notes:
                                           * --mode-cardinality-cap no longer affects the
                                             cardinality column under approx; the ">=<n>"
                                             sentinel is never emitted.
                                           * The cap STILL governs mode/antimode tracking
                                             (mode columns still emit "*HIGH_CARDINALITY"
                                             on overflow).
                                           * --infer-boolean forces exact (boolean
                                             inference needs cardinality == 2 exactness);
                                             a one-time warning is emitted.
                                           * Reproducible across --jobs values: the
                                             HLL union used at merge time is associative
                                             and order-invariant, so chunk completion
                                             order does not affect the final estimate.
                                           * Requires a little-endian target. Apache
                                             DataSketches does not support big-endian
                                             platforms (e.g., s390x); on those builds,
                                             this choice is rejected.
                              [default: exact]
    --mode-cardinality-cap <n>  Bound mode-tracking memory on high-cardinality columns.
                              When > 0, if a column's mode tracker grows past <n> UNIQUE
                              values (true cardinality, for both unweighted and weighted
                              runs), qsv drops it and emits sentinel values instead of
                              exact modes and cardinality. The cap is a direct memory
                              bound on the tracker, which stores one entry per unique
                              value.
                              Sentinel output:
                                * mode columns: "*HIGH_CARDINALITY"
                                * cardinality column: ">=<n>" (the ">=" prefix DOES break
                                  downstream parsers expecting a plain integer; cap is
                                  opt-in only).
                              Under --cardinality-method approx, the cardinality column
                              ignores this cap (HLL gives an approximate estimate at
                              fixed memory, ~1.5% RSE) — only mode/antimode columns
                              are gated.
                              Useful on wide tables with many ID/UUID/timestamp columns
                              where tracking exact cardinality is wasted work.
                              [default: 0]

    --round <decimal_places>  Round statistics to <decimal_places>. Rounding is done following
                              Midpoint Nearest Even (aka "Bankers Rounding") rule.
                              https://docs.rs/rust_decimal/latest/rust_decimal/enum.RoundingStrategy.html
                              If set to the sentinel value 9999, no rounding is done.
                              For dates - range, stddev & IQR are rounded to 1e-5 day precision
                              (sub-second), with trailing zeros trimmed in the displayed output.
                              [default: 4]
    --nulls                   Include NULLs in the population size for computing
                              mean and standard deviation.
    --weight <column>         Compute weighted statistics using the specified column as weights.
                              The weight column must be numeric. When specified, all statistics
                              (mean, stddev, variance, median, quartiles, mode, etc.) will be
                              computed using weighted algorithms. The weight column is automatically
                              excluded from statistics computation. Missing or non-numeric weights
                              default to 1.0. Zero and negative weights are ignored and do not
                              contribute to the statistics. The output filename will be
                              <FILESTEM>.stats.weighted.csv to distinguish from unweighted statistics.

                              DATE INFERENCING:
    --infer-dates             Infer date/datetime data types. This is an expensive
                              option and should only be used when you know there
                              are date/datetime fields.
                              Also, if timezone is not specified in the data, it'll
                              be set to UTC.
    --dates-whitelist <list>  The comma-separated, case-insensitive patterns to look for when
                              shortlisting fields for date inferencing.
                              i.e. if the field's name has any of these patterns,
                              it is shortlisted for date inferencing.

                              Special values:
                              * "all" - inspect ALL fields for date/datetime types
                              * "sniff" - use `qsv sniff` to auto-detect date/datetime columns

                              Note that false positive date matches WILL most likely occur
                              when using "all" as unix epoch timestamps are just numbers.
                              Be sure to only use "all" if you know ALL the columns you're
                              inspecting are dates, boolean or string fields.

                              To avoid false positives, preprocess the file first
                              with the `datefmt` command to convert unix epoch timestamp
                              columns to RFC3339 format.

                              When set to "sniff", we do two-stage date inferencing.
                              First running sniff on the input file and then second,
                              only inferring dates for the columns that sniff identifies
                              as date/datetime candidates.
                              This is much faster than "all", and more convenient than
                              manually specifying patterns in the whitelist.
                              [default: sniff]
    --prefer-dmy              Parse dates in dmy format. Otherwise, use mdy format.
                              Ignored if --infer-dates is false.

    --force                   Force recomputing stats even if valid precomputed stats
                              cache exists.
    -j, --jobs <arg>          The number of jobs to run in parallel.
                              This works only when the given CSV has an index.
                              Note that a file handle is opened for each job.
                              When not set, the number of jobs is set to the
                              number of CPUs detected.
    --stats-jsonl             Also write the stats in JSONL format.
                              If set, the stats will be written to <FILESTEM>.stats.csv.data.jsonl.
                              Note that this option used internally by other qsv "smart" commands (see
                              https://github.com/dathere/qsv/blob/master/docs/PERFORMANCE.md#stats-cache)
                              to load cached stats to make them work smarter & faster.
                              You can preemptively create the stats-jsonl file by using this option
                              BEFORE running "smart" commands and they will automatically use it.
 -c, --cache-threshold <arg>  Controls the creation of stats cache files.
                                * when greater than 1, the threshold in milliseconds before caching
                                  stats results. If a stats run takes longer than this threshold,
                                  the stats results will be cached.
                                * 0 to suppress caching.
                                * 1 to force caching.
                                * a negative number to automatically create an index when
                                  the input file size is greater than abs(arg) in bytes.
                                  If the negative number ends with 5, it will delete the index
                                  file and the stats cache file after the stats run. Otherwise,
                                  the index file and the cache files are kept.
                              [default: 5000]
    --vis-whitespace          Visualize whitespace characters in the output.
                              See https://github.com/dathere/qsv/wiki/Aggregation-and-Statistics#whitespace-markers
                              for the list of whitespace markers.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -n, --no-headers       When set, the first row will NOT be interpreted
                           as column names. i.e., They will be included
                           in statistics.
    -d, --delimiter <arg>  The field delimiter for READING CSV data.
                           Must be a single character. (default: ,)
    --memcheck             Use CONSERVATIVE heuristics for the in-memory load
                           check (file size vs. available + free_swap × platform
                           factor − headroom), instead of the default NORMAL
                           check (file size vs. total memory − headroom). The
                           CONSERVATIVE check is stricter and trips OOM far
                           more readily. Ignored when computing default,
                           streaming statistics. (See also: QSV_MEMORY_CHECK
                           env var, equivalent to passing --memcheck.)
                           Independently of this flag, the in-memory load
                           check runs whenever stats takes the non-parallel
                           path with non-streaming columns. On OOM (in either
                           NORMAL or CONSERVATIVE mode), qsv auto-creates an
                           index when no index exists (skipped for stdin) AND
                           switches to approx quantile + approx cardinality
                           methods (DataSketches t-digest and HyperLogLog)
                           where compatible. The sketch fallback can also
                           fire when an index is already present and the OOM
                           still trips (e.g., when jobs is pinned to 1 on a
                           pre-indexed file). A wwarn is emitted listing the
                           auto-enabled estimators.
"#;

/*
DEVELOPER NOTE: stats is heavily optimized and is a central command in qsv.

It was the primary reason I created the qsv fork as I needed to do GUARANTEED data type
inferencing & to compile smart Data Dictionaries in the most performant way possible
for Datapusher+ (https://github.com/dathere/datapusher-plus).

It underpins the `schema` command, which derives a JSON Schema from a CSV's summary
statistics. That generated schema is then consumed by the `validate` command to
validate complex CSVs at hundreds of thousands of records/sec.

Its type inferences are also used by the "smart" commands (see
https://github.com/dathere/qsv/blob/master/docs/PERFORMANCE.md#stats-cache)
to make them work smarter & faster.

To safeguard against undefined behavior, `stats` is the most extensively tested command,
with ~680 tests. It also employs numerous performance optimizations (skip repetitive UTF-8
validation, skip bounds checks, cache results, etc.) that may result in undefined behavior
if the CSV is not well-formed. See "safety:" comments in the code for more details.
*/

use core::hint::cold_path;
use std::{
    fmt, fs,
    io::{self, BufRead, Seek, Write},
    iter::repeat_n,
    path::{Path, PathBuf},
    str,
    sync::{Arc, OnceLock},
};

use blake3;
use crossbeam_channel;
use foldhash::{HashMap, HashMapExt};
use itertools::Itertools;
use phf::phf_map;
use qsv_dateparser::parse_with_preference;
use rayon::{
    iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator},
    slice::ParallelSliceMut,
};
use serde::{Deserialize, Serialize};
// Use serde_json on big-endian platforms (e.g. s390x) due to simd_json endianness issues
#[cfg(target_endian = "little")]
use simd_json::{OwnedValue, prelude::ValueAsScalar, prelude::ValueObjectAccess};
use smallvec::SmallVec;
use stats::{Commute, Frequencies, MinMax, OnlineStats, Unsorted};
use tempfile::Builder as TempFileBuilder;
use threadpool::ThreadPool;

use self::FieldType::{TDate, TDateTime, TFloat, TInteger, TNull, TString};
use crate::{
    CliError, CliResult,
    config::{Config, Delimiter, get_delim_by_extension},
    select::{SelectColumns, Selection},
    util,
};

#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Clone, Deserialize)]
pub struct Args {
    pub arg_input:                 Option<String>,
    pub flag_select:               SelectColumns,
    pub flag_everything:           bool,
    pub flag_typesonly:            bool,
    pub flag_infer_boolean:        bool,
    pub flag_boolean_patterns:     String,
    pub flag_mode:                 bool,
    pub flag_cardinality:          bool,
    pub flag_zero_padded_numeric:  bool,
    pub flag_median:               bool,
    pub flag_mad:                  bool,
    pub flag_quartiles:            bool,
    pub flag_percentiles:          bool,
    pub flag_percentile_list:      String,
    pub flag_quantile_method:      String,
    pub flag_cardinality_method:   String,
    pub flag_mode_cardinality_cap: u64,
    pub flag_round:                u32,
    pub flag_nulls:                bool,
    pub flag_infer_dates:          bool,
    pub flag_dates_whitelist:      String,
    pub flag_prefer_dmy:           bool,
    pub flag_force:                bool,
    pub flag_jobs:                 Option<usize>,
    pub flag_stats_jsonl:          bool,
    pub flag_cache_threshold:      isize,
    pub flag_output:               Option<String>,
    pub flag_no_headers:           bool,
    pub flag_delimiter:            Option<Delimiter>,
    pub flag_memcheck:             bool,
    pub flag_vis_whitespace:       bool,
    pub flag_weight:               Option<String>,
}

// this struct is used to serialize/deserialize the stats to
// the "".stats.csv.json" file which we check to see
// if we can skip recomputing stats.
#[derive(Clone, Serialize, Deserialize, PartialEq, Default)]
struct StatsArgs {
    arg_input: String,
    flag_select: String,
    flag_everything: bool,
    flag_typesonly: bool,
    flag_infer_boolean: bool,
    flag_mode: bool,
    flag_cardinality: bool,
    #[serde(default)]
    flag_zero_padded_numeric: bool,
    flag_median: bool,
    flag_mad: bool,
    flag_quartiles: bool,
    flag_percentiles: bool,
    flag_percentile_list: String,
    flag_quantile_method: String,
    flag_cardinality_method: String,
    flag_mode_cardinality_cap: u64,
    flag_round: u32,
    flag_nulls: bool,
    flag_infer_dates: bool,
    flag_dates_whitelist: String,
    // the original, unresolved --dates-whitelist value (e.g. "sniff") that produced
    // flag_dates_whitelist. Lets a future "sniff" run on an unchanged file reuse the
    // sniff-resolved whitelist from this cache instead of re-sniffing. Excluded from the
    // cache-validity comparison (zeroed before comparing).
    #[serde(default)]
    flag_dates_whitelist_raw: String,
    flag_prefer_dmy: bool,
    flag_no_headers: bool,
    flag_delimiter: String,
    flag_output_snappy: bool,
    canonical_input_path: String,
    canonical_stats_path: String,
    record_count: u64,
    date_generated: String,
    compute_duration_ms: u64,
    qsv_version: String,
    flag_weight: String,
    flag_boolean_patterns: String,
    flag_vis_whitespace: bool,
    field_count: u64,
    filesize_bytes: u64,
    hash: FileHash,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Default)]
struct FileHash {
    #[serde(rename = "BLAKE3", skip_serializing_if = "String::is_empty")]
    blake3: String,
}

#[cfg(target_endian = "little")]
impl StatsArgs {
    // this is for deserializing the stats.csv.jsonl file
    // we use .get() instead of [] indexing to avoid panics on missing keys
    // (e.g. when reading older cache files that don't have newer fields like flag_weight)
    fn from_owned_value(value: &OwnedValue) -> Result<Self, Box<dyn std::error::Error>> {
        // helper closures for safe access - returns default if key is missing
        let get_str = |key: &str| -> String {
            value
                .get(key)
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string()
        };
        let get_str_or = |key: &str, default: &str| -> String {
            value
                .get(key)
                .and_then(|v| v.as_str())
                .unwrap_or(default)
                .to_string()
        };
        let get_bool = |key: &str| -> bool {
            value
                .get(key)
                .and_then(simd_json::prelude::ValueAsScalar::as_bool)
                .unwrap_or_default()
        };
        let get_u64 = |key: &str| -> u64 {
            value
                .get(key)
                .and_then(simd_json::prelude::ValueAsScalar::as_u64)
                .unwrap_or_default()
        };
        let get_hash = || -> FileHash {
            value
                .get("hash")
                .map(|h| FileHash {
                    blake3: h
                        .get("BLAKE3")
                        .and_then(|v| v.as_str())
                        .unwrap_or_default()
                        .to_string(),
                })
                .unwrap_or_default()
        };

        Ok(Self {
            arg_input: get_str("arg_input"),
            flag_select: get_str("flag_select"),
            flag_everything: get_bool("flag_everything"),
            flag_typesonly: get_bool("flag_typesonly"),
            flag_infer_boolean: get_bool("flag_infer_boolean"),
            flag_mode: get_bool("flag_mode"),
            flag_cardinality: get_bool("flag_cardinality"),
            flag_zero_padded_numeric: get_bool("flag_zero_padded_numeric"),
            flag_median: get_bool("flag_median"),
            flag_mad: get_bool("flag_mad"),
            flag_quartiles: get_bool("flag_quartiles"),
            flag_percentiles: get_bool("flag_percentiles"),
            flag_percentile_list: get_str_or("flag_percentile_list", "5,10,40,60,90,95"),
            flag_quantile_method: get_str_or("flag_quantile_method", "exact"),
            flag_cardinality_method: get_str_or("flag_cardinality_method", "exact"),
            flag_mode_cardinality_cap: get_u64("flag_mode_cardinality_cap"),
            flag_round: get_u64("flag_round") as u32,
            flag_nulls: get_bool("flag_nulls"),
            flag_infer_dates: get_bool("flag_infer_dates"),
            flag_dates_whitelist: get_str("flag_dates_whitelist"),
            flag_dates_whitelist_raw: get_str("flag_dates_whitelist_raw"),
            flag_prefer_dmy: get_bool("flag_prefer_dmy"),
            flag_no_headers: get_bool("flag_no_headers"),
            flag_delimiter: get_str("flag_delimiter"),
            flag_output_snappy: get_bool("flag_output_snappy"),
            canonical_input_path: get_str("canonical_input_path"),
            canonical_stats_path: get_str("canonical_stats_path"),
            record_count: get_u64("record_count"),
            date_generated: get_str("date_generated"),
            compute_duration_ms: get_u64("compute_duration_ms"),
            qsv_version: get_str("qsv_version"),
            flag_weight: get_str("flag_weight"),
            flag_boolean_patterns: get_str("flag_boolean_patterns"),
            flag_vis_whitespace: get_bool("flag_vis_whitespace"),
            field_count: get_u64("field_count"),
            filesize_bytes: get_u64("filesize_bytes"),
            hash: get_hash(),
        })
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Default, Debug)]
pub struct StatsData {
    pub field: String,
    // type is a reserved keyword in Rust
    // so we escape it as r#type
    // we need to do this for serde to work
    pub r#type: String,
    #[serde(default)]
    pub is_ascii: bool,
    pub sum: Option<f64>,
    pub min: Option<String>,
    pub max: Option<String>,
    pub range: Option<f64>,
    pub sort_order: Option<String>,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub sum_length: Option<usize>,
    pub avg_length: Option<f64>,
    pub stddev_length: Option<f64>,
    pub variance_length: Option<f64>,
    pub cv_length: Option<f64>,
    pub mean: Option<f64>,
    pub sem: Option<f64>,
    pub stddev: Option<f64>,
    pub variance: Option<f64>,
    pub cv: Option<f64>,
    pub nullcount: u64,
    pub n_negative: Option<u64>,
    pub n_zero: Option<u64>,
    pub n_positive: Option<u64>,
    pub max_precision: Option<u32>,
    pub sparsity: Option<f64>,
    pub mad: Option<f64>,
    pub lower_outer_fence: Option<f64>,
    pub lower_inner_fence: Option<f64>,
    pub q1: Option<f64>,
    pub q2_median: Option<f64>,
    pub q3: Option<f64>,
    pub iqr: Option<f64>,
    pub upper_inner_fence: Option<f64>,
    pub upper_outer_fence: Option<f64>,
    pub skewness: Option<f64>,
    pub cardinality: u64,
    pub uniqueness_ratio: Option<f64>,
    pub mode: Option<String>,
    pub mode_count: Option<u64>,
    pub mode_occurrences: Option<u64>,
    pub antimode: Option<String>,
    pub antimode_count: Option<u64>,
    pub antimode_occurrences: Option<u64>,
    // moarstats fields (populated when moarstats has been run)
    #[serde(default)]
    pub kurtosis: Option<f64>,
    #[serde(default)]
    pub bimodality_coefficient: Option<f64>,
    #[serde(default)]
    pub pearson_skewness: Option<f64>,
    #[serde(default)]
    pub gini_coefficient: Option<f64>,
    #[serde(default)]
    pub quartile_coefficient_dispersion: Option<f64>,
    #[serde(default)]
    pub outliers_percentage: Option<f64>,
    #[serde(default)]
    pub outlier_impact_ratio: Option<f64>,
    #[serde(default)]
    pub mad_stddev_ratio: Option<f64>,
    #[serde(default)]
    pub median_mean_ratio: Option<f64>,
    #[serde(default)]
    pub normalized_entropy: Option<f64>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum JsonTypes {
    Int,
    Float,
    Bool,
    String,
}

// we use this to serialize the StatsData data structure
// to a JSONL file using serde_json
pub static STATSDATA_TYPES_MAP: phf::Map<&'static str, JsonTypes> = phf_map! {
    "field" => JsonTypes::String,
    "type" => JsonTypes::String,
    "is_ascii" => JsonTypes::Bool,
    "sum" => JsonTypes::Float,
    "min" => JsonTypes::String,
    "max" => JsonTypes::String,
    "range" => JsonTypes::Float,
    "sort_order" => JsonTypes::String,
    "sortiness" => JsonTypes::Float,
    "min_length" => JsonTypes::Int,
    "max_length" => JsonTypes::Int,
    "sum_length" => JsonTypes::Int,
    "avg_length" => JsonTypes::Float,
    "stddev_length" => JsonTypes::Float,
    "variance_length" => JsonTypes::Float,
    "cv_length" => JsonTypes::Float,
    "mean" => JsonTypes::Float,
    "sem" => JsonTypes::Float,
    "geometric_mean" => JsonTypes::Float,
    "harmonic_mean" => JsonTypes::Float,
    "stddev" => JsonTypes::Float,
    "variance" => JsonTypes::Float,
    "cv" => JsonTypes::Float,
    "nullcount" => JsonTypes::Int,
    "n_negative" => JsonTypes::Int,
    "n_zero" => JsonTypes::Int,
    "n_positive" => JsonTypes::Int,
    "max_precision" => JsonTypes::Int,
    "sparsity" => JsonTypes::Float,
    "mad" => JsonTypes::Float,
    "lower_outer_fence" => JsonTypes::Float,
    "lower_inner_fence" => JsonTypes::Float,
    "q1" => JsonTypes::Float,
    "q2_median" => JsonTypes::Float,
    "q3" => JsonTypes::Float,
    "iqr" => JsonTypes::Float,
    "upper_inner_fence" => JsonTypes::Float,
    "upper_outer_fence" => JsonTypes::Float,
    "skewness" => JsonTypes::Float,
    "cardinality" => JsonTypes::Int,
    "uniqueness_ratio" => JsonTypes::Float,
    "mode" => JsonTypes::String,
    "mode_count" => JsonTypes::Int,
    "mode_occurrences" => JsonTypes::Int,
    "antimode" => JsonTypes::String,
    "antimode_count" => JsonTypes::Int,
    "antimode_occurrences" => JsonTypes::Int,
    "zero_padded_numeric" => JsonTypes::Bool,
    // moarstats fields
    "kurtosis" => JsonTypes::Float,
    "bimodality_coefficient" => JsonTypes::Float,
    "pearson_skewness" => JsonTypes::Float,
    "gini_coefficient" => JsonTypes::Float,
    "quartile_coefficient_dispersion" => JsonTypes::Float,
    "outliers_percentage" => JsonTypes::Float,
    "outlier_impact_ratio" => JsonTypes::Float,
    "mad_stddev_ratio" => JsonTypes::Float,
    "median_mean_ratio" => JsonTypes::Float,
    "normalized_entropy" => JsonTypes::Float,
};

static INFER_DATE_FLAGS: OnceLock<SmallVec<[bool; 50]>> = OnceLock::new();
static RECORD_COUNT: OnceLock<u64> = OnceLock::new();
static ANTIMODES_LEN: OnceLock<usize> = OnceLock::new();
static STATS_SEPARATOR: OnceLock<String> = OnceLock::new();
static STATS_STRING_MAX_LENGTH: OnceLock<Option<usize>> = OnceLock::new();

// standard overflow and underflow strings
// for sum, sum_length and avg_length
const OVERFLOW_STRING: &str = "*OVERFLOW*";
const UNDERFLOW_STRING: &str = "*UNDERFLOW*";

// number of milliseconds per day
const MS_IN_DAY: f64 = 86_400_000.0;
const MS_IN_DAY_INT: i64 = 86_400_000;
// number of decimal places when rounding days
// 5 decimal places give us sub-second precision
const DAY_DECIMAL_PLACES: u32 = 5;

// maximum number of output columns
const MAX_STAT_COLUMNS: usize = 48;

// HyperLogLog precision parameter for `--cardinality-method approx`. lg_k=12
// gives ~1.5% relative standard error and ~5KB per column at the dense Hll8
// representation. Used in `Stats::new` (per-row sketch construction) and in
// `Commute::merge` (the transient `HllUnion` used to combine two sketches);
// keep both call sites in lock-step by reading from this constant.
//
// Gated to little-endian: the only consumers are `datasketches::hll` call sites,
// which are themselves gated. Defining it unconditionally would trip an
// `unused-const` lint on big-endian builds.
#[cfg(not(target_endian = "big"))]
const HLL_LG_K: u8 = 12;

// the first N columns of each full stats record are used for the dataset
// fingerprint hash. For the normal (non-`--typesonly`) output, N must equal
// the number of "streaming" stats columns emitted by `stats_headers()`
// (currently: field, type, is_ascii, sum, min, max, range, sort_order,
// sortiness, min_length, max_length, sum_length, avg_length, stddev_length,
// variance_length, cv_length, mean, sem, geometric_mean, harmonic_mean,
// stddev, variance, cv, nullcount, n_negative, n_zero, n_positive,
// max_precision, sparsity). `--typesonly` is an exception and emits only
// `field` and `type`. When adding or removing a streaming column in the
// normal `stats_headers()` output, update this constant to match.
const FINGERPRINT_HASH_COLUMNS: usize = 29;

// maximum number of antimodes to display
const MAX_ANTIMODES: usize = 10;
// default length of antimode string before truncating and appending "..."
const DEFAULT_ANTIMODES_LEN: usize = 100;

// the default separator we use for stats that have multiple values
// in one column, i.e. antimodes/modes & percentiles
pub const DEFAULT_STATS_SEPARATOR: &str = "|";

// the threshold for when to use parallel sorting for modes/antimodes etc.
const PAR_SORT_THRESHOLD: usize = 10_000;

static BOOLEAN_PATTERNS: OnceLock<Vec<BooleanPattern>> = OnceLock::new();
#[derive(Clone, Debug)]
/// Represents a pattern for boolean value inference in CSV data.
///
/// This struct defines patterns that can be used to identify boolean values in CSV columns.
/// It supports both exact matches and prefix matching with wildcards for flexible boolean
/// detection during CSV statistics computation.
///
/// # Pattern Matching
///
/// Patterns support two types of matching:
/// * **Exact match**: The value must exactly match the pattern (case-insensitive)
/// * **Prefix match**: If the pattern ends with `*`, it matches any value that starts with the
///   prefix (e.g., `"yes*"` matches `"yes"`, `"yes please"`, `"YES"`, etc.)
///
/// See the field-level docs below for details on each field.
struct BooleanPattern {
    /// The pattern that identifies `true` values (case-insensitive). Stored
    /// already-lowercased; may end with `*` for prefix matching.
    true_pattern:  String,
    /// The pattern that identifies `false` values (case-insensitive). Stored
    /// already-lowercased; may end with `*` for prefix matching.
    false_pattern: String,
    /// True iff both patterns are ASCII-only. When set (and the value being
    /// matched is also ASCII), `matches` uses an allocation-free comparison.
    ascii_only:    bool,
}

impl BooleanPattern {
    /// Checks if a value matches the boolean pattern.
    ///
    /// This method determines whether a given string value matches either the true or false
    /// pattern defined in this `BooleanPattern`. The matching is case-insensitive and supports
    /// both exact matches and prefix matching with wildcards.
    ///
    /// # Arguments
    ///
    /// * `value` - The string value to check against the boolean patterns
    ///
    /// # Returns
    ///
    /// * `Some(true)` - If the value matches the true pattern
    /// * `Some(false)` - If the value matches the false pattern
    /// * `None` - If the value doesn't match either pattern
    ///
    /// # Matching Logic
    ///
    /// 1. **Exact match**: The value is compared directly to both patterns (case-insensitive)
    /// 2. **Prefix match**: If a pattern ends with `*`, the value is checked if it starts with the
    ///    prefix (excluding the `*` character)
    /// 3. **Priority**: Exact matches are checked before prefix matches. This is also a correctness
    ///    requirement when patterns overlap (e.g. `"t*"` true vs. `"t"` false: the exact false
    ///    match must win for a value of `"t"`).
    fn matches(&self, value: &str) -> Option<bool> {
        // Fast path: when both patterns and the value are ASCII, we can use
        // `eq_ignore_ascii_case` and a byte-wise prefix compare without
        // allocating a lowercased copy of `value`. Patterns are already
        // lowercased at parse time, so case folding only needs to happen on
        // `value`, and ASCII case folding is a single bit flip per byte.
        if self.ascii_only && value.is_ascii() {
            if value.eq_ignore_ascii_case(&self.true_pattern) {
                return Some(true);
            }
            if value.eq_ignore_ascii_case(&self.false_pattern) {
                return Some(false);
            }
            if let Some(prefix) = self.true_pattern.strip_suffix('*')
                && value.len() >= prefix.len()
                && value.as_bytes()[..prefix.len()].eq_ignore_ascii_case(prefix.as_bytes())
            {
                return Some(true);
            }
            if let Some(prefix) = self.false_pattern.strip_suffix('*')
                && value.len() >= prefix.len()
                && value.as_bytes()[..prefix.len()].eq_ignore_ascii_case(prefix.as_bytes())
            {
                return Some(false);
            }
            return None;
        }

        // Slow path: at least one of the patterns or the value contains
        // non-ASCII characters, so fall back to Unicode-aware lowercasing.
        let value_lower = value.to_lowercase();

        if value_lower == self.true_pattern {
            return Some(true);
        } else if value_lower == self.false_pattern {
            return Some(false);
        }

        if let Some(prefix) = self.true_pattern.strip_suffix('*')
            && value_lower.starts_with(prefix)
        {
            return Some(true);
        }

        if let Some(prefix) = self.false_pattern.strip_suffix('*')
            && value_lower.starts_with(prefix)
        {
            return Some(false);
        }

        None
    }
}

/// Parses a comma-separated string of boolean patterns into a vector of `BooleanPattern` structs.
///
/// This function takes a string containing boolean pattern pairs and converts them into
/// `BooleanPattern` objects that can be used for boolean value inference in CSV data.
///
/// # Arguments
///
/// * `boolean_patterns` - A comma-separated string of pattern pairs in the format `"true:false"`
///
/// # Format
///
/// The input string should contain pattern pairs separated by commas, where each pair
/// consists of a true pattern and false pattern separated by a colon:
/// `"true_pattern1:false_pattern1,true_pattern2:false_pattern2"`
///
/// # Returns
///
/// * `Ok(Vec<BooleanPattern>)` - Vector of parsed boolean patterns
/// * `Err(CliError)` - If the format is invalid or patterns are empty
///
/// # Errors
///
/// * Returns an error if any pattern pair is missing the colon separator
/// * Returns an error if either the true or false pattern is empty
/// * Returns an error if no patterns are provided
fn parse_boolean_patterns(boolean_patterns: &str) -> CliResult<Vec<BooleanPattern>> {
    let mut patterns = Vec::new();
    for pair in boolean_patterns.split(',') {
        let mut parts = pair.split(':');
        let true_pattern = parts.next().unwrap_or("").trim().to_lowercase();
        let false_pattern = parts.next().unwrap_or("").trim().to_lowercase();

        // Reject more than two colon-separated parts (e.g. "a:b:c") so trailing
        // tokens aren't silently dropped.
        if parts.next().is_some() {
            return fail_incorrectusage_clierror!(
                "Invalid boolean pattern (expected `true:false`): {pair}"
            );
        }

        if true_pattern.is_empty() || false_pattern.is_empty() {
            return fail_incorrectusage_clierror!("Invalid boolean pattern: {pair}");
        }

        // A bare "*" has an empty prefix, which would match every value.
        if true_pattern == "*" || false_pattern == "*" {
            return fail_incorrectusage_clierror!(
                "Invalid boolean pattern (`*` alone matches everything): {pair}"
            );
        }

        // Identical true/false patterns make the false branch unreachable
        // (true is checked first), so reject as ambiguous.
        if true_pattern == false_pattern {
            return fail_incorrectusage_clierror!(
                "Invalid boolean pattern (true and false patterns are identical): {pair}"
            );
        }

        let ascii_only = true_pattern.is_ascii() && false_pattern.is_ascii();
        patterns.push(BooleanPattern {
            true_pattern,
            false_pattern,
            ascii_only,
        });
    }
    if patterns.is_empty() {
        return fail_incorrectusage_clierror!("Boolean patterns must have at least one pattern");
    }
    Ok(patterns)
}

/// Auto-enable approx `DataSketches` estimators when an OOM hit forces a memory
/// budget cut. Returns the list of method names that were switched, in
/// user-facing form. Caller is responsible for emitting the `wwarn!` message
/// and for verifying that this was actually invoked from the OOM branch.
///
/// `user_set_quantile_method` / `user_set_cardinality_method` indicate whether
/// the user passed `--quantile-method` / `--cardinality-method` on the command
/// line (regardless of value). When `true`, the auto-enable is suppressed for
/// that method even if its current value is `"exact"` — this honors the
/// "Re-run with explicit `--quantile-method exact` to disable the auto-enable"
/// contract surfaced in the wwarn and --memcheck docs. Without this guard, the
/// docopt default of `"exact"` would make an explicit `exact` indistinguishable
/// from omitting the flag, and the opt-out advice in the wwarn would be a no-op.
///
/// Conflict guards mirror the explicit `--quantile-method` / `--cardinality-method`
/// validation that runs in `run()` (search for the `fail_incorrectusage_clierror!`
/// guards that reject `--quantile-method approx` + `--weight`, the `wwarn!` that
/// auto-disables `--mad` under approx, and the `--infer-boolean` + approx-cardinality
/// fallback that forces exact). The intent is that this auto-enable only flips
/// methods that would have passed validation if the user had set them by hand.
///
/// On big-endian targets the Apache `DataSketches` port is unavailable, so this
/// function compiles to a no-op stub that returns an empty `Vec`. The OOM
/// branch in `run()` then falls through to returning the original error rather
/// than auto-enabling an estimator that isn't compiled in.
#[cfg(not(target_endian = "big"))]
fn try_enable_approx_sketches(
    args: &mut Args,
    user_set_quantile_method: bool,
    user_set_cardinality_method: bool,
) -> Vec<&'static str> {
    let mut enabled = Vec::new();

    // t-digest: blocked by --weight (datasketches crate has no weighted-update API).
    // Mirrors the explicit `--quantile-method approx + --weight` rejection in run();
    // also mirrors the MAD auto-disable wwarn (t-digest can't do MAD's second pass).
    if args.flag_quantile_method == "exact"
        && !user_set_quantile_method
        && args.flag_weight.is_none()
    {
        args.flag_quantile_method = "approx".to_string();
        if args.flag_mad || args.flag_everything {
            args.flag_mad = false;
            enabled.push("--quantile-method approx (MAD disabled)");
        } else {
            enabled.push("--quantile-method approx");
        }
    }

    // HLL: blocked by --infer-boolean (boolean inference requires cardinality == 2
    // exactness; HLL's ~1.5% RSE would corrupt the comparison). Mirrors the explicit
    // `--infer-boolean forces --cardinality-method exact` fallback in run().
    if args.flag_cardinality_method == "exact"
        && !user_set_cardinality_method
        && !args.flag_infer_boolean
    {
        args.flag_cardinality_method = "approx".to_string();
        enabled.push("--cardinality-method approx");
    }

    enabled
}

/// Big-endian stub: Apache DataSketches is not available on big-endian targets,
/// so we cannot auto-enable any approximate estimator. The OOM branch in `run()`
/// receives an empty list and falls through to returning the original error.
#[cfg(target_endian = "big")]
fn try_enable_approx_sketches(
    _args: &mut Args,
    _user_set_quantile_method: bool,
    _user_set_cardinality_method: bool,
) -> Vec<&'static str> {
    Vec::new()
}

/// Main entry point for the stats command.
///
/// This function orchestrates the entire CSV statistics computation process, including
/// argument parsing, configuration setup, data processing, and output generation.
/// It handles both sequential and parallel processing approaches based on the dataset size
/// and available system resources.
///
/// # Arguments
///
/// * `argv` - Command line arguments as string slices
///
/// # Returns
///
/// * `Ok(())` - Successfully completed statistics computation
/// * `Err(CliError)` - If there's an error during processing
///
/// # Process Overview
///
/// 1. **Argument Parsing**: Parses command line arguments and validates configuration
/// 2. **Boolean Inference Setup**: Configures boolean pattern matching if enabled
/// 3. **Environment Variables**: Checks for `QSV_PREFER_DMY` environment variable
/// 4. **Output Configuration**: Determines output format and compression settings
/// 5. **Statistics Computation**: Processes CSV data using sequential or parallel approach
/// 6. **Cache Management**: Handles statistics caching and cache invalidation
/// 7. **Output Generation**: Writes results to stdout or specified output file
/// 8. **Cleanup**: Removes temporary files and handles cleanup operations
///
/// # Features
///
/// * **Type Inference**: Automatically detects data types (numeric, string, date, boolean)
/// * **Date Inference**: Configurable date pattern recognition
/// * **Boolean Inference**: Pattern-based boolean value detection
/// * **Parallel Processing**: Multi-threaded computation for large datasets
/// * **Caching**: Intelligent caching of computed statistics
/// * **Multiple Output Formats**: CSV, JSON, and compressed formats
/// * **Comprehensive Statistics**: Mean, median, quartiles, mode, cardinality, etc.
///
/// # Error Handling
///
/// * Validates input file existence and format
/// * Handles CSV parsing errors gracefully
/// * Manages temporary file creation and cleanup
/// * Provides detailed error messages for configuration issues
pub fn run(argv: &[&str]) -> CliResult<()> {
    let mut args: Args = util::get_args(USAGE, argv)?;

    // Detect whether the user explicitly passed --quantile-method /
    // --cardinality-method on the command line. docopt fills in the default
    // value ("exact") regardless, so without this scan we can't honor an
    // explicit `--quantile-method exact` opt-out during the OOM auto-fallback.
    let user_set_quantile_method = util::argv_has_flag(argv, "--quantile-method");
    let user_set_cardinality_method = util::argv_has_flag(argv, "--cardinality-method");

    if args.flag_typesonly {
        args.flag_everything = false;
        args.flag_mode = false;
        args.flag_cardinality = false;
        args.flag_median = false;
        args.flag_quartiles = false;
        args.flag_mad = false;
    }

    // percentile_list special values
    // deciles and quintiles are automatically expanded to their corresponding percentile lists
    // case-insensitive comparison is used to check for these special values
    if args.flag_percentile_list.to_lowercase() == "deciles" {
        args.flag_percentile_list = "10,20,30,40,50,60,70,80,90".to_string();
    } else if args.flag_percentile_list.to_lowercase() == "quintiles" {
        args.flag_percentile_list = "20,40,60,80".to_string();
    }

    // validate percentile list
    let percentile_list = args.flag_percentile_list.split(',').collect::<Vec<&str>>();
    for p in percentile_list {
        if fast_float2::parse::<f64, &[u8]>(p.trim().as_bytes()).is_err() {
            return fail_incorrectusage_clierror!(
                "Invalid percentile list: {}: {}",
                args.flag_percentile_list,
                p
            );
        }
    }

    // validate --quantile-method (default "exact"; canonicalize to lowercase).
    args.flag_quantile_method = args.flag_quantile_method.to_lowercase();
    let approx_quantiles = match args.flag_quantile_method.as_str() {
        "exact" => false,
        "approx" => true,
        other => {
            return fail_incorrectusage_clierror!(
                "Invalid --quantile-method: {other}. Choose 'exact' or 'approx'."
            );
        },
    };

    // Apache DataSketches (t-digest) is unavailable on big-endian targets, so
    // --quantile-method approx cannot be honored there. Reject up front so the
    // downstream code can assume `approx_quantiles == false` on big-endian.
    #[cfg(target_endian = "big")]
    if approx_quantiles {
        return fail_incorrectusage_clierror!(
            "--quantile-method approx requires a little-endian target. Apache DataSketches is not \
             available on big-endian platforms (e.g., s390x). Use --quantile-method exact."
        );
    }

    // approx quantiles are not yet supported with --weight: the upstream
    // datasketches::tdigest crate does not expose a weighted-update API.
    if approx_quantiles && args.flag_weight.is_some() {
        return fail_incorrectusage_clierror!(
            "--quantile-method approx does not yet support weighted statistics. Use \
             --quantile-method exact when --weight is set."
        );
    }

    // approx quantiles cannot compute MAD (median(|x - median|) needs a second pass over
    // the absolute deviations from the median, which a t-digest cannot provide). Disable
    // MAD with a one-time warning rather than emitting a wrong value.
    if approx_quantiles && (args.flag_everything || args.flag_mad) {
        wwarn!("--quantile-method approx does not support MAD; disabling MAD for this run.");
        args.flag_mad = false;
        // which_stats() also clears mad when approx is set, so flag_everything OR flag_mad
        // cannot re-enable it.
    }

    // validate --cardinality-method (default "exact"; canonicalize to lowercase).
    // The match is value-only; which_stats() reads flag_cardinality_method directly,
    // so no local boolean is needed here — we only need to reject invalid input
    // and mutate the field for canonical comparison downstream.
    args.flag_cardinality_method = args.flag_cardinality_method.to_lowercase();
    match args.flag_cardinality_method.as_str() {
        "exact" | "approx" => {},
        other => {
            return fail_incorrectusage_clierror!(
                "Invalid --cardinality-method: {other}. Choose 'exact' or 'approx'."
            );
        },
    }

    // Apache DataSketches (HyperLogLog) is unavailable on big-endian targets, so
    // --cardinality-method approx cannot be honored there. Reject up front so the
    // downstream code can assume `flag_cardinality_method != "approx"` on big-endian.
    #[cfg(target_endian = "big")]
    if args.flag_cardinality_method == "approx" {
        return fail_incorrectusage_clierror!(
            "--cardinality-method approx requires a little-endian target. Apache DataSketches is \
             not available on big-endian platforms (e.g., s390x). Use --cardinality-method exact."
        );
    }

    // inferring boolean requires inferring cardinality
    if args.flag_infer_boolean {
        if !args.flag_cardinality {
            args.flag_cardinality = true;
        }

        // boolean inference checks cardinality == 2 exactly; HLL's ~1.5% relative error
        // would corrupt that comparison. Force exact and warn.
        if args.flag_cardinality_method == "approx" {
            wwarn!(
                "--infer-boolean requires exact cardinality; forcing --cardinality-method exact \
                 for this run."
            );
            args.flag_cardinality_method = "exact".to_string();
        }

        // validate boolean patterns
        let patterns = parse_boolean_patterns(&args.flag_boolean_patterns)?;
        let _ = BOOLEAN_PATTERNS.set(patterns);
    }

    // check prefer_dmy env var
    args.flag_prefer_dmy = args.flag_prefer_dmy || util::get_envvar_flag("QSV_PREFER_DMY");

    // set stdout output flag
    let stdout_output_flag = args.flag_output.is_none();

    // save the current args, we'll use it to generate
    // the stats.csv.json file
    let mut current_stats_args = StatsArgs {
        arg_input: args.arg_input.clone().unwrap_or_default(),
        flag_select: format!("{:?}", args.flag_select),
        flag_everything: args.flag_everything,
        flag_typesonly: args.flag_typesonly,
        flag_infer_boolean: args.flag_infer_boolean,
        flag_mode: args.flag_mode,
        flag_cardinality: args.flag_cardinality,
        flag_zero_padded_numeric: args.flag_zero_padded_numeric,
        flag_median: args.flag_median,
        flag_mad: args.flag_mad,
        flag_quartiles: args.flag_quartiles,
        flag_percentiles: args.flag_percentiles,
        flag_percentile_list: args.flag_percentile_list.clone(),
        flag_quantile_method: args.flag_quantile_method.clone(),
        flag_cardinality_method: args.flag_cardinality_method.clone(),
        flag_mode_cardinality_cap: args.flag_mode_cardinality_cap,
        flag_round: args.flag_round,
        flag_nulls: args.flag_nulls,
        flag_infer_dates: args.flag_infer_dates,
        flag_dates_whitelist: args.flag_dates_whitelist.clone(),
        // populated just before the cache sidecar is written; kept empty here so it is
        // ignored by the cache-validity comparison below
        flag_dates_whitelist_raw: String::new(),
        flag_prefer_dmy: args.flag_prefer_dmy,
        flag_no_headers: args.flag_no_headers,
        flag_delimiter: args
            .flag_delimiter
            .as_ref()
            .map(|d| (d.as_byte() as char).to_string())
            .unwrap_or_default(),
        // when we write to stdout, we don't use snappy compression
        // when we write to a file with the --output option, we use
        // snappy compression if the file ends with ".sz"
        flag_output_snappy: if stdout_output_flag {
            false
        } else {
            let p = args.flag_output.clone().unwrap();
            p.to_ascii_lowercase().ends_with(".sz")
        },
        canonical_input_path: String::new(),
        canonical_stats_path: String::new(),
        record_count: 0,
        date_generated: String::new(),
        compute_duration_ms: 0,
        // save the qsv version in the stats.csv.json file
        // so cached stats are automatically invalidated
        // when the qsv version changes
        qsv_version: env!("CARGO_PKG_VERSION").to_string(),
        flag_weight: args.flag_weight.clone().unwrap_or_default(),
        flag_boolean_patterns: args.flag_boolean_patterns.clone(),
        flag_vis_whitespace: args.flag_vis_whitespace,
        field_count: 0,
        filesize_bytes: 0,
        hash: FileHash::default(),
    };

    // create a temporary file to store the <FILESTEM>.stats.csv file
    // The cache is always plain CSV (comma-delimited, uncompressed) regardless of
    // the --output format, since it's an internal format consumed by moarstats,
    // schema, frequency, etc. Use .csv suffix so NamedTempFile RAII cleanup
    // deletes the correct file.
    let stats_csv_tempfile = TempFileBuilder::new().suffix(".csv").tempfile()?;
    // safety: we know the tempfile is a valid NamedTempFile, so we can use unwrap
    let stats_csv_tempfile_fname = stats_csv_tempfile.path().to_str().unwrap().to_string();

    // find the delimiter to use based on the extension of the output file
    // and if we need to snappy compress the output
    let (_output_extension, output_delim, snappy) = match args.flag_output {
        Some(ref output_path) => get_delim_by_extension(Path::new(&output_path), b','),
        _ => (String::new(), b',', false),
    };

    // we will write the stats to a temp file - always as plain CSV
    let wconfig = Config::new(Some(&stats_csv_tempfile_fname)).delimiter(Some(Delimiter(b',')));
    let mut wtr = wconfig.writer()?;

    let mut rconfig = args.rconfig();
    if let Some(format_error) = rconfig.format_error {
        return fail_incorrectusage_clierror!("{format_error}");
    }

    // infer delimiter when we're getting input from stdin
    // as the stats engine needs to know the delimiter or it will panic
    let mut stdin_tempfile_path = None;
    if rconfig.is_stdin() {
        // read from stdin and write to a temp file
        log::info!("Reading from stdin");

        let temp_dir =
            crate::config::TEMP_FILE_DIR.get_or_init(|| tempfile::TempDir::new().unwrap().keep());

        let mut stdin_file = TempFileBuilder::new().tempfile_in(temp_dir)?;

        let stdin = std::io::stdin();
        let mut stdin_handle = stdin.lock();
        std::io::copy(&mut stdin_handle, &mut stdin_file)?;
        drop(stdin_handle);
        let (mut preview_file, tempfile_path) = stdin_file
            .keep()
            .or(Err("Cannot keep temporary file".to_string()))?;

        // Only infer delimiter if QSV_DEFAULT_DELIMITER is not set
        if std::env::var("QSV_DEFAULT_DELIMITER").is_err() {
            // Seek to start of file before reading
            preview_file.seek(std::io::SeekFrom::Start(0))?;

            // Read first line to infer delimiter
            let mut first_line = String::new();
            let mut reader = io::BufReader::new(&preview_file);
            reader.read_line(&mut first_line)?;

            // Count occurrences of each potential delimiter
            let tab_count = first_line.matches('\t').count();
            let semicolon_count = first_line.matches(';').count();
            let comma_count = first_line.matches(',').count();

            // Special case: if we see multiple consecutive spaces but no tabs,
            // those spaces might actually be tabs in the original file
            let space_groups = first_line
                .split(|c: char| !c.is_whitespace())
                .filter(|s| !s.is_empty())
                .count();

            // Infer delimiter by finding the most frequent one
            let inferred = if tab_count > 0
                || (space_groups > 2 && comma_count == 0 && semicolon_count == 0)
            {
                "\t"
            } else if semicolon_count > 0 && semicolon_count >= comma_count {
                ";"
            } else {
                ","
            };

            // Set QSV_DEFAULT_DELIMITER environment variable
            // this is only for the current process. When qsv exits, it will not persist
            // safety: we wrap the set_var in an unsafe block because it's an unsafe function,
            // as it assumes a single-threaded environment, which we still are at this point
            unsafe { std::env::set_var("QSV_DEFAULT_DELIMITER", inferred) };
        }

        stdin_tempfile_path = Some(tempfile_path.clone());
        args.arg_input = Some(tempfile_path.to_string_lossy().to_string());
        rconfig.path = Some(tempfile_path);
    } else {
        // check if the input file exists
        if let Some(path) = rconfig.path.clone()
            && !path.exists()
        {
            return fail_clierror!("File {:?} does not exist", path.display());
        }
    }

    // Resolve "sniff" special value in dates_whitelist
    // This must happen after stdin processing so we have a valid path
    let resolved_whitelist =
        if args.flag_infer_dates && args.flag_dates_whitelist.eq_ignore_ascii_case("sniff") {
            if let Some(ref path) = rconfig.path {
                resolve_sniff_whitelist_cached(path, &args)?
            } else {
                // No path available - shouldn't happen after stdin handling
                args.flag_dates_whitelist.clone()
            }
        } else {
            args.flag_dates_whitelist.clone()
        };

    // Update the cache args with the resolved whitelist so cache comparison
    // works correctly (comparing actual column names, not "sniff" keyword)
    current_stats_args
        .flag_dates_whitelist
        .clone_from(&resolved_whitelist);

    let mut compute_stats = true;
    let mut create_cache = args.flag_cache_threshold == 1
        || args.flag_stats_jsonl
        || args.flag_cache_threshold.is_negative();

    let mut autoindex_set = false;

    let write_stats_jsonl = args.flag_stats_jsonl;

    if let Some(path) = rconfig.path.clone() {
        //safety: we know the path is a valid PathBuf, so we can use unwrap
        let path_file_stem = path.file_stem().unwrap().to_str().unwrap();
        let stats_file = stats_path(&path, false, args.flag_weight.is_some())?;
        // check if <FILESTEM>.stats.csv file already exists.
        // If it does, check if it was compiled using the same args.
        // However, if the --force flag is set,
        // recompute the stats even if the args are the same.
        if stats_file.exists() && !args.flag_force {
            let stats_args_json_file = stats_file.with_extension("csv.json");
            let existing_stats_args_json_str =
                match fs::read_to_string(stats_args_json_file.clone()) {
                    Ok(s) => s,
                    Err(e) => {
                        log::warn!(
                            "Could not read {path_file_stem}.stats.csv.json: {e:?}, recomputing..."
                        );
                        // remove stats cache files silently even if they don't exists
                        let _ = fs::remove_file(&stats_file);
                        let _ = fs::remove_file(&stats_args_json_file);
                        String::new()
                    },
                };

            if !existing_stats_args_json_str.is_empty() {
                let time_saved: u64;
                // deserialize the existing stats args json
                let existing_stats_args_json: StatsArgs = {
                    #[cfg(target_endian = "big")]
                    let mut stat_args =
                        match serde_json::from_str::<StatsArgs>(&existing_stats_args_json_str) {
                            Ok(args) => args,
                            Err(e) => {
                                log::warn!(
                                    "Could not deserialize {path_file_stem}.stats.csv.json: \
                                     {e:?}, recomputing..."
                                );
                                let _ = fs::remove_file(&stats_file);
                                let _ = fs::remove_file(&stats_args_json_file);
                                StatsArgs::default()
                            },
                        };
                    #[cfg(target_endian = "little")]
                    let mut stat_args = {
                        let mut json_buffer = existing_stats_args_json_str.into_bytes();
                        match simd_json::to_owned_value(&mut json_buffer) {
                            Ok(value) => match StatsArgs::from_owned_value(&value) {
                                Ok(args) => args,
                                Err(e) => {
                                    log::warn!(
                                        "Could not deserialize {path_file_stem}.stats.csv.json: \
                                         {e:?}, recomputing..."
                                    );
                                    let _ = fs::remove_file(&stats_file);
                                    let _ = fs::remove_file(&stats_args_json_file);
                                    StatsArgs::default()
                                },
                            },
                            Err(e) => {
                                log::warn!(
                                    "Could not parse {path_file_stem}.stats.csv.json: {e:?}, \
                                     recomputing..."
                                );
                                let _ = fs::remove_file(&stats_file);
                                let _ = fs::remove_file(&stats_args_json_file);
                                StatsArgs::default()
                            },
                        }
                    };

                    // we init these fields to empty values because we don't want to
                    // compare them when checking if the args are the same
                    stat_args.canonical_input_path = String::new();
                    stat_args.canonical_stats_path = String::new();
                    // raw whitelist is provenance metadata, not part of cache validity
                    stat_args.flag_dates_whitelist_raw = String::new();
                    stat_args.record_count = 0;
                    stat_args.date_generated = String::new();
                    time_saved = stat_args.compute_duration_ms;
                    stat_args.compute_duration_ms = 0;
                    stat_args.field_count = 0;
                    stat_args.filesize_bytes = 0;
                    stat_args.hash = FileHash::default();
                    stat_args
                };

                // check if the cached stats are current (ie the stats file is newer than the input
                // file), use the same args or if the --everything flag was set, and
                // all the other non-stats args are equal. If so, we don't need to recompute the
                // stats
                //
                // NOTE: changes to the SHAPE of the --everything output (e.g. adding the
                // `zero_padded_numeric` column) are NOT detected per-column here — the
                // `--everything` reuse path below has no per-stat guards (cardinality/mode/
                // quartiles/percentiles aren't checked either). Such output-shape changes are
                // invalidated solely by the `qsv_version` comparison below. Consequently, a
                // pre-existing same-version `--everything` cache built before the column was
                // added can still be served without it. The package version MUST therefore be
                // bumped on any release that changes the --everything column set, so old caches
                // are recomputed.
                let input_file_modified = fs::metadata(&path)?.modified()?;
                let stats_file_modified = fs::metadata(&stats_file)
                    .and_then(|m| m.modified())
                    .unwrap_or(input_file_modified);
                #[allow(clippy::nonminimal_bool)]
                if stats_file_modified > input_file_modified
                    && (existing_stats_args_json == current_stats_args
                        || existing_stats_args_json.flag_everything
                            && existing_stats_args_json.flag_infer_dates
                                == current_stats_args.flag_infer_dates
                            && existing_stats_args_json.flag_dates_whitelist
                                == current_stats_args.flag_dates_whitelist
                            && existing_stats_args_json.flag_prefer_dmy
                                == current_stats_args.flag_prefer_dmy
                            && existing_stats_args_json.flag_no_headers
                                == current_stats_args.flag_no_headers
                            && existing_stats_args_json.flag_delimiter
                                == current_stats_args.flag_delimiter
                            && existing_stats_args_json.flag_nulls == current_stats_args.flag_nulls
                            && existing_stats_args_json.flag_weight
                                == current_stats_args.flag_weight
                            && existing_stats_args_json.flag_percentile_list
                                == current_stats_args.flag_percentile_list
                            && existing_stats_args_json.flag_select
                                == current_stats_args.flag_select
                            && existing_stats_args_json.flag_round == current_stats_args.flag_round
                            && existing_stats_args_json.flag_typesonly
                                == current_stats_args.flag_typesonly
                            && existing_stats_args_json.flag_infer_boolean
                                == current_stats_args.flag_infer_boolean
                            && existing_stats_args_json.flag_boolean_patterns
                                == current_stats_args.flag_boolean_patterns
                            && existing_stats_args_json.flag_vis_whitespace
                                == current_stats_args.flag_vis_whitespace
                            && existing_stats_args_json.qsv_version
                                == current_stats_args.qsv_version)
                {
                    log::info!(
                        "{path_file_stem}.stats.csv already exists and is current. Skipping \
                         compute and using cached stats instead - {time_saved} milliseconds \
                         saved...",
                    );
                    compute_stats = false;
                } else {
                    log::info!(
                        "{path_file_stem}.stats.csv already exists, but is older than the input \
                         file or the args have changed, recomputing...",
                    );
                    let _ = fs::remove_file(&stats_file);
                }
            }
        }
        if compute_stats {
            let start_time = std::time::Instant::now();

            // check if flag_cache_threshold is a negative number,
            // if so, set the autoindex_size to absolute of the number
            if args.flag_cache_threshold.is_negative() {
                rconfig.autoindex_size = args.flag_cache_threshold.unsigned_abs() as u64;
                autoindex_set = true;
            }

            // Check if we have an index and will use parallel processing
            // If so, skip mem_file_check since memory-aware chunking will handle it
            let mut indexed_result = rconfig.indexed()?;
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
            if !will_use_parallel
                && (args.flag_everything
                    || args.flag_mode
                    || args.flag_cardinality
                    || args.flag_median
                    || args.flag_quartiles
                    || args.flag_mad
                    || args.flag_percentiles)
            {
                // Try mem_file_check, and if it fails for an unindexed file, auto-create index
                match util::mem_file_check(&path, false, args.flag_memcheck) {
                    Ok(_) => {
                        // Memory check passed, proceed with sequential processing
                    },
                    Err(e) => {
                        // Memory check failed. We have two fallbacks that can stack:
                        //   1. auto-create an index to switch to parallel/indexed mode
                        //   2. auto-enable DataSketches estimators (t-digest, HLL) to reduce
                        //      per-column memory regardless of sequential vs. parallel
                        // Only propagate the original OOM error if NEITHER fallback engages.
                        let mut index_succeeded = false;
                        if indexed_result.is_none() && !rconfig.is_stdin() {
                            log::info!(
                                "File too large for sequential processing. Auto-creating index to \
                                 enable parallel processing..."
                            );
                            match util::create_index_for_file(&path, &rconfig) {
                                Ok(()) => {
                                    indexed_result = rconfig.indexed()?;
                                    index_succeeded = indexed_result.is_some();
                                    if index_succeeded {
                                        log::info!(
                                            "Index created successfully. Switching to parallel \
                                             processing."
                                        );
                                    }
                                },
                                Err(index_err) => {
                                    log::warn!("Failed to auto-create index: {index_err}");
                                },
                            }
                        }

                        // Sketch fallback: only for OOM (not other CliErrors)
                        if matches!(e, CliError::OutOfMemory(_)) {
                            let enabled = try_enable_approx_sketches(
                                &mut args,
                                user_set_quantile_method,
                                user_set_cardinality_method,
                            );
                            if !enabled.is_empty() {
                                wwarn!(
                                    "OOM during memory check: auto-enabling DataSketches \
                                     estimators ({}). Re-run with explicit --quantile-method \
                                     exact / --cardinality-method exact to disable the \
                                     auto-enable.",
                                    enabled.join(", ")
                                );
                            } else if !index_succeeded {
                                return Err(e);
                            }
                        } else if !index_succeeded {
                            return Err(e);
                        }
                    },
                }
            }

            // we need to count the number of records in the file to calculate sparsity and
            // cardinality
            let record_count: u64;

            let (headers, stats) = match indexed_result {
                None => {
                    // without an index, we need to count the number of records in the file
                    // safety: we know util::count_rows() will not return an Err
                    record_count = util::count_rows(&rconfig).unwrap();
                    args.sequential_stats(&resolved_whitelist, record_count)
                },
                Some(idx) => {
                    // with an index, we get the rowcount instantaneously from the index
                    record_count = idx.count();
                    match args.flag_jobs {
                        Some(num_jobs) => {
                            if num_jobs == 1 {
                                args.sequential_stats(&resolved_whitelist, record_count)
                            } else {
                                args.parallel_stats(&resolved_whitelist, record_count)
                            }
                        },
                        _ => args.parallel_stats(&resolved_whitelist, record_count),
                    }
                },
            }?;
            // we cache the record count so we don't have to count the records again
            let _ = RECORD_COUNT.set(record_count);
            // log::info!("scanned {record_count} records...");

            let stats_sr_vec = args.stats_to_records(stats, args.flag_vis_whitespace);
            let mut work_br;

            // vec we use to compute dataset-level fingerprint hash
            let mut stats_br_vec: Vec<csv::ByteRecord> = Vec::with_capacity(stats_sr_vec.len());

            let stats_headers_sr = args.stats_headers();
            wtr.write_record(&stats_headers_sr)?;
            let fields = headers.iter().zip(stats_sr_vec);
            for (i, (header, stat)) in fields.enumerate() {
                let header = if args.flag_no_headers {
                    i.to_string().into_bytes()
                } else {
                    header.to_vec()
                };
                let stat = stat.iter().map(str::as_bytes);
                work_br = vec![&*header]
                    .into_iter()
                    .chain(stat)
                    .collect::<csv::ByteRecord>();
                wtr.write_record(&work_br)?;
                stats_br_vec.push(work_br);
            }

            // Always compute file-level metadata for JSON cache
            let ds_column_count = headers.len() as u64;
            let ds_filesize_bytes = fs::metadata(&path)?.len();

            // Compute hash of stats for data fingerprinting
            let stats_hash = {
                // the first FINGERPRINT_HASH_COLUMNS are used for the fingerprint hash
                let mut hash_input = Vec::with_capacity(FINGERPRINT_HASH_COLUMNS);

                // First, create a stable representation of the stats
                for record in &stats_br_vec {
                    // Take FINGERPRINT_HASH_COLUMNS columns only
                    for field in record.iter().take(FINGERPRINT_HASH_COLUMNS) {
                        let s = util::bytes_to_cow_str(field);
                        // Standardize number format
                        if let Ok(f) = s.parse::<f64>() {
                            hash_input.extend_from_slice(format!("{f:.10}").as_bytes());
                        } else {
                            hash_input.extend_from_slice(field);
                        }
                        hash_input.push(0x1F); // field separator
                    }
                    hash_input.push(b'\n');
                }

                // Add dataset stats
                hash_input.extend_from_slice(
                    format!("{record_count}\x1F{ds_column_count}\x1F{ds_filesize_bytes}\n")
                        .as_bytes(),
                );
                blake3::hash(hash_input.as_slice()).to_hex().to_string()
            };

            // populate file-level metadata in the stats args json
            current_stats_args.field_count = ds_column_count;
            current_stats_args.filesize_bytes = ds_filesize_bytes;
            current_stats_args.hash = FileHash { blake3: stats_hash };

            // update the stats args json metadata ===============
            // if the stats run took longer than the cache threshold and the threshold > 0,
            // cache the stats so we don't have to recompute it next time
            current_stats_args.compute_duration_ms = start_time.elapsed().as_millis() as u64;
            create_cache = create_cache
                || current_stats_args.compute_duration_ms > args.flag_cache_threshold as u64;

            // only init these info if we're creating a stats cache
            if create_cache {
                // safety: we know the path is a valid PathBuf, so we can use unwrap
                current_stats_args.canonical_input_path =
                    path.canonicalize()?.to_str().unwrap().to_string();
                current_stats_args.record_count = record_count;
                current_stats_args.date_generated = chrono::Utc::now().to_rfc3339();
            }
        }
    }

    wtr.flush()?;

    // record the original (unresolved) --dates-whitelist value (e.g. "sniff") in the cache
    // sidecar so a future "sniff" run on an unchanged file can reuse this sniff-resolved
    // whitelist instead of re-sniffing
    current_stats_args
        .flag_dates_whitelist_raw
        .clone_from(&args.flag_dates_whitelist);

    if let Some(pb) = stdin_tempfile_path {
        // remove the temp file we created to store stdin
        std::fs::remove_file(pb)?;
    }

    let currstats_filename = if compute_stats {
        // we computed the stats, use the stats temp file
        stats_csv_tempfile_fname
    } else {
        // we didn't compute the stats, re-use the existing stats file
        // safety: we know the path is a valid PathBuf, so we can use unwrap
        stats_path(
            rconfig.path.as_ref().unwrap(),
            false,
            args.flag_weight.is_some(),
        )?
        .to_str()
        .unwrap()
        .to_owned()
    };

    if rconfig.is_stdin() {
        // if we read from stdin, copy the temp stats file to "stdin.stats.csv" or
        // "stdin.stats.weighted.csv" safety: we know the path is a valid PathBuf, so we can
        // use unwrap
        let mut stats_pathbuf = stats_path(
            rconfig.path.as_ref().unwrap(),
            true,
            args.flag_weight.is_some(),
        )?;
        fs::copy(currstats_filename.clone(), stats_pathbuf.clone())?;

        // save the stats args to "stdin.stats.csv.json"
        stats_pathbuf.set_extension("csv.json");
        // Use platform-appropriate JSON serialization
        let json_string = cfg_select! {
            target_endian = "little" => simd_json::to_string_pretty(&current_stats_args)?,
            _ => serde_json::to_string_pretty(&current_stats_args)?,
        };
        std::fs::write(stats_pathbuf, json_string)?;
    } else if let Some(path) = rconfig.path {
        // if we read from a file, copy the temp stats file to "<FILESTEM>.stats.csv" or
        // "<FILESTEM>.stats.weighted.csv"
        let mut stats_pathbuf = path.clone();
        if args.flag_weight.is_some() {
            stats_pathbuf.set_extension("stats.weighted.csv");
        } else {
            stats_pathbuf.set_extension("stats.csv");
        }
        // safety: we know the path is a valid PathBuf, so we can use unwrap
        if currstats_filename != stats_pathbuf.to_str().unwrap() {
            // if the stats file is not the same as the input file, copy it
            fs::copy(currstats_filename.clone(), stats_pathbuf.clone())?;
        }

        if args.flag_cache_threshold == 0
            || (args.flag_cache_threshold.is_negative() && args.flag_cache_threshold % 10 == -5)
        {
            // if the cache threshold zero or is a negative number ending in 5,
            // delete both the index file and the stats cache file
            if autoindex_set {
                let index_file = path.with_extension("csv.idx");
                log::debug!("deleting index file: {}", index_file.display());
                if std::fs::remove_file(index_file.clone()).is_err() {
                    // fails silently if it can't remove the index file
                    log::warn!("Could not remove index file: {}", index_file.display());
                }
            }

            // remove the stats cache file
            if fs::remove_file(stats_pathbuf.clone()).is_err() {
                // fails silently if it can't remove the stats file
                log::warn!(
                    "Could not remove stats cache file: {}",
                    stats_pathbuf.display()
                );
            }
            // remove the stats cache JSON sidecar too, to avoid leaving an
            // orphaned sidecar from a prior run.
            let stats_json_pathbuf = stats_pathbuf.with_extension("csv.json");
            if stats_json_pathbuf.exists() && fs::remove_file(&stats_json_pathbuf).is_err() {
                log::warn!(
                    "Could not remove stats cache JSON sidecar: {}",
                    stats_json_pathbuf.display()
                );
            }
            create_cache = false;
        }

        if compute_stats && create_cache {
            // save the stats args to "<FILESTEM>.stats.csv.json"
            // if we computed the stats
            stats_pathbuf.set_extension("csv.json");
            // write empty file first so we can canonicalize it
            std::fs::File::create(stats_pathbuf.clone())?;
            // safety: we know the path is a valid PathBuf, so we can use unwrap
            current_stats_args.canonical_stats_path = stats_pathbuf
                .clone()
                .canonicalize()?
                .to_str()
                .unwrap()
                .to_string();
            // Use platform-appropriate JSON serialization
            let json_string = cfg_select! {
                target_endian = "little" => simd_json::to_string_pretty(&current_stats_args)?,
                _ => serde_json::to_string_pretty(&current_stats_args)?,
            };
            std::fs::write(stats_pathbuf.clone(), json_string)?;

            // save the stats data to "<FILESTEM>.stats.csv.data.jsonl"
            if write_stats_jsonl {
                let mut stats_jsonl_pathbuf = stats_pathbuf.clone();
                stats_jsonl_pathbuf.set_extension("data.jsonl");
                util::csv_to_jsonl(
                    &currstats_filename,
                    &STATSDATA_TYPES_MAP,
                    &stats_jsonl_pathbuf,
                    b',', // cache is always CSV (comma-delimited)
                )?;
            }
        }
    }

    if stdout_output_flag {
        // if we're outputting to stdout, copy the stats file to stdout
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        if output_delim == b',' {
            let currstats = fs::read_to_string(currstats_filename)?;
            handle.write_all(currstats.as_bytes())?;
        } else {
            // output has a non-comma delimiter, convert from CSV cache
            util::csv_to_delimited_writer(&currstats_filename, &mut handle, output_delim)?;
        }
        handle.flush()?;
    } else if let Some(output) = args.flag_output {
        // if we're outputting to a file, copy the stats file to the output file
        if currstats_filename != output {
            if output_delim == b',' && !snappy {
                // same format as cache - just copy the file
                fs::copy(currstats_filename, output)?;
            } else {
                // output needs delimiter conversion and/or Snappy compression;
                // use Config so .sz extension is handled transparently
                let out_config =
                    Config::new(Some(output).as_ref()).delimiter(Some(Delimiter(output_delim)));
                let mut out_wtr = out_config.writer()?;

                let in_file = fs::File::open(currstats_filename)?;
                let mut in_rdr = csv::ReaderBuilder::new()
                    .has_headers(true)
                    .from_reader(in_file);

                let headers = in_rdr.headers()?.clone();
                out_wtr.write_record(&headers)?;
                for result in in_rdr.records() {
                    out_wtr.write_record(&result?)?;
                }
                out_wtr.flush()?;
            }
        }
    }

    Ok(())
}

impl Args {
    /// Computes statistics for CSV data using a single-threaded sequential approach.
    ///
    /// This function processes the entire CSV file in a single thread, reading all records
    /// sequentially and computing statistics for each column. It's suitable for smaller datasets
    /// or when parallel processing overhead would be counterproductive.
    ///
    /// # Arguments
    ///
    /// * `whitelist` - A comma-separated list of column names for date inference, or "all" for all
    ///   columns
    ///
    /// # Returns
    ///
    /// * `Ok((csv::ByteRecord, Vec<Stats>))` - A tuple containing the CSV headers and computed
    ///   statistics
    /// * `Err(CliError)` - If there's an error reading the CSV or computing statistics
    ///
    /// # Process Flow
    ///
    /// 1. **Setup**: Creates a CSV reader with the configured settings
    /// 2. **Headers**: Reads and processes the CSV headers, applying column selection
    /// 3. **Date Inference**: Initializes date inference flags based on the whitelist
    /// 4. **Computation**: Processes all records sequentially to compute statistics
    /// 5. **Return**: Returns headers and computed statistics
    ///
    /// # Performance Characteristics
    ///
    /// * **Memory**: Processes records one at a time, keeping memory usage low
    /// * **CPU**: Single-threaded, no parallelization overhead
    /// * **I/O**: Sequential file reading, good for streaming data
    /// * **Best for**: Small to medium datasets, when simplicity is preferred
    ///
    /// # Error Handling
    ///
    /// * CSV parsing errors are propagated as `CliError`
    /// * Date inference initialization errors are handled
    /// * File I/O errors are wrapped in appropriate error types
    fn sequential_stats(
        &self,
        whitelist: &str,
        record_count: u64,
    ) -> CliResult<(csv::ByteRecord, Vec<Stats>)> {
        let mut rdr = self.rconfig().reader()?;
        let full_headers = rdr.byte_headers()?.clone();

        // Find weight column index and exclude it from selection
        let (weight_col_idx, sel, headers) =
            self.process_headers_with_weight_exclusion(&full_headers)?;

        init_date_inference(self.flag_infer_dates, &headers, whitelist)?;

        // record_count is only a capacity hint for the per-column accumulators;
        // the read limit stays usize::MAX so we always process the whole file.
        // With more than one job available, overlap CSV parsing with stats
        // accumulation via a two-thread pipeline - records are processed in
        // arrival order, so results are bit-identical to the single-threaded
        // path. --jobs 1 keeps everything on one thread.
        let stats = if util::njobs(self.flag_jobs) > 1 {
            self.compute_pipelined(&sel, rdr, record_count as usize, weight_col_idx)
        } else {
            self.compute(
                &sel,
                &mut rdr,
                usize::MAX,
                record_count as usize,
                weight_col_idx,
            )
        };
        Ok((headers, stats))
    }

    /// Computes statistics for CSV data using a multi-threaded parallel approach.
    ///
    /// This function processes the CSV file using multiple threads, dividing the work into
    /// chunks and processing each chunk in parallel. It requires an index file to enable
    /// random access to CSV records. For optimal performance on large datasets.
    ///
    /// # Arguments
    ///
    /// * `whitelist` - A comma-separated list of column names for date inference, or "all" for all
    ///   columns
    /// * `idx_count` - The number of records in the CSV file (from the index)
    ///
    /// # Returns
    ///
    /// * `Ok((csv::ByteRecord, Vec<Stats>))` - A tuple containing the CSV headers and computed
    ///   statistics
    /// * `Err(CliError)` - If there's an error reading the CSV or computing statistics
    ///
    /// # Process Flow
    ///
    /// 1. **Validation**: Falls back to sequential processing if `idx_count` is 0
    /// 2. **Setup**: Creates a CSV reader and processes headers
    /// 3. **Date Inference**: Initializes date inference flags based on the whitelist
    /// 4. **Parallelization**: Divides work into chunks based on available jobs
    /// 5. **Thread Pool**: Creates worker threads to process chunks concurrently
    /// 6. **Indexed Access**: Uses CSV index for random access to record chunks
    /// 7. **Merging**: Combines results from all threads using the `Commute` trait
    /// 8. **Return**: Returns headers and merged statistics
    ///
    /// # Performance Characteristics
    ///
    /// * **Memory**: Higher memory usage due to parallel processing
    /// * **CPU**: Multi-threaded, utilizes all available CPU cores
    /// * **I/O**: Random access via index, may have higher I/O overhead
    /// * **Best for**: Large datasets, when CPU utilization is important
    ///
    /// # Threading Details
    ///
    /// * Uses `ThreadPool` with number of jobs from `self.flag_jobs`
    /// * Chunk size is calculated based on total records and number of jobs
    /// * Each thread processes a contiguous chunk of records
    /// * Results are merged incrementally, in chunk order
    ///
    /// # Safety Considerations
    ///
    /// * Requires a valid CSV index file for random access
    /// * Uses unsafe code for performance-critical operations
    /// * Thread safety is ensured through channel-based communication
    /// * Index seeking operations are wrapped in `expect()` for better error messages
    ///
    /// # Fallback Behavior
    ///
    /// * Automatically falls back to `sequential_stats` when `idx_count` is 0
    /// * This handles edge cases where parallel processing isn't beneficial
    fn parallel_stats(
        &self,
        whitelist: &str,
        idx_count: u64,
    ) -> CliResult<(csv::ByteRecord, Vec<Stats>)> {
        // N.B. This method doesn't handle the case when the number of records
        // is zero correctly. So we use `sequential_stats` instead.
        if idx_count == 0 {
            return self.sequential_stats(whitelist, 0);
        }

        // Retain freed jemalloc pages for the duration of this parallel run when it
        // builds many Frequencies hashmaps (cardinality / mode / antimode). No-op
        // when background_thread is active or QSV_NO_ALLOC_TUNING is set. Guarded on
        // the extended-stats flags so plain streaming stats don't pay the extra RSS.
        if self.flag_everything || self.flag_cardinality || self.flag_mode {
            util::retain_alloc_pages_for_aggregation();
        }

        let mut rdr = self.rconfig().reader()?;
        let full_headers = rdr.byte_headers()?.clone();

        // Find weight column index and exclude it from selection
        let (weight_col_idx, sel, headers) =
            self.process_headers_with_weight_exclusion(&full_headers)?;

        init_date_inference(self.flag_infer_dates, &headers, whitelist)?;

        let njobs = util::njobs(self.flag_jobs);

        // Read memory limit from environment variable
        // If QSV_STATS_CHUNK_MEMORY_MB is set and can be parsed as a positive u64, set max chunk
        // memory. If QSV_STATS_CHUNK_MEMORY_MB is not set, use 0 (dynamic sizing).
        // If QSV_STATS_CHUNK_MEMORY_MB is set to -1, any non-positive value, or any value that
        // cannot be parsed as u64, use CPU-based chunking (None).
        let max_chunk_memory_mb = if let Ok(val) = std::env::var("QSV_STATS_CHUNK_MEMORY_MB") {
            // if valid, set max chunk memory
            // if invalid or non-positive, use CPU-based chunking
            atoi_simd::parse::<u64, false, false>(val.as_bytes()).ok()
        } else {
            Some(0) // default to dynamic sizing
        };

        // Get WhichStats configuration
        let which_stats = self.which_stats();

        // Check if non-streaming stats are enabled (require memory-aware chunking)
        let needs_memory_aware_chunking =
            which_stats.needs_memory_aware_chunking() && max_chunk_memory_mb.is_some();

        let (chunking_mode_info, chunk_size) = if needs_memory_aware_chunking {
            // Sample records for memory estimation
            let sample_records = util::sample_records(&self.rconfig(), 1000);

            // Calculate memory-aware chunk size
            let chunk_size = calculate_memory_aware_chunk_size(
                idx_count,
                njobs,
                max_chunk_memory_mb,
                &which_stats,
                sample_records.as_deref(),
            );
            // Estimate average record size from samples if available
            let avg_record_size = if let Some(samples) = sample_records {
                calculate_avg_record_size(&samples, &which_stats)
            } else {
                1024
            };

            let estimated_memory_mb =
                estimate_chunk_memory(chunk_size, avg_record_size, &which_stats, headers.len())
                    / (1024 * 1024);

            // Safety: max_chunk_memory_mb is guaranteed Some(...) here since
            // needs_memory_aware_chunking requires max_chunk_memory_mb.is_some()
            let chunking_mode = if max_chunk_memory_mb.unwrap_or(0) == 0 {
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
            // CPU-based chunking
            let chunk_size = util::chunk_size(idx_count as usize, njobs);
            (
                format!("CPU-based chunking: chunk_size={chunk_size}"),
                chunk_size,
            )
        };

        let nchunks = util::num_of_chunks(idx_count as usize, chunk_size);
        log::info!("({chunking_mode_info}) nchunks={nchunks}");

        let pool = ThreadPool::new(njobs);
        let (send, recv) = crossbeam_channel::bounded(nchunks);
        let args = Arc::new(self.clone());
        for i in 0..nchunks {
            let (send, args, sel) = (send.clone(), Arc::clone(&args), sel.clone());
            let weight_idx: Option<usize> = weight_col_idx;
            pool.execute(move || {
                // safety: indexed() is safe as we know we have an index file
                // and we know it will return an Ok
                // arguably, there is still a very small risk of a TOCTOU here,
                // but it's unlikely
                let mut idx = unsafe {
                    args.rconfig()
                        .indexed()
                        .unwrap_unchecked()
                        .unwrap_unchecked()
                };
                // safety: seek() is safe as we know we have an index file
                // we do an expect() here so that it triggers a human-panic
                // with some actionable info if the index is corrupted
                idx.seek((i * chunk_size) as u64)
                    .expect("Index seek failed.");
                // Indexed DerefMuts to csv::Reader, so compute reads the chunk
                // directly with a reused ByteRecord (no per-record allocation).
                // chunk_size doubles as the capacity hint: each worker only ever
                // accumulates one chunk's worth of values, so hinting the full
                // file row count here would balloon RSS x nchunks.
                // safety: send will only return an Error if the channel has been disconnected
                unsafe {
                    send.send((
                        i,
                        args.compute(&sel, &mut idx, chunk_size, chunk_size, weight_idx),
                    ))
                    .unwrap_unchecked();
                }
            });
        }
        drop(send);
        // Merge in CHUNK order, not completion order. Order-dependent stats
        // (sortiness/sort_order pair counts at chunk boundaries) are only
        // deterministic - and only equal to the sequential result - when
        // chunks are merged in file order. Completion-order merging made
        // sortiness vary from run to run.
        //
        // Merge INCREMENTALLY as results arrive: each chunk's Stats are folded
        // in (and freed) as soon as all earlier chunks have been merged.
        // Workers start in chunk order, so results arrive roughly in order and
        // the out-of-order buffer stays small - collecting everything before
        // merging would hold all nchunks results resident simultaneously.
        let mut pending: HashMap<usize, Vec<Stats>> = HashMap::default();
        let mut next_chunk = 0_usize;
        let mut merged: Option<Vec<Stats>> = None;
        for (i, chunk_stats) in &recv {
            pending.insert(i, chunk_stats);
            while let Some(chunk_stats) = pending.remove(&next_chunk) {
                match merged {
                    // Merge column-by-column in parallel: columns are
                    // independent, and each column still sees chunks in file
                    // order, so results are identical to a serial merge. This
                    // keeps the merge from becoming a serial tail - merging a
                    // high-cardinality column's Frequencies map hash-inserts
                    // every unique value of the incoming chunk.
                    Some(ref mut m) => m
                        .par_iter_mut()
                        .zip(chunk_stats)
                        .for_each(|(acc, chunk_col)| acc.merge(chunk_col)),
                    None => merged = Some(chunk_stats),
                }
                next_chunk += 1;
            }
        }
        // in the event of a channel error, we will return an empty vector
        Ok((headers, merged.unwrap_or_default()))
    }

    /// Converts a vector of `Stats` objects into CSV records for output.
    ///
    /// This function processes all computed statistics in parallel, converting each `Stats`
    /// object into a `csv::StringRecord` that can be written to the output file. The
    /// conversion is done using a thread pool for better performance on large datasets.
    ///
    /// # Arguments
    ///
    /// * `stats` - Vector of computed statistics for each column
    /// * `visualize_ws` - Whether to visualize whitespace characters in string outputs
    ///
    /// # Returns
    ///
    /// A vector of `csv::StringRecord` objects, one for each column's statistics
    ///
    /// # Process
    ///
    /// 1. **Setup**: Pre-allocates vectors and creates thread pool
    /// 2. **Parallel Processing**: Each `Stats` object is converted to a record in parallel
    /// 3. **Channel Communication**: Uses bounded channels for thread-safe communication
    /// 4. **Collection**: Gathers all converted records into the final vector
    ///
    /// # Performance
    ///
    /// * Uses thread pool with number of jobs from `self.flag_jobs`
    /// * Each `Stats` object is processed in its own thread
    /// * Bounded channels prevent memory explosion
    /// * Pre-allocated vectors reduce memory allocations
    ///
    /// # Safety
    ///
    /// * Uses unsafe code for performance-critical operations
    /// * Channel communication is thread-safe
    /// * Bounds checking is avoided where safe
    fn stats_to_records(&self, stats: Vec<Stats>, visualize_ws: bool) -> Vec<csv::StringRecord> {
        let round_places = self.flag_round;
        let infer_boolean = self.flag_infer_boolean;
        let mut records = Vec::with_capacity(stats.len());
        records.extend(repeat_n(csv::StringRecord::new(), stats.len()));
        let pool = ThreadPool::new(util::njobs(self.flag_jobs));
        let mut results = Vec::with_capacity(stats.len());
        for mut stat in stats {
            let (send, recv) = crossbeam_channel::bounded(0);
            results.push(recv);
            pool.execute(move || {
                // safety: this will only return an Error if the channel has been disconnected
                // which will not happen in this case
                send.send(stat.to_record(round_places, infer_boolean, visualize_ws))
                    .unwrap();
            });
        }
        for (i, recv) in results.into_iter().enumerate() {
            // safety: results.len() == records.len() so we know the index is valid
            // and doesn't require a bounds check.
            // The unwrap on recv.recv() is safe as the channel is bounded
            unsafe {
                *records.get_unchecked_mut(i) = recv.recv().unwrap();
            }
        }
        records
    }

    /// Computes statistics for CSV data read directly from a CSV reader.
    ///
    /// This function reads up to `limit` records from the reader and computes
    /// comprehensive statistics for each column. It's the core computation engine
    /// used by both sequential and parallel processing approaches.
    ///
    /// It reads records with `read_byte_record` into a single reused `ByteRecord`,
    /// avoiding the per-record allocation + copy that the `ByteRecords` iterator
    /// incurs (its `next()` clones the internal record for each row).
    ///
    /// # Arguments
    ///
    /// * `sel` - Column selection configuration
    /// * `rdr` - CSV reader positioned at the first record to process
    /// * `limit` - Maximum number of records to process (`usize::MAX` for all)
    /// * `expected_rows` - Expected number of records, used ONLY as a capacity hint for the
    ///   per-column accumulators (never as a read limit, so an inexact estimate cannot affect
    ///   results)
    /// * `weight_col_idx` - Optional index of the weight column
    ///
    /// # Returns
    ///
    /// A vector of `Stats` objects, one for each selected column
    ///
    /// # Process
    ///
    /// 1. **Initialization**: Creates `Stats` objects for each selected column
    /// 2. **Record Processing**: Reads records into a reused buffer
    /// 3. **Field Processing**: For each record, processes selected fields
    /// 4. **Statistics Accumulation**: Updates statistics for each field
    /// 5. **Type Inference**: Automatically detects data types during processing
    ///
    /// # Performance Optimizations
    ///
    /// * **Inline**: Function is marked as `#[inline]` for performance
    /// * **Unsafe Operations**: Uses unsafe code for bounds checking avoidance
    /// * **Memory Reuse**: One `ByteRecord` is reused for all rows - zero per-record allocations in
    ///   the hot loop
    /// * **Hot Loop Optimization**: Critical path is optimized for speed
    /// * **Register Usage**: Frequently accessed variables are kept in registers
    ///
    /// # Safety Considerations
    ///
    /// * Uses unsafe code for performance-critical operations
    /// * Assumes `INFER_DATE_FLAGS` is properly initialized
    /// * Bounds checking is avoided where safe
    /// * Assumes a valid CSV; read errors are not checked in this performance-critical path (same
    ///   semantics as before)
    #[inline]
    fn compute<R: std::io::Read>(
        &self,
        sel: &Selection,
        rdr: &mut csv::Reader<R>,
        limit: usize,
        expected_rows: usize,
        weight_col_idx: Option<usize>,
    ) -> Vec<Stats> {
        let sel_len = sel.len();
        let mut stats = self.new_stats(sel_len, expected_rows);

        // safety: we know INFER_DATE_FLAGS is Some because we called init_date_inference
        let infer_date_flags = INFER_DATE_FLAGS.get().unwrap();

        // so we don't need to get infer_boolean/prefer_dmy from big args struct for each iteration
        // and hopefully the compiler will optimize this and use registers in the hot loop
        let infer_boolean = self.flag_infer_boolean;
        let prefer_dmy = self.flag_prefer_dmy;

        // one reused record buffer for the entire scan - read_byte_record fills it
        // in place, so the hot loop does no per-record heap allocation
        let mut row = csv::ByteRecord::new();
        let mut records_read = 0_usize;
        while records_read < limit {
            // safety: `stats` assumes a valid CSV, so we don't check for CSV errors
            // in this performance-critical path (mirrors the previous
            // `row.unwrap_unchecked()` on the ByteRecords iterator)
            if !unsafe { rdr.read_byte_record(&mut row).unwrap_unchecked() } {
                break;
            }
            records_read += 1;

            Self::add_row(
                &mut stats,
                sel,
                &row,
                weight_col_idx,
                infer_date_flags,
                infer_boolean,
                prefer_dmy,
            );
        }
        stats
    }

    /// Processes one CSV record: extracts the optional weight and feeds every
    /// selected field into its column's `Stats` accumulator.
    ///
    /// This is the per-row hot loop, shared by `compute` (single reused buffer)
    /// and `compute_pipelined` (reader-thread batches) so it has exactly one
    /// implementation.
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn add_row(
        stats: &mut [Stats],
        sel: &Selection,
        row: &csv::ByteRecord,
        weight_col_idx: Option<usize>,
        infer_date_flags: &[bool],
        infer_boolean: bool,
        prefer_dmy: bool,
    ) {
        // Extract weight value if weight column is specified
        // in case of a parse error, invalid weight defaults to 1.0
        let weight = if let Some(widx) = weight_col_idx {
            if widx < row.len() {
                fast_float2::parse::<f64, &[u8]>(row.get(widx).unwrap_or(b"1.0")).unwrap_or(1.0)
            } else {
                1.0
            }
        } else {
            1.0
        };

        // safety: INFER_DATE_FLAGS is initialized with one flag per selected
        // column, so it has the same size as the selection - no bounds checks
        debug_assert_eq!(infer_date_flags.len(), sel.len());
        let mut i = 0;
        unsafe {
            for field in sel.select(row) {
                stats.get_unchecked_mut(i).add(
                    field,
                    weight,
                    *infer_date_flags.get_unchecked(i),
                    infer_boolean,
                    prefer_dmy,
                );
                i += 1;
            }
        }
    }

    /// Pipelined variant of `compute` for the unindexed (sequential) path:
    /// a reader thread parses CSV records into recycled batches while this
    /// thread runs the statistics accumulation, overlapping I/O + CSV parsing
    /// with stats work.
    ///
    /// Records are processed strictly in arrival (file) order, so the add()
    /// sequence - and therefore every Stats result, including order-dependent
    /// sortiness and t-digest state - is bit-identical to the single-threaded
    /// `compute`. Batches are recycled through a return channel, so
    /// steady-state processing does no per-record or per-batch allocation.
    fn compute_pipelined(
        &self,
        sel: &Selection,
        rdr: csv::Reader<Box<dyn std::io::Read + Send + 'static>>,
        expected_rows: usize,
        weight_col_idx: Option<usize>,
    ) -> Vec<Stats> {
        const BATCH_SIZE: usize = 1024;
        // one being filled + one being drained + two in flight
        const NBATCHES: usize = 4;

        let sel_len = sel.len();
        let mut stats = self.new_stats(sel_len, expected_rows);

        // safety: we know INFER_DATE_FLAGS is Some because we called init_date_inference
        let infer_date_flags = INFER_DATE_FLAGS.get().unwrap();
        let infer_boolean = self.flag_infer_boolean;
        let prefer_dmy = self.flag_prefer_dmy;

        // (batch, n): n = number of valid records in the batch
        let (full_tx, full_rx) =
            crossbeam_channel::bounded::<(Vec<csv::ByteRecord>, usize)>(NBATCHES);
        let (empty_tx, empty_rx) = crossbeam_channel::bounded::<Vec<csv::ByteRecord>>(NBATCHES);
        for _ in 0..NBATCHES {
            // safety: the channel holds NBATCHES items, so send cannot fail here
            unsafe {
                empty_tx
                    .send(vec![csv::ByteRecord::new(); BATCH_SIZE])
                    .unwrap_unchecked();
            }
        }

        std::thread::scope(|s| {
            // reader thread: fills recycled batches in place
            s.spawn(move || {
                let mut rdr = rdr;
                while let Ok(mut batch) = empty_rx.recv() {
                    let mut n = 0;
                    while n < BATCH_SIZE {
                        // safety: n < BATCH_SIZE == batch.len(); CSV read errors are
                        // not checked - same "assume valid CSV" semantics as compute()
                        if !unsafe {
                            rdr.read_byte_record(batch.get_unchecked_mut(n))
                                .unwrap_unchecked()
                        } {
                            break;
                        }
                        n += 1;
                    }
                    let eof = n < BATCH_SIZE;
                    if full_tx.send((batch, n)).is_err() || eof {
                        // dropping full_tx (and empty_rx) ends the consumer loop
                        return;
                    }
                }
            });

            // consumer (this thread): the same per-row hot loop as compute()
            for (batch, n) in &full_rx {
                for row in &batch[..n] {
                    Self::add_row(
                        &mut stats,
                        sel,
                        row,
                        weight_col_idx,
                        infer_date_flags,
                        infer_boolean,
                        prefer_dmy,
                    );
                }
                // recycle the batch; fails only after the reader exited at EOF
                let _ = empty_tx.send(batch);
            }
        });
        stats
    }

    /// Processes headers and handles weight column exclusion if needed.
    ///
    /// This function handles the logic for excluding the weight column from statistics
    /// computation. It finds the weight column index, creates a modified selection that
    /// excludes it, and returns the selected headers.
    ///
    /// # Arguments
    ///
    /// * `full_headers` - The full CSV headers as a `ByteRecord`
    ///
    /// # Returns
    ///
    /// * `Ok((Option<usize>, Selection, csv::ByteRecord))` - Tuple containing:
    ///   - Weight column index (None if no weight column specified)
    ///   - Modified selection (excluding weight column if present)
    ///   - Selected headers (excluding weight column if present)
    /// * `Err(CliError)` - If weight column is not found or no columns remain after exclusion
    ///
    /// # Process
    ///
    /// 1. **Weight Column Check**: If no weight column is specified, uses normal selection
    /// 2. **Weight Column Finding**: Finds the weight column index in full headers
    /// 3. **Selection Modification**: Removes weight column from selection
    /// 4. **Validation**: Ensures at least one column remains after exclusion
    /// 5. **Header Filtering**: Applies modified selection to get filtered headers
    fn process_headers_with_weight_exclusion(
        &self,
        full_headers: &csv::ByteRecord,
    ) -> CliResult<(Option<usize>, Selection, csv::ByteRecord)> {
        if let Some(ref weight_col) = self.flag_weight {
            // Find weight column index in full headers
            let weight_idx = full_headers
                .iter()
                .position(|h| {
                    let h_str = util::bytes_to_cow_str(h);
                    h_str.trim().eq_ignore_ascii_case(weight_col.trim())
                })
                .ok_or_else(|| {
                    CliError::Other(format!(
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
                return Err(CliError::Other(format!(
                    "After excluding weight column '{weight_col}', no columns remain for \
                     statistics computation"
                )));
            }

            let modified_sel = Selection::from_indices(sel_vec);

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

    /// Creates a CSV reader configuration based on the current arguments.
    ///
    /// This function builds a `Config` object for CSV reading that incorporates
    /// all the relevant settings from the command line arguments, including
    /// input file, delimiter, header settings, and column selection.
    ///
    /// # Returns
    ///
    /// A `Config` object configured for CSV reading with current settings
    ///
    /// # Configuration Options
    ///
    /// * **Input File**: Uses `self.arg_input` as the data source
    /// * **Delimiter**: Applies the configured delimiter from `self.flag_delimiter`
    /// * **Headers**: Sets header behavior based on `self.flag_no_headers`
    /// * **Column Selection**: Applies column selection from `self.flag_select`
    ///
    /// # Performance
    ///
    /// * **Inline**: Function is marked as `#[inline]` for performance
    /// * **Minimal Overhead**: Creates configuration without unnecessary allocations
    #[inline]
    fn rconfig(&self) -> Config {
        Config::new(self.arg_input.as_ref())
            .delimiter(self.flag_delimiter)
            .no_headers_flag(self.flag_no_headers)
            .select(self.flag_select.clone())
    }

    /// Creates a `WhichStats` configuration from the current arguments.
    #[inline]
    fn which_stats(&self) -> WhichStats {
        // approx_quantiles selects the t-digest engine for median/quartiles/percentiles.
        // run() validates this is mutually exclusive with --weight and forces mad off,
        // but we double-guard mad here in case which_stats is called from a path that
        // skipped run()'s validation (e.g. tests).
        let approx_quantiles = self.flag_quantile_method.eq_ignore_ascii_case("approx");
        // approx_cardinality selects the HyperLogLog engine for the cardinality column.
        // run() forces this off when --infer-boolean is set (boolean inference needs
        // cardinality == 2 exactness), so the flag is safe to read directly here.
        let approx_cardinality = self.flag_cardinality_method.eq_ignore_ascii_case("approx");
        WhichStats {
            include_nulls: self.flag_nulls,
            sum: !self.flag_typesonly,
            range: !self.flag_typesonly || self.flag_infer_boolean,
            dist: !self.flag_typesonly,
            cardinality: self.flag_everything || self.flag_cardinality,
            zero_padded_numeric: self.flag_everything || self.flag_zero_padded_numeric,
            median: !self.flag_everything && self.flag_median && !self.flag_quartiles,
            mad: !approx_quantiles && (self.flag_everything || self.flag_mad),
            quartiles: self.flag_everything || self.flag_quartiles,
            mode: self.flag_everything || self.flag_mode,
            typesonly: self.flag_typesonly,
            percentiles: self.flag_everything || self.flag_percentiles,
            use_weights: self.flag_weight.is_some(),
            approx_quantiles,
            approx_cardinality,
            mode_cardinality_cap: self.flag_mode_cardinality_cap,
            percentile_list: self.flag_percentile_list.clone().into_boxed_str(),
        }
    }

    /// Creates a vector of `Stats` objects for statistics computation.
    ///
    /// This function initializes a vector of `Stats` objects, one for each column
    /// that will be processed. Each `Stats` object is configured with the appropriate
    /// `WhichStats` settings based on the command line arguments.
    ///
    /// # Arguments
    ///
    /// * `record_len` - Number of columns to create statistics for
    ///
    /// # Returns
    ///
    /// A vector of initialized `Stats` objects
    ///
    /// # Configuration
    ///
    /// Each `Stats` object is configured with `WhichStats` settings:
    /// * **Nulls**: Enabled based on `self.flag_nulls`
    /// * **Sum/Range/Distribution**: Enabled unless `typesonly` is set
    /// * **Cardinality**: Enabled for `everything` or `cardinality` flags
    /// * **Median**: Enabled for `median` flag (unless `quartiles` is set)
    /// * **MAD**: Enabled for `everything` or `mad` flags
    /// * **Quartiles**: Enabled for `everything` or `quartiles` flags
    /// * **Mode**: Enabled for `everything` or `mode` flags
    /// * **Percentiles**: Enabled for `percentiles` flag
    ///
    /// # Performance
    ///
    /// * **Inline**: Function is marked as `#[inline]` for performance
    /// * **Pre-allocated**: Each `Stats` is constructed in place (NOT cloned - `Vec::clone` only
    ///   allocates `len`, which would silently discard the `with_capacity` reservations made in
    ///   `Stats::new`)
    #[inline]
    fn new_stats(&self, record_len: usize, expected_rows: usize) -> Vec<Stats> {
        let which = self.which_stats();
        (0..record_len)
            .map(|_| Stats::new(which.clone(), expected_rows))
            .collect()
    }

    pub fn stats_headers(&self) -> csv::StringRecord {
        if self.flag_typesonly {
            return csv::StringRecord::from(vec!["field", "type"]);
        }

        // with --everything, we have MAX_STAT_COLUMNS columns at most
        let mut fields = Vec::with_capacity(MAX_STAT_COLUMNS);

        // these are the standard stats columns that are always output
        // the "streaming" stats that are always included in stats output
        // aka the FINGERPRINT_HASH_COLUMNS
        fields.extend_from_slice(&[
            "field",
            "type",
            "is_ascii",
            "sum",
            "min",
            "max",
            "range",
            "sort_order",
            "sortiness",
            "min_length",
            "max_length",
            "sum_length",
            "avg_length",
            "stddev_length",
            "variance_length",
            "cv_length",
            "mean",
            "sem",
            "geometric_mean",
            "harmonic_mean",
            "stddev",
            "variance",
            "cv",
            "nullcount",
            "n_negative",
            "n_zero",
            "n_positive",
            "max_precision",
            "sparsity",
        ]);

        // these are the stats columns that are only output if the user requested them
        let everything = self.flag_everything;
        if self.flag_median && !self.flag_quartiles && !everything {
            fields.push("median");
        }
        if self.flag_mad || everything {
            fields.push("mad");
        }
        if self.flag_quartiles || everything {
            fields.extend_from_slice(&[
                "lower_outer_fence",
                "lower_inner_fence",
                "q1",
                "q2_median",
                "q3",
                "iqr",
                "upper_inner_fence",
                "upper_outer_fence",
                "skewness",
            ]);
        }
        if self.flag_cardinality || everything {
            fields.extend_from_slice(&["cardinality", "uniqueness_ratio"]);
        }
        if self.flag_mode || everything {
            fields.extend_from_slice(&[
                "mode",
                "mode_count",
                "mode_occurrences",
                "antimode",
                "antimode_count",
                "antimode_occurrences",
            ]);
        }
        if self.flag_percentiles || everything {
            fields.push("percentiles");
        }
        if self.flag_zero_padded_numeric || everything {
            fields.push("zero_padded_numeric");
        }

        csv::StringRecord::from(fields)
    }
}

/// Helper function to calculate average record size from samples
fn calculate_avg_record_size(samples: &[csv::ByteRecord], which_stats: &WhichStats) -> usize {
    if samples.is_empty() {
        1024 // Default
    } else {
        let total_size: usize = samples
            .iter()
            .map(|record| estimate_record_memory(record, which_stats))
            .sum();
        (total_size / samples.len()).max(1024)
    }
}

/// Estimates memory usage per record based on enabled statistics.
///
/// This function calculates the approximate memory footprint of a single CSV record
/// when computing statistics. The estimate includes:
/// - Base record size (sum of field lengths)
/// - Additional memory for non-streaming statistics (median, quartiles, modes, etc.)
///
/// # Arguments
///
/// * `record` - The CSV record to estimate memory for
/// * `which_stats` - Configuration indicating which statistics are enabled
///
/// # Returns
///
/// Estimated memory in bytes per record
fn estimate_record_memory(record: &csv::ByteRecord, which_stats: &WhichStats) -> usize {
    // Base memory: sum of all field lengths
    let base_size: usize = record.iter().map(<[u8]>::len).sum();

    // Additional memory for non-streaming statistics
    let mut additional_memory = 0;

    // For unsorted_stats (median, quartiles, MAD, percentiles)
    // Each numeric/date field requires 8 bytes (f64) to be stored
    if which_stats.quartiles || which_stats.median || which_stats.mad || which_stats.percentiles {
        // Estimate: assume half the fields are numeric/date (conservative)
        additional_memory += (record.len() / 2) * 8;
    }

    // For modes (mode/cardinality)
    // The Frequencies tracker stores one entry per UNIQUE value, not per row.
    // Assume 10% of rows introduce a new unique value (same heuristic as the
    // weighted_modes capacity hint) - still conservative for typical columns.
    if which_stats.mode || which_stats.cardinality {
        additional_memory += base_size / 10;
    }

    // Add overhead for Vec capacity (average of base_size and additional_memory)
    let overhead = usize::midpoint(base_size, additional_memory);

    base_size + additional_memory + overhead
}

/// Estimates total memory required for processing a chunk of records.
///
/// # Arguments
///
/// * `record_count` - Number of records in the chunk
/// * `avg_record_size` - Average size of a record in bytes
/// * `which_stats` - Configuration indicating which statistics are enabled
/// * `field_count` - Number of fields in the record
///
/// # Returns
///
/// Estimated total memory in bytes for the chunk
const fn estimate_chunk_memory(
    record_count: usize,
    avg_record_size: usize,
    which_stats: &WhichStats,
    field_count: usize,
) -> usize {
    // Base memory for records
    let base_memory = record_count.saturating_mul(avg_record_size);

    // Additional memory for non-streaming statistics
    let mut additional_memory = 0;

    // For unsorted_stats: 8 bytes per record per numeric/date field
    if which_stats.quartiles || which_stats.median || which_stats.mad || which_stats.percentiles {
        // Estimate: assume half the fields are numeric/date (conservative)
        additional_memory += record_count.saturating_mul((field_count / 2).saturating_mul(8));
    }

    // For modes: the Frequencies tracker stores one entry per UNIQUE value,
    // not per row - assume 10% of rows are unique (conservative)
    if which_stats.mode || which_stats.cardinality {
        additional_memory += record_count.saturating_mul(avg_record_size) / 10;
    }

    // Add overhead for data structures (Stats objects, Vec capacity, etc.)
    // Estimate 20% overhead
    let overhead = (base_memory + additional_memory) / 5;

    base_memory
        .saturating_add(additional_memory)
        .saturating_add(overhead)
}

/// Calculates memory-aware chunk size for parallel statistics processing.
///
/// This function determines an appropriate chunk size based on:
/// - Available memory per chunk (if configured)
/// - Dynamic estimation via sampling (if `max_chunk_memory_mb` is Some(0))
/// - CPU-based chunking (fallback)
///
/// # Arguments
///
/// * `idx_count` - Total number of records in the file
/// * `njobs` - Number of parallel jobs
/// * `max_chunk_memory_mb` - Maximum memory per chunk in MB (None = use CPU-based, Some(0) =
///   dynamic, Some(n) = fixed limit)
/// * `which_stats` - Configuration indicating which statistics are enabled
/// * `sample_records` - Optional slice of sample records for dynamic sizing
///
/// # Returns
///
/// Calculated chunk size (number of records per chunk)
fn calculate_memory_aware_chunk_size(
    idx_count: u64,
    njobs: usize,
    max_chunk_memory_mb: Option<u64>,
    which_stats: &WhichStats,
    sample_records: Option<&[csv::ByteRecord]>,
) -> usize {
    // Check if non-streaming stats are enabled (require memory-aware chunking)
    let needs_memory_aware_chunking = which_stats.needs_memory_aware_chunking();

    match max_chunk_memory_mb {
        None => {
            // No memory limit configured
            if needs_memory_aware_chunking {
                // Non-streaming stats require memory-aware chunking, default to dynamic sizing
                // This is equivalent to Some(0) - dynamic sizing
                util::calculate_dynamic_chunk_size(idx_count, njobs, sample_records, |record| {
                    estimate_record_memory(record, which_stats)
                })
            } else {
                // Streaming stats only, use CPU-based chunking
                util::chunk_size(idx_count as usize, njobs)
            }
        },
        Some(0) => {
            // Dynamic sizing: sample records to estimate average size.
            // Caller is expected to have gated on `needs_memory_aware_chunking`
            // (see `parallel_stats`). The assertion locks that invariant in
            // debug builds so a future caller can't silently bypass it.
            debug_assert!(
                needs_memory_aware_chunking,
                "Some(0) arm requires non-streaming stats; caller must gate on \
                 needs_memory_aware_chunking()"
            );
            util::calculate_dynamic_chunk_size(idx_count, njobs, sample_records, |record| {
                estimate_record_memory(record, which_stats)
            })
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
                        .map(|record| estimate_record_memory(record, which_stats))
                        .sum();
                    debug_assert!(total_size > 0, "total_size should be positive here");
                    total_size / samples.len() // samples.len() is guaranteed to be positive here
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

/// Determines the path for the statistics output file.
///
/// This function constructs the appropriate file path for the statistics output
/// based on the input file path and whether the input is from stdin. It handles
/// both regular file inputs and stdin input cases.
///
/// # Arguments
///
/// * `stats_csv_path` - The path to the input CSV file
/// * `stdin_flag` - Whether the input is from stdin
///
/// # Returns
///
/// * `Ok(PathBuf)` - The path where statistics should be written
/// * `Err(io::Error)` - If the path construction fails
///
/// # Behavior
///
/// * **Regular Files**: Creates a `.stats.csv` file in the same directory as the input
/// * **Stdin Input**: Creates a `stdin.stats.csv` file in the current directory
/// * **Path Validation**: Validates that the input path has a parent directory and filename
fn stats_path(stats_csv_path: &Path, stdin_flag: bool, weighted: bool) -> io::Result<PathBuf> {
    let parent = stats_csv_path
        .parent()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid path"))?;
    let fstem = stats_csv_path
        .file_stem()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid file name"))?;

    let new_fname = if stdin_flag {
        if weighted {
            "stdin.stats.weighted.csv".to_string()
        } else {
            "stdin.stats.csv".to_string()
        }
    } else if weighted {
        format!("{}.stats.weighted.csv", fstem.to_string_lossy())
    } else {
        format!("{}.stats.csv", fstem.to_string_lossy())
    };

    Ok(parent.join(new_fname))
}

/// Initializes date inference flags for CSV column headers.
///
/// This function sets up a global static `INFER_DATE_FLAGS` that determines which columns
/// should have date inference enabled during CSV processing. The flags are used to optimize
/// date parsing by only attempting to parse dates for columns that are likely to contain
/// date data.
///
/// # Arguments
///
/// * `infer_dates` - Whether date inference should be enabled at all
/// * `headers` - The CSV headers as a `ByteRecord` containing column names
/// * `flag_whitelist` - A comma-separated list of column name patterns to enable date inference
///   for. Use "all" (case-insensitive) to enable date inference for all columns.
///
/// # Returns
///
/// * `Ok(())` - Successfully initialized the date inference flags
/// * `Err(String)` - Error message if initialization failed
///
/// # Global State
///
/// This function modifies the global static `INFER_DATE_FLAGS` which is used throughout
/// the stats computation to determine which columns should attempt date parsing.
fn init_date_inference(
    infer_dates: bool,
    headers: &csv::ByteRecord,
    flag_whitelist: &str,
) -> Result<(), String> {
    if !infer_dates {
        // we're not inferring dates, set INFER_DATE_FLAGS to all false
        INFER_DATE_FLAGS
            .set(SmallVec::from_elem(false, headers.len()))
            .map_err(|_| "Cannot init empty date inference flags".to_string())?;
        return Ok(());
    }

    let infer_date_flags = if flag_whitelist.eq_ignore_ascii_case("all") {
        log::info!("inferring dates for ALL fields");
        SmallVec::from_elem(true, headers.len())
    } else {
        let mut header_str = String::new();
        let whitelist_lower = flag_whitelist.to_lowercase();
        log::info!("inferring dates with date-whitelist: {whitelist_lower}");

        let whitelist: SmallVec<[&str; 8]> = whitelist_lower.split(',').map(str::trim).collect();
        headers
            .iter()
            .map(|header| {
                util::to_lowercase_into(
                    simdutf8::basic::from_utf8(header).unwrap_or_default(),
                    &mut header_str,
                );
                whitelist
                    .iter()
                    .any(|whitelist_item| header_str.contains(whitelist_item))
            })
            .collect()
    };

    INFER_DATE_FLAGS
        .set(infer_date_flags)
        .map_err(|e| format!("Cannot init date inference flags: {e:?}"))?;
    Ok(())
}

/// Resolves the "sniff" dates-whitelist, reusing a previously sniff-resolved whitelist from a
/// current stats cache when possible to avoid re-sniffing.
///
/// Because "sniff" is the DEFAULT `--dates-whitelist`, the sniff used to run on every
/// `stats --infer-dates` invocation — even warm-cache hits — solely to rebuild the cache-key
/// whitelist. When a stats cache sidecar exists, is newer than the input file (so the file is
/// unchanged since the cache was built), and its whitelist was itself sniff-derived, the sniffed
/// column set is deterministic and identical, so we reuse it and skip the sniff entirely. Falls
/// back to an in-process sniff otherwise (no cache, stale cache, `--force`, or a cache whose
/// whitelist came from an explicit list rather than "sniff").
fn resolve_sniff_whitelist_cached(input_path: &std::path::Path, args: &Args) -> CliResult<String> {
    if !args.flag_force
        && let Some(cached) = read_current_sniff_whitelist(input_path, args)
    {
        log::info!("Reusing sniff-resolved dates-whitelist from current stats cache");
        return Ok(cached);
    }
    log::info!(
        "Resolving dates-whitelist 'sniff' for {}",
        input_path.display()
    );
    resolve_sniff_whitelist(input_path)
}

/// Returns the sniff-resolved dates-whitelist stored in the stats cache sidecar, but only if the
/// sidecar exists, is newer than the input file (i.e. the file is unchanged since the cache was
/// built), and its whitelist was itself derived from "sniff" (recorded in
/// `flag_dates_whitelist_raw`). Returns `None` otherwise, signalling that a fresh sniff is needed.
fn read_current_sniff_whitelist(input_path: &std::path::Path, args: &Args) -> Option<String> {
    let stats_file = stats_path(input_path, false, args.flag_weight.is_some()).ok()?;
    if !stats_file.exists() {
        return None;
    }

    // the input file must be unchanged since the cache was built
    let stats_modified = fs::metadata(&stats_file).and_then(|m| m.modified()).ok()?;
    let input_modified = fs::metadata(input_path).and_then(|m| m.modified()).ok()?;
    if stats_modified <= input_modified {
        return None;
    }

    let sidecar = stats_file.with_extension("csv.json");
    let json_str = fs::read_to_string(&sidecar).ok()?;

    #[cfg(target_endian = "little")]
    let cached: StatsArgs = {
        let mut json_buffer = json_str.into_bytes();
        let value = simd_json::to_owned_value(&mut json_buffer).ok()?;
        StatsArgs::from_owned_value(&value).ok()?
    };
    #[cfg(target_endian = "big")]
    let cached: StatsArgs = serde_json::from_str(&json_str).ok()?;

    // Only reuse when the cache actually performed sniff-based date inference. A cache built
    // WITHOUT --infer-dates stores the unresolved literal "sniff" keyword in flag_dates_whitelist
    // (resolution is gated on --infer-dates), and reusing that would skip sniffing and leave the
    // literal keyword as the whitelist, breaking date inference. Requiring flag_infer_dates (and
    // guarding against the literal "sniff" value defensively) ensures the stored whitelist is a
    // genuinely resolved column set (or the _qsv_no_date_columns_found sentinel).
    if cached.flag_infer_dates
        && cached
            .flag_dates_whitelist_raw
            .eq_ignore_ascii_case("sniff")
        && !cached.flag_dates_whitelist.is_empty()
        && !cached.flag_dates_whitelist.eq_ignore_ascii_case("sniff")
    {
        Some(cached.flag_dates_whitelist)
    } else {
        None
    }
}

/// Resolves the "sniff" special value in dates-whitelist by sniffing the file
/// in-process (via `sniff::date_columns`) and extracting the column names that have
/// Date or `DateTime` types.
fn resolve_sniff_whitelist(input_path: &std::path::Path) -> CliResult<String> {
    let date_columns = crate::cmd::sniff::date_columns(input_path)?;

    if date_columns.is_empty() {
        log::info!("sniff: no Date/DateTime columns found");
        // Return a sentinel that will not match any header, avoiding enabling
        // date inference for all columns when no Date/DateTime columns exist.
        // This is necessary because "".contains("") is always true.
        Ok("_qsv_no_date_columns_found".to_string())
    } else {
        log::info!(
            "sniff: found Date/DateTime columns: {}",
            date_columns.join(", ")
        );
        Ok(date_columns.join(","))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Serialize, Deserialize)]
struct WhichStats {
    include_nulls:        bool,
    sum:                  bool,
    range:                bool,
    dist:                 bool,
    cardinality:          bool,
    zero_padded_numeric:  bool,
    median:               bool,
    mad:                  bool,
    quartiles:            bool,
    mode:                 bool,
    typesonly:            bool,
    percentiles:          bool,
    use_weights:          bool,
    /// When true, use the Apache `DataSketches` t-digest engine for median, quartiles, and
    /// custom percentiles instead of the exact (sort-based) `Unsorted<f64>` engine. Mutually
    /// exclusive with `mad` and `use_weights`; validation in the module-level `run(argv: ...)`
    /// function rejects the bad combinations.
    approx_quantiles:     bool,
    /// When true, use the Apache `DataSketches` `HyperLogLog` engine for the cardinality
    /// column instead of the exact `Frequencies<Vec<u8>>` / `HashMap<Vec<u8>, f64>` tracker.
    /// `--infer-boolean` forces this off in `run()` (HLL's ~1.5% RSE breaks
    /// `cardinality == 2` checks). Independent of `approx_quantiles`.
    approx_cardinality:   bool,
    /// When > 0, drop mode-tracking once a column sees more than this many UNIQUE values
    /// (`Frequencies::len()` for unweighted modes, `HashMap::len()` for weighted - both are
    /// true cardinality). 0 = unbounded (the default). When the cap fires, output emits
    /// `*HIGH_CARDINALITY` for mode fields and `>=<cap>` for cardinality.
    mode_cardinality_cap: u64,
    percentile_list:      Box<str>,
}

impl Commute for WhichStats {
    #[inline]
    fn merge(&mut self, other: WhichStats) {
        assert_eq!(*self, other);
    }
}

impl WhichStats {
    const fn needs_memory_aware_chunking(&self) -> bool {
        self.quartiles
            || self.median
            || self.mad
            || self.percentiles
            || self.mode
            || self.cardinality
    }
}

/// Wrapper around `datasketches::tdigest::TDigestMut` so the `Stats` struct can keep its
/// derived `Clone` and `PartialEq` impls without forcing the upstream `TDigestMut`
/// (which only derives `Clone`/`Debug`) to grow them.
///
/// Equality between two t-digests is not meaningful (different ingestion orders can produce
/// different centroid layouts even for identical input multisets), so `PartialEq` is a
/// constant `true` — Stats's `PartialEq` is used only in tests for non-quantile fields.
///
/// Serde is intentionally NOT implemented: the field is annotated `#[serde(skip)]` on
/// `Stats::tdigest`, so the derived `Serialize`/`Deserialize` on `Stats` skip this field
/// entirely. T-digest state is intermediate; cache invalidation on `--quantile-method`
/// change rides on `StatsArgs` serialization instead.
#[cfg(not(target_endian = "big"))]
#[derive(Default)]
struct TDigestSlot(Option<datasketches::tdigest::TDigestMut>);

#[cfg(not(target_endian = "big"))]
impl Clone for TDigestSlot {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[cfg(not(target_endian = "big"))]
impl PartialEq for TDigestSlot {
    #[inline]
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

/// Big-endian fallback for `TDigestSlot`. Apache DataSketches is unavailable on
/// big-endian targets, so we substitute a tuple-struct wrapping `Option<TDigestStub>`
/// — the unit-like `TDigestStub` exposes the same no-op method surface used by the
/// little-endian call sites (`update`, `quantile`, `is_empty`). `Stats::tdigest` is
/// always `Default` (i.e. `Some` is never written) on these targets — the
/// `--quantile-method approx` codepath is rejected upstream by `run()` — so the
/// stub methods are never reached at runtime; they exist only so the shared call
/// sites in `update_modes`, `add_numeric_value`, `to_record`, etc. type-check
/// unchanged.
///
/// `PartialEq` is not needed on the stub itself: the wrapping `TDigestSlot`
/// has an explicit `impl PartialEq` (constant `true`) that mirrors the
/// little-endian semantics.
#[cfg(target_endian = "big")]
#[derive(Default, Clone)]
struct TDigestStub;

#[cfg(target_endian = "big")]
impl TDigestStub {
    #[inline]
    fn update(&mut self, _value: f64) {}

    #[inline]
    fn quantile(&self, _rank: f64) -> Option<f64> {
        None
    }

    #[inline]
    fn is_empty(&self) -> bool {
        true
    }
}

#[cfg(target_endian = "big")]
#[derive(Default, Clone)]
struct TDigestSlot(Option<TDigestStub>);

// Mirror the little-endian `PartialEq` semantics: equality between two
// t-digests isn't meaningful, so `eq` is a constant `true`. On big-endian
// the inner Option is always `None` anyway, but the explicit impl keeps
// the cross-target API symmetric (Stats's derived `PartialEq` is only used
// in tests for non-quantile fields).
#[cfg(target_endian = "big")]
impl PartialEq for TDigestSlot {
    #[inline]
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

/// Wrapper around `datasketches::hll::HllSketch` so the `Stats` struct can keep its
/// derived `Clone`, `PartialEq`, `Serialize`, `Deserialize` impls without forcing the
/// upstream `HllSketch` (which only derives `Clone`/`Debug`) to grow them.
///
/// `PartialEq` is a constant `true` for the same reason as `TDigestSlot`: two HLL sketches
/// over the same multiset can yield slightly different internal states depending on
/// register collisions, but their estimates converge — equality between sketches is not
/// a useful test (Stats's `PartialEq` is used only in tests for non-sketch fields).
///
/// Serde is intentionally NOT implemented: the field is annotated `#[serde(skip)]` on
/// `Stats::hll`, so the derived `Serialize`/`Deserialize` on `Stats` skip this field
/// entirely. HLL state is intermediate; cache invalidation on `--cardinality-method`
/// change rides on `StatsArgs` serialization instead.
#[cfg(not(target_endian = "big"))]
#[derive(Default)]
struct HllSlot(Option<datasketches::hll::HllSketch>);

#[cfg(not(target_endian = "big"))]
impl Clone for HllSlot {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[cfg(not(target_endian = "big"))]
impl PartialEq for HllSlot {
    #[inline]
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

/// Big-endian fallback for `HllSlot`. Apache DataSketches is unavailable on
/// big-endian targets, so we substitute a tuple-struct wrapping `Option<HllSketchStub>`
/// — the unit-like `HllSketchStub` exposes the same no-op method surface used by the
/// little-endian call sites (`update`, `estimate`). `Stats::hll` is always `Default`
/// (i.e. `Some` is never written) on these targets — the `--cardinality-method approx`
/// codepath is rejected upstream by `run()` — so the stub methods are never reached
/// at runtime; they exist only so the shared call sites in `update_modes` and
/// `to_record` type-check unchanged.
///
/// `PartialEq` is not needed on the stub itself: the wrapping `HllSlot` has
/// an explicit `impl PartialEq` (constant `true`) that mirrors the
/// little-endian semantics.
#[cfg(target_endian = "big")]
#[derive(Default, Clone)]
struct HllSketchStub;

#[cfg(target_endian = "big")]
impl HllSketchStub {
    #[inline]
    fn update(&mut self, _sample: &[u8]) {}

    #[inline]
    fn estimate(&self) -> f64 {
        0.0
    }
}

#[cfg(target_endian = "big")]
#[derive(Default, Clone)]
struct HllSlot(Option<HllSketchStub>);

// Mirror the little-endian `PartialEq` semantics: equality between two
// HLL sketches isn't meaningful, so `eq` is a constant `true`. On big-endian
// the inner Option is always `None` anyway, but the explicit impl keeps
// the cross-target API symmetric (Stats's derived `PartialEq` is only used
// in tests for non-sketch fields).
#[cfg(target_endian = "big")]
impl PartialEq for HllSlot {
    #[inline]
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

#[allow(clippy::unsafe_derive_deserialize)]
#[allow(clippy::struct_field_names)]
#[repr(C, align(64))] // Align to cache line size for better performance
#[derive(Clone, Serialize, Deserialize, PartialEq)]
struct Stats {
    // CACHE LINE 1: Most frequently accessed fields (hot data)
    // Group small, frequently accessed fields together
    typ:              FieldType, // 1 byte - accessed in every add() call
    is_ascii:         bool,      // 1 byte - accessed for strings
    /// Set when --mode-cardinality-cap fires and this column's `modes`/`weighted_modes`
    /// tracker is dropped mid-pass. Output uses `*HIGH_CARDINALITY` / `>=<cap>` sentinels.
    modes_dropped:    bool, // 1 byte
    /// --zero-padded-numeric accumulators (only updated when `which.zero_padded_numeric`).
    /// `zpn_disqualified` is sticky: set the first time a non-null value is not numeric-shaped
    /// (neither an all-ASCII-digit integer, a zero-padded decimal code, nor a parseable float),
    /// after which the per-cell check is skipped. `zpn_has_value` records whether at least one
    /// non-null numeric-shaped value was seen. Combined with the final inferred type at output
    /// time (flagged only when it is String — which `from_sample()` yields when at least one value
    /// carries a leading zero), they identify columns whose leading/padding zeros would be lost if
    /// cast to a number (zip codes, barcodes, padded IDs, ICD-9/Dewey/HS-style decimal
    /// classification codes).
    zpn_disqualified: bool, // 1 byte
    zpn_has_value:    bool,      // 1 byte
    max_precision:    u16,       // 2 bytes - accessed for floats

    // trailing padding inserted automatically by repr(C) to align the following u64 fields
    // (the exact number of bytes depends on the compiler-chosen size of FieldType, which is not
    // #[repr(u8)], so it is intentionally not asserted here)

    // Hot counters - all 8-byte aligned, accessed frequently
    nullcount:    u64, // 8 bytes - frequently updated counter
    sum_stotlen:  u64, // 8 bytes - frequently updated counter
    total_weight: f64, // 8 bytes - frequently updated for weighted stats

    // Configuration flags (accessed once during initialization, cold after init)
    which: WhichStats, // 32 bytes - read-only after initialization

    // CACHE LINE 2+: Less frequently accessed but still important
    // Large Option types that may be None, grouped by usage pattern
    sum: Option<TypedSum>, // 32 bytes - updated in add() for numeric types

    // CACHE LINE 3+: Statistics computation fields
    online:          Option<OnlineStats>, // 72 bytes - used for mean/variance calculations
    online_len:      Option<OnlineStats>, // 72 bytes - used for string length stats
    weighted_online: Option<WeightedOnlineStats>, // 72 bytes - Weighted online statistics

    // CACHE LINE 4+: Mode and cardinality computation
    // Counted-runs frequency map: one entry per UNIQUE value (vs the previous
    // Unsorted<Vec<u8>> which buffered every sample - one alloc+copy per row).
    modes:          Option<Frequencies<Vec<u8>>>, // 48 bytes - used for mode/cardinality
    weighted_modes: Option<HashMap<Vec<u8>, f64>>, // 48 bytes - Weighted mode/antimode tracking

    // CACHE LINE 5+: Sorting-based statistics
    #[allow(clippy::struct_field_names)]
    unsorted_stats:          Option<Unsorted<f64>>, // 32 bytes - median/quartiles/percentiles
    weighted_unsorted_stats: Option<Vec<(f64, f64)>>, /* 24 bytes - (value, weight) tuples for
                                                       * weighted
                                                       * quantiles */
    // Approximate-quantile engine. Mutually exclusive with `unsorted_stats` for a given
    // numeric column: when `which.approx_quantiles` is true and quantiles are requested,
    // values flow into `tdigest` instead of `unsorted_stats`.
    #[serde(skip)]
    tdigest:                 TDigestSlot,

    // Approximate-cardinality engine. Independent of modes: when
    // `which.approx_cardinality && which.cardinality`, every sample is fed to the HLL
    // sketch (regardless of `--mode-cardinality-cap`), and `to_record` emits the HLL
    // estimate in the cardinality column. Mode/antimode tracking is unaffected — they
    // still come from `modes` / `weighted_modes` and obey the cap.
    #[serde(skip)]
    hll: HllSlot,

    // CACHE LINE 6+: Min/Max tracking (largest field, least cache-friendly)
    minmax: Option<TypedMinMax>, // 432 bytes - largest field, accessed less frequently
}

/// Weighted online statistics using the weighted Welford's algorithm (West, 1979).
///
/// This struct implements weighted versions of mean, variance, and standard deviation
/// using an incremental algorithm that processes data in a single pass without storing
/// all values. The algorithm is numerically stable and suitable for streaming data.
///
/// The weighted mean is computed as: mean = `Σ(w_i` * `x_i`) / `Σ(w_i)`
/// The weighted variance uses the frequency weight definition: variance = `S_n` / (`W_n` - 1)
#[derive(Clone, Default, Serialize, Deserialize, PartialEq)]
struct WeightedOnlineStats {
    /// Sum of all weights: `W_n` = `Σ(w_i)`
    sum_weights:              f64,
    /// Current weighted mean: `M_n`
    weighted_mean:            f64,
    /// Sum of squared differences: `S_n` = `Σ(w_i` * (`x_i` - M_{i-1}) * (`x_i` - `M_i`))
    sum_squared_diffs:        f64,
    /// Sum of weighted logarithms: `Σ(w_i` * `ln(x_i)`) for weighted geometric mean
    sum_weighted_logs:        f64,
    /// Sum of weights for positive values (used as denominator for geometric mean)
    sum_weights_positive:     f64,
    /// Sum of weighted reciprocals: `Σ(w_i` / `x_i`) for weighted harmonic mean
    sum_weighted_reciprocals: f64,
    /// Sum of weights for non-zero values (used as denominator for harmonic mean)
    sum_weights_nonzero:      f64,
    /// Count of samples (for compatibility with `OnlineStats` interface)
    count:                    usize,
}

impl WeightedOnlineStats {
    /// Creates a new `WeightedOnlineStats` with all values initialized to zero.
    const fn new() -> Self {
        Self {
            sum_weights:              0.0,
            weighted_mean:            0.0,
            sum_squared_diffs:        0.0,
            sum_weighted_logs:        0.0,
            sum_weights_positive:     0.0,
            sum_weighted_reciprocals: 0.0,
            sum_weights_nonzero:      0.0,
            count:                    0,
        }
    }

    /// Adds a weighted sample to the statistics.
    ///
    /// # Arguments
    ///
    /// * `x` - The sample value
    /// * `w` - The weight for this sample (must be >= 0)
    ///
    /// # Algorithm
    ///
    /// Uses the weighted incremental algorithm:
    /// - `W_n` = W_{n-1} + `w_n`
    /// - `M_n` = M_{n-1} + (`w_n` / `W_n`) * (`x_n` - M_{n-1})
    /// - `S_n` = S_{n-1} + `w_n` * (`x_n` - M_{n-1}) * (`x_n` - `M_n`)
    /// - For geometric mean: accumulate `w_i` * `ln(x_i)` (only if `x_i` > 0)
    /// - For harmonic mean: accumulate `w_i` / `x_i` (only if `x_i` != 0)
    #[inline]
    fn add_weighted(&mut self, x: f64, w: f64) {
        if w <= 0.0 {
            return;
        }

        self.count += 1;
        self.sum_weights += w;

        let delta = x - self.weighted_mean;
        self.weighted_mean = (w / self.sum_weights).mul_add(delta, self.weighted_mean);
        let delta2 = x - self.weighted_mean;
        self.sum_squared_diffs = (w * delta).mul_add(delta2, self.sum_squared_diffs);

        // Accumulate weighted logs for geometric mean (only if x > 0)
        if x > 0.0 {
            self.sum_weighted_logs = w.mul_add(x.ln(), self.sum_weighted_logs);
            self.sum_weights_positive += w;
        }

        // Accumulate weighted reciprocals for harmonic mean (only if x != 0)
        if x != 0.0 {
            self.sum_weighted_reciprocals += w / x;
            self.sum_weights_nonzero += w;
        }
    }

    /// Returns the weighted mean.
    #[inline]
    const fn mean(&self) -> f64 {
        self.weighted_mean
    }

    /// Returns the weighted variance using frequency weight definition.
    ///
    /// Uses denominator (`W_n` - 1) for sample variance when weights represent frequency counts.
    #[inline]
    fn variance(&self) -> f64 {
        if self.sum_weights <= 1.0 {
            return 0.0;
        }
        self.sum_squared_diffs / (self.sum_weights - 1.0)
    }

    /// Returns the weighted standard deviation.
    #[inline]
    fn stddev(&self) -> f64 {
        self.variance().sqrt()
    }

    /// Returns the weighted geometric mean.
    ///
    /// Formula: `exp(Σ(w_i` * `ln(x_i)`) / `Σ(w_i)`) where sums are over positive values only.
    ///
    /// Returns NaN if no positive values were encountered or if `sum_weights_positive` is zero.
    #[inline]
    fn geometric_mean(&self) -> f64 {
        if self.sum_weights_positive <= 0.0 || self.sum_weighted_logs.is_nan() {
            return f64::NAN;
        }
        (self.sum_weighted_logs / self.sum_weights_positive).exp()
    }

    /// Returns the weighted harmonic mean.
    ///
    /// Formula: `Σ(w_i)` / `Σ(w_i` / `x_i`) where sums are over non-zero values only.
    ///
    /// Returns NaN if no non-zero values were encountered or if `sum_weighted_reciprocals` is zero.
    #[inline]
    fn harmonic_mean(&self) -> f64 {
        if self.sum_weights_nonzero <= 0.0 || self.sum_weighted_reciprocals <= 0.0 {
            return f64::NAN;
        }
        self.sum_weights_nonzero / self.sum_weighted_reciprocals
    }

    /// Returns the number of samples added (for compatibility).
    #[inline]
    const fn len(&self) -> usize {
        self.count
    }

    /// Merges another `WeightedOnlineStats` into this one.
    ///
    /// This is used for parallel processing where statistics from different
    /// chunks need to be combined.
    fn merge(&mut self, other: &WeightedOnlineStats) {
        if other.sum_weights == 0.0 {
            return;
        }
        if self.sum_weights == 0.0 {
            *self = other.clone();
            return;
        }

        let total_weights = self.sum_weights + other.sum_weights;
        let delta = other.weighted_mean - self.weighted_mean;

        // Update sum of squared differences using parallel merge formula
        // self.sum_squared_diffs += other.sum_squared_diffs
        //     + (self.sum_weights * other.sum_weights / total_weights) * delta * delta;
        // below is the fused multiply-add implementation of the above formula
        self.sum_squared_diffs += delta.mul_add(
            delta * (self.sum_weights * other.sum_weights / total_weights),
            other.sum_squared_diffs,
        );

        // Update weighted mean
        // self.weighted_mean = (self.sum_weights * self.weighted_mean
        //     + other.sum_weights * other.weighted_mean)
        //     / total_weights;
        // below is the fused multiply-add implementation of the above formula
        self.weighted_mean = self
            .sum_weights
            .mul_add(self.weighted_mean, other.sum_weights * other.weighted_mean)
            / total_weights;
        // Update sum of weighted logs and reciprocals (simple addition)
        self.sum_weighted_logs += other.sum_weighted_logs;
        self.sum_weights_positive += other.sum_weights_positive;
        self.sum_weighted_reciprocals += other.sum_weighted_reciprocals;
        self.sum_weights_nonzero += other.sum_weights_nonzero;
        // Update sum of weights and count
        self.sum_weights = total_weights;
        self.count += other.count;
    }
}

/// Computes weighted quantile from (value, weight) pairs.
///
/// # Arguments
///
/// * `data` - Vector of (value, weight) tuples (must be sorted by value, as sorted by
///   `to_record()`)
/// * `total_weight` - Total sum of all weights
/// * `percentile` - Percentile to compute (0.0 to 1.0, e.g., 0.5 for median)
///
/// # Returns
///
/// The value at the specified percentile, computed using the weighted nearest-rank method (no
/// interpolation).
fn weighted_quantile(data: &[(f64, f64)], total_weight: f64, percentile: f64) -> Option<f64> {
    if data.is_empty() || total_weight <= 0.0 {
        return None;
    }

    // Data is already sorted by to_record() before calling this function
    // No need to check or sort again

    let target_weight = percentile * total_weight;
    let mut cum_weight = 0.0;

    for &(value, weight) in data {
        cum_weight += weight;
        // Return the value at which cumulative weight first reaches or exceeds the target
        // This is the "nearest rank" method for weighted quantiles
        if cum_weight >= target_weight {
            return Some(value);
        }
    }

    // If we reach here, return the last value
    data.last().map(|(v, _)| *v)
}

/// Computes weighted quartiles (Q1, Q2, Q3) from (value, weight) pairs.
///
/// # Arguments
///
/// * `data` - Vector of (value, weight) tuples (must be sorted by value, as sorted by
///   `to_record()`)
/// * `total_weight` - Total sum of all weights
///
/// # Returns
///
/// Option containing (Q1, Q2, Q3) if data is not empty and `total_weight` > 0, None otherwise.
fn weighted_quartiles(data: &[(f64, f64)], total_weight: f64) -> Option<(f64, f64, f64)> {
    if data.is_empty() || total_weight <= 0.0 {
        return None;
    }
    // Data is already sorted by to_record() before calling this function
    // No need to check or sort again
    let thresholds = [
        0.25_f64 * total_weight,
        0.5_f64 * total_weight,
        0.75_f64 * total_weight,
    ];
    let mut results: [Option<f64>; 3] = [None, None, None];
    let mut cumulative_weight = 0.0_f64;
    let mut t_idx = 0_usize;
    for (value, weight) in data {
        cumulative_weight += *weight;
        // Assign values when cumulative weight first reaches/exceeds each threshold.
        while t_idx < thresholds.len() && cumulative_weight >= thresholds[t_idx] {
            if results[t_idx].is_none() {
                results[t_idx] = Some(*value);
            }
            t_idx += 1;
        }
        if t_idx >= thresholds.len() {
            break;
        }
    }
    if let (Some(q1), Some(q2), Some(q3)) = results.into() {
        Some((q1, q2, q3))
    } else {
        None
    }
}

/// Computes weighted median from (value, weight) pairs.
///
/// # Arguments
///
/// * `data` - Vector of (value, weight) tuples
/// * `total_weight` - Total sum of all weights
///
/// # Returns
///
/// The weighted median value if data is not empty, None otherwise.
fn weighted_median(data: &[(f64, f64)], total_weight: f64) -> Option<f64> {
    weighted_quantile(data, total_weight, 0.5)
}

/// Computes weighted Median Absolute Deviation (MAD) from (value, weight) pairs.
///
/// # Arguments
///
/// * `data` - Vector of (value, weight) tuples (must be sorted by value, as sorted by
///   `to_record()`)
/// * `total_weight` - Total sum of all weights
/// * `median` - The weighted median value
///
/// # Returns
///
/// The weighted MAD value if data is not empty, None otherwise.
fn weighted_mad(data: &[(f64, f64)], total_weight: f64, median: f64) -> Option<f64> {
    if data.is_empty() || total_weight <= 0.0 {
        return None;
    }

    // Calculate absolute deviations from the median
    let mut abs_deviations: Vec<(f64, f64)> = data
        .iter()
        .map(|&(value, weight)| ((value - median).abs(), weight))
        .collect();

    // Sort abs_deviations by absolute deviation value (new data, needs sorting)
    // Use parallel sort only for large datasets to avoid thread pool overhead
    if abs_deviations.len() > PAR_SORT_THRESHOLD {
        abs_deviations.par_sort_unstable_by(|a, b| {
            a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal)
        });
    } else {
        abs_deviations
            .sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    }

    // Calculate weighted median of absolute deviations
    weighted_median(&abs_deviations, total_weight)
}

/// Formats a list of antimodes into a display string with optional preview prefix,
/// NULL handling, and length truncation.
///
/// Used by both weighted and unweighted antimode formatting paths.
fn format_antimodes(
    antimodes: &[impl AsRef<[u8]>],
    antimodes_count: usize,
    separator: &str,
    max_len: usize,
    visualize_ws: bool,
) -> String {
    let mut antimodes_list = String::with_capacity(max_len);

    // We only store the first MAX_ANTIMODES antimodes
    // so if antimodes_count > MAX_ANTIMODES, add the "*PREVIEW: " prefix
    if antimodes_count > MAX_ANTIMODES {
        antimodes_list.push_str("*PREVIEW: ");
    }

    let antimodes_vals = &antimodes
        .iter()
        .map(|c| util::bytes_to_cow_str(c.as_ref()))
        .join(separator);

    // if the antimodes result starts with the separator,
    // it indicates that NULL is the first antimode. Add NULL to the list.
    if antimodes_vals.starts_with(separator) {
        antimodes_list.push_str("NULL");
    }
    antimodes_list.push_str(antimodes_vals);

    // truncate at max_len characters with an ellipsis
    if antimodes_list.len() > max_len {
        util::utf8_truncate(&mut antimodes_list, max_len + 1);
        antimodes_list.push_str("...");
    }

    if visualize_ws {
        util::visualize_whitespace(&antimodes_list)
    } else {
        antimodes_list
    }
}

/// Computes weighted percentiles from (value, weight) pairs.
///
/// # Arguments
///
/// * `data` - Vector of (value, weight) tuples (must be sorted by value, as sorted by
///   `to_record()`)
/// * `total_weight` - Total sum of all weights
/// * `percentile_list` - List of percentiles to compute (as u8 values, e.g., 5, 10, 90, 95)
///
/// # Returns
///
/// Vector of percentile values in the same order as `percentile_list`, or None if data is empty
fn weighted_percentiles(
    data: &[(f64, f64)],
    total_weight: f64,
    percentile_list: &[u8],
) -> Option<Vec<f64>> {
    if data.is_empty() || total_weight <= 0.0 {
        return None;
    }

    // Data is already sorted by to_record() before calling this function
    // No need to check or sort again

    // Precompute target cumulative weights for each percentile, keeping original index
    let mut targets: Vec<(f64, usize)> = percentile_list
        .iter()
        .enumerate()
        .map(|(idx, &p)| {
            let percentile_f64 = p as f64 / 100.0;
            let target_cum_weight = percentile_f64 * total_weight;
            (target_cum_weight, idx)
        })
        .collect();
    // targets is typically 5-10 elements (percentile list), so parallel sort overhead
    // exceeds the sorting time. Use regular sort instead.
    targets.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    let mut results = vec![0.0; percentile_list.len()];
    let mut cum_weight = 0.0;
    let mut target_idx = 0;
    for &(value, weight) in data {
        cum_weight += weight;
        while target_idx < targets.len() && cum_weight >= targets[target_idx].0 {
            let original_idx = targets[target_idx].1;
            results[original_idx] = value;
            target_idx += 1;
        }
        if target_idx == targets.len() {
            break;
        }
    }

    Some(results)
}

/// Converts a timestamp in milliseconds to RFC3339 format.
///
/// This function converts a Unix timestamp (in milliseconds) to a human-readable
/// RFC3339 formatted string. It handles both date and datetime types, returning
/// only the date component for date types.
///
/// # Arguments
///
/// * `timestamp` - Unix timestamp in milliseconds
/// * `typ` - The field type (`TDate` or `TDateTime`)
///
/// # Returns
///
/// A string in RFC3339 format (e.g., "2023-01-15T10:30:00Z" or "2023-01-15")
///
/// # Behavior
///
/// * **`TDate`**: Returns only the date component (YYYY-MM-DD)
/// * **`TDateTime`**: Returns full RFC3339 format with time and timezone
/// * **Invalid Timestamps**: Returns default RFC3339 format for invalid timestamps
#[inline]
fn timestamp_ms_to_rfc3339(timestamp: i64, typ: FieldType) -> String {
    let date_val = chrono::DateTime::from_timestamp_millis(timestamp)
        .unwrap_or_default()
        .to_rfc3339();

    // if type = Date, only return the date component
    // do not return the time component
    if typ == TDate {
        return date_val[..10].to_string();
    }
    date_val
}

impl Stats {
    /// Creates a new `Stats` object with the specified configuration.
    ///
    /// This function initializes a `Stats` object with all fields set to their default
    /// values and optional components created based on the `WhichStats` configuration.
    /// The object is optimized for performance with cache-line aligned fields.
    ///
    /// # Arguments
    ///
    /// * `which` - Configuration specifying which statistics to compute
    ///
    /// # Returns
    ///
    /// A new `Stats` object ready for statistics computation
    ///
    /// # Initialization Details
    ///
    /// * **Default Values**: All basic fields are initialized to sensible defaults
    /// * **Optional Components**: Creates `TypedSum`, `TypedMinMax`, `OnlineStats`, etc. based on
    ///   configuration
    /// * **Memory Pre-allocation**: Pre-allocates memory for unsorted statistics based on record
    ///   count
    /// * **Cache Optimization**: Fields are organized for optimal cache line usage
    ///
    /// # Performance
    ///
    /// * **Efficient Allocation**: Only allocates memory for enabled statistics
    /// * **Cache-Friendly**: Field layout optimized for CPU cache lines
    /// * **Pre-allocation**: Uses record count to pre-allocate appropriate memory sizes
    fn new(which: WhichStats, expected_rows: usize) -> Stats {
        let use_weights = which.use_weights;
        let (mut sum, mut minmax, mut online, mut online_len, mut modes, mut unsorted_stats) =
            (None, None, None, None, None, None);
        let mut weighted_online = None;
        let mut weighted_unsorted_stats = None;
        let mut weighted_modes = None;

        if which.sum {
            sum = Some(TypedSum::default());
        }
        if which.range {
            minmax = Some(TypedMinMax::default());
        }
        if which.dist {
            online = Some(stats::OnlineStats::default());
            online_len = Some(stats::OnlineStats::default());
            if use_weights {
                weighted_online = Some(WeightedOnlineStats::new());
            }
        }

        // preallocate memory for the unsorted stats structs.
        // expected_rows is the actual row count (sequential) or chunk size
        // (parallel worker) - if the caller has no estimate (0), fall back to
        // 10,000 to avoid allocating too much memory.
        // NOTE: this was previously read from RECORD_COUNT, which is only set
        // AFTER the compute pass, so the 10,000 fallback was always used (and
        // the repeat_n clone in new_stats discarded the reservation anyway).
        let record_count = if expected_rows == 0 {
            10_000
        } else {
            expected_rows
        };
        // Under --cardinality-method approx with mode tracking off, the HLL sketch
        // alone covers the cardinality column — skip allocating the exact modes
        // tracker (Unsorted/HashMap) entirely. When mode/antimode are also requested
        // we still need the exact tracker for those columns.
        let need_exact_modes_tracker =
            which.mode || (which.cardinality && !which.approx_cardinality);
        if need_exact_modes_tracker {
            if use_weights {
                // When using weights, weighted_modes handles both mode/antimode and cardinality
                // computation, so we don't need the separate modes (Unsorted) tracker
                // Estimate capacity: assume average cardinality of 10% of records
                weighted_modes = Some(HashMap::with_capacity((record_count / 10).max(16)));
            } else {
                // Frequencies grows with UNIQUE values, not rows. Use the same
                // 10%-of-rows cardinality heuristic as weighted_modes, capped:
                // hashbrown's with_capacity eagerly allocates its bucket array,
                // so an uncapped hint on a billion-row file would waste RSS.
                modes = Some(Frequencies::with_capacity(
                    (record_count / 10).clamp(16, 65_536),
                ));
            }
        }
        // we use the same Unsorted struct for median, mad, quartiles & percentiles —
        // unless --quantile-method approx is set, in which case we route values to a
        // t-digest instead. The two engines are mutually exclusive for a given Stats
        // instance.
        //
        // The `if which.approx_quantiles` branch is compiled out entirely on
        // big-endian: Apache DataSketches is unavailable there, so even an
        // internal caller that constructs `WhichStats { approx_quantiles: true,
        // .. }` directly (bypassing the CLI rejection in `run()`) takes the
        // exact path. This is stronger than gating only the allocation expression
        // with `cfg_select!`+`unreachable!()` — there is no runtime panic path.
        let needs_quantiles = which.quartiles || which.median || which.mad || which.percentiles;
        #[cfg_attr(target_endian = "big", allow(unused_mut))]
        let mut tdigest = TDigestSlot::default();
        if needs_quantiles {
            #[cfg(not(target_endian = "big"))]
            if which.approx_quantiles {
                // k=200 is the upstream default; ~1% rank error, more accurate at the tails.
                tdigest = TDigestSlot(Some(datasketches::tdigest::TDigestMut::new(200)));
            } else {
                unsorted_stats = Some(stats::Unsorted::with_capacity(record_count));
                if use_weights {
                    weighted_unsorted_stats = Some(Vec::with_capacity(record_count));
                }
            }
            #[cfg(target_endian = "big")]
            {
                // Exact path is the only path here: t-digest is compiled out.
                unsorted_stats = Some(stats::Unsorted::with_capacity(record_count));
                if use_weights {
                    weighted_unsorted_stats = Some(Vec::with_capacity(record_count));
                }
            }
        }
        // HyperLogLog cardinality engine: allocate when --cardinality-method approx
        // is selected AND cardinality output is enabled. HLL_LG_K (12) gives ~1.5%
        // RSE with ~5KB per column. Hll8 stores 8 bits per register (no decode work
        // on update); memory is the same order whether sparse or dense.
        //
        // Compiled out entirely on big-endian — Apache DataSketches is unavailable,
        // so `hll` is always the zero-sized default there (`HllSlot` is a
        // unit-like ZST on big-endian).
        #[cfg(not(target_endian = "big"))]
        let hll = if which.cardinality && which.approx_cardinality {
            HllSlot(Some(datasketches::hll::HllSketch::new(
                HLL_LG_K,
                datasketches::hll::HllType::Hll8,
            )))
        } else {
            HllSlot::default()
        };
        #[cfg(target_endian = "big")]
        let hll = HllSlot::default();
        Stats {
            typ: FieldType::default(),
            is_ascii: true,
            modes_dropped: false,
            zpn_disqualified: false,
            zpn_has_value: false,
            max_precision: 0,
            nullcount: 0,
            sum_stotlen: 0,
            total_weight: 0.0,
            which,
            sum,
            online,
            online_len,
            weighted_online,
            modes,
            weighted_modes,
            unsorted_stats,
            weighted_unsorted_stats,
            tdigest,
            hll,
            minmax,
        }
    }

    /// Adds a sample value to the statistics computation.
    ///
    /// This is the core method for accumulating statistics. It processes a single
    /// field value, updates type inference, and accumulates all relevant statistics
    /// based on the current configuration. This method is called for every field
    /// in every record during CSV processing.
    ///
    /// # Arguments
    ///
    /// * `sample` - The field value as bytes to process
    /// * `weight` - The weight for this sample (defaults to 1.0 when not using weights)
    /// * `infer_dates` - Whether to attempt date inference for this field
    /// * `infer_boolean` - Whether to attempt boolean inference for this field
    /// * `prefer_dmy` - Whether to prefer day/month/year date format over month/day/year
    ///
    /// # Process
    ///
    /// 1. **Type Inference**: Updates the field type based on the sample
    /// 2. **Early Return**: Skips computation if only type inference is needed
    /// 3. **Statistics Accumulation**: Updates all enabled statistics
    /// 4. **Performance Optimization**: Uses unsafe code for hot path operations
    ///
    /// # Statistics Updated
    ///
    /// * **Type Information**: Field type, ASCII flag, max precision
    /// * **Counters**: Null count, string length sum
    /// * **Sum Statistics**: Numeric sums for different types
    /// * **Min/Max**: Range and extreme value tracking
    /// * **Online Statistics**: Mean, variance, standard deviation
    /// * **Mode Statistics**: Mode and cardinality tracking
    /// * **Unsorted Statistics**: Data for median, quartiles, percentiles
    ///
    /// # Performance
    ///
    /// * **Always Inline**: Marked as `#[inline(always)]` for maximum performance
    /// * **Hot Path Optimization**: Critical path is highly optimized
    /// * **Unsafe Operations**: Uses unsafe code for bounds checking avoidance
    /// * **Conditional Computation**: Only computes enabled statistics
    ///
    /// # Safety
    ///
    /// * Uses unsafe code for performance-critical operations.
    /// * Updates modes/cardinality trackers with a sample value. Weighted modes and unweighted
    ///   modes are mutually exclusive.
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn update_modes(&mut self, sample: &[u8], weight: f64) {
        // HLL is independent of the mode-cardinality cap: feed every sample regardless
        // of whether the exact mode tracker has been dropped. This is what lets us
        // surface a precise cardinality estimate even when modes/antimodes are gated.
        if let Some(hll_sketch) = self.hll.0.as_mut() {
            hll_sketch.update(sample);
        }

        // --mode-cardinality-cap: when set, drop the tracker once it grows past the cap
        // and surface the abandonment via `modes_dropped` so to_record() can emit
        // sentinels. cap == 0 (default) means unbounded — preserves today's behavior.
        let cap = self.which.mode_cardinality_cap;
        if let Some(ref mut wm) = self.weighted_modes {
            // Weighted modes: HashMap::len() == unique-values seen (true cardinality).
            // Existing-key updates don't grow the map, so check the cap only on the
            // new-key branch — this preserves the get_mut fast path AND ensures the
            // tracker never exceeds `cap` entries (peak == cap exactly).
            if let Some(val) = wm.get_mut(sample) {
                *val += weight;
            } else if cap > 0 && wm.len() as u64 >= cap {
                // Adding a new key would push len to cap + 1; drop instead.
                self.weighted_modes = None;
                self.modes_dropped = true;
            } else {
                wm.insert(sample.to_vec(), weight);
            }
        } else if let Some(v) = self.modes.as_mut() {
            // Unweighted modes: Frequencies::len() == unique values seen (true
            // cardinality), same cap semantics as the weighted tracker.
            // add_borrowed_capped single-probes: existing keys always increment
            // (the map doesn't grow); a NEW key that would push the tracker past
            // `cap` unique entries is rejected and we drop the tracker instead.
            if !v.add_borrowed_capped(sample, cap) {
                self.modes = None;
                self.modes_dropped = true;
            }
        }
    }

    /// * Assumes valid UTF-8 input for string operations
    /// * Bounds checking is avoided where safe
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn add(
        &mut self,
        sample: &[u8],
        weight: f64,
        infer_dates: bool,
        infer_boolean: bool,
        prefer_dmy: bool,
    ) {
        let (sample_type, int_val, float_val) =
            FieldType::from_sample(infer_dates, prefer_dmy, sample, self.typ);
        self.typ.merge(sample_type);

        // we're inferring --typesonly, so don't add samples to compute statistics
        // unless we need to --infer-boolean. In which case, we need --cardinality
        // and --range, so we need to add samples.
        // Early return for the uncommon typesonly case
        // Most of the time we're NOT doing typesonly, so put this check first
        if self.which.typesonly && !infer_boolean {
            return;
        }

        let t = self.typ;

        // typesonly + infer_boolean: only need minmax + cardinality for boolean inference
        if self.which.typesonly {
            // safety: MinMax is enabled because range=true is set for infer_boolean.
            // The preceding `if self.which.typesonly && !infer_boolean { return; }` guard
            // ensures we only reach here when infer_boolean is true.
            debug_assert!(
                self.minmax.is_some(),
                "minmax must be enabled for typesonly+infer_boolean"
            );
            unsafe {
                self.minmax
                    .as_mut()
                    .unwrap_unchecked()
                    .add_with_parsed(t, sample, float_val, int_val);
            }
            self.update_modes(sample, weight);
            if sample_type == TNull {
                self.nullcount += 1;
            }
            return;
        }

        // --zero-padded-numeric tracking (gated by the flag; zero cost otherwise).
        // Runs for every non-typesonly, non-null sample. The sticky `zpn_disqualified` bit means
        // non-numeric columns pay only a single bool check after their first value; only genuinely
        // numeric-shaped columns get scanned for the whole pass. A value is numeric-shaped if it is
        // an all-digit integer, a zero-padded decimal code, or a plain float. A column is flagged
        // at output time when it is numeric-shaped throughout AND its final inferred type is String
        // — which from_sample() yields only when at least one value carried a leading zero (a zip
        // code/padded id, or a zero-padded float like 007.1).
        if self.which.zero_padded_numeric && !self.zpn_disqualified && b"" != sample {
            if sample_type == TFloat
                || sample.iter().all(u8::is_ascii_digit)
                || is_zero_padded_float(sample)
                || fast_float2::parse::<f64, &[u8]>(sample).is_ok()
            {
                // numeric-shaped: a freshly-parsed plain float (sample_type == TFloat — the common
                // case, no extra work), an all-digit int, a zero-padded decimal code, or a plain
                // float arriving after the column already widened to String. That last case
                // short-circuits sample_type to TString, so we re-parse with fast_float2 to avoid
                // wrongly disqualifying e.g. a 3.5 following a 007.1. The TFloat fast-path keeps
                // ordinary float columns from being re-parsed here when the flag is on.
                self.zpn_has_value = true;
            } else {
                self.zpn_disqualified = true;
            }
        }

        // Update total weight for weighted statistics
        // Skip entirely when weights aren't active (the common case)
        if self.which.use_weights && weight > 0.0 {
            self.total_weight += weight;
        }

        // Process the frequently used Option-based statistics first
        // These are commonly enabled, so check them in order of likelihood

        // microbenchmarks show 'b"" != sample' is faster than '!sample.is_empty()'
        if b"" != sample {
            // safety: sum is always enabled and if check above ensures there is a sample to add
            unsafe {
                self.sum
                    .as_mut()
                    .unwrap_unchecked()
                    .add_with_parsed(t, sample, float_val, int_val);
            }
        }

        // safety: MinMax always enabled
        unsafe {
            self.minmax
                .as_mut()
                .unwrap_unchecked()
                .add_with_parsed(t, sample, float_val, int_val);
        };

        // Modes/cardinality less common but still frequent
        // These are mutually exclusive: weighted_modes is used when weights are active,
        // otherwise the unweighted modes (Unsorted) tracker is used.
        self.update_modes(sample, weight);

        // Always track string length online stats — the column type can widen
        // to TString later (e.g. --infer-dates with dates + a non-date value,
        // or Integer/Float/Date/DateTime → String), and stddev_length /
        // variance_length / cv_length must include earlier rows that were
        // processed under a more specific type.
        //
        // Null/empty samples contribute a zero-length entry, matching the
        // pre-fix behavior for nulls that arrived *after* the column widened
        // to TString. avg_length divides stotlen by the total record count
        // (including nulls), so the online-stats distribution is intentionally
        // computed over the same all-rows population for a consistent
        // mean/stddev relationship.
        //
        // online_len is Some iff which.dist is true (which.dist ==
        // !flag_typesonly); both typesonly paths early-returned above, so it
        // is always Some here. We still use `if let` for defense-in-depth
        // since this branch is no longer gated by t == TString, and assert
        // the invariant in debug builds.
        debug_assert!(
            self.online_len.is_some(),
            "online_len must be enabled after the typesonly early-returns"
        );
        if let Some(ol) = self.online_len.as_mut() {
            ol.add(&sample.len());
        }

        if t == TString {
            // ASCII check: once false, it stays false, so check the flag first
            if self.is_ascii {
                self.is_ascii = sample.is_ascii();
            }
            if sample_type == TNull {
                self.nullcount += 1;
            }
            return; // Early return for strings
        }

        // Handle null counting - most samples are NOT null
        if sample_type == TNull {
            self.nullcount += 1;
            if self.which.include_nulls {
                // safety: online is always enabled
                unsafe {
                    self.online.as_mut().unwrap_unchecked().add_null();
                }
            }
            return; // Early return for nulls
        }

        // Process other types - from most to least frequent
        match t {
            TInteger | TFloat => {
                self.add_numeric_value(float_val, weight);

                // precision calculation (TFloat only)
                // note that we are referring to number of decimal places,
                // not the number of significant digits
                if t == TFloat {
                    let precision = if float_val == 0.0 {
                        0
                    } else {
                        // safety: we know that f is a valid f64
                        // so there will always be a fraction part, even if it's 0
                        unsafe {
                            zmij::Buffer::new()
                                .format_finite(float_val)
                                .split('.')
                                .next_back()
                                .unwrap_unchecked()
                                .len() as u16
                        }
                    };
                    self.max_precision = std::cmp::max(self.max_precision, precision);
                }
            },
            TDateTime | TDate => {
                // calculate date statistics by adding date samples as unix timestamps
                // to the sub-second precision.
                #[allow(clippy::cast_precision_loss)]
                let timestamp = int_val as f64;
                self.add_numeric_value(timestamp, weight);
            },
            _ => {},
        }
    }

    /// Adds a numeric value to online stats, unsorted stats, and weighted variants.
    /// Shared by TInteger/TFloat and TDateTime/TDate paths to reduce code duplication
    /// and improve instruction cache utilization.
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn add_numeric_value(&mut self, value: f64, weight: f64) {
        if let Some(v) = self.unsorted_stats.as_mut() {
            v.add(value);
        }
        // approx-quantile engine: only one of unsorted_stats / tdigest is Some by
        // construction in Stats::new, so this is a no-op when exact mode is in use.
        if let Some(ref mut td) = self.tdigest.0 {
            td.update(value);
        }
        // safety: online is always enabled
        unsafe {
            self.online.as_mut().unwrap_unchecked().add_f64(value);
        }
        // Skip weighted stats branches entirely when weights aren't active
        if self.which.use_weights {
            if let Some(v) = self.weighted_unsorted_stats.as_mut() {
                // Only store valid weights to avoid filtering later
                if weight > 0.0 {
                    v.push((value, weight));
                }
            }
            if let Some(ref mut wos) = self.weighted_online {
                wos.add_weighted(value, weight);
            }
        }
    }

    /// Converts the collected statistics into a CSV record for output.
    ///
    /// This function formats all the computed statistics for a single column into a
    /// `csv::StringRecord` that can be written to the output CSV file. The function
    /// handles different data types (numeric, string, date, boolean) and applies
    /// appropriate formatting based on the configuration flags.
    ///
    /// # Arguments
    ///
    /// * `round_places` - Number of decimal places to round numeric values to
    /// * `infer_boolean` - Whether to attempt boolean type inference for columns with cardinality 2
    /// * `visualize_ws` - Whether to visualize whitespace characters in string outputs
    ///
    /// # Returns
    ///
    /// A `csv::StringRecord` containing all the computed statistics for this column,
    /// formatted according to the specified parameters.
    ///
    /// # Statistics Included
    ///
    /// The function includes the following statistics (when enabled via `which` flags):
    ///
    /// * **Type information**: Data type, ASCII flag for strings
    /// * **Basic statistics**: Sum, min, max, range, sort order
    /// * **String statistics**: Length min/max/sum/avg/stddev/variance/coefficient of variation
    /// * **Numeric statistics**: Mean, standard error, geometric mean, harmonic mean, stddev,
    ///   variance, CV
    /// * **Distribution**: Null count, max precision, sparsity
    /// * **Robust statistics**: Median, MAD (Median Absolute Deviation)
    /// * **Quartiles**: Q1, Q2 (median), Q3, IQR, inner/outer fences, skewness
    /// * **Mode statistics**: Mode(s), mode count, mode occurrences, antimode(s), antimode count,
    ///   antimode occurrences
    /// * **Cardinality**: Unique value count, uniqueness ratio
    /// * **Percentiles**: Custom percentile values (when specified)
    ///
    /// # Type-Specific Behavior
    ///
    /// * **Numeric types**: All numeric statistics are computed and formatted with rounding
    /// * **String types**: Only string-relevant statistics (length, cardinality, mode) are computed
    /// * **Date/DateTime types**: Statistics are converted to RFC3339 format or days for
    ///   readability
    /// * **Boolean inference**: When enabled, columns with cardinality 2 are checked against
    ///   boolean patterns
    /// * **Null types**: Only basic type information is included
    ///
    /// # Performance Notes
    ///
    /// The function is optimized for performance with pre-allocated vectors and efficient
    /// string formatting. It reuses computed values (like median from quartiles) to avoid
    /// redundant calculations.
    #[allow(clippy::wrong_self_convention)]
    pub fn to_record(
        &mut self,
        round_places: u32,
        infer_boolean: bool,
        visualize_ws: bool,
    ) -> csv::StringRecord {
        // empty string constant to avoid repeated allocations
        const EMPTY_STR: &str = "";
        const EMPTY_STRING: String = String::new();

        // we're doing typesonly and not inferring boolean, just return the type
        if self.which.typesonly && !infer_boolean {
            return csv::StringRecord::from(vec![self.typ.to_string()]);
        }

        let typ = self.typ;
        // prealloc memory for performance
        // we have MAX_STAT_COLUMNS columns at most with --everything
        let mut record = csv::StringRecord::with_capacity(512, MAX_STAT_COLUMNS);
        // reuse a single itoa::Buffer for all integer-to-string conversions
        let mut itoa_buf = itoa::Buffer::new();

        // min/max/range/sort_order/sortiness (5 fields)
        // we do this first as we want to get the sort_order, so we can skip sorting if not
        // required. We also need to do this before --infer-boolean because we need to know
        // the min/max values to determine if the range is equal to the supported boolean
        // ranges as specified by --boolean-patterns.
        let minmax_range_sortorder_pieces: Vec<String>;
        let mut minval = String::new();
        let mut maxval = String::new();
        if let Some(mm) = self
            .minmax
            .as_ref()
            .and_then(|mm| mm.show(typ, round_places, visualize_ws))
        {
            // save min/max values for boolean inferencing
            minval.clone_from(&mm.0);
            maxval.clone_from(&mm.1);
            minmax_range_sortorder_pieces = vec![mm.0, mm.1, mm.2, mm.3, mm.4];
        } else {
            minmax_range_sortorder_pieces = vec![EMPTY_STRING; 5];
        }

        let record_count = *RECORD_COUNT.get().unwrap_or(&1);

        // get the stats separator
        let stats_separator = STATS_SEPARATOR.get_or_init(|| {
            if self.which.mode || self.which.percentiles {
                std::env::var("QSV_STATS_SEPARATOR")
                    .unwrap_or_else(|_| DEFAULT_STATS_SEPARATOR.to_string())
            } else {
                DEFAULT_STATS_SEPARATOR.to_string()
            }
        });

        // cardinality, uniqueness_ratio & modes/antimodes (3 fields each) - 8 total fields
        // we do this second because we can use the sort order with cardinality, to skip sorting
        // if its not required. This makes not only cardinality computation faster, it also makes
        // modes/antimodes computation faster.
        // We also need to know the cardinality to --infer-boolean should that be enabled
        let mut cardinality = 0;
        let mut mc_pieces: Vec<String> = Vec::new();

        // --mode-cardinality-cap: if this column's mode tracker was dropped during
        // ingestion (or merge), emit sentinels instead of the normal cardinality / mode /
        // antimode fields. cap == 0 (default) means modes_dropped is never set, so this
        // branch is dead code on the default path.
        if self.modes_dropped {
            let cap = self.which.mode_cardinality_cap;
            // Cardinality is unknown; emit ">=cap" so downstream parsers can detect that
            // the value is approximate (a leading ">=" trips integer parsers cleanly).
            if self.which.cardinality {
                mc_pieces.push(format!(">={cap}"));
                // uniqueness_ratio is also unknown.
                mc_pieces.push(EMPTY_STRING);
            }
            if self.which.mode {
                mc_pieces.extend_from_slice(&[
                    "*HIGH_CARDINALITY".to_string(),
                    EMPTY_STRING,
                    EMPTY_STRING,
                    "*HIGH_CARDINALITY".to_string(),
                    EMPTY_STRING,
                    EMPTY_STRING,
                ]);
            }
            // Sentinels emitted; the else-if chain below is skipped.
        } else if let Some(ref weighted_modes_map) = self.weighted_modes {
            // Weighted modes/antimodes computation
            mc_pieces.reserve(8);

            if self.which.cardinality {
                // Cardinality is the number of unique values
                cardinality = weighted_modes_map.len() as u64;
                mc_pieces.push(itoa_buf.format(cardinality).to_owned());
                // uniqueness_ratio = cardinality / record_count
                #[allow(clippy::cast_precision_loss)]
                mc_pieces.push(util::round_num(
                    (cardinality as f64) / (record_count as f64),
                    round_places,
                ));
            }

            if self.which.mode {
                if weighted_modes_map.is_empty() {
                    // Empty data
                    mc_pieces.extend_from_slice(&[
                        EMPTY_STRING,
                        "0".to_string(),
                        "0".to_string(),
                        EMPTY_STRING,
                        "0".to_string(),
                        "0".to_string(),
                    ]);
                } else {
                    // Check if all values are unique (cardinality == record_count)
                    let unique_count = weighted_modes_map.len() as u64;
                    if unique_count == record_count {
                        // all values unique
                        mc_pieces.extend_from_slice(
                            // modes - short-circuit modes calculation as there is none
                            &[
                                EMPTY_STRING,
                                "0".to_string(),
                                "0".to_string(),
                                // antimodes - instead of returning everything, just say *ALL
                                "*ALL".to_string(),
                                "0".to_string(),
                                "1".to_string(),
                            ],
                        );
                    } else {
                        // Find max and min weights
                        let max_weight = weighted_modes_map.values().copied().fold(0.0, f64::max);
                        let min_weight = weighted_modes_map
                            .values()
                            .copied()
                            .fold(f64::INFINITY, f64::min);

                        // Collect modes (values with max weight) in deterministic order
                        let mut modes_keys: Vec<&Vec<u8>> = weighted_modes_map
                            .iter()
                            .filter(|&(_, &weight)| (weight - max_weight).abs() < 1e-10)
                            .map(|(value, _)| value)
                            .collect();
                        if modes_keys.len() > PAR_SORT_THRESHOLD {
                            modes_keys.par_sort_unstable();
                        } else {
                            modes_keys.sort_unstable();
                        }
                        // Collect antimodes (values with min weight) in deterministic order
                        // Sort first, then truncate to MAX_ANTIMODES for deterministic output
                        let mut antimodes_keys: Vec<&Vec<u8>> = weighted_modes_map
                            .iter()
                            .filter(|&(_, &weight)| (weight - min_weight).abs() < 1e-10)
                            .map(|(value, _)| value)
                            .collect();
                        let antimodes_count = antimodes_keys.len();
                        if antimodes_keys.len() > PAR_SORT_THRESHOLD {
                            antimodes_keys.par_sort_unstable();
                        } else {
                            antimodes_keys.sort_unstable();
                        }
                        antimodes_keys.truncate(MAX_ANTIMODES);

                        let modes_count = modes_keys.len();

                        // Format modes - work with &Vec<u8> references directly,
                        // avoiding heap allocation from .cloned().collect()
                        let modes_list = if visualize_ws {
                            modes_keys
                                .iter()
                                .map(|c| util::visualize_whitespace(&util::bytes_to_cow_str(c)))
                                .join(stats_separator)
                        } else {
                            modes_keys
                                .iter()
                                .map(|c| util::bytes_to_cow_str(c))
                                .join(stats_separator)
                        };

                        // Format antimodes
                        let antimodes_len = ANTIMODES_LEN.get_or_init(|| {
                            std::env::var("QSV_ANTIMODES_LEN").map_or(
                                DEFAULT_ANTIMODES_LEN,
                                |val| {
                                    let parsed =
                                        atoi_simd::parse::<usize, false, false>(val.as_bytes())
                                            .unwrap_or(DEFAULT_ANTIMODES_LEN);
                                    // if 0, disable length limiting
                                    if parsed == 0 { usize::MAX } else { parsed }
                                },
                            )
                        });

                        // Format antimodes - work with &Vec<u8> references directly,
                        // avoiding heap allocation from .cloned().collect()
                        let antimodes_list = format_antimodes(
                            &antimodes_keys,
                            antimodes_count,
                            stats_separator,
                            *antimodes_len,
                            visualize_ws,
                        );

                        // For weighted modes, mode_occurrences is the max weight (rounded)
                        // For weighted antimodes, antimode_occurrences is the min weight (rounded)
                        #[allow(clippy::cast_possible_truncation)]
                        let mode_occurrences = max_weight.round() as u32;
                        #[allow(clippy::cast_possible_truncation)]
                        let antimode_occurrences = min_weight.round() as u32;

                        mc_pieces.extend_from_slice(&[
                            // mode/s
                            modes_list,
                            itoa_buf.format(modes_count).to_owned(),
                            itoa_buf.format(mode_occurrences).to_owned(),
                            // antimode/s
                            antimodes_list,
                            itoa_buf.format(antimodes_count).to_owned(),
                            itoa_buf.format(antimode_occurrences).to_owned(),
                        ]);
                    }
                }
            }
        } else {
            // Unweighted modes/antimodes computation (existing logic)
            match self.modes.as_ref() {
                None => {
                    if self.which.cardinality {
                        mc_pieces = vec![EMPTY_STRING; 2];
                    }
                    if self.which.mode {
                        mc_pieces = vec![EMPTY_STRING; 6];
                    }
                },
                Some(v) => {
                    mc_pieces.reserve(8);
                    if self.which.cardinality {
                        cardinality = v.cardinality();
                        mc_pieces.push(itoa_buf.format(cardinality).to_owned());
                        // uniqueness_ratio = cardinality / record_count
                        #[allow(clippy::cast_precision_loss)]
                        mc_pieces.push(util::round_num(
                            (cardinality as f64) / (record_count as f64),
                            round_places,
                        ));
                    }
                    if self.which.mode {
                        // mode/s & antimode/s
                        if cardinality == record_count {
                            // all values unique
                            mc_pieces.extend_from_slice(
                                // modes - short-circuit modes calculation as there is none
                                &[
                                    EMPTY_STRING,
                                    "0".to_string(),
                                    "0".to_string(),
                                    // antimodes - instead of returning everything, just say *ALL
                                    "*ALL".to_string(),
                                    "0".to_string(),
                                    "1".to_string(),
                                ],
                            );
                        } else {
                            let (
                                (modes_result, modes_count, mode_occurrences),
                                (antimodes_result, antimodes_count, antimode_occurrences),
                            ) = v.modes_antimodes();
                            // mode/s ============
                            let modes_list = if visualize_ws {
                                modes_result
                                    .iter()
                                    .map(|c| util::visualize_whitespace(&util::bytes_to_cow_str(c)))
                                    .join(stats_separator)
                            } else {
                                modes_result
                                    .iter()
                                    .map(|c| util::bytes_to_cow_str(c))
                                    .join(stats_separator)
                            };

                            // antimode/s ============
                            let antimodes_len = ANTIMODES_LEN.get_or_init(|| {
                                std::env::var("QSV_ANTIMODES_LEN").map_or(
                                    DEFAULT_ANTIMODES_LEN,
                                    |val| {
                                        let parsed =
                                            atoi_simd::parse::<usize, false, false>(val.as_bytes())
                                                .unwrap_or(DEFAULT_ANTIMODES_LEN);
                                        // if 0, disable length limiting
                                        if parsed == 0 { usize::MAX } else { parsed }
                                    },
                                )
                            });

                            let antimodes_list = format_antimodes(
                                &antimodes_result,
                                antimodes_count,
                                stats_separator,
                                *antimodes_len,
                                visualize_ws,
                            );

                            mc_pieces.extend_from_slice(&[
                                // mode/s
                                modes_list,
                                itoa_buf.format(modes_count).to_owned(),
                                itoa_buf.format(mode_occurrences).to_owned(),
                                // antimode/s
                                antimodes_list,
                                itoa_buf.format(antimodes_count).to_owned(),
                                itoa_buf.format(antimode_occurrences).to_owned(),
                            ]);
                        }
                    }
                },
            }
        }

        // --cardinality-method approx: override the cardinality and uniqueness_ratio
        // slots with the HyperLogLog estimate. The mode/antimode slots (positions 2-7
        // when present) are untouched — modes still come from the exact tracker,
        // gated by --mode-cardinality-cap. Stats::new guarantees mc_pieces has
        // positions 0 and 1 reserved when which.cardinality is true (every reachable
        // branch above either pushes 2 cardinality fields or starts with EMPTY_STRING
        // placeholders).
        if self.which.cardinality
            && let Some(hll_sketch) = self.hll.0.as_ref()
            && mc_pieces.len() >= 2
        {
            #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
            let est = hll_sketch.estimate().round() as u64;
            cardinality = est;
            itoa_buf.format(est).clone_into(&mut mc_pieces[0]);
            #[allow(clippy::cast_precision_loss)]
            let ratio = (est as f64) / (record_count as f64);
            mc_pieces[1] = util::round_num(ratio, round_places);
        }

        // type
        if cardinality == 2 && infer_boolean {
            // if cardinality is 2, it's a boolean if its in the true/false patterns
            let patterns = BOOLEAN_PATTERNS.get();
            if let Some(patterns) = patterns {
                let mut is_boolean = false;
                for pattern in patterns {
                    if pattern.matches(&minval).is_some() && pattern.matches(&maxval).is_some() {
                        record.push_field("Boolean");
                        is_boolean = true;
                        break;
                    }
                }
                if !is_boolean {
                    record.push_field(typ.as_str());
                }
            } else {
                record.push_field(typ.as_str());
            }
        } else {
            record.push_field(typ.as_str());
        }

        // we're doing --typesonly with --infer-boolean, we don't need to calculate anything else
        if self.which.typesonly && infer_boolean {
            return record;
        }

        // is_ascii
        if typ == FieldType::TString {
            record.push_field(&self.is_ascii.to_string());
        } else {
            record.push_field(EMPTY_STR);
        }

        // sum
        let stotlen =
            if let Some((stotlen_work, sum)) = self.sum.as_ref().and_then(|sum| sum.show(typ)) {
                if typ == FieldType::TFloat {
                    if let Ok(f64_val) = fast_float2::parse::<f64, &[u8]>(sum.as_bytes()) {
                        record.push_field(&util::round_num(f64_val, round_places));
                    } else {
                        record.push_field(&format!("ERROR: Cannot convert {sum} to a float."));
                    }
                } else {
                    record.push_field(&sum);
                }
                stotlen_work
            } else {
                record.push_field(EMPTY_STR);
                0
            };

        // min/max/range/sort_order
        // actually append it here - to preserve legacy ordering of columns
        for field in &minmax_range_sortorder_pieces {
            record.push_field(field);
        }

        // min/max/sum/avg/stddev/variance/cv length (7 fields)
        // we only show string length stats for String type
        if typ != FieldType::TString {
            for _ in 0..7 {
                record.push_field(EMPTY_STR);
            }
        } else if let Some(mm) = self.minmax.as_ref().and_then(TypedMinMax::len_range) {
            // we have a min/max length
            record.push_field(&mm.0);
            record.push_field(&mm.1);
            if stotlen < u64::MAX {
                record.push_field(itoa_buf.format(stotlen));
                #[allow(clippy::cast_precision_loss)]
                let avg_len = stotlen as f64 / record_count as f64;
                record.push_field(&util::round_num(avg_len, round_places));

                if let Some(vl) = self.online_len.as_ref() {
                    let vlen_stddev = vl.stddev();
                    record.push_field(&util::round_num(vlen_stddev, round_places));
                    record.push_field(&util::round_num(vl.variance(), round_places));
                    record.push_field(&util::round_num(vlen_stddev / avg_len, round_places));
                } else {
                    for _ in 0..3 {
                        record.push_field(EMPTY_STR);
                    }
                }
            } else {
                // we saturated the sum of string lengths, it means we had an overflow
                // so we return OVERFLOW_STRING for sum,avg,stddev,variance length
                for _ in 0..5 {
                    record.push_field(OVERFLOW_STRING);
                }
            }
        } else {
            for _ in 0..7 {
                record.push_field(EMPTY_STR);
            }
        }

        // mean, sem, geometric_mean, harmonic_mean, stddev, variance & cv (7 fields)
        if typ == TString || typ == TNull {
            for _ in 0..7 {
                record.push_field(EMPTY_STR);
            }
        } else if let Some(ref wos) = self.weighted_online {
            // Use weighted statistics
            let std_dev = wos.stddev();
            #[allow(clippy::cast_precision_loss)]
            let sem = std_dev / (wos.len() as f64).sqrt();
            let mean = wos.mean();
            let mean_string = util::round_num(mean, round_places);
            // if mean is 0, we can't calculate the CV, so we return NaN
            // we do this as checking for 0.0 floating point values is not reliable
            // so we do util::round_num() first as that is what is returned to the user
            // for 0.0 floating point values.
            let cv = if mean_string == "0" {
                f64::NAN
            } else {
                (std_dev / mean) * 100.0_f64
            };
            // Use weighted geometric and harmonic means
            let geometric_mean = wos.geometric_mean();
            let harmonic_mean = wos.harmonic_mean();
            if self.typ == TFloat || self.typ == TInteger {
                record.push_field(&mean_string);
                record.push_field(&util::round_num(sem, round_places));
                record.push_field(&util::round_num(geometric_mean, round_places));
                record.push_field(&util::round_num(harmonic_mean, round_places));
                record.push_field(&util::round_num(std_dev, round_places));
                record.push_field(&util::round_num(wos.variance(), round_places));
            } else {
                // by the time we get here, the type is a TDateTime or TDate
                record.push_field(&timestamp_ms_to_rfc3339(mean as i64, typ));
                // instead of returning sem, stdev & variance as timestamps, return it in
                // days as its more human readable and practical for real-world use cases
                // Round to at least 5 decimal places, so we have sub-second precision
                record.push_field(&util::round_num(
                    sem / MS_IN_DAY,
                    u32::max(round_places, DAY_DECIMAL_PLACES),
                ));
                record.push_field(&util::round_num(
                    geometric_mean / MS_IN_DAY,
                    u32::max(round_places, DAY_DECIMAL_PLACES),
                ));
                record.push_field(&util::round_num(
                    harmonic_mean / MS_IN_DAY,
                    u32::max(round_places, DAY_DECIMAL_PLACES),
                ));
                record.push_field(&util::round_num(
                    std_dev / MS_IN_DAY,
                    u32::max(round_places, DAY_DECIMAL_PLACES),
                ));
                record.push_field(&util::round_num(
                    wos.variance() / (MS_IN_DAY * MS_IN_DAY),
                    u32::max(round_places, DAY_DECIMAL_PLACES),
                ));
            }
            record.push_field(&util::round_num(cv, round_places));
        } else if let Some(ref v) = self.online {
            let std_dev = v.stddev();
            #[allow(clippy::cast_precision_loss)]
            let sem = std_dev / (v.len() as f64).sqrt();
            let mean = v.mean();
            let mean_string = util::round_num(mean, round_places);
            // if mean is 0, we can't calculate the CV, so we return NaN
            // we do this as checking for 0.0 floating point values is not reliable
            // so we do util::round_num() first as that is what is returned to the user
            // for 0.0 floating point values.
            let cv = if mean_string == "0" {
                f64::NAN
            } else {
                (std_dev / mean) * 100.0_f64
            };
            let geometric_mean = v.geometric_mean();
            let harmonic_mean = v.harmonic_mean();
            if self.typ == TFloat || self.typ == TInteger {
                record.push_field(&mean_string);
                record.push_field(&util::round_num(sem, round_places));
                record.push_field(&util::round_num(geometric_mean, round_places));
                record.push_field(&util::round_num(harmonic_mean, round_places));
                record.push_field(&util::round_num(std_dev, round_places));
                record.push_field(&util::round_num(v.variance(), round_places));
            } else {
                // by the time we get here, the type is a TDateTime or TDate
                record.push_field(&timestamp_ms_to_rfc3339(mean as i64, typ));
                // instead of returning sem, stdev & variance as timestamps, return it in
                // days as its more human readable and practical for real-world use cases
                // Round to at least 5 decimal places, so we have sub-second precision
                record.push_field(&util::round_num(
                    sem / MS_IN_DAY,
                    u32::max(round_places, DAY_DECIMAL_PLACES),
                ));
                record.push_field(&util::round_num(
                    geometric_mean / MS_IN_DAY,
                    u32::max(round_places, DAY_DECIMAL_PLACES),
                ));
                record.push_field(&util::round_num(
                    harmonic_mean / MS_IN_DAY,
                    u32::max(round_places, DAY_DECIMAL_PLACES),
                ));
                record.push_field(&util::round_num(
                    std_dev / MS_IN_DAY,
                    u32::max(round_places, DAY_DECIMAL_PLACES),
                ));
                record.push_field(&util::round_num(
                    v.variance() / (MS_IN_DAY * MS_IN_DAY),
                    u32::max(round_places, DAY_DECIMAL_PLACES),
                ));
            }
            record.push_field(&util::round_num(cv, round_places));
        } else {
            for _ in 0..7 {
                record.push_field(EMPTY_STR);
            }
        }

        // nullcount
        record.push_field(itoa_buf.format(self.nullcount));

        // n_negative, n_zero, n_positive
        if typ == TInteger || typ == TFloat {
            if let Some(ref v) = self.online {
                let (n_negative, n_zero, n_positive) = v.n_counts();
                record.push_field(itoa_buf.format(n_negative));
                record.push_field(itoa_buf.format(n_zero));
                record.push_field(itoa_buf.format(n_positive));
            } else {
                for _ in 0..3 {
                    record.push_field(EMPTY_STR);
                }
            }
        } else {
            for _ in 0..3 {
                record.push_field(EMPTY_STR);
            }
        }

        // max precision
        if typ == TFloat {
            record.push_field(itoa_buf.format(self.max_precision));
        } else {
            record.push_field(EMPTY_STR);
        }

        // sparsity
        #[allow(clippy::cast_precision_loss)]
        record.push_field(&util::round_num(
            self.nullcount as f64 / record_count as f64,
            round_places,
        ));

        // quartiles: lower_outer_fence, lower_inner_fence, q1, q2_median, q3, iqr,
        // upper_inner_fence, upper_outer_fence, skewness (9 fields)
        // as q2==median, cache and reuse it if the --median or --mad flags are set
        let mut existing_median = None;
        // Initialize quartile_pieces to ensure consistent field counts
        let mut quartile_pieces: Vec<String> = if self.which.quartiles {
            vec![EMPTY_STRING; 9]
        } else {
            Vec::new()
        };

        // Sort weighted data once if it exists
        // to avoid redundant sorting in multiple weighted functions
        // Take ownership to sort in-place (no clone needed - Stats object is dropped after
        // to_record)
        let sorted_weighted_data: Option<Vec<(f64, f64)>> =
            if let Some(mut weighted_data) = self.weighted_unsorted_stats.take() {
                if weighted_data.is_empty() {
                    None
                } else {
                    // Sort in-place - no clone needed since we took ownership
                    // Use parallel sort only for large datasets to avoid thread pool overhead
                    if weighted_data.len() > PAR_SORT_THRESHOLD {
                        weighted_data.par_sort_unstable_by(|a, b| {
                            a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal)
                        });
                    } else {
                        weighted_data.sort_unstable_by(|a, b| {
                            a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal)
                        });
                    }
                    Some(weighted_data)
                }
            } else {
                None
            };

        // Check if we should use weighted quartiles
        let quartiles_result = if let Some(weighted_data) = sorted_weighted_data.as_ref() {
            // Use weighted quartiles
            match typ {
                TInteger | TFloat | TDate | TDateTime => {
                    if self.which.quartiles {
                        weighted_quartiles(weighted_data, self.total_weight)
                    } else {
                        None
                    }
                },
                _ => None,
            }
        } else if let Some(ref mut td) = self.tdigest.0 {
            // Approx quartiles via t-digest. q1=p25, q2=p50, q3=p75. Returns None if the
            // digest is empty (which it will be for non-numeric columns even when the
            // engine is allocated).
            match typ {
                TInteger | TFloat | TDate | TDateTime if self.which.quartiles => {
                    match (td.quantile(0.25), td.quantile(0.50), td.quantile(0.75)) {
                        (Some(q1), Some(q2), Some(q3)) => Some((q1, q2, q3)),
                        _ => None,
                    }
                },
                _ => None,
            }
        } else {
            // Use unweighted exact quartiles
            self.unsorted_stats.as_mut().and_then(|v| match typ {
                TInteger | TFloat | TDate | TDateTime => {
                    if self.which.quartiles {
                        v.quartiles()
                    } else {
                        None
                    }
                },
                _ => None,
            })
        };

        match quartiles_result {
            None => {
                // quartile_pieces already initialized with empty strings if --quartiles is set
            },
            Some((q1, q2, q3)) => {
                existing_median = Some(q2);
                let iqr = q3 - q1;

                // use fused multiply add (mul_add)
                // fused mul_add is more accurate & is more performant if the
                // target architecture has a dedicated `fma` CPU instruction
                // https://doc.rust-lang.org/std/primitive.f64.html#method.mul_add

                // lower_outer_fence = "q1 - (3.0 * iqr)"
                let lof = 3.0f64.mul_add(-iqr, q1);
                // lower_inner_fence = "q1 - (1.5 * iqr)"
                let lif = 1.5f64.mul_add(-iqr, q1);

                // upper inner fence = "q3 + (1.5 * iqr)"
                let uif = 1.5_f64.mul_add(iqr, q3);
                // upper_outer_fence = "q3 + (3.0 * iqr)"
                let uof = 3.0_f64.mul_add(iqr, q3);

                // calculate skewness using Quantile-based measures
                // https://en.wikipedia.org/wiki/Skewness#Quantile-based_measures
                // https://blogs.sas.com/content/iml/2017/07/19/quantile-skewness.html
                // quantile skewness = ((q3 - q2) - (q2 - q1)) / iqr;
                // which is also (q3 - (2.0 * q2) + q1) / iqr
                // which in turn, is the basis of the fused multiply add version below
                let skewness = (2.0f64.mul_add(-q2, q3) + q1) / iqr;

                // Clear and replace quartile_pieces with actual values
                quartile_pieces.clear();
                quartile_pieces.reserve(9);
                if typ == TDateTime || typ == TDate {
                    // casting from f64 to i64 is OK, per
                    // https://doc.rust-lang.org/reference/expressions/operator-expr.html#numeric-cast
                    // as values larger/smaller than what i64 can handle will automatically
                    // saturate to i64 max/min values.
                    quartile_pieces.extend_from_slice(&[
                        timestamp_ms_to_rfc3339(lof as i64, typ),
                        timestamp_ms_to_rfc3339(lif as i64, typ),
                        timestamp_ms_to_rfc3339(q1 as i64, typ),
                        timestamp_ms_to_rfc3339(q2 as i64, typ), // q2 = median
                        timestamp_ms_to_rfc3339(q3 as i64, typ),
                        // return iqr in days - there are 86,400,000 ms in a day
                        util::round_num(
                            (q3 - q1) / MS_IN_DAY,
                            u32::max(round_places, DAY_DECIMAL_PLACES),
                        ),
                        timestamp_ms_to_rfc3339(uif as i64, typ),
                        timestamp_ms_to_rfc3339(uof as i64, typ),
                    ]);
                } else {
                    quartile_pieces.extend_from_slice(&[
                        util::round_num(lof, round_places),
                        util::round_num(lif, round_places),
                        util::round_num(q1, round_places),
                        util::round_num(q2, round_places), // q2 = median
                        util::round_num(q3, round_places),
                        util::round_num(iqr, round_places),
                        util::round_num(uif, round_places),
                        util::round_num(uof, round_places),
                    ]);
                }
                quartile_pieces.push(util::round_num(skewness, round_places));
            },
        }

        // median
        // Only add median field if --median is set but --quartiles is NOT set
        // (when --quartiles is set, median is included as q2_median in quartile fields)
        // Note: self.which.median is only true when !flag_quartiles, so we don't need to check
        // !self.which.quartiles
        if self.which.median {
            let median_value = if let Some(weighted_data) = sorted_weighted_data.as_ref() {
                // Use weighted median
                match typ {
                    TNull | TString => None,
                    _ => weighted_median(weighted_data, self.total_weight),
                }
            } else if let Some(ref mut td) = self.tdigest.0 {
                // Approx median via t-digest.
                if let TNull | TString = typ {
                    None
                } else {
                    td.quantile(0.50)
                }
            } else {
                // Use unweighted exact median
                self.unsorted_stats.as_mut().and_then(|v| {
                    if let TNull | TString = typ {
                        None
                    } else {
                        v.median()
                    }
                })
            };

            // Set existing_median for MAD calculation
            if median_value.is_some() {
                existing_median = median_value;
            }

            if let Some(v) = median_value {
                if typ == TDateTime || typ == TDate {
                    // median rfc3339 timestamp
                    record.push_field(&timestamp_ms_to_rfc3339(v as i64, typ));
                } else {
                    // median as a floating point number
                    record.push_field(&util::round_num(v, round_places));
                }
            } else {
                record.push_field(EMPTY_STR);
            }
        }

        // median absolute deviation (MAD)
        if self.which.mad {
            let mad_value = if let Some(weighted_data) = sorted_weighted_data.as_ref() {
                // Use weighted MAD
                match typ {
                    TNull | TString => None,
                    _ => {
                        // Get the weighted median for MAD calculation
                        existing_median
                            .or_else(|| weighted_median(weighted_data, self.total_weight))
                            .and_then(|weighted_median_val| {
                                weighted_mad(weighted_data, self.total_weight, weighted_median_val)
                            })
                    },
                }
            } else {
                // Use unweighted MAD
                self.unsorted_stats.as_mut().and_then(|v| {
                    if let TNull | TString = typ {
                        None
                    } else {
                        v.mad(existing_median)
                    }
                })
            };

            if let Some(v) = mad_value {
                if typ == TDateTime || typ == TDate {
                    // like stddev, return MAD in days when the type is a date or datetime
                    record.push_field(&util::round_num(
                        v / MS_IN_DAY,
                        u32::max(round_places, DAY_DECIMAL_PLACES),
                    ));
                } else {
                    record.push_field(&util::round_num(v, round_places));
                }
            } else {
                record.push_field(EMPTY_STR);
            }
        }

        // quartiles
        // append it here to preserve legacy ordering of columns
        for field in &quartile_pieces {
            record.push_field(field);
        }

        // mode/modes/antimodes & cardinality
        // append it here to preserve legacy ordering of columns
        for field in &mc_pieces {
            record.push_field(field);
        }

        // Add percentiles after quartiles
        // Only add percentiles field if which.percentiles is true (matching header generation)
        if self.which.percentiles {
            match typ {
                TInteger | TFloat | TDate | TDateTime => {
                    // Parse percentile list, preserving both original labels and u8 values
                    let (percentile_labels, percentile_list): (Vec<String>, Vec<u8>) = self
                        .which
                        .percentile_list
                        .split(',')
                        .filter_map(|p: &str| {
                            fast_float2::parse(p.trim())
                                .ok()
                                .map(|p_val: f64| (p.trim().to_string(), p_val as u8))
                        })
                        .unzip();

                    let percentile_values =
                        if let Some(weighted_data) = sorted_weighted_data.as_ref() {
                            // Use weighted percentiles
                            weighted_percentiles(weighted_data, self.total_weight, &percentile_list)
                        } else if let Some(ref mut td) = self.tdigest.0 {
                            // Approx percentiles via t-digest. Empty-check up front so the
                            // semantics match the exact path: `Unsorted::custom_percentiles`
                            // returns None only when there's no data, never partial output.
                            // For a non-empty digest, `quantile()` returns `Some` for every
                            // valid rank in [0.0, 1.0], so the loop below cannot land in the
                            // partial state that would have made the old `all_some` check fire.
                            if td.is_empty() {
                                None
                            } else {
                                let mut out = Vec::with_capacity(percentile_list.len());
                                for p in &percentile_list {
                                    let rank = f64::from(*p) / 100.0;
                                    // `unwrap_or(f64::NAN)` is defensive only: a non-empty
                                    // digest should never return None here. If it ever does,
                                    // a NaN cell propagates through util::round_num cleanly.
                                    out.push(td.quantile(rank).unwrap_or(f64::NAN));
                                }
                                Some(out)
                            }
                        } else {
                            // Use unweighted exact percentiles
                            self.unsorted_stats
                                .as_mut()
                                .and_then(|v| v.custom_percentiles(&percentile_list))
                        };

                    if let Some(percentile_vals) = percentile_values {
                        let formatted_values = if typ == TDateTime || typ == TDate {
                            percentile_labels
                                .iter()
                                .zip(percentile_vals.iter())
                                .map(|(label, p)| {
                                    // Explicitly cast f64 to i64 for timestamp conversion
                                    #[allow(clippy::cast_possible_truncation)]
                                    let ts = p.round() as i64;
                                    let formatted_value = timestamp_ms_to_rfc3339(ts, typ);
                                    format!("{label}: {formatted_value}")
                                })
                                .collect::<Vec<_>>()
                        } else {
                            percentile_labels
                                .iter()
                                .zip(percentile_vals.iter())
                                .map(|(label, p)| {
                                    let formatted_value = util::round_num(*p, round_places);
                                    format!("{label}: {formatted_value}")
                                })
                                .collect::<Vec<_>>()
                        };
                        record.push_field(&formatted_values.join(stats_separator));
                    } else {
                        record.push_field(EMPTY_STR);
                    }
                },
                _ => record.push_field(EMPTY_STR),
            }
        }

        // zero_padded_numeric (last optional column, matching stats_headers ordering).
        // "true" when leading/padding zeros would be lost if the column were cast to a number;
        // empty otherwise. Qualifies when every non-null value is numeric-shaped (all-digit
        // integer, zero-padded decimal code, or plain float) AND the inferred type is String —
        // which from_sample() yields only when at least one value carries a leading zero (zip
        // codes, padded IDs, and zero-padded decimal codes like 007.1 / 05.10). Plain numeric
        // columns stay Integer/Float and are not flagged.
        if self.which.zero_padded_numeric {
            let is_zpn = self.zpn_has_value && !self.zpn_disqualified && typ == TString;
            record.push_field(if is_zpn { "true" } else { EMPTY_STR });
        }

        record
    }
}

impl Commute for Stats {
    #[inline]
    fn merge(&mut self, other: Stats) {
        self.typ.merge(other.typ);
        self.is_ascii &= other.is_ascii;
        // modes_dropped is sticky: if either chunk gave up on mode-tracking, the merged
        // result also gives up (we can't recover what was dropped). The actual clearing
        // of `modes` / `weighted_modes` happens AFTER the standard `Option::merge` calls
        // below so we don't have to special-case the merge plumbing — the post-merge
        // block at the bottom of this function nukes both fields when modes_dropped fires.
        self.modes_dropped |= other.modes_dropped;
        // zero_padded_numeric accumulators: disqualification is sticky across chunks, and a
        // qualifying value seen in any chunk counts.
        self.zpn_disqualified |= other.zpn_disqualified;
        self.zpn_has_value |= other.zpn_has_value;
        self.max_precision = self.max_precision.max(other.max_precision);
        self.which.merge(other.which);
        self.nullcount += other.nullcount;
        self.sum_stotlen = self.sum_stotlen.saturating_add(other.sum_stotlen);
        self.sum.merge(other.sum);
        self.modes.merge(other.modes);
        self.unsorted_stats.merge(other.unsorted_stats);
        // Merge t-digest engines: TDigestMut::merge(&other) is associative and does not
        // modify the input. Note: it is NOT chunk-count-invariant nor merge-order-invariant
        // — outputs may differ by ~1% across runs with different --jobs values, and
        // theoretically across runs with the same --jobs >= 2 if chunk completion order
        // differs. Tests pin --jobs 1, which routes through `sequential_stats` (no merge
        // at all — see the dispatch in run() around line 1456) and is therefore exactly
        // reproducible. Determinism for --jobs >= 2 is intentionally not guaranteed; the
        // --quantile-method help text documents the caveat for users.
        //
        // Skipped on big-endian targets: `TDigestSlot` is always `Default` (the inner
        // `Option<TDigestStub>` is always `None`) since Apache DataSketches is
        // unavailable, so there's no inner state to merge.
        #[cfg(not(target_endian = "big"))]
        match (&mut self.tdigest.0, other.tdigest.0) {
            (Some(s), Some(o)) => s.merge(&o),
            (slot @ None, Some(o)) => *slot = Some(o),
            _ => {},
        }
        // Merge HLL sketches via a transient HllUnion. Unlike t-digest, HLL union is
        // associative and order-invariant — the merged estimate is bit-identical
        // regardless of chunk completion order, so --jobs >= 2 is fully reproducible
        // for the cardinality column under --cardinality-method approx. The lg_k
        // here MUST match the HLL_LG_K used in `Stats::new`; reading from the same
        // constant prevents a silent precision downgrade if one site is bumped.
        //
        // Skipped on big-endian targets: `HllSlot` is always `Default` (the inner
        // `Option<HllSketchStub>` is always `None`) since Apache DataSketches is
        // unavailable, so there's no inner state to merge.
        #[cfg(not(target_endian = "big"))]
        match (&mut self.hll.0, other.hll.0) {
            (Some(s), Some(o)) => {
                let mut union = datasketches::hll::HllUnion::new(HLL_LG_K);
                union.update(s);
                union.update(&o);
                *s = union.to_sketch(datasketches::hll::HllType::Hll8);
            },
            (slot @ None, Some(o)) => *slot = Some(o),
            _ => {},
        }
        self.online.merge(other.online);
        self.online_len.merge(other.online_len);
        self.minmax.merge(other.minmax);

        // Merge weighted statistics
        if let Some(ref mut wos) = self.weighted_online {
            if let Some(ref other_wos) = other.weighted_online {
                wos.merge(other_wos);
            }
        } else if other.weighted_online.is_some() {
            self.weighted_online = other.weighted_online;
        }

        if let Some(ref mut wus) = self.weighted_unsorted_stats {
            if let Some(mut other_wus) = other.weighted_unsorted_stats {
                wus.append(&mut other_wus);
            }
        } else if other.weighted_unsorted_stats.is_some() {
            self.weighted_unsorted_stats = other.weighted_unsorted_stats;
        }

        // Merge weighted modes
        if let Some(ref mut wm) = self.weighted_modes {
            if let Some(other_wm) = other.weighted_modes {
                for (value, weight) in other_wm {
                    *wm.entry(value).or_insert(0.0) += weight;
                }
            }
        } else if other.weighted_modes.is_some() {
            self.weighted_modes = other.weighted_modes;
        }

        // --mode-cardinality-cap: if either chunk dropped its tracker, the post-merge
        // result must also be empty (we can't reconstruct the lost samples). Also re-check
        // the cap on the merged result, since two chunks below the cap can together
        // cross it.
        if self.modes_dropped {
            self.modes = None;
            self.weighted_modes = None;
        } else {
            let cap = self.which.mode_cardinality_cap;
            if cap > 0 {
                let over = self.modes.as_ref().is_some_and(|m| m.len() as u64 > cap)
                    || self
                        .weighted_modes
                        .as_ref()
                        .is_some_and(|m| m.len() as u64 > cap);
                if over {
                    self.modes = None;
                    self.weighted_modes = None;
                    self.modes_dropped = true;
                }
            }
        }

        self.total_weight += other.total_weight;
    }
}

#[allow(clippy::enum_variant_names)]
#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
enum FieldType {
    // The default - TNull, is the most specific type.
    // Type inference proceeds by assuming the most specific type and then
    // relaxing the type as counter-examples are found.
    #[default]
    TNull,
    TString,
    TFloat,
    TInteger,
    TDate,
    TDateTime,
}

/// Returns true when `sample` is a decimal float whose integer part is zero-padded, i.e. a
/// leading `0` immediately followed by another digit (`007.1`, `05.10`, `02.03`, `0601.10`).
/// This mirrors the leading-zero rule the integer path uses to keep zip codes as text: such a
/// value already parses as a number, so it is kept as a String to preserve the padding (ICD-9,
/// Dewey Decimal & Harmonized System classification codes). A single `0` before the dot is
/// intentionally NOT padding, so ordinary fractions like `0.5`/`0.25` are excluded. Requires
/// exactly one `.` with digits on both sides and nothing else, so exponents (`0e5`) and multi-dot
/// codes (`0601.10.00`, which aren't floats anyway) are rejected. An optional leading sign is
/// tolerated, though codes are typically unsigned. Pure trailing-zero codes (`7.10`) are out of
/// scope by design — they're indistinguishable from rounded measurements without the original
/// string.
#[inline]
fn is_zero_padded_float(sample: &[u8]) -> bool {
    let b = match sample.first() {
        Some(b'+' | b'-') => &sample[1..],
        _ => sample,
    };
    // integer part must be '0' followed by another digit
    match b {
        [b'0', d, ..] if d.is_ascii_digit() => {},
        _ => return false,
    }
    let mut seen_dot = false;
    let mut frac_len = 0_usize;
    for &c in b {
        match c {
            b'.' if !seen_dot => seen_dot = true,
            b'.' => return false, // a second dot: not a float
            _ if c.is_ascii_digit() => {
                if seen_dot {
                    frac_len += 1;
                }
            },
            _ => return false, // any other byte disqualifies
        }
    }
    seen_dot && frac_len > 0
}

impl FieldType {
    /// infer data type from a given sample & current type inference
    /// `infer_dates` signals if date inference should be attempted
    /// returns the inferred type and if `infer_dates` is true,
    /// the date in ms since the epoch if the type is a date or datetime
    /// otherwise, 0
    /// it also returns the float value if the sample is a number
    #[allow(clippy::inline_always)]
    #[inline(always)]
    pub fn from_sample(
        infer_dates: bool,
        prefer_dmy: bool,
        sample: &[u8],
        current_type: FieldType,
    ) -> (FieldType, i64, f64) {
        // faster than sample.len() == 0 or sample.is_empty() per microbenchmarks
        if b"" == sample {
            return (FieldType::TNull, 0, 0.0);
        }

        // no need to do type checking if current_type is already a String
        if current_type == FieldType::TString {
            return (FieldType::TString, 0, 0.0);
        }

        // an int can be a float, but once we've seen a float, we can't go back to an int
        if current_type != FieldType::TFloat
            && let Ok(samp_int) = atoi_simd::parse::<i64, false, false>(sample)
        {
            // Check for integer, with leading zero check for strings like zip codes
            // safety: we know sample is not null as we checked earlier
            if samp_int == 0 || unsafe { *sample.get_unchecked(0) != b'0' } {
                // note that we still return samp_int as f64 even if it's an integer
                // as the qsv-stats crate expects a float value for integer fields
                #[allow(clippy::cast_precision_loss)]
                return (FieldType::TInteger, samp_int, samp_int as f64);
            }
            // If starts with '0' and a valid integer != 0, it's a string with a leading zero
            return (FieldType::TString, 0, 0.0);
        }

        // Check for float
        // we use fast_float2 as it doesn't need to validate the sample as UTF-8 first
        if let Ok(float_sample) = fast_float2::parse::<f64, &[u8]>(sample) {
            // Zero-padded floats (007.1, 05.10 — ICD-9 / Dewey / HS codes) are kept as String
            // to preserve their leading zeros, mirroring the zero-padded-integer rule above (a
            // 0-then-digit integer part is padding; a plain 0.5 / 7.1 is a real number). The
            // first-byte check inside is_zero_padded_float() makes the common (non-padded) case
            // a couple of byte comparisons.
            if is_zero_padded_float(sample) {
                return (FieldType::TString, 0, 0.0);
            }
            return (FieldType::TFloat, 0, float_sample);
        }

        // Only attempt UTF-8 validation and date parsing if infer_dates is true
        if !infer_dates {
            return (FieldType::TString, 0, 0.0);
        }

        // Check if valid UTF-8 first, return early if not.
        // On real data the Ok arm dominates; mark the Err arm cold so LLVM keeps the
        // hot date-parse path contiguous in the instruction cache.
        if let Ok(s) = simdutf8::basic::from_utf8(sample) {
            // Try date parsing
            if let Ok(parsed_date) = parse_with_preference(s, prefer_dmy) {
                let ts_val = parsed_date.timestamp_millis();
                return if ts_val % MS_IN_DAY_INT == 0 {
                    // if the date is a whole number of days, return as a date
                    (FieldType::TDate, ts_val, 0.0)
                } else {
                    // otherwise, return as a datetime
                    (FieldType::TDateTime, ts_val, 0.0)
                };
            }
        } else {
            cold_path();
            // If not valid UTF-8, it's a binary string, return as TString
            return (FieldType::TString, 0, 0.0);
        }

        // Default to TString if none of the above conditions are met
        (FieldType::TString, 0, 0.0)
    }
}

impl Commute for FieldType {
    #[inline]
    #[allow(clippy::match_same_arms)]
    // we allow match_same_arms because we want are optimizing for
    // performance and not readability, as match arms are evaluated in order
    // so we want to put the most common cases first
    fn merge(&mut self, other: FieldType) {
        *self = match (*self, other) {
            (TString, TString) => TString,
            (TFloat, TFloat) => TFloat,
            (TInteger, TInteger) => TInteger,
            // Null does not impact the type.
            (TNull, any) | (any, TNull) => any,
            // Integers can degrade to floats.
            (TFloat, TInteger) | (TInteger, TFloat) => TFloat,
            // date data types
            (TDate, TDate) => TDate,
            (TDateTime | TDate, TDateTime) | (TDateTime, TDate) => TDateTime,
            // anything else is a String
            (_, _) => TString,
        };
    }
}

const NULL_FTYPE: &str = "NULL";
const STRING_FTYPE: &str = "String";
const FLOAT_FTYPE: &str = "Float";
const INTEGER_FTYPE: &str = "Integer";
const DATE_FTYPE: &str = "Date";
const DATETIME_FTYPE: &str = "DateTime";

impl fmt::Display for FieldType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TNull => write!(f, "{NULL_FTYPE}"),
            TString => write!(f, "{STRING_FTYPE}"),
            TFloat => write!(f, "{FLOAT_FTYPE}"),
            TInteger => write!(f, "{INTEGER_FTYPE}"),
            TDate => write!(f, "{DATE_FTYPE}"),
            TDateTime => write!(f, "{DATETIME_FTYPE}"),
        }
    }
}

impl fmt::Debug for FieldType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TNull => write!(f, "{NULL_FTYPE}"),
            TString => write!(f, "{STRING_FTYPE}"),
            TFloat => write!(f, "{FLOAT_FTYPE}"),
            TInteger => write!(f, "{INTEGER_FTYPE}"),
            TDate => write!(f, "{DATE_FTYPE}"),
            TDateTime => write!(f, "{DATETIME_FTYPE}"),
        }
    }
}

impl FieldType {
    pub const fn as_str(&self) -> &str {
        match self {
            TNull => NULL_FTYPE,
            TString => STRING_FTYPE,
            TFloat => FLOAT_FTYPE,
            TInteger => INTEGER_FTYPE,
            TDate => DATE_FTYPE,
            TDateTime => DATETIME_FTYPE,
        }
    }
}

/// `TypedSum` keeps a rolling sum of the data seen.
/// It sums integers until it sees a float, at which point it sums floats.
/// It also counts the total length of strings.
#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Clone, Default, Serialize, Deserialize, PartialEq)]
struct TypedSum {
    float:   Option<f64>,
    integer: i64,
    stotlen: u64, // sum of the total length of strings
}

impl TypedSum {
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn add_with_parsed(&mut self, typ: FieldType, sample: &[u8], float_val: f64, int_val: i64) {
        // Always track byte-length sum — the column type can widen to TString
        // mid-stream (e.g. --infer-dates with dates + a non-date value, or
        // Integer/Float/Date/DateTime → String), and sum_length / avg_length
        // must include earlier rows that were processed under a more specific
        // type. show() still gates emission on the final typ == TString, so
        // pure-typed columns incur the tracking cost but no output change.
        //
        // Caller invariant: Stats::add only calls this inside `if b"" !=
        // sample`, and FieldType::from_sample returns TNull only for empty
        // samples, so we never get here with `typ == TNull` AND a non-empty
        // sample — no need to special-case TNull below.
        self.stotlen = self.stotlen.saturating_add(sample.len() as u64);

        #[allow(clippy::cast_precision_loss)]
        match typ {
            TInteger => {
                if let Some(ref mut f) = self.float {
                    *f += float_val;
                } else {
                    self.integer = self.integer.saturating_add(int_val);
                }
            },
            TFloat => {
                if let Some(ref mut f) = self.float {
                    *f += float_val;
                } else {
                    self.float = Some((self.integer as f64) + float_val);
                }
            },
            // TString length captured above; TNull / TDate / TDateTime have no
            // sum, so no per-arm work needed.
            _ => {},
        }
    }

    fn show(&self, typ: FieldType) -> Option<(u64, String)> {
        match typ {
            TNull | TDate | TDateTime => None,
            TInteger => {
                match self.integer {
                    // with saturating_add, if this is equal to i64::MAX or i64::MIN
                    // we overflowed/underflowed
                    i64::MAX => Some((self.stotlen, OVERFLOW_STRING.to_string())),
                    i64::MIN => Some((self.stotlen, UNDERFLOW_STRING.to_string())),
                    _ => Some((
                        self.stotlen,
                        itoa::Buffer::new().format(self.integer).to_owned(),
                    )),
                }
            },
            TFloat => Some((
                self.stotlen,
                zmij::Buffer::new()
                    .format(self.float.unwrap_or(0.0))
                    .to_owned(),
            )),
            TString => Some((self.stotlen, String::new())),
        }
    }
}

impl Commute for TypedSum {
    #[inline]
    fn merge(&mut self, other: TypedSum) {
        #[allow(clippy::cast_precision_loss)]
        match (self.float, other.float) {
            (Some(f1), Some(f2)) => self.float = Some(f1 + f2),
            (Some(f1), None) => self.float = Some(f1 + (other.integer as f64)),
            (None, Some(f2)) => self.float = Some((self.integer as f64) + f2),
            (None, None) => self.integer = self.integer.saturating_add(other.integer),
        }
        self.stotlen = self.stotlen.saturating_add(other.stotlen);
    }
}

/// `TypedMinMax` keeps track of `minimum/maximum/range/sort_order` values for each possible type
/// where `min/max/range/sort_order` makes sense.
#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Clone, Default, Serialize, Deserialize, PartialEq)]
struct TypedMinMax {
    floats:   MinMax<f64>,
    integers: MinMax<i64>,
    dates:    MinMax<i64>,
    strings:  MinMax<Vec<u8>>,
    str_len:  MinMax<usize>,
}

impl TypedMinMax {
    /// Add a sample with pre-parsed values to avoid redundant parsing
    #[inline]
    fn add_with_parsed(&mut self, typ: FieldType, sample: &[u8], float_val: f64, int_val: i64) {
        let sample_len = sample.len();
        if sample_len == 0 {
            self.str_len.add(0);
            return;
        }

        // Always track string length and lexical min/max — the column type can
        // widen to TString mid-stream (e.g. --infer-dates with dates + a
        // non-date value, or Integer/Float/Date/DateTime → String), and
        // min_length / max_length / lex min / lex max / sort_order / sortiness
        // must include earlier rows that were processed under a more specific
        // type. show() still gates emission on the final typ == TString, so
        // pure-typed columns are unaffected in output.
        self.str_len.add(sample_len);
        self.strings.add_bytes(sample);

        match typ {
            TInteger => {
                self.integers.add(int_val);
                self.floats.add(float_val);
            },
            TFloat => {
                self.floats.add(float_val);
            },
            // TString length & lex captured above.
            TString | TNull => {},
            // it must be a TDate or TDateTime
            // we use "_" here instead of "TDate | TDateTime" for the match to avoid
            // the overhead of matching on the OR value, however minor
            _ => {
                if int_val != 0 {
                    self.dates.add(int_val);
                }
            },
        }
    }

    fn len_range(&self) -> Option<(String, String)> {
        if let (Some(min), Some(max)) = (self.str_len.min(), self.str_len.max()) {
            Some((
                itoa::Buffer::new().format(*min).to_owned(),
                itoa::Buffer::new().format(*max).to_owned(),
            ))
        } else {
            None
        }
    }

    #[inline]
    fn show(
        &self,
        typ: FieldType,
        round_places: u32,
        visualize_ws: bool,
    ) -> Option<(String, String, String, String, String)> {
        match typ {
            TNull => None,
            TString => {
                if let (Some(min), Some(max), sort_order, sortiness) = (
                    self.strings.min(),
                    self.strings.max(),
                    self.strings.sort_order(),
                    self.strings.sortiness(),
                ) {
                    let min_str = util::bytes_to_cow_str(min).into_owned();
                    let max_str = util::bytes_to_cow_str(max).into_owned();

                    let max_length = STATS_STRING_MAX_LENGTH.get_or_init(|| {
                        std::env::var("QSV_STATS_STRING_MAX_LENGTH")
                            .ok()
                            .and_then(|s| {
                                atoi_simd::parse::<usize, false, false>(s.as_bytes()).ok()
                            })
                    });

                    let (min_str, max_str) = if let Some(max_len) = *max_length {
                        (
                            if min_str.len() > max_len {
                                format!("{}...", &min_str[..max_len])
                            } else {
                                min_str
                            },
                            if max_str.len() > max_len {
                                format!("{}...", &max_str[..max_len])
                            } else {
                                max_str
                            },
                        )
                    } else {
                        (min_str, max_str)
                    };

                    let (min_display, max_display) = if visualize_ws {
                        (
                            util::visualize_whitespace(&min_str),
                            util::visualize_whitespace(&max_str),
                        )
                    } else {
                        (min_str, max_str)
                    };
                    Some((
                        min_display,
                        max_display,
                        String::new(),
                        sort_order.to_string(),
                        util::round_num(sortiness, round_places),
                    ))
                } else {
                    None
                }
            },
            TInteger => {
                if let (Some(min), Some(max), sort_order, sortiness) = (
                    self.integers.min(),
                    self.integers.max(),
                    self.integers.sort_order(),
                    self.integers.sortiness(),
                ) {
                    Some((
                        itoa::Buffer::new().format(*min).to_owned(),
                        itoa::Buffer::new().format(*max).to_owned(),
                        itoa::Buffer::new().format(*max - *min).to_owned(),
                        sort_order.to_string(),
                        util::round_num(sortiness, round_places),
                    ))
                } else {
                    None
                }
            },
            TFloat => {
                if let (Some(min), Some(max), sort_order, sortiness) = (
                    self.floats.min(),
                    self.floats.max(),
                    self.floats.sort_order(),
                    self.floats.sortiness(),
                ) {
                    Some((
                        zmij::Buffer::new().format(*min).to_owned(),
                        zmij::Buffer::new().format(*max).to_owned(),
                        util::round_num(*max - *min, round_places),
                        sort_order.to_string(),
                        util::round_num(sortiness, round_places),
                    ))
                } else {
                    None
                }
            },
            TDateTime | TDate => {
                if let (Some(min), Some(max), sort_order, sortiness) = (
                    self.dates.min(),
                    self.dates.max(),
                    self.dates.sort_order(),
                    self.dates.sortiness(),
                ) {
                    Some((
                        timestamp_ms_to_rfc3339(*min, typ),
                        timestamp_ms_to_rfc3339(*max, typ),
                        // return in days, not timestamp in milliseconds
                        #[allow(clippy::cast_precision_loss)]
                        util::round_num(
                            (*max - *min) as f64 / MS_IN_DAY,
                            u32::max(round_places, 5),
                        ),
                        sort_order.to_string(),
                        util::round_num(sortiness, round_places),
                    ))
                } else {
                    None
                }
            },
        }
    }
}

impl Commute for TypedMinMax {
    #[inline]
    fn merge(&mut self, other: TypedMinMax) {
        self.floats.merge(other.floats);
        self.integers.merge(other.integers);
        self.dates.merge(other.dates);
        self.strings.merge(other.strings);
        self.str_len.merge(other.str_len);
    }
}
