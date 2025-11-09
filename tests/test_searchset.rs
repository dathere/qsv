use newline_converter::dos2unix;

use crate::workdir::Workdir;

fn data(headers: bool) -> Vec<Vec<String>> {
    let mut rows = vec![
        svec!["foobar", "barfoo"],
        svec!["a", "b"],
        svec!["barfoo", "foobar"],
        svec!["is waldo here", "spot"],
        svec!["Ḟooƀar", "ḃarḟoo"],
        svec!["bleh", "no, Waldo is there"],
    ];
    if headers {
        rows.insert(0, svec!["h1", "h2"]);
    }
    rows
}

fn data_with_regex_chars(headers: bool) -> Vec<Vec<String>> {
    let mut rows = vec![
        svec!["foo$bar^", "barfoo"],
        svec!["a", "b"],
        svec!["$bar^foo", "foobar"],
        svec!["is wal[do] here", "spot"],
        svec!["Ḟooƀar", "$ḃar^ḟoo"],
        svec!["bleh", "no, Wal[do] is there"],
    ];
    if headers {
        rows.insert(0, svec!["h1", "h2"]);
    }
    rows
}

fn regexset_file() -> Vec<Vec<String>> {
    let rows = vec![svec!["^foo"], svec!["bar$"], svec!["waldo"]];
    rows
}

fn regexset_literal_file() -> Vec<Vec<String>> {
    let rows = vec![svec!["$bar^"], svec!["[do]"]];
    rows
}

fn regexset_exact_file() -> Vec<Vec<String>> {
    let rows = vec![svec!["foo$bar^"], svec!["is wal[do] here"]];
    rows
}

fn data_with_dots(headers: bool) -> Vec<Vec<String>> {
    let mut rows = vec![
        svec!["1", "JM Bloggs"],
        svec!["2", "F. J. Bloggs"],
        svec!["3", "J. Bloggs"],
    ];
    if headers {
        rows.insert(0, svec!["id", "name"]);
    }
    rows
}

fn regexset_exact_dots_file() -> Vec<Vec<String>> {
    let rows = vec![svec!["J. Bloggs"]];
    rows
}

fn regexset_no_match_file() -> Vec<Vec<String>> {
    let rows = vec![svec!["^blah"], svec!["bloop$"], svec!["joel"]];
    rows
}

fn regexset_unicode_file() -> Vec<Vec<String>> {
    let rows = vec![svec!["^foo"], svec!["bar$"], svec!["waldo"], svec!["^Ḟoo"]];
    rows
}

fn empty_regexset_file() -> Vec<Vec<String>> {
    let rows = vec![svec![""]];
    rows
}

#[test]
fn searchset() {
    let wrk = Workdir::new("searchset");
    wrk.create("data.csv", data(true));
    wrk.create("regexset.txt", regexset_file());
    let mut cmd = wrk.command("searchset");
    cmd.arg("regexset.txt").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["h1", "h2"],
        svec!["foobar", "barfoo"],
        svec!["barfoo", "foobar"],
        svec!["is waldo here", "spot"],
    ];
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);
}

#[test]
fn searchset_indexed_parallel() {
    let wrk = Workdir::new("searchset_indexed_parallel");
    let data = wrk.load_test_resource("boston311-100.csv");
    wrk.create_from_string("data.csv", &data);
    wrk.create_from_string("regexset.txt", "Brighton\nMission Hill");
    let mut cmd = wrk.command("searchset");
    cmd.arg("regexset.txt").arg("data.csv");
    wrk.assert_success(&mut cmd);

    let got: String = wrk.stdout(&mut cmd);
    let expected = "case_enquiry_id,open_dt,target_dt,closed_dt,ontime,case_status,closure_reason,case_title,subject,reason,type,queue,department,submittedphoto,closedphoto,location,fire_district,pwd_district,city_council_district,police_district,neighborhood,neighborhood_services_district,ward,precinct,location_street_name,location_zipcode,latitude,longitude,source\n101004113747,2022-01-01 23:46:09,2022-01-17 08:30:00,2022-01-02 11:03:10,ONTIME,Closed,Case Closed. Closed date : Sun Jan 02 11:03:10 EST 2022 Noted Case noted. Duplicate case. Posts already marked for contractor to repair.  ,Street Light Outages,Public Works Department,Street Lights,Street Light Outages,PWDx_Street Light Outages,PWDx,https://311.boston.gov/media/boston/report/photos/61d12e0705bbcf180c29cfc2/report.jpg,,103 N Beacon St  Brighton  MA  02135,11,04,9,D14,Brighton,15,22,2205,103 N Beacon St,02135,42.3549,-71.143,Citizens Connect App\n101004113751,2022-01-01 23:53:44,2022-03-07 08:30:00,,OVERDUE,Open, ,Graffiti Removal,Property Management,Graffiti,Graffiti Removal,PROP_GRAF_GraffitiRemoval,PROP,https://311.boston.gov/media/boston/report/photos/61d12fc905bbcf180c29d11e/report.jpg,,1270 Commonwealth Ave  Allston  MA  02134,11,04,9,D14,Allston / Brighton,15,Ward 21,2105,1270 Commonwealth Ave,02134,42.3492,-71.1325,Citizens Connect App\n101004114593,2022-01-03 09:58:36,2022-01-04 09:58:36,2022-01-03 11:58:53,ONTIME,Closed,Case Closed. Closed date : Mon Jan 03 11:58:53 EST 2022 Resolved Picked up.  ,Pick up Dead Animal,Public Works Department,Street Cleaning,Pick up Dead Animal,PWDx_District 04: Allston/Brighton,PWDx,,,INTERSECTION of Greymere Rd & Washington St  Brighton  MA  ,11,04,8,D14,Allston / Brighton,15,22,2210,INTERSECTION Greymere Rd & Washington St,,42.3594,-71.0587,Citizens Connect App\n101004114624,2022-01-03 10:12:00,2022-05-03 10:12:36,2022-01-13 14:12:46,ONTIME,Closed,Case Closed. Closed date : Thu Jan 13 14:12:46 EST 2022 Noted Violations found. Notice written. ,SCHEDULED Pest Infestation - Residential,Inspectional Services,Housing,Pest Infestation - Residential,ISD_Housing (INTERNAL),ISD,,,20 Washington St  Brighton  MA  02135,11,04,9,D14,Allston / Brighton,15,Ward 21,2112,20 Washington St,02135,42.3425,-71.1412,Constituent Call\n101004114724,2022-01-03 11:36:21,,2022-01-04 16:31:31,ONTIME,Closed,Case Closed. Closed date : 2022-01-04 16:31:31.297 Bulk Item Automation ,Schedule Bulk Item Pickup,Public Works Department,Sanitation,Schedule a Bulk Item Pickup SS,PWDx_Schedule a Bulk Item Pickup,PWDx,,,352 Riverway  Boston  MA  02115,4,10A,8,B2,Mission Hill,14,Ward 10,1004,352 Riverway,02115,42.3335,-71.1113,Self Service\n101004115369,2022-01-04 06:15:33,2022-01-05 08:30:00,2022-01-04 10:00:57,ONTIME,Closed,Case Closed. Closed date : 2022-01-04 10:00:57.823 Case Noted ,Parking Enforcement,Transportation - Traffic Division,Enforcement & Abandoned Vehicles,Parking Enforcement,BTDT_Parking Enforcement,BTDT,https://311.boston.gov/media/boston/report/photos/61d42c4905bbcf180c2b73f2/report.jpg,,14 Wiltshire Rd  Brighton  MA  02135,11,04,9,D14,Allston / Brighton,15,Ward 22,2209,14 Wiltshire Rd,02135,42.3434,-71.1546,Citizens Connect App";
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);

    // now index the file
    let mut cmd = wrk.command("index");
    cmd.arg("data.csv");
    wrk.assert_success(&mut cmd);

    // should still have the same output
    let mut cmd = wrk.command("searchset");
    cmd.arg("regexset.txt").arg("data.csv");
    wrk.assert_success(&mut cmd);

    let got: String = wrk.stdout(&mut cmd);
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);
}

#[test]
fn searchset_match() {
    let wrk = Workdir::new("searchset_match");
    wrk.create("data.csv", data(true));
    wrk.create("regexset.txt", regexset_file());
    let mut cmd = wrk.command("searchset");
    cmd.arg("regexset.txt").arg("data.csv");

    wrk.assert_success(&mut cmd);

    wrk.assert_success(&mut cmd);
}

#[test]
fn searchset_match_count() {
    let wrk = Workdir::new("searchset_match");
    wrk.create("data.csv", data(true));
    wrk.create("regexset.txt", regexset_file());
    let mut cmd = wrk.command("searchset");
    cmd.arg("regexset.txt").arg("--count").arg("data.csv");

    wrk.assert_success(&mut cmd);

    let got = wrk.output_stderr(&mut cmd);
    let expected = "3\n";
    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd);
}

#[test]
fn searchset_quick() {
    let wrk = Workdir::new("searchset_quick");
    wrk.create("data.csv", data(true));
    wrk.create("regexset.txt", regexset_file());
    let mut cmd = wrk.command("searchset");
    cmd.arg("regexset.txt").arg("--quick").arg("data.csv");

    let got_err = wrk.output_stderr(&mut cmd);
    assert_eq!(got_err, "1\n");
    wrk.assert_success(&mut cmd);
    let got: String = wrk.stdout(&mut cmd);
    assert_eq!(got, "");
}

#[test]
fn searchset_nomatch() {
    let wrk = Workdir::new("searchset_nomatch");
    wrk.create("data.csv", data(true));
    wrk.create("regexset.txt", regexset_no_match_file());
    let mut cmd = wrk.command("searchset");
    cmd.arg("regexset.txt").arg("data.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
fn searchset_quick_nomatch() {
    let wrk = Workdir::new("searchset_quick_nomatch");
    wrk.create("data.csv", data(true));
    wrk.create("regexset.txt", regexset_no_match_file());
    let mut cmd = wrk.command("searchset");
    cmd.arg("regexset.txt").arg("--quick").arg("data.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
fn searchset_unicode() {
    let wrk = Workdir::new("searchset");
    wrk.create("data.csv", data(true));
    wrk.create("regexset_unicode.txt", regexset_unicode_file());
    let mut cmd = wrk.command("searchset");
    cmd.arg("regexset_unicode.txt").arg("data.csv");
    cmd.arg("--unicode");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["h1", "h2"],
        svec!["foobar", "barfoo"],
        svec!["barfoo", "foobar"],
        svec!["is waldo here", "spot"],
        svec!["Ḟooƀar", "ḃarḟoo"],
    ];
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);
}

#[test]
fn searchset_unicode_envvar() {
    let wrk = Workdir::new("searchset");
    wrk.create("data.csv", data(true));
    wrk.create("regexset_unicode.txt", regexset_unicode_file());
    let mut cmd = wrk.command("searchset");
    cmd.env("QSV_REGEX_UNICODE", "1");
    cmd.arg("regexset_unicode.txt").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["h1", "h2"],
        svec!["foobar", "barfoo"],
        svec!["barfoo", "foobar"],
        svec!["is waldo here", "spot"],
        svec!["Ḟooƀar", "ḃarḟoo"],
    ];
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);
}

#[test]
fn searchset_empty() {
    let wrk = Workdir::new("searchset_empty");
    wrk.create("data.csv", data(true));
    wrk.create("emptyregexset.txt", empty_regexset_file());
    let mut cmd = wrk.command("searchset");
    cmd.arg("emptyregexset.txt").arg("data.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
fn searchset_empty_no_headers() {
    let wrk = Workdir::new("searchset_empty_no_headers");
    wrk.create("data.csv", data(true));
    wrk.create("emptyregexset.txt", empty_regexset_file());
    let mut cmd = wrk.command("searchset");
    cmd.arg("emptyregexset.txt").arg("data.csv");
    cmd.arg("--no-headers");

    wrk.assert_err(&mut cmd);
}

#[test]
fn searchset_ignore_case() {
    let wrk = Workdir::new("searchset");
    wrk.create("data.csv", data(true));
    wrk.create("regexset.txt", regexset_file());
    let mut cmd = wrk.command("searchset");
    cmd.arg("regexset.txt").arg("data.csv");
    cmd.arg("--ignore-case");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["h1", "h2"],
        svec!["foobar", "barfoo"],
        svec!["barfoo", "foobar"],
        svec!["is waldo here", "spot"],
        svec!["bleh", "no, Waldo is there"],
    ];
    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd);
}

#[test]
fn searchset_ignore_case_count() {
    let wrk = Workdir::new("searchset");
    wrk.create("data.csv", data(true));
    wrk.create("regexset.txt", regexset_file());
    let mut cmd = wrk.command("searchset");
    cmd.arg("regexset.txt").arg("--count").arg("data.csv");
    cmd.arg("--ignore-case");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["h1", "h2"],
        svec!["foobar", "barfoo"],
        svec!["barfoo", "foobar"],
        svec!["is waldo here", "spot"],
        svec!["bleh", "no, Waldo is there"],
    ];
    assert_eq!(got, expected);

    let got = wrk.output_stderr(&mut cmd);
    let expected = "4\n";
    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd);
}

#[test]
fn searchset_no_headers() {
    let wrk = Workdir::new("searchset_no_headers");
    wrk.create("data.csv", data(false));
    wrk.create("regexset.txt", regexset_file());
    let mut cmd = wrk.command("searchset");
    cmd.arg("regexset.txt").arg("data.csv");
    cmd.arg("--no-headers");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["foobar", "barfoo"],
        svec!["barfoo", "foobar"],
        svec!["is waldo here", "spot"],
    ];
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);
}

#[test]
fn searchset_select() {
    let wrk = Workdir::new("searchset_select");
    wrk.create("data.csv", data(true));
    wrk.create("regexset.txt", regexset_file());
    let mut cmd = wrk.command("searchset");
    cmd.arg("regexset.txt").arg("data.csv");
    cmd.arg("--select").arg("h2");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["h1", "h2"], svec!["barfoo", "foobar"]];
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);
}

#[test]
fn searchset_select_no_headers() {
    let wrk = Workdir::new("searchset_select_no_headers");
    wrk.create("data.csv", data(false));
    wrk.create("regexset.txt", regexset_file());
    let mut cmd = wrk.command("searchset");
    cmd.arg("regexset.txt").arg("data.csv");
    cmd.arg("--select").arg("2");
    cmd.arg("--no-headers");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["barfoo", "foobar"]];
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);
}

#[test]
fn searchset_invert_match() {
    let wrk = Workdir::new("searchset_invert_match");
    wrk.create("data.csv", data(false));
    wrk.create("regexset.txt", regexset_file());
    let mut cmd = wrk.command("searchset");
    cmd.arg("regexset.txt").arg("data.csv");
    cmd.arg("--invert-match");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["foobar", "barfoo"],
        svec!["a", "b"],
        svec!["Ḟooƀar", "ḃarḟoo"],
        svec!["bleh", "no, Waldo is there"],
    ];
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);
}

#[test]
fn searchset_invert_match_no_headers() {
    let wrk = Workdir::new("searchset_invert_match");
    wrk.create("data.csv", data(false));
    wrk.create("regexset.txt", regexset_file());
    let mut cmd = wrk.command("searchset");
    cmd.arg("regexset.txt").arg("data.csv");
    cmd.arg("--invert-match");
    cmd.arg("--no-headers");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["a", "b"],
        svec!["Ḟooƀar", "ḃarḟoo"],
        svec!["bleh", "no, Waldo is there"],
    ];
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);
}

#[test]
fn searchset_flag() {
    let wrk = Workdir::new("searchset_flag");
    wrk.create("data.csv", data(true));
    wrk.create("regexset.txt", regexset_file());
    let mut cmd = wrk.command("searchset");
    cmd.arg("regexset.txt")
        .arg("data.csv")
        .args(["--flag", "flagged"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["h1", "h2", "flagged"],
        svec!["foobar", "barfoo", "1;1,2"],
        svec!["a", "b", "0"],
        svec!["barfoo", "foobar", "3;1,2"],
        svec!["is waldo here", "spot", "4;3"],
        svec!["Ḟooƀar", "ḃarḟoo", "0"],
        svec!["bleh", "no, Waldo is there", "0"],
    ];
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);
}

#[test]
fn searchset_flag_invert_match() {
    let wrk = Workdir::new("searchset_flag");
    wrk.create("regexset.txt", regexset_file());
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("searchset");
    cmd.arg("regexset.txt")
        .arg("data.csv")
        .args(["--flag", "flagged"]);
    cmd.arg("--invert-match");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["h1", "h2", "flagged"],
        svec!["foobar", "barfoo", "0"],
        svec!["a", "b", "2"],
        svec!["barfoo", "foobar", "0"],
        svec!["is waldo here", "spot", "0"],
        svec!["Ḟooƀar", "ḃarḟoo", "5"],
        svec!["bleh", "no, Waldo is there", "6"],
    ];
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);
}

#[test]
fn searchset_flag_complex() {
    let wrk = Workdir::new("searchset_flag_complex");
    let test_file = wrk.load_test_file("boston311-100-with-fake-pii.csv");
    let regex_file = wrk.load_test_file("pii_regex_searchset.txt");

    let mut cmd = wrk.command("searchset");
    cmd.arg(regex_file)
        .arg(test_file)
        .args(["--flag", "flagged"])
        .arg("--flag-matches-only")
        .arg("--json");

    let got: String = wrk.stdout(&mut cmd);
    let got_stderr: String = wrk.output_stderr(&mut cmd);

    let expected = wrk.load_test_resource("boston311-100-pii-searchset.csv");
    assert_eq!(dos2unix(&got), dos2unix(&expected).trim_end());

    let expected_stderr = r#"{"rows_with_matches":5,"total_matches":6,"record_count":100}"#;
    assert_eq!(got_stderr.trim_end(), expected_stderr);
    wrk.assert_success(&mut cmd);
}

#[test]
fn searchset_flag_complex_unmatched_output() {
    let wrk = Workdir::new("searchset_flag_complex");
    let test_file = wrk.load_test_file("boston311-100-with-fake-pii.csv");
    let regex_file = wrk.load_test_file("pii_regex_searchset.txt");
    let nopii_file = wrk.load_test_resource("boston311-100-nopii-searchset.csv");

    let mut cmd = wrk.command("searchset");
    cmd.arg(regex_file)
        .arg(test_file)
        .args(["--flag", "flagged"])
        .arg("--flag-matches-only")
        .arg("--unmatched-output")
        .arg("unmatched.csv")
        .arg("--json");

    let got: String = wrk.stdout(&mut cmd);
    let got_stderr: String = wrk.output_stderr(&mut cmd);

    let expected = wrk.load_test_resource("boston311-100-pii-searchset.csv");
    assert_eq!(dos2unix(&got), dos2unix(&expected).trim_end());

    let expected_stderr = r#"{"rows_with_matches":5,"total_matches":6,"record_count":100}"#;
    assert_eq!(got_stderr.trim_end(), expected_stderr);

    let unmatched_got: String = wrk.from_str(&wrk.path("unmatched.csv"));
    assert_eq!(unmatched_got, nopii_file);

    wrk.assert_success(&mut cmd);
}

#[test]
fn searchset_literal() {
    let wrk = Workdir::new("searchset_literal");
    wrk.create("data.csv", data_with_regex_chars(true));
    wrk.create("regexset.txt", regexset_literal_file());
    let mut cmd = wrk.command("searchset");
    cmd.arg("regexset.txt").arg("data.csv").arg("--literal");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["h1", "h2"],
        svec!["foo$bar^", "barfoo"],
        svec!["$bar^foo", "foobar"],
        svec!["is wal[do] here", "spot"],
        svec!["bleh", "no, Wal[do] is there"],
    ];
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);
}

#[test]
fn searchset_exact() {
    let wrk = Workdir::new("searchset_exact");
    wrk.create("data.csv", data_with_regex_chars(true));
    wrk.create("regexset.txt", regexset_exact_file());
    let mut cmd = wrk.command("searchset");
    cmd.arg("regexset.txt").arg("data.csv").arg("--exact");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["h1", "h2"],
        svec!["foo$bar^", "barfoo"],
        svec!["is wal[do] here", "spot"],
    ];
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);
}

#[test]
fn searchset_exact_with_dots() {
    let wrk = Workdir::new("searchset_exact_with_dots");
    wrk.create("data.csv", data_with_dots(true));
    wrk.create("regexset.txt", regexset_exact_dots_file());
    let mut cmd = wrk.command("searchset");
    cmd.arg("regexset.txt").arg("data.csv").arg("--exact");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // Should only match "J. Bloggs" exactly, not "F. J. Bloggs" or "JM Bloggs"
    let expected = vec![svec!["id", "name"], svec!["3", "J. Bloggs"]];
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);
}

#[test]
fn searchset_exact_case_insensitive() {
    let wrk = Workdir::new("searchset_exact_case_insensitive");
    wrk.create("data.csv", data_with_dots(true));
    wrk.create("regexset.txt", vec![svec!["j. bloggs"]]);
    let mut cmd = wrk.command("searchset");
    cmd.arg("regexset.txt")
        .arg("data.csv")
        .arg("--exact")
        .arg("--ignore-case");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["id", "name"], svec!["3", "J. Bloggs"]];
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);
}

#[test]
fn searchset_exact_no_match_substring() {
    let wrk = Workdir::new("searchset_exact_no_match_substring");
    wrk.create("data.csv", data_with_dots(true));
    wrk.create("regexset.txt", regexset_exact_dots_file());
    let mut cmd = wrk.command("searchset");
    cmd.arg("regexset.txt").arg("data.csv").arg("--exact");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // Should NOT match "F. J. Bloggs" even though it contains "J. Bloggs" as substring
    let expected = vec![svec!["id", "name"], svec!["3", "J. Bloggs"]];
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);
}
