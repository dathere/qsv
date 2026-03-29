static USAGE: &str = r#"
Convert JSON to CSV.

The JSON data is expected to be non-empty and non-nested as either:

1. An array of objects where:
   A. All objects are non-empty, have non-empty and unique keys, and the same keys are in each object.
   B. Values are not objects or arrays.
2. An object where values are not objects or arrays and the object is as described above.

Objects with duplicate keys are not recommended as only one key and its values may be used.

If your JSON data is not in the expected format and/or is nested or complex, try using
the --jaq option to pass a jq-like filter before parsing with the above constraints.
Learn more about jaqhere: https://github.com/01mf02/jaq

As an example, say we have the following JSON data in a file fruits.json:

[
    {
        "fruit": "apple",
        "price": 2.50,
        "calories": 95
    },
    {
        "fruit": "banana",
        "price": 3.00,
        "calories": 105
    }
]

To convert it to CSV format run:

  $ qsv json fruits.json

And the following is printed to the terminal:

fruit,price,calories
apple,2.5,95
banana,3.0,105

IMPORTANT:
  The order of the columns in the CSV file will be the same as the order of the keys in the first JSON object.
  The order of the rows in the CSV file will be the same as the order of the objects in the JSON array.

  Additional keys not present in the first JSON object will be appended as additional columns in the
  output CSV in the order they appear.

For example, say we have the following JSON data in a file fruits2.json:

[
    {
        "fruit": "apple",
        "cost": 1.75,
        "price": 2.50,
        "calories": 95
    },
    {
        "fruit": "mangosteen",
        "price": 5.00,
        "calories": 56
    },
    {
        "fruit": "starapple",
        "rating": 9,
        "price": 4.50,
        "calories": 95,
    },
    {
        "fruit": "banana",
        "price": 3.00,
        "calories": 105
    }
]

If we run the following command:

  $ qsv json fruits2.json | qsv table

The output CSV will have the following columns:

fruit       cost  price  calories  rating
apple       1.75  2.5    95
mangosteen        5.0    56
starapple         4.5    95        9
banana            3.0    105

Note that the "rating" column is added as an additional column in the output CSV,
though it appears as the 2nd column in the third JSON object for "starapple".

If you want to select/reorder/drop columns in the output CSV, use the --select option, for example:

  $ qsv json fruits.json --select price,fruit

The following is printed to the terminal:

price,fruit
2.5,apple
3.0,banana

Note: Trailing zeroes in decimal numbers after the decimal are truncated (2.50 becomes 2.5).

If the JSON data was provided using stdin then either use - or do not provide a file path.
For example you may copy the JSON data above to your clipboard then run:

  $ qsv clipboard | qsv json

Again, when JSON data is nested or complex, try using the --jaq option and provide a filter value.

For example we have a .json file with a "data" key and the value being the same array as before:

{
    "data": [...]
}

We may run the following to select the JSON file and convert the nested array to CSV:

  $ qsv prompt -F json | qsv json --jaq .data

For more examples, see https://github.com/dathere/qsv/blob/master/tests/test_json.rs.

Usage:
    qsv json [options] [<input>]
    qsv json --help

json options:
    --jaq <filter>         Filter JSON data using jaq syntax (https://github.com/01mf02/jaq),
                           which is identical to the popular JSON command-line tool - jq.
                           https://jqlang.github.io/jq/
                           Note that the filter is applied BEFORE converting JSON to CSV
    -s, --select <cols>    Select, reorder or drop columns for output.
                           Otherwise, all the columns will be output in the same order as
                           the first object's keys in the JSON data.
                           See 'qsv select --help' for the full syntax.
                           Note however that <cols> NEED to be a comma-delimited list
                           of column NAMES and NOT column INDICES.
                           [default: 1- ]

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
"#;

use std::{env, io::Read};

use jaq_core::{Compiler, Ctx, Vars, data, load, unwrap_valr};
use jaq_json::{Num, Val};
use json_objects_to_csv::{Json2Csv, flatten_json_object::Flattener};
use log::warn;
use serde::Deserialize;

use crate::{CliError, CliResult, config, select::SelectColumns, util};

#[derive(Deserialize)]
struct Args {
    arg_input:   Option<String>,
    flag_jaq:    Option<String>,
    flag_select: Option<SelectColumns>,
    flag_output: Option<String>,
}

impl From<json_objects_to_csv::Error> for CliError {
    fn from(err: json_objects_to_csv::Error) -> Self {
        match err {
            json_objects_to_csv::Error::Flattening(err) => {
                CliError::Other(format!("Flattening error: {err}"))
            },
            json_objects_to_csv::Error::FlattenedKeysCollision => {
                CliError::Other(format!("Flattening Key Collision error: {err}"))
            },
            json_objects_to_csv::Error::WrittingCSV(err) => {
                CliError::Other(format!("Writing CSV error: {err}"))
            },
            json_objects_to_csv::Error::ParsingJson(err) => {
                CliError::Other(format!("Parsing JSON error: {err}"))
            },
            json_objects_to_csv::Error::InputOutput(err) => CliError::Io(err),
            json_objects_to_csv::Error::IntoFile(err) => CliError::Io(err.into()),
        }
    }
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    fn get_value_from_stdin() -> CliResult<serde_json::Value> {
        // Create a buffer in memory for stdin
        let mut buffer: Vec<u8> = Vec::new();
        let stdin = std::io::stdin();
        let mut stdin_handle = stdin.lock();
        stdin_handle.read_to_end(&mut buffer)?;
        drop(stdin_handle);

        // Return the JSON contents of the buffer as serde_json::Value
        match serde_json::from_slice(&buffer) {
            Ok(value) => Ok(value),
            Err(err) => fail_clierror!("Failed to parse JSON from stdin: {err}"),
        }
    }

    fn get_value_from_path(path: String) -> CliResult<serde_json::Value> {
        // Open the file in read-only mode with buffer.
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::with_capacity(config::DEFAULT_RDR_BUFFER_CAPACITY, file);

        // Return the JSON contents of the file as serde_json::Value
        match serde_json::from_reader(reader) {
            Ok(value) => Ok(value),
            Err(err) => fail_clierror!("Failed to parse JSON from file: {err}"),
        }
    }

    let args: Args = util::get_args(USAGE, argv)?;

    let mut value = match args.arg_input {
        Some(path) if path == "-" => get_value_from_stdin()?,
        Some(path) => get_value_from_path(path)?,
        None => get_value_from_stdin()?,
    };

    if value.is_null() {
        return fail_clierror!("No JSON data found.");
    }

    if let Some(filter) = args.flag_jaq {
        // Parse jaq filter based on JSON input

        // Create the program from filter string
        let program = load::File {
            code: filter.as_str(),
            path: (),
        };

        // Setup loader and arena
        let loader = load::Loader::new(
            jaq_core::defs()
                .chain(jaq_std::defs())
                .chain(jaq_json::defs()),
        );
        let arena = load::Arena::default();

        // Parse the filter
        let modules = loader
            .load(&arena, program)
            .map_err(|e| CliError::Other(format!("Failed to parse jaq query: {e:?}")))?;

        // Compile the filter
        let jaq_filter = Compiler::default()
            .with_funs(
                jaq_core::funs()
                    .chain(jaq_std::funs())
                    .chain(jaq_json::funs()),
            )
            .compile(modules)
            .map_err(|e| CliError::Other(format!("Failed to compile jaq query: {e:?}")))?;

        // Convert serde_json::Value to jaq Val
        let input: Val = serde_json::from_value(value.clone())
            .map_err(|e| CliError::Other(format!("Failed to convert JSON to jaq value: {e}")))?;

        // Run the filter
        let ctx = Ctx::<data::JustLut<Val>>::new(&jaq_filter.lut, Vars::new([]));
        let out: Vec<Val> = jaq_filter
            .id
            .run((ctx, input))
            .map(unwrap_valr)
            .filter_map(|r| match r {
                Ok(v) => Some(v),
                Err(e) => {
                    warn!("jaq filter runtime error (value dropped): {e}");
                    None
                },
            })
            .collect();

        // Convert jaq output back to serde_json::Value
        let jaq_values: Vec<serde_json::Value> = out
            .into_iter()
            .filter_map(|v| match val_to_json_value(v) {
                Ok(val) => Some(val),
                Err(e) => {
                    warn!("jaq Val could not be converted to JSON (value dropped): {e}");
                    None
                },
            })
            .collect();

        if jaq_values.is_empty() {
            return fail_clierror!("jaq query returned no results.");
        }

        let jaq_value = if jaq_values.len() == 1 {
            jaq_values.into_iter().next().unwrap()
        } else {
            serde_json::Value::Array(jaq_values)
        };

        // If the result is an array wrapping another array, unwrap it.
        // This allows the user to filter with '.data' for {"data": [...]} instead of not being able
        // to use '.data'. Both '.data' and '.data[]' should work with this implementation.
        value = if jaq_value
            .as_array()
            .is_some_and(|arr| arr.first().is_some_and(serde_json::Value::is_array))
        {
            jaq_value.as_array().unwrap().first().unwrap().to_owned()
        } else {
            jaq_value
        };
    }

    if value.is_null() {
        return fail_clierror!("All JSON data filtered.");
    }

    let first_dict = if value.is_array() {
        value
            .as_array()
            .and_then(|arr| arr.first())
            .and_then(|val| val.as_object())
            .ok_or_else(|| CliError::Other("Expected an array of objects in JSON".to_string()))?
    } else {
        value
            .as_object()
            .ok_or_else(|| CliError::Other("Expected a JSON object".to_string()))?
    };
    if first_dict.is_empty() {
        return Err(CliError::Other(
            "Expected a non-empty JSON object".to_string(),
        ));
    }
    let mut first_dict_headers: Vec<&str> = Vec::new();
    for key in first_dict.keys() {
        if key.is_empty() {
            return Err(CliError::Other("Expected a non-empty JSON key".to_string()));
        }
        if first_dict_headers.contains(&key.as_str()) {
            return Err(CliError::Other(format!(
                "Expected non-duplicate keys, found key: {key}"
            )));
        }
        first_dict_headers.push(key.as_str());
    }

    // STEP 1: create an intermediate CSV tempfile from the JSON data
    // we need to do this so we can use qsv select to reorder headers to first dict's keys order
    // as the order of the headers in the CSV file is not guaranteed to be the same as the order of
    // the keys in the JSON object
    let temp_dir = env::temp_dir();
    let intermediate_csv = tempfile::Builder::new()
        .suffix(".csv")
        .tempfile_in(&temp_dir)?
        .into_temp_path()
        .to_string_lossy()
        .into_owned();

    // convert JSON to CSV
    // its inside a block so all the unneeded resources are freed & flushed after the block ends
    {
        let empty_values = vec![serde_json::Value::Null; 1];
        let values = if value.is_array() {
            value.as_array().unwrap_or(&empty_values)
        } else {
            &vec![value.clone()]
        };

        let flattener = Flattener::new();
        let intermediate_csv_writer = csv::WriterBuilder::new()
            .has_headers(true)
            .from_path(intermediate_csv.clone())?;
        Json2Csv::new(flattener)
            .preserve_key_order(true)
            .convert_from_array(values, intermediate_csv_writer)?;
    }

    // STEP 2: select the columns to use in the final output
    // Read the intermediate CSV to get the actual headers (which are sorted alphabetically)
    let sel_rconfig = config::Config::new(Some(intermediate_csv).as_ref()).no_headers(false);
    let mut intermediate_csv_rdr = sel_rconfig.reader()?;
    let byteheaders = intermediate_csv_rdr.byte_headers()?;

    // Convert byte headers to string headers for easier processing
    let actual_headers: Vec<String> = byteheaders
        .iter()
        .map(|h| String::from_utf8_lossy(h).to_string())
        .collect();

    // If --select is not specified, reorder the actual headers to match the first dict's key order
    let sel_cols = if let Some(select_cols) = args.flag_select {
        select_cols
    } else {
        // If all expected headers exist, use the original order
        if first_dict_headers
            .iter()
            .all(|&h| actual_headers.contains(&h.to_string()))
        {
            SelectColumns::parse(&first_dict_headers.join(",")).map_err(|e| {
                CliError::Other(format!(
                    "Failed to parse select columns in order of the first JSON dict: {e}"
                ))
            })?
        } else {
            // Otherwise, just use the headers as they appear in the CSV
            SelectColumns::parse(&actual_headers.join(","))
                .map_err(|e| CliError::Other(format!("Failed to parse select columns: {e}")))?
        }
    };

    // and write the selected columns to the final CSV file
    let sel = sel_rconfig.select(sel_cols).selection(byteheaders)?;
    let mut read_record = csv::ByteRecord::new();
    let mut write_record = csv::ByteRecord::new();
    let mut final_csv_wtr = config::Config::new(args.flag_output.as_ref())
        .no_headers(false)
        .writer()?;
    final_csv_wtr.write_record(sel.iter().map(|&i| &byteheaders[i]))?;
    while intermediate_csv_rdr.read_byte_record(&mut read_record)? {
        write_record.clear();
        write_record.extend(sel.iter().map(|&i| &read_record[i]));
        final_csv_wtr.write_byte_record(&write_record)?;
    }

    Ok(final_csv_wtr.flush()?)
}

/// Convert a jaq `Val` to a `serde_json::Value` without going through Display.
///
/// This avoids the `format!("{v}")` → `serde_json::from_str` round-trip which can
/// produce non-JSON output for NaN, Infinity, byte strings, and objects with non-string keys.
fn val_to_json_value(v: Val) -> Result<serde_json::Value, String> {
    match v {
        Val::Null => Ok(serde_json::Value::Null),
        Val::Bool(b) => Ok(serde_json::Value::Bool(b)),
        Val::Num(ref n) => match n {
            Num::Int(i) => Ok(serde_json::Value::Number((*i as i64).into())),
            Num::Float(f) => {
                if f.is_finite() {
                    serde_json::Number::from_f64(*f)
                        .map(serde_json::Value::Number)
                        .ok_or_else(|| format!("cannot represent {f} as JSON number"))
                } else {
                    // NaN and Infinity have no JSON representation — map to null like jq does
                    warn!("non-finite float {f} converted to null");
                    Ok(serde_json::Value::Null)
                }
            },
            Num::BigInt(bi) => {
                // Try to parse the BigInt string representation as a JSON number
                let s = bi.to_string();
                s.parse::<serde_json::Number>()
                    .map(serde_json::Value::Number)
                    .or_else(|_| {
                        // BigInt too large for JSON number — represent as string
                        warn!("BigInt {bi} too large for JSON number, converting to string");
                        Ok(serde_json::Value::String(s))
                    })
            },
            Num::Dec(s) => {
                // Decimal stored as string — parse it as a JSON number
                s.parse::<serde_json::Number>()
                    .map(serde_json::Value::Number)
                    .map_err(|e| format!("invalid decimal number {s}: {e}"))
            },
        },
        Val::TStr(ref s) => Ok(serde_json::Value::String(
            String::from_utf8_lossy(s).into_owned(),
        )),
        Val::BStr(ref s) => {
            warn!("byte string converted to lossy UTF-8 string for JSON");
            Ok(serde_json::Value::String(
                String::from_utf8_lossy(s).into_owned(),
            ))
        },
        Val::Arr(a) => {
            let items: Result<Vec<_>, _> = a.iter().cloned().map(val_to_json_value).collect();
            Ok(serde_json::Value::Array(items?))
        },
        Val::Obj(o) => {
            let mut map = serde_json::Map::new();
            for (k, v) in o.iter() {
                let key = match k {
                    Val::TStr(s) => String::from_utf8_lossy(s).into_owned(),
                    other => {
                        warn!("non-string object key converted to string: {other}");
                        format!("{other}")
                    },
                };
                map.insert(key, val_to_json_value(v.clone())?);
            }
            Ok(serde_json::Value::Object(map))
        },
    }
}
