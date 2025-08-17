use std::{env, process::Command};

use serial_test::serial;

use crate::workdir::Workdir;

fn is_local_llm_available() -> bool {
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
