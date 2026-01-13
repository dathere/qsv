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
    let out: String = _wrk.stdout(&mut cmd);
    assert!(out.contains("alpha"));
    assert!(out.contains("beta"));
}

#[test]
fn table_runs_with_color() {
    let (_wrk, mut cmd) = setup("table_runs_with_color");
    cmd.env("FORCE_COLOR", "1");
    _wrk.assert_success(&mut cmd);
}
