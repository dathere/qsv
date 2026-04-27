use quickcheck::TestResult;

use crate::{CsvRecord, qcheck, workdir::Workdir};

fn trim_trailing_empty(it: &CsvRecord) -> Vec<String> {
    let mut cloned = it.clone().unwrap();
    while cloned.len() > 1 && cloned.last().unwrap().is_empty() {
        cloned.pop();
    }
    cloned
}

#[test]
fn prop_fixlengths_all_maxlen() {
    fn p(rows: Vec<CsvRecord>) -> TestResult {
        let expected_len = match rows.iter().map(|r| trim_trailing_empty(r).len()).max() {
            None => return TestResult::discard(),
            Some(n) => n,
        };

        let wrk = Workdir::new("fixlengths_all_maxlen").flexible(true);
        wrk.create("in.csv", rows);

        let mut cmd = wrk.command("fixlengths");
        cmd.arg("in.csv");

        let got: Vec<CsvRecord> = wrk.read_stdout(&mut cmd);
        let got_len = got.iter().map(|r| r.len()).max().unwrap();
        for r in &got {
            assert_eq!(r.len(), got_len)
        }
        TestResult::from_bool(rassert_eq!(got_len, expected_len))
    }
    qcheck(p as fn(Vec<CsvRecord>) -> TestResult);
}

#[test]
fn fixlengths_all_maxlen_trims() {
    let rows = vec![
        svec!["h1", "h2"],
        svec!["abcdef", "ghijkl", "", ""],
        svec!["mnopqr", "stuvwx", "", ""],
    ];

    let wrk = Workdir::new("fixlengths_all_maxlen_trims").flexible(true);
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("fixlengths");
    cmd.arg("in.csv");

    let got: Vec<CsvRecord> = wrk.read_stdout(&mut cmd);
    for r in &got {
        assert_eq!(r.len(), 2)
    }
}

#[test]
fn fixlengths_insert_negative() {
    let rows = vec![
        svec!["clothes", "colours", "size"],
        svec!["shirt", "blue", "green", "grey", "small"],
        svec!["shirt", "yellow", "black", "small"],
        svec!["shorts", "blue", "medium"],
        svec!["shorts", "black", "large"],
    ];

    let wrk = Workdir::new("fixlengths_insert_negative").flexible(true);
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("fixlengths");
    cmd.arg("in.csv").args(["-i", "-2"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(
        got,
        vec![
            svec!["clothes", "colours", "", "", "size"],
            svec!["shirt", "blue", "green", "grey", "small"],
            svec!["shirt", "yellow", "", "black", "small"],
            svec!["shorts", "blue", "", "", "medium"],
            svec!["shorts", "black", "", "", "large"]
        ]
    );
}

#[test]
fn fixlengths_insert_positive() {
    let rows = vec![
        svec!["clothes", "colours", "size"],
        svec!["shirt", "blue", "green", "grey", "small"],
        svec!["shirt", "yellow", "black", "small"],
        svec!["shorts", "blue", "medium"],
        svec!["shorts", "black", "large"],
    ];

    let wrk = Workdir::new("fixlengths_insert_positive").flexible(true);
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("fixlengths");
    cmd.arg("in.csv").args(["-i", "2"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(
        got,
        vec![
            svec!["clothes", "", "", "colours", "size"],
            svec!["shirt", "blue", "green", "grey", "small"],
            svec!["shirt", "", "yellow", "black", "small"],
            svec!["shorts", "", "", "blue", "medium"],
            svec!["shorts", "", "", "black", "large"]
        ]
    );
}

#[test]
fn fixlengths_insert_positive_length_7() {
    let rows = vec![
        svec!["clothes", "colours", "size"],
        svec!["shirt", "blue", "green", "grey", "small"],
        svec!["shirt", "yellow", "black", "small"],
        svec!["shorts", "blue", "medium"],
        svec!["shorts", "black", "large"],
    ];

    let wrk = Workdir::new("fixlengths_insert_positive_length_7").flexible(true);
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("fixlengths");
    cmd.arg("in.csv")
        .args(["--insert", "2"])
        .args(["--length", "7"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(
        got,
        vec![
            svec!["clothes", "", "", "", "", "colours", "size"],
            svec!["shirt", "", "", "blue", "green", "grey", "small"],
            svec!["shirt", "", "", "", "yellow", "black", "small"],
            svec!["shorts", "", "", "", "", "blue", "medium"],
            svec!["shorts", "", "", "", "", "black", "large"]
        ]
    );
}

#[test]
fn fixlengths_insert_negative_length_7() {
    let rows = vec![
        svec!["clothes", "colours", "size"],
        svec!["shirt", "blue", "green", "grey", "small"],
        svec!["shirt", "yellow", "black", "small"],
        svec!["shorts", "blue", "medium"],
        svec!["shorts", "black", "large"],
    ];

    let wrk = Workdir::new("fixlengths_insert_negative_length_7").flexible(true);
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("fixlengths");
    cmd.arg("in.csv")
        .args(["--insert", "-2"])
        .args(["--length", "7"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(
        got,
        vec![
            svec!["clothes", "colours", "size", "", "", "", ""],
            svec!["shirt", "blue", "green", "grey", "", "", "small"],
            svec!["shirt", "yellow", "black", "small", "", "", ""],
            svec!["shorts", "blue", "medium", "", "", "", ""],
            svec!["shorts", "black", "large", "", "", "", "",]
        ]
    );
}

#[test]
fn fixlengths_all_maxlen_trims_at_least_1() {
    let rows = vec![svec![""], svec!["", ""], svec!["", "", ""]];

    let wrk = Workdir::new("fixlengths_all_maxlen_trims_at_least_1").flexible(true);
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("fixlengths");
    cmd.arg("in.csv");

    let got: Vec<CsvRecord> = wrk.read_stdout(&mut cmd);
    for r in &got {
        assert_eq!(r.len(), 1)
    }
}

#[test]
fn prop_fixlengths_explicit_len() {
    fn p(rows: Vec<CsvRecord>, expected_len: usize) -> TestResult {
        if expected_len == 0 || rows.is_empty() || expected_len > 10 {
            return TestResult::discard();
        }

        let wrk = Workdir::new("fixlengths_explicit_len").flexible(true);
        wrk.create("in.csv", rows);

        let mut cmd = wrk.command("fixlengths");
        cmd.arg("in.csv").args(["-l", &*expected_len.to_string()]);

        let got: Vec<CsvRecord> = wrk.read_stdout(&mut cmd);
        let got_len = got.iter().map(|r| r.len()).max().unwrap();
        for r in &got {
            assert_eq!(r.len(), got_len)
        }
        TestResult::from_bool(rassert_eq!(got_len, expected_len))
    }
    qcheck(p as fn(Vec<CsvRecord>, usize) -> TestResult);
}

#[test]
fn fixlengths_remove_empty_basic() {
    let rows = vec![
        svec!["a", "", "c", "", "e"],
        svec!["f", "", "h", "", "j"],
        svec!["k", "", "m", "", "o"],
    ];

    let wrk = Workdir::new("fixlengths_remove_empty_basic").flexible(true);
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("fixlengths");
    cmd.arg("in.csv").args(["-r"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(
        got,
        vec![
            svec!["a", "c", "e"],
            svec!["f", "h", "j"],
            svec!["k", "m", "o"],
        ]
    );
}

#[test]
fn fixlengths_remove_empty_with_length() {
    let rows = vec![
        svec!["a", "", "c", "", "e"],
        svec!["f", "", "h", "", "j"],
        svec!["k", "", "m", "", "o"],
    ];

    let wrk = Workdir::new("fixlengths_remove_empty_with_length").flexible(true);
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("fixlengths");
    cmd.arg("in.csv").args(["-r"]).args(["-l", "4"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(
        got,
        vec![
            svec!["a", "c", "e", ""],
            svec!["f", "h", "j", ""],
            svec!["k", "m", "o", ""],
        ]
    );
}

#[test]
fn fixlengths_remove_empty_with_insert() {
    let rows = vec![
        svec!["a", "", "c", "", "e"],
        svec!["f", "", "h", "", "j"],
        svec!["k", "", "m", "", "o"],
    ];

    let wrk = Workdir::new("fixlengths_remove_empty_with_insert").flexible(true);
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("fixlengths");
    cmd.arg("in.csv").args(["-r"]).args(["-i", "2"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(
        got,
        vec![
            svec!["a", "c", "e"],
            svec!["f", "h", "j"],
            svec!["k", "m", "o"],
        ]
    );
}

#[test]
fn fixlengths_remove_empty_with_length_and_insert() {
    let rows = vec![
        svec!["a", "", "c", "", "e"],
        svec!["f", "", "h", "", "j"],
        svec!["k", "", "m", "", "o"],
    ];

    let wrk = Workdir::new("fixlengths_remove_empty_with_length_and_insert").flexible(true);
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("fixlengths");
    cmd.arg("in.csv")
        .args(["-r"])
        .args(["-l", "5"])
        .args(["-i", "2"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(
        got,
        vec![
            svec!["a", "", "", "c", "e"],
            svec!["f", "", "", "h", "j"],
            svec!["k", "", "", "m", "o"],
        ]
    );
}

#[test]
fn fixlengths_remove_empty_all_empty_columns() {
    let rows = vec![
        svec!["a", "", "", "", "e"],
        svec!["f", "", "", "", "j"],
        svec!["k", "", "", "", "o"],
    ];

    let wrk = Workdir::new("fixlengths_remove_empty_all_empty_columns").flexible(true);
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("fixlengths");
    cmd.arg("in.csv").args(["-r"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(
        got,
        vec![svec!["a", "e"], svec!["f", "j"], svec!["k", "o"],]
    );
}

#[test]
fn fixlengths_remove_empty_first_row_narrow() {
    // Regression: prior to fix, col_is_empty_vec was sized from the first
    // record only, so a wider later record caused an index out of bounds
    // panic (debug) or undefined behavior (release).
    let rows = vec![
        svec!["a", "b"],
        svec!["c", "d", "", "", "e"],
        svec!["f", "g", "", "", "h"],
    ];

    let wrk = Workdir::new("fixlengths_remove_empty_first_row_narrow").flexible(true);
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("fixlengths");
    cmd.arg("in.csv").args(["-r"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(
        got,
        vec![
            svec!["a", "b", ""],
            svec!["c", "d", "e"],
            svec!["f", "g", "h"],
        ]
    );
}

#[test]
fn fixlengths_remove_empty_jagged_no_universal_empty() {
    // Mixed-width input with no universally-empty column: the filter is a
    // no-op and the auto-detected length is the widest record.
    let rows = vec![
        svec!["a", "b"],
        svec!["c", "", "d"],
        svec!["e", "f", "g", "h", "i"],
    ];

    let wrk = Workdir::new("fixlengths_remove_empty_jagged_no_universal_empty").flexible(true);
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("fixlengths");
    cmd.arg("in.csv").args(["-r"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(
        got,
        vec![
            svec!["a", "b", "", "", ""],
            svec!["c", "", "d", "", ""],
            svec!["e", "f", "g", "h", "i"],
        ]
    );
}

#[test]
fn prop_fixlengths_remove_empty() {
    // Quickcheck variant of prop_fixlengths_all_maxlen exercising flexible
    // input with --remove-empty. Verifies no panic and that all output rows
    // share the same width.
    fn p(rows: Vec<CsvRecord>) -> TestResult {
        if rows.is_empty() {
            return TestResult::discard();
        }

        let wrk = Workdir::new("fixlengths_remove_empty_prop").flexible(true);
        wrk.create("in.csv", rows);

        let mut cmd = wrk.command("fixlengths");
        cmd.arg("in.csv").args(["-r"]);

        let got: Vec<CsvRecord> = wrk.read_stdout(&mut cmd);
        if got.is_empty() {
            return TestResult::discard();
        }
        let got_len = got.iter().map(|r| r.len()).max().unwrap();
        for r in &got {
            assert_eq!(r.len(), got_len);
        }
        TestResult::passed()
    }
    qcheck(p as fn(Vec<CsvRecord>) -> TestResult);
}

#[test]
fn fixlengths_remove_empty_with_negative_insert() {
    let rows = vec![
        svec!["a", "", "c", "", "e"],
        svec!["f", "", "h", "", "j"],
        svec!["k", "", "m", "", "o"],
    ];

    let wrk = Workdir::new("fixlengths_remove_empty_with_negative_insert").flexible(true);
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("fixlengths");
    cmd.arg("in.csv")
        .args(["-r"])
        .args(["-l", "5"])
        .args(["-i", "-2"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(
        got,
        vec![
            svec!["a", "c", "", "", "e"],
            svec!["f", "h", "", "", "j"],
            svec!["k", "m", "", "", "o"],
        ]
    );
}
