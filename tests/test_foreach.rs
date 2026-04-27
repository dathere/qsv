use crate::workdir::Workdir;

#[test]
#[cfg(target_family = "unix")]
fn foreach() {
    let wrk = Workdir::new("foreach");
    wrk.create(
        "data.csv",
        vec![svec!["name"], svec!["John"], svec!["Mary"]],
    );
    let mut cmd = wrk.command("foreach");
    cmd.arg("name")
        .arg("echo 'NAME = {}'")
        .arg("data.csv")
        .args(["--dry-run", "false"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["NAME = John"], svec!["NAME = Mary"]];
    assert_eq!(got, expected);
}

#[test]
fn foreach_dry_run() {
    let wrk = Workdir::new("foreach_dry_run");
    wrk.create(
        "data.csv",
        vec![svec!["name"], svec!["John"], svec!["Mary"]],
    );
    let mut cmd = wrk.command("foreach");
    cmd.arg("name").arg("echo 'NAME = {}'").arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"echo NAME = John
echo NAME = Mary"#;
    assert_eq!(got, expected);
}

#[test]
#[cfg(target_family = "unix")]
fn foreach_multiple_braces() {
    let wrk = Workdir::new("foreach");
    wrk.create(
        "data.csv",
        vec![svec!["name"], svec!["John"], svec!["Mary"]],
    );
    let mut cmd = wrk.command("foreach");
    cmd.arg("name")
        .arg("echo 'NAME = {}, {}, {}'")
        .arg("data.csv")
        .args(["--dry-run", "false"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["NAME = John", " John", " John"],
        svec!["NAME = Mary", " Mary", " Mary"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn foreach_multiple_braces_dry_run() {
    let wrk = Workdir::new("foreach");
    wrk.create(
        "data.csv",
        vec![svec!["name"], svec!["John"], svec!["Mary"]],
    );
    let mut cmd = wrk.command("foreach");
    cmd.arg("name")
        .arg("echo 'NAME = {}, {}, {}'")
        .arg("data.csv")
        .args(["--dry-run", "true"]);

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"echo NAME = John, John, John
echo NAME = Mary, Mary, Mary"#;
    assert_eq!(got, expected);
}

#[test]
#[cfg(target_family = "unix")]
fn foreach_special_chars_1171() {
    let wrk = Workdir::new("foreach_special_chars");
    wrk.create(
        "data.csv",
        vec![
            svec!["host"],
            svec!["omadhina.co.NA"],
            svec!["https://www.google.com"],
            svec!["www.apple.com"],
            svec!["https://civic-data-ecosystem.github.io"],
        ],
    );
    let mut cmd = wrk.command("foreach");
    cmd.arg("host")
        .arg("echo 'dig +short {} a'")
        .arg("data.csv")
        .args(["--dry-run", "false"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["dig +short omadhina.co.NA a"],
        svec!["dig +short https://www.google.com a"],
        svec!["dig +short www.apple.com a"],
        svec!["dig +short https://civic-data-ecosystem.github.io a"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn foreach_special_chars_1171_dry_run() {
    let wrk = Workdir::new("foreach_special_chars_dry_run");
    wrk.create(
        "data.csv",
        vec![
            svec!["host"],
            svec!["omadhina.co.NA"],
            svec!["https://www.google.com"],
            svec!["www.apple.com"],
            svec!["https://civic-data-ecosystem.github.io"],
        ],
    );
    let mut cmd = wrk.command("foreach");
    cmd.arg("host")
        .arg("echo 'dig +short {} a'")
        .arg("data.csv")
        .args(["--dry-run", "true"]);

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"echo dig +short omadhina.co.NA a
echo dig +short https://www.google.com a
echo dig +short www.apple.com a
echo dig +short https://civic-data-ecosystem.github.io a"#;

    assert_eq!(got, expected);
}

#[test]
#[cfg(target_family = "unix")]
fn foreach_unify() {
    let wrk = Workdir::new("foreach_unify");
    wrk.create(
        "data.csv",
        vec![svec!["name"], svec!["John"], svec!["Mary"]],
    );
    let mut cmd = wrk.command("foreach");
    cmd.arg("name")
        .arg("echo 'name,value\n{},1'")
        .arg("--unify")
        .arg("data.csv")
        .args(["--dry-run", "false"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "value"],
        svec!["John", "1"],
        svec!["Mary", "1"],
    ];
    assert_eq!(got, expected);
}

#[test]
#[cfg(target_family = "unix")]
fn foreach_new_column() {
    let wrk = Workdir::new("foreach_nc");
    wrk.create(
        "data.csv",
        vec![svec!["name"], svec!["John"], svec!["Mary"]],
    );
    let mut cmd = wrk.command("foreach");
    cmd.arg("name")
        .arg("echo 'name,value\n{},1'")
        .arg("--unify")
        .arg("--new-column")
        .arg("current_value")
        .arg("data.csv")
        .args(["--dry-run", "false"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "value", "current_value"],
        svec!["John", "1", "John"],
        svec!["Mary", "1", "Mary"],
    ];
    assert_eq!(got, expected);
}

#[test]
#[cfg(target_family = "unix")]
fn foreach_multiple_commands_with_shell_script() {
    let wrk = Workdir::new("foreach_multiple_commands_with_shell_script");
    wrk.create(
        "data.csv",
        vec![svec!["name"], svec!["John"], svec!["Mary"]],
    );
    wrk.create_from_string(
        "multiple_commands.sh",
        r#"REVERSED_NAME=$(echo $1 | rev)
echo $1 $REVERSED_NAME"#,
    );
    let mut cmd = wrk.command("foreach");
    cmd.arg("name")
        .arg("sh multiple_commands.sh {}")
        .arg("data.csv")
        .args(["--dry-run", "false"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["John nhoJ"], svec!["Mary yraM"]];
    assert_eq!(got, expected);
}

#[test]
fn foreach_multiple_commands_with_shell_script_dry_run() {
    let wrk = Workdir::new("foreach_multiple_commands_with_shell_script_dry_run");
    wrk.create(
        "data.csv",
        vec![svec!["name"], svec!["John"], svec!["Mary"]],
    );
    wrk.create_from_string(
        "multiple_commands.sh",
        r#"REVERSED_NAME=$(echo $1 | rev)
echo $1 $REVERSED_NAME"#,
    );
    let mut cmd = wrk.command("foreach");
    cmd.arg("name")
        .arg("sh multiple_commands.sh {}")
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "sh multiple_commands.sh John\nsh multiple_commands.sh Mary";
    assert_eq!(got, expected);
}

#[test]
fn foreach_issue_2753() {
    let wrk = Workdir::new("foreach_issue_2753");
    wrk.create("data.csv", vec![svec!["a", "b"], svec!["1", "1/"]]);
    let mut cmd = wrk.command("foreach");
    cmd.arg("b").arg("echo {}test").arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"echo 1/test"#;
    assert_eq!(got, expected);

    let mut cmd = wrk.command("foreach");
    cmd.arg("b").arg(r#"echo {}/test"#).arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"echo 1//test"#;
    assert_eq!(got, expected);
}

#[test]
fn foreach_empty_command_errors() {
    // Whitespace-only commands used to panic at unwrap() on the splitter
    // result. Now they should be rejected up front with an IncorrectUsage
    // error and a non-zero exit, never panic.
    let wrk = Workdir::new("foreach_empty_command_errors");
    wrk.create("data.csv", vec![svec!["name"], svec!["John"]]);
    let mut cmd = wrk.command("foreach");
    cmd.arg("name").arg("   ").arg("data.csv");
    wrk.assert_err(&mut cmd);
    let stderr = wrk.output_stderr(&mut cmd);
    assert!(
        stderr.contains("cannot be empty"),
        "expected 'cannot be empty' in stderr, got: {stderr}"
    );
}

#[test]
fn foreach_multi_column_errors() {
    // foreach historically silently used only the first selected column when
    // the user passed multiple. Now it errors instead so the user notices.
    let wrk = Workdir::new("foreach_multi_column_errors");
    wrk.create("data.csv", vec![svec!["a", "b"], svec!["1", "2"]]);
    let mut cmd = wrk.command("foreach");
    cmd.arg("a,b").arg("echo {}").arg("data.csv");
    wrk.assert_err(&mut cmd);
    let stderr = wrk.output_stderr(&mut cmd);
    assert!(
        stderr.contains("single column"),
        "expected 'single column' in stderr, got: {stderr}"
    );
}

#[test]
fn foreach_dry_run_file_preserved_on_conflict() {
    // Regression: passing both --dry-run=<file> and --unify used to truncate
    // the file as a write-permission probe, then error out — leaving the
    // user's file empty. The cleanup defers file creation until after flag
    // validation, so the file should be untouched.
    let wrk = Workdir::new("foreach_dry_run_file_preserved_on_conflict");
    wrk.create("data.csv", vec![svec!["name"], svec!["John"]]);
    wrk.create_from_string("dryrun.txt", "preexisting content\n");

    let mut cmd = wrk.command("foreach");
    cmd.arg("name")
        .arg("echo {}")
        .arg("data.csv")
        .args(["--dry-run", "dryrun.txt"])
        .arg("--unify");
    wrk.assert_err(&mut cmd);

    let contents = std::fs::read_to_string(wrk.path("dryrun.txt")).unwrap();
    assert_eq!(
        contents, "preexisting content\n",
        "dry-run file was clobbered despite the conflicting flag"
    );
}

#[test]
#[cfg(target_family = "unix")]
fn foreach_child_failure_propagates() {
    // A child command that exits non-zero should cause foreach itself to
    // exit non-zero, but only AFTER still processing the remaining rows.
    // Row 1 fails; row 2 succeeds and writes to a side file. We verify both:
    // the run exits non-zero, and the side file shows row 2 ran — which
    // would not be the case if the loop bailed on the first failure.
    let wrk = Workdir::new("foreach_child_failure_propagates");
    wrk.create(
        "data.csv",
        vec![svec!["arg"], svec!["/no/such/path/abc123"], svec!["row2"]],
    );

    let touch_path = wrk.path("row2_ran.txt");
    let touch_path_str = touch_path.to_str().unwrap();
    wrk.create_from_string(
        "run.sh",
        &format!(
            r#"#!/bin/sh
if [ "$1" = "row2" ]; then
    echo "$1" >> {touch_path_str}
    exit 0
fi
cat "$1"
"#
        ),
    );
    std::process::Command::new("chmod")
        .arg("+x")
        .arg(wrk.path("run.sh"))
        .status()
        .unwrap();

    let mut cmd = wrk.command("foreach");
    cmd.arg("arg")
        .arg("sh run.sh {}")
        .arg("data.csv")
        .args(["--dry-run", "false"]);
    wrk.assert_err(&mut cmd);

    let contents = std::fs::read_to_string(&touch_path)
        .expect("row2_ran.txt should exist — row 2 should still have run after row 1 failed");
    assert!(
        contents.contains("row2"),
        "expected row 2 to have run after row 1 failed, file contents: {contents}"
    );
}

#[test]
#[cfg(target_family = "unix")]
fn foreach_empty_after_substitution_continues() {
    // If the templated command becomes empty after substitution at row N,
    // foreach should warn, mark the run as failed, and continue with the
    // remaining rows — same behaviour as a non-zero child exit.
    //
    // Row 1's value is "" so substitution produces an empty command (warn +
    // continue). Row 2's value points at a small shell script that writes
    // to a marker file; checking that file confirms the loop did not bail
    // out on row 1. Exit status, stderr, and the side effect are all
    // observed from a single command execution.
    let wrk = Workdir::new("foreach_empty_after_substitution_continues");

    let marker_path = wrk.path("row2_ran.txt");
    let marker_str = marker_path.to_str().unwrap();
    wrk.create_from_string(
        "run.sh",
        &format!(
            r#"#!/bin/sh
echo "ran" > {marker_str}
"#
        ),
    );
    std::process::Command::new("chmod")
        .arg("+x")
        .arg(wrk.path("run.sh"))
        .status()
        .unwrap();

    wrk.create("data.csv", vec![svec!["v"], svec![""], svec!["./run.sh"]]);

    let mut cmd = wrk.command("foreach");
    cmd.arg("v")
        .arg("{}")
        .arg("data.csv")
        .args(["--dry-run", "false"]);

    let output = cmd.output().unwrap();
    assert!(
        !output.status.success(),
        "expected non-zero exit, got: {:?}",
        output.status
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("empty after substitution"),
        "expected per-row warning in stderr, got: {stderr}"
    );
    let contents = std::fs::read_to_string(&marker_path).expect(
        "row2_ran.txt should exist — row 2 should still have run after row 1's empty command",
    );
    assert!(
        contents.contains("ran"),
        "expected marker contents, got: {contents}"
    );
}
