#![cfg(target_family = "unix")]
use std::path::Path;

use assert_json_diff::assert_json_eq;
use serde_json::Value;
use serial_test::file_serial;

use crate::workdir::Workdir;

#[test]
#[file_serial]
fn generate_schema_with_defaults_and_validate_trim_with_no_errors() {
    // create workspace and invoke schema command with value constraints flag
    let wrk = Workdir::new("fn generate_schema_with_defaults_and_validate_trim_with_no_errors")
        .flexible(true);
    wrk.clear_contents().unwrap();

    // copy csv file to workdir
    let csv = wrk.load_test_resource("adur-public-toilets.csv");
    wrk.create_from_string("adur-public-toilets.csv", &csv);

    // run schema command with value constraints option
    let mut cmd = wrk.command("schema");
    cmd.arg("adur-public-toilets.csv");
    wrk.output(&mut cmd);

    wrk.assert_success(&mut cmd);

    // load output schema file
    let output_schema_string: String =
        wrk.from_str(&wrk.path("adur-public-toilets.csv.schema.json"));
    let output_schema_json =
        serde_json::from_str(&output_schema_string).expect("parse schema json");

    // make sure it's a valid JSON Schema by compiling with jsonschema library
    jsonschema::Validator::options()
        .build(&output_schema_json)
        .expect("valid JSON Schema");

    // diff output json with expected json
    let expected_schema: String =
        wrk.load_test_resource("adur-public-toilets.csv.schema-default.expected.json");
    let expected_schema_json: Value = serde_json::from_str(&expected_schema).unwrap();
    assert_json_eq!(expected_schema_json, output_schema_json);

    // invoke validate command from schema created above
    let mut cmd3 = wrk.command("validate");
    cmd3.arg("adur-public-toilets.csv");
    cmd3.arg("--trim");
    cmd3.arg("adur-public-toilets.csv.schema.json");
    wrk.output(&mut cmd3);

    // not expecting any invalid rows, so confirm there are NO output files generated
    let validation_error_path = &wrk.path("adur-public-toilets.csv.validation-errors.tsv");
    // println!("not expecting validation error file at: {validation_error_path:?}");
    assert!(!Path::new(validation_error_path).exists());
    assert!(!Path::new(&wrk.path("adur-public-toilets.csv.valid")).exists());
    assert!(!Path::new(&wrk.path("adur-public-toilets.csv.invalid")).exists());
    wrk.assert_success(&mut cmd);
}

#[test]
#[file_serial]
fn generate_schema_with_optional_flags_notrim_and_validate_with_errors() {
    // create workspace and invoke schema command with value constraints flag
    let wrk = Workdir::new("generate_schema_with_optional_flags_notrim_and_validate_with_errors")
        .flexible(true);
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
    jsonschema::Validator::options()
        .build(&output_schema_json)
        .expect("valid JSON Schema");

    // diff output json with expected json
    let expected_schema: String =
        wrk.load_test_resource("adur-public-toilets.csv.schema-strict.expected.json");
    let expected_schema_json: Value = serde_json::from_str(&expected_schema).unwrap();
    assert_json_eq!(expected_schema_json, output_schema_json);

    // invoke validate command from schema created above
    let mut cmd3 = wrk.command("validate");
    cmd3.arg("adur-public-toilets.csv");
    cmd3.arg("adur-public-toilets.csv.schema.json");
    wrk.output(&mut cmd3);

    // validation report
    let validation_errors_expected = r#"row_number	field	error
1	OpeningHours	"S = 09:00 - 21:00 W = 09:00 - 17:00 " is not one of null, "09.00 - 17.00" or 3 other candidates
2	ExtractDate	"07/07/2014 00:00" is not a "date"
3	ExtractDate	"2014-07-07 00:00" is not a "date"
4	ExtractDate	"07/07/2014 00:00" is not a "date"
5	ExtractDate	"07/07/2014 00:00" is not a "date"
6	ExtractDate	"07/07/2014 00:00" is not a "date"
7	ExtractDate	"07/07/2014 00:00" is not a "date"
8	ExtractDate	"07/07/2014 00:00" is not a "date"
9	ExtractDate	"07/07/2014 00:00" is not a "date"
10	ExtractDate	"07/07/2014 00:00" is not a "date"
11	ExtractDate	"07/07/2014 00:00" is not a "date"
12	ExtractDate	"07/07/2014 00:00" is not a "date"
13	ExtractDate	"07/07/2014 00:00" is not a "date"
14	ExtractDate	"07/07/2014 00:00" is not a "date"
15	ExtractDate	"07/07/2014 00:00" is not a "date"
"#;

    // expecting invalid rows, so confirm there ARE output files generated
    let validation_error_path = &wrk.path("adur-public-toilets.csv.validation-errors.tsv");
    // println!("expecting validation error file at: {validation_error_path:?}");

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
    wrk.assert_err(&mut cmd3);
}

#[test]
#[file_serial]
fn generate_schema_with_optional_flags_trim_and_validate_with_errors() {
    // create workspace and invoke schema command with value constraints flag
    let wrk = Workdir::new("generate_schema_with_optional_flags_trim_and_validate_with_errors")
        .flexible(true);
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
    jsonschema::Validator::options()
        .build(&output_schema_json)
        .expect("valid JSON Schema");

    // diff output json with expected json
    let expected_schema: String =
        wrk.load_test_resource("adur-public-toilets.csv.schema-strict.expected.json");
    let expected_schema_json: Value = serde_json::from_str(&expected_schema).unwrap();
    assert_json_eq!(expected_schema_json, output_schema_json);

    // invoke validate command from schema created above
    let mut cmd3 = wrk.command("validate");
    cmd3.arg("adur-public-toilets.csv");
    cmd3.arg("--trim");
    cmd3.arg("adur-public-toilets.csv.schema.json");
    wrk.output(&mut cmd3);

    // validation report
    let validation_errors_expected = r#"row_number	field	error
2	ExtractDate	"07/07/2014 00:00" is not a "date"
3	ExtractDate	"2014-07-07 00:00" is not a "date"
4	ExtractDate	"07/07/2014 00:00" is not a "date"
5	ExtractDate	"07/07/2014 00:00" is not a "date"
6	ExtractDate	"07/07/2014 00:00" is not a "date"
7	ExtractDate	"07/07/2014 00:00" is not a "date"
8	ExtractDate	"07/07/2014 00:00" is not a "date"
9	ExtractDate	"07/07/2014 00:00" is not a "date"
10	ExtractDate	"07/07/2014 00:00" is not a "date"
11	ExtractDate	"07/07/2014 00:00" is not a "date"
12	ExtractDate	"07/07/2014 00:00" is not a "date"
13	ExtractDate	"07/07/2014 00:00" is not a "date"
14	ExtractDate	"07/07/2014 00:00" is not a "date"
15	ExtractDate	"07/07/2014 00:00" is not a "date"
"#;

    // expecting invalid rows, so confirm there ARE output files generated
    let validation_error_path = &wrk.path("adur-public-toilets.csv.validation-errors.tsv");
    // println!("expecting validation error file at: {validation_error_path:?}");

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
    wrk.assert_err(&mut cmd3);
}

#[test]
#[file_serial]
fn generate_schema_with_defaults_to_stdout() {
    // create workspace and invoke schema command with value constraints flag
    let wrk = Workdir::new("generate_schema_with_defaults_to_stdout").flexible(true);
    wrk.clear_contents().unwrap();

    // copy csv file to workdir
    let csv = wrk.load_test_resource("adur-public-toilets.csv");
    wrk.create_from_string("adur-public-toilets.csv", &csv);

    // run schema command
    let mut cmd = wrk.command("schema");
    cmd.arg("adur-public-toilets.csv");
    wrk.output(&mut cmd);

    wrk.assert_success(&mut cmd);

    // load output schema file
    let output_schema_string: String =
        wrk.from_str(&wrk.path("adur-public-toilets.csv.schema.json"));
    let output_schema_json: Value =
        serde_json::from_str(&output_schema_string).expect("parse schema json");

    // diff output json with expected json
    let expected_schema: String =
        wrk.load_test_resource("adur-public-toilets.csv.schema-default.expected.json");
    let expected_schema_json: Value = serde_json::from_str(&expected_schema).unwrap();

    assert_json_eq!(expected_schema_json, output_schema_json);
}

#[test]
#[file_serial]
fn generate_schema_with_const_and_enum_constraints() {
    // create workspace and invoke schema command with value constraints flag
    let wrk = Workdir::new("generate_schema_with_const_and_enum_constraints").flexible(true);
    wrk.clear_contents().unwrap();

    let csv = "first,second,const_col,enum_col
1,r1,const,alpha
2,r2,const,beta
3,r3,const,charlie
4,r4,const,delta
5,r5,const,echo
6,r6,const,foxtrot
7,r7,const,echo
8,r8,const,delta
9,r9,const,charlie
";

    wrk.create_from_string("enum_const_test.csv", &csv);

    // run schema command with value constraints option
    let mut cmd = wrk.command("schema");
    cmd.arg("enum_const_test.csv");
    cmd.arg("--enum-threshold");
    cmd.arg("5");

    wrk.assert_success(&mut cmd);

    // load output schema file
    let output_schema_string: String = wrk.from_str(&wrk.path("enum_const_test.csv.schema.json"));
    let output_schema_json =
        serde_json::from_str(&output_schema_string).expect("parse schema json");

    // make sure it's a valid JSON Schema by compiling with jsonschema library
    jsonschema::Validator::options()
        .build(&output_schema_json)
        .expect("valid JSON Schema");

    let expected_schema = r#"{
  "$schema": "https://json-schema.org/draft-07/schema",
  "title": "JSON Schema for enum_const_test.csv",
  "description": "Inferred JSON Schema with `qsv schema enum_const_test.csv --enum-threshold 5`",
  "type": "object",
  "properties": {
    "first": {
      "description": "first column from enum_const_test.csv",
      "minimum": 1,
      "maximum": 9,
      "type": [
        "integer"
      ],
      "enum": [
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
        9
      ]
    },
    "second": {
      "description": "second column from enum_const_test.csv",
      "minLength": 2,
      "maxLength": 2,
      "type": [
        "string"
      ],
      "enum": [
        "r1",
        "r2",
        "r3",
        "r4",
        "r5",
        "r6",
        "r7",
        "r8",
        "r9"
      ]
    },
    "const_col": {
      "description": "const_col column from enum_const_test.csv",
      "minLength": 5,
      "maxLength": 5,
      "type": [
        "string"
      ],
      "const": "const"
    },
    "enum_col": {
      "description": "enum_col column from enum_const_test.csv",
      "minLength": 4,
      "maxLength": 7,
      "type": [
        "string"
      ],
      "enum": [
        "alpha",
        "beta",
        "charlie",
        "delta",
        "echo",
        "foxtrot"
      ]
    }
  },
  "required": [
    "first",
    "second",
    "const_col",
    "enum_col"
  ]
}"#;

    assert_eq!(output_schema_string, expected_schema);
}

#[test]
#[file_serial]
fn generate_schema_tsv_delimiter_detection_issue_2997() {
    // Test for issue #2997: schema command should correctly detect tab delimiter for TSV files
    // Previously, TSV files were parsed with comma delimiter, causing all columns to be
    // concatenated into a single property. This test ensures proper delimiter detection.
    let wrk = Workdir::new("generate_schema_tsv_delimiter_detection").flexible(true);
    wrk.clear_contents().unwrap();

    // Create TSV data from the GitHub issue #2997
    let tsv_data = "date\tcategory\tsub_category\tamount\tfrom\tto\tdescription\turl
2025-01-01\tSTART\tN/A\t3023.46\t-\t-\tBalance from 2024\tN/A
2025-01-28\tDEBIT\tTRADE\t-3023\tDeployer 0xb1\tVelodrome\tSell for USDC\thttps://optimistic.etherscan.io/tx/0x79e857395945f03a57454efcccdc5ca00430f5886018d71a9a137808a60fe7fc";

    wrk.create_from_string("test_data.tsv", &tsv_data);

    // run schema command (without explicit delimiter - should auto-detect)
    let mut cmd = wrk.command("schema");
    cmd.arg("test_data.tsv");

    wrk.assert_success(&mut cmd);

    // load output schema file
    let output_schema_string: String = wrk.from_str(&wrk.path("test_data.tsv.schema.json"));
    let output_schema_json: Value =
        serde_json::from_str(&output_schema_string).expect("parse schema json");

    // make sure it's a valid JSON Schema by compiling with jsonschema library
    jsonschema::Validator::options()
        .build(&output_schema_json)
        .expect("valid JSON Schema");

    // Verify that the schema has separate properties for each column
    // (not a single concatenated property as in the bug report)
    let properties = output_schema_json["properties"].as_object().unwrap();

    // Should have 8 separate properties, one for each column
    assert_eq!(properties.len(), 8);

    // Verify each expected column exists as a separate property
    let expected_columns = [
        "date",
        "category",
        "sub_category",
        "amount",
        "from",
        "to",
        "description",
        "url",
    ];
    for column in expected_columns {
        assert!(
            properties.contains_key(column),
            "Missing column: {}",
            column
        );

        // Each property should have a proper description referencing the individual column
        let column_desc = properties[column]["description"].as_str().unwrap();
        assert!(
            column_desc.contains(&format!("{} column", column)),
            "Column '{}' description should reference the column name",
            column
        );
    }

    // Verify that there is NO concatenated property (the bug from #2997)
    let concatenated_key = "date\tcategory\tsub_category\tamount\tfrom\tto\tdescription\turl";
    assert!(
        !properties.contains_key(concatenated_key),
        "Should not have concatenated property key: {}",
        concatenated_key
    );

    // Verify required fields include all 8 columns
    let required_fields = output_schema_json["required"].as_array().unwrap();
    assert_eq!(required_fields.len(), 8);
    for column in expected_columns {
        assert!(
            required_fields
                .iter()
                .any(|v| v.as_str().unwrap() == column),
            "Required fields should include: {}",
            column
        );
    }
}

#[test]
#[file_serial]
#[cfg(target_arch = "s390x")]
fn diagnose_schema_generation_s390x() {
    // This diagnostic test helps identify why validation error files aren't created on s390x
    // It outputs detailed schema and validation information to stderr (visible in CI logs)
    // ONLY RUNS ON S390X - use conditional compilation to avoid running on other platforms

    use std::fs;

    eprintln!("\n=== S390X SCHEMA DIAGNOSTIC TEST START ===");
    eprintln!(
        "Target endianness: {}",
        if cfg!(target_endian = "big") {
            "BIG"
        } else {
            "LITTLE"
        }
    );
    eprintln!("Target arch: {}", std::env::consts::ARCH);

    let wrk = Workdir::new("diagnose_schema_generation_s390x").flexible(true);
    wrk.clear_contents().unwrap();

    // Copy CSV file to workdir
    let csv = wrk.load_test_resource("adur-public-toilets.csv");
    wrk.create_from_string("adur-public-toilets.csv", &csv);

    eprintln!("\n--- Step 1: Generate Schema with --strict-dates ---");
    let mut cmd = wrk.command("schema");
    cmd.arg("--strict-dates");
    cmd.arg("--enum-threshold");
    cmd.arg("50");
    cmd.arg("adur-public-toilets.csv");
    let output = wrk.output(&mut cmd);

    eprintln!("Schema generation exit code: {:?}", output.status.code());
    eprintln!(
        "Schema generation stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let schema_path = wrk.path("adur-public-toilets.csv.schema.json");
    let schema_exists = Path::new(&schema_path).exists();
    eprintln!("Schema file exists: {}", schema_exists);

    if schema_exists {
        let schema_content: String = wrk.from_str(&schema_path);
        let schema_json: Value =
            serde_json::from_str(&schema_content).expect("Failed to parse generated schema");

        eprintln!("\n--- Generated Schema Analysis ---");
        if let Some(properties) = schema_json.get("properties").and_then(|p| p.as_object()) {
            eprintln!("Number of properties: {}", properties.len());

            // Check specific fields that might differ on big-endian
            for field_name in ["ExtractDate", "OpeningHours", "DateUpdated"] {
                if let Some(field_def) = properties.get(field_name) {
                    eprintln!("\nField '{}' definition:", field_name);
                    eprintln!("  Type: {:?}", field_def.get("type"));
                    eprintln!("  Format: {:?}", field_def.get("format"));

                    if let Some(enum_vals) = field_def.get("enum").and_then(|e| e.as_array()) {
                        eprintln!(
                            "  Enum values (first 5): {:?}",
                            enum_vals.iter().take(5).collect::<Vec<_>>()
                        );
                        eprintln!("  Enum value count: {}", enum_vals.len());
                    }

                    if let Some(const_val) = field_def.get("const") {
                        eprintln!("  Const value: {:?}", const_val);
                    }
                }
            }
        }

        // Load expected schema for comparison
        eprintln!("\n--- Comparing with Expected Schema ---");
        let expected_schema: String =
            wrk.load_test_resource("adur-public-toilets.csv.schema-strict.expected.json");
        let expected_json: Value =
            serde_json::from_str(&expected_schema).expect("Failed to parse expected schema");

        if let (Some(gen_props), Some(exp_props)) = (
            schema_json.get("properties").and_then(|p| p.as_object()),
            expected_json.get("properties").and_then(|p| p.as_object()),
        ) {
            for field_name in ["ExtractDate", "OpeningHours"] {
                if let (Some(gen_field), Some(exp_field)) =
                    (gen_props.get(field_name), exp_props.get(field_name))
                {
                    let fields_match = gen_field == exp_field;
                    eprintln!("Field '{}' matches expected: {}", field_name, fields_match);

                    if !fields_match {
                        eprintln!(
                            "  Generated: {}",
                            serde_json::to_string_pretty(gen_field).unwrap()
                        );
                        eprintln!(
                            "  Expected:  {}",
                            serde_json::to_string_pretty(exp_field).unwrap()
                        );
                    }
                }
            }
        }
    }

    eprintln!("\n--- Step 2: Run Validation ---");
    let mut cmd_validate = wrk.command("validate");
    cmd_validate.arg("adur-public-toilets.csv");
    cmd_validate.arg("adur-public-toilets.csv.schema.json");
    let validate_output = wrk.output(&mut cmd_validate);

    eprintln!("Validation exit code: {:?}", validate_output.status.code());
    eprintln!(
        "Validation stdout: {}",
        String::from_utf8_lossy(&validate_output.stdout)
    );
    eprintln!(
        "Validation stderr: {}",
        String::from_utf8_lossy(&validate_output.stderr)
    );

    eprintln!("\n--- Step 3: Check Output Files ---");
    let error_file = wrk.path("adur-public-toilets.csv.validation-errors.tsv");
    let valid_file = wrk.path("adur-public-toilets.csv.valid");
    let invalid_file = wrk.path("adur-public-toilets.csv.invalid");

    eprintln!(
        "Validation error file exists: {}",
        Path::new(&error_file).exists()
    );
    eprintln!("Valid file exists: {}", Path::new(&valid_file).exists());
    eprintln!("Invalid file exists: {}", Path::new(&invalid_file).exists());

    if Path::new(&error_file).exists() {
        let error_content = fs::read_to_string(&error_file).unwrap();
        let lines: Vec<&str> = error_content.lines().collect();
        eprintln!("\nValidation error file (first 10 lines):");
        for line in lines.iter().take(10) {
            eprintln!("  {}", line);
        }
        eprintln!("Total error lines: {}", lines.len());
    } else {
        eprintln!("\n⚠️  Validation error file was NOT created!");
        eprintln!("This suggests validation passed when it should have failed.");
    }

    // List all files in workdir
    eprintln!("\n--- All files in workdir ---");
    if let Ok(entries) = fs::read_dir(wrk.path(".")) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                eprintln!(
                    "  {} ({} bytes)",
                    entry.file_name().to_string_lossy(),
                    metadata.len()
                );
            }
        }
    }

    eprintln!("\n=== S390X SCHEMA DIAGNOSTIC TEST END ===\n");

    // This test always passes - we just want the diagnostic output
    // The actual assertion failures are in the other tests
}
