//
// old
//

use crate::workdir::Workdir;

static EXPECTED_TABLE: &str = "\
h1       h2   h3
abcdefg  a    a
a        abc  z";

fn data() -> Vec<Vec<String>> {
    vec![
        svec!["h1", "h2", "h3"],
        svec!["abcdefg", "a", "a"],
        svec!["a", "abc", "z"],
    ]
}

#[test]
fn table() {
    let wrk = Workdir::new("table");
    wrk.create("in.csv", data());

    let mut cmd = wrk.command("table");
    cmd.env("QSV_DEFAULT_DELIMITER", "\t");
    cmd.arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    assert_eq!(&*got, EXPECTED_TABLE)
}

#[test]
fn table_tsv() {
    let wrk = Workdir::new("table");
    wrk.create_with_delim("in.tsv", data(), b'\t');

    let mut cmd = wrk.command("table");
    cmd.env("QSV_DEFAULT_DELIMITER", "\t");
    cmd.arg("in.tsv");

    let got: String = wrk.stdout(&mut cmd);
    assert_eq!(&*got, EXPECTED_TABLE)
}

#[test]
fn table_ssv() {
    let wrk = Workdir::new("table");
    wrk.create_with_delim("in.ssv", data(), b';');

    let mut cmd = wrk.command("table");
    cmd.env("QSV_DEFAULT_DELIMITER", ";");
    cmd.arg("in.ssv");

    let got: String = wrk.stdout(&mut cmd);
    assert_eq!(&*got, EXPECTED_TABLE)
}

#[test]
fn table_default() {
    let wrk = Workdir::new("table");
    wrk.create_with_delim("in.file", data(), b'\t');

    let mut cmd = wrk.command("table");
    cmd.env("QSV_DEFAULT_DELIMITER", "\t");
    cmd.env("QSV_SKIP_FORMAT_CHECK", "1");
    cmd.arg("in.file");

    let got: String = wrk.stdout(&mut cmd);
    assert_eq!(&*got, EXPECTED_TABLE)
}

#[test]
fn table_pipe_delimiter_env() {
    let wrk = Workdir::new("table_pipe_delimiter");
    wrk.create_with_delim("in.file", data(), b'|');

    let mut cmd = wrk.command("table");
    cmd.env("QSV_SKIP_FORMAT_CHECK", "1");
    cmd.env("QSV_DEFAULT_DELIMITER", "|");
    cmd.arg("in.file");

    let got: String = wrk.stdout(&mut cmd);
    assert_eq!(&*got, EXPECTED_TABLE)
}

#[test]
fn table_pipe_delimiter() {
    let wrk = Workdir::new("table_pipe_delimiter");
    wrk.create_with_delim("in.file", data(), b'|');

    let mut cmd = wrk.command("table");
    cmd.env("QSV_SKIP_FORMAT_CHECK", "1");
    cmd.arg("--delimiter").arg("|").arg("in.file");

    let got: String = wrk.stdout(&mut cmd);
    assert_eq!(&*got, EXPECTED_TABLE)
}

#[test]
fn invalid_delimiter_len() {
    let wrk = Workdir::new("invalid_delimiter_len");
    wrk.create_with_delim("in.file", data(), b'|');

    let mut cmd = wrk.command("table");
    cmd.env("QSV_SKIP_FORMAT_CHECK", "1");
    cmd.arg("--delimiter").arg("||").arg("in.file");

    let got: String = wrk.output_stderr(&mut cmd);
    assert_eq!(
        &*got,
        "Could not convert '||' to a single ASCII character.\n"
    )
}

#[test]
fn table_right_align() {
    let wrk = Workdir::new("table");
    wrk.create("in.csv", data());

    let mut cmd = wrk.command("table");
    cmd.arg("--align");
    cmd.arg("right");
    cmd.arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    assert_eq!(
        &*got,
        concat!("     h1   h2  h3\n", "abcdefg    a  a\n", "      a  abc  z",)
    );
}

#[test]
fn table_center_align() {
    let wrk = Workdir::new("table");
    wrk.create("in.csv", data());

    let mut cmd = wrk.command("table");
    cmd.arg("-a");
    cmd.arg("center");
    cmd.arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    assert_eq!(
        &*got,
        concat!("  h1     h2   h3\n", "abcdefg   a   a\n", "   a     abc  z",)
    );
}

#[test]
fn table_leftendtab_align() {
    let wrk = Workdir::new("table");
    wrk.create("in.csv", data());

    let mut cmd = wrk.command("table");
    cmd.arg("--align");
    cmd.arg("leftendtab");
    cmd.arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    // leftendtab alignment should left-align content with tab-separated columns
    // This creates a TSV-like format that's both human-readable and machine-parseable
    assert_eq!(
        &*got,
        "h1      \th2  \th3\nabcdefg \ta   \ta\na       \tabc \tz"
    );
}

#[test]
fn table_leftfwf_align() {
    let wrk = Workdir::new("table");
    wrk.create("in.csv", data());

    let mut cmd = wrk.command("table");
    cmd.arg("--align");
    cmd.arg("leftfwf");
    cmd.arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    // leftfwf alignment should be similar to left but with the first line being a comment
    // that enumerates the position (1-based, comma-separated) of each column
    assert_eq!(
        &*got,
        "#1,10,15\nh1       h2   h3\nabcdefg  a    a\na        abc  z"
    );
}

//
// new
//

// use std::process;

// use crate::workdir::Workdir;

// fn setup(name: &str) -> (Workdir, process::Command) {
//     let rows = vec![
//         svec!["c1", "c2"],
//         svec!["alpha", "beta"],
//         svec!["gamma", "delta"],
//     ];

//     let wrk = Workdir::new(name);
//     wrk.create("in.csv", rows);

//     let mut cmd = wrk.command("table");
//     cmd.arg("in.csv");

//     (wrk, cmd)
// }

// #[test]
// fn table_runs_basic() {
//     let (_wrk, mut cmd) = setup("table_runs_basic");
//     // default (no color) should succeed and produce aligned output.
//     let out: String = _wrk.stdout(&mut cmd);
//     assert!(out.contains("alpha"));
//     assert!(out.contains("beta"));
// }

// #[test]
// fn table_runs_with_color() {
//     let (_wrk, mut cmd) = setup("table_runs_with_color");
//     cmd.env("FORCE_COLOR", "1");
//     _wrk.assert_success(&mut cmd);
// }
