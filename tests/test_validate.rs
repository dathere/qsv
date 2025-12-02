use crate::workdir::Workdir;

#[test]
fn validate_good_csv() {
    let wrk = Workdir::new("validate").flexible(true);
    wrk.create(
        "data.csv",
        vec![
            svec!["title", "name", "age"],
            svec!["Professor", "Xaviers", "60"],
            svec!["Prisoner", "Magneto", "90"],
            svec!["First Class Student", "Iceman", "14"],
        ],
    );
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv");

    wrk.assert_success(&mut cmd);
}

#[test]
fn validate_good_tab() {
    let wrk = Workdir::new("validate_good_tab").flexible(true);
    let tabfile = wrk.load_test_file("boston311-100.tab");
    let mut cmd = wrk.command("validate");
    cmd.arg(tabfile);

    wrk.assert_success(&mut cmd);

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"Valid: 29 Columns: ("case_enquiry_id", "open_dt", "target_dt", "closed_dt", "ontime", "case_status", "closure_reason", "case_title", "subject", "reason", "type", "queue", "department", "submittedphoto", "closedphoto", "location", "fire_district", "pwd_district", "city_council_district", "police_district", "neighborhood", "neighborhood_services_district", "ward", "precinct", "location_street_name", "location_zipcode", "latitude", "longitude", "source"); Records: 100; Delimiter: TAB"#;
    assert_eq!(got, expected);
}

#[test]
fn validate_bad_tsv() {
    let wrk = Workdir::new("validate_bad_tsv").flexible(true);
    let tabfile = wrk.load_test_file("boston311-100-bad.tsv");
    let mut cmd = wrk.command("validate");
    cmd.arg(tabfile);

    wrk.assert_err(&mut cmd);
}

#[test]
fn validate_good_csv_msg() {
    let wrk = Workdir::new("validate_good_csv_msg").flexible(true);
    wrk.create(
        "data.csv",
        vec![
            svec!["title", "name", "real age (earth years)"],
            svec!["Professor", "Xaviers", "60"],
            svec!["Prisoner", "Magneto", "90"],
            svec!["First Class Student", "Iceman", "14"],
        ],
    );
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"Valid: 3 Columns: ("title", "name", "real age (earth years)"); Records: 3; Delimiter: ,"#;
    assert_eq!(got, expected);
}

#[test]
fn validate_empty_csv_msg() {
    let wrk = Workdir::new("validate_empty_csv_msg").flexible(true);
    wrk.create(
        "data.csv",
        vec![svec!["title", "name", "real age (earth years)"]],
    );
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"Valid: 3 Columns: ("title", "name", "real age (earth years)"); Records: 0; Delimiter: ,"#;
    assert_eq!(got, expected);
}

#[test]
fn validate_good_csv_pretty_json() {
    let wrk = Workdir::new("validate_good_csv_pretty_json").flexible(true);
    wrk.create(
        "data.csv",
        vec![
            svec!["title", "name", "real age (earth years)"],
            svec!["Professor", "Xaviers", "60"],
            svec!["Prisoner", "Magneto", "90"],
            svec!["First Class Student", "Iceman", "14"],
        ],
    );
    let mut cmd = wrk.command("validate");
    cmd.arg("--pretty-json").arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{
  "delimiter_char": ",","header_row": true,"quote_char": "\"","num_records": 3,"num_fields": 3,"fields": [
    "title",
    "name",
    "real age (earth years)"
  ]
}"#;
    assert_eq!(got, expected);
}

#[test]
fn validate_good_csv_json() {
    let wrk = Workdir::new("validate_good_csv_json").flexible(true);
    wrk.create(
        "data.csv",
        vec![
            svec!["title", "name", "age"],
            svec!["Professor", "Xaviers", "60"],
            svec!["Prisoner", "Magneto", "90"],
            svec!["First Class Student", "Iceman", "14"],
        ],
    );
    let mut cmd = wrk.command("validate");
    cmd.arg("--json").arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"delimiter_char":",","header_row":true,"quote_char":"\"","num_records":3,"num_fields":3,"fields":["title","name","age"]}"#;
    assert_eq!(got, expected);
}

#[test]
fn validate_bad_csv() {
    let wrk = Workdir::new("validate").flexible(true);
    wrk.create(
        "data.csv",
        vec![
            svec!["title", "name", "age"],
            svec!["Professor", "Xaviers", "60"],
            svec!["Magneto", "90",],
            svec!["First Class Student", "Iceman", "14"],
        ],
    );
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv");

    let got: String = wrk.output_stderr(&mut cmd);
    let expected = r#"Validation error: CSV error: record 2 (line: 3, byte: 36): found record with 2 fields, but the previous record has 3 fields.
Use `qsv fixlengths` to fix record length issues.
"#;
    assert_eq!(got, expected);

    wrk.assert_err(&mut cmd);
}

#[test]
fn validate_bad_csv_first_record() {
    let wrk = Workdir::new("validate_bad_csv_first_record").flexible(true);
    wrk.create(
        "data.csv",
        vec![
            svec!["title", "name", "age"],
            svec!["Professor", "Xaviers",],
            svec!["Doctor", "Magneto", "90",],
            svec!["First Class Student", "Iceman", "14"],
        ],
    );
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv");

    let got: String = wrk.output_stderr(&mut cmd);
    let expected = r#"Validation error: CSV error: record 1 (line: 2, byte: 15): found record with 2 fields, but the previous record has 3 fields.
Use `qsv fixlengths` to fix record length issues.
"#;
    assert_eq!(got, expected);

    wrk.assert_err(&mut cmd);
}

#[test]
fn validate_bad_csv_last_record() {
    let wrk = Workdir::new("validate_bad_csv_last_record").flexible(true);
    wrk.create(
        "data.csv",
        vec![
            svec!["title", "name", "age"],
            svec!["Professor", "Xaviers", "60"],
            svec!["Doctor", "Magneto", "90"],
            svec!["First Class Student", "Iceman", "14", "extra field"],
        ],
    );
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv");

    let got: String = wrk.output_stderr(&mut cmd);
    let expected = r#"Validation error: CSV error: record 3 (line: 4, byte: 54): found record with 4 fields, but the previous record has 3 fields.
Use `qsv fixlengths` to fix record length issues.
"#;
    assert_eq!(got, expected);

    wrk.assert_err(&mut cmd);
}

#[test]
fn validate_bad_csv_prettyjson() {
    let wrk = Workdir::new("validate_bad_csv_prettyjson").flexible(true);
    wrk.create(
        "data.csv",
        vec![
            svec!["title", "name", "age"],
            svec!["Professor", "Xaviers", "60"],
            svec!["Magneto", "90",],
            svec!["First Class Student", "Iceman", "14"],
        ],
    );
    let mut cmd = wrk.command("validate");
    cmd.arg("--pretty-json").arg("data.csv");

    let got: String = wrk.output_stderr(&mut cmd);
    let expected = r#"{
  "errors": [
    {
      "title": "Validation error",
      "detail": "CSV error: record 2 (line: 3, byte: 36): found record with 2 fields, but the previous record has 3 fields",
      "meta": {
        "last_valid_record": "1"
      }
    }
  ]
}
"#;
    assert_eq!(got, expected);

    wrk.assert_err(&mut cmd);
}

fn adur_errors() -> &'static str {
    r#"row_number	field	error
1	ExtractDate	null is not of type "string"
1	OrganisationLabel	null is not of type "string"
3	CoordinateReferenceSystem	"OSGB3" does not match "(WGS84|OSGB36)"
3	Category	"Mens" does not match "(Female|Male|Female and Male|Unisex|Male urinal|Children only|None)"
"#
}

// invalid records with index from original csv
// row 1: missing values for ExtractDate and OrganisationLabel
// row 3: wrong value for CoordinateReferenceSystem and Category
// note: removed unnecessary quotes for string column "OpeningHours"
fn adur_invalids() -> &'static str {
    r#"ExtractDate,OrganisationURI,OrganisationLabel,ServiceTypeURI,ServiceTypeLabel,LocationText,CoordinateReferenceSystem,GeoX,GeoY,GeoPointLicensingURL,Category,AccessibleCategory,RADARKeyNeeded,BabyChange,FamilyToilet,ChangingPlace,AutomaticPublicConvenience,FullTimeStaffing,PartOfCommunityScheme,CommunitySchemeName,ChargeAmount,InfoURL,OpeningHours,ManagedBy,ReportEmail,ReportTel,Notes,UPRN,Postcode,StreetAddress,GeoAreaURI,GeoAreaLabel
,http://opendatacommunities.org/id/district-council/adur,,http://id.esd.org.uk/service/579,Public toilets,BEACH GREEN PUBLIC CONVENIENCES BRIGHTON ROAD LANCING,OSGB36,518072,103649,http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html,Female and male,Unisex,Yes,No,No,No,No,No,No,,,http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/,S = 09:00 - 21:00 W = 09:00 - 17:00 ,ADC,surveyor_1@adur-worthing.gov.uk,01903 221471,,60001449,,BEACH GREEN PUBLIC CONVENIENCES BRIGHTON ROAD LANCING,,
2014-07-07 00:00,http://opendatacommunities.org/id/district-council/adur,Adur,http://id.esd.org.uk/service/579,Public toilets,PUBLIC CONVENIENCES SHOPSDAM ROAD LANCING,OSGB3,518915,103795,http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html,Mens,Unisex,Yes,No,No,No,No,No,No,,,http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/,S = 09:00 - 21:00 W = 09:00 - 17:00,ADC,surveyor_3@adur-worthing.gov.uk,01903 221471,,60007428,,,,
"#
}

#[test]
fn validate_adur_public_toilets_dataset_with_json_schema() {
    let wrk = Workdir::new("validate").flexible(true);

    // copy schema file to workdir
    let schema: String = wrk.load_test_resource("public-toilets-schema.json");
    wrk.create_from_string("schema.json", &schema);

    // copy csv file to workdir
    let csv: String = wrk.load_test_resource("adur-public-toilets.csv");
    wrk.create_from_string("data.csv", &csv);

    // run validate command
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv").arg("schema.json");

    wrk.output(&mut cmd);

    // check invalid file output
    let invalid_output: String = wrk.from_str(&wrk.path("data.csv.invalid"));
    assert_eq!(adur_invalids().to_string(), invalid_output);

    // check validation error output

    let validation_error_output: String = wrk.from_str(&wrk.path("data.csv.validation-errors.tsv"));
    assert_eq!(adur_errors(), validation_error_output);
    wrk.assert_err(&mut cmd);
}

#[test]
fn validate_adur_public_toilets_dataset_with_json_schema_valid_output() {
    let wrk = Workdir::new("validate_valid_output").flexible(true);

    // copy schema file to workdir
    let schema: String = wrk.load_test_resource("public-toilets-schema.json");
    wrk.create_from_string("schema.json", &schema);

    // copy csv file to workdir
    let csv: String = wrk.load_test_resource("adur-public-toilets-valid.csv");
    wrk.create_from_string("data.csv", &csv);

    // run validate command
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv")
        .arg("schema.json")
        .args(["--valid-output", "-"]);

    let out = wrk.output_stderr(&mut cmd);
    let expected = "13\n";
    assert_eq!(out, expected);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["ExtractDate", "OrganisationURI", "OrganisationLabel", "ServiceTypeURI", "ServiceTypeLabel", "LocationText", "CoordinateReferenceSystem", "GeoX", "GeoY", "GeoPointLicensingURL", "Category", "AccessibleCategory", "RADARKeyNeeded", "BabyChange", "FamilyToilet", "ChangingPlace", "AutomaticPublicConvenience", "FullTimeStaffing", "PartOfCommunityScheme", "CommunitySchemeName", "ChargeAmount", "InfoURL", "OpeningHours", "ManagedBy", "ReportEmail", "ReportTel", "Notes", "UPRN", "Postcode", "StreetAddress", "GeoAreaURI", "GeoAreaLabel"], 
        svec!["07/07/2014 00:00", "http://opendatacommunities.org/id/district-council/adur", "Adur", "http://id.esd.org.uk/service/579", "Public toilets", "PUBLIC CONVENIENCES MONKS RECREATION GROUND CRABTREE LANE LANCING", "OSGB36", "518225", "104730", "http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html", "Female and male", "None", "Yes", "No", "No", "No", "No", "No", "No", "", "", "http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/", "S = 09:00 - 15:00 W = 09:00 - 15:00", "ADC", "surveyor_2@adur-worthing.gov.uk", "01903 221471", "", "60002210", "", "PUBLIC CONVENIENCES MONKS RECREATION GROUND CRABTREE LANE LANCING", "", ""], 
        svec!["07/07/2014 00:00", "http://opendatacommunities.org/id/district-council/adur", "Adur", "http://id.esd.org.uk/service/579", "Public toilets", "PUBLIC CONVENIENCES YEW TREE CLOSE LANCING", "OSGB36", "518222", "104168", "http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html", "Female and male", "Unisex", "Yes", "No", "No", "No", "No", "No", "No", "", "", "http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/", "S = 09:00 - 21:00 W = 09:00 - 17:00", "ADC", "surveyor_4@adur-worthing.gov.uk", "01903 221471", "", "60008859", "", "PUBLIC CONVENIENCES YEW TREE CLOSE LANCING", "", ""], 
        svec!["07/07/2014 00:00", "http://opendatacommunities.org/id/district-council/adur", "Adur", "http://id.esd.org.uk/service/579", "Public toilets", "PUBLIC CONVENIENCES BEACH GREEN SHOREHAM-BY-SEA", "OSGB36", "521299", "104515", "http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html", "Female and male", "Unisex", "Yes", "No", "No", "No", "No", "No", "No", "", "", "http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/", "S = 09:00 - 21:00 W = 09:00 - 17:00", "ADC", "surveyor_5@adur-worthing.gov.uk", "01903 221471", "", "60009402", "", "PUBLIC CONVENIENCES BEACH GREEN SHOREHAM-BY-SEA", "", ""], 
        svec!["07/07/2014 00:00", "http://opendatacommunities.org/id/district-council/adur", "Adur", "http://id.esd.org.uk/service/579", "Public toilets", "PUBLIC CONVENIENCES ADUR RECREATION GROUND BRIGHTON ROAD SHOREHAM-BY-SEA", "OSGB36", "521048", "104977", "http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html", "Female and male", "Unisex", "Yes", "No", "No", "No", "No", "No", "No", "", "", "http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/", "S = 08:00 - 21:00 W = 08:00 - 17:00", "ADC", "surveyor_6@adur-worthing.gov.uk", "01903 221471", "", "60009666", "", "PUBLIC CONVENIENCES ADUR RECREATION GROUND BRIGHTON ROAD SHOREHAM-BY-SEA", "", ""], 
        svec!["07/07/2014 00:00", "http://opendatacommunities.org/id/district-council/adur", "Adur", "http://id.esd.org.uk/service/579", "Public toilets", "PUBLIC CONVENIENCES FORTHAVEN SHOREHAM-BY-SEA", "OSGB36", "523294", "104588", "http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html", "Female and male", "Unisex", "Yes", "No", "No", "No", "No", "No", "No", "", "", "http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/", "S = 09:00 - 21:00 W = 09:00 - 17:00", "ADC", "surveyor_7@adur-worthing.gov.uk", "01903 221471", "", "60011970", "", "PUBLIC CONVENIENCES FORTHAVEN SHOREHAM-BY-SEA", "", ""], 
        svec!["07/07/2014 00:00", "http://opendatacommunities.org/id/district-council/adur", "Adur", "http://id.esd.org.uk/service/579", "Public toilets", "PUBLIC CONVENIENCES MIDDLE STREET SHOREHAM-BY-SEA", "OSGB36", "521515", "105083", "http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html", "Female and male", "Unisex", "Yes", "No", "No", "No", "No", "No", "No", "", "", "http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/", "S = 09:00 - 21:00 W = 09:00 - 17:00", "ADC", "surveyor_8@adur-worthing.gov.uk", "01903 221471", "", "60014163", "", "PUBLIC CONVENIENCES MIDDLE STREET SHOREHAM-BY-SEA", "", ""], 
        svec!["07/07/2014 00:00", "http://opendatacommunities.org/id/district-council/adur", "Adur", "http://id.esd.org.uk/service/579", "Public toilets", "PUBLIC CONVENIENCES CEMETERY MILL LANE SHOREHAM-BY-SEA", "OSGB36", "521440", "105725", "http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html", "Female and male", "None", "No", "No", "No", "No", "No", "No", "No", "", "", "http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/", "", "ADC", "surveyor_9@adur-worthing.gov.uk", "01903 221471", "Grounds staff only not public", "60014340", "", "PUBLIC CONVENIENCES CEMETERY MILL LANE SHOREHAM-BY-SEA", "", ""], 
        svec!["07/07/2014 00:00", "http://opendatacommunities.org/id/district-council/adur", "Adur", "http://id.esd.org.uk/service/579", "Public toilets", "PUBLIC CONVENIENCES SOUTH PAVILION BUCKINGHAM PARK UPPER SHOREHAM ROAD SHOREHAM-BY-SEA", "OSGB36", "522118", "105939", "http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html", "Female and male", "None", "No", "No", "No", "No", "No", "No", "No", "", "", "http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/", "S = 09:00 - 21:00 W = 09:00 - 17:00", "ADC", "surveyor_10@adur-worthing.gov.uk", "01903 221471", "", "60017866", "", "PUBLIC CONVENIENCES SOUTH PAVILION BUCKINGHAM PARK UPPER SHOREHAM ROAD SHOREHAM-BY-SEA", "", ""], 
        svec!["07/07/2014 00:00", "http://opendatacommunities.org/id/district-council/adur", "Adur", "http://id.esd.org.uk/service/579", "Public toilets", "PUBLIC CONVENIENCE SOUTHWICK STREET SOUTHWICK", "OSGB36", "524401", "105405", "http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html", "Female and male", "Unisex", "Yes", "No", "No", "No", "No", "No", "No", "", "", "http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/", "S = 08:00 - 21:00 W = 08:00 - 17:00", "ADC", "surveyor_11@adur-worthing.gov.uk", "01903 221471", "", "60026354", "", "PUBLIC CONVENIENCE SOUTHWICK STREET SOUTHWICK", "", ""], 
        svec!["07/07/2014 00:00", "http://opendatacommunities.org/id/district-council/adur", "Adur", "http://id.esd.org.uk/service/579", "Public toilets", "WEST BEACH PUBLIC CONVENIENCES WEST BEACH ROAD LANCING", "OSGB36", "520354", "104246", "http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html", "Female and male", "Unisex", "Yes", "No", "No", "No", "No", "No", "No", "", "", "http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/", "S = 09:00 - 21:00 W = 09:00 - 17:00", "", "surveyor_12@adur-worthing.gov.uk", "01903 221471", "", "60028994", "", "WEST BEACH PUBLIC CONVENIENCES WEST BEACH ROAD LANCING", "", ""], 
        svec!["07/07/2014 00:00", "http://opendatacommunities.org/id/district-council/adur", "Adur", "http://id.esd.org.uk/service/579", "Public toilets", "BEACH TOILETS BASIN ROAD SOUTH SOUTHWICK", "OSGB36", "524375", "104753", "http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html", "Female and male", "Unisex", "Yes", "No", "No", "No", "No", "No", "No", "", "", "http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/", "S = 09:00 - 21:00 W = 09:00 - 17:00", "ADC", "surveyor_13@adur-worthing.gov.uk", "01903 221471", "", "60029181", "", "BEACH TOILETS BASIN ROAD SOUTH SOUTHWICK", "", ""], 
        svec!["07/07/2014 00:00", "http://opendatacommunities.org/id/district-council/adur", "Adur", "http://id.esd.org.uk/service/579", "Public toilets", "BEACH TOILETS BASIN ROAD SOUTH SOUTHWICK", "OSGB36", "522007", "106062", "http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html", "Female and male", "None", "No", "No", "No", "No", "No", "No", "No", "", "", "http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/", "", "ADC", "surveyor_14@adur-worthing.gov.uk", "01903 221471", "Grounds staff only not public", "60032527", "", "PUBLIC CONVENIENCE NORTH PAVILION BUCKINGHAM PARK UPPER SHOREHAM ROAD SHOREHAM-BY-SEA", "", ""], 
        svec!["07/07/2014 00:00", "http://opendatacommunities.org/id/district-council/adur", "Adur", "http://id.esd.org.uk/service/579", "Public toilets", "BEACH TOILETS BASIN ROAD SOUTH SOUTHWICK", "OSGB36", "522083", "105168", "http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html", "Female and male", "Unisex", "Yes", "No", "No", "No", "No", "No", "No", "", "", "http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/", "09.00 - 17.00", "ADC", "surveyor_15@adur-worthing.gov.uk", "01903 221471", "", "60034215", "", "PUBLIC CONVENIENCES CIVIC CENTRE HAM ROAD SHOREHAM-BY-SEA", "", ""]    
    ];
    assert_eq!(got, expected);

    wrk.assert_err(&mut cmd);
}

#[test]
fn validate_with_schema_noheader() {
    let wrk = Workdir::new("validate_with_schema_noheader").flexible(true);

    // copy schema file to workdir
    let schema: String = wrk.load_test_resource("public-toilets-schema.json");
    wrk.create_from_string("schema.json", &schema);

    // copy csv file to workdir
    let csv: String = wrk.load_test_resource("adur-public-toilets-valid.csv");
    wrk.create_from_string("data.csv", &csv);

    // run validate command
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv")
        .arg("schema.json")
        .arg("--no-headers")
        .args(["--valid-output", "-"]);

    let got = wrk.output_stderr(&mut cmd);
    let expected = "Cannot validate CSV without headers against a JSON Schema.\n".to_string();
    assert_eq!(got, expected);

    wrk.assert_err(&mut cmd);
}

#[test]
fn validate_adur_public_toilets_dataset_with_json_schema_url() {
    let wrk = Workdir::new("validate").flexible(true);

    // copy csv file to workdir
    let csv: String = wrk.load_test_resource("adur-public-toilets.csv");
    wrk.create_from_string("data.csv", &csv);

    // run validate command
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv").arg("https://raw.githubusercontent.com/dathere/qsv/master/resources/test/public-toilets-schema.json");

    wrk.output(&mut cmd);

    let invalid_output: String = wrk.from_str(&wrk.path("data.csv.invalid"));
    assert_eq!(adur_invalids().to_string(), invalid_output);

    // check validation error output
    let validation_error_output: String = wrk.from_str(&wrk.path("data.csv.validation-errors.tsv"));
    assert_eq!(adur_errors(), validation_error_output);
    wrk.assert_err(&mut cmd);
}

#[test]
fn validate_dynenum_with_column() {
    let wrk = Workdir::new("validate_dynenum_with_column").flexible(true);

    // Create lookup file first
    wrk.create(
        "lookup.csv",
        vec![
            svec!["code", "name", "category"],
            svec!["A1", "Apple", "fruit"],
            svec!["B2", "Banana", "fruit"],
            svec!["C3", "Carrot", "vegetable"],
        ],
    );

    // Create test data
    wrk.create(
        "data.csv",
        vec![
            svec!["id", "product", "type"],
            svec!["1", "Apple", "fruit"],
            svec!["2", "Banana", "fruit"],
            svec!["3", "Orange", "fruit"], // Invalid - not in lookup
            svec!["4", "Grape", "fruit"],  // Invalid - not in lookup
        ],
    );

    // Create schema using dynamicEnum with column specification
    wrk.create_from_string(
        "schema.json",
        r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "id": { "type": "string" },
                "product": { 
                    "type": "string",
                    "dynamicEnum": "lookup.csv|name"
                },
                "type": { "type": "string" }
            }
        }"#,
    );

    // Run validate command
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv").arg("schema.json");
    wrk.output(&mut cmd);

    wrk.assert_err(&mut cmd);

    // Check validation-errors.tsv
    let validation_errors: String = wrk.from_str(&wrk.path("data.csv.validation-errors.tsv"));

    let expected_errors = r#"row_number	field	error
3	product	"Orange" is not a valid dynamicEnum value
4	product	"Grape" is not a valid dynamicEnum value
"#;
    assert_eq!(validation_errors, expected_errors);

    // Check valid records
    let valid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.valid");
    let expected_valid = vec![svec!["1", "Apple", "fruit"], svec!["2", "Banana", "fruit"]];
    assert_eq!(valid_records, expected_valid);

    // Check invalid records
    let invalid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.invalid");
    let expected_invalid = vec![svec!["3", "Orange", "fruit"], svec!["4", "Grape", "fruit"]];
    assert_eq!(invalid_records, expected_invalid);

    wrk.assert_err(&mut cmd);
}

#[test]
fn validate_dynenum_with_column_index() {
    let wrk = Workdir::new("validate_dynenum_with_column_index").flexible(true);

    // Create a sample CSV file with multiple columns
    wrk.create(
        "lookup.csv",
        vec![
            svec!["code", "name", "category"],
            svec!["A1", "Apple", "fruit"],
            svec!["B2", "Banana", "fruit"],
            svec!["C3", "Carrot", "vegetable"],
        ],
    );

    // Create test data
    wrk.create(
        "data.csv",
        vec![
            svec!["id", "category", "code"],
            svec!["1", "fruit", "A1"],
            svec!["2", "vegetable", "D4"], // Invalid - code not in lookup
            svec!["3", "fruit", "B2"],
            svec!["4", "fruit", "X9"], // Invalid - code not in lookup
        ],
    );

    // Create schema using dynamicEnum with column index
    wrk.create_from_string(
        "schema.json",
        r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "id": { "type": "string" },
                "category": { "type": "string" },
                "code": { 
                    "type": "string",
                    "dynamicEnum": "lookup.csv|0"
                }
            }
        }"#,
    );

    // Run validate command
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv").arg("schema.json");
    wrk.output(&mut cmd);

    wrk.assert_err(&mut cmd);

    // Check validation-errors.tsv
    let validation_errors = wrk
        .read_to_string("data.csv.validation-errors.tsv")
        .unwrap();
    let expected_errors = "row_number\tfield\terror\n2\tcode\t\"D4\" is not a valid dynamicEnum \
                           value\n4\tcode\t\"X9\" is not a valid dynamicEnum value\n";
    assert_eq!(validation_errors, expected_errors);

    // Check valid records
    let valid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.valid");
    let expected_valid = vec![svec!["1", "fruit", "A1"], svec!["3", "fruit", "B2"]];
    assert_eq!(valid_records, expected_valid);

    // Check invalid records
    let invalid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.invalid");
    let expected_invalid = vec![svec!["2", "vegetable", "D4"], svec!["4", "fruit", "X9"]];
    assert_eq!(invalid_records, expected_invalid);

    wrk.assert_err(&mut cmd);
}

#[test]
fn validate_dynenum_with_invalid_column() {
    let wrk = Workdir::new("validate_dynenum_with_invalid_column").flexible(true);

    // Create lookup file first
    wrk.create(
        "lookup.csv",
        vec![
            svec!["code", "name"],
            svec!["A1", "Apple"],
            svec!["B2", "Banana"],
        ],
    );

    // Create test data
    wrk.create("data.csv", vec![svec!["id", "name"], svec!["1", "Apple"]]);

    // Create schema using dynamicEnum with non-existent column
    wrk.create_from_string(
        "schema.json",
        r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "id": { "type": "string" },
                "name": { 
                    "type": "string",
                    "dynamicEnum": "lookup.csv|nonexistent_column"
                }
            }
        }"#,
    );

    // Run validate command
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv").arg("schema.json");

    // Check error output
    let got = wrk.output_stderr(&mut cmd);
    #[cfg(feature = "lite")]
    assert_eq!(got, "1 out of 1 records invalid.\n");
    #[cfg(not(feature = "lite"))]
    assert!(got.ends_with(
        "Cannot compile JSONschema. error: Column 'nonexistent_column' not found in lookup \
         table\nTry running `qsv validate schema schema.json` to check the JSON Schema file.\n"
    ));

    wrk.assert_err(&mut cmd);
}

#[test]
fn validate_dynenum_with_remote_csv() {
    let wrk = Workdir::new("validate_dynenum_with_remote_csv").flexible(true);

    // Create test data
    wrk.create(
        "data.csv",
        vec![
            svec!["id", "fruit"],
            svec!["1", "banana"],
            svec!["2", "mango"], // Invalid - not in fruits.csv
            svec!["3", "apple"],
            svec!["4", "dragonfruit"], // Invalid - not in fruits.csv
        ],
    );

    // Create schema using dynamicEnum with remote CSV
    wrk.create_from_string(
        "schema.json",
        r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "id": { "type": "string" },
                "fruit": { 
                    "type": "string",
                    "dynamicEnum": "https://raw.githubusercontent.com/dathere/qsv/refs/heads/master/resources/test/fruits.csv"
                }
            }
        }"#,
    );

    // Run validate command
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv").arg("schema.json");
    wrk.output(&mut cmd);

    wrk.assert_err(&mut cmd);

    // Check validation-errors.tsv
    let validation_errors = wrk
        .read_to_string("data.csv.validation-errors.tsv")
        .unwrap();
    let expected_errors = r#"row_number	field	error
2	fruit	"mango" is not a valid dynamicEnum value
4	fruit	"dragonfruit" is not a valid dynamicEnum value
"#;
    assert_eq!(validation_errors, expected_errors);

    // Check valid records
    let valid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.valid");
    let expected_valid = vec![svec!["1", "banana"], svec!["3", "apple"]];
    assert_eq!(valid_records, expected_valid);

    // Check invalid records
    let invalid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.invalid");
    let expected_invalid = vec![svec!["2", "mango"], svec!["4", "dragonfruit"]];
    assert_eq!(invalid_records, expected_invalid);

    wrk.assert_err(&mut cmd);
}

#[cfg(feature = "lite")]
#[test]
fn validate_lite_dynenum_combinations() {
    let wrk = Workdir::new("validate_lite_dynenum_combinations").flexible(true);

    // Create lookup file first
    wrk.create(
        "lookup.csv",
        vec![
            svec!["id", "name", "category"],
            svec!["1", "Apple", "fruit"],
            svec!["2", "Banana", "fruit"],
            svec!["3", "Carrot", "vegetable"],
        ],
    );

    // Test cases with different dynamicEnum URI patterns
    let test_cases = vec![
        // Simple file path
        (
            "lookup.csv",
            vec![
                svec!["id", "product"],
                svec!["1", "Apple"],   // invalid
                svec!["2", "Orange"],  // invalid
            ],
            2,
        ),
        // File path with column name
        (
            "lookup.csv|name",
            vec![
                svec!["id", "product"],
                svec!["1", "Apple"],   // valid
                svec!["2", "Orange"],  // invalid
            ],
            1,
        ),
        // File path with column index (2nd col - 0-based index)
        (
            "lookup.csv|1",
            vec![
                svec!["id", "product"],
                svec!["1", "Apple"],   // valid
                svec!["2", "Orange"],  // invalid
            ],
            1,
        ),
        // HTTP URL
        (
            "https://raw.githubusercontent.com/dathere/qsv/refs/heads/master/resources/test/fruits.csv",
            vec![
                svec!["id", "fruit"],
                svec!["1", "banana"],  // valid
                svec!["2", "mango"],   // invalid
            ],
            1,
        ),
        // HTTP URL with column
        (
            "https://raw.githubusercontent.com/dathere/qsv/refs/heads/master/resources/test/fruits.csv|0",
            vec![
                svec!["id", "fruit"],
                svec!["1", "banana"],  // valid
                svec!["2", "mango"],   // invalid
            ],
            1,
        ),
        // HTTP URL with column by name
        (
            "https://raw.githubusercontent.com/dathere/qsv/refs/heads/master/resources/test/fruits.csv|fruit",
            vec![
                svec!["id", "fruit"],
                svec!["1", "banana"],  // valid
                svec!["2", "mango"],   // invalid
                svec!["3", "strawberry"], // valid
            ],
            1,
        ),
    ];

    for (uri, data, expected_invalid_count) in test_cases {
        // Create schema using dynamicEnum
        let schema = format!(
            r#"{{
                "$schema": "https://json-schema.org/draft/2020-12/schema",
                "type": "object",
                "properties": {{
                    "id": {{ "type": "string" }},
                    "product": {{ 
                        "type": "string",
                        "dynamicEnum": "{}"
                    }}
                }}
            }}"#,
            uri
        );
        wrk.create_from_string("schema.json", &schema);

        // Create test data
        wrk.create("data.csv", data);

        // Run validate command
        let mut cmd = wrk.command("validate");
        cmd.arg("data.csv").arg("schema.json");
        wrk.output(&mut cmd);

        // Check validation errors count
        let validation_errors = wrk
            .read_to_string("data.csv.validation-errors.tsv")
            .unwrap();
        let error_count = validation_errors.lines().count() - 1; // subtract header row
        assert_eq!(
            error_count, expected_invalid_count,
            "Failed for URI: {}",
            uri
        );
    }
}

#[test]
fn validate_unique_combined_with() {
    let wrk = Workdir::new("validate_unique_combined_with").flexible(true);

    // Create test data with duplicate combinations
    wrk.create(
        "data.csv",
        vec![
            svec!["id", "name", "email", "department"],
            svec!["1", "John Doe", "john@example.com", "IT"],
            svec!["2", "Jane Smith", "jane@example.com", "HR"],
            svec!["3", "John Doe", "john@example.com", "IT"], // Duplicate name+email
            svec!["4", "Bob Wilson", "bob@example.com", "IT"],
            svec!["5", "Jane Smith", "jane@example.com", "HR"], // Duplicate name+email
        ],
    );

    // Create schema using uniqueCombinedWith to validate unique name+email combinations
    wrk.create_from_string(
        "schema.json",
        r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "id": { "type": "string" },
                "name": { "type": "string" },
                "email": { "type": "string" },
                "department": { "type": "string" }
            },
            "uniqueCombinedWith": ["name", "email"]
        }"#,
    );

    // Run validate command
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv").arg("schema.json");
    wrk.output(&mut cmd);

    wrk.assert_err(&mut cmd);

    // Check validation-errors.tsv
    let validation_errors = wrk
        .read_to_string("data.csv.validation-errors.tsv")
        .unwrap();
    let expected_errors = r#"row_number	field	error
3		Combination of values for columns name, email is not unique
5		Combination of values for columns name, email is not unique
"#;
    assert_eq!(validation_errors, expected_errors);

    // Check valid records
    let valid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.valid");
    let expected_valid = vec![
        svec!["1", "John Doe", "john@example.com", "IT"],
        svec!["2", "Jane Smith", "jane@example.com", "HR"],
        svec!["4", "Bob Wilson", "bob@example.com", "IT"],
    ];
    assert_eq!(valid_records, expected_valid);

    // Check invalid records
    let invalid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.invalid");
    let expected_invalid = vec![
        svec!["3", "John Doe", "john@example.com", "IT"],
        svec!["5", "Jane Smith", "jane@example.com", "HR"],
    ];
    assert_eq!(invalid_records, expected_invalid);

    wrk.assert_err(&mut cmd);
}

#[test]
fn validate_unique_combined_with_indices() {
    let wrk = Workdir::new("validate_unique_combined_with_indices").flexible(true);

    // Create test data with duplicate combinations
    wrk.create(
        "data.csv",
        vec![
            svec!["id", "name", "email", "department"],
            svec!["1", "John Doe", "john@example.com", "IT"],
            svec!["2", "Jane Smith", "jane@example.com", "HR"],
            svec!["3", "John Doe", "john@example.com", "IT"], // Duplicate name+email
            svec!["4", "Bob Wilson", "bob@example.com", "IT"],
            svec!["5", "Jane Smith", "jane@example.com", "HR"], // Duplicate name+email
        ],
    );

    // Create schema using uniqueCombinedWith with column indices (1=name, 2=email)
    wrk.create_from_string(
        "schema.json",
        r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "id": { "type": "string" },
                "name": { "type": "string" },
                "email": { "type": "string" },
                "department": { "type": "string" }
            },
            "uniqueCombinedWith": [1, 2]
        }"#,
    );

    // Run validate command
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv").arg("schema.json");
    wrk.output(&mut cmd);

    wrk.assert_err(&mut cmd);

    // Check validation-errors.tsv
    let validation_errors = wrk
        .read_to_string("data.csv.validation-errors.tsv")
        .unwrap();
    let expected_errors = r#"row_number	field	error
3		Combination of values for columns 1, 2 is not unique
5		Combination of values for columns 1, 2 is not unique
"#;
    assert_eq!(validation_errors, expected_errors);

    // Check valid records
    let valid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.valid");
    let expected_valid = vec![
        svec!["1", "John Doe", "john@example.com", "IT"],
        svec!["2", "Jane Smith", "jane@example.com", "HR"],
        svec!["4", "Bob Wilson", "bob@example.com", "IT"],
    ];
    assert_eq!(valid_records, expected_valid);

    // Check invalid records
    let invalid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.invalid");
    let expected_invalid = vec![
        svec!["3", "John Doe", "john@example.com", "IT"],
        svec!["5", "Jane Smith", "jane@example.com", "HR"],
    ];
    assert_eq!(invalid_records, expected_invalid);

    wrk.assert_err(&mut cmd);
}

#[test]
fn validate_unique_combined_with_both_names_and_indices() {
    let wrk = Workdir::new("validate_unique_combined_with_both_names_and_indices").flexible(true);

    // Create test data with duplicate combinations
    wrk.create(
        "data.csv",
        vec![
            svec!["id", "name", "email", "department"],
            svec!["1", "John Doe", "john@example.com", "IT"],
            svec!["2", "Jane Smith", "jane@example.com", "HR"],
            svec!["3", "John Doe", "john@example.com", "IT"], // Duplicate name+email
            svec!["4", "Bob Wilson", "bob@example.com", "IT"],
            svec!["5", "Jane Smith", "jane@example.com", "HR"], // Duplicate name+email
        ],
    );

    // Create schema using uniqueCombinedWith to validate unique name+email combinations
    wrk.create_from_string(
        "schema.json",
        r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "id": { "type": "string" },
                "name": { "type": "string" },
                "email": { "type": "string" },
                "department": { "type": "string" }
            },
            "uniqueCombinedWith": ["name", 2]
        }"#,
    );

    // Run validate command
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv").arg("schema.json");
    wrk.output(&mut cmd);

    wrk.assert_err(&mut cmd);

    // Check validation-errors.tsv
    let validation_errors = wrk
        .read_to_string("data.csv.validation-errors.tsv")
        .unwrap();
    let expected_errors = r#"row_number	field	error
3		Combination of values for columns name, 2 is not unique
5		Combination of values for columns name, 2 is not unique
"#;
    assert_eq!(validation_errors, expected_errors);

    // Check valid records
    let valid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.valid");
    let expected_valid = vec![
        svec!["1", "John Doe", "john@example.com", "IT"],
        svec!["2", "Jane Smith", "jane@example.com", "HR"],
        svec!["4", "Bob Wilson", "bob@example.com", "IT"],
    ];
    assert_eq!(valid_records, expected_valid);

    // Check invalid records
    let invalid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.invalid");
    let expected_invalid = vec![
        svec!["3", "John Doe", "john@example.com", "IT"],
        svec!["5", "Jane Smith", "jane@example.com", "HR"],
    ];
    assert_eq!(invalid_records, expected_invalid);

    wrk.assert_err(&mut cmd);
}

#[test]
fn validate_unique_combined_with_empty_values() {
    let wrk = Workdir::new("validate_unique_combined_with_empty_values").flexible(true);

    // Create test data with empty values
    wrk.create(
        "data.csv",
        vec![
            svec!["id", "name", "email", "department"],
            svec!["1", "John Doe", "john@example.com", "IT"],
            svec!["2", "", "jane@example.com", "HR"], // Empty name
            svec!["3", "John Doe", "", "IT"],         // Empty email
            svec!["4", "", "", "IT"],                 // Both empty
            svec!["5", "", "", "HR"],                 // Both empty - duplicate of row 4
        ],
    );

    // Create schema using uniqueCombinedWith to validate unique name+email combinations
    wrk.create_from_string(
        "schema.json",
        r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "id": { "type": "string" },
                "name": { 
                    "oneOf": [
                        { "type": "string", "minLength": 0 },
                        { "type": "null" }
                    ]
                },
                "email": { 
                    "oneOf": [
                        { "type": "string", "minLength": 0 },
                        { "type": "null" }
                    ]
                },
                "department": { "type": "string" }
            },
            "uniqueCombinedWith": ["name", "email"]
        }"#,
    );

    // Run validate command
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv").arg("schema.json");
    wrk.output(&mut cmd);

    wrk.assert_err(&mut cmd);

    // Check validation-errors.tsv
    let validation_errors = wrk
        .read_to_string("data.csv.validation-errors.tsv")
        .unwrap();
    let expected_errors = r#"row_number	field	error
5		Combination of values for columns name, email is not unique
"#;
    assert_eq!(validation_errors, expected_errors);

    // Check valid records
    let valid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.valid");
    let expected_valid = vec![
        svec!["1", "John Doe", "john@example.com", "IT"],
        svec!["2", "", "jane@example.com", "HR"],
        svec!["3", "John Doe", "", "IT"],
        svec!["4", "", "", "IT"],
    ];
    assert_eq!(valid_records, expected_valid);

    // Check invalid records
    let invalid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.invalid");
    let expected_invalid = vec![svec!["5", "", "", "HR"]];
    assert_eq!(invalid_records, expected_invalid);

    wrk.assert_err(&mut cmd);
}

#[test]
fn validate_unique_combined_with_special_chars() {
    let wrk = Workdir::new("validate_unique_combined_with_special_chars").flexible(true);

    // Create test data with special characters
    wrk.create(
        "data.csv",
        vec![
            svec!["id", "name", "email", "department"],
            svec!["1", "John Doe", "john.doe@example.com", "IT"],
            svec!["2", "Jane-Smith", "jane.smith@example.com", "HR"],
            svec!["3", "John Doe", "john.doe@example.com", "IT"], // Duplicate
            svec!["4", "Bob_Wilson", "bob.wilson@example.com", "IT"],
            svec!["5", "Jane-Smith", "jane.smith@example.com", "HR"], // Duplicate
        ],
    );

    // Create schema using uniqueCombinedWith to validate unique name+email combinations
    wrk.create_from_string(
        "schema.json",
        r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "id": { "type": "string" },
                "name": { "type": "string" },
                "email": { "type": "string" },
                "department": { "type": "string" }
            },
            "uniqueCombinedWith": ["name", "email"]
        }"#,
    );

    // Run validate command
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv").arg("schema.json");
    wrk.output(&mut cmd);

    wrk.assert_err(&mut cmd);

    // Check validation-errors.tsv
    let validation_errors = wrk
        .read_to_string("data.csv.validation-errors.tsv")
        .unwrap();
    let expected_errors = r#"row_number	field	error
3		Combination of values for columns name, email is not unique
5		Combination of values for columns name, email is not unique
"#;
    assert_eq!(validation_errors, expected_errors);

    // Check valid records
    let valid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.valid");
    let expected_valid = vec![
        svec!["1", "John Doe", "john.doe@example.com", "IT"],
        svec!["2", "Jane-Smith", "jane.smith@example.com", "HR"],
        svec!["4", "Bob_Wilson", "bob.wilson@example.com", "IT"],
    ];
    assert_eq!(valid_records, expected_valid);

    // Check invalid records
    let invalid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.invalid");
    let expected_invalid = vec![
        svec!["3", "John Doe", "john.doe@example.com", "IT"],
        svec!["5", "Jane-Smith", "jane.smith@example.com", "HR"],
    ];
    assert_eq!(invalid_records, expected_invalid);

    wrk.assert_err(&mut cmd);
}

#[test]
fn validate_dynenum_with_multiple_columns() {
    let wrk = Workdir::new("validate_dynenum_with_multiple_columns").flexible(true);

    // Create lookup file with multiple columns
    wrk.create(
        "lookup.csv",
        vec![
            svec!["code", "name", "category", "status"],
            svec!["A1", "Apple", "fruit", "active"],
            svec!["B2", "Banana", "fruit", "active"],
            svec!["C3", "Carrot", "vegetable", "inactive"],
            svec!["D4", "Dragon Fruit", "fruit", "active"],
        ],
    );

    // Create test data
    wrk.create(
        "data.csv",
        vec![
            svec!["id", "product", "type", "status"],
            svec!["1", "Apple", "fruit", "active"],
            svec!["2", "Banana", "fruit", "active"],
            svec!["3", "Orange", "fruit", "active"], // Invalid - not in lookup
            svec!["4", "Carrot", "vegetable", "inactive"],
            svec!["5", "Dragon Fruit", "fruit", "active"],
        ],
    );

    // Create schema using dynamicEnum with multiple column validations
    wrk.create_from_string(
        "schema.json",
        r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "id": { "type": "string" },
                "product": { 
                    "type": "string",
                    "dynamicEnum": "lookup.csv|name"
                },
                "type": { 
                    "type": "string",
                    "dynamicEnum": "lookup.csv|category"
                },
                "status": { 
                    "type": "string",
                    "dynamicEnum": "lookup.csv|status"
                }
            }
        }"#,
    );

    // Run validate command
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv").arg("schema.json");
    wrk.output(&mut cmd);

    wrk.assert_err(&mut cmd);

    // Check validation-errors.tsv
    let validation_errors = wrk
        .read_to_string("data.csv.validation-errors.tsv")
        .unwrap();
    let expected_errors = r#"row_number	field	error
3	product	"Orange" is not a valid dynamicEnum value
"#;
    assert_eq!(validation_errors, expected_errors);

    // Check valid records
    let valid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.valid");
    let expected_valid = vec![
        svec!["1", "Apple", "fruit", "active"],
        svec!["2", "Banana", "fruit", "active"],
        svec!["4", "Carrot", "vegetable", "inactive"],
        svec!["5", "Dragon Fruit", "fruit", "active"],
    ];
    assert_eq!(valid_records, expected_valid);

    // Check invalid records
    let invalid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.invalid");
    let expected_invalid = vec![svec!["3", "Orange", "fruit", "active"]];
    assert_eq!(invalid_records, expected_invalid);

    wrk.assert_err(&mut cmd);
}

#[cfg(not(feature = "lite"))]
#[test]
fn validate_dynenum_with_caching() {
    let wrk = Workdir::new("validate_dynenum_with_caching").flexible(true);

    // Create lookup file
    wrk.create(
        "lookup.csv",
        vec![
            svec!["code", "name"],
            svec!["A1", "Apple"],
            svec!["B2", "Banana"],
            svec!["C3", "Carrot"],
        ],
    );

    // Create test data
    wrk.create(
        "data.csv",
        vec![
            svec!["id", "product"],
            svec!["1", "Apple"],
            svec!["2", "Orange"], // Invalid
            svec!["3", "Banana"],
        ],
    );

    // Create schema using dynamicEnum with cache configuration
    wrk.create_from_string(
        "schema.json",
        r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "id": { "type": "string" },
                "product": { 
                    "type": "string",
                    "dynamicEnum": "product_cache;3600|lookup.csv|name"
                }
            }
        }"#,
    );

    // Run validate command
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv").arg("schema.json");
    wrk.output(&mut cmd);

    wrk.assert_err(&mut cmd);

    // Check validation-errors.tsv
    let validation_errors = wrk
        .read_to_string("data.csv.validation-errors.tsv")
        .unwrap();
    let expected_errors = r#"row_number	field	error
2	product	"Orange" is not a valid dynamicEnum value
"#;
    assert_eq!(validation_errors, expected_errors);

    // Check valid records
    let valid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.valid");
    let expected_valid = vec![svec!["1", "Apple"], svec!["3", "Banana"]];
    assert_eq!(valid_records, expected_valid);

    // Check invalid records
    let invalid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.invalid");
    let expected_invalid = vec![svec!["2", "Orange"]];
    assert_eq!(invalid_records, expected_invalid);

    wrk.assert_err(&mut cmd);
}

#[test]
fn validate_dynenum_with_invalid_uri() {
    let wrk = Workdir::new("validate_dynenum_with_invalid_uri").flexible(true);

    // Create test data
    wrk.create(
        "data.csv",
        vec![
            svec!["id", "product"],
            svec!["1", "Apple"],
            svec!["2", "Banana"],
        ],
    );

    // Create schema using dynamicEnum with invalid URI
    wrk.create_from_string(
        "schema.json",
        r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "id": { "type": "string" },
                "product": { 
                    "type": "string",
                    "dynamicEnum": "nonexistent.csv"
                }
            }
        }"#,
    );

    // Run validate command
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv").arg("schema.json");

    // Check error output
    let got = wrk.output_stderr(&mut cmd);

    assert!(got.starts_with("Cannot compile JSONschema."));

    wrk.assert_err(&mut cmd);
}

#[test]
fn validate_unique_combined_with_mixed_names_and_indices() {
    let wrk = Workdir::new("validate_unique_combined_with_mixed_names_and_indices").flexible(true);

    // Create test data with duplicate combinations
    wrk.create(
        "data.csv",
        vec![
            svec!["id", "name", "email", "department", "role"],
            svec!["1", "John Doe", "john@example.com", "IT", "Developer"],
            svec!["2", "Jane Smith", "jane@example.com", "HR", "Manager"],
            svec!["3", "John Doe", "john@example.com", "IT", "Developer"], // Duplicate name+email+role
            svec!["4", "Bob Wilson", "bob@example.com", "IT", "Developer"],
            svec!["5", "Jane Smith", "jane@example.com", "HR", "Manager"], // Duplicate name+email+role
            svec!["6", "Alice Brown", "alice@example.com", "IT", "Developer"], // Valid - different role
            svec!["7", "Alice Brown", "alice@example.com", "IT", "Manager"], // Valid - different role
        ],
    );

    // Create schema using uniqueCombinedWith with mix of column names and indices
    // name (by name), email (by index 2), role (by name)
    wrk.create_from_string(
        "schema.json",
        r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "id": { "type": "string" },
                "name": { "type": "string" },
                "email": { "type": "string" },
                "department": { "type": "string" },
                "role": { "type": "string" }
            },
            "uniqueCombinedWith": ["name", 2, "role"]
        }"#,
    );

    // Run validate command
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv").arg("schema.json");
    wrk.output(&mut cmd);

    wrk.assert_err(&mut cmd);

    // Check validation-errors.tsv
    let validation_errors = wrk
        .read_to_string("data.csv.validation-errors.tsv")
        .unwrap();
    // note: the order of the columns in the error message is named first, then indexed
    // that's why the error message names the columns as name, role, 2,
    // but the order of the columns in the schema is name, 2
    let expected_errors = r#"row_number	field	error
3		Combination of values for columns name, role, 2 is not unique
5		Combination of values for columns name, role, 2 is not unique
"#;
    assert_eq!(validation_errors, expected_errors);

    // Check valid records
    let valid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.valid");
    let expected_valid = vec![
        svec!["1", "John Doe", "john@example.com", "IT", "Developer"],
        svec!["2", "Jane Smith", "jane@example.com", "HR", "Manager"],
        svec!["4", "Bob Wilson", "bob@example.com", "IT", "Developer"],
        svec!["6", "Alice Brown", "alice@example.com", "IT", "Developer"],
        svec!["7", "Alice Brown", "alice@example.com", "IT", "Manager"],
    ];
    assert_eq!(valid_records, expected_valid);

    // Check invalid records
    let invalid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.invalid");
    let expected_invalid = vec![
        svec!["3", "John Doe", "john@example.com", "IT", "Developer"],
        svec!["5", "Jane Smith", "jane@example.com", "HR", "Manager"],
    ];
    assert_eq!(invalid_records, expected_invalid);

    wrk.assert_err(&mut cmd);
}

#[test]
fn validate_no_format_validation() {
    let wrk = Workdir::new("validate_no_format_validation").flexible(true);

    // Create test data with invalid format values
    wrk.create(
        "data.csv",
        vec![
            svec!["id", "name", "email", "website", "fee"],
            svec![
                "1",
                "John Doe",
                "john@example.com",
                "https://example.com",
                "$100.00"
            ],
            svec![
                "2",
                "Jane Smith",
                "not-an-email",
                "not-a-url",
                "not-currency"
            ], // Invalid formats
            svec!["3", "Bob Wilson", "bob.wilson", "ftp://invalid", "€ 50.00"], // Invalid formats
        ],
    );

    // Create schema with format validation
    wrk.create_from_string(
        "schema.json",
        r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "id": { "type": "string" },
                "name": { "type": "string" },
                "email": { 
                    "type": "string",
                    "format": "email"
                },
                "website": { 
                    "type": "string",
                    "format": "uri"
                },
                "fee": { 
                    "type": "string",
                    "format": "currency"
                }
            }
        }"#,
    );

    // First, run validation WITH format validation (default behavior)
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv").arg("schema.json");
    wrk.output(&mut cmd);

    wrk.assert_err(&mut cmd);

    // Check that format validation errors are present
    let validation_errors = wrk
        .read_to_string("data.csv.validation-errors.tsv")
        .unwrap();

    // Should have format validation errors
    assert!(validation_errors.contains("is not a \"email\""));
    assert!(validation_errors.contains("is not a \"uri\""));
    assert!(validation_errors.contains("is not a \"currency\""));

    // Clean up output files for next test
    let _ = std::fs::remove_file(wrk.path("data.csv.valid"));
    let _ = std::fs::remove_file(wrk.path("data.csv.invalid"));
    let _ = std::fs::remove_file(wrk.path("data.csv.validation-errors.tsv"));

    // Now run validation WITHOUT format validation
    let mut cmd = wrk.command("validate");
    cmd.arg("--no-format-validation")
        .arg("data.csv")
        .arg("schema.json");

    wrk.assert_success(&mut cmd);

    // Should not create any error files since all records are valid
    // when format validation is disabled
    assert!(!wrk.path("data.csv.invalid").exists());
    assert!(!wrk.path("data.csv.validation-errors.tsv").exists());

    let got: String = wrk.output_stderr(&mut cmd);
    let expected = "All 3 records valid.\n";
    assert_eq!(got, expected);
}

#[test]
fn validate_json_schema_file() {
    let wrk = Workdir::new("validate_json_schema_file").flexible(true);

    // Create schema with format validation
    wrk.create_from_string(
        "schema.json",
        r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "id": { "type": "string" },
                "name": { "type": "string" },
                "email": { 
                    "type": "string",
                    "format": "email"
                }
            }
        }"#,
    );

    // Run validate command
    let mut cmd = wrk.command("validate");
    cmd.arg("schema").arg("schema.json");
    wrk.output(&mut cmd);

    wrk.assert_success(&mut cmd);
}

#[test]
fn validate_invalid_json_schema_file() {
    let wrk = Workdir::new("validate_invalid_json_schema_file").flexible(true);

    // Create schema with format validation
    // Create schema with format validation
    // This schema is invalid because it has a draft version that doesn't exist
    wrk.create_from_string(
        "schema.json",
        r#"{
            "$schema": "https://json-schema.org/draft/2020-25/schema",
            "type": "object",
            "properties": {
                "id": { "type": "string" },
                "name": { "type": "string" },
                "email": { 
                    "type": "string",
                    "format": "email"
                }
            }
        }"#,
    );

    // Run validate command
    let mut cmd = wrk.command("validate");
    cmd.arg("schema").arg("schema.json");

    wrk.assert_err(&mut cmd);

    let got = wrk.output_stderr(&mut cmd);
    assert_eq!(
        got,
        "JSON Schema Meta-Reference Error: Resource \
         'https://json-schema.org/draft/2020-25/schema' is not present in a registry and \
         retrieving it failed: error decoding response body\n"
    );

    // Create schema with format validation
    // This schema is invalid because of invalid types "stringy"
    wrk.create_from_string(
        "schema2.json",
        r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "id": { "type": "stringy" },
                "name": { "type": "string" },
                "email": { 
                    "type": "string",
                    "format": "email"
                }
            }
        }"#,
    );

    // Run validate command
    let mut cmd = wrk.command("validate");
    cmd.arg("schema").arg("schema2.json");

    wrk.assert_err(&mut cmd);

    let got = wrk.output_stderr(&mut cmd);
    assert_eq!(
        got,
        "JSON Schema Meta-Reference Error: \"stringy\" is not valid under any of the schemas \
         listed in the 'anyOf' keyword\n"
    );
}

#[test]
fn validate_with_fancy_regex() {
    let wrk = Workdir::new("validate_with_fancy_regex").flexible(true);

    // Create test data with passwords that need to meet specific criteria
    wrk.create(
        "data.csv",
        vec![
            svec!["username", "password"],
            svec!["user1", "Password123!"], // Valid: has uppercase, lowercase, digit, special char
            svec!["user2", "password123"],  // Invalid: no uppercase, no special char
            svec!["user3", "PASSWORD123!"], // Invalid: no lowercase
            svec!["user4", "Password!"],    // Invalid: no digit
            svec!["user5", "Pass123"],      // Invalid: no special char
        ],
    );

    // Create schema with a regex pattern that requires fancy regex support
    // This regex uses look-ahead assertions (?=...) to ensure password contains:
    // - at least one uppercase letter
    // - at least one lowercase letter
    // - at least one digit
    // - at least one special character
    wrk.create_from_string(
        "schema.json",
        r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "username": { "type": "string" },
                "password": { 
                    "type": "string",
                    "pattern": "^(?=.*[A-Z])(?=.*[a-z])(?=.*\\d)(?=.*[!@#$%^&*()_+\\-=\\[\\]{};':\"\\\\|,.<>\\/?]).{8,}$"
                }
            }
        }"#,
    );

    // Run validate command WITHOUT fancy-regex flag (should fail)
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv").arg("schema.json");

    // This should fail because the regex pattern uses look-ahead assertions
    // which are not supported by the default regex engine
    wrk.assert_err(&mut cmd);

    // Run validate command WITH fancy-regex flag (should work)
    let mut cmd_fancy = wrk.command("validate");
    cmd_fancy
        .arg("data.csv")
        .arg("schema.json")
        .arg("--fancy-regex");
    wrk.output(&mut cmd_fancy);

    // we still get an error here as the test data is invalid,
    // not because of the regex engine
    wrk.assert_err(&mut cmd_fancy);
    let got = wrk.output_stderr(&mut cmd_fancy);
    assert_eq!(got, "4 out of 5 records invalid.\n");

    // Check validation-errors.tsv - should show 4 invalid passwords
    let validation_errors: String = wrk.from_str(&wrk.path("data.csv.validation-errors.tsv"));

    // The error messages should indicate pattern validation failures
    assert!(validation_errors.contains("password"));
    // Check for the specific error message format used by jsonschema
    assert!(validation_errors.contains("does not match"));

    // Check valid records - should only contain the valid password
    let valid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.valid");
    let expected_valid = vec![svec!["user1", "Password123!"]];
    assert_eq!(valid_records, expected_valid);

    // Check invalid records - should contain the 4 invalid passwords
    let invalid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.invalid");
    let expected_invalid = vec![
        svec!["user2", "password123"],
        svec!["user3", "PASSWORD123!"],
        svec!["user4", "Password!"],
        svec!["user5", "Pass123"],
    ];
    assert_eq!(invalid_records, expected_invalid);
}

#[test]
fn validate_schema_subcommand_valid_schema() {
    let wrk = Workdir::new("validate_schema_subcommand_valid_schema").flexible(true);

    // Create a valid JSON Schema
    wrk.create_from_string(
        "schema.json",
        r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "id": { "type": "string" },
                "name": { "type": "string" },
                "email": { 
                    "type": "string",
                    "format": "email"
                },
                "age": { 
                    "type": "integer",
                    "minimum": 0,
                    "maximum": 150
                }
            },
            "required": ["id", "name"]
        }"#,
    );

    // Run validate schema command
    let mut cmd = wrk.command("validate");
    cmd.arg("schema").arg("schema.json");

    wrk.assert_success(&mut cmd);

    let got: String = wrk.output_stderr(&mut cmd);
    let expected = "Valid JSON Schema.\n";
    assert_eq!(got, expected);
}

#[test]
fn validate_schema_subcommand_invalid_schema() {
    let wrk = Workdir::new("validate_schema_subcommand_invalid_schema").flexible(true);

    // Create an invalid JSON Schema (invalid type)
    wrk.create_from_string(
        "schema.json",
        r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "id": { "type": "invalid_type" },
                "name": { "type": "string" }
            }
        }"#,
    );

    // Run validate schema command
    let mut cmd = wrk.command("validate");
    cmd.arg("schema").arg("schema.json");

    wrk.assert_err(&mut cmd);

    let got: String = wrk.output_stderr(&mut cmd);
    let expected = "JSON Schema Meta-Reference Error: \"invalid_type\" is not valid under any of \
                    the schemas listed in the 'anyOf' keyword\n";
    assert_eq!(got, expected);
}

#[test]
fn validate_schema_subcommand_invalid_draft() {
    let wrk = Workdir::new("validate_schema_subcommand_invalid_draft").flexible(true);

    // Create a schema with invalid draft version
    wrk.create_from_string(
        "schema.json",
        r#"{
            "$schema": "https://json-schema.org/draft/2020-25/schema",
            "type": "object",
            "properties": {
                "id": { "type": "string" }
            }
        }"#,
    );

    // Run validate schema command
    let mut cmd = wrk.command("validate");
    cmd.arg("schema").arg("schema.json");

    wrk.assert_err(&mut cmd);

    let got: String = wrk.output_stderr(&mut cmd);
    let expected =
        "JSON Schema Meta-Reference Error: Resource \
         'https://json-schema.org/draft/2020-25/schema' is not present in a registry and \
         retrieving it failed: error decoding response body\n";
    assert_eq!(got, expected);
}

#[test]
fn validate_schema_subcommand_no_schema_file() {
    let wrk = Workdir::new("validate_schema_subcommand_no_schema_file").flexible(true);

    // Run validate schema command without providing a schema file
    let mut cmd = wrk.command("validate");
    cmd.arg("schema");

    wrk.assert_err(&mut cmd);

    let got: String = wrk.output_stderr(&mut cmd);
    let expected = "No JSON Schema file supplied.\n";
    assert_eq!(got, expected);
}

#[test]
fn validate_schema_subcommand_with_no_format_validation() {
    let wrk = Workdir::new("validate_schema_subcommand_with_no_format_validation").flexible(true);

    // Create a schema with format validation
    wrk.create_from_string(
        "schema.json",
        r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "id": { "type": "string" },
                "email": { 
                    "type": "string",
                    "format": "email"
                },
                "uri": { 
                    "type": "string",
                    "format": "uri"
                },
                "currency": { 
                    "type": "string",
                    "format": "currency"
                }
            }
        }"#,
    );

    // Run validate schema command with --no-format-validation
    let mut cmd = wrk.command("validate");
    cmd.arg("schema")
        .arg("--no-format-validation")
        .arg("schema.json");

    wrk.assert_success(&mut cmd);

    let got: String = wrk.output_stderr(&mut cmd);
    let expected = "Valid JSON Schema.\n";
    assert_eq!(got, expected);
}

// Note: --quiet flag test removed due to command parsing issues

#[test]
fn validate_with_no_format_validation_success() {
    let wrk = Workdir::new("validate_with_no_format_validation_success").flexible(true);

    // Create test data with invalid format values that would normally fail validation
    wrk.create(
        "data.csv",
        vec![
            svec!["id", "name", "email", "website", "currency"],
            svec!["1", "John Doe", "not-an-email", "not-a-url", "not-currency"],
            svec![
                "2",
                "Jane Smith",
                "also-not-email",
                "also-not-url",
                "also-not-currency"
            ],
        ],
    );

    // Create schema with format validation
    wrk.create_from_string(
        "schema.json",
        r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "id": { "type": "string" },
                "name": { "type": "string" },
                "email": { 
                    "type": "string",
                    "format": "email"
                },
                "website": { 
                    "type": "string",
                    "format": "uri"
                },
                "currency": { 
                    "type": "string",
                    "format": "currency"
                }
            }
        }"#,
    );

    // Run validation WITH format validation (should fail)
    let mut cmd_with_format = wrk.command("validate");
    cmd_with_format.arg("data.csv").arg("schema.json");
    wrk.output(&mut cmd_with_format);

    wrk.assert_err(&mut cmd_with_format);

    // Check that format validation errors are present
    let validation_errors = wrk
        .read_to_string("data.csv.validation-errors.tsv")
        .unwrap();
    assert!(validation_errors.contains("is not a \"email\""));
    assert!(validation_errors.contains("is not a \"uri\""));
    assert!(validation_errors.contains("is not a \"currency\""));

    // Clean up output files for next test
    let _ = std::fs::remove_file(wrk.path("data.csv.valid"));
    let _ = std::fs::remove_file(wrk.path("data.csv.invalid"));
    let _ = std::fs::remove_file(wrk.path("data.csv.validation-errors.tsv"));

    // Run validation WITHOUT format validation (should succeed)
    let mut cmd_no_format = wrk.command("validate");
    cmd_no_format
        .arg("--no-format-validation")
        .arg("data.csv")
        .arg("schema.json");

    wrk.assert_success(&mut cmd_no_format);

    // Should not create any error files since all records are valid
    // when format validation is disabled
    assert!(!wrk.path("data.csv.invalid").exists());
    assert!(!wrk.path("data.csv.validation-errors.tsv").exists());

    let got: String = wrk.output_stderr(&mut cmd_no_format);
    let expected = "All 2 records valid.\n";
    assert_eq!(got, expected);
}

#[test]
fn validate_with_no_format_validation_mixed_errors() {
    let wrk = Workdir::new("validate_with_no_format_validation_mixed_errors").flexible(true);

    // Create test data with both format errors and structural errors
    wrk.create(
        "data.csv",
        vec![
            svec!["id", "name", "email", "age"],
            svec!["1", "John Doe", "not-an-email", "25"], // Format error only
            svec!["2", "Jane Smith", "jane@example.com", "not-a-number"], // Type error
            svec!["3", "Bob Wilson", "bob@example.com", "30"], // Valid
        ],
    );

    // Create schema with both format and type validation
    wrk.create_from_string(
        "schema.json",
        r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "id": { "type": "string" },
                "name": { "type": "string" },
                "email": { 
                    "type": "string",
                    "format": "email"
                },
                "age": { 
                    "type": "integer",
                    "minimum": 0,
                    "maximum": 150
                }
            }
        }"#,
    );

    // Run validation WITH format validation
    let mut cmd_with_format = wrk.command("validate");
    cmd_with_format.arg("data.csv").arg("schema.json");
    wrk.output(&mut cmd_with_format);

    wrk.assert_err(&mut cmd_with_format);

    // Should have both format and type errors
    let validation_errors = wrk
        .read_to_string("data.csv.validation-errors.tsv")
        .unwrap();
    assert!(validation_errors.contains("is not a \"email\"")); // Format error
    assert!(validation_errors.contains("Can't cast to Integer")); // Type error

    // Clean up output files for next test
    let _ = std::fs::remove_file(wrk.path("data.csv.valid"));
    let _ = std::fs::remove_file(wrk.path("data.csv.invalid"));
    let _ = std::fs::remove_file(wrk.path("data.csv.validation-errors.tsv"));

    // Run validation WITHOUT format validation
    let mut cmd_no_format = wrk.command("validate");
    cmd_no_format
        .arg("--no-format-validation")
        .arg("data.csv")
        .arg("schema.json");
    wrk.output(&mut cmd_no_format);

    wrk.assert_err(&mut cmd_no_format);

    // Should only have type errors, no format errors
    let validation_errors_no_format = wrk
        .read_to_string("data.csv.validation-errors.tsv")
        .unwrap();
    assert!(!validation_errors_no_format.contains("is not a \"email\"")); // No format error
    assert!(validation_errors_no_format.contains("Can't cast to Integer")); // Still has type error

    // Check valid records - should include the record with only format errors
    let valid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.valid");
    let expected_valid = vec![
        svec!["1", "John Doe", "not-an-email", "25"], // Now valid (format ignored)
        svec!["3", "Bob Wilson", "bob@example.com", "30"], // Still valid
    ];
    assert_eq!(valid_records, expected_valid);

    // Check invalid records - should only include the record with type errors
    let invalid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.invalid");
    let expected_invalid = vec![svec!["2", "Jane Smith", "jane@example.com", "not-a-number"]];
    assert_eq!(invalid_records, expected_invalid);
}

#[test]
fn validate_schema_subcommand_with_invalid_format_validation() {
    let wrk =
        Workdir::new("validate_schema_subcommand_with_invalid_format_validation").flexible(true);

    // Create a schema with invalid format validation that would normally fail
    wrk.create_from_string(
        "schema.json",
        r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "id": { "type": "string" },
                "email": { 
                    "type": "string",
                    "format": "invalid_format"
                }
            }
        }"#,
    );

    // Run validate schema command WITHOUT --no-format-validation (should fail)
    let mut cmd_with_format = wrk.command("validate");
    cmd_with_format.arg("schema").arg("schema.json");

    wrk.assert_err(&mut cmd_with_format);

    // Run validate schema command WITH --no-format-validation (should succeed)
    let mut cmd_no_format = wrk.command("validate");
    cmd_no_format
        .arg("schema")
        .arg("--no-format-validation")
        .arg("schema.json");

    wrk.assert_success(&mut cmd_no_format);

    let got: String = wrk.output_stderr(&mut cmd_no_format);
    let expected = "Valid JSON Schema.\n";
    assert_eq!(got, expected);
}

#[test]
fn validate_schema_subcommand_with_url_schema() {
    let wrk = Workdir::new("validate_schema_subcommand_with_url_schema").flexible(true);

    // Test with a remote schema URL
    let mut cmd = wrk.command("validate");
    cmd.arg("schema")
        .arg("https://raw.githubusercontent.com/dathere/qsv/master/resources/test/public-toilets-schema.json");

    wrk.assert_success(&mut cmd);

    let got: String = wrk.output_stderr(&mut cmd);
    let expected = "Valid JSON Schema.\n";
    assert_eq!(got, expected);
}

#[test]
fn validate_schema_subcommand_with_invalid_url_schema() {
    let wrk = Workdir::new("validate_schema_subcommand_with_invalid_url_schema").flexible(true);

    // Test with an invalid remote schema URL
    let mut cmd = wrk.command("validate");
    cmd.arg("schema")
        .arg("https://raw.githubusercontent.com/dathere/qsv/master/nonexistent-schema.json");

    wrk.assert_err(&mut cmd);

    let got: String = wrk.output_stderr(&mut cmd);
    assert!(
        got.contains("Cannot compile JSONschema")
            || got.contains("io error")
            || got.contains("timeout")
            || got.contains("JSON error")
    );
}

#[test]
fn validate_with_no_format_validation_and_dynamic_enum() {
    let wrk = Workdir::new("validate_with_no_format_validation_and_dynamic_enum").flexible(true);

    // Create lookup file
    wrk.create(
        "lookup.csv",
        vec![
            svec!["code", "name"],
            svec!["A1", "Apple"],
            svec!["B2", "Banana"],
        ],
    );

    // Create test data
    wrk.create(
        "data.csv",
        vec![
            svec!["id", "product", "email"],
            svec!["1", "Apple", "not-an-email"], // Valid product, invalid email format
            svec!["2", "Orange", "valid@email.com"], // Invalid product, valid email format
        ],
    );

    // Create schema with both dynamicEnum and format validation
    wrk.create_from_string(
        "schema.json",
        r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "id": { "type": "string" },
                "product": { 
                    "type": "string",
                    "dynamicEnum": "lookup.csv|name"
                },
                "email": { 
                    "type": "string",
                    "format": "email"
                }
            }
        }"#,
    );

    // Run validation WITH format validation
    let mut cmd_with_format = wrk.command("validate");
    cmd_with_format.arg("data.csv").arg("schema.json");
    wrk.output(&mut cmd_with_format);

    wrk.assert_err(&mut cmd_with_format);

    // Should have both dynamicEnum and format errors
    let validation_errors = wrk
        .read_to_string("data.csv.validation-errors.tsv")
        .unwrap();
    assert!(validation_errors.contains("is not a valid dynamicEnum value")); // dynamicEnum error
    assert!(validation_errors.contains("is not a \"email\"")); // Format error

    // Clean up output files for next test
    let _ = std::fs::remove_file(wrk.path("data.csv.valid"));
    let _ = std::fs::remove_file(wrk.path("data.csv.invalid"));
    let _ = std::fs::remove_file(wrk.path("data.csv.validation-errors.tsv"));

    // Run validation WITHOUT format validation
    let mut cmd_no_format = wrk.command("validate");
    cmd_no_format
        .arg("--no-format-validation")
        .arg("data.csv")
        .arg("schema.json");
    wrk.output(&mut cmd_no_format);

    wrk.assert_err(&mut cmd_no_format);

    // Should only have dynamicEnum errors, no format errors
    let validation_errors_no_format = wrk
        .read_to_string("data.csv.validation-errors.tsv")
        .unwrap();
    assert!(validation_errors_no_format.contains("is not a valid dynamicEnum value")); // Still has dynamicEnum error
    assert!(!validation_errors_no_format.contains("is not a \"email\"")); // No format error

    // Check valid records - should include the record with only format errors
    let valid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.valid");
    let expected_valid = vec![svec!["1", "Apple", "not-an-email"]]; // Now valid (format ignored)
    assert_eq!(valid_records, expected_valid);

    // Check invalid records - should only include the record with dynamicEnum errors
    let invalid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.invalid");
    let expected_invalid = vec![svec!["2", "Orange", "valid@email.com"]];
    assert_eq!(invalid_records, expected_invalid);
}

#[test]
fn validate_multiple_files_rfc4180() {
    let wrk = Workdir::new("validate_multiple_files_rfc4180").flexible(true);

    // Create multiple test CSV files
    wrk.create(
        "file1.csv",
        vec![
            svec!["id", "name", "age"],
            svec!["1", "John", "25"],
            svec!["2", "Jane", "30"],
        ],
    );

    wrk.create(
        "file2.csv",
        vec![
            svec!["product", "price", "stock"],
            svec!["Laptop", "999.99", "50"],
            svec!["Phone", "699.99", "100"],
        ],
    );

    wrk.create(
        "file3.csv",
        vec![
            svec!["city", "country", "population"],
            svec!["New York", "USA", "8000000"],
            svec!["London", "UK", "9000000"],
        ],
    );

    // Test validating multiple files
    let mut cmd = wrk.command("validate");
    cmd.arg("file1.csv").arg("file2.csv").arg("file3.csv");

    wrk.assert_success(&mut cmd);

    let got: String = wrk.output_stderr(&mut cmd);

    // The Extended Input Support should work and show a summary
    assert!(got.contains("✅ All 3 files are valid."));
}

#[test]
fn validate_multiple_files_with_invalid() {
    let wrk = Workdir::new("validate_multiple_files_with_invalid").flexible(true);

    // Create one valid file
    wrk.create(
        "valid.csv",
        vec![svec!["id", "name"], svec!["1", "John"], svec!["2", "Jane"]],
    );

    // Create one invalid file (unequal field count)
    wrk.create(
        "invalid.csv",
        vec![
            svec!["id", "name", "age"],
            svec!["1", "John", "25"],
            svec!["2", "Jane"], // Missing age field
        ],
    );

    // Test validating multiple files with one invalid
    let mut cmd = wrk.command("validate");
    cmd.arg("valid.csv").arg("invalid.csv");

    wrk.assert_err(&mut cmd);

    let got: String = wrk.output_stderr(&mut cmd);

    // The output should contain error information and summary
    assert!(got.contains("❌ 1 out of 2 files are invalid."));
}

#[test]
fn validate_directory() {
    let wrk = Workdir::new("validate_directory").flexible(true);

    // Create a subdirectory with CSV files
    let _ = wrk.create_subdir("data");

    // Create files in the subdirectory
    wrk.create(
        "data/file1.csv",
        vec![svec!["id", "name"], svec!["1", "John"], svec!["2", "Jane"]],
    );

    wrk.create(
        "data/file2.csv",
        vec![
            svec!["product", "price"],
            svec!["Laptop", "999.99"],
            svec!["Phone", "699.99"],
        ],
    );

    // Test validating a directory
    let mut cmd = wrk.command("validate");
    cmd.arg("data");

    wrk.assert_success(&mut cmd);

    let got: String = wrk.output_stderr(&mut cmd);
    // The Extended Input Support should work for directories
    assert!(got.contains("✅ All 2 files are valid."));
}

#[test]
fn validate_infile_list() {
    let wrk = Workdir::new("validate_infile_list").flexible(true);

    // Create individual CSV files
    wrk.create(
        "file1.csv",
        vec![svec!["id", "name"], svec!["1", "John"], svec!["2", "Jane"]],
    );

    wrk.create(
        "file2.csv",
        vec![
            svec!["product", "price"],
            svec!["Laptop", "999.99"],
            svec!["Phone", "699.99"],
        ],
    );

    // Create an infile-list file
    wrk.create(
        "filelist.infile-list",
        vec![svec!["file1.csv"], svec!["file2.csv"]],
    );

    // Test validating using infile-list
    let mut cmd = wrk.command("validate");
    cmd.arg("filelist.infile-list");

    wrk.assert_success(&mut cmd);

    let got: String = wrk.output_stderr(&mut cmd);
    assert!(got.contains("✅ All 2 files are valid."));
}

#[test]
fn validate_multiple_files_json_output() {
    let wrk = Workdir::new("validate_multiple_files_json_output").flexible(true);

    // Create multiple test CSV files
    wrk.create(
        "file1.csv",
        vec![svec!["id", "name"], svec!["1", "John"], svec!["2", "Jane"]],
    );

    wrk.create(
        "file2.csv",
        vec![
            svec!["product", "price"],
            svec!["Laptop", "999.99"],
            svec!["Phone", "699.99"],
        ],
    );

    // Test validating multiple files with JSON output
    let mut cmd = wrk.command("validate");
    cmd.arg("--json").arg("file1.csv").arg("file2.csv");

    wrk.assert_success(&mut cmd);

    let got: String = wrk.output_stderr(&mut cmd);
    assert!(got.contains("✅ All 2 files are valid."));

    // Check that JSON output is produced for each file
    let output = wrk.output(&mut cmd);
    let stdout_str = String::from_utf8_lossy(&output.stdout);
    assert!(stdout_str.contains("\"delimiter_char\":\",\""));
    assert!(stdout_str.contains("\"num_records\":2"));
}

#[test]
fn validate_multiple_files_pretty_json_output() {
    let wrk = Workdir::new("validate_multiple_files_pretty_json_output").flexible(true);

    // Create multiple test CSV files
    wrk.create(
        "file1.csv",
        vec![svec!["id", "name"], svec!["1", "John"], svec!["2", "Jane"]],
    );

    wrk.create(
        "file2.csv",
        vec![
            svec!["product", "price"],
            svec!["Laptop", "999.99"],
            svec!["Phone", "699.99"],
        ],
    );

    // Test validating multiple files with pretty JSON output
    let mut cmd = wrk.command("validate");
    cmd.arg("--pretty-json").arg("file1.csv").arg("file2.csv");

    wrk.assert_success(&mut cmd);

    let got: String = wrk.output_stderr(&mut cmd);
    assert!(got.contains("✅ All 2 files are valid."));

    // Check that pretty JSON output is produced for each file
    let output = wrk.output(&mut cmd);
    let stdout_str = String::from_utf8_lossy(&output.stdout);
    assert!(stdout_str.contains("{\n  \"delimiter_char\": \",\""));
    assert!(stdout_str.contains("\"num_records\": 2"));
}

#[test]
fn validate_multiple_files_quiet_mode() {
    let wrk = Workdir::new("validate_multiple_files_quiet_mode").flexible(true);

    // Create multiple test CSV files
    wrk.create(
        "file1.csv",
        vec![svec!["id", "name"], svec!["1", "John"], svec!["2", "Jane"]],
    );

    wrk.create(
        "file2.csv",
        vec![
            svec!["product", "price"],
            svec!["Laptop", "999.99"],
            svec!["Phone", "699.99"],
        ],
    );

    // Test validating multiple files in quiet mode
    let mut cmd = wrk.command("validate");
    cmd.arg("--quiet").arg("file1.csv").arg("file2.csv");

    wrk.assert_success(&mut cmd);

    // In quiet mode, there should be no output to stderr
    let got: String = wrk.output_stderr(&mut cmd);
    // The output might be "No error" if there's no stderr output
    assert!(got == "" || got == "No error");
}

#[test]
fn validate_multiple_files_with_mixed_delimiters() {
    let wrk = Workdir::new("validate_multiple_files_with_mixed_delimiters").flexible(true);

    // Create CSV file with comma delimiter
    wrk.create(
        "comma.csv",
        vec![
            svec!["id", "name", "age"],
            svec!["1", "John", "25"],
            svec!["2", "Jane", "30"],
        ],
    );

    // Create TSV file with tab delimiter
    wrk.create(
        "tab.tsv",
        vec![
            svec!["id\tname\tage"],
            svec!["1\tJohn\t25"],
            svec!["2\tJane\t30"],
        ],
    );

    // Test validating multiple files with different delimiters
    let mut cmd = wrk.command("validate");
    cmd.arg("comma.csv").arg("tab.tsv");

    wrk.assert_success(&mut cmd);

    let got: String = wrk.output_stderr(&mut cmd);
    assert!(got.contains("✅ All 2 files are valid."));
}

#[test]
fn validate_multiple_files_no_headers() {
    let wrk = Workdir::new("validate_multiple_files_no_headers").flexible(true);

    // Create files without headers
    wrk.create(
        "file1.csv",
        vec![svec!["1", "John", "25"], svec!["2", "Jane", "30"]],
    );

    wrk.create(
        "file2.csv",
        vec![
            svec!["Laptop", "999.99", "50"],
            svec!["Phone", "699.99", "100"],
        ],
    );

    // Test validating multiple files without headers
    let mut cmd = wrk.command("validate");
    cmd.arg("--no-headers").arg("file1.csv").arg("file2.csv");

    wrk.assert_success(&mut cmd);

    let got: String = wrk.output_stderr(&mut cmd);
    assert!(got.contains("✅ All 2 files are valid."));
}

#[test]
fn validate_single_file_backward_compatibility() {
    let wrk = Workdir::new("validate_single_file_backward_compatibility").flexible(true);

    // Create a single CSV file
    wrk.create(
        "data.csv",
        vec![
            svec!["id", "name", "age"],
            svec!["1", "John", "25"],
            svec!["2", "Jane", "30"],
        ],
    );

    // Test that single file validation still works exactly as before
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv");

    wrk.assert_success(&mut cmd);

    let got_stderr: String = wrk.output_stderr(&mut cmd);
    let output = wrk.output(&mut cmd);
    let got_stdout = String::from_utf8_lossy(&output.stdout);

    let expected = "Valid: 3 Columns: (\"id\", \"name\", \"age\"); Records: 2; Delimiter: ,";
    // The output might be on stdout instead of stderr
    if got_stderr.contains("No error") {
        assert_eq!(got_stdout.trim(), expected);
    } else {
        assert_eq!(got_stderr, expected);
    }
}

#[test]
fn validate_single_file_error_backward_compatibility() {
    let wrk = Workdir::new("validate_single_file_error_backward_compatibility").flexible(true);

    // Create a single CSV file with invalid data
    wrk.create(
        "data.csv",
        vec![
            svec!["id", "name", "age"],
            svec!["1", "John", "25"],
            svec!["2", "Jane"], // Missing age field
        ],
    );

    // Test that single file error handling still works exactly as before
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv");

    wrk.assert_err(&mut cmd);

    let got: String = wrk.output_stderr(&mut cmd);
    // The exact byte position may vary, so we check for key components
    assert!(got.contains("Validation error: CSV error: record 2 (line: 3, byte:"));
    assert!(got.contains("found record with 2 fields, but the previous record has 3 fields."));
    assert!(got.contains("Use `qsv fixlengths` to fix record length issues."));
}

#[test]
fn validate_json_schema_still_single_file() {
    let wrk = Workdir::new("validate_json_schema_still_single_file").flexible(true);

    // Create a JSON schema
    wrk.create_from_string(
        "schema.json",
        r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "id": { "type": "string" },
                "name": { "type": "string" },
                "age": { "type": "integer" }
            }
        }"#,
    );

    // Create a CSV file
    wrk.create(
        "data.csv",
        vec![
            svec!["id", "name", "age"],
            svec!["1", "John", "25"],
            svec!["2", "Jane", "30"],
        ],
    );

    // Test that JSON Schema validation still works with single file
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv").arg("schema.json");

    wrk.assert_success(&mut cmd);

    let got: String = wrk.output_stderr(&mut cmd);
    let expected = "All 2 records valid.\n";
    assert_eq!(got, expected);
}

#[test]
fn validate_json_schema_rejects_multiple_files() {
    let wrk = Workdir::new("validate_json_schema_rejects_multiple_files").flexible(true);

    // Create a JSON schema
    wrk.create_from_string(
        "schema.json",
        r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "id": { "type": "string" },
                "name": { "type": "string" }
            }
        }"#,
    );

    // Create multiple CSV files
    wrk.create("file1.csv", vec![svec!["id", "name"], svec!["1", "John"]]);

    wrk.create("file2.csv", vec![svec!["id", "name"], svec!["2", "Jane"]]);

    // Test that JSON Schema validation rejects multiple files
    let mut cmd = wrk.command("validate");
    cmd.arg("file1.csv").arg("file2.csv").arg("schema.json");

    wrk.assert_err(&mut cmd);

    let got: String = wrk.output_stderr(&mut cmd);
    let expected = "JSON Schema validation only supports a single input file. Use RFC 4180 \
                    validation mode for multiple files.\n";
    assert_eq!(got, expected);
}

#[test]
fn validate_with_email_format_strict_default() {
    let wrk = Workdir::new("validate_with_email_format_strict_default").flexible(true);

    // Create test data with valid and invalid email addresses
    wrk.create(
        "data.csv",
        vec![
            svec!["id", "name", "email"],
            svec!["1", "John Doe", "user@example.com"], // Valid email
            svec!["2", "Jane Smith", "admin@company.co.uk"], // Valid email
            svec!["3", "Bob Wilson", "not-an-email"],   // Invalid email
            // Technically valid email if "domain" is a local host
            svec!["4", "Alice Brown", "missing@domain"],
            svec!["5", "Charlie Davis", "@nodomain.com"], // Invalid email
            // Display text (invalid by default)
            svec!["6", "David Evans", "David Evans <devans@example.com>"],
            svec!["7", "Eve Green", "eve@[127.0.0.1]"], // Domain literal (invalid by default)
            svec!["8", "Frank Hall", "frank@sub.example.local"], // Valid email
        ],
    );

    // Create schema with email format constraint
    wrk.create_from_string(
        "schema.json",
        r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "id": { "type": "string" },
                "name": { "type": "string" },
                "email": { 
                    "type": "string",
                    "format": "email"
                }
            }
        }"#,
    );

    // Run validation WITH format validation (default - strict format validation)
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv").arg("schema.json");
    wrk.output(&mut cmd);

    wrk.assert_err(&mut cmd);

    // Check that format validation errors are present
    let validation_errors = wrk
        .read_to_string("data.csv.validation-errors.tsv")
        .unwrap();
    assert!(validation_errors.contains("is not a \"email\""));

    // Check valid records - should contain only valid emails
    let valid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.valid");
    let expected_valid = vec![
        svec!["1", "John Doe", "user@example.com"],
        svec!["2", "Jane Smith", "admin@company.co.uk"],
        svec!["4", "Alice Brown", "missing@domain"],
        svec!["8", "Frank Hall", "frank@sub.example.local"],
    ];
    assert_eq!(valid_records, expected_valid);

    // Check invalid records - should contain invalid emails
    let invalid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.invalid");
    let expected_invalid = vec![
        svec!["3", "Bob Wilson", "not-an-email"],
        svec!["5", "Charlie Davis", "@nodomain.com"],
        svec!["6", "David Evans", "David Evans <devans@example.com>"],
        svec!["7", "Eve Green", "eve@[127.0.0.1]"],
    ];
    assert_eq!(invalid_records, expected_invalid);
}

#[test]
fn validate_with_email_format_strict_email_options() {
    let wrk = Workdir::new("validate_with_email_format_strict_email_options").flexible(true);

    // Create test data with valid and invalid email addresses
    wrk.create(
        "data.csv",
        vec![
            svec!["id", "name", "email"],
            svec!["1", "John Doe", "user@example.com"],
            svec!["2", "Jane Smith", "admin@company.co.uk"],
            svec!["3", "Bob Wilson", "not-an-email"],
            svec!["4", "Alice Brown", "missing@domain"],
            svec!["5", "Charlie Davis", "@nodomain.com"],
            svec!["6", "David Evans", "David Evans <devans@example.com>"],
            svec!["7", "Eve Green", "eve@[127.0.0.1]"],
            svec!["8", "Frank Hall", "frank@sub.example.local"],
            svec![
                "9",
                "George Hall",
                "Georgy Dark <george@thedark.website.net>"
            ],
        ],
    );

    // Create schema with email format constraint
    wrk.create_from_string(
        "schema.json",
        r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "id": { "type": "string" },
                "name": { "type": "string" },
                "email": { 
                    "type": "string",
                    "format": "email"
                }
            }
        }"#,
    );

    // Run validation WITH email options
    let mut cmd = wrk.command("validate");
    cmd.arg("--email-display-text")
        .arg("--email-required-tld")
        .arg("--email-domain-literal")
        .arg("data.csv")
        .arg("schema.json");
    wrk.output(&mut cmd);

    wrk.assert_err(&mut cmd);

    // Check that format validation errors are present
    let validation_errors = wrk
        .read_to_string("data.csv.validation-errors.tsv")
        .unwrap();
    assert!(validation_errors.contains("is not a \"email\""));

    // Check valid records - should contain only valid emails
    let valid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.valid");
    let expected_valid = vec![
        svec!["1", "John Doe", "user@example.com"],
        svec!["2", "Jane Smith", "admin@company.co.uk"],
        svec!["6", "David Evans", "David Evans <devans@example.com>"],
        svec!["7", "Eve Green", "eve@[127.0.0.1]"],
        svec!["8", "Frank Hall", "frank@sub.example.local"],
        svec![
            "9",
            "George Hall",
            "Georgy Dark <george@thedark.website.net>"
        ],
    ];
    assert_eq!(valid_records, expected_valid);

    // Check invalid records - should contain invalid emails
    let invalid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.invalid");
    let expected_invalid = vec![
        svec!["3", "Bob Wilson", "not-an-email"],
        svec!["4", "Alice Brown", "missing@domain"],
        svec!["5", "Charlie Davis", "@nodomain.com"],
    ];
    assert_eq!(invalid_records, expected_invalid);

    // Run validation WITH email options and minimum subdomains
    let mut cmd = wrk.command("validate");
    cmd.arg("--email-display-text")
        .arg("--email-required-tld")
        .args(["--email-min-subdomains", "3"])
        .arg("data.csv")
        .arg("schema.json");
    wrk.output(&mut cmd);

    wrk.assert_err(&mut cmd);

    // Check that format validation errors are present
    let validation_errors = wrk
        .read_to_string("data.csv.validation-errors.tsv")
        .unwrap();
    assert!(validation_errors.contains("is not a \"email\""));

    // Check valid records - should contain only valid emails
    let valid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.valid");
    let expected_valid = vec![
        svec!["2", "Jane Smith", "admin@company.co.uk"],
        svec!["8", "Frank Hall", "frank@sub.example.local"],
        svec![
            "9",
            "George Hall",
            "Georgy Dark <george@thedark.website.net>"
        ],
    ];
    assert_eq!(valid_records, expected_valid);

    // Check invalid records - should contain invalid emails
    let invalid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.invalid");
    let expected_invalid = vec![
        svec!["1", "John Doe", "user@example.com"],
        svec!["3", "Bob Wilson", "not-an-email"],
        svec!["4", "Alice Brown", "missing@domain"],
        svec!["5", "Charlie Davis", "@nodomain.com"],
        svec!["6", "David Evans", "David Evans <devans@example.com>"],
        svec!["7", "Eve Green", "eve@[127.0.0.1]"],
    ];
    assert_eq!(invalid_records, expected_invalid);
}

#[test]
fn validate_with_hostname_format_strict() {
    let wrk = Workdir::new("validate_with_hostname_format_strict").flexible(true);

    // Create test data with valid and invalid hostnames
    wrk.create(
        "data.csv",
        vec![
            svec!["id", "name", "hostname"],
            svec!["1", "Server 1", "example.com"], // Valid hostname
            svec!["2", "Server 2", "subdomain.example.com"], // Valid hostname
            svec!["3", "Server 3", "host-name.co.uk"], // Valid hostname
            svec!["4", "Server 4", "not a hostname"], // Invalid hostname
            svec!["5", "Server 5", "host name with spaces"], // Invalid hostname
            svec!["6", "Server 6", "invalid..hostname"], // Invalid hostname
        ],
    );

    // Create schema with hostname format constraint
    wrk.create_from_string(
        "schema.json",
        r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "id": { "type": "string" },
                "name": { "type": "string" },
                "hostname": { 
                    "type": "string",
                    "format": "hostname"
                }
            }
        }"#,
    );

    // Run validation WITH format validation (default - strict format validation)
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv").arg("schema.json");
    wrk.output(&mut cmd);

    wrk.assert_err(&mut cmd);

    // Check that format validation errors are present
    let validation_errors = wrk
        .read_to_string("data.csv.validation-errors.tsv")
        .unwrap();
    assert!(validation_errors.contains("is not a \"hostname\""));

    // Check valid records - should contain only valid hostnames
    let valid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.valid");
    let expected_valid = vec![
        svec!["1", "Server 1", "example.com"],
        svec!["2", "Server 2", "subdomain.example.com"],
        svec!["3", "Server 3", "host-name.co.uk"],
    ];
    assert_eq!(valid_records, expected_valid);

    // Check invalid records - should contain invalid hostnames
    let invalid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.invalid");
    let expected_invalid = vec![
        svec!["4", "Server 4", "not a hostname"],
        svec!["5", "Server 5", "host name with spaces"],
        svec!["6", "Server 6", "invalid..hostname"],
    ];
    assert_eq!(invalid_records, expected_invalid);
}

#[test]
fn validate_with_ipv4_format_strict() {
    let wrk = Workdir::new("validate_with_ipv4_format_strict").flexible(true);

    // Create test data with valid and invalid IPv4 addresses
    wrk.create(
        "data.csv",
        vec![
            svec!["id", "name", "ip_address"],
            svec!["1", "Server 1", "192.168.1.1"], // Valid IPv4
            svec!["2", "Server 2", "10.0.0.1"],    // Valid IPv4
            svec!["3", "Server 3", "8.8.8.8"],     // Valid IPv4
            svec!["4", "Server 4", "999.999.999.999"], // Invalid IPv4
            svec!["5", "Server 5", "192.168.1"],   // Invalid IPv4
            svec!["6", "Server 6", "not.an.ip.address"], // Invalid IPv4
        ],
    );

    // Create schema with ipv4 format constraint
    wrk.create_from_string(
        "schema.json",
        r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "id": { "type": "string" },
                "name": { "type": "string" },
                "ip_address": { 
                    "type": "string",
                    "format": "ipv4"
                }
            }
        }"#,
    );

    // Run validation WITH format validation (default - strict format validation)
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv").arg("schema.json");
    wrk.output(&mut cmd);

    wrk.assert_err(&mut cmd);

    // Check that format validation errors are present
    let validation_errors = wrk
        .read_to_string("data.csv.validation-errors.tsv")
        .unwrap();
    assert!(validation_errors.contains("is not a \"ipv4\""));

    // Check valid records - should contain only valid IPv4 addresses
    let valid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.valid");
    let expected_valid = vec![
        svec!["1", "Server 1", "192.168.1.1"],
        svec!["2", "Server 2", "10.0.0.1"],
        svec!["3", "Server 3", "8.8.8.8"],
    ];
    assert_eq!(valid_records, expected_valid);

    // Check invalid records - should contain invalid IPv4 addresses
    let invalid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.invalid");
    let expected_invalid = vec![
        svec!["4", "Server 4", "999.999.999.999"],
        svec!["5", "Server 5", "192.168.1"],
        svec!["6", "Server 6", "not.an.ip.address"],
    ];
    assert_eq!(invalid_records, expected_invalid);
}

#[test]
fn validate_with_ipv6_format_strict() {
    let wrk = Workdir::new("validate_with_ipv6_format_strict").flexible(true);

    // Create test data with valid and invalid IPv6 addresses
    wrk.create(
        "data.csv",
        vec![
            svec!["id", "name", "ip_address"],
            svec!["1", "Server 1", "2001:0db8:85a3:0000:0000:8a2e:0370:7334"], // Valid IPv6
            svec!["2", "Server 2", "2001:db8:85a3::8a2e:370:7334"],            // Valid IPv6
            svec!["3", "Server 3", "::1"],                                     // Valid IPv6
            svec!["4", "Server 4", "2001:db8::1"],                             // Valid IPv6
            svec!["5", "Server 5", "not:an:ipv6:address"],                     // Invalid IPv6
            svec!["6", "Server 6", "2001::db8::1"], // Invalid IPv6 (double colon)
            svec!["7", "Server 7", "2001:db8:"],    // Invalid IPv6
        ],
    );

    // Create schema with ipv6 format constraint
    wrk.create_from_string(
        "schema.json",
        r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "id": { "type": "string" },
                "name": { "type": "string" },
                "ip_address": { 
                    "type": "string",
                    "format": "ipv6"
                }
            }
        }"#,
    );

    // Run validation WITH format validation (default - strict format validation)
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv").arg("schema.json");
    wrk.output(&mut cmd);

    wrk.assert_err(&mut cmd);

    // Check that format validation errors are present
    let validation_errors = wrk
        .read_to_string("data.csv.validation-errors.tsv")
        .unwrap();
    assert!(validation_errors.contains("is not a \"ipv6\""));

    // Check valid records - should contain only valid IPv6 addresses
    let valid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.valid");
    let expected_valid = vec![
        svec!["1", "Server 1", "2001:0db8:85a3:0000:0000:8a2e:0370:7334"],
        svec!["2", "Server 2", "2001:db8:85a3::8a2e:370:7334"],
        svec!["3", "Server 3", "::1"],
        svec!["4", "Server 4", "2001:db8::1"],
    ];
    assert_eq!(valid_records, expected_valid);

    // Check invalid records - should contain invalid IPv6 addresses
    let invalid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.invalid");
    let expected_invalid = vec![
        svec!["5", "Server 5", "not:an:ipv6:address"],
        svec!["6", "Server 6", "2001::db8::1"],
        svec!["7", "Server 7", "2001:db8:"],
    ];
    assert_eq!(invalid_records, expected_invalid);
}

#[test]
fn validate_with_multiple_formats_strict() {
    let wrk = Workdir::new("validate_with_multiple_formats_strict").flexible(true);

    // Create test data with mixed valid and invalid formats
    wrk.create(
        "data.csv",
        vec![
            svec!["id", "email", "hostname", "ipv4_address", "ipv6_address"],
            svec![
                "1",
                "user@example.com",
                "example.com",
                "192.168.1.1",
                "2001:db8::1"
            ], // All valid
            svec![
                "2",
                "not-an-email",
                "example.com",
                "192.168.1.1",
                "2001:db8::1"
            ], // Invalid email
            svec![
                "3",
                "user@example.com",
                "not a hostname",
                "192.168.1.1",
                "2001:db8::1"
            ], // Invalid hostname
            svec![
                "4",
                "user@example.com",
                "example.com",
                "999.999.999.999",
                "2001:db8::1"
            ], // Invalid IPv4
            svec![
                "5",
                "user@example.com",
                "example.com",
                "192.168.1.1",
                "not:an:ipv6"
            ], // Invalid IPv6
            svec![
                "6",
                "admin@company.co.uk",
                "subdomain.example.com",
                "10.0.0.1",
                "::1"
            ], // All valid
        ],
    );

    // Create schema with multiple format constraints (email, hostname, ipv4, ipv6)
    wrk.create_from_string(
        "schema.json",
        r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "id": { "type": "string" },
                "email": { 
                    "type": "string",
                    "format": "email"
                },
                "hostname": { 
                    "type": "string",
                    "format": "hostname"
                },
                "ipv4_address": { 
                    "type": "string",
                    "format": "ipv4"
                },
                "ipv6_address": { 
                    "type": "string",
                    "format": "ipv6"
                }
            }
        }"#,
    );

    // Run validation WITH format validation (default - strict format validation)
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv").arg("schema.json");
    wrk.output(&mut cmd);

    wrk.assert_err(&mut cmd);

    // Check that format validation errors are present for each format type
    let validation_errors = wrk
        .read_to_string("data.csv.validation-errors.tsv")
        .unwrap();
    assert!(validation_errors.contains("is not a \"email\""));
    assert!(validation_errors.contains("is not a \"hostname\""));
    assert!(validation_errors.contains("is not a \"ipv4\""));
    assert!(validation_errors.contains("is not a \"ipv6\""));

    // Check valid records - should contain only records with all valid formats
    let valid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.valid");
    let expected_valid = vec![
        svec![
            "1",
            "user@example.com",
            "example.com",
            "192.168.1.1",
            "2001:db8::1"
        ],
        svec![
            "6",
            "admin@company.co.uk",
            "subdomain.example.com",
            "10.0.0.1",
            "::1"
        ],
    ];
    assert_eq!(valid_records, expected_valid);

    // Check invalid records - should contain records with any invalid format
    let invalid_records: Vec<Vec<String>> = wrk.read_csv("data.csv.invalid");
    let expected_invalid = vec![
        svec![
            "2",
            "not-an-email",
            "example.com",
            "192.168.1.1",
            "2001:db8::1"
        ],
        svec![
            "3",
            "user@example.com",
            "not a hostname",
            "192.168.1.1",
            "2001:db8::1"
        ],
        svec![
            "4",
            "user@example.com",
            "example.com",
            "999.999.999.999",
            "2001:db8::1"
        ],
        svec![
            "5",
            "user@example.com",
            "example.com",
            "192.168.1.1",
            "not:an:ipv6"
        ],
    ];
    assert_eq!(invalid_records, expected_invalid);
}
