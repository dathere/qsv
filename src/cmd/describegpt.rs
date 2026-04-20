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
QSV_DUCKDB_PATH environment variable is set to the absolute path of the DuckDB binary,
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
input file and using it as the primary cache key along with the prompt type, model and every flag that
influences the rendered prompt (including prompt-file, language, tag-vocab, num-tags, enum-threshold,
sample-size, fewshot-examples, the QSV_DUCKDB_PATH toggle and the generated Data Dictionary), so
changing any of them produces a fresh LLM call rather than stale cached output.

The default disk cache is stored in the ~/.qsv-cache/describegpt directory with a default TTL of 28 days
and cache hits NOT refreshing an existing cached value's TTL.
Adjust the QSV_DISKCACHE_TTL_SECS & QSV_DISKCACHE_TTL_REFRESH env vars to change disk cache settings.

Alternatively a Redis cache can be used instead of the disk cache. This is especially useful if you want
to share the cache across the network with other users or computers.
The Redis cache is stored in database 3 by default with a TTL of 28 days and cache hits NOT refreshing
an existing cached value's TTL. Adjust the QSV_DG_REDIS_CONNSTR, QSV_REDIS_MAX_POOL_SIZE,
QSV_REDIS_TTL_SECS & QSV_REDIS_TTL_REFRESH env vars to change Redis cache settings.

Examples:

  # Generate a Data Dictionary, Description & Tags of data.csv using default OpenAI gpt-oss-20b model
  # (replace <API_KEY> with your OpenAI API key)
  qsv describegpt data.csv --api-key <API_KEY> --all

  # Generate a Data Dictionary of data.csv using the DeepSeek R1:14b model on a local Ollama instance
  qsv describegpt data.csv -u http://localhost:11434/v1 --model deepseek-r1:14b --dictionary

  # Ask questions about the sample NYC 311 dataset using LM Studio with the default gpt-oss-20b model.
  # Questions that can be answered using the Summary Statistics & Frequency Distribution of the dataset.
  qsv describegpt NYC_311.csv --prompt "What is the most common complaint?"

  # Ask detailed natural language questions that require SQL queries and auto-invoke SQL RAG mode
  # Generate a DuckDB SQL query to answer the question
  QSV_DUCKDB_PATH=/path/to/duckdb \
  qsv describegpt NYC_311.csv -p "What's the breakdown of complaint types by borough descending order?"
  
  # Prompt requires a natural language query. Convert query to SQL using the LLM and save results to
  # a file with the --sql-results option.  If generated SQL query runs successfully,
  # the file is "results.csv". Otherwise, it is "results.sql".
  qsv describegpt NYC_311.csv -p "Aggregate complaint types by community board" --sql-results results

  # Cache Dictionary, Description & Tags inference results using the Redis cache instead of the disk cache
  qsv describegpt data.csv --all --redis-cache

  # Get fresh Description & Tags inference results from the LLM and refresh disk cache entries for both
  qsv describegpt data.csv --description --tags --fresh

  # Get fresh inference results from the LLM and refresh the Redis cache entries for all three
  qsv describegpt data.csv --all --redis-cache --fresh

  # Forget a cached response for data.csv's data dictionary if it exists and then exit
  qsv describegpt data.csv --dictionary --forget

  # Flush/Remove ALL cached entries in the disk cache
  qsv describegpt --flush-cache

  # Flush/Remove ALL cached entries in the Redis cache
  qsv describegpt --redis-cache --flush-cache

  # Generate Data Dictionary but exclude ID columns from frequency analysis to reduce overhead
  qsv describegpt data.csv --dictionary --freq-options "--select '!id,!uuid' --limit 20"

  # Generate Data Dictionary, Description & Tags but reduce frequency context
  # by showing only top 5 values per field
  qsv describegpt data.csv --all --freq-options "--limit 5"

  # Generate Description using weighted frequencies with ascending sort
  qsv describegpt data.csv --description --freq-options "--limit 50 --asc --weight count_column"

  # Generate a Data Dictionary, Description & Tags using a previously compiled stats CSV file and
  # frequency CSV file instead of running the stats and frequency commands
  qsv describegpt data.csv --all --stats-options "file:my_stats.csv" --freq-options "file:my_freq.csv"

For more examples, see https://github.com/dathere/qsv/blob/master/tests/test_describegpt.rs.

For more detailed info on how describegpt works and how to prepare a prompt file,
see https://github.com/dathere/qsv/blob/master/docs/Describegpt.md

Usage:
    qsv describegpt [options] [<input>]
    qsv describegpt --prepare-context [options] [<input>]
    qsv describegpt --process-response [options]
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
                           If it starts with "file:" prefix, the statistics are read from the
                           specified CSV file instead of running the stats command.
                           e.g. "file:my_custom_stats.csv"
                           [default: --infer-dates --infer-boolean --mad --quartiles --percentiles --force --stats-jsonl]
    --freq-options <arg>   Options for the frequency command used to generate frequency distributions.
                           You can use this to exclude certain variable types from frequency analysis
                           (e.g., --select '!id,!uuid'), limit results differently per use case, or
                           control output format. If --limit is specified here, it takes precedence
                           over --enum-threshold.
                           If it starts with "file:" prefix, the frequency data is read from the
                           specified CSV file instead of running the frequency command.
                           e.g. "file:my_custom_frequency.csv"
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
                           If the "polars" or the "QSV_DUCKDB_PATH" environment variable is set
                           & the `--sql-results` option is used, the SQL query will be automatically
                           executed and its results returned.
                           Otherwise, the SQL query will be returned along with the reasoning behind it.
                           If it starts with "file:" prefix, the prompt is read from the file specified.
                           e.g. "file:my_long_prompt.txt"
    --sql-results <file>   The file to save the SQL query results to.
                           Only valid if the --prompt option is used & the "polars" or the
                           "QSV_DUCKDB_PATH" environment variable is set.
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
    --no-score-sql         Disable scoresql validation of generated SQL queries before execution.
                           By default, when --prompt generates a SQL query and --sql-results is set,
                           the query is scored and iteratively improved if below threshold.
    --score-threshold <n>  Minimum scoresql score for a SQL query to be accepted.
                           Typical range is 0-100; values >100 will always trigger retries
                           and the below-threshold warning.
                           [default: 50]
    --score-max-retries <n>  Max LLM re-prompts to improve a low-scoring SQL query.
                           [default: 3]

                           LLM API OPTIONS:
    -u, --base-url <url>   The LLM API URL. Supports APIs & local LLMs compatible with
                           the OpenAI API specification. Some common base URLs:
                             OpenAI: https://api.openai.com/v1
                             Gemini: https://generativelanguage.googleapis.com/v1beta/openai
                             TogetherAI: https://api.together.ai/v1
                           Local LLMs:
                             Ollama: http://localhost:11434/v1
                             Jan: https://localhost:1337/v1
                             LM Studio: http://localhost:1234/v1
                           NOTE: If set, takes precedence over the QSV_LLM_BASE_URL environment variable
                           and the base URL specified in the prompt file.
                           [default: http://localhost:1234/v1]
    -m, --model <model>    The model to use for inferencing.
                           If set, takes precedence over the QSV_LLM_MODEL environment variable.
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
    -k, --api-key <key>    The API key to use. If set, takes precedence over the QSV_LLM_APIKEY envvar.
                           Required when the base URL is not localhost.
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
  --disk-cache-dir <dir>   The directory to store the disk cache. Note that if the directory does not exist,
                           it will be created. If the directory exists, it will be used as is, and will not
                           be flushed. This option allows you to maintain several disk caches for different
                           describegpt jobs (e.g. one for a data portal, another for internal data exchange).
                           [default: ~/.qsv-cache/describegpt]
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

                           MCP SAMPLING OPTIONS:
    --prepare-context      Output the prompt context as JSON to stdout without calling the LLM.
                           JSON includes system/user prompts, cache state, and analysis results
                           for each inference phase. Useful for inspecting prompts or piping to
                           custom LLM integrations. Used by the MCP server for sampling mode.
    --process-response     Process LLM responses provided as JSON via stdin. Takes the output
                           format from --prepare-context with LLM responses filled in, and
                           produces the final output (dictionary, description, tags, or prompt
                           results). Used by the MCP server for sampling mode.

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
    env, fs,
    io::{Read, Write},
    path::{Path, PathBuf},
    sync::OnceLock,
    time::{Duration, Instant},
};

use cached::{
    DiskCache, IOCached, RedisCache, Return, proc_macro::io_cached, stores::DiskCacheBuilder,
};
use foldhash::{HashMap, HashMapExt, HashSet};
use indexmap::IndexSet;
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

mod dictionary;
mod duckdb_sql;
mod formatters;
mod session;

use dictionary::{
    combine_dictionary_entries, generate_code_based_dictionary, parse_frequency_csv,
    parse_llm_dictionary_response, parse_stats_csv,
};
use duckdb_sql::{
    build_score_refinement_prompt, escape_sql_string, extract_sql_sample, handle_sql_error,
    run_duckdb_query, score_sql_query, should_use_duckdb,
};
use session::{
    SessionMessage, SessionState, apply_sliding_window, check_message_relevance, load_session,
    normalize_session_path, save_session, track_sql_error_in_session,
    update_session_after_sql_success,
};

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
    arg_input:              Option<String>,
    flag_dictionary:        bool,
    flag_description:       bool,
    flag_tags:              bool,
    flag_all:               bool,
    flag_num_tags:          u16,
    flag_tag_vocab:         Option<String>,
    #[allow(dead_code)]
    flag_cache_dir:         String,
    #[allow(dead_code)]
    flag_ckan_api:          String,
    #[allow(dead_code)]
    flag_ckan_token:        Option<String>,
    flag_stats_options:     String,
    flag_freq_options:      String,
    flag_enum_threshold:    usize,
    flag_num_examples:      u16,
    flag_truncate_str:      usize,
    flag_prompt:            Option<String>,
    flag_sql_results:       Option<String>,
    flag_prompt_file:       Option<String>,
    flag_sample_size:       u16,
    flag_fewshot_examples:  bool,
    flag_base_url:          Option<String>,
    flag_model:             Option<String>,
    flag_language:          Option<String>,
    flag_addl_props:        Option<String>,
    flag_api_key:           Option<String>,
    flag_max_tokens:        u32,
    flag_timeout:           u16,
    flag_user_agent:        Option<String>,
    flag_export_prompt:     Option<String>,
    flag_no_cache:          bool,
    flag_disk_cache_dir:    Option<String>,
    flag_redis_cache:       bool,
    flag_fresh:             bool,
    flag_forget:            bool,
    flag_flush_cache:       bool,
    flag_prepare_context:   bool,
    flag_process_response:  bool,
    flag_format:            Option<String>,
    flag_output:            Option<String>,
    flag_quiet:             bool,
    flag_addl_cols:         bool,
    flag_addl_cols_list:    Option<String>,
    flag_session:           Option<String>,
    flag_session_len:       usize,
    flag_no_score_sql:      bool,
    flag_score_threshold:   u32,
    flag_score_max_retries: u32,
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

/// JSON interchange format for --prepare-context output.
/// Contains all prompts and cache state for each inference phase.
#[derive(Debug, Serialize, Deserialize)]
struct PrepareContextOutput {
    phases:           Vec<PhaseContext>,
    analysis_results: AnalysisResults,
    model:            String,
    max_tokens:       u32,
}

/// Context for a single inference phase (dictionary, description, tags, or prompt).
#[derive(Debug, Serialize, Deserialize)]
struct PhaseContext {
    kind:            String,
    system_prompt:   String,
    user_prompt:     String,
    max_tokens:      u32,
    cache_key:       String,
    cached_response: Option<CompletionResponse>,
}

/// JSON interchange format for --process-response input (read from stdin).
/// Contains LLM responses for each phase that needed inference.
#[derive(Debug, Serialize, Deserialize)]
struct ProcessResponseInput {
    phases:           Vec<PhaseResponse>,
    analysis_results: AnalysisResults,
    model:            String,
}

/// Response for a single inference phase from the MCP server.
#[derive(Debug, Serialize, Deserialize)]
struct PhaseResponse {
    kind:        String,
    response:    String,
    reasoning:   String,
    token_usage: TokenUsage,
}

// environment variables
static QSV_REDIS_CONNSTR_ENV: &str = "QSV_DG_REDIS_CONNSTR";
static QSV_REDIS_MAX_POOL_SIZE_ENV: &str = "QSV_REDIS_MAX_POOL_SIZE";
static QSV_REDIS_TTL_SECS_ENV: &str = "QSV_REDIS_TTL_SECS";
static QSV_REDIS_TTL_REFRESH_ENV: &str = "QSV_REDIS_TTL_REFRESH";
static QSV_DUCKDB_PATH_ENV: &str = "QSV_DUCKDB_PATH";

// Shared regex for matching read_csv_auto function calls
static READ_CSV_AUTO_REGEX: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
    regex::Regex::new("read_csv_auto\\([^)]*\\)").expect("Invalid regex pattern")
});

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
static PROMPT_VALIDITY_FLAGS: std::sync::LazyLock<std::sync::Mutex<HashMap<String, String>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(HashMap::new()));

/// Detect language from prompt text using whatlang
/// Returns the detected language name if confidence >= threshold, otherwise None
/// Default threshold is 0.8 (80%)
#[cfg(feature = "whatlang")]
fn detect_language_from_prompt(prompt: &str, threshold: f64) -> Option<String> {
    let lang_info = detect(prompt)?;
    let detected_lang = lang_info.lang().eng_name();
    let lang_confidence = lang_info.confidence();

    // We only care about capturing the first detected language and confidence;
    // ignore errors if they were already set by a previous call.
    let _ = DETECTED_LANGUAGE.set(detected_lang.to_string());
    let _ = DETECTED_LANGUAGE_CONFIDENCE.set(lang_confidence);

    (lang_confidence >= threshold).then(|| detected_lang.to_string())
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
        prompt_file.tokens =
            if args.flag_max_tokens == 0 || prompt_file.base_url.contains("localhost") {
                0
            } else {
                args.flag_max_tokens
            };
        prompt_file.system_prompt = prompt_file
            .system_prompt
            .replace("{TOP_N}", &args.flag_enum_threshold.to_string());

        // Set the global prompt file
        PROMPT_FILE.set(prompt_file).unwrap();
        Ok(PROMPT_FILE.get().unwrap())
    }
}

/// Extract a single JSON value from an LLM response.
///
/// Strategy (in order):
/// 1. If a markdown code fence (```json … ``` or ``` … ```) is present, try to parse the content
///    inside it.
/// 2. Starting at the first `{` or `[` in the response, use `serde_json`'s streaming parser so
///    balanced braces/brackets are matched correctly even when the JSON contains `}` or `]` inside
///    string values. The streaming parser also allows trailing explanation text after the JSON.
/// 3. As a last resort, run `try_fix_json` (which escapes unescaped newlines / tabs / CRs inside
///    strings) on the same substring and re-parse.
fn extract_json_from_output(output: &str) -> CliResult<serde_json::Value> {
    fn parse_first_value(s: &str) -> Option<serde_json::Value> {
        serde_json::Deserializer::from_str(s)
            .into_iter::<serde_json::Value>()
            .next()
            .and_then(Result::ok)
    }

    /// Escape literal newlines / CRs / tabs that appear inside string values.
    /// Only runs when strict parsing fails, so already-valid JSON is untouched.
    fn try_fix_json(json_str: &str) -> String {
        let mut result = String::with_capacity(json_str.len());
        let mut in_string = false;
        let mut escape_next = false;

        for ch in json_str.chars() {
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
                '\n' if in_string => result.push_str("\\n"),
                '\r' if in_string => result.push_str("\\r"),
                '\t' if in_string => result.push_str("\\t"),
                _ => result.push(ch),
            }
        }
        result
    }

    fn attempt(candidate: &str) -> Option<serde_json::Value> {
        if let Some(v) = parse_first_value(candidate) {
            return Some(v);
        }
        parse_first_value(&try_fix_json(candidate))
    }

    // Accept ```json\n…\n``` or ```\n…\n```. The fence inner can contain any text.
    let fence_re = regex_oncelock!(r"(?s)```(?:json)?\s*\n(.*?)\n\s*```");
    if let Some(caps) = fence_re.captures(output)
        && let Some(m) = caps.get(1)
        && let Some(v) = attempt(m.as_str().trim())
    {
        return Ok(v);
    }

    // Scan every `{`/`[` in the response, not just the first — the LLM may
    // precede the actual JSON with markdown bullet points (`- {thing}`) or
    // other text that contains a bare `{`. Return the first position that
    // yields a valid parse.
    for (idx, ch) in output.char_indices() {
        if (ch == '{' || ch == '[')
            && let Some(v) = attempt(&output[idx..])
        {
            return Ok(v);
        }
    }

    fail_clierror!(
        "Failed to extract JSON content from LLM response. Output: {}",
        if output.is_empty() { "<empty>" } else { output }
    )
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

/// Format prompt as TSV - delegates to format_description_tsv (same format)
fn format_prompt_tsv(response: &str, reasoning: &str, token_usage: &TokenUsage) -> String {
    format_description_tsv(response, reasoning, token_usage)
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
    let json_add = match get_output_format(args)? {
        OutputFormat::Json => {
            " (in valid, pretty-printed JSON format, ensuring string values are properly escaped)"
        },
        OutputFormat::Toon => " (in TOON format)",
        _ => " (in Markdown format)",
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

    let max_tokens = (prompt_file.tokens > 0).then_some(prompt_file.tokens);

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
    // For prompt kind, include the currently-stored validity flag so an invalidated
    // prompt misses the cache. Other kinds are always "valid".
    let validity_flag = if kind == PromptType::Prompt {
        get_prompt_validity_flag(args, args.flag_prompt.as_ref())
    } else {
        "valid".to_string()
    };
    get_cache_key_with_flag(args, kind, actual_model, &validity_flag)
}

/// Per-process memo of `path_fingerprint` results, keyed by `"<path>:<mtime_nanos>:<size>"`.
/// `get_cache_key_with_flag` runs once per phase lookup (and again on invalidation paths), so
/// without memoization a ~100MB DuckDB binary would be re-hashed several times per run.
/// Keying on stat metadata as well as path means in-place edits (which bump mtime and/or size)
/// still produce a cache miss and fresh hash; we only short-circuit when the file is
/// demonstrably unchanged on disk. `stat` is ~microseconds while `hash_blake3_file` on a big
/// binary is ~tens of milliseconds, so the stat probe is essentially free.
static PATH_FINGERPRINT_CACHE: std::sync::OnceLock<std::sync::Mutex<HashMap<String, String>>> =
    std::sync::OnceLock::new();

/// Content fingerprint of a local file as a short BLAKE3 hex prefix, or empty string if
/// `path` is empty, a remote URL (`http://`, `https://`, `ckan://`, `dathere://` —
/// case-insensitive), or unreadable. Used to catch in-place edits of files inlined into the
/// rendered prompt (the tag-vocabulary CSV, the DuckDB binary). Content hashing is preferred
/// over `stat(mtime, size)` alone because same-second same-size rewrites (possible on
/// HFS+ / FAT / NFS mtime granularity) would otherwise collide; `hash_blake3_file` uses
/// mmap + rayon so even a ~100MB DuckDB binary hashes in tens of milliseconds. Results are
/// memoized per `(path, mtime, size)` tuple via `PATH_FINGERPRINT_CACHE`, so cache-key
/// rebuilds in the same process are free when the file is unchanged.
fn path_fingerprint(path: &str) -> String {
    if path.is_empty() {
        return String::new();
    }
    let lower = path.to_lowercase();
    if lower.starts_with("http://")
        || lower.starts_with("https://")
        || lower.starts_with("ckan://")
        || lower.starts_with("dathere://")
    {
        return String::new();
    }
    let meta = match fs::metadata(path) {
        Ok(m) => m,
        Err(_) => return String::new(),
    };
    let mtime_nanos = meta
        .modified()
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map_or(0, |d| d.as_nanos());
    let memo_key = format!("{path}:{mtime_nanos}:{}", meta.len());
    let cache = PATH_FINGERPRINT_CACHE.get_or_init(|| std::sync::Mutex::new(HashMap::new()));
    if let Some(cached) = cache.lock().unwrap().get(&memo_key) {
        return cached.clone();
    }
    let fp = match util::hash_blake3_file(Path::new(path)) {
        Ok(hex) => hex[..16].to_string(),
        Err(_) => String::new(),
    };
    cache.lock().unwrap().insert(memo_key, fp.clone());
    fp
}

/// Build a cache key with an explicit validity flag. Used by both `get_cache_key`
/// (which reads the current flag) and cache-invalidation paths (which need to
/// reconstruct keys for both "valid" and "invalid" flags to purge stored entries).
///
/// The key incorporates every input that affects the rendered prompt so the cache
/// never returns output that was produced under different flags. When adding a
/// new template-affecting flag, append it here and add a unit-test assertion.
fn get_cache_key_with_flag(
    args: &Args,
    kind: PromptType,
    actual_model: &str,
    validity_flag: &str,
) -> String {
    let file_hash = FILE_HASH.get().map_or("", String::as_str);
    // Only include prompt content in cache key for "prompt" kind
    let prompt_content = if kind == PromptType::Prompt {
        args.flag_prompt.as_ref()
    } else {
        None
    };
    // Short fingerprint of the generated data dictionary (when populated) — this JSON
    // is injected as the `dictionary` template var for description/tags kinds, so a
    // change in the dictionary must miss the cache.
    let dictionary_fingerprint = DATA_DICTIONARY_JSON
        .get()
        .map(|s| blake3::hash(s.as_bytes()).to_hex()[..16].to_string());
    let duckdb_enabled = should_use_duckdb();
    // The tag-vocab CSV contents are inlined into the Tags prompt; track local-file
    // edits by BLAKE3 content hash. Remote URLs (http://, https://, ckan://, dathere://)
    // return empty here and are fingerprinted by URL via `tag_vocab`.
    let tag_vocab_fp = args
        .flag_tag_vocab
        .as_deref()
        .map_or(String::new(), path_fingerprint);
    // When DuckDB is enabled, `get_prompt` queries the binary for version() and loaded
    // extensions and bakes them into the SQL guidance. Fingerprint the binary so a
    // binary swap / upgrade invalidates cached prompts. `should_use_duckdb()` is true
    // iff `QSV_DUCKDB_PATH` is set and non-empty, so the env var is guaranteed present
    // here — no PATH-resolved default path exists for describegpt. The env var *value*
    // is included in the key alongside the content fingerprint so different paths never
    // collide under a single cache slot even if hashing the binary fails (permission
    // error, binary removed between lookup and hash, etc.).
    let (duckdb_path, duckdb_binary_fp) = if duckdb_enabled {
        let p = std::env::var(QSV_DUCKDB_PATH_ENV).unwrap_or_default();
        let fp = path_fingerprint(&p);
        (p, fp)
    } else {
        (String::new(), String::new())
    };

    format!(
        "{file_hash};{prompt_file:?};{prompt_content:?};{max_tokens};{addl_props:?};\
         {actual_model};{kind};{validity_flag};{language:?};{tag_vocab:?};{tag_vocab_fp};\
         {num_tags};{enum_threshold};{sample_size};{fewshot_examples};{duckdb_enabled};\
         {duckdb_path};{duckdb_binary_fp};{dictionary_fingerprint:?}",
        prompt_file = args.flag_prompt_file,
        max_tokens = args.flag_max_tokens,
        addl_props = args.flag_addl_props,
        language = args.flag_language,
        tag_vocab = args.flag_tag_vocab,
        num_tags = args.flag_num_tags,
        enum_threshold = args.flag_enum_threshold,
        sample_size = args.flag_sample_size,
        fewshot_examples = args.flag_fewshot_examples,
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

/// Look up a cached completion response without triggering an API call.
/// Returns Some(CompletionResponse) on cache hit, None on cache miss.
fn lookup_cache(cache_type: &CacheType, cache_key: &str) -> Option<CompletionResponse> {
    let key = cache_key.to_string();
    match cache_type {
        CacheType::Disk | CacheType::Fresh => {
            GET_DISKCACHE_COMPLETION.cache_get(&key).ok().flatten()
        },
        CacheType::Redis => GET_REDIS_COMPLETION.cache_get(&key).ok().flatten(),
        CacheType::None => None,
    }
}

// Get output format (markdown is default)
fn get_output_format(args: &Args) -> CliResult<OutputFormat> {
    // Command-line flags take precedence over prompt file settings
    let format_str = if let Some(fmt) = &args.flag_format {
        fmt.clone()
    } else {
        get_prompt_file(args)?.format.clone()
    };

    match format_str.to_lowercase().as_str() {
        "markdown" | "md" => Ok(OutputFormat::Markdown),
        "tsv" => Ok(OutputFormat::Tsv),
        "json" => Ok(OutputFormat::Json),
        "toon" => Ok(OutputFormat::Toon),
        _ => fail_incorrectusage_clierror!(
            "Invalid format '{format_str}'. Must be one of: Markdown, TSV, JSON, TOON"
        ),
    }
}

// Generate TSV output file path for a given PromptKind
// Extracts filestem from base output path and appends .{kind}.tsv
fn get_tsv_output_path(base_output: &str, kind: PromptType) -> String {
    let path = Path::new(base_output);
    let filestem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(base_output);
    let kind_str = kind.to_string().to_lowercase();
    let filename = format!("{filestem}.{kind_str}.tsv");

    path.parent()
        .map(|p| p.join(&filename).to_string_lossy().to_string())
        .unwrap_or(filename)
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

/// Unescape literal escape sequences emitted by the LLM (\n, \t, \", \', \`) and
/// append trailing blank lines. Used before rendering an LLM response as Markdown.
fn unescape_llm_output_str(s: &str) -> String {
    s.replace("\\n", "\n")
        .replace("\\t", "\t")
        .replace("\\\"", "\"")
        .replace("\\'", "'")
        .replace("\\`", "`")
        + "\n\n"
}

/// Run the shared dictionary-entry build pipeline used by both dictionary output
/// paths: parse the stats + frequency CSVs, merge code-generated entries with any
/// LLM-provided labels / descriptions, and return the combined list.
fn build_combined_dictionary_entries(
    args: &Args,
    analysis_results: &AnalysisResults,
    completion_response: &CompletionResponse,
) -> CliResult<Vec<dictionary::DictionaryEntry>> {
    let (stats_records, ordered_col_names) = parse_stats_csv(&analysis_results.stats)?;
    let frequency_records = parse_frequency_csv(&analysis_results.frequency)?;
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
    Ok(combine_dictionary_entries(
        code_entries,
        &llm_labels_descriptions,
    ))
}

/// Dictionary phase when `--prompt` is active: still build the dictionary JSON and stash it
/// in `DATA_DICTIONARY_JSON` for later prompt rendering, but emit no output.
fn emit_dictionary_context_only(
    args: &Args,
    analysis_results: &AnalysisResults,
    completion_response: &CompletionResponse,
    model: &str,
    base_url: &str,
) -> CliResult<()> {
    let combined_entries =
        build_combined_dictionary_entries(args, analysis_results, completion_response)?;
    let mut dictionary_json = formatters::format_dictionary_json(
        &combined_entries,
        args.flag_enum_threshold,
        args.flag_num_examples,
        args.flag_truncate_str,
    );
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
    DATA_DICTIONARY_JSON.get_or_init(|| serde_json::to_string_pretty(&dictionary_json).unwrap());
    Ok(())
}

/// Full Dictionary phase output across JSON/TOON/TSV/Markdown formats.
#[allow(clippy::too_many_arguments)]
fn format_dictionary_phase(
    kind: PromptType,
    args: &Args,
    analysis_results: &AnalysisResults,
    completion_response: &CompletionResponse,
    total_json_output: &mut serde_json::Value,
    model: &str,
    base_url: &str,
    output_format: OutputFormat,
) -> CliResult<()> {
    let combined_entries =
        build_combined_dictionary_entries(args, analysis_results, completion_response)?;

    if output_format == OutputFormat::Json || output_format == OutputFormat::Toon {
        let mut dictionary_json = formatters::format_dictionary_json(
            &combined_entries,
            args.flag_enum_threshold,
            args.flag_num_examples,
            args.flag_truncate_str,
        );
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
        let mut tsv_output = formatters::format_dictionary_tsv(&combined_entries);
        tsv_output.push_str(&format_token_usage_comments(
            &completion_response.reasoning,
            &completion_response.token_usage,
        ));
        let dictionary_json = formatters::format_dictionary_json(
            &combined_entries,
            args.flag_enum_threshold,
            args.flag_num_examples,
            args.flag_truncate_str,
        );
        DATA_DICTIONARY_JSON
            .get_or_init(|| serde_json::to_string_pretty(&dictionary_json).unwrap());
        if let Some(output) = &args.flag_output {
            let tsv_path = get_tsv_output_path(output, kind);
            fs::write(&tsv_path, tsv_output.as_bytes())?;
        } else {
            print!("{tsv_output}");
        }
    } else {
        let mut markdown_output = formatters::format_dictionary_markdown(&combined_entries);
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
            kind, markdown_output, completion_response.reasoning, completion_response.token_usage
        );
        let dictionary_json = formatters::format_dictionary_json(
            &combined_entries,
            args.flag_enum_threshold,
            args.flag_num_examples,
            args.flag_truncate_str,
        );
        DATA_DICTIONARY_JSON
            .get_or_init(|| serde_json::to_string_pretty(&dictionary_json).unwrap());
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

/// Non-dictionary phase output to JSON (accumulates into `total_json_output`).
fn format_phase_json(
    kind: PromptType,
    args: &Args,
    completion_response: &CompletionResponse,
    total_json_output: &mut serde_json::Value,
) {
    total_json_output[kind.to_string()] =
        if kind == PromptType::Description || kind == PromptType::Prompt {
            json!({
                "response": completion_response.response,
                "reasoning": completion_response.reasoning,
                "token_usage": completion_response.token_usage,
            })
        } else {
            let mut output_value =
                if let Ok(json_value) = extract_json_from_output(&completion_response.response) {
                    json!({
                        "response": json_value,
                        "reasoning": completion_response.reasoning,
                        "token_usage": completion_response.token_usage,
                    })
                } else {
                    json!({
                        "response": completion_response.response,
                        "reasoning": completion_response.reasoning,
                        "token_usage": completion_response.token_usage,
                    })
                };
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
}

/// Non-dictionary phase output to TSV.
fn format_phase_tsv(
    kind: PromptType,
    args: &Args,
    completion_response: &CompletionResponse,
) -> CliResult<()> {
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
        format_description_tsv(
            &completion_response.response,
            &completion_response.reasoning,
            &completion_response.token_usage,
        )
    };
    if let Some(output) = &args.flag_output {
        let tsv_path = get_tsv_output_path(output, kind);
        fs::write(&tsv_path, tsv_output.as_bytes())?;
    } else {
        print!("{tsv_output}");
    }
    Ok(())
}

/// Non-dictionary phase output to TOON (accumulates into `total_json_output`;
/// TOON encoding happens later by the caller).
fn format_phase_toon(
    kind: PromptType,
    args: &Args,
    completion_response: &CompletionResponse,
    total_json_output: &mut serde_json::Value,
    model: &str,
    base_url: &str,
) {
    total_json_output[kind.to_string()] = if kind == PromptType::Description
        || kind == PromptType::Prompt
    {
        json!({
            "response": completion_response.response,
            "reasoning": completion_response.reasoning,
            "token_usage": completion_response.token_usage,
        })
    } else {
        let mut response_value = completion_response.response.clone();
        let mut attribution_value = serde_json::Value::Null;
        if kind == PromptType::Tags {
            if let Some(attr_start) = response_value.find("Generated by") {
                let attribution_text = response_value[attr_start..].trim().to_string();
                response_value = response_value[..attr_start].trim().to_string();
                attribution_value = json!(attribution_text);
            } else if response_value.contains("{GENERATED_BY_SIGNATURE}") {
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
        let mut output_value = if let Ok(json_value) = extract_json_from_output(&response_value) {
            json!({
                "response": json_value,
                "reasoning": completion_response.reasoning,
                "token_usage": completion_response.token_usage,
            })
        } else {
            json!({
                "response": response_value,
                "reasoning": completion_response.reasoning,
                "token_usage": completion_response.token_usage,
            })
        };
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
            if attribution_value != serde_json::Value::Null {
                obj.insert("attribution".to_string(), attribution_value);
            }
        }
        output_value
    };
}

/// Non-dictionary phase output to Markdown / plaintext.
/// Also handles the SQL-response fallthrough when `is_sql_response` is true for the Prompt kind.
/// `is_sql_response` can only be true when `kind == Prompt` — enforced via `debug_assert!`.
fn format_phase_markdown(
    kind: PromptType,
    args: &Args,
    completion_response: &CompletionResponse,
    is_sql_response: bool,
) -> CliResult<()> {
    debug_assert!(
        !is_sql_response || kind == PromptType::Prompt,
        "is_sql_response must only be set for PromptType::Prompt, got kind={kind:?}"
    );
    let mut formatted_output = unescape_llm_output_str(&completion_response.response);
    if kind == PromptType::Prompt && is_sql_response {
        formatted_output = {
            let input_path = args.arg_input.as_deref().unwrap_or("input.csv");
            if READ_CSV_AUTO_REGEX.is_match(&formatted_output) {
                let escaped_path = escape_sql_string(input_path);
                READ_CSV_AUTO_REGEX
                    .replace_all(
                        &formatted_output,
                        format!("read_csv_auto('{escaped_path}', strict_mode=false)"),
                    )
                    .into_owned()
            } else {
                formatted_output.replace(INPUT_TABLE_NAME, "_t_1")
            }
        };
    }
    formatted_output = format!(
        "# {}\n{}\n## REASONING\n\n{}\n## TOKEN USAGE\n\n{:?}\n---\n",
        kind, formatted_output, completion_response.reasoning, completion_response.token_usage
    );
    if let Some(output) = &args.flag_output {
        fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(output)?
            .write_all(formatted_output.as_bytes())?;
    } else {
        println!("{formatted_output}");
    }
    Ok(())
}

/// Process the output of a single inference phase.
/// Extracted from run_inference_options::process_output for reuse by --process-response.
#[allow(clippy::too_many_arguments)]
fn process_phase_output(
    kind: PromptType,
    completion_response: &CompletionResponse,
    total_json_output: &mut serde_json::Value,
    args: &Args,
    analysis_results: &AnalysisResults,
    model: &str,
    base_url: &str,
    output_format: OutputFormat,
) -> CliResult<()> {
    // Dictionary when --prompt is active: generate dictionary JSON for prompt context, no output.
    if kind == PromptType::Dictionary && args.flag_prompt.is_some() {
        return emit_dictionary_context_only(
            args,
            analysis_results,
            completion_response,
            model,
            base_url,
        );
    }

    if kind == PromptType::Dictionary {
        return format_dictionary_phase(
            kind,
            args,
            analysis_results,
            completion_response,
            total_json_output,
            model,
            base_url,
            output_format,
        );
    }

    let is_sql_response = kind == PromptType::Prompt
        && args.flag_sql_results.is_some()
        && completion_response.response.contains("```sql");

    // SQL responses (for any requested output format) always fall through to the markdown
    // helper, which renders the SQL block and handles the read_csv_auto / INPUT_TABLE_NAME
    // rewrite. Markdown requests also land there. The match is exhaustive on (OutputFormat,
    // is_sql_response) so adding a new OutputFormat variant is a compile error here,
    // forcing an explicit routing decision.
    match (output_format, is_sql_response) {
        (OutputFormat::Json, false) => {
            format_phase_json(kind, args, completion_response, total_json_output);
            Ok(())
        },
        (OutputFormat::Tsv, false) => format_phase_tsv(kind, args, completion_response),
        (OutputFormat::Toon, false) => {
            format_phase_toon(
                kind,
                args,
                completion_response,
                total_json_output,
                model,
                base_url,
            );
            Ok(())
        },
        (OutputFormat::Markdown, _)
        | (OutputFormat::Json, true)
        | (OutputFormat::Tsv, true)
        | (OutputFormat::Toon, true) => {
            format_phase_markdown(kind, args, completion_response, is_sql_response)
        },
    }
}

/// Build the messages JSON array sent to the LLM API for a single inference phase.
/// Incorporates the system prompt, optional data-dictionary context, optional session
/// state (for refinement / follow-up prompts), and the user prompt.
fn build_inference_messages(
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

        // Add recent messages (within sliding window)
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
                "User request: {prompt}\n\nPlease refine the baseline SQL query above to address \
                 this request. Return the complete refined SQL query that modifies the baseline \
                 query."
            );
            messages.push(json!({"role": "user", "content": refined_prompt}));
        } else {
            messages.push(json!({"role": "user", "content": prompt}));
        }
    } else {
        // No session, just add the prompt
        messages.push(json!({"role": "user", "content": prompt}));
    }

    serde_json::Value::Array(messages)
}

/// Run the Data Dictionary inference phase. Returns the completion so later phases
/// (Description / Tags / Prompt) can inline its response as context.
#[allow(clippy::too_many_arguments)]
fn run_dictionary_phase(
    args: &Args,
    client: &reqwest::blocking::Client,
    model: &str,
    api_key: &str,
    cache_type: &CacheType,
    analysis_results: &AnalysisResults,
    total_json_output: &mut serde_json::Value,
    base_url: &str,
    output_format: OutputFormat,
) -> CliResult<CompletionResponse> {
    let (prompt, system_prompt) = get_prompt(PromptType::Dictionary, Some(analysis_results), args)?;
    let start_time = Instant::now();
    print_status("  Inferring Data Dictionary...", None);
    let messages = build_inference_messages(&prompt, &system_prompt, "", None);

    // Special case: if --prompt is used with --fresh, force non-Fresh cache for the
    // dictionary so the prompt phase can reuse it without re-inferring.
    // Gate on caching actually being enabled — otherwise `--no-cache --prompt X --fresh`
    // would silently re-enable Disk/Redis caching just for the dictionary phase.
    let dictionary_cache_type =
        if args.flag_prompt.is_some() && args.flag_fresh && cache_type != &CacheType::None {
            if args.flag_redis_cache {
                &CacheType::Redis
            } else {
                &CacheType::Disk
            }
        } else {
            cache_type
        };

    let data_dict = get_cached_completion(
        args,
        client,
        model,
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
    process_phase_output(
        PromptType::Dictionary,
        &data_dict,
        total_json_output,
        args,
        analysis_results,
        model,
        base_url,
        output_format,
    )?;
    Ok(data_dict)
}

/// Run the Description inference phase.
#[allow(clippy::too_many_arguments)]
fn run_description_phase(
    args: &Args,
    client: &reqwest::blocking::Client,
    model: &str,
    api_key: &str,
    cache_type: &CacheType,
    analysis_results: &AnalysisResults,
    dictionary_response: &str,
    total_json_output: &mut serde_json::Value,
    base_url: &str,
    output_format: OutputFormat,
) -> CliResult<CompletionResponse> {
    let (prompt, system_prompt) =
        get_prompt(PromptType::Description, Some(analysis_results), args)?;
    let messages = build_inference_messages(&prompt, &system_prompt, dictionary_response, None);
    let start_time = Instant::now();
    print_status("  Inferring Description...", None);
    let completion_response = get_cached_completion(
        args,
        client,
        model,
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
    process_phase_output(
        PromptType::Description,
        &completion_response,
        total_json_output,
        args,
        analysis_results,
        model,
        base_url,
        output_format,
    )?;
    Ok(completion_response)
}

/// Run the Tags inference phase.
#[allow(clippy::too_many_arguments)]
fn run_tags_phase(
    args: &Args,
    client: &reqwest::blocking::Client,
    model: &str,
    api_key: &str,
    cache_type: &CacheType,
    analysis_results: &AnalysisResults,
    dictionary_context: &str,
    total_json_output: &mut serde_json::Value,
    base_url: &str,
    output_format: OutputFormat,
) -> CliResult<CompletionResponse> {
    let (prompt, system_prompt) = get_prompt(PromptType::Tags, Some(analysis_results), args)?;
    let messages = build_inference_messages(&prompt, &system_prompt, dictionary_context, None);
    let start_time = Instant::now();
    if let Some(ref tag_vocab_uri) = args.flag_tag_vocab {
        print_status(
            &format!("  Inferring Tags with Tag Vocabulary ({tag_vocab_uri})..."),
            None,
        );
    } else {
        print_status("  Inferring Tags...", None);
    }
    let completion_response = get_cached_completion(
        args,
        client,
        model,
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
    process_phase_output(
        PromptType::Tags,
        &completion_response,
        total_json_output,
        args,
        analysis_results,
        model,
        base_url,
        output_format,
    )?;
    Ok(completion_response)
}

/// Outcome of the custom-prompt phase, returned to `run_inference_options` so it can
/// apply the max-tokens check and optionally route into `execute_sql_query_phase`.
struct PromptPhaseOutcome {
    completion_response: CompletionResponse,
    has_sql_query:       bool,
    session_state:       Option<SessionState>,
    system_prompt:       String,
}

/// Run the Custom Prompt inference phase. Handles session loading, relevance check against
/// the baseline SQL, sliding-window trimming, LLM call, and appending the new user / assistant
/// turn to the in-memory session state (the session file is persisted by the caller).
/// Does NOT execute any embedded SQL query — that's `execute_sql_query_phase`.
#[allow(clippy::too_many_arguments)]
fn run_prompt_phase(
    args: &Args,
    user_prompt: &str,
    client: &reqwest::blocking::Client,
    model: &str,
    api_key: &str,
    cache_type: &CacheType,
    analysis_results: &AnalysisResults,
    dictionary_response: &str,
    normalized_session_path: Option<&str>,
    total_json_output: &mut serde_json::Value,
    base_url: &str,
    output_format: OutputFormat,
) -> CliResult<PromptPhaseOutcome> {
    let mut session_state: Option<SessionState> = None;

    // Handle session if --session is provided
    if let Some(normalized_path) = normalized_session_path {
        let session_path = Path::new(normalized_path);
        // `flag_session_len == 0` is a sentinel that collapses "user did not set it"
        // and "user explicitly set 0" into the default window size (10). Preserved
        // from the pre-refactor behavior; if we ever want to distinguish them, move
        // to `Option<usize>` at the arg level.
        let session_len = if args.flag_session_len == 0 {
            10
        } else {
            args.flag_session_len
        };

        session_state = Some(load_session(session_path)?);
        if let Some(ref mut state) = session_state
            && !state.messages.is_empty()
        {
            if let Some(ref baseline_sql) = state.baseline_sql
                && !check_message_relevance(user_prompt, baseline_sql, args, client, api_key)?
            {
                return fail_clierror!(
                    "The current message does not appear to be related to refining the baseline \
                     SQL query. Please start a new session for unrelated queries."
                );
            }
            apply_sliding_window(state, session_len, args, client, api_key)?;
        }
    }

    let (prompt, system_prompt) = get_prompt(PromptType::Prompt, Some(analysis_results), args)?;
    let start_time = Instant::now();
    print_status("  Answering Custom Prompt...", None);
    let messages = build_inference_messages(
        &prompt,
        &system_prompt,
        dictionary_response,
        session_state.as_ref(),
    );
    let completion_response = get_cached_completion(
        args,
        client,
        model,
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
    let has_sql_query = completion_response.response.contains("```sql");
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

    // Append the new user / assistant turn to the session (not saved yet — caller persists
    // after SQL execution to avoid overwriting multiple times per run).
    if let Some(ref mut state) = session_state {
        state.messages.push(SessionMessage {
            role:      "user".to_string(),
            content:   user_prompt.to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        });
        state.messages.push(SessionMessage {
            role:      "assistant".to_string(),
            content:   completion_response.response.clone(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        });
    }

    process_phase_output(
        PromptType::Prompt,
        &completion_response,
        total_json_output,
        args,
        analysis_results,
        model,
        base_url,
        output_format,
    )?;

    Ok(PromptPhaseOutcome {
        completion_response,
        has_sql_query,
        session_state,
        system_prompt,
    })
}

/// Execute the SQL query embedded in a Custom Prompt response:
/// 1. Validate the SQL-results file is writable.
/// 2. Extract the ```sql ... ``` fence from the LLM response.
/// 3. Optionally score the SQL via `scoresql` and refine via the LLM up to N times.
/// 4. Run the final query through DuckDB (when `QSV_DUCKDB_PATH` is set) or Polars (`sqlp`).
/// 5. Track the outcome (success / error rows) in the session state for future refinements.
#[allow(clippy::too_many_arguments)]
fn execute_sql_query_phase(
    input_path: &str,
    args: &Args,
    client: &reqwest::blocking::Client,
    model: &str,
    api_key: &str,
    cache_type: &CacheType,
    completion_response: &CompletionResponse,
    system_prompt: &str,
    sql_results: &str,
    session_state: &mut Option<SessionState>,
    normalized_session_path: Option<&str>,
) -> CliResult<()> {
    // Bind once so the error-path track_sql_error_in_session call sites below
    // pass a borrow instead of allocating a fresh String each time.
    let session_path_owned: Option<String> = normalized_session_path.map(String::from);

    // Check that the primary SQL-results path is writable, or can be created. Also probe
    // the `.csv` sibling the Polars path renames to (DuckDB writes directly to
    // `sql_results`, so the sibling may not exist there — read-only detection is
    // best-effort and only fires if the sibling already exists).
    let sql_results_path = Path::new(sql_results);
    let sql_results_csv_path = sql_results_path.with_extension("csv");
    for candidate in [sql_results_path, sql_results_csv_path.as_path()] {
        if candidate.exists() && fs::metadata(candidate)?.permissions().readonly() {
            return fail_clierror!(
                "SQL results file exists but is not writable: {}",
                candidate.display()
            );
        }
    }
    if !sql_results_path.exists() {
        match fs::File::create(sql_results_path) {
            Ok(_) => {
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
        // the dictionary
        if cache_type != &CacheType::None {
            let _ = invalidate_cache_entry(args, PromptType::Prompt);
        }
        return fail_clierror!("Failed to extract SQL query from custom prompt response");
    };

    // Score SQL before execution (enabled by default, disable with --no-score-sql)
    // When polars feature is disabled, scoresql only works with --duckdb
    #[cfg(feature = "polars")]
    let can_score = !args.flag_no_score_sql;
    #[cfg(not(feature = "polars"))]
    let can_score = !args.flag_no_score_sql && should_use_duckdb();

    if can_score {
        let use_duckdb = should_use_duckdb();
        let threshold = args.flag_score_threshold;
        let max_retries = args.flag_score_max_retries.min(100);
        if args.flag_score_max_retries > 100 {
            print_status(
                &format!(
                    "  Warning: --score-max-retries {} clamped to 100.",
                    args.flag_score_max_retries
                ),
                None,
            );
        }

        let file_stem = Path::new(input_path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("input");
        let mut scoring_sql = sql_query.replace(INPUT_TABLE_NAME, file_stem);
        let mut best_sql_template = sql_query.clone();
        let mut best_score: u32 = 0;

        // Targeted regex: only replace file_stem when it appears as a table name
        // (after FROM/JOIN/INTO/UPDATE), optionally quoted, to avoid corrupting column
        // names or literals that contain the file stem.
        // NOTE: INPUT_TABLE_NAME must not contain regex replacement-special chars
        // (e.g. `$`); the current value `{INPUT_TABLE_NAME}` is safe.
        let table_re = regex::Regex::new(&format!(
            r#"(?i)\b(FROM|JOIN|INTO|UPDATE)\s+["'`]?{}["'`]?(?:\b|$)"#,
            regex::escape(file_stem)
        ))
        .expect("Invalid table-name regex");

        for attempt in 1..=max_retries.saturating_add(1) {
            match score_sql_query(input_path, &scoring_sql, use_duckdb) {
                Ok((score, rating, report_json)) => {
                    print_status(
                        &format!("  SQL score: {score}/100 ({rating}) [attempt {attempt}]"),
                        None,
                    );

                    if score > best_score {
                        best_score = score;
                        best_sql_template = table_re
                            .replace_all(&scoring_sql, format!("${{1}} {INPUT_TABLE_NAME}"))
                            .to_string();
                    }

                    if score >= threshold || attempt > max_retries {
                        if score < threshold {
                            print_status(
                                &format!(
                                    "  Warning: Best SQL score {best_score}/100 below threshold \
                                     {threshold} after {max_retries} retries. Using best query."
                                ),
                                None,
                            );
                        }
                        // Restore {INPUT_TABLE_NAME} so the downstream replacement works
                        sql_query.clone_from(&best_sql_template);
                        break;
                    }

                    // Ask LLM to improve — use file_stem as the table name so the LLM
                    // returns SQL we can score directly.
                    let refinement_prompt = build_score_refinement_prompt(
                        &scoring_sql,
                        &report_json,
                        attempt,
                        max_retries,
                        file_stem,
                    );
                    let refinement_messages = json!([
                        {"role": "system", "content": system_prompt},
                        {"role": "user", "content": refinement_prompt}
                    ]);

                    match get_completion(
                        args,
                        client,
                        model,
                        api_key,
                        &refinement_messages,
                        PromptType::Prompt,
                    ) {
                        Ok(response) => {
                            if let Some(new_sql) = regex_oncelock!(r"(?s)```sql\s*\n(.*?)\n\s*```")
                                .captures(&response.response)
                                .and_then(|caps| caps.get(1).map(|m| m.as_str().trim().to_string()))
                            {
                                scoring_sql = new_sql;
                            } else {
                                print_status(
                                    "  LLM refinement had no SQL block. Using best query.",
                                    None,
                                );
                                sql_query.clone_from(&best_sql_template);
                                break;
                            }
                        },
                        Err(e) => {
                            log::warn!("SQL refinement LLM call failed: {e}");
                            sql_query.clone_from(&best_sql_template);
                            break;
                        },
                    }
                },
                Err(e) => {
                    // scoresql itself failed — SQL is likely invalid, counts as score=0
                    log::warn!("scoresql failed: {e}");
                    if attempt > max_retries {
                        print_status(
                            "  scoresql failed on all attempts. Proceeding with original query.",
                            None,
                        );
                        break;
                    }
                    // Feed the error to the LLM as feedback
                    let error_prompt = format!(
                        "The SQL query failed validation:\n```sql\n{scoring_sql}\n```\n\nError: \
                         {e}\n\nFix the SQL query. Use `{file_stem}` as the table name. Return \
                         ONLY the corrected SQL in a ```sql code block."
                    );
                    let error_messages = json!([
                        {"role": "system", "content": system_prompt},
                        {"role": "user", "content": error_prompt}
                    ]);
                    match get_completion(
                        args,
                        client,
                        model,
                        api_key,
                        &error_messages,
                        PromptType::Prompt,
                    ) {
                        Ok(response) => {
                            if let Some(new_sql) = regex_oncelock!(r"(?s)```sql\s*\n(.*?)\n\s*```")
                                .captures(&response.response)
                                .and_then(|caps| caps.get(1).map(|m| m.as_str().trim().to_string()))
                            {
                                scoring_sql = new_sql;
                            } else {
                                break;
                            }
                        },
                        Err(_) => break,
                    }
                },
            }
        }
    }

    if should_use_duckdb() {
        // For DuckDB, replace {INPUT_TABLE_NAME} with a read_csv_auto call
        // Escape single quotes in path to prevent SQL injection
        let escaped_path = escape_sql_string(input_path);
        if READ_CSV_AUTO_REGEX.is_match(&sql_query) {
            sql_query = READ_CSV_AUTO_REGEX
                .replace_all(
                    &sql_query,
                    format!("read_csv_auto('{escaped_path}', strict_mode=false)"),
                )
                .into_owned();
        } else {
            // Fallback: replace {INPUT_TABLE_NAME} directly
            sql_query = sql_query.replace(
                INPUT_TABLE_NAME,
                &format!("read_csv_auto('{escaped_path}', strict_mode=false)"),
            );
        }
        log::debug!("DuckDB SQL query:\n{sql_query}");

        let (_, stderr) =
            match run_duckdb_query(&sql_query, sql_results, "  DuckDB SQL query issued.") {
                Ok((stdout, stderr)) => {
                    if stderr.to_ascii_lowercase().contains(" error:") {
                        track_sql_error_in_session(
                            session_state.as_mut(),
                            session_path_owned.as_ref(),
                            format!("DuckDB SQL query execution failed: {stderr}"),
                        );
                        if cache_type != &CacheType::None {
                            let _ = invalidate_cache_entry(args, PromptType::Prompt);
                        }
                        return fail_clierror!("DuckDB SQL query execution failed: {stderr}");
                    }
                    (stdout, stderr)
                },
                Err(e) => {
                    track_sql_error_in_session(
                        session_state.as_mut(),
                        session_path_owned.as_ref(),
                        format!("DuckDB SQL query execution failed: {e}"),
                    );
                    if cache_type != &CacheType::None {
                        let _ = invalidate_cache_entry(args, PromptType::Prompt);
                    }
                    return Err(e);
                },
            };

        update_session_after_sql_success(session_state.as_mut(), sql_results, &sql_query);

        print_status(
            &format!("DuckDB SQL query successful. Saved results to {sql_results} {stderr}"),
            Some(sql_query_start.elapsed()),
        );
        return Ok(());
    }

    #[cfg(feature = "polars")]
    {
        sql_query = sql_query.replace(INPUT_TABLE_NAME, "_t_1");
        log::debug!("SQL query:\n{sql_query}");

        // Clone sql_query before moving it into fs::write, so we can use it later for
        // baseline SQL
        let sql_query_for_baseline = sql_query.clone();

        // Save SQL query to a temporary file with a .sql extension. tempfile is
        // deleted automatically when dropped.
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
                if stderr.to_ascii_lowercase().contains("error:") {
                    track_sql_error_in_session(
                        session_state.as_mut(),
                        session_path_owned.as_ref(),
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
                // Polars writes to sql_results, then we rename to *.csv
                let csv_path = sql_results_path.with_extension("csv");
                let _ = fs::rename(sql_results_path, &csv_path);

                // Track successful execution in session. We can't use
                // update_session_after_sql_success here because Polars renames the file.
                if let Some(state) = session_state.as_mut() {
                    if csv_path.exists()
                        && let Ok(sample) = extract_sql_sample(&csv_path)
                    {
                        state.sql_results = Some(sample);
                        state.sql_errors.clear();
                    }
                    // Store baseline SQL only after successful execution.
                    if state.baseline_sql.is_none() {
                        state.baseline_sql = Some(sql_query_for_baseline);
                    }
                }

                (stdout, stderr)
            },
            Err(e) => {
                track_sql_error_in_session(
                    session_state.as_mut(),
                    session_path_owned.as_ref(),
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
                session_path_owned.as_ref(),
                stderr.clone(),
            );
            return handle_sql_error(
                args,
                cache_type,
                sql_query_file.path(),
                sql_results_path,
                &stderr,
            );
        }
        print_status(
            &format!("Polars SQL query successful. Saved results to {sql_results} {stderr}"),
            Some(sql_query_start.elapsed()),
        );
        Ok(())
    }

    #[cfg(not(feature = "polars"))]
    {
        if cache_type != &CacheType::None {
            let _ = invalidate_cache_entry(args, PromptType::Prompt);
        }
        fail_clierror!(
            "Cannot answer the prompt using just Summary Statistics & Frequency Distribution \
             data. However, \"SQL RAG\" mode is only supported when the `polars` feature is \
             enabled, or when using DuckDB via the QSV_DUCKDB_PATH environment variable."
        )
    }
}

/// Emit the accumulated JSON / TOON output, if the configured `OutputFormat` produces one.
/// Markdown / TSV phases wrote to stdout or a file inline, so this is a no-op for those.
fn finalize_structured_output(
    args: &Args,
    total_json_output: &serde_json::Value,
    output_format: OutputFormat,
) -> CliResult<()> {
    match output_format {
        OutputFormat::Json => {
            let json_output = &simd_json::to_string_pretty(total_json_output)?;
            if let Some(output_file_path) = &args.flag_output {
                fs::write(output_file_path, json_output)?;
            } else {
                println!("{json_output}");
            }
        },
        OutputFormat::Toon => {
            let opts = EncodeOptions::new();
            let toon_output = encode(total_json_output, &opts)
                .map_err(|e| CliError::Other(format!("Failed to encode to TOON: {e}")))?;
            if let Some(output_file_path) = &args.flag_output {
                fs::write(output_file_path, toon_output)?;
            } else {
                println!("{toon_output}");
            }
        },
        OutputFormat::Markdown | OutputFormat::Tsv => {
            // Already written inline by per-phase helpers.
        },
    }
    Ok(())
}

/// Top-level orchestrator for all inference phases. Thin: runs `check_model`, dispatches
/// to the per-phase helpers in `Dictionary → Description → Tags → Prompt` order, applies
/// the max-tokens gate against the final phase's token usage, routes any SQL response
/// through `execute_sql_query_phase`, emits the accumulated JSON/TOON output, and persists
/// the session file.
fn run_inference_options(
    input_path: &str,
    args: &Args,
    api_key: &str,
    cache_type: &CacheType,
    analysis_results: &AnalysisResults,
) -> CliResult<()> {
    let llm_start = Instant::now();

    let client = util::create_reqwest_blocking_client(
        args.flag_user_agent.clone(),
        // unwrap_or 0 because 0 is a valid timeout per the usage text (local LLMs)
        util::timeout_secs(args.flag_timeout).unwrap_or(0) as u16,
        args.flag_base_url.clone(),
    )?;

    let model = check_model(&client, Some(api_key), args)?;
    let output_format = get_output_format(args)?;
    let base_url = get_prompt_file(args)?.base_url.clone();

    let mut total_json_output: serde_json::Value = json!({});
    let mut data_dict = CompletionResponse::default();
    let mut last_completion = CompletionResponse::default();
    let mut has_sql_query = false;
    let mut session_state: Option<SessionState> = None;
    let mut prompt_system_prompt = String::new();

    let normalized_session_path: Option<String> = args
        .flag_session
        .as_ref()
        .map(|p| normalize_session_path(p));

    if args.flag_dictionary || args.flag_all || args.flag_prompt.is_some() {
        data_dict = run_dictionary_phase(
            args,
            &client,
            &model,
            api_key,
            cache_type,
            analysis_results,
            &mut total_json_output,
            &base_url,
            output_format,
        )?;
        // Intentionally do NOT update `last_completion` here: the original
        // behavior is that the max-tokens gate only fires against a
        // description/tags/prompt completion, so a dictionary-only run with
        // high token usage is not treated as an error.
    }

    if args.flag_description || args.flag_all {
        last_completion = run_description_phase(
            args,
            &client,
            &model,
            api_key,
            cache_type,
            analysis_results,
            &data_dict.response,
            &mut total_json_output,
            &base_url,
            output_format,
        )?;
    }

    if args.flag_tags || args.flag_all {
        // Only include dictionary context if dictionary was actually generated
        let dictionary_context = if args.flag_dictionary || args.flag_all {
            data_dict.response.as_str()
        } else {
            ""
        };
        last_completion = run_tags_phase(
            args,
            &client,
            &model,
            api_key,
            cache_type,
            analysis_results,
            dictionary_context,
            &mut total_json_output,
            &base_url,
            output_format,
        )?;
    }

    if let Some(ref user_prompt) = args.flag_prompt {
        let outcome = run_prompt_phase(
            args,
            user_prompt,
            &client,
            &model,
            api_key,
            cache_type,
            analysis_results,
            &data_dict.response,
            normalized_session_path.as_deref(),
            &mut total_json_output,
            &base_url,
            output_format,
        )?;
        last_completion = outcome.completion_response;
        has_sql_query = outcome.has_sql_query;
        session_state = outcome.session_state;
        prompt_system_prompt = outcome.system_prompt;
    }

    // If --max-tokens is set and the last phase's completion token usage exceeded it,
    // fail now. This matches the pre-refactor behavior of checking against whichever
    // phase ran last.
    if args.flag_max_tokens > 0
        && last_completion.token_usage.completion >= args.flag_max_tokens as u64
    {
        return fail_clierror!(
            "Completion token usage is greater than or equal to --max-tokens ({}): {}",
            args.flag_max_tokens,
            last_completion.token_usage.completion,
        );
    }

    print_status("LLM inference/s completed.", Some(llm_start.elapsed()));

    if let Some(output) = &args.flag_output {
        // TSV mode doesn't write a single file at the --output path; per-phase helpers
        // derive per-kind siblings via `get_tsv_output_path` (e.g. `{filestem}.tags.tsv`).
        // Surface that in the status so users don't look for `output` verbatim.
        let message = if output_format == OutputFormat::Tsv {
            format!(
                "TSV output written using {output} as the base path (one derived file per phase, \
                 e.g. {{filestem}}.{{kind}}.tsv)"
            )
        } else {
            format!("Output written to {output}")
        };
        print_status(&message, None);
    }

    if let Some(sql_results) = &args.flag_sql_results
        && has_sql_query
    {
        execute_sql_query_phase(
            input_path,
            args,
            &client,
            &model,
            api_key,
            cache_type,
            &last_completion,
            &prompt_system_prompt,
            sql_results,
            &mut session_state,
            normalized_session_path.as_deref(),
        )?;
    }

    finalize_structured_output(args, &total_json_output, output_format)?;

    if let Some(ref state) = session_state
        && let Some(ref normalized_path) = normalized_session_path
    {
        save_session(Path::new(normalized_path), state)?;
    }

    Ok(())
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

/// Remove a cache entry by key from whichever backend is active.
///
/// Backend selection mirrors `run`'s match on `(flag_no_cache, flag_redis_cache)`:
/// `--redis-cache` wins over the default disk cache, and `--no-cache` is a
/// no-op. Previously this always took the disk path when `--no-cache` was
/// false, so `--forget` / SQL-failure invalidation silently did nothing on
/// Redis installations.
fn remove_cache_entry_by_key(key: &str, args: &Args, kind: PromptType, success_msg: &str) {
    if args.flag_redis_cache {
        let conn_str = &REDISCONFIG.get().unwrap().conn_str;
        match redis::Client::open(conn_str.to_string()) {
            Err(e) => print_status(
                &format!(
                    "Warning: Cannot open Redis client for removing cache entry for {kind}: {e:?}"
                ),
                None,
            ),
            Ok(redis_client) => match redis_client.get_connection() {
                Err(e) => print_status(
                    &format!(
                        "Warning: Cannot connect to Redis for removing cache entry for {kind}: \
                         {e:?}"
                    ),
                    None,
                ),
                Ok(mut redis_conn) => match redis::cmd("DEL").arg(key).exec(&mut redis_conn) {
                    Ok(()) => print_status(success_msg, None),
                    Err(e) => print_status(
                        &format!("Warning: Cannot remove cache entry for {kind}: {e:?}"),
                        None,
                    ),
                },
            },
        }
    } else if !args.flag_no_cache {
        let key_string = key.to_string();
        if let Err(e) = GET_DISKCACHE_COMPLETION.cache_remove(&key_string) {
            print_status(
                &format!("Warning: Cannot remove cache entry for {kind}: {e:?}"),
                None,
            );
        } else {
            print_status(success_msg, None);
            if let Err(e) = GET_DISKCACHE_COMPLETION.connection().flush() {
                print_status(&format!("Warning: Cannot flush disk cache: {e:?}"), None);
            } else if success_msg.contains("removed") {
                print_status("Flushed disk cache after removing cache entry", None);
            }
        }
    }
}

// Helper function to invalidate a specific cache entry by modifying the cache key
fn invalidate_cache_entry(args: &Args, kind: PromptType) -> CliResult<()> {
    let prompt_file = get_prompt_file(args)?;

    if kind == PromptType::Prompt {
        // Invalidate the validity flag so future cache lookups miss.
        invalidate_prompt_validity_flag(args, args.flag_prompt.as_ref());

        // The stored key encodes whichever flag was active at write time, so purge
        // both "valid" and "invalid" variants via the same helper used to build keys.
        remove_prompt_cache_entries_for_both_flags(
            args,
            kind,
            &prompt_file.model,
            &format!("Removed cache entry for {kind} due to SQL execution failure"),
        );
    } else {
        // For other kinds, remove the cache entry directly
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

/// Remove `PromptType::Prompt` cache entries for both "valid" and "invalid" validity
/// flags, since the stored key is whichever flag was current at the time it was cached.
fn remove_prompt_cache_entries_for_both_flags(
    args: &Args,
    kind: PromptType,
    actual_model: &str,
    success_msg: &str,
) {
    for flag in ["valid", "invalid"] {
        let key = get_cache_key_with_flag(args, kind, actual_model, flag);
        remove_cache_entry_by_key(&key, args, kind, success_msg);
    }
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
    let std_cols: HashSet<&str> = ["field", "type", "cardinality", "nullcount", "min", "max"]
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
            util::expand_tilde(dir)
                .ok_or_else(|| {
                    CliError::Other(format!(
                        "Cannot expand tilde in --disk-cache-dir '{dir}': HOME is not set",
                    ))
                })?
                .to_string_lossy()
                .to_string()
        } else {
            dir.to_string()
        }
    } else {
        util::expand_tilde("~/.qsv-cache/describegpt")
            .ok_or_else(|| {
                CliError::Other(
                    "Cannot resolve default disk cache directory: HOME is not set".to_string(),
                )
            })?
            .to_string_lossy()
            .to_string()
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
                        // For prompt kind, the stored key depends on which validity
                        // flag was active when cached, so remove both variants.
                        remove_prompt_cache_entries_for_both_flags(
                            &args,
                            kind,
                            &prompt_file.model,
                            &format!("Found and removed cache entry for {kind}"),
                        );
                    } else {
                        // For other kinds, use the normal key format
                        let key = get_cache_key(&args, kind, &prompt_file.model);
                        remove_cache_entry_by_key(
                            &key,
                            &args,
                            kind,
                            &format!("Found and removed cache entry for {kind}"),
                        );
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

            let mut redis_conn = match redis_client.get_connection() {
                Err(e) => {
                    return fail_clierror!(r#"Cannot connect to Redis using "{conn_str}": {e:?}"#);
                },
                Ok(x) => x,
            };

            if args.flag_flush_cache {
                redis::cmd("FLUSHDB")
                    .exec(&mut redis_conn)
                    .map_err(|_| "Cannot flush Redis cache")?;
                print_status("Flushed Redis database.", None);
                return Ok(());
            }

            // If --forget is set, remove cache entries and exit
            if args.flag_forget {
                let kinds_to_remove = determine_cache_kinds_to_remove(&args);
                let prompt_file = get_prompt_file(&args)?;

                for kind in kinds_to_remove {
                    if kind == PromptType::Prompt {
                        // PromptType::Prompt keys include the validity flag, so purge
                        // both "valid" and "invalid" variants — same as the disk path.
                        remove_prompt_cache_entries_for_both_flags(
                            &args,
                            kind,
                            &prompt_file.model,
                            &format!("Found and removed cache entry for {kind}"),
                        );
                    } else {
                        let key = get_cache_key(&args, kind, &prompt_file.model);
                        remove_cache_entry_by_key(
                            &key,
                            &args,
                            kind,
                            &format!("Found and removed cache entry for {kind}"),
                        );
                    }
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

    // Validate MCP sampling flags
    if args.flag_prepare_context && args.flag_process_response {
        return fail_incorrectusage_clierror!(
            "--prepare-context and --process-response are mutually exclusive."
        );
    }

    // --process-response mode: read LLM responses from stdin, process and output results
    // This branch is early because it doesn't need an input file or analysis step.
    if args.flag_process_response {
        // 50 MB limit to prevent unbounded memory usage from malformed input
        const MAX_STDIN_SIZE: u64 = 50 * 1024 * 1024;
        let stdin_data = {
            let mut buf = String::new();
            std::io::stdin()
                .take(MAX_STDIN_SIZE)
                .read_to_string(&mut buf)?;
            buf
        };
        if stdin_data.len() as u64 >= MAX_STDIN_SIZE {
            return fail_clierror!(
                "--process-response input exceeds {MAX_STDIN_SIZE} byte limit. Provide smaller \
                 JSON input on stdin."
            );
        }
        let input: ProcessResponseInput = serde_json::from_str(&stdin_data).map_err(|e| {
            CliError::Other(format!(
                "Failed to parse --process-response JSON from stdin: {e}"
            ))
        })?;

        let prompt_file = get_prompt_file(&args)?;
        let base_url = if prompt_file.base_url.is_empty() {
            "MCP sampling (no direct API call)".to_string()
        } else {
            prompt_file.base_url.clone()
        };
        let model = &input.model;
        let output_format = get_output_format(&args)?;

        let mut total_json_output: serde_json::Value = json!({});

        for phase in &input.phases {
            let kind: PromptType = phase
                .kind
                .parse()
                .map_err(|_| CliError::Other(format!("Unknown phase kind: {}", phase.kind)))?;

            let completion = CompletionResponse {
                response:    phase.response.clone(),
                reasoning:   phase.reasoning.clone(),
                token_usage: phase.token_usage.clone(),
            };

            // In --process-response mode, FILE_HASH is not initialized, so updating the cache
            // here would use an inconsistent cache key. We intentionally skip cache updates
            // in this mode; the MCP server can re-run the full flow when needed.

            // Process the output for this phase
            process_phase_output(
                kind,
                &completion,
                &mut total_json_output,
                &args,
                &input.analysis_results,
                model,
                &base_url,
                output_format,
            )?;
        }

        // Write accumulated JSON/TOON output
        if output_format == OutputFormat::Json {
            let json_str = serde_json::to_string_pretty(&total_json_output)?;
            if let Some(output) = &args.flag_output {
                fs::write(output, json_str.as_bytes())?;
            } else {
                println!("{json_str}");
            }
        } else if output_format == OutputFormat::Toon {
            let toon_str = encode(&total_json_output, &EncodeOptions::new())
                .map_err(|e| CliError::Other(format!("TOON encoding error: {e}")))?;
            if let Some(output) = &args.flag_output {
                fs::write(output, toon_str.as_bytes())?;
            } else {
                println!("{toon_str}");
            }
        }

        return Ok(());
    }

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

    // --prepare-context mode: output prompts and cache state as JSON, then exit
    if args.flag_prepare_context {
        let prompt_file = get_prompt_file(&args)?;
        let model = args
            .flag_model
            .clone()
            .unwrap_or_else(|| prompt_file.model.clone());
        let cache_key_model = &model;

        let mut phases = Vec::new();

        // Determine which phases to run
        let run_dictionary = args.flag_dictionary || args.flag_all || args.flag_prompt.is_some();
        let run_description = args.flag_description || args.flag_all;
        let run_tags = args.flag_tags || args.flag_all;
        let run_prompt = args.flag_prompt.is_some();

        if run_dictionary {
            let (user_prompt, system_prompt) =
                get_prompt(PromptType::Dictionary, Some(&analysis_results), &args)?;
            let cache_key = get_cache_key(&args, PromptType::Dictionary, cache_key_model);
            let cached = lookup_cache(&cache_type, &cache_key);
            phases.push(PhaseContext {
                kind: PromptType::Dictionary.to_string(),
                system_prompt,
                user_prompt,
                max_tokens: args.flag_max_tokens,
                cache_key,
                cached_response: cached,
            });
        }
        if run_description {
            let (user_prompt, system_prompt) =
                get_prompt(PromptType::Description, Some(&analysis_results), &args)?;
            let cache_key = get_cache_key(&args, PromptType::Description, cache_key_model);
            let cached = lookup_cache(&cache_type, &cache_key);
            phases.push(PhaseContext {
                kind: PromptType::Description.to_string(),
                system_prompt,
                user_prompt,
                max_tokens: args.flag_max_tokens,
                cache_key,
                cached_response: cached,
            });
        }
        if run_tags {
            let (user_prompt, system_prompt) =
                get_prompt(PromptType::Tags, Some(&analysis_results), &args)?;
            let cache_key = get_cache_key(&args, PromptType::Tags, cache_key_model);
            let cached = lookup_cache(&cache_type, &cache_key);
            phases.push(PhaseContext {
                kind: PromptType::Tags.to_string(),
                system_prompt,
                user_prompt,
                max_tokens: args.flag_max_tokens,
                cache_key,
                cached_response: cached,
            });
        }
        if run_prompt {
            let (user_prompt, system_prompt) =
                get_prompt(PromptType::Prompt, Some(&analysis_results), &args)?;
            let cache_key = get_cache_key(&args, PromptType::Prompt, cache_key_model);
            let cached = lookup_cache(&cache_type, &cache_key);
            phases.push(PhaseContext {
                kind: PromptType::Prompt.to_string(),
                system_prompt,
                user_prompt,
                max_tokens: args.flag_max_tokens,
                cache_key,
                cached_response: cached,
            });
        }

        let output = PrepareContextOutput {
            phases,
            analysis_results,
            model,
            max_tokens: args.flag_max_tokens,
        };

        let json_output = serde_json::to_string_pretty(&output)?;
        println!("{json_output}");
        return Ok(());
    }

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

    // Check if stats should be read from a file (file: prefix)
    let stats = if let Some(stats_file) = args.flag_stats_options.strip_prefix("file:") {
        let stats_path = Path::new(stats_file);
        print_status(
            &format!("  Reading Summary Statistics from file '{stats_file}'..."),
            None,
        );
        fs::read_to_string(stats_path).map_err(|e| {
            CliError::Other(format!(
                "Failed to read stats file '{}': {e}",
                stats_path.display()
            ))
        })?
    } else {
        // Decide if we want to use moarstats or stats
        let (stats_output, _) = match args
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
                let stats_args_vec: Vec<&str> =
                    args.flag_stats_options.split_whitespace().collect();
                run_qsv_cmd("stats", &stats_args_vec, input_path, " ")?
            },
        };
        stats_output
    };

    // Check if frequency should be read from a file (file: prefix)
    let frequency = if let Some(freq_file) = args.flag_freq_options.strip_prefix("file:") {
        let freq_path = Path::new(freq_file);
        print_status(
            &format!("  Reading Frequency Distribution from file '{freq_file}'..."),
            None,
        );
        fs::read_to_string(freq_path).map_err(|e| {
            CliError::Other(format!(
                "Failed to read frequency file '{}': {e}",
                freq_path.display()
            ))
        })?
    } else {
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

        let (freq_output, _) = run_qsv_cmd("frequency", &frequency_args_str, input_path, " ")?;
        freq_output
    };

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_json_handles_fenced_json() {
        let out = "Here is the result:\n```json\n{\"a\": 1, \"b\": \"x\"}\n```\nTrailing text.";
        let v = extract_json_from_output(out).unwrap();
        assert_eq!(v["a"], 1);
        assert_eq!(v["b"], "x");
    }

    #[test]
    fn extract_json_handles_unfenced_with_trailing_text() {
        let out = "{\"k\": 42}\n\nThe value is 42.";
        let v = extract_json_from_output(out).unwrap();
        assert_eq!(v["k"], 42);
    }

    #[test]
    fn extract_json_handles_braces_inside_strings() {
        // The old non-greedy regex would stop at the first `}`, truncating and
        // corrupting this. The streaming parser handles it correctly.
        let out = r#"Response: {"sql": "SELECT * FROM t WHERE x = '}'", "ok": true}"#;
        let v = extract_json_from_output(out).unwrap();
        assert_eq!(v["sql"], "SELECT * FROM t WHERE x = '}'");
        assert_eq!(v["ok"], true);
    }

    #[test]
    fn extract_json_handles_array() {
        let out = "The tags are:\n[\"one\", \"two\", \"three\"]";
        let v = extract_json_from_output(out).unwrap();
        assert_eq!(v.as_array().unwrap().len(), 3);
    }

    #[test]
    fn extract_json_repairs_unescaped_newlines_in_string() {
        // Actual newline between "line1" and "line2" inside a JSON string — invalid JSON,
        // but try_fix_json should escape it and allow parsing.
        let out = "{\"s\": \"line1\nline2\"}";
        let v = extract_json_from_output(out).unwrap();
        assert_eq!(v["s"], "line1\nline2");
    }

    #[test]
    fn extract_json_errors_on_non_json() {
        let err = extract_json_from_output("no json here").unwrap_err();
        assert!(format!("{err}").contains("Failed to extract JSON"));
    }

    #[test]
    fn extract_json_skips_earlier_non_json_brace() {
        // The LLM preceded the real JSON with a markdown-style mention like
        // `use {thing}` — a bare `{` that doesn't open valid JSON. The old code
        // gave up after failing at the first `{`; the fix scans subsequent
        // positions until one parses successfully.
        let out = "Per RFC {thing}, the answer is:\n{\"k\": 1}";
        let v = extract_json_from_output(out).unwrap();
        assert_eq!(v["k"], 1);
    }

    #[test]
    fn cache_key_round_trip_preserves_format() {
        // get_cache_key and get_cache_key_with_flag must produce matching keys
        // so that invalidation can reconstruct the stored key.
        let mut args = default_args_for_test();
        args.arg_input = Some("foo.csv".to_string());
        args.flag_prompt = Some("ask".to_string());
        args.flag_max_tokens = 1000;
        args.flag_language = Some("en".to_string());

        let key_via_get = get_cache_key(&args, PromptType::Prompt, "gpt-x");
        let key_via_flag = get_cache_key_with_flag(&args, PromptType::Prompt, "gpt-x", "valid");
        assert_eq!(key_via_get, key_via_flag);
    }

    #[test]
    fn cache_key_reflects_template_affecting_flags() {
        // Every flag that feeds into the rendered prompt must change the cache key,
        // otherwise tweaking the flag silently returns stale cached output.
        let args = default_args_for_test();
        let baseline = get_cache_key_with_flag(&args, PromptType::Tags, "gpt-x", "valid");

        let cases: Vec<(&str, Box<dyn Fn(&mut Args)>)> = vec![
            (
                "flag_tag_vocab",
                Box::new(|a| a.flag_tag_vocab = Some("vocab.csv".to_string())),
            ),
            ("flag_num_tags", Box::new(|a| a.flag_num_tags = 7)),
            (
                "flag_enum_threshold",
                Box::new(|a| a.flag_enum_threshold = 42),
            ),
            ("flag_sample_size", Box::new(|a| a.flag_sample_size = 99)),
            (
                "flag_fewshot_examples",
                Box::new(|a| a.flag_fewshot_examples = true),
            ),
        ];

        for (label, mutate) in cases {
            let mut mutated = default_args_for_test();
            mutate(&mut mutated);
            let key = get_cache_key_with_flag(&mutated, PromptType::Tags, "gpt-x", "valid");
            assert_ne!(key, baseline, "{label} did not change the cache key");
        }

        // Restoring defaults must reproduce the baseline key exactly.
        let restored = default_args_for_test();
        let restored_key = get_cache_key_with_flag(&restored, PromptType::Tags, "gpt-x", "valid");
        assert_eq!(restored_key, baseline);
    }

    #[test]
    fn cache_key_reflects_tag_vocab_file_contents() {
        // Editing a local tag-vocab CSV in place must change the cache key so the
        // cache doesn't return output generated against different vocabulary.
        // Uses tempfile for RAII cleanup — tmpfile drops even if the assert panics.
        use std::io::Write;
        let mut tmp = tempfile::Builder::new()
            .prefix("qsv_describegpt_vocab_")
            .suffix(".csv")
            .tempfile()
            .expect("create vocab tmpfile");
        writeln!(tmp, "tag,description\nfoo,first version").expect("write v1");
        tmp.flush().expect("flush v1");

        let mut args = default_args_for_test();
        args.flag_tag_vocab = Some(tmp.path().to_string_lossy().into_owned());
        let key_v1 = get_cache_key_with_flag(&args, PromptType::Tags, "gpt-x", "valid");

        // Rewrite in place with different bytes. Content hashing (not mtime) detects
        // this, so no mtime-tick sleep needed.
        let mut f = fs::File::create(tmp.path()).expect("rewrite vocab tmpfile");
        writeln!(f, "tag,description\nbar,different contents entirely").expect("write v2");
        f.flush().expect("flush v2");
        drop(f);

        let key_v2 = get_cache_key_with_flag(&args, PromptType::Tags, "gpt-x", "valid");
        assert_ne!(
            key_v1, key_v2,
            "in-place vocab edit must change the cache key"
        );
    }

    #[test]
    fn path_fingerprint_is_empty_for_remote_or_missing() {
        assert_eq!(path_fingerprint(""), "");
        assert_eq!(path_fingerprint("http://example.com/vocab.csv"), "");
        assert_eq!(path_fingerprint("https://example.com/vocab.csv"), "");
        assert_eq!(path_fingerprint("HTTPS://Example.com/vocab.csv"), "");
        assert_eq!(path_fingerprint("ckan://some-resource"), "");
        assert_eq!(path_fingerprint("CKAN://Some-Resource"), "");
        assert_eq!(path_fingerprint("dathere://some-dataset"), "");
        assert_eq!(path_fingerprint("Dathere://Some-Dataset"), "");
        assert_eq!(
            path_fingerprint("/path/that/definitely/does/not/exist.csv"),
            ""
        );
    }

    #[test]
    fn path_fingerprint_returns_hex_for_local_file() {
        // Positive-path assertion: a readable local file yields a 16-char hex fingerprint.
        // Guards against a regression that makes path_fingerprint always return empty,
        // which would silently neutralize the tag-vocab and DuckDB binary cache-key fields.
        use std::io::Write;
        let mut tmp = tempfile::NamedTempFile::new().expect("create tmpfile");
        writeln!(tmp, "payload").expect("write payload");
        tmp.flush().expect("flush");
        let fp = path_fingerprint(&tmp.path().to_string_lossy());
        assert_eq!(fp.len(), 16, "expected 16-char hex prefix, got {fp:?}");
        assert!(
            fp.chars().all(|c| c.is_ascii_hexdigit()),
            "expected hex chars, got {fp:?}"
        );
    }

    #[test]
    fn cache_key_reflects_dictionary_fingerprint() {
        // The generated Data Dictionary JSON is injected into description/tags prompts.
        // If the key doesn't track it, dictionary-aware and naked outputs would share
        // a cache slot. `OnceLock::set` can only succeed once per process, so we check
        // whichever state is live for this test-binary run and assert the key reflects it.
        let args = default_args_for_test();
        let key = get_cache_key_with_flag(&args, PromptType::Tags, "gpt-x", "valid");
        match DATA_DICTIONARY_JSON.get() {
            Some(dict) => {
                let expected_prefix = &blake3::hash(dict.as_bytes()).to_hex()[..16];
                assert!(
                    key.contains(expected_prefix),
                    "key {key:?} should contain fingerprint {expected_prefix:?}"
                );
            },
            None => {
                assert!(
                    key.ends_with(";None"),
                    "unset dictionary should serialize as None: {key}"
                );
            },
        }
    }

    /// Minimal Args for unit tests: zero-values everywhere.
    fn default_args_for_test() -> Args {
        Args {
            arg_input:              None,
            flag_dictionary:        false,
            flag_description:       false,
            flag_tags:              false,
            flag_all:               false,
            flag_num_tags:          0,
            flag_tag_vocab:         None,
            flag_cache_dir:         String::new(),
            flag_ckan_api:          String::new(),
            flag_ckan_token:        None,
            flag_stats_options:     String::new(),
            flag_freq_options:      String::new(),
            flag_enum_threshold:    0,
            flag_num_examples:      0,
            flag_truncate_str:      0,
            flag_prompt:            None,
            flag_sql_results:       None,
            flag_prompt_file:       None,
            flag_sample_size:       0,
            flag_fewshot_examples:  false,
            flag_base_url:          None,
            flag_model:             None,
            flag_language:          None,
            flag_addl_props:        None,
            flag_api_key:           None,
            flag_max_tokens:        0,
            flag_timeout:           0,
            flag_user_agent:        None,
            flag_export_prompt:     None,
            flag_no_cache:          true,
            flag_disk_cache_dir:    None,
            flag_redis_cache:       false,
            flag_fresh:             false,
            flag_forget:            false,
            flag_flush_cache:       false,
            flag_prepare_context:   false,
            flag_process_response:  false,
            flag_format:            None,
            flag_output:            None,
            flag_quiet:             false,
            flag_addl_cols:         false,
            flag_addl_cols_list:    None,
            flag_session:           None,
            flag_session_len:       0,
            flag_no_score_sql:      false,
            flag_score_threshold:   0,
            flag_score_max_retries: 0,
        }
    }
}
