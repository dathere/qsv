use crate::workdir::Workdir;

#[test]
fn replace() {
    let wrk = Workdir::new("replace");
    wrk.create(
        "data.csv",
        vec![
            svec!["identifier", "color"],
            svec!["164.0", "yellow"],
            svec!["165.0", "yellow"],
            svec!["166.0", "yellow"],
            svec!["167.0", "yellow.0"],
        ],
    );
    let mut cmd = wrk.command("replace");
    cmd.arg("\\.0$").arg("").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["identifier", "color"],
        svec!["164", "yellow"],
        svec!["165", "yellow"],
        svec!["166", "yellow"],
        svec!["167", "yellow"],
    ];
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);
}

#[test]
fn replace_regex_literal() {
    let wrk = Workdir::new("replace");
    wrk.create(
        "data.csv",
        vec![
            svec!["identifier", "color"],
            svec!["164.0", "yel$low^"],
            svec!["165.0", "yellow"],
            svec!["166.0", "yellow"],
            svec!["167.0", "yel$low^.0"],
        ],
    );
    let mut cmd = wrk.command("replace");
    cmd.arg("$low^").arg("low").arg("--literal").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["identifier", "color"],
        svec!["164.0", "yellow"],
        svec!["165.0", "yellow"],
        svec!["166.0", "yellow"],
        svec!["167.0", "yellow.0"],
    ];
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);
}

#[test]
fn replace_match() {
    let wrk = Workdir::new("replace_match");
    wrk.create(
        "data.csv",
        vec![
            svec!["identifier", "color"],
            svec!["164.0", "yellow"],
            svec!["165.0", "yellow"],
            svec!["166.0", "yellow"],
            svec!["167.0", "yellow.0"],
        ],
    );
    let mut cmd = wrk.command("replace");
    cmd.arg("\\.0$").arg("").arg("data.csv");

    wrk.assert_success(&mut cmd);
}

#[test]
fn replace_nomatch() {
    let wrk = Workdir::new("replace_nomatch");
    wrk.create(
        "data.csv",
        vec![
            svec!["identifier", "color"],
            svec!["164.5", "yellow"],
            svec!["165.6", "yellow"],
            svec!["166.7", "yellow"],
            svec!["167.8", "yellow.1"],
        ],
    );
    let mut cmd = wrk.command("replace");
    cmd.arg("\\.0$").arg("").arg("data.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
fn replace_nomatch_notone() {
    let wrk = Workdir::new("replace_nomatch_notone");
    wrk.create(
        "data.csv",
        vec![
            svec!["identifier", "color"],
            svec!["164.5", "yellow"],
            svec!["165.6", "yellow"],
            svec!["166.7", "yellow"],
            svec!["167.8", "yellow.1"],
        ],
    );
    let mut cmd = wrk.command("replace");
    cmd.arg("\\.0$").arg("").arg("data.csv").arg("--not-one");

    wrk.assert_success(&mut cmd);
}

#[test]
fn replace_null() {
    let wrk = Workdir::new("replace_null");
    wrk.create(
        "data.csv",
        vec![
            svec!["identifier", "color"],
            svec!["164.0", "yellow"],
            svec!["165.0", "yellow"],
            svec!["166.0", "yellow"],
            svec!["167.0", "yellow.0"],
        ],
    );
    let mut cmd = wrk.command("replace");
    cmd.arg("\\.0$").arg("<NULL>").arg("data.csv");

    let got_err = wrk.output_stderr(&mut cmd);
    let expected_err = "5\n";
    assert_eq!(got_err, expected_err);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["identifier", "color"],
        svec!["164", "yellow"],
        svec!["165", "yellow"],
        svec!["166", "yellow"],
        svec!["167", "yellow"],
    ];
    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd);
}

#[test]
fn replace_unicode() {
    let wrk = Workdir::new("replace");
    wrk.create(
        "data.csv",
        vec![
            svec!["identifier", "color"],
            svec!["164.0", "ŷellow"],
            svec!["165.0", "yellow"],
            svec!["166.0", "yellѳwish"],
            svec!["167.0", "yelloψ"],
            svec!["167.0", "belloψ"],
            svec!["167.0", "bellowish"],
        ],
    );
    let mut cmd = wrk.command("replace");
    cmd.arg("[\\s\\S]ell[\\s\\S]w")
        .arg("Ƀellow")
        .arg("--unicode")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["identifier", "color"],
        svec!["164.0", "Ƀellow"],
        svec!["165.0", "Ƀellow"],
        svec!["166.0", "Ƀellowish"],
        svec!["167.0", "yelloψ"],
        svec!["167.0", "belloψ"],
        svec!["167.0", "Ƀellowish"],
    ];
    assert_eq!(got, expected);

    let got_err = wrk.output_stderr(&mut cmd);
    let expected_err = "4\n";
    assert_eq!(got_err, expected_err);

    wrk.assert_success(&mut cmd);
}

#[test]
fn replace_unicode_envvar() {
    let wrk = Workdir::new("replace");
    wrk.create(
        "data.csv",
        vec![
            svec!["identifier", "color"],
            svec!["164.0", "ŷellow"],
            svec!["165.0", "yellow"],
            svec!["166.0", "yellѳwish"],
            svec!["167.0", "yelloψ"],
            svec!["167.0", "belloψ"],
            svec!["167.0", "bellowish"],
        ],
    );
    let mut cmd = wrk.command("replace");
    cmd.env("QSV_REGEX_UNICODE", "1");
    cmd.arg("[\\s\\S]ell[\\s\\S]w")
        .arg("Ƀellow")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["identifier", "color"],
        svec!["164.0", "Ƀellow"],
        svec!["165.0", "Ƀellow"],
        svec!["166.0", "Ƀellowish"],
        svec!["167.0", "yelloψ"],
        svec!["167.0", "belloψ"],
        svec!["167.0", "Ƀellowish"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn replace_no_headers() {
    let wrk = Workdir::new("replace");
    wrk.create(
        "data.csv",
        vec![
            svec!["164.0", "yellow"],
            svec!["165.0", "yellow"],
            svec!["166.0", "yellow"],
            svec!["167.0", "yellow.0"],
        ],
    );
    let mut cmd = wrk.command("replace");
    cmd.arg("\\.0$")
        .arg("")
        .arg("--no-headers")
        .arg("--select")
        .arg("1")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["164", "yellow"],
        svec!["165", "yellow"],
        svec!["166", "yellow"],
        svec!["167", "yellow.0"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn replace_select() {
    let wrk = Workdir::new("replace");
    wrk.create(
        "data.csv",
        vec![
            svec!["identifier", "color"],
            svec!["164.0", "yellow"],
            svec!["165.0", "yellow"],
            svec!["166.0", "yellow"],
            svec!["167.0", "yellow.0"],
        ],
    );
    let mut cmd = wrk.command("replace");
    cmd.arg("\\.0$")
        .arg("")
        .arg("--select")
        .arg("identifier")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["identifier", "color"],
        svec!["164", "yellow"],
        svec!["165", "yellow"],
        svec!["166", "yellow"],
        svec!["167", "yellow.0"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn replace_groups() {
    let wrk = Workdir::new("replace");
    wrk.create(
        "data.csv",
        vec![
            svec!["identifier", "color"],
            svec!["164.0", "yellow"],
            svec!["165.0", "yellow"],
            svec!["166.0", "yellow"],
            svec!["167.0", "yellow.0"],
        ],
    );
    let mut cmd = wrk.command("replace");
    cmd.arg("\\d+(\\d)\\.0$")
        .arg("$1")
        .arg("--select")
        .arg("identifier")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["identifier", "color"],
        svec!["4", "yellow"],
        svec!["5", "yellow"],
        svec!["6", "yellow"],
        svec!["7", "yellow.0"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn replace_exact() {
    let wrk = Workdir::new("replace_exact");
    wrk.create(
        "data.csv",
        vec![
            svec!["id", "name"],
            svec!["1", "JM Bloggs"],
            svec!["2", "F. J. Bloggs"],
            svec!["3", "J. Bloggs"],
        ],
    );
    let mut cmd = wrk.command("replace");
    cmd.arg("--exact")
        .arg("J. Bloggs")
        .arg("John Bloggs")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // Should only replace exact match "J. Bloggs", not "F. J. Bloggs"
    let expected = vec![
        svec!["id", "name"],
        svec!["1", "JM Bloggs"],
        svec!["2", "F. J. Bloggs"],
        svec!["3", "John Bloggs"],
    ];
    assert_eq!(got, expected);

    let got_err = wrk.output_stderr(&mut cmd);
    let expected_err = "1\n";
    assert_eq!(got_err, expected_err);

    wrk.assert_success(&mut cmd);
}

#[test]
fn replace_exact_with_special_chars() {
    let wrk = Workdir::new("replace_exact_with_special_chars");
    wrk.create(
        "data.csv",
        vec![
            svec!["identifier", "color"],
            svec!["164.0", "yel$low^"],
            svec!["165.0", "yellow"],
            svec!["166.0", "$low^"],
            svec!["167.0", "yel$low^.0"],
        ],
    );
    let mut cmd = wrk.command("replace");
    cmd.arg("--exact")
        .arg("yel$low^")
        .arg("yellow")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // Should only replace exact field match, not substring
    let expected = vec![
        svec!["identifier", "color"],
        svec!["164.0", "yellow"],
        svec!["165.0", "yellow"],
        svec!["166.0", "$low^"],
        svec!["167.0", "yel$low^.0"],
    ];
    assert_eq!(got, expected);

    let got_err = wrk.output_stderr(&mut cmd);
    let expected_err = "1\n";
    assert_eq!(got_err, expected_err);

    wrk.assert_success(&mut cmd);
}

#[test]
fn replace_exact_no_substring_match() {
    let wrk = Workdir::new("replace_exact_no_substring_match");
    wrk.create(
        "data.csv",
        vec![
            svec!["id", "name"],
            svec!["1", "JM Bloggs"],
            svec!["2", "F. J. Bloggs"],
            svec!["3", "J. Bloggs"],
        ],
    );
    let mut cmd = wrk.command("replace");
    cmd.arg("--exact")
        .arg("J. Bloggs")
        .arg("REPLACED")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // Should NOT replace "F. J. Bloggs" even though it contains "J. Bloggs"
    let expected = vec![
        svec!["id", "name"],
        svec!["1", "JM Bloggs"],
        svec!["2", "F. J. Bloggs"],
        svec!["3", "REPLACED"],
    ];
    assert_eq!(got, expected);

    let got_err = wrk.output_stderr(&mut cmd);
    let expected_err = "1\n";
    assert_eq!(got_err, expected_err);

    wrk.assert_success(&mut cmd);
}

#[test]
fn replace_exact_case_insensitive() {
    let wrk = Workdir::new("replace_exact_case_insensitive");
    wrk.create(
        "data.csv",
        vec![
            svec!["id", "name"],
            svec!["1", "JM Bloggs"],
            svec!["2", "F. J. Bloggs"],
            svec!["3", "J. Bloggs"],
            svec!["4", "j. bloggs"],
        ],
    );
    let mut cmd = wrk.command("replace");
    cmd.arg("--exact")
        .arg("--ignore-case")
        .arg("j. bloggs")
        .arg("John Bloggs")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // Should replace both "J. Bloggs" and "j. bloggs" with case-insensitive exact match
    let expected = vec![
        svec!["id", "name"],
        svec!["1", "JM Bloggs"],
        svec!["2", "F. J. Bloggs"],
        svec!["3", "John Bloggs"],
        svec!["4", "John Bloggs"],
    ];
    assert_eq!(got, expected);

    let got_err = wrk.output_stderr(&mut cmd);
    let expected_err = "2\n";
    assert_eq!(got_err, expected_err);

    wrk.assert_success(&mut cmd);
}

#[test]
fn replace_exact_with_select() {
    let wrk = Workdir::new("replace_exact_with_select");
    wrk.create(
        "data.csv",
        vec![
            svec!["id", "name", "email"],
            svec!["1", "test", "test@example.com"],
            svec!["2", "test", "other@example.com"],
            svec!["3", "testing", "test@example.com"],
        ],
    );
    let mut cmd = wrk.command("replace");
    cmd.arg("--exact")
        .arg("--select")
        .arg("name")
        .arg("test")
        .arg("REPLACED")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // Should only replace exact "test" in name column, not "testing"
    let expected = vec![
        svec!["id", "name", "email"],
        svec!["1", "REPLACED", "test@example.com"],
        svec!["2", "REPLACED", "other@example.com"],
        svec!["3", "testing", "test@example.com"],
    ];
    assert_eq!(got, expected);

    let got_err = wrk.output_stderr(&mut cmd);
    let expected_err = "2\n";
    assert_eq!(got_err, expected_err);

    wrk.assert_success(&mut cmd);
}

#[test]
fn replace_all_emails_with_placeholder() {
    let wrk = Workdir::new("replace_all_emails_with_placeholder");
    wrk.create(
        "data.csv",
        vec![
            svec!["email"],
            svec!["test@example.com"],
            svec!["other@example.com"],
            svec!["test@example.com"],
            svec!["NOT an email"],
            svec!["johm.doe@gmail.org"],
            svec!["jane.doe+amazon@gmail.com"],
            svec!["hello world"],
        ],
    );
    let mut cmd = wrk.command("replace");
    cmd.arg(r"([a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,})")
        .arg("<EMAIL>")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["email"],
        svec!["<EMAIL>"],
        svec!["<EMAIL>"],
        svec!["<EMAIL>"],
        svec!["NOT an email"],
        svec!["<EMAIL>"],
        svec!["<EMAIL>"],
        svec!["hello world"],
    ];
    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd);
}

#[test]
fn replace_indexed_parallel() {
    let wrk = Workdir::new("replace_indexed_parallel");
    let data = wrk.load_test_resource("NYC311-5.csv");
    wrk.create_from_string("data.csv", &data);

    // replace "Police" with "Pulisya" (tagalog for "Police")
    let mut cmd = wrk.command("replace");
    cmd.arg("Police").arg("Pulisya").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![
            "Unique Key",
            "Created Date",
            "Closed Date",
            "Agency",
            "Agency Name",
            "Complaint Type",
            "Descriptor",
            "Location Type",
            "Incident Zip",
            "Incident Address",
            "Street Name",
            "Cross Street 1",
            "Cross Street 2",
            "Intersection Street 1",
            "Intersection Street 2",
            "Address Type",
            "City",
            "Landmark",
            "Facility Type",
            "Status",
            "Due Date",
            "Resolution Description",
            "Resolution Action Updated Date",
            "Community Board",
            "BBL",
            "Borough",
            "X Coordinate (State Plane)",
            "Y Coordinate (State Plane)",
            "Open Data Channel Type",
            "Park Facility Name",
            "Park Borough",
            "Vehicle Type",
            "Taxi Company Borough",
            "Taxi Pick Up Location",
            "Bridge Highway Name",
            "Bridge Highway Direction",
            "Road Ramp",
            "Bridge Highway Segment",
            "Latitude",
            "Longitude",
            "Location"
        ],
        svec![
            "34675190",
            "10/31/2016 11:07:15 PM",
            "10/31/2016 11:25:37 PM",
            "NYPD",
            "New York City Pulisya Department",
            "Noise - Residential",
            "Banging/Pounding",
            "Residential Building/House",
            "10035",
            "117 EAST 118 STREET",
            "EAST 118 STREET",
            "PARK AVENUE",
            "LEXINGTON AVENUE",
            "",
            "",
            "ADDRESS",
            "NEW YORK",
            "",
            "Precinct",
            "Closed",
            "11/01/2016 07:07:15 AM",
            "The Pulisya Department responded to the complaint and determined that police action \
             was not necessary.",
            "10/31/2016 11:25:37 PM",
            "11 MANHATTAN",
            "1017670005",
            "MANHATTAN",
            "1000445",
            "230851",
            "MOBILE",
            "Unspecified",
            "MANHATTAN",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "40.8002938",
            "-73.9415055",
            "(40.8002938, -73.9415055)"
        ],
        svec![
            "42096612",
            "03/30/2019 04:06:23 AM",
            "03/30/2019 04:15:23 AM",
            "NYPD",
            "New York City Pulisya Department",
            "Noise - Residential",
            "Banging/Pounding",
            "Residential Building/House",
            "10025",
            "4 WEST 105 STREET",
            "WEST 105 STREET",
            "CENTRAL PARK WEST",
            "MANHATTAN AVENUE",
            "",
            "",
            "ADDRESS",
            "NEW YORK",
            "",
            "Precinct",
            "Closed",
            "03/30/2019 12:06:23 PM",
            "The Pulisya Department responded to the complaint and with the information available \
             observed no evidence of the violation at that time.",
            "03/30/2019 04:15:23 AM",
            "07 MANHATTAN",
            "1018400037",
            "MANHATTAN",
            "995106",
            "229820",
            "ONLINE",
            "Unspecified",
            "MANHATTAN",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "40.7974721",
            "-73.960791",
            "(40.7974721, -73.960791)"
        ],
        svec![
            "20520945",
            "05/27/2011 12:00:00 AM",
            "",
            "HPD",
            "Department of Housing Preservation and Development",
            "PAINT - PLASTER",
            "WALLS",
            "RESIDENTIAL BUILDING",
            "11225",
            "1700 BEDFORD AVENUE",
            "BEDFORD AVENUE",
            "MONTGOMERY STREET",
            "SULLIVAN PLACE",
            "",
            "",
            "ADDRESS",
            "BROOKLYN",
            "",
            "N/A",
            "Open",
            "",
            "The following complaint conditions are still open.HPD may attempt to contact you to \
             verify the correction of the condition or may conduct an inspection.",
            "06/15/2011 12:00:00 AM",
            "09 BROOKLYN",
            "3013020001",
            "BROOKLYN",
            "996197",
            "181752",
            "UNKNOWN",
            "Unspecified",
            "BROOKLYN",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            ""
        ],
        svec![
            "39773697",
            "07/18/2018 11:10:04 AM",
            "07/18/2018 11:53:46 PM",
            "NYPD",
            "New York City Pulisya Department",
            "Noise - Street/Sidewalk",
            "Loud Talking",
            "Street/Sidewalk",
            "11373",
            "48-10 91 PLACE",
            "91 PLACE",
            "48 AVENUE",
            "50 AVENUE",
            "",
            "",
            "ADDRESS",
            "ELMHURST",
            "",
            "Precinct",
            "Closed",
            "07/18/2018 07:10:04 PM",
            "The Pulisya Department reviewed your complaint and provided additional information \
             below.",
            "07/18/2018 11:53:46 PM",
            "04 QUEENS",
            "4018500012",
            "QUEENS",
            "1019446",
            "209453",
            "MOBILE",
            "Unspecified",
            "QUEENS",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "40.7415066",
            "-73.8729882",
            "(40.7415066, -73.8729882)"
        ],
        svec![
            "47976463",
            "10/25/2020 01:19:00 PM",
            "10/26/2020 02:30:00 AM",
            "DEP",
            "Department of Environmental Protection",
            "Water System",
            "Hydrant Leaking (WC1)",
            "",
            "10033",
            "52 PINEHURST AVENUE",
            "PINEHURST AVENUE",
            "W 179 ST",
            "W 180 ST",
            "",
            "",
            "ADDRESS",
            "NEW YORK",
            "",
            "",
            "Closed",
            "",
            "The Department of Environmental Protection investigated this complaint and shut the \
             running hydrant.",
            "10/26/2020 02:30:00 AM",
            "12 MANHATTAN",
            "1021770161",
            "MANHATTAN",
            "1000926",
            "248966",
            "ONLINE",
            "Unspecified",
            "MANHATTAN",
            "",
            "",
            "",
            "",
            "",
            "",
            "",
            "40.85001341890072",
            "-73.93972316718485",
            "(40.85001341890072, -73.93972316718485)"
        ],
    ];
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);

    // now index the file
    wrk.create_from_string("data.csv", &data);
    let mut cmd = wrk.command("index");
    cmd.arg("data.csv");
    wrk.assert_success(&mut cmd);

    // get the timestamp of the index file
    let index_file = wrk.path("data.csv.idx");
    let index_file_metadata = std::fs::metadata(index_file).unwrap();
    let index_file_timestamp = index_file_metadata.modified().unwrap();

    std::thread::sleep(std::time::Duration::from_secs(2));

    // should still have the same output
    let mut cmd = wrk.command("replace");
    cmd.arg("Police")
        .arg("Pulisya")
        .arg("data.csv")
        .arg("--jobs")
        .arg("2");
    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);

    // get the timestamp of the index file
    // it should be higher than the original timestamp
    // as we auto-reindex the file after a replace operation
    let index_file = wrk.path("data.csv.idx");
    let index_file_metadata = std::fs::metadata(index_file).unwrap();
    let index_file_timestamp_new = index_file_metadata.modified().unwrap();
    assert!(index_file_timestamp_new > index_file_timestamp);
}
