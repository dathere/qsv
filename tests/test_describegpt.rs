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
                        if let Ok(response_str) = String::from_utf8(output.stdout)
                            && let Ok(json_value) =
                                serde_json::from_str::<serde_json::Value>(&response_str)
                            && let Some(data) = json_value.get("data")
                            && let Some(models) = data.as_array()
                        {
                            let mut has_deepseek = false;
                            let mut has_openai = false;

                            for model in models {
                                if let Some(id) = model.get("id").and_then(|v| v.as_str()) {
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
        Err(e) => panic!("Error parsing JSON: {e}"),
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

// Test that --context-file injects its contents into the rendered prompt as the
// `{{ context }}` template variable. Uses --prepare-context so no LLM is needed:
// the prompts are rendered and emitted as JSON without calling the model.
#[test]
fn describegpt_context_file_injects_into_prompt() {
    let wrk = Workdir::new("describegpt_context");

    wrk.create(
        "in.csv",
        vec![svec!["id", "city"], svec!["1", "NYC"], svec!["2", "LA"]],
    );
    wrk.create_from_string(
        "context.md",
        "# Provenance\nData from the 2020 municipal survey. The city field uses IATA-style \
         codes.\n",
    );

    // With --context-file, the rendered system prompt must contain the context block.
    // Capture Output once and assert success before inspecting stdout, so a non-zero
    // exit that still emits partial output can't mask a failure.
    let mut cmd = wrk.command("describegpt");
    cmd.arg("in.csv")
        .arg("--dictionary")
        .arg("--prepare-context")
        .args(["--context-file", "context.md"])
        .arg("--no-cache");
    let output = cmd.output().expect("describegpt should run");
    assert!(
        output.status.success(),
        "describegpt exited non-zero\nstderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let got = String::from_utf8(output.stdout).expect("describegpt stdout should be UTF-8");
    assert!(
        got.contains("ADDITIONAL CONTEXT"),
        "context block missing from prepared prompt"
    );
    assert!(
        got.contains("municipal survey"),
        "context file contents missing from prepared prompt"
    );

    // Without --context-file, the context block must be absent.
    let mut cmd_no_ctx = wrk.command("describegpt");
    cmd_no_ctx
        .arg("in.csv")
        .arg("--dictionary")
        .arg("--prepare-context")
        .arg("--no-cache");
    let output_no_ctx = cmd_no_ctx.output().expect("describegpt should run");
    assert!(
        output_no_ctx.status.success(),
        "describegpt exited non-zero\nstderr: {}",
        String::from_utf8_lossy(&output_no_ctx.stderr)
    );
    let got_no_ctx =
        String::from_utf8(output_no_ctx.stdout).expect("describegpt stdout should be UTF-8");
    assert!(
        !got_no_ctx.contains("ADDITIONAL CONTEXT"),
        "context block should be absent when --context-file is not set"
    );
}

// Test that a missing --context-file is a hard error.
#[test]
fn describegpt_context_file_missing_errors() {
    let wrk = Workdir::new("describegpt_context_err");

    wrk.create("in.csv", vec![svec!["id"], svec!["1"]]);

    let mut cmd = wrk.command("describegpt");
    cmd.arg("in.csv")
        .arg("--dictionary")
        .arg("--prepare-context")
        .args(["--context-file", "does_not_exist.md"])
        .arg("--no-cache");

    // Run once and check both exit status and stderr, avoiding a duplicate invocation.
    let output = cmd.output().expect("describegpt should run");
    assert!(
        !output.status.success(),
        "describegpt should fail on a missing --context-file"
    );
    let got = String::from_utf8_lossy(&output.stderr);
    assert!(
        got.contains("Failed to read --context-file"),
        "expected context-file read error, got: {got}"
    );
}

// Test that a text --context-file is injected into the USER message, NOT the system prompt
// (the historical behavior was to inject into the system prompt).
#[test]
fn describegpt_context_file_moves_to_user_role() {
    let wrk = Workdir::new("describegpt_context_user_role");

    wrk.create(
        "in.csv",
        vec![svec!["id", "city"], svec!["1", "NYC"], svec!["2", "LA"]],
    );
    wrk.create_from_string(
        "context.md",
        "# Provenance\nData from the 2020 municipal survey.\n",
    );

    let mut cmd = wrk.command("describegpt");
    cmd.arg("in.csv")
        .arg("--dictionary")
        .arg("--prepare-context")
        .args(["--context-file", "context.md"])
        .arg("--no-cache");
    let output = cmd.output().expect("describegpt should run");
    assert!(
        output.status.success(),
        "describegpt exited non-zero\nstderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let got = String::from_utf8(output.stdout).expect("describegpt stdout should be UTF-8");
    let v: serde_json::Value = serde_json::from_str(&got).expect("prepare-context emits JSON");
    let phase = &v["phases"][0];
    let sys = phase["system_prompt"].as_str().expect("system_prompt");
    let usr = phase["user_prompt"].as_str().expect("user_prompt");

    assert!(
        usr.contains("ADDITIONAL CONTEXT") && usr.contains("municipal survey"),
        "context must be injected into the user prompt, got: {usr}"
    );
    assert!(
        !sys.contains("ADDITIONAL CONTEXT") && !sys.contains("municipal survey"),
        "context must NOT be in the system prompt, got: {sys}"
    );
}

// Test that a PDF --context-file is detected and surfaces an attachment MARKER in the user
// prompt (the base64 bytes ride in a separate content block built only at inference time, so
// they don't appear in --prepare-context output).
#[test]
fn describegpt_context_file_pdf_attachment_marker() {
    let wrk = Workdir::new("describegpt_context_pdf");

    wrk.create("in.csv", vec![svec!["id", "city"], svec!["1", "NYC"]]);

    // A minimal but well-formed PDF (the "%PDF-" signature drives MIME detection).
    wrk.create_from_string(
        "notes.pdf",
        "%PDF-1.4\n1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n2 0 obj\n<< /Type /Pages \
         /Kids [3 0 R] /Count 1 >>\nendobj\n3 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 \
         612 792] >>\nendobj\nxref\n0 4\n0000000000 65535 f \ntrailer\n<< /Root 1 0 R /Size 4 \
         >>\nstartxref\n0\n%%EOF\n",
    );

    let mut cmd = wrk.command("describegpt");
    cmd.arg("in.csv")
        .arg("--dictionary")
        .arg("--prepare-context")
        .args(["--context-file", "notes.pdf"])
        .arg("--no-cache");
    let output = cmd.output().expect("describegpt should run");
    assert!(
        output.status.success(),
        "describegpt exited non-zero\nstderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let got = String::from_utf8(output.stdout).expect("describegpt stdout should be UTF-8");
    let v: serde_json::Value = serde_json::from_str(&got).expect("prepare-context emits JSON");
    let phase = &v["phases"][0];
    let usr = phase["user_prompt"].as_str().expect("user_prompt");

    assert!(
        usr.contains("notes.pdf") && usr.contains("Attached document"),
        "PDF attachment marker must appear in the user prompt, got: {usr}"
    );
    // The raw base64 file bytes must NOT leak into the rendered prompt text.
    assert!(
        !usr.contains("base64,"),
        "base64 data must not appear in the prepared prompt text, got: {usr}"
    );
}

// Test that --dictionary --infer-content-type infers inter-column relationships
// and emits them as a structurally-valid `relationships` array (consumed by
// `synthesize` to preserve inter-column structure).
#[test]
#[serial]
fn describegpt_dictionary_relationships() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt");

    // A dataset with three obvious inter-column relationships: a date ordering
    // (created <= closed), a numeric ordering (subtotal <= total), and a
    // categorical functional dependency (city -> state).
    wrk.create_indexed(
        "in.csv",
        vec![
            svec![
                "created_date",
                "closed_date",
                "city",
                "state",
                "subtotal",
                "total"
            ],
            svec!["2023-01-05", "2023-01-20", "Boston", "MA", "100", "108"],
            svec!["2023-02-10", "2023-03-15", "Chicago", "IL", "250", "270"],
            svec!["2023-03-01", "2023-03-05", "Boston", "MA", "50", "54"],
            svec!["2023-04-12", "2023-06-01", "Chicago", "IL", "500", "540"],
            svec!["2023-05-20", "2023-05-28", "Boston", "MA", "75", "81"],
            svec!["2023-06-15", "2023-08-22", "Chicago", "IL", "320", "346"],
        ],
    );

    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("in.csv")
        .arg("--dictionary")
        .arg("--infer-content-type")
        .args(["--format", "JSON"])
        .arg("--no-cache");

    // Capture and assert success on the same run, so a non-zero exit that
    // still emits parseable JSON on stdout cannot mask a failure.
    let output = cmd.output().expect("describegpt should run");
    assert!(
        output.status.success(),
        "describegpt exited non-zero\nstderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).expect("describegpt stdout should be UTF-8");
    let parsed: serde_json::Value =
        serde_json::from_str(&stdout).expect("describegpt JSON output should parse");
    let response = parsed
        .get("Dictionary")
        .and_then(|d| d.get("response"))
        .expect("describegpt JSON should have Dictionary.response");
    let relationships = response
        .get("relationships")
        .and_then(|v| v.as_array())
        .expect("dictionary should carry a relationships array for an obviously-related dataset");

    assert!(
        !relationships.is_empty(),
        "the LLM should infer at least one relationship for this dataset"
    );
    for rel in relationships {
        let kind = rel.get("kind").and_then(|v| v.as_str()).unwrap_or_default();
        assert!(
            matches!(kind, "joint" | "ordered" | "correlated"),
            "unexpected relationship kind in {rel}"
        );
        let members = rel
            .get("members")
            .and_then(|v| v.as_array())
            .expect("every relationship must have a members array");
        assert!(members.len() >= 2, "relationship needs >= 2 members: {rel}");
        assert!(
            members.iter().all(serde_json::Value::is_string),
            "relationship members must be strings: {rel}"
        );
    }
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
        Err(e) => panic!("Error parsing JSON from output file: {e}"),
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
    wrk.create_from_string("prompt.toml", prompt_file_content);

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
        Err(e) => panic!("Error parsing JSON: {e}"),
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

// Test with empty dataset (header row only, zero data rows)
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

    // describegpt must reject a dataset with no data rows: its first step is to
    // run `stats`, which cannot compile summary statistics for an empty file.
    wrk.assert_err(&mut cmd);
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
        // The base URL is being used correctly (respected)
    } else if stderr.contains("openai") {
        panic!("Base URL flag is not being respected - still using OpenAI URL");
    } else {
        // Some other error occurred, which is fine for this test (base URL flag appears to be
        // working)
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

// Regression test for codex review job 2363: an explicit `--base-url`
// that happens to match the documented default URL must still beat
// QSV_LLM_BASE_URL. Before the fix, the sentinel-based precedence
// couldn't tell the explicit-default-URL case from "no CLI flag" — both
// produced `flag_base_url == Some(DEFAULT_BASE_URL)` — so the env var
// silently overrode the explicit CLI flag.
#[test]
fn describegpt_baseurl_precedence_cli_default_over_env() {
    let wrk = Workdir::new("describegpt_baseurl_cli_default_over_env");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
        ],
    );

    let mut cmd = wrk.command("describegpt");
    // Env var points at a URL that would be obviously wrong if it leaked
    // through. The CLI flag explicitly pins the documented default
    // localhost URL — the precedence MUST honor it.
    cmd.env("QSV_LLM_BASE_URL", "http://env-url.example.com/v1")
        .args(["--base-url", "http://localhost:1234/v1"])
        .arg("in.csv")
        .arg("--all")
        .arg("--no-cache")
        .args(["--api-key", "test"]);

    let got = wrk.output_stderr(&mut cmd);
    assert!(
        !got.contains("env-url.example.com"),
        "Explicit --base-url http://localhost:1234/v1 must override QSV_LLM_BASE_URL even when \
         its value equals the documented default.\nGot: {}",
        got
    );
}

// Regression test for codex review job 2372: when --prompt-file points
// at a non-localhost provider and neither --base-url nor QSV_LLM_BASE_URL
// is set, the api-key gate MUST require an API key (the request will go
// to the prompt-file URL, not localhost). Before the fix, the localhost
// gate read the CLI > env > built-in-default URL only — which was
// "http://localhost:1234/v1" — passed the localhost check, and allowed
// describegpt to fire unauthenticated requests at the prompt-file URL.
#[test]
fn describegpt_baseurl_remote_prompt_file_requires_api_key() {
    let wrk = Workdir::new("describegpt_baseurl_remote_prompt_file");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
        ],
    );

    // Prompt file pointing at a non-localhost provider. We never actually
    // hit the URL — the api-key gate runs before any request — but its
    // host is what determines whether the gate fires. Every field on
    // PromptFile (other than dictionary_refine_prompt, which has a
    // serde default) is required by the TOML deserializer, so populate
    // them all here.
    let prompt_file_content = r#"name = "test"
description = "test prompt file pointing at a remote provider"
author = "test"
version = "1.0"
tokens = 0
base_url = "https://api.openai.com/v1"
model = "openai/gpt-oss-20b"
timeout = 30
format = "markdown"
language = ""
system_prompt = "test"
dictionary_prompt = "test"
description_prompt = "test"
tags_prompt = "test"
prompt = ""
custom_prompt_guidance = ""
duckdb_sql_guidance = ""
polars_sql_guidance = ""
dd_fewshot_examples = ""
p_fewshot_examples = ""
"#;
    wrk.create_from_string("prompt.toml", prompt_file_content);

    let mut cmd = wrk.command("describegpt");
    // Deliberately don't set QSV_LLM_BASE_URL or pass --base-url; don't
    // pass --api-key either. The prompt_file's base_url is remote, so
    // the api-key gate MUST fire and reject the run.
    cmd.env_remove("QSV_LLM_BASE_URL")
        .env_remove("QSV_LLM_APIKEY")
        .args(["--prompt-file", "prompt.toml"])
        .arg("in.csv")
        .arg("--all")
        .arg("--no-cache");

    wrk.assert_err(&mut cmd);
    let got = wrk.output_stderr(&mut cmd);
    assert!(
        got.contains("QSV_LLM_APIKEY") || got.to_lowercase().contains("api"),
        "Remote prompt-file URL must trigger the missing-API-key error.\nGot: {}",
        got
    );
}

// Regression test for codex review job 2373: --prepare-context emits
// prompt/context JSON locally and never calls the LLM API, so it must
// NOT require credentials even when the prompt-file points at a
// non-localhost provider. Before the fix, the api-key gate ran ahead of
// the --prepare-context branch and rejected the run.
#[test]
fn describegpt_prepare_context_remote_prompt_file_no_api_key() {
    let wrk = Workdir::new("describegpt_prepare_context_remote_prompt_file");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
        ],
    );

    let prompt_file_content = r#"name = "test"
description = "test prompt file pointing at a remote provider"
author = "test"
version = "1.0"
tokens = 0
base_url = "https://api.openai.com/v1"
model = "openai/gpt-oss-20b"
timeout = 30
format = "markdown"
language = ""
system_prompt = "test"
dictionary_prompt = "test"
description_prompt = "test"
tags_prompt = "test"
prompt = ""
custom_prompt_guidance = ""
duckdb_sql_guidance = ""
polars_sql_guidance = ""
dd_fewshot_examples = ""
p_fewshot_examples = ""
"#;
    wrk.create_from_string("prompt.toml", prompt_file_content);

    let mut cmd = wrk.command("describegpt");
    cmd.env_remove("QSV_LLM_BASE_URL")
        .env_remove("QSV_LLM_APIKEY")
        .args(["--prompt-file", "prompt.toml"])
        .arg("--prepare-context")
        .arg("in.csv")
        .arg("--all")
        .arg("--no-cache");

    wrk.assert_success(&mut cmd);
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
letter,String,true,,alpha,gamma,,Ascending,4,5,14,4.67,,,,,,,,0,0,0,0,,,,,,,,,,3,alpha,1,1,alpha,1,1,1
number,Integer,true,74,13,37,24,Ascending,2,2,6,2,24.67,6.94,22.66,20.54,12.01,144.33,0.49,0,0,0,0,-25.5,-7.5,10.5,24,34.5,24,70.5,106.5,0.1,3,13,1,1,13,1,1,1
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
letter,String,true,,alpha,gamma,,Ascending,4,5,14,4.67,,,,,,,,0,0,0,0,,,,,,,,,,3,alpha,1,1,alpha,1,1,1
number,Integer,true,74,13,37,24,Ascending,2,2,6,2,24.67,6.94,22.66,20.54,12.01,144.33,0.49,0,0,0,0,-25.5,-7.5,10.5,24,34.5,24,70.5,106.5,0.1,3,13,1,1,13,1,1,1
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

// Test that frequency bucket "Other" entries get "…" suffix to disambiguate
// from literal "Other" values in the Examples column
#[test]
#[serial]
fn describegpt_other_bucket_disambiguation() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt_other_disambig");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["pickup_location", "value"],
            svec!["JFK Airport", "1"],
            svec!["Other", "2"],
            svec!["LaGuardia", "3"],
        ],
    );

    // Create a frequency file where "Other (5)" has rank=0 (bucket entry)
    // and literal "Other" has rank=2 (actual data value)
    let freq_content = r#"field,value,count,percentage,rank
pickup_location,Other (5),10,50.0,0
pickup_location,Other,4,20.0,2
pickup_location,JFK Airport,3,15.0,1
pickup_location,LaGuardia,3,15.0,1
value,1,1,33.33,1
value,2,1,33.33,1
value,3,1,33.33,1
"#;
    wrk.create_from_string("freq.csv", freq_content);

    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("in.csv")
        .arg("--dictionary")
        .args(["--freq-options", "file:freq.csv"])
        .args(["--format", "json"])
        .arg("--no-cache");

    let got = wrk.stdout::<String>(&mut cmd);

    // The bucket "Other (5)" should become "Other…" (stripped parens + ellipsis)
    assert!(
        got.contains("Other…"),
        "Bucket 'Other' entry should have '…' suffix for disambiguation.\nGot: {got}"
    );
    // The literal "Other" value should remain as-is (no ellipsis)
    assert!(
        got.contains("Other [4]"),
        "Literal 'Other' value should not have '…' suffix.\nGot: {got}"
    );
}

// ====== MCP Sampling Mode Tests ======
// These tests verify the --prepare-context and --process-response flags
// without requiring an LLM.

#[test]
fn describegpt_prepare_context_dictionary() {
    let wrk = Workdir::new("describegpt_prepare_context_dict");
    wrk.create_indexed(
        "data.csv",
        vec![svec!["name", "age", "city"], svec!["Alice", "30", "NYC"]],
    );

    let mut cmd = wrk.command("describegpt");
    cmd.arg("--prepare-context")
        .arg("--dictionary")
        .arg("--no-cache")
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let json: serde_json::Value = serde_json::from_str(&got).unwrap();

    // Verify top-level structure
    assert!(json.get("phases").is_some(), "Should have phases");
    assert!(
        json.get("analysis_results").is_some(),
        "Should have analysis_results"
    );
    assert!(json.get("model").is_some(), "Should have model");
    assert!(json.get("max_tokens").is_some(), "Should have max_tokens");

    // Verify phase structure
    let phases = json["phases"].as_array().unwrap();
    assert_eq!(phases.len(), 1, "Should have 1 phase for --dictionary");
    assert_eq!(phases[0]["kind"], "Dictionary");
    assert!(
        phases[0]["system_prompt"].is_string(),
        "Should have system_prompt"
    );
    assert!(
        phases[0]["user_prompt"].is_string(),
        "Should have user_prompt"
    );
    assert!(phases[0]["cache_key"].is_string(), "Should have cache_key");
    assert!(
        phases[0]["cached_response"].is_null(),
        "Should have null cached_response with --no-cache"
    );
}

#[test]
fn describegpt_prepare_context_all() {
    let wrk = Workdir::new("describegpt_prepare_context_all");
    wrk.create_indexed(
        "data.csv",
        vec![
            svec!["name", "age"],
            svec!["Alice", "30"],
            svec!["Bob", "25"],
        ],
    );

    let mut cmd = wrk.command("describegpt");
    cmd.arg("--prepare-context")
        .arg("--all")
        .arg("--no-cache")
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let json: serde_json::Value = serde_json::from_str(&got).unwrap();

    let phases = json["phases"].as_array().unwrap();
    assert_eq!(phases.len(), 3, "Should have 3 phases for --all");

    let kinds: Vec<&str> = phases.iter().map(|p| p["kind"].as_str().unwrap()).collect();
    assert!(kinds.contains(&"Dictionary"), "Should include Dictionary");
    assert!(kinds.contains(&"Description"), "Should include Description");
    assert!(kinds.contains(&"Tags"), "Should include Tags");
}

#[test]
fn describegpt_prepare_context_and_process_response_mutually_exclusive() {
    let wrk = Workdir::new("describegpt_mutual_exclusive");
    wrk.create_indexed("data.csv", vec![svec!["a", "b"], svec!["1", "2"]]);

    let mut cmd = wrk.command("describegpt");
    cmd.arg("--prepare-context")
        .arg("--process-response")
        .arg("--dictionary")
        .arg("data.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
fn describegpt_process_response_produces_output() {
    let wrk = Workdir::new("describegpt_process_response");
    wrk.create_indexed(
        "data.csv",
        vec![
            svec!["name", "age", "city"],
            svec!["Alice", "30", "NYC"],
            svec!["Bob", "25", "LA"],
        ],
    );

    // Phase 1: Get prepare-context output
    let mut cmd = wrk.command("describegpt");
    cmd.arg("--prepare-context")
        .arg("--description")
        .arg("--no-cache")
        .arg("data.csv");

    let prep_json: String = wrk.stdout(&mut cmd);
    let prep: serde_json::Value = serde_json::from_str(&prep_json).unwrap();

    // Build process-response input with mock LLM responses
    let phases: Vec<serde_json::Value> = prep["phases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| {
            serde_json::json!({
                "kind": p["kind"],
                "response": if p["kind"] == "Dictionary" {
                    "Field: name\nLabel: Name\nDescription: Person name\n\nField: age\nLabel: Age\nDescription: Person age\n\nField: city\nLabel: City\nDescription: City name"
                } else {
                    "This dataset contains demographic information about people."
                },
                "reasoning": "Test reasoning",
                "token_usage": {"prompt": 100, "completion": 50, "total": 150, "elapsed": 500}
            })
        })
        .collect();

    let process_input = serde_json::json!({
        "phases": phases,
        "analysis_results": prep["analysis_results"],
        "model": prep["model"]
    });

    // Phase 2: Run --process-response with mock data via stdin
    // Use std::process::Command with stdin piping
    use std::{io::Write, process::Stdio};

    let mut cmd_2 = wrk.command("describegpt");
    cmd_2
        .arg("--process-response")
        .arg("--description")
        .arg("--no-cache")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = cmd_2.spawn().unwrap();
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(process_input.to_string().as_bytes())
        .unwrap();

    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "process-response should succeed. stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!stdout.is_empty(), "Should produce output");
    assert!(
        stdout.contains("Description") || stdout.contains("dataset"),
        "Output should contain description content. Got: {stdout}"
    );
}

#[test]
fn describegpt_prepare_context_analysis_results_structure() {
    let wrk = Workdir::new("describegpt_prep_analysis");
    wrk.create_indexed(
        "data.csv",
        vec![svec!["x", "y"], svec!["1", "2"], svec!["3", "4"]],
    );

    let mut cmd = wrk.command("describegpt");
    cmd.arg("--prepare-context")
        .arg("--dictionary")
        .arg("--no-cache")
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let json: serde_json::Value = serde_json::from_str(&got).unwrap();

    let ar = &json["analysis_results"];
    assert!(
        ar["stats"].is_string(),
        "analysis_results should have stats"
    );
    assert!(
        ar["frequency"].is_string(),
        "analysis_results should have frequency"
    );
    assert!(
        ar["headers"].is_string(),
        "analysis_results should have headers"
    );
    assert!(
        ar["file_hash"].is_string(),
        "analysis_results should have file_hash"
    );
}

// =========================================================================
// scoresql integration tests (--no-score-sql, --score-threshold, --score-max-retries)
// =========================================================================

fn is_duckdb_available() -> bool {
    std::env::var("QSV_DUCKDB_PATH").is_ok_and(|val| !val.is_empty())
}

// Test that --no-score-sql disables scoring and the command still works with --prompt +
// --sql-results
#[test]
#[serial]
fn describegpt_no_score_sql_flag() {
    if !is_local_llm_available() {
        eprintln!("Skipping: local LLM not available");
        return;
    }
    let wrk = Workdir::new("describegpt_no_score_sql");
    wrk.create_indexed(
        "data.csv",
        vec![
            svec!["name", "age", "score"],
            svec!["Alice", "30", "85"],
            svec!["Bob", "25", "92"],
            svec!["Carol", "35", "78"],
        ],
    );

    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("data.csv")
        .args([
            "--prompt",
            "Write a SQL query to select all rows where age > 28. Return ONLY a SQL query in a \
             ```sql code block.",
        ])
        .args(["--sql-results", "results.csv"])
        .arg("--no-score-sql")
        .arg("--no-cache");

    let got = wrk.output(&mut cmd);
    let stderr = String::from_utf8_lossy(&got.stderr);

    // Command should succeed
    assert!(
        got.status.success(),
        "Command should succeed with --no-score-sql. stderr: {stderr}"
    );
    // Should NOT contain any SQL score messages
    assert!(
        !stderr.contains("SQL score:"),
        "Expected no scoring output with --no-score-sql, but got: {stderr}"
    );
}

// Test that scoring is enabled by default when --prompt + --sql-results are used
// and that stderr contains scoring status messages (polars feature enabled)
#[test]
#[serial]
fn describegpt_score_sql_enabled_by_default() {
    if !is_local_llm_available() {
        eprintln!("Skipping: local LLM not available");
        return;
    }
    let wrk = Workdir::new("describegpt_score_enabled");
    wrk.create_indexed(
        "data.csv",
        vec![
            svec!["name", "age", "score"],
            svec!["Alice", "30", "85"],
            svec!["Bob", "25", "92"],
            svec!["Carol", "35", "78"],
            svec!["Dave", "28", "95"],
            svec!["Eve", "32", "88"],
        ],
    );

    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("data.csv")
        .args([
            "--prompt",
            "Write a SQL query to select all rows where age > 28. Return ONLY a SQL query in a \
             ```sql code block.",
        ])
        .args(["--sql-results", "results.csv"])
        .arg("--no-cache");

    let got = wrk.output(&mut cmd);
    let stderr = String::from_utf8_lossy(&got.stderr);

    // Command should succeed
    assert!(
        got.status.success(),
        "Command should succeed with default scoring. stderr: {stderr}"
    );
    // Should contain SQL score output since scoring is enabled by default
    assert!(
        stderr.contains("SQL score:"),
        "Expected scoring output by default, stderr: {stderr}"
    );
    // Should contain attempt indicator
    assert!(
        stderr.contains("[attempt"),
        "Expected attempt indicator in scoring output, stderr: {stderr}"
    );
}

// Test that --score-threshold 0 accepts any query immediately (no retries)
#[test]
#[serial]
fn describegpt_score_threshold_zero_accepts_immediately() {
    if !is_local_llm_available() {
        eprintln!("Skipping: local LLM not available");
        return;
    }
    let wrk = Workdir::new("describegpt_score_threshold_zero");
    wrk.create_indexed(
        "data.csv",
        vec![
            svec!["name", "age", "score"],
            svec!["Alice", "30", "85"],
            svec!["Bob", "25", "92"],
            svec!["Carol", "35", "78"],
        ],
    );

    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("data.csv")
        .args([
            "--prompt",
            "Write a SQL query to select all rows where age > 28. Return ONLY a SQL query in a \
             ```sql code block.",
        ])
        .args(["--sql-results", "results.csv"])
        .args(["--score-threshold", "0"])
        .arg("--no-cache");

    let got = wrk.output(&mut cmd);
    let stderr = String::from_utf8_lossy(&got.stderr);

    // Command should succeed
    assert!(
        got.status.success(),
        "Command should succeed with threshold 0. stderr: {stderr}"
    );
    // With threshold 0, the first attempt should be accepted (score >= 0 is always true)
    assert!(
        stderr.contains("[attempt 1]"),
        "Expected attempt 1 in output, stderr: {stderr}"
    );
    // Should NOT have attempt 2 since threshold 0 accepts any score
    assert!(
        !stderr.contains("[attempt 2]"),
        "Expected no retry with threshold 0, stderr: {stderr}"
    );
}

// Test that --score-max-retries 0 scores once and does not retry
#[test]
#[serial]
fn describegpt_score_max_retries_zero() {
    if !is_local_llm_available() {
        eprintln!("Skipping: local LLM not available");
        return;
    }
    let wrk = Workdir::new("describegpt_score_max_retries_zero");
    wrk.create_indexed(
        "data.csv",
        vec![
            svec!["name", "age", "score"],
            svec!["Alice", "30", "85"],
            svec!["Bob", "25", "92"],
            svec!["Carol", "35", "78"],
        ],
    );

    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("data.csv")
        .args([
            "--prompt",
            "Write a SQL query to select all rows where age > 28. Return ONLY a SQL query in a \
             ```sql code block.",
        ])
        .args(["--sql-results", "results.csv"])
        .args(["--score-max-retries", "0"])
        .arg("--no-cache");

    let got = wrk.output(&mut cmd);
    let stderr = String::from_utf8_lossy(&got.stderr);

    // Command should succeed
    assert!(
        got.status.success(),
        "Command should succeed with max-retries 0. stderr: {stderr}"
    );
    // With max-retries 0, the loop runs once (attempt 1 only)
    assert!(
        stderr.contains("[attempt 1]"),
        "Expected attempt 1 in output, stderr: {stderr}"
    );
    // Should NOT have attempt 2
    assert!(
        !stderr.contains("[attempt 2]"),
        "Expected no retry with max-retries 0, stderr: {stderr}"
    );
}

// Test that a high threshold triggers retries
#[test]
#[serial]
fn describegpt_score_high_threshold_triggers_retries() {
    if !is_local_llm_available() {
        eprintln!("Skipping: local LLM not available");
        return;
    }
    let wrk = Workdir::new("describegpt_score_high_threshold");
    wrk.create_indexed(
        "data.csv",
        vec![
            svec!["name", "age", "score"],
            svec!["Alice", "30", "85"],
            svec!["Bob", "25", "92"],
            svec!["Carol", "35", "78"],
            svec!["Dave", "28", "95"],
            svec!["Eve", "32", "88"],
        ],
    );

    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("data.csv")
        .args([
            "--prompt",
            "Write a SQL query using SELECT * FROM data. Return ONLY a SQL query in a ```sql code \
             block.",
        ])
        .args(["--sql-results", "results.csv"])
        // Use threshold 101 (above max score of 100) to guarantee retry/warning
        .args(["--score-threshold", "101"])
        .args(["--score-max-retries", "1"])
        .arg("--no-cache");

    let got = wrk.output(&mut cmd);
    let stderr = String::from_utf8_lossy(&got.stderr);

    // Command should still succeed (uses best query even when threshold not met)
    assert!(
        got.status.success(),
        "Command should succeed even with unreachable threshold. stderr: {stderr}"
    );
    // With threshold 101 (impossible to reach), must retry and show warning
    assert!(
        stderr.contains("[attempt 1]"),
        "Expected attempt 1 in output, stderr: {stderr}"
    );
    // With threshold 101, attempt 1 can never be accepted, so the loop always
    // starts a second iteration. Any of these messages proves the retry path
    // ran — which exact one appears depends on the (non-deterministic) refined
    // query: it may score ([attempt 2] / below threshold), fail to validate
    // (scoresql failed on all attempts), or come back with no SQL block.
    let has_retry_or_warning = stderr.contains("[attempt 2]")
        || stderr.contains("below threshold")
        || stderr.contains("scoresql failed on all attempts")
        || stderr.contains("LLM refinement had no SQL block");
    assert!(
        has_retry_or_warning,
        "Expected retry/threshold evidence with score-threshold 101, stderr: {stderr}"
    );
}

// Test scoring with DuckDB backend when QSV_DUCKDB_PATH is available
#[test]
#[serial]
fn describegpt_score_sql_with_duckdb() {
    if !is_local_llm_available() || !is_duckdb_available() {
        eprintln!("Skipping: local LLM or DuckDB not available");
        return;
    }
    let wrk = Workdir::new("describegpt_score_duckdb");
    wrk.create_indexed(
        "data.csv",
        vec![
            svec!["name", "age", "score"],
            svec!["Alice", "30", "85"],
            svec!["Bob", "25", "92"],
            svec!["Carol", "35", "78"],
            svec!["Dave", "28", "95"],
            svec!["Eve", "32", "88"],
        ],
    );

    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    // Pass QSV_DUCKDB_PATH through to the subprocess
    if let Ok(duckdb_path) = env::var("QSV_DUCKDB_PATH") {
        cmd.env("QSV_DUCKDB_PATH", duckdb_path);
    }
    cmd.arg("data.csv")
        .args([
            "--prompt",
            "Write a SQL query to find the average age. Return ONLY a SQL query in a ```sql code \
             block.",
        ])
        .args(["--sql-results", "results.csv"])
        .args(["--score-threshold", "0"])
        .arg("--no-cache");

    let got = wrk.output(&mut cmd);
    let stderr = String::from_utf8_lossy(&got.stderr);

    // Scoring should work with DuckDB
    assert!(
        stderr.contains("SQL score:"),
        "Expected scoring output with DuckDB, stderr: {stderr}"
    );
    // Should succeed on first attempt with threshold 0
    assert!(
        got.status.success(),
        "Command should succeed with DuckDB scoring. stderr: {stderr}"
    );
}

// Test that the SQL results file is actually created when scoring is enabled
#[test]
#[serial]
fn describegpt_score_sql_produces_results_file() {
    if !is_local_llm_available() {
        eprintln!("Skipping: local LLM not available");
        return;
    }
    let wrk = Workdir::new("describegpt_score_results_file");
    wrk.create_indexed(
        "data.csv",
        vec![
            svec!["name", "age", "score"],
            svec!["Alice", "30", "85"],
            svec!["Bob", "25", "92"],
            svec!["Carol", "35", "78"],
        ],
    );

    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("data.csv")
        .args([
            "--prompt",
            "Write a SQL query to select name and age where age > 25. Return ONLY a SQL query in \
             a ```sql code block.",
        ])
        .args(["--sql-results", "results.csv"])
        // Use threshold 0 to accept any query and proceed to execution
        .args(["--score-threshold", "0"])
        .arg("--no-cache");

    wrk.assert_success(&mut cmd);

    // Verify the results file was created
    let results_path = wrk.path("results.csv");
    assert!(
        results_path.exists(),
        "SQL results file should be created at {}",
        results_path.display()
    );

    // Verify the file has content
    let content = std::fs::read_to_string(&results_path).unwrap();
    assert!(!content.is_empty(), "SQL results file should not be empty");
}

// Test scoring with DuckDB and high threshold triggers retries
#[test]
#[serial]
fn describegpt_score_duckdb_high_threshold_retries() {
    if !is_local_llm_available() || !is_duckdb_available() {
        eprintln!("Skipping: local LLM or DuckDB not available");
        return;
    }
    let wrk = Workdir::new("describegpt_score_duckdb_retries");
    wrk.create_indexed(
        "data.csv",
        vec![
            svec!["name", "age", "score"],
            svec!["Alice", "30", "85"],
            svec!["Bob", "25", "92"],
            svec!["Carol", "35", "78"],
            svec!["Dave", "28", "95"],
            svec!["Eve", "32", "88"],
        ],
    );

    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    if let Ok(duckdb_path) = env::var("QSV_DUCKDB_PATH") {
        cmd.env("QSV_DUCKDB_PATH", duckdb_path);
    }
    cmd.arg("data.csv")
        .args([
            "--prompt",
            "Write a SQL query using SELECT * FROM data. Return ONLY a SQL query in a ```sql code \
             block.",
        ])
        .args(["--sql-results", "results.csv"])
        // Use threshold 101 (above max score of 100) to guarantee retry/warning
        .args(["--score-threshold", "101"])
        .args(["--score-max-retries", "2"])
        .arg("--no-cache");

    let got = wrk.output(&mut cmd);
    let stderr = String::from_utf8_lossy(&got.stderr);

    // Command should still succeed (uses best query even when threshold not met)
    assert!(
        got.status.success(),
        "Command should succeed even with unreachable DuckDB threshold. stderr: {stderr}"
    );
    // With threshold 101 and DuckDB, must attempt scoring and retry
    assert!(
        stderr.contains("SQL score:"),
        "Expected scoring output with DuckDB and high threshold, stderr: {stderr}"
    );
    // Should see warning or multiple attempts
    let has_retry_activity = stderr.contains("[attempt 2]") || stderr.contains("below threshold");
    assert!(
        has_retry_activity,
        "Expected retry activity with threshold 101, stderr: {stderr}"
    );
}

// --format jsonschema CLI validation: rejects when neither --dictionary nor --all is set.
// Non-LLM test (validation runs before any LLM call).
#[test]
fn describegpt_jsonschema_requires_dictionary() {
    let wrk = Workdir::new("describegpt_jsonschema_requires_dictionary");
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
        .args(["--format", "jsonschema"])
        .arg("--description")
        .arg("--no-cache");

    let got = wrk.output(&mut cmd);
    assert!(
        !got.status.success(),
        "Expected --format jsonschema without --dictionary to fail"
    );
    let stderr = String::from_utf8_lossy(&got.stderr);
    assert!(
        stderr.contains("--format jsonschema requires --dictionary"),
        "stderr did not mention the dictionary requirement: {stderr}"
    );
}

// --format semanticmd CLI validation: rejects when neither --dictionary nor --all is set.
// Non-LLM test (validation runs before any LLM call).
#[test]
fn describegpt_semanticmd_requires_dictionary() {
    let wrk = Workdir::new("describegpt_semanticmd_requires_dictionary");
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
        .args(["--format", "semanticmd"])
        .arg("--description")
        .arg("--no-cache");

    let got = wrk.output(&mut cmd);
    assert!(
        !got.status.success(),
        "Expected --format semanticmd without --dictionary to fail"
    );
    let stderr = String::from_utf8_lossy(&got.stderr);
    assert!(
        stderr.contains("--format semanticmd requires --dictionary"),
        "stderr did not mention the dictionary requirement: {stderr}"
    );
}

// --format semanticmd is incompatible with --prompt. Non-LLM test.
#[test]
fn describegpt_semanticmd_prompt_incompatible() {
    let wrk = Workdir::new("describegpt_semanticmd_prompt_incompatible");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
        ],
    );

    // --dictionary satisfies the dictionary-required check so the prompt-incompatibility
    // check is the one that fires.
    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("in.csv")
        .args(["--format", "semanticmd"])
        .arg("--dictionary")
        .args(["--prompt", "What is this?"])
        .arg("--no-cache");

    let got = wrk.output(&mut cmd);
    assert!(
        !got.status.success(),
        "Expected --format semanticmd with --prompt to fail"
    );
    let stderr = String::from_utf8_lossy(&got.stderr);
    assert!(
        stderr.contains("--format semanticmd is not compatible with --prompt"),
        "stderr did not mention the prompt incompatibility: {stderr}"
    );
}

// --format semanticmd with --dictionary emits a semantic-md data dictionary document.
#[test]
#[serial]
fn describegpt_semanticmd_dictionary() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt_semanticmd_dictionary");
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
        .args(["--format", "semanticmd"])
        .arg("--no-cache");

    let got = wrk.stdout::<String>(&mut cmd);

    // Frontmatter + structural section anchors. Placeholders must be resolved
    // (no --description/--tags here => description blank, no tags block).
    assert!(
        got.starts_with("---\nsemantic-md: datadict.yaml\n"),
        "missing/incorrect frontmatter:\n{got}"
    );
    assert!(
        !got.contains("{DATASET_DESCRIPTION}"),
        "unsubstituted description placeholder:\n{got}"
    );
    assert!(
        !got.contains("{SEMANTICMD_TAGS}"),
        "unsubstituted tags placeholder:\n{got}"
    );
    assert!(
        got.contains("# Dataset in"),
        "missing # Dataset section:\n{got}"
    );
    assert!(
        got.contains("| Resource | Schema | Title |"),
        "missing dataset table:\n{got}"
    );
    assert!(
        got.contains("# Schema `in`"),
        "missing # Schema section:\n{got}"
    );
    assert!(
        got.contains("| Column | Type | Label |"),
        "missing column table:\n{got}"
    );
    assert!(
        got.contains("# Resource `in.csv`"),
        "missing # Resource section:\n{got}"
    );
    assert!(
        got.contains("## Statistics"),
        "missing ## Statistics section:\n{got}"
    );
    assert!(
        got.contains("## Column `letter`") && got.contains("## Column `number`"),
        "missing per-column subsections:\n{got}"
    );
}

// --format okf CLI validation: rejects when neither --dictionary nor --all is set.
// Non-LLM test (validation runs before any LLM call).
#[test]
fn describegpt_okf_requires_dictionary() {
    let wrk = Workdir::new("describegpt_okf_requires_dictionary");
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
        .args(["--format", "okf"])
        .arg("--description")
        .arg("--no-cache");

    let got = wrk.output(&mut cmd);
    assert!(
        !got.status.success(),
        "Expected --format okf without --dictionary to fail"
    );
    let stderr = String::from_utf8_lossy(&got.stderr);
    assert!(
        stderr.contains("--format okf requires --dictionary"),
        "stderr did not mention the dictionary requirement: {stderr}"
    );
}

// --format okf is incompatible with --prompt. Non-LLM test.
#[test]
fn describegpt_okf_prompt_incompatible() {
    let wrk = Workdir::new("describegpt_okf_prompt_incompatible");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
        ],
    );

    // --dictionary satisfies the dictionary-required check so the prompt-incompatibility
    // check is the one that fires.
    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("in.csv")
        .args(["--format", "okf"])
        .arg("--dictionary")
        .args(["--prompt", "What is this?"])
        .arg("--no-cache");

    let got = wrk.output(&mut cmd);
    assert!(
        !got.status.success(),
        "Expected --format okf with --prompt to fail"
    );
    let stderr = String::from_utf8_lossy(&got.stderr);
    assert!(
        stderr.contains("--format okf is not compatible with --prompt"),
        "stderr did not mention the prompt incompatibility: {stderr}"
    );
}

// --format okf with --dictionary emits an Open Knowledge Format document.
#[test]
#[serial]
fn describegpt_okf_dictionary() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt_okf_dictionary");
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
        .args(["--format", "okf"])
        .arg("--no-cache");

    let got = wrk.stdout::<String>(&mut cmd);

    // OKF frontmatter: required `type` (default), plus resolved placeholders
    // (no --description/--tags here => description blank, no tags block).
    assert!(
        got.starts_with("---\ntype: \"CSV Table\"\n"),
        "missing/incorrect frontmatter:\n{got}"
    );
    assert!(
        got.contains("title:"),
        "missing title frontmatter key:\n{got}"
    );
    assert!(
        !got.contains("{DATASET_DESCRIPTION}") && !got.contains("{DATASET_DESCRIPTION_FM}"),
        "unsubstituted description placeholder:\n{got}"
    );
    assert!(
        !got.contains("{OKF_TAGS}"),
        "unsubstituted tags placeholder:\n{got}"
    );
    assert!(got.contains("# Schema"), "missing # Schema section:\n{got}");
    assert!(
        got.contains("| Column | Type | Content Type | Description | Enumeration |"),
        "missing schema table:\n{got}"
    );
    // Lean OKF: no semantic-md profile line nor Role/Concept/Join schema columns.
    assert!(
        !got.contains("semantic-md:") && !got.contains("Join?"),
        "OKF output leaked semantic-md structure:\n{got}"
    );
}

// --format okf honors --okf-type for the required `type` frontmatter key.
#[test]
#[serial]
fn describegpt_okf_custom_type() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt_okf_custom_type");
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
        .args(["--format", "okf"])
        .args(["--okf-type", "BigQuery Table"])
        .arg("--no-cache");

    let got = wrk.stdout::<String>(&mut cmd);
    assert!(
        got.starts_with("---\ntype: \"BigQuery Table\"\n"),
        "--okf-type did not set the type frontmatter key:\n{got}"
    );
}

// --format jsonschema with --dictionary emits a valid draft 2020-12 schema.
#[test]
#[serial]
fn describegpt_jsonschema_dictionary() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt_jsonschema_dictionary");
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
        .args(["--format", "jsonschema"])
        .arg("--no-cache");

    let got = wrk.stdout::<String>(&mut cmd);
    let schema: serde_json::Value = serde_json::from_str(&got)
        .unwrap_or_else(|e| panic!("Output is not valid JSON: {e}\noutput: {got}"));

    // Root-level shape.
    assert_eq!(
        schema.get("$schema").and_then(|v| v.as_str()),
        Some("https://json-schema.org/draft/2020-12/schema")
    );
    assert_eq!(schema.get("type").and_then(|v| v.as_str()), Some("object"));
    assert_eq!(
        schema.get("additionalProperties").and_then(|v| v.as_bool()),
        Some(false),
        "additionalProperties should default to false"
    );
    let required = schema
        .get("required")
        .and_then(|v| v.as_array())
        .expect("required array");
    assert_eq!(required.len(), 2, "required should list every column");

    // Per-property x-qsv extras present.
    let properties = schema
        .get("properties")
        .and_then(|v| v.as_object())
        .expect("properties object");
    for col in ["letter", "number"] {
        let prop = properties
            .get(col)
            .unwrap_or_else(|| panic!("missing property {col}"));
        let x_qsv = prop
            .get("x-qsv")
            .and_then(|v| v.as_object())
            .unwrap_or_else(|| panic!("missing x-qsv on {col}"));
        assert!(
            x_qsv.contains_key("cardinality"),
            "{col} x-qsv missing cardinality"
        );
        assert!(
            x_qsv.contains_key("null_count"),
            "{col} x-qsv missing null_count"
        );
        assert!(
            x_qsv.contains_key("qsv_type"),
            "{col} x-qsv missing qsv_type"
        );
    }

    // Schema must compile under jsonschema (which implicitly meta-validates against 2020-12).
    jsonschema::Validator::options()
        .build(&schema)
        .expect("emitted schema must be a valid draft 2020-12 JSON Schema");
}

// --allow-extra-cols flips additionalProperties to true.
#[test]
#[serial]
fn describegpt_jsonschema_allow_extra_cols() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt_jsonschema_allow_extra_cols");
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
        .args(["--format", "jsonschema"])
        .arg("--allow-extra-cols")
        .arg("--no-cache");

    let got = wrk.stdout::<String>(&mut cmd);
    let schema: serde_json::Value =
        serde_json::from_str(&got).expect("output should be valid JSON");
    assert_eq!(
        schema.get("additionalProperties").and_then(|v| v.as_bool()),
        Some(true),
        "additionalProperties should be true with --allow-extra-cols"
    );
}

// Emitted schema actually validates the source CSV when piped through `qsv validate`.
#[test]
#[serial]
fn describegpt_jsonschema_roundtrip() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt_jsonschema_roundtrip");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
            svec!["gamma", "37"],
        ],
    );

    // Emit schema to file.
    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("in.csv")
        .arg("--dictionary")
        .args(["--format", "jsonschema"])
        .args(["--output", "in.schema.json"])
        .arg("--no-cache");
    wrk.assert_success(&mut cmd);

    // Use `qsv validate` to assert the schema describes the data.
    let mut validate_cmd = wrk.command("validate");
    validate_cmd.arg("in.csv").arg("in.schema.json");
    wrk.assert_success(&mut validate_cmd);
}

// Regression: a CSV with a permissively-inferred Date column (e.g.
// "June 27, 1968") must produce a schema that `qsv validate` accepts. Without
// this guarantee, re-introducing an unconditional `format: "date"` emission in
// `map_qsv_type` / `build_property_schema` would silently break the roundtrip
// for real-world date data. Also asserts the date property does NOT carry a
// `format` keyword by default.
#[test]
#[serial]
fn describegpt_jsonschema_roundtrip_permissive_dates() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt_jsonschema_roundtrip_permissive_dates");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["name", "birth_date"],
            svec!["Alice", "June 27, 1968"],
            svec!["Bob", "March 3, 1972"],
            svec!["Carol", "November 11, 1981"],
            svec!["Dave", "April 18, 1990"],
        ],
    );

    // Emit schema with default settings (no --strict-dates).
    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("in.csv")
        .arg("--dictionary")
        .args(["--format", "jsonschema"])
        .args(["--output", "in.schema.json"])
        .arg("--no-cache");
    wrk.assert_success(&mut cmd);

    // Schema must not emit `format` on the permissively-inferred date column.
    let schema_str: String = wrk.from_str(&wrk.path("in.schema.json"));
    let schema: serde_json::Value =
        serde_json::from_str(&schema_str).expect("output should be valid JSON");
    let birth_date = schema
        .get("properties")
        .and_then(|p| p.get("birth_date"))
        .and_then(|v| v.as_object())
        .expect("birth_date property");
    assert!(
        !birth_date.contains_key("format"),
        "permissive-date column should NOT emit a `format` keyword by default; doing so would \
         break the validate roundtrip on non-RFC-3339 dates. birth_date schema: {birth_date:?}"
    );

    // qsv validate must accept the schema for the source CSV.
    let mut validate_cmd = wrk.command("validate");
    validate_cmd.arg("in.csv").arg("in.schema.json");
    wrk.assert_success(&mut validate_cmd);
}

// --strict-dates re-enables `format: "date"` / `"date-time"` emission for
// columns whose stats type is Date / DateTime. Verifies feature parity with
// `qsv schema --strict-dates`.
#[test]
#[serial]
fn describegpt_jsonschema_strict_dates_flag() {
    if !is_local_llm_available() {
        return;
    }
    let wrk = Workdir::new("describegpt_jsonschema_strict_dates_flag");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["name", "birth_date"],
            svec!["Alice", "1968-06-27"],
            svec!["Bob", "1972-03-03"],
            svec!["Carol", "1981-11-11"],
        ],
    );

    let mut cmd = wrk.command("describegpt");
    set_describegpt_testing_envvars(&mut cmd);
    cmd.arg("in.csv")
        .arg("--dictionary")
        .args(["--format", "jsonschema"])
        .arg("--strict-dates")
        .arg("--no-cache");

    let got = wrk.stdout::<String>(&mut cmd);
    let schema: serde_json::Value =
        serde_json::from_str(&got).expect("output should be valid JSON");
    let birth_date = schema
        .get("properties")
        .and_then(|p| p.get("birth_date"))
        .and_then(|v| v.as_object())
        .expect("birth_date property");
    assert_eq!(
        birth_date.get("format").and_then(|v| v.as_str()),
        Some("date"),
        "--strict-dates should emit `format: \"date\"` on Date columns"
    );
}

/// End-to-end proof, through the public `--prepare-context` / `--process-response`
/// interface, of the ONE property that makes `--infer-null-values` safe: the LLM
/// proposes a flat list, and qsv - not the model - decides what may be trusted.
///
/// The canned response proposes the same token shape for two columns:
///   * `status` (String)  - "null", which the data really does contain (as NULL/null/NuLl)
///   * `depth`  (Integer) - "-999", which the data really does contain too
///
/// Only `status` may reach `null_values`. `-999` is a legal integer that no scan can
/// distinguish from a real reading, so it must be demoted to a `confirm_required`
/// candidate even though it IS present in the data. This also pins the CHANGELOG's
/// claim that this complements `denull`: `status` is a purely categorical column that
/// `denull` deliberately never reports, because blanking it promotes nothing.
#[test]
fn describegpt_infer_null_values_confirms_strings_and_demotes_numerics() {
    use std::{io::Write, process::Stdio};

    let wrk = Workdir::new("describegpt_infer_null_values");
    // Repeats matter: a column whose cardinality equals the row count is <ALL_UNIQUE>,
    // which carries no frequency detail and would demote everything by default.
    let mut rows = vec![svec!["status", "depth"]];
    for i in 0..30 {
        let status = match i % 5 {
            0 => "NULL",
            1 => "null",
            2 => "NuLl",
            3 => "ok",
            _ => "pending",
        };
        let depth = if i % 3 == 0 { "-999" } else { "42" };
        rows.push(svec![status, depth]);
    }
    wrk.create_indexed("data.csv", rows);

    let mut cmd = wrk.command("describegpt");
    cmd.arg("--prepare-context")
        .arg("--dictionary")
        .arg("--infer-null-values")
        .arg("--no-cache")
        .arg("data.csv");
    let prep: serde_json::Value = serde_json::from_str(&wrk.stdout::<String>(&mut cmd)).unwrap();

    // The model echoes lowercase "null", proposes an unseen "N/A", and proposes the
    // numeric placeholder "-999".
    let llm_response = serde_json::json!({
        "status": {"label": "Status", "description": "Record status.",
                   "null_values": ["null", "N/A"]},
        "depth":  {"label": "Depth",  "description": "Depth reading.",
                   "null_values": ["-999"]},
    })
    .to_string();

    let phases: Vec<serde_json::Value> = prep["phases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| {
            serde_json::json!({
                "kind": p["kind"],
                "response": llm_response,
                "reasoning": "",
                "token_usage": {"prompt": 1, "completion": 1, "total": 2, "elapsed": 1}
            })
        })
        .collect();
    let process_input = serde_json::json!({
        "phases": phases,
        "analysis_results": prep["analysis_results"],
        "model": prep["model"],
    });

    let mut cmd_2 = wrk.command("describegpt");
    cmd_2
        .arg("--process-response")
        .arg("--dictionary")
        .arg("--infer-null-values")
        .arg("--no-cache")
        .arg("--format")
        .arg("JSONSchema")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = cmd_2.spawn().unwrap();
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(process_input.to_string().as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "process-response failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let schema: serde_json::Value =
        serde_json::from_str(&String::from_utf8_lossy(&output.stdout)).unwrap();
    let status_x = &schema["properties"]["status"]["x-qsv"];
    let depth_x = &schema["properties"]["depth"]["x-qsv"];

    // Every observed casing is confirmed, spelled as it appears in the DATA - not the
    // model's lowercase echo.
    let confirmed: Vec<&str> = status_x["null_values"]
        .as_array()
        .expect("status must have confirmed null_values")
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect();
    assert_eq!(confirmed.len(), 3, "got {confirmed:?}");
    for casing in ["NULL", "null", "NuLl"] {
        assert!(
            confirmed.contains(&casing),
            "missing {casing} in {confirmed:?}"
        );
    }

    // The unseen token is demoted, and carries the confirm flag.
    assert_eq!(
        status_x["null_candidates"],
        serde_json::json!([{"value": "N/A", "confirm_required": true}])
    );

    // THE load-bearing assertion: -999 is present in the data, yet an Integer column can
    // never confirm a sentinel. It must be a candidate, never a confirmed null_value.
    assert!(
        depth_x.get("null_values").is_none(),
        "a numeric column must never confirm a sentinel, got {:?}",
        depth_x.get("null_values")
    );
    assert_eq!(
        depth_x["null_candidates"],
        serde_json::json!([{"value": "-999", "confirm_required": true}]),
        "-999 must be reported as requiring human confirmation"
    );
}
