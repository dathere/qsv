use crate::workdir::Workdir;

fn data(headers: bool) -> Vec<Vec<String>> {
    let mut rows = vec![
        svec!["foobar", "barfoo"],
        svec!["a", "b"],
        svec!["barfoo", "foobar"],
        svec!["Ḟooƀar", "ḃarḟoo"],
    ];
    if headers {
        rows.insert(0, svec!["h1", "h2"]);
    }
    rows
}

fn data_with_regex_chars(headers: bool) -> Vec<Vec<String>> {
    let mut rows = vec![
        svec!["foo^bar", "barfoo"],
        svec!["a", "b"],
        svec!["^barfoo", "foobar"],
        svec!["Ḟooƀar", "ḃarḟoo"],
    ];
    if headers {
        rows.insert(0, svec!["h1", "h2"]);
    }
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

#[test]
fn search() {
    let wrk = Workdir::new("search");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["h1", "h2"],
        svec!["foobar", "barfoo"],
        svec!["barfoo", "foobar"],
    ];
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);
}

#[test]
fn search_indexed_parallel() {
    let wrk = Workdir::new("search_indexed_parallel");
    let data = wrk.load_test_resource("boston311-100.csv");
    wrk.create_from_string("data.csv", &data);

    let mut cmd = wrk.command("search");
    cmd.arg("Charlestown")
        .arg("data.csv")
        .arg("--jobs")
        .arg("2");
    let got: String = wrk.stdout(&mut cmd);
    let expected = "case_enquiry_id,open_dt,target_dt,closed_dt,ontime,case_status,closure_reason,case_title,subject,reason,type,queue,department,submittedphoto,closedphoto,location,fire_district,pwd_district,city_council_district,police_district,neighborhood,neighborhood_services_district,ward,precinct,location_street_name,location_zipcode,latitude,longitude,source\n101004154423,2022-01-31 08:05:00,,,ONTIME,Open, ,Sidewalk Cover / Manhole,Boston Water & Sewer Commission,Sidewalk Cover / Manhole,Sidewalk Cover / Manhole,INFO01_GenericeFormforOtherServiceRequestTypes,INFO,,,8 Putnam St  Charlestown  MA  02129,3,1A,1,A15,Charlestown,2,Ward 2,0201,8 Putnam St,02129,42.3735,-71.0599,Constituent Call\n101004114776,2022-01-03 12:13:47,2022-01-04 12:13:47,2022-01-03 12:41:17,ONTIME,Closed,Case Closed. Closed date : 2022-01-03 12:41:17.887 Case Resolved Area ticketed  ,Parking Enforcement,Transportation - Traffic Division,Enforcement & Abandoned Vehicles,Parking Enforcement,BTDT_Parking Enforcement,BTDT,https://311.boston.gov/media/boston/report/photos/61d32ebc05bbcf180c2b01ff/report.jpg,,126 Elm St  Charlestown  MA  02129,3,1A,1,A15,Charlestown,2,02,0204,126 Elm St,02129,42.3806,-71.0616,Citizens Connect App";
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);

    // now index the file
    let mut cmd = wrk.command("index");
    cmd.arg("data.csv");
    wrk.assert_success(&mut cmd);

    // should still have the same output
    let mut cmd = wrk.command("search");
    cmd.arg("Charlestown")
        .arg("data.csv")
        .arg("--jobs")
        .arg("2");
    let got: String = wrk.stdout(&mut cmd);
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);
}

#[test]
fn search_indexed_parallel_json() {
    let wrk = Workdir::new("search_indexed_parallel_json");
    let data = wrk.load_test_resource("boston311-100.csv");
    wrk.create_from_string("data.csv", &data);

    let mut cmd = wrk.command("search");
    cmd.arg("Charlestown")
        .arg("data.csv")
        .arg("--jobs")
        .arg("2")
        .arg("--json");
    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"[{"case_enquiry_id":"101004154423","open_dt":"2022-01-31 08:05:00","target_dt":null,"closed_dt":null,"ontime":"ONTIME","case_status":"Open","closure_reason":" ","case_title":"Sidewalk Cover / Manhole","subject":"Boston Water & Sewer Commission","reason":"Sidewalk Cover / Manhole","type":"Sidewalk Cover / Manhole","queue":"INFO01_GenericeFormforOtherServiceRequestTypes","department":"INFO","submittedphoto":null,"closedphoto":null,"location":"8 Putnam St  Charlestown  MA  02129","fire_district":"3","pwd_district":"1A","city_council_district":"1","police_district":"A15","neighborhood":"Charlestown","neighborhood_services_district":"2","ward":"Ward 2","precinct":"0201","location_street_name":"8 Putnam St","location_zipcode":"02129","latitude":"42.3735","longitude":"-71.0599","source":"Constituent Call"},{"case_enquiry_id":"101004114776","open_dt":"2022-01-03 12:13:47","target_dt":"2022-01-04 12:13:47","closed_dt":"2022-01-03 12:41:17","ontime":"ONTIME","case_status":"Closed","closure_reason":"Case Closed. Closed date : 2022-01-03 12:41:17.887 Case Resolved Area ticketed  ","case_title":"Parking Enforcement","subject":"Transportation - Traffic Division","reason":"Enforcement & Abandoned Vehicles","type":"Parking Enforcement","queue":"BTDT_Parking Enforcement","department":"BTDT","submittedphoto":"https://311.boston.gov/media/boston/report/photos/61d32ebc05bbcf180c2b01ff/report.jpg","closedphoto":null,"location":"126 Elm St  Charlestown  MA  02129","fire_district":"3","pwd_district":"1A","city_council_district":"1","police_district":"A15","neighborhood":"Charlestown","neighborhood_services_district":"2","ward":"02","precinct":"0204","location_street_name":"126 Elm St","location_zipcode":"02129","latitude":"42.3806","longitude":"-71.0616","source":"Citizens Connect App"}]"#;
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);

    // now index the file
    let mut cmd = wrk.command("index");
    cmd.arg("data.csv");
    wrk.assert_success(&mut cmd);

    // should still have the same output
    let mut cmd = wrk.command("search");
    cmd.arg("Charlestown")
        .arg("data.csv")
        .arg("--jobs")
        .arg("2")
        .arg("--json");
    let got: String = wrk.stdout(&mut cmd);
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);
}

#[test]
fn search_json() {
    let wrk = Workdir::new("search_json");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("data.csv").arg("--json");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"[{"h1":"foobar","h2":"barfoo"},{"h1":"barfoo","h2":"foobar"}]"#;
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);
}

#[test]
fn search_matchonly_json() {
    let wrk = Workdir::new("search_matchonly_json");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo")
        .arg("data.csv")
        .arg("--json")
        .args(["--flag", "M"]);

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"[{"M":"1"},{"M":"3"}]"#;
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);
}

#[test]
fn search_match() {
    let wrk = Workdir::new("search_match");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("data.csv");

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["h1", "h2"],
        svec!["foobar", "barfoo"],
        svec!["barfoo", "foobar"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn search_match_json() {
    let wrk = Workdir::new("search_match_json");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("data.csv").arg("--json");

    wrk.assert_success(&mut cmd);

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"[{"h1":"foobar","h2":"barfoo"},{"h1":"barfoo","h2":"foobar"}]"#;
    assert_eq!(got, expected);
}

#[test]
fn search_match_with_count() {
    let wrk = Workdir::new("search_match");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("--count").arg("data.csv");

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["h1", "h2"],
        svec!["foobar", "barfoo"],
        svec!["barfoo", "foobar"],
    ];
    assert_eq!(got, expected);

    let got_err = wrk.output_stderr(&mut cmd);
    assert_eq!(got_err, "2\n");
}

#[test]
fn search_match_quick() {
    let wrk = Workdir::new("search_match_quick");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("^a").arg("--quick").arg("data.csv");

    let got_err = wrk.output_stderr(&mut cmd);
    assert_eq!(got_err, "2\n");
    wrk.assert_success(&mut cmd);
    let got: String = wrk.stdout(&mut cmd);
    assert_eq!(got, "");
}

#[test]
fn search_nomatch() {
    let wrk = Workdir::new("search_nomatch");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("waldo").arg("data.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
fn search_empty() {
    let wrk = Workdir::new("search");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("xxx").arg("data.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
fn search_empty_no_headers() {
    let wrk = Workdir::new("search");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("xxx").arg("data.csv");
    cmd.arg("--no-headers");

    wrk.assert_err(&mut cmd);
}

#[test]
fn search_ignore_case() {
    let wrk = Workdir::new("search");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("^FoO").arg("data.csv");
    cmd.arg("--ignore-case");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["h1", "h2"],
        svec!["foobar", "barfoo"],
        svec!["barfoo", "foobar"],
    ];
    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_ignore_case_count() {
    let wrk = Workdir::new("search");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("^FoO").arg("--count").arg("data.csv");
    cmd.arg("--ignore-case");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["h1", "h2"],
        svec!["foobar", "barfoo"],
        svec!["barfoo", "foobar"],
    ];
    assert_eq!(got, expected);

    let got_err = wrk.output_stderr(&mut cmd);
    assert_eq!(got_err, "2\n");

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_unicode() {
    let wrk = Workdir::new("search");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("^Ḟoo").arg("data.csv");
    cmd.arg("--unicode");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["h1", "h2"], svec!["Ḟooƀar", "ḃarḟoo"]];
    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_unicode_count() {
    let wrk = Workdir::new("search");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("^Ḟoo").arg("--count").arg("data.csv");
    cmd.arg("--unicode");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["h1", "h2"], svec!["Ḟooƀar", "ḃarḟoo"]];
    assert_eq!(got, expected);

    let got_err = wrk.output_stderr(&mut cmd);
    assert_eq!(got_err, "1\n");

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_unicode_envvar() {
    let wrk = Workdir::new("search");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.env("QSV_REGEX_UNICODE", "1");
    cmd.arg("^Ḟoo").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["h1", "h2"], svec!["Ḟooƀar", "ḃarḟoo"]];
    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_unicode_envvar_count() {
    let wrk = Workdir::new("search");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.env("QSV_REGEX_UNICODE", "1");
    cmd.arg("^Ḟoo").arg("--count").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["h1", "h2"], svec!["Ḟooƀar", "ḃarḟoo"]];
    assert_eq!(got, expected);

    let got_err = wrk.output_stderr(&mut cmd);
    assert_eq!(got_err, "1\n");

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_no_headers() {
    let wrk = Workdir::new("search_no_headers");
    wrk.create("data.csv", data(false));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("data.csv");
    cmd.arg("--no-headers");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["foobar", "barfoo"], svec!["barfoo", "foobar"]];
    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_no_headers_json() {
    let wrk = Workdir::new("search_no_headers_json");
    wrk.create("data.csv", data(false));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("data.csv").arg("--json");
    cmd.arg("--no-headers");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"[{"0":"foobar","1":"barfoo"},{"0":"barfoo","1":"foobar"}]"#;
    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_no_headers_count() {
    let wrk = Workdir::new("search_no_headers");
    wrk.create("data.csv", data(false));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("--count").arg("data.csv");
    cmd.arg("--no-headers");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["foobar", "barfoo"], svec!["barfoo", "foobar"]];
    assert_eq!(got, expected);

    let got_err = wrk.output_stderr(&mut cmd);
    assert_eq!(got_err, "2\n");

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_select() {
    let wrk = Workdir::new("search_select");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("data.csv");
    cmd.arg("--select").arg("h2");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["h1", "h2"], svec!["barfoo", "foobar"]];
    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_select_count() {
    let wrk = Workdir::new("search_select");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("--count").arg("data.csv");
    cmd.arg("--select").arg("h2");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["h1", "h2"], svec!["barfoo", "foobar"]];
    assert_eq!(got, expected);

    let got_err = wrk.output_stderr(&mut cmd);
    assert_eq!(got_err, "1\n");

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_select_no_headers() {
    let wrk = Workdir::new("search_select_no_headers");
    wrk.create("data.csv", data(false));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("data.csv");
    cmd.arg("--select").arg("2");
    cmd.arg("--no-headers");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["barfoo", "foobar"]];
    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_select_no_headers_count() {
    let wrk = Workdir::new("search_select_no_headers");
    wrk.create("data.csv", data(false));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("--count").arg("data.csv");
    cmd.arg("--select").arg("2");
    cmd.arg("--no-headers");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["barfoo", "foobar"]];
    assert_eq!(got, expected);

    let got_err = wrk.output_stderr(&mut cmd);
    assert_eq!(got_err, "1\n");

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_invert_match() {
    let wrk = Workdir::new("search_invert_match");
    wrk.create("data.csv", data(false));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("data.csv");
    cmd.arg("--invert-match");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["foobar", "barfoo"],
        svec!["a", "b"],
        svec!["Ḟooƀar", "ḃarḟoo"],
    ];
    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_invert_match_count() {
    let wrk = Workdir::new("search_invert_match");
    wrk.create("data.csv", data(false));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("--count").arg("data.csv");
    cmd.arg("--invert-match");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["foobar", "barfoo"],
        svec!["a", "b"],
        svec!["Ḟooƀar", "ḃarḟoo"],
    ];
    assert_eq!(got, expected);

    let got = wrk.output_stderr(&mut cmd);
    let expected = "2\n";
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);
}

#[test]
fn search_invert_match_no_headers() {
    let wrk = Workdir::new("search_invert_match");
    wrk.create("data.csv", data(false));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("data.csv");
    cmd.arg("--invert-match");
    cmd.arg("--no-headers");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["a", "b"], svec!["Ḟooƀar", "ḃarḟoo"]];
    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_invert_match_no_headers_count() {
    let wrk = Workdir::new("search_invert_match");
    wrk.create("data.csv", data(false));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("--count").arg("data.csv");
    cmd.arg("--invert-match");
    cmd.arg("--no-headers");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["a", "b"], svec!["Ḟooƀar", "ḃarḟoo"]];
    assert_eq!(got, expected);

    let got_err = wrk.output_stderr(&mut cmd);
    assert_eq!(got_err, "2\n");

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_flag() {
    let wrk = Workdir::new("search_flag");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("data.csv").args(["--flag", "flagged"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["h1", "h2", "flagged"],
        svec!["foobar", "barfoo", "1"],
        svec!["a", "b", "0"],
        svec!["barfoo", "foobar", "3"],
        svec!["Ḟooƀar", "ḃarḟoo", "0"],
    ];
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);
}

#[test]
fn search_flag_no_headers() {
    let wrk = Workdir::new("search_flag_no_headers");
    wrk.create("data.csv", data(false));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("data.csv").args(["--flag", "flagged"]);
    cmd.arg("--no-headers");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["foobar", "barfoo", "1"],
        svec!["a", "b", "0"],
        svec!["barfoo", "foobar", "3"],
        svec!["Ḟooƀar", "ḃarḟoo", "0"],
    ];
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);
}

#[test]
fn search_flag_match_only() {
    let wrk = Workdir::new("search_flag_match_only");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("data.csv").args(["--flag", "M"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["M"], svec!["1"], svec!["3"]];
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);
}

#[test]
fn search_flag_match_only_no_headers() {
    let wrk = Workdir::new("search_flag_match_only_no_headers");
    wrk.create("data.csv", data(false));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo")
        .arg("data.csv")
        .args(["--flag", "M"])
        .arg("--no-headers");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["1"], svec!["3"]];
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);
}

#[test]
fn search_flag_invert_match() {
    let wrk = Workdir::new("search_flag");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("data.csv").args(["--flag", "flagged"]);
    cmd.arg("--invert-match");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["h1", "h2", "flagged"],
        svec!["foobar", "barfoo", "0"],
        svec!["a", "b", "2"],
        svec!["barfoo", "foobar", "0"],
        svec!["Ḟooƀar", "ḃarḟoo", "4"],
    ];
    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_flag_invert_match_matchonly() {
    let wrk = Workdir::new("search_flag_invert_match_matchonly");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo").arg("data.csv").args(["--flag", "M"]);
    cmd.arg("--invert-match");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["M"], svec!["2"], svec!["4"]];
    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_flag_invert_match_count() {
    let wrk = Workdir::new("search_flag");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo")
        .arg("--count")
        .arg("data.csv")
        .args(["--flag", "flagged"]);
    cmd.arg("--invert-match");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["h1", "h2", "flagged"],
        svec!["foobar", "barfoo", "0"],
        svec!["a", "b", "2"],
        svec!["barfoo", "foobar", "0"],
        svec!["Ḟooƀar", "ḃarḟoo", "4"],
    ];
    assert_eq!(got, expected);

    let got_err = wrk.output_stderr(&mut cmd);
    assert_eq!(got_err, "2\n");

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_flag_invert_matchonly_count() {
    let wrk = Workdir::new("search_flag_invert_matchonly_count");
    wrk.create("data.csv", data(true));
    let mut cmd = wrk.command("search");
    cmd.arg("^foo")
        .arg("--count")
        .arg("data.csv")
        .args(["--flag", "M"]);
    cmd.arg("--invert-match");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["M"], svec!["2"], svec!["4"]];
    assert_eq!(got, expected);

    let got_err = wrk.output_stderr(&mut cmd);
    assert_eq!(got_err, "2\n");

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_preview() {
    let wrk = Workdir::new("search_preview");

    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("search");
    cmd.arg("Beacon Hill")
        .arg(test_file)
        .args(["--preview-match", "2"]);

    let preview = wrk.output_stderr(&mut cmd);
    let expected_preview = r#"case_enquiry_id,open_dt,target_dt,closed_dt,ontime,case_status,closure_reason,case_title,subject,reason,type,queue,department,submittedphoto,closedphoto,location,fire_district,pwd_district,city_council_district,police_district,neighborhood,neighborhood_services_district,ward,precinct,location_street_name,location_zipcode,latitude,longitude,source
101004113298,2022-01-01 00:16:00,2022-04-01 00:16:06,2022-01-10 08:42:23,ONTIME,Closed,Case Closed. Closed date : Mon Jan 10 08:42:23 EST 2022 Resolved No Cause 1/10/22 ,SCHEDULED Unsatisfactory Utilities - Electrical  Plumbing,Inspectional Services,Housing,Unsatisfactory Utilities - Electrical  Plumbing,ISD_Housing (INTERNAL),ISD,,,47 W Cedar St  Boston  MA  02114,3,1B,8,A1,Beacon Hill,14,Ward 5,0504,47 W Cedar St,02114,42.3594,-71.07,Constituent Call
101004141354,2022-01-20 08:07:49,2022-01-21 08:30:00,2022-01-20 08:45:03,ONTIME,Closed,Case Closed. Closed date : Thu Jan 20 08:45:03 EST 2022 Noted ,CE Collection,Public Works Department,Street Cleaning,CE Collection,PWDx_District 1B: North End,PWDx,,,21-23 Temple St  Boston  MA  02114,3,1B,1,A1,Beacon Hill,3,Ward 3,0306,21-23 Temple St,02114,42.3606,-71.0638,City Worker App
Previewed 2 matches in 100 initial records in"#;
    assert!(preview.starts_with(expected_preview));

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["case_enquiry_id", "open_dt", "target_dt", "closed_dt", "ontime", "case_status", "closure_reason", "case_title", "subject", "reason", "type", "queue", "department", "submittedphoto", "closedphoto", "location", "fire_district", "pwd_district", "city_council_district", "police_district", "neighborhood", "neighborhood_services_district", "ward", "precinct", "location_street_name", "location_zipcode", "latitude", "longitude", "source"], 
        svec!["101004113298", "2022-01-01 00:16:00", "2022-04-01 00:16:06", "2022-01-10 08:42:23", "ONTIME", "Closed", "Case Closed. Closed date : Mon Jan 10 08:42:23 EST 2022 Resolved No Cause 1/10/22 ", "SCHEDULED Unsatisfactory Utilities - Electrical  Plumbing", "Inspectional Services", "Housing", "Unsatisfactory Utilities - Electrical  Plumbing", "ISD_Housing (INTERNAL)", "ISD", "", "", "47 W Cedar St  Boston  MA  02114", "3", "1B", "8", "A1", "Beacon Hill", "14", "Ward 5", "0504", "47 W Cedar St", "02114", "42.3594", "-71.07", "Constituent Call"], 
        svec!["101004141354", "2022-01-20 08:07:49", "2022-01-21 08:30:00", "2022-01-20 08:45:03", "ONTIME", "Closed", "Case Closed. Closed date : Thu Jan 20 08:45:03 EST 2022 Noted ", "CE Collection", "Public Works Department", "Street Cleaning", "CE Collection", "PWDx_District 1B: North End", "PWDx", "", "", "21-23 Temple St  Boston  MA  02114", "3", "1B", "1", "A1", "Beacon Hill", "3", "Ward 3", "0306", "21-23 Temple St", "02114", "42.3606", "-71.0638", "City Worker App"], 
        svec!["101004141367", "2022-01-20 08:15:45", "2022-01-21 08:30:00", "2022-01-20 08:45:12", "ONTIME", "Closed", "Case Closed. Closed date : Thu Jan 20 08:45:12 EST 2022 Noted ", "CE Collection", "Public Works Department", "Street Cleaning", "CE Collection", "PWDx_District 1B: North End", "PWDx", "", "", "12 Derne St  Boston  MA  02114", "3", "1B", "1", "A1", "Beacon Hill", "3", "Ward 3", "0306", "12 Derne St", "02114", "42.3596", "-71.0634", "City Worker App"], 
        svec!["101004113348", "2022-01-01 06:46:29", "2022-01-05 08:30:00", "2022-01-01 15:10:16", "ONTIME", "Closed", "Case Closed. Closed date : Sat Jan 01 15:10:16 EST 2022 Noted Trash bags sent in for collection. No evidence or code violations found at this time  ", "Improper Storage of Trash (Barrels)", "Public Works Department", "Code Enforcement", "Improper Storage of Trash (Barrels)", "PWDx_Code Enforcement", "PWDx", "https://311.boston.gov/media/boston/report/photos/61d03f0d05bbcf180c2965fd/report.jpg", "", "14 S Russell St  Boston  MA  02114", "3", "1B", "1", "A1", "Beacon Hill", "3", "Ward 3", "0306", "14 S Russell St", "02114", "42.3607", "-71.0659", "Citizens Connect App"], 
        svec!["101004113431", "2022-01-01 10:35:45", "2022-01-05 08:30:00", "2022-01-01 14:59:41", "ONTIME", "Closed", "Case Closed. Closed date : Sat Jan 01 14:59:41 EST 2022 Noted Bags sent in for collection. Ticket issued  ", "Improper Storage of Trash (Barrels)", "Public Works Department", "Code Enforcement", "Improper Storage of Trash (Barrels)", "PWDx_Code Enforcement", "PWDx", "https://311.boston.gov/media/boston/report/photos/61d074c005bbcf180c298048/report.jpg", "", "40 Anderson St  Boston  MA  02114", "3", "1B", "8", "A1", "Beacon Hill", "14", "Ward 5", "0504", "40 Anderson St", "02114", "42.3598", "-71.0676", "Citizens Connect App"], 
        svec!["101004113717", "2022-01-01 21:11:00", "2022-01-04 08:30:00", "2022-01-04 09:30:03", "OVERDUE", "Closed", "Case Closed. Closed date : 2022-01-04 09:30:03.91 Case Noted Dear Constituent     NGRID is aware of the broken gate and will send a crew to repair.    We are waiting on there schedule to do so.    Regards   Rich DiMarzo  781-853-9016 ", "Request for Pothole Repair", "Public Works Department", "Highway Maintenance", "Request for Pothole Repair", "PWDx_Contractor Complaints", "PWDx", "https://311.boston.gov/media/boston/report/photos/61d109cf05bbcf180c29c167/Pothole_1.jpg", "", "INTERSECTION of Charles River Plz & Cambridge St  Boston  MA  ", "3", "1B", "7", "A1", "Beacon Hill", "3", "3", "0305", "INTERSECTION Charles River Plz & Cambridge St", "", "42.3594", "-71.0587", "Citizens Connect App"], 
        svec!["101004115066", "2022-01-03 15:51:00", "2022-01-04 15:51:30", "", "OVERDUE", "Open", " ", "Sidewalk Repair (Make Safe)", "Public Works Department", "Highway Maintenance", "Sidewalk Repair (Make Safe)", "PWDx_Highway Construction", "PWDx", "https://311.boston.gov/media/boston/report/photos/61d361c905bbcf180c2b1dd3/report.jpg", "", "64 Anderson St  Boston  MA  02114", "3", "1B", "8", "A1", "Beacon Hill", "14", "Ward 5", "0503", "64 Anderson St", "02114", "42.359", "-71.0676", "Citizens Connect App"],
    ];
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);
}

#[test]
fn search_preview_json() {
    let wrk = Workdir::new("search_preview_json");

    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("search");
    cmd.arg("Beacon Hill")
        .arg(test_file)
        .arg("--json")
        .arg("--quiet")
        .args(["--preview-match", "2"]);

    let preview = wrk.output_stderr(&mut cmd);
    let expected_preview = r#"[{"case_enquiry_id":"101004113298","open_dt":"2022-01-01 00:16:00","target_dt":"2022-04-01 00:16:06","closed_dt":"2022-01-10 08:42:23","ontime":"ONTIME","case_status":"Closed","closure_reason":"Case Closed. Closed date : Mon Jan 10 08:42:23 EST 2022 Resolved No Cause 1/10/22 ","case_title":"SCHEDULED Unsatisfactory Utilities - Electrical  Plumbing","subject":"Inspectional Services","reason":"Housing","type":"Unsatisfactory Utilities - Electrical  Plumbing","queue":"ISD_Housing (INTERNAL)","department":"ISD","submittedphoto":null,"closedphoto":null,"location":"47 W Cedar St  Boston  MA  02114","fire_district":"3","pwd_district":"1B","city_council_district":"8","police_district":"A1","neighborhood":"Beacon Hill","neighborhood_services_district":"14","ward":"Ward 5","precinct":"0504","location_street_name":"47 W Cedar St","location_zipcode":"02114","latitude":"42.3594","longitude":"-71.07","source":"Constituent Call"},{"case_enquiry_id":"101004141354","open_dt":"2022-01-20 08:07:49","target_dt":"2022-01-21 08:30:00","closed_dt":"2022-01-20 08:45:03","ontime":"ONTIME","case_status":"Closed","closure_reason":"Case Closed. Closed date : Thu Jan 20 08:45:03 EST 2022 Noted ","case_title":"CE Collection","subject":"Public Works Department","reason":"Street Cleaning","type":"CE Collection","queue":"PWDx_District 1B: North End","department":"PWDx","submittedphoto":null,"closedphoto":null,"location":"21-23 Temple St  Boston  MA  02114","fire_district":"3","pwd_district":"1B","city_council_district":"1","police_district":"A1","neighborhood":"Beacon Hill","neighborhood_services_district":"3","ward":"Ward 3","precinct":"0306","location_street_name":"21-23 Temple St","location_zipcode":"02114","latitude":"42.3606","longitude":"-71.0638","source":"City Worker App"}]"#;
    let preview_json: serde_json::Value = serde_json::from_str(&preview).unwrap();
    let expected_json: serde_json::Value = serde_json::from_str(expected_preview).unwrap();
    let preview_json_pretty = serde_json::to_string_pretty(&preview_json).unwrap();
    let expected_json_pretty = serde_json::to_string_pretty(&expected_json).unwrap();
    assert_eq!(preview_json_pretty, expected_json_pretty);

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"[{"case_enquiry_id":"101004113298","open_dt":"2022-01-01 00:16:00","target_dt":"2022-04-01 00:16:06","closed_dt":"2022-01-10 08:42:23","ontime":"ONTIME","case_status":"Closed","closure_reason":"Case Closed. Closed date : Mon Jan 10 08:42:23 EST 2022 Resolved No Cause 1/10/22 ","case_title":"SCHEDULED Unsatisfactory Utilities - Electrical  Plumbing","subject":"Inspectional Services","reason":"Housing","type":"Unsatisfactory Utilities - Electrical  Plumbing","queue":"ISD_Housing (INTERNAL)","department":"ISD","submittedphoto":null,"closedphoto":null,"location":"47 W Cedar St  Boston  MA  02114","fire_district":"3","pwd_district":"1B","city_council_district":"8","police_district":"A1","neighborhood":"Beacon Hill","neighborhood_services_district":"14","ward":"Ward 5","precinct":"0504","location_street_name":"47 W Cedar St","location_zipcode":"02114","latitude":"42.3594","longitude":"-71.07","source":"Constituent Call"},{"case_enquiry_id":"101004141354","open_dt":"2022-01-20 08:07:49","target_dt":"2022-01-21 08:30:00","closed_dt":"2022-01-20 08:45:03","ontime":"ONTIME","case_status":"Closed","closure_reason":"Case Closed. Closed date : Thu Jan 20 08:45:03 EST 2022 Noted ","case_title":"CE Collection","subject":"Public Works Department","reason":"Street Cleaning","type":"CE Collection","queue":"PWDx_District 1B: North End","department":"PWDx","submittedphoto":null,"closedphoto":null,"location":"21-23 Temple St  Boston  MA  02114","fire_district":"3","pwd_district":"1B","city_council_district":"1","police_district":"A1","neighborhood":"Beacon Hill","neighborhood_services_district":"3","ward":"Ward 3","precinct":"0306","location_street_name":"21-23 Temple St","location_zipcode":"02114","latitude":"42.3606","longitude":"-71.0638","source":"City Worker App"},{"case_enquiry_id":"101004141367","open_dt":"2022-01-20 08:15:45","target_dt":"2022-01-21 08:30:00","closed_dt":"2022-01-20 08:45:12","ontime":"ONTIME","case_status":"Closed","closure_reason":"Case Closed. Closed date : Thu Jan 20 08:45:12 EST 2022 Noted ","case_title":"CE Collection","subject":"Public Works Department","reason":"Street Cleaning","type":"CE Collection","queue":"PWDx_District 1B: North End","department":"PWDx","submittedphoto":null,"closedphoto":null,"location":"12 Derne St  Boston  MA  02114","fire_district":"3","pwd_district":"1B","city_council_district":"1","police_district":"A1","neighborhood":"Beacon Hill","neighborhood_services_district":"3","ward":"Ward 3","precinct":"0306","location_street_name":"12 Derne St","location_zipcode":"02114","latitude":"42.3596","longitude":"-71.0634","source":"City Worker App"},{"case_enquiry_id":"101004113348","open_dt":"2022-01-01 06:46:29","target_dt":"2022-01-05 08:30:00","closed_dt":"2022-01-01 15:10:16","ontime":"ONTIME","case_status":"Closed","closure_reason":"Case Closed. Closed date : Sat Jan 01 15:10:16 EST 2022 Noted Trash bags sent in for collection. No evidence or code violations found at this time  ","case_title":"Improper Storage of Trash (Barrels)","subject":"Public Works Department","reason":"Code Enforcement","type":"Improper Storage of Trash (Barrels)","queue":"PWDx_Code Enforcement","department":"PWDx","submittedphoto":"https://311.boston.gov/media/boston/report/photos/61d03f0d05bbcf180c2965fd/report.jpg","closedphoto":null,"location":"14 S Russell St  Boston  MA  02114","fire_district":"3","pwd_district":"1B","city_council_district":"1","police_district":"A1","neighborhood":"Beacon Hill","neighborhood_services_district":"3","ward":"Ward 3","precinct":"0306","location_street_name":"14 S Russell St","location_zipcode":"02114","latitude":"42.3607","longitude":"-71.0659","source":"Citizens Connect App"},{"case_enquiry_id":"101004113431","open_dt":"2022-01-01 10:35:45","target_dt":"2022-01-05 08:30:00","closed_dt":"2022-01-01 14:59:41","ontime":"ONTIME","case_status":"Closed","closure_reason":"Case Closed. Closed date : Sat Jan 01 14:59:41 EST 2022 Noted Bags sent in for collection. Ticket issued  ","case_title":"Improper Storage of Trash (Barrels)","subject":"Public Works Department","reason":"Code Enforcement","type":"Improper Storage of Trash (Barrels)","queue":"PWDx_Code Enforcement","department":"PWDx","submittedphoto":"https://311.boston.gov/media/boston/report/photos/61d074c005bbcf180c298048/report.jpg","closedphoto":null,"location":"40 Anderson St  Boston  MA  02114","fire_district":"3","pwd_district":"1B","city_council_district":"8","police_district":"A1","neighborhood":"Beacon Hill","neighborhood_services_district":"14","ward":"Ward 5","precinct":"0504","location_street_name":"40 Anderson St","location_zipcode":"02114","latitude":"42.3598","longitude":"-71.0676","source":"Citizens Connect App"},{"case_enquiry_id":"101004113717","open_dt":"2022-01-01 21:11:00","target_dt":"2022-01-04 08:30:00","closed_dt":"2022-01-04 09:30:03","ontime":"OVERDUE","case_status":"Closed","closure_reason":"Case Closed. Closed date : 2022-01-04 09:30:03.91 Case Noted Dear Constituent     NGRID is aware of the broken gate and will send a crew to repair.    We are waiting on there schedule to do so.    Regards   Rich DiMarzo  781-853-9016 ","case_title":"Request for Pothole Repair","subject":"Public Works Department","reason":"Highway Maintenance","type":"Request for Pothole Repair","queue":"PWDx_Contractor Complaints","department":"PWDx","submittedphoto":"https://311.boston.gov/media/boston/report/photos/61d109cf05bbcf180c29c167/Pothole_1.jpg","closedphoto":null,"location":"INTERSECTION of Charles River Plz & Cambridge St  Boston  MA  ","fire_district":"3","pwd_district":"1B","city_council_district":"7","police_district":"A1","neighborhood":"Beacon Hill","neighborhood_services_district":"3","ward":"3","precinct":"0305","location_street_name":"INTERSECTION Charles River Plz & Cambridge St","location_zipcode":null,"latitude":"42.3594","longitude":"-71.0587","source":"Citizens Connect App"},{"case_enquiry_id":"101004115066","open_dt":"2022-01-03 15:51:00","target_dt":"2022-01-04 15:51:30","closed_dt":null,"ontime":"OVERDUE","case_status":"Open","closure_reason":" ","case_title":"Sidewalk Repair (Make Safe)","subject":"Public Works Department","reason":"Highway Maintenance","type":"Sidewalk Repair (Make Safe)","queue":"PWDx_Highway Construction","department":"PWDx","submittedphoto":"https://311.boston.gov/media/boston/report/photos/61d361c905bbcf180c2b1dd3/report.jpg","closedphoto":null,"location":"64 Anderson St  Boston  MA  02114","fire_district":"3","pwd_district":"1B","city_council_district":"8","police_district":"A1","neighborhood":"Beacon Hill","neighborhood_services_district":"14","ward":"Ward 5","precinct":"0503","location_street_name":"64 Anderson St","location_zipcode":"02114","latitude":"42.359","longitude":"-71.0676","source":"Citizens Connect App"}]"#;
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);
}

#[test]
fn search_literal() {
    let wrk = Workdir::new("search_literal");
    wrk.create("data.csv", data_with_regex_chars(false));
    let mut cmd = wrk.command("search");
    cmd.arg("^bar").arg("data.csv").arg("--literal");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["foo^bar", "barfoo"], svec!["^barfoo", "foobar"]];
    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_exact() {
    let wrk = Workdir::new("search_exact");
    wrk.create("data.csv", data_with_dots(true));
    let mut cmd = wrk.command("search");
    cmd.arg("--exact").arg("J. Bloggs").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["id", "name"], svec!["3", "J. Bloggs"]];
    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_exact_with_special_chars() {
    let wrk = Workdir::new("search_exact_with_special_chars");
    wrk.create("data.csv", data_with_regex_chars(true));
    let mut cmd = wrk.command("search");
    cmd.arg("--exact").arg("foo^bar").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["h1", "h2"], svec!["foo^bar", "barfoo"]];
    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_exact_no_match_substring() {
    let wrk = Workdir::new("search_exact_no_match_substring");
    wrk.create("data.csv", data_with_dots(true));
    let mut cmd = wrk.command("search");
    cmd.arg("--exact").arg("J. Bloggs").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // Should only match "J. Bloggs", not "F. J. Bloggs" (substring)
    let expected = vec![svec!["id", "name"], svec!["3", "J. Bloggs"]];
    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_exact_case_insensitive() {
    let wrk = Workdir::new("search_exact_case_insensitive");
    wrk.create("data.csv", data_with_dots(true));
    let mut cmd = wrk.command("search");
    cmd.arg("--exact")
        .arg("--ignore-case")
        .arg("j. bloggs")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["id", "name"], svec!["3", "J. Bloggs"]];
    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd);
}

// Test for https://github.com/dathere/qsv/issues/3437
// QSV_NO_HEADERS env var should work the same as --no-headers flag
#[test]
fn search_no_headers_envvar() {
    let wrk = Workdir::new("search_no_headers_envvar");
    wrk.create("data.csv", data(false));
    let mut cmd = wrk.command("search");
    cmd.env("QSV_NO_HEADERS", "1");
    cmd.arg("^foo").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["foobar", "barfoo"], svec!["barfoo", "foobar"]];
    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd);
}

#[test]
fn search_no_headers_envvar_select() {
    let wrk = Workdir::new("search_no_headers_envvar_select");
    wrk.create("data.csv", data(false));
    let mut cmd = wrk.command("search");
    cmd.env("QSV_NO_HEADERS", "1");
    cmd.arg("^foo").arg("data.csv");
    cmd.arg("--select").arg("1");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["foobar", "barfoo"]];
    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd);
}

// Exact reproduction of https://github.com/dathere/qsv/issues/3437
// Given headerless CSV where first row has no '@' in column 1,
// QSV_NO_HEADERS=1 with `qsv search -s 1 '@'` should NOT output that first row.
#[test]
fn search_no_headers_envvar_issue_3437() {
    let wrk = Workdir::new("search_no_headers_envvar_issue_3437");
    wrk.create(
        "data.csv",
        vec![
            svec!["", "foo"],
            svec!["@", "bar"],
            svec!["@", "bar"],
            svec!["@", "bar"],
        ],
    );
    let mut cmd = wrk.command("search");
    cmd.env("QSV_NO_HEADERS", "1");
    cmd.arg("-s").arg("1").arg("@").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["@", "bar"], svec!["@", "bar"], svec!["@", "bar"]];
    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd);
}
