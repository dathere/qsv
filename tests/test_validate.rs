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
  "delimiter_char": ",",
  "header_row": true,
  "quote_char": "\"",
  "num_records": 3,
  "num_fields": 3,
  "fields": [
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
    assert_eq!(got, "JSON Schema Meta-Reference Error: Unknown specification: https://json-schema.org/draft/2020-25/schema\n");

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
    assert_eq!(got, "Invalid JSON Schema.\n");
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
