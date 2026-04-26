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
    // in-memory path). Capture stdout and stderr from a single invocation
    // so the assertions describe one run of the command.
    let wrk = Workdir::new("dedup_sorted_empty");
    wrk.create("in.csv", vec![svec!["N", "S"]]);

    let mut cmd = wrk.command("dedup");
    cmd.arg("--sorted").arg("in.csv");
    let output = wrk.output(&mut cmd);

    // safety: test output is expected to be valid UTF-8 CSV data.
    let stdout = String::from_utf8(output.stdout).unwrap();
    let expected_stdout = "N,S\n";
    assert_eq!(stdout.replace("\r\n", "\n"), expected_stdout);

    // dedup emits only the dupe count to stderr (just the number, no prose),
    // so assert exact equality on the trimmed output to actually catch a
    // regression that drops or reformats the line.
    // safety: test stderr is expected to be valid UTF-8 text output.
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert_eq!(stderr.trim(), "0", "got stderr: {stderr:?}");
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
fn dedup_sorted_select_dupes_output_run_of_three() {
    // Regression for Copilot review on PR 3754: with --sorted + --select +
    // --dupes-output, a run of three rows that are equal-on-selection but
    // differ on non-selected columns must write the actual dropped rows
    // to the dupes file (next_record on each Equal step), not the survivor
    // row repeated. Previously the streaming Equal branch wrote &record
    // (the survivor) every iteration, so a 3-row run produced two copies
    // of the survivor and dropped the actually-removed rows from view.
    let wrk = Workdir::new("dedup_sorted_select_dupes_output_run_of_three");
    wrk.create(
        "in.csv",
        vec![
            svec!["key", "val"],
            svec!["1", "a"],
            svec!["1", "b"],
            svec!["1", "c"],
            svec!["2", "d"],
        ],
    );

    let mut cmd = wrk.command("dedup");
    cmd.arg("--sorted")
        .args(["--select", "key"])
        .args(["--dupes-output", "dupes.csv"])
        .arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["key", "val"], svec!["1", "a"], svec!["2", "d"]];
    assert_eq!(got, expected);

    let dupes: String = wrk.from_str(&wrk.path("dupes.csv"));
    let expected_dupes = "key,val\n1,b\n1,c\n";
    assert_eq!(dupes.replace("\r\n", "\n"), expected_dupes);
}

#[test]
fn dedup_no_headers_dupes_output() {
    // Regression for Copilot review on PR 3754: with --no-headers, the
    // dupes file must not contain a phantom "header" line — previously
    // the dupes writer was seeded with rdr.byte_headers() unconditionally,
    // which under --no-headers returns the first DATA row, producing a
    // dupes file that started with that row even though it wasn't a
    // duplicate. Use rconfig.write_headers, which already respects the
    // no-headers flag.
    let wrk = Workdir::new("dedup_no_headers_dupes_output");
    wrk.create(
        "in.csv",
        vec![svec!["10"], svec!["20"], svec!["20"], svec!["30"]],
    );

    let mut cmd = wrk.command("dedup");
    cmd.arg("--no-headers")
        .args(["--dupes-output", "dupes.csv"])
        .arg("in.csv");
    wrk.output(&mut cmd);

    let dupes: String = wrk.from_str(&wrk.path("dupes.csv"));
    let expected_dupes = "20\n";
    assert_eq!(dupes.replace("\r\n", "\n"), expected_dupes);
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
