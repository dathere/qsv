use newline_converter::dos2unix;

use crate::workdir::Workdir;

#[test]
fn extsort_linemode() {
    let wrk = Workdir::new("extsort_linemode").flexible(true);
    wrk.clear_contents().unwrap();

    // copy csv file to workdir
    let unsorted_csv = wrk.load_test_resource("adur-public-toilets.csv");
    wrk.create_from_string("adur-public-toilets.csv", &unsorted_csv);

    let mut cmd = wrk.command("extsort");
    cmd.arg("adur-public-toilets.csv")
        .arg("adur-public-toilets-extsort-test.csv");
    wrk.output(&mut cmd);

    // load sorted output
    let sorted_output: String = wrk.from_str(&wrk.path("adur-public-toilets-extsort-test.csv"));

    let expected_csv = wrk.load_test_resource("adur-public-toilets-sorted.csv");
    wrk.create_from_string("adur-public-toilets-sorted.csv", &expected_csv);

    assert_eq!(dos2unix(&sorted_output), dos2unix(&expected_csv));
}

#[test]
fn extsort_csvmode() {
    let wrk = Workdir::new("extsort_csvmode").flexible(true);
    wrk.clear_contents().unwrap();

    // copy csv file to workdir
    let unsorted_csv = wrk.load_test_resource("adur-public-toilets.csv");
    wrk.create_from_string("adur-public-toilets.csv", &unsorted_csv);

    let mut cmd = wrk.command("extsort");
    cmd.env("QSV_AUTOINDEX_SIZE", "1")
        .arg("adur-public-toilets.csv")
        .args(["--select", "OpeningHours,StreetAddress,LocationText"])
        .arg("adur-public-toilets-extsort-csvmode.csv");
    wrk.output(&mut cmd);

    // load sorted output
    let sorted_output: String = wrk.from_str(&wrk.path("adur-public-toilets-extsort-csvmode.csv"));

    let expected_csv = wrk.load_test_resource("adur-public-toilets-extsorted-csvmode.csv");
    wrk.create_from_string("adur-public-toilets-extsorted-csvmode.csv", &expected_csv);

    assert_eq!(dos2unix(&sorted_output), dos2unix(&expected_csv));
}

#[test]
fn extsort_issue_2391() {
    let wrk = Workdir::new("extsort_issue_2391").flexible(true);
    wrk.clear_contents().unwrap();

    let unsorted_csv = wrk.load_test_resource("issue2391-test_ids.csv");
    wrk.create_from_string("issue2391-test_ids.csv", &unsorted_csv);
    // create index
    let mut cmd_wrk = wrk.command("index");
    cmd_wrk.arg("issue2391-test_ids.csv");

    wrk.assert_success(&mut cmd_wrk);

    // as git mangles line endings, we need to convert manually to CRLF as per issue 2391
    // see https://github.com/dathere/qsv/issues/2391
    // convert LF to CRLF in test file to ensure consistent line endings
    #[cfg(target_os = "windows")]
    {
        let mut cmd = wrk.command("cmd");
        cmd.args([
            "/C",
            "type issue2391-test_ids.csv > issue2391-test_ids.tmp.csv && move /Y \
             issue2391-test_ids.tmp.csv issue2391-test_ids.csv",
        ]);
        wrk.output(&mut cmd);
    }
    #[cfg(not(target_os = "windows"))]
    {
        let mut cmd = wrk.command("sh");
        cmd.args([
            "-c",
            "sed 's/$/\r/' issue2391-test_ids.csv > issue2391-test_ids.tmp.csv && mv \
             issue2391-test_ids.tmp.csv issue2391-test_ids.csv",
        ]);
        wrk.output(&mut cmd);
    }

    let mut cmd = wrk.command("extsort");
    cmd.arg("issue2391-test_ids.csv")
        .args(["--select", "tc_id,pnm,pc_id"]);

    wrk.assert_success(&mut cmd);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["pnm", "tc_id", "pc_id"],
        svec!["405", "139280", "9730000630075"],
        svec!["405", "139281", "9730000630075"],
        svec!["252", "139282", "9730000630075"],
        svec!["131", "139282862", "9730065908379"],
        svec!["138", "139282863", "9730065908379"],
        svec!["138", "139282864", "9730065908379"],
        svec!["405", "139282865", "9730065908379"],
        svec!["138", "139282866", "9730065908379"],
        svec!["138", "139282867", "9730065908379"],
        svec!["138", "139282868", "9730065908379"],
        svec!["138", "139282869", "9730065908379"],
        svec!["138", "139282870", "9730065908379"],
        svec!["138", "139282871", "9730065908379"],
        svec!["241", "139283", "9730000630075"],
        svec!["272", "139284", "9730000630075"],
        svec!["273", "139285", "9730000630075"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn extsort_csvmode_no_headers() {
    // Guards the no-headers CSV-mode path: every input row must appear
    // exactly once in the output. Previously, hard-coding position_delta=1
    // duplicated the first row and dropped the last.
    let wrk = Workdir::new("extsort_csvmode_no_headers").flexible(true);
    wrk.clear_contents().unwrap();

    let csv = "9\n2\n5\n1\n7\n4\n8\n3\n6\n";
    wrk.create_from_string("nh.csv", csv);

    let mut idx_cmd = wrk.command("index");
    idx_cmd.arg("nh.csv");
    wrk.assert_success(&mut idx_cmd);

    let mut cmd = wrk.command("extsort");
    cmd.arg("nh.csv")
        .args(["--select", "1"])
        .arg("--no-headers");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["1"],
        svec!["2"],
        svec!["3"],
        svec!["4"],
        svec!["5"],
        svec!["6"],
        svec!["7"],
        svec!["8"],
        svec!["9"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn extsort_csvmode_crlf_no_headers() {
    // Guards CRLF + no-headers + CSV mode: combines the two paths the
    // earlier off-by-one fixes regressed individually.
    let wrk = Workdir::new("extsort_csvmode_crlf_no_headers").flexible(true);
    wrk.clear_contents().unwrap();

    let csv = "9\r\n2\r\n5\r\n1\r\n7\r\n4\r\n8\r\n3\r\n6\r\n";
    wrk.create_from_string("nh_crlf.csv", csv);

    let mut idx_cmd = wrk.command("index");
    idx_cmd.arg("nh_crlf.csv");
    wrk.assert_success(&mut idx_cmd);

    let mut cmd = wrk.command("extsort");
    cmd.arg("nh_crlf.csv")
        .args(["--select", "1"])
        .arg("--no-headers");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["1"],
        svec!["2"],
        svec!["3"],
        svec!["4"],
        svec!["5"],
        svec!["6"],
        svec!["7"],
        svec!["8"],
        svec!["9"],
    ];
    assert_eq!(got, expected);
}

// ---------------------------------------------------------------------------
// stats-cache awareness (issue #2116)
//
// Build a stats cache, TAMPER its `sort_order`, then prove the CSV-mode
// short-circuit either fires (passthrough of the unsorted input) or correctly
// falls through (real external sort).
// ---------------------------------------------------------------------------

fn build_stats_cache(wrk: &Workdir, csv: &str) {
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg(csv).arg("--stats-jsonl");
    wrk.assert_success(&mut stats_cmd);
}

fn build_index(wrk: &Workdir, csv: &str) {
    let mut idx_cmd = wrk.command("index");
    idx_cmd.arg(csv);
    wrk.assert_success(&mut idx_cmd);
}

fn tamper_sort_order(wrk: &Workdir, stem: &str, field: &str, new_order: &str) {
    let path = wrk.path(&format!("{stem}.stats.csv.data.jsonl"));
    let contents = std::fs::read_to_string(&path).unwrap();
    let mut lines = Vec::new();
    let mut found = false;
    for line in contents.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let mut v: serde_json::Value = serde_json::from_str(line).unwrap();
        if v.get("field").and_then(serde_json::Value::as_str) == Some(field) {
            v["sort_order"] = serde_json::Value::String(new_order.to_string());
            found = true;
        }
        lines.push(serde_json::to_string(&v).unwrap());
    }
    assert!(found, "field {field} not found in stats cache");
    std::fs::write(&path, lines.join("\n")).unwrap();
}

// unsorted ASCII single column; tampered cache says Ascending -> passthrough
// (output preserves the unsorted input order, proving the sort was skipped).
#[test]
fn extsort_statscache_passthrough() {
    let wrk = Workdir::new("extsort_statscache_passthrough").flexible(true);
    wrk.create_from_string("in.csv", "name\nbanana\napple\ncherry\n");
    build_stats_cache(&wrk, "in.csv");
    tamper_sort_order(&wrk, "in", "name", "Ascending");
    build_index(&wrk, "in.csv");

    let mut cmd = wrk.command("extsort");
    cmd.args(["--select", "name"]).arg("in.csv").arg("out.csv");
    wrk.assert_success(&mut cmd);

    let got: String = wrk.from_str(&wrk.path("out.csv"));
    assert_eq!(dos2unix(&got), "name\nbanana\napple\ncherry\n");
}

// QSV_STATSCACHE_MODE=none disables the short-circuit -> real external sort.
#[test]
fn extsort_statscache_optout_none() {
    let wrk = Workdir::new("extsort_statscache_optout_none").flexible(true);
    wrk.create_from_string("in.csv", "name\nbanana\napple\ncherry\n");
    build_stats_cache(&wrk, "in.csv");
    tamper_sort_order(&wrk, "in", "name", "Ascending");
    build_index(&wrk, "in.csv");

    let mut cmd = wrk.command("extsort");
    cmd.env("QSV_STATSCACHE_MODE", "none")
        .args(["--select", "name"])
        .arg("in.csv")
        .arg("out.csv");
    wrk.assert_success(&mut cmd);

    let got: String = wrk.from_str(&wrk.path("out.csv"));
    assert_eq!(dos2unix(&got), "name\napple\nbanana\ncherry\n");
}

// a non-ASCII column can't be safely short-circuited (extsort uses lossy UTF-8),
// so even a tampered "Ascending" must fall through to a real sort.
#[test]
fn extsort_statscache_is_ascii_guard() {
    let wrk = Workdir::new("extsort_statscache_is_ascii_guard").flexible(true);
    wrk.create_from_string("in.csv", "name\nbanana\napple\nz\u{00fc}rich\n");
    build_stats_cache(&wrk, "in.csv");
    tamper_sort_order(&wrk, "in", "name", "Ascending");
    build_index(&wrk, "in.csv");

    let mut cmd = wrk.command("extsort");
    cmd.args(["--select", "name"]).arg("in.csv").arg("out.csv");
    wrk.assert_success(&mut cmd);

    let got: String = wrk.from_str(&wrk.path("out.csv"));
    assert_eq!(dos2unix(&got), "name\napple\nbanana\nz\u{00fc}rich\n");
}

// --reverse is never short-circuited (its anti-stable duplicate-key tie-break
// can't be reproduced by a passthrough), so even a cached "Descending" must fall
// through to a real reverse external sort.
#[test]
fn extsort_statscache_reverse_no_shortcircuit() {
    let wrk = Workdir::new("extsort_statscache_reverse_no_shortcircuit").flexible(true);
    wrk.create_from_string("in.csv", "name\nbanana\napple\ncherry\n");
    build_stats_cache(&wrk, "in.csv");
    tamper_sort_order(&wrk, "in", "name", "Descending");
    build_index(&wrk, "in.csv");

    let mut cmd = wrk.command("extsort");
    cmd.arg("--reverse")
        .args(["--select", "name"])
        .arg("in.csv")
        .arg("out.csv");
    wrk.assert_success(&mut cmd);

    // real reverse sort (descending lex), NOT the unsorted input passthrough
    let got: String = wrk.from_str(&wrk.path("out.csv"));
    assert_eq!(dos2unix(&got), "name\ncherry\nbanana\napple\n");
}

// the cache is only valid for the parser options it was generated with: a cache
// built with headers must not be used by a --no-headers run (args metadata guard).
#[test]
fn extsort_statscache_options_mismatch_no_shortcircuit() {
    let wrk = Workdir::new("extsort_statscache_options_mismatch_no_shortcircuit").flexible(true);
    wrk.create_from_string("in.csv", "name\nbanana\napple\ncherry\n");
    build_stats_cache(&wrk, "in.csv"); // generated WITH headers
    tamper_sort_order(&wrk, "in", "name", "Ascending");
    build_index(&wrk, "in.csv");

    let mut cmd = wrk.command("extsort");
    cmd.arg("--no-headers")
        .args(["--select", "1"])
        .arg("in.csv")
        .arg("out.csv");
    wrk.assert_success(&mut cmd);

    // metadata flag_no_headers=false != --no-headers -> fail closed -> real sort
    // (the "name" header is now data and sorts after banana/apple/cherry)
    let got: String = wrk.from_str(&wrk.path("out.csv"));
    assert_eq!(dos2unix(&got), "apple\nbanana\ncherry\nname\n");
}

// multi-column selection can't be proven by a per-column cache -> real sort.
#[test]
fn extsort_statscache_multicolumn_no_shortcircuit() {
    let wrk = Workdir::new("extsort_statscache_multicolumn_no_shortcircuit").flexible(true);
    wrk.create_from_string("in.csv", "c1,c2\nb,2\na,1\nc,3\n");
    build_stats_cache(&wrk, "in.csv");
    tamper_sort_order(&wrk, "in", "c1", "Ascending");
    tamper_sort_order(&wrk, "in", "c2", "Ascending");
    build_index(&wrk, "in.csv");

    let mut cmd = wrk.command("extsort");
    cmd.args(["--select", "c1,c2"]).arg("in.csv").arg("out.csv");
    wrk.assert_success(&mut cmd);

    let got: String = wrk.from_str(&wrk.path("out.csv"));
    assert_eq!(dos2unix(&got), "c1,c2\na,1\nb,2\nc,3\n");
}
