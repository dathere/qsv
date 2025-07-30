use crate::workdir::Workdir;

// Providing an invalid API key with --api-key without
// the environment variable set should result in an error
#[test]
// #[ignore = "Requires environment variable to NOT be set."]
fn describegpt_invalid_api_key() {
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
        .args(["--max-tokens", "1000"]);

    wrk.assert_err(&mut cmd);
}

// Verify --user-agent is passed to OpenAI
#[test]
#[ignore = "Requires environment variable to be set."]
fn describegpt_user_agent() {
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
        .args([
            "--user-agent",
            "Mozilla/5.0 (platform; rv:geckoversion) Gecko/geckotrail Firefox/firefoxversion",
        ])
        .args(["--max-tokens", "1000"]);

    // Check that the command ran successfully
    wrk.assert_success(&mut cmd);
}

// Valid use of describegpt
#[test]
#[ignore = "Requires environment variable to be set."]
fn describegpt_valid() {
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
#[ignore = "Requires environment variable to be set."]
fn describegpt_valid_json() {
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
