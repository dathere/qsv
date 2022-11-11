static USAGE: &str = r#"
Validate CSV data with JSON Schema, and put invalid records into a separate file.
When run without JSON Schema, only a simple CSV check (RFC 4180) is performed.

Example output files from `mydata.csv`. If piped from stdin, then filename is `stdin.csv`.

* mydata.csv.valid
* mydata.csv.invalid
* mydata.csv.validation-errors.tsv

JSON Schema can be a local file or a URL.

Returns exitcode 0 when the CSV file is valid, exitcode 1 otherwise.

For examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_validate.rs.

Usage:
    qsv validate [options] [<input>] [<json-schema>]
    qsv validate --help

Validate options:
    --fail-fast                Stops on first error.
    --valid <suffix>           Valid record output file suffix. [default: valid]
    --invalid <suffix>         Invalid record output file suffix. [default: invalid]
    --json                     When validating without a schema, return the RFC 4180 check
                               as a JSON file instead of a message.
    --pretty-json              Same as --json, but pretty printed.
    -j, --jobs <arg>           The number of jobs to run in parallel.
                               When not set, the number of jobs is set to the
                               number of CPUs detected.


Common options:
    -h, --help                 Display this message
    -n, --no-headers           When set, the first row will not be interpreted
                               as headers. Namely, it will be sorted with the rest
                               of the rows. Otherwise, the first row will always
                               appear as the header row in the output.
    -d, --delimiter <arg>      The field delimiter for reading CSV data.
                               Must be a single character. [default: ,]
    -p, --progressbar          Show progress bars. Not valid for stdin.
"#;

use std::{
    env,
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    str,
};

use csv::ByteRecord;
#[cfg(any(feature = "full", feature = "lite"))]
use indicatif::{ProgressBar, ProgressDrawTarget};
use itertools::Itertools;
use jsonschema::{output::BasicOutput, paths::PathChunk, JSONSchema};
#[allow(unused_imports)]
use log::{debug, info};
use once_cell::sync::OnceCell;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, value::Number, Map, Value};
use thousands::Separable;

use crate::{
    config::{Config, Delimiter, DEFAULT_WTR_BUFFER_CAPACITY},
    util, CliResult,
};

// number of CSV rows to process in a batch
const BATCH_SIZE: usize = 24_000;

// to save on repeated init/allocs
static NULL_TYPE: once_cell::sync::OnceCell<Value> = OnceCell::new();

#[allow(dead_code)]
#[derive(Deserialize)]
struct Args {
    flag_fail_fast:   bool,
    flag_valid:       Option<String>,
    flag_invalid:     Option<String>,
    flag_json:        bool,
    flag_pretty_json: bool,
    flag_jobs:        Option<usize>,
    flag_no_headers:  bool,
    flag_delimiter:   Option<Delimiter>,
    flag_progressbar: bool,
    arg_input:        Option<String>,
    arg_json_schema:  Option<String>,
}

#[derive(Serialize, Deserialize)]
struct RFC4180Struct {
    delimiter_char: char,
    header_row:     bool,
    quote_char:     char,
    num_records:    u64,
    num_fields:     usize,
    fields:         Vec<String>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    #[cfg(any(feature = "full", feature = "lite"))]
    let mut rconfig = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers);
    #[cfg(feature = "datapusher_plus")]
    let rconfig = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers);

    let mut rdr = rconfig.reader()?;

    // prep progress bar
    #[cfg(any(feature = "full", feature = "lite"))]
    let progress = ProgressBar::with_draw_target(None, ProgressDrawTarget::stderr_with_hz(5));

    #[cfg(any(feature = "full", feature = "lite"))]
    let show_progress =
        (args.flag_progressbar || std::env::var("QSV_PROGRESSBAR").is_ok()) && !rconfig.is_stdin();

    #[cfg(any(feature = "full", feature = "lite"))]
    if show_progress {
        // for full row count, prevent CSV reader from aborting on inconsistent column count
        rconfig = rconfig.flexible(true);
        let record_count = util::count_rows(&rconfig)?;
        rconfig = rconfig.flexible(false);
        util::prep_progress(&progress, record_count);
    } else {
        progress.set_draw_target(ProgressDrawTarget::hidden());
    }

    // if no json schema supplied, only let csv reader RFC4180-validate csv file
    if args.arg_json_schema.is_none() {
        // just read csv file and let csv reader report problems

        let mut header_msg = String::new();
        let mut header_len = 0;
        let mut field_vec = vec![];
        if !args.flag_no_headers {
            let fields_result = rdr.headers();
            match fields_result {
                Ok(fields) => {
                    header_len = fields.len();
                    for field in fields.iter() {
                        field_vec.push(field.to_string());
                    }
                    let field_list = field_vec.join("\", \"");
                    header_msg = format!(
                        "{} columns (\"{field_list}\") and ",
                        header_len.separate_with_commas()
                    );
                }
                Err(e) => {
                    if args.flag_json || args.flag_pretty_json {
                        let header_error = json!({
                            "errors": [{
                                "title" : "Cannot read header",
                                "detail" : format!("{e}")
                            }]
                        });
                        return fail!(header_error.to_string());
                    }
                    return fail_clierror!("Cannot read header ({e}).");
                }
            }
        }

        let mut record_count: u64 = 0;
        for result in rdr.records() {
            #[cfg(any(feature = "full", feature = "lite"))]
            if show_progress {
                progress.inc(1);
            }

            if let Err(e) = result {
                if args.flag_json || args.flag_pretty_json {
                    let validation_error = json!({
                        "errors": [{
                            "title" : "Validation error",
                            "detail" : format!("{e}")
                        }]
                    });
                    return fail!(validation_error.to_string());
                }
                return fail_clierror!(
                    r#"Validation error: {e}. Try "qsv fixlengths" or "qsv fmt" to fix it."#
                );
            }
            record_count += 1;
        }

        #[cfg(any(feature = "full", feature = "lite"))]
        if show_progress {
            progress.set_message(format!(
                " validated {} records.",
                progress.length().unwrap().separate_with_commas()
            ));
            util::finish_progress(&progress);
        }

        let msg = if args.flag_json || args.flag_pretty_json {
            let rfc4180 = RFC4180Struct {
                delimiter_char: rconfig.get_delimiter() as char,
                header_row:     !rconfig.no_headers,
                quote_char:     rconfig.quote as char,
                num_records:    record_count,
                num_fields:     header_len,
                fields:         field_vec,
            };

            if args.flag_pretty_json {
                serde_json::to_string_pretty(&rfc4180).unwrap()
            } else {
                serde_json::to_string(&rfc4180).unwrap()
            }
        } else {
            format!(
                "Valid: {header_msg}{} records detected.",
                record_count.separate_with_commas()
            )
        };
        info!("{msg}");
        println!("{msg}");

        return Ok(());
    }

    let headers = rdr.byte_headers()?.clone();
    let headers_len = headers.len();

    // parse and compile supplied JSON Schema
    let (schema_json, schema_compiled): (Value, JSONSchema) =
        match load_json(&args.arg_json_schema.unwrap()) {
            Ok(s) => {
                // parse JSON string
                match serde_json::from_str(&s) {
                    Ok(json) => {
                        // compile JSON Schema
                        match JSONSchema::options().compile(&json) {
                            Ok(schema) => (json, schema),
                            Err(e) => {
                                return fail_clierror!("Cannot compile schema json. error: {e}");
                            }
                        }
                    }
                    Err(e) => {
                        return fail_clierror!("Unable to parse schema json. error: {e}");
                    }
                }
            }
            Err(e) => {
                return fail_clierror!("Unable to retrieve json. error: {e}");
            }
        };

    // debug!("compiled schema: {:?}", &schema_compiled);

    // how many rows read and processed as batches
    let mut row_number: usize = 0;
    // how many invalid rows found
    let mut invalid_count: usize = 0;

    // amortize memory allocation by reusing record
    #[allow(unused_assignments)]
    let mut record = csv::ByteRecord::new();
    // reuse batch buffer
    let mut batch = Vec::with_capacity(BATCH_SIZE);
    let mut validation_results = Vec::with_capacity(BATCH_SIZE);
    let mut valid_flags: Vec<bool> = Vec::with_capacity(BATCH_SIZE);
    let mut validation_error_messages: Vec<String> = Vec::with_capacity(50);

    // set RAYON_NUM_THREADS
    util::njobs(args.flag_jobs);

    // main loop to read CSV and construct batches for parallel processing.
    // each batch is processed via Rayon parallel iterator.
    // loop exits when batch is empty.
    loop {
        for _ in 0..BATCH_SIZE {
            match rdr.read_byte_record(&mut record) {
                Ok(has_data) => {
                    if has_data {
                        row_number += 1;
                        record.push_field(row_number.to_string().as_bytes());
                        // non-allocating trimming in place is much faster on the record level
                        // with our csv fork than doing per field std::str::trim which is allocating
                        record.trim();
                        batch.push(record.clone());
                    } else {
                        // nothing else to add to batch
                        break;
                    }
                }
                Err(e) => {
                    return fail_clierror!("Error reading row: {row_number}: {e}");
                }
            }
        }

        if batch.is_empty() {
            // break out of infinite loop when at EOF
            break;
        }

        // do actual validation via Rayon parallel iterator
        // validation_results vector should have same row count and in same order as input CSV
        batch
            .par_iter()
            .map(|record| {
                do_json_validation(
                    &headers,
                    headers_len,
                    record,
                    &schema_json,
                    &schema_compiled,
                )
            })
            .collect_into_vec(&mut validation_results);

        // write to validation error report, but keep Vec<bool> to gen valid/invalid files later
        // because Rayon collect() guaranteeds original order, can sequentially append results to
        // vector with each batch
        for result in &validation_results {
            if let Some(validation_error_msg) = result {
                invalid_count += 1;
                valid_flags.push(false);

                validation_error_messages.push(validation_error_msg.to_string());
            } else {
                valid_flags.push(true);
            }
        }

        #[cfg(any(feature = "full", feature = "lite"))]
        if show_progress {
            progress.inc(batch.len() as u64);
        }
        batch.clear();

        // for fail-fast, exit loop if batch has any error
        if args.flag_fail_fast && invalid_count > 0 {
            break;
        }
    } // end infinite loop

    #[cfg(any(feature = "full", feature = "lite"))]
    if show_progress {
        progress.set_message(format!(
            " validated {} records.",
            progress.length().unwrap().separate_with_commas()
        ));
        util::finish_progress(&progress);
    }

    // only write out invalid/valid/errors output files if there are actually invalid records.
    // if 100% invalid, valid file is not needed, but this is rare so OK with creating empty file.
    if invalid_count > 0 {
        let msg = "Writing invalid/valid/error files...";
        info!("{msg}");
        println!("{msg}");

        let input_path = args
            .arg_input
            .clone()
            .unwrap_or_else(|| "stdin.csv".to_string());

        write_error_report(&input_path, validation_error_messages)?;

        let valid_suffix = args.flag_valid.unwrap_or_else(|| "valid".to_string());
        let invalid_suffix = args.flag_invalid.unwrap_or_else(|| "invalid".to_string());

        split_invalid_records(
            &rconfig,
            &valid_flags[..],
            &headers,
            &input_path,
            &valid_suffix,
            &invalid_suffix,
        )?;
    }

    // done with validation; print output
    if invalid_count > 0 {
        let fail_fast_msg = if args.flag_fail_fast {
            format!(
                "fail-fast enabled. stopped after row {}.\n",
                row_number.separate_with_commas()
            )
        } else {
            String::new()
        };

        return fail_clierror!(
            "{fail_fast_msg}{} out of {} records invalid.",
            invalid_count.separate_with_commas(),
            row_number.separate_with_commas()
        );
    } else {
        winfo!("All {} records valid.", row_number.separate_with_commas());
    }

    Ok(())
}

fn split_invalid_records(
    rconfig: &Config,
    valid_flags: &[bool],
    headers: &ByteRecord,
    input_path: &str,
    valid_suffix: &str,
    invalid_suffix: &str,
) -> CliResult<()> {
    // track how many rows read for splitting into valid/invalid
    // should not exceed row_number when aborted early due to fail-fast
    let mut split_row_num: usize = 0;

    // prepare output writers
    let mut valid_wtr = Config::new(&Some(input_path.to_owned() + "." + valid_suffix)).writer()?;
    valid_wtr.write_byte_record(headers)?;

    let mut invalid_wtr =
        Config::new(&Some(input_path.to_owned() + "." + invalid_suffix)).writer()?;
    invalid_wtr.write_byte_record(headers)?;

    let mut rdr = rconfig.reader()?;

    let mut record = csv::ByteRecord::new();
    while rdr.read_byte_record(&mut record)? {
        split_row_num += 1;

        // length of valid_flags is max number of rows we can split
        if split_row_num > valid_flags.len() {
            break;
        }

        // vector is 0-based, row_num is 1-based
        let is_valid = valid_flags[split_row_num - 1];

        if is_valid {
            valid_wtr.write_byte_record(&record)?;
        } else {
            invalid_wtr.write_byte_record(&record)?;
        }
    }

    valid_wtr.flush()?;
    invalid_wtr.flush()?;

    Ok(())
}

fn write_error_report(input_path: &str, validation_error_messages: Vec<String>) -> CliResult<()> {
    let wtr_capacitys = env::var("QSV_WTR_BUFFER_CAPACITY")
        .unwrap_or_else(|_| DEFAULT_WTR_BUFFER_CAPACITY.to_string());
    let wtr_buffer_size: usize = wtr_capacitys.parse().unwrap_or(DEFAULT_WTR_BUFFER_CAPACITY);

    let output_file = File::create(input_path.to_owned() + ".validation-errors.tsv")?;

    let mut output_writer = BufWriter::with_capacity(wtr_buffer_size, output_file);

    output_writer.write_all(b"row_number\tfield\terror\n")?;

    // write out error report
    for error_msg in validation_error_messages {
        output_writer.write_all(error_msg.as_bytes())?;
        // since writer is buffered, it's more efficient to do additional write than append Newline
        // to message
        output_writer.write_all(&[b'\n'])?;
    }

    // flush error report; file gets closed automagically when out-of-scope
    output_writer.flush()?;

    Ok(())
}

/// if given record is valid, return None, otherwise, error file entry string
fn do_json_validation(
    headers: &ByteRecord,
    headers_len: usize,
    record: &ByteRecord,
    schema_json: &Value,
    schema_compiled: &JSONSchema,
) -> Option<String> {
    // row number was added as last column. We use unsafe from_utf8_unchecked to
    // skip UTF8 validation since we know its safe as we added it earlier
    let row_number_string = unsafe { str::from_utf8_unchecked(record.get(headers_len).unwrap()) };

    // debug!("instance[{row_number}]: {instance:?}");
    validate_json_instance(
        &(match to_json_instance(headers, headers_len, record, schema_json) {
            Ok(obj) => obj,
            Err(e) => {
                return Some(format!("{row_number_string}\t<RECORD>\t{e}"));
            }
        }),
        schema_compiled,
    )
    .map(|validation_errors| {
        // squash multiple errors into one long String with linebreaks
        let combined_errors: String = validation_errors
            .iter()
            .map(|tuple| {
                // validation error file format: row_number, field, error
                format!("{row_number_string}\t{}\t{}", tuple.0, tuple.1)
            })
            .join("\n");

        combined_errors
    })
}

/// convert CSV Record into JSON instance by referencing Type from Schema
fn to_json_instance(
    headers: &ByteRecord,
    headers_len: usize,
    record: &ByteRecord,
    schema: &Value,
) -> Result<Value, String> {
    // make sure schema has expected structure
    let schema_properties = match schema.get("properties") {
        Some(properties) => properties,
        None => {
            return fail!("JSON Schema missing 'properties' object");
        }
    };

    // map holds individual CSV fields converted as serde_json::Value
    // we use with_capacity to minimize allocs
    let mut json_object_map: Map<String, Value> = Map::with_capacity(headers_len);

    let null_type = NULL_TYPE.get_or_init(|| Value::String("null".to_string()));

    // iterate over each CSV field and convert to JSON type
    for (i, header) in headers.iter().enumerate() {
        // convert csv header to string
        let header_string = unsafe { std::str::from_utf8_unchecked(header).to_string() };
        // convert csv value to string; no trimming reqd as it's done on the record level beforehand
        let value_string = unsafe { std::str::from_utf8_unchecked(&record[i]).to_string() };

        // get json type from schema; defaults to STRING if not specified
        let field_def: &Value = schema_properties
            .get(&header_string)
            .unwrap_or(&Value::Null);

        let field_type_def: &Value = field_def.get("type").unwrap_or(&Value::Null);

        let json_type = match field_type_def {
            Value::String(s) => s,
            Value::Array(vec) => {
                // if can't find usable type info, defaults to "string"
                let mut return_val = "string";

                // grab the first entry that's not a "null", since it just means value is optional
                for val in vec {
                    if *val == *null_type {
                        // keep looking
                        continue;
                    }
                    return_val = match val.as_str() {
                        Some(s) => s,
                        None => {
                            return fail!("type info should be a JSON string");
                        }
                    };
                }

                return_val
            }
            _ => {
                // default to JSON String
                "string"
            }
        };

        // dbg!(i, &header_string, &value_string, &json_type);

        // if value_string is empty, then just put an empty JSON String
        if value_string.is_empty() {
            json_object_map.insert(header_string, Value::Null);
            continue;
        }

        match json_type {
            "string" => {
                json_object_map.insert(header_string, Value::String(value_string));
            }
            "number" => {
                if let Ok(float) = value_string.parse::<f64>() {
                    json_object_map.insert(
                        header_string,
                        Value::Number(Number::from_f64(float).expect("not a valid f64 float")),
                    );
                } else {
                    return fail_format!(
                        "Can't cast into Float. header: {header_string}, value: {value_string}, \
                         json type: {json_type}"
                    );
                }
            }
            "integer" => {
                if let Ok(int) = value_string.parse::<i64>() {
                    json_object_map.insert(header_string, Value::Number(Number::from(int)));
                } else {
                    return fail_format!(
                        "Can't cast into Integer. header: {header_string}, value: {value_string}, \
                         json type: {json_type}"
                    );
                }
            }
            "boolean" => {
                if let Ok(boolean) = value_string.parse::<bool>() {
                    json_object_map.insert(header_string, Value::Bool(boolean));
                } else {
                    return fail_format!(
                        "Can't cast into Boolean. header: {header_string}, value: {value_string}, \
                         json type: {json_type}"
                    );
                }
            }
            _ => {
                return fail_format!(
                    "Unsupported JSON type. header: {header_string}, value: {value_string}, json \
                     type: {json_type}"
                );
            }
        }
    }

    // dbg!(&json_object_map);

    Ok(Value::Object(json_object_map))
}

#[cfg(test)]
mod tests_for_csv_to_json_conversion {

    use serde_json::json;

    use super::*;

    /// get schema used for unit tests
    fn schema_json() -> Value {
        // from https://json-schema.org/learn/miscellaneous-examples.html
        serde_json::json!({
            "$id": "https://example.com/test.schema.json",
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "title": "test",
            "type": "object",
            "properties": {
                "A": {
                    "type": "string",
                },
                "B": {
                    "type": "number",
                },
                "C": {
                    "type": "integer",
                },
                "D": {
                    "type": "boolean",
                },
                "E": {
                    "type": ["string", "null"],
                },
                "F": {
                    "type": ["number", "null"],
                },
                "G": {
                    "type": ["integer", "null"],
                },
                "H": {
                    "type": ["boolean", "null"],
                },
                "I": {
                    "type": ["string", "null"],
                },
                "J": {
                    "type": ["number", "null"],
                },
                "K": {
                    "type": ["null", "integer"],
                },
                "L": {
                    "type": ["boolean", "null"],
                },
            }
        })
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn test_to_json_instance() {
        let csv = "A,B,C,D,E,F,G,H,I,J,K,L
        hello,3.1415,300000000,true,,,,,hello,3.1415,300000000,true";

        let mut rdr = csv::Reader::from_reader(csv.as_bytes());
        let headers = rdr.byte_headers().unwrap().clone();
        let mut record = rdr.byte_records().next().unwrap().unwrap();
        record.trim();

        assert_eq!(
            to_json_instance(&headers, headers.len(), &record, &schema_json())
                .expect("can't convert csv to json instance"),
            json!({
                "A": "hello",
                "B": 3.1415,
                "C": 300_000_000,
                "D": true,
                "E": null,
                "F": null,
                "G": null,
                "H": null,
                "I": "hello",
                "J": 3.1415,
                "K": 300_000_000,
                "L": true,
            })
        );
    }

    #[test]
    fn test_to_json_instance_cast_integer_error() {
        let csv = "A,B,C,D,E,F,G,H
        hello,3.1415,3.0e8,true,,,,";

        let mut rdr = csv::Reader::from_reader(csv.as_bytes());
        let headers = rdr.byte_headers().unwrap().clone();

        let result = to_json_instance(
            &headers,
            headers.len(),
            &rdr.byte_records().next().unwrap().unwrap(),
            &schema_json(),
        );
        assert!(&result.is_err());
        let error = result.err().unwrap();
        assert_eq!(
            "Can't cast into Integer. header: C, value: 3.0e8, json type: integer",
            error
        );
    }
}

/// Validate JSON instance against compiled JSON schema
/// If invalid, returns Some(Vec<(String,String)>) holding the error messages
fn validate_json_instance(
    instance: &Value,
    schema_compiled: &JSONSchema,
) -> Option<Vec<(String, String)>> {
    let validation_output = schema_compiled.apply(instance);

    // If validation output is Invalid, then grab field names and errors
    if validation_output.flag() {
        None
    } else {
        // get validation errors as String
        let validation_errors: Vec<(String, String)> = match validation_output.basic() {
            BasicOutput::Invalid(errors) => errors
                .iter()
                .map(|e| {
                    if let Some(PathChunk::Property(box_str)) = e.instance_location().last() {
                        (box_str.to_string(), e.error_description().to_string())
                    } else {
                        (
                            e.instance_location().to_string(),
                            e.error_description().to_string(),
                        )
                    }
                })
                .collect(),
            BasicOutput::Valid(_annotations) => {
                // shouldn't happen
                unreachable!("Unexpected error.");
            }
        };

        Some(validation_errors)
    }
}

#[cfg(test)]
mod tests_for_schema_validation {
    use super::*;

    fn schema_json() -> Value {
        // from https://json-schema.org/learn/miscellaneous-examples.html
        serde_json::json!({
            "$id": "https://example.com/person.schema.json",
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "title": "Person",
            "type": "object",
            "properties": {
                "title": {
                    "type": "string",
                    "description": "The person's title.",
                    "minLength": 2
                },
                "name": {
                    "type": "string",
                    "description": "The person's name.",
                    "minLength": 2
                },
                "age": {
                    "description": "Age in years which must be equal to or greater than 18.",
                    "type": "integer",
                    "minimum": 18
                }
            }
        })
    }

    fn compiled_schema() -> JSONSchema {
        JSONSchema::options()
            .compile(&schema_json())
            .expect("Invalid schema")
    }

    #[test]
    fn test_validate_with_no_errors() {
        let csv = "title,name,age
        Professor,Xaviers,60";

        let mut rdr = csv::Reader::from_reader(csv.as_bytes());
        let headers = rdr.byte_headers().unwrap().clone();

        let record = &rdr.byte_records().next().unwrap().unwrap();

        let instance = to_json_instance(&headers, headers.len(), record, &schema_json()).unwrap();

        let result = validate_json_instance(&instance, &compiled_schema());

        assert!(result.is_none());
    }

    #[test]
    fn test_validate_with_error() {
        let csv = "title,name,age
        Professor,X,60";

        let mut rdr = csv::Reader::from_reader(csv.as_bytes());
        let headers = rdr.byte_headers().unwrap().clone();

        let record = &rdr.byte_records().next().unwrap().unwrap();

        let instance = to_json_instance(&headers, headers.len(), record, &schema_json()).unwrap();

        let result = validate_json_instance(&instance, &compiled_schema());

        assert!(result.is_some());

        assert_eq!(
            vec![(
                "name".to_string(),
                "\"X\" is shorter than 2 characters".to_string()
            )],
            result.unwrap()
        );
    }
}

fn load_json(uri: &str) -> Result<String, String> {
    let json_string = match uri {
        url if url.starts_with("http") => {
            use reqwest::blocking::Client;
            let client = match Client::builder()
                .user_agent(util::DEFAULT_USER_AGENT)
                .brotli(true)
                .gzip(true)
                .deflate(true)
                .use_rustls_tls()
                .http2_adaptive_window(true)
                .build()
            {
                Ok(c) => c,
                Err(e) => {
                    return fail_format!("Cannot build reqwest client: {e}.");
                }
            };

            match client.get(url).send() {
                Ok(response) => response.text().unwrap_or_default(),
                Err(e) => return fail_format!("Cannot read JSON at url {url}: {e}."),
            }
        }
        path => {
            let mut buffer = String::new();
            match File::open(path) {
                Ok(p) => {
                    BufReader::new(p)
                        .read_to_string(&mut buffer)
                        .unwrap_or_default();
                }
                Err(e) => return fail_format!("Cannot read JSON file {path}: {e}."),
            }
            buffer
        }
    };

    Ok(json_string)
}

#[test]
fn test_load_json_via_url() {
    let json_string_result = load_json("https://geojson.org/schema/FeatureCollection.json");
    assert!(&json_string_result.is_ok());

    let json_result: Result<Value, serde_json::Error> =
        serde_json::from_str(&json_string_result.unwrap());
    assert!(&json_result.is_ok());
}
