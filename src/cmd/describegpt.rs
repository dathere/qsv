static USAGE: &str = r#"
Infer a Description, a Data Dictionary & Tags about a CSV using a Large Language Model (LLM).

You can also use the --prompt option to issue a custom prompt, with the ability to embed
summary statistics, frequency data & headers in the prompt using the {stats}, {frequency} &
{headers} variables respectively.

Note that LLMs are prone to inaccurate information being produced.
Verify output results before using them.

Examples:

  # Generate a data dictionary of a CSV file using Ollama using the DeepSeek R1:14b model
  $ qsv describegpt data.csv -u http://localhost:11434/v1 -k ollama -m deepseek-r1:14b --dictionary

  # Generate a data dictionary, description & tags of a CSV file using OpenAI's gpt-oss-20b model
  # (replace <API_KEY> with your OpenAI API key)
  $ qsv describegpt data.csv -k <API_KEY> --all

  # use the disk cache to speed up the process and save on API calls
  $ qsv describegpt data.csv -k <API_KEY> --all --disk-cache

  # save the response to a JSON file
  $ qsv describegpt data.csv -k <API_KEY> --all --json > data.json

  # Ask a question about the sample NYC 311 dataset using LM Studio using the default model
  $ qsv describegpt NYC_311.csv -u http://localhost:1234/v1 --prompt "What is the most common complaint?"

For more examples, see https://github.com/dathere/qsv/blob/master/tests/test_describegpt.rs.

For more detailed info on how describegpt works and how to prepare a prompt file,
see https://github.com/dathere/qsv/blob/master/docs/Describegpt.md

Usage:
    qsv describegpt [options] [<input>]
    qsv describegpt --help

describegpt options:
    -A, --all              Print all extended metadata options output.
    --description          Print a general description of the dataset.
    --dictionary           Create a data dictionary. For each field, prints an inferred type,
                           a human-readable label, a description, and stats.
    --tags                 Prints tags that categorize the dataset. Useful
                           for grouping datasets and filtering.
    -k, --api-key <key>    The API key to use. If the QSV_LLM_APIKEY envvar is set,
                           it will be used instead. Required when the base URL is not localhost.
    -t, --max-tokens <n>   Limits the number of generated tokens in the output.
                           Set to 0 to disable token limits.
                           [default: 1000]
    --json                 Return results in JSON format.
    --jsonl                Return results in JSON Lines format.
    --prompt <prompt>      Custom prompt passed as text (alternative to --description, etc.).
                           Replaces {stats}, {frequency} & {headers} in prompt with
                           corresponding qsv command outputs. If the prompt does not
                           contain {stats}, {frequency} or {headers}, they will be
                           automatically added to the prompt.
    --prompt-file <file>   The JSON file containing the prompts to use for inferencing.
                           If not specified, default prompts will be used.
    -u, --base-url <url>   The LLM API URL. Supports APIs & local LLMs compatible with
                           the OpenAI API specification (Ollama, Jan, LM Studio, etc.).
                           The default base URL for Ollama is http://localhost:11434/v1.
                           The default for Jan is https://localhost:1337/v1.
                           The default for LM Studio is http://localhost:1234/v1.
                           If --prompt-file is used, the base URL will be the base URL
                           of the prompt file.
                           If the QSV_LLM_BASE_URL environment variable is set, it will be
                           used instead.
                           [default: https://api.openai.com/v1]
    -m, --model <model>    The model to use for inferencing.
                           If the QSV_LLM_MODEL environment variable is set, it will be
                           used instead.
                           [default: gpt-oss-20b]
    --timeout <secs>       Timeout for completions in seconds. If 0, no timeout is used.
                           [default: 120]
    --user-agent <agent>   Specify custom user agent. It supports the following variables -
                           $QSV_VERSION, $QSV_TARGET, $QSV_BIN_NAME, $QSV_KIND and $QSV_COMMAND.
                           Try to follow the syntax here -
                           https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/User-Agent

                             CACHING OPTIONS:
    --disk-cache             Use a persistent disk cache for LLM completions. The cache is stored in the
                             directory specified by --disk-cache-dir. If the directory does not exist, it will
                             be created. If the directory exists, it will be used as is. It has a default
                             Time To Live (TTL)/lifespan of 28 days and cache hits do not refresh the TTL
                             of cached values. Adjust the QSV_DISKCACHE_TTL_SECS & QSV_DISKCACHE_TTL_REFRESH
                             env vars to change DiskCache settings.
    --disk-cache-dir <dir>   The directory <dir> to store the disk cache. Note that if the directory
                             does not exist, it will be created. If the directory exists, it will be used as is,
                             and will not be flushed. This option allows you to maintain several disk caches
                             for different describegpt jobs (e.g. one for a data portal, another for internal
                             data exchange, etc.)
                             [default: ~/.qsv/cache/describegpt]
    --redis-cache            Use Redis to cache LLM completions. It connects to "redis://127.0.0.1:6379/3"
                             with a connection pool size of 20, with a TTL of 28 days, and a cache hit
                             NOT renewing an entry's TTL.
                             Adjust the QSV_REDIS_CONNSTR, QSV_REDIS_MAX_POOL_SIZE, QSV_REDIS_TTL_SECONDS &
                             QSV_REDIS_TTL_REFRESH env vars respectively to change Redis settings.
                             This option is ignored if the --disk-cache option is enabled.
    --fresh                  Send a fresh request to the LLM API, refreshing a cached response if it exists.
    --forget                 Remove a cached response if it exists and then exit.
    --flush-cache            Flush all the keys in the current cache on startup.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -q, --quiet            Do not print status messages to stderr.
"#;

use std::{
    env, fs,
    io::Write,
    path::PathBuf,
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

use crate::{CliError, CliResult, regex_oncelock, util, util::process_input};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PromptType {
    Dictionary,
    Description,
    Tags,
    Custom,
}

#[derive(Debug, Deserialize)]
struct Args {
    arg_input:           Option<String>,
    flag_all:            bool,
    flag_description:    bool,
    flag_dictionary:     bool,
    flag_tags:           bool,
    flag_api_key:        Option<String>,
    flag_max_tokens:     u32,
    flag_base_url:       Option<String>,
    flag_model:          Option<String>,
    flag_json:           bool,
    flag_jsonl:          bool,
    flag_prompt:         Option<String>,
    flag_prompt_file:    Option<String>,
    flag_user_agent:     Option<String>,
    flag_timeout:        u16,
    flag_output:         Option<String>,
    flag_quiet:          bool,
    flag_disk_cache:     bool,
    flag_disk_cache_dir: Option<String>,
    flag_redis_cache:    bool,
    flag_fresh:          bool,
    flag_forget:         bool,
    flag_flush_cache:    bool,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct PromptFile {
    name:               String,
    description:        String,
    author:             String,
    version:            String,
    tokens:             u32,
    system_prompt:      String,
    dictionary_prompt:  String,
    description_prompt: String,
    tags_prompt:        String,
    prompt:             String,
    json:               bool,
    jsonl:              bool,
    base_url:           String,
    model:              String,
    timeout:            u32,
}

const LLM_APIKEY_ERROR: &str = "Error: QSV_LLM_APIKEY environment variable not found.\nNote that \
                                this command uses LLMs for inferencing and is therefore prone to \
                                inaccurate information being produced. Verify output results \
                                before using them.";

const DEFAULT_SYSTEM_PROMPT: &str =
    "You are an expert library scientist with a background in statistics and data science. \
    You are also an expert on the DCAT-US 3 specification (https://doi-do.github.io/dcat-us/).";

const DEFAULT_DICTIONARY_PROMPT: &str =
    "Here are the columns for each field in a data dictionary:\n\n- Type: the data type of this \
     column as indicated in the Summary Statistics below.\n- Label: a human-friendly label for \
     this column\n- Description: a full description for this column (can be multiple \
     sentences)\n\nGenerate a data dictionary as aforementioned {json_add} where each field has \
     Name, Type, Label, and Description (so four columns in total) based on the following summary \
     statistics and frequency data (both in CSV format) of the input CSV file.\n\nSummary \
     Statistics:\n\n{stats}\n\nFrequency:\n\n{frequency}";
const DEFAULT_DESCRIPTION_PROMPT: &str =
    "Generate only a description that is within 8 sentences about the entire dataset based on the \
     following summary statistics and frequency data derived from the CSV file it came \
     from.\n\nSummary Statistics:\n\n{stats}\n\nFrequency:\n\n{frequency}\n\nDo not output the \
     summary statistics for each field. Do not output the frequency for each field. Do not output \
     data about each field individually, but instead output about the dataset as a whole in one \
     1-8 sentence description.";
const DEFAULT_TAGS_PROMPT: &str =
    "A tag is a keyword or label that categorizes datasets with other, similar datasets. Using \
     the right tags makes it easier for others to find and use datasets.\n\nGenerate no more than \
     15 most thematic tags{json_add} about the contents of the dataset in descending order of \
     importance (lowercase only and use _ to separate words) based on the following summary \
     statistics and frequency data (both in CSV format) of the input CSV file. Do not use field \
     names in the tags. \n\nSummary Statistics:\n\n{stats}\n\nFrequency:\n\n{frequency}";

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
    token_usage: TokenUsage,
}

static QSV_REDIS_CONNSTR_ENV: &str = "QSV_REDIS_CONNSTR";
static QSV_REDIS_MAX_POOL_SIZE_ENV: &str = "QSV_REDIS_MAX_POOL_SIZE";
static QSV_REDIS_TTL_SECS_ENV: &str = "QSV_REDIS_TTL_SECS";
static QSV_REDIS_TTL_REFRESH_ENV: &str = "QSV_REDIS_TTL_REFRESH";
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

static QUIET_FLAG: OnceLock<AtomicBool> = OnceLock::new();
static QSV_PATH: OnceLock<String> = OnceLock::new();

fn print_status(msg: &str, elapsed: Option<std::time::Duration>) {
    let quiet_flag = QUIET_FLAG.get().unwrap();
    if !quiet_flag.load(Ordering::Relaxed) {
        if let Some(duration) = elapsed {
            eprintln!("{msg} (elapsed: {:.2}s)", duration.as_secs_f64());
        } else {
            eprintln!("{msg}");
        }
    }
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
        prompt_file.base_url
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

    let given_model = env::var("QSV_LLM_MODEL")
        .ok()
        .or_else(|| args.flag_model.clone())
        .or_else(|| {
            args.flag_prompt_file
                .as_ref()
                .map(|_| prompt_file.model.clone())
        })
        // safety: model has a docopt default
        .unwrap()
        .to_string();

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
            print_status(format!("  Using model: {model_id}").as_str(), None);
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

fn get_prompt_file(args: &Args) -> CliResult<PromptFile> {
    // Get prompt file if --prompt-file is used
    let prompt_file = if let Some(prompt_file) = &args.flag_prompt_file {
        // Read prompt file
        let prompt_file = fs::read_to_string(prompt_file)?;
        // Try to parse prompt file as JSON, if error then show it in JSON format
        let prompt_file: PromptFile = match serde_json::from_str(&prompt_file) {
            Ok(val) => val,
            Err(e) => {
                let error_json = json!({"error": e.to_string()});
                return fail_clierror!("{error_json}");
            },
        };
        prompt_file
    }
    // Otherwise, get default prompt file
    else {
        #[allow(clippy::let_and_return)]
        let default_prompt_file = PromptFile {
            name:               "My Prompt File".to_string(),
            description:        "My prompt file for qsv's describegpt command.".to_string(),
            author:             "My Name".to_string(),
            version:            "1.0.0".to_string(),
            tokens:             1000,
            system_prompt:      DEFAULT_SYSTEM_PROMPT.to_owned(),
            dictionary_prompt:  DEFAULT_DICTIONARY_PROMPT.to_owned(),
            description_prompt: DEFAULT_DESCRIPTION_PROMPT.to_owned(),
            tags_prompt:        DEFAULT_TAGS_PROMPT.to_owned(),
            prompt:             "Summary statistics: {stats}\n\nFrequency: {frequency}\n\nWhat is \
                                 this dataset about?"
                .to_owned(),
            json:               true,
            jsonl:              false,
            base_url:           "https://api.openai.com/v1".to_owned(),
            model:              "gpt-oss-20b".to_owned(),
            timeout:            120,
        };
        default_prompt_file
    };
    Ok(prompt_file)
}

// Generate prompt for prompt type based on either the prompt file (if given) or default prompts
fn get_prompt(
    prompt_type: PromptType,
    stats: Option<&str>,
    frequency: Option<&str>,
    headers: Option<&str>,
    args: &Args,
) -> CliResult<(String, String)> {
    // Get prompt file if --prompt-file is used, otherwise get default prompt file
    let prompt_file = get_prompt_file(args)?;

    // Get prompt from prompt file
    let prompt = match prompt_type {
        PromptType::Dictionary => prompt_file.dictionary_prompt,
        PromptType::Description => prompt_file.description_prompt,
        PromptType::Tags => prompt_file.tags_prompt,
        PromptType::Custom => {
            if let Some(prompt) = &args.flag_prompt {
                let mut working_prompt = prompt.clone();
                // if the prompt does not contain {stats}, {frequency} and {headers},
                // automatically add them to the prompt
                let contains_stats = working_prompt.contains("{stats}");
                let contains_frequency = working_prompt.contains("{frequency}");
                let contains_headers = working_prompt.contains("{headers}");
                if !contains_stats && !contains_frequency && !contains_headers {
                    working_prompt += "\n\nSummary statistics of the dataset (CSV format): \
                                       {stats}\n\nFrequency of the dataset (CSV format): \
                                       {frequency}\n\nHeaders of the dataset: {headers}";
                }
                working_prompt
            } else {
                prompt_file.prompt
            }
        },
    };
    // Replace variable data in prompt
    #[allow(clippy::to_string_in_format_args)]
    #[allow(clippy::literal_string_with_formatting_args)]
    let prompt = prompt
        .replace("{stats}", stats.unwrap_or(""))
        .replace("{frequency}", frequency.unwrap_or(""))
        .replace("{headers}", headers.unwrap_or(""))
        .replace(
            "{json_add}",
            if prompt_file.json
                || prompt_file.jsonl
                || (args.flag_prompt_file.is_none() && (args.flag_json || args.flag_jsonl))
            {
                " (in valid JSON format. Surround it with ```json and ```)"
            } else {
                ""
            },
        );

    // Return prompt
    Ok((prompt, prompt_file.system_prompt))
}

fn get_completion(
    args: &Args,
    client: &Client,
    model: &str,
    api_key: &str,
    messages: &serde_json::Value,
) -> CliResult<CompletionResponse> {
    let prompt_file = get_prompt_file(args)?;

    // If max_tokens is 0, always disable the limit, even if a prompt file is present.
    let max_tokens = if args.flag_max_tokens == 0 {
        None
    } else if args.flag_prompt_file.is_some() && args.flag_max_tokens > 0 {
        // Only use prompt_file.tokens if max_tokens is not explicitly set to 0
        Some(prompt_file.tokens)
    } else {
        Some(args.flag_max_tokens)
    };

    let model_to_use = if args.flag_prompt_file.is_some() {
        prompt_file.model
    } else {
        model.to_string()
    };

    let base_url = if args.flag_prompt_file.is_some() {
        prompt_file.base_url
    } else {
        // safety: base_url has a docopt default
        args.flag_base_url.as_deref().unwrap().to_string()
    };

    // Create request data
    let request_data = json!({
        "model": model_to_use,
        "max_tokens": max_tokens,
        "messages": messages,
        "stream": false
    });

    // Get response from POST request to chat completions endpoint
    let completions_endpoint = "/chat/completions";
    let response = send_request(
        client,
        Some(api_key),
        Some(&request_data),
        "POST",
        format!("{base_url}{completions_endpoint}").as_str(),
    )?;

    // Parse response as JSON
    let response_json: serde_json::Value = response.json()?;
    // If response is an error, print error message
    if let serde_json::Value::Object(ref map) = response_json
        && map.contains_key("error")
    {
        return fail_clierror!("LLM API Error: {}", map["error"]);
    }

    // Get completion from response
    let Some(completion) = response_json["choices"]
        .get(0)
        .and_then(|choice| choice["message"]["content"].as_str())
    else {
        return fail_clierror!("Invalid response: missing or malformed completion content");
    };

    // Get token usage from response
    let Some(usage) = response_json["usage"].as_object() else {
        return fail_clierror!("Invalid response: missing or malformed usage");
    };
    let token_usage = TokenUsage {
        prompt:     usage["prompt_tokens"].as_u64().unwrap_or(0),
        completion: usage["completion_tokens"].as_u64().unwrap_or(0),
        total:      usage["total_tokens"].as_u64().unwrap_or(0),
    };

    Ok(CompletionResponse {
        response: completion.to_string(),
        token_usage,
    })
}

// this is a disk cache that can be used across qsv sessions
#[io_cached(
    disk = true,
    ty = "cached::DiskCache<String, CompletionResponse>",
    cache_prefix_block = r##"{ "descdc_" }"##,
    key = "String",
    convert = r##"{ format!("{:?}{:?}{:?}{}{}", args.arg_input, args.flag_prompt_file, args.flag_prompt, model, kind) }"##,
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
    map_error = r##"|e| CliError::Other(format!("Diskcache Error: {:?}", e))"##,
    with_cached_flag = true
)]
fn get_diskcache_completion(
    args: &Args,
    client: &Client,
    model: &str,
    api_key: &str,
    #[allow(unused_variables)] kind: &str,
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
    convert = r##"{ format!("{:?}{:?}{:?}{}{}", args.arg_input, args.flag_prompt_file, args.flag_prompt, model, kind) }"##,
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
    map_error = r##"|e| CliError::Other(format!("Redis Error: {:?}", e))"##,
    with_cached_flag = true
)]
fn get_redis_completion(
    args: &Args,
    client: &Client,
    model: &str,
    api_key: &str,
    #[allow(unused_variables)] kind: &str,
    messages: &serde_json::Value,
) -> CliResult<Return<CompletionResponse>> {
    Ok(Return::new(get_completion(
        args, client, model, api_key, messages,
    )?))
}

// Check if JSON output is expected
fn is_json_output(args: &Args) -> CliResult<bool> {
    // By default expect plaintext output
    let mut json_output = false;
    // Set expect_json to true if --prompt-file is used & the "json" field is true
    if args.flag_prompt_file.is_some() {
        let prompt_file = get_prompt_file(args)?;
        if prompt_file.json {
            json_output = true;
        }
    }
    // Set expect_json to true if --prompt-file is not used & --json is used
    else if args.flag_json {
        json_output = true;
    }
    Ok(json_output)
}
// Check if JSONL output is expected
fn is_jsonl_output(args: &Args) -> CliResult<bool> {
    // By default expect plaintext output
    let mut jsonl_output = false;
    // Set expect_jsonl to true if --prompt-file is used & the "jsonl" field is true
    if args.flag_prompt_file.is_some() {
        let prompt_file = get_prompt_file(args)?;
        if prompt_file.jsonl {
            jsonl_output = true;
        }
    }
    // Set expect_jsonl to true if --prompt-file is not used & --jsonl is used
    else if args.flag_jsonl {
        jsonl_output = true;
    }
    Ok(jsonl_output)
}

// Unified function to handle cached completions
fn get_cached_completion(
    args: &Args,
    client: &Client,
    model: &str,
    api_key: &str,
    cache_type: &CacheType,
    kind: &str,
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
    args: &Args,
    api_key: &str,
    cache_type: &CacheType,
    stats_str: Option<&str>,
    frequency_str: Option<&str>,
    headers_str: Option<&str>,
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
            "content": format!("The following is the data dictionary for the input data:\n\n{dictionary_completion}")},
            {"role": "user", "content": prompt},
            ])
        }
    }
    // Format output by replacing escape characters
    fn format_output(str: &str) -> String {
        str.replace("\\n", "\n")
            .replace("\\t", "\t")
            .replace("\\\"", "\"")
            .replace("\\'", "'")
            .replace("\\`", "`")
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
        option: &str,
        output: &str,
        total_json_output: &mut serde_json::Value,
        args: &Args,
    ) -> CliResult<()> {
        // Process JSON output if expected or JSONL output is expected
        if is_json_output(args)? || is_jsonl_output(args)? {
            total_json_output[option] = if option == "description" {
                serde_json::Value::String(output.to_string())
            } else {
                extract_json_from_output(output)?
            };
        }
        // Process plaintext output
        else {
            let formatted_output = format_output(output);
            println!("{formatted_output}");
            // If --output is used, append plaintext to file, do not overwrite
            if let Some(output) = &args.flag_output {
                fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(output)?
                    .write_all(formatted_output.as_bytes())?;
            }
        }
        Ok(())
    }

    // Get completion from API
    let llm_start = Instant::now();
    print_status("\nInteracting with LLM...", None);

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
    let mut completion_response: CompletionResponse;

    // Generate dictionary output
    if args.flag_dictionary || args.flag_all {
        (prompt, system_prompt) = get_prompt(
            PromptType::Dictionary,
            stats_str,
            frequency_str,
            headers_str,
            args,
        )?;
        let start_time = Instant::now();
        print_status("  Generating data dictionary...", None);
        messages = get_messages(&prompt, &system_prompt, "");
        data_dict = get_cached_completion(
            args,
            &client,
            &model,
            api_key,
            cache_type,
            "dictionary",
            &messages,
        )?;
        print_status(
            format!(
                "   Received dictionary completion.\n   {:?}\n  ",
                data_dict.token_usage
            )
            .as_str(),
            Some(start_time.elapsed()),
        );
        process_output(
            "dictionary",
            &data_dict.response,
            &mut total_json_output,
            args,
        )?;
    }

    // Generate description output
    if args.flag_description || args.flag_all {
        (prompt, system_prompt) = if args.flag_dictionary {
            get_prompt(PromptType::Description, None, None, None, args)?
        } else {
            get_prompt(
                PromptType::Description,
                stats_str,
                frequency_str,
                headers_str,
                args,
            )?
        };
        messages = get_messages(&prompt, &system_prompt, &data_dict.response);
        let start_time = Instant::now();
        print_status("  Generating description...", None);
        completion_response = get_cached_completion(
            args,
            &client,
            &model,
            api_key,
            cache_type,
            "description",
            &messages,
        )?;
        print_status(
            format!(
                "   Received description completion.\n   {:?}\n  ",
                completion_response.token_usage
            )
            .as_str(),
            Some(start_time.elapsed()),
        );
        process_output(
            "description",
            &completion_response.response,
            &mut total_json_output,
            args,
        )?;
    }

    // Generate tags output
    if args.flag_tags || args.flag_all {
        (prompt, system_prompt) = if args.flag_dictionary {
            get_prompt(PromptType::Tags, None, None, None, args)?
        } else {
            get_prompt(
                PromptType::Tags,
                stats_str,
                frequency_str,
                headers_str,
                args,
            )?
        };
        // Only include dictionary context if dictionary was actually generated
        let dictionary_context = if args.flag_dictionary || args.flag_all {
            &data_dict.response
        } else {
            ""
        };
        messages = get_messages(&prompt, &system_prompt, dictionary_context);
        let start_time = Instant::now();
        print_status("  Generating tags...", None);
        completion_response = get_cached_completion(
            args, &client, &model, api_key, cache_type, "tags", &messages,
        )?;
        print_status(
            format!(
                "   Received tags completion.\n   {:?}\n  ",
                completion_response.token_usage
            )
            .as_str(),
            Some(start_time.elapsed()),
        );
        process_output(
            "tags",
            &completion_response.response,
            &mut total_json_output,
            args,
        )?;
    }

    // Generate custom prompt output
    if args.flag_prompt.is_some() {
        (prompt, system_prompt) = get_prompt(
            PromptType::Custom,
            stats_str,
            frequency_str,
            headers_str,
            args,
        )?;
        let start_time = Instant::now();
        print_status("  Generating custom prompt output...", None);
        messages = get_messages(&prompt, &system_prompt, &data_dict.response);
        completion_response = get_cached_completion(
            args, &client, &model, api_key, cache_type, "prompt", &messages,
        )?;
        print_status(
            format!(
                "   Received custom prompt completion.\n   {:?}\n  ",
                completion_response.token_usage
            )
            .as_str(),
            Some(start_time.elapsed()),
        );
        process_output(
            "prompt",
            &completion_response.response,
            &mut total_json_output,
            args,
        )?;
    }

    print_status("LLM completions received.", Some(llm_start.elapsed()));

    // Expecting JSON output
    if is_json_output(args)? && !is_jsonl_output(args)? {
        // Format & print JSON output
        let json_output = &simd_json::to_string_pretty(&total_json_output)?;
        println!("{json_output}");
        // Write to file if --output is used, or overwrite if already exists
        if let Some(output_file_path) = &args.flag_output {
            fs::write(output_file_path, json_output)?;
        }
    }
    // Expecting JSONL output
    else if is_jsonl_output(args)? {
        // If --prompt-file is used, add prompt file name and timestamp to JSONL output
        if args.flag_prompt_file.is_some() {
            let prompt_file = get_prompt_file(args)?;
            total_json_output["prompt_file"] = json!(prompt_file.name);
            total_json_output["timestamp"] = json!(chrono::offset::Utc::now().to_rfc3339());
        }
        // Format & print JSONL output
        let json_output = &simd_json::to_string(&total_json_output)?;
        println!("{json_output}");
        // Write to file if --output is used, or append if already exists
        if let Some(output_file_path) = &args.flag_output {
            fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(output_file_path)?
                .write_all(format!("\n{json_output}").as_bytes())?;
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
) -> CliResult<String> {
    let start_time = Instant::now();

    let qsv_path = QSV_PATH.get().unwrap();
    let mut cmd = Command::new(qsv_path);
    cmd.arg(command).args(args).arg(input_path);

    let output = cmd
        .output()
        .map_err(|e| CliError::Other(format!("Error while executing command {command}: {e:?}")))?;

    print_status(&format!("  {status_msg}."), Some(start_time.elapsed()));

    let output_str = std::str::from_utf8(&output.stdout).map_err(|e| {
        CliError::Other(format!(
            "Unable to parse output of qsv command {command}: {e:?}"
        ))
    })?;

    Ok(output_str.to_string())
}

fn determine_cache_kinds_to_remove(args: &Args) -> Vec<&'static str> {
    if args.flag_dictionary {
        vec!["dictionary"]
    } else if args.flag_description {
        vec!["description"]
    } else if args.flag_tags {
        vec!["tags"]
    } else if args.flag_prompt.is_some() {
        vec!["prompt"]
    } else {
        vec!["dictionary", "description", "tags", "prompt"]
    }
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let start_time = Instant::now();
    let mut args: Args = util::get_args(USAGE, argv)?;

    // Initialize the global quiet flag
    QUIET_FLAG.set(AtomicBool::new(args.flag_quiet)).unwrap();

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
    // If --prompt-file flag is specified but the prompt file does not exist, print error message.
    if let Some(prompt_file) = &args.flag_prompt_file
        && !PathBuf::from(prompt_file).exists()
    {
        return fail_incorrectusage_clierror!("Prompt file '{prompt_file}' does not exist.");
    }
    // If --json and --jsonl flags are specified, print error message.
    if is_json_output(&args)? && is_jsonl_output(&args)? {
        return fail_incorrectusage_clierror!(
            "--json and --jsonl options cannot be specified together."
        );
    }

    // setup diskcache dir response caching
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
        _ => String::new(),
    };

    let cache_type = if args.flag_disk_cache {
        // if --flush-cache is set, flush the cache directory first if it exists
        if args.flag_flush_cache
            && !diskcache_dir.is_empty()
            && fs::metadata(&diskcache_dir).is_ok()
        {
            if let Err(e) = fs::remove_dir_all(&diskcache_dir) {
                return fail_clierror!(r#"Cannot remove cache directory "{diskcache_dir}": {e:?}"#);
            }
            print_status(
                &format!("flushed DiskCache directory: {diskcache_dir}"),
                None,
            );
            return Ok(());
        }
        // check if the cache directory exists, if it doesn't, create it
        if !diskcache_dir.is_empty()
            && let Err(e) = fs::create_dir_all(&diskcache_dir)
        {
            return fail_clierror!(r#"Cannot create cache directory "{diskcache_dir}": {e:?}"#);
        }

        // initialize DiskCache Config
        // safety: we set and get in the next few lines
        DISKCACHE_DIR.set(diskcache_dir).unwrap();
        DISKCACHECONFIG.set(DiskCacheConfig::new()).unwrap();

        // If --forget is set, remove cache entries and exit
        if args.flag_forget {
            // Determine which cache entries to remove
            let kinds_to_remove = determine_cache_kinds_to_remove(&args);

            // Create the same cache instance that the #[io_cached] macro uses
            let cache_dir = DISKCACHE_DIR.get().unwrap();
            let diskcache_config = DISKCACHECONFIG.get().unwrap();
            let io_cache: DiskCache<String, CompletionResponse> =
                DiskCacheBuilder::new("describegpt")
                    .set_disk_directory(cache_dir)
                    .set_lifespan(diskcache_config.ttl_secs)
                    .set_refresh(diskcache_config.ttl_refresh)
                    .build()
                    .expect("error building diskcache");

            // Remove cache entries for all specified kinds using the same key format as the macro
            for kind in kinds_to_remove {
                let key = format!(
                    "{:?}{:?}{:?}{:?}{:?}",
                    args.arg_input,
                    args.flag_prompt_file,
                    args.flag_prompt,
                    args.flag_model.as_deref().unwrap_or(""),
                    kind
                );
                if let Err(e) = io_cache.cache_remove(&key) {
                    print_status(
                        format!("Warning: Cannot remove cache entry for {kind}: {e:?}").as_str(),
                        None,
                    );
                } else {
                    print_status(
                        format!("Found and removed cache entry for {kind}").as_str(),
                        None,
                    );
                }
            }
            return Ok(());
        } else if args.flag_fresh {
            // If --fresh is set, use CacheType::Fresh to force refresh but still update cache
            CacheType::Fresh
        } else {
            CacheType::Disk
        }
    } else if args.flag_redis_cache {
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
            log::info!("flushed Redis database.");
        }

        // If --forget is set, remove cache entries and exit
        if args.flag_forget {
            // Determine which cache entries to remove
            let kinds_to_remove = determine_cache_kinds_to_remove(&args);

            // Remove cache entries for all specified kinds using the same key format as the macro
            for kind in kinds_to_remove {
                let key = format!(
                    "{:?}{:?}{:?}{:?}{:?}",
                    args.arg_input,
                    args.flag_prompt_file,
                    args.flag_prompt,
                    args.flag_model.as_deref().unwrap_or(""),
                    kind
                );
                if let Err(e) = redis::cmd("DEL").arg(&key).exec(&mut redis_conn) {
                    print_status(
                        format!("Warning: Cannot remove cache entry for {kind}: {e:?}").as_str(),
                        None,
                    );
                } else {
                    print_status(
                        format!("Found and removed cache entry for {kind}").as_str(),
                        None,
                    );
                }
            }
            return Ok(());
        } else if args.flag_fresh {
            // If --fresh is set, use CacheType::Fresh to force refresh but still update cache
            CacheType::Fresh
        } else {
            CacheType::Redis
        }
    } else {
        CacheType::None
    };
    log::info!("Cache Type: {cache_type:?}");

    // Initialize the global qsv path
    QSV_PATH.set(util::current_exe()?.to_string_lossy().to_string())?;

    // Get input file's name
    // safety: we just checked that there is at least one input file
    let input_filename = args.arg_input.as_deref().unwrap();

    let analysis_start = Instant::now();
    print_status(format!("Analyzing {input_filename}...").as_str(), None);

    let _ = run_qsv_cmd("index", &[], &input_path, "Indexed")?;

    // Run qsv commands to gather data
    let stats = run_qsv_cmd("stats", &["--everything"], &input_path, "Generated stats")?;

    let frequency = run_qsv_cmd(
        "frequency",
        &["--limit", "10"],
        &input_path,
        "Generated frequency",
    )?;

    let headers = run_qsv_cmd(
        "slice",
        &["--len", "1", "--no-headers"],
        &input_path,
        "Got headers",
    )?;

    print_status("Analyzed data.", Some(analysis_start.elapsed()));

    // Run inference options
    run_inference_options(
        &args,
        &api_key,
        &cache_type,
        Some(&stats),
        Some(&frequency),
        Some(&headers),
    )?;

    // Print total elapsed time
    print_status("\ndescribegpt DONE!", Some(start_time.elapsed()));

    // if using a Diskcache, explicitly flush it to ensure entries are written to disk
    if cache_type == CacheType::Disk {
        GET_DISKCACHE_COMPLETION
            .connection()
            .flush()
            .map_err(|e| CliError::Other(format!("Error flushing DiskCache: {e}")))?;
    }
    Ok(())
}
