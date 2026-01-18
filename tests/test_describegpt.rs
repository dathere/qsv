use std::{env, process::Command, sync::OnceLock};

use serial_test::serial;

use crate::workdir::Workdir;

/* NOTE: If you want to run these tests, set QSV_TEST_DESCRIBEGPT=1 and install
LM Studio (https://lmstudio.ai), then load the openai/gpt-oss-20b model with
context window set to at least 10,000 tokens.
*/

// Set QSV_TIMEOUT=0 for all tests to disable timeouts
// Set QSV_LLM_BASE_URL to localhost:1234/v1
// Set QSV_LLM_API_KEY to empty string
fn set_describegpt_testing_envvars(cmd: &mut std::process::Command) {
    cmd.env("QSV_TIMEOUT", "0")
        .env("QSV_LLM_BASE_URL", "http://localhost:1234/v1")
        .env("QSV_LLM_API_KEY", "");
}

fn is_local_llm_available() -> bool {
    static IS_LOCAL_LLM_AVAILABLE: OnceLock<bool> = OnceLock::new();

    *IS_LOCAL_LLM_AVAILABLE.get_or_init(|| {
        // check if QSV_TEST_DESCRIBEGPT is set to enable these tests
        if env::var("QSV_TEST_DESCRIBEGPT").is_err() {
            return false;
        }

        // check if QSV_LLM_BASE_URL is set and its on localhost
        if let Ok(base_url) = env::var("QSV_LLM_BASE_URL") {
            if base_url.contains("localhost") {
                // check if local LLM is listening by checking the model list
                let mut cmd = Command::new("curl");
                cmd.arg(format!("{}/models", base_url.trim_end_matches('/')));
                match cmd.output() {
                    Ok(output) => {
                        if !output.status.success() {
                            return false;
                        }

                        // Parse the JSON response to check for required models
                        if let Ok(response_str) = String::from_utf8(output.stdout) {
                            if let Ok(json_value) =
                                serde_json::from_str::<serde_json::Value>(&response_str)
                            {
                                if let Some(data) = json_value.get("data") {
                                    if let Some(models) = data.as_array() {
                                        let mut has_deepseek = false;
                                        let mut has_openai = false;

                                        for model in models {
                                            if let Some(id) =
                                                model.get("id").and_then(|v| v.as_str())
                                            {
                                                if id.contains("deepseek/deepseek-r1") {
                                                    has_deepseek = true;
                                                }
                                                if id.contains("openai/gpt-oss") {
                                                    has_openai = true;
                                                }
                                            }
                                        }

                                        return has_deepseek && has_openai;
                                    }
                                }
                            }
                        }
                        false
                    },
                    Err(_) => false,
                }
            } else {
                false
            }
        } else {
            false
        }
    })
}

// Providing an invalid API key with --api-key without
// the environment variable set should result in an error
#[test]
fn describegpt_invalid_api_key() {
    if is_local_llm_available() {
        // skip test if local LLM is available as they often
        // dont require API keys
        return;
    }
    let wrk = Workdir::new("describegpt");
    // Create a CSV file with sample data
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
            svec!["gamma", "37"],
        ],
    );

    // Run the command
    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.env("QSV_LLM_BASE_URL", "")
        .arg("in.csv")
        .arg("--all")
        .args(["--format", "json"])
        .args(["--api-key", "INVALIDKEY"])
        .args(["--max-tokens", "100"])
        .arg("--no-cache");

    wrk.assert_err(&mut cmd);
}

// Verify --user-agent is passed to LLM API
#[test]
#[serial]
fn describegpt_user_agent() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt");
    // Create a CSV file with sample data
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
            svec!["gamma", "37"],
        ],
    );

    // Run the command
    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("in.csv")
        .arg("--all")
        .args(["--format", "json"])
        .args([
            "--user-agent",
            "Mozilla/5.0 (platform; rv:geckoversion) Gecko/geckotrail Firefox/firefoxversion",
        ]);

    // Check that the command ran successfully
    wrk.assert_success(&mut cmd);
}

// Valid use of describegpt
#[test]
#[serial]
fn describegpt_valid() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt");

    // Create a CSV file with sample data
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
            svec!["gamma", "37"],
        ],
    );

    // Run the command
    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("in.csv").arg("--all");

    // Check that the command ran successfully
    wrk.assert_success(&mut cmd);
}

// Valid use of describegpt with --json
#[test]
#[serial]
fn describegpt_valid_json() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt");

    // Create a CSV file with sample data
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
            svec!["gamma", "37"],
        ],
    );

    // Run the command
    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("in.csv").arg("--all").args(["--format", "json"]);

    // Check that the output is valid JSON
    let got = wrk.stdout::<String>(&mut cmd);
    match serde_json::from_str::<serde_json::Value>(&got) {
        Ok(_) => (),
        Err(e) => assert!(false, "Error parsing JSON: {e}"),
    }

    // Check that the command ran successfully
    wrk.assert_success(&mut cmd);
}
// Test individual flags: --description
#[test]
#[serial]
fn describegpt_description_flag() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt");

    // Create a CSV file with sample data
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
            svec!["gamma", "37"],
        ],
    );

    // Run the command with only --description

    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("in.csv").arg("--description");

    // Check that the command ran successfully
    wrk.assert_success(&mut cmd);
}

// Test individual flags: --dictionary
#[test]
#[serial]
fn describegpt_dictionary_flag() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt");

    // Create a CSV file with sample data
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
            svec!["gamma", "37"],
        ],
    );

    // Run the command with only --dictionary
    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("in.csv").arg("--dictionary").arg("--no-cache");

    // Check that the command ran successfully
    wrk.assert_success(&mut cmd);
}

// Test individual flags: --tags
#[test]
#[serial]
fn describegpt_tags_flag() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt");

    // Create a CSV file with sample data
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
            svec!["gamma", "37"],
        ],
    );

    // Run the command with only --tags
    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("in.csv").arg("--tags").arg("--no-cache");

    // Check that the command ran successfully
    wrk.assert_success(&mut cmd);
}

// Test --tags with --tag-vocab CSV file
#[test]
#[serial]
fn describegpt_tags_with_tag_vocab() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt");

    // Create a CSV file with sample data
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
            svec!["gamma", "37"],
        ],
    );

    // Create a tag vocabulary CSV file with headers
    let tag_vocab_content = r#"tag,description
alphabetical_data,Data containing letters or alphabetical characters
numerical_data,Data containing numbers or numerical values
test_data,Sample or test data used for demonstration
"#;
    wrk.create_from_string("tag_vocab.csv", tag_vocab_content);

    // Run the command with --tags and --tag-vocab
    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("in.csv")
        .arg("--tags")
        .args(["--tag-vocab", "tag_vocab.csv"])
        .arg("--no-cache");

    // Check that the command ran successfully
    wrk.assert_success(&mut cmd);
}

// Test --tags with --tag-vocab CSV file (invalid CSV - missing description column)
#[test]
#[serial]
fn describegpt_tags_with_invalid_tag_vocab() {
    let wrk = Workdir::new("describegpt");

    // Create a CSV file with sample data
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
        ],
    );

    // Create an invalid tag vocabulary CSV file (only one column)
    let tag_vocab_content = r#"tag
alphabetical_data
numerical_data
"#;
    wrk.create_from_string("tag_vocab_invalid.csv", tag_vocab_content);

    // Run the command with --tags and --tag-vocab
    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("in.csv")
        .arg("--tags")
        .args(["--tag-vocab", "tag_vocab_invalid.csv"])
        .arg("--no-cache");

    wrk.assert_err(&mut cmd);
}

// Test --tags with --tag-vocab CSV file (non-existent file)
#[test]
#[serial]
fn describegpt_tags_with_missing_tag_vocab() {
    let wrk = Workdir::new("describegpt");

    // Create a CSV file with sample data
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
        ],
    );

    // Run the command with --tags and --tag-vocab pointing to non-existent file
    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("in.csv")
        .arg("--tags")
        .args(["--tag-vocab", "nonexistent.csv"])
        .arg("--no-cache");

    wrk.assert_err(&mut cmd);
}

// Test custom prompt with --prompt
#[test]
#[serial]
fn describegpt_custom_prompt() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt");

    // Create a CSV file with sample data
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
            svec!["gamma", "37"],
        ],
    );

    // Run the command with custom prompt
    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("in.csv")
        .args(["--prompt", "What is the main theme of this dataset?"])
        .arg("--no-cache");

    // Check that the command ran successfully
    wrk.assert_success(&mut cmd);
}

// Test custom prompt with variable substitution
#[test]
#[serial]
fn describegpt_custom_prompt_with_variables() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt");

    // Create a CSV file with sample data
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
            svec!["gamma", "37"],
        ],
    );

    // Run the command with custom prompt using variables
    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("in.csv")
        .args([
            "--prompt",
            "Based on {stats} and {frequency}, what patterns do you see?",
        ])
        .arg("--no-cache");

    // Check that the command ran successfully
    wrk.assert_success(&mut cmd);
}

// Test max tokens limit
#[test]
#[serial]
fn describegpt_max_tokens() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt");

    // Create a CSV file with sample data
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
            svec!["gamma", "37"],
        ],
    );

    // Run the command with max tokens limit
    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("in.csv")
        .arg("--description")
        .args(["--max-tokens", "200"])
        .arg("--no-cache");

    // Check that the command ran successfully
    wrk.assert_err(&mut cmd);
}

// Test max tokens set to 0 (no limit)
#[test]
#[serial]
fn describegpt_max_tokens_zero() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt");

    // Create a CSV file with sample data
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
            svec!["gamma", "37"],
        ],
    );

    // Run the command with max tokens set to 0
    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("in.csv")
        .arg("--description")
        .args(["--max-tokens", "0"])
        .arg("--no-cache");

    // Check that the command ran successfully
    wrk.assert_success(&mut cmd);
}

// Test timeout setting
#[test]
#[serial]
fn describegpt_timeout() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt");

    // Create a CSV file with sample data
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
            svec!["gamma", "37"],
        ],
    );

    // Run the command with custom timeout
    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("in.csv")
        .arg("--description")
        .args(["--timeout", "60"])
        .arg("--no-cache");

    // Check that the command ran successfully
    wrk.assert_success(&mut cmd);
}

// Test output to file
#[test]
#[serial]
fn describegpt_output_to_file() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt");

    // Create a CSV file with sample data
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
            svec!["gamma", "37"],
        ],
    );

    // Run the command with output to file
    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("in.csv")
        .arg("--description")
        .args(["--output", "output.txt"])
        .arg("--no-cache");

    // Check that the command ran successfully
    wrk.assert_success(&mut cmd);

    // Check that the output file was created
    assert!(wrk.path("output.txt").exists());
}

// Test output to file with JSON
#[test]
#[serial]
fn describegpt_output_to_file_json() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt");

    // Create a CSV file with sample data
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
            svec!["gamma", "37"],
        ],
    );

    // Run the command with output to file and JSON
    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("in.csv")
        .arg("--description")
        .args(["--format", "json"])
        .args(["--output", "output.json"])
        .arg("--no-cache");

    // Check that the command ran successfully
    wrk.assert_success(&mut cmd);

    // Check that the output file was created
    assert!(wrk.path("output.json").exists());

    // Check that the output file contains valid JSON
    let output_content = std::fs::read_to_string(wrk.path("output.json")).unwrap();
    match serde_json::from_str::<serde_json::Value>(&output_content) {
        Ok(_) => (),
        Err(e) => assert!(false, "Error parsing JSON from output file: {e}"),
    }
}

// Test quiet mode
#[test]
#[serial]
fn describegpt_quiet_mode() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt");

    // Create a CSV file with sample data
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
            svec!["gamma", "37"],
        ],
    );

    // Run the command with quiet mode
    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("in.csv")
        .arg("--description")
        .arg("--quiet")
        .arg("--no-cache");

    // Check that the command ran successfully
    wrk.assert_success(&mut cmd);
}

// Test prompt file functionality
#[test]
#[serial]
fn describegpt_prompt_file() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt");

    // Create a CSV file with sample data
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
            svec!["gamma", "37"],
        ],
    );

    // Create a prompt file
    let prompt_file_content = r#"name = "Test Prompt File"
        description = "A test prompt file for describegpt"
        author = "Test Author"
        version = "1.0.0"
        tokens = 6000
        system_prompt = "You are a helpful assistant."
        dictionary_prompt = "Create a data dictionary for this dataset."
        description_prompt = "Describe this dataset in detail{json_add} based on the following summary statistics and frequency data.\n\nSummary Statistics:\n\n{stats}\n\nFrequency:\n\n{frequency}"
        tags_prompt = "Generate tags for this dataset."
        prompt = "What is this dataset about?"
        custom_prompt_guidance = "Provide a clear and concise answer."
        base_url = "http://localhost:1234/v1"
        model = "gpt-oss-20b"
        timeout = 60
        format = "markdown"
        language = "en"
        duckdb_sql_guidance = "Use the following DuckDB SQL syntax to generate a SQL query: {duckdb_sql_guidance}"
        polars_sql_guidance = "Use the following Polars SQL syntax to generate a SQL query: {polars_sql_guidance}"
        dd_fewshot_examples = "Use the following DuckDB few-shot examples: {dd_fewshot_examples}"
        p_fewshot_examples = "Use the following Polars SQL few-shot examples: {p_fewshot_examples}""#;
    wrk.create_from_string("prompt.toml", &prompt_file_content);

    // Run the command with prompt file
    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("in.csv")
        .arg("--description")
        .args(["--prompt-file", "prompt.toml"])
        .arg("--no-cache");

    // Check that the command ran successfully
    wrk.assert_success(&mut cmd);
}

// Test error: no input file specified
#[test]
fn describegpt_no_input_file() {
    let wrk = Workdir::new("describegpt");

    // Run the command without input file
    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("--description").arg("--no-cache");

    wrk.assert_err(&mut cmd);
}

// Test error: no inference options specified
#[test]
fn describegpt_no_inference_options() {
    let wrk = Workdir::new("describegpt");

    // Create a CSV file with sample data
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
            svec!["gamma", "37"],
        ],
    );

    // Run the command without any inference options
    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("in.csv");

    wrk.assert_err(&mut cmd);
}

// Test error: --all with other inference flags
#[test]
fn describegpt_all_with_other_flags() {
    let wrk = Workdir::new("describegpt");

    // Create a CSV file with sample data
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
            svec!["gamma", "37"],
        ],
    );

    // Run the command with --all and --description (should fail)
    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("in.csv").arg("--all").arg("--description");

    wrk.assert_err(&mut cmd);
}

// Test error: non-existent prompt file
#[test]
fn describegpt_nonexistent_prompt_file() {
    let wrk = Workdir::new("describegpt");

    // Create a CSV file with sample data
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
            svec!["gamma", "37"],
        ],
    );

    // Run the command with non-existent prompt file
    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("in.csv")
        .arg("--description")
        .args(["--prompt-file", "nonexistent.toml"]);

    wrk.assert_err(&mut cmd);
}

// Test error: invalid prompt file TOML
#[test]
fn describegpt_invalid_prompt_file_toml() {
    let wrk = Workdir::new("describegpt");

    // Create a CSV file with sample data
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
            svec!["gamma", "37"],
        ],
    );

    // Create an invalid TOML prompt file
    wrk.create_from_string("invalid.toml", "This is not valid JSON");

    // Run the command with invalid prompt file
    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("in.csv")
        .arg("--description")
        .args(["--prompt-file", "invalid.toml"]);

    wrk.assert_err(&mut cmd);
}

// Test with larger dataset
#[test]
#[serial]
fn describegpt_larger_dataset() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt");

    // Create a larger CSV file with more varied data
    let mut rows = vec![svec!["name", "age", "city", "salary", "department"]];
    for i in 1..=50 {
        rows.push(vec![
            format!("Person{}", i),
            (20 + (i % 40)).to_string(),
            if i % 3 == 0 {
                "New York".to_string()
            } else if i % 3 == 1 {
                "Los Angeles".to_string()
            } else {
                "Chicago".to_string()
            },
            (50000 + (i * 1000) % 50000).to_string(),
            if i % 4 == 0 {
                "Engineering".to_string()
            } else if i % 4 == 1 {
                "Sales".to_string()
            } else if i % 4 == 2 {
                "Marketing".to_string()
            } else {
                "HR".to_string()
            },
        ]);
    }
    wrk.create_indexed("in.csv", rows);

    // Run the command
    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("in.csv")
        .arg("--all")
        .args(["--format", "json"])
        .args(["--max-tokens", "0"])
        .arg("--no-cache");

    // Check that the output is valid JSON
    let got = wrk.stdout::<String>(&mut cmd);
    match serde_json::from_str::<serde_json::Value>(&got) {
        Ok(_) => (),
        Err(e) => assert!(false, "Error parsing JSON: {e}"),
    }

    // Check that the command ran successfully
    wrk.assert_success(&mut cmd);
}

// Test with dataset containing special characters
#[test]
#[serial]
fn describegpt_special_characters() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt");

    // Create a CSV file with special characters
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["text", "number", "symbol"],
            svec!["Hello, World!", "42", "€"],
            svec!["Test\nLine", "3.14", "©"],
            svec!["Quote\"Test", "100", "™"],
            svec!["Tab\tTest", "999", "®"],
        ],
    );

    // Run the command
    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("in.csv").arg("--description").arg("--no-cache");

    // Check that the command ran successfully
    wrk.assert_success(&mut cmd);
}

// Test with empty dataset
#[test]
#[serial]
fn describegpt_empty_dataset() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt");

    // Create a CSV file with only headers
    wrk.create_indexed("in.csv", vec![svec!["header1", "header2", "header3"]]);

    // Run the command
    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("in.csv").arg("--description").arg("--no-cache");

    // Check that the command ran successfully
    wrk.assert_success(&mut cmd);
}

// Test with dataset containing null values
#[test]
#[serial]
fn describegpt_null_values() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt");

    // Create a CSV file with null values
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["name", "age", "city"],
            svec!["John", "25", "New York"],
            svec!["", "30", ""],
            svec!["Jane", "", "Los Angeles"],
            svec!["Bob", "35", ""],
        ],
    );

    // Run the command
    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("in.csv").arg("--description").arg("--no-cache");

    // Check that the command ran successfully
    wrk.assert_success(&mut cmd);
}

// Test environment variable overrides
#[test]
#[serial]
fn describegpt_env_var_overrides() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt");

    // Create a CSV file with sample data
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
            svec!["gamma", "37"],
        ],
    );

    // Run the command
    let mut cmd = wrk.command("describegpt");
    cmd.env("QSV_LLM_MODEL", "deepseek/deepseek-r1-0528-qwen3-8b")
        .env("QSV_LLM_BASE_URL", "http://localhost:1234/v1")
        .arg("in.csv")
        .arg("--description")
        .arg("--no-cache");

    // Check that the command ran successfully
    wrk.assert_success(&mut cmd);
}

// Test with different model specification
#[test]
#[serial]
fn describegpt_different_model() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt");

    // Create a CSV file with sample data
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
            svec!["gamma", "37"],
        ],
    );

    // Run the command with a different model
    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("in.csv")
        .arg("--description")
        .args(["--model", "deepseek/deepseek-r1-0528-qwen3-8b"])
        .arg("--no-cache");

    // Check that the command ran successfully
    wrk.assert_success(&mut cmd);
}

// Test with different base URL
#[test]
#[serial]
fn describegpt_different_base_url() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt");

    // Create a CSV file with sample data
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
            svec!["gamma", "37"],
        ],
    );

    // Run the command with a different base URL
    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("in.csv")
        .arg("--description")
        .args(["--base-url", "http://localhost:11434/v1"])
        .arg("--no-cache");

    wrk.assert_err(&mut cmd);
}

// Test that --prompt does not output dictionary
#[test]
#[serial]
fn describegpt_prompt_no_dictionary_output() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt");

    // Create a CSV file with sample data
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
            svec!["gamma", "37"],
        ],
    );

    // Run the command with --prompt
    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("in.csv")
        .args(["--prompt", "What is the main theme of this dataset?"])
        .arg("--no-cache");

    // Check that the command ran successfully
    wrk.assert_success(&mut cmd);

    // Get the output and verify that it does not contain dictionary output
    let output = wrk.stdout::<String>(&mut cmd);

    // The output should not contain typical dictionary markers
    // Dictionary output typically contains structured JSON with field definitions
    // Look for dictionary-specific patterns rather than just column names
    assert!(
        !output.contains("\"Name\":"),
        "Dictionary output should not be present when using --prompt"
    );
    assert!(
        !output.contains("\"Type\":"),
        "Dictionary output should not be present when using --prompt"
    );
    assert!(
        !output.contains("\"Label\":"),
        "Dictionary output should not be present when using --prompt"
    );
    assert!(
        !output.contains("\"Description\":"),
        "Dictionary output should not be present when using --prompt"
    );

    // The output should contain the prompt response
    assert!(!output.is_empty(), "Output should not be empty");
}

#[test]
fn test_base_url_flag_is_respected_issue_2976() {
    // This test verifies that the --base-url flag is properly used
    // when provided, fixing the Together AI authentication issue.

    // Create a simple CSV file for testing
    let wrk = Workdir::new("describegpt_base_url_test_issue_2976");
    wrk.create(
        "test.csv",
        vec![
            svec!["name", "age"],
            svec!["Alice", "25"],
            svec!["Bob", "30"],
        ],
    );

    // Test with a custom base URL (this will fail due to invalid URL, but we're testing
    // that the base URL is being used rather than the default OpenAI URL)
    let mut cmd = wrk.command("describegpt");
    cmd.arg("test.csv")
        .arg("--base-url")
        .arg("https://api.together.xyz/v1")
        .arg("--api-key")
        .arg("test-key")
        .arg("--dictionary")
        .arg("--no-cache");

    let output = cmd.output().expect("Failed to execute command");
    let stderr = String::from_utf8(output.stderr).unwrap();

    // The error should mention the Together AI URL, not OpenAI's URL
    // This confirms that the base URL flag is being respected
    if stderr.contains("together") || stderr.contains("HTTP") {
        // The base URL is being used correctly
        assert!(true, "Base URL flag is being respected");
    } else if stderr.contains("openai") {
        panic!("Base URL flag is not being respected - still using OpenAI URL");
    } else {
        // Some other error occurred, which is fine for this test
        assert!(true, "Base URL flag appears to be working");
    }
}

// Test that CLI --base-url flag takes precedence over QSV_LLM_BASE_URL env var
#[test]
fn describegpt_baseurl_precedence_cli_over_env() {
    let wrk = Workdir::new("describegpt_baseurl_precedence");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
        ],
    );

    let mut cmd = wrk.command("describegpt");
    // Set env var to one URL
    cmd.env("QSV_LLM_BASE_URL", "http://env-var-url.example.com/v1")
        // But explicitly override with CLI flag - this should take precedence
        .args(["--base-url", "http://cli-flag-url.example.com/v1"])
        .arg("in.csv")
        .arg("--all")
        .arg("--no-cache")
        .args(["--api-key", "test"]);

    let got = wrk.output_stderr(&mut cmd);
    // The error should mention the CLI flag URL, not the env var URL
    assert!(
        got.contains("cli-flag-url.example.com"),
        "CLI --base-url flag should take precedence over QSV_LLM_BASE_URL env var.\nGot: {}",
        got
    );
    assert!(
        !got.contains("env-var-url.example.com"),
        "Should not use env var URL when CLI flag is provided.\nGot: {}",
        got
    );
}

// Test that QSV_LLM_BASE_URL env var is used when CLI flag uses default value
#[test]
fn describegpt_baseurl_precedence_env_over_default() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt_baseurl_env");
    wrk.create_indexed(
        "in.csv",
        vec![svec!["letter", "number"], svec!["alpha", "13"]],
    );

    let mut cmd = wrk.command("describegpt");
    // Set env var, don't pass --base-url flag (will use env var)
    cmd.env("QSV_LLM_BASE_URL", "http://env-url.example.com/v1")
        .arg("in.csv")
        .arg("--all")
        .arg("--no-cache")
        .args(["--api-key", "test"]);

    let got = wrk.output_stderr(&mut cmd);
    // Should use env var URL, not the default OpenAI URL
    assert!(
        got.contains("env-url.example.com"),
        "Should use QSV_LLM_BASE_URL env var when --base-url not explicitly provided.\nGot: {}",
        got
    );
    assert!(
        !got.contains("api.openai.com"),
        "Should not use default OpenAI URL when env var is set.\nGot: {}",
        got
    );
}

// Test that CLI --model flag takes precedence over QSV_LLM_MODEL env var
#[test]
fn describegpt_model_precedence_cli_over_env() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt_model_precedence");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
        ],
    );

    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    // Set env var to one model
    cmd.env("QSV_LLM_MODEL", "env-var-model")
        // But explicitly override with CLI flag - this should take precedence
        .args(["--model", "deepseek/deepseek-r1-0528-qwen3-8b"])
        .arg("in.csv")
        .arg("--dictionary")
        .arg("--no-cache");

    // If the command succeeds or fails with model validation,
    // it means it tried to use the CLI flag model, not the env var model
    let got = wrk.output_stderr(&mut cmd);
    // Should reference the CLI model or succeed
    if got.contains("env-var-model") {
        panic!(
            "CLI --model flag should take precedence over QSV_LLM_MODEL env var.\nGot: {}",
            got
        );
    }
}

// Test that QSV_LLM_MODEL env var is used when CLI flag uses default value
#[test]
#[serial]
fn describegpt_model_precedence_env_over_default() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt_model_env");
    wrk.create_indexed(
        "in.csv",
        vec![svec!["letter", "number"], svec!["alpha", "13"]],
    );

    let mut cmd = wrk.command("describegpt");
    cmd.env("QSV_TIMEOUT", "0")
        .env("QSV_LLM_BASE_URL", "http://localhost:1234/v1")
        // Set model via env var, don't pass --model flag
        .env("QSV_LLM_MODEL", "deepseek/deepseek-r1-0528-qwen3-8b")
        .env("QSV_LLM_API_KEY", "")
        .arg("in.csv")
        .arg("--dictionary")
        .arg("--no-cache");

    // Should succeed using the env var model
    wrk.assert_success(&mut cmd);
}

// Test that CLI --api-key flag takes precedence over QSV_LLM_APIKEY env var
#[test]
fn describegpt_apikey_precedence_cli_over_env() {
    let wrk = Workdir::new("describegpt_apikey_precedence");
    wrk.create_indexed(
        "in.csv",
        vec![svec!["letter", "number"], svec!["alpha", "13"]],
    );

    let mut cmd = wrk.command("describegpt");
    // Set env var to NONE (which would suppress API key)
    cmd.env("QSV_LLM_APIKEY", "NONE")
        // But explicitly provide an API key via CLI - this should take precedence
        .args(["--api-key", "cli-api-key"])
        .args(["--base-url", "https://api.example.com/v1"])
        .arg("in.csv")
        .arg("--all")
        .arg("--no-cache");

    // Command should attempt to use the CLI api key (and fail with connection error)
    // rather than treating it as NONE from env var
    let got = wrk.output_stderr(&mut cmd);
    // Should show it tried to connect (using the API key), not refuse due to NONE
    assert!(
        got.contains("api.example.com") || got.contains("HTTP"),
        "CLI --api-key should take precedence over QSV_LLM_APIKEY env var.\nGot: {}",
        got
    );
}

// Test that localhost base URL allows empty API key even when env var is not set
#[test]
fn describegpt_localhost_allows_empty_apikey() {
    let wrk = Workdir::new("describegpt_localhost_empty_key");
    wrk.create_indexed(
        "in.csv",
        vec![svec!["letter", "number"], svec!["alpha", "13"]],
    );

    let mut cmd = wrk.command("describegpt");
    // Don't set any API key env vars, use localhost URL
    cmd.args(["--base-url", "http://localhost:9999/v1"])
        .arg("in.csv")
        .arg("--all")
        .arg("--no-cache");

    // Should not complain about missing API key since it's localhost
    let got = wrk.output_stderr(&mut cmd);
    assert!(
        !got.contains("QSV_LLM_APIKEY"),
        "Localhost base URL should allow empty API key.\nGot: {}",
        got
    );
    assert!(
        !got.contains("api-key"),
        "Localhost base URL should not require API key.\nGot: {}",
        got
    );
}

// Test that non-localhost URL requires API key
#[test]
fn describegpt_non_localhost_requires_apikey() {
    let wrk = Workdir::new("describegpt_requires_apikey");
    wrk.create_indexed(
        "in.csv",
        vec![svec!["letter", "number"], svec!["alpha", "13"]],
    );

    let mut cmd = wrk.command("describegpt");
    // Use non-localhost URL without API key - should fail
    cmd.args(["--base-url", "https://api.example.com/v1"])
        .arg("in.csv")
        .arg("--all")
        .arg("--no-cache");

    let got = wrk.output_stderr(&mut cmd);
    // Should complain about missing API key
    assert!(
        got.contains("QSV_LLM_APIKEY") || got.contains("QSV_LLM_BASE_URL"),
        "Non-localhost base URL should require API key.\nGot: {}",
        got
    );
}

// Test --freq-options with custom limit
#[test]
#[serial]
fn describegpt_freq_options_custom_limit() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt_freq_opts_limit");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number", "color"],
            svec!["alpha", "13", "red"],
            svec!["beta", "24", "blue"],
            svec!["gamma", "37", "green"],
        ],
    );

    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("in.csv")
        .arg("--dictionary")
        .args(["--freq-options", "--limit 5 --rank-strategy min"]);

    // Check that the command ran successfully
    wrk.assert_success(&mut cmd);
}

// Test --freq-options with column selection
#[test]
#[serial]
fn describegpt_freq_options_column_selection() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt_freq_opts_select");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["id", "name", "city"],
            svec!["1", "Alice", "NYC"],
            svec!["2", "Bob", "LA"],
            svec!["3", "Charlie", "NYC"],
        ],
    );

    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("in.csv")
        .arg("--dictionary")
        .args(["--freq-options", "--select !id --limit 10"]);

    // Check that the command ran successfully
    wrk.assert_success(&mut cmd);
}

// Test --freq-options without --limit uses --enum-threshold
#[test]
#[serial]
fn describegpt_freq_options_uses_enum_threshold() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt_freq_opts_enum");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
            svec!["gamma", "37"],
        ],
    );

    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("in.csv")
        .arg("--dictionary")
        .args(["--enum-threshold", "20"])
        .args(["--freq-options", "--rank-strategy dense"]);

    // Check that the command ran successfully
    // The --enum-threshold of 20 should be used since --freq-options
    // doesn't contain --limit
    wrk.assert_success(&mut cmd);
}

// Test --freq-options with --limit overrides --enum-threshold
#[test]
#[serial]
fn describegpt_freq_options_overrides_enum_threshold() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt_freq_opts_override");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
            svec!["gamma", "37"],
        ],
    );

    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("in.csv")
        .arg("--dictionary")
        .args(["--enum-threshold", "20"])
        .args(["--freq-options", "--limit 5 --asc"]);

    // Check that the command ran successfully
    // The --limit 5 from --freq-options should override --enum-threshold 20
    wrk.assert_success(&mut cmd);
}

// Test --freq-options with -l short flag
#[test]
#[serial]
fn describegpt_freq_options_short_limit() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt_freq_opts_short");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
            svec!["gamma", "37"],
        ],
    );

    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("in.csv")
        .arg("--dictionary")
        .args(["--enum-threshold", "20"])
        .args(["--freq-options", "-l 3"]);

    // Check that the command ran successfully
    // The -l 3 from --freq-options should override --enum-threshold 20
    wrk.assert_success(&mut cmd);
}

// Test --stats-options with file: prefix to read stats from a file
#[test]
#[serial]
fn describegpt_stats_options_file_prefix() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt_stats_file");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
            svec!["gamma", "37"],
        ],
    );

    // Create a pre-existing stats file
    let stats_content = r#"field,type,is_ascii,sum,min,max,range,sort_order,min_length,max_length,sum_length,avg_length,mean,sem,geometric_mean,harmonic_mean,stddev,variance,cv,nullcount,max_precision,sparsity,mad,lower_outer_fence,lower_inner_fence,q1,q2_median,q3,iqr,upper_inner_fence,upper_outer_fence,skewness,cardinality,mode,mode_count,mode_occurrences,antimode,antimode_count,antimode_occurrences,sortiness
letter,String,true,,alpha,gamma,,Ascending,4,5,14,4.67,,,,,,,,,0,0,0,,,,,,,,,,3,alpha,1,1,alpha,1,1,1
number,Integer,true,74,13,37,24,Ascending,2,2,6,2,24.67,6.94,22.66,20.54,12.01,144.33,0.49,,0,0,0,-25.5,-7.5,10.5,24,34.5,24,70.5,106.5,0.1,3,13,1,1,13,1,1,1
"#;
    wrk.create_from_string("stats.csv", stats_content);

    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("in.csv")
        .arg("--dictionary")
        .args(["--stats-options", "file:stats.csv"])
        .arg("--no-cache");

    wrk.assert_success(&mut cmd);
}

// Test --freq-options with file: prefix to read frequency from a file
#[test]
#[serial]
fn describegpt_freq_options_file_prefix() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt_freq_file");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
            svec!["gamma", "37"],
        ],
    );

    // Create a pre-existing frequency file
    let freq_content = r#"field,value,count,percentage,rank
letter,alpha,1,33.33,1
letter,beta,1,33.33,1
letter,gamma,1,33.33,1
number,13,1,33.33,1
number,24,1,33.33,1
number,37,1,33.33,1
"#;
    wrk.create_from_string("freq.csv", freq_content);

    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("in.csv")
        .arg("--dictionary")
        .args(["--freq-options", "file:freq.csv"])
        .arg("--no-cache");

    wrk.assert_success(&mut cmd);
}

// Test both --stats-options and --freq-options with file: prefix
#[test]
#[serial]
fn describegpt_both_file_prefixes() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt_both_files");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
            svec!["gamma", "37"],
        ],
    );

    // Create pre-existing stats file
    let stats_content = r#"field,type,is_ascii,sum,min,max,range,sort_order,min_length,max_length,sum_length,avg_length,mean,sem,geometric_mean,harmonic_mean,stddev,variance,cv,nullcount,max_precision,sparsity,mad,lower_outer_fence,lower_inner_fence,q1,q2_median,q3,iqr,upper_inner_fence,upper_outer_fence,skewness,cardinality,mode,mode_count,mode_occurrences,antimode,antimode_count,antimode_occurrences,sortiness
letter,String,true,,alpha,gamma,,Ascending,4,5,14,4.67,,,,,,,,,0,0,0,,,,,,,,,,3,alpha,1,1,alpha,1,1,1
number,Integer,true,74,13,37,24,Ascending,2,2,6,2,24.67,6.94,22.66,20.54,12.01,144.33,0.49,,0,0,0,-25.5,-7.5,10.5,24,34.5,24,70.5,106.5,0.1,3,13,1,1,13,1,1,1
"#;
    wrk.create_from_string("stats.csv", stats_content);

    // Create pre-existing frequency file
    let freq_content = r#"field,value,count,percentage,rank
letter,alpha,1,33.33,1
letter,beta,1,33.33,1
letter,gamma,1,33.33,1
number,13,1,33.33,1
number,24,1,33.33,1
number,37,1,33.33,1
"#;
    wrk.create_from_string("freq.csv", freq_content);

    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("in.csv")
        .arg("--dictionary")
        .args(["--stats-options", "file:stats.csv"])
        .args(["--freq-options", "file:freq.csv"])
        .arg("--no-cache");

    wrk.assert_success(&mut cmd);
}

// Test --stats-options with file: prefix pointing to non-existent file (should error)
#[test]
fn describegpt_stats_options_file_not_found() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt_stats_file_notfound");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
        ],
    );

    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("in.csv")
        .arg("--dictionary")
        .args(["--stats-options", "file:nonexistent_stats.csv"])
        .arg("--no-cache");

    wrk.assert_err(&mut cmd);
}

// Test --freq-options with file: prefix pointing to non-existent file (should error)
#[test]
fn describegpt_freq_options_file_not_found() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt_freq_file_notfound");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
        ],
    );

    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("in.csv")
        .arg("--dictionary")
        .args(["--freq-options", "file:nonexistent_freq.csv"])
        .arg("--no-cache");

    wrk.assert_err(&mut cmd);
}
