use std::path::Path;

use assert_json_diff::assert_json_eq;
use serde_json::Value;

use crate::workdir::Workdir;

#[test]
fn generate_schema_with_defaults_and_validate_with_no_errors() {
    // create workspace and invoke schema command with value constraints flag
    let wrk = Workdir::new("schema").flexible(true);
    wrk.clear_contents().unwrap();

    // copy csv file to workdir
    let csv = wrk.load_test_resource("adur-public-toilets.csv");
    wrk.create_from_string("adur-public-toilets.csv", &csv);

    // run schema command with value constraints option
    let mut cmd = wrk.command("schema");
    cmd.arg("adur-public-toilets.csv");
    wrk.output(&mut cmd);

    // load output schema file
    let output_schema_string: String =
        wrk.from_str(&wrk.path("adur-public-toilets.csv.schema.json"));
    let output_schema_json =
        serde_json::from_str(&output_schema_string).expect("parse schema json");

    // make sure it's a valid JSON Schema by compiling with jsonschema library
    jsonschema::JSONSchema::options()
        .compile(&output_schema_json)
        .expect("valid JSON Schema");

    // diff output json with expected json
    let expected_schema: String =
        wrk.load_test_resource("adur-public-toilets.csv.schema-default.expected.json");
    let expected_schema_json: Value = serde_json::from_str(&expected_schema).unwrap();
    assert_json_eq!(expected_schema_json, output_schema_json);

    // invoke validate command from schema created above
    let mut cmd2 = wrk.command("validate");
    cmd2.arg("adur-public-toilets.csv");
    cmd2.arg("adur-public-toilets.csv.schema.json");
    wrk.output(&mut cmd2);

    // not expecting any invalid rows, so confirm there are NO output files generated
    let validation_error_path = &wrk.path("adur-public-toilets.csv.validation-errors.tsv");
    println!("not expecting validation error file at: {validation_error_path:?}");
    assert!(!Path::new(validation_error_path).exists());
    assert!(!Path::new(&wrk.path("adur-public-toilets.csv.valid")).exists());
    assert!(!Path::new(&wrk.path("adur-public-toilets.csv.invalid")).exists());
    wrk.assert_success(&mut cmd);
}

#[test]
fn generate_schema_with_optional_flags_and_validate_with_errors() {
    // create workspace and invoke schema command with value constraints flag
    let wrk = Workdir::new("schema").flexible(true);
    wrk.clear_contents().unwrap();

    // copy csv file to workdir
    let csv = wrk.load_test_resource("adur-public-toilets.csv");
    wrk.create_from_string("adur-public-toilets.csv", &csv);

    // run schema command with value constraints option
    let mut cmd = wrk.command("schema");
    cmd.arg("adur-public-toilets.csv");
    cmd.arg("--enum-threshold");
    cmd.arg("13");
    cmd.arg("--pattern-columns");
    cmd.arg("ReportEmail,OpeningHours");
    cmd.arg("--strict-dates");
    wrk.output(&mut cmd);

    // load output schema file
    let output_schema_string: String =
        wrk.from_str(&wrk.path("adur-public-toilets.csv.schema.json"));
    let output_schema_json =
        serde_json::from_str(&output_schema_string).expect("parse schema json");

    // make sure it's a valid JSON Schema by compiling with jsonschema library
    jsonschema::JSONSchema::options()
        .compile(&output_schema_json)
        .expect("valid JSON Schema");

    // diff output json with expected json
    let expected_schema: String =
        wrk.load_test_resource("adur-public-toilets.csv.schema-strict.expected.json");
    let expected_schema_json: Value = serde_json::from_str(&expected_schema).unwrap();
    assert_json_eq!(expected_schema_json, output_schema_json);

    // invoke validate command from schema created above
    let mut cmd2 = wrk.command("validate");
    cmd2.arg("adur-public-toilets.csv");
    cmd2.arg("adur-public-toilets.csv.schema.json");
    wrk.output(&mut cmd2);

    // validation report
    let validation_errors_expected = r#"row_number	field	error
2	ExtractDate	"07/07/2014 00:00" is not a "date-time"
3	ExtractDate	"2014-07-07 00:00" is not a "date-time"
4	ExtractDate	"07/07/2014 00:00" is not a "date-time"
5	ExtractDate	"07/07/2014 00:00" is not a "date-time"
6	ExtractDate	"07/07/2014 00:00" is not a "date-time"
7	ExtractDate	"07/07/2014 00:00" is not a "date-time"
8	ExtractDate	"07/07/2014 00:00" is not a "date-time"
9	ExtractDate	"07/07/2014 00:00" is not a "date-time"
10	ExtractDate	"07/07/2014 00:00" is not a "date-time"
11	ExtractDate	"07/07/2014 00:00" is not a "date-time"
12	ExtractDate	"07/07/2014 00:00" is not a "date-time"
13	ExtractDate	"07/07/2014 00:00" is not a "date-time"
14	ExtractDate	"07/07/2014 00:00" is not a "date-time"
15	ExtractDate	"07/07/2014 00:00" is not a "date-time"
"#;

    // expecting invalid rows, so confirm there ARE output files generated
    let validation_error_path = &wrk.path("adur-public-toilets.csv.validation-errors.tsv");
    println!("expecting validation error file at: {validation_error_path:?}");

    assert!(Path::new(validation_error_path).exists());
    assert!(Path::new(&wrk.path("adur-public-toilets.csv.valid")).exists());
    assert!(Path::new(&wrk.path("adur-public-toilets.csv.invalid")).exists());

    // check validation error output
    let validation_error_output: String =
        wrk.from_str(&wrk.path("adur-public-toilets.csv.validation-errors.tsv"));

    assert!(!validation_error_output.is_empty());

    assert_eq!(
        validation_errors_expected.to_string(),
        validation_error_output
    );
    wrk.assert_err(&mut cmd2);
}

#[test]
fn generate_schema_with_defaults_to_stdout() {
    // create workspace and invoke schema command with value constraints flag
    let wrk = Workdir::new("schema_stdout").flexible(true);
    wrk.clear_contents().unwrap();

    // copy csv file to workdir
    let csv = wrk.load_test_resource("adur-public-toilets.csv");
    wrk.create_from_string("adur-public-toilets.csv", &csv);

    // run schema command
    let mut cmd = wrk.command("schema");
    cmd.arg("--stdout").arg("adur-public-toilets.csv");
    let stdout = wrk.stdout::<String>(&mut cmd);
    let output_schema_json: Value = serde_json::from_str(&stdout).unwrap();

    // diff output json with expected json
    let expected_schema: String =
        wrk.load_test_resource("adur-public-toilets.csv.schema-default.expected.json");
    let expected_schema_json: Value = serde_json::from_str(&expected_schema).unwrap();

    assert_json_eq!(expected_schema_json, output_schema_json);
}
