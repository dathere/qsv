#![cfg(not(feature = "datapusher_plus"))]
use std::process;

use serial_test::serial;

use crate::{Csv, CsvData, qcheck, quickcheck::TestResult, workdir::Workdir};

fn no_headers(cmd: &mut process::Command) {
    cmd.arg("--no-headers");
}

fn pad(cmd: &mut process::Command) {
    cmd.arg("--pad");
}

fn run_cat<X, Y, Z, F>(test_name: &str, which: &str, rows1: X, rows2: Y, modify_cmd: F) -> Z
where
    X: Csv,
    Y: Csv,
    Z: Csv,
    F: FnOnce(&mut process::Command),
{
    let wrk = Workdir::new(test_name);
    wrk.create("in1.csv", rows1);
    wrk.create("in2.csv", rows2);

    let mut cmd = wrk.command("cat");
    modify_cmd(
        cmd.env("QSV_SKIP_FORMAT_CHECK", "1")
            .arg(which)
            .arg("in1.csv")
            .arg("in2.csv"),
    );
    wrk.read_stdout(&mut cmd)
}

#[test]
#[serial]
fn prop_cat_rows() {
    fn p(rows: CsvData) -> bool {
        let expected = rows.clone();
        let (rows1, rows2) = if rows.is_empty() {
            (vec![], vec![])
        } else {
            let (rows1, rows2) = rows.split_at(rows.len() / 2);
            (rows1.to_vec(), rows2.to_vec())
        };
        let got: CsvData = run_cat("cat_rows", "rows", rows1, rows2, no_headers);
        rassert_eq!(got, expected)
    }
    qcheck(p as fn(CsvData) -> bool);
}

#[test]
fn cat_rows_space() {
    let rows = vec![svec!["\u{0085}"]];
    let expected = rows.clone();
    let (rows1, rows2) = if rows.is_empty() {
        (vec![], vec![])
    } else {
        let (rows1, rows2) = rows.split_at(rows.len() / 2);
        (rows1.to_vec(), rows2.to_vec())
    };
    let got: Vec<Vec<String>> = run_cat("cat_rows_space", "rows", rows1, rows2, no_headers);
    assert_eq!(got, expected);
}

#[test]
fn cat_rows_headers() {
    let rows1 = vec![svec!["h1", "h2"], svec!["a", "b"]];
    let rows2 = vec![svec!["h1", "h2"], svec!["y", "z"]];

    let mut expected = rows1.clone();
    expected.extend(rows2.clone().into_iter().skip(1));

    let got: Vec<Vec<String>> = run_cat("cat_rows_headers", "rows", rows1, rows2, |_| ());
    assert_eq!(got, expected);
}

#[test]
fn cat_rowskey() {
    let wrk = Workdir::new("cat_rowskey");
    wrk.create(
        "in1.csv",
        vec![
            svec!["a", "b", "c"],
            svec!["1", "2", "3"],
            svec!["2", "3", "4"],
        ],
    );

    wrk.create(
        "in2.csv",
        vec![
            svec!["c", "a", "b"],
            svec!["3", "1", "2"],
            svec!["4", "2", "3"],
        ],
    );

    wrk.create(
        "in3.csv",
        vec![
            svec!["a", "b", "d", "c"],
            svec!["1", "2", "4", "3"],
            svec!["2", "3", "5", "4"],
            svec!["z", "y", "w", "x"],
        ],
    );

    let mut cmd = wrk.command("cat");
    cmd.arg("rowskey")
        .arg("in1.csv")
        .arg("in2.csv")
        .arg("in3.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["a", "b", "c", "d"],
        svec!["1", "2", "3", ""],
        svec!["2", "3", "4", ""],
        svec!["1", "2", "3", ""],
        svec!["2", "3", "4", ""],
        svec!["1", "2", "3", "4"],
        svec!["2", "3", "4", "5"],
        svec!["z", "y", "x", "w"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn cat_rowskey_ssv_tsv() {
    let wrk = Workdir::new("cat_rowskey_ssv_tsv");
    wrk.create_with_delim(
        "in1.tsv",
        vec![
            svec!["a", "b", "c"],
            svec!["1", "2", "3"],
            svec!["2", "3", "4"],
        ],
        b'\t',
    );

    wrk.create(
        "in2.csv",
        vec![
            svec!["c", "a", "b"],
            svec!["3", "1", "2"],
            svec!["4", "2", "3"],
        ],
    );

    wrk.create_with_delim(
        "in3.ssv",
        vec![
            svec!["a", "b", "d", "c"],
            svec!["1", "2", "4", "3"],
            svec!["2", "3", "5", "4"],
            svec!["z", "y", "w", "x"],
        ],
        b';',
    );

    let mut cmd = wrk.command("cat");
    cmd.arg("rowskey")
        .arg("in1.tsv")
        .arg("in2.csv")
        .arg("in3.ssv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["a", "b", "c", "d"],
        svec!["1", "2", "3", ""],
        svec!["2", "3", "4", ""],
        svec!["1", "2", "3", ""],
        svec!["2", "3", "4", ""],
        svec!["1", "2", "3", "4"],
        svec!["2", "3", "4", "5"],
        svec!["z", "y", "x", "w"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn cat_rows_flexible() {
    let wrk = Workdir::new("cat_rows_flexible");
    wrk.create(
        "in1.csv",
        vec![
            svec!["a", "b", "c"],
            svec!["1", "2", "3"],
            svec!["2", "3", "4"],
        ],
    );

    wrk.create(
        "in2.csv",
        vec![
            svec!["a", "b", "c"],
            svec!["3", "1", "2"],
            svec!["4", "2", "3"],
        ],
    );

    wrk.create(
        "in3.csv",
        vec![
            svec!["a", "b", "c", "d"],
            svec!["1", "2", "4", "3"],
            svec!["2", "3", "5", "4"],
            svec!["z", "y", "w", "x"],
        ],
    );

    let mut cmd = wrk.command("cat");
    cmd.arg("rows")
        .arg("--flexible")
        .arg("in1.csv")
        .arg("in2.csv")
        .arg("in3.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["a", "b", "c"],
        svec!["1", "2", "3"],
        svec!["2", "3", "4"],
        svec!["3", "1", "2"],
        svec!["4", "2", "3"],
        svec!["1", "2", "4", "3"],
        svec!["2", "3", "5", "4"],
        svec!["z", "y", "w", "x"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn cat_rows_flexible_infile() {
    let wrk = Workdir::new("cat_rows_flexible_infile");
    wrk.create(
        "in1.csv",
        vec![
            svec!["a", "b", "c"],
            svec!["1", "2", "3"],
            svec!["2", "3", "4"],
        ],
    );

    wrk.create(
        "in2.csv",
        vec![
            svec!["a", "b", "c"],
            svec!["3", "1", "2"],
            svec!["4", "2", "3"],
        ],
    );

    wrk.create(
        "in3.csv",
        vec![
            svec!["a", "b", "c", "d"],
            svec!["1", "2", "4", "3"],
            svec!["2", "3", "5", "4"],
            svec!["z", "y", "w", "x"],
        ],
    );

    wrk.create_from_string("testdata.infile-list", "in1.csv\nin2.csv\nin3.csv\n");

    let mut cmd = wrk.command("cat");
    cmd.arg("rows")
        .arg("--flexible")
        .arg("testdata.infile-list");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["a", "b", "c"],
        svec!["1", "2", "3"],
        svec!["2", "3", "4"],
        svec!["3", "1", "2"],
        svec!["4", "2", "3"],
        svec!["1", "2", "4", "3"],
        svec!["2", "3", "5", "4"],
        svec!["z", "y", "w", "x"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn cat_rowskey_grouping() {
    let wrk = Workdir::new("cat_rowskey_grouping");
    wrk.create(
        "in1.csv",
        vec![
            svec!["a", "b", "c"],
            svec!["1", "2", "3"],
            svec!["2", "3", "4"],
        ],
    );

    wrk.create(
        "in2.csv",
        vec![
            svec!["c", "a", "b"],
            svec!["3", "1", "2"],
            svec!["4", "2", "3"],
        ],
    );

    wrk.create(
        "in3.csv",
        vec![
            svec!["a", "b", "d", "c"],
            svec!["1", "2", "4", "3"],
            svec!["2", "3", "5", "4"],
            svec!["z", "y", "w", "x"],
        ],
    );

    let mut cmd = wrk.command("cat");
    cmd.arg("rowskey")
        .args(["--group", "fstem"])
        .arg("in1.csv")
        .arg("in2.csv")
        .arg("in3.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["file", "a", "b", "c", "d"],
        svec!["in1", "1", "2", "3", ""],
        svec!["in1", "2", "3", "4", ""],
        svec!["in2", "1", "2", "3", ""],
        svec!["in2", "2", "3", "4", ""],
        svec!["in3", "1", "2", "3", "4"],
        svec!["in3", "2", "3", "4", "5"],
        svec!["in3", "z", "y", "x", "w"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn cat_rowskey_grouping_noheader() {
    let wrk = Workdir::new("cat_rowskey_grouping_noheader");
    wrk.create(
        "in1.csv",
        vec![
            svec!["a", "b", "c"],
            svec!["1", "2", "3"],
            svec!["2", "3", "4"],
        ],
    );

    wrk.create(
        "in2.csv",
        vec![
            svec!["c", "a", "b"],
            svec!["3", "1", "2"],
            svec!["4", "2", "3"],
        ],
    );

    wrk.create(
        "in3.csv",
        vec![
            svec!["a", "b", "d", "c"],
            svec!["1", "2", "4", "3"],
            svec!["2", "3", "5", "4"],
            svec!["z", "y", "w", "x"],
        ],
    );

    let mut cmd = wrk.command("cat");
    cmd.arg("rowskey")
        .args(["--group", "fstem"])
        .arg("--no-headers")
        .arg("in1.csv")
        .arg("in2.csv")
        .arg("in3.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["in1", "a", "b", "c", ""],
        svec!["in1", "1", "2", "3", ""],
        svec!["in1", "2", "3", "4", ""],
        svec!["in2", "c", "a", "b", ""],
        svec!["in2", "3", "1", "2", ""],
        svec!["in2", "4", "2", "3", ""],
        svec!["in3", "a", "b", "d", "c"],
        svec!["in3", "1", "2", "4", "3"],
        svec!["in3", "2", "3", "5", "4"],
        svec!["in3", "z", "y", "w", "x"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn cat_rowskey_grouping_parentdirfname() {
    let wrk = Workdir::new("cat_rowskey_grouping_parentdirfname");
    wrk.create(
        "in1.csv",
        vec![
            svec!["a", "b", "c"],
            svec!["1", "2", "3"],
            svec!["2", "3", "4"],
        ],
    );

    wrk.create_with_delim(
        "in2.tsv",
        vec![
            svec!["c", "a", "b"],
            svec!["3", "1", "2"],
            svec!["4", "2", "3"],
        ],
        b'\t',
    );

    // create a subdirectory and put in3.csv in it
    let _ = wrk.create_subdir("testdir");

    wrk.create(
        "testdir/in3.csv",
        vec![
            svec!["a", "b", "d", "c"],
            svec!["1", "2", "4", "3"],
            svec!["2", "3", "5", "4"],
            svec!["z", "y", "w", "x"],
        ],
    );

    let mut cmd = wrk.command("cat");
    cmd.arg("rowskey")
        .args(["--group", "parentdirfname"])
        .arg("in1.csv")
        .arg("in2.tsv")
        .arg("testdir/in3.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // on Windows, the directory separator is backslash, which is an escape character in CSV
    // strings. So we get double backslashes in the output.
    #[cfg(windows)]
    let expected = vec![
        svec!["file", "a", "b", "c", "d"],
        svec!["in1.csv", "1", "2", "3", ""],
        svec!["in1.csv", "2", "3", "4", ""],
        svec!["in2.tsv", "1", "2", "3", ""],
        svec!["in2.tsv", "2", "3", "4", ""],
        svec!["testdir\\in3.csv", "1", "2", "3", "4"],
        svec!["testdir\\in3.csv", "2", "3", "4", "5"],
        svec!["testdir\\in3.csv", "z", "y", "x", "w"],
    ];
    #[cfg(not(windows))]
    let expected = vec![
        svec!["file", "a", "b", "c", "d"],
        svec!["in1.csv", "1", "2", "3", ""],
        svec!["in1.csv", "2", "3", "4", ""],
        svec!["in2.tsv", "1", "2", "3", ""],
        svec!["in2.tsv", "2", "3", "4", ""],
        svec!["testdir/in3.csv", "1", "2", "3", "4"],
        svec!["testdir/in3.csv", "2", "3", "4", "5"],
        svec!["testdir/in3.csv", "z", "y", "x", "w"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn cat_rowskey_grouping_parentdirfstem() {
    let wrk = Workdir::new("cat_rowskey_grouping_parentdirfstem");
    wrk.create(
        "in1.csv",
        vec![
            svec!["a", "b", "c"],
            svec!["1", "2", "3"],
            svec!["2", "3", "4"],
        ],
    );

    wrk.create(
        "in2.csv",
        vec![
            svec!["c", "a", "b"],
            svec!["3", "1", "2"],
            svec!["4", "2", "3"],
        ],
    );

    // create a subdirectory and put in3.csv in it
    let _ = wrk.create_subdir("testdir");

    wrk.create(
        "testdir/in3.csv",
        vec![
            svec!["a", "b", "d", "c"],
            svec!["1", "2", "4", "3"],
            svec!["2", "3", "5", "4"],
            svec!["z", "y", "w", "x"],
        ],
    );

    let mut cmd = wrk.command("cat");
    cmd.arg("rowskey")
        .args(["--group", "parentdirfstem"])
        .arg("in1.csv")
        .arg("in2.csv")
        .arg("testdir/in3.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // on Windows, the directory separator is backslash, which is an escape character in CSV
    // strings. So we get double backslashes in the output.
    #[cfg(windows)]
    let expected = vec![
        svec!["file", "a", "b", "c", "d"],
        svec!["in1", "1", "2", "3", ""],
        svec!["in1", "2", "3", "4", ""],
        svec!["in2", "1", "2", "3", ""],
        svec!["in2", "2", "3", "4", ""],
        svec!["testdir\\in3", "1", "2", "3", "4"],
        svec!["testdir\\in3", "2", "3", "4", "5"],
        svec!["testdir\\in3", "z", "y", "x", "w"],
    ];
    #[cfg(not(windows))]
    let expected = vec![
        svec!["file", "a", "b", "c", "d"],
        svec!["in1", "1", "2", "3", ""],
        svec!["in1", "2", "3", "4", ""],
        svec!["in2", "1", "2", "3", ""],
        svec!["in2", "2", "3", "4", ""],
        svec!["testdir/in3", "1", "2", "3", "4"],
        svec!["testdir/in3", "2", "3", "4", "5"],
        svec!["testdir/in3", "z", "y", "x", "w"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn cat_rowskey_grouping_infile() {
    let wrk = Workdir::new("cat_rowskey_grouping_infile");
    wrk.create(
        "in1.csv",
        vec![
            svec!["a", "b", "c"],
            svec!["1", "2", "3"],
            svec!["2", "3", "4"],
        ],
    );

    wrk.create(
        "in2.csv",
        vec![
            svec!["c", "a", "b"],
            svec!["3", "1", "2"],
            svec!["4", "2", "3"],
        ],
    );

    wrk.create(
        "in3.csv",
        vec![
            svec!["a", "b", "d", "c"],
            svec!["1", "2", "4", "3"],
            svec!["2", "3", "5", "4"],
            svec!["z", "y", "w", "x"],
        ],
    );

    wrk.create_from_string("testdata.infile-list", "in1.csv\nin2.csv\nin3.csv\n");

    let mut cmd = wrk.command("cat");
    cmd.arg("rowskey")
        .args(["-g", "FStem"])
        .arg("testdata.infile-list");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["file", "a", "b", "c", "d"],
        svec!["in1", "1", "2", "3", ""],
        svec!["in1", "2", "3", "4", ""],
        svec!["in2", "1", "2", "3", ""],
        svec!["in2", "2", "3", "4", ""],
        svec!["in3", "1", "2", "3", "4"],
        svec!["in3", "2", "3", "4", "5"],
        svec!["in3", "z", "y", "x", "w"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn cat_rowskey_grouping_customname() {
    let wrk = Workdir::new("cat_rowskey_grouping_customname");
    wrk.create(
        "in1.csv",
        vec![
            svec!["a", "b", "c"],
            svec!["1", "2", "3"],
            svec!["2", "3", "4"],
        ],
    );

    wrk.create(
        "in2.csv",
        vec![
            svec!["c", "a", "b"],
            svec!["3", "1", "2"],
            svec!["4", "2", "3"],
        ],
    );

    wrk.create(
        "in3.csv",
        vec![
            svec!["a", "b", "d", "c"],
            svec!["1", "2", "4", "3"],
            svec!["2", "3", "5", "4"],
            svec!["z", "y", "w", "x"],
        ],
    );

    let mut cmd = wrk.command("cat");
    cmd.arg("rowskey")
        .args(["--group", "fstem"])
        .args(&["--group-name", "file group label"])
        .arg("in1.csv")
        .arg("in2.csv")
        .arg("in3.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["file group label", "a", "b", "c", "d"],
        svec!["in1", "1", "2", "3", ""],
        svec!["in1", "2", "3", "4", ""],
        svec!["in2", "1", "2", "3", ""],
        svec!["in2", "2", "3", "4", ""],
        svec!["in3", "1", "2", "3", "4"],
        svec!["in3", "2", "3", "4", "5"],
        svec!["in3", "z", "y", "x", "w"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn cat_rowskey_insertion_order() {
    let wrk = Workdir::new("cat_rowskey_insertion_order");
    wrk.create(
        "in1.csv",
        vec![
            svec!["j", "b", "c"],
            svec!["1", "2", "3"],
            svec!["2", "3", "4"],
        ],
    );

    wrk.create(
        "in2.csv",
        vec![
            svec!["c", "j", "b"],
            svec!["3", "1", "2"],
            svec!["4", "2", "3"],
        ],
    );

    wrk.create(
        "in3.csv",
        vec![
            svec!["j", "b", "d", "c"],
            svec!["1", "2", "4", "3"],
            svec!["2", "3", "5", "4"],
            svec!["z", "y", "w", "x"],
        ],
    );

    let mut cmd = wrk.command("cat");
    cmd.arg("rowskey")
        .arg("in1.csv")
        .arg("in2.csv")
        .arg("in3.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["j", "b", "c", "d"],
        svec!["1", "2", "3", ""],
        svec!["2", "3", "4", ""],
        svec!["1", "2", "3", ""],
        svec!["2", "3", "4", ""],
        svec!["1", "2", "3", "4"],
        svec!["2", "3", "4", "5"],
        svec!["z", "y", "x", "w"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn cat_rowskey_insertion_order_noheader() {
    let wrk = Workdir::new("cat_rowskey_insertion_order_noheader");
    wrk.create(
        "in1.csv",
        vec![
            svec!["j", "b", "c"],
            svec!["1", "2", "3"],
            svec!["2", "3", "4"],
        ],
    );

    wrk.create(
        "in2.csv",
        vec![
            svec!["c", "j", "b"],
            svec!["3", "1", "2"],
            svec!["4", "2", "3"],
        ],
    );

    wrk.create(
        "in3.csv",
        vec![
            svec!["j", "b", "d", "c"],
            svec!["1", "2", "4", "3"],
            svec!["2", "3", "5", "4"],
            svec!["z", "y", "w", "x"],
        ],
    );

    let mut cmd = wrk.command("cat");
    cmd.arg("rowskey")
        .arg("--no-headers")
        .arg("in1.csv")
        .arg("in2.csv")
        .arg("in3.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["j", "b", "c", ""],
        svec!["1", "2", "3", ""],
        svec!["2", "3", "4", ""],
        svec!["c", "j", "b", ""],
        svec!["3", "1", "2", ""],
        svec!["4", "2", "3", ""],
        svec!["j", "b", "d", "c"],
        svec!["1", "2", "4", "3"],
        svec!["2", "3", "5", "4"],
        svec!["z", "y", "w", "x"],
    ];
    assert_eq!(got, expected);
}

#[test]
#[serial]
fn prop_cat_cols() {
    fn p(rows1: CsvData, rows2: CsvData) -> TestResult {
        let got: Vec<Vec<String>> = run_cat(
            "cat_cols",
            "columns",
            rows1.clone(),
            rows2.clone(),
            no_headers,
        );

        let mut expected: Vec<Vec<String>> = vec![];
        let (rows1, rows2) = (rows1.to_vecs().into_iter(), rows2.to_vecs().into_iter());
        for (mut r1, r2) in rows1.zip(rows2) {
            r1.extend(r2.into_iter());
            expected.push(r1);
        }
        assert_eq!(got, expected);
        TestResult::passed()
    }
    qcheck(p as fn(CsvData, CsvData) -> TestResult);
}

#[test]
fn cat_cols_headers() {
    let rows1 = vec![svec!["h1", "h2"], svec!["a", "b"]];
    let rows2 = vec![svec!["h3", "h4"], svec!["y", "z"]];

    let expected = vec![svec!["h1", "h2", "h3", "h4"], svec!["a", "b", "y", "z"]];
    let got: Vec<Vec<String>> = run_cat("cat_cols_headers", "columns", rows1, rows2, |_| ());
    assert_eq!(got, expected);
}

#[test]
fn cat_cols_no_pad() {
    let rows1 = vec![svec!["a", "b"]];
    let rows2 = vec![svec!["y", "z"], svec!["y", "z"]];

    let expected = vec![svec!["a", "b", "y", "z"]];
    let got: Vec<Vec<String>> = run_cat("cat_cols_headers", "columns", rows1, rows2, no_headers);
    assert_eq!(got, expected);
}

#[test]
fn cat_cols_pad() {
    let rows1 = vec![svec!["a", "b"]];
    let rows2 = vec![svec!["y", "z"], svec!["y", "z"]];

    let expected = vec![svec!["a", "b", "y", "z"], svec!["", "", "y", "z"]];
    let got: Vec<Vec<String>> = run_cat("cat_cols_headers", "columns", rows1, rows2, pad);
    assert_eq!(got, expected);
}

#[test]
fn cat_rows_directory_skip_format_check() {
    let wrk = Workdir::new("cat_rows_directory_skip_format_check");

    // Create a subdirectory to test directory processing
    let _ = wrk.create_subdir("test");

    // Create a file with unsupported extension (.txt) that would normally be filtered out
    wrk.create_from_string("test/test.txt", "col_name");

    // Also create a supported CSV file to ensure both are processed
    wrk.create("test/valid.csv", vec![svec!["header"], svec!["data"]]);

    let mut cmd = wrk.command("cat");
    cmd.env("QSV_SKIP_FORMAT_CHECK", "1")
        .arg("rows")
        .arg("--no-headers") // Use no-headers since the .txt file doesn't have proper headers
        .arg("test");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // When QSV_SKIP_FORMAT_CHECK is set, both files should be processed
    // The exact order may vary, but we should see content from both files
    // Since we're using --no-headers, all lines are treated as data
    let expected_lines = vec!["col_name", "header", "data"];

    // Convert output to a flat list of strings for easier comparison
    let got_flat: Vec<String> = got.into_iter().flatten().collect();

    // Check that we got all expected content (order may vary due to directory traversal)
    for expected_line in expected_lines {
        assert!(
            got_flat.contains(&expected_line.to_string()),
            "Expected to find '{}' in output: {:?}",
            expected_line,
            got_flat
        );
    }

    // Ensure we got the expected number of lines
    assert_eq!(
        got_flat.len(),
        3,
        "Expected 3 lines of output, got: {:?}",
        got_flat
    );
}

#[test]
fn cat_rows_directory_skip_format_check_only_unsupported() {
    let wrk = Workdir::new("cat_rows_directory_skip_format_check_only_unsupported");

    // Create a subdirectory to test directory processing
    let _ = wrk.create_subdir("test");

    // Create only a file with unsupported extension - this reproduces the exact issue scenario
    wrk.create_from_string("test/test.txt", "col_name");

    let mut cmd = wrk.command("cat");
    cmd.env("QSV_SKIP_FORMAT_CHECK", "1")
        .arg("rows")
        .arg("--no-headers")
        .arg("test");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["col_name"]];

    assert_eq!(got, expected);
}

#[test]
fn cat_rows_directory_without_skip_format_check_fails() {
    let wrk = Workdir::new("cat_rows_directory_without_skip_format_check");

    // Create a subdirectory to test directory processing
    let _ = wrk.create_subdir("test");

    // Create only a file with unsupported extension
    wrk.create_from_string("test/test.txt", "col_name");

    let mut cmd = wrk.command("cat");
    cmd.arg("rows").arg("test");

    // This should fail with the error message mentioned in the issue
    let output = wrk.output(&mut cmd);
    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains(
            "No data on stdin. Please provide at least one input file or pipe data to stdin."
        ),
        "Expected error message not found. Stderr: {}",
        stderr
    );
}
