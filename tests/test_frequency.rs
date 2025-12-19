use std::{borrow::ToOwned, collections::hash_map::Entry, process};

use foldhash::{HashMap, HashMapExt};
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
        .arg("--no-headers");

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
    let (wrk, mut cmd) = setup("frequency_nulls");
    cmd.args(["--limit", "0"]).args(["--select", "h1"]);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["h1", "(NULL)", "1", "14.28571", "2"],
        svec!["h1", "(NULL)", "1", "14.28571", "2"],
        svec!["h1", "a", "4", "57.14286", "1"],
        svec!["h1", "b", "1", "14.28571", "2"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_limit() {
    let (wrk, mut cmd) = setup("frequency_limit");
    cmd.args(["--limit", "1"]);

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
    cmd.args(["--limit", "1"]).args(["--pct-dec-places", "3"]);

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
    cmd.args(["--limit", "1"]).args(["--pct-dec-places", "-4"]);

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
    cmd.args(["--limit", "1"]).args(["--other-text", "<NONE>"]);

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
    cmd.args(["--limit", "-4"]);

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
    cmd.args(["--limit", "-4"]).args(["--lmt-threshold", "4"]);

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
    cmd.args(["--limit", "-2"]).args(["--lmt-threshold", "3"]);

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
        .args(["--other-text", "其他"]);

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
        .arg("--other-sorted");

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
        .arg("--other-sorted");

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
        .args(["--other-text", "<NONE>"]);

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
        .arg(testdata);

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
        .arg("--vis-whitespace");

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["header", "value", "8", "44.44444", "1"],
        svec!["header", "(NULL)", "1", "5.55556", "2"],
        svec!["header", "no_whitespace", "1", "5.55556", "2"],
        svec!["header", "value《⍽》", "1", "5.55556", "2"],
        svec!["header", "value《emsp》", "1", "5.55556", "2"],
        svec!["header", "value《figsp》", "1", "5.55556", "2"],
        svec!["header", "value《zwsp》", "1", "5.55556", "2"],
        svec!["header", "《⍽》value", "1", "5.55556", "2"],
        svec!["header", "《emsp》value", "1", "5.55556", "2"],
        svec!["header", "《figsp》value", "1", "5.55556", "2"],
        svec!["header", "《zwsp》value", "1", "5.55556", "2"],
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
        .arg("--ignore-case");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["field", "value", "count", "percentage", "rank"],
        svec!["header", "value", "12", "70.58824", "1"],
        svec!["header", "(NULL)", "1", "5.88235", "2"],
        svec!["header", "no_whitespace", "1", "5.88235", "2"],
        svec!["header", "value《zwsp》", "1", "5.88235", "2"],
        svec!["header", "value《␎》", "1", "5.88235", "2"],
        svec!["header", "value《␏》", "1", "5.88235", "2"],
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
        .arg("--json");
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
    let expected = vec![
        ("a", 4, 50.0, 1.0),
        ("(NULL)", 1, 12.5, 2.0),
        ("(NULL)", 1, 12.5, 2.0),
        ("b", 1, 12.5, 2.0),
        ("h1", 1, 12.5, 2.0),
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
    cmd.args(["--limit", "1"]).arg("--json");
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
        .arg("--json");
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
    let expected = vec![
        ("value", 4, 66.66667),
        ("(NULL)", 1, 16.66667),
        ("no_whitespace", 1, 16.66667),
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
    let expected =
        "input: in.csv\ndescription: \"Generated with `qsv frequency in.csv --limit 0 --select h2 \
         --toon`\"\nrowcount: 7\nfieldcount: 1\nfields[1]:\n  - field: h2\n    type: String\n    \
         cardinality: 4\n    nullcount: 0\n    sparsity: 0\n    uniqueness_ratio: 0.5714\n    \
         stats[10]{name,value}:\n    min,Y\n    max,z\n    sort_order,Unsorted\n    \
         min_length,1\n    max_length,1\n    sum_length,7\n    avg_length,1\n    \
         stddev_length,0\n    variance_length,0\n    cv_length,0\n    \
         frequencies[4]{value,count,percentage,rank}:\n    z,3,42.85714,1\n    y,2,28.57143,2\n    \
         Y,1,14.28571,3\n    x,1,14.28571,3\nrank_strategy: dense"
            .to_string();
    assert_eq!(got, expected);
}

#[test]
fn frequency_toon_no_headers() {
    let (wrk, mut cmd) = setup("frequency_toon_no_headers");
    cmd.args(["--limit", "0"])
        .args(["--select", "1"])
        .arg("--no-headers")
        .arg("--toon");
    let got: String = wrk.stdout(&mut cmd);
    let expected = "input: in.csv\ndescription: \"Generated with `qsv frequency in.csv --limit 0 \
                    --select 1 --no-headers --toon`\"\nrowcount: 8\nfieldcount: 1\nfields[1]:\n  \
                    - field: \"1\"\n    type: \"\"\n    cardinality: 5\n    nullcount: 0\n    \
                    sparsity: 0\n    uniqueness_ratio: 0.625\n    \
                    frequencies[5]{value,count,percentage,rank}:\n    a,4,50,1\n    \
                    (NULL),1,12.5,2\n    (NULL),1,12.5,2\n    b,1,12.5,2\n    \
                    h1,1,12.5,2\nrank_strategy: dense"
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
    let expected = "input: in.csv\ndescription: \"Generated with `qsv frequency in.csv \
                    --ignore-case --limit 0 --select h2 --toon`\"\nrowcount: 7\nfieldcount: \
                    1\nfields[1]:\n  - field: h2\n    type: String\n    cardinality: 3\n    \
                    nullcount: 0\n    sparsity: 0\n    uniqueness_ratio: 0.4286\n    \
                    stats[10]{name,value}:\n    min,Y\n    max,z\n    sort_order,Unsorted\n    \
                    min_length,1\n    max_length,1\n    sum_length,7\n    avg_length,1\n    \
                    stddev_length,0\n    variance_length,0\n    cv_length,0\n    \
                    frequencies[3]{value,count,percentage,rank}:\n    y,3,42.85714,1\n    \
                    z,3,42.85714,1\n    x,1,14.28571,2\nrank_strategy: dense"
        .to_string();
    assert_eq!(got, expected);
}

#[test]
fn frequency_toon_limit() {
    let (wrk, mut cmd) = setup("frequency_toon_limit");
    cmd.args(["--limit", "1"]).arg("--toon");
    let got: String = wrk.stdout(&mut cmd);
    let expected =
        "input: in.csv\ndescription: \"Generated with `qsv frequency in.csv --limit 1 \
         --toon`\"\nrowcount: 7\nfieldcount: 2\nfields[2]:\n  - field: h1\n    type: String\n    \
         cardinality: 4\n    nullcount: 1\n    sparsity: 0.1429\n    uniqueness_ratio: 0.5714\n    \
         stats[10]{name,value}:\n    min,(NULL)\n    max,b\n    sort_order,Unsorted\n    \
         min_length,0\n    max_length,6\n    sum_length,11\n    avg_length,1.5714\n    \
         stddev_length,1.8406\n    variance_length,3.3878\n    cv_length,1.1713\n    \
         frequencies[2]{value,count,percentage,rank}:\n    a,4,57.14286,1\n    Other \
         (3),3,42.85714,0\n  - field: h2\n    type: String\n    cardinality: 4\n    nullcount: \
         0\n    sparsity: 0\n    uniqueness_ratio: 0.5714\n    stats[10]{name,value}:\n    \
         min,Y\n    max,z\n    sort_order,Unsorted\n    min_length,1\n    max_length,1\n    \
         sum_length,7\n    avg_length,1\n    stddev_length,0\n    variance_length,0\n    \
         cv_length,0\n    frequencies[2]{value,count,percentage,rank}:\n    z,3,42.85714,1\n    \
         Other (3),4,57.14286,0\nrank_strategy: dense"
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
    let expected = "rowcount: 100\nfieldcount: 1\nfields[1]:\n  - field: case_enquiry_id\n    \
                    type: Integer\n    cardinality: 100\n    nullcount: 0\n    sparsity: 0\n    \
                    uniqueness_ratio: 1\n    stats[10]{name,value}:\n    sum,10100411645180\n    \
                    min,101004113298\n    max,101004155594\n    range,42296\n    \
                    sort_order,Unsorted\n    mean,101004116451.8\n    sem,790.552\n    \
                    stddev,7905.5202\n    variance,62497248.9352\n    cv,0\n    \
                    frequencies[1]{value,count,percentage,rank}:\n    \
                    <ALL_UNIQUE>,100,100,0\nrank_strategy: dense";
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
        .arg("--toon");
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
    let expected = vec![
        ("value", 4, 66.66667),
        ("(NULL)", 1, 16.66667),
        ("no_whitespace", 1, 16.66667),
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
