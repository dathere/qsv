static USAGE: &str = r#"
Generate JSON Schema or Polars Schema (with the `--polars` option) from CSV data.

JSON Schema Validation:
=======================
This command derives a JSON Schema Validation (Draft 7) file from CSV data, 
including validation rules based on data type and input data domain/range.
https://json-schema.org/draft/2020-12/json-schema-validation.html

Running `validate` command on original input CSV with generated schema 
should not flag any invalid records.

The intended workflow is to use the `schema` command to generate a JSON schema file
from representative CSV data, fine-tune the JSON schema file as needed, and then use
the `validate` command to validate other CSV data with the same structure using the
generated JSON schema.

After manually fine-tuning the JSON schema file, note that you can also use the
`validate` command to validate the JSON Schema file as well:

  `qsv validate schema manually-tuned-jsonschema.json`

The generated JSON schema file has `.schema.json` suffix appended. For example, 
for input `mydata.csv`, the generated JSON schema is `mydata.csv.schema.json`.

If piped from stdin, the schema file will be `stdin.csv.schema.json` and
a `stdin.csv` file will be created with stdin's contents as well.

Note that `stdin.csv` will be overwritten if it already exists.

Schema generation can be a compute-intensive process, especially for large CSV files.
To speed up generation, the `schema` command will reuse a `stats.csv.data.jsonl` file if it
exists and is current (i.e. stats generated with --cardinality and --infer-dates options).
Otherwise, it will run the `stats` command to generate the `stats.csv.data.jsonl` file first,
and then use that to generate the schema file.

Polars Schema:
==============
When the "polars" feature is enabled, the `--polars` option will generate a Polars schema
instead of a JSON Schema. The generated Polars schema will be written to a file with the
`.pschema.json` suffix appended to the input file stem.

The Polars schema is a JSON object that describes the schema of a CSV file. When present,
the `sqlp`, `joinp`, and `pivotp` commands will use the Polars schema to read the CSV file
instead of inferring the schema from the CSV data. Not only does this allow these commands to
skip schema inferencing which may fail when the inferencing sample is too low, it also allows 
Polars to optimize the query and gives the user the option to tailor the schema to their specific
query needs (e.g. using a Decimal type instead of a Float type).

For examples, see https://github.com/dathere/qsv/blob/master/tests/test_schema.rs.

Usage:
    qsv schema [options] [<input>]
    qsv schema --help

Schema options:
    --enum-threshold <num>     Cardinality threshold for adding enum constraints.
                               Enum constraints are compiled for String & Integer types.
                               [default: 50]
    -i, --ignore-case          Ignore case when compiling unique values for enum constraints.
                               Do note however that the `validate` command is case-sensitive
                               when validating against enum constraints.
    --strict-dates             Enforce Internet Datetime format (RFC-3339) for
                               detected date/datetime columns. Otherwise, even if
                               columns are inferred as date/datetime, they are set
                               to type "string" in the schema instead of
                               "date" or "date-time".
    --pattern-columns <args>   Select columns to derive regex pattern constraints.
                               That is, this will create a regular expression
                               that matches all values for each specified column.
                               Columns are selected using `select` syntax 
                               (see `qsv select --help` for details).
    --dates-whitelist <list>   The case-insensitive patterns to look for when 
                               shortlisting fields for date inference.
                               i.e. if the field's name has any of these patterns,
                               it is shortlisted for date inferencing.
                               Set to "all" to inspect ALL fields for
                               date/datetime types.
                               [default: date,time,due,open,close,created]
    --prefer-dmy               Prefer to parse dates in dmy format.
                               Otherwise, use mdy format.
    --force                    Force recomputing cardinality and unique values
                               even if stats cache file exists and is current.
    --stdout                   Send generated JSON schema file to stdout instead.
    -j, --jobs <arg>           The number of jobs to run in parallel.
                               When not set, the number of jobs is set to the
                               number of CPUs detected.

    --polars                   Infer a Polars schema instead of a JSON Schema.
                               This option is only available if the `polars` feature is enabled.
                               The generated Polars schema will be written to a file with the
                               `.pschema.json` suffix appended to the input file stem.

Common options:
    -h, --help                 Display this message
    -n, --no-headers           When set, the first row will not be interpreted
                               as headers. Namely, it will be processed with the rest
                               of the rows. Otherwise, the first row will always
                               appear as the header row in the output.
    -d, --delimiter <arg>      The field delimiter for reading CSV data.
                               Must be a single character. [default: ,]
    --memcheck                 Check if there is enough memory to load the entire
                               CSV into memory using CONSERVATIVE heuristics.
"#;

use std::{fs::File, io::Write, path::Path};

use csv::ByteRecord;
use foldhash::{HashMap, HashMapExt, HashSet};
use grex::RegExpBuilder;
use itertools::Itertools;
use log::{debug, error, info, warn};
use rayon::slice::ParallelSliceMut;
use serde_json::{Map, Value, json, value::Number};
use stats::Frequencies;

use crate::{
    CliResult,
    cmd::stats::StatsData,
    config::Config,
    util::{self, StatsMode},
};

const STDIN_CSV: &str = "stdin.csv";

pub fn run(argv: &[&str]) -> CliResult<()> {
    let mut args: util::SchemaArgs = util::get_args(USAGE, argv)?;

    #[cfg(feature = "polars")]
    if args.flag_polars {
        if let Some(input) = args.arg_input {
            let input_path = Path::new(&input);
            let schema_file = input_path.with_extension("pschema.json");
            if util::infer_polars_schema(
                args.flag_delimiter,
                log::log_enabled!(log::Level::Debug),
                input_path,
                &schema_file,
            )? {
                return Ok(());
            }
            return fail_clierror!(
                "Failed to infer Polars schema from {}",
                input_path.display()
            );
        }
        return fail_clierror!("Input file is required when using the --polars option.");
    }
    #[cfg(not(feature = "polars"))]
    if args.flag_polars {
        return fail_clierror!(
            "The --polars option is only available if the `polars` feature is enabled."
        );
    }

    // if using stdin, we create a stdin.csv file as stdin is not seekable and we need to
    // open the file multiple times to compile stats/unique values, etc.
    // We use a fixed "stdin.csv" filename instead of a temporary file with random characters
    // so the name of the generated schema.json file is readable and predictable
    // (stdin.csv.schema.json)
    let (input_path, input_filename) = if args.arg_input.is_none() {
        let mut stdin_file = File::create(STDIN_CSV)?;
        let stdin = std::io::stdin();
        let mut stdin_handle = stdin.lock();
        std::io::copy(&mut stdin_handle, &mut stdin_file)?;
        drop(stdin_handle);
        args.arg_input = Some(STDIN_CSV.to_string());
        (STDIN_CSV.to_string(), STDIN_CSV.to_string())
    } else {
        let filename = Path::new(args.arg_input.as_ref().unwrap())
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();
        (args.arg_input.clone().unwrap(), filename)
    };

    // we're loading the entire file into memory, we need to check avail mem
    util::mem_file_check(
        &std::path::PathBuf::from(&input_path),
        false,
        args.flag_memcheck,
    )?;

    // we can do this directly here, since args is mutable and
    // Config has not been created yet at this point
    args.flag_prefer_dmy = args.flag_prefer_dmy || util::get_envvar_flag("QSV_PREFER_DMY");
    if args.flag_prefer_dmy {
        winfo!("Prefer DMY set.");
    }

    // build schema for each field by their inferred type, min/max value/length, and unique values
    let mut properties_map: Map<String, Value> =
        match infer_schema_from_stats(&args, &input_filename, false) {
            Ok(map) => map,
            Err(e) => {
                return fail_clierror!(
                    "Failed to infer schema via stats and frequency from {input_filename}: {e}"
                );
            },
        };

    // generate regex pattern for selected String columns
    let pattern_map = generate_string_patterns(&args, &properties_map)?;

    // enrich properties map with pattern constraint for String fields
    for (field_name, field_def) in &mut properties_map {
        // dbg!(&field_name, &field_def);
        if pattern_map.contains_key(field_name) && should_emit_pattern_constraint(field_def) {
            let field_def_map = field_def.as_object_mut().unwrap();
            let pattern = Value::String(pattern_map[field_name].clone());
            field_def_map.insert("pattern".to_string(), pattern.clone());
            winfo!("Added regex pattern constraint for field: {field_name} -> {pattern}");
        }
    }

    // generate list of required fields
    let required_fields = get_required_fields(&properties_map);

    // create final JSON object for output
    let schema = json!({
        "$schema": "https://json-schema.org/draft-07/schema",
        "title": format!("JSON Schema for {input_filename}"),
        "description": format!("Inferred JSON Schema with `qsv {}`", argv[1..].join(" ")),
        "type": "object",
        "properties": Value::Object(properties_map),
        "required": Value::Array(required_fields)
    });

    let schema_pretty = match serde_json::to_string_pretty(&schema) {
        Ok(s) => s,
        Err(e) => return fail_clierror!("Cannot prettify schema json: {e}"),
    };

    if args.flag_stdout {
        let stdout = std::io::stdout();
        let mut handle = stdout.lock();

        handle.write_all(schema_pretty.as_bytes())?;
        handle.flush()?;

        info!("Schema written to stdout");
    } else {
        let schema_output_filename = input_path + ".schema.json";
        let mut schema_output_file = File::create(&schema_output_filename)?;

        schema_output_file.write_all(schema_pretty.as_bytes())?;
        schema_output_file.flush()?;

        woutinfo!("Schema written to {schema_output_filename}");
    }

    Ok(())
}

/// Builds JSON MAP object that corresponds to the "properties" object of JSON Schema (Draft 7
/// 2020-12) by looking at CSV value stats Supported JSON Schema validation vocabularies:
///  * type
///    - "null", "boolean", "number", "integer", or "string", with built-in support for date/datetime
///      as "string" with "format" constraint (https://json-schema.org/draft/2020-12/json-schema-validation#section-7.3.1)
///  * enum
///  * const
///  * minLength
///  * maxLength
///  * minimum
///  * maximum
pub fn infer_schema_from_stats(
    args: &util::SchemaArgs,
    input_filename: &str,
    quiet: bool,
) -> CliResult<Map<String, Value>> {
    // invoke cmd::stats
    let (csv_fields, csv_stats, _) = util::get_stats_records(args, StatsMode::Schema)?;

    // amortize memory allocation
    let mut low_cardinality_column_indices: Vec<u64> =
        Vec::with_capacity(args.flag_enum_threshold as usize);
    let mut const_column_indices: Vec<u64> = Vec::new();

    // build column selector arg to invoke cmd::frequency with
    let column_select_arg: String = build_low_cardinality_column_selector_arg(
        &mut low_cardinality_column_indices,
        args.flag_enum_threshold,
        &mut const_column_indices,
        &csv_stats,
    );

    // invoke cmd::frequency to get unique values for each field
    let unique_values_map = get_unique_values(args, &column_select_arg)?;

    // map holds "properties" object of json schema
    let mut properties_map: Map<String, Value> = Map::with_capacity(csv_fields.len());

    // amortize memory allocations
    let mut field_map: Map<String, Value> = Map::with_capacity(10);
    let mut type_list: Vec<Value> = Vec::with_capacity(4);
    let mut enum_list: Vec<Value> = Vec::with_capacity(args.flag_enum_threshold as usize);
    let mut const_value: Value;
    let mut header_string;
    let mut stats_record;
    let mut col_type;
    let mut col_null_count;
    let empty_string = String::new();

    // generate definition for each CSV column/field and add to properties_map
    for (i, csv_field) in csv_fields.iter().enumerate() {
        // convert csv header to string
        header_string = convert_to_string(csv_field)?;

        // grab stats record for current column
        stats_record = csv_stats[i].clone();

        // get Type from stats record
        col_type = stats_record.r#type.clone();

        // get NullCount
        col_null_count = stats_record.nullcount;

        // debug!(
        //     "{header_string}: type={col_type}, optional={}",
        //     col_null_count > 0
        // );

        // map for holding field definition
        field_map.clear();
        let desc = format!("{header_string} column from {input_filename}");
        field_map.insert("description".to_string(), Value::String(desc));

        // use list to hold types, since optional fields get appended a "null" type
        type_list.clear();
        enum_list.clear();
        const_value = Value::Null;

        match col_type.as_str() {
            "String" => {
                type_list.push(Value::String("string".to_string()));

                // minLength constraint
                if let Some(min_length) = stats_record.min_length {
                    field_map.insert(
                        "minLength".to_string(),
                        Value::Number(Number::from(min_length)),
                    );
                }

                // maxLength constraint
                if let Some(max_length) = stats_record.max_length {
                    field_map.insert(
                        "maxLength".to_string(),
                        Value::Number(Number::from(max_length)),
                    );
                }

                // const or enum constraint
                if const_column_indices.contains(&((i + 1) as u64))
                    && unique_values_map.contains_key(&header_string)
                {
                    const_value = Value::String(
                        unique_values_map[&header_string]
                            .first()
                            .unwrap_or(&empty_string)
                            .to_string(),
                    );
                } else if let Some(values) = unique_values_map.get(&header_string) {
                    for value in values {
                        enum_list.push(Value::String(value.to_string()));
                    }
                }
            },
            "Integer" => {
                type_list.push(Value::String("integer".to_string()));

                if let Some(min) = stats_record.min {
                    field_map.insert(
                        "minimum".to_string(),
                        Value::Number(Number::from(
                            atoi_simd::parse::<i64>(min.as_bytes()).unwrap(),
                        )),
                    );
                }

                if let Some(max) = stats_record.max {
                    field_map.insert(
                        "maximum".to_string(),
                        Value::Number(Number::from(
                            atoi_simd::parse::<i64>(max.as_bytes()).unwrap(),
                        )),
                    );
                }

                // enum constraint
                if let Some(values) = unique_values_map.get(&header_string) {
                    for value in values {
                        let int_value = atoi_simd::parse::<i64>(value.as_bytes()).unwrap();
                        enum_list.push(Value::Number(Number::from(int_value)));
                    }
                }
            },
            "Float" => {
                type_list.push(Value::String("number".to_string()));

                if let Some(min) = stats_record.min {
                    field_map.insert(
                        "minimum".to_string(),
                        Value::Number(Number::from_f64(min.parse::<f64>().unwrap()).unwrap()),
                    );
                }

                if let Some(max) = stats_record.max {
                    field_map.insert(
                        "maximum".to_string(),
                        Value::Number(Number::from_f64(max.parse::<f64>().unwrap()).unwrap()),
                    );
                }
            },
            "NULL" => {
                type_list.push(Value::String("null".to_string()));
            },
            "Date" => {
                type_list.push(Value::String("string".to_string()));

                if args.flag_strict_dates {
                    field_map.insert("format".to_string(), Value::String("date".to_string()));
                }
            },
            "DateTime" => {
                type_list.push(Value::String("string".to_string()));

                if args.flag_strict_dates {
                    field_map.insert("format".to_string(), Value::String("date-time".to_string()));
                }
            },
            _ => {
                // we do not support other types like Array or Object, default to JSON String
                wwarn!("Stats gave unexpected field type '{col_type}', default to JSON String.");
                type_list.push(Value::String("string".to_string()));
            },
        }

        if col_null_count > 0 && !type_list.contains(&Value::String("null".to_string())) {
            // for fields that are not mandatory,
            // having JSON String "null" in Type lists indicates that value can be missing
            type_list.push(Value::String("null".to_string()));
        }

        if col_null_count > 0 && !enum_list.is_empty() {
            // for fields that are not mandatory and actually have enum list generated,
            // having JSON NULL indicates that missing value is allowed
            enum_list.push(Value::Null);
        }

        if !type_list.is_empty() {
            field_map.insert("type".to_string(), Value::Array(type_list.clone()));
        }

        // add an enum or const constraint
        // if enum list is empty, see if we have a const
        if enum_list.is_empty() {
            if const_value != Value::Null {
                field_map.insert("const".to_string(), const_value.clone());
                if !quiet {
                    winfo!("Const generated for field '{header_string}': {const_value:?}");
                }
            }
        } else {
            // sort enum list
            enum_list.sort_unstable_by(|a, b| {
                match (a, b) {
                    (Value::Null, Value::Null) => std::cmp::Ordering::Equal,
                    (Value::Null, _) => std::cmp::Ordering::Less,
                    (_, Value::Null) => std::cmp::Ordering::Greater,
                    (Value::String(a_str), Value::String(b_str)) => a_str.cmp(b_str),
                    (Value::Number(a_num), Value::Number(b_num)) => a_num
                        .as_f64()
                        .unwrap_or_default()
                        .partial_cmp(&b_num.as_f64().unwrap_or_default())
                        .unwrap_or(std::cmp::Ordering::Equal),
                    // Compare types by their "priority"
                    _ => {
                        let type_priority = |v: &Value| match v {
                            Value::Null => 0,
                            Value::Bool(_) => 1,
                            Value::Number(_) => 2,
                            Value::String(_) => 3,
                            Value::Array(_) => 4,
                            Value::Object(_) => 5,
                        };
                        type_priority(a).cmp(&type_priority(b))
                    },
                }
            });

            field_map.insert("enum".to_string(), Value::Array(enum_list.clone()));
            if !quiet {
                winfo!(
                    "Enum list generated for field '{header_string}' ({} value/s)",
                    enum_list.len()
                );
            }
        }

        // add current field definition to properties map
        properties_map.insert(header_string, Value::Object(field_map.clone()));
    }

    Ok(properties_map)
}

/// get column selector argument string for low cardinality columns
fn build_low_cardinality_column_selector_arg(
    low_cardinality_column_indices: &mut Vec<u64>,
    enum_cardinality_threshold: u64,
    const_column_indices: &mut Vec<u64>,
    csv_stats: &[StatsData],
) -> String {
    low_cardinality_column_indices.clear();

    // identify low cardinality columns
    csv_stats.iter().enumerate().for_each(|(i, stat)| {
        // get Cardinality
        let col_cardinality = stat.cardinality;

        if col_cardinality == 1 {
            const_column_indices.push((i + 1) as u64);
        } else if col_cardinality > 1 && col_cardinality <= enum_cardinality_threshold {
            // column selector uses 1-based index
            low_cardinality_column_indices.push((i + 1) as u64);
        }
    });

    debug!("low cardinality columns: {low_cardinality_column_indices:?}");

    let column_select_arg: String = low_cardinality_column_indices
        .iter()
        .map(ToString::to_string)
        .join(",");

    column_select_arg
}

/// get frequency tables from `cmd::frequency`
/// returns map of unique values keyed by header
fn get_unique_values(
    args: &util::SchemaArgs,
    column_select_arg: &str,
) -> CliResult<HashMap<String, Vec<String>>> {
    // prepare arg for invoking cmd::frequency
    let freq_args = crate::cmd::frequency::Args {
        arg_input:            args.arg_input.clone(),
        flag_select:          crate::select::SelectColumns::parse(column_select_arg).unwrap(),
        flag_limit:           args.flag_enum_threshold as isize,
        flag_unq_limit:       args.flag_enum_threshold as usize,
        flag_lmt_threshold:   0,
        flag_pct_dec_places:  -5,
        flag_other_sorted:    false,
        flag_other_text:      "Other".to_string(),
        flag_asc:             false,
        flag_no_nulls:        true,
        flag_no_trim:         false,
        flag_ignore_case:     args.flag_ignore_case,
        flag_all_unique_text: "<ALL UNIQUE>".to_string(),
        flag_jobs:            Some(util::njobs(args.flag_jobs)),
        flag_output:          None,
        flag_no_headers:      args.flag_no_headers,
        flag_delimiter:       args.flag_delimiter,
        flag_memcheck:        args.flag_memcheck,
        flag_vis_whitespace:  false,
    };

    let curr_mode = std::env::var("QSV_STATSCACHE_MODE");
    // safety: we are in single-threaded code.
    unsafe { std::env::set_var("QSV_STATSCACHE_MODE", "none") };
    let (headers, ftables) = match freq_args.rconfig().indexed()? {
        Some(ref mut idx) => freq_args.parallel_ftables(idx),
        _ => freq_args.sequential_ftables(),
    }?;
    if let Ok(orig_mode) = curr_mode {
        // safety: we are in single-threaded code.
        unsafe { std::env::set_var("QSV_STATSCACHE_MODE", orig_mode) };
    }

    let unique_values_map = construct_map_of_unique_values(&headers, &ftables)?;
    Ok(unique_values_map)
}

/// construct map of unique values keyed by header
fn construct_map_of_unique_values(
    freq_csv_fields: &ByteRecord,
    frequency_tables: &[Frequencies<Vec<u8>>],
) -> CliResult<HashMap<String, Vec<String>>> {
    let mut unique_values_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut unique_values = Vec::with_capacity(freq_csv_fields.len());
    // iterate through fields and gather unique values for each field
    for (i, header_byte_slice) in freq_csv_fields.iter().enumerate() {
        unique_values.clear();

        for (val_byte_vec, _count) in frequency_tables[i].most_frequent().0 {
            let val_string = convert_to_string(val_byte_vec.as_slice())?;
            unique_values.push(val_string);
        }

        let header_string = convert_to_string(header_byte_slice)?;

        // sort the values so enum list so schema can be diff'ed between runs
        unique_values.par_sort_unstable();

        // if log::log_enabled!(log::Level::Debug) {
        //     // we do this as this debug is relatively expensive
        //     debug!(
        //         "enum[{header_string}]: len={}, val={:?}",
        //         unique_values.len(),
        //         unique_values
        //     );
        // }
        unique_values_map.insert(header_string, unique_values.clone());
    }
    // dbg!(&unique_values_map);

    Ok(unique_values_map)
}

/// convert byte slice to UTF8 String
#[inline]
fn convert_to_string(byte_slice: &[u8]) -> CliResult<String> {
    // convert csv header to string
    if let Ok(s) = simdutf8::basic::from_utf8(byte_slice) {
        Ok(s.to_string())
    } else {
        let lossy_string = String::from_utf8_lossy(byte_slice);
        fail_clierror!(
            "Can't convert byte slice to utf8 string. slice={byte_slice:?}: {lossy_string}"
        )
    }
}

/// determine required fields
fn get_required_fields(properties_map: &Map<String, Value>) -> Vec<Value> {
    let mut fields: Vec<Value> = Vec::with_capacity(properties_map.len());

    // for CSV, all columns in original input file are assume required
    for key in properties_map.keys() {
        fields.push(Value::String(key.clone()));
    }

    fields
}

/// generate map of regex patterns from selected String column of CSV
fn generate_string_patterns(
    args: &util::SchemaArgs,
    properties_map: &Map<String, Value>,
) -> CliResult<HashMap<String, String>> {
    let rconfig = Config::new(args.arg_input.as_ref())
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers)
        .select(args.flag_pattern_columns.clone());

    let mut rdr = rconfig.reader()?;

    let headers = rdr.byte_headers()?.clone();
    let sel = rconfig.selection(&headers)?;

    let mut pattern_map: HashMap<String, String> = HashMap::new();

    // return empty pattern map when:
    //  * no columns are selected
    //  * all columns are selected (by default, all columns are selected when no columns are
    //    explicitly specified)
    if sel.is_empty() || sel.len() == headers.len() {
        debug!("no pattern columns selected");
        return Ok(pattern_map);
    }

    // Map each Header to its unique Set of values
    let mut unique_values_map: HashMap<String, HashSet<String>> = HashMap::new();

    #[allow(unused_assignments)]
    let mut record = csv::ByteRecord::new();
    let mut header_byte_slice: &[u8];
    let mut header_string: String;
    let mut value_string: String;

    while rdr.read_byte_record(&mut record)? {
        for (i, value_byte_slice) in sel.select(&record).enumerate() {
            // get header based on column index in Selection array
            header_byte_slice = headers.get(sel[i]).unwrap();

            // convert header and value byte arrays to UTF8 strings
            header_string = convert_to_string(header_byte_slice)?;

            // pattern validation only applies to String type, so skip if not String
            if !should_emit_pattern_constraint(&properties_map[&header_string]) {
                continue;
            }

            value_string = convert_to_string(value_byte_slice)?;

            let set = unique_values_map.entry(header_string).or_default();
            set.insert(value_string);
        }
    }

    // build regex pattern for each header
    pattern_map.reserve(unique_values_map.len());
    let mut values: Vec<&String>;
    let mut regexp: String;

    for (header, value_set) in &unique_values_map {
        // Convert Set to Vector
        values = Vec::from_iter(value_set);

        // build regex based on unique values
        regexp = RegExpBuilder::from(&values)
            .with_conversion_of_repetitions()
            .with_minimum_repetitions(2)
            .build();

        pattern_map.insert(header.clone(), regexp);
    }

    // debug!("pattern map: {pattern_map:?}");

    Ok(pattern_map)
}

// only emit "pattern" constraint for String fields without enum constraint
fn should_emit_pattern_constraint(field_def: &Value) -> bool {
    let type_list = field_def[&"type"].as_array().unwrap();
    let has_enum = field_def.get("enum").is_some();

    type_list.contains(&Value::String("string".to_string())) && !has_enum
}
