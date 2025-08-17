use std::{env, process::Command, sync::OnceLock};

use serial_test::serial;

use crate::workdir::Workdir;

// Set QSV_TIMEOUT=0 for all tests to disable timeouts
// Set QSV_LLM_BASE_URL to localhost:1234/v1
// Set QSV_LLM_API_KEY to empty string
fn set_describegpt_testing_envvars() {
    unsafe {
        env::set_var("QSV_TIMEOUT", "0");
        env::set_var("QSV_LLM_BASE_URL", "http://localhost:1234/v1");
        env::set_var("QSV_LLM_API_KEY", "");
    }
}

fn is_local_llm_available() -> bool {
    static IS_LOCAL_LLM_AVAILABLE: OnceLock<bool> = OnceLock::new();

    *IS_LOCAL_LLM_AVAILABLE.get_or_init(|| {
        // check if QSV_LLM_BASE_URL is set and its on localhost
        if let Ok(base_url) = env::var("QSV_LLM_BASE_URL") {
            if base_url.contains("localhost") {
                // check if local LLM is listening by checking the model list
                let mut cmd = Command::new("curl");
                cmd.arg(format!("{}/models", base_url.trim_end_matches('/')));
                match cmd.output() {
                    Ok(output) => output.status.success(),
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
    set_describegpt_testing_envvars();
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
    cmd.arg("in.csv")
        .arg("--all")
        .arg("--json")
        .args(["--api-key", "INVALIDKEY"])
        .args(["--max-tokens", "100"]);

    wrk.assert_err(&mut cmd);
}

// Verify --user-agent is passed to LLM API
#[test]
#[serial]
fn describegpt_user_agent() {
    set_describegpt_testing_envvars();
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
    cmd.arg("in.csv").arg("--all").arg("--json").args([
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
    set_describegpt_testing_envvars();
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
    cmd.arg("in.csv").arg("--all");

    // Check that the command ran successfully
    wrk.assert_success(&mut cmd);
}

// Valid use of describegpt with --json
#[test]
#[serial]
fn describegpt_valid_json() {
    set_describegpt_testing_envvars();
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
    cmd.arg("in.csv").arg("--all").arg("--json");

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
    set_describegpt_testing_envvars();
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
    cmd.arg("in.csv").arg("--description");

    // Check that the command ran successfully
    wrk.assert_success(&mut cmd);
}

// Test individual flags: --dictionary
#[test]
#[serial]
fn describegpt_dictionary_flag() {
    set_describegpt_testing_envvars();
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
    cmd.arg("in.csv").arg("--dictionary");

    // Check that the command ran successfully
    wrk.assert_success(&mut cmd);
}

// Test individual flags: --tags
#[test]
#[serial]
fn describegpt_tags_flag() {
    set_describegpt_testing_envvars();
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
    cmd.arg("in.csv").arg("--tags");

    // Check that the command ran successfully
    wrk.assert_success(&mut cmd);
}

// Test custom prompt with --prompt
#[test]
#[serial]
fn describegpt_custom_prompt() {
    set_describegpt_testing_envvars();
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
    cmd.arg("in.csv")
        .args(["--prompt", "What is the main theme of this dataset?"]);

    // Check that the command ran successfully
    wrk.assert_success(&mut cmd);
}

// Test custom prompt with variable substitution
#[test]
#[serial]
fn describegpt_custom_prompt_with_variables() {
    set_describegpt_testing_envvars();
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
    cmd.arg("in.csv").args([
        "--prompt",
        "Based on {stats} and {frequency}, what patterns do you see?",
    ]);

    // Check that the command ran successfully
    wrk.assert_success(&mut cmd);
}

// Test JSONL output format
#[test]
#[serial]
fn describegpt_jsonl_output() {
    set_describegpt_testing_envvars();
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

    // Run the command with --jsonl
    let mut cmd = wrk.command("describegpt");
    cmd.arg("in.csv").arg("--all").arg("--jsonl");

    // Check that the output is valid JSON
    let got = wrk.stdout::<String>(&mut cmd);
    match serde_json::from_str::<serde_json::Value>(&got) {
        Ok(_) => (),
        Err(e) => assert!(false, "Error parsing JSON: {e}"),
    }

    // Check that the command ran successfully
    wrk.assert_success(&mut cmd);
}

// Test max tokens limit
#[test]
#[serial]
fn describegpt_max_tokens() {
    set_describegpt_testing_envvars();
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
    cmd.arg("in.csv")
        .arg("--description")
        .args(["--max-tokens", "200"]);

    // Check that the command ran successfully
    wrk.assert_success(&mut cmd);
}

// Test max tokens set to 0 (no limit)
#[test]
#[serial]
fn describegpt_max_tokens_zero() {
    set_describegpt_testing_envvars();
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
    cmd.arg("in.csv")
        .arg("--description")
        .args(["--max-tokens", "0"]);

    // Check that the command ran successfully
    wrk.assert_success(&mut cmd);
}

// Test timeout setting
#[test]
#[serial]
fn describegpt_timeout() {
    set_describegpt_testing_envvars();
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
    cmd.arg("in.csv")
        .arg("--description")
        .args(["--timeout", "60"]);

    // Check that the command ran successfully
    wrk.assert_success(&mut cmd);
}

// Test output to file
#[test]
#[serial]
fn describegpt_output_to_file() {
    set_describegpt_testing_envvars();
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
    cmd.arg("in.csv")
        .arg("--description")
        .args(["--output", "output.txt"]);

    // Check that the command ran successfully
    wrk.assert_success(&mut cmd);

    // Check that the output file was created
    assert!(wrk.path("output.txt").exists());
}

// Test output to file with JSON
#[test]
#[serial]
fn describegpt_output_to_file_json() {
    set_describegpt_testing_envvars();
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
    cmd.arg("in.csv")
        .arg("--description")
        .arg("--json")
        .args(["--output", "output.json"]);

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
    set_describegpt_testing_envvars();
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
    cmd.arg("in.csv").arg("--description").arg("--quiet");

    // Check that the command ran successfully
    wrk.assert_success(&mut cmd);
}

// Test prompt file functionality
#[test]
#[serial]
fn describegpt_prompt_file() {
    set_describegpt_testing_envvars();
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
    let prompt_file_content = r#"{
        "name": "Test Prompt File",
        "description": "A test prompt file for describegpt",
        "author": "Test Author",
        "version": "1.0.0",
        "tokens": 6000,
        "system_prompt": "You are a helpful assistant.",
        "dictionary_prompt": "Create a data dictionary for this dataset.",
        "description_prompt": "Describe this dataset in detail{json_add} based on the following summary statistics and frequency data.\n\nSummary Statistics:\n\n{stats}\n\nFrequency:\n\n{frequency}",
        "tags_prompt": "Generate tags for this dataset.",
        "prompt": "What is this dataset about?",
        "json": true,
        "jsonl": false,
        "base_url": "http://localhost:1234/v1",
        "model": "gpt-oss-20b",
        "timeout": 60
    }"#;
    wrk.create_from_string("prompt.json", prompt_file_content);

    // Run the command with prompt file
    let mut cmd = wrk.command("describegpt");
    cmd.arg("in.csv")
        .arg("--description")
        .args(["--prompt-file", "prompt.json"]);

    // Check that the command ran successfully
    wrk.assert_success(&mut cmd);
}

// Test error: no input file specified
#[test]
fn describegpt_no_input_file() {
    set_describegpt_testing_envvars();
    let wrk = Workdir::new("describegpt");

    // Run the command without input file
    let mut cmd = wrk.command("describegpt");
    cmd.arg("--description");

    wrk.assert_err(&mut cmd);
}

// Test error: no inference options specified
#[test]
fn describegpt_no_inference_options() {
    set_describegpt_testing_envvars();
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
    cmd.arg("in.csv");

    wrk.assert_err(&mut cmd);
}

// Test error: --all with other inference flags
#[test]
fn describegpt_all_with_other_flags() {
    set_describegpt_testing_envvars();
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
    cmd.arg("in.csv").arg("--all").arg("--description");

    wrk.assert_err(&mut cmd);
}

// Test error: --json and --jsonl together
#[test]
fn describegpt_json_and_jsonl_together() {
    set_describegpt_testing_envvars();
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

    // Run the command with both --json and --jsonl (should fail)
    let mut cmd = wrk.command("describegpt");
    cmd.arg("in.csv")
        .arg("--description")
        .arg("--json")
        .arg("--jsonl");

    wrk.assert_err(&mut cmd);
}

// Test error: non-existent prompt file
#[test]
fn describegpt_nonexistent_prompt_file() {
    set_describegpt_testing_envvars();
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
    cmd.arg("in.csv")
        .arg("--description")
        .args(["--prompt-file", "nonexistent.json"]);

    wrk.assert_err(&mut cmd);
}

// Test error: invalid prompt file JSON
#[test]
fn describegpt_invalid_prompt_file_json() {
    set_describegpt_testing_envvars();
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

    // Create an invalid JSON prompt file
    wrk.create_from_string("invalid.json", "This is not valid JSON");

    // Run the command with invalid prompt file
    let mut cmd = wrk.command("describegpt");
    cmd.arg("in.csv")
        .arg("--description")
        .args(["--prompt-file", "invalid.json"]);

    wrk.assert_err(&mut cmd);
}

// Test with larger dataset
#[test]
#[serial]
fn describegpt_larger_dataset() {
    set_describegpt_testing_envvars();
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
    cmd.arg("in.csv")
        .arg("--all")
        .arg("--json")
        .args(["--max-tokens", "0"]);

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
    set_describegpt_testing_envvars();
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
    cmd.arg("in.csv").arg("--description");

    // Check that the command ran successfully
    wrk.assert_success(&mut cmd);
}

// Test with empty dataset
#[test]
#[serial]
fn describegpt_empty_dataset() {
    set_describegpt_testing_envvars();
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt");

    // Create a CSV file with only headers
    wrk.create_indexed("in.csv", vec![svec!["header1", "header2", "header3"]]);

    // Run the command
    let mut cmd = wrk.command("describegpt");
    cmd.arg("in.csv").arg("--description");

    // Check that the command ran successfully
    wrk.assert_success(&mut cmd);
}

// Test with dataset containing null values
#[test]
#[serial]
fn describegpt_null_values() {
    set_describegpt_testing_envvars();
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
    cmd.arg("in.csv").arg("--description");

    // Check that the command ran successfully
    wrk.assert_success(&mut cmd);
}

// Test environment variable overrides
#[test]
#[serial]
fn describegpt_env_var_overrides() {
    set_describegpt_testing_envvars();
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

    // Set environment variables
    unsafe {
        env::set_var("QSV_LLM_MODEL", "deepseek/deepseek-r1-0528-qwen3-8b");
        env::set_var("QSV_LLM_BASE_URL", "http://localhost:1234/v1");
    }

    // Run the command
    let mut cmd = wrk.command("describegpt");
    cmd.arg("in.csv").arg("--description");

    // Check that the command ran successfully
    wrk.assert_success(&mut cmd);

    // Clean up environment variables
    unsafe {
        env::remove_var("QSV_LLM_MODEL");
        env::remove_var("QSV_LLM_BASE_URL");
    }
}

// Test with different model specification
#[test]
#[serial]
fn describegpt_different_model() {
    set_describegpt_testing_envvars();
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
    cmd.arg("in.csv")
        .arg("--description")
        .args(["--model", "deepseek/deepseek-r1-0528-qwen3-8b"]);

    // Check that the command ran successfully
    wrk.assert_success(&mut cmd);
}

// Test with different base URL
#[test]
#[serial]
fn describegpt_different_base_url() {
    set_describegpt_testing_envvars();
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
    cmd.arg("in.csv")
        .arg("--description")
        .args(["--base-url", "http://localhost:11434/v1"]);

    // Check that the command ran successfully
    wrk.assert_success(&mut cmd);
}
