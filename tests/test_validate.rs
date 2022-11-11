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

    wrk.output(&mut cmd);
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
    let expected =
        r#"Valid: 3 columns ("title", "name", "real age (earth years)") and 3 records detected."#;
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

    wrk.assert_err(&mut cmd);
}

#[test]
fn validate_bad_csv_json() {
    let wrk = Workdir::new("validate_bad_csv_json").flexible(true);
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
    cmd.arg("--json").arg("data.csv");

    let got: String = wrk.output_stderr(&mut cmd);
    assert!(got.contains("Validation error"));
}

fn adur_errors() -> &'static str {
    "row_number\tfield\terror\n1\tExtractDate\tnull is not of type \
     \"string\"\n1\tOrganisationLabel\tnull is not of type \
     \"string\"\n3\tCoordinateReferenceSystem\t\"OSGB3\" does not match \
     \"(WGS84|OSGB36)\"\n3\tCategory\t\"Mens\" does not match \"(Female|Male|Female and \
     Male|Unisex|Male urinal|Children only|None)\"\n"
}

// invalid records with index from original csv
// row 1: missing values for ExtractDate and OrganisationLabel
// row 3: wrong value for CoordinateReferenceSystem and Category
// note: removed unnecessary quotes for string column "OpeningHours"
fn adur_invalids() -> &'static str {
    "ExtractDate,OrganisationURI,OrganisationLabel,ServiceTypeURI,ServiceTypeLabel,LocationText,CoordinateReferenceSystem,GeoX,GeoY,GeoPointLicensingURL,Category,AccessibleCategory,RADARKeyNeeded,BabyChange,FamilyToilet,ChangingPlace,AutomaticPublicConvenience,FullTimeStaffing,PartOfCommunityScheme,CommunitySchemeName,ChargeAmount,InfoURL,OpeningHours,ManagedBy,ReportEmail,ReportTel,Notes,UPRN,Postcode,StreetAddress,GeoAreaURI,GeoAreaLabel\n\
    ,http://opendatacommunities.org/id/district-council/adur,,http://id.esd.org.uk/service/579,Public toilets,BEACH GREEN PUBLIC CONVENIENCES BRIGHTON ROAD LANCING,OSGB36,518072,103649,http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html,Female and male,Unisex,Yes,No,No,No,No,No,No,,,http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/,S = 09:00 - 21:00 W = 09:00 - 17:00 ,ADC,surveyor_1@adur-worthing.gov.uk,01903 221471,,60001449,,BEACH GREEN PUBLIC CONVENIENCES BRIGHTON ROAD LANCING,,\n\
    2014-07-07 00:00,http://opendatacommunities.org/id/district-council/adur,Adur,http://id.esd.org.uk/service/579,Public toilets,PUBLIC CONVENIENCES SHOPSDAM ROAD LANCING,OSGB3,518915,103795,http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html,Mens,Unisex,Yes,No,No,No,No,No,No,,,http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/,S = 09:00 - 21:00 W = 09:00 - 17:00,ADC,surveyor_3@adur-worthing.gov.uk,01903 221471,,60007428,,,,\n"
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
fn validate_adur_public_toilets_dataset_with_json_schema_url() {
    let wrk = Workdir::new("validate").flexible(true);

    // copy csv file to workdir
    let csv: String = wrk.load_test_resource("adur-public-toilets.csv");
    wrk.create_from_string("data.csv", &csv);

    // run validate command
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv").arg("https://raw.githubusercontent.com/jqnatividad/qsv/master/resources/test/public-toilets-schema.json");

    wrk.output(&mut cmd);

    let invalid_output: String = wrk.from_str(&wrk.path("data.csv.invalid"));
    assert_eq!(adur_invalids().to_string(), invalid_output);

    // check validation error output

    let validation_error_output: String = wrk.from_str(&wrk.path("data.csv.validation-errors.tsv"));
    assert_eq!(adur_errors(), validation_error_output);
    wrk.assert_err(&mut cmd);
}
