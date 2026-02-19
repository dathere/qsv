use crate::workdir::Workdir;

#[test]
fn pragmastat_onesample_basic() {
    let wrk = Workdir::new("pragmastat_onesample_basic");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("pragmastat");
    cmd.arg(&test_file);

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
    cmd.arg("--select")
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
    cmd_default.arg("--select").arg("latitude").arg(&test_file);
    let got_default: Vec<Vec<String>> = wrk.read_stdout(&mut cmd_default);

    // Run with stricter misrate (1e-6)
    let mut cmd_strict = wrk.command("pragmastat");
    cmd_strict
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
    cmd.arg("--select").arg("case_status").arg(&test_file);

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
    cmd.arg(wrk.path("empty.csv").to_str().unwrap());

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
    cmd.arg("--no-headers")
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
