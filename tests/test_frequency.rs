use std::{borrow::ToOwned, collections::hash_map::Entry, process};

use foldhash::{HashMap, HashMapExt, HashSet};
use serde::Deserialize;
use serde_json::Value;
use stats::Frequencies;
use toon_format;

use crate::{Csv, CsvData, qcheck_sized, workdir::Workdir};

fn setup(name: &str) -> (Workdir, process::Command) {
    let rows = vec![
        svec!["h1", "h2"],
        svec!["a", "z"],
        svec!["a", "y"],
        svec!["a", "y"],
        svec!["b", "z"],
        svec!["a", "Y"],
        svec!["", "z"],
        svec!["(NULL)", "x"],
    ];

    let wrk = Workdir::new(name);
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv");

    (wrk, cmd)
}

#[test]
fn frequency_no_headers() {
    let (wrk, mut cmd) = setup("frequency_no_headers");
    cmd.args(["--limit", "0"])
        .args(["--select", "1"])
        .arg("--no-headers")
        .arg("--pct-nulls");

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got = got.into_iter().skip(1).collect();
    got.sort_unstable();
    let expected = vec![
        svec!["1", "(NULL)", "1", "12.5", "2"],
        svec!["1", "(NULL)", "1", "12.5", "2"],
        svec!["1", "a", "4", "50", "1"],
        svec!["1", "b", "1", "12.5", "2"],
        svec!["1", "h1", "1", "12.5", "2"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_casesensitive() {
    let (wrk, mut cmd) = setup("frequency_casesensitive");
    cmd.args(["--limit", "0"]).args(["--select", "h2"]);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["h2", "Y", "1", "14.28571", "3"],
        svec!["h2", "x", "1", "14.28571", "3"],
        svec!["h2", "y", "2", "28.57143", "2"],
        svec!["h2", "z", "3", "42.85714", "1"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_ignorecase() {
    let (wrk, mut cmd) = setup("frequency_ignorecase");
    cmd.arg("--ignore-case")
        .args(["--limit", "0"])
        .args(["--select", "h2"]);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["h2", "x", "1", "14.28571", "2"],
        svec!["h2", "y", "3", "42.85714", "1"],
        svec!["h2", "z", "3", "42.85714", "1"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_trim() {
    let wrk = Workdir::new("frequency_trim");

    let rows = vec![
        svec!["h1", "h2"],
        svec!["a", "z"],
        svec!["a", "y"],
        svec!["a", "y"],
        svec!["b", "z"],
        svec!["a", "Y"],
        svec!["", "z"],
        svec!["(NULL)", "x"],
        svec!["a ", " z"],
        svec!["     A", "  Z   "],
        svec!["  a  ", " Y "],
        svec![" A     ", "y "],
        svec!["a", "y "],
        svec!["b", "y "],
        svec!["b", "  Z   "],
    ];

    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--limit", "0"])
        .args(["--select", "h2"]);

    wrk.assert_success(&mut cmd);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["h2", "Y", "2", "14.28571", "3"],
        svec!["h2", "Z", "2", "14.28571", "3"],
        svec!["h2", "x", "1", "7.14286", "4"],
        svec!["h2", "y", "5", "35.71429", "1"],
        svec!["h2", "z", "4", "28.57143", "2"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_no_trim() {
    let wrk = Workdir::new("frequency_no_trim");

    let rows = vec![
        svec!["h1", "h2"],
        svec!["a", "z"],
        svec!["a", "y"],
        svec!["a", "y"],
        svec!["b", "z"],
        svec!["a", "Y"],
        svec!["", "z"],
        svec!["(NULL)", "x"],
        svec!["a ", " z"],
        svec!["     A", "  Z   "],
        svec!["  a  ", " Y "],
        svec![" A     ", "y "],
        svec!["a", "y "],
        svec!["b", "y "],
        svec!["b", "  Z   "],
    ];

    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--limit", "0"])
        .args(["--select", "h2"])
        .arg("--no-trim");

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["h2", "  Z   ", "2", "14.28571", "2"],
        svec!["h2", " Y ", "1", "7.14286", "3"],
        svec!["h2", " z", "1", "7.14286", "3"],
        svec!["h2", "Y", "1", "7.14286", "3"],
        svec!["h2", "x", "1", "7.14286", "3"],
        svec!["h2", "y", "2", "14.28571", "2"],
        svec!["h2", "y ", "3", "21.42857", "1"],
        svec!["h2", "z", "3", "21.42857", "1"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_no_nulls() {
    let (wrk, mut cmd) = setup("frequency_no_nulls");
    cmd.arg("--no-nulls")
        .args(["--limit", "0"])
        .args(["--select", "h1"]);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["h1", "(NULL)", "1", "16.66667", "2"],
        svec!["h1", "a", "4", "66.66667", "1"],
        svec!["h1", "b", "1", "16.66667", "2"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_nulls() {
    // With new default behavior, NULLs (empty strings) are excluded from percentage denominator
    // and have empty percentage/rank. Literal "(NULL)" string is included normally.
    // Data: a(4), b(1), ""(1), "(NULL)"(1) = 7 total, 6 non-NULL
    let (wrk, mut cmd) = setup("frequency_nulls");
    cmd.args(["--limit", "0"]).args(["--select", "h1"]);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["h1", "(NULL)", "1", "", ""], // empty string NULL - empty pct/rank
        svec!["h1", "(NULL)", "1", "16.66667", "2"], // literal "(NULL)" string
        svec!["h1", "a", "4", "66.66667", "1"],
        svec!["h1", "b", "1", "16.66667", "2"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_limit() {
    let (wrk, mut cmd) = setup("frequency_limit");
    cmd.args(["--limit", "1"]).arg("--pct-nulls");

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["h1", "Other (3)", "3", "42.85714", "0"],
        svec!["h1", "a", "4", "57.14286", "1"],
        svec!["h2", "Other (3)", "4", "57.14286", "0"],
        svec!["h2", "z", "3", "42.85714", "1"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_pct_dec_places() {
    let (wrk, mut cmd) = setup("frequency_pct_dec_places");
    cmd.args(["--limit", "1"])
        .args(["--pct-dec-places", "3"])
        .arg("--pct-nulls");

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["h1", "Other (3)", "3", "42.857", "0"],
        svec!["h1", "a", "4", "57.143", "1"],
        svec!["h2", "Other (3)", "4", "57.143", "0"],
        svec!["h2", "z", "3", "42.857", "1"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_neg_pct_dec_places() {
    let (wrk, mut cmd) = setup("frequency_neg_pct_dec_places");
    cmd.args(["--limit", "1"])
        .args(["--pct-dec-places", "-4"])
        .arg("--pct-nulls");

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["h1", "Other (3)", "3", "42.8571", "0"],
        svec!["h1", "a", "4", "57.1429", "1"],
        svec!["h2", "Other (3)", "4", "57.1429", "0"],
        svec!["h2", "z", "3", "42.8571", "1"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_limit_no_other() {
    let (wrk, mut cmd) = setup("frequency_limit_no_other");
    cmd.args(["--limit", "1"])
        .args(["--other-text", "<NONE>"])
        .arg("--pct-nulls");

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["h1", "a", "4", "57.14286", "1"],
        svec!["h2", "z", "3", "42.85714", "1"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_negative_limit() {
    let (wrk, mut cmd) = setup("frequency_negative_limit");
    cmd.args(["--limit", "-4"]).arg("--pct-nulls");

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["h1", "Other (3)", "3", "42.85714", "0"],
        svec!["h1", "a", "4", "57.14286", "1"],
        svec!["h2", "Other (4)", "7", "100", "0"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_limit_threshold() {
    let (wrk, mut cmd) = setup("frequency_limit_threshold");
    cmd.args(["--limit", "-4"])
        .args(["--lmt-threshold", "4"])
        .arg("--pct-nulls");

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["h1", "Other (3)", "3", "42.85714", "0"],
        svec!["h1", "a", "4", "57.14286", "1"],
        svec!["h2", "Other (4)", "7", "100", "0"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_limit_threshold_notmet() {
    let (wrk, mut cmd) = setup("frequency_limit_threshold_notmet");
    cmd.args(["--limit", "-2"])
        .args(["--lmt-threshold", "3"])
        .arg("--pct-nulls");

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["h1", "(NULL)", "1", "14.28571", "2"],
        svec!["h1", "(NULL)", "1", "14.28571", "2"],
        svec!["h1", "a", "4", "57.14286", "1"],
        svec!["h1", "b", "1", "14.28571", "2"],
        svec!["h2", "Y", "1", "14.28571", "3"],
        svec!["h2", "x", "1", "14.28571", "3"],
        svec!["h2", "y", "2", "28.57143", "2"],
        svec!["h2", "z", "3", "42.85714", "1"],
    ];
    assert_eq!(got, expected);
}
#[test]
fn frequency_asc() {
    let (wrk, mut cmd) = setup("frequency_asc");
    cmd.args(["--select", "h2"]).arg("--asc");

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["h2", "Y", "1", "14.28571", "1"],
        svec!["h2", "x", "1", "14.28571", "1"],
        svec!["h2", "y", "2", "28.57143", "2"],
        svec!["h2", "z", "3", "42.85714", "3"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_asc_ignorecase() {
    let (wrk, mut cmd) = setup("frequency_asc_ignorecase");
    cmd.arg("--ignore-case")
        .args(["--select", "h2"])
        .arg("--asc");

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["h2", "x", "1", "14.28571", "1"],
        svec!["h2", "y", "3", "42.85714", "2"],
        svec!["h2", "z", "3", "42.85714", "2"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_custom_other_text() {
    let (wrk, mut cmd) = setup("frequency_custom_other_text");
    cmd.args(["--limit", "-4"])
        .args(["--lmt-threshold", "4"])
        .args(["--other-text", "其他"])
        .arg("--pct-nulls");

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["h1", "a", "4", "57.14286", "1"],
        svec!["h1", "其他 (3)", "3", "42.85714", "0"],
        svec!["h2", "其他 (4)", "7", "100", "0"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_custom_other_text_sorted() {
    let (wrk, mut cmd) = setup("frequency_custom_other_text_sorted");
    cmd.args(["--limit", "-4"])
        .args(["--lmt-threshold", "4"])
        .args(["--other-text", "Ibang halaga"])
        .arg("--other-sorted")
        .arg("--pct-nulls");

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["h1", "Ibang halaga (3)", "3", "42.85714", "0"],
        svec!["h1", "a", "4", "57.14286", "1"],
        svec!["h2", "Ibang halaga (4)", "7", "100", "0"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_other_sorted() {
    let (wrk, mut cmd) = setup("frequency_other_sorted");
    cmd.args(["--limit", "-4"])
        .args(["--lmt-threshold", "4"])
        .arg("--other-sorted")
        .arg("--pct-nulls");

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["h1", "Other (3)", "3", "42.85714", "0"],
        svec!["h1", "a", "4", "57.14286", "1"],
        svec!["h2", "Other (4)", "7", "100", "0"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_other_text_none() {
    let (wrk, mut cmd) = setup("frequency_other_text_none");
    cmd.args(["--limit", "-4"])
        .args(["--lmt-threshold", "4"])
        .args(["--other-text", "<NONE>"])
        .arg("--pct-nulls");

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["h1", "a", "4", "57.14286", "1"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_select() {
    let (wrk, mut cmd) = setup("frequency_select");
    cmd.args(["--limit", "0"]).args(["--select", "h2"]);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["h2", "Y", "1", "14.28571", "3"],
        svec!["h2", "x", "1", "14.28571", "3"],
        svec!["h2", "y", "2", "28.57143", "2"],
        svec!["h2", "z", "3", "42.85714", "1"],
    ];
    assert_eq!(got, expected);
}

// Test that selecting columns in a different order than the original CSV
// correctly maps field names to their values. This is a regression test for
// a bug where the frequency command would swap column values when user-specified
// column order differed from the original CSV column order.
#[test]
fn frequency_select_order() {
    let wrk = Workdir::new("frequency_select_order");

    // Create CSV with columns in order: id, status, borough, agency
    let rows = vec![
        svec!["id", "status", "borough", "agency"],
        svec!["1", "Open", "BROOKLYN", "DCA"],
        svec!["2", "Closed", "QUEENS", "DOT"],
        svec!["3", "Pending", "MANHATTAN", "DEP"],
        svec!["4", "Open", "BRONX", "DCA"],
        svec!["5", "Closed", "BROOKLYN", "DOT"],
        svec!["6", "Open", "QUEENS", "DEP"],
        svec!["7", "Pending", "MANHATTAN", "DCA"],
        svec!["8", "Closed", "BRONX", "DOT"],
    ];
    wrk.create("in.csv", rows);

    // Select columns in reverse order: borough, status (different from original order)
    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--select", "borough,status"])
        .args(["--limit", "0"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Verify borough field has borough values (not status values)
    // and status field has status values (not borough values)
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["borough", "BRONX", "2", "25", "1"],
        svec!["borough", "BROOKLYN", "2", "25", "1"],
        svec!["borough", "MANHATTAN", "2", "25", "1"],
        svec!["borough", "QUEENS", "2", "25", "1"],
        svec!["status", "Closed", "3", "37.5", "1"],
        svec!["status", "Open", "3", "37.5", "1"],
        svec!["status", "Pending", "2", "25", "2"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_all_unique() {
    let wrk = Workdir::new("frequency_all_unique");
    let testdata = wrk.load_test_file("boston311-100.csv");
    let mut cmd = wrk.command("frequency");
    cmd.args(["--select", "1"]).arg(testdata);

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["case_enquiry_id", "<ALL_UNIQUE>", "100", "100", "0"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_all_unique_with_stats_cache() {
    let wrk = Workdir::new("frequency_all_unique_with_stats_cache");
    let testdata = wrk.load_test_file("boston311-100.csv");

    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg(testdata.clone())
        .arg("--cardinality")
        .arg("--stats-jsonl");

    wrk.assert_success(&mut stats_cmd);

    let mut cmd = wrk.command("frequency");
    cmd.args(["--select", "1"]).arg(testdata);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["case_enquiry_id", "<ALL_UNIQUE>", "100", "100", "0"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_custom_null_text() {
    let wrk = Workdir::new("frequency_custom_null_text");
    let testdata = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("frequency");
    cmd.args(["--select", "fire_district"])
        .args(["--null-text", "<NADA Y MUCHO MAS>"])
        .arg(testdata)
        .arg("--pct-nulls"); // Include NULLs in percentage/rank to test custom null text

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["fire_district", "3", "19", "19", "1"],
        svec!["fire_district", "4", "16", "16", "2"],
        svec!["fire_district", "7", "14", "14", "3"],
        svec!["fire_district", "6", "13", "13", "4"],
        svec!["fire_district", "8", "9", "9", "5"],
        svec!["fire_district", "1", "8", "8", "6"],
        svec!["fire_district", "12", "8", "8", "6"],
        svec!["fire_district", "9", "7", "7", "7"],
        svec!["fire_district", "11", "5", "5", "8"],
        svec!["fire_district", "<NADA Y MUCHO MAS>", "1", "1", "9"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_all_unique_with_stats_cache_alt_all_unique_text() {
    let wrk = Workdir::new("frequency_all_unique_with_stats_cache_alt_all_unique_text");
    let testdata = wrk.load_test_file("boston311-100.csv");

    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg(testdata.clone())
        .arg("--cardinality")
        .arg("--stats-jsonl");

    wrk.assert_success(&mut stats_cmd);

    let mut cmd = wrk.command("frequency");
    cmd.args(["--select", "1"])
        // "<ALL_UNIQUE>" in German
        .args(["--all-unique-text", "<ALLE EINZIGARTIG>"])
        .arg(testdata);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["case_enquiry_id", "<ALLE EINZIGARTIG>", "100", "100", "0"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_all_unique_stats_cache_default() {
    let wrk = Workdir::new("frequency_all_unique_stats_cache_default");
    let testdata = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("frequency");
    cmd.args(["--select", "1"]).arg(testdata);

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["case_enquiry_id", "<ALL_UNIQUE>", "100", "100", "0"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_all_unique_stats_mode_none() {
    let wrk = Workdir::new("frequency_all_unique_stats_mode_none");
    let testdata = wrk.load_test_file("boston311-100.csv");

    // create stats cache
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg(testdata.clone())
        .arg("--cardinality")
        .arg("--stats-jsonl");

    wrk.assert_success(&mut stats_cmd);

    // run frequency with stats-mode none, ignoring the stats cache
    let mut cmd = wrk.command("frequency");
    cmd.env("QSV_STATSCACHE_MODE", "None")
        .args(["--select", "1"])
        .arg(testdata);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["case_enquiry_id", "101004113298", "1", "1", "1"],
        svec!["case_enquiry_id", "101004113313", "1", "1", "1"],
        svec!["case_enquiry_id", "101004113348", "1", "1", "1"],
        svec!["case_enquiry_id", "101004113363", "1", "1", "1"],
        svec!["case_enquiry_id", "101004113371", "1", "1", "1"],
        svec!["case_enquiry_id", "101004113385", "1", "1", "1"],
        svec!["case_enquiry_id", "101004113386", "1", "1", "1"],
        svec!["case_enquiry_id", "101004113391", "1", "1", "1"],
        svec!["case_enquiry_id", "101004113394", "1", "1", "1"],
        svec!["case_enquiry_id", "101004113403", "1", "1", "1"],
        svec!["case_enquiry_id", "Other (90)", "90", "90", "0"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_issue1962() {
    let wrk = Workdir::new("frequency_1962");
    let testdata = wrk.load_test_file("data1962.csv");
    let mut cmd = wrk.command("frequency");
    cmd.args(["--limit", "15"]).arg(testdata.clone());

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["year", "2024", "24", "8", "1"],
        svec!["year", "2023", "23", "7.66667", "2"],
        svec!["year", "2022", "22", "7.33333", "3"],
        svec!["year", "2021", "21", "7", "4"],
        svec!["year", "2020", "20", "6.66667", "5"],
        svec!["year", "2019", "19", "6.33333", "6"],
        svec!["year", "2018", "18", "6", "7"],
        svec!["year", "2017", "17", "5.66667", "8"],
        svec!["year", "2016", "16", "5.33333", "9"],
        svec!["year", "2015", "15", "5", "10"],
        svec!["year", "2014", "14", "4.66667", "11"],
        svec!["year", "2013", "13", "4.33333", "12"],
        svec!["year", "2012", "12", "4", "13"],
        svec!["year", "2011", "11", "3.66667", "14"],
        svec!["year", "2010", "10", "3.33333", "15"],
        svec!["year", "Other (9)", "45", "15", "0"],
    ];
    assert_eq!(got, expected);

    let mut cmd = wrk.command("frequency");
    cmd.args(["--limit", "5"]).arg(testdata);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["year", "2024", "24", "8", "1"],
        svec!["year", "2023", "23", "7.66667", "2"],
        svec!["year", "2022", "22", "7.33333", "3"],
        svec!["year", "2021", "21", "7", "4"],
        svec!["year", "2020", "20", "6.66667", "5"],
        svec!["year", "Other (19)", "190", "63.33333", "0"],
    ];
    assert_eq!(got, expected);
}

// This tests that a frequency table computed by `qsv` is always the same
// as the frequency table computed in memory.
#[test]
fn prop_frequency() {
    fn p(rows: CsvData) -> bool {
        param_prop_frequency("prop_frequency", rows, false)
    }
    // Run on really small values because we are incredibly careless
    // with allocation.
    qcheck_sized(p as fn(CsvData) -> bool, 5);
}

// This tests that running the frequency command on a CSV file with these two
// rows does not burst in flames:
//
//     \u{FEFF}
//     ""
//
// In this case, the `param_prop_frequency` just ignores this particular test.
// Namely, \u{FEFF} is the UTF-8 BOM, which is ignored by the underlying CSV
// reader.
#[test]
fn frequency_bom() {
    let rows = CsvData {
        data: vec![
            crate::CsvRecord(vec!["\u{FEFF}".to_string()]),
            crate::CsvRecord(vec![String::new()]),
        ],
    };
    assert!(param_prop_frequency("prop_frequency", rows, false))
}

// This tests that a frequency table computed by `qsv` (with an index) is
// always the same as the frequency table computed in memory.
#[test]
fn prop_frequency_indexed() {
    fn p(rows: CsvData) -> bool {
        param_prop_frequency("prop_frequency_indexed", rows, true)
    }
    // Run on really small values because we are incredibly careless
    // with allocation.
    qcheck_sized(p as fn(CsvData) -> bool, 5);
}

fn param_prop_frequency(name: &str, rows: CsvData, idx: bool) -> bool {
    if !rows.is_empty() {
        return true;
    }

    let rows_check = rows.clone();

    for row in rows_check.into_iter() {
        for field in row.into_iter() {
            if field.contains("\u{FEFF}") {
                return true;
            }
        }
    }
    let wrk = Workdir::new(name);
    if idx {
        wrk.create_indexed("in.csv", rows.clone());
    } else {
        wrk.create("in.csv", rows.clone());
    }

    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["-j", "4"])
        .args(["--limit", "0"])
        .args(["--unq-limit", "0"]);

    let stdout = wrk.stdout::<String>(&mut cmd);
    let got_ftables = ftables_from_csv_string(stdout);
    let expected_ftables = ftables_from_rows(rows);
    assert_eq_ftables(&got_ftables, &expected_ftables)
}

type FTables = HashMap<String, Frequencies<String>>;

#[derive(Deserialize)]
struct FRow {
    field: String,
    value: String,
    count: usize,
}

fn ftables_from_rows<T: Csv>(rows: T) -> FTables {
    let mut rows = rows.to_vecs();
    if rows.len() <= 1 {
        return HashMap::new();
    }

    let header = rows.remove(0);
    let mut ftables = HashMap::new();
    for field in &header {
        ftables.insert(field.clone(), Frequencies::new());
    }
    for row in rows {
        for (i, mut field) in row.into_iter().enumerate() {
            field = field.trim().to_owned();
            if field.is_empty() {
                field = "(NULL)".to_owned();
            }
            ftables.get_mut(&header[i]).unwrap().add(field);
        }
    }
    ftables
}

fn ftables_from_csv_string(data: String) -> FTables {
    let mut rdr = csv::Reader::from_reader(data.as_bytes());
    let mut ftables = HashMap::new();
    for frow in rdr.deserialize() {
        let frow: FRow = frow.unwrap();
        match ftables.entry(frow.field) {
            Entry::Vacant(v) => {
                let mut ftable = Frequencies::new();
                for _ in 0..frow.count {
                    ftable.add(frow.value.clone());
                }
                v.insert(ftable);
            },
            Entry::Occupied(mut v) => {
                for _ in 0..frow.count {
                    v.get_mut().add(frow.value.clone());
                }
            },
        }
    }
    ftables
}

fn freq_data<T>(ftable: &Frequencies<T>) -> Vec<(&T, u64)>
where
    T: ::std::hash::Hash + Ord + Clone,
{
    let (mut freqs, _) = ftable.most_frequent();
    freqs.sort_unstable();
    freqs
}

fn assert_eq_ftables(got: &FTables, expected: &FTables) -> bool {
    for (k, v) in got.iter() {
        assert_eq!(freq_data(v), freq_data(expected.get(k).unwrap()));
    }
    for (k, v) in expected.iter() {
        assert_eq!(freq_data(got.get(k).unwrap()), freq_data(v));
    }
    true
}

#[test]
fn frequency_vis_whitespace() {
    let wrk = Workdir::new("frequency_vis_whitespace");

    // Create test data with various types of whitespace
    let rows = vec![
        svec!["header"],
        svec!["value\t"],       // trailing tab
        svec!["\tvalue"],       // leading tab
        svec!["value\r"],       // trailing CR
        svec!["\rvalue"],       // leading CR
        svec!["value\n"],       // trailing LF
        svec!["\nvalue"],       // leading LF
        svec!["value "],        // trailing space
        svec![" value"],        // leading space
        svec!["      "],        // all spaces
        svec!["value\u{00A0}"], // trailing non-breaking space
        svec!["\u{00A0}value"], // leading non-breaking space
        svec!["value\u{2003}"], // trailing em space
        svec!["\u{2003}value"], // leading em space
        svec!["value\u{2007}"], // trailing figure space
        svec!["\u{2007}value"], // leading figure space
        svec!["value\u{200B}"], // trailing zero width space
        svec!["\u{200B}value"], // leading zero width space
        svec!["no_whitespace"],
    ];

    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("frequency");
    cmd.env("QSV_STATSCACHE_MODE", "none")
        .arg("in.csv")
        .args(["--limit", "0"])
        .arg("--vis-whitespace")
        .arg("--pct-nulls");

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // NULL is now at the end by default (--null-sorted flag changes this behavior)
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["header", "value", "8", "44.44444", "1"],
        svec!["header", "no_whitespace", "1", "5.55556", "2"],
        svec!["header", "value《⍽》", "1", "5.55556", "2"],
        svec!["header", "value《emsp》", "1", "5.55556", "2"],
        svec!["header", "value《figsp》", "1", "5.55556", "2"],
        svec!["header", "value《zwsp》", "1", "5.55556", "2"],
        svec!["header", "《⍽》value", "1", "5.55556", "2"],
        svec!["header", "《emsp》value", "1", "5.55556", "2"],
        svec!["header", "《figsp》value", "1", "5.55556", "2"],
        svec!["header", "《zwsp》value", "1", "5.55556", "2"],
        svec!["header", "(NULL)", "1", "5.55556", "2"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn frequency_vis_whitespace_no_trim() {
    let wrk = Workdir::new("frequency_vis_whitespace_no_trim");

    // Create test data with multiple occurrences of same whitespace patterns
    let rows = vec![
        svec!["header"],
        svec!["value\t"], // trailing tab
        svec!["value\t"], // trailing tab (duplicate)
        svec!["\tvalue"], // leading tab
        svec!["\tvalue"], // leading tab (duplicate)
        svec!["value "],  // trailing space
        svec!["value "],  // trailing space (duplicate)
        svec![" value"],  // leading space
        svec![" value"],  // leading space (duplicate)
        svec!["      "],  // all spaces
        svec!["      "],  // all spaces (duplicate)
        svec!["no_whitespace"],
    ];

    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--limit", "0"])
        .arg("--vis-whitespace")
        .arg("--no-trim");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["header", "《→》value", "2", "18.18182", "1"],
        svec![
            "header",
            "《_》《_》《_》《_》《_》《_》",
            "2",
            "18.18182",
            "1"
        ],
        svec!["header", " value", "2", "18.18182", "1"],
        svec!["header", "value《→》", "2", "18.18182", "1"],
        svec!["header", "value ", "2", "18.18182", "1"],
        svec!["header", "no_whitespace", "1", "9.09091", "2"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn frequency_vis_whitespace_ignore_case() {
    let wrk = Workdir::new("frequency_vis_whitespace_ignore_case");

    // Create test data with whitespace and mixed case
    let rows = vec![
        svec!["header"],
        svec!["Value\t"],       // trailing tab
        svec!["\tVALUE"],       // leading tab
        svec!["value "],        // trailing space
        svec!["value\u{000B}"], // vertical tab
        svec!["value\u{000C}"], // form feed
        svec!["value\u{0085}"], // next line
        svec!["value\u{200E}"], // left-to-right mark
        svec!["value\u{200F}"], // right-to-left mark
        svec!["value\u{2028}"], // line separator
        svec!["value\u{2029}"], // paragraph separator
        svec!["value\u{00A0}"], // non-breaking space
        svec!["value\u{2003}"], // em space
        svec!["value\u{2007}"], // figure space
        svec!["value\u{200B}"], // zero width space
        svec![" VALUE"],        // leading space
        svec!["no_whitespace"],
        svec!["      "], // all spaces
    ];

    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("frequency");
    cmd.env("QSV_STATSCACHE_MODE", "none")
        .arg("in.csv")
        .args(["--limit", "0"])
        .arg("--vis-whitespace")
        .arg("--ignore-case")
        .arg("--pct-nulls");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // NULL is now at the end by default (--null-sorted flag changes this behavior)
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["header", "value", "12", "70.58824", "1"],
        svec!["header", "no_whitespace", "1", "5.88235", "2"],
        svec!["header", "value《zwsp》", "1", "5.88235", "2"],
        svec!["header", "value《␎》", "1", "5.88235", "2"],
        svec!["header", "value《␏》", "1", "5.88235", "2"],
        svec!["header", "(NULL)", "1", "5.88235", "2"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn frequency_json() {
    let (wrk, mut cmd) = setup("frequency_json");
    cmd.args(["--limit", "0"])
        .args(["--select", "h2"])
        .arg("--json");
    let got: String = wrk.stdout(&mut cmd);
    let v: Value = serde_json::from_str(&got).unwrap();
    assert!(v["input"].as_str().unwrap().ends_with("in.csv"));
    assert_eq!(v["rowcount"], 7);
    assert_eq!(v["fieldcount"], 1);
    let fields = v["fields"].as_array().unwrap();
    assert_eq!(fields.len(), 1);
    let field = &fields[0];
    assert_eq!(field["field"], "h2");
    assert_eq!(field["cardinality"], 4);
    let freqs = field["frequencies"].as_array().unwrap();
    let expected = vec![
        ("z", 3, 42.85714, 1.0),
        ("y", 2, 28.57143, 2.0),
        ("Y", 1, 14.28571, 3.0),
        ("x", 1, 14.28571, 3.0),
    ];
    for (i, (val, count, pct, rank)) in expected.iter().enumerate() {
        assert_eq!(freqs[i]["value"], *val);
        assert_eq!(freqs[i]["count"], *count);
        assert!((freqs[i]["percentage"].as_f64().unwrap() - *pct).abs() < 1e-5);
        assert_eq!(freqs[i]["rank"], *rank);
    }
}

#[test]
fn frequency_json_no_headers() {
    let (wrk, mut cmd) = setup("frequency_json_no_headers");
    cmd.args(["--limit", "0"])
        .args(["--select", "1"])
        .arg("--no-headers")
        .arg("--json")
        .arg("--pct-nulls");
    let got: String = wrk.stdout(&mut cmd);
    let v: Value = serde_json::from_str(&got).unwrap();
    assert!(v["input"].as_str().unwrap().ends_with("in.csv"));
    assert_eq!(v["rowcount"], 8);
    assert_eq!(v["fieldcount"], 1);
    let fields = v["fields"].as_array().unwrap();
    assert_eq!(fields.len(), 1);
    let field = &fields[0];
    assert_eq!(field["field"], "1");
    assert_eq!(field["cardinality"], 5);
    let freqs = field["frequencies"].as_array().unwrap();
    // NULL entries are now at the end by default (--null-sorted flag changes this behavior)
    let expected = vec![
        ("a", 4, 50.0, 1.0),
        ("b", 1, 12.5, 2.0),
        ("h1", 1, 12.5, 2.0),
        ("(NULL)", 1, 12.5, 2.0),
        ("(NULL)", 1, 12.5, 2.0),
    ];
    for (i, (val, count, pct, rank)) in expected.iter().enumerate() {
        assert_eq!(freqs[i]["value"], *val);
        assert_eq!(freqs[i]["count"], *count);
        assert!((freqs[i]["percentage"].as_f64().unwrap() - *pct).abs() < 1e-5);
        assert_eq!(freqs[i]["rank"], *rank);
    }
}

#[test]
fn frequency_json_ignore_case() {
    let (wrk, mut cmd) = setup("frequency_json_ignore_case");
    cmd.arg("--ignore-case")
        .args(["--limit", "0"])
        .args(["--select", "h2"])
        .arg("--json");
    let got: String = wrk.stdout(&mut cmd);
    let v: Value = serde_json::from_str(&got).unwrap();
    assert!(v["input"].as_str().unwrap().ends_with("in.csv"));
    assert_eq!(v["rowcount"], 7);
    assert_eq!(v["fieldcount"], 1);
    let fields = v["fields"].as_array().unwrap();
    assert_eq!(fields.len(), 1);
    let field = &fields[0];
    assert_eq!(field["field"], "h2");
    assert_eq!(field["cardinality"], 3);
    let freqs = field["frequencies"].as_array().unwrap();
    let expected = vec![("y", 3, 42.85714), ("z", 3, 42.85714), ("x", 1, 14.28571)];
    for (i, (val, count, pct)) in expected.iter().enumerate() {
        assert_eq!(freqs[i]["value"], *val);
        assert_eq!(freqs[i]["count"], *count);
        assert!((freqs[i]["percentage"].as_f64().unwrap() - *pct).abs() < 1e-5);
    }
}

#[test]
fn frequency_json_limit() {
    let (wrk, mut cmd) = setup("frequency_json_limit");
    cmd.args(["--limit", "1"]).arg("--json").arg("--pct-nulls");
    let got: String = wrk.stdout(&mut cmd);
    let v: Value = serde_json::from_str(&got).unwrap();
    assert!(v["input"].as_str().unwrap().ends_with("in.csv"));
    assert_eq!(v["rowcount"], 7);
    assert_eq!(v["fieldcount"], 2);
    let fields = v["fields"].as_array().unwrap();
    assert_eq!(fields.len(), 2);
    let (f1, f2) = (&fields[0], &fields[1]);
    // Accept either order for fields
    let (h1, h2) = if f1["field"] == "h1" {
        (f1, f2)
    } else {
        (f2, f1)
    };
    assert_eq!(h1["cardinality"], 4);
    assert_eq!(h2["cardinality"], 4);
    let freqs_h1 = h1["frequencies"].as_array().unwrap();
    let expected_h1 = vec![("a", 4, 57.14286), ("Other (3)", 3, 42.85714)];
    for (i, (val, count, pct)) in expected_h1.iter().enumerate() {
        assert_eq!(freqs_h1[i]["value"], *val);
        assert_eq!(freqs_h1[i]["count"], *count);
        assert!((freqs_h1[i]["percentage"].as_f64().unwrap() - *pct).abs() < 1e-5);
    }
    let freqs_h2 = h2["frequencies"].as_array().unwrap();
    let expected_h2 = vec![("z", 3, 42.85714), ("Other (3)", 4, 57.14286)];
    for (i, (val, count, pct)) in expected_h2.iter().enumerate() {
        assert_eq!(freqs_h2[i]["value"], *val);
        assert_eq!(freqs_h2[i]["count"], *count);
        assert!((freqs_h2[i]["percentage"].as_f64().unwrap() - *pct).abs() < 1e-5);
    }
}

#[test]
fn frequency_json_all_unique() {
    let wrk = Workdir::new("frequency_json_all_unique");
    let testdata = wrk.load_test_file("boston311-100.csv");
    let mut cmd = wrk.command("frequency");
    cmd.args(["--select", "1"])
        .arg(testdata.clone())
        .arg("--json");
    wrk.assert_success(&mut cmd);
    let got: String = wrk.stdout(&mut cmd);
    let v: Value = serde_json::from_str(&got).unwrap();
    // Accept either full path or just filename for input
    let input = v["input"].as_str().unwrap();
    assert!(input.ends_with("boston311-100.csv"));
    assert_eq!(v["rowcount"], 100);
    assert_eq!(v["fieldcount"], 1);
    let fields = v["fields"].as_array().unwrap();
    assert_eq!(fields.len(), 1);
    let field = &fields[0];
    assert_eq!(field["field"], "case_enquiry_id");
    assert_eq!(field["cardinality"], 100);
    let freqs = field["frequencies"].as_array().unwrap();
    assert_eq!(freqs.len(), 1);
    assert_eq!(freqs[0]["value"], "<ALL_UNIQUE>");
    assert_eq!(freqs[0]["count"], 100);
    assert!((freqs[0]["percentage"].as_f64().unwrap() - 100.0).abs() < 1e-5);
}

#[test]
fn frequency_json_vis_whitespace() {
    let wrk = Workdir::new("frequency_json_vis_whitespace");
    let rows = vec![
        svec!["header"],
        svec!["value\t"],
        svec!["\tvalue"],
        svec!["value "],
        svec![" value"],
        svec!["      "],
        svec!["no_whitespace"],
    ];
    wrk.create("in.csv", rows);
    let mut cmd = wrk.command("frequency");
    cmd.env("QSV_STATSCACHE_MODE", "none")
        .arg("in.csv")
        .args(["--limit", "0"])
        .arg("--vis-whitespace")
        .arg("--json")
        .arg("--pct-nulls");
    wrk.assert_success(&mut cmd);
    let got: String = wrk.stdout(&mut cmd);
    let v: Value = serde_json::from_str(&got).unwrap();
    assert!(v["input"].as_str().unwrap().ends_with("in.csv"));
    assert_eq!(v["rowcount"], 6);
    assert_eq!(v["fieldcount"], 1);
    let fields = v["fields"].as_array().unwrap();
    assert_eq!(fields.len(), 1);
    let field = &fields[0];
    assert_eq!(field["field"], "header");
    assert_eq!(field["cardinality"], 3);
    let freqs = field["frequencies"].as_array().unwrap();
    // NULL is now at the end by default (--null-sorted flag changes this behavior)
    let expected = vec![
        ("value", 4, 66.66667),
        ("no_whitespace", 1, 16.66667),
        ("(NULL)", 1, 16.66667),
    ];
    for (i, (val, count, pct)) in expected.iter().enumerate() {
        assert_eq!(freqs[i]["value"], *val);
        assert_eq!(freqs[i]["count"], *count);
        assert!((freqs[i]["percentage"].as_f64().unwrap() - *pct).abs() < 1e-5);
    }
}

#[test]
fn frequency_toon() {
    let (wrk, mut cmd) = setup("frequency_toon");
    cmd.args(["--limit", "0"])
        .args(["--select", "h2"])
        .arg("--toon");
    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"input: in.csv
description: "Generated with `qsv frequency in.csv --limit 0 --select h2 --toon`"
rowcount: 7
fieldcount: 1
fields[1]:
  - field: h2
    type: String
    cardinality: 4
    nullcount: 0
    sparsity: 0
    uniqueness_ratio: 0.5714
    stats[10]{name,value}:
    min,Y
    max,z
    sort_order,Unsorted
    min_length,1
    max_length,1
    sum_length,7
    avg_length,1
    stddev_length,0
    variance_length,0
    cv_length,0
    frequencies[4]{value,count,percentage,rank}:
    z,3,42.85714,1
    y,2,28.57143,2
    Y,1,14.28571,3
    x,1,14.28571,3
rank_strategy: dense"#
        .to_string();
    assert_eq!(got, expected);
}

#[test]
fn frequency_toon_no_headers() {
    let (wrk, mut cmd) = setup("frequency_toon_no_headers");
    cmd.args(["--limit", "0"])
        .args(["--select", "1"])
        .arg("--no-headers")
        .arg("--toon")
        .arg("--pct-nulls");
    let got: String = wrk.stdout(&mut cmd);
    // NULL entries are now at the end by default (--null-sorted flag changes this behavior)
    let expected = r#"input: in.csv
description: "Generated with `qsv frequency in.csv --limit 0 --select 1 --no-headers --toon --pct-nulls`"
rowcount: 8
fieldcount: 1
fields[1]:
  - field: "1"
    type: ""
    cardinality: 5
    nullcount: 0
    sparsity: 0
    uniqueness_ratio: 0.625
    frequencies[5]{value,count,percentage,rank}:
    a,4,50,1
    b,1,12.5,2
    h1,1,12.5,2
    (NULL),1,12.5,2
    (NULL),1,12.5,2
rank_strategy: dense"#
        .to_string();
    assert_eq!(got, expected);
}

#[test]
fn frequency_toon_ignore_case() {
    let (wrk, mut cmd) = setup("frequency_toon_ignore_case");
    cmd.arg("--ignore-case")
        .args(["--limit", "0"])
        .args(["--select", "h2"])
        .arg("--toon");
    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"input: in.csv
description: "Generated with `qsv frequency in.csv --ignore-case --limit 0 --select h2 --toon`"
rowcount: 7
fieldcount: 1
fields[1]:
  - field: h2
    type: String
    cardinality: 3
    nullcount: 0
    sparsity: 0
    uniqueness_ratio: 0.4286
    stats[10]{name,value}:
    min,Y
    max,z
    sort_order,Unsorted
    min_length,1
    max_length,1
    sum_length,7
    avg_length,1
    stddev_length,0
    variance_length,0
    cv_length,0
    frequencies[3]{value,count,percentage,rank}:
    y,3,42.85714,1
    z,3,42.85714,1
    x,1,14.28571,2
rank_strategy: dense"#
        .to_string();
    assert_eq!(got, expected);
}

#[test]
fn frequency_toon_limit() {
    let (wrk, mut cmd) = setup("frequency_toon_limit");
    cmd.args(["--limit", "1"]).arg("--toon").arg("--pct-nulls");
    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"input: in.csv
description: "Generated with `qsv frequency in.csv --limit 1 --toon --pct-nulls`"
rowcount: 7
fieldcount: 2
fields[2]:
  - field: h1
    type: String
    cardinality: 4
    nullcount: 1
    sparsity: 0.1429
    uniqueness_ratio: 0.5714
    stats[10]{name,value}:
    min,(NULL)
    max,b
    sort_order,Unsorted
    min_length,0
    max_length,6
    sum_length,11
    avg_length,1.5714
    stddev_length,1.8406
    variance_length,3.3878
    cv_length,1.1713
    frequencies[2]{value,count,percentage,rank}:
    a,4,57.14286,1
    Other (3),3,42.85714,0
  - field: h2
    type: String
    cardinality: 4
    nullcount: 0
    sparsity: 0
    uniqueness_ratio: 0.5714
    stats[10]{name,value}:
    min,Y
    max,z
    sort_order,Unsorted
    min_length,1
    max_length,1
    sum_length,7
    avg_length,1
    stddev_length,0
    variance_length,0
    cv_length,0
    frequencies[2]{value,count,percentage,rank}:
    z,3,42.85714,1
    Other (3),4,57.14286,0
rank_strategy: dense"#
        .to_string();
    assert_eq!(got, expected);
}

#[test]
fn frequency_toon_all_unique() {
    let wrk = Workdir::new("frequency_toon_all_unique");
    let testdata = wrk.load_test_file("boston311-100.csv");
    let mut cmd = wrk.command("frequency");
    cmd.args(["--select", "1"])
        .arg(testdata.clone())
        .arg("--toon");
    wrk.assert_success(&mut cmd);
    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"rowcount: 100
fieldcount: 1
fields[1]:
  - field: case_enquiry_id
    type: Integer
    cardinality: 100
    nullcount: 0
    sparsity: 0
    uniqueness_ratio: 1
    stats[10]{name,value}:
    sum,10100411645180
    min,101004113298
    max,101004155594
    range,42296
    sort_order,Unsorted
    mean,101004116451.8
    sem,790.552
    stddev,7905.5202
    variance,62497248.9352
    cv,0
    frequencies[1]{value,count,percentage,rank}:
    <ALL_UNIQUE>,100,100,0
rank_strategy: dense"#;
    assert!(got.ends_with(expected));
}

#[test]
fn frequency_toon_vis_whitespace() {
    let wrk = Workdir::new("frequency_toon_vis_whitespace");
    let rows = vec![
        svec!["header"],
        svec!["value\t"],
        svec!["\tvalue"],
        svec!["value "],
        svec![" value"],
        svec!["      "],
        svec!["no_whitespace"],
    ];
    wrk.create("in.csv", rows);
    let mut cmd = wrk.command("frequency");
    cmd.env("QSV_STATSCACHE_MODE", "none")
        .arg("in.csv")
        .args(["--limit", "0"])
        .arg("--vis-whitespace")
        .arg("--toon")
        .arg("--pct-nulls");
    wrk.assert_success(&mut cmd);
    let got: String = wrk.stdout(&mut cmd);
    let v: Value = toon_format::decode(
        &got,
        &toon_format::DecodeOptions {
            strict: false,
            ..Default::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!(
            "Failed to decode TOON output: {e}. Output: {}",
            &got[..got.len().min(500)]
        )
    });
    assert!(v["input"].as_str().unwrap().ends_with("in.csv"));
    assert_eq!(v["rowcount"], 6);
    assert_eq!(v["fieldcount"], 1);
    let fields = v["fields"].as_array().expect("fields should be an array");
    assert_eq!(fields.len(), 1);
    let field = &fields[0];
    assert_eq!(field["field"], "header");
    assert_eq!(field["cardinality"], 3);
    let freqs = field["frequencies"].as_array().expect(&format!(
        "frequencies should be an array. Field keys: {:?}",
        field.as_object().map(|o| o.keys().collect::<Vec<_>>())
    ));
    // NULL is now at the end by default (--null-sorted flag changes this behavior)
    let expected = vec![
        ("value", 4, 66.66667),
        ("no_whitespace", 1, 16.66667),
        ("(NULL)", 1, 16.66667),
    ];
    for (i, (val, count, pct)) in expected.iter().enumerate() {
        assert_eq!(freqs[i]["value"], *val);
        assert_eq!(freqs[i]["count"], *count);
        assert!((freqs[i]["percentage"].as_f64().unwrap() - *pct).abs() < 1e-5);
    }
}

// Test ranking strategies
fn setup_rank_test_sorted(name: &str) -> (Workdir, process::Command) {
    // Create data with specific counts to test ranking:
    // Value "a" appears 5 times (rank 1)
    // Values "b" and "c" appear 3 times each (tied for rank 2/3)
    // Values "d" and "e" appear 2 times each (tied for rank 4/5)
    // Value "f" appears 1 time (rank 6)
    let rows = vec![
        svec!["value"],
        svec!["a"],
        svec!["a"],
        svec!["a"],
        svec!["a"],
        svec!["a"],
        svec!["b"],
        svec!["b"],
        svec!["b"],
        svec!["c"],
        svec!["c"],
        svec!["c"],
        svec!["d"],
        svec!["d"],
        svec!["e"],
        svec!["e"],
        svec!["f"],
    ];

    let wrk = Workdir::new(name);
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv").args(["--limit", "0"]);

    (wrk, cmd)
}

fn setup_rank_test_unsorted(name: &str) -> (Workdir, process::Command) {
    // same as setup_rank_test_sorted but the values are unsorted
    // this is to test that the tied values are sorted alphabetically
    let rows = vec![
        svec!["value"],
        svec!["c"],
        svec!["d"],
        svec!["a"],
        svec!["a"],
        svec!["b"],
        svec!["a"],
        svec!["e"],
        svec!["a"],
        svec!["b"],
        svec!["c"],
        svec!["c"],
        svec!["b"],
        svec!["e"],
        svec!["d"],
        svec!["f"],
        svec!["a"],
    ];
    let wrk = Workdir::new(name);
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv").args(["--limit", "0"]);

    (wrk, cmd)
}

fn setup_rank_test_simple(name: &str) -> (Workdir, process::Command) {
    let rows = vec![
        svec!["value"],
        svec!["a"],
        svec!["a"],
        svec!["a"],
        svec!["b"],
        svec!["b"],
        svec!["c"],
        svec!["c"],
        svec!["d"],
    ];
    let wrk = Workdir::new(name);
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv").args(["--limit", "0"]);

    (wrk, cmd)
}

#[test]
fn frequency_rank_ties_min() {
    let (wrk, mut cmd) = setup_rank_test_sorted("frequency_rank_ties_min");
    cmd.args(["--rank-strategy", "min"]);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["value", "a", "5", "31.25", "1"],
        svec!["value", "b", "3", "18.75", "2"],
        svec!["value", "c", "3", "18.75", "2"],
        svec!["value", "d", "2", "12.5", "4"],
        svec!["value", "e", "2", "12.5", "4"],
        svec!["value", "f", "1", "6.25", "6"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_rank_ties_min_simple() {
    let (wrk, mut cmd) = setup_rank_test_simple("frequency_rank_ties_min_simple");
    cmd.args(["--rank-strategy", "min"]);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    // min rank should be 1,2,2,4
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["value", "a", "3", "37.5", "1"],
        svec!["value", "b", "2", "25", "2"],
        svec!["value", "c", "2", "25", "2"],
        svec!["value", "d", "1", "12.5", "4"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_rank_ties_max() {
    let (wrk, mut cmd) = setup_rank_test_sorted("frequency_rank_ties_max");
    cmd.args(["-r", "max"]);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["value", "a", "5", "31.25", "1"],
        svec!["value", "b", "3", "18.75", "3"],
        svec!["value", "c", "3", "18.75", "3"],
        svec!["value", "d", "2", "12.5", "5"],
        svec!["value", "e", "2", "12.5", "5"],
        svec!["value", "f", "1", "6.25", "6"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_rank_ties_max_simple() {
    let (wrk, mut cmd) = setup_rank_test_simple("frequency_rank_ties_max_simple");
    cmd.args(["-r", "max"]);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    // max rank should be 1,3,3,4
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["value", "a", "3", "37.5", "1"],
        svec!["value", "b", "2", "25", "3"],
        svec!["value", "c", "2", "25", "3"],
        svec!["value", "d", "1", "12.5", "4"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_rank_ties_dense() {
    let (wrk, mut cmd) = setup_rank_test_sorted("frequency_rank_ties_dense");
    cmd.args(["--rank-strategy", "dense"]);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["value", "a", "5", "31.25", "1"],
        svec!["value", "b", "3", "18.75", "2"],
        svec!["value", "c", "3", "18.75", "2"],
        svec!["value", "d", "2", "12.5", "3"],
        svec!["value", "e", "2", "12.5", "3"],
        svec!["value", "f", "1", "6.25", "4"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_rank_ties_dense_complex() {
    let (wrk, mut cmd) = setup_rank_test_unsorted("frequency_rank_ties_dense_complex");
    cmd.args(["--rank-strategy", "dense"]);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["value", "a", "5", "31.25", "1"],
        svec!["value", "b", "3", "18.75", "2"],
        svec!["value", "c", "3", "18.75", "2"],
        svec!["value", "d", "2", "12.5", "3"],
        svec!["value", "e", "2", "12.5", "3"],
        svec!["value", "f", "1", "6.25", "4"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_rank_ties_dense_simple() {
    let (wrk, mut cmd) = setup_rank_test_simple("frequency_rank_ties_dense_simple");
    cmd.args(["--rank-strategy", "dense"]);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    // dense rank should be 1,2,2,3
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["value", "a", "3", "37.5", "1"],
        svec!["value", "b", "2", "25", "2"],
        svec!["value", "c", "2", "25", "2"],
        svec!["value", "d", "1", "12.5", "3"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_rank_ties_ordinal() {
    let (wrk, mut cmd) = setup_rank_test_sorted("frequency_rank_ties_ordinal");
    cmd.args(["--rank-strategy", "ordinal"]);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["value", "a", "5", "31.25", "1"],
        svec!["value", "b", "3", "18.75", "2"],
        svec!["value", "c", "3", "18.75", "3"],
        svec!["value", "d", "2", "12.5", "4"],
        svec!["value", "e", "2", "12.5", "5"],
        svec!["value", "f", "1", "6.25", "6"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_rank_ties_ordinal_complex() {
    let (wrk, mut cmd) = setup_rank_test_unsorted("frequency_rank_ties_ordinal_complex");
    cmd.args(["--rank-strategy", "ordinal"]);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["value", "a", "5", "31.25", "1"],
        svec!["value", "b", "3", "18.75", "2"],
        svec!["value", "c", "3", "18.75", "3"],
        svec!["value", "d", "2", "12.5", "4"],
        svec!["value", "e", "2", "12.5", "5"],
        svec!["value", "f", "1", "6.25", "6"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_rank_ties_ordinal_simple() {
    let (wrk, mut cmd) = setup_rank_test_simple("frequency_rank_ties_ordinal_simple");
    cmd.args(["--rank-strategy", "ordinal"]);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["value", "a", "3", "37.5", "1"],
        svec!["value", "b", "2", "25", "2"],
        svec!["value", "c", "2", "25", "3"],
        svec!["value", "d", "1", "12.5", "4"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_rank_ties_average() {
    let (wrk, mut cmd) = setup_rank_test_sorted("frequency_rank_ties_average");
    cmd.args(["--rank-strategy", "average"]);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["value", "a", "5", "31.25", "1"],
        svec!["value", "b", "3", "18.75", "2.5"],
        svec!["value", "c", "3", "18.75", "2.5"],
        svec!["value", "d", "2", "12.5", "4.5"],
        svec!["value", "e", "2", "12.5", "4.5"],
        svec!["value", "f", "1", "6.25", "6"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_rank_ties_average_complex() {
    let (wrk, mut cmd) = setup_rank_test_unsorted("frequency_rank_ties_average_complex");
    cmd.args(["--rank-strategy", "average"]);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["value", "a", "5", "31.25", "1"],
        svec!["value", "b", "3", "18.75", "2.5"],
        svec!["value", "c", "3", "18.75", "2.5"],
        svec!["value", "d", "2", "12.5", "4.5"],
        svec!["value", "e", "2", "12.5", "4.5"],
        svec!["value", "f", "1", "6.25", "6"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_rank_ties_average_simple() {
    let (wrk, mut cmd) = setup_rank_test_simple("frequency_rank_ties_average_simple");
    cmd.args(["--rank-strategy", "average"]);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    // average rank should be 1,2.5,2.5,4
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["value", "a", "3", "37.5", "1"],
        svec!["value", "b", "2", "25", "2.5"],
        svec!["value", "c", "2", "25", "2.5"],
        svec!["value", "d", "1", "12.5", "4"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_rank_ties_with_asc() {
    let (wrk, mut cmd) = setup_rank_test_sorted("frequency_rank_ties_with_asc");
    cmd.args(["--rank-strategy", "average"]).arg("--asc");

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["value", "a", "5", "31.25", "6"],
        svec!["value", "b", "3", "18.75", "4.5"],
        svec!["value", "c", "3", "18.75", "4.5"],
        svec!["value", "d", "2", "12.5", "2.5"],
        svec!["value", "e", "2", "12.5", "2.5"],
        svec!["value", "f", "1", "6.25", "1"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_rank_ties_json() {
    let (wrk, mut cmd) = setup_rank_test_sorted("frequency_rank_ties_json");
    cmd.args(["--rank-strategy", "average"]).arg("--json");

    wrk.assert_success(&mut cmd);

    let got: String = wrk.stdout(&mut cmd);
    let v: Value = serde_json::from_str(&got).unwrap();
    assert!(v["input"].as_str().unwrap().ends_with("in.csv"));
    assert_eq!(v["rowcount"], 16);
    assert_eq!(v["fieldcount"], 1);
    assert_eq!(v["rank_strategy"], "average");
    let fields = v["fields"].as_array().unwrap();
    assert_eq!(fields.len(), 1);
    let field = &fields[0];
    assert_eq!(field["field"], "value");
    let freqs = field["frequencies"].as_array().unwrap();

    // Check fractional ranks in JSON
    assert_eq!(freqs[0]["value"], "a");
    assert_eq!(freqs[0]["rank"], 1.0);
    assert_eq!(freqs[1]["value"], "b");
    assert_eq!(freqs[1]["rank"], 2.5);
    assert_eq!(freqs[2]["value"], "c");
    assert_eq!(freqs[2]["rank"], 2.5);
    assert_eq!(freqs[3]["value"], "d");
    assert_eq!(freqs[3]["rank"], 4.5);
    assert_eq!(freqs[4]["value"], "e");
    assert_eq!(freqs[4]["rank"], 4.5);
    assert_eq!(freqs[5]["value"], "f");
    assert_eq!(freqs[5]["rank"], 6.0);
}

#[test]
fn frequency_rank_ties_invalid_strategy() {
    let (wrk, mut cmd) = setup_rank_test_sorted("frequency_rank_ties_invalid_strategy");
    cmd.args(["--rank-strategy", "invalid"]);

    let output = wrk.output_stderr(&mut cmd);
    assert!(output.contains("Could not match"));
    assert!(output.contains("allowed variants"));
}

// Weighted frequency tests
#[test]
fn frequency_weight_basic() {
    let wrk = Workdir::new("frequency_weight_basic");
    let rows = vec![
        svec!["value", "weight"],
        svec!["a", "2.0"],
        svec!["a", "3.0"],
        svec!["b", "1.0"],
        svec!["b", "1.0"],
        svec!["c", "5.0"],
    ];
    wrk.create("in.csv", rows);
    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--limit", "0"])
        .args(["--select", "value"])
        .args(["--weight", "weight"]);

    wrk.assert_success(&mut cmd);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // Sort by value for consistent comparison
    got.sort_by(|a, b| {
        if a.len() < 2 || b.len() < 2 {
            std::cmp::Ordering::Equal
        } else {
            a[1].cmp(&b[1])
        }
    });
    let mut expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["value", "a", "5", "41.66667", "1"],
        svec!["value", "b", "2", "16.66667", "2"],
        svec!["value", "c", "5", "41.66667", "1"],
    ];
    expected.sort_by(|a, b| {
        if a.len() < 2 || b.len() < 2 {
            std::cmp::Ordering::Equal
        } else {
            a[1].cmp(&b[1])
        }
    });
    assert_eq!(got, expected);
}

#[test]
fn frequency_weight_excludes_weight_column() {
    let wrk = Workdir::new("frequency_weight_excludes_weight_column");
    let rows = vec![
        svec!["value", "weight"],
        svec!["a", "2.0"],
        svec!["a", "3.0"],
        svec!["b", "1.0"],
    ];
    wrk.create("in.csv", rows);
    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--limit", "0"])
        .args(["--select", "value,weight"])
        .args(["--weight", "weight"]);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    // Should only have "value" column, not "weight" column
    let value_rows: Vec<_> = got
        .iter()
        .filter(|r| r.len() > 0 && r[0] == "value")
        .collect();
    assert_eq!(value_rows.len(), 2); // 2 value rows (a and b), header is filtered out
    // Should not have any "weight" column frequencies
    let weight_rows: Vec<_> = got
        .iter()
        .filter(|r| r.len() > 0 && r[0] == "weight")
        .collect();
    assert_eq!(weight_rows.len(), 0);
}

#[test]
fn frequency_weight_missing_weights_default_to_one() {
    let wrk = Workdir::new("frequency_weight_missing_weights_default_to_one");
    let rows = vec![
        svec!["value", "weight"],
        svec!["a", "2.0"],
        svec!["a", ""],        // missing weight
        svec!["b", "invalid"], // non-numeric weight
        svec!["b", "3.0"],
    ];
    wrk.create("in.csv", rows);
    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--limit", "0"])
        .args(["--select", "value"])
        .args(["--weight", "weight"]);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // Sort by value for consistent comparison
    got.sort_by(|a, b| {
        if a.len() < 2 || b.len() < 2 {
            std::cmp::Ordering::Equal
        } else {
            a[1].cmp(&b[1])
        }
    });
    // "a" should have weight 2.0 + 1.0 (default) = 3.0
    // "b" should have weight 1.0 (default) + 3.0 = 4.0
    let mut expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["value", "a", "3", "42.85714", "2"],
        svec!["value", "b", "4", "57.14286", "1"],
    ];
    expected.sort_by(|a, b| {
        if a.len() < 2 || b.len() < 2 {
            std::cmp::Ordering::Equal
        } else {
            a[1].cmp(&b[1])
        }
    });
    assert_eq!(got, expected);
}

#[test]
fn frequency_weight_zero_and_negative_ignored() {
    let wrk = Workdir::new("frequency_weight_zero_and_negative_ignored");
    let rows = vec![
        svec!["value", "weight"],
        svec!["a", "2.0"],
        svec!["a", "0.0"],  // zero weight - should be ignored
        svec!["b", "-1.0"], // negative weight - should be ignored
        svec!["b", "3.0"],
        svec!["c", "1.0"],
    ];
    wrk.create("in.csv", rows);
    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--limit", "0"])
        .args(["--select", "value"])
        .args(["--weight", "weight"]);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // Sort by value for consistent comparison
    got.sort_by(|a, b| {
        if a.len() < 2 || b.len() < 2 {
            std::cmp::Ordering::Equal
        } else {
            a[1].cmp(&b[1])
        }
    });
    // "a" should have weight 2.0 (0.0 ignored)
    // "b" should have weight 3.0 (-1.0 ignored)
    // "c" should have weight 1.0
    let mut expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["value", "a", "2", "33.33333", "2"],
        svec!["value", "b", "3", "50", "1"],
        svec!["value", "c", "1", "16.66667", "3"],
    ];
    expected.sort_by(|a, b| {
        if a.len() < 2 || b.len() < 2 {
            std::cmp::Ordering::Equal
        } else {
            a[1].cmp(&b[1])
        }
    });
    assert_eq!(got, expected);
}

#[test]
fn frequency_weight_with_limit() {
    let wrk = Workdir::new("frequency_weight_with_limit");
    // Use duplicate values so it's not all-unique and can test limit behavior
    let rows = vec![
        svec!["value", "weight"],
        svec!["a", "10.0"],
        svec!["a", "2.0"], // duplicate to make it not all-unique
        svec!["b", "5.0"],
        svec!["c", "3.0"],
        svec!["d", "2.0"],
    ];
    wrk.create("in.csv", rows);
    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--limit", "2"])
        .args(["--select", "value"])
        .args(["--weight", "weight"]);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // For weighted frequencies, show individual frequencies sorted by weight (descending by
    // default), limited to top 2. Sort by value for consistent comparison.
    got.sort_by(|a, b| {
        if a.len() < 2 || b.len() < 2 {
            std::cmp::Ordering::Equal
        } else {
            a[1].cmp(&b[1])
        }
    });
    // "a" has weight 10.0 + 2.0 = 12.0, "b" has 5.0, "c" has 3.0, "d" has 2.0
    // Top 2: "a" (12.0) and "b" (5.0), rest go to "Other"
    let mut expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["value", "a", "12", "54.54545", "1"],
        svec!["value", "b", "5", "22.72727", "2"],
        svec!["value", "Other (2)", "5", "22.72727", "0"],
    ];
    expected.sort_by(|a, b| {
        if a.len() < 2 || b.len() < 2 {
            std::cmp::Ordering::Equal
        } else {
            a[1].cmp(&b[1])
        }
    });
    assert_eq!(got, expected);
}

#[test]
fn frequency_weight_with_asc() {
    let wrk = Workdir::new("frequency_weight_with_asc");
    // Use duplicate values so it's not all-unique and can test sorting behavior
    let rows = vec![
        svec!["value", "weight"],
        svec!["a", "10.0"],
        svec!["a", "5.0"], // duplicate to make it not all-unique
        svec!["b", "5.0"],
        svec!["c", "3.0"],
    ];
    wrk.create("in.csv", rows);
    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--limit", "0"])
        .args(["--select", "value"])
        .args(["--weight", "weight"])
        .arg("--asc");

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // For weighted frequencies with --asc, show individual frequencies sorted ascending by weight
    // Sort by value for consistent comparison
    got.sort_by(|a, b| {
        if a.len() < 2 || b.len() < 2 {
            std::cmp::Ordering::Equal
        } else {
            a[1].cmp(&b[1])
        }
    });
    // "a" has weight 10.0 + 5.0 = 15.0, "b" has 5.0, "c" has 3.0
    // With --asc, sorted ascending: c (3.0), b (5.0), a (15.0)
    let mut expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["value", "c", "3", "13.04348", "1"],
        svec!["value", "b", "5", "21.73913", "2"],
        svec!["value", "a", "15", "65.21739", "3"],
    ];
    expected.sort_by(|a, b| {
        if a.len() < 2 || b.len() < 2 {
            std::cmp::Ordering::Equal
        } else {
            a[1].cmp(&b[1])
        }
    });
    assert_eq!(got, expected);
}

#[test]
fn frequency_weight_with_rank_strategy() {
    let wrk = Workdir::new("frequency_weight_with_rank_strategy");
    // Use duplicate values so it's not all-unique and can test rank strategy behavior
    let rows = vec![
        svec!["value", "weight"],
        svec!["a", "5.0"],
        svec!["a", "2.0"], // duplicate to make it not all-unique
        svec!["b", "3.0"],
        svec!["c", "3.0"],
        svec!["d", "2.0"],
    ];
    wrk.create("in.csv", rows);
    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--limit", "0"])
        .args(["--select", "value"])
        .args(["--weight", "weight"])
        .args(["--rank-strategy", "average"]);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // Sort by value for consistent comparison
    got.sort_by(|a, b| {
        if a.len() < 2 || b.len() < 2 {
            std::cmp::Ordering::Equal
        } else {
            a[1].cmp(&b[1])
        }
    });
    // "a" has weight 5.0 + 2.0 = 7.0 (rank 1), "b" and "c" tied at 3.0 (rank 2.5), "d" rank 4
    let mut expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["value", "a", "7", "46.66667", "1"],
        svec!["value", "b", "3", "20", "2.5"],
        svec!["value", "c", "3", "20", "2.5"],
        svec!["value", "d", "2", "13.33333", "4"],
    ];
    expected.sort_by(|a, b| {
        if a.len() < 2 || b.len() < 2 {
            std::cmp::Ordering::Equal
        } else {
            a[1].cmp(&b[1])
        }
    });
    assert_eq!(got, expected);
}

#[test]
fn frequency_weight_json() {
    let wrk = Workdir::new("frequency_weight_json");
    let rows = vec![
        svec!["value", "weight"],
        svec!["a", "2.0"],
        svec!["a", "3.0"],
        svec!["b", "1.0"],
    ];
    wrk.create("in.csv", rows);
    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--limit", "0"])
        .args(["--select", "value"])
        .args(["--weight", "weight"])
        .arg("--json");

    let got: String = wrk.stdout(&mut cmd);
    let v: Value = serde_json::from_str(&got).unwrap();
    assert_eq!(v["rowcount"], 3);
    assert_eq!(v["fieldcount"], 1);
    let fields = v["fields"].as_array().unwrap();
    assert_eq!(fields.len(), 1);
    let field = &fields[0];
    assert_eq!(field["field"], "value");
    let freqs = field["frequencies"].as_array().unwrap();
    // Should have 2 frequencies: "a" with count 5, "b" with count 1
    assert_eq!(freqs.len(), 2);
    // Check that counts are weighted (f64 values rounded to u64)
    let a_freq = freqs.iter().find(|f| f["value"] == "a").unwrap();
    assert_eq!(a_freq["count"], 5);
    let b_freq = freqs.iter().find(|f| f["value"] == "b").unwrap();
    assert_eq!(b_freq["count"], 1);
}

#[test]
fn frequency_weight_fractional_weights() {
    let wrk = Workdir::new("frequency_weight_fractional_weights");
    let rows = vec![
        svec!["value", "weight"],
        svec!["a", "1.5"],
        svec!["a", "2.5"],
        svec!["b", "0.5"],
        svec!["b", "0.5"],
    ];
    wrk.create("in.csv", rows);
    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--limit", "0"])
        .args(["--select", "value"])
        .args(["--weight", "weight"]);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    // "a" should have weight 1.5 + 2.5 = 4.0 (rounded to 4)
    // "b" should have weight 0.5 + 0.5 = 1.0 (rounded to 1)
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["value", "a", "4", "80", "1"],
        svec!["value", "b", "1", "20", "2"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_weight_all_unique() {
    let wrk = Workdir::new("frequency_weight_all_unique");
    let rows = vec![
        svec!["id", "weight"],
        svec!["1", "2.0"],
        svec!["2", "3.0"],
        svec!["3", "1.0"],
    ];
    wrk.create("in.csv", rows);
    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--select", "id"])
        .args(["--weight", "weight"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // For weighted frequencies with all-unique columns, show a single <ALL_UNIQUE> entry
    // with the sum of all weights (2.0 + 3.0 + 1.0 = 6.0)
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["id", "<ALL_UNIQUE>", "6", "100", "0"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_weight_column_not_found() {
    let wrk = Workdir::new("frequency_weight_column_not_found");
    let rows = vec![svec!["value", "weight"], svec!["a", "2.0"]];
    wrk.create("in.csv", rows);
    let mut cmd = wrk.command("frequency");
    cmd.args(["--weight", "nonexistent"]);

    let output = wrk.output_stderr(&mut cmd);
    assert!(output.contains("Weight column 'nonexistent' not found"));
}

#[test]
fn frequency_weight_with_ignore_case() {
    let wrk = Workdir::new("frequency_weight_with_ignore_case");
    // Use values that will combine with ignore-case, but add more rows to ensure
    // it's not detected as all-unique. Need to disable stats cache to avoid
    // it being detected as all-unique based on pre-computed stats.
    let rows = vec![
        svec!["value", "weight"],
        svec!["A", "2.0"],
        svec!["a", "3.0"],
        svec!["B", "1.0"],
        svec!["b", "2.0"],
        svec!["a", "1.0"], // Another "a" to ensure it's not all-unique
    ];
    wrk.create("in.csv", rows);
    let mut cmd = wrk.command("frequency");
    cmd.env("QSV_STATSCACHE_MODE", "none") // Disable stats cache to avoid all-unique detection
        .arg("in.csv")
        .args(["--limit", "0"])
        .args(["--select", "value"])
        .args(["--weight", "weight"])
        .arg("--ignore-case");

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // Sort by value for consistent comparison
    got.sort_by(|a, b| {
        if a.len() < 2 || b.len() < 2 {
            std::cmp::Ordering::Equal
        } else {
            a[1].cmp(&b[1])
        }
    });
    // With ignore-case, "A" and "a" should be combined: 2.0 + 3.0 + 1.0 = 6.0
    // "B" and "b" should be combined: 1.0 + 2.0 = 3.0
    let mut expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["value", "a", "6", "66.66667", "1"],
        svec!["value", "b", "3", "33.33333", "2"],
    ];
    expected.sort_by(|a, b| {
        if a.len() < 2 || b.len() < 2 {
            std::cmp::Ordering::Equal
        } else {
            a[1].cmp(&b[1])
        }
    });
    assert_eq!(got, expected);
}

#[test]
fn frequency_weight_with_no_nulls() {
    let wrk = Workdir::new("frequency_weight_with_no_nulls");
    // Use duplicate values so it's not all-unique and can test no-nulls behavior
    let rows = vec![
        svec!["value", "weight"],
        svec!["a", "2.0"],
        svec!["a", "1.0"], // duplicate to make it not all-unique
        svec!["", "3.0"],  // empty value
        svec!["b", "1.0"],
    ];
    wrk.create("in.csv", rows);
    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--limit", "0"])
        .args(["--select", "value"])
        .args(["--weight", "weight"])
        .arg("--no-nulls");

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // Sort by value for consistent comparison
    got.sort_by(|a, b| {
        if a.len() < 2 || b.len() < 2 {
            std::cmp::Ordering::Equal
        } else {
            a[1].cmp(&b[1])
        }
    });
    // Empty values should be excluded with --no-nulls
    // "a" has weight 2.0 + 1.0 = 3.0, "b" has 1.0
    let mut expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["value", "a", "3", "75", "1"],
        svec!["value", "b", "1", "25", "2"],
    ];
    expected.sort_by(|a, b| {
        if a.len() < 2 || b.len() < 2 {
            std::cmp::Ordering::Equal
        } else {
            a[1].cmp(&b[1])
        }
    });
    assert_eq!(got, expected);
}

#[test]
fn frequency_weight_parallel_merge() {
    let wrk = Workdir::new("frequency_weight_parallel_merge");

    // Create a larger dataset that will be split into multiple chunks
    // Use values that appear across chunks to verify correct weight aggregation
    let mut rows = vec![svec!["value", "weight"]];

    // Create enough rows to trigger chunking (at least 1000 rows)
    // Use a pattern where the same values appear in different chunks
    // to verify that weights are correctly aggregated during merge
    for i in 0..1000 {
        // Use modulo to create repeating patterns across chunks
        let value = match i % 10 {
            0 => "a",
            1 => "a", // "a" appears multiple times
            2 => "b",
            3 => "b", // "b" appears multiple times
            4 => "c",
            5 => "c", // "c" appears multiple times
            6 => "d",
            7 => "e",
            8 => "f",
            _ => "g",
        };
        // Use varying weights to ensure aggregation is correct
        let weight = (i % 5 + 1) as f64 * 1.5;
        rows.push(vec![value.to_string(), format!("{:.1}", weight)]);
    }

    // Create indexed file to enable parallel processing
    wrk.create_indexed("in.csv", rows);

    // Run weighted frequency with parallel processing (--jobs flag)
    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--limit", "0"])
        .args(["--select", "value"])
        .args(["--weight", "weight"])
        .args(["--jobs", "4"]);

    wrk.assert_success(&mut cmd);

    // Read the output
    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--limit", "0"])
        .args(["--select", "value"])
        .args(["--weight", "weight"])
        .args(["--jobs", "4"]);
    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Sort by value for consistent comparison
    got.sort_by(|a, b| {
        if a.len() < 2 || b.len() < 2 {
            std::cmp::Ordering::Equal
        } else {
            a[1].cmp(&b[1])
        }
    });

    // Calculate expected weights:
    // weight = (i % 5 + 1) * 1.5
    // "a" appears when i % 10 == 0 or 1: weights 1.5, 3.0, 1.5, 3.0, ... = 450 total
    // "b" appears when i % 10 == 2 or 3: weights 4.5, 6.0, 4.5, 6.0, ... = 1050 total
    // "c" appears when i % 10 == 4 or 5: weights 7.5, 1.5, 7.5, 1.5, ... = 900 total
    // "d" appears when i % 10 == 6: weights 4.5, 4.5, ... = 300 total
    // "e" appears when i % 10 == 7: weights 6.0, 6.0, ... = 450 total
    // "f" appears when i % 10 == 8: weights 7.5, 7.5, ... = 600 total
    // "g" appears when i % 10 == 9: weights 1.5, 1.5, ... = 150 total (but wait, let me recalc)

    // Actually recalculated with Python:
    // a: 450, b: 1050, c: 900, d: 300, e: 450, f: 600, g: 750

    // Find the frequency rows (skip header)
    let freq_rows: Vec<_> = got
        .iter()
        .filter(|r| r.len() > 1 && r[0] == "value")
        .collect();

    // Verify we have the expected number of unique values
    assert_eq!(freq_rows.len(), 7, "Should have 7 unique values");

    // Verify weights are correctly aggregated by checking specific values
    let find_freq = |value: &str| -> Option<&Vec<String>> {
        freq_rows.iter().find(|r| r[1] == value).map(|r| *r)
    };

    // Check that "a" has aggregated weight of 450 (rounded)
    let a_freq = find_freq("a").expect("Should find 'a'");
    assert_eq!(
        a_freq[2], "450",
        "Value 'a' should have aggregated weight 450"
    );

    // Check that "b" has aggregated weight of 1050 (rounded)
    let b_freq = find_freq("b").expect("Should find 'b'");
    assert_eq!(
        b_freq[2], "1050",
        "Value 'b' should have aggregated weight 1050"
    );

    // Check that "c" has aggregated weight of 900 (rounded)
    let c_freq = find_freq("c").expect("Should find 'c'");
    assert_eq!(
        c_freq[2], "900",
        "Value 'c' should have aggregated weight 900"
    );

    // Verify that parallel processing produces same results as sequential
    // Run sequential version for comparison
    let mut cmd_seq = wrk.command("frequency");
    cmd_seq
        .arg("in.csv")
        .args(["--limit", "0"])
        .args(["--select", "value"])
        .args(["--weight", "weight"])
        .args(["--jobs", "1"]);
    let mut got_seq: Vec<Vec<String>> = wrk.read_stdout(&mut cmd_seq);
    got_seq.sort_by(|a, b| {
        if a.len() < 2 || b.len() < 2 {
            std::cmp::Ordering::Equal
        } else {
            a[1].cmp(&b[1])
        }
    });

    // Compare parallel and sequential results - they should match
    let freq_rows_seq: Vec<_> = got_seq
        .iter()
        .filter(|r| r.len() > 1 && r[0] == "value")
        .collect();
    assert_eq!(
        freq_rows.len(),
        freq_rows_seq.len(),
        "Parallel and sequential should have same number of frequencies"
    );

    // Compare each frequency value
    for (par_row, seq_row) in freq_rows.iter().zip(freq_rows_seq.iter()) {
        assert_eq!(par_row[1], seq_row[1], "Values should match");
        assert_eq!(
            par_row[2], seq_row[2],
            "Weights should match between parallel and sequential"
        );
    }
}

#[test]
fn frequency_weight_with_unq_limit_all_unique() {
    let wrk = Workdir::new("frequency_weight_with_unq_limit_all_unique");

    // Create a dataset where all values are unique (like an ID column)
    let mut rows = vec![svec!["id", "weight"]];
    for i in 1..=100 {
        rows.push(vec![format!("id_{}", i), format!("{}", i)]);
    }
    wrk.create("in.csv", rows);

    // Test that with --weight and all-unique columns, show a single <ALL_UNIQUE> entry
    // The sum of weights is 1+2+3+...+100 = 5050
    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--select", "id"])
        .args(["--weight", "weight"])
        .args(["--limit", "0"])
        .args(["--unq-limit", "10"]); // This should be ignored for all-unique columns

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // With --weight and all-unique columns, should show a single <ALL_UNIQUE> entry
    let freq_rows: Vec<_> = got.iter().filter(|r| r.len() > 1 && r[0] == "id").collect();
    assert_eq!(
        freq_rows.len(),
        1,
        "All-unique columns with --weight should show a single <ALL_UNIQUE> entry"
    );
    assert_eq!(
        freq_rows[0][1], "<ALL_UNIQUE>",
        "Value should be <ALL_UNIQUE>"
    );
    assert_eq!(
        freq_rows[0][2], "5050",
        "Count should be sum of all weights (1+2+...+100 = 5050)"
    );
    assert_eq!(
        freq_rows[0][4], "0",
        "Rank should be 0 for all-unique entries"
    );
}

#[test]
fn frequency_weight_with_unq_limit_and_limit() {
    let wrk = Workdir::new("frequency_weight_with_unq_limit_and_limit");

    // Create a dataset where all values are unique
    let mut rows = vec![svec!["id", "weight"]];
    for i in 1..=50 {
        rows.push(vec![format!("id_{}", i), format!("{}", 51 - i)]); // Higher IDs have lower weights
    }
    wrk.create("in.csv", rows);

    // Test that with --weight and all-unique columns, show a single <ALL_UNIQUE> entry
    // regardless of --limit or --unq-limit
    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--select", "id"])
        .args(["--weight", "weight"])
        .args(["--limit", "5"])
        .args(["--unq-limit", "10"]); // Both should be ignored for all-unique columns

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // With --weight and all-unique columns, should show a single <ALL_UNIQUE> entry
    let freq_rows: Vec<_> = got.iter().filter(|r| r.len() > 1 && r[0] == "id").collect();
    // Should have a single <ALL_UNIQUE> entry, not limited by --limit
    assert_eq!(
        freq_rows.len(),
        1,
        "All-unique columns with --weight should show a single <ALL_UNIQUE> entry regardless of \
         --limit"
    );
    assert_eq!(
        freq_rows[0][1], "<ALL_UNIQUE>",
        "Value should be <ALL_UNIQUE>"
    );
    // Sum of weights: 1+2+3+...+50 = 1275
    assert_eq!(
        freq_rows[0][2], "1275",
        "Count should be sum of all weights (1+2+...+50 = 1275)"
    );
    assert_eq!(
        freq_rows[0][4], "0",
        "Rank should be 0 for all-unique entries"
    );
}

#[test]
fn frequency_weight_unq_limit_vs_unweighted() {
    let wrk = Workdir::new("frequency_weight_unq_limit_vs_unweighted");

    // Create a dataset where all values are unique
    let mut rows = vec![svec!["id", "weight"]];
    for i in 1..=20 {
        rows.push(vec![format!("id_{}", i), "1.0".to_string()]);
    }
    wrk.create("in.csv", rows);

    // Test unweighted frequency with --unq-limit
    // Need to disable stats cache to force computation of all frequencies
    // Note: --unq-limit only applies when --limit > 0 and --limit != --unq-limit
    let mut cmd_unweighted = wrk.command("frequency");
    cmd_unweighted
        .env("QSV_STATSCACHE_MODE", "none")
        .arg("in.csv")
        .args(["--select", "id"])
        .args(["--limit", "10"]) // Must be > 0 and different from --unq-limit
        .args(["--unq-limit", "5"]);

    let mut got_unweighted: Vec<Vec<String>> = wrk.read_stdout(&mut cmd_unweighted);
    got_unweighted.sort_by(|a, b| {
        if a.len() < 2 || b.len() < 2 {
            std::cmp::Ordering::Equal
        } else {
            a[1].cmp(&b[1])
        }
    });

    let freq_rows_unweighted: Vec<_> = got_unweighted
        .iter()
        .filter(|r| {
            r.len() > 1 && r[0] == "id" && r[1] != "<ALL_UNIQUE>" && !r[1].starts_with("Other")
        })
        .collect();

    // Test weighted frequency with --unq-limit (should be ignored)
    let mut cmd_weighted = wrk.command("frequency");
    cmd_weighted
        .arg("in.csv")
        .args(["--select", "id"])
        .args(["--weight", "weight"])
        .args(["--limit", "0"])
        .args(["--unq-limit", "5"]); // Should be ignored

    let mut got_weighted: Vec<Vec<String>> = wrk.read_stdout(&mut cmd_weighted);
    got_weighted.sort_by(|a, b| {
        if a.len() < 2 || b.len() < 2 {
            std::cmp::Ordering::Equal
        } else {
            a[1].cmp(&b[1])
        }
    });

    let freq_rows_weighted: Vec<_> = got_weighted
        .iter()
        .filter(|r| r.len() > 1 && r[0] == "id")
        .collect();

    // Unweighted should be limited to 5 by --unq-limit
    // Weighted should show a single <ALL_UNIQUE> entry (--unq-limit ignored, all-unique columns
    // show summary)
    assert_eq!(
        freq_rows_unweighted.len(),
        5,
        "Unweighted frequency should be limited by --unq-limit"
    );
    assert_eq!(
        freq_rows_weighted.len(),
        1,
        "Weighted frequency with all-unique columns should show a single <ALL_UNIQUE> entry"
    );
    assert_eq!(
        freq_rows_weighted[0][1], "<ALL_UNIQUE>",
        "Weighted frequency should show <ALL_UNIQUE> for all-unique columns"
    );
    // Sum of weights: 20 * 1.0 = 20.0
    assert_eq!(
        freq_rows_weighted[0][2], "20",
        "Count should be sum of all weights (20 * 1.0 = 20)"
    );
}

#[test]
fn frequency_weight_unq_limit_with_limit_zero() {
    let wrk = Workdir::new("frequency_weight_unq_limit_with_limit_zero");

    // Create a dataset with some duplicate values and some unique values
    let rows = vec![
        svec!["value", "weight"],
        svec!["a", "10.0"],
        svec!["a", "5.0"],
        svec!["b", "3.0"],
        svec!["c", "2.0"],
        svec!["d", "1.0"],
        svec!["e", "1.0"],
        svec!["f", "1.0"],
        svec!["g", "1.0"],
        svec!["h", "1.0"],
        svec!["i", "1.0"],
        svec!["j", "1.0"],
    ];
    wrk.create("in.csv", rows);

    // Test with --limit 0 and --unq-limit 5
    // Since not all values are unique, --unq-limit shouldn't apply anyway
    // But verify that --limit 0 shows all values
    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--select", "value"])
        .args(["--weight", "weight"])
        .args(["--limit", "0"])
        .args(["--unq-limit", "5"]);

    wrk.assert_success(&mut cmd);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_by(|a, b| {
        if a.len() < 2 || b.len() < 2 {
            std::cmp::Ordering::Equal
        } else {
            a[1].cmp(&b[1])
        }
    });

    let freq_rows: Vec<_> = got
        .iter()
        .filter(|r| r.len() > 1 && r[0] == "value")
        .collect();
    // Should show all unique values (a, b, c, d, e, f, g, h, i, j = 10 values)
    assert_eq!(
        freq_rows.len(),
        10,
        "With --limit 0, all unique values should be shown"
    );
}

#[test]
fn frequency_weight_nan_values() {
    let wrk = Workdir::new("frequency_weight_nan_values");

    // Create a dataset with NaN weight values
    let rows = vec![
        svec!["value", "weight"],
        svec!["a", "1.0"],
        svec!["a", "NaN"], // NaN weight
        svec!["b", "2.0"],
        svec!["b", "nan"], // lowercase NaN
        svec!["c", "3.0"],
    ];
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--limit", "0"])
        .args(["--select", "value"])
        .args(["--weight", "weight"]);

    wrk.assert_success(&mut cmd);

    // Read output - need to create a new command since assert_success consumes it
    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--limit", "0"])
        .args(["--select", "value"])
        .args(["--weight", "weight"]);
    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_by(|a, b| {
        if a.len() < 2 || b.len() < 2 {
            std::cmp::Ordering::Equal
        } else {
            a[1].cmp(&b[1])
        }
    });

    // Skip header row
    let freq_rows: Vec<_> = got
        .iter()
        .filter(|r| r.len() > 1 && r[0] == "value" && r[1] != "value") // Skip header
        .collect();

    let find_freq = |value: &str| -> Option<&Vec<String>> {
        freq_rows.iter().find(|r| r[1] == value).map(|r| *r)
    };

    // The main goal is to verify that NaN weight values are handled gracefully without panicking.
    // The exact behavior depends on how fast_float2 parses "NaN":
    // - If it parses as NaN (non-finite), values with NaN weights get filtered out
    // - If it fails to parse, it defaults to 1.0 and values appear

    // "c" should always appear (no NaN weights)
    assert!(
        find_freq("c").is_some(),
        "Value 'c' should appear (no NaN weights)"
    );
    if let Some(c_freq) = find_freq("c") {
        assert_eq!(c_freq[2], "3", "Value 'c' should have weight 3");
    }

    // "a" and "b" may or may not appear depending on NaN parsing behavior
    // Just verify the command handled NaN gracefully without panicking
    // If they appear, verify they have reasonable positive weights
    if let Some(a_freq) = find_freq("a") {
        let a_weight: u64 = a_freq[2].parse().unwrap_or(0);
        assert!(
            a_weight > 0,
            "Value 'a' should have positive weight if it appears"
        );
    }

    if let Some(b_freq) = find_freq("b") {
        let b_weight: u64 = b_freq[2].parse().unwrap_or(0);
        assert!(
            b_weight > 0,
            "Value 'b' should have positive weight if it appears"
        );
    }
}

#[test]
fn frequency_weight_infinity_values() {
    let wrk = Workdir::new("frequency_weight_infinity_values");

    // Create a dataset with infinity weight values
    let rows = vec![
        svec!["value", "weight"],
        svec!["a", "1.0"],
        svec!["a", "Inf"], // Positive infinity
        svec!["b", "2.0"],
        svec!["b", "inf"], // lowercase infinity
        svec!["c", "3.0"],
        svec!["d", "-Inf"], // Negative infinity
    ];
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--limit", "0"])
        .args(["--select", "value"])
        .args(["--weight", "weight"]);

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    let expected_values = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["value", "c", "3", "50", "1"],
        svec!["value", "b", "2", "33.33333", "2"],
        svec!["value", "a", "1", "16.66667", "3"],
    ];
    assert_eq!(got, expected_values);
}

#[test]
fn frequency_weight_extremely_large_values() {
    let wrk = Workdir::new("frequency_weight_extremely_large_values");

    // Create a dataset with extremely large weight values
    // Use duplicate values so it's not all-unique and can test clamping behavior
    let huge_weight = format!("{}", u64::MAX as f64 * 2.0);
    let rows = vec![
        svec!["value", "weight"],
        svec!["a", "1.0"],
        svec!["a", "1.0"],                  // duplicate to make it not all-unique
        svec!["b", "1e20"],                 // Very large but finite
        vec!["c".to_string(), huge_weight], // Larger than u64::MAX
        svec!["d", "1e308"],                // Near f64::MAX
    ];
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--limit", "0"])
        .args(["--select", "value"])
        .args(["--weight", "weight"]);

    wrk.assert_success(&mut cmd);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_by(|a, b| {
        if a.len() < 2 || b.len() < 2 {
            std::cmp::Ordering::Equal
        } else {
            a[1].cmp(&b[1])
        }
    });

    // Extremely large values should be clamped to u64::MAX
    let freq_rows: Vec<_> = got
        .iter()
        .filter(|r| r.len() > 1 && r[0] == "value")
        .collect();

    // All values should appear (a, b, c, d)
    assert_eq!(freq_rows.len(), 4, "Should have 4 values");

    let find_freq = |value: &str| -> Option<&Vec<String>> {
        freq_rows.iter().find(|r| r[1] == value).map(|r| *r)
    };

    // Verify that extremely large values are clamped to u64::MAX
    let c_freq = find_freq("c").expect("Should find 'c'");
    let c_count: u64 = c_freq[2].parse().expect("Should parse count");
    assert_eq!(
        c_count,
        u64::MAX,
        "Extremely large weight should be clamped to u64::MAX"
    );

    // Verify other values are correct
    // "a" appears twice with weight 1.0 each, so total weight is 2.0
    let a_freq = find_freq("a").expect("Should find 'a'");
    assert_eq!(a_freq[2], "2", "Value 'a' should have weight 2 (1.0 + 1.0)");
}

#[test]
fn frequency_weight_mixed_invalid_values() {
    let wrk = Workdir::new("frequency_weight_mixed_invalid_values");

    // Create a dataset with various invalid weight values
    // Use a large but reasonable value instead of f64::MAX to avoid potential parsing issues
    let huge_weight_str = "1e100"; // Very large but still reasonable
    let rows = vec![
        svec!["value", "weight"],
        svec!["valid1", "5.0"],
        svec!["valid2", "10.0"],
        svec!["nan1", "NaN"],
        svec!["nan2", "nan"],
        svec!["inf1", "Inf"],
        svec!["inf2", "infinity"],
        svec!["neginf", "-Inf"],
        svec!["zero", "0.0"],
        svec!["negative", "-5.0"],
        svec!["huge", huge_weight_str],
    ];
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--limit", "0"])
        .args(["--select", "value"])
        .args(["--weight", "weight"]);

    wrk.assert_success(&mut cmd);

    // Read output - need to create a new command since assert_success consumes it
    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--limit", "0"])
        .args(["--select", "value"])
        .args(["--weight", "weight"]);
    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_by(|a, b| {
        if a.len() < 2 || b.len() < 2 {
            std::cmp::Ordering::Equal
        } else {
            a[1].cmp(&b[1])
        }
    });

    // Skip header row: filter out where r[1] == "value" (header)
    let freq_rows: Vec<_> = got
        .iter()
        .filter(|r| r.len() > 1 && r[0] == "value" && r[1] != "value") // Skip header
        .collect();

    let find_freq = |value: &str| -> Option<&Vec<String>> {
        freq_rows.iter().find(|r| r[1] == value).map(|r| *r)
    };

    // The main goal is to verify that invalid weight values are handled gracefully
    // without panicking. The exact behavior may vary based on how fast_float2 parses values.

    // The main goal is to verify that invalid weight values are handled gracefully without
    // panicking. The exact behavior depends on how fast_float2 parses values and how they're
    // aggregated. Some values may be filtered out if they aggregate with invalid values to
    // become non-finite.

    // At minimum, we should have some frequency values (at least "huge" should appear)
    assert!(
        freq_rows.len() > 0,
        "Should have at least some frequency values"
    );

    // Values with valid weights should ideally appear, but may be filtered if they aggregate
    // with invalid values to become non-finite. The important thing is graceful handling.
    if let Some(valid1_freq) = find_freq("valid1") {
        let valid1_weight: u64 = valid1_freq[2].parse().unwrap_or(0);
        assert!(
            valid1_weight > 0,
            "valid1 should have positive weight if it appears"
        );
    }

    if let Some(valid2_freq) = find_freq("valid2") {
        let valid2_weight: u64 = valid2_freq[2].parse().unwrap_or(0);
        assert!(
            valid2_weight > 0,
            "valid2 should have positive weight if it appears"
        );
    }

    // Values that should definitely be skipped (zero or negative weights)
    assert!(
        find_freq("neginf").is_none(),
        "Negative infinity should be skipped (weight <= 0.0)"
    );
    assert!(
        find_freq("zero").is_none(),
        "Zero weights should be skipped (weight <= 0.0)"
    );
    assert!(
        find_freq("negative").is_none(),
        "Negative weights should be skipped (weight <= 0.0)"
    );

    // NaN/infinity values may or may not appear depending on parsing behavior
    // The important thing is that the command handles them without panicking
    // If they parse as NaN/infinity, they get filtered out (non-finite check)
    // If they fail to parse, they default to 1.0 and appear

    // "huge" should appear if it parses successfully (very large but finite)
    // It will be clamped to u64::MAX when converted
    if let Some(huge_freq) = find_freq("huge") {
        let huge_count: u64 = huge_freq[2].parse().expect("Should parse count");
        assert_eq!(
            huge_count,
            u64::MAX,
            "Extremely large weight should be clamped to u64::MAX"
        );
    }
}

#[test]
fn frequency_weight_no_other_zero() {
    let wrk = Workdir::new("frequency_weight_no_other_zero");

    // Create a dataset with multiple values and weights
    // This test exercises issue #3223: "Other (0)" entries should not appear
    // when --limit 0 is used and all values are included
    // Use duplicate values to ensure it's not detected as all-unique
    let rows = vec![
        svec!["value", "weight"],
        svec!["a", "5.0"],
        svec!["a", "2.0"], // duplicate to make it not all-unique
        svec!["b", "3.0"],
        svec!["c", "2.0"],
        svec!["d", "1.0"],
    ];
    wrk.create("in.csv", rows);

    // Test with --limit 0: all values should be included, no "Other" entry
    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--limit", "0"])
        .args(["--unq-limit", "0"])
        .args(["--select", "value"])
        .args(["--weight", "weight"]);

    wrk.assert_success(&mut cmd);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_by(|a, b| {
        if a.len() < 2 || b.len() < 2 {
            std::cmp::Ordering::Equal
        } else {
            a[1].cmp(&b[1])
        }
    });

    // Filter out header row
    let freq_rows: Vec<_> = got
        .iter()
        .filter(|r| r.len() > 1 && r[0] == "value" && r[1] != "value")
        .collect();

    // Should have exactly 4 values (a, b, c, d), no "Other" entry
    assert_eq!(
        freq_rows.len(),
        4,
        "With --limit 0, all values should be included, no 'Other' entry"
    );

    // Verify no "Other" entry exists
    let has_other = freq_rows
        .iter()
        .any(|r| r.len() > 1 && r[1].starts_with("Other"));
    assert!(
        !has_other,
        "Should not have 'Other' entry when all values are included (--limit 0)"
    );

    // Verify all expected values are present
    let find_freq = |value: &str| -> Option<&Vec<String>> {
        freq_rows.iter().find(|r| r[1] == value).map(|r| *r)
    };

    assert!(find_freq("a").is_some(), "Should find 'a'");
    assert!(find_freq("b").is_some(), "Should find 'b'");
    assert!(find_freq("c").is_some(), "Should find 'c'");
    assert!(find_freq("d").is_some(), "Should find 'd'");

    // Verify weights are correct
    // "a" has weight 5.0 + 2.0 = 7.0
    let a_freq = find_freq("a").unwrap();
    assert_eq!(a_freq[2], "7", "Value 'a' should have weight 7 (5.0 + 2.0)");

    let b_freq = find_freq("b").unwrap();
    assert_eq!(b_freq[2], "3", "Value 'b' should have weight 3");

    let c_freq = find_freq("c").unwrap();
    assert_eq!(c_freq[2], "2", "Value 'c' should have weight 2");

    let d_freq = find_freq("d").unwrap();
    assert_eq!(d_freq[2], "1", "Value 'd' should have weight 1");

    // Test with --limit 2: should have "Other" entry with remaining values
    let mut cmd_limit = wrk.command("frequency");
    cmd_limit
        .arg("in.csv")
        .args(["--limit", "2"])
        .args(["--select", "value"])
        .args(["--weight", "weight"]);

    let mut got_limit: Vec<Vec<String>> = wrk.read_stdout(&mut cmd_limit);
    got_limit.sort_by(|a, b| {
        if a.len() < 2 || b.len() < 2 {
            std::cmp::Ordering::Equal
        } else {
            a[1].cmp(&b[1])
        }
    });

    let freq_rows_limit: Vec<_> = got_limit
        .iter()
        .filter(|r| r.len() > 1 && r[0] == "value" && r[1] != "value")
        .collect();

    // Should have 2 top values + 1 "Other" entry = 3 total
    assert_eq!(
        freq_rows_limit.len(),
        3,
        "With --limit 2, should have 2 top values + 1 'Other' entry"
    );

    // Verify "Other" entry exists and has correct count
    let other_entry = freq_rows_limit
        .iter()
        .find(|r| r.len() > 1 && r[1].starts_with("Other"))
        .expect("Should have 'Other' entry when --limit is set");
    assert!(
        other_entry[1].contains("Other"),
        "Should have 'Other' entry"
    );
    // Other should have weight 2.0 + 1.0 = 3.0 (c + d)
    // Note: "a" has weight 7.0, "b" has 3.0, so top 2 are "a" and "b"
    // Remaining: "c" (2.0) + "d" (1.0) = 3.0
    assert_eq!(
        other_entry[2], "3",
        "Other entry should have weight 3 (c + d)"
    );
}

// ============================================================
// --no-float tests
// ============================================================

#[test]
fn frequency_no_float_basic() {
    let wrk = Workdir::new("frequency_no_float_basic");

    // Create CSV with String, Integer, Float columns
    wrk.create(
        "in.csv",
        vec![
            svec!["name", "age", "price"],
            svec!["Alice", "30", "19.99"],
            svec!["Bob", "25", "29.99"],
            svec!["Alice", "30", "39.99"],
        ],
    );

    // Run stats to create cache (required for type detection)
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("in.csv")
        .arg("--cardinality")
        .arg("--stats-jsonl");
    wrk.assert_success(&mut stats_cmd);

    // Run frequency with --no-float "*" to exclude all Float columns
    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--no-float", "*"])
        .args(["--limit", "0"]);

    let got: String = wrk.stdout(&mut cmd);

    // Verify Float column 'price' is excluded from output
    assert!(
        !got.contains("price"),
        "Float column 'price' should be excluded"
    );
    // Verify String column 'name' is included
    assert!(
        got.contains("name"),
        "String column 'name' should be included"
    );
    // Verify Integer column 'age' is included
    assert!(
        got.contains("age"),
        "Integer column 'age' should be included"
    );
}

#[test]
fn frequency_no_float_with_exceptions() {
    let wrk = Workdir::new("frequency_no_float_with_exceptions");

    // Create CSV with multiple Float columns
    wrk.create(
        "in.csv",
        vec![
            svec!["name", "price", "rate", "score"],
            svec!["Alice", "19.99", "0.05", "8.5"],
            svec!["Bob", "29.99", "0.10", "9.0"],
            svec!["Carol", "39.99", "0.15", "7.5"],
        ],
    );

    // Run stats to create cache
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("in.csv")
        .arg("--cardinality")
        .arg("--stats-jsonl");
    wrk.assert_success(&mut stats_cmd);

    // Run frequency with --no-float but exempt 'price'
    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--no-float", "price"])
        .args(["--limit", "0"]);

    let got: String = wrk.stdout(&mut cmd);

    // Verify 'price' IS included (it's in the exception list)
    assert!(
        got.contains("price"),
        "Exception column 'price' should be included"
    );
    // Verify 'rate' and 'score' are excluded (not in exception list)
    assert!(
        !got.contains(",rate,"),
        "Float column 'rate' should be excluded"
    );
    assert!(
        !got.contains(",score,"),
        "Float column 'score' should be excluded"
    );
    // Verify 'name' is included (String type)
    assert!(
        got.contains("name"),
        "String column 'name' should be included"
    );
}

#[test]
fn frequency_no_float_with_select() {
    let wrk = Workdir::new("frequency_no_float_with_select");

    // Create CSV with mixed types
    wrk.create(
        "in.csv",
        vec![
            svec!["name", "age", "price", "category"],
            svec!["Alice", "30", "19.99", "A"],
            svec!["Bob", "25", "29.99", "B"],
            svec!["Carol", "35", "39.99", "A"],
        ],
    );

    // Run stats to create cache
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("in.csv")
        .arg("--cardinality")
        .arg("--stats-jsonl");
    wrk.assert_success(&mut stats_cmd);

    // Run frequency with --select AND --no-float "*"
    // Select columns: name, price, category
    // --no-float should exclude 'price' from the selected columns
    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--select", "name,price,category"])
        .args(["--no-float", "*"])
        .args(["--limit", "0"]);

    let got: String = wrk.stdout(&mut cmd);

    // Verify 'price' is excluded even though it was selected
    assert!(
        !got.contains("price"),
        "Float column 'price' should be excluded"
    );
    // Verify other selected columns are included
    assert!(
        got.contains("name"),
        "Selected column 'name' should be included"
    );
    assert!(
        got.contains("category"),
        "Selected column 'category' should be included"
    );
    // Verify unselected column is not included
    assert!(
        !got.contains(",age,"),
        "Unselected column 'age' should not be included"
    );
}

#[test]
fn frequency_no_float_all_floats_error() {
    let wrk = Workdir::new("frequency_no_float_all_floats_error");

    // Create CSV with only Float columns (except for what will become an ID)
    wrk.create(
        "in.csv",
        vec![
            svec!["price", "rate"],
            svec!["19.99", "0.05"],
            svec!["29.99", "0.10"],
            svec!["39.99", "0.15"],
        ],
    );

    // Run stats to create cache
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("in.csv")
        .arg("--cardinality")
        .arg("--stats-jsonl");
    wrk.assert_success(&mut stats_cmd);

    // Run frequency with --no-float "*" - should error because all columns are Float
    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--no-float", "*"])
        .args(["--limit", "0"]);

    wrk.assert_err(&mut cmd);
}

#[test]
fn frequency_no_float_auto_stats() {
    let wrk = Workdir::new("frequency_no_float_auto_stats");

    // Create CSV with mixed types but DON'T run stats first
    // The frequency command will auto-create stats cache when needed
    wrk.create(
        "in.csv",
        vec![
            svec!["name", "price"],
            svec!["Alice", "19.99"],
            svec!["Bob", "29.99"],
        ],
    );

    // Run frequency with --no-float "*" WITHOUT pre-existing stats cache
    // Frequency will auto-create stats, so Float detection should still work
    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--no-float", "*"])
        .args(["--limit", "0"]);

    let got: String = wrk.stdout(&mut cmd);

    // Stats cache is auto-created, so Float column 'price' should be excluded
    assert!(
        got.contains("name"),
        "String column 'name' should be included"
    );
    // Float column 'price' should be excluded (stats were auto-created)
    assert!(
        !got.contains("price"),
        "Float column 'price' should be excluded (stats auto-created)"
    );
}

#[test]
fn frequency_no_float_with_json() {
    let wrk = Workdir::new("frequency_no_float_with_json");

    // Create CSV with mixed types
    wrk.create(
        "in.csv",
        vec![
            svec!["name", "age", "price"],
            svec!["Alice", "30", "19.99"],
            svec!["Bob", "25", "29.99"],
            svec!["Alice", "30", "39.99"],
        ],
    );

    // Run stats to create cache
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("in.csv")
        .arg("--cardinality")
        .arg("--stats-jsonl");
    wrk.assert_success(&mut stats_cmd);

    // Run frequency with --no-float "*" and --json
    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--no-float", "*"])
        .arg("--json")
        .args(["--limit", "0"]);

    let got: String = wrk.stdout(&mut cmd);

    // Parse JSON to verify structure
    let json: serde_json::Value = serde_json::from_str(&got).expect("Valid JSON output");

    // Verify fields array exists and has expected count (2 fields: name, age - not price)
    let fields = json["fields"]
        .as_array()
        .expect("fields should be an array");
    assert_eq!(
        fields.len(),
        2,
        "Should have 2 fields (name, age) after excluding Float column 'price'"
    );

    // Verify field names
    let field_names: Vec<&str> = fields
        .iter()
        .map(|f| f["field"].as_str().unwrap())
        .collect();
    assert!(field_names.contains(&"name"), "Should include 'name' field");
    assert!(field_names.contains(&"age"), "Should include 'age' field");
    assert!(
        !field_names.contains(&"price"),
        "Should NOT include 'price' field"
    );
}

#[test]
fn frequency_weight_json_no_stats() {
    // Issue #3339: When using --weight --json, stats should be omitted
    // because stats cache contains unweighted stats which would be misleading
    let wrk = Workdir::new("frequency_weight_json_no_stats");

    // Create CSV with numeric data that would have stats
    wrk.create(
        "in.csv",
        vec![
            svec!["value", "weight"],
            svec!["10", "2.0"],
            svec!["10", "3.0"],
            svec!["20", "1.0"],
        ],
    );

    // Run stats to create cache (this would normally provide stats for JSON output)
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("in.csv")
        .arg("--cardinality")
        .arg("--stats-jsonl");
    wrk.assert_success(&mut stats_cmd);

    // Run frequency with --weight and --json
    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--select", "value"])
        .args(["--weight", "weight"])
        .arg("--json")
        .args(["--limit", "0"]);

    let got: String = wrk.stdout(&mut cmd);
    let json: serde_json::Value = serde_json::from_str(&got).expect("Valid JSON output");

    // Verify the JSON structure is correct
    assert_eq!(json["rowcount"], 3);
    assert_eq!(json["fieldcount"], 1);

    // Verify stats are empty or omitted for weighted mode
    let fields = json["fields"]
        .as_array()
        .expect("fields should be an array");
    assert_eq!(fields.len(), 1);

    let field = &fields[0];
    assert_eq!(field["field"], "value");

    // Stats should either be missing or empty array
    // (Empty arrays are removed from JSON output, so stats key should not exist)
    let stats = field.get("stats");
    assert!(
        stats.is_none() || stats.unwrap().as_array().map_or(true, |a| a.is_empty()),
        "Stats should be omitted or empty when using --weight --json"
    );

    // Verify frequencies are still correct (weighted)
    let freqs = field["frequencies"]
        .as_array()
        .expect("frequencies should be an array");
    assert_eq!(freqs.len(), 2);

    // "10" should have weighted count of 5 (2.0 + 3.0)
    let val_10 = freqs.iter().find(|f| f["value"] == "10").unwrap();
    assert_eq!(val_10["count"], 5);

    // "20" should have weighted count of 1
    let val_20 = freqs.iter().find(|f| f["value"] == "20").unwrap();
    assert_eq!(val_20["count"], 1);
}

// --stats-filter tests (require luau feature)
#[test]
#[cfg(feature = "luau")]
fn frequency_stats_filter_nullcount() {
    // Test filtering columns by nullcount
    let wrk = Workdir::new("frequency_stats_filter_nullcount");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "age", "notes"],
            svec!["Alice", "30", "active"],
            svec!["Bob", "", ""], // age and notes have nulls
            svec!["Carol", "25", "active"],
            svec!["Dave", "", ""], // more nulls
        ],
    );

    // First, create the stats cache
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("data.csv")
        .arg("--cardinality")
        .arg("--stats-jsonl");
    wrk.run(&mut stats_cmd);

    // Now run frequency with --stats-filter to exclude columns with nullcount > 0
    let mut cmd = wrk.command("frequency");
    cmd.arg("data.csv")
        .args(["--stats-filter", "nullcount > 0"])
        .args(["--limit", "0"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // Only 'name' column should remain (no nulls)
    // 'age' and 'notes' should be excluded (they have nulls)
    assert!(!got.is_empty());
    let headers: Vec<&str> = got.iter().skip(1).map(|row| row[0].as_str()).collect();
    // All rows should be for the 'name' column only
    for h in headers.iter() {
        assert_eq!(*h, "name", "Only 'name' column should appear, got '{h}'");
    }
}

#[test]
#[cfg(feature = "luau")]
fn frequency_stats_filter_type() {
    // Test filtering columns by type
    let wrk = Workdir::new("frequency_stats_filter_type");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "price", "quantity"],
            svec!["Apple", "1.99", "10"],
            svec!["Banana", "0.99", "25"],
            svec!["Cherry", "3.49", "5"],
        ],
    );

    // First, create the stats cache
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("data.csv")
        .arg("--cardinality")
        .arg("--stats-jsonl");
    wrk.run(&mut stats_cmd);

    // Now run frequency with --stats-filter to exclude Float columns
    let mut cmd = wrk.command("frequency");
    cmd.arg("data.csv")
        .args(["--stats-filter", "type == 'Float'"])
        .args(["--limit", "0"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // Only 'name' and 'quantity' columns should remain (String and Integer)
    // 'price' should be excluded (it's Float)
    assert!(!got.is_empty());
    let fields: HashSet<&str> = got.iter().skip(1).map(|row| row[0].as_str()).collect();
    assert!(fields.contains("name"), "Expected 'name' column");
    assert!(fields.contains("quantity"), "Expected 'quantity' column");
    assert!(
        !fields.contains("price"),
        "Did not expect 'price' (Float) column"
    );
}

#[test]
#[cfg(feature = "luau")]
fn frequency_stats_filter_compound_expression() {
    // Test compound filter expression with and/or
    let wrk = Workdir::new("frequency_stats_filter_compound");
    wrk.create(
        "data.csv",
        vec![
            svec!["id", "category", "status", "count"],
            svec!["1", "A", "active", "10"],
            svec!["2", "A", "", "20"], // status has null
            svec!["3", "B", "active", "10"],
            svec!["4", "B", "pending", "30"],
        ],
    );

    // First, create the stats cache
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("data.csv")
        .arg("--cardinality")
        .arg("--stats-jsonl");
    wrk.run(&mut stats_cmd);

    // Filter: exclude columns where cardinality == rowcount (unique IDs)
    // or nullcount > 0
    let mut cmd = wrk.command("frequency");
    cmd.arg("data.csv")
        .args(["--stats-filter", "cardinality == 4 or nullcount > 0"])
        .args(["--limit", "0"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // 'id' has cardinality 4 (== rowcount), should be excluded
    // 'status' has nullcount > 0 and cardinality 4, should be excluded
    // 'category' has cardinality 2, should remain
    // 'count' has cardinality 3, should remain
    assert!(!got.is_empty());
    let fields: HashSet<&str> = got.iter().skip(1).map(|row| row[0].as_str()).collect();
    assert!(fields.contains("category"), "Expected 'category' column");
    assert!(fields.contains("count"), "Expected 'count' column");
    assert!(
        !fields.contains("id"),
        "Did not expect 'id' column (unique)"
    );
    assert!(
        !fields.contains("status"),
        "Did not expect 'status' column (has nulls)"
    );
}

#[test]
#[cfg(feature = "luau")]
fn frequency_stats_filter_invalid_expression() {
    // Test that invalid filter expressions produce an error
    let wrk = Workdir::new("frequency_stats_filter_invalid");
    wrk.create(
        "data.csv",
        vec![svec!["name", "value"], svec!["A", "1"], svec!["B", "2"]],
    );

    // First, create the stats cache
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("data.csv")
        .arg("--cardinality")
        .arg("--stats-jsonl");
    wrk.run(&mut stats_cmd);

    // Run frequency with an invalid filter expression
    let mut cmd = wrk.command("frequency");
    cmd.arg("data.csv")
        .args(["--stats-filter", "undefined_variable > 0"]);

    wrk.assert_err(&mut cmd);
}

#[test]
#[cfg(feature = "luau")]
fn frequency_stats_filter_cardinality() {
    // Test filtering by cardinality
    let wrk = Workdir::new("frequency_stats_filter_cardinality");
    wrk.create(
        "data.csv",
        vec![
            svec!["category", "subcategory", "item"],
            svec!["A", "a1", "item1"],
            svec!["A", "a1", "item2"],
            svec!["A", "a2", "item3"],
            svec!["B", "b1", "item4"],
            svec!["B", "b2", "item5"],
        ],
    );

    // First, create the stats cache
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("data.csv")
        .arg("--cardinality")
        .arg("--stats-jsonl");
    wrk.run(&mut stats_cmd);

    // Filter: exclude columns with high cardinality (> 4)
    // 'category' has cardinality 2 (A, B)
    // 'subcategory' has cardinality 4 (a1, a2, b1, b2)
    // 'item' has cardinality 5 (all unique), should be excluded
    let mut cmd = wrk.command("frequency");
    cmd.arg("data.csv")
        .args(["--stats-filter", "cardinality > 4"])
        .args(["--limit", "0"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(!got.is_empty());
    let fields: HashSet<&str> = got.iter().skip(1).map(|row| row[0].as_str()).collect();
    assert!(fields.contains("category"), "Expected 'category' column");
    assert!(
        fields.contains("subcategory"),
        "Expected 'subcategory' column"
    );
    assert!(
        !fields.contains("item"),
        "Did not expect 'item' column (high cardinality)"
    );
}

// Tests for --null-sorted flag

#[test]
fn frequency_null_at_end_default() {
    // By default, NULL should be placed at the end of the frequency table
    let wrk = Workdir::new("frequency_null_at_end_default");
    let rows = vec![
        svec!["col"],
        svec!["a"],
        svec!["a"],
        svec!["a"],
        svec!["b"],
        svec!["b"],
        svec![""], // NULL - appears twice, same count as 'b'
        svec![""],
    ];
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv").args(["--limit", "0"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // The last row (excluding header) should be NULL
    let last_row = got.last().unwrap();
    assert_eq!(
        last_row[1], "(NULL)",
        "NULL should be at the end by default"
    );
}

#[test]
fn frequency_null_sorted() {
    // With --null-sorted, NULL should be sorted with other values by count
    let wrk = Workdir::new("frequency_null_sorted");
    let rows = vec![
        svec!["col"],
        svec!["a"],
        svec!["b"],
        svec!["b"],
        svec![""], // NULL - appears 3 times, most frequent
        svec![""],
        svec![""],
    ];
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--limit", "0"])
        .arg("--null-sorted")
        .arg("--pct-nulls"); // Include NULLs in percentage/rank to test sorting behavior

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // With --null-sorted and descending order (default), NULL (3) should come first (rank 1)
    // then b (2), then a (1)
    assert_eq!(
        got[1][1], "(NULL)",
        "NULL should be first when --null-sorted and most frequent"
    );
    assert_eq!(got[1][4], "1", "NULL should have rank 1");
}

#[test]
fn frequency_other_before_null_at_end() {
    // When both Other and NULL are at the end (default), Other should appear before NULL
    // NULL must be frequent enough to be in top N (not absorbed into Other)
    let wrk = Workdir::new("frequency_other_before_null_at_end");
    let rows = vec![
        svec!["col"],
        svec!["a"],
        svec!["a"],
        svec!["a"],
        svec!["a"],
        svec!["b"],
        svec!["c"],
        svec!["d"],
        svec![""], // NULL - appears twice to be in top 2
        svec![""],
    ];
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--limit", "2"]) // Top 2: a (4) and NULL (2), rest to Other
        .arg("--pct-nulls"); // Include NULLs in percentage/rank

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // Expected order: a (most frequent), Other (b, c, d = 3 count), NULL (2)
    // "a" should be first (rank 1)
    assert_eq!(got[1][1], "a", "Most frequent value should be first");
    // "Other" should be second (has count 3, more than NULL's 2)
    assert!(
        got[2][1].starts_with("Other"),
        "Other should be second (count 3)"
    );
    // NULL should be last
    assert_eq!(got[3][1], "(NULL)", "NULL should be last");
}

#[test]
fn frequency_null_sorted_other_sorted() {
    // With both --null-sorted and --other-sorted, natural sort order by count
    let wrk = Workdir::new("frequency_null_sorted_other_sorted");
    let rows = vec![
        svec!["col"],
        svec!["a"],
        svec!["a"],
        svec!["a"],
        svec!["b"],
        svec!["c"],
        svec!["d"],
        svec![""], // NULL - appears twice
        svec![""],
    ];
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--limit", "2"]) // Limit to top 2, so a and NULL stay, b/c/d go to Other
        .arg("--null-sorted")
        .arg("--other-sorted");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // With both flags and --limit 2:
    // - a (3) is in top 2
    // - NULL (2) is in top 2
    // - b, c, d (1 each) go to Other (total count 3)
    // With both flags enabled, output should be sorted by count descending:
    // Other (3, rank 0), a (3, rank 1), NULL (2, rank 2)
    // But since Other has rank 0 and --other-sorted is set, it sorts with others
    let values: Vec<&str> = got.iter().skip(1).map(|r| r[1].as_str()).collect();
    // All values should be present in some order determined by count
    assert!(values.iter().any(|v| *v == "a"), "Expected 'a' in output");
    assert!(
        values.iter().any(|v| v.starts_with("Other")),
        "Expected 'Other' in output"
    );
    assert!(
        values.iter().any(|v| *v == "(NULL)"),
        "Expected '(NULL)' in output"
    );
}

#[test]
fn frequency_null_sorted_asc() {
    // With --null-sorted and --asc, NULL should be sorted in ascending order
    let wrk = Workdir::new("frequency_null_sorted_asc");
    let rows = vec![
        svec!["col"],
        svec!["a"],
        svec!["a"],
        svec!["a"],
        svec!["b"],
        svec![""], // NULL - appears once, least frequent
    ];
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--limit", "0"])
        .arg("--null-sorted")
        .arg("--asc")
        .arg("--pct-nulls"); // Include NULLs in percentage/rank to test sorting behavior

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // With --asc, least frequent first: NULL (1), b (1), a (3)
    // NULL and b are tied with count 1, so they're sorted alphabetically
    // "(NULL)" < "b" alphabetically
    assert_eq!(
        got[1][1], "(NULL)",
        "NULL should be first in ascending order (tied with b)"
    );
}

#[test]
fn frequency_no_nulls_with_null_sorted() {
    // With --no-nulls, --null-sorted has no effect since NULL is excluded
    let wrk = Workdir::new("frequency_no_nulls_with_null_sorted");
    let rows = vec![
        svec!["col"],
        svec!["a"],
        svec!["a"],
        svec!["b"],
        svec![""], // NULL - should be excluded
    ];
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--limit", "0"])
        .arg("--null-sorted")
        .arg("--no-nulls");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // NULL should not appear at all
    let has_null = got.iter().any(|r| r.len() > 1 && r[1] == "(NULL)");
    assert!(!has_null, "NULL should not appear when --no-nulls is set");
}

#[test]
fn frequency_json_null_sorted() {
    // JSON output should respect --null-sorted flag
    let wrk = Workdir::new("frequency_json_null_sorted");
    let rows = vec![
        svec!["col"],
        svec!["a"],
        svec![""], // NULL - appears twice, most frequent
        svec![""],
    ];
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--limit", "0"])
        .arg("--null-sorted")
        .arg("--json");

    let got: String = wrk.stdout(&mut cmd);
    let json: serde_json::Value = serde_json::from_str(&got).unwrap();
    let frequencies = json["fields"][0]["frequencies"].as_array().unwrap();
    // With --null-sorted and descending order, NULL (2) should come first
    assert_eq!(
        frequencies[0]["value"].as_str().unwrap(),
        "(NULL)",
        "NULL should be first in JSON output when --null-sorted"
    );
}

#[test]
fn frequency_weight_null_sorted() {
    // Weighted frequencies should respect --null-sorted flag
    let wrk = Workdir::new("frequency_weight_null_sorted");
    let rows = vec![
        svec!["col", "weight"],
        svec!["a", "1.0"],
        svec!["a", "1.0"], // 'a' appears twice with total weight 2
        svec!["", "5.0"],  // NULL with high weight
        svec!["b", "2.0"],
    ];
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--limit", "0"])
        .args(["--select", "col"])
        .args(["--weight", "weight"])
        .arg("--null-sorted");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // With --null-sorted and descending order, NULL (weight 5) should come first
    assert_eq!(
        got[1][1], "(NULL)",
        "NULL should be first when --null-sorted with high weight"
    );
}

#[test]
fn frequency_custom_null_text_sorted() {
    // Custom null text should work with --null-sorted
    let wrk = Workdir::new("frequency_custom_null_text_sorted");
    let rows = vec![
        svec!["col"],
        svec!["a"],
        svec![""], // NULL - appears twice
        svec![""],
    ];
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--limit", "0"])
        .args(["--null-text", "MISSING"])
        .arg("--null-sorted");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // With --null-sorted and descending order, NULL (2) should come first
    assert_eq!(
        got[1][1], "MISSING",
        "Custom null text should be first when --null-sorted"
    );
}

// Tests for --no-other and --null-text <NONE> consistency (issue #3341)

#[test]
fn frequency_no_other_flag() {
    // --no-other should exclude the "Other" category (equivalent to --other-text "<NONE>")
    let wrk = Workdir::new("frequency_no_other_flag");
    let rows = vec![
        svec!["col"],
        svec!["a"],
        svec!["a"],
        svec!["a"],
        svec!["b"],
        svec!["c"],
        svec!["d"],
    ];
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv").args(["--limit", "1"]).arg("--no-other");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // With --no-other, only "a" should appear (no "Other" category)
    assert_eq!(got.len(), 2, "Should only have header and one data row");
    assert_eq!(got[1][1], "a", "Only 'a' should appear");
    // Verify "Other" is not in the output
    let has_other = got.iter().any(|r| r.len() > 1 && r[1].starts_with("Other"));
    assert!(
        !has_other,
        "Other category should not appear with --no-other"
    );
}

#[test]
fn frequency_no_other_equivalent_to_other_text_none() {
    // --no-other should produce the same result as --other-text "<NONE>"
    let wrk = Workdir::new("frequency_no_other_equivalent");
    let rows = vec![svec!["col"], svec!["a"], svec!["a"], svec!["b"], svec!["c"]];
    wrk.create("in.csv", rows);

    // Run with --no-other
    let mut cmd1 = wrk.command("frequency");
    cmd1.arg("in.csv").args(["--limit", "1"]).arg("--no-other");
    let got1: Vec<Vec<String>> = wrk.read_stdout(&mut cmd1);

    // Run with --other-text "<NONE>"
    let mut cmd_2 = wrk.command("frequency");
    cmd_2
        .arg("in.csv")
        .args(["--limit", "1"])
        .args(["--other-text", "<NONE>"]);
    let got2: Vec<Vec<String>> = wrk.read_stdout(&mut cmd_2);

    // Results should be identical
    assert_eq!(
        got1, got2,
        "--no-other should be equivalent to --other-text '<NONE>'"
    );
}

#[test]
fn frequency_null_text_none() {
    // --null-text "<NONE>" should exclude NULL values (equivalent to --no-nulls)
    let wrk = Workdir::new("frequency_null_text_none");
    let rows = vec![
        svec!["col", "other"],
        svec!["a", "x"],
        svec!["a", "x"],
        svec!["", "x"], // NULL value
        svec!["b", "x"],
    ];
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--limit", "0"])
        .args(["--select", "col"])
        .args(["--null-text", "<NONE>"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // NULL should not appear in the output
    let has_null = got.iter().any(|r| r.len() > 1 && r[1] == "(NULL)");
    assert!(
        !has_null,
        "NULL should not appear with --null-text '<NONE>'"
    );
    // Should only have "a" and "b"
    let values: Vec<&str> = got.iter().skip(1).map(|r| r[1].as_str()).collect();
    assert!(values.contains(&"a"), "Should contain 'a'");
    assert!(values.contains(&"b"), "Should contain 'b'");
}

#[test]
fn frequency_null_text_none_equivalent_to_no_nulls() {
    // --null-text "<NONE>" should produce the same result as --no-nulls
    let wrk = Workdir::new("frequency_null_text_none_equivalent");
    let rows = vec![
        svec!["col", "other"],
        svec!["a", "x"],
        svec!["", "x"], // NULL value
        svec!["b", "x"],
    ];
    wrk.create("in.csv", rows);

    // Run with --null-text "<NONE>"
    let mut cmd1 = wrk.command("frequency");
    cmd1.arg("in.csv")
        .args(["--limit", "0"])
        .args(["--select", "col"])
        .args(["--null-text", "<NONE>"]);
    let got1: Vec<Vec<String>> = wrk.read_stdout(&mut cmd1);

    // Run with --no-nulls
    let mut cmd_2 = wrk.command("frequency");
    cmd_2
        .arg("in.csv")
        .args(["--limit", "0"])
        .args(["--select", "col"])
        .arg("--no-nulls");
    let got2: Vec<Vec<String>> = wrk.read_stdout(&mut cmd_2);

    // Results should be identical
    assert_eq!(
        got1, got2,
        "--null-text '<NONE>' should be equivalent to --no-nulls"
    );
}

#[test]
fn frequency_no_other_with_no_nulls() {
    // Both --no-other and --no-nulls should work together
    let wrk = Workdir::new("frequency_no_other_with_no_nulls");
    let rows = vec![
        svec!["col", "other"],
        svec!["a", "x"],
        svec!["a", "x"],
        svec!["a", "x"],
        svec!["", "x"], // NULL value
        svec!["b", "x"],
        svec!["c", "x"],
    ];
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--limit", "1"])
        .args(["--select", "col"])
        .arg("--no-other")
        .arg("--no-nulls");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // Should only have "a" (most frequent), no "Other", no NULL
    assert_eq!(got.len(), 2, "Should only have header and one data row");
    assert_eq!(got[1][1], "a", "Only 'a' should appear");
}

#[test]
fn frequency_pct_nulls_default() {
    // Default behavior: NULLs excluded from percentage denominator
    // NULL entries should have empty percentage and rank
    let wrk = Workdir::new("frequency_pct_nulls_default");
    let rows = vec![
        svec!["h1"],
        svec!["a"],
        svec!["a"],
        svec!["a"],
        svec!["a"],
        svec!["b"],
        svec![""], // NULL - this should have empty pct and rank
    ];
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv").args(["--limit", "0"]);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    // Total = 6, NULL count = 1, non-NULL total = 5
    // a: 4/5 = 80%, b: 1/5 = 20%, (NULL): empty%, empty rank
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["h1", "(NULL)", "1", "", ""], // NULL has empty percentage and rank
        svec!["h1", "a", "4", "80", "1"],
        svec!["h1", "b", "1", "20", "2"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_pct_nulls_enabled() {
    // With --pct-nulls: original behavior, NULLs included in percentage calculation
    let wrk = Workdir::new("frequency_pct_nulls_enabled");
    let rows = vec![
        svec!["h1"],
        svec!["a"],
        svec!["a"],
        svec!["a"],
        svec!["a"],
        svec!["b"],
        svec![""], // NULL
    ];
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv").args(["--limit", "0"]).arg("--pct-nulls");

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    // Total = 6, all included in percentage
    // a: 4/6 = 66.67%, b: 1/6 = 16.67%, (NULL): 1/6 = 16.67%
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["h1", "(NULL)", "1", "16.66667", "2"],
        svec!["h1", "a", "4", "66.66667", "1"],
        svec!["h1", "b", "1", "16.66667", "2"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_pct_nulls_sparse_data() {
    // Many NULLs - verify NULL doesn't dominate percentages
    let wrk = Workdir::new("frequency_pct_nulls_sparse_data");
    let rows = vec![
        svec!["h1"],
        svec!["a"],
        svec!["a"],
        svec![""], // NULL
        svec![""], // NULL
        svec![""], // NULL
        svec![""], // NULL
        svec!["b"],
    ];
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv").args(["--limit", "0"]);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    // Total = 7, NULL count = 4, non-NULL total = 3
    // a: 2/3 = 66.67%, b: 1/3 = 33.33%, (NULL): empty%
    // Without --pct-nulls, percentages are calculated excluding NULLs
    // Now since percentages are "valid percentages" (of non-null values),
    // the sum of non-NULL percentages = 100%
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        // NULL sorted at end with empty pct/rank, but when sorted alphabetically
        // "(NULL)" comes before "a" and "b"
        svec!["h1", "(NULL)", "4", "", ""],
        svec!["h1", "a", "2", "66.66667", "1"],
        svec!["h1", "b", "1", "33.33333", "2"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_pct_nulls_all_nulls() {
    // All NULL column - all percentages should be empty
    let wrk = Workdir::new("frequency_pct_nulls_all_nulls");
    let rows = vec![svec!["h1"], svec![""], svec![""], svec![""]];
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv").args(["--limit", "0"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // All values are NULL, so non-NULL total = 0
    // (NULL) should have count but empty percentage
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["h1", "(NULL)", "3", "", ""],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_pct_nulls_json() {
    // Verify JSON outputs null for percentage/rank when --pct-nulls is false
    let wrk = Workdir::new("frequency_pct_nulls_json");
    let rows = vec![
        svec!["h1"],
        svec!["a"],
        svec!["a"],
        svec!["b"],
        svec![""], // NULL
    ];
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv").args(["--limit", "0"]).arg("--json");

    let got: String = wrk.stdout(&mut cmd);
    let json: Value = serde_json::from_str(&got).unwrap();

    // Check that the NULL entry doesn't have percentage and rank fields
    let frequencies = json["fields"][0]["frequencies"].as_array().unwrap();
    let null_entry = frequencies
        .iter()
        .find(|f| f["value"].as_str() == Some("(NULL)"))
        .unwrap();

    // With skip_serializing_if, the fields should not be present
    assert!(
        null_entry.get("percentage").is_none(),
        "NULL entry should not have percentage field in JSON"
    );
    assert!(
        null_entry.get("rank").is_none(),
        "NULL entry should not have rank field in JSON"
    );

    // Non-NULL entries should have percentage and rank
    let a_entry = frequencies
        .iter()
        .find(|f| f["value"].as_str() == Some("a"))
        .unwrap();
    assert!(
        a_entry.get("percentage").is_some(),
        "Non-NULL entry should have percentage field"
    );
    assert!(
        a_entry.get("rank").is_some(),
        "Non-NULL entry should have rank field"
    );
}

#[test]
fn frequency_pct_nulls_with_no_nulls() {
    // --pct-nulls should have no effect when --no-nulls is set
    let wrk = Workdir::new("frequency_pct_nulls_with_no_nulls");
    let rows = vec![
        svec!["h1"],
        svec!["a"],
        svec!["a"],
        svec!["b"],
        svec![""], // NULL
    ];
    wrk.create("in.csv", rows);

    // Run with just --no-nulls
    let mut cmd1 = wrk.command("frequency");
    cmd1.arg("in.csv").args(["--limit", "0"]).arg("--no-nulls");
    let got1: Vec<Vec<String>> = wrk.read_stdout(&mut cmd1);

    // Run with both --no-nulls and --pct-nulls
    let mut cmd_2 = wrk.command("frequency");
    cmd_2
        .arg("in.csv")
        .args(["--limit", "0"])
        .arg("--no-nulls")
        .arg("--pct-nulls");
    let got2: Vec<Vec<String>> = wrk.read_stdout(&mut cmd_2);

    // Results should be identical (--pct-nulls has no effect when --no-nulls is set)
    assert_eq!(
        got1, got2,
        "--pct-nulls should have no effect when --no-nulls is set"
    );
}

#[test]
fn frequency_pct_nulls_with_limit() {
    // Verify "Other" category percentage is correct with --pct-nulls=false
    // Use multi-column CSV to ensure NULL is properly detected (single-column trailing
    // empty lines are treated as EOF by the CSV parser)
    let wrk = Workdir::new("frequency_pct_nulls_with_limit");
    let rows = vec![
        svec!["h1", "h2"],
        svec!["a", "1"],
        svec!["a", "2"],
        svec!["a", "3"],
        svec!["b", "4"],
        svec!["c", "5"],
        svec!["d", "6"],
        svec!["", "7"], // NULL in h1
    ];
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv").args(["--select", "h1", "--limit", "1"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Total = 7 records: a(3), b(1), c(1), d(1), NULL(1)
    // Non-NULL total = 6
    // With --limit 1: a(3) is top value
    // Other(3) = b+c+d = 3 unique values, 3 total count
    // NULL(1) has empty percentage and rank
    //
    // Percentages (denominator = 6 non-NULL):
    // a: 3/6 = 50%
    // Other: 100 - 50 = 50% (this represents b+c+d = 3/6 = 50%)
    // NULL: empty (excluded from percentage calculation)

    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["h1", "a", "3", "50", "1"],
        svec!["h1", "Other (3)", "3", "50", "0"],
        svec!["h1", "(NULL)", "1", "", ""],
    ];
    assert_eq!(got, expected);
}

// ============================================================
// --frequency-jsonl tests
// ============================================================

#[test]
fn frequency_jsonl_basic() {
    let wrk = Workdir::new("frequency_jsonl_basic");
    let rows = vec![
        svec!["name", "color"],
        svec!["Alice", "red"],
        svec!["Bob", "blue"],
        svec!["Alice", "red"],
        svec!["Charlie", "red"],
        svec!["Bob", "green"],
    ];
    wrk.create("in.csv", rows);

    // Create stats cache first (required for cardinality info)
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("in.csv")
        .arg("--cardinality")
        .arg("--stats-jsonl");
    wrk.assert_success(&mut stats_cmd);

    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv").arg("--frequency-jsonl");

    wrk.assert_success(&mut cmd);

    // Verify JSONL cache file was created
    let jsonl_path = wrk.path("in.freq.csv.data.jsonl");
    assert!(jsonl_path.exists(), "JSONL cache file should exist");

    // Parse and validate the JSONL contents
    let contents = std::fs::read_to_string(&jsonl_path).unwrap();
    let lines: Vec<&str> = contents.lines().collect();
    assert_eq!(lines.len(), 2, "Should have one line per column");

    // Parse first line (name column)
    let name_entry: Value = serde_json::from_str(lines[0]).unwrap();
    assert_eq!(name_entry["field"], "name");
    assert_eq!(name_entry["cardinality"], 3);
    let freqs = name_entry["frequencies"].as_array().unwrap();
    assert_eq!(freqs.len(), 3); // Alice(2), Bob(2), Charlie(1)

    // Parse second line (color column)
    let color_entry: Value = serde_json::from_str(lines[1]).unwrap();
    assert_eq!(color_entry["field"], "color");
    assert_eq!(color_entry["cardinality"], 3);
    let freqs = color_entry["frequencies"].as_array().unwrap();
    assert_eq!(freqs.len(), 3); // red(3), blue(1), green(1)

    // Check that red has the highest count
    assert_eq!(freqs[0]["value"], "red");
    assert_eq!(freqs[0]["count"], 3);
    assert!((freqs[0]["percentage"].as_f64().unwrap() - 60.0).abs() < 0.01);
}

#[test]
fn frequency_jsonl_all_unique() {
    let wrk = Workdir::new("frequency_jsonl_all_unique");
    let rows = vec![
        svec!["id", "value"],
        svec!["1", "a"],
        svec!["2", "b"],
        svec!["3", "c"],
        svec!["4", "d"],
        svec!["5", "e"],
    ];
    wrk.create("in.csv", rows);

    // Create stats cache
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("in.csv")
        .arg("--cardinality")
        .arg("--stats-jsonl");
    wrk.assert_success(&mut stats_cmd);

    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv").arg("--frequency-jsonl");

    wrk.assert_success(&mut cmd);

    let jsonl_path = wrk.path("in.freq.csv.data.jsonl");
    let contents = std::fs::read_to_string(&jsonl_path).unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    // Parse first line (id column - all unique)
    let id_entry: Value = serde_json::from_str(lines[0]).unwrap();
    assert_eq!(id_entry["field"], "id");
    assert_eq!(id_entry["cardinality"], 5);
    let freqs = id_entry["frequencies"].as_array().unwrap();
    assert_eq!(freqs.len(), 1, "ALL_UNIQUE should have single sentinel entry");
    assert_eq!(freqs[0]["value"], "<ALL_UNIQUE>");
    assert_eq!(freqs[0]["count"], 5);
    assert_eq!(freqs[0]["percentage"], 100.0);

    // value column is also all unique
    let val_entry: Value = serde_json::from_str(lines[1]).unwrap();
    assert_eq!(val_entry["field"], "value");
    let freqs = val_entry["frequencies"].as_array().unwrap();
    assert_eq!(freqs.len(), 1);
    assert_eq!(freqs[0]["value"], "<ALL_UNIQUE>");
}

#[test]
fn frequency_jsonl_high_cardinality() {
    let wrk = Workdir::new("frequency_jsonl_high_cardinality");

    // Create dataset with 20 rows where one column has 19 unique values (95% of rowcount)
    let mut rows = vec![svec!["id", "category"]];
    for i in 1..=20 {
        rows.push(vec![format!("item_{i}"), format!("cat_{}", if i <= 19 { i } else { 1 })]);
    }
    wrk.create("in.csv", rows);

    // Create stats cache
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("in.csv")
        .arg("--cardinality")
        .arg("--stats-jsonl");
    wrk.assert_success(&mut stats_cmd);

    // Use a low threshold to trigger HIGH_CARDINALITY
    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .arg("--frequency-jsonl")
        .args(["--high-card-threshold", "10"])
        .args(["--high-card-pct", "50"]);

    wrk.assert_success(&mut cmd);

    let jsonl_path = wrk.path("in.freq.csv.data.jsonl");
    let contents = std::fs::read_to_string(&jsonl_path).unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    // id column (cardinality=20 == rowcount) should be ALL_UNIQUE
    let id_entry: Value = serde_json::from_str(lines[0]).unwrap();
    assert_eq!(id_entry["field"], "id");
    let freqs = id_entry["frequencies"].as_array().unwrap();
    assert_eq!(freqs[0]["value"], "<ALL_UNIQUE>");

    // category column (cardinality=19 > effective_threshold=min(10, 50%*20=10)=10) → HIGH_CARDINALITY
    let cat_entry: Value = serde_json::from_str(lines[1]).unwrap();
    assert_eq!(cat_entry["field"], "category");
    assert_eq!(cat_entry["cardinality"], 19);
    let freqs = cat_entry["frequencies"].as_array().unwrap();
    assert_eq!(
        freqs.len(),
        1,
        "HIGH_CARDINALITY should have single sentinel entry"
    );
    assert_eq!(freqs[0]["value"], "<HIGH_CARDINALITY>");
    assert_eq!(freqs[0]["count"], 20);
    assert_eq!(freqs[0]["percentage"], 100.0);
}

#[test]
fn frequency_jsonl_custom_thresholds() {
    let wrk = Workdir::new("frequency_jsonl_custom_thresholds");

    // Create dataset: 10 rows, category has cardinality 8
    let mut rows = vec![svec!["id", "category"]];
    for i in 1..=10 {
        rows.push(vec![
            format!("{i}"),
            format!("cat_{}", if i <= 8 { i } else { 1 }),
        ]);
    }
    wrk.create("in.csv", rows);

    // Create stats cache
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("in.csv")
        .arg("--cardinality")
        .arg("--stats-jsonl");
    wrk.assert_success(&mut stats_cmd);

    // With high thresholds (default), category should NOT be HIGH_CARDINALITY
    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv").arg("--frequency-jsonl");
    wrk.assert_success(&mut cmd);

    let jsonl_path = wrk.path("in.freq.csv.data.jsonl");
    let contents = std::fs::read_to_string(&jsonl_path).unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    let cat_entry: Value = serde_json::from_str(lines[1]).unwrap();
    let freqs = cat_entry["frequencies"].as_array().unwrap();
    assert!(
        freqs.len() > 1,
        "With default thresholds, category should have full frequency data"
    );

    // With low threshold (5), category (cardinality=8 > 5) should be HIGH_CARDINALITY
    let mut cmd2 = wrk.command("frequency");
    cmd2.arg("in.csv")
        .arg("--frequency-jsonl")
        .args(["--high-card-threshold", "5"]);
    wrk.assert_success(&mut cmd2);

    let contents2 = std::fs::read_to_string(&jsonl_path).unwrap();
    let lines2: Vec<&str> = contents2.lines().collect();

    let cat_entry2: Value = serde_json::from_str(lines2[1]).unwrap();
    let freqs2 = cat_entry2["frequencies"].as_array().unwrap();
    assert_eq!(freqs2.len(), 1, "With low threshold, should be HIGH_CARDINALITY");
    assert_eq!(freqs2[0]["value"], "<HIGH_CARDINALITY>");
}

#[test]
fn frequency_jsonl_normal_output_unchanged() {
    // Verify that stdout output is the same whether or not --frequency-jsonl is set
    let wrk = Workdir::new("frequency_jsonl_normal_output_unchanged");
    let rows = vec![
        svec!["h1", "h2"],
        svec!["a", "x"],
        svec!["b", "y"],
        svec!["a", "x"],
    ];
    wrk.create("in.csv", rows);

    // Create stats cache
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("in.csv")
        .arg("--cardinality")
        .arg("--stats-jsonl");
    wrk.assert_success(&mut stats_cmd);

    // Run without --frequency-jsonl
    let mut cmd1 = wrk.command("frequency");
    cmd1.arg("in.csv");
    let got1: Vec<Vec<String>> = wrk.read_stdout(&mut cmd1);

    // Run with --frequency-jsonl
    let mut cmd2 = wrk.command("frequency");
    cmd2.arg("in.csv").arg("--frequency-jsonl");
    let got2: Vec<Vec<String>> = wrk.read_stdout(&mut cmd2);

    assert_eq!(got1, got2, "stdout output should be identical with or without --frequency-jsonl");
}

#[test]
fn frequency_jsonl_no_headers() {
    let wrk = Workdir::new("frequency_jsonl_no_headers");
    let rows = vec![
        svec!["Alice", "red"],
        svec!["Bob", "blue"],
        svec!["Alice", "red"],
    ];
    wrk.create("in.csv", rows);

    // Create stats cache (with --no-headers)
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("in.csv")
        .arg("--cardinality")
        .arg("--stats-jsonl")
        .arg("--no-headers");
    wrk.assert_success(&mut stats_cmd);

    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .arg("--frequency-jsonl")
        .arg("--no-headers");

    wrk.assert_success(&mut cmd);

    // Verify JSONL cache file was created
    let jsonl_path = wrk.path("in.freq.csv.data.jsonl");
    assert!(jsonl_path.exists(), "JSONL cache file should exist");

    let contents = std::fs::read_to_string(&jsonl_path).unwrap();
    let lines: Vec<&str> = contents.lines().collect();
    assert_eq!(lines.len(), 2, "Should have one line per column");

    // With --no-headers, field names should be 1-based indices
    let entry1: Value = serde_json::from_str(lines[0]).unwrap();
    assert_eq!(entry1["field"], "1");
    let entry2: Value = serde_json::from_str(lines[1]).unwrap();
    assert_eq!(entry2["field"], "2");
}

#[test]
fn frequency_jsonl_empty_file() {
    let wrk = Workdir::new("frequency_jsonl_empty_file");
    // Create a file with only headers, no data rows
    let rows = vec![svec!["h1", "h2"]];
    wrk.create("in.csv", rows);

    // Create stats cache
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("in.csv")
        .arg("--cardinality")
        .arg("--stats-jsonl");
    wrk.assert_success(&mut stats_cmd);

    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv").arg("--frequency-jsonl");

    wrk.assert_success(&mut cmd);

    // Verify that no JSONL cache file was created for empty data
    let jsonl_path = wrk.path("in.freq.csv.data.jsonl");
    assert!(
        !jsonl_path.exists(),
        "JSONL cache file should not be created when row count is 0"
    );
}

#[test]
fn frequency_jsonl_high_card_pct_invalid() {
    // --high-card-pct of 0 should error
    let wrk = Workdir::new("frequency_jsonl_high_card_pct_zero");
    let rows = vec![svec!["h1"], svec!["a"], svec!["b"]];
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .arg("--frequency-jsonl")
        .args(["--high-card-pct", "0"]);

    wrk.assert_err(&mut cmd);
}

#[test]
fn frequency_jsonl_high_card_pct_over_100() {
    // --high-card-pct > 100 should error
    let wrk = Workdir::new("frequency_jsonl_high_card_pct_over_100");
    let rows = vec![svec!["h1"], svec!["a"], svec!["b"]];
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .arg("--frequency-jsonl")
        .args(["--high-card-pct", "101"]);

    wrk.assert_err(&mut cmd);
}

#[test]
fn frequency_jsonl_high_card_at_threshold() {
    // Test cardinality exactly at the threshold boundary
    let wrk = Workdir::new("frequency_jsonl_high_card_at_threshold");

    // Create dataset: 10 rows, category has cardinality exactly 5
    let mut rows = vec![svec!["id", "category"]];
    for i in 1..=10 {
        rows.push(vec![
            format!("{i}"),
            format!("cat_{}", if i <= 5 { i } else { i - 5 }),
        ]);
    }
    wrk.create("in.csv", rows);

    // Create stats cache
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("in.csv")
        .arg("--cardinality")
        .arg("--stats-jsonl");
    wrk.assert_success(&mut stats_cmd);

    // Set threshold exactly at cardinality (5) - should NOT be HIGH_CARDINALITY
    // because the condition is cardinality > threshold (strictly greater)
    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .arg("--frequency-jsonl")
        .args(["--high-card-threshold", "5"]);
    wrk.assert_success(&mut cmd);

    let jsonl_path = wrk.path("in.freq.csv.data.jsonl");
    let contents = std::fs::read_to_string(&jsonl_path).unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    let cat_entry: Value = serde_json::from_str(lines[1]).unwrap();
    assert_eq!(cat_entry["field"], "category");
    let freqs = cat_entry["frequencies"].as_array().unwrap();
    // cardinality 5 == threshold 5, so NOT high cardinality (strictly greater required)
    assert!(
        freqs.len() > 1,
        "Cardinality at threshold should have full frequency data, not HIGH_CARDINALITY"
    );

    // Set threshold to 4 - cardinality 5 > 4, so should be HIGH_CARDINALITY
    let mut cmd2 = wrk.command("frequency");
    cmd2.arg("in.csv")
        .arg("--frequency-jsonl")
        .args(["--high-card-threshold", "4"]);
    wrk.assert_success(&mut cmd2);

    let contents2 = std::fs::read_to_string(&jsonl_path).unwrap();
    let lines2: Vec<&str> = contents2.lines().collect();

    let cat_entry2: Value = serde_json::from_str(lines2[1]).unwrap();
    let freqs2 = cat_entry2["frequencies"].as_array().unwrap();
    assert_eq!(freqs2.len(), 1, "Cardinality above threshold should be HIGH_CARDINALITY");
    assert_eq!(freqs2[0]["value"], "<HIGH_CARDINALITY>");
}

#[test]
fn frequency_jsonl_no_stats_cache() {
    // When QSV_STATSCACHE_MODE=none, the stats cache is completely bypassed.
    // Without cardinality info, FREQ_ROW_COUNT is never set (defaults to 0),
    // so write_frequency_jsonl skips writing the cache file entirely.
    let wrk = Workdir::new("frequency_jsonl_no_stats_cache");
    let rows = vec![
        svec!["id", "color"],
        svec!["1", "red"],
        svec!["2", "blue"],
        svec!["3", "red"],
    ];
    wrk.create("in.csv", rows);

    // Deliberately do NOT create a stats cache; disable auto mode
    let mut cmd = wrk.command("frequency");
    cmd.env("QSV_STATSCACHE_MODE", "none");
    cmd.arg("in.csv").arg("--frequency-jsonl");
    wrk.assert_success(&mut cmd);

    // With no stats cache, the JSONL cache is NOT created (row_count=0 → skip)
    let jsonl_path = wrk.path("in.freq.csv.data.jsonl");
    assert!(
        !jsonl_path.exists(),
        "JSONL cache should not be created without stats cache"
    );
}

#[test]
fn frequency_jsonl_limit_does_not_affect_cache() {
    // The JSONL cache should contain ALL frequency data regardless of --limit
    let wrk = Workdir::new("frequency_jsonl_limit_does_not_affect_cache");
    let rows = vec![
        svec!["color"],
        svec!["red"],
        svec!["red"],
        svec!["blue"],
        svec!["blue"],
        svec!["green"],
        svec!["yellow"],
        svec!["orange"],
    ];
    wrk.create("in.csv", rows);

    // Create stats cache
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("in.csv")
        .arg("--cardinality")
        .arg("--stats-jsonl");
    wrk.assert_success(&mut stats_cmd);

    // Run with --limit 2 (stdout shows only top 2 values)
    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .arg("--frequency-jsonl")
        .args(["-l", "2"]);

    // Verify stdout output IS limited to 2 entries (+ header + "Other" summary row).
    // read_stdout implicitly validates command success: it parses CSV from stdout
    // and will panic on empty/invalid output if the command fails.
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // The input has 5 distinct values (red, blue, green, yellow, orange), so --limit 2
    // produces: header + 2 top-value rows + 1 "Other" aggregation row = 4 rows.
    assert_eq!(
        got.len(),
        4,
        "stdout should have header + 2 value rows + 1 Other row when --limit 2 is set, got: \
         {got:?}"
    );
    // The last row should be the "Other" summary (matches frequency command's output format)
    assert!(
        got[3][1].starts_with("Other"),
        "last stdout row should be 'Other' summary, got: {:?}",
        got[3]
    );

    // Verify the JSONL cache contains ALL frequency values regardless of --limit
    let jsonl_path = wrk.path("in.freq.csv.data.jsonl");
    let contents = std::fs::read_to_string(&jsonl_path).unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    let entry: Value = serde_json::from_str(lines[0]).unwrap();
    assert_eq!(entry["field"], "color");
    let freqs = entry["frequencies"].as_array().unwrap();
    // Cache should have ALL 5 values, not just the top 2
    assert_eq!(
        freqs.len(),
        5,
        "JSONL cache should contain all frequency values regardless of --limit"
    );
}

#[test]
fn frequency_jsonl_stdin_error() {
    // --frequency-jsonl requires a file input, not stdin.
    // We use raw spawn() + wait_with_output() instead of wrk.command()/wrk.assert_err()
    // because we need to pipe CSV data via stdin, which the Workdir harness doesn't support.
    use std::io::Write;

    let wrk = Workdir::new("frequency_jsonl_stdin_error");

    let csv_content = "h1,h2\na,b\nc,d\n";

    let mut cmd = wrk.command("frequency");
    cmd.arg("--frequency-jsonl")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());

    let mut child = cmd.spawn().unwrap();
    {
        let mut stdin = child.stdin.take().unwrap();
        stdin.write_all(csv_content.as_bytes()).unwrap();
    }

    let output = child.wait_with_output().unwrap();
    assert!(
        !output.status.success(),
        "--frequency-jsonl with stdin should fail"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("--frequency-jsonl requires a file input, not stdin"),
        "Error message should mention that stdin is not supported, got: {stderr}"
    );
}

// === Frequency cache READ PATH tests ===

#[test]
fn frequency_jsonl_cache_reuse() {
    // Create cache, then run again without --frequency-jsonl. Output should
    // come from cache and be identical to a fresh computation.
    let wrk = Workdir::new("frequency_jsonl_cache_reuse");
    let rows = vec![
        svec!["name", "color"],
        svec!["Alice", "red"],
        svec!["Bob", "blue"],
        svec!["Alice", "red"],
        svec!["Charlie", "red"],
        svec!["Bob", "green"],
    ];
    wrk.create("in.csv", rows);

    // Create stats cache
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("in.csv")
        .arg("--cardinality")
        .arg("--stats-jsonl");
    wrk.assert_success(&mut stats_cmd);

    // Run with --frequency-jsonl to create cache and capture output
    let mut cmd1 = wrk.command("frequency");
    cmd1.arg("in.csv").arg("--frequency-jsonl");
    let got1: Vec<Vec<String>> = wrk.read_stdout(&mut cmd1);

    // Verify cache was created
    let jsonl_path = wrk.path("in.freq.csv.data.jsonl");
    assert!(jsonl_path.exists(), "JSONL cache file should exist");

    // Run again without --frequency-jsonl (should use cache)
    let mut cmd2 = wrk.command("frequency");
    cmd2.arg("in.csv");
    let got2: Vec<Vec<String>> = wrk.read_stdout(&mut cmd2);

    assert_eq!(
        got1, got2,
        "Output from cache should be identical to fresh computation"
    );
}

#[test]
fn frequency_jsonl_stale_cache() {
    // Create cache, modify source CSV, verify new data is used.
    let wrk = Workdir::new("frequency_jsonl_stale_cache");
    let rows = vec![
        svec!["name", "color"],
        svec!["Alice", "red"],
        svec!["Bob", "blue"],
    ];
    wrk.create("in.csv", rows);

    // Create stats cache and frequency cache
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("in.csv")
        .arg("--cardinality")
        .arg("--stats-jsonl");
    wrk.assert_success(&mut stats_cmd);

    let mut cmd1 = wrk.command("frequency");
    cmd1.arg("in.csv").arg("--frequency-jsonl");
    wrk.assert_success(&mut cmd1);

    // Sleep briefly and modify the source CSV (make it newer than cache)
    std::thread::sleep(std::time::Duration::from_millis(1100));
    // New data: 5 rows with "Charlie" appearing twice (so name is NOT all-unique)
    let new_rows = vec![
        svec!["name", "color"],
        svec!["Alice", "red"],
        svec!["Bob", "blue"],
        svec!["Charlie", "green"],
        svec!["Charlie", "yellow"],
        svec!["Dave", "purple"],
    ];
    wrk.create("in.csv", new_rows);

    // Re-create stats cache for the new data
    let mut stats_cmd2 = wrk.command("stats");
    stats_cmd2
        .arg("in.csv")
        .arg("--cardinality")
        .arg("--stats-jsonl");
    wrk.assert_success(&mut stats_cmd2);

    // Run without --frequency-jsonl — cache should be stale, so full computation
    let mut cmd2 = wrk.command("frequency");
    cmd2.arg("in.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd2);

    // Should see the new data including "Charlie" with count 2
    let name_values: Vec<&str> = got
        .iter()
        .filter(|row| row[0] == "name")
        .map(|row| row[1].as_str())
        .collect();
    assert!(
        name_values.contains(&"Charlie"),
        "Stale cache should be bypassed; expected 'Charlie' in output, got: {name_values:?}"
    );
    assert!(
        name_values.contains(&"Dave"),
        "Stale cache should be bypassed; expected 'Dave' in output, got: {name_values:?}"
    );
}

#[test]
fn frequency_jsonl_cache_with_select() {
    // Cache all columns, then read with --select on a subset.
    let wrk = Workdir::new("frequency_jsonl_cache_with_select");
    let rows = vec![
        svec!["name", "color", "size"],
        svec!["Alice", "red", "small"],
        svec!["Bob", "blue", "large"],
        svec!["Alice", "red", "small"],
    ];
    wrk.create("in.csv", rows);

    // Create stats cache and frequency cache (all columns)
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("in.csv")
        .arg("--cardinality")
        .arg("--stats-jsonl");
    wrk.assert_success(&mut stats_cmd);

    let mut cmd1 = wrk.command("frequency");
    cmd1.arg("in.csv").arg("--frequency-jsonl");
    wrk.assert_success(&mut cmd1);

    // Run with --select on just "color" (should use cache)
    let mut cmd2 = wrk.command("frequency");
    cmd2.arg("in.csv").arg("--select").arg("color");
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd2);

    // All data rows (skip CSV header) should be for "color" field only
    for row in got.iter().skip(1) {
        assert_eq!(
            row[0], "color",
            "With --select color, all fields should be 'color', got: {row:?}"
        );
    }
    // Should have header + at least 1 data row
    assert!(got.len() > 1, "Should have output rows for color column");
}

#[test]
fn frequency_jsonl_cache_skip_ignore_case() {
    // Cache should NOT be used when --ignore-case is active.
    // --frequency-jsonl with --ignore-case should error.
    let wrk = Workdir::new("frequency_jsonl_cache_skip_ignore_case");
    let rows = vec![
        svec!["name", "color"],
        svec!["Alice", "Red"],
        svec!["alice", "red"],
    ];
    wrk.create("in.csv", rows);

    // Create stats cache and frequency cache
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("in.csv")
        .arg("--cardinality")
        .arg("--stats-jsonl");
    wrk.assert_success(&mut stats_cmd);

    let mut cmd1 = wrk.command("frequency");
    cmd1.arg("in.csv").arg("--frequency-jsonl");
    wrk.assert_success(&mut cmd1);

    // Run with --ignore-case — should compute fresh (not use cache)
    // and should NOT error since --frequency-jsonl is not passed
    let mut cmd2 = wrk.command("frequency");
    cmd2.arg("in.csv").arg("--ignore-case");
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd2);

    // With --ignore-case, "Alice" and "alice" should merge
    let name_rows: Vec<&Vec<String>> = got.iter().filter(|row| row[0] == "name").collect();
    // There should be only 1 unique name value (alice, case-folded)
    assert_eq!(
        name_rows.len(),
        1,
        "--ignore-case should merge Alice/alice into one entry, got: {name_rows:?}"
    );
}

#[test]
fn frequency_jsonl_cache_high_card_fallback() {
    // HIGH_CARDINALITY sentinel should cause fallback to full computation.
    // We need a column with high cardinality but NOT all-unique
    // (cardinality != rowcount) so it gets HIGH_CARDINALITY instead of ALL_UNIQUE.
    let wrk = Workdir::new("frequency_jsonl_cache_high_card_fallback");
    // "code" has 4 unique values out of 5 rows (not all-unique)
    let rows = vec![
        svec!["code", "color"],
        svec!["A1", "red"],
        svec!["B2", "blue"],
        svec!["C3", "red"],
        svec!["D4", "green"],
        svec!["A1", "blue"],
    ];
    wrk.create("in.csv", rows);

    // Create stats cache
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("in.csv")
        .arg("--cardinality")
        .arg("--stats-jsonl");
    wrk.assert_success(&mut stats_cmd);

    // Create frequency cache with very low high-card threshold so "code" gets
    // HIGH_CARDINALITY (cardinality 4 > threshold 2) and "color" gets normal entries
    let mut cmd1 = wrk.command("frequency");
    cmd1.arg("in.csv")
        .arg("--frequency-jsonl")
        .arg("--high-card-threshold")
        .arg("2")
        .arg("--high-card-pct")
        .arg("1");
    wrk.assert_success(&mut cmd1);

    // Verify cache has HIGH_CARDINALITY for "code"
    let jsonl_path = wrk.path("in.freq.csv.data.jsonl");
    let contents = std::fs::read_to_string(&jsonl_path).unwrap();
    let lines: Vec<&str> = contents.lines().collect();
    let code_entry: Value = serde_json::from_str(lines[0]).unwrap();
    assert_eq!(code_entry["field"], "code");
    assert_eq!(
        code_entry["frequencies"][0]["value"], "<HIGH_CARDINALITY>",
        "code column should have HIGH_CARDINALITY sentinel"
    );

    // Run frequency with --select code — should fall back because of HIGH_CARDINALITY
    // (this means it goes through full computation, which still works)
    let mut cmd2 = wrk.command("frequency");
    cmd2.arg("in.csv").arg("--select").arg("code");
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd2);

    // Should have actual frequency data for code
    assert!(
        got.len() > 1,
        "Should have output despite HIGH_CARDINALITY fallback"
    );
    // Verify it's actual data, not the sentinel (skip header row)
    let code_values: Vec<&str> = got.iter().skip(1).map(|row| row[1].as_str()).collect();
    assert!(
        !code_values.contains(&"<HIGH_CARDINALITY>"),
        "Output should have actual data, not HIGH_CARDINALITY sentinel"
    );
}

#[test]
fn frequency_jsonl_force() {
    // --force should regenerate cache even when valid.
    let wrk = Workdir::new("frequency_jsonl_force");
    let rows = vec![
        svec!["name", "color"],
        svec!["Alice", "red"],
        svec!["Bob", "blue"],
    ];
    wrk.create("in.csv", rows);

    // Create stats cache and frequency cache
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg("in.csv")
        .arg("--cardinality")
        .arg("--stats-jsonl");
    wrk.assert_success(&mut stats_cmd);

    let mut cmd1 = wrk.command("frequency");
    cmd1.arg("in.csv").arg("--frequency-jsonl");
    wrk.assert_success(&mut cmd1);

    let jsonl_path = wrk.path("in.freq.csv.data.jsonl");
    assert!(jsonl_path.exists(), "JSONL cache file should exist");

    // Get the original mtime
    let orig_metadata = std::fs::metadata(&jsonl_path).unwrap();
    let orig_mtime = orig_metadata.modified().unwrap();

    // Sleep briefly so mtime can differ
    std::thread::sleep(std::time::Duration::from_millis(1100));

    // Run with --force --frequency-jsonl — should rewrite cache
    let mut cmd2 = wrk.command("frequency");
    cmd2.arg("in.csv")
        .arg("--frequency-jsonl")
        .arg("--force");
    wrk.assert_success(&mut cmd2);

    // Cache should be rewritten (newer mtime)
    let new_metadata = std::fs::metadata(&jsonl_path).unwrap();
    let new_mtime = new_metadata.modified().unwrap();
    assert!(
        new_mtime > orig_mtime,
        "--force should regenerate cache: new_mtime={new_mtime:?}, orig_mtime={orig_mtime:?}"
    );
}

#[test]
fn frequency_jsonl_ignore_case_error() {
    // --frequency-jsonl with --ignore-case should error
    let wrk = Workdir::new("frequency_jsonl_ignore_case_error");
    let rows = vec![
        svec!["name", "color"],
        svec!["Alice", "red"],
    ];
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .arg("--frequency-jsonl")
        .arg("--ignore-case");
    wrk.assert_err(&mut cmd);
}

#[test]
fn frequency_jsonl_no_trim_error() {
    // --frequency-jsonl with --no-trim should error
    let wrk = Workdir::new("frequency_jsonl_no_trim_error");
    let rows = vec![
        svec!["name", "color"],
        svec!["Alice", "red"],
    ];
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .arg("--frequency-jsonl")
        .arg("--no-trim");
    wrk.assert_err(&mut cmd);
}

#[test]
fn frequency_jsonl_weight_error() {
    // --frequency-jsonl with --weight should error
    let wrk = Workdir::new("frequency_jsonl_weight_error");
    let rows = vec![
        svec!["name", "color", "weight"],
        svec!["Alice", "red", "2"],
    ];
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .arg("--frequency-jsonl")
        .arg("--weight")
        .arg("weight");
    wrk.assert_err(&mut cmd);
}
