static USAGE: &str = r#"
Compute summary statistics & infers data types for each column in a CSV.

> IMPORTANT: `stats` is heavily optimized for speed. It ASSUMES the CSV is well-formed & UTF-8 encoded.
> This allows it to employ numerous performance optimizations (skip repetitive UTF-8 validation, skip
> bounds checks, cache results, etc.) that may result in undefined behavior if the CSV is not well-formed.
> All these optimizations are GUARANTEED to work with well-formed CSVs.
> If you encounter problems generating stats, use `qsv validate` FIRST to confirm the CSV is valid.

Summary stats include sum, min/max/range, sort order/sortiness, min/max/sum/avg/stddev/variance/cv length,
mean, standard error of the mean (SEM), geometric mean, harmonic mean, stddev, variance, coefficient of
variation (CV), nullcount, max_precision, sparsity, Median Absolute Deviation (MAD), quartiles,
interquartile range (IQR), lower/upper fences, skewness, median, cardinality/uniqueness ratio,
mode/s & "antimode/s" & percentiles.

Note that some stats require loading the entire file into memory, so they must be enabled explicitly. 

By default, the following "streaming" statistics are reported for *every* column:
  sum, min/max/range values, sort order/sortiness, min/max/sum/avg/stddev/variance/cv length, mean, sem,
  geometric_mean, harmonic_mean,stddev, variance, cv, nullcount, max_precision & sparsity.

The default set of statistics corresponds to ones that can be computed efficiently on a stream of data
(i.e., constant memory) and works with arbitrarily large CSVs.

The following additional "non-streaming" statistics require loading the entire file into memory:
cardinality/uniqueness ratio, modes/antimodes, median, MAD, quartiles and its related measures
(q1, q2, q3, IQR, lower/upper fences & skewness) and percentiles.

When computing "non-streaming" statistics, an Out-Of-Memory (OOM) heuristic check is done.
If the file is larger than the available memory minus a headroom buffer of 20% (which can be
adjusted using the QSV_FREEMEMORY_HEADROOM_PCT environment variable), processing will be
preemptively prevented.

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
first with 'qsv index' to enable multithreading. With an index, the file is split into equal
chunks and each chunk is processed in parallel. The number of chunks is determined by the
number of logical CPUs detected. You can override this by setting the --jobs option.

As stats is a central command in qsv, and can be expensive to compute, `stats` caches results
in <FILESTEM>.stats.csv & if the --stats-json option is used, <FILESTEM>.stats.csv.data.jsonl
(e.g., qsv stats nyc311.csv will create nyc311.stats.csv & nyc311.stats.csv.data.jsonl).
The arguments used to generate the cached stats are saved in <FILESTEM>.stats.csv.jsonl.

If stats have already been computed for the input file with similar arguments and the file
hasn't changed, the stats will be loaded from the cache instead of recomputing it.

These cached stats are also used by other qsv commands (currently `describegpt`, `frequency`,
`joinp`, `pivotp`, `schema`, `sqlp` & `tojsonl`) to work smarter & faster.
If the cached stats are not current (i.e., the input file is newer than the cached stats),
the cached stats will be ignored and recomputed. For example, see the "boston311" test files in 
https://github.com/dathere/qsv/blob/4529d51273218347fef6aca15ac24e22b85b2ec4/tests/test_stats.rs#L608.

Examples:

Compute "streaming" statistics for the "nyc311.csv" file:
   $ qsv stats nyc311.csv

Compute all statistics for the "nyc311.csv" file:
    $ qsv stats --everything nyc311.csv
    $ qsv stats -E nyc311.csv

Compute all statistics for "nyc311.csv", inferring dates using default date column name patterns:
    $ qsv stats -E --infer-dates nyc311.csv

Compute all statistics for "nyc311.csv", inferring dates only for columns with "_date" & "_dte"
in the column names:
    $ qsv stats -E --infer-dates --dates-whitelist _date,_dte nyc311.csv

In addition, also infer boolean data types for "nyc311.csv" file:
    $ qsv stats -E --infer-dates --dates-whitelist _date --infer-boolean nyc311.csv

In addition to basic "streaming" stats, also compute cardinality for the "nyc311.csv" file:
    $ qsv stats --cardinality nyc311.csv

Prefer DMY format when inferring dates for the "nyc311.csv" file:
    $ qsv stats -E --infer-dates --prefer-dmy nyc311.csv    

Infer data types only for the "nyc311.csv" file:
    $ qsv stats --typesonly nyc311.csv

Infer data types only, including boolean and date types for the "nyc311.csv" file:
    $ qsv stats --typesonly --infer-boolean --infer-dates nyc311.csv

Automatically create an index for the "nyc311.csv" file to enable multithreading
if it's larger than 5MB and there is no existing index file:
    $ qsv stats -E --cache-threshold -5000000 nyc311.csv

Auto-create a TEMPORARY index for the "nyc311.csv" file to enable multithreading
if it's larger than 5MB and delete the index and the stats cache file after the stats run:
    $ qsv stats -E --cache-threshold -5000005 nyc311.csv

Prompt for CSV/TSV/TAB file to compute stats for:
    $ qsv prompt -F tsv,csv,tab | qsv stats -E | qsv table

Prompt for a file to save the stats to in the ~/Documents directory:
    $ qsv stats -E nyc311.csv | qsv prompt -d ~/Documents --fd-output

Prompt for both INPUT and OUTPUT files in the ~/Documents dir with custom prompts:
    $ qsv prompt -m 'Select a CSV file to summarize' -d ~/Documents -F csv | \
      qsv stats -E --infer-dates | \
      qsv prompt -m 'Save summary to...' -d ~/Documents --fd-output --save-fname summarystats.csv

For more examples, see https://github.com/dathere/qsv/tree/master/resources/test
For more info, see https://github.com/dathere/qsv/wiki/Supplemental#stats-command-output-explanation

Usage:
    qsv stats [options] [<input>]
    qsv stats --help

stats options:
    -s, --select <arg>        Select a subset of columns to compute stats for.
                              See 'qsv select --help' for the format details.
                              This is provided here because piping 'qsv select'
                              into 'qsv stats' will prevent the use of indexing.
    -E, --everything          Compute all statistics available EXCEPT --dataset-stats.
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
                              [default: 5,10,40,60,90,95]

    --round <decimal_places>  Round statistics to <decimal_places>. Rounding is done following
                              Midpoint Nearest Even (aka "Bankers Rounding") rule.
                              https://docs.rs/rust_decimal/latest/rust_decimal/enum.RoundingStrategy.html
                              If set to the sentinel value 9999, no rounding is done.
                              For dates - range, stddev & IQR are always at least 5 decimal places as
                              they are reported in days, and 5 places gives us millisecond precision.
                              [default: 4]
    --nulls                   Include NULLs in the population size for computing
                              mean and standard deviation.

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
                              Set to "all" to inspect ALL fields for
                              date/datetime types. Ignored if --infer-dates is false.

                              Note that false positive date matches WILL most likely occur
                              when using "all" as unix epoch timestamps are just numbers.
                              Be sure to only use "all" if you know ALL the columns you're
                              inspecting are dates, boolean or string fields.

                              To avoid false positives, preprocess the file first 
                              with the `datefmt` command to convert unix epoch timestamp
                              columns to RFC3339 format.
                              [default: date,time,due,open,close,created]
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
                                - when greater than 1, the threshold in milliseconds before caching
                                  stats results. If a stats run takes longer than this threshold,
                                  the stats results will be cached.
                                - 0 to suppress caching. 
                                - 1 to force caching.
                                - a negative number to automatically create an index when
                                  the input file size is greater than abs(arg) in bytes.
                                  If the negative number ends with 5, it will delete the index
                                  file and the stats cache file after the stats run. Otherwise,
                                  the index file and the cache files are kept.
                              [default: 5000]
    --vis-whitespace          Visualize whitespace characters in the output.
                              See https://github.com/dathere/qsv/wiki/Supplemental#whitespace-markers
                              for the list of whitespace markers.
    --dataset-stats           Compute dataset statistics (e.g. row count, column count, file size and
                              fingerprint hash) and add them as additional rows to the output, with
                              the qsv__ prefix and an additional qsv__value column.
                              The --everything option DOES NOT enable this option.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -n, --no-headers       When set, the first row will NOT be interpreted
                           as column names. i.e., They will be included
                           in statistics.
    -d, --delimiter <arg>  The field delimiter for READING CSV data.
                           Must be a single character. (default: ,)
    --memcheck             Check if there is enough memory to load the entire
                           CSV into memory using CONSERVATIVE heuristics.
                           This option is ignored when computing default, streaming
                           statistics, as it is not needed.
"#;

/*
DEVELOPER NOTE: stats is heavily optimized and is a central command in qsv.

It was the primary reason I created the qsv fork as I needed to do GUARANTEED data type
inferencing & to compile smart Data Dictionaries in the most performant way possible
for Datapusher+ (https://github.com/dathere/datapusher-plus).

It underpins the `schema` and `validate` commands - enabling the automatic creation of
a JSON Schema based on a CSV's summary statistics; and use the generated JSON Schema
to quickly validate complex CSVs hundreds of thousands of records/sec.

It's type inferences are also used by the "smart" commands (see
https://github.com/dathere/qsv/blob/master/docs/PERFORMANCE.md#stats-cache)
to make them work smarter & faster.

To safeguard against undefined behavior, `stats` is the most extensively tested command,
with ~520 tests. It also employs numerous performance optimizations (skip repetitive UTF-8
validation, skip bounds checks, cache results, etc.) that may result in undefined behavior
if the CSV is not well-formed. See "safety:" comments in the code for more details.
*/

use std::{
    fmt, fs,
    io::{self, BufRead, Seek, Write},
    iter::repeat_n,
    path::{Path, PathBuf},
    str,
    sync::OnceLock,
};

use crossbeam_channel;
use itertools::Itertools;
use phf::phf_map;
use qsv_dateparser::parse_with_preference;
use serde::{Deserialize, Serialize};
use simd_json::{OwnedValue, prelude::ValueAsScalar};
use smallvec::SmallVec;
use stats::{Commute, MinMax, OnlineStats, Unsorted, merge_all};
use tempfile::NamedTempFile;
use threadpool::ThreadPool;

use self::FieldType::{TDate, TDateTime, TFloat, TInteger, TNull, TString};
use crate::{
    CliResult,
    config::{Config, Delimiter, get_delim_by_extension},
    select::{SelectColumns, Selection},
    util,
};

#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Clone, Deserialize)]
pub struct Args {
    pub arg_input:             Option<String>,
    pub flag_select:           SelectColumns,
    pub flag_everything:       bool,
    pub flag_typesonly:        bool,
    pub flag_infer_boolean:    bool,
    pub flag_boolean_patterns: String,
    pub flag_mode:             bool,
    pub flag_cardinality:      bool,
    pub flag_median:           bool,
    pub flag_mad:              bool,
    pub flag_quartiles:        bool,
    pub flag_percentiles:      bool,
    pub flag_percentile_list:  String,
    pub flag_round:            u32,
    pub flag_nulls:            bool,
    pub flag_infer_dates:      bool,
    pub flag_dates_whitelist:  String,
    pub flag_prefer_dmy:       bool,
    pub flag_force:            bool,
    pub flag_jobs:             Option<usize>,
    pub flag_stats_jsonl:      bool,
    pub flag_cache_threshold:  isize,
    pub flag_output:           Option<String>,
    pub flag_no_headers:       bool,
    pub flag_delimiter:        Option<Delimiter>,
    pub flag_memcheck:         bool,
    pub flag_vis_whitespace:   bool,
    pub flag_dataset_stats:    bool,
}

// this struct is used to serialize/deserialize the stats to
// the "".stats.csv.json" file which we check to see
// if we can skip recomputing stats.
#[derive(Clone, Serialize, Deserialize, PartialEq, Default)]
struct StatsArgs {
    arg_input:            String,
    flag_select:          String,
    flag_everything:      bool,
    flag_typesonly:       bool,
    flag_infer_boolean:   bool,
    flag_mode:            bool,
    flag_cardinality:     bool,
    flag_median:          bool,
    flag_mad:             bool,
    flag_quartiles:       bool,
    flag_percentiles:     bool,
    flag_percentile_list: String,
    flag_round:           u32,
    flag_nulls:           bool,
    flag_infer_dates:     bool,
    flag_dates_whitelist: String,
    flag_prefer_dmy:      bool,
    flag_no_headers:      bool,
    flag_delimiter:       String,
    flag_output_snappy:   bool,
    canonical_input_path: String,
    canonical_stats_path: String,
    record_count:         u64,
    date_generated:       String,
    compute_duration_ms:  u64,
    qsv_version:          String,
}

impl StatsArgs {
    // this is for deserializing the stats.csv.jsonl file
    fn from_owned_value(value: &OwnedValue) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            arg_input:            value["arg_input"].as_str().unwrap_or_default().to_string(),
            flag_select:          value["flag_select"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
            flag_everything:      value["flag_everything"].as_bool().unwrap_or_default(),
            flag_typesonly:       value["flag_typesonly"].as_bool().unwrap_or_default(),
            flag_infer_boolean:   value["flag_infer_boolean"].as_bool().unwrap_or_default(),
            flag_mode:            value["flag_mode"].as_bool().unwrap_or_default(),
            flag_cardinality:     value["flag_cardinality"].as_bool().unwrap_or_default(),
            flag_median:          value["flag_median"].as_bool().unwrap_or_default(),
            flag_mad:             value["flag_mad"].as_bool().unwrap_or_default(),
            flag_quartiles:       value["flag_quartiles"].as_bool().unwrap_or_default(),
            flag_percentiles:     value["flag_percentiles"].as_bool().unwrap_or_default(),
            flag_percentile_list: value["flag_percentile_list"]
                .as_str()
                .unwrap_or("5,10,40,60,90,95")
                .to_string(),
            flag_round:           value["flag_round"].as_u64().unwrap_or_default() as u32,
            flag_nulls:           value["flag_nulls"].as_bool().unwrap_or_default(),
            flag_infer_dates:     value["flag_infer_dates"].as_bool().unwrap_or_default(),
            flag_dates_whitelist: value["flag_dates_whitelist"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
            flag_prefer_dmy:      value["flag_prefer_dmy"].as_bool().unwrap_or_default(),
            flag_no_headers:      value["flag_no_headers"].as_bool().unwrap_or_default(),
            flag_delimiter:       value["flag_delimiter"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
            flag_output_snappy:   value["flag_output_snappy"].as_bool().unwrap_or_default(),
            canonical_input_path: value["canonical_input_path"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
            canonical_stats_path: value["canonical_stats_path"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
            record_count:         value["record_count"].as_u64().unwrap_or_default(),
            date_generated:       value["date_generated"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
            compute_duration_ms:  value["compute_duration_ms"].as_u64().unwrap_or_default(),
            qsv_version:          value["qsv_version"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
        })
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Default, Debug)]
pub struct StatsData {
    pub field:                String,
    // type is a reserved keyword in Rust
    // so we escape it as r#type
    // we need to do this for serde to work
    pub r#type:               String,
    #[serde(default)]
    pub is_ascii:             bool,
    pub sum:                  Option<f64>,
    pub min:                  Option<String>,
    pub max:                  Option<String>,
    pub range:                Option<f64>,
    pub sort_order:           Option<String>,
    pub min_length:           Option<usize>,
    pub max_length:           Option<usize>,
    pub sum_length:           Option<usize>,
    pub avg_length:           Option<f64>,
    pub stddev_length:        Option<f64>,
    pub variance_length:      Option<f64>,
    pub cv_length:            Option<f64>,
    pub mean:                 Option<f64>,
    pub sem:                  Option<f64>,
    pub stddev:               Option<f64>,
    pub variance:             Option<f64>,
    pub cv:                   Option<f64>,
    pub nullcount:            u64,
    pub max_precision:        Option<u32>,
    pub sparsity:             Option<f64>,
    pub mad:                  Option<f64>,
    pub lower_outer_fence:    Option<f64>,
    pub lower_inner_fence:    Option<f64>,
    pub q1:                   Option<f64>,
    pub q2_median:            Option<f64>,
    pub q3:                   Option<f64>,
    pub iqr:                  Option<f64>,
    pub upper_inner_fence:    Option<f64>,
    pub upper_outer_fence:    Option<f64>,
    pub skewness:             Option<f64>,
    pub cardinality:          u64,
    pub uniqueness_ratio:     Option<f64>,
    pub mode:                 Option<String>,
    pub mode_count:           Option<u64>,
    pub mode_occurrences:     Option<u64>,
    pub antimode:             Option<String>,
    pub antimode_count:       Option<u64>,
    pub antimode_occurrences: Option<u64>,
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
    "qsv__value" => JsonTypes::Int,
};

static INFER_DATE_FLAGS: OnceLock<SmallVec<[bool; 50]>> = OnceLock::new();
static RECORD_COUNT: OnceLock<u64> = OnceLock::new();
static ANTIMODES_LEN: OnceLock<usize> = OnceLock::new();

// standard overflow and underflow strings
// for sum, sum_length and avg_length
const OVERFLOW_STRING: &str = "*OVERFLOW*";
const UNDERFLOW_STRING: &str = "*UNDERFLOW*";

// number of milliseconds per day
const MS_IN_DAY: f64 = 86_400_000.0;
const MS_IN_DAY_INT: i64 = 86_400_000;
// number of decimal places when rounding days
// 5 decimal places give us millisecond precision
const DAY_DECIMAL_PLACES: u32 = 5;

// maximum number of output columns
const MAX_STAT_COLUMNS: usize = 44;

// the first N columns are fingerprint hash columns
const FINGERPRINT_HASH_COLUMNS: usize = 26;

// maximum number of antimodes to display
const MAX_ANTIMODES: usize = 10;
// default length of antimode string before truncating and appending "..."
const DEFAULT_ANTIMODES_LEN: usize = 100;

// the default separator we use for stats that have multiple values
// in one column, i.e. antimodes/modes & percentiles
pub const DEFAULT_STATS_SEPARATOR: &str = "|";

static BOOLEAN_PATTERNS: OnceLock<Vec<BooleanPattern>> = OnceLock::new();
#[derive(Clone, Debug)]
/// Represents a pattern for boolean value inference in CSV data.
///
/// This struct defines patterns that can be used to identify boolean values in CSV columns.
/// It supports both exact matches and prefix matching with wildcards for flexible boolean
/// detection during CSV statistics computation.
///
/// # Fields
///
/// * `true_pattern` - The pattern that identifies `true` values (case-insensitive)
/// * `false_pattern` - The pattern that identifies `false` values (case-insensitive)
///
/// # Pattern Matching
///
/// Patterns support two types of matching:
/// * **Exact match**: The value must exactly match the pattern (case-insensitive)
/// * **Prefix match**: If the pattern ends with `*`, it matches any value that starts with the
///   prefix (e.g., `"yes*"` matches `"yes"`, `"yes please"`, `"YES"`, etc.)
struct BooleanPattern {
    true_pattern:  String,
    false_pattern: String,
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
    /// 3. **Priority**: Exact matches are checked before prefix matches for better performance
    fn matches(&self, value: &str) -> Option<bool> {
        let value_lower = value.to_lowercase();

        // Check for exact match first
        if value_lower == self.true_pattern {
            return Some(true);
        } else if value_lower == self.false_pattern {
            return Some(false);
        }

        // Check for prefix match if pattern ends with "*"
        if self.true_pattern.ends_with('*') {
            let prefix = &self.true_pattern[..self.true_pattern.len() - 1];
            if value_lower.starts_with(prefix) {
                return Some(true);
            }
        }

        if self.false_pattern.ends_with('*') {
            let prefix = &self.false_pattern[..self.false_pattern.len() - 1];
            if value_lower.starts_with(prefix) {
                return Some(false);
            }
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

        if true_pattern.is_empty() || false_pattern.is_empty() {
            return fail_incorrectusage_clierror!("Invalid boolean pattern: {pair}");
        }

        patterns.push(BooleanPattern {
            true_pattern,
            false_pattern,
        });
    }
    if patterns.is_empty() {
        return fail_incorrectusage_clierror!("Boolean patterns must have at least one pattern");
    }
    Ok(patterns)
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
/// 3. **Environment Variables**: Checks for QSV_PREFER_DMY environment variable
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
    if args.flag_typesonly {
        args.flag_everything = false;
        args.flag_mode = false;
        args.flag_cardinality = false;
        args.flag_median = false;
        args.flag_quartiles = false;
        args.flag_mad = false;
    }

    // inferring boolean requires inferring cardinality
    if args.flag_infer_boolean {
        if !args.flag_cardinality {
            args.flag_cardinality = true;
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
        arg_input:            format!("{:?}", args.arg_input),
        flag_select:          format!("{:?}", args.flag_select),
        flag_everything:      args.flag_everything,
        flag_typesonly:       args.flag_typesonly,
        flag_infer_boolean:   args.flag_infer_boolean,
        flag_mode:            args.flag_mode,
        flag_cardinality:     args.flag_cardinality,
        flag_median:          args.flag_median,
        flag_mad:             args.flag_mad,
        flag_quartiles:       args.flag_quartiles,
        flag_percentiles:     args.flag_percentiles,
        flag_percentile_list: args.flag_percentile_list.clone(),
        flag_round:           args.flag_round,
        flag_nulls:           args.flag_nulls,
        flag_infer_dates:     args.flag_infer_dates,
        flag_dates_whitelist: args.flag_dates_whitelist.clone(),
        flag_prefer_dmy:      args.flag_prefer_dmy,
        flag_no_headers:      args.flag_no_headers,
        flag_delimiter:       format!("{:?}", args.flag_delimiter.clone()),
        // when we write to stdout, we don't use snappy compression
        // when we write to a file with the --output option, we use
        // snappy compression if the file ends with ".sz"
        flag_output_snappy:   if stdout_output_flag {
            false
        } else {
            let p = args.flag_output.clone().unwrap();
            p.to_ascii_lowercase().ends_with(".sz")
        },
        canonical_input_path: String::new(),
        canonical_stats_path: String::new(),
        record_count:         0,
        date_generated:       String::new(),
        compute_duration_ms:  0,
        // save the qsv version in the stats.csv.json file
        // so cached stats are automatically invalidated
        // when the qsv version changes
        qsv_version:          env!("CARGO_PKG_VERSION").to_string(),
    };

    // create a temporary file to store the <FILESTEM>.stats.csv file
    let stats_csv_tempfile = if current_stats_args.flag_output_snappy {
        tempfile::Builder::new().suffix(".sz").tempfile()?
    } else {
        NamedTempFile::new()?
    };

    // find the delimiter to use based on the extension of the output file
    // and if we need to snappy compress the output
    let (output_extension, output_delim, snappy) = match args.flag_output {
        Some(ref output_path) => get_delim_by_extension(Path::new(&output_path), b','),
        _ => (String::new(), b',', false),
    };
    let stats_csv_tempfile_fname = format!(
        "{stem}.{prime_ext}{snappy_ext}",
        //safety: we know the tempfile is a valid NamedTempFile, so we can use unwrap
        stem = stats_csv_tempfile.path().to_str().unwrap(),
        prime_ext = output_extension,
        snappy_ext = if snappy { ".sz" } else { "" }
    );

    // we will write the stats to a temp file
    let wconfig = Config::new(Some(stats_csv_tempfile_fname.clone()).as_ref())
        .delimiter(Some(Delimiter(output_delim)));
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

        let mut stdin_file = tempfile::Builder::new().tempfile_in(temp_dir)?;

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

    let mut compute_stats = true;
    let mut create_cache = args.flag_cache_threshold == 1
        || args.flag_stats_jsonl
        || args.flag_cache_threshold.is_negative();

    let mut autoindex_set = false;

    let write_stats_jsonl = args.flag_stats_jsonl;

    if let Some(path) = rconfig.path.clone() {
        //safety: we know the path is a valid PathBuf, so we can use unwrap
        let path_file_stem = path.file_stem().unwrap().to_str().unwrap();
        let stats_file = stats_path(&path, false)?;
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
                    let mut json_buffer = existing_stats_args_json_str.into_bytes();
                    match simd_json::to_owned_value(&mut json_buffer) {
                        Ok(value) => {
                            // Convert OwnedValue to StatsArgs
                            match StatsArgs::from_owned_value(&value) {
                                Ok(mut stat_args) => {
                                    // we init these fields to empty values because we don't want to
                                    // compare them when checking if the
                                    // args are the same
                                    stat_args.canonical_input_path = String::new();
                                    stat_args.canonical_stats_path = String::new();
                                    stat_args.record_count = 0;
                                    stat_args.date_generated = String::new();
                                    time_saved = stat_args.compute_duration_ms;
                                    stat_args.compute_duration_ms = 0;
                                    stat_args
                                },
                                Err(e) => {
                                    time_saved = 0;
                                    log::warn!(
                                        "Could not deserialize {path_file_stem}.stats.csv.json: \
                                         {e:?}, recomputing..."
                                    );
                                    let _ = fs::remove_file(&stats_file);
                                    let _ = fs::remove_file(&stats_args_json_file);
                                    StatsArgs::default()
                                },
                            }
                        },
                        Err(e) => {
                            time_saved = 0;
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

                // check if the cached stats are current (ie the stats file is newer than the input
                // file), use the same args or if the --everything flag was set, and
                // all the other non-stats args are equal. If so, we don't need to recompute the
                // stats
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
                            && existing_stats_args_json.flag_dates_whitelist
                                == current_stats_args.flag_dates_whitelist
                            && existing_stats_args_json.flag_delimiter
                                == current_stats_args.flag_delimiter
                            && existing_stats_args_json.flag_nulls == current_stats_args.flag_nulls
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

            // we're loading the entire file into memory, we need to check avail mem
            if args.flag_everything
                || args.flag_mode
                || args.flag_cardinality
                || args.flag_median
                || args.flag_quartiles
                || args.flag_mad
            {
                util::mem_file_check(&path, false, args.flag_memcheck)?;
            }

            // check if flag_cache_threshold is a negative number,
            // if so, set the autoindex_size to absolute of the number
            if args.flag_cache_threshold.is_negative() {
                rconfig.autoindex_size = args.flag_cache_threshold.unsigned_abs() as u64;
                autoindex_set = true;
            }

            // we need to count the number of records in the file to calculate sparsity and
            // cardinality
            let record_count: u64;

            let (headers, stats) = match rconfig.indexed()? {
                None => {
                    // without an index, we need to count the number of records in the file
                    // safety: we know util::count_rows() will not return an Err
                    record_count = util::count_rows(&rconfig).unwrap();
                    args.sequential_stats(&args.flag_dates_whitelist)
                },
                Some(idx) => {
                    // with an index, we get the rowcount instantaneously from the index
                    record_count = idx.count();
                    match args.flag_jobs {
                        Some(num_jobs) => {
                            if num_jobs == 1 {
                                args.sequential_stats(&args.flag_dates_whitelist)
                            } else {
                                args.parallel_stats(&args.flag_dates_whitelist, record_count)
                            }
                        },
                        _ => args.parallel_stats(&args.flag_dates_whitelist, record_count),
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

            if args.flag_dataset_stats {
                // Add dataset-level stats as additional rows ====================
                let num_stats_fields = stats_headers_sr.len();
                let mut dataset_stats_br = csv::ByteRecord::with_capacity(128, num_stats_fields);

                // Helper closure to write a dataset stat row
                let mut write_dataset_stat = |name: &[u8], value: u64| -> CliResult<()> {
                    dataset_stats_br.clear();
                    dataset_stats_br.push_field(name);
                    // Fill middle columns with empty strings
                    for _ in 2..num_stats_fields {
                        dataset_stats_br.push_field(b"");
                    }
                    // write qsv__value as last column
                    dataset_stats_br.push_field(itoa::Buffer::new().format(value).as_bytes());
                    wtr.write_byte_record(&dataset_stats_br)
                        .map_err(std::convert::Into::into)
                };

                // Write qsv__rowcount
                write_dataset_stat(b"qsv__rowcount", record_count)?;

                // Write qsv__columncount
                let ds_column_count = headers.len() as u64;
                write_dataset_stat(b"qsv__columncount", ds_column_count)?;

                // Write qsv__filesize_bytes
                let ds_filesize_bytes = fs::metadata(&path)?.len();
                write_dataset_stat(b"qsv__filesize_bytes", ds_filesize_bytes)?;

                // Compute hash of stats for data fingerprinting
                let stats_hash = {
                    // the first FINGERPRINT_HASH_COLUMNS are used for the fingerprint hash
                    let mut hash_input = Vec::with_capacity(FINGERPRINT_HASH_COLUMNS);

                    // First, create a stable representation of the stats
                    for record in &stats_br_vec {
                        // Take FINGERPRINT_HASH_COLUMNS columns only
                        for field in record.iter().take(FINGERPRINT_HASH_COLUMNS) {
                            let s = String::from_utf8_lossy(field);
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
                    sha256::digest(hash_input.as_slice())
                };

                dataset_stats_br.clear();
                dataset_stats_br.push_field(b"qsv__fingerprint_hash");
                // Fill middle columns with empty strings
                for _ in 2..num_stats_fields {
                    dataset_stats_br.push_field(b"");
                }
                // write qsv__value as last column
                dataset_stats_br.push_field(stats_hash.as_bytes());
                wtr.write_byte_record(&dataset_stats_br)?;
            }

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
        stats_path(rconfig.path.as_ref().unwrap(), false)?
            .to_str()
            .unwrap()
            .to_owned()
    };

    if rconfig.is_stdin() {
        // if we read from stdin, copy the temp stats file to "stdin.stats.csv"
        // safety: we know the path is a valid PathBuf, so we can use unwrap
        let mut stats_pathbuf = stats_path(rconfig.path.as_ref().unwrap(), true)?;
        fs::copy(currstats_filename.clone(), stats_pathbuf.clone())?;

        // save the stats args to "stdin.stats.csv.json"
        stats_pathbuf.set_extension("csv.json");
        std::fs::write(
            stats_pathbuf,
            serde_json::to_string_pretty(&current_stats_args)?,
        )?;
    } else if let Some(path) = rconfig.path {
        // if we read from a file, copy the temp stats file to "<FILESTEM>.stats.csv"
        let mut stats_pathbuf = path.clone();
        stats_pathbuf.set_extension("stats.csv");
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
            std::fs::write(
                stats_pathbuf.clone(),
                serde_json::to_string_pretty(&current_stats_args)?,
            )?;

            // save the stats data to "<FILESTEM>.stats.csv.data.jsonl"
            if write_stats_jsonl {
                let mut stats_jsonl_pathbuf = stats_pathbuf.clone();
                stats_jsonl_pathbuf.set_extension("data.jsonl");
                util::csv_to_jsonl(
                    &currstats_filename,
                    &STATSDATA_TYPES_MAP,
                    &stats_jsonl_pathbuf,
                )?;
            }
        }
    }

    if stdout_output_flag {
        // if we're outputting to stdout, copy the stats file to stdout
        let currstats = fs::read_to_string(currstats_filename)?;
        io::stdout().write_all(currstats.as_bytes())?;
        io::stdout().flush()?;
    } else if let Some(output) = args.flag_output {
        // if we're outputting to a file, copy the stats file to the output file
        if currstats_filename != output {
            // if the stats file is not the same as the output file, copy it
            fs::copy(currstats_filename, output)?;
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
    fn sequential_stats(&self, whitelist: &str) -> CliResult<(csv::ByteRecord, Vec<Stats>)> {
        let mut rdr = self.rconfig().reader()?;
        let (headers, sel) = self.sel_headers(&mut rdr)?;

        init_date_inference(self.flag_infer_dates, &headers, whitelist)?;

        let stats = self.compute(&sel, rdr.byte_records());
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
    /// * Results are merged using the `merge_all` function
    ///
    /// # Safety Considerations
    ///
    /// * Requires a valid CSV index file for random access
    /// * Uses unsafe code for performance-critical operations
    /// * Thread safety is ensured through channel-based communication
    /// * Index seeking operations are wrapped in expect() for better error messages
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
            return self.sequential_stats(whitelist);
        }

        let mut rdr = self.rconfig().reader()?;
        let (headers, sel) = self.sel_headers(&mut rdr)?;

        init_date_inference(self.flag_infer_dates, &headers, whitelist)?;

        let njobs = util::njobs(self.flag_jobs);
        let chunk_size = util::chunk_size(idx_count as usize, njobs);
        let nchunks = util::num_of_chunks(idx_count as usize, chunk_size);

        let pool = ThreadPool::new(njobs);
        let (send, recv) = crossbeam_channel::bounded(nchunks);
        for i in 0..nchunks {
            let (send, args, sel) = (send.clone(), self.clone(), sel.clone());
            pool.execute(move || {
                // safety: indexed() is safe as we know we have an index file
                // and we know it will return an Ok
                let mut idx = unsafe {
                    args.rconfig()
                        .indexed()
                        .unwrap_unchecked()
                        .unwrap_unchecked()
                };
                // safety: seek() is safe as we know we have an index file
                // we do an expect() here so that it triggers a human-panic
                // with some actionable infoif the index is corrupted
                idx.seek((i * chunk_size) as u64)
                    .expect("File seek failed.");
                let it = idx.byte_records().take(chunk_size);
                // safety: this will only return an Error if the channel has been disconnected
                unsafe {
                    send.send(args.compute(&sel, it)).unwrap_unchecked();
                }
            });
        }
        drop(send);
        Ok((headers, merge_all(recv.iter()).unwrap_or_default()))
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
        let dataset_stats = self.flag_dataset_stats;
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
                send.send(stat.to_record(round_places, infer_boolean, visualize_ws, dataset_stats))
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

    /// Computes statistics for CSV data from an iterator of records.
    ///
    /// This function processes CSV records from an iterator and computes comprehensive
    /// statistics for each column. It's the core computation engine used by both
    /// sequential and parallel processing approaches.
    ///
    /// # Arguments
    ///
    /// * `sel` - Column selection configuration
    /// * `it` - Iterator over CSV records (ByteRecord results)
    ///
    /// # Returns
    ///
    /// A vector of `Stats` objects, one for each selected column
    ///
    /// # Process
    ///
    /// 1. **Initialization**: Creates `Stats` objects for each selected column
    /// 2. **Record Processing**: Iterates through all CSV records
    /// 3. **Field Processing**: For each record, processes selected fields
    /// 4. **Statistics Accumulation**: Updates statistics for each field
    /// 5. **Type Inference**: Automatically detects data types during processing
    ///
    /// # Performance Optimizations
    ///
    /// * **Inline**: Function is marked as `#[inline]` for performance
    /// * **Unsafe Operations**: Uses unsafe code for bounds checking avoidance
    /// * **Memory Reuse**: Reuses ByteRecord objects to reduce allocations
    /// * **Hot Loop Optimization**: Critical path is optimized for speed
    /// * **Register Usage**: Frequently accessed variables are kept in registers
    ///
    /// # Safety Considerations
    ///
    /// * Uses unsafe code for performance-critical operations
    /// * Assumes `INFER_DATE_FLAGS` is properly initialized
    /// * Bounds checking is avoided where safe
    /// * Iterator errors are handled gracefully
    #[inline]
    fn compute<I>(&self, sel: &Selection, it: I) -> Vec<Stats>
    where
        I: Iterator<Item = csv::Result<csv::ByteRecord>>,
    {
        let sel_len = sel.len();
        let mut stats = self.new_stats(sel_len);

        // safety: we know INFER_DATE_FLAGS is Some because we called init_date_inference
        let infer_date_flags = INFER_DATE_FLAGS.get().unwrap();

        // so we don't need to get infer_boolean/prefer_dmy from big args struct for each iteration
        // and hopefully the compiler will optimize this and use registers in the hot loop
        let infer_boolean = self.flag_infer_boolean;
        let prefer_dmy = self.flag_prefer_dmy;

        let mut i;
        #[allow(unused_assignments)]
        let mut current_row = csv::ByteRecord::with_capacity(1024, sel_len);
        for row in it {
            i = 0;
            // safety: because we're using iterators and INFER_DATE_FLAGS has the same size,
            // we know we don't need to bounds check
            unsafe {
                current_row = row.unwrap_unchecked();
                for field in sel.select(&current_row) {
                    stats.get_unchecked_mut(i).add(
                        field,
                        *infer_date_flags.get_unchecked(i),
                        infer_boolean,
                        prefer_dmy,
                    );
                    i += 1;
                }
            }
        }
        stats
    }

    /// Reads and processes CSV headers with column selection.
    ///
    /// This function reads the CSV headers from the reader and applies column selection
    /// based on the configuration. It returns both the selected headers and the selection
    /// object for use in subsequent processing.
    ///
    /// # Arguments
    ///
    /// * `rdr` - CSV reader with the input data
    ///
    /// # Returns
    ///
    /// * `Ok((csv::ByteRecord, Selection))` - Tuple containing selected headers and selection
    ///   object
    /// * `Err(CliError)` - If there's an error reading headers or applying selection
    ///
    /// # Process
    ///
    /// 1. **Header Reading**: Reads byte headers from the CSV reader
    /// 2. **Selection Creation**: Creates a selection object based on configuration
    /// 3. **Header Filtering**: Applies the selection to filter headers
    /// 4. **Return**: Returns both selected headers and selection object
    ///
    /// # Performance
    ///
    /// * **Inline**: Function is marked as `#[inline]` for performance
    /// * **Minimal Allocations**: Reuses existing data structures where possible
    #[inline]
    fn sel_headers<R: io::Read>(
        &self,
        rdr: &mut csv::Reader<R>,
    ) -> CliResult<(csv::ByteRecord, Selection)> {
        let headers = rdr.byte_headers()?.clone();
        let sel = self.rconfig().selection(&headers)?;
        Ok((sel.select(&headers).collect(), sel))
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
            .no_headers(self.flag_no_headers)
            .select(self.flag_select.clone())
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
    /// * **Pre-allocated**: Uses `Vec::with_capacity` for efficient allocation
    /// * **Bulk Initialization**: Uses `repeat_n` for efficient object creation
    #[inline]
    fn new_stats(&self, record_len: usize) -> Vec<Stats> {
        let mut stats: Vec<Stats> = Vec::with_capacity(record_len);
        stats.extend(repeat_n(
            Stats::new(WhichStats {
                include_nulls:   self.flag_nulls,
                sum:             !self.flag_typesonly || self.flag_infer_boolean,
                range:           !self.flag_typesonly || self.flag_infer_boolean,
                dist:            !self.flag_typesonly || self.flag_infer_boolean,
                cardinality:     self.flag_everything || self.flag_cardinality,
                median:          !self.flag_everything && self.flag_median && !self.flag_quartiles,
                mad:             self.flag_everything || self.flag_mad,
                quartiles:       self.flag_everything || self.flag_quartiles,
                mode:            self.flag_everything || self.flag_mode,
                typesonly:       self.flag_typesonly,
                percentiles:     self.flag_everything || self.flag_percentiles,
                percentile_list: self.flag_percentile_list.clone(),
            }),
            record_len,
        ));
        stats
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
        if self.flag_dataset_stats {
            // we add the qsv__value field at the end for dataset-level stats
            fields.push("qsv__value");
        }

        csv::StringRecord::from(fields)
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
fn stats_path(stats_csv_path: &Path, stdin_flag: bool) -> io::Result<PathBuf> {
    let parent = stats_csv_path
        .parent()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid path"))?;
    let fstem = stats_csv_path
        .file_stem()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid file name"))?;

    let new_fname = if stdin_flag {
        "stdin.stats.csv".to_string()
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
/// * `headers` - The CSV headers as a ByteRecord containing column names
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

#[derive(Clone, Debug, Eq, PartialEq, Default, Serialize, Deserialize)]
struct WhichStats {
    include_nulls:   bool,
    sum:             bool,
    range:           bool,
    dist:            bool,
    cardinality:     bool,
    median:          bool,
    mad:             bool,
    quartiles:       bool,
    mode:            bool,
    typesonly:       bool,
    percentiles:     bool,
    percentile_list: String,
}

impl Commute for WhichStats {
    #[inline]
    fn merge(&mut self, other: WhichStats) {
        assert_eq!(*self, other);
    }
}

#[allow(clippy::unsafe_derive_deserialize)]
#[repr(C, align(64))] // Align to cache line size for better performance
#[derive(Clone, Serialize, Deserialize, PartialEq)]
struct Stats {
    // CACHE LINE 1: Most frequently accessed fields (hot data)
    // Group small, frequently accessed fields together
    typ:           FieldType, // 1 byte - accessed in every add() call
    is_ascii:      bool,      // 1 byte - accessed for strings
    max_precision: u16,       // 2 bytes - accessed for floats

    // 4 bytes padding here for alignment
    nullcount:   u64, // 8 bytes - frequently updated counter
    sum_stotlen: u64, // 8 bytes - frequently updated counter

    // Configuration flags (accessed once during initialization)
    which: WhichStats, // 10 bytes - read-only after initialization

    // 6 bytes padding here to reach 64 bytes (cache line boundary)

    // CACHE LINE 2+: Less frequently accessed but still important
    // Large Option types that may be None, grouped by usage pattern
    sum: Option<TypedSum>, // 32 bytes - updated in add() for numeric types

    // CACHE LINE 3+: Statistics computation fields
    online:     Option<OnlineStats>, // 48 bytes - used for mean/variance calculations
    online_len: Option<OnlineStats>, // 48 bytes - used for string length stats

    // CACHE LINE 4+: Mode and cardinality computation
    modes: Option<Unsorted<Vec<u8>>>, // 32 bytes - used for mode/cardinality

    // CACHE LINE 5+: Sorting-based statistics
    #[allow(clippy::struct_field_names)]
    unsorted_stats: Option<Unsorted<f64>>, // 32 bytes - median/quartiles/percentiles

    // CACHE LINE 6+: Min/Max tracking (largest field, least cache-friendly)
    minmax: Option<TypedMinMax>, // 432 bytes - largest field, accessed less frequently
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
/// * `typ` - The field type (TDate or TDateTime)
///
/// # Returns
///
/// A string in RFC3339 format (e.g., "2023-01-15T10:30:00Z" or "2023-01-15")
///
/// # Behavior
///
/// * **TDate**: Returns only the date component (YYYY-MM-DD)
/// * **TDateTime**: Returns full RFC3339 format with time and timezone
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
    fn new(which: WhichStats) -> Stats {
        let (mut sum, mut minmax, mut online, mut online_len, mut modes, mut unsorted_stats) =
            (None, None, None, None, None, None);
        if which.sum {
            sum = Some(TypedSum::default());
        }
        if which.range {
            minmax = Some(TypedMinMax::default());
        }
        if which.dist {
            online = Some(stats::OnlineStats::default());
            online_len = Some(stats::OnlineStats::default());
        }

        // preallocate memory for the unsorted stats structs
        // if we dont't have a record count, we use a default of 10,000
        // to avoid allocating too much memory
        let record_count = *RECORD_COUNT.get().unwrap_or(&10_000) as usize;
        if which.mode || which.cardinality {
            modes = Some(stats::Unsorted::with_capacity(record_count));
        }
        // we use the same Unsorted struct for median, mad, quartiles & percentiles
        if which.quartiles || which.median || which.mad || which.percentiles {
            unsorted_stats = Some(stats::Unsorted::with_capacity(record_count));
        }
        Stats {
            typ: FieldType::default(),
            is_ascii: true,
            max_precision: 0,
            which,
            nullcount: 0,
            sum_stotlen: 0,
            sum,
            modes,
            unsorted_stats,
            online,
            online_len,
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
    /// * Uses unsafe code for performance-critical operations
    /// * Assumes valid UTF-8 input for string operations
    /// * Bounds checking is avoided where safe
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn add(&mut self, sample: &[u8], infer_dates: bool, infer_boolean: bool, prefer_dmy: bool) {
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
        if let Some(v) = self.modes.as_mut() {
            v.add(sample.to_vec());
        }

        if t == TString {
            // safety: online_len is always enabled when t == TString
            unsafe {
                self.online_len
                    .as_mut()
                    .unwrap_unchecked()
                    .add(&sample.len());
            }
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
            TInteger => {
                if let Some(v) = self.unsorted_stats.as_mut() {
                    v.add(float_val);
                }
                // safety: online is always enabled
                unsafe {
                    self.online.as_mut().unwrap_unchecked().add_f64(float_val);
                }
            },
            TFloat => {
                if let Some(v) = self.unsorted_stats.as_mut() {
                    v.add(float_val);
                }
                // safety: online is always enabled
                unsafe {
                    self.online.as_mut().unwrap_unchecked().add_f64(float_val);
                }

                // precision calculation
                // note that we are referring to number of decimal places,
                // not the number of significant digits
                let precision = if float_val == 0.0 {
                    0
                } else {
                    // safety: we know that f is a valid f64
                    // so there will always be a fraction part, even if it's 0
                    unsafe {
                        ryu::Buffer::new()
                            .format_finite(float_val)
                            .split('.')
                            .next_back()
                            .unwrap_unchecked()
                            .len() as u16
                    }
                };
                self.max_precision = std::cmp::max(self.max_precision, precision);
            },
            TDateTime | TDate => {
                // calculate date statistics by adding date samples as unix timestamps
                // to the millisecond precision.
                #[allow(clippy::cast_precision_loss)]
                let timestamp = int_val as f64;
                if let Some(v) = self.unsorted_stats.as_mut() {
                    v.add(timestamp);
                }
                // safety: online is always enabled
                unsafe {
                    self.online.as_mut().unwrap_unchecked().add_f64(timestamp);
                }
            },
            _ => {},
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
    /// * `dataset_stats` - Whether to include dataset-level statistics (adds empty field for
    ///   qsv__value)
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
        dataset_stats: bool,
    ) -> csv::StringRecord {
        // we're doing typesonly and not inferring boolean, just return the type
        if self.which.typesonly && !infer_boolean {
            return csv::StringRecord::from(vec![self.typ.to_string()]);
        }

        let typ = self.typ;
        // prealloc memory for performance
        // we have MAX_STAT_COLUMNS columns at most with --everything
        let mut pieces = Vec::with_capacity(MAX_STAT_COLUMNS);

        let empty = String::new;

        // min/max/range/sort_order/sortiness
        // we do this first as we want to get the sort_order, so we can skip sorting if not
        // required. We also need to do this before --infer-boolean because we need to know
        // the min/max values to determine if the range is equal to the supported boolean
        // ranges as specified by --boolean-patterns.
        let mut minmax_range_sortorder_pieces = Vec::with_capacity(5);
        let mut minval = String::new();
        let mut maxval = String::new();
        let mut column_sorted = false;
        if let Some(mm) = self
            .minmax
            .as_ref()
            .and_then(|mm| mm.show(typ, round_places, visualize_ws))
        {
            // save min/max values for boolean inferencing
            minval.clone_from(&mm.0);
            maxval.clone_from(&mm.1);
            if mm.3.starts_with("Ascending") {
                column_sorted = true;
            }
            minmax_range_sortorder_pieces.extend_from_slice(&[mm.0, mm.1, mm.2, mm.3, mm.4]);
        } else {
            minmax_range_sortorder_pieces.extend_from_slice(&[
                empty(),
                empty(),
                empty(),
                empty(),
                empty(),
            ]);
        }

        let record_count = *RECORD_COUNT.get().unwrap_or(&1);

        // get the stats separator
        let stats_separator = if self.which.mode || self.which.percentiles {
            std::env::var("QSV_STATS_SEPARATOR")
                .unwrap_or_else(|_| DEFAULT_STATS_SEPARATOR.to_string())
        } else {
            DEFAULT_STATS_SEPARATOR.to_string()
        };

        // modes/antimodes & cardinality/uniqueness_ratio
        // we do this second because we can use the sort order with cardinality, to skip sorting
        // if its not required. This makes not only cardinality computation faster, it also makes
        // modes/antimodes computation faster.
        // We also need to know the cardinality to --infer-boolean should that be enabled
        let mut cardinality = 0;
        let mut mc_pieces = Vec::with_capacity(8);
        match self.modes.as_mut() {
            None => {
                if self.which.cardinality {
                    mc_pieces.extend_from_slice(&[empty(), empty()]);
                }
                if self.which.mode {
                    mc_pieces.extend_from_slice(&[
                        empty(),
                        empty(),
                        empty(),
                        empty(),
                        empty(),
                        empty(),
                    ]);
                }
            },
            Some(ref mut v) => {
                if self.which.cardinality {
                    cardinality = v.cardinality(column_sorted, 1);
                    #[allow(clippy::cast_precision_loss)]
                    let uniqueness_ratio = (cardinality as f64) / (record_count as f64);
                    mc_pieces.extend_from_slice(&[
                        itoa::Buffer::new().format(cardinality).to_owned(),
                        util::round_num(uniqueness_ratio, round_places),
                    ]);
                }
                if self.which.mode {
                    // mode/s & antimode/s
                    if cardinality == record_count {
                        // all values unique
                        mc_pieces.extend_from_slice(
                            // modes - short-circuit modes calculation as there is none
                            &[
                                empty(),
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
                                .map(|c| util::visualize_whitespace(&String::from_utf8_lossy(c)))
                                .join(&stats_separator)
                        } else {
                            modes_result
                                .iter()
                                .map(|c| String::from_utf8_lossy(c))
                                .join(&stats_separator)
                        };

                        // antimode/s ============
                        let antimodes_len = ANTIMODES_LEN.get_or_init(|| {
                            std::env::var("QSV_ANTIMODES_LEN")
                                .map(|val| {
                                    let parsed =
                                        val.parse::<usize>().unwrap_or(DEFAULT_ANTIMODES_LEN);
                                    // if 0, disable length limiting
                                    if parsed == 0 { usize::MAX } else { parsed }
                                })
                                .unwrap_or(DEFAULT_ANTIMODES_LEN)
                        });

                        let mut antimodes_list = String::with_capacity(*antimodes_len);

                        // We only store the first 10 antimodes
                        // so if antimodes_count > 10, add the "*PREVIEW: " prefix
                        if antimodes_count > MAX_ANTIMODES {
                            antimodes_list.push_str("*PREVIEW: ");
                        }

                        let antimodes_vals = &antimodes_result
                            .iter()
                            .map(|c| String::from_utf8_lossy(c))
                            .join(&stats_separator);

                        // if the antimodes result starts with the separator,
                        // it indicates that NULL is the first antimode. Add NULL to the list.
                        if antimodes_vals.starts_with(&stats_separator) {
                            antimodes_list.push_str("NULL");
                        }
                        antimodes_list.push_str(antimodes_vals);

                        // and truncate at antimodes_len characters with an ellipsis
                        if antimodes_list.len() > *antimodes_len {
                            util::utf8_truncate(&mut antimodes_list, *antimodes_len + 1);
                            antimodes_list.push_str("...");
                        }

                        mc_pieces.extend_from_slice(&[
                            // mode/s
                            modes_list,
                            modes_count.to_string(),
                            mode_occurrences.to_string(),
                            // antimode/s
                            if visualize_ws {
                                util::visualize_whitespace(&antimodes_list)
                            } else {
                                antimodes_list
                            },
                            antimodes_count.to_string(),
                            antimode_occurrences.to_string(),
                        ]);
                    }
                }
            },
        }

        // type
        if cardinality == 2 && infer_boolean {
            // if cardinality is 2, it's a boolean if its in the true/false patterns
            let patterns = BOOLEAN_PATTERNS.get();
            if let Some(patterns) = patterns {
                let mut is_boolean = false;
                for pattern in patterns {
                    if pattern.matches(&minval).is_some() && pattern.matches(&maxval).is_some() {
                        pieces.push("Boolean".to_string());
                        is_boolean = true;
                        break;
                    }
                }
                if !is_boolean {
                    pieces.push(typ.to_string());
                }
            } else {
                pieces.push(typ.to_string());
            }
        } else {
            pieces.push(typ.to_string());
        }

        // we're doing --typesonly with --infer-boolean, we don't need to calculate anything else
        if self.which.typesonly && infer_boolean {
            return csv::StringRecord::from(pieces);
        }

        // is_ascii
        if typ == FieldType::TString {
            pieces.push(self.is_ascii.to_string());
        } else {
            pieces.push(empty());
        }

        // sum
        let stotlen =
            if let Some((stotlen_work, sum)) = self.sum.as_ref().and_then(|sum| sum.show(typ)) {
                if typ == FieldType::TFloat {
                    if let Ok(f64_val) = fast_float2::parse::<f64, &[u8]>(sum.as_bytes()) {
                        pieces.push(util::round_num(f64_val, round_places));
                    } else {
                        pieces.push(format!("ERROR: Cannot convert {sum} to a float."));
                    }
                } else {
                    pieces.push(sum);
                }
                stotlen_work
            } else {
                pieces.push(empty());
                0
            };

        // min/max/range/sort_order
        // actually append it here - to preserve legacy ordering of columns
        pieces.extend_from_slice(&minmax_range_sortorder_pieces);

        // min/max/sum/avg/stddev/variance/cv length
        // we only show string length stats for String type
        if typ != FieldType::TString {
            pieces.extend_from_slice(&[
                empty(),
                empty(),
                empty(),
                empty(),
                empty(),
                empty(),
                empty(),
            ]);
        } else if let Some(mm) = self.minmax.as_ref().and_then(TypedMinMax::len_range) {
            // we have a min/max length
            pieces.extend_from_slice(&[mm.0, mm.1]);
            if stotlen < u64::MAX {
                pieces.push(itoa::Buffer::new().format(stotlen).to_owned());
                #[allow(clippy::cast_precision_loss)]
                let avg_len = stotlen as f64 / record_count as f64;
                pieces.push(util::round_num(avg_len, round_places));

                if let Some(vl) = self.online_len.as_ref() {
                    let vlen_stddev = vl.stddev();
                    let vlen_variance = vl.variance();
                    pieces.push(util::round_num(vlen_stddev, round_places));
                    pieces.push(util::round_num(vlen_variance, round_places));
                    pieces.push(util::round_num(vlen_stddev / avg_len, round_places));
                } else {
                    pieces.push(empty());
                    pieces.push(empty());
                    pieces.push(empty());
                }
            } else {
                // we saturated the sum of string lengths, it means we had an overflow
                // so we return OVERFLOW_STRING for sum,avg,stddev,variance length
                pieces.extend_from_slice(&[
                    OVERFLOW_STRING.to_string(),
                    OVERFLOW_STRING.to_string(),
                    OVERFLOW_STRING.to_string(),
                    OVERFLOW_STRING.to_string(),
                    OVERFLOW_STRING.to_string(),
                ]);
            }
        } else {
            pieces.extend_from_slice(&[
                empty(),
                empty(),
                empty(),
                empty(),
                empty(),
                empty(),
                empty(),
            ]);
        }

        // mean, sem, geometric_mean, harmonic_mean, stddev, variance & cv
        if typ == TString || typ == TNull {
            pieces.extend_from_slice(&[
                empty(),
                empty(),
                empty(),
                empty(),
                empty(),
                empty(),
                empty(),
            ]);
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
                pieces.extend_from_slice(&[
                    mean_string,
                    util::round_num(sem, round_places),
                    util::round_num(geometric_mean, round_places),
                    util::round_num(harmonic_mean, round_places),
                    util::round_num(std_dev, round_places),
                    util::round_num(v.variance(), round_places),
                    util::round_num(cv, round_places),
                ]);
            } else {
                // by the time we get here, the type is a TDateTime or TDate
                pieces.push(timestamp_ms_to_rfc3339(mean as i64, typ));
                // instead of returning sem, stdev & variance as timestamps, return it in
                // days as its more human readable and practical for real-world use cases
                // Round to at least 5 decimal places, so we have millisecond precision
                pieces.push(util::round_num(
                    sem / MS_IN_DAY,
                    u32::max(round_places, DAY_DECIMAL_PLACES),
                ));
                pieces.push(util::round_num(
                    geometric_mean / MS_IN_DAY,
                    u32::max(round_places, DAY_DECIMAL_PLACES),
                ));
                pieces.push(util::round_num(
                    harmonic_mean / MS_IN_DAY,
                    u32::max(round_places, DAY_DECIMAL_PLACES),
                ));
                pieces.push(util::round_num(
                    std_dev / MS_IN_DAY,
                    u32::max(round_places, DAY_DECIMAL_PLACES),
                ));
                pieces.push(util::round_num(
                    v.variance() / (MS_IN_DAY * MS_IN_DAY),
                    u32::max(round_places, DAY_DECIMAL_PLACES),
                ));
                pieces.push(util::round_num(cv, round_places));
            }
        } else {
            pieces.extend_from_slice(&[
                empty(),
                empty(),
                empty(),
                empty(),
                empty(),
                empty(),
                empty(),
            ]);
        }

        // nullcount
        pieces.push(itoa::Buffer::new().format(self.nullcount).to_owned());

        // max precision
        if typ == TFloat {
            pieces.push(self.max_precision.to_string());
        } else {
            pieces.push(empty());
        }

        // sparsity
        #[allow(clippy::cast_precision_loss)]
        let sparsity: f64 = self.nullcount as f64 / *RECORD_COUNT.get().unwrap_or(&1) as f64;
        pieces.push(util::round_num(sparsity, round_places));

        // quartiles
        // as q2==median, cache and reuse it if the --median or --mad flags are set
        let mut existing_median = None;
        let mut quartile_pieces = Vec::with_capacity(9);
        match self.unsorted_stats.as_mut().and_then(|v| match typ {
            TInteger | TFloat | TDate | TDateTime => {
                if self.which.quartiles {
                    v.quartiles()
                } else {
                    None
                }
            },
            _ => None,
        }) {
            None => {
                if self.which.quartiles {
                    quartile_pieces.extend_from_slice(&[
                        empty(),
                        empty(),
                        empty(),
                        empty(),
                        empty(),
                        empty(),
                        empty(),
                        empty(),
                        empty(),
                    ]);
                }
            },
            Some((q1, q2, q3)) => {
                existing_median = Some(q2);
                let iqr = q3 - q1;

                // use fused multiply add (mul_add) when possible
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
        if let Some(v) = self.unsorted_stats.as_mut().and_then(|v| {
            if let TNull | TString = typ {
                None
            } else if let Some(existing_median) = existing_median {
                // if we already calculated the q2 (median) in the quartiles, return it
                if self.which.median {
                    Some(existing_median)
                } else {
                    None
                }
            } else if self.which.median {
                // otherwise, calculate the median
                v.median()
            } else {
                None
            }
        }) {
            if typ == TDateTime || typ == TDate {
                pieces.push(timestamp_ms_to_rfc3339(v as i64, typ));
            } else {
                pieces.push(util::round_num(v, round_places));
            }
        } else if self.which.median {
            pieces.push(empty());
        }

        // median absolute deviation (MAD)
        if let Some(v) = self.unsorted_stats.as_mut().and_then(|v| {
            if let TNull | TString = typ {
                None
            } else if self.which.mad {
                v.mad(existing_median)
            } else {
                None
            }
        }) {
            if typ == TDateTime || typ == TDate {
                // like stddev, return MAD in days when the type is a date or datetime
                pieces.push(util::round_num(
                    v / MS_IN_DAY,
                    u32::max(round_places, DAY_DECIMAL_PLACES),
                ));
            } else {
                pieces.push(util::round_num(v, round_places));
            }
        } else if self.which.mad {
            pieces.push(empty());
        }

        // quartiles
        // append it here to preserve legacy ordering of columns
        pieces.extend_from_slice(&quartile_pieces);

        // mode/modes/antimodes & cardinality
        // append it here to preserve legacy ordering of columns
        pieces.extend_from_slice(&mc_pieces);

        // Add percentiles after quartiles
        if let Some(v) = self.unsorted_stats.as_mut() {
            match typ {
                TInteger | TFloat | TDate | TDateTime => {
                    let percentile_list = self
                        .which
                        .percentile_list
                        .split(',')
                        .filter_map(|p| p.trim().parse::<f64>().ok())
                        .map(|p| p as u8)
                        .collect::<Vec<_>>();

                    if let Some(percentile_values) = v.custom_percentiles(&percentile_list) {
                        let formatted_values = if typ == TDateTime || typ == TDate {
                            percentile_values
                                .iter()
                                .map(|p| {
                                    // Explicitly cast f64 to i64 for timestamp conversion
                                    #[allow(clippy::cast_possible_truncation)]
                                    let ts = p.round() as i64;
                                    timestamp_ms_to_rfc3339(ts, typ)
                                })
                                .collect::<Vec<_>>()
                        } else {
                            percentile_values
                                .iter()
                                .map(|p| util::round_num(*p, round_places))
                                .collect::<Vec<_>>()
                        };
                        pieces.push(formatted_values.join(&stats_separator));
                    } else {
                        pieces.push(empty());
                    }
                },
                _ => pieces.push(empty()),
            }
        } else if self.which.percentiles {
            pieces.push(empty());
        }

        if dataset_stats {
            // add an empty field for qsv__value
            pieces.push(empty());
        }

        csv::StringRecord::from(pieces)
    }
}

impl Commute for Stats {
    #[inline]
    fn merge(&mut self, other: Stats) {
        self.typ.merge(other.typ);
        self.is_ascii &= other.is_ascii;
        self.max_precision = self.max_precision.max(other.max_precision);
        self.which.merge(other.which);
        self.nullcount += other.nullcount;
        self.sum_stotlen = self.sum_stotlen.saturating_add(other.sum_stotlen);
        self.sum.merge(other.sum);
        self.modes.merge(other.modes);
        self.unsorted_stats.merge(other.unsorted_stats);
        self.online.merge(other.online);
        self.online_len.merge(other.online_len);
        self.minmax.merge(other.minmax);
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

impl FieldType {
    /// infer data type from a given sample & current type inference
    /// infer_dates signals if date inference should be attempted
    /// returns the inferred type and if infer_dates is true,
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
        if current_type != FieldType::TFloat {
            if let Ok(samp_int) = atoi_simd::parse::<i64>(sample) {
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
        }

        // Check for float
        // we use fast_float2 as it doesn't need to validate the sample as UTF-8 first
        if let Ok(float_sample) = fast_float2::parse::<f64, &[u8]>(sample) {
            return (FieldType::TFloat, 0, float_sample);
        }

        // Only attempt UTF-8 validation and date parsing if infer_dates is true
        if !infer_dates {
            return (FieldType::TString, 0, 0.0);
        }

        // Check if valid UTF-8 first, return early if not
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

impl fmt::Display for FieldType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TNull => write!(f, "NULL"),
            TString => write!(f, "String"),
            TFloat => write!(f, "Float"),
            TInteger => write!(f, "Integer"),
            TDate => write!(f, "Date"),
            TDateTime => write!(f, "DateTime"),
        }
    }
}

impl fmt::Debug for FieldType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TNull => write!(f, "NULL"),
            TString => write!(f, "String"),
            TFloat => write!(f, "Float"),
            TInteger => write!(f, "Integer"),
            TDate => write!(f, "Date"),
            TDateTime => write!(f, "DateTime"),
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
        #[allow(clippy::cast_precision_loss)]
        match typ {
            TInteger => {
                if let Some(ref mut float) = self.float {
                    *float += float_val;
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
            TString => {
                self.stotlen = self.stotlen.saturating_add(sample.len() as u64);
            },
            // we don't need to do anything for TNull, TDate or TDateTime
            // as they don't have a sum
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
                ryu::Buffer::new()
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

/// `TypedMinMax` keeps track of minimum/maximum/range/sort_order values for each possible type
/// where min/max/range/sort_order makes sense.
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

        match typ {
            TString => {
                self.str_len.add(sample_len);
                self.strings.add(sample.to_vec());
            },
            TInteger => {
                self.integers.add(int_val);
                self.floats.add(float_val);
            },
            TFloat => {
                self.floats.add(float_val);
            },
            TNull => {},
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
                    let min_str = String::from_utf8_lossy(min).to_string();
                    let max_str = String::from_utf8_lossy(max).to_string();

                    let max_length = std::env::var("QSV_STATS_STRING_MAX_LENGTH")
                        .ok()
                        .and_then(|s| s.parse::<usize>().ok());

                    let (min_str, max_str) = if let Some(max_len) = max_length {
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
                        ryu::Buffer::new().format(*min).to_owned(),
                        ryu::Buffer::new().format(*max).to_owned(),
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
