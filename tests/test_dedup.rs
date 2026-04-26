use crate::workdir::Workdir;

#[test]
fn dedup_normal() {
    let wrk = Workdir::new("dedup_normal");
    wrk.create(
        "in.csv",
        vec![
            svec!["N", "S"],
            svec!["10", "a"],
            svec!["10", "a"],
            svec!["2", "b"],
            svec!["2", "B"],
        ],
    );

    let mut cmd = wrk.command("dedup");
    cmd.arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["N", "S"],
        svec!["10", "a"],
        svec!["2", "B"],
        svec!["2", "b"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn dedup_no_case() {
    let wrk = Workdir::new("dedup_no_case");
    wrk.create(
        "in.csv",
        vec![
            svec!["N", "S"],
            svec!["10", "a"],
            svec!["10", "a"],
            svec!["2", "b"],
            svec!["2", "B"],
        ],
    );

    let mut cmd = wrk.command("dedup");
    cmd.arg("-i").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["N", "S"], svec!["10", "a"], svec!["2", "B"]];
    assert_eq!(got, expected);
}

#[test]
fn dedup_issue_1381() {
    let wrk = Workdir::new("dedup_issue_1381");
    wrk.create(
        "in.csv",
        vec![
            svec!["office"],
            svec!["Member of legislative assembly"],
            svec!["Member of Legislative Assembly"],
            svec!["Member of Tamil Nadu Legislative Assembly"],
        ],
    );

    let mut cmd = wrk.command("dedup");
    cmd.arg("-i").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["office"],
        svec!["Member of Legislative Assembly"],
        svec!["Member of Tamil Nadu Legislative Assembly"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn dedup_issue_1665_numeric() {
    let wrk = Workdir::new("dedup_issue_1665_numeric");
    wrk.create(
        "in.csv",
        vec![
            svec!["data"],
            svec!["1"],
            svec!["3"],
            svec!["3"],
            svec!["5"],
            svec!["10"],
        ],
    );

    let mut cmd = wrk.command("dedup");
    cmd.arg("-N").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["data"],
        svec!["1"],
        svec!["3"],
        svec!["5"],
        svec!["10"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn dedup_select() {
    let wrk = Workdir::new("dedup_select");
    wrk.create(
        "in.csv",
        vec![
            svec!["N", "S"],
            svec!["10", "a"],
            svec!["10", "a"],
            svec!["2", "b"],
            svec!["2", "B"],
        ],
    );

    let mut cmd = wrk.command("dedup");
    cmd.args(["-s", "N"]).arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["N", "S"], svec!["10", "a"], svec!["2", "B"]];
    assert_eq!(got, expected);
}

#[test]
fn dedup_select_issue774() {
    let wrk = Workdir::new("dedup_select_issue774");
    let test_file = wrk.load_test_file("dedup-test.csv");

    let mut cmd = wrk.command("dedup");
    cmd.args(["-s", "id"]).arg(test_file);

    let got: String = wrk.stdout(&mut cmd);
    let expected = wrk.load_test_resource("dedup-by-id-test-expected.csv");

    assert_eq!(got, expected);
}

#[test]
fn dedup_sorted() {
    let wrk = Workdir::new("dedup_sorted");
    wrk.create(
        "in.csv",
        vec![
            svec!["N", "S"],
            svec!["10", "a"],
            svec!["10", "a"],
            svec!["10", "b"],
            svec!["20", "B"],
            svec!["20", "b"],
            svec!["3", "c"],
            svec!["4", "d"],
        ],
    );

    let mut cmd = wrk.command("dedup");
    cmd.arg("--sorted").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["N", "S"],
        svec!["10", "a"],
        svec!["10", "b"],
        svec!["20", "B"],
        svec!["20", "b"],
        svec!["3", "c"],
        svec!["4", "d"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn dedup_sorted_nocase() {
    let wrk = Workdir::new("dedup_sorted_nocase");
    wrk.create(
        "in.csv",
        vec![
            svec!["N", "S"],
            svec!["10", "a"],
            svec!["10", "A"],
            svec!["10", "a"],
            svec!["10", "A"],
            svec!["11", "c"],
            svec!["20", "b"],
            svec!["20", "b"],
            svec!["20", "B"],
            svec!["20", "B"],
            svec!["3", "c"],
            svec!["4", "d"],
        ],
    );

    let mut cmd = wrk.command("dedup");
    cmd.arg("--sorted").arg("--ignore-case").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["N", "S"],
        svec!["10", "a"],
        svec!["11", "c"],
        svec!["20", "b"],
        svec!["3", "c"],
        svec!["4", "d"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn dedup_alreadysorted_nocase() {
    let wrk = Workdir::new("dedup_alreadysorted_nocase");
    wrk.create(
        "in.csv",
        vec![
            svec!["N", "S"],
            svec!["10", "a"],
            svec!["10", "a"],
            svec!["10", "a"],
            svec!["100", "a"],
            svec!["100", "a"],
            svec!["20", "b"],
            svec!["20", "b"],
            svec!["20", "B"],
            svec!["20", "B"],
            svec!["3", "c"],
            svec!["4", "d"],
        ],
    );

    let mut cmd = wrk.command("dedup");
    cmd.arg("--ignore-case").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["N", "S"],
        svec!["10", "a"],
        svec!["100", "a"],
        svec!["20", "B"],
        svec!["3", "c"],
        svec!["4", "d"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn dedup_sorted_empty() {
    // Regression: --sorted on empty input must not emit a stray empty row,
    // and must still print the duplicate count to stderr (parity with the
    // in-memory path).
    let wrk = Workdir::new("dedup_sorted_empty");
    wrk.create("in.csv", vec![svec!["N", "S"]]);

    let mut cmd = wrk.command("dedup");
    cmd.arg("--sorted").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["N", "S"]];
    assert_eq!(got, expected);

    let stderr = wrk.output_stderr(&mut cmd);
    assert!(
        stderr.contains("0"),
        "expected duplicate count of 0 on stderr, got: {stderr:?}"
    );
}

#[test]
fn dedup_sorted_empty_dupes_output() {
    // --sorted + --dupes-output on empty input: both the main output and the
    // dupes file should contain only the header, with no stray rows.
    let wrk = Workdir::new("dedup_sorted_empty_dupes_output");
    wrk.create("in.csv", vec![svec!["N", "S"]]);

    let mut cmd = wrk.command("dedup");
    cmd.arg("--sorted")
        .args(["--dupes-output", "dupes.csv"])
        .arg("in.csv");
    wrk.output(&mut cmd);

    let dupes: String = wrk.from_str(&wrk.path("dupes.csv"));
    let expected_header = "N,S\n";
    assert_eq!(dupes.replace("\r\n", "\n"), expected_header);
}

#[test]
fn dedup_dupes_output_run_of_three() {
    // In-memory path: a run of 3 identical rows must produce 2 rows in the
    // dupes file (the duplicates) and 1 survivor in the main output.
    // Exercises the scan_dedup! macro's prev-write-on-equal branch.
    let wrk = Workdir::new("dedup_dupes_output_run_of_three");
    wrk.create(
        "in.csv",
        vec![
            svec!["N", "S"],
            svec!["1", "a"],
            svec!["1", "a"],
            svec!["1", "a"],
            svec!["2", "b"],
        ],
    );

    let mut cmd = wrk.command("dedup");
    cmd.args(["--dupes-output", "dupes.csv"]).arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["N", "S"], svec!["1", "a"], svec!["2", "b"]];
    assert_eq!(got, expected);

    let dupes: String = wrk.from_str(&wrk.path("dupes.csv"));
    let expected_dupes = "N,S\n1,a\n1,a\n";
    assert_eq!(dupes.replace("\r\n", "\n"), expected_dupes);
}

#[test]
fn dedup_not_sorted() {
    let wrk = Workdir::new("dedup__not_sorted");
    wrk.create(
        "in.csv",
        vec![
            svec!["N", "S"],
            svec!["30", "c"],
            svec!["10", "a"],
            svec!["10", "a"],
            svec!["20", "b"],
            svec!["20", "B"],
        ],
    );

    let mut cmd = wrk.command("dedup");
    cmd.arg("--sorted").arg("in.csv");

    let got: String = wrk.output_stderr(&mut cmd);
    assert!(got.contains("Aborting! Input not sorted!"));
}

#[test]
fn dedup_not_sorted2() {
    let wrk = Workdir::new("dedup__not_sorted2");
    wrk.create(
        "in.csv",
        vec![
            svec!["N", "S"],
            svec!["10", "a"],
            svec!["10", "a"],
            svec!["20", "b"],
            svec!["20", "B"],
            svec!["1", "c"],
        ],
    );

    let mut cmd = wrk.command("dedup");
    cmd.arg("--sorted").arg("in.csv");

    let got: String = wrk.output_stderr(&mut cmd);
    assert!(got.contains("Aborting! Input not sorted!"));
}
