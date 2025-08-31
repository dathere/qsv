static USAGE: &str = r#"
Infer a Data Dictionary, Description & Tags about a Dataset using any OpenAI API-compatible
Large Language Model (LLM).

It infers these extended metadata by compiling Summary Statistics & a Frequency Distribution
of the Dataset, and then prompting the LLM with detailed, configurable prompts with these
extended context.

You can also use the --prompt option to ask a natural language question about the Dataset.

If the question cannot be answered using the Dataset's Summary Statistics & Frequency Distribution,
it will auto-infer a Data Dictionary & provide it to the LLM as additional context to create a
SQL query that DETERMINISTICALLY answers the natural language question ("SQL RAG" mode).

SQL RAG MODE:
Two SQL dialects are currently supported - DuckDB (recommended) & Polars. If the
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

OpenAI's open-weights gpt-oss-20b model was used during development & is recommended for most use cases.

NOTE: LLMs are prone to inaccurate information being produced. Verify output results before using them.

CACHING:
As LLM inferecing takes time and can be expensive, describegpt caches the LLM inferencing results
in a either a disk cache (default) or a Redis cache. It does so by calculating the SHA256 hash of the
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

  # Generate a Data Dictionary of data.csv using the DeepSeek R1:14b model
  $ qsv describegpt data.csv -u http://localhost:11434/v1 --model deepseek-r1:14b --dictionary

  # Ask questions about the sample NYC 311 dataset using LM Studio with the default gpt-oss-20b model.
  # Questions that can be answered using the Summary Statistics & Frequency Distribution of the dataset.
  $ export QSV_LLM_BASE_URL=http://localhost:1234/v1
  $ qsv describegpt NYC_311.csv --prompt "What is the most common complaint?"
  $ qsv describegpt NYC_311.csv --prompt "List the top 10 complaints."
  $ qsv describegpt NYC_311.csv -p "Can you tell me how many complaints were resolved?"

  # Ask detailed questions that require SQL queries and auto-invoke SQL RAG mode
  # use DuckDB to answer the question
  $ export QSV_DESCRIBEGPT_DB_ENGINE=/path/to/duckdb
  $ qsv describegpt NYC_311.csv -p "What's the breakdown of complaint types by borough descending order?"

  # Cache Dictionary, Description & Tags inference results using the Redis cache instead of the disk cache
  $ qsv describegpt data.csv --all --redis-cache
  # Get fresh Description & Tags inference results from the LLM and refresh disk cache entries for both
  $ qsv describegpt data.csv --description --tags --fresh
  # Get fresh inference results from the LLM and refresh the Redis cache entries for all three
  $ qsv describegpt data.csv --all --redis-cache --fresh

  # Flush the disk cache
  $ qsv describegpt --flush-cache
  # Flush the Redis cache
  $ qsv describegpt --redis-cache --flush-cache

For more examples, see https://github.com/dathere/qsv/blob/master/tests/test_describegpt.rs.

For more detailed info on how describegpt works and how to prepare a prompt file,
see https://github.com/dathere/qsv/blob/master/docs/Describegpt.md

Usage:
    qsv describegpt [options] [<input>]
    qsv describegpt (--redis-cache) (--flush-cache)
    qsv describegpt --help

describegpt options:
                           DATA ANALYSIS/INFERENCING OPTIONS:
    --dictionary           Infer a Data Dictionary. For each field, prints an inferred type,
                           a human-readable label and a description.
    --description          Infer a general Description of the dataset.
    --tags                 Infer Tags that categorize the dataset. Useful
                           for grouping datasets and filtering.
    -A, --all              Shortcut for --dictionary --description --tags.
    --stats-options <arg>  Options for the stats command used to generate summary statistics.
                           [default: --infer-dates --everything]
    --enum-threshold <n>   The threshold for compiling enumerations with the frequency command
                           before bucketing other unique values into the "Other" category.
                           [default: 20]

                           CUSTOM PROMPT OPTIONS:
    -p, --prompt <prompt>  Custom prompt to answer questions about the dataset.
                           The prompt will be answered based on the dataset's Summary Statistics,
                           Frequency data & Data Dictionary. If the prompt CANNOT be answered by looking
                           at these metadata, a SQL query will be generated to answer the question.
                           If the "polars" or the "QSV_DESCRIBEGPT_DB_ENGINE" environment variable is set
                           & the `--sql-results` option is used, the SQL query will be automatically
                           executed and its results returned.
                           Otherwise, only the SQL query will be returned.
    --sql-results <csv>    The CSV to save the SQL query results to.
                           Only valid if the --prompt option is used & the "polars" or the
                           "QSV_DESCRIBEGPT_DB_ENGINE" environment variable is set.
    --prompt-file <file>   The JSON file containing prompts to use for inferencing.
                           If no prompt file is provided, default prompts will be used.

                           LLM API OPTIONS:
    -u, --base-url <url>   The LLM API URL. Supports APIs & local LLMs compatible with
                           the OpenAI API specification (Ollama, Jan, LM Studio, etc.).
                           The default base URL for Ollama is http://localhost:11434/v1.
                           The default for Jan is https://localhost:1337/v1.
                           The default for LM Studio is http://localhost:1234/v1.
                           The base URL will be the base URL of the prompt file.
                           If the QSV_LLM_BASE_URL environment variable is set, it will be
                           used instead.
                           [default: https://api.openai.com/v1]
    -m, --model <model>    The model to use for inferencing.
                           If the QSV_LLM_MODEL environment variable is set, it will be
                           used instead.
                           [default: gpt-oss-20b]
    --addl-props <json>    Additional model properties to pass to the LLM chat/completion API.
                           Various models support different properties beyond the standard ones.
                           For instance, gpt-oss-20b supports the "reasoning_effort" property.
                           e.g. to set the "reasoning_effort" property to "high" & "temperature"
                           to 0.5, use '{"reasoning_effort": "high", "temperature": 0.5}'
    -k, --api-key <key>    The API key to use. If the QSV_LLM_APIKEY envvar is set,
                           it will be used instead. Required when the base URL is not localhost.
    -t, --max-tokens <n>   Limits the number of generated tokens in the output.
                           Set to 0 to disable token limits.
                           If the --base-url is localhost, indicating a local LLM,
                           the default is automatically set to 0.
                           [default: 2000]
    --timeout <secs>       Timeout for completions in seconds. If 0, no timeout is used.
                           [default: 600]
    --user-agent <agent>   Specify custom user agent. It supports the following variables -
                           $QSV_VERSION, $QSV_TARGET, $QSV_BIN_NAME, $QSV_KIND and $QSV_COMMAND.
                           Try to follow the syntax here -
                           https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/User-Agent

    --json                 Return results in JSON format.
    --jsonl                Return results in JSON Lines format.

                             CACHING OPTIONS:
    --no-cache               Disable default disk cache.
    --disk-cache-dir <dir>   The directory <dir> to store the disk cache. Note that if the directory
                             does not exist, it will be created. If the directory exists, it will be used as is,
                             and will not be flushed. This option allows you to maintain several disk caches
                             for different describegpt jobs (e.g. one for a data portal, another for internal
                             data exchange, etc.)
                             [default: ~/.qsv/cache/describegpt]
    --redis-cache            Use Redis instead of the default disk cache to cache LLM completions.
                             It connects to "redis://127.0.0.1:6379/3" by default, with a connection pool
                             size of 20, with a TTL of 28 days, and cache hits NOT refreshing an existing
                             cached value's TTL.
                             This option automatically disables the disk cache.
    --fresh                  Send a fresh request to the LLM API, refreshing a cached response if it exists.
                             When a --prompt SQL query fails, you can also use this option to request the
                             LLM to generate a new SQL query.
    --forget                 Remove a cached response if it exists and then exit.
    --flush-cache            Flush the current cache entries on startup.
                             WARNING: This operation is irreversible.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -q, --quiet            Do not print status messages to stderr.
"#;

use std::{
    env, fs,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
    sync::{
        OnceLock,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};

use cached::{
    DiskCache, IOCached, RedisCache, Return, proc_macro::io_cached, stores::DiskCacheBuilder,
};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use strum_macros::{Display, EnumString};

use crate::{CliError, CliResult, config::Config, regex_oncelock, util, util::process_input};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, Display)]
#[strum(ascii_case_insensitive)]
enum PromptType {
    Dictionary,
    Description,
    Tags,
    Prompt,
}
#[derive(Debug, Deserialize)]
struct Args {
    arg_input:           Option<String>,
    flag_dictionary:     bool,
    flag_description:    bool,
    flag_tags:           bool,
    flag_all:            bool,
    flag_stats_options:  String,
    flag_enum_threshold: usize,
    flag_prompt:         Option<String>,
    flag_sql_results:    Option<String>,
    flag_prompt_file:    Option<String>,
    flag_base_url:       Option<String>,
    flag_model:          Option<String>,
    flag_addl_props:     Option<String>,
    flag_api_key:        Option<String>,
    flag_max_tokens:     u32,
    flag_timeout:        u16,
    flag_user_agent:     Option<String>,
    flag_json:           bool,
    flag_jsonl:          bool,
    flag_no_cache:       bool,
    flag_disk_cache_dir: Option<String>,
    flag_redis_cache:    bool,
    flag_fresh:          bool,
    flag_forget:         bool,
    flag_flush_cache:    bool,
    flag_output:         Option<String>,
    flag_quiet:          bool,
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
    json:                   bool,
    jsonl:                  bool,
    base_url:               String,
    model:                  String,
    timeout:                u32,
    custom_prompt_guidance: String,
}

const LLM_APIKEY_ERROR: &str = r#"Error: Neither QSV_LLM_BASE_URL nor QSV_LLM_APIKEY environment variables are set.
Either set the URL to an address with "localhost" in it (indicating a local LLM), or set the API key.

Note that this command uses LLMs for inferencing and is therefore prone to inaccurate information being produced.
Verify output results before using them."#;

const INPUT_TABLE_NAME: &str = "<INPUT_TABLE_NAME>";

// Global variable to store DuckDB binary path
static DUCKDB_PATH: OnceLock<String> = OnceLock::new();

static DATA_DICTIONARY_JSON: OnceLock<String> = OnceLock::new();
#[allow(dead_code)]
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
struct TokenUsage {
    prompt:     u64,
    completion: u64,
    total:      u64,
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

static QSV_REDIS_CONNSTR_ENV: &str = "QSV_DG_REDIS_CONNSTR";
static QSV_REDIS_MAX_POOL_SIZE_ENV: &str = "QSV_REDIS_MAX_POOL_SIZE";
static QSV_REDIS_TTL_SECS_ENV: &str = "QSV_REDIS_TTL_SECS";
static QSV_REDIS_TTL_REFRESH_ENV: &str = "QSV_REDIS_TTL_REFRESH";
static QSV_DESCRIBEGPT_DB_ENGINE_ENV: &str = "QSV_DESCRIBEGPT_DB_ENGINE";
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

static QUIET_FLAG: AtomicBool = AtomicBool::new(false);
static QSV_PATH: OnceLock<String> = OnceLock::new();
static FILE_HASH: OnceLock<String> = OnceLock::new();
static PROMPT_FILE: OnceLock<PromptFile> = OnceLock::new();
static PROMPT_VALIDITY_FLAGS: std::sync::LazyLock<
    std::sync::Mutex<std::collections::HashMap<String, String>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(std::collections::HashMap::new()));

fn print_status(msg: &str, elapsed: Option<std::time::Duration>) {
    if !QUIET_FLAG.load(Ordering::Relaxed) {
        if let Some(duration) = elapsed {
            eprintln!("{msg} (elapsed: {:.2}s)", duration.as_secs_f64());
        } else {
            eprintln!("{msg}");
        }
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

// Send an HTTP request using a client to a URL
// Optionally include an API key and request data
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
    if let Some(key) = api_key {
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

/// Check if model is valid, including the default model
/// If the model is not valid, try to find a valid model that matches the end of the given model
/// If no valid model is found, fail with list of valid models
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
    let Some(models) = response_json["data"].as_array() else {
        return fail_clierror!(
            "Invalid response: 'data' field is not an array or is missing\n\n{}",
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

fn get_prompt_file(args: &Args) -> CliResult<&PromptFile> {
    if let Some(prompt_file) = PROMPT_FILE.get() {
        Ok(prompt_file)
    } else {
        let prompt_file_content = if args.flag_prompt_file.is_none() {
            // If no prompt file is provided, use the default prompt file
            let default_prompt_file = include_str!("../../resources/describegpt_defaults.json");
            default_prompt_file
        } else {
            let prompt_file_path = args.flag_prompt_file.as_ref().unwrap();
            &fs::read_to_string(prompt_file_path)?
        };

        // Try to parse prompt file as JSON, if error then show it in JSON format
        let mut prompt_file: PromptFile = match serde_json::from_str(prompt_file_content) {
            Ok(val) => val,
            Err(e) => {
                let error_json = json!({"error": e.to_string()});
                return fail_clierror!("{error_json}");
            },
        };

        // If QSV_LLM_BASE_URL environment variable is set, use it as the base URL
        if let Ok(base_url) = env::var("QSV_LLM_BASE_URL") {
            prompt_file.base_url = base_url;
        }

        let model_to_use = env::var("QSV_LLM_MODEL")
            .ok()
            .or_else(|| args.flag_model.clone())
            .or_else(|| {
                args.flag_prompt_file
                    .as_ref()
                    .map(|_| prompt_file.model.clone())
            })
            // safety: model has a docopt default
            .unwrap();

        prompt_file.model = model_to_use;

        // If max_tokens is 0 or the base URL contains "localhost", always disable the limit
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

// Generate prompt for prompt type based on either the prompt file (if given) or default prompts
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
            return Err(anyhow::anyhow!("Missing analysis results required for prompt generation."));
        }
    };

    // If this is a custom prompt and DuckDB should be used, modify the SQL query generation
    // guidelines
    if prompt_type == PromptType::Prompt && should_use_duckdb() {
        // Look for the SQL query generation guidelines section & replace it with DuckDB guidance
        let sql_guidelines_start = "SQL Query Generation Guidelines:\n\n";
        let sql_guidelines_end = "\nEND SQL Query Generation Guidelines\n";

        if let Some(start_pos) = prompt.find(sql_guidelines_start)
            && let Some(end_pos) = prompt[start_pos..].find(sql_guidelines_end)
        {
            let end_pos = start_pos + end_pos;
            let before_guidelines = &prompt[..start_pos];
            let after_guidelines = &prompt[end_pos..];

            let duckdb_guidelines = format!(
                r#"
- Use DuckDB v1.0 syntax
- The input csv has headers and uses {delimiter} as the delimiter
- Only use the `read_csv_auto` table function to read the input CSV
- Use the placeholder {INPUT_TABLE_NAME} for the input csv in the `read_csv_auto` table function call
- Only use functions from the Loaded DuckDB extensions listed below
- Make sure the generated SQL query is valid and has comments to explain the query
- Add a comment with the placeholder "GENERATED_BY_SIGNATURE" at the top of the query"#
            );

            prompt = format!(
                "{before_guidelines}{sql_guidelines_start}{duckdb_guidelines}{after_guidelines}"
            );

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
            // add the valid extensions to the prompt
            prompt = format!("{prompt}\n\nLoaded DuckDB extensions: {valid_extensions}");
        }
        log::debug!("prompt using DuckDB: {prompt}");
    }

    // Replace variable data in prompt
    #[allow(clippy::to_string_in_format_args)]
    #[allow(clippy::literal_string_with_formatting_args)]
    let prompt = prompt
        .replace("{stats}", &stats)
        .replace("{frequency}", &frequency)
        .replace("{headers}", &headers)
        .replace(
            "{dictionary}",
            DATA_DICTIONARY_JSON.get().map_or("", |s| s.as_str()),
        )
        .replace(
            "{json_add}",
            if prompt_file.json || prompt_file.jsonl || args.flag_json || args.flag_jsonl {
                " (in valid JSON format. Surround it with ```json and ```)"
            } else {
                ""
            },
        );

    // Return prompt
    Ok((prompt, prompt_file.system_prompt.clone()))
}

fn get_completion(
    args: &Args,
    client: &Client,
    model: &str,
    api_key: &str,
    messages: &serde_json::Value,
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
        for (key, value) in addl_props_json.as_object().unwrap() {
            request_data[key] = value.clone();
        }
    }

    // deserializing request_data is relatively expensive, so only do it if debug is enabled
    if log::log_enabled!(log::Level::Debug) {
        log::debug!("Request data: {request_data:?}");
    }

    // Get response from POST request to chat completions endpoint
    let completions_endpoint = "/chat/completions";
    let response = send_request(
        client,
        Some(api_key),
        Some(&request_data),
        "POST",
        &format!("{base_url}{completions_endpoint}"),
    )?;

    // Parse response as JSON
    let response_json: serde_json::Value = response.json()?;
    if log::log_enabled!(log::Level::Debug) {
        log::debug!("Response: {response_json:?}");
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
    let token_usage = TokenUsage {
        prompt:     usage["prompt_tokens"].as_u64().unwrap_or(0),
        completion: usage["completion_tokens"].as_u64().unwrap_or(0),
        total:      usage["total_tokens"].as_u64().unwrap_or(0),
    };

    // Replace "GENERATED_BY_SIGNATURE" with the model name and the date and time the query was
    // generated
    let completion = completion.replace(
        "GENERATED_BY_SIGNATURE",
        &format!(
            "Generated by qsv's describegpt command using {model} on {}",
            chrono::Utc::now().to_rfc3339()
        ),
    );

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
        "{:?}{:?}{:?}{:?}{:?}{:?}{:?}{:?}",
        file_hash,
        args.flag_prompt_file,
        prompt_content,
        args.flag_max_tokens,
        args.flag_addl_props,
        actual_model,
        kind,
        validity_flag
    )
}

fn get_analysis_cache_key(args: &Args, file_hash: &str) -> String {
    format!(
        "analysis_{:?}{:?}{:?}",
        file_hash, args.flag_stats_options, args.flag_enum_threshold,
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
    // safety: we just checked that the cache is initialized
    GET_DISKCACHE_COMPLETION.connection().flush().unwrap();

    removed
}

// this is a disk cache that can be used across qsv sessions
#[io_cached(
    disk = true,
    ty = "cached::DiskCache<String, CompletionResponse>",
    cache_prefix_block = r##"{ "descdc_" }"##,
    key = "String",
    convert = r##"{ get_cache_key(args, kind, model) }"##,
    create = r##"{
        let cache_dir = DISKCACHE_DIR.get().unwrap();
        let diskcache_config = DISKCACHECONFIG.get().unwrap();
        let diskcache: DiskCache<String, CompletionResponse> = DiskCacheBuilder::new("describegpt")
            .set_disk_directory(cache_dir)
            .set_lifespan(diskcache_config.ttl_secs)
            .set_refresh(diskcache_config.ttl_refresh)
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
        args, client, model, api_key, messages,
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
        args, client, model, api_key, messages,
    )?))
}

// Cached analysis results for disk cache
#[io_cached(
    disk = true,
    ty = "cached::DiskCache<String, AnalysisResults>",
    cache_prefix_block = r##"{ "desc_analysis_dc_" }"##,
    key = "String",
    convert = r##"{ get_analysis_cache_key(args, file_hash) }"##,
    create = r##"{
        let cache_dir = DISKCACHE_DIR.get().unwrap();
        let diskcache_config = DISKCACHECONFIG.get().unwrap();
        let diskcache: DiskCache<String, AnalysisResults> = DiskCacheBuilder::new("describegpt_analysis")
            .set_disk_directory(cache_dir)
            .set_lifespan(diskcache_config.ttl_secs)
            .set_refresh(diskcache_config.ttl_refresh)
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

// Check if JSON output is expected
fn is_json_output(args: &Args) -> CliResult<bool> {
    // Command-line flags take precedence over prompt file settings
    if args.flag_json {
        return Ok(true);
    }
    if args.flag_jsonl {
        return Ok(false);
    }
    // If no command-line flags, check prompt file
    let prompt_file = get_prompt_file(args)?;
    Ok(prompt_file.json)
}
// Check if JSONL output is expected
fn is_jsonl_output(args: &Args) -> CliResult<bool> {
    // Command-line flags take precedence over prompt file settings
    if args.flag_jsonl {
        return Ok(true);
    }
    if args.flag_json {
        return Ok(false);
    }
    // If no command-line flags, check prompt file
    let prompt_file = get_prompt_file(args)?;
    Ok(prompt_file.jsonl)
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
            let fresh_result = get_completion(args, client, model, api_key, messages)?;
            // Manually update the appropriate cache with the fresh result
            if args.flag_redis_cache {
                let _ = get_redis_completion(args, client, model, api_key, kind, messages);
            } else {
                let _ = get_diskcache_completion(args, client, model, api_key, kind, messages);
            }
            Ok(fresh_result)
        },
        CacheType::None => get_completion(args, client, model, api_key, messages),
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
    ) -> serde_json::Value {
        if dictionary_completion.is_empty() {
            json!([{"role": "system", "content": system_prompt},
            {"role": "user", "content": prompt}])
        } else {
            json!([{"role": "system", "content": system_prompt},
            {"role": "assistant",
            "content": format!("The following is the Data Dictionary for the Dataset:\n\n{dictionary_completion}")},
            {"role": "user", "content": prompt},
            ])
        }
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

    // Helper function to extract JSON from various LLM response formats
    fn extract_json_from_output(output: &str) -> CliResult<serde_json::Value> {
        // Helper function to validate and return JSON candidate
        fn validate_json_candidate(candidate: &str) -> Option<serde_json::Value> {
            serde_json::from_str::<serde_json::Value>(candidate.trim()).ok()
        }

        // Pattern 1: JSON wrapped in ```json and ``` blocks
        if let Some(caps) = regex_oncelock!(r"(?s)```json\n(.*?)\n```").captures(output)
            && let Some(m) = caps.get(1)
            && let Some(valid_json) = validate_json_candidate(m.as_str())
        {
            return Ok(valid_json);
        }

        // Pattern 2: JSON wrapped in ``` and ``` blocks (without json specifier)
        if let Some(caps) = regex_oncelock!(r"(?s)```\n(.*?)\n```").captures(output)
            && let Some(m) = caps.get(1)
            && let Some(valid_json) = validate_json_candidate(m.as_str())
        {
            return Ok(valid_json);
        }

        // Pattern 3: Try to find JSON array or object at the start of the response
        if let Some(caps) = regex_oncelock!(r"(?s)^\s*(\[.*?\]|\{.*?\})").captures(output)
            && let Some(m) = caps.get(1)
            && let Some(valid_json) = validate_json_candidate(m.as_str())
        {
            return Ok(valid_json);
        }

        // Pattern 4: Try to find JSON array or object anywhere in the response (non-greedy)
        if let Some(caps) = regex_oncelock!(r"(?s)(\[.*?\]|\{.*?\})").captures(output)
            && let Some(m) = caps.get(1)
            && let Some(valid_json) = validate_json_candidate(m.as_str())
        {
            return Ok(valid_json);
        }

        // If no pattern matches, return the entire output (might be raw JSON)
        if (output.trim().starts_with('[') || output.trim().starts_with('{'))
            && let Some(valid_json) = validate_json_candidate(output)
        {
            return Ok(valid_json);
        }

        fail_clierror!(
            "Failed to extract JSON content from LLM response. Output: {}",
            if output.is_empty() { "<empty>" } else { output }
        )
    }

    // Generate the plaintext and/or JSON output of an inference option
    fn process_output(
        kind: PromptType,
        completion_response: &CompletionResponse,
        total_json_output: &mut serde_json::Value,
        args: &Args,
    ) -> CliResult<()> {
        // Skip outputting dictionary when using --prompt (but still generate it for context)
        if kind == PromptType::Dictionary && args.flag_prompt.is_some() {
            // Still store the dictionary in DATA_DICTIONARY_JSON for context, but don't output it
            DATA_DICTIONARY_JSON.get_or_init(|| completion_response.response.clone());
            // Don't add to total_json_output and don't output anything
            return Ok(());
        }

        // Check if this is a custom prompt response that contains SQL code
        let is_sql_response = kind == PromptType::Prompt
            && args.flag_sql_results.is_some()
            && completion_response.response.contains("```sql");

        // Process JSON output if expected or JSONL output is expected
        if (is_json_output(args)? || is_jsonl_output(args)?) && !is_sql_response {
            total_json_output[kind.to_string()] = if kind == PromptType::Description
                || kind == PromptType::Prompt
            {
                // For description and prompt, create an object with both response and reasoning
                json!({
                    "response": completion_response.response,
                    "reasoning": completion_response.reasoning
                })
            } else {
                // For dictionary and tags, try to extract JSON from response, but include reasoning
                match extract_json_from_output(&completion_response.response) {
                    Ok(json_value) => {
                        // Create a structured object with data and reasoning
                        json!({
                            "response": json_value,
                            "reasoning": completion_response.reasoning
                        })
                    },
                    Err(_) => {
                        // Fall back to string format with reasoning
                        json!({
                            "response": completion_response.response,
                            "reasoning": completion_response.reasoning
                        })
                    },
                }
            };
            if kind == PromptType::Dictionary {
                DATA_DICTIONARY_JSON.get_or_init(|| {
                    serde_json::to_string_pretty(&total_json_output["dictionary"]["response"])
                        .unwrap()
                });
            }
        }
        // Process plaintext output
        else {
            let mut formatted_output = format_output(&completion_response.response);
            if kind == PromptType::Prompt && is_sql_response {
                // replace INPUT_TABLE_NAME with input_path
                formatted_output = {
                    let input_path = args.arg_input.as_deref().unwrap_or("input.csv");
                    let re = regex_oncelock!("read_csv_auto\\([^)]*\\)");
                    if re.is_match(&formatted_output) {
                        // DuckDB with read_csv_auto so replace with quoted path
                        re.replace_all(
                            &formatted_output,
                            format!("read_csv_auto('{}')", input_path),
                        )
                        .into_owned()
                    } else {
                        // Polars SQL - use table alias _t_1
                        formatted_output.replace(INPUT_TABLE_NAME, "_t_1")
                    }
                };
            }
            // If --output is used, append plaintext to file, do not overwrite
            if let Some(output) = &args.flag_output {
                fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(output)
                    .map_err(|e| format!("Failed to open output file {output}: {e}"))?
                    .write_all(formatted_output.as_bytes())
                    .map_err(|e| format!("Failed to write to output file {output}: {e}"))?;
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
        (prompt, system_prompt) =
            get_prompt(PromptType::Dictionary, Some(&analysis_results), args)?;
        let start_time = Instant::now();
        print_status("  Inferring Data Dictionary...", None);
        messages = get_messages(&prompt, &system_prompt, "");

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
        process_output(
            PromptType::Dictionary,
            &data_dict,
            &mut total_json_output,
            args,
        )?;
    }

    // Generate description output
    if args.flag_description || args.flag_all {
        (prompt, system_prompt) = if args.flag_dictionary {
            get_prompt(PromptType::Description, None, args)?
        } else {
            get_prompt(PromptType::Description, Some(&analysis_results), args)?
        };
        messages = get_messages(&prompt, &system_prompt, &data_dict.response);
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
        process_output(
            PromptType::Description,
            &completion_response,
            &mut total_json_output,
            args,
        )?;
    }

    // Generate tags output
    if args.flag_tags || args.flag_all {
        (prompt, system_prompt) = if args.flag_dictionary {
            get_prompt(PromptType::Tags, None, args)?
        } else {
            get_prompt(PromptType::Tags, Some(&analysis_results), args)?
        };
        // Only include dictionary context if dictionary was actually generated
        let dictionary_context = if args.flag_dictionary || args.flag_all {
            &data_dict.response
        } else {
            ""
        };
        messages = get_messages(&prompt, &system_prompt, dictionary_context);
        let start_time = Instant::now();
        print_status("  Inferring Tags...", None);
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
        process_output(
            PromptType::Tags,
            &completion_response,
            &mut total_json_output,
            args,
        )?;
    }

    // Generate custom prompt output
    let mut has_sql_query = false;
    if args.flag_prompt.is_some() {
        (prompt, system_prompt) = get_prompt(PromptType::Prompt, Some(&analysis_results), args)?;
        let start_time = Instant::now();
        print_status("  Answering Custom Prompt...", None);
        messages = get_messages(&prompt, &system_prompt, &data_dict.response);
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
                    "Cannot answer the prompt using just Summary Statistics & Frequency \
                     Distribution data.\nGenerated a SQL {} query to answer the prompt \
                     deterministically.",
                    if should_use_duckdb() {
                        "DuckDB"
                    } else {
                        "Polars"
                    }
                ),
                None,
            );
            // append the reasoning to the sql query as a separate markdown section
            completion_response.response = format!(
                "{}\n\n## REASONING\n\n{}\n",
                completion_response.response, completion_response.reasoning
            );
        }
        process_output(
            PromptType::Prompt,
            &completion_response,
            &mut total_json_output,
            args,
        )?;
    }

    print_status("LLM inference/s completed.", Some(llm_start.elapsed()));

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
                "`--sql-results` specified.\nExecuting SQL query and saving results to {}...",
                { sql_results }
            ),
            None,
        );

        // Extract SQL query code block using regex
        let Some(mut sql_query) = regex_oncelock!(r"(?s)```sql\n(.*?)\n```")
            .captures(&completion_response.response)
            .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
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
            // For DuckDB, replace INPUT_TABLE_NAME with read_csv function call
            let re = regex_oncelock!("read_csv_auto\\([^)]*\\)");
            if re.is_match(&sql_query) {
                // DuckDB with read_csv_auto so replace with quoted path
                sql_query = re
                    .replace_all(&sql_query, format!("read_csv_auto('{}')", input_path))
                    .into_owned()
            } else {
                return fail_clierror!("Failed to extract SQL query from custom prompt response");
            };
            log::debug!("DuckDB SQL query:\n{sql_query}");

            let (_, stderr) =
                match run_duckdb_query(&sql_query, sql_results, "DuckDB SQL query issued.") {
                    Ok((stdout, stderr)) => {
                        // Check stderr for error messages
                        if stderr.to_ascii_lowercase().contains(" error:") {
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
                        // Invalidate the prompt cache entry so user can try again
                        if cache_type != &CacheType::Fresh && cache_type != &CacheType::None {
                            let _ = invalidate_cache_entry(args, PromptType::Prompt);
                        }
                        return Err(e);
                    },
                };

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

                // save sql query to a temporary file with a .sql extension
                // this tempfile is automatically deleted after the command finishes
                let sql_query_file = tempfile::Builder::new().suffix(".sql").tempfile()?;
                fs::write(&sql_query_file, sql_query)?;

                let (_, stderr) = match run_qsv_cmd(
                    "sqlp",
                    &[
                        &sql_query_file.path().display().to_string(),
                        "--try-parsedates",
                        "--output",
                        sql_results,
                    ],
                    input_path,
                    "Polars SQL query issued.",
                ) {
                    Ok((stdout, stderr)) => {
                        // Check stderr for error messages
                        if stderr.to_ascii_lowercase().contains("error:") {
                            // Invalidate cache entry so user can try again without
                            // reinferring dictionary
                            if cache_type != &CacheType::Fresh && cache_type != &CacheType::None {
                                let _ = invalidate_cache_entry(args, PromptType::Prompt);
                            }
                            return fail_clierror!("Polars SQL query execution failed: {stderr}");
                        }
                        (stdout, stderr)
                    },
                    Err(e) => {
                        // Invalidate cache entry so user can try again without
                        // reinferring dictionary
                        if cache_type != &CacheType::Fresh && cache_type != &CacheType::None {
                            let _ = invalidate_cache_entry(args, PromptType::Prompt);
                        }
                        return Err(e);
                    },
                };

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

    // Expecting JSON output
    if is_json_output(args)? && !is_jsonl_output(args)? {
        // Format & print JSON output
        let json_output = &simd_json::to_string_pretty(&total_json_output)?;
        // Write to file if --output is used, or overwrite if already exists
        if let Some(output_file_path) = &args.flag_output {
            fs::write(output_file_path, json_output)?;
        } else {
            println!("{json_output}");
        }
    }
    // Expecting JSONL output
    else if is_jsonl_output(args)? {
        // Add prompt file name and timestamp to JSONL output
        let prompt_file = get_prompt_file(args)?;
        total_json_output["prompt_file"] = json!(prompt_file.name);
        total_json_output["timestamp"] = json!(chrono::offset::Utc::now().to_rfc3339());
        // Format & print JSONL output
        let json_output = &simd_json::to_string(&total_json_output)?;
        // Write to file if --output is used, or append if already exists
        if let Some(output_file_path) = &args.flag_output {
            fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(output_file_path)?
                .write_all(format!("\n{json_output}").as_bytes())?;
        } else {
            println!("{json_output}");
        }
    }

    Ok(())
}

// Helper function to run qsv commands with consistent error handling and timing
fn run_qsv_cmd(
    command: &str,
    args: &[&str],
    input_path: &str,
    status_msg: &str,
) -> CliResult<(String, String)> {
    let start_time = Instant::now();

    let qsv_path = QSV_PATH.get().unwrap();
    let mut cmd = Command::new(qsv_path);
    cmd.arg(command).arg(input_path).args(args);

    let output = cmd
        .output()
        .map_err(|e| CliError::Other(format!("Error while executing command {command}: {e:?}")))?;
    log::debug!("qsv command {command} output: {output:?}");

    print_status(status_msg, Some(start_time.elapsed()));

    let stdout_str = std::str::from_utf8(&output.stdout).map_err(|e| {
        CliError::Other(format!(
            "Unable to parse output of qsv command {command}: {e:?}"
        ))
    })?;
    let stderr_str = std::str::from_utf8(&output.stderr).map_err(|e| {
        CliError::Other(format!(
            "Unable to parse stderr of qsv command {command}: {e:?}"
        ))
    })?;

    Ok((stdout_str.to_string(), stderr_str.to_string()))
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
        return fail_clierror!("DuckDB SQL query execution failed: {output:?}");
    }

    let Ok(stdout_str) = simdutf8::basic::from_utf8(&output.stdout) else {
        return fail_clierror!("Unable to parse stdout of DuckDB command: {output:?}");
    };
    let Ok(stderr_str) = simdutf8::basic::from_utf8(&output.stderr) else {
        return fail_clierror!("Unable to parse stderr of DuckDB command: {output:?}");
    };

    // Also check stderr for error messages even if exit status is 0
    if stderr_str.to_ascii_lowercase().contains(" error:") {
        return fail_clierror!("DuckDB SQL query execution failed: {stderr_str}");
    }

    // Write the output to the specified file
    if !output_path.is_empty()
        && let Err(e) = fs::write(output_path, stdout_str)
    {
        return fail_clierror!("Failed to write SQL results to {output_path}: {e}");
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
                "{:?}{:?}{:?}{:?}{:?}{:?}{}{}",
                args.arg_input,
                args.flag_prompt_file,
                prompt_content_for_key,
                args.flag_max_tokens,
                args.flag_addl_props,
                &prompt_file.model,
                kind,
                file_hash
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

        if !args.flag_no_cache {
            if let Err(e) = GET_DISKCACHE_COMPLETION.cache_remove(&key) {
                print_status(
                    &format!("Warning: Cannot remove cache entry for {kind}: {e:?}"),
                    None,
                );
            } else {
                print_status(
                    &format!("Removed cache entry for {kind} due to SQL execution failure"),
                    None,
                );
                // Flush the disk cache to ensure changes are persisted
                if let Err(e) = GET_DISKCACHE_COMPLETION.connection().flush() {
                    print_status(&format!("Warning: Cannot flush disk cache: {e:?}"), None);
                } else {
                    print_status("Flushed disk cache after removing cache entry", None);
                }
            }
        } else if args.flag_redis_cache {
            let conn_str = &REDISCONFIG.get().unwrap().conn_str;
            let redis_client = redis::Client::open(conn_str.to_string())
                .map_err(|e| CliError::Other(format!("Invalid Redis connection string: {e:?}")))?;

            let mut redis_conn = redis_client
                .get_connection()
                .map_err(|e| CliError::Other(format!("Cannot connect to Redis: {e:?}")))?;

            match redis::cmd("DEL").arg(&key).exec(&mut redis_conn) {
                Ok(()) => print_status(
                    &format!("Removed cache entry for {kind} due to SQL execution failure"),
                    None,
                ),
                Err(e) => print_status(
                    &format!("Warning: Cannot remove cache entry for {kind}: {e:?}"),
                    None,
                ),
            }
        }
    }

    Ok(())
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
    QUIET_FLAG.store(args.flag_quiet, Ordering::Relaxed);

    // Check if QSV_LLM_BASE_URL is set
    if let Ok(base_url) = env::var("QSV_LLM_BASE_URL") {
        args.flag_base_url = Some(base_url);
    }

    // Check for QSV_LLM_APIKEY is set
    let apikey_env_var = env::var("QSV_LLM_APIKEY");
    let api_key: String = if args
        .flag_base_url
        .as_deref()
        .unwrap_or_default()
        .contains("localhost")
    {
        // Allow empty API key for localhost
        args.flag_api_key
            .clone()
            .or_else(|| apikey_env_var.ok())
            .unwrap_or_default()
    } else {
        // Require API key for non-localhost
        match apikey_env_var {
            Ok(val) => val,
            Err(_) => {
                // Check if the --api-key flag is present
                if let Some(api_key) = &args.flag_api_key {
                    api_key.clone()
                } else {
                    return fail!(LLM_APIKEY_ERROR);
                }
            },
        }
    };

    // If both --json and --jsonl flags are specified, print error message.
    if is_json_output(&args)? && is_jsonl_output(&args)? {
        return fail_incorrectusage_clierror!(
            "--json and --jsonl options cannot be specified together."
        );
    }

    let cache_type = match (args.flag_no_cache, args.flag_redis_cache) {
        (false, false) => {
            // DISK CACHE
            let diskcache_dir = match &args.flag_disk_cache_dir {
                Some(dir) => {
                    if dir.starts_with('~') {
                        // expand the tilde
                        let expanded_dir = util::expand_tilde(dir).unwrap();
                        expanded_dir.to_string_lossy().to_string()
                    } else {
                        dir.to_string()
                    }
                },
                _ => {
                    // Default disk cache directory
                    let default_dir = util::expand_tilde("~/.qsv/cache/describegpt").unwrap();
                    default_dir.to_string_lossy().to_string()
                },
            };

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

            // initialize DiskCache Config
            // safety: the init values were just set above
            DISKCACHE_DIR.set(diskcache_dir).unwrap();
            DISKCACHECONFIG.set(DiskCacheConfig::new()).unwrap();

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
                            "{:?}{:?}{:?}{:?}{:?}{:?}{}{}",
                            args.arg_input,
                            args.flag_prompt_file,
                            args.flag_prompt,
                            args.flag_max_tokens,
                            args.flag_addl_props,
                            prompt_file.model,
                            kind,
                            FILE_HASH.get().unwrap_or(&String::new())
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
                            print_status(
                                &format!("Found and removed cache entry for {kind}"),
                                None,
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
                    match redis::cmd("DEL").arg(&key).exec(&mut redis_conn) {
                        Ok(()) => {
                            print_status(
                                &format!("Found and removed cache entry for {kind}"),
                                None,
                            );
                        },
                        Err(e) => print_status(
                            &format!("Warning: Cannot remove cache entry for {kind}: {e:?}"),
                            None,
                        ),
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
    }

    // Initialize the global qsv path
    QSV_PATH.set(util::current_exe()?.to_string_lossy().to_string())?;

    // Calculate SHA256 hash of the input file early for cache key generation
    print_status(&format!("Calculating SHA256 hash of {input_path}..."), None);
    let start_hash_time = Instant::now();
    let file_hash = util::hash_sha256_file(Path::new(&input_path))?;
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
    Ok(())
}

// Perform the actual data analysis (stats, frequency, headers)
fn perform_analysis(args: &Args, input_path: &str) -> CliResult<AnalysisResults> {
    // Initialize the global qsv path if not already set
    if QSV_PATH.get().is_none() {
        QSV_PATH.set(util::current_exe()?.to_string_lossy().to_string())?;
    }

    // check if the input file is indexed, if not, index it for performance
    let config = Config::new(Some(&input_path.to_string()));
    if config.index_files().is_err() {
        let _ = run_qsv_cmd("index", &[], input_path, "  Indexed")?;
    }

    // get the delimiter of the input file
    let delimiter = config.get_delimiter();

    // Run qsv commands to analyze data
    print_status(
        &format!(
            "  Compiling Summary Statistics (options: '{}')...",
            args.flag_stats_options
        ),
        None,
    );
    let stats_args_vec = args
        .flag_stats_options
        .split_whitespace()
        .collect::<Vec<&str>>();
    let (stats, _) = run_qsv_cmd("stats", &stats_args_vec, input_path, " ")?;

    print_status(
        &format!(
            "  Compiling Frequency Distribution (enum threshold: {})...",
            args.flag_enum_threshold
        ),
        None,
    );
    let (frequency, _) = run_qsv_cmd(
        "frequency",
        &["--limit", &args.flag_enum_threshold.to_string()],
        input_path,
        " ",
    )?;

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
            // Force fresh analysis but still update cache
            let fresh_result = perform_analysis(args, input_path)?;

            // Manually update the appropriate cache with the fresh result
            let key = get_analysis_cache_key(args, file_hash);
            if args.flag_redis_cache {
                GET_REDIS_ANALYSIS.cache_set(key, fresh_result.clone())?;
            } else {
                GET_DISKCACHE_ANALYSIS.cache_set(key, fresh_result.clone())?;
            }
            Ok(Some(fresh_result))
        },
        CacheType::None => Ok(None),
    }
}
