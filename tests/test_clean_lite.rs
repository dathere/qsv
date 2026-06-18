use crate::workdir::Workdir;

// qsvlite must NOT include the `clean` command in its `--list` output.
#[test]
fn clean_absent_from_qsvlite_list() {
    let wrk = Workdir::new("clean_absent_from_qsvlite_list");
    let mut cmd = wrk.command("--list");
    let out = wrk.output(&mut cmd);
    // assert success first, so a failing command with empty output can't pass vacuously
    assert!(
        out.status.success(),
        "qsvlite --list failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    // check for a `clean` COMMAND entry (a line whose first token is `clean`),
    // not a mere substring (a description elsewhere could contain "clean").
    let has_clean_cmd = combined
        .lines()
        .any(|l| l.trim_start().split_whitespace().next() == Some("clean"));
    assert!(
        !has_clean_cmd,
        "qsvlite --list must not list the clean command:\n{combined}"
    );
}

// qsvlite must reject the `clean` subcommand.
#[test]
fn clean_rejected_by_qsvlite() {
    let wrk = Workdir::new("clean_rejected_by_qsvlite");
    let mut cmd = wrk.command("clean");
    let out = wrk.output(&mut cmd);
    assert!(
        !out.status.success(),
        "qsvlite should reject the clean subcommand"
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("clean"),
        "expected an unknown-command error mentioning clean, got: {stderr}"
    );
}
