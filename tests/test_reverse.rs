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
