use crate::{Csv, CsvData, qcheck, workdir::Workdir};

fn prop_reverse(name: &str, rows: CsvData, headers: bool) -> bool {
    if rows.is_empty() {
        return true;
    }

    // Check for BOM characters in any row
    for row in rows.iter() {
        for field in row.iter() {
            if field.contains("\u{feff}") {
                return true;
            }
        }
    }

    let wrk = Workdir::new(name);
    wrk.create("in.csv", rows.clone());

    let mut cmd = wrk.command("reverse");
    cmd.arg("in.csv");
    if !headers {
        cmd.arg("--no-headers");
    }

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let mut expected = rows.to_vecs();
    let headers = if headers && !expected.is_empty() {
        expected.remove(0)
    } else {
        vec![]
    };
    expected.reverse();
    // Check for BOM characters in expected results
    for row in &expected {
        for field in row {
            if field.contains("\u{feff}") {
                return true;
            }
        }
    }
    if !headers.is_empty() {
        expected.insert(0, headers);
    }
    rassert_eq!(got, expected)
}

#[test]
fn prop_reverse_headers() {
    fn p(rows: CsvData) -> bool {
        prop_reverse("prop_reverse_headers", rows, true)
    }
    qcheck(p as fn(CsvData) -> bool);
}

#[test]
fn prop_reverse_no_headers() {
    fn p(rows: CsvData) -> bool {
        prop_reverse("prop_reverse_no_headers", rows, false)
    }
    qcheck(p as fn(CsvData) -> bool);
}

fn prop_reverse_indexed(name: &str, rows: CsvData, headers: bool) -> bool {
    if rows.is_empty() {
        return true;
    }

    // Check for BOM characters in any row, not just first and last
    for row in rows.iter() {
        for field in row.iter() {
            if field.contains("\u{feff}") {
                return true;
            }
        }
    }

    let wrk = Workdir::new(name);
    wrk.create_indexed("in.csv", rows.clone());

    let mut cmd = wrk.command("reverse");
    cmd.arg("in.csv");
    if !headers {
        cmd.arg("--no-headers");
    }

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let mut expected = rows.to_vecs();
    let headers = if headers && !expected.is_empty() {
        expected.remove(0)
    } else {
        vec![]
    };
    expected.reverse();
    // Check for BOM characters in expected results
    for row in &expected {
        for field in row {
            if field.contains("\u{feff}") {
                return true;
            }
        }
    }
    if !headers.is_empty() {
        expected.insert(0, headers);
    }
    rassert_eq!(got, expected)
}

#[test]
fn prop_reverse_headers_indexed() {
    fn p(rows: CsvData) -> bool {
        prop_reverse_indexed("prop_reverse_headers_indexed", rows, true)
    }
    qcheck(p as fn(CsvData) -> bool);
}

#[test]
fn prop_reverse_no_headers_indexed() {
    fn p(rows: CsvData) -> bool {
        prop_reverse_indexed("prop_reverse_no_headers_indexed", rows, false)
    }
    qcheck(p as fn(CsvData) -> bool);
}

// Regression test for the indexed-reverse u64 underflow.
// The original bug wrote correct output to stdout and then panicked on
// `pos -= 1` after reading record 0; debug builds exit non-zero, but the
// `prop_reverse_*_indexed` property tests only diff stdout via
// `read_stdout` and so missed it. This test asserts the exit status is
// success in addition to checking the output, covering the minimal
// 1-data-row case that originally triggered the panic, plus a multi-row
// case for both `--no-headers` on and off.
#[test]
fn reverse_indexed_asserts_success() {
    // 1-row indexed CSV with headers — the minimal panic-triggering case.
    let wrk = Workdir::new("reverse_indexed_asserts_success_1row");
    wrk.create_indexed("in.csv", vec![svec!["h1", "h2"], svec!["a", "b"]]);
    let mut cmd = wrk.command("reverse");
    cmd.arg("in.csv");
    wrk.assert_success(&mut cmd);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["h1", "h2"], svec!["a", "b"]];
    assert_eq!(got, expected);

    // Multi-row indexed CSV with headers.
    let wrk = Workdir::new("reverse_indexed_asserts_success_multi");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["h1", "h2"],
            svec!["a", "1"],
            svec!["b", "2"],
            svec!["c", "3"],
        ],
    );
    let mut cmd = wrk.command("reverse");
    cmd.arg("in.csv");
    wrk.assert_success(&mut cmd);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["h1", "h2"],
        svec!["c", "3"],
        svec!["b", "2"],
        svec!["a", "1"],
    ];
    assert_eq!(got, expected);

    // Multi-row indexed CSV with --no-headers.
    let wrk = Workdir::new("reverse_indexed_asserts_success_no_headers");
    wrk.create_indexed(
        "in.csv",
        vec![svec!["a", "1"], svec!["b", "2"], svec!["c", "3"]],
    );
    let mut cmd = wrk.command("reverse");
    cmd.arg("--no-headers").arg("in.csv");
    wrk.assert_success(&mut cmd);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["c", "3"], svec!["b", "2"], svec!["a", "1"]];
    assert_eq!(got, expected);
}
