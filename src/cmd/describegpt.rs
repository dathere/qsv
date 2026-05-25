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
infer-content-type, sample-size, fewshot-examples, the QSV_DUCKDB_PATH toggle and the generated Data
Dictionary), so changing any of them produces a fresh LLM call rather than stale cached output.

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

  # Generate a Data Dictionary that also infers a semantic Content Type for each field
  # (e.g. email, city, latitude) so the dictionary can later drive synthetic data generation
  qsv describegpt data.csv --dictionary --infer-content-type

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
    --infer-content-type   Also have the LLM classify each field's semantic "Content Type", mapped to a
                           curated, documented vocabulary (e.g. email, city, category, name, credit card, etc.)
                           see https://github.com/dathere/qsv/blob/master/src/cmd/synthesize/faker_map.rs.
                           Adds a "Content Type" column/field to the Data Dictionary output.
                           Fields where cardinality equals the row count (i.e. every row has a distinct
                           non-null value - primary keys, surrogate keys, sequence numbers) are
                           deterministically classified as "unique_id", overriding any token the LLM
                           returned for that field.
    --two-pass             Run a second LLM call that takes the full first-pass Data Dictionary
                           as JSON context and refines each field's Label, Description and
                           (when --infer-content-type is set) Content Type using cross-field
                           awareness. The LLM can then relate fields that belong together
                           (e.g. street_no + street_name + city + state + zip describing a single
                           mailing address; first_name + last_name naming a single person;
                           lat + lng forming a coordinate pair). The refined dictionary becomes the
                           emitted output and is also what downstream Description, Tags and Prompt
                           inference phases see as dictionary context.
                           Roughly doubles dictionary LLM cost and latency, so opt-in.
                           Most useful when combined with --infer-content-type.
                           Allowed with the --dictionary, --all and --prompt inference flags.
                           Mutually exclusive with --prepare-context and --process-response
                           (MCP sampling is single-turn per inference phase).
    --addl-cols            Add additional columns to the dictionary from the Summary Statistics.
  --addl-cols-list <list>  A comma-separated list of additional stats columns to add to the dictionary.
                           The columns must be present in the Summary Statistics.
                           If the columns are not present in the Summary Statistics or already in the
                           dictionary, they will be ignored.
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
                           A "file:"-backed CSV is assumed to use frequency's default "(NULL)"
                           null text; a custom --null-text in a file-supplied CSV is not
                           recognized when validating inferred date/datetime formats.
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
    --markdown-template <file>  TOML file with Mini Jinja templates for Markdown output. The TOML
                           contains four wrapper templates - one per inference kind:
                           dictionary_md_template, description_md_template, tags_md_template
                           and custom_prompt_md_template - plus a dictionary_md_body_template
                           that drives the per-field dictionary table that fills the
                           dictionary wrapper's {{ llm_response }}.
                           All template fields are optional; any omitted field falls back to
                           the embedded default, so a minimal TOML can override just the
                           templates you want to change.
                           Custom Mini Jinja filters (pipe_escape, br_replace, human_count,
                           dict_cell, humanize_examples) and template variables are documented
                           inline in the default TOML referenced below.
                           If no file is provided, built-in defaults are used (matching legacy output).
                           See https://github.com/dathere/qsv/blob/master/resources/describegpt_md_defaults.toml
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
                           Precedence: explicit CLI flag > QSV_LLM_BASE_URL env var > prompt file
                           base_url > built-in default (http://localhost:1234/v1).
                           NOTE: no docopt default — the absence of an explicit flag is what
                           lets the env var and the prompt file actually take effect.
    -m, --model <model>    The model to use for inferencing. This model must be compatible with OpenAI API spec.
                           Works with both cloud LLM providers and local LLMs.
                           Tested open weights models include OpenAI's gpt-oss-20b and gpt-oss-120b;
                           Google's Gemma family of open models; and Mistral's Magistral reasoning models.
                           Precedence: explicit CLI flag > QSV_LLM_MODEL env var > prompt file model
                           > built-in default (openai/gpt-oss-20b). No docopt default — same
                           rationale as --base-url above.
    --language <lang>      The output language/dialect/tone to use for the response. (e.g., "Spanish", "French",
                           "Hindi", "Mandarin", "Italian", "Castilian", "Franglais", "Taglish", "Pig Latin",
                           "Valley Girl", "Pirate", "Shakespearean English", "Chavacano", "Gen Z", "Yoda", etc.)
    
                             CHAT MODE (--prompt) LANGUAGE DETECTION BEHAVIOR:
                             When --prompt is used and --language is not set, automatically detects
                             the language of the prompt with an 80% confidence threshold using whatlang.
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
    --format <format>      Output format: Markdown, TSV, JSON, TOON, or JSONSchema.
                           TOON is a compact, human-readable encoding of the JSON data model for LLM prompts.
                           See https://toonformat.dev/ for more info.
                           JSONSchema emits the Data Dictionary as a JSON Schema (draft 2020-12)
                           document, enriched with LLM-inferred Label, Description and Content Type
                           (the latter only when the infer-content-type flag is set). qsv- and LLM-
                           specific metadata not modeled by the JSON Schema spec (cardinality,
                           null_count, weighted example counts, content_type, addl stats columns)
                           is preserved via a single x-qsv annotation object per property; unknown
                           keywords are ignored by validators per the 2020-12 spec.
                           The JSONSchema format requires the dictionary inference phase
                           (the dictionary or all flag). The description inference, when also run,
                           becomes the schema's top-level description; tags, when also run, are
                           embedded at x-qsv.tags. The prompt inference is not supported.
                           [default: Markdown]
    --allow-extra-cols     When the format is JSONSchema, emit additionalProperties as true at the
                           schema root (default is false, strict). Only meaningful with the
                           JSONSchema format; ignored otherwise.
    --strict-dates         When the format is JSONSchema, emit format date or date-time for
                           columns that stats infers as Date or DateTime. Off by default because
                           qsv's --infer-dates is permissive (accepts strings like
                           "June 27, 1968") and JSON Schema's date formats require RFC 3339, so
                           the validate roundtrip would otherwise fail. Set this only when your
                           source columns are guaranteed to be RFC 3339 full-date / date-time.
                           Mirrors the same flag on the schema command.
    -o, --output <file>    Write output to <file> instead of stdout. If --format is set to TSV,
                           separate files will be created for each prompt type with the pattern
                           {filestem}.{kind}.tsv (e.g., output.dictionary.tsv, output.tags.tsv).
    -q, --quiet            Do not print status messages to stderr.
"#;

use std::{
    env, fs,
    io::{Read, Write},
    path::{Path, PathBuf},
    sync::{LazyLock, OnceLock},
    time::{Duration, Instant},
};

use cached::{
    ConcurrentCached, DiskCache, DiskCacheBuilder, RedisCache, Return, macros::concurrent_cached,
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

pub(crate) mod dictionary;
mod duckdb_sql;
mod formatters;
mod session;

use dictionary::{
    combine_dictionary_entries, combine_dictionary_entries_with_baseline,
    generate_code_based_dictionary, parse_frequency_csv, parse_llm_dictionary_response,
    parse_stats_csv,
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
    DictionaryRefine,
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
    JsonSchema,
}
#[derive(Debug, Deserialize)]
struct Args {
    arg_input:               Option<String>,
    flag_dictionary:         bool,
    flag_description:        bool,
    flag_tags:               bool,
    flag_all:                bool,
    flag_num_tags:           u16,
    flag_tag_vocab:          Option<String>,
    #[allow(dead_code)]
    flag_cache_dir:          String,
    #[allow(dead_code)]
    flag_ckan_api:           String,
    #[allow(dead_code)]
    flag_ckan_token:         Option<String>,
    flag_stats_options:      String,
    flag_freq_options:       String,
    flag_enum_threshold:     usize,
    flag_num_examples:       u16,
    flag_truncate_str:       usize,
    flag_prompt:             Option<String>,
    flag_sql_results:        Option<String>,
    flag_prompt_file:        Option<String>,
    flag_markdown_template:  Option<String>,
    flag_sample_size:        u16,
    flag_fewshot_examples:   bool,
    flag_base_url:           Option<String>,
    flag_model:              Option<String>,
    flag_language:           Option<String>,
    flag_addl_props:         Option<String>,
    flag_api_key:            Option<String>,
    flag_max_tokens:         u32,
    flag_timeout:            u16,
    flag_user_agent:         Option<String>,
    flag_export_prompt:      Option<String>,
    flag_no_cache:           bool,
    flag_disk_cache_dir:     Option<String>,
    flag_redis_cache:        bool,
    flag_fresh:              bool,
    flag_forget:             bool,
    flag_flush_cache:        bool,
    flag_prepare_context:    bool,
    flag_process_response:   bool,
    flag_format:             Option<String>,
    flag_allow_extra_cols:   bool,
    flag_strict_dates:       bool,
    flag_output:             Option<String>,
    flag_quiet:              bool,
    flag_addl_cols:          bool,
    flag_addl_cols_list:     Option<String>,
    flag_infer_content_type: bool,
    flag_two_pass:           bool,
    flag_session:            Option<String>,
    flag_session_len:        usize,
    flag_no_score_sql:       bool,
    flag_score_threshold:    u32,
    flag_score_max_retries:  u32,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct PromptFile {
    name:                     String,
    description:              String,
    author:                   String,
    version:                  String,
    tokens:                   u32,
    system_prompt:            String,
    dictionary_prompt:        String,
    /// Refine-pass prompt template for `--two-pass`. Added in a later qsv version after
    /// existing user `--prompt-file` TOMLs were already in the wild — make it optional via
    /// `#[serde(default)]` so those TOMLs keep parsing. When the field is absent, the
    /// built-in default (mirroring `resources/describegpt_defaults.toml`) is used.
    #[serde(default = "default_dictionary_refine_prompt")]
    dictionary_refine_prompt: String,
    description_prompt:       String,
    tags_prompt:              String,
    prompt:                   String,
    format:                   String,
    language:                 String,
    base_url:                 String,
    model:                    String,
    timeout:                  u32,
    custom_prompt_guidance:   String,
    duckdb_sql_guidance:      String,
    polars_sql_guidance:      String,
    dd_fewshot_examples:      String, //DuckDB few-shot examples
    p_fewshot_examples:       String, //Polars SQL few-shot examples
}

#[derive(Debug, Deserialize)]
struct MarkdownTemplateFile {
    // Metadata fields are deserialized for round-tripping but never read by code.
    // (No `#[serde(default)]` needed here — the embedded default TOML always supplies them,
    //  and user overrides go through `MarkdownTemplateOverride` which IS optional.)
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    description: String,
    #[allow(dead_code)]
    author: String,
    #[allow(dead_code)]
    version: String,
    dictionary_md_body_template: String,
    dictionary_md_template: String,
    description_md_template: String,
    tags_md_template: String,
    custom_prompt_md_template: String,
}

/// User-supplied `--markdown-template` overrides. Every field is optional so a user can
/// drop in a TOML containing just the one template they want to change — the rest fall
/// back to the embedded defaults. Errors only surface for genuinely malformed TOML or
/// fields whose declared type doesn't match (e.g. a number where a string is expected).
#[derive(Debug, Default, Deserialize)]
struct MarkdownTemplateOverride {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    author: Option<String>,
    #[serde(default)]
    version: Option<String>,
    #[serde(default)]
    dictionary_md_body_template: Option<String>,
    #[serde(default)]
    dictionary_md_template: Option<String>,
    #[serde(default)]
    description_md_template: Option<String>,
    #[serde(default)]
    tags_md_template: Option<String>,
    #[serde(default)]
    custom_prompt_md_template: Option<String>,
}

impl MarkdownTemplateOverride {
    /// Per-field overlay: any `Some` field replaces the corresponding `base` field;
    /// `None` keeps the base (embedded default) value.
    fn apply_to(self, base: MarkdownTemplateFile) -> MarkdownTemplateFile {
        MarkdownTemplateFile {
            name: self.name.unwrap_or(base.name),
            description: self.description.unwrap_or(base.description),
            author: self.author.unwrap_or(base.author),
            version: self.version.unwrap_or(base.version),
            dictionary_md_body_template: self
                .dictionary_md_body_template
                .unwrap_or(base.dictionary_md_body_template),
            dictionary_md_template: self
                .dictionary_md_template
                .unwrap_or(base.dictionary_md_template),
            description_md_template: self
                .description_md_template
                .unwrap_or(base.description_md_template),
            tags_md_template: self.tags_md_template.unwrap_or(base.tags_md_template),
            custom_prompt_md_template: self
                .custom_prompt_md_template
                .unwrap_or(base.custom_prompt_md_template),
        }
    }
}

// The base_url / model values these constants represent are also baked
// into the bundled default prompt file (resources/describegpt_defaults.toml).
// At runtime, the effective URL/model is resolved by `get_prompt_file`
// with CLI > env > prompt_file precedence — and the prompt_file's own
// fields fall back to these constants via the bundled TOML when no
// custom --prompt-file is given. The constants are therefore not used
// as direct runtime fallbacks; they're kept here for tests that build
// an `Args` struct manually and need a sensible default to plug in.
//
// Crucially, neither --base-url nor --model carries a docopt
// `[default: ...]` — that would make flag_base_url/flag_model always
// Some and erase the "user passed nothing" signal `get_prompt_file`'s
// precedence depends on (codex review job 2363).
#[allow(dead_code)]
const DEFAULT_BASE_URL: &str = "http://localhost:1234/v1";
#[allow(dead_code)]
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

/// First-pass Data Dictionary JSON, used as `{{ first_pass_dictionary }}` context only when
/// rendering `PromptType::DictionaryRefine` during a `--two-pass` run. Cleared (set to `None`)
/// otherwise. `RwLock<Option<String>>` rather than `OnceLock` because the slot is
/// per-invocation: it gets populated before the refine prompt is rendered and reset on the
/// next first pass, so multiple in-process runs (and the test harness) need to overwrite it.
/// `DATA_DICTIONARY_JSON` (the `{{ dictionary }}` ctx var read by description / tags / prompt
/// phases) is populated separately, from the REFINED dictionary, only after the refine pass
/// succeeds — so downstream phases always see the better dictionary.
static FIRST_PASS_DICT_JSON: std::sync::RwLock<Option<String>> = std::sync::RwLock::new(None);

/// RAII guard that clears `FIRST_PASS_DICT_JSON` on drop. Built via `seed()` which sets the
/// slot and returns the guard; the slot is cleared when the guard goes out of scope, whether
/// the caller returns `Ok` or short-circuits with `?` on an error from the refine LLM call.
/// Without this guard, an error path between the manual seed and manual clear in
/// `run_dictionary_phase` would leave the slot populated for the rest of the process
/// lifetime — fine for a one-shot CLI run, but a leak in the documented test-harness /
/// repeated-invocation case where a subsequent first pass would briefly see stale data
/// before overwriting it.
struct FirstPassDictGuard;

impl FirstPassDictGuard {
    /// Seed `FIRST_PASS_DICT_JSON` with the given JSON string and return a guard that
    /// clears the slot on drop. Panics if the lock is poisoned, matching the write side
    /// in `get_prompt` — a poisoned lock means a panic already broke an invariant somewhere
    /// and silently continuing would mask the problem.
    fn seed(json: String) -> Self {
        let mut guard = FIRST_PASS_DICT_JSON
            .write()
            .expect("FIRST_PASS_DICT_JSON write-lock poisoned");
        *guard = Some(json);
        Self
    }
}

impl Drop for FirstPassDictGuard {
    fn drop(&mut self) {
        // Best-effort clear on drop. If the lock is poisoned (a panic broke an invariant
        // somewhere else), there's nothing useful to do from a Drop — panicking during
        // unwind would abort the process. The next `seed()` call (or the test-only direct
        // write) will overwrite the slot anyway.
        if let Ok(mut guard) = FIRST_PASS_DICT_JSON.write() {
            *guard = None;
        }
    }
}

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
// NOTE: PROMPT_FILE and MD_TEMPLATE_FILE are process-wide singletons whose content is derived
// from `args.flag_prompt_file` / `args.flag_markdown_template`. In the CLI process this is a
// non-issue (there is exactly one Args). In the test binary the FIRST test to call
// `get_prompt_file` / `get_md_template_file` pins the value for the rest of the binary, so any
// future test that exercises a CUSTOM template must run in its own process (e.g. via
// `#[test_with::process]` or by running with `--test-threads=1` *and* a separate test binary)
// or refactor the loaders to thread the parsed templates through call sites instead of caching.
static PROMPT_FILE: OnceLock<PromptFile> = OnceLock::new();
static MD_TEMPLATE_FILE: OnceLock<MarkdownTemplateFile> = OnceLock::new();
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
    // get_prompt_file already applied CLI > env > prompt_file precedence
    // (the prompt_file's own base_url falls back to the built-in default
    // via the bundled default prompt TOML when no --prompt-file is given),
    // so prompt_file.base_url IS the effective URL. Use it for both the
    // --prompt-file and no-flag cases — codex review job 2372 flagged the
    // previous branch where the no-flag case skipped the prompt-file
    // fallback.
    let base_url = prompt_file.base_url.clone();
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

/// Built-in fallback for the `dictionary_refine_prompt` PromptFile field. Used when a user
/// supplies a `--prompt-file` TOML that pre-dates `--two-pass` and therefore lacks the field.
/// Kept byte-identical to the `dictionary_refine_prompt` block in
/// `resources/describegpt_defaults.toml` (the active default for new users); the two MUST be
/// kept in sync — `tests::default_dictionary_refine_prompt_matches_resource` enforces it.
fn default_dictionary_refine_prompt() -> String {
    DEFAULT_DICTIONARY_REFINE_PROMPT.to_string()
}

/// String constant mirroring the `dictionary_refine_prompt` block from
/// `resources/describegpt_defaults.toml`. Defined here so `default_dictionary_refine_prompt`
/// (the `#[serde(default)]` fallback) doesn't need to parse the TOML at deserialize time.
// No leading newline: TOML's triple-quoted string spec trims the newline that
// immediately follows the opening `"""`, so a byte-identical const must also start
// directly with `R` (not with a `\n`). `default_dictionary_refine_prompt_matches_resource`
// catches any drift from the TOML block.
const DEFAULT_DICTIONARY_REFINE_PROMPT: &str = r#"Refine the previously-generated Data Dictionary using full cross-field context.

You are given the FIRST-PASS DATA DICTIONARY below, which already contains a Label{% if infer_content_type %}, Description and Content Type{% else %} and Description{% endif %} for every field. The first pass produced these field-by-field without seeing the dictionary as a coherent whole, so it may have missed obvious cross-field relationships.

Look at all fields together and identify groups that semantically belong to a single higher-level concept, for example:
- street number + street name + unit + city + state/region + postal code = a single MAILING ADDRESS
- first name + middle initial/name + last name + suffix = a single PERSON NAME
- latitude + longitude (and optionally altitude) = a GEOGRAPHIC COORDINATE
- start date + end date (or begin/end timestamps) = a DATE RANGE / DURATION
- date column + time column = a single TIMESTAMP split across two columns
- price/cost + currency code = a MONEY value
- quantity + unit-of-measure = a MEASUREMENT

Then re-emit Label{% if infer_content_type %}, Description and Content Type{% else %} and Description{% endif %} for EVERY field, not just the ones you change. For fields that belong to a composite concept, mention the relationship in the Description (e.g. "Street name component of the mailing address; combine with street_no, city, state and zip to form the full address.") and refine the Label to make the role explicit.

{% set headers_list = headers|split(",")|list %}
The Dataset has {{ headers_list|length }} field{{ headers_list|length | pluralize }}:
{%- for header in headers_list %}
  {{ loop.index }}. {{ header | trim }}
{%- endfor %}

FIRST-PASS DATA DICTIONARY (JSON):
{{ first_pass_dictionary }}

{% if infer_content_type %}For Content Type, the same rules from the first pass apply:
- Choose exactly ONE token from the fixed vocabulary (lowercased; the ":<fmt>" suffix on
  date/datetime tokens is case-sensitive chrono strftime syntax).
- The `duration:N` suffix form is allowed (seconds upper bound).
- "date"/"datetime" tokens carry a chrono strftime ":<fmt>" suffix matching the column's raw
  values (e.g. "datetime:%m/%d/%Y %I:%M:%S %p"). The token itself is re-derived deterministically
  by qsv from the Type column; you may correct or add the ":<fmt>" suffix using cross-field
  context, but must NOT reclassify a Date/DateTime column to a non-date token.
- "unique_id" is RESERVED and set deterministically by qsv based on cardinality; for any field
  whose first-pass Content Type is "unique_id", OMIT the `content_type` key entirely from your
  output for that field — qsv will re-apply the deterministic value. Do not echo "unique_id"
  yourself; it will be stripped.
- Use cross-field context to pick a BETTER token when warranted. For example, a column named
  "street1" originally classified as "free_text" should become "address_street" once you see
  the sibling city / state / zip columns.
- If you genuinely cannot improve a field's Content Type, keep the first-pass value verbatim.
Allowed Content Type tokens: {{ content_type_vocab }} (plus the optional "duration:N" and
"date:<fmt>" / "datetime:<fmt>" suffix forms).

{% endif %}Return the results in the SAME JSON shape as the first pass:
{% raw %}{
  "field_name_1": {
    "label": "Refined human-friendly label",
    "description": "Refined full description, referencing cross-field relationships when applicable"{% endraw %}{% if infer_content_type %},
    "content_type": "address_street"{% endif %}{% raw %}
  },
  "field_name_2": {
    "label": "...",
    "description": "..."{% endraw %}{% if infer_content_type %},
    "content_type": "..."{% endif %}{% raw %}
  }
}{% endraw %}

Let's think step by step, correcting yourself as needed.
"#;

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
        // With no docopt default on --base-url, flag_base_url is Some
        // ONLY when the user explicitly passed the flag — so a plain
        // Some/None check correctly distinguishes "CLI explicit" from
        // "fall through to env/prompt_file" (codex review job 2363).
        if let Some(cli_base_url) = args.flag_base_url.as_ref() {
            prompt_file.base_url.clone_from(cli_base_url);
        } else if let Ok(env_base_url) = env::var("QSV_LLM_BASE_URL") {
            prompt_file.base_url = env_base_url;
        }
        // else: keep the base_url from the prompt file

        // Priority: Explicit CLI flag > Env var > Prompt file model.
        // Same rationale as --base-url above.
        let model_to_use = if let Some(cli_model) = args.flag_model.as_ref() {
            cli_model.clone()
        } else if let Ok(env_model) = env::var("QSV_LLM_MODEL") {
            env_model
        } else {
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

        // Set the global prompt file. Ignore Err: another thread may have set it
        // concurrently; that's fine — the value is the same and we just use the winner.
        let _ = PROMPT_FILE.set(prompt_file);
        Ok(PROMPT_FILE.get().unwrap())
    }
}

/// Returns the embedded default markdown template TOML content.
const fn get_default_md_template_content() -> &'static str {
    include_str!("../../resources/describegpt_md_defaults.toml")
}

fn get_md_template_file(args: &Args) -> CliResult<&MarkdownTemplateFile> {
    if let Some(file) = MD_TEMPLATE_FILE.get() {
        return Ok(file);
    }
    // Normalize CRLF -> LF so rendered output is byte-identical across platforms. Windows
    // checkouts bundle the embedded default TOML with CRLF (via include_str!), and a
    // user's --markdown-template file may also have CRLF — without normalization, every
    // template line break in the rendered Markdown would carry the platform line ending,
    // diverging from the legacy LF-only `format!()` output and breaking the byte-identity
    // tests on Windows.
    #[allow(clippy::items_after_statements)]
    fn normalize_line_endings(s: &str) -> String {
        s.replace("\r\n", "\n")
    }

    // Always parse the embedded default first — it serves as the per-field fallback for
    // any field a user-supplied --markdown-template TOML omits, so users can drop in a
    // TOML containing only the templates they want to change.
    let base: MarkdownTemplateFile =
        toml::from_str(&normalize_line_endings(get_default_md_template_content()))
            .expect("embedded default markdown template TOML must parse");

    let resolved = if let Some(ref path) = args.flag_markdown_template {
        let content = fs::read_to_string(path).map_err(|e| {
            CliError::Other(format!(
                "Could not read --markdown-template file '{path}': {e}"
            ))
        })?;
        let overlay: MarkdownTemplateOverride = toml::from_str(&normalize_line_endings(&content))
            .map_err(|e| {
            CliError::Other(format!("Markdown template parsing error in '{path}': {e}"))
        })?;
        overlay.apply_to(base)
    } else {
        base
    };

    // Ignore Err: another thread may have set it concurrently; that's fine.
    let _ = MD_TEMPLATE_FILE.set(resolved);
    Ok(MD_TEMPLATE_FILE.get().unwrap())
}

/// Per-phase render context shared across the dictionary body template and the wrapper
/// template. Computed once per `process_phase_output` call so the body footer's attribution
/// timestamp matches the wrapper's `{{ timestamp }}` and `{{ generated_by_signature }}`
/// timestamps in the same rendered document.
struct SharedRenderCtx {
    /// Rendered Markdown attribution block (already substituted for `{GENERATED_BY_SIGNATURE}`).
    attribution: String,
    /// RFC3339 UTC timestamp string, captured once per phase.
    timestamp:   String,
}

impl SharedRenderCtx {
    fn new(args: &Args, model: &str, base_url: &str, kind: PromptType) -> Self {
        Self {
            attribution: replace_attribution_placeholder(
                "{GENERATED_BY_SIGNATURE}",
                args,
                model,
                base_url,
                AttributionFormat::Markdown,
                kind,
            ),
            timestamp:   chrono::Utc::now().to_rfc3339(),
        }
    }
}

fn make_describegpt_md_env() -> &'static Environment<'static> {
    use indicatif::HumanCount;

    static ENV: LazyLock<Environment<'static>> = LazyLock::new(|| {
        let mut env = Environment::new();
        minijinja_contrib::add_to_environment(&mut env);
        // Preserve trailing newlines so default templates byte-match the legacy
        // `format!()` output.
        env.set_keep_trailing_newline(true);

        env.add_filter("pipe_escape", |v: String| v.replace('|', "\\|"));
        env.add_filter("br_replace", |v: String| v.replace('\n', "<br>"));
        env.add_filter("human_count", |v: u64| HumanCount(v).to_string());
        env.add_filter("dict_cell", |v: String, col: String| -> String {
            if col == "percentiles" {
                v.replace(['|', '\n'], "<br>")
            } else {
                v.replace('|', "\\|").replace('\n', "<br>")
            }
        });
        env.add_filter("humanize_examples", |examples: String| -> String {
            if examples == "<ALL_UNIQUE>" {
                return examples;
            }
            examples
                .lines()
                .map(|line| {
                    if let Some(pos) = line.rfind(" [") {
                        let (value_part, count_part) = line.split_at(pos + 2);
                        if let Some(end_pos) = count_part.find(']')
                            && let Ok(count) = count_part[..end_pos].parse::<u64>()
                        {
                            return format!(
                                "{} [{}]",
                                value_part.trim_end_matches(" ["),
                                HumanCount(count)
                            );
                        }
                    }
                    line.to_string()
                })
                .collect::<Vec<String>>()
                .join("<br>")
        });

        env
    });

    &ENV
}

fn render_dictionary_md_body(
    args: &Args,
    entries: &[dictionary::DictionaryEntry],
    addl_col_names: &[String],
    model: &str,
    base_url: &str,
    shared: &SharedRenderCtx,
) -> CliResult<String> {
    let md_file = get_md_template_file(args)?;
    let env = make_describegpt_md_env();

    let ctx = context! {
        entries => entries,
        addl_col_names => addl_col_names,
        infer_content_type => args.flag_infer_content_type,
        kind => PromptType::Dictionary.to_string(),
        generated_by_signature => &shared.attribution,
        model => model,
        base_url => base_url,
        input_filename => args.arg_input.as_deref().unwrap_or("stdin"),
        timestamp => &shared.timestamp,
    };

    env.render_str(&md_file.dictionary_md_body_template, &ctx)
        .map_err(|e| CliError::Other(format!("Dictionary body template render error: {e}")))
}

fn render_markdown_template(
    kind: PromptType,
    args: &Args,
    response_body: &str,
    reasoning: &str,
    token_usage: &TokenUsage,
    model: &str,
    base_url: &str,
    shared: &SharedRenderCtx,
) -> CliResult<String> {
    let md_file = get_md_template_file(args)?;
    let template_str: &str = match kind {
        // DictionaryRefine reuses the Dictionary markdown wrapper — the refine pass only
        // differs in *how* the dictionary fields are produced, not in how they're rendered.
        // In practice, run_dictionary_phase always invokes the output path with
        // PromptType::Dictionary even after a successful refine, so this arm is defensive
        // exhaustiveness rather than a hot code path.
        PromptType::Dictionary | PromptType::DictionaryRefine => &md_file.dictionary_md_template,
        PromptType::Description => &md_file.description_md_template,
        PromptType::Tags => &md_file.tags_md_template,
        PromptType::Prompt => &md_file.custom_prompt_md_template,
    };

    let env = make_describegpt_md_env();

    // Pre-format token_usage with Debug to preserve byte-identical legacy default output.
    // Users who want structured access can use {{ token_usage_struct.prompt }}, etc.
    let token_usage_debug = format!("{token_usage:?}");

    let ctx = context! {
        llm_response => response_body,
        kind => kind.to_string(),
        reasoning => reasoning,
        token_usage => token_usage_debug,
        token_usage_struct => token_usage,
        generated_by_signature => &shared.attribution,
        model => model,
        base_url => base_url,
        input_filename => args.arg_input.as_deref().unwrap_or("stdin"),
        timestamp => &shared.timestamp,
    };

    env.render_str(template_str, &ctx)
        .map_err(|e| CliError::Other(format!("Markdown template render error: {e}")))
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
        // DictionaryRefine reuses the Dictionary warning — the refine pass produces the same
        // shape of output (Label / Description / Content Type), just informed by cross-field
        // context. This arm is defensive: run_dictionary_phase emits with PromptType::Dictionary
        // even after a successful refine.
        PromptType::Dictionary | PromptType::DictionaryRefine => {
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
/// * Whether the SOURCE template referenced `{{ dictionary }}` (any whitespace variation). The
///   Description / Tags / Prompt phase callers use this to avoid the redundant chat-message-side
///   dictionary injection in `build_inference_messages` when the template already inlines
///   DATA_DICTIONARY_JSON. Custom `--prompt-file` users whose templates DON'T reference `{{
///   dictionary }}` continue to receive the dictionary via the chat-message path unchanged.
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
) -> CliResult<(String, String, bool)> {
    // Get prompt file if --prompt-file is used, otherwise get default prompt file
    let prompt_file = get_prompt_file(args)?;

    // Get prompt from prompt file
    let mut prompt = match prompt_type {
        PromptType::Dictionary => prompt_file.dictionary_prompt.clone(),
        PromptType::DictionaryRefine => prompt_file.dictionary_refine_prompt.clone(),
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
        infer_content_type => args.flag_infer_content_type,
        content_type_vocab => dictionary::content_type_vocab_list(),
        // Empty string unless we're rendering PromptType::DictionaryRefine during a
        // --two-pass run, where run_dictionary_phase seeds FIRST_PASS_DICT_JSON with the
        // first-pass dictionary JSON before calling get_prompt.
        //
        // `.expect("...poisoned")` matches `FirstPassDictGuard::seed`'s write-side
        // behavior: a poisoned RwLock means a panic broke an invariant elsewhere and
        // silently coercing to an empty string here would render the refine prompt with
        // no first-pass context, wasting the refine LLM call with no signal to the user.
        // Fail loud instead.
        first_pass_dictionary => FIRST_PASS_DICT_JSON
            .read()
            .expect("FIRST_PASS_DICT_JSON read-lock poisoned")
            .clone()
            .unwrap_or_default(),
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

    // Check the SOURCE template (post-selection, pre-render) for `{{ dictionary }}` so
    // callers can avoid the redundant chat-message-side injection in
    // `build_inference_messages`. The regex tolerates `{{ dictionary }}`,
    // `{{dictionary}}`, `{{  dictionary  }}`, etc.
    let template_inlines_dictionary =
        regex_oncelock!(r"\{\{\s*dictionary\s*\}\}").is_match(&prompt);

    Ok((
        rendered_prompt,
        rendered_system_prompt,
        template_inlines_dictionary,
    ))
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
    let Ok(meta) = fs::metadata(path) else {
        return String::new();
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
    // Short fingerprint of the relevant dictionary JSON. For description / tags / prompt
    // kinds it's the FINAL (refined-when-two-pass) dictionary stashed in
    // `DATA_DICTIONARY_JSON`, which gets injected as the `{{ dictionary }}` template var.
    // For DictionaryRefine, `DATA_DICTIONARY_JSON` is not yet populated at cache-key
    // computation time (it's only set after the refine pass succeeds), so read from
    // `FIRST_PASS_DICT_JSON` instead — that's the JSON the refine prompt actually saw via
    // `{{ first_pass_dictionary }}`. This ties the refine cache entry to the exact
    // first-pass content; if the first pass output changes, the refine entry invalidates.
    let dictionary_fingerprint = if kind == PromptType::DictionaryRefine {
        // Consistent with the get_prompt read side: panic loudly on a poisoned lock
        // rather than silently producing an empty fingerprint (which would let the refine
        // cache return stale entries keyed against a missing first-pass JSON).
        FIRST_PASS_DICT_JSON
            .read()
            .expect("FIRST_PASS_DICT_JSON read-lock poisoned")
            .as_deref()
            .map(|s| blake3::hash(s.as_bytes()).to_hex()[..16].to_string())
    } else {
        DATA_DICTIONARY_JSON
            .get()
            .map(|s| blake3::hash(s.as_bytes()).to_hex()[..16].to_string())
    };
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
         {num_tags};{enum_threshold};{infer_content_type};{sample_size};{fewshot_examples};\
         {duckdb_enabled};{duckdb_path};{duckdb_binary_fp};{dictionary_fingerprint:?}",
        prompt_file = args.flag_prompt_file,
        max_tokens = args.flag_max_tokens,
        addl_props = args.flag_addl_props,
        language = args.flag_language,
        tag_vocab = args.flag_tag_vocab,
        num_tags = args.flag_num_tags,
        enum_threshold = args.flag_enum_threshold,
        infer_content_type = args.flag_infer_content_type,
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
#[concurrent_cached(
    disk = true,
    ty = "cached::DiskCache<String, CompletionResponse>",
    key = "String",
    convert = r##"{ get_cache_key(args, kind, model) }"##,
    create = r##"{
        let cache_dir = DISKCACHE_DIR.get().unwrap();
        let diskcache_config = DISKCACHECONFIG.get().unwrap();
        let diskcache: DiskCache<String, CompletionResponse> = DiskCacheBuilder::new("describegpt")
            .disk_directory(cache_dir)
            .ttl(diskcache_config.ttl_secs)
            .refresh(diskcache_config.ttl_refresh)
            .sync_to_disk_on_cache_change(true)
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
    // this unused_variable lint is a false positive as we use kind in the concurrent_cached macro
    #[allow(unused_variables)] kind: PromptType,
    messages: &serde_json::Value,
) -> Result<Return<CompletionResponse>, CliError> {
    Ok(Return::new(get_completion(
        args, client, model, api_key, messages, kind,
    )?))
}

// this is a redis cache that can be used across qsv sessions
#[concurrent_cached(
    ty = "cached::RedisCache<String, CompletionResponse>",
    key = "String",
    convert = r##"{ get_cache_key(args, kind, model) }"##,
    create = r##" {
        let redis_config = REDISCONFIG.get().unwrap();
        let rediscache: RedisCache<String, CompletionResponse> = RedisCache::new("f", redis_config.ttl_secs)
            .namespace("descq")
            .refresh(redis_config.ttl_refresh)
            .connection_string(&redis_config.conn_str)
            .connection_pool_max_size(redis_config.max_pool_size)
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
) -> Result<Return<CompletionResponse>, CliError> {
    Ok(Return::new(get_completion(
        args, client, model, api_key, messages, kind,
    )?))
}

// Cached analysis results for disk cache
#[concurrent_cached(
    disk = true,
    ty = "cached::DiskCache<String, AnalysisResults>",
    key = "String",
    convert = r##"{ get_analysis_cache_key(args, file_hash) }"##,
    create = r##"{
        let cache_dir = DISKCACHE_DIR.get().unwrap();
        let diskcache_config = DISKCACHECONFIG.get().unwrap();
        let diskcache: DiskCache<String, AnalysisResults> = DiskCacheBuilder::new("describegpt_analysis")
            .disk_directory(cache_dir)
            .ttl(diskcache_config.ttl_secs)
            .refresh(diskcache_config.ttl_refresh)
            .sync_to_disk_on_cache_change(true)
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
) -> Result<Return<AnalysisResults>, CliError> {
    Ok(Return::new(perform_analysis(args, input_path)?))
}

// Cached analysis results for redis cache
#[concurrent_cached(
    ty = "cached::RedisCache<String, AnalysisResults>",
    key = "String",
    convert = r##"{ get_analysis_cache_key(args, file_hash) }"##,
    create = r##" {
        let redis_config = REDISCONFIG.get().unwrap();
        let rediscache: RedisCache<String, AnalysisResults> = RedisCache::new("analysis", redis_config.ttl_secs)
            .namespace("descq")
            .refresh(redis_config.ttl_refresh)
            .connection_string(&redis_config.conn_str)
            .connection_pool_max_size(redis_config.max_pool_size)
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
) -> Result<Return<AnalysisResults>, CliError> {
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
        "jsonschema" | "json-schema" | "json_schema" => Ok(OutputFormat::JsonSchema),
        _ => fail_incorrectusage_clierror!(
            "Invalid format '{format_str}'. Must be one of: Markdown, TSV, JSON, TOON, JSONSchema"
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

/// The `--null-text` configured for the `frequency` command via `--freq-options`,
/// or `frequency`'s default (`(NULL)`) when none is set. `frequency` writes the
/// null row's `value` with exactly this text; the dictionary date-format checks
/// pair it with the column null count to recognize that row.
///
/// When frequency data is supplied via a `file:`, the configured null text is
/// unknown, so the default `(NULL)` is assumed. A `file:`-backed CSV generated
/// with BOTH a custom `--null-text` AND `--pct-nulls` may therefore leave its
/// ranked null row among the date-format samples — a documented limitation of
/// `file:` input (see the `--freq-options` help).
fn configured_null_text(freq_options: &str) -> &str {
    let mut tokens = freq_options.split_whitespace();
    while let Some(tok) = tokens.next() {
        if let Some(val) = tok.strip_prefix("--null-text=") {
            return val;
        }
        if tok == "--null-text"
            && let Some(val) = tokens.next()
        {
            return val;
        }
    }
    "(NULL)"
}

fn build_combined_dictionary_entries(
    args: &Args,
    analysis_results: &AnalysisResults,
    completion_response: &CompletionResponse,
) -> CliResult<(Vec<dictionary::DictionaryEntry>, Vec<serde_json::Value>)> {
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
        args.flag_infer_content_type,
    );
    let field_names: Vec<String> = code_entries.iter().map(|e| e.name.clone()).collect();
    let llm_fields = parse_llm_dictionary_response(
        &completion_response.response,
        &field_names,
        args.flag_infer_content_type,
    )
    .unwrap_or_default();
    let mut entries =
        combine_dictionary_entries(code_entries, &llm_fields, args.flag_infer_content_type);
    if args.flag_infer_content_type {
        let null_text = configured_null_text(&args.flag_freq_options);
        // strip any LLM-inferred date/datetime strftime suffix that does not
        // actually parse the column's real values
        dictionary::validate_date_formats(&mut entries, &frequency_records, null_text);
        // reclassify a datetime column as date when its frequency-sampled
        // values are all at midnight (a date stored with a zero time-of-day)
        dictionary::downgrade_all_midnight_datetime_columns(
            &mut entries,
            &frequency_records,
            null_text,
        );
    }
    // LLM-inferred inter-column relationships, validated structurally against the
    // real field names. `synthesize` re-validates them against the data.
    let relationships =
        dictionary::parse_llm_relationships(&completion_response.response, &field_names);
    Ok((entries, relationships))
}

/// Two-pass variant: parse stats/frequency, parse BOTH the baseline (first-pass) and refine
/// (second-pass) LLM responses, then merge via `combine_dictionary_entries_with_baseline`
/// so the final entries inherit baseline Label/Description for any field the refine pass
/// omitted. Used by `run_dictionary_phase`'s `--two-pass` branch.
///
/// Relationships are taken from the FIRST-PASS response: the first pass renders the
/// relationship-aware `dictionary_prompt`, while the refine prompt only revisits
/// per-field Label/Description/Content Type.
fn build_combined_dictionary_entries_two_pass(
    args: &Args,
    analysis_results: &AnalysisResults,
    baseline_completion: &CompletionResponse,
    refine_completion: &CompletionResponse,
) -> CliResult<(Vec<dictionary::DictionaryEntry>, Vec<serde_json::Value>)> {
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
        args.flag_infer_content_type,
    );
    let field_names: Vec<String> = code_entries.iter().map(|e| e.name.clone()).collect();
    let baseline_fields = parse_llm_dictionary_response(
        &baseline_completion.response,
        &field_names,
        args.flag_infer_content_type,
    )
    .unwrap_or_default();
    let refine_fields = parse_llm_dictionary_response(
        &refine_completion.response,
        &field_names,
        args.flag_infer_content_type,
    )
    .unwrap_or_default();
    let mut entries = combine_dictionary_entries_with_baseline(
        code_entries,
        &baseline_fields,
        &refine_fields,
        args.flag_infer_content_type,
    );
    if args.flag_infer_content_type {
        let null_text = configured_null_text(&args.flag_freq_options);
        // strip any LLM-inferred date/datetime strftime suffix that does not
        // actually parse the column's real values
        dictionary::validate_date_formats(&mut entries, &frequency_records, null_text);
        // reclassify a datetime column as date when its frequency-sampled
        // values are all at midnight (a date stored with a zero time-of-day)
        dictionary::downgrade_all_midnight_datetime_columns(
            &mut entries,
            &frequency_records,
            null_text,
        );
    }
    let relationships =
        dictionary::parse_llm_relationships(&baseline_completion.response, &field_names);
    Ok((entries, relationships))
}

/// Produce the prettified first-pass dictionary JSON string used as `{{ first_pass_dictionary }}`
/// context for the refine prompt AND as the input to the `DictionaryRefine` cache-key
/// fingerprint.
///
/// Critically, this STRIPS the `attribution` field that `format_dictionary_json` always emits.
/// `format_dictionary_json` sets `attribution` to the literal placeholder
/// `"{GENERATED_BY_SIGNATURE}"`, and the user-facing emit paths
/// (`format_dictionary_phase` / `emit_dictionary_context_only`) expand it via
/// `replace_attribution_placeholder`, which injects a live `chrono::Utc::now()` timestamp
/// and the full process `command_line`. If we did the same expansion here, every invocation
/// would produce a different `FIRST_PASS_DICT_JSON` string even for byte-identical first-pass
/// dictionary content — and since the refine-cache key is fingerprinted from that string,
/// the refine cache would effectively never hit, forcing a fresh second LLM call on every run.
///
/// Stripping the field entirely (rather than leaving the unexpanded placeholder) also keeps
/// the JSON the LLM sees free of bookkeeping noise — attribution metadata is for the
/// user-facing output, not the refine prompt context.
fn build_first_pass_dictionary_json_string(
    args: &Args,
    combined_entries: &[dictionary::DictionaryEntry],
) -> String {
    // The intermediate first-pass JSON the refine prompt sees never carries
    // relationships — they are inferred once and emitted only in the final output.
    let mut dictionary_json = formatters::format_dictionary_json(
        combined_entries,
        args.flag_enum_threshold,
        args.flag_num_examples,
        args.flag_truncate_str,
        args.flag_infer_content_type,
        &[],
    );
    if let Some(obj) = dictionary_json.as_object_mut() {
        obj.remove("attribution");
    }
    serde_json::to_string_pretty(&dictionary_json).unwrap_or_default()
}

fn emit_dictionary_context_only(
    args: &Args,
    combined_entries: &[dictionary::DictionaryEntry],
    relationships: &[serde_json::Value],
    model: &str,
    base_url: &str,
) -> CliResult<()> {
    let mut dictionary_json = formatters::format_dictionary_json(
        combined_entries,
        args.flag_enum_threshold,
        args.flag_num_examples,
        args.flag_truncate_str,
        args.flag_infer_content_type,
        relationships,
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

/// Full Dictionary phase output across JSON/TOON/TSV/Markdown/JSONSchema formats.
///
/// Takes pre-built `combined_entries` rather than re-parsing `analysis_results +
/// completion_response` so the same emit path serves both the single-pass flow
/// (`process_phase_output` builds entries from one completion) and the two-pass flow
/// (`run_dictionary_phase` builds entries by merging baseline + refine completions via
/// `combine_dictionary_entries_with_baseline`).
///
/// `relationships` carries the LLM-inferred inter-column relationships; it is empty
/// unless the dictionary prompt inferred any.
#[allow(clippy::too_many_arguments)]
fn format_dictionary_phase(
    kind: PromptType,
    args: &Args,
    combined_entries: &[dictionary::DictionaryEntry],
    relationships: &[serde_json::Value],
    completion_response: &CompletionResponse,
    total_json_output: &mut serde_json::Value,
    model: &str,
    base_url: &str,
    output_format: OutputFormat,
) -> CliResult<()> {
    if output_format == OutputFormat::JsonSchema {
        // Build the draft 2020-12 JSON Schema base. The final emission (with any
        // --description / --tags integration) happens in `finalize_structured_output`,
        // which reads back what we stash under `total_json_output[kind]["response"]`.
        let input_filename = args.arg_input.as_deref().map_or("input", |p| {
            std::path::Path::new(p)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(p)
        });
        let schema_value = formatters::format_dictionary_jsonschema(
            combined_entries,
            input_filename,
            args.flag_enum_threshold,
            args.flag_num_examples,
            args.flag_truncate_str,
            args.flag_infer_content_type,
            args.flag_allow_extra_cols,
            args.flag_strict_dates,
        );
        let attribution = replace_attribution_placeholder(
            "{GENERATED_BY_SIGNATURE}",
            args,
            model,
            base_url,
            AttributionFormat::Markdown,
            PromptType::Dictionary,
        );
        // Re-emit the schema with the attribution baked into `x-qsv.generated_by`
        // (the formatter writes the literal placeholder; replace it once here so we
        // don't have to thread args/model/base_url through the formatter).
        let mut schema_value = schema_value;
        if let Some(x_qsv) = schema_value
            .get_mut("x-qsv")
            .and_then(serde_json::Value::as_object_mut)
        {
            x_qsv.insert("generated_by".to_string(), json!(attribution));
        }
        // Downstream Description/Tags prompts read `DATA_DICTIONARY_JSON` as the
        // `{{ dictionary }}` Mini-Jinja variable, and that variable's contract is the
        // dictionary-entries shape produced by `format_dictionary_json` — NOT a JSON
        // Schema scaffold. Cache that shape regardless of the chosen output format
        // so `--description --format jsonschema` and `--tags --format jsonschema`
        // feed the LLM the same context every other format does. The schema document
        // stays in `total_json_output[Dictionary]["response"]` for
        // `finalize_structured_output` to consume.
        let dictionary_json = formatters::format_dictionary_json(
            combined_entries,
            args.flag_enum_threshold,
            args.flag_num_examples,
            args.flag_truncate_str,
            args.flag_infer_content_type,
            relationships,
        );
        DATA_DICTIONARY_JSON
            .get_or_init(|| serde_json::to_string_pretty(&dictionary_json).unwrap());
        total_json_output[kind.to_string()] = json!({
            "response": schema_value,
            "reasoning": completion_response.reasoning,
            "token_usage": completion_response.token_usage,
        });
    } else if output_format == OutputFormat::Json || output_format == OutputFormat::Toon {
        let mut dictionary_json = formatters::format_dictionary_json(
            combined_entries,
            args.flag_enum_threshold,
            args.flag_num_examples,
            args.flag_truncate_str,
            args.flag_infer_content_type,
            relationships,
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
        let mut tsv_output =
            formatters::format_dictionary_tsv(combined_entries, args.flag_infer_content_type);
        tsv_output.push_str(&format_token_usage_comments(
            &completion_response.reasoning,
            &completion_response.token_usage,
        ));
        let dictionary_json = formatters::format_dictionary_json(
            combined_entries,
            args.flag_enum_threshold,
            args.flag_num_examples,
            args.flag_truncate_str,
            args.flag_infer_content_type,
            relationships,
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
        let addl_col_names = formatters::extract_ordered_addl_cols(combined_entries);
        // Compute attribution + timestamp once per phase so the body footer's attribution
        // matches the wrapper's `{{ generated_by_signature }}` / `{{ timestamp }}` byte for byte.
        let shared = SharedRenderCtx::new(args, model, base_url, PromptType::Dictionary);
        let mut markdown_output = render_dictionary_md_body(
            args,
            combined_entries,
            &addl_col_names,
            model,
            base_url,
            &shared,
        )?;
        // Belt-and-suspenders: also substitute any literal {GENERATED_BY_SIGNATURE} that may
        // have leaked through from a custom dictionary prompt or template.
        markdown_output = replace_attribution_placeholder(
            &markdown_output,
            args,
            model,
            base_url,
            AttributionFormat::Markdown,
            PromptType::Dictionary,
        );
        let formatted_output = render_markdown_template(
            kind,
            args,
            &markdown_output,
            &completion_response.reasoning,
            &completion_response.token_usage,
            model,
            base_url,
            &shared,
        )?;
        let dictionary_json = formatters::format_dictionary_json(
            combined_entries,
            args.flag_enum_threshold,
            args.flag_num_examples,
            args.flag_truncate_str,
            args.flag_infer_content_type,
            relationships,
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

fn format_phase_markdown(
    kind: PromptType,
    args: &Args,
    completion_response: &CompletionResponse,
    is_sql_response: bool,
    model: &str,
    base_url: &str,
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
    let shared = SharedRenderCtx::new(args, model, base_url, kind);
    let rendered = render_markdown_template(
        kind,
        args,
        &formatted_output,
        &completion_response.reasoning,
        &completion_response.token_usage,
        model,
        base_url,
        &shared,
    )?;
    if let Some(output) = &args.flag_output {
        fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(output)?
            .write_all(rendered.as_bytes())?;
    } else {
        println!("{rendered}");
    }
    Ok(())
}

/// Process the output of a single inference phase.
/// Extracted from run_inference_options::process_output for reuse by --process-response.
///
/// Dictionary kinds build `combined_entries` from the single `completion_response` here
/// (the single-pass and `--process-response` flows). The two-pass flow does NOT route the
/// refined output through this function: `run_dictionary_phase` calls the refactored
/// `format_dictionary_phase` / `emit_dictionary_context_only` directly with pre-merged
/// baseline+refine entries, because routing through here would lose the baseline values
/// for any field the refine pass omitted.
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
        let (combined_entries, relationships) =
            build_combined_dictionary_entries(args, analysis_results, completion_response)?;
        return emit_dictionary_context_only(
            args,
            &combined_entries,
            &relationships,
            model,
            base_url,
        );
    }

    if kind == PromptType::Dictionary {
        let (combined_entries, relationships) =
            build_combined_dictionary_entries(args, analysis_results, completion_response)?;
        return format_dictionary_phase(
            kind,
            args,
            &combined_entries,
            &relationships,
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
        (OutputFormat::Json | OutputFormat::JsonSchema, false) => {
            // JsonSchema mode reuses the JSON accumulator for Description/Tags so
            // `finalize_structured_output` can fold their LLM responses into the
            // schema's top-level `description` / `x-qsv.tags`.
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
        | (
            OutputFormat::Json | OutputFormat::Tsv | OutputFormat::Toon | OutputFormat::JsonSchema,
            true,
        ) => format_phase_markdown(
            kind,
            args,
            completion_response,
            is_sql_response,
            model,
            base_url,
        ),
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

/// Run the Data Dictionary inference phase. Returns the completion the downstream phases
/// (Description / Tags / Prompt) should treat as the dictionary context.
///
/// In `--two-pass` mode, a second LLM call refines the first-pass dictionary with full
/// cross-field context (the LLM sees the entire first-pass dictionary JSON via
/// `{{ first_pass_dictionary }}` and can recognize that, for example, fields like
/// street_no, street_name, city, state and zip together describe a single mailing
/// address). The returned `CompletionResponse` then carries the REFINED response text
/// (so downstream phases see the better dictionary as `{{ dictionary }}` context), the
/// concatenated reasoning trace from both passes, and the SUMMED token usage. The
/// first-pass response is intentionally not emitted to the user — only the refined
/// dictionary appears in the output.
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
    // --- Pass 1: existing single-pass behavior, prompt = PromptType::Dictionary. ---
    let (prompt, system_prompt, _) =
        get_prompt(PromptType::Dictionary, Some(analysis_results), args)?;
    let pass1_start = Instant::now();
    print_status(
        if args.flag_two_pass {
            "  Inferring Data Dictionary (pass 1/2)..."
        } else {
            "  Inferring Data Dictionary..."
        },
        None,
    );
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
            "   Received {}dictionary inference.\n   {:?}\n  ",
            if args.flag_two_pass {
                "first-pass "
            } else {
                ""
            },
            data_dict.token_usage
        ),
        Some(pass1_start.elapsed()),
    );

    if !args.flag_two_pass {
        // Single-pass: existing behavior, emit and return.
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
        return Ok(data_dict);
    }

    // --- Two-pass: build first-pass entries (without emitting), seed
    // --- FIRST_PASS_DICT_JSON, render the refine prompt, and call the LLM again. ---
    // First-pass relationships are discarded here; the final ones are re-parsed
    // from the (relationship-aware) first-pass response by the two-pass builder.
    let (first_pass_entries, _) =
        build_combined_dictionary_entries(args, analysis_results, &data_dict)?;
    let first_pass_json = build_first_pass_dictionary_json_string(args, &first_pass_entries);
    // Seed the global the refine prompt template reads via `{{ first_pass_dictionary }}`.
    // The returned guard's `Drop` impl clears `FIRST_PASS_DICT_JSON` on ALL exit paths from
    // this point onward — both the Ok path below and any `?`-propagated error from
    // `get_prompt`, `get_cached_completion`, or `build_combined_dictionary_entries_two_pass`.
    // Without the guard, an error path would leave the slot populated for the rest of the
    // process lifetime (harmless in CLI one-shot, but a leak in the test-harness /
    // repeated-invocation case the static's doc comment calls out).
    let _first_pass_guard = FirstPassDictGuard::seed(first_pass_json);

    let (refine_prompt, refine_system_prompt, _) =
        get_prompt(PromptType::DictionaryRefine, Some(analysis_results), args)?;
    let pass2_start = Instant::now();
    print_status(
        "  Refining Data Dictionary with cross-field context (pass 2/2)...",
        None,
    );
    let refine_messages = build_inference_messages(&refine_prompt, &refine_system_prompt, "", None);

    let refine_completion = get_cached_completion(
        args,
        client,
        model,
        api_key,
        dictionary_cache_type,
        PromptType::DictionaryRefine,
        &refine_messages,
    )?;
    print_status(
        &format!(
            "   Received refined dictionary.\n   {:?}\n  ",
            refine_completion.token_usage
        ),
        Some(pass2_start.elapsed()),
    );

    // Note: FIRST_PASS_DICT_JSON is cleared automatically when `_first_pass_guard` goes
    // out of scope at the end of this function. The guard's `Drop` impl handles both the
    // success path here and any `?`-propagated error above.

    // Merge baseline + refine via the baseline-preserving combine so refine omissions
    // don't wipe first-pass Label/Description. Then emit using the SAME format functions
    // the single-pass flow uses — process_phase_output is bypassed here because routing
    // back through it would re-parse only the refine response and lose the baseline.
    let (merged_entries, relationships) = build_combined_dictionary_entries_two_pass(
        args,
        analysis_results,
        &data_dict,
        &refine_completion,
    )?;

    // Synthesize the CompletionResponse that emit functions receive: response text comes
    // from the refine pass (it's what gets shown in the output's `response` JSON field
    // for --format json/toon), reasoning concatenates both traces, token_usage sums both
    // calls so the reported total reflects actual cost.
    let combined_completion = CompletionResponse {
        response:    refine_completion.response.clone(),
        reasoning:   format!(
            "FIRST PASS REASONING:\n{}\n\nREFINE PASS REASONING:\n{}",
            data_dict.reasoning, refine_completion.reasoning,
        ),
        token_usage: TokenUsage {
            prompt:     data_dict.token_usage.prompt + refine_completion.token_usage.prompt,
            completion: data_dict.token_usage.completion + refine_completion.token_usage.completion,
            total:      data_dict.token_usage.total + refine_completion.token_usage.total,
            elapsed:    data_dict.token_usage.elapsed + refine_completion.token_usage.elapsed,
        },
    };

    if args.flag_prompt.is_some() {
        // --prompt mode: stash refined dictionary JSON for downstream SQL RAG context, no
        // user-visible dictionary output.
        emit_dictionary_context_only(args, &merged_entries, &relationships, model, base_url)?;
    } else {
        format_dictionary_phase(
            PromptType::Dictionary,
            args,
            &merged_entries,
            &relationships,
            &combined_completion,
            total_json_output,
            model,
            base_url,
            output_format,
        )?;
    }

    Ok(combined_completion)
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
    let (prompt, system_prompt, template_inlines_dictionary) =
        get_prompt(PromptType::Description, Some(analysis_results), args)?;
    // If the description template already inlines `{{ dictionary }}`, skip the redundant
    // chat-message-side injection. Templates that don't reference it (custom prompt files
    // authored before the default added the dictionary section) still get the chat-message
    // path, preserving backward compatibility.
    let chat_dict_injection = if template_inlines_dictionary {
        ""
    } else {
        dictionary_response
    };
    let messages = build_inference_messages(&prompt, &system_prompt, chat_dict_injection, None);
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
    let (prompt, system_prompt, template_inlines_dictionary) =
        get_prompt(PromptType::Tags, Some(analysis_results), args)?;
    // Skip the redundant chat-message-side dictionary injection when the tags template
    // already inlines `{{ dictionary }}` (the default since the dictionary-in-template
    // change). Custom prompt files without `{{ dictionary }}` keep the chat-message path.
    let chat_dict_injection = if template_inlines_dictionary {
        ""
    } else {
        dictionary_context
    };
    let messages = build_inference_messages(&prompt, &system_prompt, chat_dict_injection, None);
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

    let (prompt, system_prompt, template_inlines_dictionary) =
        get_prompt(PromptType::Prompt, Some(analysis_results), args)?;
    let start_time = Instant::now();
    print_status("  Answering Custom Prompt...", None);
    // `custom_prompt_guidance` (the default body for PromptType::Prompt) inlines
    // `{{ dictionary }}` for SQL RAG mode, so skip the chat-message-side injection in
    // that case. Custom prompt files without the placeholder still get the chat-message
    // path so they aren't silently regressed.
    let chat_dict_injection = if template_inlines_dictionary {
        ""
    } else {
        dictionary_response
    };
    let messages = build_inference_messages(
        &prompt,
        &system_prompt,
        chat_dict_injection,
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
        OutputFormat::JsonSchema => {
            // Pull the schema base emitted by `format_dictionary_phase`. Required —
            // run() rejects --format jsonschema without --dictionary/--all, so this
            // entry must exist by the time we get here.
            let mut schema = total_json_output
                .get(PromptType::Dictionary.to_string())
                .and_then(|v| v.get("response"))
                .cloned()
                .ok_or_else(|| {
                    CliError::Other(
                        "--format jsonschema: dictionary schema missing from phase output. This \
                         is a bug — please report it."
                            .to_string(),
                    )
                })?;

            // Fold the Description phase response (if it ran) into the schema's
            // top-level `description`, overriding the placeholder set by the formatter.
            if let Some(desc_value) = total_json_output
                .get(PromptType::Description.to_string())
                .and_then(|v| v.get("response"))
                && let Some(desc_str) = desc_value.as_str()
                && !desc_str.is_empty()
                && let Some(obj) = schema.as_object_mut()
            {
                obj.insert("description".to_string(), json!(desc_str));
            }

            // Fold the Tags phase response (if it ran) into `x-qsv.tags`.
            if let Some(tags_value) = total_json_output
                .get(PromptType::Tags.to_string())
                .and_then(|v| v.get("response"))
                && !tags_value.is_null()
                && let Some(obj) = schema.as_object_mut()
                && let Some(x_qsv) = obj
                    .get_mut("x-qsv")
                    .and_then(serde_json::Value::as_object_mut)
            {
                x_qsv.insert("tags".to_string(), tags_value.clone());
            }

            // Meta-validate the emitted schema against the draft 2020-12 meta-schema
            // (auto-detected from the embedded `$schema` URL). Converts authoring bugs
            // into immediate, loud failures rather than silently producing schemas that
            // downstream consumers reject.
            if let Err(e) = jsonschema::meta::validate(&schema) {
                return fail_clierror!(
                    "Emitted JSON Schema failed meta-validation against draft 2020-12: {e}"
                );
            }

            let schema_output = serde_json::to_string_pretty(&schema)?;
            if let Some(output_file_path) = &args.flag_output {
                fs::write(output_file_path, schema_output)?;
            } else {
                println!("{schema_output}");
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

    // Read the effective base URL FIRST so we can hand it to the
    // reqwest client (for retry host classification — reqwest's
    // `for_host` matches this against actual request hosts). The
    // effective URL is CLI > env > prompt_file > built-in default;
    // get_prompt_file applies that precedence and stores the result in
    // prompt_file.base_url. Using resolve_base_url(args) here would
    // skip the prompt_file fallback — codex review job 2372.
    let base_url = get_prompt_file(args)?.base_url.clone();

    let client = util::create_reqwest_blocking_client(
        args.flag_user_agent.clone(),
        // unwrap_or 0 because 0 is a valid timeout per the usage text (local LLMs)
        util::timeout_secs(args.flag_timeout).unwrap_or(0) as u16,
        Some(base_url.clone()),
    )?;

    let model = check_model(&client, Some(api_key), args)?;
    let output_format = get_output_format(args)?;

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
    // When removing a Dictionary cache entry we ALSO remove DictionaryRefine: the refine
    // entry's cache key embeds the first-pass dictionary fingerprint, so it's logically
    // downstream of the Dictionary entry. Without the cascade, `--forget --dictionary`
    // would leave a stale refine entry that would no longer match any first-pass content.
    if args.flag_dictionary {
        vec![PromptType::Dictionary, PromptType::DictionaryRefine]
    } else if args.flag_description {
        vec![PromptType::Description]
    } else if args.flag_tags {
        vec![PromptType::Tags]
    } else if args.flag_prompt.is_some() {
        vec![PromptType::Prompt]
    } else {
        vec![
            PromptType::Dictionary,
            PromptType::DictionaryRefine,
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

    // --format jsonschema specific validation: schema is dictionary-centric, so require
    // a dictionary phase. --description/--tags piggyback on the dictionary schema (their
    // outputs become top-level `description` and `x-qsv.tags`); --prompt produces a
    // natural-language/SQL answer which is conceptually separate from a schema.
    if get_output_format(&args)? == OutputFormat::JsonSchema {
        if !(args.flag_dictionary || args.flag_all) {
            return fail_incorrectusage_clierror!(
                "--format jsonschema requires --dictionary (or --all)."
            );
        }
        if args.flag_prompt.is_some() {
            return fail_incorrectusage_clierror!(
                "--format jsonschema is not compatible with --prompt."
            );
        }
    } else if args.flag_allow_extra_cols && !args.flag_quiet {
        wwarn!("--allow-extra-cols is only meaningful with --format jsonschema; ignoring.");
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

        // Delegate accumulated-output emission to the same finalizer the live
        // inference path uses so every accumulating format (Json, Toon,
        // JsonSchema, …) gets emitted consistently. Markdown/TSV are inline
        // emit-as-you-go and finalize_structured_output is a no-op for them.
        finalize_structured_output(&args, &total_json_output, output_format)?;

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

    // Resolve the EFFECTIVE base URL — the actual host the API request
    // will go to. This MUST include the prompt-file's base_url in the
    // precedence: CLI > env > prompt_file > built-in default. Using
    // resolve_base_url(args) here would skip the prompt_file fallback
    // and, with --prompt-file pointing at a remote provider and no
    // CLI/env URL, leave us reading "http://localhost:1234/v1" — passing
    // the localhost gate below and allowing an unauthenticated request
    // to the remote provider (codex review job 2372). get_prompt_file
    // applies the same precedence and writes the result into
    // prompt_file.base_url, so reading it here is the canonical value.
    //
    // (The earlier fix for codex review job 2363 — removing the docopt
    // default for --base-url — is what lets get_prompt_file's
    // Some-iff-explicit check on args.flag_base_url work correctly.)
    let effective_base_url = get_prompt_file(&args)?.base_url.clone();

    // Priority: CLI flag > Env var > default/error
    //
    // --prepare-context only emits prompt/context JSON and never calls
    // the LLM API, so skip the api-key requirement for that mode —
    // requiring credentials for a remote prompt-file URL would block a
    // legitimate no-network use case (codex review job 2373). The
    // --process-response branch above already returns before reaching
    // here, so it's not affected. `api_key` is unused on the
    // --prepare-context path; the empty string is a typed placeholder.
    let api_key: String = if args.flag_prepare_context {
        String::new()
    } else if effective_base_url.contains("localhost") {
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

    // --two-pass requires a dictionary-producing inference flag (--dictionary, --all, or
    // --prompt, which internally builds a dictionary for SQL RAG context).
    if args.flag_two_pass && !args.flag_dictionary && !args.flag_all && args.flag_prompt.is_none() {
        return fail_incorrectusage_clierror!(
            "--two-pass requires --dictionary, --all, or --prompt."
        );
    }

    // --two-pass is incompatible with the MCP sampling flags. The refine pass needs the
    // first-pass LLM response as input, but --prepare-context emits prompts without ever
    // calling the LLM and --process-response consumes a single phase response per kind,
    // so neither can plumb the two-call dependency in this iteration.
    if args.flag_two_pass && (args.flag_prepare_context || args.flag_process_response) {
        return fail_incorrectusage_clierror!(
            "--two-pass cannot be used with --prepare-context or --process-response. MCP sampling \
             is single-turn per phase; the refine pass requires the first-pass LLM response and \
             is not currently supported in MCP mode."
        );
    }

    // --two-pass doubles dictionary LLM cost and latency. Without caching, every invocation
    // pays both passes; warn so the user doesn't accidentally burn tokens on retries.
    if args.flag_two_pass && args.flag_no_cache {
        print_status(
            "Warning: --two-pass with --no-cache will issue two uncached LLM calls per run (first \
             pass + refine). Consider omitting --no-cache to amortize cost across runs.",
            None,
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
            let (user_prompt, system_prompt, _) =
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
            let (user_prompt, system_prompt, _) =
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
            let (user_prompt, system_prompt, _) =
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
            let (user_prompt, system_prompt, _) =
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
            (
                "flag_infer_content_type",
                Box::new(|a| a.flag_infer_content_type = true),
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

    #[test]
    fn dictionary_refine_cache_key_differs_from_dictionary_cache_key() {
        // The refine pass must NEVER share a cache slot with the first pass. The
        // distinct PromptType variant is what guarantees this — the `{kind}` field
        // appears in the cache key format string.
        let args = default_args_for_test();
        let first_pass_key =
            get_cache_key_with_flag(&args, PromptType::Dictionary, "gpt-x", "valid");
        let refine_key =
            get_cache_key_with_flag(&args, PromptType::DictionaryRefine, "gpt-x", "valid");
        assert_ne!(
            first_pass_key, refine_key,
            "Dictionary and DictionaryRefine must produce distinct cache keys, otherwise two-pass \
             would silently return the first-pass cached completion as the refine output (or vice \
             versa)"
        );
    }

    #[test]
    fn two_pass_flag_does_not_change_first_pass_cache_key() {
        // The first-pass cache key is intentionally identical whether or not --two-pass
        // is set: the first pass renders the same `dictionary_prompt` template regardless,
        // and we want first-pass cache hits to be reusable across --two-pass toggles.
        // Refining is gated by a separate cache entry under PromptType::DictionaryRefine.
        let mut args_off = default_args_for_test();
        args_off.flag_two_pass = false;
        let mut args_on = default_args_for_test();
        args_on.flag_two_pass = true;

        let key_off = get_cache_key_with_flag(&args_off, PromptType::Dictionary, "gpt-x", "valid");
        let key_on = get_cache_key_with_flag(&args_on, PromptType::Dictionary, "gpt-x", "valid");
        assert_eq!(
            key_off, key_on,
            "flipping --two-pass must NOT change the first-pass cache key — first-pass prompt \
             content is identical and cache reuse across toggles is intentional"
        );
    }

    /// Test-only mutex serializing tests that mutate `FIRST_PASS_DICT_JSON`. Cargo runs
    /// unit tests in the same binary in parallel by default, so tests that
    /// read-then-mutate-then-read would otherwise race and observe each other's writes.
    /// Lock this BEFORE reading or writing `FIRST_PASS_DICT_JSON` and HOLD it for the
    /// full sequence of operations.
    static FIRST_PASS_DICT_JSON_TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    #[test]
    fn dictionary_refine_cache_key_reflects_first_pass_dict_json() {
        // When kind == DictionaryRefine, the dictionary_fingerprint is sourced from
        // FIRST_PASS_DICT_JSON (not DATA_DICTIONARY_JSON, which isn't populated yet at
        // refine-cache-key computation time). This ties the refine cache entry to the
        // exact first-pass content; a different first-pass output must miss the cache.
        let _guard = FIRST_PASS_DICT_JSON_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let args = default_args_for_test();
        // Save the current FIRST_PASS_DICT_JSON to restore after — other tests in this
        // binary may have set it. RwLock allows overwrite, so this is safe.
        let saved = FIRST_PASS_DICT_JSON.read().ok().and_then(|g| g.clone());

        // Refine key with FIRST_PASS_DICT_JSON = None.
        {
            let mut guard = FIRST_PASS_DICT_JSON.write().unwrap();
            *guard = None;
        }
        let key_empty =
            get_cache_key_with_flag(&args, PromptType::DictionaryRefine, "gpt-x", "valid");

        // Refine key with FIRST_PASS_DICT_JSON = Some(content_A).
        {
            let mut guard = FIRST_PASS_DICT_JSON.write().unwrap();
            *guard = Some(r#"{"f": "first-pass A"}"#.to_string());
        }
        let key_a = get_cache_key_with_flag(&args, PromptType::DictionaryRefine, "gpt-x", "valid");

        // Refine key with FIRST_PASS_DICT_JSON = Some(content_B) (different content).
        {
            let mut guard = FIRST_PASS_DICT_JSON.write().unwrap();
            *guard = Some(r#"{"f": "first-pass B"}"#.to_string());
        }
        let key_b = get_cache_key_with_flag(&args, PromptType::DictionaryRefine, "gpt-x", "valid");

        // Restore prior state so we don't leak state into sibling tests.
        {
            let mut guard = FIRST_PASS_DICT_JSON.write().unwrap();
            *guard = saved;
        }

        assert_ne!(
            key_empty, key_a,
            "refine key must differ when FIRST_PASS_DICT_JSON goes from None to Some"
        );
        assert_ne!(
            key_a, key_b,
            "refine key must differ when first-pass content differs"
        );
    }

    #[test]
    fn dictionary_refine_prompt_renders_first_pass_dictionary_var() {
        // The refine prompt template MUST reference `{{ first_pass_dictionary }}` and the
        // renderer MUST surface FIRST_PASS_DICT_JSON's value. If either link breaks, the
        // LLM never sees the first-pass dictionary during the refine pass — silent
        // regression to a useless second LLM call. Save/restore FIRST_PASS_DICT_JSON so
        // this test doesn't poison sibling tests in the same binary run.
        let _guard = FIRST_PASS_DICT_JSON_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let saved = FIRST_PASS_DICT_JSON.read().ok().and_then(|g| g.clone());

        let sentinel = "SENTINEL_FIRST_PASS_PAYLOAD_42";
        {
            let mut guard = FIRST_PASS_DICT_JSON.write().unwrap();
            *guard = Some(format!(r#"{{"some_field": "{sentinel}"}}"#));
        }

        let analysis = AnalysisResults {
            stats: "field,type\nname,String\n".to_string(),
            frequency: "field,value,count,percentage,rank\n".to_string(),
            headers: "name".to_string(),
            ..AnalysisResults::default()
        };
        let mut args = default_args_for_test();
        args.flag_two_pass = true;
        // get_prompt -> get_prompt_file unwraps flag_base_url whenever it differs from
        // DEFAULT_BASE_URL, so populate it with the default to avoid the unwrap-on-None
        // panic in tests that bypass `util::get_args`'s docopt-default population.
        args.flag_base_url = Some(DEFAULT_BASE_URL.to_string());
        args.flag_model = Some(DEFAULT_MODEL.to_string());

        let result = get_prompt(PromptType::DictionaryRefine, Some(&analysis), &args);

        // Restore before any assertion so a failure doesn't leak state.
        {
            let mut guard = FIRST_PASS_DICT_JSON.write().unwrap();
            *guard = saved;
        }

        let (rendered, _system, _inlines_dict) = result.expect("refine prompt should render");
        assert!(
            rendered.contains(sentinel),
            "refine prompt must surface FIRST_PASS_DICT_JSON contents via {{{{ \
             first_pass_dictionary }}}}; got:\n{rendered}"
        );
    }

    #[test]
    fn get_prompt_signals_when_template_inlines_dictionary() {
        // The default Description and Tags templates now include `{{ dictionary }}`
        // (gated on `{% if dictionary %}`), so get_prompt's third return value MUST be
        // true for them. The Description / Tags / Prompt phase callers rely on this
        // flag to skip the redundant chat-message-side dictionary injection.
        //
        // Also asserts the negative path: the Dictionary first-pass template (which
        // never inlines the dictionary — that would be self-referential) returns false.
        let mut args = default_args_for_test();
        args.flag_base_url = Some(DEFAULT_BASE_URL.to_string());
        args.flag_model = Some(DEFAULT_MODEL.to_string());

        let analysis = AnalysisResults {
            stats: "field,type\nname,String\n".to_string(),
            frequency: "field,value,count,percentage,rank\n".to_string(),
            headers: "name".to_string(),
            ..AnalysisResults::default()
        };

        let (_p, _s, dict_in_desc) = get_prompt(PromptType::Description, Some(&analysis), &args)
            .expect("Description prompt");
        assert!(
            dict_in_desc,
            "default description_prompt now references `{{{{ dictionary }}}}` — the signal MUST \
             be true"
        );

        let (_p, _s, dict_in_tags) =
            get_prompt(PromptType::Tags, Some(&analysis), &args).expect("Tags prompt");
        assert!(
            dict_in_tags,
            "default tags_prompt now references `{{{{ dictionary }}}}` — the signal MUST be true"
        );

        let (_p, _s, dict_in_dict) =
            get_prompt(PromptType::Dictionary, Some(&analysis), &args).expect("Dictionary prompt");
        assert!(
            !dict_in_dict,
            "dictionary_prompt does NOT reference `{{{{ dictionary }}}}` (it would be \
             self-referential) — the signal MUST be false"
        );
    }

    #[test]
    fn prompt_file_without_dictionary_refine_prompt_field_still_parses() {
        // Backward compat: any user --prompt-file TOML authored before --two-pass was
        // introduced will not have a `dictionary_refine_prompt` field. The
        // #[serde(default = "default_dictionary_refine_prompt")] attribute must make
        // serde fall back to the built-in template instead of failing the parse.
        // Build a minimal TOML missing only that field.
        let toml_without_refine = r#"
name = "test"
description = "test"
author = "test"
version = "1.0"
tokens = 1000
system_prompt = "system"
dictionary_prompt = "dictionary"
description_prompt = "description"
tags_prompt = "tags"
prompt = "prompt"
format = "markdown"
language = ""
base_url = "http://localhost"
model = "test-model"
timeout = 30
custom_prompt_guidance = ""
duckdb_sql_guidance = ""
polars_sql_guidance = ""
dd_fewshot_examples = ""
p_fewshot_examples = ""
"#;
        let parsed: Result<PromptFile, _> = toml::from_str(toml_without_refine);
        let pf = parsed.expect(
            "PromptFile must parse even without a `dictionary_refine_prompt` field — pre-existing \
             user prompt files would break without the serde(default)",
        );
        assert!(
            !pf.dictionary_refine_prompt.is_empty(),
            "fallback `dictionary_refine_prompt` must be non-empty so --two-pass works \
             out-of-the-box for users on older prompt files"
        );
        assert!(
            pf.dictionary_refine_prompt
                .contains("first_pass_dictionary"),
            "fallback refine prompt must reference `{{{{ first_pass_dictionary }}}}` — otherwise \
             the refine LLM call never sees the first-pass dictionary"
        );
    }

    #[test]
    fn first_pass_dictionary_json_is_stable_across_invocations() {
        // Regression guard against the cache-busting bug: if
        // `build_first_pass_dictionary_json_string` ever re-introduces attribution
        // expansion (live timestamp / command line) into the first-pass JSON, the
        // DictionaryRefine cache key would change on every invocation and the refine
        // cache would never hit. The string MUST be byte-identical across calls with
        // the same inputs. Sleep a hair between calls so that any latent
        // chrono::Utc::now() in the call chain would observably differ.
        use std::{thread::sleep, time::Duration};
        let args = default_args_for_test();
        let entries = vec![dictionary::DictionaryEntry {
            name:         "f".to_string(),
            r#type:       "String".to_string(),
            label:        "F".to_string(),
            description:  "d".to_string(),
            content_type: String::new(),
            min:          String::new(),
            max:          String::new(),
            cardinality:  10,
            enumeration:  String::new(),
            null_count:   0,
            addl_cols:    indexmap::IndexMap::new(),
            examples:     String::new(),
        }];
        let first = build_first_pass_dictionary_json_string(&args, &entries);
        sleep(Duration::from_millis(10));
        let second = build_first_pass_dictionary_json_string(&args, &entries);
        assert_eq!(
            first, second,
            "first-pass JSON must be byte-identical across runs — any drift busts the \
             DictionaryRefine cache fingerprint and forces a fresh second LLM call every \
             invocation"
        );
        assert!(
            !first.contains("\"attribution\""),
            "first-pass JSON must NOT contain an `attribution` field — that field is for the \
             user-facing emit path and includes a live timestamp / command line that would \
             destabilize the cache fingerprint"
        );
        assert!(
            !first.contains("{GENERATED_BY_SIGNATURE}"),
            "first-pass JSON must NOT leave the unexpanded attribution placeholder either — strip \
             the field outright"
        );
    }

    #[test]
    fn default_dictionary_refine_prompt_matches_resource() {
        // The const DEFAULT_DICTIONARY_REFINE_PROMPT (used as the #[serde(default)] fallback)
        // MUST be byte-identical to the dictionary_refine_prompt block in
        // resources/describegpt_defaults.toml. The TOML is the active default for users
        // who let the defaults load via include_str!; the const is the fallback for users
        // on older --prompt-file TOMLs. Both audiences must see the same template.
        let toml: PromptFile =
            toml::from_str(get_default_prompt_file_content()).expect("default TOML must parse");
        assert_eq!(
            toml.dictionary_refine_prompt,
            default_dictionary_refine_prompt(),
            "DEFAULT_DICTIONARY_REFINE_PROMPT const drifted from the \
             resources/describegpt_defaults.toml `dictionary_refine_prompt` block — keep them in \
             sync (or two cohorts of users will see different refine prompts)"
        );
    }

    /// Minimal Args for unit tests: zero-values everywhere.
    fn default_args_for_test() -> Args {
        Args {
            arg_input:               None,
            flag_dictionary:         false,
            flag_description:        false,
            flag_tags:               false,
            flag_all:                false,
            flag_num_tags:           0,
            flag_tag_vocab:          None,
            flag_cache_dir:          String::new(),
            flag_ckan_api:           String::new(),
            flag_ckan_token:         None,
            flag_stats_options:      String::new(),
            flag_freq_options:       String::new(),
            flag_enum_threshold:     0,
            flag_num_examples:       0,
            flag_truncate_str:       0,
            flag_prompt:             None,
            flag_sql_results:        None,
            flag_prompt_file:        None,
            flag_markdown_template:  None,
            flag_sample_size:        0,
            flag_fewshot_examples:   false,
            flag_base_url:           None,
            flag_model:              None,
            flag_language:           None,
            flag_addl_props:         None,
            flag_api_key:            None,
            flag_max_tokens:         0,
            flag_timeout:            0,
            flag_user_agent:         None,
            flag_export_prompt:      None,
            flag_no_cache:           true,
            flag_disk_cache_dir:     None,
            flag_redis_cache:        false,
            flag_fresh:              false,
            flag_forget:             false,
            flag_flush_cache:        false,
            flag_prepare_context:    false,
            flag_process_response:   false,
            flag_format:             None,
            flag_allow_extra_cols:   false,
            flag_strict_dates:       false,
            flag_output:             None,
            flag_quiet:              false,
            flag_addl_cols:          false,
            flag_addl_cols_list:     None,
            flag_infer_content_type: false,
            flag_two_pass:           false,
            flag_session:            None,
            flag_session_len:        0,
            flag_no_score_sql:       false,
            flag_score_threshold:    0,
            flag_score_max_retries:  0,
        }
    }

    /// Verifies that the default `describegpt_md_defaults.toml` produces byte-identical
    /// output to the legacy hardcoded `format!("# {}\n{}\n## REASONING\n\n{}\n## TOKEN \
    /// USAGE\n\n{:?}\n---\n", ...)` wrapper for every PromptType. If this test breaks,
    /// the default template was edited in a way that changes legacy output — either
    /// fix the template or bump the major version and document the change.
    #[test]
    fn markdown_default_template_byte_identical_to_legacy() {
        let mut args = default_args_for_test();
        // get_prompt_file (called transitively via replace_attribution_placeholder)
        // expects these to be populated as docopt would at runtime.
        args.flag_base_url = Some(DEFAULT_BASE_URL.to_string());
        args.flag_model = Some(DEFAULT_MODEL.to_string());
        let response_body = "Hello, *world*.";
        let reasoning = "Some reasoning here.";
        let token_usage = TokenUsage {
            prompt:     12,
            completion: 34,
            total:      46,
            elapsed:    789,
        };
        let model = "openai/gpt-oss-20b";
        let base_url = "http://localhost:11434/v1";

        for kind in [
            PromptType::Dictionary,
            PromptType::Description,
            PromptType::Tags,
            PromptType::Prompt,
        ] {
            let legacy = format!(
                "# {}\n{}\n## REASONING\n\n{}\n## TOKEN USAGE\n\n{:?}\n---\n",
                kind, response_body, reasoning, token_usage
            );
            let shared = SharedRenderCtx::new(&args, model, base_url, kind);
            let rendered = render_markdown_template(
                kind,
                &args,
                response_body,
                reasoning,
                &token_usage,
                model,
                base_url,
                &shared,
            )
            .unwrap();
            assert_eq!(
                rendered, legacy,
                "default markdown template diverged from legacy output for kind={kind:?}"
            );
            // Reset cached template between iterations is unnecessary because the same
            // default templates are used for all kinds; OnceLock is set once and reused.
        }
    }

    #[test]
    fn dictionary_body_default_template_byte_identical_to_legacy() {
        use indexmap::IndexMap;

        let mut args = default_args_for_test();
        args.flag_base_url = Some(DEFAULT_BASE_URL.to_string());
        args.flag_model = Some(DEFAULT_MODEL.to_string());
        let model = "openai/gpt-oss-20b";
        let base_url = "http://localhost:11434/v1";

        // Fixture: two entries, with a "percentiles" addl_col and a regular "mean" addl_col.
        let mut addl1 = IndexMap::new();
        addl1.insert("mean".to_string(), "12.5".to_string());
        addl1.insert(
            "percentiles".to_string(),
            "p25: 10\np50: 12\np75: 15".to_string(),
        );
        let mut addl2 = IndexMap::new();
        addl2.insert("mean".to_string(), "0".to_string());
        addl2.insert("percentiles".to_string(), String::new());

        let entries = vec![
            dictionary::DictionaryEntry {
                name:         "id".to_string(),
                r#type:       "Integer".to_string(),
                label:        "ID".to_string(),
                description:  "Unique identifier\nfor the row.".to_string(),
                content_type: String::new(),
                min:          "1".to_string(),
                max:          "1000".to_string(),
                cardinality:  1000,
                enumeration:  String::new(),
                null_count:   0,
                addl_cols:    addl1,
                examples:     "<ALL_UNIQUE>".to_string(),
            },
            dictionary::DictionaryEntry {
                name:         "category|raw".to_string(),
                r#type:       "String".to_string(),
                label:        "Category".to_string(),
                description:  "Top-level grouping.".to_string(),
                content_type: String::new(),
                min:          String::new(),
                max:          String::new(),
                cardinality:  3,
                enumeration:  "alpha\nbeta\ngamma".to_string(),
                null_count:   12_345,
                addl_cols:    addl2,
                examples:     "alpha [9000]\nbeta [1234]".to_string(),
            },
        ];

        // Build a single SharedRenderCtx (one attribution + timestamp). Threaded into
        // render_dictionary_md_body AND used to construct the expected string, so
        // byte-equality holds without timestamp masking.
        let shared = SharedRenderCtx::new(&args, model, base_url, PromptType::Dictionary);

        // Hand-rolled expected output that mirrors the legacy table format precisely:
        //   - pipe in "category|raw" escaped to "\|"
        //   - newlines in description and enumeration become "<br>"
        //   - cardinality and null_count humanized via thousands separator
        //   - percentiles column collapses pipes AND newlines to "<br>"
        //   - "<ALL_UNIQUE>" passes through unchanged
        //   - "alpha [9000]" gets count humanized to "alpha [9,000]"
        //   - row-count separator pieces match column count
        //   - footer is "*Attribution: <rendered attribution block>*"
        let expected = format!(
            "| Name | Type | Label | Description | Min | Max | Cardinality | Enumeration | Null \
             Count | mean | percentiles | Examples |\n|------|------|-------|-------------|-----|-\
             ----|-------------|-------------|------------|----------|----------|----------|\n| \
             **id** | Integer | ID | Unique identifier<br>for the row. | 1 | 1000 | 1,000 |  | 0 \
             | 12.5 | p25: 10<br>p50: 12<br>p75: 15 | <ALL_UNIQUE> |\n| **category\\|raw** | \
             String | Category | Top-level grouping. |  |  | 3 | alpha<br>beta<br>gamma | 12,345 \
             | 0 |  | alpha [9,000]<br>beta [1,234] |\n\n*Attribution: {}*\n",
            shared.attribution,
        );

        let addl_col_names = formatters::extract_ordered_addl_cols(&entries);
        let rendered =
            render_dictionary_md_body(&args, &entries, &addl_col_names, model, base_url, &shared)
                .unwrap();

        assert_eq!(
            rendered, expected,
            "default dictionary_md_body_template diverged from legacy table output"
        );
    }

    #[test]
    fn dictionary_body_template_with_content_type() {
        use indexmap::IndexMap;

        let mut args = default_args_for_test();
        args.flag_base_url = Some(DEFAULT_BASE_URL.to_string());
        args.flag_model = Some(DEFAULT_MODEL.to_string());
        args.flag_infer_content_type = true;
        let model = "openai/gpt-oss-20b";
        let base_url = "http://localhost:11434/v1";

        let mut addl = IndexMap::new();
        addl.insert("mean".to_string(), "12.5".to_string());

        let entries = vec![
            dictionary::DictionaryEntry {
                name:         "id".to_string(),
                r#type:       "Integer".to_string(),
                label:        "ID".to_string(),
                description:  "Unique identifier\nfor the row.".to_string(),
                content_type: "uuid".to_string(),
                min:          "1".to_string(),
                max:          "1000".to_string(),
                cardinality:  1000,
                enumeration:  String::new(),
                null_count:   0,
                addl_cols:    addl.clone(),
                examples:     "<ALL_UNIQUE>".to_string(),
            },
            dictionary::DictionaryEntry {
                name:         "category".to_string(),
                r#type:       "String".to_string(),
                label:        "Category".to_string(),
                description:  "Top-level grouping.".to_string(),
                content_type: "category".to_string(),
                min:          String::new(),
                max:          String::new(),
                cardinality:  3,
                enumeration:  "alpha\nbeta\ngamma".to_string(),
                null_count:   12_345,
                addl_cols:    addl,
                examples:     "alpha [9000]\nbeta [1234]".to_string(),
            },
        ];

        let shared = SharedRenderCtx::new(&args, model, base_url, PromptType::Dictionary);
        let addl_col_names = formatters::extract_ordered_addl_cols(&entries);
        let rendered =
            render_dictionary_md_body(&args, &entries, &addl_col_names, model, base_url, &shared)
                .unwrap();

        // The header and separator gain a "Content Type" column right after "Description".
        assert!(
            rendered.contains("| Description | Content Type | Min |"),
            "header missing Content Type column:\n{rendered}"
        );
        assert!(
            rendered.contains("|-------------|--------------|-----|"),
            "separator missing Content Type column:\n{rendered}"
        );
        // Each data row carries its content_type cell between description and min.
        assert!(
            rendered.contains("Unique identifier<br>for the row. | uuid | 1 |"),
            "row 1 missing content_type cell:\n{rendered}"
        );
        assert!(
            rendered.contains("Top-level grouping. | category |  |"),
            "row 2 missing content_type cell:\n{rendered}"
        );
    }

    #[test]
    fn dictionary_prompt_gates_content_type_on_flag() {
        // The dictionary_prompt template interleaves {% raw %}/{% endraw %} with
        // {% if infer_content_type %} blocks; render it both ways to lock in that the
        // Content Type instruction, vocabulary, and JSON example key are gated correctly.
        let prompt_file: PromptFile = toml::from_str(get_default_prompt_file_content()).unwrap();

        let mut env = Environment::new();
        minijinja_contrib::add_to_environment(&mut env);

        let render = |infer: bool| {
            env.render_str(
                &prompt_file.dictionary_prompt,
                context! {
                    stats => "field,type\nid,Integer\n",
                    frequency => "field,value,count,percentage,rank\n",
                    headers => "id,email",
                    language => "",
                    infer_content_type => infer,
                    content_type_vocab => dictionary::content_type_vocab_list(),
                },
            )
            .unwrap()
        };

        let off = render(false);
        assert!(
            !off.contains("Content Type:"),
            "flag-off prompt must not mention the Content Type instruction:\n{off}"
        );
        assert!(
            !off.contains("content_type"),
            "flag-off prompt must not include the content_type JSON key:\n{off}"
        );
        assert!(
            off.contains("\"label\" and \"description\" properties"),
            "flag-off prompt must keep the legacy properties sentence:\n{off}"
        );

        let on = render(true);
        assert!(
            on.contains("- Content Type: classify the SEMANTIC CONTENT"),
            "flag-on prompt must include the Content Type instruction:\n{on}"
        );
        assert!(
            on.contains("Allowed Content Type tokens: first_name"),
            "flag-on prompt must inject the curated vocabulary:\n{on}"
        );
        assert!(
            on.contains("\"content_type\": \"email\""),
            "flag-on prompt must include the content_type JSON example key:\n{on}"
        );
        assert!(
            on.contains("\"label\", \"description\" and \"content_type\" properties"),
            "flag-on prompt must list content_type in the properties sentence:\n{on}"
        );
    }

    /// Locks in that the four default wrapper templates are byte-identical to each other.
    /// They render the same `# {kind}` header and the same REASONING / TOKEN USAGE sections,
    /// only differing on the `{{ kind }}` substitution. If a future tweak edits one of them
    /// without updating the others, this test catches the drift before it ships.
    #[test]
    fn default_wrapper_templates_are_byte_identical() {
        // Read the embedded default TOML directly so this test is independent of
        // get_md_template_file's OnceLock cache and any test-ordering effects.
        let toml_text = get_default_md_template_content();
        let parsed: MarkdownTemplateFile = toml::from_str(toml_text).unwrap();

        assert_eq!(
            parsed.dictionary_md_template, parsed.description_md_template,
            "dictionary and description wrapper templates diverged"
        );
        assert_eq!(
            parsed.description_md_template, parsed.tags_md_template,
            "description and tags wrapper templates diverged"
        );
        assert_eq!(
            parsed.tags_md_template, parsed.custom_prompt_md_template,
            "tags and custom_prompt wrapper templates diverged"
        );
    }

    /// Verifies the partial-override fallback in MarkdownTemplateOverride::apply_to:
    /// any field a user TOML omits falls back to the embedded default, so a minimal
    /// override (one template field set, all others omitted) Just Works.
    #[test]
    fn markdown_template_override_falls_back_per_field() {
        let base: MarkdownTemplateFile = toml::from_str(get_default_md_template_content()).unwrap();
        let base_dict_md = base.dictionary_md_template.clone();
        let base_tags_md = base.tags_md_template.clone();
        let base_dict_body = base.dictionary_md_body_template.clone();
        let base_custom_md = base.custom_prompt_md_template.clone();

        // Minimal user TOML: override only `description_md_template`.
        let user_toml = r#"
description_md_template = "OVERRIDDEN: {{ llm_response }}"
"#;
        let overlay: MarkdownTemplateOverride = toml::from_str(user_toml).unwrap();
        let merged = overlay.apply_to(base);

        assert_eq!(
            merged.description_md_template,
            "OVERRIDDEN: {{ llm_response }}"
        );
        // All other template fields keep the embedded default value.
        assert_eq!(merged.dictionary_md_template, base_dict_md);
        assert_eq!(merged.tags_md_template, base_tags_md);
        assert_eq!(merged.dictionary_md_body_template, base_dict_body);
        assert_eq!(merged.custom_prompt_md_template, base_custom_md);
    }
}
