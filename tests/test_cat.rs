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

/// Name of the real directory holding `path`, as `parentdirfname` sees it.
fn parentdir_name(path: &std::path::Path) -> String {
    path.canonicalize()
        .unwrap()
        .parent()
        .unwrap()
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned()
}

/// An expected rowskey output row: the grouping value, then the data fields.
fn grouped(group: &str, rest: &[&str]) -> Vec<String> {
    std::iter::once(group)
        .chain(rest.iter().copied())
        .map(String::from)
        .collect()
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
    // the parent is the file's real directory, so files given as bare names
    // get the work directory's own name, not an empty parent.
    let wd = parentdir_name(&wrk.path("in1.csv"));
    let sep = std::path::MAIN_SEPARATOR;
    let in1 = format!("{wd}{sep}in1.csv");
    let in2 = format!("{wd}{sep}in2.tsv");
    let in3 = format!("testdir{sep}in3.csv");
    let expected = vec![
        svec!["file", "a", "b", "c", "d"],
        grouped(&in1, &["1", "2", "3", ""]),
        grouped(&in1, &["2", "3", "4", ""]),
        grouped(&in2, &["1", "2", "3", ""]),
        grouped(&in2, &["2", "3", "4", ""]),
        grouped(&in3, &["1", "2", "3", "4"]),
        grouped(&in3, &["2", "3", "4", "5"]),
        grouped(&in3, &["z", "y", "x", "w"]),
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
    let wd = parentdir_name(&wrk.path("in1.csv"));
    let sep = std::path::MAIN_SEPARATOR;
    let in1 = format!("{wd}{sep}in1");
    let in2 = format!("{wd}{sep}in2");
    let in3 = format!("testdir{sep}in3");
    let expected = vec![
        svec!["file", "a", "b", "c", "d"],
        grouped(&in1, &["1", "2", "3", ""]),
        grouped(&in1, &["2", "3", "4", ""]),
        grouped(&in2, &["1", "2", "3", ""]),
        grouped(&in2, &["2", "3", "4", ""]),
        grouped(&in3, &["1", "2", "3", "4"]),
        grouped(&in3, &["2", "3", "4", "5"]),
        grouped(&in3, &["z", "y", "x", "w"]),
    ];
    assert_eq!(got, expected);
}

// #1506: same-named files in sibling directories are told apart by their
// immediate parent, however deep or however the path is spelled.
#[test]
fn cat_rowskey_grouping_parentdirfname_immediate_parent() {
    let wrk = Workdir::new("cat_rowskey_grouping_parentdirfname_immediate_parent");
    let sep = std::path::MAIN_SEPARATOR;
    wrk.create_subdir(&format!("2024{sep}jan")).unwrap();
    wrk.create_subdir(&format!("2024{sep}feb")).unwrap();
    wrk.create_from_string(&format!("2024{sep}jan{sep}data.csv"), "a\n1\n");
    wrk.create_from_string(&format!("2024{sep}feb{sep}data.csv"), "a\n2\n");

    let mut cmd = wrk.command("cat");
    cmd.arg("rowskey")
        .args(["--group", "parentdirfname"])
        .arg(format!("2024{sep}jan{sep}data.csv"))
        .arg(format!(".{sep}2024{sep}jan{sep}..{sep}feb{sep}data.csv"));

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["file", "a"],
        grouped(&format!("jan{sep}data.csv"), &["1"]),
        grouped(&format!("feb{sep}data.csv"), &["2"]),
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
        .args(["--group-name", "file group label"])
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

// Regression test: --no-headers second pass used to reuse only the LAST file's
// synthetic header for ALL files, so a wider file appearing BEFORE a narrower
// one would silently lose its trailing columns. With wider-first ordering, the
// expected behavior is that every input cell survives in the right slot.
#[test]
fn cat_rowskey_no_headers_widerfirst() {
    let wrk = Workdir::new("cat_rowskey_no_headers_widerfirst");
    wrk.create(
        "in1.csv",
        vec![
            svec!["a", "b", "c", "d", "e"],
            svec!["1", "2", "3", "4", "5"],
        ],
    );
    wrk.create("in2.csv", vec![svec!["x", "y", "z"], svec!["6", "7", "8"]]);

    let mut cmd = wrk.command("cat");
    cmd.arg("rowskey")
        .arg("--no-headers")
        .arg("in1.csv")
        .arg("in2.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["a", "b", "c", "d", "e"],
        svec!["1", "2", "3", "4", "5"],
        svec!["x", "y", "z", "", ""],
        svec!["6", "7", "8", "", ""],
    ];
    assert_eq!(got, expected);
}

// Mirror of the wider-first case: narrower file first, wider file second.
// Used to "work by accident" in the old code because the last file's
// synthetic header happened to be the widest. Pinning the behavior so a
// future refactor cannot regress only this direction.
#[test]
fn cat_rowskey_no_headers_narrowerfirst() {
    let wrk = Workdir::new("cat_rowskey_no_headers_narrowerfirst");
    wrk.create("in1.csv", vec![svec!["x", "y", "z"], svec!["6", "7", "8"]]);
    wrk.create(
        "in2.csv",
        vec![
            svec!["a", "b", "c", "d", "e"],
            svec!["1", "2", "3", "4", "5"],
        ],
    );

    let mut cmd = wrk.command("cat");
    cmd.arg("rowskey")
        .arg("--no-headers")
        .arg("in1.csv")
        .arg("in2.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["x", "y", "z", "", ""],
        svec!["6", "7", "8", "", ""],
        svec!["a", "b", "c", "d", "e"],
        svec!["1", "2", "3", "4", "5"],
    ];
    assert_eq!(got, expected);
}

// A row wider than its file's first row must not lose its extra fields:
// rowskey reads flexibly, so the synthetic width must be the file's widest row.
#[test]
fn cat_rowskey_no_headers_ragged_later_row_wider() {
    let wrk = Workdir::new("cat_rowskey_no_headers_ragged_later_row_wider");
    wrk.create_from_string("in1.csv", "x,y\n1,2,3,4\n");
    wrk.create_from_string("in2.csv", "p,q,r\n");

    let mut cmd = wrk.command("cat");
    cmd.arg("rowskey")
        .arg("--no-headers")
        .arg("in1.csv")
        .arg("in2.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["x", "y", "", ""],
        svec!["1", "2", "3", "4"],
        svec!["p", "q", "r", ""],
    ];
    assert_eq!(got, expected);
}

// A record wider than the ones before it must fail BEFORE it is written: the
// csv writer checks width only after writing a record's fields, which left a
// half-written, unterminated record in the output. The error names the file.
#[test]
fn cat_rows_width_mismatch_across_files_no_partial_write() {
    let wrk = Workdir::new("cat_rows_width_mismatch_across_files_no_partial_write");
    wrk.create_from_string("in1.csv", "h1,h2\nv1,v2\n");
    // in2.csv is consistent with its own header, but wider than the output
    wrk.create_from_string("in2.csv", "h1,h2,h3\n7,8,9\n");

    let mut cmd = wrk.command("cat");
    cmd.arg("rows")
        .arg("in1.csv")
        .arg("in2.csv")
        .args(["-o", "out.csv"]);

    let stderr = wrk.stderr_on_error(&mut cmd);
    assert!(
        stderr.contains("`in2.csv` line 2: found a record with 3 fields")
            && stderr.contains("have 2"),
        "stderr: {stderr}"
    );
    assert_eq!(wrk.read_to_string("out.csv").unwrap(), "h1,h2\nv1,v2\n");
}

#[test]
fn cat_rows_width_mismatch_within_file_names_file() {
    let wrk = Workdir::new("cat_rows_width_mismatch_within_file_names_file");
    wrk.create_from_string("in1.csv", "h1,h2\nv1,v2\n");
    wrk.create_from_string("in2.csv", "h1,h2\nw1,w2\nx1\n");

    let mut cmd = wrk.command("cat");
    cmd.arg("rows").arg("in1.csv").arg("in2.csv");

    let stderr = wrk.stderr_on_error(&mut cmd);
    assert!(
        stderr.contains("`in2.csv` line 3: found a record with 1 fields"),
        "stderr: {stderr}"
    );
}

// A later file's header is skipped, but its records must still match it: a
// record that fits the output width yet contradicts its own file's header is
// malformed input, as a strict reader would report.
#[test]
fn cat_rows_record_must_match_its_own_header() {
    let wrk = Workdir::new("cat_rows_record_must_match_its_own_header");
    wrk.create_from_string("in1.csv", "x,y\n1,2\n");
    wrk.create_from_string("in2.csv", "a,b,c\n1,2\n");

    let mut cmd = wrk.command("cat");
    cmd.arg("rows").arg("in1.csv").arg("in2.csv");

    let stderr = wrk.stderr_on_error(&mut cmd);
    assert!(
        stderr.contains("`in2.csv` line 2: found a record with 2 fields, but its header has 3"),
        "stderr: {stderr}"
    );
}

// A decompressed `stdin.csv.sz` lands in the temp dir as `stdin.csv`, the same
// name piped stdin gets; without piped input it must be named as itself.
#[test]
fn cat_rows_width_mismatch_compressed_stdin_named_file_is_not_stdin() {
    let wrk = Workdir::new("cat_rows_width_mismatch_compressed_stdin_named_file_is_not_stdin");
    wrk.create_from_string("in1.csv", "h1,h2\nv1,v2\n");
    wrk.create_from_string("stdin.csv", "h1,h2\n7,8,9\n");
    let mut snappy = wrk.command("snappy");
    snappy
        .args(["compress", "stdin.csv"])
        .args(["-o", "stdin.csv.sz"]);
    wrk.assert_success(&mut snappy);

    let mut cmd = wrk.command("cat");
    cmd.arg("rows").arg("in1.csv").arg("stdin.csv.sz");

    let stderr = wrk.stderr_on_error(&mut cmd);
    assert!(
        stderr.contains("`stdin.csv` line 2: found a record with 3 fields"),
        "stderr: {stderr}"
    );
}

// stdin is materialized to a temp file; the error must not show that path.
#[test]
fn cat_rows_width_mismatch_names_stdin() {
    use std::io::Write;

    let wrk = Workdir::new("cat_rows_width_mismatch_names_stdin");
    wrk.create_from_string("in1.csv", "h1,h2\nv1,v2\n");

    let mut cmd = wrk.command("cat");
    cmd.arg("rows")
        .arg("in1.csv")
        .arg("-")
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped());
    let mut child = cmd.spawn().unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"h1,h2\n7,8,9\n")
        .unwrap();
    let output = child.wait_with_output().unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("`<stdin>` line 2: found a record with 3 fields"),
        "stderr: {stderr}"
    );
}

fn snappy_compress(wrk: &Workdir, input: &str) {
    let mut snappy = wrk.command("snappy");
    snappy
        .args(["compress", input])
        .args(["-o", &format!("{input}.sz")]);
    wrk.assert_success(&mut snappy);
}

/// Run `cat rows` over `args`, piping `stdin_data` in for `-`.
fn cat_rows_with_stdin(wrk: &Workdir, args: &[&str], stdin_data: &str) -> process::Output {
    use std::io::Write;

    let mut cmd = wrk.command("cat");
    cmd.arg("rows")
        .args(args)
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped());
    let mut child = cmd.spawn().unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(stdin_data.as_bytes())
        .unwrap();
    child.wait_with_output().unwrap()
}

// Same-named .sz inputs in different directories decompress to the same file
// name; each must keep its own data (#4779).
#[test]
fn cat_rows_same_named_sz_inputs_keep_their_data() {
    let wrk = Workdir::new("cat_rows_same_named_sz_inputs_keep_their_data");
    wrk.create_subdir("a").unwrap();
    wrk.create_subdir("b").unwrap();
    wrk.create_from_string("a/data.csv", "src\nfrom_a\n");
    wrk.create_from_string("b/data.csv", "src\nfrom_b\n");
    snappy_compress(&wrk, "a/data.csv");
    snappy_compress(&wrk, "b/data.csv");

    let mut cmd = wrk.command("cat");
    cmd.arg("rows").arg("a/data.csv.sz").arg("b/data.csv.sz");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["src"], svec!["from_a"], svec!["from_b"]];
    assert_eq!(got, expected);
}

// An input whose final name looks like a decompression scratch file must not be
// overwritten by the next input's scratch file (#4779).
#[test]
fn cat_rows_sz_input_named_like_scratch_file_keeps_its_data() {
    let wrk = Workdir::new("cat_rows_sz_input_named_like_scratch_file_keeps_its_data");
    wrk.create_from_string(
        "qsv_temp_decompressed__data.csv",
        "src\nfrom_scratch_named\n",
    );
    wrk.create_from_string("data.csv", "src\nfrom_data\n");
    snappy_compress(&wrk, "qsv_temp_decompressed__data.csv");
    snappy_compress(&wrk, "data.csv");

    let mut cmd = wrk.command("cat");
    cmd.arg("rows")
        .arg("qsv_temp_decompressed__data.csv.sz")
        .arg("data.csv.sz");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["src"],
        svec!["from_scratch_named"],
        svec!["from_data"],
    ];
    assert_eq!(got, expected);
}

// stdin is read only when its turn comes: a missing earlier input must fail
// without waiting for stdin to close.
#[test]
fn cat_rows_missing_input_fails_before_reading_stdin() {
    let wrk = Workdir::new("cat_rows_missing_input_fails_before_reading_stdin");

    let mut cmd = wrk.command("cat");
    cmd.arg("rows")
        .arg("missing.csv")
        .arg("-")
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped());
    let mut child = cmd.spawn().unwrap();
    // keep stdin open: an eager read would block until it closes
    let stdin_handle = child.stdin.take().unwrap();

    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(60);
    let status = loop {
        if let Some(status) = child.try_wait().unwrap() {
            break status;
        }
        if std::time::Instant::now() > deadline {
            child.kill().unwrap();
            panic!("cat blocked on stdin instead of failing on the missing input");
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    };
    drop(stdin_handle);

    assert!(!status.success());
    let output = child.wait_with_output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("missing.csv"), "stderr: {stderr}");
}

// Same-named zips extract to the same directory name (#4779).
#[test]
fn cat_rows_same_named_zip_inputs_keep_their_data() {
    use std::io::Write;

    let wrk = Workdir::new("cat_rows_same_named_zip_inputs_keep_their_data");
    for (dir, data) in [("a", "src\nfrom_a\n"), ("b", "src\nfrom_b\n")] {
        wrk.create_subdir(dir).unwrap();
        let zf = std::fs::File::create(wrk.path(&format!("{dir}/data.zip"))).unwrap();
        let mut zw = zip::ZipWriter::new(zf);
        zw.start_file("data.csv", zip::write::SimpleFileOptions::default())
            .unwrap();
        zw.write_all(data.as_bytes()).unwrap();
        zw.finish().unwrap();
    }

    let mut cmd = wrk.command("cat");
    cmd.arg("rows").arg("a/data.zip").arg("b/data.zip");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["src"], svec!["from_a"], svec!["from_b"]];
    assert_eq!(got, expected);
}

// Piped stdin and a decompressed `stdin.csv.sz` both want `stdin.csv`; in
// either argument order each must keep its own data (#4779).
#[test]
fn cat_rows_stdin_and_stdin_named_sz_keep_their_data() {
    let wrk = Workdir::new("cat_rows_stdin_and_stdin_named_sz_keep_their_data");
    wrk.create_from_string("stdin.csv", "src\nfrom_sz\n");
    snappy_compress(&wrk, "stdin.csv");

    for (args, expected) in [
        (["-", "stdin.csv.sz"], "src\nfrom_pipe\nfrom_sz\n"),
        (["stdin.csv.sz", "-"], "src\nfrom_sz\nfrom_pipe\n"),
    ] {
        let output = cat_rows_with_stdin(&wrk, &args, "src\nfrom_pipe\n");
        assert!(output.status.success(), "{args:?}: {output:?}");
        assert_eq!(
            String::from_utf8_lossy(&output.stdout),
            expected,
            "{args:?}"
        );
    }
}

// With both present, errors still tell `stdin.csv.sz` and piped stdin apart,
// even when the .sz file is unpacked first.
#[test]
fn cat_rows_width_mismatch_stdin_and_stdin_named_sz_are_named_apart() {
    let wrk = Workdir::new("cat_rows_width_mismatch_stdin_and_stdin_named_sz_are_named_apart");
    wrk.create_from_string("stdin.csv", "h1,h2\n7,8,9\n");
    snappy_compress(&wrk, "stdin.csv");
    wrk.create_subdir("ok").unwrap();
    wrk.create_from_string("ok/stdin.csv", "h1,h2\nv1,v2\n");
    snappy_compress(&wrk, "ok/stdin.csv");

    // the .sz file is bad, stdin is good
    let output = cat_rows_with_stdin(&wrk, &["stdin.csv.sz", "-"], "h1,h2\nv1,v2\n");
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("`stdin.csv` line 2: found a record with 3 fields"),
        "stderr: {stderr}"
    );

    // stdin is bad, the .sz file is good
    let output = cat_rows_with_stdin(&wrk, &["ok/stdin.csv.sz", "-"], "h1,h2\n7,8,9\n");
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("`<stdin>` line 2: found a record with 3 fields"),
        "stderr: {stderr}"
    );
}

// An empty (0-byte) first input must not cost the output its header: the
// header comes from the first non-empty input.
#[test]
fn cat_rows_empty_first_file_keeps_header() {
    let wrk = Workdir::new("cat_rows_empty_first_file_keeps_header");
    wrk.create_from_string("empty.csv", "");
    wrk.create_from_string("in1.csv", "h1,h2\nv1,v2\n");
    wrk.create_from_string("in2.csv", "h1,h2\nw1,w2\n");

    let mut cmd = wrk.command("cat");
    cmd.arg("rows")
        .arg("empty.csv")
        .arg("in1.csv")
        .arg("in2.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["h1", "h2"], svec!["v1", "v2"], svec!["w1", "w2"]];
    assert_eq!(got, expected);
}

// Regression test: a column whose header collides with --group-name should
// still produce valid output (the file's value wins for its own rows) and
// emit a warning to stderr so the user notices.
#[test]
fn cat_rowskey_group_name_collision() {
    let wrk = Workdir::new("cat_rowskey_group_name_collision");
    wrk.create(
        "in1.csv",
        vec![svec!["file", "value"], svec!["preset", "42"]],
    );
    wrk.create("in2.csv", vec![svec!["value"], svec!["99"]]);

    let mut cmd = wrk.command("cat");
    cmd.arg("rowskey")
        .arg("--group")
        .arg("fname")
        .arg("in1.csv")
        .arg("in2.csv");

    // single execution: stderr and stdout must come from the same run.
    let output = wrk.output(&mut cmd);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("collides with --group-name"),
        "expected collision warning in stderr, got: {stderr}",
    );

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(output.stdout.as_slice());
    let got: Vec<Vec<String>> = rdr
        .records()
        .map(|record| {
            let record = record.unwrap();
            record.iter().map(ToOwned::to_owned).collect()
        })
        .collect();
    let expected = vec![
        svec!["file", "value"],
        // in1 has its own `file` column, so its value wins (documented behavior)
        svec!["preset", "42"],
        // in2 has no `file` column, so the grouping value (filename) is used
        svec!["in2.csv", "99"],
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
