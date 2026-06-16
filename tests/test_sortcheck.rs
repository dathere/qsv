use crate::workdir::Workdir;

#[test]
fn sortcheck_select_notsorted() {
    let wrk = Workdir::new("sortcheck_select_notsorted");
    wrk.create(
        "in.csv",
        vec![
            svec!["1", "d"],
            svec!["2", "c"],
            svec!["3", "b"],
            svec!["4", "a"],
        ],
    );

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--no-headers")
        .args(["--select", "2"])
        .arg("in.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
fn sortcheck_select_sorted() {
    let wrk = Workdir::new("sortcheck_select_sorted");
    wrk.create(
        "in.csv",
        vec![
            svec!["1", "d"],
            svec!["2", "c"],
            svec!["3", "b"],
            svec!["4", "a"],
        ],
    );

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--no-headers")
        .args(["--select", "1"])
        .arg("in.csv");

    wrk.assert_success(&mut cmd);
}

#[test]
fn sortcheck_select_unsorted() {
    let wrk = Workdir::new("sortcheck_select_sorted");
    wrk.create(
        "in.csv",
        vec![
            svec!["1", "d"],
            svec!["2", "c"],
            svec!["3", "b"],
            svec!["4", "a"],
        ],
    );

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--no-headers")
        .args(["--select", "2"])
        .arg("in.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
fn sortcheck_simple_sorted() {
    let wrk = Workdir::new("sortcheck_simple_sorted");
    wrk.create(
        "in.csv",
        vec![
            svec!["1", "d"],
            svec!["2", "c"],
            svec!["3", "b"],
            svec!["4", "a"],
        ],
    );

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--no-headers").arg("in.csv");

    wrk.assert_success(&mut cmd);
}

#[test]
fn sortcheck_simple_unsorted() {
    let wrk = Workdir::new("sortcheck_simple_unsorted");
    wrk.create(
        "in.csv",
        vec![
            svec!["col11", "col2"],
            svec!["1", "d"],
            svec!["5", "c"],
            svec!["3", "b"],
            svec!["4", "a"],
        ],
    );

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("in.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
fn sortcheck_simple_all() {
    let wrk = Workdir::new("sortcheck_simple_all");
    wrk.create(
        "in.csv",
        vec![
            svec!["col11", "col2"],
            svec!["1", "d"],
            svec!["5", "c"],
            svec!["3", "b"],
            svec!["4", "a"],
            svec!["6", "a"],
            svec!["2", "y"],
            svec!["3", "z"],
        ],
    );

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--all").arg("in.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
fn sortcheck_simple_all_json() {
    let wrk = Workdir::new("sortcheck_simple_all_json");
    wrk.create(
        "in.csv",
        vec![
            svec!["col11", "col2"],
            svec!["1", "d"],
            svec!["5", "c"],
            svec!["5", "c"],
            svec!["3", "b"],
            svec!["4", "a"],
            svec!["6", "a"],
            svec!["6", "a"],
            svec!["2", "y"],
            svec!["3", "z"],
        ],
    );

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--all").arg("--json").arg("in.csv");

    let output = cmd.output().unwrap();
    let got_stdout = std::str::from_utf8(&output.stdout).unwrap_or_default();

    assert_eq!(
        got_stdout,
        r#"{"sorted":false,"record_count":9,"unsorted_breaks":2,"dupe_count":-1}
"#
    );
    wrk.assert_err(&mut cmd);
}

#[test]
fn sortcheck_simple_json() {
    let wrk = Workdir::new("sortcheck_simple_json");
    wrk.create(
        "in.csv",
        vec![
            svec!["col11", "col2"],
            svec!["1", "d"],
            svec!["5", "c"],
            svec!["5", "c"],
            svec!["3", "b"],
            svec!["4", "a"],
            svec!["6", "a"],
            svec!["6", "a"],
            svec!["2", "y"],
            svec!["3", "z"],
        ],
    );

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--json").arg("in.csv");

    let output = cmd.output().unwrap();
    let got_stdout = std::str::from_utf8(&output.stdout).unwrap_or_default();

    assert_eq!(
        got_stdout,
        r#"{"sorted":false,"record_count":9,"unsorted_breaks":2,"dupe_count":-1}
"#
    );
    wrk.assert_err(&mut cmd);
}

#[test]
fn sortcheck_ignore_case_notsorted() {
    let wrk = Workdir::new("sortcheck_ignore_case_notsorted");
    wrk.create(
        "in.csv",
        vec![
            svec!["name"],
            svec!["alpha"],
            svec!["Beta"],
            svec!["gamma"],
            svec!["Delta"],
        ],
    );

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--ignore-case").arg("in.csv");

    wrk.assert_err(&mut cmd);

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("in.csv");
    wrk.assert_err(&mut cmd);
}

#[test]
fn sortcheck_ignore_case_actually_sorted() {
    let wrk = Workdir::new("sortcheck_ignore_case_actually_sorted");
    wrk.create(
        "in.csv",
        vec![
            svec!["name"],
            svec!["alpha"],
            svec!["Beta"],
            svec!["Charlie"],
            svec!["delta"],
        ],
    );

    // Lex sort sees uppercase < lowercase, so this is NOT sorted lex.
    let mut cmd = wrk.command("sortcheck");
    cmd.arg("in.csv");
    wrk.assert_err(&mut cmd);

    // But it IS sorted ignoring case.
    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--ignore-case").arg("in.csv");
    wrk.assert_success(&mut cmd);
}

#[test]
fn sortcheck_empty() {
    let wrk = Workdir::new("sortcheck_empty");
    wrk.create("in.csv", vec![svec!["col1", "col2"]]);

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("in.csv");
    wrk.assert_success(&mut cmd);

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--json").arg("in.csv");
    let output = cmd.output().unwrap();
    let got_stdout = std::str::from_utf8(&output.stdout).unwrap_or_default();
    assert_eq!(
        got_stdout,
        "{\"sorted\":true,\"record_count\":0,\"unsorted_breaks\":0,\"dupe_count\":0}\n"
    );
    wrk.assert_success(&mut cmd);
}

#[test]
fn sortcheck_single_row() {
    let wrk = Workdir::new("sortcheck_single_row");
    wrk.create("in.csv", vec![svec!["col1"], svec!["only"]]);

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("in.csv");
    wrk.assert_success(&mut cmd);
}

#[test]
fn sortcheck_numeric_sorted() {
    let wrk = Workdir::new("sortcheck_numeric_sorted");
    wrk.create(
        "in.csv",
        vec![
            svec!["n"],
            svec!["2"],
            svec!["10"],
            svec!["30"],
            svec!["200"],
        ],
    );

    // Lex sees "10" < "2", so the file is NOT sorted lex.
    let mut cmd = wrk.command("sortcheck");
    cmd.arg("in.csv");
    wrk.assert_err(&mut cmd);

    // But it IS sorted numerically.
    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--numeric").arg("in.csv");
    wrk.assert_success(&mut cmd);
}

#[test]
fn sortcheck_numeric_unsorted() {
    let wrk = Workdir::new("sortcheck_numeric_unsorted");
    wrk.create(
        "in.csv",
        vec![svec!["n"], svec!["2"], svec!["30"], svec!["10"]],
    );

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--numeric").arg("in.csv");
    wrk.assert_err(&mut cmd);
}

#[test]
fn sortcheck_natural_sorted() {
    let wrk = Workdir::new("sortcheck_natural_sorted");
    wrk.create(
        "in.csv",
        vec![
            svec!["item"],
            svec!["item1"],
            svec!["item2"],
            svec!["item10"],
            svec!["item20"],
        ],
    );

    // Lex sees "item10" < "item2", so the file is NOT sorted lex.
    let mut cmd = wrk.command("sortcheck");
    cmd.arg("in.csv");
    wrk.assert_err(&mut cmd);

    // But it IS sorted naturally.
    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--natural").arg("in.csv");
    wrk.assert_success(&mut cmd);
}

#[test]
fn sortcheck_natural_unsorted() {
    let wrk = Workdir::new("sortcheck_natural_unsorted");
    wrk.create(
        "in.csv",
        vec![
            svec!["item"],
            svec!["item1"],
            svec!["item10"],
            svec!["item2"],
        ],
    );

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--natural").arg("in.csv");
    wrk.assert_err(&mut cmd);
}

#[test]
fn sortcheck_natural_ignore_case() {
    let wrk = Workdir::new("sortcheck_natural_ignore_case");
    wrk.create(
        "in.csv",
        vec![
            svec!["item"],
            svec!["Item1"],
            svec!["item2"],
            svec!["ITEM10"],
        ],
    );

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--natural").arg("--ignore-case").arg("in.csv");
    wrk.assert_success(&mut cmd);
}

#[test]
fn sortcheck_natural_overrides_numeric() {
    // Mirrors sort/dedup precedence: --natural beats --numeric. A file that
    // is sorted naturally but NOT numerically should pass when both flags
    // are set.
    let wrk = Workdir::new("sortcheck_natural_overrides_numeric");
    wrk.create(
        "in.csv",
        vec![
            svec!["item"],
            svec!["item1"],
            svec!["item2"],
            svec!["item10"],
        ],
    );

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--natural").arg("--numeric").arg("in.csv");
    wrk.assert_success(&mut cmd);
}

#[test]
fn sortcheck_simple_all_json_progressbar() {
    let wrk = Workdir::new("sortcheck_simple_all_json_progressbar");
    wrk.create(
        "in.csv",
        vec![
            svec!["col11", "col2"],
            svec!["1", "d"],
            svec!["5", "c"],
            svec!["5", "c"],
            svec!["3", "b"],
            svec!["4", "a"],
            svec!["6", "a"],
            svec!["6", "a"],
            svec!["2", "y"],
            svec!["3", "z"],
        ],
    );

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--all")
        .arg("--json")
        .arg("--progressbar")
        .arg("in.csv");

    let output = cmd.output().unwrap();
    let got_stdout = std::str::from_utf8(&output.stdout).unwrap_or_default();

    assert_eq!(
        got_stdout,
        r#"{"sorted":false,"record_count":9,"unsorted_breaks":2,"dupe_count":-1}
"#
    );
    wrk.assert_err(&mut cmd);
}

// ---------------------------------------------------------------------------
// stats-cache awareness (issue #2116)
//
// These tests build a stats cache, then TAMPER its `sort_order` field so the
// cache's answer differs from a genuine scan. That lets us prove the cache was
// consulted (short-circuit fired) vs. a full scan happened (fell through).
// ---------------------------------------------------------------------------

fn build_stats_cache(wrk: &Workdir, csv: &str) {
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg(csv).arg("--stats-jsonl");
    wrk.assert_success(&mut stats_cmd);
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

// genuinely unsorted single String column; the cache (after tamper) claims it
// is Ascending, so a short-circuit returns "sorted" (exit 0) without scanning.
#[test]
fn sortcheck_statscache_shortcircuit() {
    let wrk = Workdir::new("sortcheck_statscache_shortcircuit");
    wrk.create_from_string("in.csv", "name\nbanana\napple\ncherry\n");
    build_stats_cache(&wrk, "in.csv");
    tamper_sort_order(&wrk, "in", "name", "Ascending");

    let mut cmd = wrk.command("sortcheck");
    cmd.args(["--select", "name"]).arg("in.csv");
    // cache says Ascending -> reported sorted even though the file isn't
    wrk.assert_success(&mut cmd);
}

// QSV_STATSCACHE_MODE=none disables the short-circuit -> full scan -> not sorted.
#[test]
fn sortcheck_statscache_optout_none() {
    let wrk = Workdir::new("sortcheck_statscache_optout_none");
    wrk.create_from_string("in.csv", "name\nbanana\napple\ncherry\n");
    build_stats_cache(&wrk, "in.csv");
    tamper_sort_order(&wrk, "in", "name", "Ascending");

    let mut cmd = wrk.command("sortcheck");
    cmd.env("QSV_STATSCACHE_MODE", "none")
        .args(["--select", "name"])
        .arg("in.csv");
    wrk.assert_err(&mut cmd);
}

// --ignore-case has no matching cache semantics -> must NOT short-circuit.
#[test]
fn sortcheck_statscache_ignorecase_no_shortcircuit() {
    let wrk = Workdir::new("sortcheck_statscache_ignorecase_no_shortcircuit");
    wrk.create_from_string("in.csv", "name\nbanana\napple\ncherry\n");
    build_stats_cache(&wrk, "in.csv");
    tamper_sort_order(&wrk, "in", "name", "Ascending");

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--ignore-case")
        .args(["--select", "name"])
        .arg("in.csv");
    wrk.assert_err(&mut cmd);
}

// a per-column cache can't prove multi-column order -> must NOT short-circuit.
#[test]
fn sortcheck_statscache_multicolumn_no_shortcircuit() {
    let wrk = Workdir::new("sortcheck_statscache_multicolumn_no_shortcircuit");
    wrk.create_from_string("in.csv", "c1,c2\nb,2\na,1\nc,3\n");
    build_stats_cache(&wrk, "in.csv");
    tamper_sort_order(&wrk, "in", "c1", "Ascending");
    tamper_sort_order(&wrk, "in", "c2", "Ascending");

    let mut cmd = wrk.command("sortcheck");
    cmd.args(["--select", "c1,c2"]).arg("in.csv");
    wrk.assert_err(&mut cmd);
}

// a column with nulls is excluded from the cache's sort_order computation, so
// even a tampered "Ascending" must NOT short-circuit (nullcount gate).
#[test]
fn sortcheck_statscache_nullcount_guard() {
    let wrk = Workdir::new("sortcheck_statscache_nullcount_guard");
    wrk.create_from_string("in.csv", "id,name\n1,banana\n2,\n3,apple\n");
    build_stats_cache(&wrk, "in.csv");
    tamper_sort_order(&wrk, "in", "name", "Ascending");

    let mut cmd = wrk.command("sortcheck");
    cmd.args(["--select", "name"]).arg("in.csv");
    wrk.assert_err(&mut cmd);
}

// --json always does a full scan for exact counts, ignoring the cache. The
// tampered cache says Ascending but the scan correctly reports sorted:false.
#[test]
fn sortcheck_statscache_json_full_scan() {
    let wrk = Workdir::new("sortcheck_statscache_json_full_scan");
    wrk.create_from_string("in.csv", "name\nbanana\napple\ncherry\n");
    build_stats_cache(&wrk, "in.csv");
    tamper_sort_order(&wrk, "in", "name", "Ascending");

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--json").args(["--select", "name"]).arg("in.csv");

    let output = cmd.output().unwrap();
    let got = std::str::from_utf8(&output.stdout).unwrap_or_default();
    assert!(
        got.contains("\"sorted\":false"),
        "expected full-scan json sorted:false, got: {got}"
    );
}

// genuinely sorted single column with a valid (untampered) cache -> sorted.
#[test]
fn sortcheck_statscache_valid_positive() {
    let wrk = Workdir::new("sortcheck_statscache_valid_positive");
    wrk.create_from_string("in.csv", "name\napple\nbanana\ncherry\n");
    build_stats_cache(&wrk, "in.csv");

    let mut cmd = wrk.command("sortcheck");
    cmd.args(["--select", "name"]).arg("in.csv");
    wrk.assert_success(&mut cmd);
}

// the cache was built WITH headers; a --no-headers run must not reuse it (the
// recorded args metadata mismatches), so it falls through to a full scan.
#[test]
fn sortcheck_statscache_options_mismatch_no_shortcircuit() {
    let wrk = Workdir::new("sortcheck_statscache_options_mismatch_no_shortcircuit");
    wrk.create_from_string("in.csv", "name\nbanana\napple\ncherry\n");
    build_stats_cache(&wrk, "in.csv");
    tamper_sort_order(&wrk, "in", "name", "Ascending");

    let mut cmd = wrk.command("sortcheck");
    // --no-headers makes "name" a data row; the column is not ascending
    cmd.arg("--no-headers")
        .args(["--select", "1"])
        .arg("in.csv");
    wrk.assert_err(&mut cmd);
}

// fail closed when the companion args metadata (<input>.stats.csv.json) is missing.
#[test]
fn sortcheck_statscache_missing_metadata_no_shortcircuit() {
    let wrk = Workdir::new("sortcheck_statscache_missing_metadata_no_shortcircuit");
    wrk.create_from_string("in.csv", "name\nbanana\napple\ncherry\n");
    build_stats_cache(&wrk, "in.csv");
    tamper_sort_order(&wrk, "in", "name", "Ascending");
    std::fs::remove_file(wrk.path("in.stats.csv.json")).unwrap();

    let mut cmd = wrk.command("sortcheck");
    cmd.args(["--select", "name"]).arg("in.csv");
    wrk.assert_err(&mut cmd);
}
