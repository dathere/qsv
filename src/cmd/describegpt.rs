static USAGE: &str = r#"
Create a "neuro-procedural" Data Dictionary and/or infer Description & Tags about a Dataset
using an OpenAI API-compatible Large Language Model (LLM).

It does this by compiling Summary Statistics & a Frequency Distribution of the Dataset,
and then prompting the LLM with detailed, configurable, Mini Jinja-templated prompts with
these extended statistical context.

The Data Dictionary is "neuro-procedural" as it uses a hybrid approach. It's primarily populated
deterministically using Summary Statistics & Frequency Distribution data, and only the human-friendly
Label & Description are populated by the "neural network" LLM using the same statistical context.

CHAT MODE:
You can also use the --prompt option to ask a natural language question about the Dataset.

If the question can be answered by solely using the Dataset's Summary Statistics and
Frequency Distribution data, the LLM will return the answer directly.

CHAT SQL RETRIEVAL-AUGMENTED GENERATION (RAG) SUB-MODE:
If the question cannot be answered using the Dataset's Summary Statistics & Frequency Distribution,
it will first create a Data Dictionary and a small random sample (default: 100 rows) of the Dataset
and provide it to the LLM as additional context to help it generate a SQL query that DETERMINISTICALLY
answers the natural language question.

Two SQL dialects are currently supported - DuckDB (highly recommended) & Polars. If the
QSV_DESCRIBEGPT_DB_ENGINE environment variable is set to the absolute path of the DuckDB binary,
DuckDB will be used to answer the question. Otherwise, if the "polars" feature is enabled,
Polars SQL will be used.

If neither DuckDB nor Polars is available, the SQL query will be returned in a Markdown code block,
along with the reasoning behind the query.

Even in "SQL RAG" mode, though the SQL query is guaranteed to be deterministic, the query itself
may not be correct. In the event of a SQL query execution failure, run the same --prompt with
the --fresh option to request the LLM to generate a new SQL query.

When using DuckDB, all loaded DuckDB extensions will be sent as additional context to the LLM to let
it know what functions (even UDFs!) it can use in the SQL queries it generates. If you want a
specific function or technique to be used in the SQL query, mention it in the prompt.

SUPPORTED MODELS & LLM PROVIDERS:
OpenAI's open-weights gpt-oss model (both 20b and 120b variants) was used during development &
is recommended for most use cases.
It was also tested with OpenAI, TogetherAI, OpenRouter and Google Gemini cloud providers.
For Gemini, use the base URL "https://generativelanguage.googleapis.com/v1beta/openai".
Local LLMs tested include Ollama, Jan and LM Studio.

NOTE: LLMs are prone to inaccurate information being produced. Verify output results before using them.

CACHING:
As LLM inferencing takes time and can be expensive, describegpt caches the LLM inferencing results
in a either a disk cache (default) or a Redis cache. It does so by calculating the BLAKE3 hash of the
input file and using it as the primary cache key along with the prompt type, model and other parameters
as required.

The default disk cache is stored in the ~/.qsv/cache/describegpt directory with a default TTL of 28 days
and cache hits NOT refreshing an existing cached value's TTL.
Adjust the QSV_DISKCACHE_TTL_SECS & QSV_DISKCACHE_TTL_REFRESH env vars to change disk cache settings.

Alternatively a Redis cache can be used instead of the disk cache. This is especially useful if you want
to share the cache across the network with other users or computers.
The Redis cache is stored in database 3 by default with a TTL of 28 days and cache hits NOT refreshing
an existing cached value's TTL. Adjust the QSV_DG_REDIS_CONNSTR, QSV_REDIS_MAX_POOL_SIZE,
QSV_REDIS_TTL_SECONDS & QSV_REDIS_TTL_REFRESH env vars to change Redis cache settings.

Examples:

  # Generate a Data Dictionary, Description & Tags of data.csv using default OpenAI gpt-oss-20b model
  # (replace <API_KEY> with your OpenAI API key)
  $ qsv describegpt data.csv --api-key <API_KEY> --all

  # Generate a Data Dictionary of data.csv using the DeepSeek R1:14b model on a local Ollama instance
  $ qsv describegpt data.csv -u http://localhost:11434/v1 --model deepseek-r1:14b --dictionary

  # Ask questions about the sample NYC 311 dataset using LM Studio with the default gpt-oss-20b model.
  # Questions that can be answered using the Summary Statistics & Frequency Distribution of the dataset.
  $ export QSV_LLM_BASE_URL=http://localhost:1234/v1
  $ qsv describegpt NYC_311.csv --prompt "What is the most common complaint?"
  $ qsv describegpt NYC_311.csv --prompt "List the top 10 complaints."
  $ qsv describegpt NYC_311.csv -p "Can you tell me how many complaints were resolved?"

  # Ask detailed questions that require SQL queries and auto-invoke SQL RAG mode
  # Generate a DuckDB SQL query to answer the question
  $ export QSV_DESCRIBEGPT_DB_ENGINE=/path/to/duckdb
  $ qsv describegpt NYC_311.csv -p "What's the breakdown of complaint types by borough descending order?"
  # Prompt requires a SQL query. Execute query and save results to a file with the --sql-results option.
  # If generated SQL query runs successfully, the file is "results.csv". Otherwise, it is "results.sql".
  $ qsv describegpt NYC_311.csv -p "Aggregate complaint types by community board" --sql-results results

  # Cache Dictionary, Description & Tags inference results using the Redis cache instead of the disk cache
  $ qsv describegpt data.csv --all --redis-cache
  # Get fresh Description & Tags inference results from the LLM and refresh disk cache entries for both
  $ qsv describegpt data.csv --description --tags --fresh
  # Get fresh inference results from the LLM and refresh the Redis cache entries for all three
  $ qsv describegpt data.csv --all --redis-cache --fresh

  # Forget a cached response for data.csv's data dictionary if it exists and then exit
  $ qsv describegpt data.csv --dictionary --forget
  # Flush/Remove ALL cached entries in the disk cache
  $ qsv describegpt --flush-cache
  # Flush/Remove ALL cached entries in the Redis cache
  $ qsv describegpt --redis-cache --flush-cache

  # Exclude ID columns from frequency analysis to reduce overhead
  $ qsv describegpt data.csv --dictionary --freq-options "--select '!id,!uuid' --limit 20"

  # Reduce frequency context by showing only top 5 values per field
  $ qsv describegpt data.csv --all --freq-options "--limit 5"

  # Get weighted frequencies with ascending sort
  $ qsv describegpt data.csv --description --freq-options "--limit 50 --asc --weight count_column"

For more examples, see https://github.com/dathere/qsv/blob/master/tests/test_describegpt.rs.

For more detailed info on how describegpt works and how to prepare a prompt file,
see https://github.com/dathere/qsv/blob/master/docs/Describegpt.md

Usage:
    qsv describegpt [options] [<input>]
    qsv describegpt (--redis-cache) (--flush-cache)
    qsv describegpt --help

describegpt options:
                           DATA ANALYSIS/INFERENCING OPTIONS:
    --dictionary           Create a Data Dictionary using a hybrid "neuro-procedural" pipeline - i.e.
                           the Dictionary is populated deterministically using Summary Statistics and
                           Frequency Distribution data, and only the human-friendly Label and Description
                           are populated by the LLM using the same statistical context.
    --description          Infer a general Description of the dataset based on detailed statistical context.
                           An Attribution signature is embedded in the Description.
    --tags                 Infer Tags that categorize the dataset based on detailed statistical context.
                           Useful for grouping datasets and filtering.
    -A, --all              Shortcut for --dictionary --description --tags.

                           DICTIONARY OPTIONS:
    --num-examples <n>     The number of Example values to include in the dictionary.
                           [default: 5]
    --truncate-str <n>     The maximum length of an Example value in the dictionary.
                           An ellipsis is appended to the truncated value.
                           If zero, no truncation is performed.
                           [default: 25]
    --addl-cols            Add additional columns to the dictionary from the Summary Statistics.
  --addl-cols-list <list>  A comma-separated list of additional stats columns to add to the dictionary.
                           The columns must be present in the Summary Statistics.
                           If the columns are not present in the Summary Statistics or already in the
                           dictionary, they will be ignored.
                           CONVENIENCE VALUES:
                           These values are case-insensitive and automatically set the --addl-cols option to true.
                           "everything" can be used to add all 45 "available" statistics columns.
                           You can adjust the available columns with --stats-options.
                           "everything!" automatically sets --stats-options to compute "all" 51 supported stats.
                           The 6 addl cols are the mode/s & antimode/s stats with each having counts & occurrences.
                           "moar" gets you even moar stats, with detailed outliers info.
                           "moar!" gets you even moar with --advanced stats (Kurtosis, Gini Coefficient & Shannon Entropy)
                           [default: sort_order, sortiness, mean, median, mad, stddev, variance, cv]

                           TAG OPTIONS:
    --num-tags <n>         The maximum number of tags to infer when the --tags option is used.
                           Maximum allowed value is 50.
                           [default: 10]
    --tag-vocab <file>     The CSV file containing the tag vocabulary to use for inferring tags.
                           If no tag vocabulary file is provided, the model will use free-form tags.
                           Supports local files, remote URLs (http/https), CKAN resources (ckan://),
                           and dathere:// scheme. Remote resources are cached locally.
                           The CSV file must have two columns with headers: first column is the tag,
                           second column is the description. Note that qsvlite only supports local files.
    --cache-dir <dir>      The directory to use for caching downloaded tag vocabulary resources.
                           If the directory does not exist, qsv will attempt to create it.
                           If the QSV_CACHE_DIR envvar is set, it will be used instead.
                           [default: ~/.qsv-cache]
    --ckan-api <url>       The URL of the CKAN API to use for downloading tag vocabulary resources
                           with the "ckan://" scheme.
                           If the QSV_CKAN_API envvar is set, it will be used instead.
                           [default: https://data.dathere.com/api/3/action]
    --ckan-token <token>   The CKAN API token to use. Only required if downloading private resources.
                           If the QSV_CKAN_TOKEN envvar is set, it will be used instead.

                           STATS/FREQUENCY OPTIONS:
    --stats-options <arg>  Options for the stats command used to generate summary statistics.
                           [default: --infer-dates --infer-boolean --mad --quartiles --percentiles --force --stats-jsonl]
    --freq-options <arg>   Options for the frequency command used to generate frequency distributions.
                           You can use this to exclude certain variable types from frequency analysis
                           (e.g., --select '!id,!uuid'), limit results differently per use case, or
                           control output format. If --limit is specified here, it takes precedence
                           over --enum-threshold.
                           [default: --rank-strategy dense]
    --enum-threshold <n>   The threshold for compiling Enumerations with the frequency command
                           before bucketing other unique values into the "Other" category.
                           This is a convenience shortcut for --freq-options --limit <n>.
                           If --freq-options contains --limit, this flag is ignored.
                           [default: 10]

                           CUSTOM PROMPT OPTIONS:
    -p, --prompt <prompt>  Custom prompt to answer questions about the dataset.
                           The prompt will be answered based on the dataset's Summary Statistics,
                           Frequency data & Data Dictionary. If the prompt CANNOT be answered by looking
                           at these metadata, a SQL query will be generated to answer the question.
                           If the "polars" or the "QSV_DESCRIBEGPT_DB_ENGINE" environment variable is set
                           & the `--sql-results` option is used, the SQL query will be automatically
                           executed and its results returned.
                           Otherwise, the SQL query will be returned along with the reasoning behind it.
                           If it starts with "file:" prefix, the prompt is read from the file specified.
                           e.g. "file:my_long_prompt.txt"
    --sql-results <file>   The file to save the SQL query results to.
                           Only valid if the --prompt option is used & the "polars" or the
                           "QSV_DESCRIBEGPT_DB_ENGINE" environment variable is set.
                           If the SQL query executes successfully, the results will be saved with a
                           ".csv" extension. Otherwise, it will be saved with a ".sql" extension so
                           the user can inspect why it failed and modify it.
    --prompt-file <file>   The configurable TOML file containing prompts to use for inferencing.
                           If no file is provided, default prompts will be used.
                           The prompt file uses the Mini Jinja template engine (https://docs.rs/minijinja)
                           See https://github.com/dathere/qsv/blob/master/resources/describegpt_defaults.toml
    --sample-size <n>      The number of rows to randomly sample from the input file for the sample data.
                           Uses the INDEXED sampling method with the qsv sample command.
                           [default: 100]
    --fewshot-examples     By default, few-shot examples are NOT included in the LLM prompt when
                           generating SQL queries. When this option is set, few-shot examples in the default
                           prompt file are included.
                           Though this will increase the quality of the generated SQL, it comes at
                           a cost - increased LLM API call cost in terms of tokens and execution time.
                           See https://en.wikipedia.org/wiki/Prompt_engineering for more info.
    --session <name>       Enable stateful session mode for iterative SQL RAG refinement.
                           The session name is the file path of the markdown file where session messages
                           will be stored. When used with --prompt, subsequent queries in the same session
                           will refine the baseline SQL query. SQL query results (10-row sample) and errors
                           are automatically included in subsequent messages for context.
    --session-len <n>      Maximum number of recent messages to keep in session context before
                           summarizing older messages. Only used when --session is specified.
                           [default: 10]

                           LLM API OPTIONS:
    -u, --base-url <url>   The LLM API URL. Supports APIs & local LLMs compatible with
                           the OpenAI API specification (Ollama, Jan, LM Studio, TogetherAI, etc.).
                           The default base URL for Ollama is http://localhost:11434/v1.
                           The default for Jan is https://localhost:1337/v1.
                           The default for LM Studio is http://localhost:1234/v1.
                           The base URL will be the base URL of the prompt file.
                           If the QSV_LLM_BASE_URL environment variable is set, it'll be used instead.
                           [default: https://api.openai.com/v1]
    -m, --model <model>    The model to use for inferencing.
                           If the QSV_LLM_MODEL environment variable is set, it'll be used instead.
                           [default: openai/gpt-oss-20b]
    --language <lang>      The output language/dialect to use for the response. (e.g., "Spanish", "French",
                           "Hindi", "Mandarin", "Italian", "Castilian", "Franglais", "Taglish", "Pig Latin",
                           "Valley Girl", "Pirate", "Shakespearean English", "Chavacano", "Gen Z", "Yoda", etc.)
    
                             CHAT MODE (--prompt) LANGUAGE DETECTION BEHAVIOR:
                             When --prompt is used and --language is not set, automatically detects
                             the language of the prompt with an 80% confidence threshold.
                             If the threshold is met, it will specify the detected language in its response.
                             If set to a float (0.0 to 1.0), specifies the detection confidence threshold.
                             If set to a string, specifies the language/dialect to use for the response.
                             Note that LLMs often detect the language independently, but will often respond
                             in the model's default language. This option is here to ensure responses are
                             in the detected language of the prompt.
    --addl-props <json>    Additional model properties to pass to the LLM chat/completion API.
                           Various models support different properties beyond the standard ones.
                           For instance, gpt-oss-20b supports the "reasoning_effort" property.
                           e.g. to set the "reasoning_effort" property to "high" & "temperature"
                           to 0.5, use '{"reasoning_effort": "high", "temperature": 0.5}'
    -k, --api-key <key>    The API key to use. If the QSV_LLM_APIKEY envvar is set,
                           it will be used instead. Required when the base URL is not localhost.
                           Set to NONE to suppress sending the API key.
    -t, --max-tokens <n>   Limits the number of generated tokens in the output.
                           Set to 0 to disable token limits.
                           If the --base-url is localhost, indicating a local LLM,
                           the default is automatically set to 0.
                           [default: 10000]
    --timeout <secs>       Timeout for completions in seconds. If 0, no timeout is used.
                           [default: 300]
    --user-agent <agent>   Specify custom user agent. It supports the following variables -
                           $QSV_VERSION, $QSV_TARGET, $QSV_BIN_NAME, $QSV_KIND and $QSV_COMMAND.
                           Try to follow the syntax here -
                           https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/User-Agent
  --export-prompt <file>   Export the default prompts to the specified file that
                           can be used with the --prompt-file option.
                           The file will be saved with a .toml extension.
                           If the file already exists, it will be overwritten.
                           It will exit after exporting the prompts.

                           CACHING OPTIONS:
    --no-cache             Disable default disk cache.
  --disk-cache-dir <dir>   The directory <dir> to store the disk cache. Note that if the directory
                           does not exist, it will be created. If the directory exists, it will be used as is,
                           and will not be flushed. This option allows you to maintain several disk caches
                           for different describegpt jobs (e.g. one for a data portal, another for internal
                           data exchange, etc.)
                           [default: ~/.qsv/cache/describegpt]
    --redis-cache          Use Redis instead of the default disk cache to cache LLM completions.
                           It connects to "redis://127.0.0.1:6379/3" by default, with a connection pool
                           size of 20, with a TTL of 28 days, and cache hits NOT refreshing an existing
                           cached value's TTL.
                           This option automatically disables the disk cache.
    --fresh                Send a fresh request to the LLM API, refreshing a cached response if it exists.
                           When a --prompt SQL query fails, you can also use this option to request the
                           LLM to generate a new SQL query.
    --forget               Remove a cached response if it exists and then exit.
    --flush-cache          Flush the current cache entries on startup.
                           WARNING: This operation is irreversible.

Common options:
    -h, --help             Display this message
    --format <format>      Output format: Markdown, TSV, JSON, or TOON.
                           TOON is a compact, human-readable encoding of the JSON data model for LLM prompts.
                           See https://toonformat.dev/ for more info.
                           [default: Markdown]
    -o, --output <file>    Write output to <file> instead of stdout. If --format is set to TSV,
                           separate files will be created for each prompt type with the pattern
                           {filestem}.{kind}.tsv (e.g., output.dictionary.tsv, output.tags.tsv).
    -q, --quiet            Do not print status messages to stderr.
"#;

use std::{
    collections::HashMap,
    env, fs,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
    sync::OnceLock,
    time::{Duration, Instant},
};

use cached::{
    DiskCache, IOCached, RedisCache, Return, proc_macro::io_cached, stores::DiskCacheBuilder,
};
use indexmap::{IndexMap, IndexSet};
use indicatif::HumanCount;
use minijinja::{Environment, context};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use strum_macros::{Display, EnumString};
use toml;
use toon_format::{EncodeOptions, encode};
#[cfg(feature = "whatlang")]
use whatlang::detect;

use crate::{
    CliError, CliResult,
    config::Config,
    regex_oncelock, util,
    util::{QUIET_FLAG, print_status, process_input, run_qsv_cmd},
};
#[cfg(feature = "feature_capable")]
use crate::{lookup, lookup::LookupTableOptions};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, Display)]
#[strum(ascii_case_insensitive)]
enum PromptType {
    Dictionary,
    Description,
    Tags,
    Prompt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AttributionFormat {
    Markdown,
    SqlComment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OutputFormat {
    Markdown,
    Tsv,
    Json,
    Toon,
}
#[derive(Debug, Deserialize)]
struct Args {
    arg_input:             Option<String>,
    flag_dictionary:       bool,
    flag_description:      bool,
    flag_tags:             bool,
    flag_all:              bool,
    flag_num_tags:         u16,
    flag_tag_vocab:        Option<String>,
    #[allow(dead_code)]
    flag_cache_dir:        String,
    #[allow(dead_code)]
    flag_ckan_api:         String,
    #[allow(dead_code)]
    flag_ckan_token:       Option<String>,
    flag_stats_options:    String,
    flag_freq_options:     String,
    flag_enum_threshold:   usize,
    flag_num_examples:     u16,
    flag_truncate_str:     usize,
    flag_prompt:           Option<String>,
    flag_sql_results:      Option<String>,
    flag_prompt_file:      Option<String>,
    flag_sample_size:      u16,
    flag_fewshot_examples: bool,
    flag_base_url:         Option<String>,
    flag_model:            Option<String>,
    flag_language:         Option<String>,
    flag_addl_props:       Option<String>,
    flag_api_key:          Option<String>,
    flag_max_tokens:       u32,
    flag_timeout:          u16,
    flag_user_agent:       Option<String>,
    flag_export_prompt:    Option<String>,
    flag_no_cache:         bool,
    flag_disk_cache_dir:   Option<String>,
    flag_redis_cache:      bool,
    flag_fresh:            bool,
    flag_forget:           bool,
    flag_flush_cache:      bool,
    flag_format:           Option<String>,
    flag_output:           Option<String>,
    flag_quiet:            bool,
    flag_addl_cols:        bool,
    flag_addl_cols_list:   Option<String>,
    flag_session:          Option<String>,
    flag_session_len:      usize,
}

#[derive(Debug, Clone)]
struct SessionMessage {
    role:      String,
    content:   String,
    #[allow(dead_code)]
    timestamp: String,
}

#[derive(Debug, Clone)]
struct SessionState {
    baseline_sql: Option<String>,
    messages:     Vec<SessionMessage>,
    sql_results:  Option<String>,
    sql_errors:   Vec<String>,
    summary:      Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct PromptFile {
    name:                   String,
    description:            String,
    author:                 String,
    version:                String,
    tokens:                 u32,
    system_prompt:          String,
    dictionary_prompt:      String,
    description_prompt:     String,
    tags_prompt:            String,
    prompt:                 String,
    format:                 String,
    language:               String,
    base_url:               String,
    model:                  String,
    timeout:                u32,
    custom_prompt_guidance: String,
    duckdb_sql_guidance:    String,
    polars_sql_guidance:    String,
    dd_fewshot_examples:    String, //DuckDB few-shot examples
    p_fewshot_examples:     String, //Polars SQL few-shot examples
}

const DEFAULT_BASE_URL: &str = "https://api.openai.com/v1";
const DEFAULT_MODEL: &str = "openai/gpt-oss-20b";
const LLM_APIKEY_ERROR: &str = r#"Error: Neither QSV_LLM_BASE_URL nor QSV_LLM_APIKEY environment variables are set.
Either set `--base-url` to an address with "localhost" in it (indicating a local LLM), or set `--api-key`.
If your Local LLM is not running on localhost, set QSV_LLM_APIKEY or `--api-key` to NONE.

Note that this command uses LLMs for inferencing and is therefore prone to inaccurate information being produced.
Verify output results before using them."#;

const INPUT_TABLE_NAME: &str = "{INPUT_TABLE_NAME}";

const DEFAULT_LANGDETECTION_THRESHOLD: f64 = 0.8; // 80% default confidence threshold
static DETECTED_LANGUAGE: OnceLock<String> = OnceLock::new();
static DETECTED_LANGUAGE_CONFIDENCE: OnceLock<f64> = OnceLock::new();

static DUCKDB_PATH: OnceLock<String> = OnceLock::new();
static SAMPLE_FILE: OnceLock<String> = OnceLock::new();

static DATA_DICTIONARY_JSON: OnceLock<String> = OnceLock::new();

#[cfg(feature = "feature_capable")]
static TAG_VOCAB_CACHE_DIR: OnceLock<String> = OnceLock::new();
#[cfg(feature = "feature_capable")]
static TAG_VOCAB_CKAN_API: OnceLock<String> = OnceLock::new();
#[cfg(feature = "feature_capable")]
static TAG_VOCAB_CKAN_TOKEN: OnceLock<Option<String>> = OnceLock::new();

#[allow(dead_code)]
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
struct TokenUsage {
    prompt:     u64,
    completion: u64,
    total:      u64,
    elapsed:    u64,
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub enum CacheType {
    #[default]
    None,
    Disk,
    Redis,
    Fresh, // Forces fresh API call but still updates cache
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
struct CompletionResponse {
    response:    String,
    reasoning:   String,
    token_usage: TokenUsage,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
struct AnalysisResults {
    stats:     String,
    frequency: String,
    headers:   String,
    file_hash: String,
    delimiter: char,
}

// Data structures for neuro-procedural dictionary generation
#[derive(Debug, Clone, Serialize, Deserialize)]
struct DictionaryEntry {
    name:        String,
    r#type:      String,
    label:       String,
    description: String,
    min:         String, // Empty string if not available
    max:         String, // Empty string if not available
    cardinality: u64,
    enumeration: String, // Empty string if not enumerable, otherwise values on separate lines
    null_count:  u64,
    addl_cols:   IndexMap<String, String>, // Addl columns from stats (preserves order)
    examples:    String,                   // Format: "val1 [cnt1], ... or "<ALL_UNIQUE>"
}

// Helper structs for parsing CSV data
#[derive(Debug, Clone)]
struct StatsRecord {
    field:       String,
    r#type:      String,
    cardinality: u64,
    nullcount:   u64,
    min:         String,                   // Empty string if not available
    max:         String,                   // Empty string if not available
    addl_cols:   IndexMap<String, String>, // Addl columns from stats CSV (preserves order)
}

#[derive(Debug, Clone)]
struct FrequencyRecord {
    field:      String,
    value:      String,
    count:      u64,
    percentage: f64,
    rank:       f64,
}

// environment variables
static QSV_REDIS_CONNSTR_ENV: &str = "QSV_DG_REDIS_CONNSTR";
static QSV_REDIS_MAX_POOL_SIZE_ENV: &str = "QSV_REDIS_MAX_POOL_SIZE";
static QSV_REDIS_TTL_SECS_ENV: &str = "QSV_REDIS_TTL_SECS";
static QSV_REDIS_TTL_REFRESH_ENV: &str = "QSV_REDIS_TTL_REFRESH";
static QSV_DESCRIBEGPT_DB_ENGINE_ENV: &str = "QSV_DESCRIBEGPT_DB_ENGINE";

// Shared regex for matching read_csv_auto function calls
static READ_CSV_AUTO_REGEX: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
    regex::Regex::new("read_csv_auto\\([^)]*\\)").expect("Invalid regex pattern")
});

/// Escape a string for safe usage as a SQL string literal.
///
/// This function ensures that common problematic characters (such as single quotes, backslashes,
/// newlines, carriage returns, and null bytes) are properly escaped according to SQL string
/// literal rules.
///
/// - Single quotes are escaped by doubling them (`'` → `''`), as per the SQL standard.
/// - Backslashes are escaped by doubling (`\` → `\\`). Backslash escaping is non-standard SQL but
///   prevents certain injection scenarios, and must come first in this implementation.
/// - Newline (`\n`), carriage return (`\r`), and null byte (`\0`) are replaced by their C-like
///   escape sequence representations (`\\n`, `\\r`, `\\0`).
fn escape_sql_string(s: &str) -> String {
    s.replace('\\', "\\\\") // Backslash must be first!
        .replace('\'', "''")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\0', "\\0")
}

static DEFAULT_REDIS_CONN_STRING: OnceLock<String> = OnceLock::new();
static DEFAULT_REDIS_TTL_SECS: u64 = 60 * 60 * 24 * 28; // 28 days in seconds
static DEFAULT_REDIS_POOL_SIZE: u32 = 20;

// disk cache TTL is also 28 days by default
static DEFAULT_DISKCACHE_TTL_SECS: u64 = 60 * 60 * 24 * 28;

static DISKCACHE_DIR: OnceLock<String> = OnceLock::new();
static REDISCONFIG: OnceLock<RedisConfig> = OnceLock::new();
static DISKCACHECONFIG: OnceLock<DiskCacheConfig> = OnceLock::new();

#[derive(Debug)]
struct RedisConfig {
    conn_str:      String,
    max_pool_size: u32,
    ttl_secs:      Duration,
    ttl_refresh:   bool,
}
impl RedisConfig {
    fn new() -> RedisConfig {
        Self {
            conn_str:      std::env::var(QSV_REDIS_CONNSTR_ENV)
                .unwrap_or_else(|_| DEFAULT_REDIS_CONN_STRING.get().unwrap().to_string()),
            max_pool_size: std::env::var(QSV_REDIS_MAX_POOL_SIZE_ENV)
                .unwrap_or_else(|_| DEFAULT_REDIS_POOL_SIZE.to_string())
                .parse()
                .unwrap_or(DEFAULT_REDIS_POOL_SIZE),
            ttl_secs:      Duration::from_secs(
                std::env::var(QSV_REDIS_TTL_SECS_ENV)
                    .unwrap_or_else(|_| DEFAULT_REDIS_TTL_SECS.to_string())
                    .parse()
                    .unwrap_or(DEFAULT_REDIS_TTL_SECS),
            ),
            ttl_refresh:   util::get_envvar_flag(QSV_REDIS_TTL_REFRESH_ENV),
        }
    }
}

#[derive(Debug)]
struct DiskCacheConfig {
    ttl_secs:    Duration,
    ttl_refresh: bool,
}
impl DiskCacheConfig {
    fn new() -> DiskCacheConfig {
        Self {
            ttl_secs:    Duration::from_secs(
                std::env::var("QSV_DISKCACHE_TTL_SECS")
                    .unwrap_or_else(|_| DEFAULT_DISKCACHE_TTL_SECS.to_string())
                    .parse()
                    .unwrap_or(DEFAULT_DISKCACHE_TTL_SECS),
            ),
            ttl_refresh: util::get_envvar_flag("QSV_DISKCACHE_TTL_REFRESH"),
        }
    }
}

static FILE_HASH: OnceLock<String> = OnceLock::new();
static PROMPT_FILE: OnceLock<PromptFile> = OnceLock::new();
static PROMPT_VALIDITY_FLAGS: std::sync::LazyLock<
    std::sync::Mutex<std::collections::HashMap<String, String>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(std::collections::HashMap::new()));

/// Detect language from prompt text using whatlang
/// Returns the detected language name if confidence >= threshold, otherwise None
/// Default threshold is 0.8 (80%)
#[cfg(feature = "whatlang")]
fn detect_language_from_prompt(prompt: &str, threshold: f64) -> Option<String> {
    let lang_info = detect(prompt);
    if let Some(lang_info) = lang_info {
        let detected_lang = lang_info.lang().eng_name();
        let lang_confidence = lang_info.confidence();

        // safety: these all have valid values, so it's safe to unwrap
        DETECTED_LANGUAGE.set(detected_lang.to_string()).unwrap();
        DETECTED_LANGUAGE_CONFIDENCE.set(lang_confidence).unwrap();

        if lang_confidence >= threshold {
            Some(detected_lang.to_string())
        } else {
            None
        }
    } else {
        None
    }
}

/// Parse the --language option: if it's autodetect, a threshold, or an explicit language
/// Returns (is_autodetect, threshold, explicit_language)
/// - is_autodetect: true if language should be auto-detected
/// - threshold: confidence threshold for autodetect (0.0-1.0)
/// - explicit_language: Some(language) if an explicit language was specified, None otherwise
fn parse_language_option(language: Option<&String>) -> (bool, f64, Option<String>) {
    if let Some(lang) = language {
        // Try to parse as a number (threshold)
        if let Ok(threshold_float) = lang.parse::<f64>() {
            // Float 0.0-1.0
            if (0.0..=1.0).contains(&threshold_float) {
                (true, threshold_float, None)
            } else {
                // Invalid float, treat as explicit language
                (false, DEFAULT_LANGDETECTION_THRESHOLD, Some(lang.clone()))
            }
        } else {
            // Not a number, treat as explicit language string
            (false, DEFAULT_LANGDETECTION_THRESHOLD, Some(lang.clone()))
        }
    } else {
        (true, DEFAULT_LANGDETECTION_THRESHOLD, None)
    }
}

// Check if DuckDB should be used based on environment variable
fn should_use_duckdb() -> bool {
    env::var(QSV_DESCRIBEGPT_DB_ENGINE_ENV)
        .map(|val| val.to_lowercase().contains("duckdb"))
        .unwrap_or(false)
}

// Get DuckDB binary path from environment variable
fn get_duckdb_path() -> CliResult<String> {
    // Return cached path if already initialized
    if let Some(path) = DUCKDB_PATH.get() {
        return Ok(path.clone());
    }

    let duckdb_path = env::var(QSV_DESCRIBEGPT_DB_ENGINE_ENV)
        .map_err(|_| "QSV_DESCRIBEGPT_DB_ENGINE env var not set")?;

    // Check if the binary exists
    let path = Path::new(&duckdb_path);
    if !path.exists() {
        return fail_clierror!("DuckDB binary not found at path: {duckdb_path}");
    }
    if !path.is_file() {
        return fail_clierror!("DuckDB path is not a file: {duckdb_path}");
    }
    if !util::is_executable(&duckdb_path)? {
        return fail_clierror!("DuckDB path is not executable: {duckdb_path}");
    }

    // Cache the path
    // safety: we're only setting the path once, so it's safe to unwrap
    DUCKDB_PATH.set(duckdb_path.clone()).unwrap();

    Ok(duckdb_path)
}

/// Sends an HTTP request using the provided client and parameters.
///
/// # Arguments
///
/// * `client` - The HTTP client used to make the request
/// * `api_key` - Optional API key for authentication via Bearer token
/// * `request_data` - Optional JSON data to include in POST requests
/// * `method` - HTTP method to use ("GET" or "POST")
/// * `url` - The URL to send the request to
///
/// # Returns
///
/// Returns a `CliResult` containing the HTTP response on success.
///
/// # Errors
///
/// Returns a `CliError` if:
/// * An unsupported HTTP method is specified
/// * GET request includes request data
/// * POST request is missing required request data
/// * The HTTP request fails
/// * The response has a non-success status code
fn send_request(
    client: &Client,
    api_key: Option<&str>,
    request_data: Option<&serde_json::Value>,
    method: &str,
    url: &str,
) -> CliResult<reqwest::blocking::Response> {
    // Build request based on method
    let mut request = match method {
        "GET" => {
            if request_data.is_some() {
                return fail_clierror!("GET requests cannot include request data");
            }
            client.get(url)
        },
        "POST" => {
            let Some(data) = request_data else {
                return fail_clierror!("POST requests require request data");
            };
            client
                .post(url)
                .header("Content-Type", "application/json")
                .body(data.to_string())
        },
        other => {
            let error_json = json!({"Unsupported HTTP method ": other});
            return fail_clierror!("{error_json}");
        },
    };

    // Add API key header if provided
    if let Some(key) = api_key
        && !key.is_empty()
    {
        request = request.header("Authorization", format!("Bearer {key}"));
    }

    // Send request and handle response
    let response = request.send()?;

    // Check for HTTP error status
    if !response.status().is_success() {
        let status = response.status();
        let output = response
            .text()
            .unwrap_or_else(|_| "Unable to read error response".to_string());
        return fail_clierror!("HTTP {status} error: {output}");
    }

    Ok(response)
}

/// Validates the provided model against available models from the API endpoint.
///
/// # Arguments
///
/// * `client` - The HTTP client used to make API requests
/// * `api_key` - Optional API key for authentication
/// * `args` - Command line arguments containing model and configuration options
///
/// # Returns
///
/// Returns a valid model string, either exactly matching the provided model or a suffix match.
///
/// # Details
///
/// This function:
/// 1. Gets the base URL and model from prompt file or command line args
/// 2. Makes a GET request to the /models endpoint
/// 3. Checks for an exact match of the provided model
/// 4. If no exact match, tries to find a model ending with the provided string
/// 5. Returns the first matching model found
///
/// # Errors
///
/// Returns a CliError if:
/// * The API request fails
/// * The response cannot be parsed as JSON
/// * No matching model is found (includes list of valid models in error)
fn check_model(client: &Client, api_key: Option<&str>, args: &Args) -> CliResult<String> {
    // Get prompt file if --prompt-file is used, otherwise get default prompt file
    let prompt_file = get_prompt_file(args)?;
    let models_endpoint = "/models";
    let base_url = if args.flag_prompt_file.is_some() {
        prompt_file.base_url.clone()
    } else {
        // safety: base_url has a docopt default
        args.flag_base_url.as_deref().unwrap().to_string()
    };
    let response = send_request(
        client,
        api_key,
        None,
        "GET",
        format!("{base_url}{models_endpoint}").as_str(),
    );

    // Get response and parse JSON
    let response = response?;
    let response_json: serde_json::Value = response.json()?;

    // Handle both OpenAI format (with "data" field) and Together format (direct array)
    let models = if let Some(data_array) = response_json["data"].as_array() {
        data_array //OpenAI
    } else if let Some(direct_array) = response_json.as_array() {
        direct_array //Together AI
    } else {
        return fail_clierror!(
            "Invalid response: expected either 'data' field with array or direct array\n\n{}",
            simd_json::to_string_pretty(&response_json).unwrap_or_default()
        );
    };

    let given_model = prompt_file.model.clone();

    // Check for exact model match
    for model in models {
        if let Some(model_id) = model["id"].as_str()
            && model_id == given_model
        {
            return Ok(given_model);
        }
    }

    // Check for partial model match (suffix matching)
    for model in models {
        if let Some(model_id) = model["id"].as_str()
            && model_id.ends_with(&given_model)
        {
            print_status(&format!("  Using model: {model_id}"), None);
            return Ok(model_id.to_string());
        }
    }

    // Otherwise, fail with list of valid models
    let models_list = models
        .iter()
        .filter_map(|m| m["id"].as_str())
        .collect::<Vec<_>>()
        .join(", ");
    fail_clierror!("Invalid model: {given_model}\n  Valid models: {models_list}")
}

/// Returns the default prompt file content as a string.
const fn get_default_prompt_file_content() -> &'static str {
    include_str!("../../resources/describegpt_defaults.toml")
}

/// Retrieves or initializes a prompt file configuration from either a provided file or defaults.
///
/// # Arguments
///
/// * `args` - Command line arguments containing prompt file path and other configuration options
///
/// # Returns
///
/// Returns a reference to the global PromptFile configuration
///
/// # Details
///
/// This function:
/// 1. Checks if a prompt file is already loaded in the global PROMPT_FILE
/// 2. If not, loads either a custom prompt file or the default one
/// 3. Applies any overrides from environment variables or command line flags
/// 4. Updates the configuration with max tokens, model, and system prompt settings
/// 5. Stores the result in the global PROMPT_FILE
///
/// Environment variables that affect behavior:
/// * QSV_LLM_BASE_URL - Override the base URL for API calls
/// * QSV_LLM_MODEL - Override the model to use
///
/// # Errors
///
/// Returns a CliError if:
/// * The prompt file cannot be read
/// * The TOML parsing fails
/// * The global PROMPT_FILE cannot be set
fn get_prompt_file(args: &Args) -> CliResult<&PromptFile> {
    if let Some(prompt_file) = PROMPT_FILE.get() {
        Ok(prompt_file)
    } else {
        let prompt_file_content = if let Some(ref prompt_file) = args.flag_prompt_file {
            &fs::read_to_string(prompt_file)?
        } else {
            // If no prompt file is provided, use the default prompt file
            get_default_prompt_file_content()
        };

        // Try to parse prompt file as TOML
        let mut prompt_file: PromptFile = match toml::from_str(prompt_file_content) {
            Ok(val) => val,
            Err(e) => {
                return fail_clierror!("Prompt file parsing error: {e}");
            },
        };

        // Priority: Explicit CLI flag > Env var > Prompt file base_url
        // Check if user explicitly provided --base-url (not the default value)
        if args.flag_base_url.as_deref() != Some(DEFAULT_BASE_URL) {
            // User explicitly provided a different base URL, use it
            // safety: args.flag_base_url is guaranteed to be Some here, as it
            // differs from the docopt default DEFAULT_BASE_URL checked above.
            let base_url = args.flag_base_url.as_ref().unwrap();
            prompt_file.base_url.clone_from(base_url);
        } else if let Ok(base_url) = env::var("QSV_LLM_BASE_URL") {
            // User didn't provide explicit --base-url, but env var is set
            prompt_file.base_url = base_url;
        }
        // else: keep the base_url from the prompt file

        // Priority: Explicit CLI flag > Env var > Prompt file model
        // The --model flag has a docopt default
        let model_to_use = if args.flag_model.as_deref() != Some(DEFAULT_MODEL) {
            // User explicitly provided a different model via CLI, use it
            args.flag_model.clone().unwrap() // safety: flag_model has a docopt default
        } else if let Ok(env_model) = env::var("QSV_LLM_MODEL") {
            // User didn't provide explicit --model, but env var is set
            env_model
        } else {
            // Use prompt file model or default
            prompt_file.model.clone()
        };

        prompt_file.model = model_to_use;

        // If max_tokens is 0 or the base URL contains "localhost", disable max_tokens limit
        let max_tokens = if args.flag_max_tokens == 0 || prompt_file.base_url.contains("localhost")
        {
            0
        } else if args.flag_max_tokens > 0 {
            args.flag_max_tokens
        } else {
            prompt_file.tokens
        };
        prompt_file.tokens = max_tokens;
        prompt_file.system_prompt = prompt_file
            .system_prompt
            .replace("{TOP_N}", &args.flag_enum_threshold.to_string());

        // Set the global prompt file
        PROMPT_FILE.set(prompt_file).unwrap();
        Ok(PROMPT_FILE.get().unwrap())
    }
}

/// Parse stats CSV into structured records
/// Returns the records and the ordered list of additional column names (in CSV order)
fn parse_stats_csv(stats_csv: &str) -> CliResult<(Vec<StatsRecord>, Vec<String>)> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_csv.as_bytes());

    let headers = rdr.headers()?.clone();

    // Standard column names that we handle explicitly
    let std_cols: std::collections::HashSet<&str> =
        ["field", "type", "cardinality", "nullcount", "min", "max"]
            .iter()
            .copied()
            .collect();

    // Find column indices for standard columns
    let field_idx = headers
        .iter()
        .position(|h| h == "field")
        .ok_or_else(|| CliError::Other("Stats CSV missing 'field' column".to_string()))?;

    let type_idx = headers
        .iter()
        .position(|h| h == "type")
        .ok_or_else(|| CliError::Other("Stats CSV missing 'type' column".to_string()))?;

    let cardinality_idx = headers.iter().position(|h| h == "cardinality");
    let nullcount_idx = headers
        .iter()
        .position(|h| h == "nullcount")
        .ok_or_else(|| CliError::Other("Stats CSV missing 'nullcount' column".to_string()))?;
    let min_idx = headers.iter().position(|h| h == "min");
    let max_idx = headers.iter().position(|h| h == "max");

    // Collect indices of additional (non-standard) columns
    let addl_col_indices: Vec<(usize, String)> = headers
        .iter()
        .enumerate()
        .filter_map(|(idx, header)| {
            if std_cols.contains(header) {
                None
            } else {
                Some((idx, header.to_string()))
            }
        })
        .collect();

    let mut records = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let field = record
            .get(field_idx)
            .ok_or_else(|| CliError::Other("Stats CSV record missing field value".to_string()))?
            .to_string();

        let r#type = record
            .get(type_idx)
            .ok_or_else(|| CliError::Other("Stats CSV record missing type value".to_string()))?
            .to_string();

        let cardinality = cardinality_idx
            .and_then(|idx| record.get(idx))
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);

        let nullcount = record
            .get(nullcount_idx)
            .ok_or_else(|| CliError::Other("Stats CSV record missing nullcount value".to_string()))?
            .parse::<u64>()
            .map_err(|e| CliError::Other(format!("Failed to parse nullcount: {e}")))?;

        let min = min_idx
            .and_then(|idx| record.get(idx))
            .map(std::string::ToString::to_string)
            .unwrap_or_default();

        let max = max_idx
            .and_then(|idx| record.get(idx))
            .map(std::string::ToString::to_string)
            .unwrap_or_default();

        // Collect additional columns, preserving CSV order
        // Ensure all cols are present (w/ empty string if missing) to maintain consistent order
        let mut addl_cols = IndexMap::new();
        for (idx, col_name) in &addl_col_indices {
            let value = record
                .get(*idx)
                .map(std::string::ToString::to_string)
                .unwrap_or_default();
            addl_cols.insert(col_name.clone(), value);
        }

        records.push(StatsRecord {
            field,
            r#type,
            cardinality,
            nullcount,
            min,
            max,
            addl_cols,
        });
    }

    // Extract ordered column names (preserving CSV order)
    let ordered_col_names: Vec<String> =
        addl_col_indices.into_iter().map(|(_, name)| name).collect();

    Ok((records, ordered_col_names))
}

/// Parse frequency CSV into structured records
fn parse_frequency_csv(frequency_csv: &str) -> CliResult<Vec<FrequencyRecord>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(frequency_csv.as_bytes());

    let headers = rdr.headers()?.clone();

    // Find column indices
    let field_idx = headers
        .iter()
        .position(|h| h == "field")
        .ok_or_else(|| CliError::Other("Frequency CSV missing 'field' column".to_string()))?;

    let value_idx = headers
        .iter()
        .position(|h| h == "value")
        .ok_or_else(|| CliError::Other("Frequency CSV missing 'value' column".to_string()))?;

    let count_idx = headers
        .iter()
        .position(|h| h == "count")
        .ok_or_else(|| CliError::Other("Frequency CSV missing 'count' column".to_string()))?;

    let percentage_idx = headers
        .iter()
        .position(|h| h == "percentage")
        .ok_or_else(|| CliError::Other("Frequency CSV missing 'percentage' column".to_string()))?;

    let rank_idx = headers
        .iter()
        .position(|h| h == "rank")
        .ok_or_else(|| CliError::Other("Frequency CSV missing 'rank' column".to_string()))?;

    let mut records = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let field = record
            .get(field_idx)
            .ok_or_else(|| CliError::Other("Frequency CSV record missing field value".to_string()))?
            .to_string();

        let value = record
            .get(value_idx)
            .ok_or_else(|| CliError::Other("Frequency CSV record missing value".to_string()))?
            .to_string();

        let count = record
            .get(count_idx)
            .ok_or_else(|| CliError::Other("Frequency CSV record missing count".to_string()))?
            .parse::<u64>()
            .map_err(|e| CliError::Other(format!("Failed to parse count in frequency CSV: {e}")))?;

        let percentage = record
            .get(percentage_idx)
            .ok_or_else(|| CliError::Other("Frequency CSV record missing percentage".to_string()))?
            .parse::<f64>()
            .map_err(|e| {
                CliError::Other(format!("Failed to parse percentage in frequency CSV: {e}"))
            })?;

        let rank = record
            .get(rank_idx)
            .ok_or_else(|| CliError::Other("Frequency CSV record missing rank".to_string()))?
            .parse::<f64>()
            .map_err(|e| CliError::Other(format!("Failed to parse rank in frequency CSV: {e}")))?;

        records.push(FrequencyRecord {
            field,
            value,
            count,
            percentage,
            rank,
        });
    }

    Ok(records)
}

/// Generate code-based dictionary entries from stats and frequency data
fn generate_code_based_dictionary(
    stats_records: &[StatsRecord],
    frequency_records: &[FrequencyRecord],
    enum_threshold: usize,
    num_examples: u16,
    truncate_str: usize,
    addl_cols: &[String],
) -> Vec<DictionaryEntry> {
    // Group frequency records by field
    let mut frequency_by_field: HashMap<String, Vec<&FrequencyRecord>> = HashMap::new();
    for freq_record in frequency_records {
        frequency_by_field
            .entry(freq_record.field.clone())
            .or_default()
            .push(freq_record);
    }

    let mut dictionary_entries = Vec::new();

    for stats_record in stats_records {
        let field_name = &stats_record.field;
        let field_frequencies = frequency_by_field
            .get(field_name)
            .cloned()
            .unwrap_or_default();

        // Generate enumeration
        let enumeration = if stats_record.cardinality <= enum_threshold as u64 {
            // Check if there's a rank=0 entry (Other category) or <ALL_UNIQUE> value
            let has_other = field_frequencies
                .iter()
                .any(|f| f.rank == 0.0 && !f.value.contains("<ALL_UNIQUE>"));
            if has_other {
                String::new()
            } else {
                // Enumerate all values (excluding <ALL_UNIQUE>), each on its own line
                let mut enum_values: Vec<String> = field_frequencies
                    .iter()
                    .filter(|f| !f.value.contains("<ALL_UNIQUE>"))
                    .map(|f| f.value.clone())
                    .collect();
                enum_values.sort(); // Sort alphabetically for consistency
                enum_values.join("\n")
            }
        } else {
            String::new()
        };

        // Generate examples
        let examples = if field_frequencies
            .iter()
            .any(|f| (f.percentage - 100.0).abs() < 0.0001)
        {
            "<ALL_UNIQUE>".to_string()
        } else {
            // Get top N values sorted by count descending
            let mut sorted_freqs = field_frequencies.clone();
            sorted_freqs.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.value.cmp(&b.value)));

            let top_n: Vec<String> = sorted_freqs
                .iter()
                .take(num_examples as usize)
                .map(|f| {
                    let v = if truncate_str > 0 && f.value.chars().count() > truncate_str {
                        let mut s = f.value.chars().take(truncate_str).collect::<String>();
                        s.push('…');
                        s
                    } else {
                        f.value.clone()
                    };
                    format!("{} [{}]", v, f.count)
                })
                .collect();

            top_n.join("\n")
        };

        // Collect additional columns for this entry, preserving order
        let mut entry_addl_cols = IndexMap::new();
        for col_name in addl_cols {
            if let Some(value) = stats_record.addl_cols.get(col_name) {
                entry_addl_cols.insert(col_name.clone(), value.clone());
            }
        }

        dictionary_entries.push(DictionaryEntry {
            name: stats_record.field.clone(),
            r#type: stats_record.r#type.clone(),
            label: String::new(),       // Will be filled by LLM
            description: String::new(), // Will be filled by LLM
            min: stats_record.min.clone(),
            max: stats_record.max.clone(),
            cardinality: stats_record.cardinality,
            enumeration,
            null_count: stats_record.nullcount,
            addl_cols: entry_addl_cols,
            examples,
        });
    }

    dictionary_entries
}

/// Extract JSON from LLM response AND Output using common pattern
fn extract_json_from_output(output: &str) -> CliResult<serde_json::Value> {
    // Helper function to validate and return JSON candidate
    fn validate_json_candidate(candidate: &str) -> Option<serde_json::Value> {
        serde_json::from_str::<serde_json::Value>(candidate.trim()).ok()
    }

    // Helper function to try to fix common JSON issues,
    // particularly unescaped newlines in strings
    fn try_fix_json(json_str: &str) -> String {
        let mut result = String::with_capacity(json_str.len());
        let chars = json_str.chars();
        let mut in_string = false;
        let mut escape_next = false;

        for ch in chars {
            if escape_next {
                result.push(ch);
                escape_next = false;
                continue;
            }

            match ch {
                '\\' => {
                    result.push(ch);
                    escape_next = true;
                },
                '"' => {
                    result.push(ch);
                    in_string = !in_string;
                },
                '\n' if in_string => {
                    // Escape newlines inside strings
                    result.push_str("\\n");
                },
                '\r' if in_string => {
                    // Escape carriage returns inside strings
                    result.push_str("\\r");
                },
                '\t' if in_string => {
                    // Escape tabs inside strings
                    result.push_str("\\t");
                },
                _ => {
                    result.push(ch);
                },
            }
        }

        result
    }

    // Helper function to try parsing with fallback to fixed version
    fn try_parse_json(candidate: &str) -> Option<serde_json::Value> {
        // First try parsing as-is
        if let Some(json) = validate_json_candidate(candidate) {
            return Some(json);
        }
        // If that fails, try fixing common issues
        let fixed = try_fix_json(candidate);
        validate_json_candidate(&fixed)
    }

    // Pattern 1: JSON wrapped in ```json and ``` blocks (improved regex to handle multiline)
    if let Some(caps) = regex_oncelock!(r"(?s)```json\s*\n(.*?)\n```").captures(output)
        && let Some(m) = caps.get(1)
        && let Some(valid_json) = try_parse_json(m.as_str())
    {
        return Ok(valid_json);
    }

    // Pattern 1b: JSON wrapped in ```json and ``` blocks (greedy match as fallback)
    if let Some(caps) = regex_oncelock!(r"(?s)```json\s*\n(.*)\n```").captures(output)
        && let Some(m) = caps.get(1)
        && let Some(valid_json) = try_parse_json(m.as_str())
    {
        return Ok(valid_json);
    }

    // Pattern 2: JSON wrapped in ``` and ``` blocks (without json specifier)
    if let Some(caps) = regex_oncelock!(r"(?s)```\s*\n(.*?)\n```").captures(output)
        && let Some(m) = caps.get(1)
        && let Some(valid_json) = try_parse_json(m.as_str())
    {
        return Ok(valid_json);
    }

    // Pattern 2b: JSON wrapped in ``` and ``` blocks (greedy match as fallback)
    if let Some(caps) = regex_oncelock!(r"(?s)```\s*\n(.*)\n```").captures(output)
        && let Some(m) = caps.get(1)
        && let Some(valid_json) = try_parse_json(m.as_str())
    {
        return Ok(valid_json);
    }

    // Pattern 3: Try to find JSON array or object at the start of the response
    if let Some(caps) = regex_oncelock!(r"(?s)^\s*(\[.*?\]|\{.*?\})").captures(output)
        && let Some(m) = caps.get(1)
        && let Some(valid_json) = try_parse_json(m.as_str())
    {
        return Ok(valid_json);
    }

    // Pattern 4: Try to find JSON array or object anywhere in the response (non-greedy)
    if let Some(caps) = regex_oncelock!(r"(?s)(\[.*?\]|\{.*?\})").captures(output)
        && let Some(m) = caps.get(1)
        && let Some(valid_json) = try_parse_json(m.as_str())
    {
        return Ok(valid_json);
    }

    // If no pattern matches, return the entire output (might be raw JSON)
    if (output.trim().starts_with('[') || output.trim().starts_with('{'))
        && let Some(valid_json) = try_parse_json(output)
    {
        return Ok(valid_json);
    }

    fail_clierror!(
        "Failed to extract JSON content from LLM response. Output: {}",
        if output.is_empty() { "<empty>" } else { output }
    )
}

/// Parse LLM JSON response to extract Label and Description for each field
fn parse_llm_dictionary_response(
    llm_response: &str,
    field_names: &[String],
) -> CliResult<HashMap<String, (String, String)>> {
    let json_value = extract_json_from_output(llm_response)?;

    let mut result = HashMap::new();

    // Parse JSON object
    if let Some(obj) = json_value.as_object() {
        for field_name in field_names {
            if let Some(field_obj) = obj.get(field_name)
                && let Some(field_map) = field_obj.as_object()
            {
                let label = field_map
                    .get("label")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();

                let description = field_map
                    .get("description")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();

                result.insert(field_name.clone(), (label, description));
            }
        }
    }

    Ok(result)
}

/// Combine code-generated dictionary entries with LLM-generated Label/Description
fn combine_dictionary_entries(
    mut code_entries: Vec<DictionaryEntry>,
    llm_labels_descriptions: &HashMap<String, (String, String)>,
) -> Vec<DictionaryEntry> {
    for entry in &mut code_entries {
        if let Some((label, description)) = llm_labels_descriptions.get(&entry.name) {
            entry.label = label.clone();
            entry.description = description.clone();
        }
    }
    code_entries
}

/// Replace {GENERATED_BY_SIGNATURE} placeholder with actual attribution
fn replace_attribution_placeholder(
    text: &str,
    args: &Args,
    model: &str,
    base_url: &str,
    format: AttributionFormat,
    prompt_type: PromptType,
) -> String {
    const ATTRIBUTION_BORDER: &str =
        "===============================================================================";

    let prompt_file = get_prompt_file(args).ok();
    let (prompt_file_kind, prompt_file_ver, prompt_file_lang) =
        if let Some(prompt_file) = prompt_file {
            let prompt_file_kind = if let Some(prompt_file_path) = args.flag_prompt_file.as_ref() {
                format!("Custom (file: {prompt_file_path})")
            } else {
                "Default".to_string()
            };
            (
                prompt_file_kind,
                prompt_file.version.clone(),
                prompt_file.language.clone(),
            )
        } else {
            ("Default".to_string(), "unknown".to_string(), String::new())
        };

    // detected language with confidence if available, otherwise use model default
    let detected_lang = DETECTED_LANGUAGE
        .get()
        .map_or_else(|| prompt_file_lang.clone(), String::to_string);
    let detected_confidence = DETECTED_LANGUAGE_CONFIDENCE.get().copied().unwrap_or(0.0);

    // Compute the display language string for attribution.
    //
    // - For PromptType::Prompt:
    //      * If the detected language matches the language specified in the prompt file, simply use
    //        the prompt file's language.
    //      * If the detected language differs from the prompt file's language, display the detected
    //        language with the confidence score as a percentage (one decimal). e.g., "Spanish
    //        (85.0%)"
    //      This exposes to the user both the auto-detected language and how confident describegpt
    // is in detection.
    //
    // - For other prompt types (Dictionary, Description, Tags), just use the prompt file's
    //   language.
    //
    // This enables clearer reporting in attribution blocks and helps users quickly determine
    // which language/dialect is being used in LLM responses and what confidence describegpt had
    // in the detection case.
    let lang_display = if prompt_type == PromptType::Prompt {
        if detected_lang == prompt_file_lang {
            prompt_file_lang
        } else {
            format!("{detected_lang} ({:.1}%)", detected_confidence * 100.0)
        }
    } else {
        prompt_file_lang
    };

    // Custom warning message based on PromptType
    let warning_message = match prompt_type {
        PromptType::Dictionary => {
            "WARNING: Label and Description generated by an LLM and may contain inaccuracies. \
             Verify before using!"
        },
        PromptType::Description => {
            "WARNING: Description generated by an LLM and may contain inaccuracies. Verify before \
             using!"
        },
        PromptType::Tags => {
            "WARNING: Tags generated by an LLM and may contain inaccuracies. Verify before using!"
        },
        PromptType::Prompt => {
            "WARNING: Generated by an LLM and may contain inaccuracies. Verify before using!"
        },
    };

    // Handle prompt info wrapping for SQL comment format
    let (prompt_info, att_prefix, extra_separator) = if format == AttributionFormat::SqlComment
        && let Some(prompt) = &args.flag_prompt
    {
        let wrapped_prompt = textwrap::fill(
            prompt,
            textwrap::Options::new(75).subsequent_indent("--         "),
        );
        (
            format!(
                r#"{ATTRIBUTION_BORDER}
-- Prompt: {wrapped_prompt}
--"#
            ),
            "-- ",
            format!("-- {ATTRIBUTION_BORDER}\n--"),
        )
    } else {
        (String::new(), "", String::new())
    };

    let attribution = format!(
        r#"{prompt_info_display}{att_prefix}Generated by {qsv_variant} v{qsv_version} describegpt
{att_prefix}Command line: {command_line}
{att_prefix}Prompt file: {prompt_file_kind} v{prompt_file_ver}
{att_prefix}Model: {model}
{att_prefix}LLM API URL: {base_url}
{att_prefix}Language: {lang_display}
{att_prefix}Timestamp: {ts}
{att_prefix}
{att_prefix}{warning_message}
{extra_separator}"#,
        prompt_info_display = if prompt_info.is_empty() {
            String::new()
        } else {
            format!("{prompt_info}\n")
        },
        qsv_variant = util::CARGO_BIN_NAME,
        qsv_version = util::CARGO_PKG_VERSION,
        command_line = std::env::args().collect::<Vec<_>>().join(" "),
        ts = chrono::Utc::now().to_rfc3339(),
    );

    text.replace("{GENERATED_BY_SIGNATURE}", &attribution)
}

/// Extract ordered additional column names from entries
/// Returns columns in the order they appear in IndexMap (preserves insertion order)
fn extract_ordered_addl_cols(entries: &[DictionaryEntry]) -> Vec<String> {
    // Get the ordered column names from the first entry (all entries should have the same order)
    // IndexMap preserves insertion order, so we can iterate over keys directly
    entries
        .first()
        .map(|e| e.addl_cols.keys().cloned().collect())
        .unwrap_or_default()
}

/// Format dictionary entries as markdown table
fn format_dictionary_markdown(entries: &[DictionaryEntry]) -> String {
    use std::fmt::Write;

    // Determine which additional columns are present (preserving order)
    let addl_col_names = extract_ordered_addl_cols(entries);

    let mut output = String::with_capacity(1024); //from("# Data Dictionary\n");

    // Build header row
    output.push_str(
        "| Name | Type | Label | Description | Min | Max | Cardinality | Enumeration | Null Count",
    );
    for col_name in &addl_col_names {
        let _ = write!(output, " | {col_name}");
    }
    output.push_str(" | Examples |\n");

    // Build separator row
    output.push_str(
        "|------|------|-------|-------------|-----|-----|-------------|-------------|------------",
    );
    for _ in &addl_col_names {
        output.push_str("|----------");
    }
    output.push_str("|----------|\n");

    for entry in entries {
        // Escape pipe characters in markdown table cells
        let name = entry.name.replace('|', "\\|");
        let r#type = entry.r#type.replace('|', "\\|");
        let label = entry.label.replace('|', "\\|");
        let description = entry.description.replace('|', "\\|").replace('\n', "<br>");
        let min = entry.min.replace('|', "\\|");
        let max = entry.max.replace('|', "\\|");
        let enumeration = entry.enumeration.replace('|', "\\|");
        let examples = entry.examples.replace('|', "\\|");

        // Format enumeration: if empty, show empty string, otherwise show values on separate lines
        let enumeration_display = if enumeration.is_empty() {
            String::new()
        } else {
            // Replace newlines with <br> for markdown table compatibility
            enumeration.replace('\n', "<br>")
        };

        // Format examples: replace newlines with <br> and format counts using HumanCount
        let examples_display = if examples == "<ALL_UNIQUE>" {
            examples.clone()
        } else {
            // Parse and reformat counts in examples (format: "value [count]")
            examples
                .lines()
                .map(|line| {
                    if let Some(pos) = line.rfind(" [") {
                        let (value_part, count_part) = line.split_at(pos + 2);
                        if let Some(end_pos) = count_part.find(']') {
                            let count_str = &count_part[..end_pos];
                            if let Ok(count) = count_str.parse::<u64>() {
                                format!(
                                    "{} [{}]",
                                    value_part.trim_end_matches(" ["),
                                    HumanCount(count)
                                )
                            } else {
                                line.to_string()
                            }
                        } else {
                            line.to_string()
                        }
                    } else {
                        line.to_string()
                    }
                })
                .collect::<Vec<String>>()
                .join("<br>")
        };

        // Build row with additional columns
        let _ = write!(
            output,
            "| **{}** | {} | {} | {} | {} | {} | {} | {} | {}",
            name,
            r#type,
            label,
            description,
            min,
            max,
            HumanCount(entry.cardinality),
            enumeration_display,
            HumanCount(entry.null_count)
        );

        // Add additional columns
        for col_name in &addl_col_names {
            let value = entry
                .addl_cols
                .get(col_name)
                .map(|v| {
                    if col_name == "percentiles" {
                        // Replace | with <br> for readability in percentiles
                        v.replace(['|', '\n'], "<br>")
                    } else {
                        v.replace('|', "\\|").replace('\n', "<br>")
                    }
                })
                .unwrap_or_default();
            let _ = write!(output, " | {value}");
        }

        // Add Examples column
        let _ = writeln!(output, " | {examples_display} |");
    }

    // Add attribution at the bottom
    output.push_str("\n*Attribution: {GENERATED_BY_SIGNATURE}*\n");

    output
}

/// Format dictionary entries as JSON
fn format_dictionary_json(entries: &[DictionaryEntry], args: &Args) -> serde_json::Value {
    let entries_json: Vec<serde_json::Value> = entries
        .iter()
        .map(|e| {
            let mut entry_obj = json!({
                "name": e.name,
                "type": e.r#type,
                "label": e.label,
                "description": e.description,
                "min": if e.min.is_empty() { serde_json::Value::Null } else { serde_json::Value::String(e.min.clone()) },
                "max": if e.max.is_empty() { serde_json::Value::Null } else { serde_json::Value::String(e.max.clone()) },
                "cardinality": e.cardinality,
                "enumeration": if e.enumeration.is_empty() { serde_json::Value::Null } else { serde_json::Value::String(e.enumeration.clone()) },
                "null_count": e.null_count,
            });

            // Add additional columns to the JSON object
            if let Some(obj) = entry_obj.as_object_mut() {
                for (key, value) in &e.addl_cols {
                    let json_value = if value.is_empty() {
                        serde_json::Value::Null
                    } else if key == "percentiles" {
                        // Replace | with \n for readability in percentiles
                        serde_json::Value::String(value.replace('|', "\n"))
                    } else {
                        serde_json::Value::String(value.clone())
                    };
                    obj.insert(key.clone(), json_value);
                }
                // Add examples at the end
                obj.insert("examples".to_string(), json!(e.examples));
            }

            entry_obj
        })
        .collect();

    json!({
        "fields": entries_json,
        "enum_threshold": args.flag_enum_threshold,
        "num_examples": args.flag_num_examples,
        "truncate_str": args.flag_truncate_str,
        "attribution": "{GENERATED_BY_SIGNATURE}"
    })
}

/// Format dictionary entries as TSV
fn format_dictionary_tsv(entries: &[DictionaryEntry]) -> String {
    use std::fmt::Write;

    // Determine which additional columns are present (preserving order)
    let addl_col_names = extract_ordered_addl_cols(entries);

    let mut output = String::with_capacity(1024);
    // TSV header
    output
        .push_str("Name\tType\tLabel\tDescription\tMin\tMax\tCardinality\tEnumeration\tNull Count");
    for col_name in &addl_col_names {
        let _ = write!(output, "\t{col_name}");
    }
    output.push_str("\tExamples\n");

    for entry in entries {
        // Escape tabs and newlines in TSV cells
        let name = entry.name.replace(['\t', '\n', '\r'], " ");
        let r#type = entry.r#type.replace(['\t', '\n', '\r'], " ");
        let label = entry.label.replace(['\t', '\n', '\r'], " ");
        let description = entry.description.replace(['\t', '\n', '\r'], " ");
        let min = entry.min.replace(['\t', '\n', '\r'], " ");
        let max = entry.max.replace(['\t', '\n', '\r'], " ");
        let enumeration = entry.enumeration.replace(['\t', '\n', '\r'], " ");
        let examples = entry.examples.replace(['\t', '\n', '\r'], " ");

        // Build row with additional columns
        let _ = write!(
            output,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            name,
            r#type,
            label,
            description,
            min,
            max,
            HumanCount(entry.cardinality),
            enumeration,
            HumanCount(entry.null_count)
        );

        // Add additional columns
        for col_name in &addl_col_names {
            let value = entry
                .addl_cols
                .get(col_name)
                .map(|v| {
                    if col_name == "percentiles" {
                        // Replace | and newlines with "; " for readability in percentiles
                        // then escape tabs and carriage returns
                        v.replace(['|', '\n'], "; ").replace(['\t', '\r'], " ")
                    } else {
                        v.replace(['\t', '\n', '\r'], " ")
                    }
                })
                .unwrap_or_default();
            let _ = write!(output, "\t{value}");
        }

        // Add Examples column
        let _ = writeln!(output, "\t{examples}");
    }

    output
}

/// Format token usage and reasoning as comment lines for TSV
fn format_token_usage_comments(reasoning: &str, token_usage: &TokenUsage) -> String {
    format!(
        "# REASONING\n# {}\n# TOKEN USAGE\n# prompt: {}\n# completion: {}\n# total: {}\n# \
         elapsed: {} ms\n",
        reasoning.replace('\n', "\n# "),
        token_usage.prompt,
        token_usage.completion,
        token_usage.total,
        token_usage.elapsed
    )
}

/// Format tags as TSV (single row with columns: tag, reasoning, token_usage fields)
#[rustfmt::skip]
fn format_tags_tsv(
    tags_json: &serde_json::Value,
    reasoning: &str,
    token_usage: &TokenUsage,
) -> String {
    // Extract tags from JSON - tags might be an array or an object with a tags field
    let tags_vec = if let Some(tags_array) = tags_json.as_array() {
        tags_array
            .iter()
            .filter_map(|v| v.as_str())
            .collect::<Vec<&str>>()
    } else if let Some(obj) = tags_json.as_object() {
        if let Some(tags_array) = obj.get("tags").and_then(|v| v.as_array()) {
            tags_array
                .iter()
                .filter_map(|v| v.as_str())
                .collect::<Vec<&str>>()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    let tags_str = tags_vec.join(", ");
    // Escape tabs and newlines
    let tags_escaped = tags_str.replace(['\t', '\n', '\r'], " ");
    let reasoning_escaped = reasoning.replace(['\t', '\n', '\r'], " ");

    format!(
        "tags\treasoning\ttoken_prompt\ttoken_completion\ttoken_total\telapsed\n{}\t{}\t{}\t{}\t{}\t{}\n",
        tags_escaped,
        reasoning_escaped,
        token_usage.prompt,
        token_usage.completion,
        token_usage.total,
        token_usage.elapsed
    )
}

/// Format description as TSV (single row with columns: response, reasoning, token_usage fields)
#[rustfmt::skip]
fn format_description_tsv(response: &str, reasoning: &str, token_usage: &TokenUsage) -> String {
    // Escape tabs and newlines
    let response_escaped = response.replace(['\t', '\n', '\r'], " ");
    let reasoning_escaped = reasoning.replace(['\t', '\n', '\r'], " ");

    format!(
        "response\treasoning\ttoken_prompt\ttoken_completion\ttoken_total\telapsed\n{}\t{}\t{}\t{}\t{}\t{}\n",
        response_escaped,
        reasoning_escaped,
        token_usage.prompt,
        token_usage.completion,
        token_usage.total,
        token_usage.elapsed
    )
}

/// Format prompt as TSV (single row with columns: response, reasoning, token_usage fields)
#[rustfmt::skip]
fn format_prompt_tsv(response: &str, reasoning: &str, token_usage: &TokenUsage) -> String {
    // Escape tabs and newlines
    let response_escaped = response.replace(['\t', '\n', '\r'], " ");
    let reasoning_escaped = reasoning.replace(['\t', '\n', '\r'], " ");

    format!(
        "response\treasoning\ttoken_prompt\ttoken_completion\ttoken_total\telapsed\n{}\t{}\t{}\t{}\t{}\t{}\n",
        response_escaped,
        reasoning_escaped,
        token_usage.prompt,
        token_usage.completion,
        token_usage.total,
        token_usage.elapsed
    )
}

/// Generates a prompt for a given prompt type based on either a custom prompt file or default
/// prompts. Uses the Mini Jinja template engine to render prompt templates with variables.
///
/// # Arguments
///
/// * `prompt_type` - The type of prompt to generate (Dictionary, Description, Tags, or Custom
///   Prompt)
/// * `analysis_results` - Optional analysis results containing stats, frequency data, headers and
///   delimiter
/// * `args` - Command line arguments
///
/// # Returns
///
/// Returns a tuple containing:
/// * The generated prompt string (rendered from Mini Jinja template)
/// * The system prompt string (rendered from Mini Jinja template)
///
/// # Errors
///
/// Returns a CliError if:
/// * Analysis results are missing when required
/// * SQL guidelines markers cannot be found in the prompt template
/// * DuckDB query execution fails when getting extension info
/// * Mini Jinja template rendering fails
fn get_prompt(
    prompt_type: PromptType,
    analysis_results: Option<&AnalysisResults>,
    args: &Args,
) -> CliResult<(String, String)> {
    // Get prompt file if --prompt-file is used, otherwise get default prompt file
    let prompt_file = get_prompt_file(args)?;

    // Get prompt from prompt file
    let mut prompt = match prompt_type {
        PromptType::Dictionary => prompt_file.dictionary_prompt.clone(),
        PromptType::Description => prompt_file.description_prompt.clone(),
        PromptType::Tags => prompt_file.tags_prompt.clone(),
        PromptType::Prompt => {
            let working_prompt = args
                .flag_prompt
                .clone()
                .unwrap_or_else(|| prompt_file.prompt.clone());
            format!(
                "User's Prompt: {working_prompt}\n\n{}",
                prompt_file.custom_prompt_guidance
            )
        },
    };

    let (stats, frequency, headers, delimiter) = match analysis_results {
        Some(ar) => (&ar.stats, &ar.frequency, &ar.headers, &ar.delimiter),
        None => {
            return fail!("Analysis results required for prompt generation.");
        },
    };

    let mut duckdb_version = String::new();

    // If custom prompt and DuckDB should be used, modify SQL generation guidelines
    if prompt_type == PromptType::Prompt {
        // Look for the SQL query generation guidelines section & replace it with DuckDB guidance
        let sql_guidelines_start = "SQL Query Generation Guidelines:\n";
        let sql_guidelines_end = "\nEND SQL Query Generation Guidelines\n";

        let start_pos = prompt.find(sql_guidelines_start).ok_or_else(|| {
            CliError::Other("Could not find SQL guidelines start marker in prompt".to_string())
        })?;
        let end_pos = start_pos
            + prompt[start_pos..]
                .find(sql_guidelines_end)
                .ok_or_else(|| {
                    CliError::Other(
                        "Could not find SQL guidelines end marker in prompt".to_string(),
                    )
                })?;
        let before_guidelines = &prompt[..start_pos];
        let after_guidelines = &prompt[(end_pos + sql_guidelines_end.len())..];

        if should_use_duckdb() {
            // call DuckDB to get the list of valid extensions
            let duckdb_query = "SELECT extension_name FROM duckdb_extensions() where loaded = true";
            let duckdb_response = run_duckdb_query(duckdb_query, "", "")?;
            // duckdb_response.0 is a CSV with a header row, so skip the header row
            // and convert to a comma-separated list
            let valid_extensions = duckdb_response
                .0
                .lines()
                .skip(1) // Skip header row
                .collect::<Vec<_>>()
                .join(", ");

            // get the DuckDB minor version
            let duckdb_version_query = "SELECT version()";
            let duckdb_version_response = run_duckdb_query(duckdb_version_query, "", "")?;
            duckdb_version = duckdb_version_response
                .0
                .lines()
                .next()
                .unwrap_or("")
                .to_string();
            // extract only up to the minor version
            duckdb_version = duckdb_version
                .split('.')
                .take(2)
                .collect::<Vec<_>>()
                .join(".");
            log::debug!("DuckDB minor version: {duckdb_version}");

            let duckdb_sql_guidance = prompt_file.duckdb_sql_guidance.trim_end().to_string();

            // Generate prompt for DuckDB SQL
            prompt = format!(
                "{before_guidelines}{sql_guidelines_start}{duckdb_sql_guidance}\n- Only use \
                 functions from the following Loaded DuckDB extensions: \
                 {valid_extensions}\n{after_guidelines}",
            );

            if args.flag_fewshot_examples {
                prompt = format!(
                    "{prompt}\n\n{dd_fewshot_examples}",
                    dd_fewshot_examples = prompt_file.dd_fewshot_examples
                );
            }
            log::debug!("DuckDB SQL prompt:\n{prompt}");
        } else {
            // Generate prompt for Polars SQL
            prompt = format!(
                "{before_guidelines}{sql_guidelines_start}{polars_sql_guidance}{after_guidelines}",
                polars_sql_guidance = prompt_file.polars_sql_guidance.trim_end(),
            );
            if args.flag_fewshot_examples {
                prompt = format!(
                    "{prompt}\n\n{p_fewshot_examples}",
                    p_fewshot_examples = prompt_file.p_fewshot_examples
                );
            }
            log::debug!("Polars SQL prompt:\n{prompt}");
        }
    }

    let tag_vocab = if prompt_type == PromptType::Tags
        && let Some(ref tag_vocab_uri) = args.flag_tag_vocab
    {
        // Load tag vocabulary CSV using lookup support
        // (handles local files, remote URLs, ckan:// and dathere:// schemes)
        let tag_vocab_filepath = {
            #[cfg(feature = "feature_capable")]
            {
                if tag_vocab_uri.to_lowercase().starts_with("http")
                    || tag_vocab_uri.starts_with("ckan://")
                    || tag_vocab_uri.starts_with("dathere://")
                {
                    // Use lookup to download/cache remote CSV resources
                    let cache_dir = TAG_VOCAB_CACHE_DIR.get().ok_or_else(|| {
                        CliError::Other(
                            "Tag vocabulary cache directory not initialized".to_string(),
                        )
                    })?;
                    let lookup_opts = LookupTableOptions {
                        name:           "tag_vocab".to_string(),
                        uri:            tag_vocab_uri.clone(),
                        cache_dir:      cache_dir.clone(),
                        cache_age_secs: 3600, // Default 1 hour cache
                        delimiter:      None,
                        ckan_api_url:   TAG_VOCAB_CKAN_API.get().cloned(),
                        ckan_token:     TAG_VOCAB_CKAN_TOKEN
                            .get()
                            .and_then(std::clone::Clone::clone),
                        timeout_secs:   args.flag_timeout,
                    };
                    let lookup_result = lookup::load_lookup_table(&lookup_opts).map_err(|e| {
                        CliError::Other(format!(
                            "Failed to load tag vocabulary from {tag_vocab_uri}: {e}",
                        ))
                    })?;
                    lookup_result.filepath
                } else {
                    // Local file - check if it exists
                    if fs::metadata(tag_vocab_uri).map_or(true, |m| !m.is_file()) {
                        return fail_incorrectusage_clierror!(
                            "Tag vocabulary file does not exist or is not a file: {tag_vocab_uri}"
                        );
                    }
                    tag_vocab_uri.clone()
                }
            }
            #[cfg(not(feature = "feature_capable"))]
            {
                // Lite build: only support local files
                if tag_vocab_uri.to_lowercase().starts_with("http")
                    || tag_vocab_uri.starts_with("ckan://")
                    || tag_vocab_uri.starts_with("dathere://")
                {
                    return fail_incorrectusage_clierror!(
                        "Remote tag vocabulary URLs are not supported in qsvlite. Please use a \
                         local CSV file."
                    );
                }
                // Local file - check if it exists
                if fs::metadata(tag_vocab_uri).map_or(true, |m| !m.is_file()) {
                    return fail_incorrectusage_clierror!(
                        "Tag vocabulary file does not exist or is not a file: {tag_vocab_uri}"
                    );
                }
                tag_vocab_uri.clone()
            }
        };

        // Validate tag vocabulary CSV file
        let conf = Config::new(Some(tag_vocab_filepath.clone()).as_ref()).no_headers(false);
        let mut rdr = conf
            .reader()
            .map_err(|e| CliError::Other(format!("Failed to read tag vocabulary CSV: {e}")))?;

        // validate that the tag vocabulary CSV file has at least 2 columns and have the expected
        // column names
        let headers = rdr.headers()?.clone();
        if headers.len() != 2
            || headers.get(0).unwrap_or("").trim() != "tag"
            || headers.get(1).unwrap_or("").trim() != "description"
        {
            return fail_incorrectusage_clierror!(
                "Tag vocabulary CSV must have exactly 2 columns (tag and description)"
            );
        }

        // scan the tag vocabulary CSV file to see if each record is valid
        for rec_iter in rdr.records() {
            let record = rec_iter.map_err(|e| {
                CliError::Other(format!("Failed to parse tag vocabulary CSV record: {e}"))
            })?;
            if record.len() < 2 {
                return fail_incorrectusage_clierror!(
                    "Tag vocabulary CSV must have at least 2 columns (tag and description)"
                );
            }
        }
        // close the reader
        drop(rdr);

        // and just load the tag vocabulary CSV file into a string
        fs::read_to_string(tag_vocab_filepath)
            .map_err(|e| CliError::Other(format!("Failed to read tag vocabulary CSV file: {e}")))?
    } else {
        String::new()
    };

    // Set up Mini Jinja environment for template rendering
    let mut env = Environment::new();

    // add all the Mini Jinja contrib filters to the environment
    minijinja_contrib::add_to_environment(&mut env);

    // Build context with all variables needed for template rendering
    let json_add = if get_output_format(args)? == OutputFormat::Json {
        " (in valid, pretty-printed JSON format, ensuring string values are properly escaped)"
    } else if get_output_format(args)? == OutputFormat::Toon {
        " (in TOON format)"
    } else {
        " (in Markdown format)"
    };

    let ctx = context! {
        stats => stats,
        frequency => frequency,
        dictionary => DATA_DICTIONARY_JSON.get().map_or("", |s| s.as_str()),
        json_add => json_add,
        duckdb_version => duckdb_version.as_str(),
        top_n => args.flag_enum_threshold,
        num_tags => args.flag_num_tags,
        tag_vocab => tag_vocab,
        language => args.flag_language.as_ref().map_or("", |s| s.as_str()),
        headers => headers,
        delimiter => delimiter.to_string(),
        input_table_name => INPUT_TABLE_NAME,
        sample_file => SAMPLE_FILE.get().map_or("", |s| s.as_str()),
        sample_size => args.flag_sample_size.to_string(),
        generated_by_signature => "{GENERATED_BY_SIGNATURE}",
    };

    // Render prompt using Mini Jinja
    let rendered_prompt = env
        .render_str(&prompt, &ctx)
        .map_err(|e| CliError::Other(format!("Failed to render prompt template: {e}")))?;

    // Also render system_prompt if it contains template variables
    let rendered_system_prompt = env
        .render_str(&prompt_file.system_prompt, &ctx)
        .map_err(|e| CliError::Other(format!("Failed to render system_prompt template: {e}")))?;

    if log::log_enabled!(log::Level::Debug) {
        log::debug!("Prompt Type: {prompt_type}");
        log::debug!("Rendered system prompt: {rendered_system_prompt}");
        log::debug!("Rendered prompt: {rendered_prompt}");
    }

    // Return rendered prompt
    Ok((rendered_prompt, rendered_system_prompt))
}

/// Makes a completion request to the LLM API and processes the response.
///
/// # Arguments
///
/// * `args` - Command line arguments containing configuration options
/// * `client` - The HTTP client used to make API requests
/// * `model` - The model to use for completion
/// * `api_key` - API key for authentication
/// * `messages` - The messages to send to the API
///
/// # Returns
///
/// Returns a `CompletionResponse` containing:
/// * The completion text
/// * Optional reasoning
/// * Token usage statistics
///
/// # Details
///
/// This function:
/// 1. Gets prompt file configuration
/// 2. Constructs the API request with model, max tokens, messages
/// 3. Adds any additional model properties specified
/// 4. Makes POST request to chat completions endpoint
/// 5. Processes response to extract completion, reasoning, token usage
/// 6. Replaces placeholder signature with model name and timestamp
///
/// # Errors
///
/// Returns a CliError if:
/// * The API request fails
/// * The response cannot be parsed
/// * Required fields are missing from response
/// * The API returns an error message
fn get_completion(
    args: &Args,
    client: &Client,
    model: &str,
    api_key: &str,
    messages: &serde_json::Value,
    kind: PromptType,
) -> CliResult<CompletionResponse> {
    let prompt_file = get_prompt_file(args)?;

    let base_url = prompt_file.base_url.clone();

    let max_tokens = if prompt_file.tokens > 0 {
        Some(prompt_file.tokens)
    } else {
        None
    };

    // Create request data
    let mut request_data = json!({
        "model": model,
        "max_tokens": max_tokens,
        "messages": messages,
        "stream": false
    });

    // Add additional model properties if provided
    if let Some(addl_props) = args.flag_addl_props.as_ref() {
        let addl_props_json: serde_json::Value = serde_json::from_str(addl_props)
            .map_err(|e| CliError::Other(format!("Invalid JSON in --addl-props: {e:?}")))?;

        // If addl_props_json is an object, extend/overlay all its keys into request_data
        if let Some(obj) = addl_props_json.as_object() {
            for (key, value) in obj {
                request_data[key] = value.clone();
            }
        } else {
            // If it is not an object, treat as error (must be JSON object of key/values)
            return fail_clierror!(
                "--addl-props should be a JSON object mapping keys to values; got: {}",
                addl_props_json
            );
        }
    }

    // deserializing request_data is relatively expensive, so only do it if debug is enabled
    if log::log_enabled!(log::Level::Trace) {
        log::trace!("Request data: {request_data:?}");
    }

    // Get response from POST request to chat completions endpoint
    let completions_endpoint = "/chat/completions";
    let start_time = Instant::now();
    let response = send_request(
        client,
        Some(api_key),
        Some(&request_data),
        "POST",
        &format!("{base_url}{completions_endpoint}"),
    )?;

    // Parse response as JSON
    let response_json: serde_json::Value = response.json()?;
    if log::log_enabled!(log::Level::Trace) {
        log::trace!("Response: {response_json:?}");
    }

    // If response is an error, print error message
    if let serde_json::Value::Object(ref map) = response_json
        && map.contains_key("error")
    {
        return fail_clierror!("LLM API Error: {}", map["error"]);
    }

    // Get completion and reasoning from response
    let Some(completion) = response_json["choices"]
        .get(0)
        .and_then(|choice| choice["message"]["content"].as_str())
    else {
        return fail_clierror!("Invalid response: missing or malformed completion content");
    };
    // Reasoning is optional - use empty string if not provided
    let reasoning = response_json["choices"]
        .get(0)
        .and_then(|choice| choice["message"]["reasoning"].as_str())
        .unwrap_or("");

    // Get token usage from response
    let Some(usage) = response_json["usage"].as_object() else {
        return fail_clierror!("Invalid response: missing or malformed usage");
    };
    let elapsed_ms = start_time.elapsed().as_millis() as u64;
    let token_usage = TokenUsage {
        prompt:     usage["prompt_tokens"].as_u64().unwrap_or(0),
        completion: usage["completion_tokens"].as_u64().unwrap_or(0),
        total:      usage["total_tokens"].as_u64().unwrap_or(0),
        elapsed:    elapsed_ms,
    };

    // Determine format based on prompt type and flag_prompt
    let format = if kind == PromptType::Prompt && args.flag_prompt.is_some() {
        AttributionFormat::SqlComment
    } else {
        AttributionFormat::Markdown
    };

    // Replace attribution placeholder using unified function
    let completion =
        replace_attribution_placeholder(completion, args, model, &base_url, format, kind);

    Ok(CompletionResponse {
        response: completion,
        reasoning: reasoning.to_string(),
        token_usage,
    })
}

fn get_cache_key(args: &Args, kind: PromptType, actual_model: &str) -> String {
    let file_hash = FILE_HASH.get().unwrap_or(&String::new()).clone();
    // Only include prompt content in cache key for "prompt" kind
    let prompt_content = if kind == PromptType::Prompt {
        args.flag_prompt.as_ref()
    } else {
        None
    };

    // For prompt kind, include a validity flag that can be invalidated
    let validity_flag = if kind == PromptType::Prompt {
        // Check if there's a validity flag stored for this prompt
        get_prompt_validity_flag(args, prompt_content)
    } else {
        "valid".to_string()
    };

    format!(
        "{file_hash};{prompt_file:?};{prompt_content:?};{max_tokens};{addl_props:?};\
         {actual_model};{kind};{validity_flag};{language:?}",
        prompt_file = args.flag_prompt_file,
        max_tokens = args.flag_max_tokens,
        addl_props = args.flag_addl_props,
        language = args.flag_language,
    )
}

fn get_analysis_cache_key(args: &Args, file_hash: &str) -> String {
    format!(
        "analysis_{:?}{:?}{:?}{:?}",
        file_hash, args.flag_stats_options, args.flag_freq_options, args.flag_enum_threshold,
    )
}

// Get the validity flag for a prompt
fn get_prompt_validity_flag(args: &Args, prompt_content: Option<&String>) -> String {
    let flags = PROMPT_VALIDITY_FLAGS.lock().unwrap();

    // Create a key for this prompt
    let prompt_key = if let Some(content) = prompt_content {
        format!("{:?}{:?}{}", args.arg_input, args.flag_prompt_file, content)
    } else {
        format!("{:?}{:?}", args.arg_input, args.flag_prompt_file)
    };

    // Return the validity flag, or "valid" if not found
    flags
        .get(&prompt_key)
        .cloned()
        .unwrap_or_else(|| "valid".to_string())
}

// Invalidate the validity flag for a prompt
fn invalidate_prompt_validity_flag(args: &Args, prompt_content: Option<&String>) {
    let mut flags = PROMPT_VALIDITY_FLAGS.lock().unwrap();

    // Create a key for this prompt
    let prompt_key = if let Some(content) = prompt_content {
        format!("{:?}{:?}{}", args.arg_input, args.flag_prompt_file, content)
    } else {
        format!("{:?}{:?}", args.arg_input, args.flag_prompt_file)
    };

    // Simply mark as invalid - no need for timestamps
    flags.insert(prompt_key, "invalid".to_string());
}

// Try to remove prompt cache entries with different validity flags
fn try_remove_prompt_cache_entries(base_key: &str) -> bool {
    let mut removed = false;

    // Try with "valid" flag
    let key_with_valid = format!("{base_key}valid");
    if GET_DISKCACHE_COMPLETION
        .cache_remove(&key_with_valid)
        .is_ok()
    {
        removed = true;
    }

    // Try with "invalid" flag
    let key_with_invalid = format!("{base_key}invalid");
    if GET_DISKCACHE_COMPLETION
        .cache_remove(&key_with_invalid)
        .is_ok()
    {
        removed = true;
    }

    // Flush the disk cache to ensure changes are persisted
    if let Err(e) = GET_DISKCACHE_COMPLETION.connection().flush() {
        log::warn!("Failed to flush disk cache: {e:?}");
    }

    removed
}

// this is a disk cache that can be used across qsv sessions
#[io_cached(
    disk = true,
    ty = "cached::DiskCache<String, CompletionResponse>",
    key = "String",
    convert = r##"{ get_cache_key(args, kind, model) }"##,
    create = r##"{
        let cache_dir = DISKCACHE_DIR.get().unwrap();
        let diskcache_config = DISKCACHECONFIG.get().unwrap();
        let diskcache: DiskCache<String, CompletionResponse> = DiskCacheBuilder::new("describegpt")
            .set_disk_directory(cache_dir)
            .set_lifespan(diskcache_config.ttl_secs)
            .set_refresh(diskcache_config.ttl_refresh)
            .set_sync_to_disk_on_cache_change(true)
            .build()
            .expect("error building diskcache");
        log::info!("Disk cache created - dir: {cache_dir} - ttl: {ttl_secs:?}",
            ttl_secs = diskcache_config.ttl_secs);
        diskcache
    }"##,
    map_error = r##"|e| CliError::Other(format!("Diskcache Error: {e:?}"))"##,
    with_cached_flag = true
)]
fn get_diskcache_completion(
    args: &Args,
    client: &Client,
    model: &str,
    api_key: &str,
    // this unused_variable lint is a false positive as we use kind in the io_cached macro
    #[allow(unused_variables)] kind: PromptType,
    messages: &serde_json::Value,
) -> CliResult<Return<CompletionResponse>> {
    Ok(Return::new(get_completion(
        args, client, model, api_key, messages, kind,
    )?))
}

// this is a redis cache that can be used across qsv sessions
#[io_cached(
    ty = "cached::RedisCache<String, CompletionResponse>",
    key = "String",
    convert = r##"{ get_cache_key(args, kind, model) }"##,
    create = r##" {
        let redis_config = REDISCONFIG.get().unwrap();
        let rediscache: RedisCache<String, CompletionResponse> = RedisCache::new("f", redis_config.ttl_secs)
            .set_namespace("descq")
            .set_refresh(redis_config.ttl_refresh)
            .set_connection_string(&redis_config.conn_str)
            .set_connection_pool_max_size(redis_config.max_pool_size)
            .build()
            .expect("error building redis cache");
        log::info!("Redis cache created - conn_str: {conn_str} - refresh: {ttl_refresh} - ttl: {ttl_secs:?} - pool_size: {pool_size}",
            conn_str = redis_config.conn_str,
            ttl_refresh = redis_config.ttl_refresh,
            ttl_secs = redis_config.ttl_secs,
            pool_size = redis_config.max_pool_size);
        rediscache
    } "##,
    map_error = r##"|e| CliError::Other(format!("Redis Error: {e:?}"))"##,
    with_cached_flag = true
)]
fn get_redis_completion(
    args: &Args,
    client: &Client,
    model: &str,
    api_key: &str,
    #[allow(unused_variables)] kind: PromptType,
    messages: &serde_json::Value,
) -> CliResult<Return<CompletionResponse>> {
    Ok(Return::new(get_completion(
        args, client, model, api_key, messages, kind,
    )?))
}

// Cached analysis results for disk cache
#[io_cached(
    disk = true,
    ty = "cached::DiskCache<String, AnalysisResults>",
    key = "String",
    convert = r##"{ get_analysis_cache_key(args, file_hash) }"##,
    create = r##"{
        let cache_dir = DISKCACHE_DIR.get().unwrap();
        let diskcache_config = DISKCACHECONFIG.get().unwrap();
        let diskcache: DiskCache<String, AnalysisResults> = DiskCacheBuilder::new("describegpt_analysis")
            .set_disk_directory(cache_dir)
            .set_lifespan(diskcache_config.ttl_secs)
            .set_refresh(diskcache_config.ttl_refresh)
            .set_sync_to_disk_on_cache_change(true)
            .build()
            .expect("error building analysis diskcache");
        log::info!("Analysis disk cache created - dir: {cache_dir} - ttl: {ttl_secs:?}",
            ttl_secs = diskcache_config.ttl_secs);
        diskcache
    }"##,
    map_error = r##"|e| CliError::Other(format!("Analysis Diskcache Error: {e:?}"))"##,
    with_cached_flag = true
)]
fn get_diskcache_analysis(
    args: &Args,
    #[allow(unused_variables)] file_hash: &str,
    input_path: &str,
) -> CliResult<Return<AnalysisResults>> {
    Ok(Return::new(perform_analysis(args, input_path)?))
}

// Cached analysis results for redis cache
#[io_cached(
    ty = "cached::RedisCache<String, AnalysisResults>",
    key = "String",
    convert = r##"{ get_analysis_cache_key(args, file_hash) }"##,
    create = r##" {
        let redis_config = REDISCONFIG.get().unwrap();
        let rediscache: RedisCache<String, AnalysisResults> = RedisCache::new("analysis", redis_config.ttl_secs)
            .set_namespace("descq")
            .set_refresh(redis_config.ttl_refresh)
            .set_connection_string(&redis_config.conn_str)
            .set_connection_pool_max_size(redis_config.max_pool_size)
            .build()
            .expect("error building analysis redis cache");
        log::info!("Analysis Redis cache created - conn_str: {conn_str} - refresh: {ttl_refresh} - ttl: {ttl_secs:?} - pool_size: {pool_size}",
            conn_str = redis_config.conn_str,
            ttl_refresh = redis_config.ttl_refresh,
            ttl_secs = redis_config.ttl_secs,
            pool_size = redis_config.max_pool_size);
        rediscache
    } "##,
    map_error = r##"|e| CliError::Other(format!("Analysis Redis Error: {e:?}"))"##,
    with_cached_flag = true
)]
fn get_redis_analysis(
    args: &Args,
    #[allow(unused_variables)] file_hash: &str,
    input_path: &str,
) -> CliResult<Return<AnalysisResults>> {
    Ok(Return::new(perform_analysis(args, input_path)?))
}

// Get output format (markdown is default)
fn get_output_format(args: &Args) -> CliResult<OutputFormat> {
    // Command-line flags take precedence over prompt file settings
    if let Some(format_str) = &args.flag_format {
        match format_str.to_lowercase().as_str() {
            "markdown" | "md" => Ok(OutputFormat::Markdown),
            "tsv" => Ok(OutputFormat::Tsv),
            "json" => Ok(OutputFormat::Json),
            "toon" => Ok(OutputFormat::Toon),
            _ => fail_incorrectusage_clierror!(
                "Invalid format '{}'. Must be one of: Markdown, TSV, JSON, TOON",
                format_str
            ),
        }
    } else {
        // If no command-line flags, check prompt file
        let prompt_file = get_prompt_file(args)?;
        match prompt_file.format.to_lowercase().as_str() {
            "markdown" | "md" => Ok(OutputFormat::Markdown),
            "tsv" => Ok(OutputFormat::Tsv),
            "json" => Ok(OutputFormat::Json),
            "toon" => Ok(OutputFormat::Toon),
            _ => fail_incorrectusage_clierror!(
                "Invalid format '{}'. Must be one of: Markdown, TSV, JSON, TOON",
                prompt_file.format
            ),
        }
    }
}

// Generate TSV output file path for a given PromptKind
// Extracts filestem from base output path and appends .{kind}.tsv
fn get_tsv_output_path(base_output: &str, kind: PromptType) -> String {
    let path = Path::new(base_output);
    let filestem = path.file_stem().and_then(|s| s.to_str()).unwrap_or(
        // If no file stem, use the whole path as base
        base_output,
    );

    // Get parent directory if it exists
    let parent = path.parent();
    let kind_str = kind.to_string().to_lowercase();

    if let Some(parent_path) = parent {
        parent_path
            .join(format!("{filestem}.{kind_str}.tsv"))
            .to_string_lossy()
            .to_string()
    } else {
        format!("{filestem}.{kind_str}.tsv")
    }
}

// Unified function to handle cached completions
fn get_cached_completion(
    args: &Args,
    client: &Client,
    model: &str,
    api_key: &str,
    cache_type: &CacheType,
    kind: PromptType,
    messages: &serde_json::Value,
) -> CliResult<CompletionResponse> {
    match cache_type {
        CacheType::Disk => {
            let dc_result = get_diskcache_completion(args, client, model, api_key, kind, messages)?;
            if dc_result.was_cached {
                print_status("    Disk cache hit!", None);
            }
            Ok(dc_result.value)
        },
        CacheType::Redis => {
            let rc_result = get_redis_completion(args, client, model, api_key, kind, messages)?;
            if rc_result.was_cached {
                print_status("    Redis cache hit!", None);
            }
            Ok(rc_result.value)
        },
        CacheType::Fresh => {
            // Make fresh API call and manually update cache
            let fresh_result = get_completion(args, client, model, api_key, messages, kind)?;
            // Manually update the appropriate cache with the fresh result
            if args.flag_redis_cache {
                let _ = get_redis_completion(args, client, model, api_key, kind, messages);
            } else {
                let _ = get_diskcache_completion(args, client, model, api_key, kind, messages);
            }
            Ok(fresh_result)
        },
        CacheType::None => get_completion(args, client, model, api_key, messages, kind),
    }
}

// Generates output for all inference options
fn run_inference_options(
    input_path: &str,
    args: &Args,
    api_key: &str,
    cache_type: &CacheType,
    analysis_results: &AnalysisResults,
) -> CliResult<()> {
    // Add --dictionary output as context if it is not empty
    fn get_messages(
        prompt: &str,
        system_prompt: &str,
        dictionary_completion: &str,
        session_state: Option<&SessionState>,
    ) -> serde_json::Value {
        let mut messages: Vec<serde_json::Value> = Vec::new();

        // Start with system prompt
        messages.push(json!({"role": "system", "content": system_prompt}));

        // Add dictionary completion if present
        if !dictionary_completion.is_empty() {
            messages.push(json!({
                "role": "assistant",
                "content": format!("The following is the Data Dictionary for the Dataset:\n\n{dictionary_completion}")
            }));
        }

        // Add session context if present
        if let Some(session) = session_state {
            // Add summary if present
            if let Some(ref summary) = session.summary {
                messages.push(json!({
                    "role": "system",
                    "content": format!("Previous conversation summary:\n\n{summary}")
                }));
            }

            let is_refinement = !session.messages.is_empty();

            // Add baseline SQL if this is a refinement request
            if is_refinement {
                let baseline_sql_used = if let Some(ref baseline_sql) = session.baseline_sql
                    && !baseline_sql.trim().is_empty()
                {
                    messages.push(json!({
                            "role": "assistant",
                            "content": format!("The baseline SQL query we are refining is:\n\n```sql\n{baseline_sql}\n```\n\nIMPORTANT: You must refine and modify this existing SQL query based on the user's request. Do NOT create a completely new query. Modify the baseline query to incorporate the requested changes.")
                        }));
                    true
                } else {
                    false
                };

                // If no baseline SQL in state but we have messages, try to extract it from the last
                // assistant message
                if !baseline_sql_used
                    && let Some(last_msg) = session
                        .messages
                        .iter()
                        .rev()
                        .find(|m| m.role == "assistant")
                    && let Some(sql) = regex_oncelock!(r"(?s)```sql\s*\n(.*?)\n\s*```")
                        .captures(&last_msg.content)
                        .and_then(|caps| caps.get(1).map(|m| m.as_str().trim().to_string()))
                    && !sql.is_empty()
                {
                    messages.push(json!({
                                    "role": "assistant",
                                    "content": format!("The baseline SQL query we are refining is:\n\n```sql\n{sql}\n```\n\nIMPORTANT: You must refine and modify this existing SQL query based on the user's request. Do NOT create a completely new query. Modify the baseline query to incorporate the requested changes.")
                                }));
                }
            }

            // Add recent messages (within sliding window) - but skip the last assistant message if
            // it's the baseline
            // We want to show the conversation history but emphasize refinement
            for msg in &session.messages {
                messages.push(json!({
                    "role": msg.role,
                    "content": msg.content
                }));
            }

            // Add SQL results if available (for refinement context)
            if is_refinement && let Some(ref results) = session.sql_results {
                messages.push(json!({
                        "role": "assistant",
                        "content": format!("Here are the first 10 rows from the last successful SQL query execution:\n\n```csv\n{results}\n```")
                    }));
            }

            // Add SQL errors if any (for refinement context)
            if is_refinement && !session.sql_errors.is_empty() {
                let errors_text = session.sql_errors.join("\n");
                messages.push(json!({
                    "role": "assistant",
                    "content": format!("Previous SQL execution errors encountered:\n\n{errors_text}")
                }));
            }

            // Modify the prompt to emphasize refinement
            if is_refinement {
                let refined_prompt = format!(
                    "User request: {prompt}\n\nPlease refine the baseline SQL query above to \
                     address this request. Return the complete refined SQL query that modifies \
                     the baseline query."
                );
                messages.push(json!({"role": "user", "content": refined_prompt}));
            } else {
                messages.push(json!({"role": "user", "content": prompt}));
            }
        } else {
            // No session, just add the prompt
            messages.push(json!({"role": "user", "content": prompt}));
        }

        json!(messages)
    }
    // Format output by replacing escape characters & adding two newlines
    fn format_output(str: &str) -> String {
        str.replace("\\n", "\n")
            .replace("\\t", "\t")
            .replace("\\\"", "\"")
            .replace("\\'", "'")
            .replace("\\`", "`")
            + "\n\n"
    }

    // Generate the plaintext and/or JSON output of an inference option
    fn process_output(
        kind: PromptType,
        completion_response: &CompletionResponse,
        total_json_output: &mut serde_json::Value,
        args: &Args,
        analysis_results: &AnalysisResults,
        model: &str,
        base_url: &str,
    ) -> CliResult<()> {
        // Skip outputting dictionary when using --prompt (but still generate it for context)
        if kind == PromptType::Dictionary && args.flag_prompt.is_some() {
            // Still store the dictionary in DATA_DICTIONARY_JSON for context, but don't output it
            // For --prompt mode, we still need to generate the full dictionary for context
            let (stats_records, ordered_col_names) = parse_stats_csv(&analysis_results.stats)?;
            let frequency_records = parse_frequency_csv(&analysis_results.frequency)?;

            // Determine which additional columns to include
            // Build IndexSet from ordered_column_names to preserve CSV order
            let avail_cols: IndexSet<String> = ordered_col_names.iter().cloned().collect();
            let addl_cols = determine_addl_cols(args, &avail_cols);

            let code_entries = generate_code_based_dictionary(
                &stats_records,
                &frequency_records,
                args.flag_enum_threshold,
                args.flag_num_examples,
                args.flag_truncate_str,
                &addl_cols,
            );

            let field_names: Vec<String> = code_entries.iter().map(|e| e.name.clone()).collect();
            let llm_labels_descriptions =
                parse_llm_dictionary_response(&completion_response.response, &field_names)
                    .unwrap_or_default();

            let combined_entries =
                combine_dictionary_entries(code_entries, &llm_labels_descriptions);
            let mut dictionary_json = format_dictionary_json(&combined_entries, args);
            // Replace attribution placeholder in JSON
            if let Some(attribution) = dictionary_json.get_mut("attribution")
                && let Some(attr_str) = attribution.as_str()
            {
                *attribution = json!(replace_attribution_placeholder(
                    attr_str,
                    args,
                    model,
                    base_url,
                    AttributionFormat::Markdown,
                    PromptType::Dictionary
                ));
            }

            DATA_DICTIONARY_JSON
                .get_or_init(|| serde_json::to_string_pretty(&dictionary_json).unwrap());
            // Don't add to total_json_output and don't output anything
            return Ok(());
        }

        let output_format = get_output_format(args)?;

        // Handle Dictionary type with neuro-procedural approach
        if kind == PromptType::Dictionary {
            // Parse stats and frequency data
            let (stats_records, ordered_col_names) = parse_stats_csv(&analysis_results.stats)?;
            let frequency_records = parse_frequency_csv(&analysis_results.frequency)?;

            // Determine which additional columns to include
            // Build IndexSet from ordered_column_names to preserve CSV order
            let avail_cols: IndexSet<String> = ordered_col_names.iter().cloned().collect();
            let addl_cols = determine_addl_cols(args, &avail_cols);

            // Generate code-based dictionary entries
            let code_entries = generate_code_based_dictionary(
                &stats_records,
                &frequency_records,
                args.flag_enum_threshold,
                args.flag_num_examples,
                args.flag_truncate_str,
                &addl_cols,
            );

            // Parse LLM response to get Label and Description
            let field_names: Vec<String> = code_entries.iter().map(|e| e.name.clone()).collect();
            let llm_labels_descriptions =
                parse_llm_dictionary_response(&completion_response.response, &field_names)
                    .unwrap_or_default();

            // Combine code-generated and LLM-generated fields
            let combined_entries =
                combine_dictionary_entries(code_entries, &llm_labels_descriptions);

            // Format output
            if output_format == OutputFormat::Json {
                let mut dictionary_json = format_dictionary_json(&combined_entries, args);
                // Replace attribution placeholder in JSON
                if let Some(attribution) = dictionary_json.get_mut("attribution")
                    && let Some(attr_str) = attribution.as_str()
                {
                    *attribution = json!(replace_attribution_placeholder(
                        attr_str,
                        args,
                        model,
                        base_url,
                        AttributionFormat::Markdown,
                        PromptType::Dictionary
                    ));
                }
                total_json_output[kind.to_string()] = json!({
                    "response": dictionary_json,
                    "reasoning": completion_response.reasoning,
                    "token_usage": completion_response.token_usage,
                });
                DATA_DICTIONARY_JSON
                    .get_or_init(|| serde_json::to_string_pretty(&dictionary_json).unwrap());
            } else if output_format == OutputFormat::Tsv {
                // TSV output
                let mut tsv_output = format_dictionary_tsv(&combined_entries);
                // Add comment lines for token usage and reasoning after the last TSV record
                tsv_output.push_str(&format_token_usage_comments(
                    &completion_response.reasoning,
                    &completion_response.token_usage,
                ));

                // Store in DATA_DICTIONARY_JSON for use by other prompts
                let dictionary_json = format_dictionary_json(&combined_entries, args);
                DATA_DICTIONARY_JSON
                    .get_or_init(|| serde_json::to_string_pretty(&dictionary_json).unwrap());

                // Write output to separate file per kind
                if let Some(output) = &args.flag_output {
                    let tsv_path = get_tsv_output_path(output, kind);
                    fs::write(&tsv_path, tsv_output.as_bytes())?;
                } else {
                    // This should not happen due to validation, but handle gracefully
                    print!("{tsv_output}");
                }
            } else if output_format == OutputFormat::Toon {
                // TOON output - accumulate in total_json_output like JSON format
                let mut dictionary_json = format_dictionary_json(&combined_entries, args);
                // Replace attribution placeholder in JSON (same as JSON format)
                if let Some(attribution) = dictionary_json.get_mut("attribution")
                    && let Some(attr_str) = attribution.as_str()
                {
                    *attribution = json!(replace_attribution_placeholder(
                        attr_str,
                        args,
                        model,
                        base_url,
                        AttributionFormat::Markdown,
                        PromptType::Dictionary
                    ));
                }
                total_json_output[kind.to_string()] = json!({
                    "response": dictionary_json,
                    "reasoning": completion_response.reasoning,
                    "token_usage": completion_response.token_usage,
                });
                DATA_DICTIONARY_JSON
                    .get_or_init(|| serde_json::to_string_pretty(&dictionary_json).unwrap());
            } else {
                // Markdown output
                let mut markdown_output = format_dictionary_markdown(&combined_entries);
                // Replace attribution placeholder in markdown
                markdown_output = replace_attribution_placeholder(
                    &markdown_output,
                    args,
                    model,
                    base_url,
                    AttributionFormat::Markdown,
                    PromptType::Dictionary,
                );
                let formatted_output = format!(
                    "# {}\n{}\n## REASONING\n\n{}\n## TOKEN USAGE\n\n{:?}\n---\n",
                    kind,
                    markdown_output,
                    completion_response.reasoning,
                    completion_response.token_usage
                );

                // Store in DATA_DICTIONARY_JSON for use by other prompts
                let dictionary_json = format_dictionary_json(&combined_entries, args);
                DATA_DICTIONARY_JSON
                    .get_or_init(|| serde_json::to_string_pretty(&dictionary_json).unwrap());

                // Write output
                if let Some(output) = &args.flag_output {
                    fs::OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(output)?
                        .write_all(formatted_output.as_bytes())?;
                } else {
                    println!("{formatted_output}");
                }
            }
            return Ok(());
        }

        // Check if this is a custom prompt response that contains SQL code
        let is_sql_response = kind == PromptType::Prompt
            && args.flag_sql_results.is_some()
            && completion_response.response.contains("```sql");

        // Process JSON output if expected
        if output_format == OutputFormat::Json && !is_sql_response {
            total_json_output[kind.to_string()] = if kind == PromptType::Description
                || kind == PromptType::Prompt
            {
                // For description and prompt, create an object
                // with both response, reasoning, and token usage
                json!({
                    "response": completion_response.response,
                    "reasoning": completion_response.reasoning,
                    "token_usage": completion_response.token_usage,
                })
            } else {
                // For dictionary and tags, try to extract JSON from response, but include reasoning
                let mut output_value = if let Ok(json_value) =
                    extract_json_from_output(&completion_response.response)
                {
                    // Create a structured object with data and reasoning
                    json!({
                        "response": json_value,
                        "reasoning": completion_response.reasoning,
                        "token_usage": completion_response.token_usage,
                    })
                } else {
                    // Fall back to string format with reasoning
                    json!({
                        "response": completion_response.response,
                        "reasoning": completion_response.reasoning,
                        "token_usage": completion_response.token_usage,
                    })
                };
                // Add metadata properties for Tags at the top level (always present)
                if kind == PromptType::Tags
                    && let Some(obj) = output_value.as_object_mut()
                {
                    obj.insert("num_tags".to_string(), json!(args.flag_num_tags));
                    obj.insert(
                        "tag_vocab".to_string(),
                        match &args.flag_tag_vocab {
                            Some(path) => json!(path.as_str()),
                            None => serde_json::Value::Null,
                        },
                    );
                }

                output_value
            };
            if kind == PromptType::Dictionary {
                DATA_DICTIONARY_JSON.get_or_init(|| {
                    serde_json::to_string_pretty(&total_json_output["dictionary"]["response"])
                        .unwrap()
                });
            }
        }
        // Process TSV output
        else if output_format == OutputFormat::Tsv && !is_sql_response {
            let tsv_output = if kind == PromptType::Description {
                format_description_tsv(
                    &completion_response.response,
                    &completion_response.reasoning,
                    &completion_response.token_usage,
                )
            } else if kind == PromptType::Prompt {
                format_prompt_tsv(
                    &completion_response.response,
                    &completion_response.reasoning,
                    &completion_response.token_usage,
                )
            } else if kind == PromptType::Tags {
                // Extract tags JSON from response
                let tags_json = match extract_json_from_output(&completion_response.response) {
                    Ok(json_value) => json_value,
                    Err(_) => json!({"tags": []}),
                };
                format_tags_tsv(
                    &tags_json,
                    &completion_response.reasoning,
                    &completion_response.token_usage,
                )
            } else {
                // Should not happen for TSV (Dictionary is handled separately above)
                // Fallback to description format
                format_description_tsv(
                    &completion_response.response,
                    &completion_response.reasoning,
                    &completion_response.token_usage,
                )
            };

            // Write output to separate file per kind
            if let Some(output) = &args.flag_output {
                let tsv_path = get_tsv_output_path(output, kind);
                fs::write(&tsv_path, tsv_output.as_bytes())?;
            } else {
                // This should not happen due to validation, but handle gracefully
                print!("{tsv_output}");
            }
        }
        // Process TOON output - accumulate in total_json_output like JSON format
        else if output_format == OutputFormat::Toon && !is_sql_response {
            total_json_output[kind.to_string()] = if kind == PromptType::Description
                || kind == PromptType::Prompt
            {
                // For description and prompt, create an object
                // with both response, reasoning, and token usage
                json!({
                    "response": completion_response.response,
                    "reasoning": completion_response.reasoning,
                    "token_usage": completion_response.token_usage,
                })
            } else {
                // For tags, try to extract JSON from response, but include reasoning
                let mut response_value = completion_response.response.clone();
                let mut attribution_value = serde_json::Value::Null;

                // For Tags, extract attribution from response if embedded
                if kind == PromptType::Tags {
                    // Look for attribution pattern: "Generated by..." or "{GENERATED_BY_SIGNATURE}"
                    if let Some(attr_start) = response_value.find("Generated by") {
                        // Extract attribution and remove from response
                        let attribution_text = response_value[attr_start..].trim().to_string();
                        response_value = response_value[..attr_start].trim().to_string();
                        attribution_value = json!(attribution_text);
                    } else if response_value.contains("{GENERATED_BY_SIGNATURE}") {
                        // Replace placeholder with actual attribution
                        let attribution_text = replace_attribution_placeholder(
                            "{GENERATED_BY_SIGNATURE}",
                            args,
                            model,
                            base_url,
                            AttributionFormat::Markdown,
                            PromptType::Tags,
                        );
                        response_value = response_value
                            .replace("{GENERATED_BY_SIGNATURE}", "")
                            .trim()
                            .to_string();
                        attribution_value = json!(attribution_text);
                    }
                }

                let mut output_value =
                    if let Ok(json_value) = extract_json_from_output(&response_value) {
                        // Create a structured object with data and reasoning
                        json!({
                            "response": json_value,
                            "reasoning": completion_response.reasoning,
                            "token_usage": completion_response.token_usage,
                        })
                    } else {
                        // Fall back to string format with reasoning
                        json!({
                            "response": response_value,
                            "reasoning": completion_response.reasoning,
                            "token_usage": completion_response.token_usage,
                        })
                    };
                // Add metadata properties for Tags at the top level (always present)
                if kind == PromptType::Tags
                    && let Some(obj) = output_value.as_object_mut()
                {
                    obj.insert("num_tags".to_string(), json!(args.flag_num_tags));
                    obj.insert(
                        "tag_vocab".to_string(),
                        match &args.flag_tag_vocab {
                            Some(path) => json!(path.as_str()),
                            None => serde_json::Value::Null,
                        },
                    );
                    // Add attribution as separate top-level key
                    if attribution_value != serde_json::Value::Null {
                        obj.insert("attribution".to_string(), attribution_value);
                    }
                }

                output_value
            };
        }
        // Process plaintext output
        else {
            let mut formatted_output = format_output(&completion_response.response);
            if kind == PromptType::Prompt && is_sql_response {
                // replace INPUT_TABLE_NAME with input_path
                formatted_output = {
                    let input_path = args.arg_input.as_deref().unwrap_or("input.csv");
                    if READ_CSV_AUTO_REGEX.is_match(&formatted_output) {
                        // DuckDB with read_csv_auto so replace with quoted path
                        // Escape single quotes in path to prevent SQL injection
                        let escaped_path = escape_sql_string(input_path);
                        READ_CSV_AUTO_REGEX
                            .replace_all(
                                &formatted_output,
                                format!("read_csv_auto('{escaped_path}', strict_mode=false)"),
                            )
                            .into_owned()
                    } else {
                        // Polars SQL - use table alias _t_1
                        formatted_output.replace(INPUT_TABLE_NAME, "_t_1")
                    }
                };
            }
            // append the reasoning to the output as a separate markdown section
            formatted_output = format!(
                "# {}\n{}\n## REASONING\n\n{}\n## TOKEN USAGE\n\n{:?}\n---\n",
                kind,
                formatted_output,
                completion_response.reasoning,
                completion_response.token_usage
            );
            // If --output is used, append plaintext to file, do not overwrite
            if let Some(output) = &args.flag_output {
                fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(output)?
                    .write_all(formatted_output.as_bytes())?;
            } else {
                println!("{formatted_output}");
            }
        }
        Ok(())
    }

    // Get completion from API
    let llm_start = Instant::now();

    let client = util::create_reqwest_blocking_client(
        args.flag_user_agent.clone(),
        // we do unwrap_or 0 here as we allow 0 as a valid timeout
        // per the usage text (normally, when using a local LLM)
        util::timeout_secs(args.flag_timeout).unwrap_or(0) as u16,
        args.flag_base_url.clone(),
    )?;

    // Verify model is valid
    let model = check_model(&client, Some(api_key), args)?;

    let mut total_json_output: serde_json::Value = json!({});
    let mut prompt: String;
    let mut system_prompt: String;
    let mut messages: serde_json::Value;
    let mut data_dict: CompletionResponse = CompletionResponse::default();
    let mut completion_response: CompletionResponse = CompletionResponse::default();

    // Generate dictionary output
    if args.flag_dictionary || args.flag_all || args.flag_prompt.is_some() {
        (prompt, system_prompt) = get_prompt(PromptType::Dictionary, Some(analysis_results), args)?;
        let start_time = Instant::now();
        print_status("  Inferring Data Dictionary...", None);
        messages = get_messages(&prompt, &system_prompt, "", None);

        // Special case: if --prompt is used with --fresh, use normal cache for dictionary
        let dictionary_cache_type = if args.flag_prompt.is_some() && args.flag_fresh {
            if args.flag_redis_cache {
                &CacheType::Redis
            } else {
                &CacheType::Disk
            }
        } else {
            cache_type
        };

        data_dict = get_cached_completion(
            args,
            &client,
            &model,
            api_key,
            dictionary_cache_type,
            PromptType::Dictionary,
            &messages,
        )?;
        print_status(
            &format!(
                "   Received dictionary inference.\n   {:?}\n  ",
                data_dict.token_usage
            ),
            Some(start_time.elapsed()),
        );
        let prompt_file = get_prompt_file(args)?;
        process_output(
            PromptType::Dictionary,
            &data_dict,
            &mut total_json_output,
            args,
            analysis_results,
            &model,
            &prompt_file.base_url,
        )?;
    }

    // Generate description output
    if args.flag_description || args.flag_all {
        (prompt, system_prompt) =
            get_prompt(PromptType::Description, Some(analysis_results), args)?;
        messages = get_messages(&prompt, &system_prompt, &data_dict.response, None);
        let start_time = Instant::now();
        print_status("  Inferring Description...", None);
        completion_response = get_cached_completion(
            args,
            &client,
            &model,
            api_key,
            cache_type,
            PromptType::Description,
            &messages,
        )?;
        print_status(
            format!(
                "   Received Description Inference.\n   {:?}\n  ",
                completion_response.token_usage
            )
            .as_str(),
            Some(start_time.elapsed()),
        );
        let prompt_file = get_prompt_file(args)?;
        process_output(
            PromptType::Description,
            &completion_response,
            &mut total_json_output,
            args,
            analysis_results,
            &model,
            &prompt_file.base_url,
        )?;
    }

    // Generate tags output
    if args.flag_tags || args.flag_all {
        (prompt, system_prompt) = get_prompt(PromptType::Tags, Some(analysis_results), args)?;
        // Only include dictionary context if dictionary was actually generated
        let dictionary_context = if args.flag_dictionary || args.flag_all {
            &data_dict.response
        } else {
            ""
        };
        messages = get_messages(&prompt, &system_prompt, dictionary_context, None);
        let start_time = Instant::now();
        if let Some(ref tag_vocab_uri) = args.flag_tag_vocab {
            print_status(
                &format!("  Inferring Tags with Tag Vocabulary ({tag_vocab_uri})...",),
                None,
            );
        } else {
            print_status("  Inferring Tags...", None);
        }
        completion_response = get_cached_completion(
            args,
            &client,
            &model,
            api_key,
            cache_type,
            PromptType::Tags,
            &messages,
        )?;
        print_status(
            &format!(
                "   Received Tags inference.\n   {:?}\n  ",
                completion_response.token_usage
            ),
            Some(start_time.elapsed()),
        );
        let prompt_file = get_prompt_file(args)?;
        process_output(
            PromptType::Tags,
            &completion_response,
            &mut total_json_output,
            args,
            analysis_results,
            &model,
            &prompt_file.base_url,
        )?;
    }

    // Generate custom prompt output
    let mut has_sql_query = false;
    let mut session_state: Option<SessionState> = None;

    // Normalize session path once if provided
    let normalized_session_path: Option<String> = args
        .flag_session
        .as_ref()
        .map(|p| normalize_session_path(p));

    if let Some(ref user_prompt) = args.flag_prompt {
        // Handle session if --session is provided
        if let Some(ref normalized_path) = normalized_session_path {
            let session_path = Path::new(normalized_path);

            // Set default session length if not provided
            let session_len = if args.flag_session_len == 0 {
                10
            } else {
                args.flag_session_len
            };

            session_state = Some(load_session(session_path)?);
            if let Some(ref mut state) = session_state {
                // If not first message, check relevance and apply sliding window
                if !state.messages.is_empty() {
                    if let Some(ref baseline_sql) = state.baseline_sql
                        && !check_message_relevance(
                            user_prompt,
                            baseline_sql,
                            args,
                            &client,
                            api_key,
                        )?
                    {
                        return fail_clierror!(
                            "The current message does not appear to be related to refining the \
                             baseline SQL query. Please start a new session for unrelated queries."
                        );
                    }

                    // Apply sliding window
                    apply_sliding_window(state, session_len, args, &client, api_key)?;
                }
            }
        }

        (prompt, system_prompt) = get_prompt(PromptType::Prompt, Some(analysis_results), args)?;
        let start_time = Instant::now();
        print_status("  Answering Custom Prompt...", None);
        messages = get_messages(
            &prompt,
            &system_prompt,
            &data_dict.response,
            session_state.as_ref(),
        );
        completion_response = get_cached_completion(
            args,
            &client,
            &model,
            api_key,
            cache_type,
            PromptType::Prompt,
            &messages,
        )?;
        print_status(
            &format!(
                "   Received Custom Prompt Answer.\n   {:?}\n  ",
                completion_response.token_usage
            ),
            Some(start_time.elapsed()),
        );
        has_sql_query = completion_response.response.contains("```sql");
        if has_sql_query {
            print_status(
                &format!(
                    "  Cannot answer the prompt using just Summary Statistics & Frequency \
                     Distribution data.\n  Generated a {} SQL query to answer the prompt \
                     deterministically.",
                    if should_use_duckdb() {
                        "DuckDB"
                    } else {
                        "Polars"
                    }
                ),
                None,
            );
        }

        // Update session state with new messages
        if let Some(ref mut state) = session_state {
            // Add user message
            state.messages.push(SessionMessage {
                role:      "user".to_string(),
                content:   user_prompt.clone(),
                timestamp: chrono::Utc::now().to_rfc3339(),
            });

            // Add assistant response
            state.messages.push(SessionMessage {
                role:      "assistant".to_string(),
                content:   completion_response.response.clone(),
                timestamp: chrono::Utc::now().to_rfc3339(),
            });

            // Note: We don't save here to avoid overwriting the session file multiple times
            // The session will be saved after SQL execution (if any) or at the end of the function
        }

        let prompt_file = get_prompt_file(args)?;
        process_output(
            PromptType::Prompt,
            &completion_response,
            &mut total_json_output,
            args,
            analysis_results,
            &model,
            &prompt_file.base_url,
        )?;
    }

    // if max-tokens is set and completion token usage is greater than max-tokens, return an error
    if args.flag_max_tokens > 0
        && completion_response.token_usage.completion >= args.flag_max_tokens as u64
    {
        return fail_clierror!(
            "Completion token usage is greater than or equal to --max-tokens ({}): {}",
            args.flag_max_tokens,
            completion_response.token_usage.completion,
        );
    }

    print_status("LLM inference/s completed.", Some(llm_start.elapsed()));

    // if args.flag_output is set, display the output to the console
    if let Some(output) = &args.flag_output {
        print_status(&format!("Output written to {output}"), None);
    }

    if let Some(sql_results) = &args.flag_sql_results
        && has_sql_query
    {
        // Check if file exists and is writeable, or can be created
        let sql_results_path = Path::new(sql_results);
        if sql_results_path.exists() {
            if fs::metadata(sql_results_path)?.permissions().readonly() {
                return fail_clierror!(
                    "SQL results file exists but is not writeable: {}",
                    sql_results_path.display()
                );
            }
        } else {
            // Try creating the file to verify we can write to it
            match fs::File::create(sql_results_path) {
                Ok(_) => {
                    // Clean up the test file
                    fs::remove_file(sql_results_path)?;
                },
                Err(e) => {
                    return fail_clierror!(
                        "Cannot create SQL results file {}: {}",
                        sql_results_path.display(),
                        e
                    );
                },
            }
        }

        let sql_query_start = Instant::now();
        print_status(
            &format!(
                "\nSQL results file specified.\n  Executing SQL query and saving results to \
                 {sql_results}..."
            ),
            None,
        );

        // Extract SQL query code block using regex
        let Some(mut sql_query) = regex_oncelock!(r"(?s)```sql\s*\n(.*?)\n\s*```")
            .captures(&completion_response.response)
            .and_then(|caps| caps.get(1).map(|m| m.as_str().trim().to_string()))
        else {
            // Invalidate the prompt cache entry so user can try again without reinferring
            // dictionary
            if cache_type != &CacheType::Fresh && cache_type != &CacheType::None {
                let _ = invalidate_cache_entry(args, PromptType::Prompt);
            }
            return fail_clierror!("Failed to extract SQL query from custom prompt response");
        };

        // Check if DuckDB should be used
        if should_use_duckdb() {
            // For DuckDB, replace {INPUT_TABLE_NAME} with read_csv function call
            // Escape single quotes in path to prevent SQL injection
            let escaped_path = escape_sql_string(input_path);
            if READ_CSV_AUTO_REGEX.is_match(&sql_query) {
                // DuckDB with read_csv_auto so replace with quoted path
                sql_query = READ_CSV_AUTO_REGEX
                    .replace_all(
                        &sql_query,
                        format!("read_csv_auto('{escaped_path}', strict_mode=false)"),
                    )
                    .into_owned();
            } else {
                // if READ_CSV_AUTO_REGEX doesn't match, add fallback to replace {INPUT_TABLE_NAME}
                // with read_csv_auto function call
                sql_query = sql_query.replace(
                    INPUT_TABLE_NAME,
                    &format!("read_csv_auto('{escaped_path}', strict_mode=false)"),
                );
            }
            log::debug!("DuckDB SQL query:\n{sql_query}");

            let (_, stderr) =
                match run_duckdb_query(&sql_query, sql_results, "  DuckDB SQL query issued.") {
                    Ok((stdout, stderr)) => {
                        // Check stderr for error messages
                        if stderr.to_ascii_lowercase().contains(" error:") {
                            track_sql_error_in_session(
                                session_state.as_mut(),
                                normalized_session_path.as_ref(),
                                format!("DuckDB SQL query execution failed: {stderr}"),
                            );
                            // Invalidate the prompt cache entry so user can try again without
                            // reinferring dictionary
                            if cache_type != &CacheType::Fresh && cache_type != &CacheType::None {
                                let _ = invalidate_cache_entry(args, PromptType::Prompt);
                            }
                            return fail_clierror!("DuckDB SQL query execution failed: {stderr}");
                        }
                        (stdout, stderr)
                    },
                    Err(e) => {
                        track_sql_error_in_session(
                            session_state.as_mut(),
                            normalized_session_path.as_ref(),
                            format!("DuckDB SQL query execution failed: {e}"),
                        );
                        // Invalidate the prompt cache entry so user can try again
                        if cache_type != &CacheType::Fresh && cache_type != &CacheType::None {
                            let _ = invalidate_cache_entry(args, PromptType::Prompt);
                        }
                        return Err(e);
                    },
                };

            // Track successful execution in session
            update_session_after_sql_success(session_state.as_mut(), sql_results, &sql_query);

            print_status(
                &format!("DuckDB SQL query successful. Saved results to {sql_results} {stderr}"),
                Some(sql_query_start.elapsed()),
            );
        } else {
            #[cfg(feature = "polars")]
            {
                // Use the existing sqlp functionality
                sql_query = sql_query.replace(INPUT_TABLE_NAME, "_t_1");
                log::debug!("SQL query:\n{sql_query}");

                // Clone sql_query before moving it into fs::write, so we can use it later for
                // baseline SQL
                let sql_query_for_baseline = sql_query.clone();

                // save sql query to a temporary file with a .sql extension
                // this tempfile is automatically deleted after the command finishes
                let sql_query_file = tempfile::Builder::new().suffix(".sql").tempfile()?;
                fs::write(&sql_query_file, sql_query)?;

                let (_, stderr) = match run_qsv_cmd(
                    "sqlp",
                    &[
                        &sql_query_file.path().display().to_string(),
                        "--try-parsedates",
                        "--infer-len",
                        "10000",
                        "--output",
                        sql_results,
                    ],
                    input_path,
                    "  Polars SQL query issued.",
                ) {
                    Ok((stdout, stderr)) => {
                        // Check stderr for error messages
                        if stderr.to_ascii_lowercase().contains("error:") {
                            track_sql_error_in_session(
                                session_state.as_mut(),
                                normalized_session_path.as_ref(),
                                format!("Polars SQL query error detected: {stderr}"),
                            );
                            return handle_sql_error(
                                args,
                                cache_type,
                                sql_query_file.path(),
                                sql_results_path,
                                &format!("Polars SQL query error detected: {stderr}"),
                            );
                        }
                        // the polars sql query is successful
                        // set the sql_results file to have a .csv extension
                        let csv_path = sql_results_path.with_extension("csv");
                        let _ = fs::rename(sql_results_path, &csv_path);

                        // Track successful execution in session
                        if let Some(ref mut state) = session_state {
                            if csv_path.exists()
                                && let Ok(sample) = extract_sql_sample(&csv_path)
                            {
                                state.sql_results = Some(sample);
                                state.sql_errors.clear(); // Clear errors on success
                            }

                            // Extract and store baseline SQL only after successful execution
                            // This ensures baseline SQL is only set when the query executes
                            // successfully
                            if state.baseline_sql.is_none() {
                                state.baseline_sql = Some(sql_query_for_baseline);
                            }
                        }
                        // Note: We can't use update_session_after_sql_success here because
                        // Polars renames the file, so we need to handle it inline

                        (stdout, stderr)
                    },
                    Err(e) => {
                        track_sql_error_in_session(
                            session_state.as_mut(),
                            normalized_session_path.as_ref(),
                            format!("Polars SQL query execution failed: {e}"),
                        );
                        return handle_sql_error(
                            args,
                            cache_type,
                            sql_query_file.path(),
                            sql_results_path,
                            &format!("Polars SQL query execution failed: {e}"),
                        );
                    },
                };

                if stderr.starts_with("Failed to execute query:") {
                    track_sql_error_in_session(
                        session_state.as_mut(),
                        normalized_session_path.as_ref(),
                        stderr.clone(),
                    );
                    return handle_sql_error(
                        args,
                        cache_type,
                        sql_query_file.path(),
                        sql_results_path,
                        // "Polars SQL query failed. Failed SQL query saved to output file",
                        &stderr,
                    );
                }
                print_status(
                    &format!(
                        "Polars SQL query successful. Saved results to {sql_results} {stderr}"
                    ),
                    Some(sql_query_start.elapsed()),
                );
            }
            #[cfg(not(feature = "polars"))]
            {
                // Invalidate cache entry so user can try again without reinferring dictionary
                if cache_type != &CacheType::Fresh && cache_type != &CacheType::None {
                    let _ = invalidate_cache_entry(args, PromptType::Prompt);
                }
                return fail_clierror!(
                    "Cannot answer the prompt using just Summary Statistics & Frequency \
                     Distribution data. However, \"SQL RAG\" mode is only supported when the \
                     `polars` feature is enabled, or when using DuckDB via the \
                     QSV_DESCRIBEGPT_DB_ENGINE environment variable."
                );
            }
        }
    }

    // Expecting JSON or TOON output
    let output_format = get_output_format(args)?;
    if output_format == OutputFormat::Json {
        // Format & print JSON output
        let json_output = &simd_json::to_string_pretty(&total_json_output)?;
        // Write to file if --output is used, or overwrite if already exists
        if let Some(output_file_path) = &args.flag_output {
            fs::write(output_file_path, json_output)?;
        } else {
            println!("{json_output}");
        }
    } else if output_format == OutputFormat::Toon {
        // Format & print TOON output - encode the entire accumulated JSON structure
        let opts = EncodeOptions::new();
        let toon_output = encode(&total_json_output, &opts)
            .map_err(|e| CliError::Other(format!("Failed to encode to TOON: {e}")))?;
        // Write to file if --output is used, or overwrite if already exists
        if let Some(output_file_path) = &args.flag_output {
            fs::write(output_file_path, toon_output)?;
        } else {
            println!("{toon_output}");
        }
    }

    // Save session if it exists
    if let Some(ref state) = session_state
        && let Some(ref normalized_path) = normalized_session_path
    {
        save_session(Path::new(normalized_path), state)?;
    }

    Ok(())
}

// Helper function to run DuckDB queries
fn run_duckdb_query(
    sql_query: &str,
    output_path: &str,
    status_msg: &str,
) -> CliResult<(String, String)> {
    let duckdb_path = get_duckdb_path()?;
    let start_time = Instant::now();

    let mut cmd = Command::new(duckdb_path);
    cmd.arg("-csv").arg("-c").arg(sql_query);

    let output = cmd
        .output()
        .map_err(|e| CliError::Other(format!("Error while executing DuckDB command: {e:?}")))?;

    if !status_msg.is_empty() {
        print_status(status_msg, Some(start_time.elapsed()));
    }

    // Check if DuckDB command failed (non-zero exit status)
    if !output.status.success() {
        // If SQL execution failed, write the SQL query to output file with a .sql extension
        let output_path = Path::new(output_path).with_extension("sql");
        if let Err(e) = fs::write(&output_path, sql_query) {
            return fail_clierror!("Failed to write SQL query to {output_path:?}: {e}");
        }
        let stderr_str =
            simdutf8::basic::from_utf8(&output.stderr).unwrap_or("<unable to parse stderr>");
        return fail_clierror!(
            "DuckDB SQL query execution failed:\n{stderr_str}\nFailed SQL query saved to \
             {output_path:?}"
        );
    }

    let Ok(stdout_str) = simdutf8::basic::from_utf8(&output.stdout) else {
        return fail_clierror!("Unable to parse stdout of DuckDB command:\n{output:?}");
    };
    let Ok(stderr_str) = simdutf8::basic::from_utf8(&output.stderr) else {
        return fail_clierror!("Unable to parse stderr of DuckDB command:\n{output:?}");
    };

    // Also check stderr for error messages even if exit status is 0
    if stderr_str.to_ascii_lowercase().contains(" error:") {
        return fail_clierror!("DuckDB SQL query error detected:\n{stderr_str}");
    }

    // SQL successful, write the output to the specified file with a .csv extension
    if !output_path.is_empty() {
        let output_path = Path::new(output_path).with_extension("csv");

        if let Err(e) = fs::write(&output_path, stdout_str) {
            return fail_clierror!("Failed to write SQL results to {output_path:?}: {e}");
        }
    }

    Ok((stdout_str.to_string(), stderr_str.to_string()))
}

fn determine_cache_kinds_to_remove(args: &Args) -> Vec<PromptType> {
    if args.flag_dictionary {
        vec![PromptType::Dictionary]
    } else if args.flag_description {
        vec![PromptType::Description]
    } else if args.flag_tags {
        vec![PromptType::Tags]
    } else if args.flag_prompt.is_some() {
        vec![PromptType::Prompt]
    } else {
        vec![
            PromptType::Dictionary,
            PromptType::Description,
            PromptType::Tags,
            PromptType::Prompt,
        ]
    }
}

/// Remove a cache entry by key (works for both disk and redis cache)
fn remove_cache_entry_by_key(key: &str, args: &Args, kind: PromptType, success_msg: &str) {
    if !args.flag_no_cache {
        // Disk cache
        let key_string = key.to_string();
        if let Err(e) = GET_DISKCACHE_COMPLETION.cache_remove(&key_string) {
            print_status(
                &format!("Warning: Cannot remove cache entry for {kind}: {e:?}"),
                None,
            );
        } else {
            print_status(success_msg, None);
            // Flush the disk cache to ensure changes are persisted
            if let Err(e) = GET_DISKCACHE_COMPLETION.connection().flush() {
                print_status(&format!("Warning: Cannot flush disk cache: {e:?}"), None);
            } else if success_msg.contains("removed") {
                print_status("Flushed disk cache after removing cache entry", None);
            }
        }
    } else if args.flag_redis_cache {
        // Redis cache
        let conn_str = &REDISCONFIG.get().unwrap().conn_str;
        if let Ok(redis_client) = redis::Client::open(conn_str.to_string())
            && let Ok(mut redis_conn) = redis_client.get_connection()
        {
            match redis::cmd("DEL").arg(key).exec(&mut redis_conn) {
                Ok(()) => print_status(success_msg, None),
                Err(e) => print_status(
                    &format!("Warning: Cannot remove cache entry for {kind}: {e:?}"),
                    None,
                ),
            }
        }
    }
}

// Helper function to invalidate a specific cache entry by modifying the cache key
fn invalidate_cache_entry(args: &Args, kind: PromptType) -> CliResult<()> {
    if kind == PromptType::Prompt {
        // For prompt kind, invalidate the validity flag
        let prompt_content = args.flag_prompt.as_ref();
        invalidate_prompt_validity_flag(args, prompt_content);

        // Use the existing helper function to remove cache entries with both "valid" and "invalid"
        // flags
        let prompt_file = get_prompt_file(args)?;
        let base_key = {
            let file_hash = FILE_HASH.get().unwrap_or(&String::new()).clone();
            let prompt_content_for_key = args.flag_prompt.as_ref();

            format!(
                "{:?}{:?}{:?}{:?}{:?}{:?}{}{}{:?}",
                args.arg_input,
                args.flag_prompt_file,
                prompt_content_for_key,
                args.flag_max_tokens,
                args.flag_addl_props,
                &prompt_file.model,
                kind,
                file_hash,
                args.flag_language
            )
        };

        let removed = try_remove_prompt_cache_entries(&base_key);
        if removed {
            print_status(
                &format!("Removed cache entry for {kind} due to SQL execution failure"),
                None,
            );
        } else {
            print_status(
                &format!("Warning: Could not remove cache entry for {kind}"),
                None,
            );
        }
    } else {
        // For other kinds, try to remove the cache entry directly
        let prompt_file = get_prompt_file(args)?;
        let key = get_cache_key(args, kind, &prompt_file.model);
        remove_cache_entry_by_key(
            &key,
            args,
            kind,
            &format!("Removed cache entry for {kind} due to SQL execution failure"),
        );
    }

    Ok(())
}

/// Track SQL error in session state if present
fn track_sql_error_in_session(
    session_state: Option<&mut SessionState>,
    normalized_session_path: Option<&String>,
    error_msg: String,
) {
    if let Some(state) = session_state {
        state.sql_errors.push(error_msg);
        if let Some(normalized_path) = normalized_session_path {
            let _ = save_session(Path::new(normalized_path), state);
        }
    }
}

/// Update session state after successful SQL execution
fn update_session_after_sql_success(
    session_state: Option<&mut SessionState>,
    sql_results: &str,
    sql_query: &str,
) {
    if let Some(state) = session_state {
        let results_path = Path::new(sql_results).with_extension("csv");
        if results_path.exists()
            && let Ok(sample) = extract_sql_sample(&results_path)
        {
            state.sql_results = Some(sample);
            state.sql_errors.clear(); // Clear errors on success
        }

        // Extract and store baseline SQL only after successful execution
        // This ensures baseline SQL is only set when the query executes successfully
        if state.baseline_sql.is_none() {
            state.baseline_sql = Some(sql_query.to_string());
        }
    }
}

#[allow(dead_code)]
/// Helper function to handle SQL error cases by invalidating cache and saving the failed query
fn handle_sql_error(
    args: &Args,
    cache_type: &CacheType,
    sql_query_file: &std::path::Path,
    sql_results_path: &std::path::Path,
    error_msg: &str,
) -> CliResult<()> {
    // Invalidate cache entry so user can try again without reinferring dictionary
    if cache_type != &CacheType::Fresh && cache_type != &CacheType::None {
        let _ = invalidate_cache_entry(args, PromptType::Prompt);
    }
    // SQL execution failed, copy sql_query_file to sql_results_path
    let output_path = Path::new(sql_results_path).with_extension("sql");
    if let Err(e) = fs::copy(sql_query_file, &output_path) {
        return fail_clierror!("Failed to copy SQL query to {sql_results_path:?}: {e}");
    }
    fail_clierror!("{error_msg}")
}

/// Normalize session path to always have .md extension
fn normalize_session_path(session_path: &str) -> String {
    let path = Path::new(session_path);
    if let Some(ext) = path.extension()
        && ext == "md"
    {
        return session_path.to_string();
    }
    // If no extension or wrong extension, ensure .md extension
    // Use with_extension which replaces existing extension or adds if none exists
    path.with_extension("md").to_string_lossy().to_string()
}

/// Load session state from a markdown file
fn load_session(session_path: &Path) -> CliResult<SessionState> {
    if !session_path.exists() {
        return Ok(SessionState {
            baseline_sql: None,
            messages:     Vec::new(),
            sql_results:  None,
            sql_errors:   Vec::new(),
            summary:      None,
        });
    }

    let content = fs::read_to_string(session_path)?;
    let mut state = SessionState {
        baseline_sql: None,
        messages:     Vec::new(),
        sql_results:  None,
        sql_errors:   Vec::new(),
        summary:      None,
    };

    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;
    let mut current_content = String::new();
    let mut current_role = String::new();

    while i < lines.len() {
        let line = lines[i].trim();

        if line.starts_with("# Session:") {
            // Skip header
        } else if line == "## Baseline SQL Query" {
            current_content.clear();
            i += 1;
            // Skip opening ```sql
            if i < lines.len() && lines[i].trim() == "```sql" {
                i += 1;
            }
            // Read SQL until closing ```
            while i < lines.len() && !lines[i].trim().starts_with("```") {
                if !current_content.is_empty() {
                    current_content.push('\n');
                }
                current_content.push_str(lines[i]);
                i += 1;
            }
            state.baseline_sql = Some(current_content.trim().to_string());
            current_content.clear();
        } else if line == "## Conversation History" {
            i += 1;
            // Parse messages
            let mut in_content_section = false;
            let mut in_code_block = false;
            while i < lines.len() {
                let msg_line = lines[i];
                let msg_line_trimmed = msg_line.trim();

                // Check if we've hit the next section header (## at start of line, not indented)
                // Only break if we're not inside a code block
                if !in_code_block
                    && msg_line_trimmed.starts_with("##")
                    && !msg_line_trimmed.starts_with("###")
                {
                    // Save the current message before breaking
                    if !current_content.is_empty() && !current_role.is_empty() {
                        state.messages.push(SessionMessage {
                            role:      current_role.clone(),
                            content:   current_content.trim().to_string(),
                            timestamp: chrono::Utc::now().to_rfc3339(),
                        });
                    }
                    break;
                }

                // Track code blocks to avoid breaking on ## inside them
                if msg_line_trimmed.starts_with("```") {
                    in_code_block = !in_code_block;
                }

                if msg_line_trimmed.starts_with("### Message") {
                    // New message - save previous if exists
                    if !current_content.is_empty() && !current_role.is_empty() {
                        state.messages.push(SessionMessage {
                            role:      current_role.clone(),
                            content:   current_content.trim().to_string(),
                            timestamp: chrono::Utc::now().to_rfc3339(),
                        });
                    }
                    // Reset for new message
                    current_content.clear();
                    current_role.clear();
                    in_content_section = false;
                    in_code_block = false;
                } else if msg_line_trimmed.starts_with("**Role:**") {
                    current_role = msg_line_trimmed.replace("**Role:**", "").trim().to_string();
                    in_content_section = false;
                } else if msg_line_trimmed.starts_with("**Content:**") {
                    // Content section starts - clear any previous content for this message
                    current_content.clear();
                    in_content_section = true;
                } else if in_content_section {
                    // We're in the content section - add everything (including empty lines and code
                    // blocks) Always add a newline before adding content
                    // (except for the very first line)
                    if !current_content.is_empty() {
                        current_content.push('\n');
                    }
                    // Add the line as-is (preserving original formatting)
                    current_content.push_str(msg_line);
                }
                i += 1;
            }
            // Add last message if we didn't break on a section header
            if !current_content.is_empty() && !current_role.is_empty() {
                state.messages.push(SessionMessage {
                    role:      current_role.clone(),
                    content:   current_content.trim().to_string(),
                    timestamp: chrono::Utc::now().to_rfc3339(),
                });
            }
            continue;
        } else if line == "## SQL Results (Last Successful)" {
            current_content.clear();
            i += 1;
            // Skip opening ```csv
            if i < lines.len() && lines[i].trim() == "```csv" {
                i += 1;
            }
            // Read CSV until closing ```
            while i < lines.len() && !lines[i].trim().starts_with("```") {
                if !current_content.is_empty() {
                    current_content.push('\n');
                }
                current_content.push_str(lines[i]);
                i += 1;
            }
            state.sql_results = Some(current_content.trim().to_string());
            current_content.clear();
        } else if line == "## SQL Errors" {
            i += 1;
            // Read error list items
            while i < lines.len() && !lines[i].trim().starts_with("##") {
                let line = lines[i].trim();
                if line.starts_with("- ") {
                    let error = line.strip_prefix("- ").unwrap_or(line).to_string();
                    state.sql_errors.push(error);
                }
                i += 1;
            }
            continue;
        } else if line == "## Summary" {
            current_content.clear();
            i += 1;
            // Read summary until next section or end
            while i < lines.len() && !lines[i].trim().starts_with("##") {
                if !current_content.is_empty() {
                    current_content.push('\n');
                }
                current_content.push_str(lines[i]);
                i += 1;
            }
            state.summary = Some(current_content.trim().to_string());
            current_content.clear();
            continue;
        }
        i += 1;
    }

    Ok(state)
}

/// Save session state to a markdown file
fn save_session(session_path: &Path, state: &SessionState) -> CliResult<()> {
    use std::fmt::Write as _; // import without risk of name clashing

    // Ensure parent directory exists
    if let Some(parent) = session_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut content = String::new();
    let _ = write!(
        content,
        "# Session: {}\n\n",
        session_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
    );

    // Baseline SQL Query
    if let Some(ref sql) = state.baseline_sql {
        content.push_str("## Baseline SQL Query\n\n");
        content.push_str("```sql\n");
        content.push_str(sql);
        content.push_str("\n```\n\n");
    }

    // Conversation History
    content.push_str("## Conversation History\n\n");
    for (idx, msg) in state.messages.iter().enumerate() {
        let _ = write!(content, "### Message {}\n\n", idx + 1);
        let _ = write!(content, "**Role:** {}\n\n", msg.role);
        let _ = write!(content, "**Content:**\n\n{}\n\n", msg.content);
    }

    // SQL Results
    if let Some(ref results) = state.sql_results {
        content.push_str("## SQL Results (Last Successful)\n\n");
        content.push_str("```csv\n");
        content.push_str(results);
        content.push_str("\n```\n\n");
    }

    // SQL Errors
    if !state.sql_errors.is_empty() {
        content.push_str("## SQL Errors\n\n");
        for error in &state.sql_errors {
            let _ = writeln!(content, "- {error}");
        }
        content.push('\n');
    }

    // Summary
    if let Some(ref summary) = state.summary {
        content.push_str("## Summary\n\n");
        content.push_str(summary);
        content.push('\n');
    }

    fs::write(session_path, content)?;
    Ok(())
}

/// Extract first 10 rows from a CSV file
fn extract_sql_sample(csv_path: &Path) -> CliResult<String> {
    use std::io::{BufRead, BufReader};

    let file = fs::File::open(csv_path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut result = String::new();

    // Read header
    if let Some(Ok(header)) = lines.next() {
        result.push_str(&header);
        result.push('\n');
    }

    // Read up to 10 data rows
    for _ in 0..10 {
        if let Some(Ok(line)) = lines.next() {
            result.push_str(&line);
            result.push('\n');
        } else {
            break;
        }
    }

    Ok(result.trim().to_string())
}

/// Generate summary of old messages using LLM
fn generate_summary(
    old_messages: &[SessionMessage],
    args: &Args,
    client: &Client,
    api_key: &str,
) -> CliResult<String> {
    use std::fmt::Write as _; // import without risk of name clashing

    let mut summary_prompt = String::from(
        "Please provide a concise summary of the following conversation history. Focus on the key \
         SQL query refinements, user requests, and assistant responses:\n\n",
    );

    for msg in old_messages {
        let _ = write!(summary_prompt, "{}: {}\n\n", msg.role, msg.content);
    }

    summary_prompt
        .push_str("\nProvide a brief summary that captures the essence of this conversation:");

    let system_prompt = "You are a helpful assistant that summarizes conversation history for SQL \
                         query refinement sessions.";
    let messages = json!([
        {"role": "system", "content": system_prompt},
        {"role": "user", "content": summary_prompt}
    ]);

    let model = check_model(client, Some(api_key), args)?;
    let completion = get_completion(args, client, &model, api_key, &messages, PromptType::Prompt)?;
    Ok(completion.response)
}

/// Apply sliding window to session messages, summarizing older ones if needed
fn apply_sliding_window(
    state: &mut SessionState,
    max_len: usize,
    args: &Args,
    client: &Client,
    api_key: &str,
) -> CliResult<()> {
    if state.messages.len() <= max_len {
        return Ok(());
    }

    let num_to_summarize = state.messages.len() - max_len;
    let old_messages: Vec<SessionMessage> = state.messages.drain(..num_to_summarize).collect();

    // Generate summary of old messages
    let summary = generate_summary(&old_messages, args, client, api_key)?;

    // Combine with existing summary if present
    if let Some(ref existing_summary) = state.summary {
        state.summary = Some(format!("{existing_summary}\n\n{summary}"));
    } else {
        state.summary = Some(summary);
    }

    Ok(())
}

/// Check if a message is relevant to the baseline SQL query
fn check_message_relevance(
    prompt: &str,
    baseline_sql: &str,
    args: &Args,
    client: &Client,
    api_key: &str,
) -> CliResult<bool> {
    // Heuristic check: Look for SQL-related keywords
    let sql_keywords = [
        "sql",
        "query",
        "select",
        "where",
        "join",
        "group",
        "order",
        "filter",
        "refine",
        "modify",
        "change",
        "update",
        "fix",
        "correct",
        "improve",
        "add",
        "remove",
        "include",
        "exclude",
        "sort",
        "aggregate",
        "count",
    ];

    let prompt_lower = prompt.to_lowercase();
    let has_sql_keywords = sql_keywords.iter().any(|kw| prompt_lower.contains(kw));

    // Also check if prompt references previous query/results
    let has_references = prompt_lower.contains("previous")
        || prompt_lower.contains("last")
        || prompt_lower.contains("above")
        || prompt_lower.contains("before");

    if has_sql_keywords || has_references {
        return Ok(true);
    }

    // LLM check: Ask LLM if message is related to refining the SQL query
    let relevance_prompt = format!(
        "The user has been working on refining a SQL query. The baseline SQL query \
         is:\n\n```sql\n{baseline_sql}\n```\n\nUser's new message: \"{prompt}\"\n\nIs this \
         message related to refining, modifying, or improving the SQL query above? Answer with \
         only 'yes' or 'no'."
    );

    let system_prompt = "You are a helpful assistant that determines if user messages are related \
                         to SQL query refinement.";
    let messages = json!([
        {"role": "system", "content": system_prompt},
        {"role": "user", "content": relevance_prompt}
    ]);

    let model = check_model(client, Some(api_key), args)?;
    let completion = get_completion(args, client, &model, api_key, &messages, PromptType::Prompt)?;
    let response_lower = completion.response.to_lowercase().trim().to_string();

    Ok(response_lower.contains("yes") || response_lower == "y")
}

/// Determine which additional columns to include based on args
/// Returns a vector of column names in the order they should appear
/// Only adds columns when --addl-cols flag is set
/// available_columns: IndexSet of all additional columns (preserves CSV order)
fn determine_addl_cols(args: &Args, avail_cols: &IndexSet<String>) -> Vec<String> {
    // Default list of additional columns
    const DEFAULT_COLUMNS: &[&str] = &[
        "sort_order",
        "sortiness",
        "mean",
        "median",
        "mad",
        "stddev",
        "variance",
        "cv",
    ];

    // Only add additional columns if --addl-cols flag is set
    if !args.flag_addl_cols {
        return Vec::new();
    }

    // Standard columns that should never be included as additional columns
    let std_cols: std::collections::HashSet<&str> =
        ["field", "type", "cardinality", "nullcount", "min", "max"]
            .iter()
            .copied()
            .collect();

    let cols_to_include = if let Some(list_str) = &args.flag_addl_cols_list {
        // Parse comma-separated list
        if list_str.trim().to_lowercase().starts_with("everything")
            || list_str.trim().to_lowercase().starts_with("moar")
        {
            // note that we use starts_with("everything") to match "everything" and "everything!"
            // the same is true for "moar" and "moar!"
            // Include all available columns except standard ones, preserving CSV order
            // IndexSet preserves insertion order, so we can iterate directly
            avail_cols
                .iter()
                .filter(|col| !std_cols.contains(col.as_str()))
                .cloned()
                .collect::<Vec<String>>()
        } else {
            // Parse comma-separated list
            list_str
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        }
    } else {
        // Use default list when --addl-cols is set but --addl-cols-list is not provided
        DEFAULT_COLUMNS
            .iter()
            .map(std::string::ToString::to_string)
            .collect()
    };

    // Filter to only include columns that exist in avail_cols and are not std cols,
    // preserving user-specified order for custom lists, CSV order for "everything"
    cols_to_include
        .into_iter()
        .filter(|col| avail_cols.contains(col) && !std_cols.contains(col.as_str()))
        .collect()
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let start_time = Instant::now();
    let mut args: Args = util::get_args(USAGE, argv)?;

    // Initialize Redis default connection string to localhost, using database 3 by default
    // when --redis-cache is enabled
    // describegpt uses db 3 by default, fetch uses db 1, and fetchpost uses db 2
    DEFAULT_REDIS_CONN_STRING
        .set("redis://127.0.0.1:6379/3".to_string())
        .unwrap();

    // Initialize the global quiet flag
    QUIET_FLAG.store(args.flag_quiet, std::sync::atomic::Ordering::Relaxed);

    // If --export-prompt is set, export the default prompts and exit
    if let Some(file_path) = &args.flag_export_prompt {
        let default_prompts = get_default_prompt_file_content();

        // Ensure the file path has a .toml extension
        let output_path = Path::new(file_path);
        let output_path = if output_path.extension().and_then(|ext| ext.to_str()) == Some("toml") {
            output_path.to_path_buf()
        } else {
            output_path.with_extension("toml")
        };

        // Write the default prompts to the file
        fs::write(&output_path, default_prompts)?;

        print_status(
            &format!("Exported default prompts to: {}", output_path.display()),
            None,
        );

        return Ok(());
    }

    // If --format tsv is used, require --output option
    if get_output_format(&args)? == OutputFormat::Tsv && args.flag_output.is_none() {
        return fail_incorrectusage_clierror!(
            "--format tsv requires the --output option to be specified."
        );
    }

    // --prompt specific parameter validation
    if let Some(mut prompt) = args.flag_prompt.take() {
        // Check if prompt is a file path and read its contents if so
        if prompt.starts_with(util::FILE_PATH_PREFIX) {
            let prompt_file = prompt.strip_prefix(util::FILE_PATH_PREFIX).unwrap();
            prompt = fs::read_to_string(prompt_file)?;
        }
        args.flag_prompt = Some(prompt);

        // Now handle language auto-detection or explicit setting if necessary
        #[allow(unused)] // whatlang threshold is not used in qsvlite
        let (is_autodetect, threshold, explicit_language) =
            parse_language_option(args.flag_language.as_ref());

        if is_autodetect {
            #[cfg(feature = "whatlang")]
            {
                if let Some(prompt_text) = &args.flag_prompt {
                    if let Some(detected_lang) = detect_language_from_prompt(prompt_text, threshold)
                    {
                        args.flag_language = Some(detected_lang);
                        if log::log_enabled!(log::Level::Debug) {
                            log::debug!(
                                "Auto-detected language from prompt: {}",
                                args.flag_language.as_ref().unwrap()
                            );
                        }
                    } else {
                        // Detection failed or confidence below threshold, clear language to use
                        // model default
                        args.flag_language = None;
                        if log::log_enabled!(log::Level::Debug) {
                            log::debug!(
                                "Language detection failed or confidence below threshold \
                                 ({:.1}%), using model default",
                                threshold * 100.0
                            );
                        }
                    }
                }
            }
            #[cfg(not(feature = "whatlang"))]
            {
                // whatlang feature not available, clear language to use model default
                args.flag_language = None;
            }
        } else if let Some(explicit_lang) = explicit_language {
            // Explicit language specified, use it as-is
            args.flag_language = Some(explicit_lang);
        }

        // now validate sample size
        if args.flag_sample_size < 10 {
            return fail_incorrectusage_clierror!("--sample-size must be at least 10.");
        }
    }

    // Initialize cache variables unconditionally
    // even when --no-cache is set as we need to initialize the cache variables
    // to prevent panics in the #[io_cached] macros.
    let diskcache_dir = if let Some(dir) = &args.flag_disk_cache_dir {
        if dir.starts_with('~') {
            // expand the tilde
            let expanded_dir = util::expand_tilde(dir).unwrap();
            expanded_dir.to_string_lossy().to_string()
        } else {
            dir.to_string()
        }
    } else {
        // Default disk cache directory
        let default_dir = util::expand_tilde("~/.qsv/cache/describegpt").unwrap();
        default_dir.to_string_lossy().to_string()
    };

    // Initialize DiskCache Config unconditionally
    // safety: we're setting these OnceLocks unconditionally with guaranteed valid values
    DISKCACHE_DIR.set(diskcache_dir.clone()).unwrap();
    DISKCACHECONFIG.set(DiskCacheConfig::new()).unwrap();

    let cache_type = match (args.flag_no_cache, args.flag_redis_cache) {
        (false, false) => {
            // DISK CACHE
            // if --flush-cache is set, flush the cache directory
            if args.flag_flush_cache {
                if fs::metadata(&diskcache_dir).is_ok() {
                    if let Err(e) = fs::remove_dir_all(&diskcache_dir) {
                        return fail_clierror!(
                            r#"Cannot remove cache directory "{diskcache_dir}": {e:?}"#
                        );
                    }
                    print_status(
                        &format!("Flushed DiskCache directory: {diskcache_dir}"),
                        None,
                    );
                } else {
                    print_status(
                        &format!("Warning: DiskCache directory does not exist: {diskcache_dir}"),
                        None,
                    );
                }
                return Ok(());
            }

            // check if the cache directory exists, if it doesn't, create it
            if !diskcache_dir.is_empty()
                && let Err(e) = fs::create_dir_all(&diskcache_dir)
            {
                return fail_clierror!(r#"Cannot create cache directory "{diskcache_dir}": {e:?}"#);
            }

            // If --forget is set, remove cache entries and exit
            if args.flag_forget {
                // Determine which cache entries to remove
                let kinds_to_remove = determine_cache_kinds_to_remove(&args);

                // Get the model from prompt file for cache key generation
                let prompt_file = get_prompt_file(&args)?;

                // Remove cache entries for all specified kinds using the same key format as the
                // macro
                for kind in kinds_to_remove {
                    if kind == PromptType::Prompt {
                        // For prompt kind, we need to remove cache entries with any validity flag
                        // Get the base key without validity flag
                        let base_key = format!(
                            "{:?}{:?}{:?}{:?}{:?}{:?}{}{}{:?}",
                            args.arg_input,
                            args.flag_prompt_file,
                            args.flag_prompt,
                            args.flag_max_tokens,
                            args.flag_addl_props,
                            prompt_file.model,
                            kind,
                            FILE_HASH.get().unwrap_or(&String::new()),
                            args.flag_language
                        );

                        let removed = try_remove_prompt_cache_entries(&base_key);

                        if removed {
                            print_status(
                                &format!("Found and removed cache entry for {kind}"),
                                None,
                            );
                        } else {
                            print_status(
                                &format!("Warning: Cannot remove cache entry for {kind}"),
                                None,
                            );
                        }
                    } else {
                        // For other kinds, use the normal key format
                        let key = get_cache_key(&args, kind, &prompt_file.model);

                        if let Err(e) = GET_DISKCACHE_COMPLETION.cache_remove(&key) {
                            print_status(
                                &format!("Warning: Cannot remove cache entry for {kind}: {e:?}"),
                                None,
                            );
                        } else {
                            remove_cache_entry_by_key(
                                &key,
                                &args,
                                kind,
                                &format!("Found and removed cache entry for {kind}"),
                            );
                        }
                    }
                }
                return Ok(());
            }

            if args.flag_fresh {
                CacheType::Fresh
            } else {
                CacheType::Disk
            }
        },
        (false, true) => {
            // REDIS CACHE
            // initialize Redis Config
            REDISCONFIG.set(RedisConfig::new()).unwrap();

            // check if redis connection is valid
            let conn_str = &REDISCONFIG.get().unwrap().conn_str;
            let redis_client = match redis::Client::open(conn_str.to_string()) {
                Ok(rc) => rc,
                Err(e) => {
                    return fail_incorrectusage_clierror!(
                        r#"Invalid Redis connection string "{conn_str}": {e:?}"#
                    );
                },
            };

            let mut redis_conn;
            match redis_client.get_connection() {
                Err(e) => {
                    return fail_clierror!(r#"Cannot connect to Redis using "{conn_str}": {e:?}"#);
                },
                Ok(x) => redis_conn = x,
            }

            if args.flag_flush_cache {
                redis::cmd("FLUSHDB")
                    .exec(&mut redis_conn)
                    .map_err(|_| "Cannot flush Redis cache")?;
                print_status("Flushed Redis database.", None);
                return Ok(());
            }

            // If --forget is set, remove cache entries and exit
            if args.flag_forget {
                // Determine which cache entries to remove
                let kinds_to_remove = determine_cache_kinds_to_remove(&args);

                // Get the model from prompt file for cache key generation
                let prompt_file = get_prompt_file(&args)?;

                // Remove cache entries for all specified kinds
                for kind in kinds_to_remove {
                    let key = get_cache_key(&args, kind, &prompt_file.model);
                    remove_cache_entry_by_key(
                        &key,
                        &args,
                        kind,
                        &format!("Found and removed cache entry for {kind}"),
                    );
                }
                return Ok(());
            }

            if args.flag_fresh {
                CacheType::Fresh
            } else {
                CacheType::Redis
            }
        },
        (true, false) => CacheType::None,
        (true, true) => {
            // This case shouldn't be possible due to CLI arg validation,
            // but handle it gracefully just in case
            CacheType::None
        },
    };
    log::info!("Cache Type: {cache_type:?}");

    // Initialize tag vocabulary cache directory and CKAN settings if tag vocabulary is used
    #[cfg(feature = "feature_capable")]
    if args.flag_tag_vocab.is_some() {
        let qsv_cache_dir = lookup::set_qsv_cache_dir(&args.flag_cache_dir)?;
        TAG_VOCAB_CACHE_DIR.set(qsv_cache_dir)?;

        // Check the QSV_CKAN_API environment variable
        TAG_VOCAB_CKAN_API.set(if let Ok(api) = std::env::var("QSV_CKAN_API") {
            api
        } else {
            args.flag_ckan_api.clone()
        })?;

        // Check the QSV_CKAN_TOKEN environment variable
        TAG_VOCAB_CKAN_TOKEN
            .set(if let Ok(token) = std::env::var("QSV_CKAN_TOKEN") {
                Some(token)
            } else {
                args.flag_ckan_token.clone()
            })
            // safety: This OnceLock is being set unconditionally with a valid value before any
            // concurrent access, so unwrap is safe here.
            .unwrap();
    }

    // Priority: Explicit CLI flag > Env var > Default
    // Since --base-url has a docopt default, we check if the current value is the default
    // If it is, then the user didn't explicitly provide it, so env var should take precedence
    if args.flag_base_url.as_deref() == Some(DEFAULT_BASE_URL) {
        // Current value is default, check if env var is set
        if let Ok(base_url) = env::var("QSV_LLM_BASE_URL") {
            args.flag_base_url = Some(base_url);
        }
    }
    // else: value is not default, so user explicitly provided it - keep it

    // Priority: CLI flag > Env var > default/error
    let api_key: String = if args
        .flag_base_url
        .as_deref()
        .unwrap_or_default()
        .contains("localhost")
    {
        // Allow empty API key for localhost
        // Priority: CLI flag > Env var > empty
        args.flag_api_key
            .clone()
            .or_else(|| env::var("QSV_LLM_APIKEY").ok())
            .unwrap_or_default()
    } else {
        // Require API key for non-localhost
        // Priority: CLI flag > Env var > error
        if let Some(api_key) = &args.flag_api_key {
            // Allow "NONE" to suppress the API key
            if api_key.eq_ignore_ascii_case("NONE") {
                String::new()
            } else {
                api_key.clone()
            }
        } else if let Ok(val) = env::var("QSV_LLM_APIKEY") {
            val
        } else {
            return fail!(LLM_APIKEY_ERROR);
        }
    };

    // Check if num_tags is between 1 and 50
    if args.flag_num_tags < 1 || args.flag_num_tags > 50 {
        return fail_incorrectusage_clierror!("The --num-tags option must be between 1 and 50.");
    }

    // Check if addl-cols-list is set to "everything" or "everything!"
    if let Some(list_str) = &args.flag_addl_cols_list {
        // as a convenience, if addl-cols-list starts with "everything" or "moar"
        // set addl-cols to true
        let addl_cols_list = list_str.trim().to_lowercase();
        if addl_cols_list.starts_with("everything") || addl_cols_list.starts_with("moar") {
            args.flag_addl_cols = true;
        }

        // further, if addl-cols-list is set to "everything!" (exclamation point)
        // set stats-options to use --everything to force stats to compute "all" supported stats
        // we don't need to do this for "moar" as it will automatically compute all supported stats
        if list_str.trim().eq_ignore_ascii_case("everything!") {
            args.flag_stats_options =
                "--infer-dates --infer-boolean --everything --force --stats-jsonl".to_string();
        }
    }

    // Check if user gives arg_input
    if args.arg_input.is_none() {
        return fail_incorrectusage_clierror!("No input file specified.");
    }

    // Process input file
    // support stdin and auto-decompress snappy file
    // stdin/decompressed file is written to a temporary file in tmpdir
    // which is automatically deleted after the command finishes
    let tmpdir = tempfile::tempdir()?;
    let work_input = process_input(
        vec![PathBuf::from(
            // if no input file is specified, read from stdin "-"
            args.arg_input.as_deref().unwrap_or("-"),
        )],
        &tmpdir,
        "",
    )?;
    // safety: we just checked that there is at least one input file
    let input_path = work_input[0]
        .canonicalize()?
        .into_os_string()
        .into_string()
        // safety: canonicalize() ensures the path is valid
        .unwrap();

    // If no inference flags specified, print error message.
    if !args.flag_all
        && !args.flag_dictionary
        && !args.flag_description
        && !args.flag_tags
        && args.flag_prompt.is_none()
    {
        return fail_incorrectusage_clierror!("No inference options specified.");
    // If --all flag is specified, but other inference flags are also set, print error message.
    } else if args.flag_all
        && (args.flag_dictionary
            || args.flag_description
            || args.flag_tags
            || args.flag_prompt.is_some())
    {
        return fail_incorrectusage_clierror!(
            "--all option cannot be specified with other inference flags."
        );
    } else if args.flag_prompt.is_some()
        && (args.flag_dictionary || args.flag_description || args.flag_tags)
    {
        return fail_incorrectusage_clierror!(
            "--prompt cannot be specified together with --dictionary, --description, or --tags."
        );
    }

    // Calculate BLAKE3 hash of the input file early for cache key generation
    print_status(&format!("Calculating BLAKE3 hash of {input_path}..."), None);
    let start_hash_time = Instant::now();
    let file_hash = util::hash_blake3_file(Path::new(&input_path))?;
    FILE_HASH.set(file_hash.clone()).unwrap();
    print_status(
        &format!("(elapsed: {:.2?})", start_hash_time.elapsed()),
        None,
    );

    // Perform analysis
    print_status("Analyzing data...", None);
    let analysis_start = Instant::now();
    let analysis_results = if cache_type == CacheType::None {
        // No caching enabled, perform analysis directly
        perform_analysis(&args, &input_path)?
    } else {
        // Caching enabled, check cache
        print_status("  Checking analysis cache...", None);
        if let Some(results) = get_cached_analysis(&args, &cache_type, &file_hash, &input_path)? {
            // Cache hit, return cached results
            results
        } else {
            print_status("  Analysis cache miss. Performing data analysis...", None);
            let analysis_cachemiss_start = Instant::now();
            let results = perform_analysis(&args, &input_path)?;
            print_status("Analyzed data.", Some(analysis_cachemiss_start.elapsed()));
            results
        }
    };

    // get a random sample of the input file
    // only do this if --prompt is set
    let sample_file = tempfile::Builder::new()
        .prefix("qsv_sample_")
        .suffix(".csv")
        .tempfile()?;
    let sample_file_path = sample_file.path().display().to_string();
    if args.flag_prompt.is_some() {
        let sample_size = args.flag_sample_size.to_string();
        let sample_result = run_qsv_cmd(
            "sample",
            &[&sample_size, "--output", &sample_file_path],
            &input_path,
            &format!("Getting {sample_size} row sample data..."),
        );

        // If sample command fails, try slice as fallback
        if sample_result.is_err() {
            run_qsv_cmd(
                "slice",
                &["--len", &sample_size, "--output", &sample_file_path],
                &input_path,
                &format!("Getting {sample_size} row sample data (using slice)..."),
            )?;
        } else {
            sample_result?;
        }

        let _ = sample_file.keep();
        SAMPLE_FILE.set(sample_file_path)?;
    } else {
        SAMPLE_FILE.set(String::new())?;
    }

    print_status("Analyzed data.", Some(analysis_start.elapsed()));

    print_status("\nInteracting with LLM...", None);

    // Run inference options
    run_inference_options(&input_path, &args, &api_key, &cache_type, &analysis_results)?;

    // Print total elapsed time
    print_status("\ndescribegpt DONE!", Some(start_time.elapsed()));

    // if using a Diskcache, explicitly flush it to ensure entries are written to disk
    if cache_type == CacheType::Disk || (!args.flag_no_cache && cache_type == CacheType::Fresh) {
        GET_DISKCACHE_COMPLETION
            .connection()
            .flush()
            .map_err(|e| CliError::Other(format!("Error flushing DiskCache: {e}")))?;

        // Also flush the analysis cache
        GET_DISKCACHE_ANALYSIS
            .connection()
            .flush()
            .map_err(|e| CliError::Other(format!("Error flushing Analysis DiskCache: {e}")))?;
    }

    // cleanup the sample file
    if let Some(sample_file_path) = SAMPLE_FILE.get()
        && !sample_file_path.is_empty()
    {
        // ignore failure to remove the file
        let _ = fs::remove_file(sample_file_path);
    }

    Ok(())
}

// Perform the actual data analysis (stats, frequency, headers)
fn perform_analysis(args: &Args, input_path: &str) -> CliResult<AnalysisResults> {
    // check if the input file is indexed, if not, index it for performance
    let config = Config::new(Some(&input_path.to_string()));
    if config.index_files().is_err() {
        let _ = run_qsv_cmd("index", &[], input_path, "  Indexed")?;
    }

    // get the delimiter of the input file
    let delimiter = config.get_delimiter();

    // Decide if we want to use moarstats or stats
    let (stats, _) = match args
        .flag_addl_cols_list
        .as_deref()
        .map(|s| s.trim().to_lowercase())
    {
        Some(ref addl) if addl == "moar" || addl == "moar!" => {
            // Use moarstats when requested
            let stats_cmd: Vec<&str> = if addl == "moar!" {
                vec!["--advanced"] // also get gini coefficient, kurtosis, and shannon entropy
            } else {
                vec![]
            };
            print_status(
                &format!("  Compiling Summary Statistics (options: '{addl}')..."),
                None,
            );
            // moarstats writes output to <input>.stats.csv
            run_qsv_cmd("moarstats", &stats_cmd, input_path, " ")?;
            let stats_csv_path = Path::new(input_path).with_extension("stats.csv");
            let stats = fs::read_to_string(&stats_csv_path).map_err(|e| {
                CliError::Other(format!(
                    "Failed to read moarstats output file '{}': {e}",
                    stats_csv_path.display()
                ))
            })?;
            (stats, String::new())
        },
        _ => {
            // Use regular stats
            print_status(
                &format!(
                    "  Compiling Summary Statistics (options: '{}')...",
                    args.flag_stats_options
                ),
                None,
            );
            let stats_args_vec: Vec<&str> = args.flag_stats_options.split_whitespace().collect();
            run_qsv_cmd("stats", &stats_args_vec, input_path, " ")?
        },
    };

    // Build frequency command arguments with smart merging
    // If --freq-options contains --limit, use it as-is
    // Otherwise, prepend --limit from --enum-threshold for backward compatibility
    let frequency_args_vec: Vec<String> = args
        .flag_freq_options
        .split_whitespace()
        .map(std::string::ToString::to_string)
        .collect();

    let contains_limit = frequency_args_vec
        .iter()
        .any(|arg| arg == "--limit" || arg == "-l");

    let final_frequency_args: Vec<String> = if contains_limit {
        frequency_args_vec
    } else {
        // Prepend --limit <enum_threshold> if not present
        let mut combined = vec!["--limit".to_string(), args.flag_enum_threshold.to_string()];
        combined.extend(frequency_args_vec);
        combined
    };

    print_status(
        &format!(
            "  Compiling Frequency Distribution (options: '{}')...",
            final_frequency_args.join(" ")
        ),
        None,
    );

    let frequency_args_str: Vec<&str> = final_frequency_args
        .iter()
        .map(std::string::String::as_str)
        .collect();

    let (frequency, _) = run_qsv_cmd("frequency", &frequency_args_str, input_path, " ")?;

    // this is instantaneous, so no need to print start/end status
    let (headers, _) = run_qsv_cmd(
        "slice",
        &["--len", "1", "--no-headers"],
        input_path,
        "  Headers retrieved",
    )?;

    // Get the file hash that was already calculated
    let file_hash = FILE_HASH.get().unwrap_or(&String::new()).clone();

    Ok(AnalysisResults {
        stats,
        frequency,
        headers,
        file_hash,
        delimiter: delimiter as char,
    })
}

// Get cached analysis results
fn get_cached_analysis(
    args: &Args,
    cache_type: &CacheType,
    file_hash: &str,
    input_path: &str,
) -> CliResult<Option<AnalysisResults>> {
    match cache_type {
        CacheType::Disk => {
            let result = get_diskcache_analysis(args, file_hash, input_path)?;
            if result.was_cached {
                print_status("    Analysis disk cache hit!", None);
            }
            // Always return the result, whether it was cached or not,
            // since the cache function has already performed the analysis
            Ok(Some(result.value))
        },
        CacheType::Redis => {
            let result = get_redis_analysis(args, file_hash, input_path)?;
            if result.was_cached {
                print_status("    Analysis Redis cache hit!", None);
            }
            // Always return the result, whether it was cached or not,
            // since the cache function has already performed the analysis
            Ok(Some(result.value))
        },
        CacheType::Fresh => {
            // Always use cached analysis results, even with --fresh
            // as the file hash guarantees the cached analysis results are valid
            let result = if args.flag_redis_cache {
                get_redis_analysis(args, file_hash, input_path)?
            } else {
                get_diskcache_analysis(args, file_hash, input_path)?
            };
            if result.was_cached {
                print_status("    Analysis cache hit!", None);
            }
            Ok(Some(result.value))
        },
        CacheType::None => Ok(None),
    }
}
