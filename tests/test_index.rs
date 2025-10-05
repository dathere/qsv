use std::fs;

use filetime::{FileTime, set_file_times};

use crate::workdir::Workdir;

#[test]
fn index_outdated_count() {
    let wrk = Workdir::new("index_outdated_count");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "13"],
            svec!["b", "24"],
        ],
    );

    let md = fs::metadata(wrk.path("in.csv.idx")).unwrap();
    set_file_times(
        wrk.path("in.csv"),
        future_time(FileTime::from_last_modification_time(&md)),
        future_time(FileTime::from_last_access_time(&md)),
    )
    .unwrap();

    let mut cmd = wrk.command("count");
    cmd.arg("in.csv");

    // count works even if index is stale
    let expected_count = 2;
    let got_count: usize = wrk.stdout(&mut cmd);
    rassert_eq!(got_count, expected_count);
}

#[test]
fn index_outdated_stats() {
    let wrk = Workdir::new("index_outdated_stats");

    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "3"],
            svec!["b", "2"],
            svec!["c", "1"],
            svec!["d", "0"],
        ],
    );

    let md = fs::metadata(wrk.path("in.csv.idx")).unwrap();
    set_file_times(
        wrk.path("in.csv"),
        future_time(FileTime::from_last_access_time(&md)),
        future_time(FileTime::from_last_modification_time(&md)),
    )
    .unwrap();

    std::thread::sleep(std::time::Duration::from_secs(2));

    // even if the index is stale, stats should succeed
    // as the index is automatically updated
    let mut cmd = wrk.command("stats");
    cmd.args(&["--dataset-stats", "in.csv"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        [
            "field",
            "type",
            "is_ascii",
            "sum",
            "min",
            "max",
            "range",
            "sort_order",
            "sortiness",
            "min_length",
            "max_length",
            "sum_length",
            "avg_length",
            "stddev_length",
            "variance_length",
            "cv_length",
            "mean",
            "sem",
            "geometric_mean",
            "harmonic_mean",
            "stddev",
            "variance",
            "cv",
            "nullcount",
            "max_precision",
            "sparsity",
            "qsv__value",
        ],
        [
            "letter",
            "String",
            "true",
            "",
            "a",
            "d",
            "",
            "Ascending",
            "1",
            "1",
            "1",
            "4",
            "1",
            "0",
            "0",
            "0",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "0",
            "",
            "0",
            "",
        ],
        [
            "number",
            "Integer",
            "",
            "6",
            "0",
            "3",
            "3",
            "Descending",
            "-1",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "1.5",
            "0.559",
            "0",
            "",
            "1.118",
            "1.25",
            "74.5356",
            "0",
            "",
            "0",
            "",
        ],
        [
            "qsv__rowcount",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "4",
        ],
        [
            "qsv__columncount",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "2",
        ],
        [
            "qsv__filesize_bytes",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "30",
        ],
        [
            "qsv__fingerprint_hash",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "bb72a7f19b12c07ee5b2dfbd8705206e2bfcd4103433ee62e9c7d9127d45d715",
        ],
    ];

    assert_eq!(got, expected);
}

#[test]
fn index_outdated_index() {
    let wrk = Workdir::new("index_outdated_index");

    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "1"],
            svec!["b", "2"],
            svec!["c", "3"],
        ],
    );

    let md = fs::metadata(wrk.path("in.csv.idx")).unwrap();
    set_file_times(
        wrk.path("in.csv"),
        future_time(FileTime::from_last_access_time(&md)),
        future_time(FileTime::from_last_modification_time(&md)),
    )
    .unwrap();

    // slice should NOT fail if the index is stale
    // as stale indexes are automatically updated
    let mut cmd = wrk.command("slice");
    cmd.arg("-i").arg("2").arg("in.csv");

    wrk.assert_success(&mut cmd);
}

#[test]
fn index_autoindex_threshold_reached() {
    let wrk = Workdir::new("index_autoindex_threshold_reached");

    wrk.create(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "1"],
            svec!["b", "2"],
            svec!["c", "3"],
            svec!["d", "4"],
        ],
    );

    // slice should automatically create an index
    // as the file size is greater than the QSV_AUTOINDEX_SIZE threshold
    let mut cmd = wrk.command("slice");
    cmd.env("QSV_AUTOINDEX_SIZE", "1")
        .arg("-i")
        .arg("2")
        .arg("in.csv");
    wrk.assert_success(&mut cmd);

    // index should be created
    assert!(wrk.path("in.csv.idx").exists());
}

#[test]
fn index_autoindex_threshold_not_reached() {
    let wrk = Workdir::new("index_autoindex_threshold_not_reached");

    wrk.create(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "1"],
            svec!["b", "2"],
            svec!["c", "3"],
            svec!["d", "4"],
        ],
    );

    // slice will NOT automatically create an index
    // as the file size is less than the QSV_AUTOINDEX_SIZE threshold
    let mut cmd = wrk.command("slice");
    cmd.env("QSV_AUTOINDEX_SIZE", "10000000")
        .arg("-i")
        .arg("2")
        .arg("in.csv");
    wrk.assert_success(&mut cmd);

    // index should NOT be created
    assert!(!wrk.path("in.csv.idx").exists());
}

fn future_time(ft: FileTime) -> FileTime {
    let secs = ft.unix_seconds();
    FileTime::from_unix_time(secs + 10_000, 0)
}
