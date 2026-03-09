use crate::workdir::Workdir;

#[test]
fn pragmastat_onesample_basic() {
    let wrk = Workdir::new("pragmastat_onesample_basic");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("pragmastat");
    cmd.arg("--standalone").arg(&test_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Header row
    assert_eq!(
        got[0],
        svec![
            "field",
            "n",
            "center",
            "spread",
            "center_lower",
            "center_upper",
            "spread_lower",
            "spread_upper",
        ]
    );

    // Verify latitude: deterministic values for center and center bounds
    let lat_row = got.iter().find(|r| r[0] == "latitude").unwrap();
    assert_eq!(lat_row[0], "latitude");
    assert_eq!(lat_row[1], "100");
    assert_eq!(lat_row[2], "42.3405"); // center
    assert_eq!(lat_row[3], "0.0259"); // spread
    assert_eq!(lat_row[4], "42.3272"); // center_lower (deterministic)
    assert_eq!(lat_row[5], "42.3503"); // center_upper (deterministic)
    // spread_lower/upper are randomized — verify bounds are valid and straddle spread
    let spread: f64 = lat_row[3].parse().unwrap();
    let spread_lower: f64 = lat_row[6]
        .parse()
        .expect("spread_lower should be non-empty");
    let spread_upper: f64 = lat_row[7]
        .parse()
        .expect("spread_upper should be non-empty");
    assert!(
        spread_lower <= spread,
        "spread_lower ({spread_lower}) should be <= spread ({spread})"
    );
    assert!(
        spread_upper >= spread,
        "spread_upper ({spread_upper}) should be >= spread ({spread})"
    );

    // Verify longitude: deterministic center bounds, randomized spread bounds
    let lon_row = got.iter().find(|r| r[0] == "longitude").unwrap();
    assert_eq!(lon_row[1], "100");
    assert_eq!(lon_row[2], "-71.068"); // center
    assert_eq!(lon_row[3], "0.0249"); // spread
    assert_eq!(lon_row[4], "-71.0814"); // center_lower (deterministic)
    assert_eq!(lon_row[5], "-71.0587"); // center_upper (deterministic)
    let lon_spread: f64 = lon_row[3].parse().unwrap();
    let lon_spread_lower: f64 = lon_row[6]
        .parse()
        .expect("spread_lower should be non-empty");
    let lon_spread_upper: f64 = lon_row[7]
        .parse()
        .expect("spread_upper should be non-empty");
    assert!(
        lon_spread_lower <= lon_spread,
        "spread_lower ({lon_spread_lower}) should be <= spread ({lon_spread})"
    );
    assert!(
        lon_spread_upper >= lon_spread,
        "spread_upper ({lon_spread_upper}) should be >= spread ({lon_spread})"
    );

    // Non-numeric columns should have n=0 and empty estimator cells
    let ontime_row = got.iter().find(|r| r[0] == "ontime").unwrap();
    assert_eq!(ontime_row[1], "0");
    assert!(
        ontime_row[2].is_empty(),
        "center should be empty for non-numeric"
    );
}

#[test]
fn pragmastat_onesample_select() {
    let wrk = Workdir::new("pragmastat_onesample_select");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("pragmastat");
    cmd.arg("--standalone")
        .arg("--select")
        .arg("latitude,longitude")
        .arg(&test_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Header + 2 data rows (latitude, longitude)
    assert_eq!(got.len(), 3);
    assert_eq!(got[1][0], "latitude");
    assert_eq!(got[2][0], "longitude");
}

#[test]
fn pragmastat_onesample_custom_misrate() {
    let wrk = Workdir::new("pragmastat_onesample_custom_misrate");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Run with default misrate (0.001)
    let mut cmd_default = wrk.command("pragmastat");
    cmd_default
        .arg("--standalone")
        .arg("--select")
        .arg("latitude")
        .arg(&test_file);
    let got_default: Vec<Vec<String>> = wrk.read_stdout(&mut cmd_default);

    // Run with stricter misrate (1e-6)
    let mut cmd_strict = wrk.command("pragmastat");
    cmd_strict
        .arg("--standalone")
        .arg("--select")
        .arg("latitude")
        .arg("--misrate")
        .arg("0.000001")
        .arg(&test_file);
    let got_strict: Vec<Vec<String>> = wrk.read_stdout(&mut cmd_strict);

    // center_lower is at index 4, center_upper at index 5
    let default_row = &got_default[1];
    let strict_row = &got_strict[1];

    assert!(
        !default_row[4].is_empty(),
        "center_lower with default misrate"
    );
    assert!(
        !strict_row[4].is_empty(),
        "center_lower with strict misrate"
    );

    // Stricter misrate => wider bounds => lower center_lower, higher center_upper
    let default_lower: f64 = default_row[4].parse().unwrap();
    let strict_lower: f64 = strict_row[4].parse().unwrap();
    let default_upper: f64 = default_row[5].parse().unwrap();
    let strict_upper: f64 = strict_row[5].parse().unwrap();

    assert!(
        strict_lower <= default_lower,
        "stricter misrate should give lower or equal lower bound: {strict_lower} <= \
         {default_lower}"
    );
    assert!(
        strict_upper >= default_upper,
        "stricter misrate should give higher or equal upper bound: {strict_upper} >= \
         {default_upper}"
    );
}

#[test]
fn pragmastat_twosample_basic() {
    let wrk = Workdir::new("pragmastat_twosample_basic");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("pragmastat");
    cmd.arg("--twosample")
        .arg("--select")
        .arg("latitude,longitude")
        .arg(&test_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Header row
    assert_eq!(
        got[0],
        svec![
            "field_x",
            "field_y",
            "n_x",
            "n_y",
            "shift",
            "ratio",
            "disparity",
            "shift_lower",
            "shift_upper",
            "ratio_lower",
            "ratio_upper",
            "disparity_lower",
            "disparity_upper",
        ]
    );

    // Single pair: latitude vs longitude
    assert_eq!(got.len(), 2);
    let row = &got[1];
    assert_eq!(row[0], "latitude");
    assert_eq!(row[1], "longitude");
    assert_eq!(row[2], "100");
    assert_eq!(row[3], "100");
    // estimators
    assert_eq!(row[4], "113.4114"); // shift
    assert!(
        row[5].is_empty(),
        "ratio should be empty (longitude is negative)"
    );
    assert_eq!(row[6], "4465.0157"); // disparity (deterministic)
    // bounds: shift bounds are deterministic
    assert_eq!(row[7], "113.3964"); // shift_lower
    assert_eq!(row[8], "113.4205"); // shift_upper
    // ratio bounds are empty (ratio was empty)
    assert!(row[9].is_empty(), "ratio_lower should be empty");
    assert!(row[10].is_empty(), "ratio_upper should be empty");
    // disparity_lower/upper are randomized — verify bounds are valid and straddle disparity
    let disparity: f64 = row[6].parse().unwrap();
    let disparity_lower: f64 = row[11]
        .parse()
        .expect("disparity_lower should be non-empty");
    let disparity_upper: f64 = row[12]
        .parse()
        .expect("disparity_upper should be non-empty");
    assert!(
        disparity_lower <= disparity,
        "disparity_lower ({disparity_lower}) should be <= disparity ({disparity})"
    );
    assert!(
        disparity_upper >= disparity,
        "disparity_upper ({disparity_upper}) should be >= disparity ({disparity})"
    );
}

#[test]
fn pragmastat_twosample_single_column() {
    let wrk = Workdir::new("pragmastat_twosample_single_column");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("pragmastat");
    cmd.arg("--twosample")
        .arg("--select")
        .arg("latitude")
        .arg(&test_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Only header row; k < 2 guard prevents any pair computation
    assert_eq!(got.len(), 1);
    assert_eq!(got[0][0], "field_x");
}

#[test]
fn pragmastat_twosample_select() {
    let wrk = Workdir::new("pragmastat_twosample_select");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("pragmastat");
    cmd.arg("--twosample")
        .arg("--select")
        .arg("case_enquiry_id,latitude,longitude")
        .arg(&test_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // 3 columns => 3 pairs: (case_enquiry_id, latitude), (case_enquiry_id, longitude), (latitude,
    // longitude)
    assert_eq!(got.len(), 4); // header + 3 pairs
    assert_eq!(got[1][0], "case_enquiry_id");
    assert_eq!(got[1][1], "latitude");
    assert_eq!(got[2][0], "case_enquiry_id");
    assert_eq!(got[2][1], "longitude");
    assert_eq!(got[3][0], "latitude");
    assert_eq!(got[3][1], "longitude");
}

#[test]
fn pragmastat_non_numeric_columns() {
    let wrk = Workdir::new("pragmastat_non_numeric");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("pragmastat");
    cmd.arg("--standalone")
        .arg("--select")
        .arg("case_status")
        .arg(&test_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // case_status is text ("Open"/"Closed") => n=0, all estimators empty
    assert_eq!(got[1][0], "case_status");
    assert_eq!(got[1][1], "0");
    for i in 2..8 {
        assert!(
            got[1][i].is_empty(),
            "column {} should be empty for non-numeric data",
            got[0][i]
        );
    }
}

#[test]
fn pragmastat_empty_input() {
    let wrk = Workdir::new("pragmastat_empty_input");
    wrk.create("empty.csv", vec![svec!["a", "b", "c"]]);

    let mut cmd = wrk.command("pragmastat");
    cmd.arg("--standalone")
        .arg(wrk.path("empty.csv").to_str().unwrap());

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Header + 3 rows (one per column), all with n=0
    assert_eq!(got.len(), 4);
    assert_eq!(got[0][0], "field");
    for row in &got[1..] {
        assert_eq!(row[1], "0");
    }
}

#[test]
fn pragmastat_no_headers() {
    let wrk = Workdir::new("pragmastat_no_headers");
    wrk.create(
        "data.csv",
        vec![
            svec!["1.0", "2.0", "3.0"],
            svec!["4.0", "5.0", "6.0"],
            svec!["7.0", "8.0", "9.0"],
        ],
    );

    let mut cmd = wrk.command("pragmastat");
    cmd.arg("--standalone")
        .arg("--no-headers")
        .arg(wrk.path("data.csv").to_str().unwrap());

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Field names are 1-based column indices (standard qsv --no-headers behavior)
    assert_eq!(got[1][0], "1");
    assert_eq!(got[2][0], "2");
    assert_eq!(got[3][0], "3");

    // All 3 rows included in statistics (first row counted as data)
    assert_eq!(got[1][1], "3");
    assert_eq!(got[2][1], "3");
    assert_eq!(got[3][1], "3");

    // Verify deterministic center: center of [1,4,7]=4, [2,5,8]=5, [3,6,9]=6
    assert_eq!(got[1][2], "4");
    assert_eq!(got[2][2], "5");
    assert_eq!(got[3][2], "6");
}

#[test]
fn pragmastat_twosample_all_columns() {
    let wrk = Workdir::new("pragmastat_twosample_all_columns");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("pragmastat");
    cmd.arg("--twosample").arg(&test_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // boston311-100.csv has 29 columns => C(29,2) = 406 pairs + 1 header
    assert_eq!(got.len(), 407);

    // latitude vs longitude pair should be present with n_x=100, n_y=100
    let lat_lon = got
        .iter()
        .find(|r| r[0] == "latitude" && r[1] == "longitude")
        .unwrap();
    assert_eq!(lat_lon[2], "100");
    assert_eq!(lat_lon[3], "100");
    assert!(!lat_lon[4].is_empty(), "shift should be non-empty");

    // Purely non-numeric pair should have n=0 and empty estimators
    let text_pair = got
        .iter()
        .find(|r| r[0] == "case_status" && r[1] == "closure_reason")
        .unwrap();
    assert_eq!(text_pair[2], "0");
    assert_eq!(text_pair[3], "0");
    assert!(text_pair[4].is_empty(), "shift should be empty for n=0");
}

#[test]
fn pragmastat_invalid_misrate() {
    let wrk = Workdir::new("pragmastat_invalid_misrate");
    wrk.create("data.csv", vec![svec!["a"], svec!["1.0"]]);

    let mut cmd = wrk.command("pragmastat");
    cmd.arg("--misrate")
        .arg("2.0")
        .arg(wrk.path("data.csv").to_str().unwrap());
    wrk.assert_err(&mut cmd);

    let mut cmd = wrk.command("pragmastat");
    cmd.arg("--misrate")
        .arg("-0.5")
        .arg(wrk.path("data.csv").to_str().unwrap());
    wrk.assert_err(&mut cmd);
}

// ---------------------------------------------------------------------------
// Compare1 tests
// ---------------------------------------------------------------------------

#[test]
fn pragmastat_compare1_basic() {
    let wrk = Workdir::new("pragmastat_compare1_basic");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("pragmastat");
    cmd.arg("--compare1")
        .arg("center:42.0")
        .arg("--select")
        .arg("latitude")
        .arg(&test_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Header
    assert_eq!(
        got[0],
        svec![
            "field",
            "n",
            "metric",
            "threshold",
            "estimate",
            "lower",
            "upper",
            "verdict",
        ]
    );

    // One data row
    assert_eq!(got.len(), 2);
    let row = &got[1];
    assert_eq!(row[0], "latitude");
    assert_eq!(row[1], "100");
    assert_eq!(row[2], "center");
    assert_eq!(row[3], "42");
    assert!(!row[4].is_empty(), "estimate should be non-empty");
    assert!(!row[5].is_empty(), "lower should be non-empty");
    assert!(!row[6].is_empty(), "upper should be non-empty");
    // Verdict should be one of the valid values
    assert!(
        row[7] == "less" || row[7] == "greater" || row[7] == "inconclusive",
        "verdict should be less/greater/inconclusive, got: {}",
        row[7]
    );
}

#[test]
fn pragmastat_compare1_multiple_thresholds() {
    let wrk = Workdir::new("pragmastat_compare1_multiple_thresholds");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("pragmastat");
    cmd.arg("--compare1")
        .arg("center:42.0,spread:0.5")
        .arg("--select")
        .arg("latitude")
        .arg(&test_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Header + 2 rows (one per threshold)
    assert_eq!(got.len(), 3);
    assert_eq!(got[1][2], "center");
    assert_eq!(got[2][2], "spread");
}

#[test]
fn pragmastat_compare1_select() {
    let wrk = Workdir::new("pragmastat_compare1_select");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("pragmastat");
    cmd.arg("--compare1")
        .arg("center:42.0")
        .arg("--select")
        .arg("latitude,longitude")
        .arg(&test_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Header + 2 rows (one per column)
    assert_eq!(got.len(), 3);
    assert_eq!(got[1][0], "latitude");
    assert_eq!(got[2][0], "longitude");
}

#[test]
fn pragmastat_compare1_non_numeric() {
    let wrk = Workdir::new("pragmastat_compare1_non_numeric");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("pragmastat");
    cmd.arg("--compare1")
        .arg("center:0")
        .arg("--select")
        .arg("case_status")
        .arg(&test_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    assert_eq!(got.len(), 2);
    assert_eq!(got[1][0], "case_status");
    assert_eq!(got[1][1], "0");
    // estimate, lower, upper should be empty
    assert!(got[1][4].is_empty(), "estimate should be empty for n=0");
    assert!(got[1][5].is_empty(), "lower should be empty for n=0");
    assert!(got[1][6].is_empty(), "upper should be empty for n=0");
    // verdict should be empty
    assert!(got[1][7].is_empty(), "verdict should be empty for n=0");
}

// ---------------------------------------------------------------------------
// Compare2 tests
// ---------------------------------------------------------------------------

#[test]
fn pragmastat_compare2_basic() {
    let wrk = Workdir::new("pragmastat_compare2_basic");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("pragmastat");
    cmd.arg("--compare2")
        .arg("shift:0")
        .arg("--select")
        .arg("latitude,longitude")
        .arg(&test_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Header
    assert_eq!(
        got[0],
        svec![
            "field_x",
            "field_y",
            "n_x",
            "n_y",
            "metric",
            "threshold",
            "estimate",
            "lower",
            "upper",
            "verdict",
        ]
    );

    // One data row (one pair)
    assert_eq!(got.len(), 2);
    let row = &got[1];
    assert_eq!(row[0], "latitude");
    assert_eq!(row[1], "longitude");
    assert_eq!(row[2], "100");
    assert_eq!(row[3], "100");
    assert_eq!(row[4], "shift");
    assert_eq!(row[5], "0");
    assert!(!row[6].is_empty(), "estimate should be non-empty");
    assert!(
        row[9] == "less" || row[9] == "greater" || row[9] == "inconclusive",
        "verdict should be less/greater/inconclusive, got: {}",
        row[9]
    );
}

#[test]
fn pragmastat_compare2_multiple_thresholds() {
    let wrk = Workdir::new("pragmastat_compare2_multiple_thresholds");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("pragmastat");
    cmd.arg("--compare2")
        .arg("shift:0,disparity:0.8")
        .arg("--select")
        .arg("latitude,longitude")
        .arg(&test_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Header + 2 rows (one per threshold for the single pair)
    assert_eq!(got.len(), 3);
    assert_eq!(got[1][4], "shift");
    assert_eq!(got[2][4], "disparity");
}

#[test]
fn pragmastat_compare2_single_column() {
    let wrk = Workdir::new("pragmastat_compare2_single_column");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("pragmastat");
    cmd.arg("--compare2")
        .arg("shift:0")
        .arg("--select")
        .arg("latitude")
        .arg(&test_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Only header row; k < 2 guard prevents any pair computation
    assert_eq!(got.len(), 1);
    assert_eq!(got[0][0], "field_x");
}

// ---------------------------------------------------------------------------
// Error tests for compare
// ---------------------------------------------------------------------------

#[test]
fn pragmastat_compare1_invalid_metric() {
    let wrk = Workdir::new("pragmastat_compare1_invalid_metric");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("pragmastat");
    cmd.arg("--compare1")
        .arg("shift:0")
        .arg("--select")
        .arg("latitude")
        .arg(&test_file);
    wrk.assert_err(&mut cmd);
}

#[test]
fn pragmastat_compare2_invalid_metric() {
    let wrk = Workdir::new("pragmastat_compare2_invalid_metric");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("pragmastat");
    cmd.arg("--compare2")
        .arg("center:42")
        .arg("--select")
        .arg("latitude,longitude")
        .arg(&test_file);
    wrk.assert_err(&mut cmd);
}

#[test]
fn pragmastat_compare_bad_format() {
    let wrk = Workdir::new("pragmastat_compare_bad_format");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("pragmastat");
    cmd.arg("--compare1")
        .arg("center")
        .arg("--select")
        .arg("latitude")
        .arg(&test_file);
    wrk.assert_err(&mut cmd);
}

#[test]
fn pragmastat_mutual_exclusivity() {
    let wrk = Workdir::new("pragmastat_mutual_exclusivity");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // --compare1 + --twosample
    let mut cmd = wrk.command("pragmastat");
    cmd.arg("--compare1")
        .arg("center:0")
        .arg("--twosample")
        .arg("--select")
        .arg("latitude")
        .arg(&test_file);
    wrk.assert_err(&mut cmd);

    // --compare1 + --compare2
    let mut cmd = wrk.command("pragmastat");
    cmd.arg("--compare1")
        .arg("center:0")
        .arg("--compare2")
        .arg("shift:0")
        .arg("--select")
        .arg("latitude")
        .arg(&test_file);
    wrk.assert_err(&mut cmd);

    // --compare2 + --twosample
    let mut cmd = wrk.command("pragmastat");
    cmd.arg("--compare2")
        .arg("shift:0")
        .arg("--twosample")
        .arg("--select")
        .arg("latitude")
        .arg(&test_file);
    wrk.assert_err(&mut cmd);
}

// ---------------------------------------------------------------------------
// Stats cache integration tests
// ---------------------------------------------------------------------------

#[test]
fn pragmastat_stats_cache_filters_nonnumeric() {
    let wrk = Workdir::new("pragmastat_stats_cache_filters");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Generate stats cache
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.args([&test_file, "-E", "--stats-jsonl"]);
    wrk.assert_success(&mut stats_cmd);

    // Run pragmastat without --select; cache should filter to numeric columns only
    let mut cmd = wrk.command("pragmastat");
    cmd.arg("--standalone").arg(&test_file);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // No row should have a non-numeric column like "ontime" or "case_status"
    let fields: Vec<&str> = got.iter().skip(1).map(|r| r[0].as_str()).collect();
    assert!(
        !fields.contains(&"ontime"),
        "non-numeric column 'ontime' should be filtered by stats cache"
    );
    assert!(
        !fields.contains(&"case_status"),
        "non-numeric column 'case_status' should be filtered by stats cache"
    );

    // All rows should have n > 0
    for row in &got[1..] {
        let n: usize = row[1].parse().unwrap();
        assert!(
            n > 0,
            "all columns should be numeric with n > 0, got field={}",
            row[0]
        );
    }

    // latitude and longitude should still be present
    assert!(fields.contains(&"latitude"), "latitude should be present");
    assert!(fields.contains(&"longitude"), "longitude should be present");
}

#[test]
fn pragmastat_stats_cache_ignored_with_select() {
    let wrk = Workdir::new("pragmastat_stats_cache_select");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Generate stats cache
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.args([&test_file, "-E", "--stats-jsonl"]);
    wrk.assert_success(&mut stats_cmd);

    // Run pragmastat with --select including a non-numeric column;
    // explicit selection should bypass cache filtering
    let mut cmd = wrk.command("pragmastat");
    cmd.arg("--standalone")
        .arg("--select")
        .arg("latitude,ontime")
        .arg(&test_file);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Header + 2 data rows
    assert_eq!(got.len(), 3);
    assert_eq!(got[1][0], "latitude");
    assert_eq!(got[1][1], "100");
    assert_eq!(got[2][0], "ontime");
    assert_eq!(got[2][1], "0"); // non-numeric => n=0
}

// ---------------------------------------------------------------------------
// Date/DateTime support tests
// ---------------------------------------------------------------------------

#[test]
fn pragmastat_date_columns_formatted() {
    let wrk = Workdir::new("pragmastat_date_columns");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Generate stats cache WITH date inference so open_dt/closed_dt are typed as DateTime
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.args([&test_file, "-E", "--infer-dates", "--stats-jsonl"]);
    wrk.assert_success(&mut stats_cmd);

    // Run pragmastat selecting a date column (standalone mode for stdout)
    let mut cmd = wrk.command("pragmastat");
    cmd.arg("--standalone")
        .arg("--select")
        .arg("open_dt")
        .arg(&test_file);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    assert_eq!(got.len(), 2); // header + 1 row
    assert_eq!(got[1][0], "open_dt");
    let n: usize = got[1][1].parse().unwrap();
    assert!(n > 0, "date column should have parsed values");

    // center (col 2) should look like an RFC3339 date/datetime
    let center = &got[1][2];
    assert!(
        center.contains('T') || center.contains('-'),
        "center should be a date/datetime string, got: {center}"
    );

    // spread (col 3) should be a numeric days value (no 'T', no '-' prefix)
    let spread = &got[1][3];
    let spread_val: f64 = spread
        .parse()
        .expect("spread should be a numeric days value");
    assert!(spread_val > 0.0, "spread should be positive days");
}

#[test]
fn pragmastat_twosample_date_shift_as_days() {
    let wrk = Workdir::new("pragmastat_date_shift");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Generate stats cache with date inference
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.args([&test_file, "-E", "--infer-dates", "--stats-jsonl"]);
    wrk.assert_success(&mut stats_cmd);

    // Two-sample with two date columns
    let mut cmd = wrk.command("pragmastat");
    cmd.arg("--twosample")
        .arg("--select")
        .arg("open_dt,closed_dt")
        .arg(&test_file);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    assert_eq!(got.len(), 2); // header + 1 pair
    assert_eq!(got[1][0], "open_dt");
    assert_eq!(got[1][1], "closed_dt");

    // shift (col 4) should be numeric days
    let shift = &got[1][4];
    assert!(
        !shift.is_empty(),
        "shift should be non-empty for date columns"
    );
    let _shift_val: f64 = shift.parse().expect("shift should be a numeric days value");

    // disparity (col 6) should be numeric (dimensionless)
    let disparity = &got[1][6];
    if !disparity.is_empty() {
        let _disp_val: f64 = disparity
            .parse()
            .expect("disparity should be a numeric value");
    }
}

#[test]
fn pragmastat_parallel_reading() {
    // Generate a CSV with >10k rows to trigger the indexed parallel reading path
    let wrk = Workdir::new("pragmastat_parallel_reading");
    let mut data = String::from("a,b\n");
    for i in 0..15_000 {
        data.push_str(&format!("{},{}\n", i as f64 * 0.1, (i as f64 * 0.3) + 1.0));
    }
    wrk.create_from_string("data.csv", &data);

    // Build an index so the parallel path is triggered
    let mut idx_cmd = wrk.command("index");
    idx_cmd.arg(wrk.path("data.csv"));
    wrk.run(&mut idx_cmd);

    // Run with --jobs 1 to force single-threaded parallel path (deterministic order)
    let mut cmd = wrk.command("pragmastat");
    cmd.arg("--standalone")
        .arg("--select")
        .arg("a,b")
        .arg("--jobs")
        .arg("1")
        .arg(wrk.path("data.csv"));
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    assert_eq!(got.len(), 3); // header + 2 columns
    assert_eq!(got[1][0], "a");
    assert_eq!(got[1][1], "15000"); // n
    assert_eq!(got[2][0], "b");
    assert_eq!(got[2][1], "15000");

    // Verify center values are reasonable
    let center_a: f64 = got[1][2].parse().expect("center for a should be numeric");
    let center_b: f64 = got[2][2].parse().expect("center for b should be numeric");
    // a ranges 0..1499.9, center should be near 750
    assert!(
        (center_a - 750.0).abs() < 1.0,
        "center_a ({center_a}) should be near 750"
    );
    // b ranges 1..4501, center should be near 2251
    assert!(
        (center_b - 2251.0).abs() < 1.0,
        "center_b ({center_b}) should be near 2251"
    );
}

#[test]
fn pragmastat_parallel_reading_no_headers() {
    // Same test but with --no-headers to verify header handling in parallel path
    let wrk = Workdir::new("pragmastat_parallel_no_headers");
    let mut data = String::new();
    for i in 0..12_000 {
        data.push_str(&format!("{},{}\n", i as f64 * 0.1, (i as f64 * 0.3) + 1.0));
    }
    wrk.create_from_string("data.csv", &data);

    let mut idx_cmd = wrk.command("index");
    idx_cmd.arg(wrk.path("data.csv"));
    wrk.run(&mut idx_cmd);

    let mut cmd = wrk.command("pragmastat");
    cmd.arg("--standalone")
        .arg("--no-headers")
        .arg("--select")
        .arg("1,2")
        .arg("--jobs")
        .arg("1")
        .arg(wrk.path("data.csv"));
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    assert_eq!(got.len(), 3); // header + 2 columns
    assert_eq!(got[1][0], "1"); // column named "1" (no-headers mode)
    assert_eq!(got[1][1], "12000"); // n
    assert_eq!(got[2][0], "2");
    assert_eq!(got[2][1], "12000");
}

// ---------------------------------------------------------------------------
// Cache append tests
// ---------------------------------------------------------------------------

#[test]
fn pragmastat_cache_append_basic() {
    let wrk = Workdir::new("pragmastat_cache_append_basic");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Generate stats cache first
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.args([&test_file, "-E", "--infer-dates", "--stats-jsonl"]);
    wrk.assert_success(&mut stats_cmd);

    // Run pragmastat (default = cache append mode)
    let mut cmd = wrk.command("pragmastat");
    cmd.arg(&test_file);
    wrk.assert_success(&mut cmd);

    // Read the stats CSV and verify ps_* columns were appended
    let stats_csv_path = std::path::Path::new(&test_file)
        .with_extension("")
        .with_file_name(format!(
            "{}.stats.csv",
            std::path::Path::new(&test_file)
                .file_stem()
                .unwrap()
                .to_string_lossy()
        ));
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(&stats_csv_path)
        .unwrap();
    let headers = rdr.headers().unwrap().clone();

    // Check that all 7 ps_* columns exist
    let ps_cols = [
        "ps_n",
        "ps_center",
        "ps_spread",
        "ps_center_lower",
        "ps_center_upper",
        "ps_spread_lower",
        "ps_spread_upper",
    ];
    for col in &ps_cols {
        assert!(
            headers.iter().any(|h| h == *col),
            "Header '{}' not found in stats CSV. Headers: {:?}",
            col,
            headers.iter().collect::<Vec<_>>()
        );
    }

    // Verify latitude has numeric ps_* values
    let records: Vec<csv::StringRecord> = rdr.records().map(|r| r.unwrap()).collect();
    let field_idx = headers.iter().position(|h| h == "field").unwrap();
    let ps_n_idx = headers.iter().position(|h| h == "ps_n").unwrap();
    let ps_center_idx = headers.iter().position(|h| h == "ps_center").unwrap();

    let lat_record = records
        .iter()
        .find(|r| r.get(field_idx) == Some("latitude"))
        .unwrap();
    let lat_n: usize = lat_record.get(ps_n_idx).unwrap().parse().unwrap();
    assert_eq!(lat_n, 100, "latitude should have ps_n=100");
    let lat_center: f64 = lat_record.get(ps_center_idx).unwrap().parse().unwrap();
    assert!(
        (lat_center - 42.3405).abs() < 0.001,
        "latitude ps_center should be ~42.3405, got {lat_center}"
    );

    // Verify non-numeric column has empty ps_* fields
    let type_idx = headers.iter().position(|h| h == "type").unwrap();
    let string_record = records
        .iter()
        .find(|r| r.get(type_idx) == Some("String"))
        .unwrap();
    assert!(
        string_record.get(ps_n_idx).unwrap().is_empty(),
        "String column should have empty ps_n"
    );
    assert!(
        string_record.get(ps_center_idx).unwrap().is_empty(),
        "String column should have empty ps_center"
    );
}

#[test]
fn pragmastat_cache_append_auto_stats() {
    let wrk = Workdir::new("pragmastat_cache_append_auto_stats");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Do NOT generate stats cache first — pragmastat should auto-generate it
    let mut cmd = wrk.command("pragmastat");
    cmd.arg(&test_file);
    wrk.assert_success(&mut cmd);

    // Verify the stats CSV was created with ps_* columns
    let stats_csv_path = std::path::Path::new(&test_file)
        .with_extension("")
        .with_file_name(format!(
            "{}.stats.csv",
            std::path::Path::new(&test_file)
                .file_stem()
                .unwrap()
                .to_string_lossy()
        ));
    assert!(
        stats_csv_path.exists(),
        "Stats CSV should have been auto-generated"
    );

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(&stats_csv_path)
        .unwrap();
    let headers = rdr.headers().unwrap().clone();
    assert!(
        headers.iter().any(|h| h == "ps_center"),
        "ps_center should exist in auto-generated stats CSV"
    );
}

#[test]
fn pragmastat_cache_append_idempotent() {
    let wrk = Workdir::new("pragmastat_cache_append_idempotent");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Generate stats cache
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.args([&test_file, "-E", "--stats-jsonl"]);
    wrk.assert_success(&mut stats_cmd);

    // First run
    let mut cmd1 = wrk.command("pragmastat");
    cmd1.arg(&test_file);
    wrk.assert_success(&mut cmd1);

    // Read the stats CSV after first run
    let stats_csv_path = std::path::Path::new(&test_file)
        .with_extension("")
        .with_file_name(format!(
            "{}.stats.csv",
            std::path::Path::new(&test_file)
                .file_stem()
                .unwrap()
                .to_string_lossy()
        ));
    let content1 = std::fs::read_to_string(&stats_csv_path).unwrap();

    // Second run — should skip (ps_* already present)
    let mut cmd2 = wrk.command("pragmastat");
    cmd2.arg(&test_file);
    wrk.assert_success(&mut cmd2);

    let content2 = std::fs::read_to_string(&stats_csv_path).unwrap();
    assert_eq!(content1, content2, "Second run should not modify the file");
}

#[test]
fn pragmastat_cache_append_force() {
    let wrk = Workdir::new("pragmastat_cache_append_force");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Generate stats cache
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.args([&test_file, "-E", "--stats-jsonl"]);
    wrk.assert_success(&mut stats_cmd);

    // First run
    let mut cmd1 = wrk.command("pragmastat");
    cmd1.arg(&test_file);
    wrk.assert_success(&mut cmd1);

    // Force recompute — should succeed and rewrite the file
    let mut cmd2 = wrk.command("pragmastat");
    cmd2.arg("--force").arg(&test_file);
    wrk.assert_success(&mut cmd2);

    // Verify ps_* columns still exist and there's no duplication
    let stats_csv_path = std::path::Path::new(&test_file)
        .with_extension("")
        .with_file_name(format!(
            "{}.stats.csv",
            std::path::Path::new(&test_file)
                .file_stem()
                .unwrap()
                .to_string_lossy()
        ));
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(&stats_csv_path)
        .unwrap();
    let headers = rdr.headers().unwrap().clone();

    // Count ps_center occurrences — should be exactly 1
    let ps_center_count = headers.iter().filter(|h| *h == "ps_center").count();
    assert_eq!(
        ps_center_count, 1,
        "ps_center should appear exactly once after --force"
    );
}

#[test]
fn pragmastat_standalone_flag() {
    let wrk = Workdir::new("pragmastat_standalone_flag");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Run with --standalone — should produce old standalone CSV output
    let mut cmd = wrk.command("pragmastat");
    cmd.arg("--standalone")
        .arg("--select")
        .arg("latitude")
        .arg(&test_file);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Standalone output has the old header format
    assert_eq!(
        got[0],
        svec![
            "field",
            "n",
            "center",
            "spread",
            "center_lower",
            "center_upper",
            "spread_lower",
            "spread_upper",
        ]
    );
    assert_eq!(got[1][0], "latitude");
    assert_eq!(got[1][1], "100");
}

#[test]
fn pragmastat_cache_append_select() {
    let wrk = Workdir::new("pragmastat_cache_append_select");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Generate stats cache
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.args([&test_file, "-E", "--stats-jsonl"]);
    wrk.assert_success(&mut stats_cmd);

    // Run pragmastat with --select for only latitude
    let mut cmd = wrk.command("pragmastat");
    cmd.arg("--select").arg("latitude").arg(&test_file);
    wrk.assert_success(&mut cmd);

    // Read stats CSV — latitude should have ps_* values, longitude should be empty
    let stats_csv_path = std::path::Path::new(&test_file)
        .with_extension("")
        .with_file_name(format!(
            "{}.stats.csv",
            std::path::Path::new(&test_file)
                .file_stem()
                .unwrap()
                .to_string_lossy()
        ));
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(&stats_csv_path)
        .unwrap();
    let headers = rdr.headers().unwrap().clone();
    let records: Vec<csv::StringRecord> = rdr.records().map(|r| r.unwrap()).collect();

    let field_idx = headers.iter().position(|h| h == "field").unwrap();
    let ps_n_idx = headers.iter().position(|h| h == "ps_n").unwrap();

    let lat = records
        .iter()
        .find(|r| r.get(field_idx) == Some("latitude"))
        .unwrap();
    assert_eq!(
        lat.get(ps_n_idx).unwrap(),
        "100",
        "latitude should have ps_n=100"
    );

    let lon = records
        .iter()
        .find(|r| r.get(field_idx) == Some("longitude"))
        .unwrap();
    assert!(
        lon.get(ps_n_idx).unwrap().is_empty(),
        "longitude should have empty ps_n since it was not selected"
    );
}

#[test]
fn pragmastat_cache_with_moarstats() {
    let wrk = Workdir::new("pragmastat_cache_with_moarstats");
    let test_file = wrk.load_test_file("boston311-100.csv");

    // Generate stats cache
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.args([&test_file, "-E", "--infer-dates", "--stats-jsonl"]);
    wrk.assert_success(&mut stats_cmd);

    // Run moarstats first
    let mut moarstats_cmd = wrk.command("moarstats");
    moarstats_cmd.arg(&test_file);
    wrk.assert_success(&mut moarstats_cmd);

    // Then run pragmastat — should coexist with moarstats columns
    let mut cmd = wrk.command("pragmastat");
    cmd.arg(&test_file);
    wrk.assert_success(&mut cmd);

    // Read stats CSV — verify both moarstats and pragmastat columns exist
    let stats_csv_path = std::path::Path::new(&test_file)
        .with_extension("")
        .with_file_name(format!(
            "{}.stats.csv",
            std::path::Path::new(&test_file)
                .file_stem()
                .unwrap()
                .to_string_lossy()
        ));
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(&stats_csv_path)
        .unwrap();
    let headers = rdr.headers().unwrap().clone();

    // moarstats adds columns like "pearson_skewness"
    assert!(
        headers.iter().any(|h| h == "pearson_skewness"),
        "moarstats pearson_skewness column should still be present"
    );
    assert!(
        headers.iter().any(|h| h == "ps_center"),
        "pragmastat ps_center column should be present"
    );
}
