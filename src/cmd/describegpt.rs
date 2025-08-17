static USAGE: &str = r#"
Infers extended metadata about a CSV using a Large Language Model (LLM).

Note that this command uses LLMs for inferencing and is therefore prone to
inaccurate information being produced. Verify output results before using them.

Let's say you have Ollama installed (v0.2.0 or above) to use LLMs locally with qsv describegpt.
To attempt generating a data dictionary of a spreadsheet file you may run (replace <> values):

  $ qsv describegpt <filepath> -u http://localhost:11434/v1 -k ollama -m <model> -t <number> --dictionary

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
                           corresponding qsv command outputs.
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

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -q, --quiet            Do not print status messages to stderr.
"#;

use std::{env, fs, io::Write, path::PathBuf, process::Command, time::Instant};

use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::json;

use crate::{CliError, CliResult, regex_oncelock, util, util::process_input};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PromptType {
    Dictionary,
    Description,
    Tags,
    Custom,
}

#[derive(Deserialize)]
struct Args {
    arg_input:        Option<String>,
    flag_all:         bool,
    flag_description: bool,
    flag_dictionary:  bool,
    flag_tags:        bool,
    flag_api_key:     Option<String>,
    flag_max_tokens:  u32,
    flag_base_url:    Option<String>,
    flag_model:       Option<String>,
    flag_json:        bool,
    flag_jsonl:       bool,
    flag_prompt:      Option<String>,
    flag_prompt_file: Option<String>,
    flag_user_agent:  Option<String>,
    flag_timeout:     u16,
    flag_output:      Option<String>,
    flag_quiet:       bool,
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
    "Generate only a description that is within 8 sentences about the entire dataset{json_add} \
     based on the following summary statistics and frequency data derived from the CSV file it \
     came from.\n\nSummary Statistics:\n\n{stats}\n\nFrequency:\n\n{frequency}\n\nDo not output \
     the summary statistics for each field. Do not output the frequency for each field. Do not \
     output data about each field individually, but instead output about the dataset as a whole \
     in one 1-8 sentence description.";
const DEFAULT_TAGS_PROMPT: &str =
    "A tag is a keyword or label that categorizes datasets with other, similar datasets. Using \
     the right tags makes it easier for others to find and use datasets.\n\nGenerate no more than \
     15 most thematic tags{json_add} about the contents of the dataset in descending order of \
     importance (lowercase only and use _ to separate words) based on the following summary \
     statistics and frequency data (both in CSV format) of the input CSV file. Do not use field \
     names in the tags. \n\nSummary Statistics:\n\n{stats}\n\nFrequency:\n\n{frequency}";

#[allow(dead_code)]
#[derive(Debug, Default)]
struct TokenUsage {
    prompt:     u64,
    completion: u64,
    total:      u64,
}

fn print_status(args: &Args, msg: &str, elapsed: Option<std::time::Duration>) {
    if !args.flag_quiet {
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
            let error_json = json!({"Error: Unsupported HTTP method ": other});
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
            print_status(args, format!("  Using model: {model_id}").as_str(), None);
            return Ok(model_id.to_string());
        }
    }

    // Otherwise, fail with list of valid models
    let models_list = models
        .iter()
        .filter_map(|m| m["id"].as_str())
        .collect::<Vec<_>>()
        .join(", ");
    fail_clierror!("Error: Invalid model: {given_model}\n  Valid models: {models_list}")
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
                prompt.clone()
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
) -> CliResult<(String, TokenUsage)> {
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

    Ok((completion.to_string(), token_usage))
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

// Generates output for all inference options
fn run_inference_options(
    args: &Args,
    api_key: &str,
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
            {"role": "assistant", "content": dictionary_completion},
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

        Err(CliError::Other(format!(
            "Failed to extract JSON content from LLM response. Output: {}",
            if output.is_empty() { "<empty>" } else { output }
        )))
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
            let completion_json = extract_json_from_output(output)?;
            total_json_output[option] = completion_json;
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
    print_status(args, "\nInteracting with LLM...", None);

    let client = util::create_reqwest_blocking_client(
        args.flag_user_agent.clone(),
        args.flag_timeout,
        args.flag_base_url.clone(),
    )?;

    // Verify model is valid
    let model = check_model(&client, Some(api_key), args)?;

    let mut total_json_output: serde_json::Value = json!({});
    let mut prompt: String;
    let mut system_prompt: String;
    let mut messages: serde_json::Value;
    let mut data_dict = String::new();
    let mut completion: String;
    let mut token_usage: TokenUsage;

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
        print_status(args, "  Generating data dictionary...", None);
        messages = get_messages(&prompt, &system_prompt, &data_dict);
        (data_dict, token_usage) = get_completion(args, &client, &model, api_key, &messages)?;
        print_status(
            args,
            &format!("   Received dictionary completion.\n   {token_usage:?}\n  "),
            Some(start_time.elapsed()),
        );
        process_output("dictionary", &data_dict, &mut total_json_output, args)?;
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
        messages = get_messages(&prompt, &system_prompt, &data_dict);
        let start_time = Instant::now();
        print_status(args, "  Generating description...", None);
        (completion, token_usage) = get_completion(args, &client, &model, api_key, &messages)?;
        print_status(
            args,
            &format!("   Received description completion.\n   {token_usage:?}\n  "),
            Some(start_time.elapsed()),
        );
        process_output("description", &completion, &mut total_json_output, args)?;
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
        messages = get_messages(&prompt, &system_prompt, &data_dict);
        let start_time = Instant::now();
        print_status(args, "  Generating tags...", None);
        (completion, token_usage) = get_completion(args, &client, &model, api_key, &messages)?;
        print_status(
            args,
            &format!("   Received tags completion.\n   {token_usage:?}\n  "),
            Some(start_time.elapsed()),
        );
        process_output("tags", &completion, &mut total_json_output, args)?;
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
        print_status(args, "  Generating custom prompt output...", None);
        messages = get_messages(&prompt, &system_prompt, &data_dict);
        (completion, token_usage) = get_completion(args, &client, &model, api_key, &messages)?;
        print_status(
            args,
            &format!("   Received custom prompt completion.\n   {token_usage:?}\n  "),
            Some(start_time.elapsed()),
        );
        process_output("prompt", &completion, &mut total_json_output, args)?;
    }

    print_status(args, "LLM completions received.", Some(llm_start.elapsed()));

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

pub fn run(argv: &[&str]) -> CliResult<()> {
    let start_time = Instant::now();
    let mut args: Args = util::get_args(USAGE, argv)?;

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
        return fail_incorrectusage_clierror!("Error: No input file specified.");
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
        return fail_incorrectusage_clierror!("Error: No inference options specified.");
    // If --all flag is specified, but other inference flags are also set, print error message.
    } else if args.flag_all
        && (args.flag_dictionary
            || args.flag_description
            || args.flag_tags
            || args.flag_prompt.is_some())
    {
        return fail_incorrectusage_clierror!(
            "Error: --all option cannot be specified with other inference flags."
        );
    }
    // If --prompt-file flag is specified but the prompt file does not exist, print error message.
    if let Some(prompt_file) = &args.flag_prompt_file
        && !PathBuf::from(prompt_file).exists()
    {
        return fail_incorrectusage_clierror!("Error: Prompt file '{prompt_file}' does not exist.");
    }
    // If --json and --jsonl flags are specified, print error message.
    if is_json_output(&args)? && is_jsonl_output(&args)? {
        return fail_incorrectusage_clierror!(
            "Error: --json and --jsonl options cannot be specified together."
        );
    }

    // Get qsv executable's path
    let qsv_path = std::env::current_exe()?;
    // Get input file's name
    // safety: we just checked that there is at least one input file
    let input_filename = args.arg_input.as_deref().unwrap();

    print_status(&args, &format!("Analyzing {input_filename}..."), None);

    // index the input file
    let index_start = Instant::now();
    let Ok(_index) = Command::new(&qsv_path)
        .arg("index")
        .arg(&input_path)
        .output()
    else {
        return fail!("Error: Error while indexing.");
    };
    print_status(&args, "  Indexed.", Some(index_start.elapsed()));

    // Get stats from qsv stats on input file with --everything flag
    let stats_start = Instant::now();
    print_status(&args, "  Generating stats...", None);
    let Ok(stats) = Command::new(&qsv_path)
        .arg("stats")
        .arg("--everything")
        .arg(&input_path)
        .output()
    else {
        return fail!("Error: Error while generating stats.");
    };
    print_status(&args, "  Generated stats.", Some(stats_start.elapsed()));

    // Parse the stats as &str
    let Ok(stats_str) = std::str::from_utf8(&stats.stdout) else {
        return fail!("Error: Unable to parse stats.");
    };

    // Get frequency from qsv frequency on input file
    let freq_start = Instant::now();
    print_status(&args, "  Generating frequency...", None);
    let Ok(frequency) = Command::new(&qsv_path)
        .arg("frequency")
        .args(["--limit", "10"])
        .arg(&input_path)
        .output()
    else {
        return fail!("Error: Error while generating frequency.");
    };
    print_status(&args, "  Generated frequency.", Some(freq_start.elapsed()));
    // Parse the frequency as &str
    let Ok(frequency_str) = std::str::from_utf8(&frequency.stdout) else {
        return fail!("Error: Unable to parse frequency as &str.");
    };

    // Get headers from qsv slice on input file
    print_status(&args, "  Getting headers...", None);
    let Ok(headers) = Command::new(&qsv_path)
        .arg("slice")
        .arg(&input_path)
        .args(["--len", "1"])
        .arg("--no-headers")
        .output()
    else {
        return fail!("Error: Error while getting headers.");
    };

    // Parse the headers as &str
    let Ok(headers_str) = std::str::from_utf8(&headers.stdout) else {
        return fail!("Error: Unable to parse headers as &str.");
    };

    print_status(&args, "Analyzed data.", Some(index_start.elapsed()));

    // Run inference options
    run_inference_options(
        &args,
        &api_key,
        Some(stats_str),
        Some(frequency_str),
        Some(headers_str),
    )?;

    // Print total elapsed time
    print_status(&args, "\ndescribegpt DONE!", Some(start_time.elapsed()));

    Ok(())
}
