use std::process;

use crate::workdir::Workdir;

fn setup(name: &str) -> (Workdir, process::Command) {
    let rows = vec![
        svec!["c1", "c2"],
        svec!["alpha", "beta"],
        svec!["gamma", "delta"],
    ];

    let wrk = Workdir::new(name);
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("table");
    cmd.arg("in.csv");

    (wrk, cmd)
}

#[test]
fn table_runs_basic() {
    let (_wrk, mut cmd) = setup("table_runs_basic");
    // default (no color) should succeed and produce aligned output.
    cmd.arg("--width").arg("2");
    let out: String = _wrk.stdout(&mut cmd);
    assert!(out.contains("alpha"));
    assert!(out.contains("beta"));
}

#[test]
fn table_runs_with_color_max_width() {
    let (_wrk, mut cmd) = setup("table_runs_with_color_max_width");
    cmd.args(["--max-width", "20"]);
    cmd.env("FORCE_COLOR", "1"); // keep forcing color so TabWriter stays ansi-aware
    _wrk.assert_success(&mut cmd);
}
