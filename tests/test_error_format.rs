// QSV_ERROR_FORMAT=json and --capabilities: the machine-readable surface for programs that
// drive qsv as a subprocess. Every command used here exists in qsv, qsvlite and qsvdp.
use std::process;

use crate::workdir::Workdir;

/// Run `cmd`, returning (exit code, stderr).
fn run(cmd: &mut process::Command) -> (i32, String) {
    let o = cmd.output().unwrap();
    (
        o.status.code().unwrap(),
        String::from_utf8_lossy(&o.stderr).into_owned(),
    )
}

/// The JSON error object from the LAST stderr line, asserting that line is the whole document.
fn json_error(stderr: &str) -> serde_json::Value {
    let last = stderr.lines().last().expect("stderr is empty");
    let v: serde_json::Value =
        serde_json::from_str(last).unwrap_or_else(|e| panic!("not JSON ({e}): {last:?}"));
    v["error"].clone()
}

fn ragged_workdir(name: &str) -> Workdir {
    let wrk = Workdir::new(name);
    wrk.create_from_string("ok.csv", "a,b\n1,2\n");
    wrk.create_from_string("ragged.csv", "a,b\n1,2\n3\n");
    wrk
}

#[test]
fn error_format_json_io_error() {
    let wrk = ragged_workdir("error_format_json_io_error");
    let mut cmd = wrk.command("count");
    cmd.env("QSV_ERROR_FORMAT", "json")
        .arg("does-not-exist.csv");

    let (code, stderr) = run(&mut cmd);
    assert_eq!(code, 1);
    assert_eq!(stderr.lines().count(), 1, "{stderr}");
    let err = json_error(&stderr);
    assert_eq!(err["kind"], "io");
    assert_eq!(err["level"], "error");
    assert_eq!(err["exit_code"], 1);
    assert_eq!(err["command"], "count");
    assert_eq!(err["qsv_version"], env!("CARGO_PKG_VERSION"));
    // the kind carries the category, so the text-mode prefix is not repeated in the message
    assert!(!err["message"].as_str().unwrap().starts_with("io error"));
}

#[test]
fn error_format_json_usage_error() {
    let wrk = ragged_workdir("error_format_json_usage_error");
    let mut cmd = wrk.command("count");
    cmd.env("QSV_ERROR_FORMAT", "json")
        .args(["--bogus", "ok.csv"]);

    let (code, stderr) = run(&mut cmd);
    assert_eq!(code, 2);
    let err = json_error(&stderr);
    assert_eq!(err["kind"], "usage");
    assert_eq!(err["exit_code"], 2);
    assert!(err["message"].as_str().unwrap().contains("--bogus"));
}

#[test]
fn error_format_json_csv_error() {
    let wrk = ragged_workdir("error_format_json_csv_error");
    let mut cmd = wrk.command("select");
    cmd.env("QSV_ERROR_FORMAT", "json")
        .args(["a", "ragged.csv"]);

    let (code, stderr) = run(&mut cmd);
    assert_eq!(code, 1);
    let err = json_error(&stderr);
    assert_eq!(err["kind"], "csv");
    assert_eq!(err["command"], "select");
}

#[test]
fn error_format_json_other_error() {
    let wrk = ragged_workdir("error_format_json_other_error");
    let mut cmd = wrk.command("select");
    cmd.env("QSV_ERROR_FORMAT", "json").args(["9", "ok.csv"]);

    let (code, stderr) = run(&mut cmd);
    assert_eq!(code, 1);
    let err = json_error(&stderr);
    assert_eq!(err["kind"], "other");
    assert!(err["message"].as_str().unwrap().contains("out of bounds"));
}

// Text mode prints nothing for "no match" (the exit code is the signal, as for grep). JSON mode
// must still emit a line, or a consumer cannot tell it from a failure that printed nothing.
#[test]
fn error_format_json_no_match_is_reported() {
    let wrk = ragged_workdir("error_format_json_no_match_is_reported");
    let mut cmd = wrk.command("search");
    cmd.env("QSV_ERROR_FORMAT", "json").args(["zzz", "ok.csv"]);

    let (code, stderr) = run(&mut cmd);
    assert_eq!(code, 1);
    let err = json_error(&stderr);
    assert_eq!(err["kind"], "no_match");
    assert_eq!(err["exit_code"], 1);
}

#[test]
fn error_format_text_no_match_stays_silent() {
    let wrk = ragged_workdir("error_format_text_no_match_stays_silent");
    let mut cmd = wrk.command("search");
    cmd.args(["zzz", "ok.csv"]);

    let (code, stderr) = run(&mut cmd);
    assert_eq!(code, 1);
    assert_eq!(stderr, "");
}

// Unset or any value other than "json" keeps the historical free-text output.
#[test]
fn error_format_text_is_unchanged() {
    let wrk = ragged_workdir("error_format_text_is_unchanged");
    for format in [None, Some("text"), Some("yaml")] {
        let mut cmd = wrk.command("select");
        if let Some(f) = format {
            cmd.env("QSV_ERROR_FORMAT", f);
        }
        cmd.args(["a", "ragged.csv"]);
        let (code, stderr) = run(&mut cmd);
        assert_eq!(code, 1);
        assert!(
            stderr.starts_with("csv error: CSV error: record 2"),
            "{format:?}: {stderr}"
        );

        let mut cmd = wrk.command("count");
        if let Some(f) = format {
            cmd.env("QSV_ERROR_FORMAT", f);
        }
        cmd.args(["--bogus", "ok.csv"]);
        let (code, stderr) = run(&mut cmd);
        assert_eq!(code, 2);
        assert!(
            stderr.starts_with("Unknown flag: '--bogus'"),
            "{format:?}: {stderr}"
        );
    }
}

#[test]
fn error_format_value_is_case_insensitive() {
    let wrk = ragged_workdir("error_format_value_is_case_insensitive");
    let mut cmd = wrk.command("select");
    cmd.env("QSV_ERROR_FORMAT", " JSON ")
        .args(["a", "ragged.csv"]);

    let (_, stderr) = run(&mut cmd);
    assert_eq!(json_error(&stderr)["kind"], "csv");
}

#[test]
fn error_format_success_prints_no_error() {
    let wrk = ragged_workdir("error_format_success_prints_no_error");
    let mut cmd = wrk.command("count");
    cmd.env("QSV_ERROR_FORMAT", "json").arg("ok.csv");

    let o = cmd.output().unwrap();
    assert!(o.status.success());
    assert_eq!(String::from_utf8_lossy(&o.stdout).trim(), "1");
    assert_eq!(String::from_utf8_lossy(&o.stderr), "");
}

#[test]
fn capabilities_json() {
    let wrk = Workdir::new("capabilities_json");
    let mut cmd = wrk.command("");
    cmd.arg("--capabilities");

    let o = cmd.output().unwrap();
    assert!(o.status.success());
    let caps: serde_json::Value = serde_json::from_slice(&o.stdout).unwrap();

    assert_eq!(caps["version"], env!("CARGO_PKG_VERSION"));
    let binary = caps["binary"].as_str().unwrap();
    assert!(
        ["qsv", "qsvlite", "qsvdp", "qsvmcp"].contains(&binary),
        "{binary}"
    );
    let commands: Vec<&str> = caps["commands"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| c.as_str().unwrap())
        .collect();
    for expected in ["count", "search", "select"] {
        assert!(commands.contains(&expected), "missing {expected}");
    }
    assert!(caps["features"].is_array());
    assert!(caps["feature_versions"].is_object());
    assert!(
        caps["error_formats"]
            .as_array()
            .unwrap()
            .contains(&serde_json::json!("json"))
    );
}

// The feature list in --capabilities and the one in --version come from the same source.
#[test]
fn capabilities_features_match_version() {
    let wrk = Workdir::new("capabilities_features_match_version");
    let mut cmd = wrk.command("");
    cmd.arg("--capabilities");
    let caps: serde_json::Value = serde_json::from_slice(&cmd.output().unwrap().stdout).unwrap();

    let mut cmd = wrk.command("");
    cmd.arg("--version");
    let version = String::from_utf8_lossy(&cmd.output().unwrap().stdout).into_owned();

    for feature in caps["features"].as_array().unwrap() {
        let name = feature.as_str().unwrap();
        // --version spells luau as "Luau <ver>" and python as "python-<ver>"
        let needle = if name == "luau" { "Luau" } else { name };
        assert!(version.contains(needle), "{name} not in {version}");
    }
}
