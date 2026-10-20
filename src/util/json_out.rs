use std::{
    fs,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use super::{DEFAULT_BATCH_SIZE, DerivedFile, ROW_COUNT, bytes_to_cow_str};
use crate::{CliResult, cmd::stats::JsonTypes, config, config::Config};

pub fn create_json_writer(
    output: Option<&String>,
    buffer_capacity: usize,
) -> std::io::Result<Box<dyn Write + Send + 'static>> {
    // create a JSON writer
    // if flag_output is None or "-" then write to stdout
    let output = output.as_ref().map_or("-", |s| s.as_str());
    let buffer_size = if buffer_capacity == 0 {
        config::DEFAULT_WTR_BUFFER_CAPACITY
    } else {
        buffer_capacity
    };
    let writer: Box<dyn Write + Send + 'static> = match output {
        "-" => Box::new(std::io::BufWriter::with_capacity(
            buffer_size,
            std::io::stdout(),
        )),
        "stderr" => Box::new(std::io::BufWriter::with_capacity(
            buffer_size,
            std::io::stderr(),
        )),
        _ => Box::new(std::io::BufWriter::with_capacity(
            buffer_size,
            fs::File::create(output)?,
        )),
    };
    Ok(writer)
}

/// iterate over the CSV `ByteRecords` and write them to the JSON file.
///
/// Records are taken as `csv::Result<ByteRecord>` so callers can pass the
/// raw `byte_records()` iterator and have parse errors propagate with `?`
/// without buffering the entire slice in memory first.
///
/// **Failure semantics:** because records stream straight to the writer,
/// a CSV parse error mid-stream means the opening `[` and any prior records
/// will already have been written before the error propagates. The closing
/// `]` is only written after the loop completes cleanly, so the output
/// (stdout or `output` file) may be a truncated, syntactically invalid
/// JSON document on error. Callers that need all-or-nothing semantics must
/// validate / collect records before invoking this.
pub fn write_json(
    output: Option<&String>,
    no_headers: bool,
    headers: &csv::ByteRecord,
    records: impl Iterator<Item = csv::Result<csv::ByteRecord>>,
) -> CliResult<()> {
    let mut json_wtr = create_json_writer(output, config::DEFAULT_WTR_BUFFER_CAPACITY * 4)?;

    // keys are escaped once here, per JSON spec. Note that the escaped form
    // returned by serde_json includes the enclosing double quotes, so the
    // format string below must NOT add its own quotes around the key.
    let header_vec: Vec<String> = headers
        .iter()
        .enumerate()
        .map(|(col_idx, b)| {
            let key = if no_headers {
                col_idx.to_string()
            } else {
                bytes_to_cow_str(b).into_owned()
            };
            serde_json::Value::String(key).to_string()
        })
        .collect();

    // Write the opening bracket for the JSON array
    write!(json_wtr, "[")?;
    let mut is_first = true;

    let mut temp_val;
    let null_val = "null".to_string();
    let mut json_string_val: serde_json::Value;

    for record in records {
        let record = record?;
        if is_first {
            is_first = false;
        } else {
            // Write a comma before each record except the first one
            write!(json_wtr, ",")?;
        }
        write!(json_wtr, "{{")?;
        // Emit field separators using a leading-comma join: write a comma
        // *before* each non-first field. This keeps the output valid for
        // both flexible-CSV cases — records longer than headers (skip
        // extras) and records shorter than headers (no trailing comma).
        let mut first_field = true;
        for (idx, b) in record.iter().enumerate() {
            let Some(key) = header_vec.get(idx) else {
                break;
            };
            temp_val = bytes_to_cow_str(b).into_owned();
            if temp_val.is_empty() {
                temp_val.clone_from(&null_val);
            } else {
                // we round-trip the value to serde_json
                // to escape the string properly per JSON spec
                json_string_val = serde_json::Value::String(temp_val);
                temp_val = json_string_val.to_string();
            }
            if first_field {
                first_field = false;
            } else {
                write!(&mut json_wtr, ",")?;
            }
            write!(&mut json_wtr, r#"{key}:{temp_val}"#)?;
        }
        write!(json_wtr, "}}")?;
    }
    // Write the closing bracket for the JSON array
    writeln!(json_wtr, "]")?;

    Ok(json_wtr.flush()?)
}

/// write a single `csv::ByteRecord` to a JSON record writer
/// if `no_headers` is true, the column index (0-based) is used as the key
/// if `no_headers` is false, the header is used as the key
/// if `is_first` is true, a comma is not written before the record
/// if `is_first` is false, a comma is written before the record
/// `is_first` is passed as a mutable reference so that it can be updated
/// in this helper function efficiently
/// in this way, we can stream JSON records to a writer
pub fn write_json_record<W: std::io::Write>(
    json_wtr: &mut W,
    no_headers: bool,
    headers: &csv::ByteRecord,
    record: &csv::ByteRecord,
    is_first: &mut bool,
) -> std::io::Result<()> {
    // keys are escaped once here, per JSON spec. Note that the escaped form
    // returned by serde_json includes the enclosing double quotes, so the
    // format strings below must NOT add their own quotes around the key.
    let header_vec: Vec<String> = headers
        .iter()
        .enumerate()
        .map(|(col_idx, b)| {
            let key = if no_headers {
                col_idx.to_string()
            } else {
                bytes_to_cow_str(b).into_owned()
            };
            serde_json::Value::String(key).to_string()
        })
        .collect();

    let rec_len = header_vec.len().saturating_sub(1);
    let mut temp_val;
    let mut json_string_val: serde_json::Value;
    let null_val = "null".to_string();

    if *is_first {
        write!(json_wtr, "{{")?;
        *is_first = false;
    } else {
        write!(json_wtr, ",{{")?;
    }
    for (idx, b) in record.iter().enumerate() {
        temp_val = bytes_to_cow_str(b).into_owned();
        if temp_val.is_empty() {
            temp_val.clone_from(&null_val);
        } else {
            json_string_val = serde_json::Value::String(temp_val);
            temp_val = json_string_val.to_string();
        }
        if idx < rec_len {
            unsafe {
                write!(
                    json_wtr,
                    r#"{key}:{value},"#,
                    key = header_vec.get_unchecked(idx),
                    value = temp_val
                )?;
            }
        } else {
            unsafe {
                write!(
                    json_wtr,
                    r#"{key}:{value}"#,
                    key = header_vec.get_unchecked(idx),
                    value = temp_val
                )?;
            }
        }
    }
    Ok(write!(json_wtr, "}}")?)
}

pub fn csv_to_jsonl(
    input_csv: &str,
    csv_types: &phf::Map<&'static str, JsonTypes>,
    output_jsonl: &Path,
    delimiter: u8,
    perm_source: &Path,
) -> CliResult<()> {
    // The stats JSONL carries data values - a string column's min/max, its mode and antimode -
    // so it is a derived artifact: never wider than the data file it describes, which is
    // `perm_source` (the ORIGINAL input, not `input_csv`, which is usually a 0600 tempfile).
    let output = DerivedFile::create(perm_source, output_jsonl, "statsjsonl")?;
    let mut writer = BufWriter::new(output);
    csv_to_jsonl_writer(input_csv, csv_types, &mut writer, delimiter)?;
    writer
        .into_inner()
        .map_err(std::io::IntoInnerError::into_error)?
        .install()
}

// Build a `serde_json::Map` from a CSV record, coercing each cell to the JSON type
// declared in `csv_types` (defaulting to String for unknown keys). Empty cells are
// skipped entirely (no key is emitted), matching the JSONL sidecar behavior - with the
// sole exception of `field`, which is always emitted (see below).
fn csv_record_to_json_map(
    record: &csv::StringRecord,
    key_vec: &[String],
    csv_types: &phf::Map<&'static str, JsonTypes>,
    json_object: &mut serde_json::Map<String, serde_json::Value>,
) {
    json_object.clear();
    for (i, val) in record.iter().enumerate() {
        // Skip extra fields when a record has more columns than headers.
        let Some(key) = key_vec.get(i) else {
            break;
        };
        let data_type = csv_types.get(key.as_str()).unwrap_or(&JsonTypes::String);
        let value = if val.is_empty() {
            // `field` carries the column's identity. Dropping it makes the record
            // unattributable to a column and breaks StatsData deserialization when the
            // column name is itself empty (issue #4410). Keyed by name rather than by
            // position so a future reordering of the stats columns can't silently
            // reintroduce the bug. All other empty cells are still skipped.
            if key == "field" {
                json_object.insert(key.to_string(), serde_json::Value::String(String::new()));
            }
            continue;
        } else {
            match *data_type {
                JsonTypes::String => serde_json::Value::String(val.to_owned()),
                JsonTypes::Int => {
                    if let Ok(num) = val.parse::<u64>() {
                        serde_json::Value::Number(serde_json::Number::from(num))
                    } else {
                        serde_json::Value::String(val.to_owned())
                    }
                },
                JsonTypes::Float => {
                    if let Ok(num) = val.parse::<f64>() {
                        serde_json::Number::from_f64(num).map_or_else(
                            // NaN/infinity have no JSON representation. Keep the historical
                            // 0.0 here: unlike the rendering case below, this cell IS a
                            // number, and emitting "NaN" as a string would make every
                            // Option<f64> field reject the line and discard the whole cache.
                            // safety: 0.0 is a valid f64
                            || {
                                serde_json::Value::Number(
                                    serde_json::Number::from_f64(0.0).unwrap(),
                                )
                            },
                            serde_json::Value::Number,
                        )
                    } else {
                        // A Float-typed cell that does NOT parse is a type-dependent
                        // RENDERING, not a number: for a Date/DateTime column stats_to_records()
                        // emits RFC3339 for `mean`, `q1`, `q2_median`, `q3` and the four fences.
                        // Coercing those to 0.0 silently corrupted every stats-cache consumer
                        // (viz smart, schema, pivotp, tojsonl, describegpt) while the stats CSV
                        // beside it held the correct date. Emitting the cell verbatim is lossless
                        // for both cases - the row's `type` tells a consumer which one it is.
                        //
                        // Deliberately general rather than keyed to those 8 columns: the map is
                        // per-COLUMN but a rendering is per-ROW, so the map cannot express this.
                        // Verified across String/Boolean/NULL/Integer/Float/Date/DateTime columns
                        // that those 8 are the complete set of Float-typed cells that can be
                        // non-numeric, so no other field's Option<f64> can start rejecting lines.
                        serde_json::Value::String(val.to_owned())
                    }
                },
                JsonTypes::Bool => serde_json::Value::Bool(val.parse::<bool>().unwrap_or(false)),
            }
        };
        json_object.insert(key.to_string(), value);
    }
}

// Stream a CSV file to a writer as JSON Lines (NDJSON) - one JSON object per record.
// This is the shared core of `csv_to_jsonl` (which writes to a file) and the
// `qsv stats --jsonl` stdout output mode.
pub fn csv_to_jsonl_writer<W: Write>(
    input_csv: &str,
    csv_types: &phf::Map<&'static str, JsonTypes>,
    mut writer: W,
    delimiter: u8,
) -> CliResult<()> {
    let file = File::open(input_csv)?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(delimiter)
        .from_reader(file);

    let headers = rdr.headers()?;
    let key_vec: Vec<String> = headers
        .iter()
        .map(std::string::ToString::to_string)
        .collect();

    let mut json_object = serde_json::Map::with_capacity(key_vec.len());
    let mut record = csv::StringRecord::new();
    let mut json_line: String;

    while rdr.read_record(&mut record)? {
        csv_record_to_json_map(&record, &key_vec, csv_types, &mut json_object);

        // Use platform-appropriate JSON serialization
        json_line = cfg_select! {
            target_endian = "little" => simd_json::to_string(&json_object)?,
            _ => serde_json::to_string(&json_object)?,
        };
        writeln!(writer, "{json_line}")?;
    }

    Ok(writer.flush()?)
}

// Write a CSV file to a writer as a single pretty-printed JSON array of per-record
// objects. Used by the `qsv stats --pretty-json` stdout output mode.
pub fn csv_to_json_array_writer<W: Write>(
    input_csv: &str,
    csv_types: &phf::Map<&'static str, JsonTypes>,
    mut writer: W,
    delimiter: u8,
) -> CliResult<()> {
    let file = File::open(input_csv)?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(delimiter)
        .from_reader(file);

    let headers = rdr.headers()?;
    let key_vec: Vec<String> = headers
        .iter()
        .map(std::string::ToString::to_string)
        .collect();

    let mut json_object = serde_json::Map::with_capacity(key_vec.len());
    let mut record = csv::StringRecord::new();
    let mut json_array: Vec<serde_json::Value> = Vec::new();

    while rdr.read_record(&mut record)? {
        csv_record_to_json_map(&record, &key_vec, csv_types, &mut json_object);
        json_array.push(serde_json::Value::Object(json_object.clone()));
    }

    // Use platform-appropriate JSON serialization
    let json_output = cfg_select! {
        target_endian = "little" => simd_json::to_string_pretty(&json_array)?,
        _ => serde_json::to_string_pretty(&json_array)?,
    };
    writeln!(writer, "{json_output}")?;

    Ok(writer.flush()?)
}

/// get the optimal batch size
/// if CSV is not indexed and `ROW_COUNT` is not set, return `DEFAULT_BATCH_SIZE`
/// if `batch_size` is 0, return the number of rows in the CSV, effectively disabling batching
/// if `batch_size` is 1, force `batch_size` to be set to "`optimal_size`", even though
/// its not recommended (number of rows is too small for parallel processing)
/// if `batch_size` is equal to `DEFAULT_BATCH_SIZE`, return the `optimal_size`
/// failing everything above, return the requested `batch_size`
#[inline]
pub fn optimal_batch_size(rconfig: &Config, batch_size: usize, num_jobs: usize) -> usize {
    if batch_size > 1 && batch_size < DEFAULT_BATCH_SIZE {
        return DEFAULT_BATCH_SIZE;
    }

    let num_rows = match ROW_COUNT.get() {
        // ROW_COUNT is OnceLock<Option<u64>>; the inner Option is None when
        // count_rows_with_best_method failed (both polars and the CSV reader).
        // Fall through to the indexed path in that case rather than panicking.
        Some(Some(count)) => *count as usize,
        _ => match rconfig.indexed() {
            Ok(Some(idx)) => idx.count() as usize,
            _ => {
                return DEFAULT_BATCH_SIZE;
            },
        },
    };

    if batch_size == 0 {
        // disable batching, handle all rows in one batch
        num_rows
    } else if (num_rows > DEFAULT_BATCH_SIZE && (batch_size == DEFAULT_BATCH_SIZE))
        || batch_size == 1
    {
        // the optimal batch size is the number of rows divided by the number of jobs
        if num_rows.is_multiple_of(num_jobs) {
            // there is no remainder as num_rows is divisible by num_jobs
            num_rows / num_jobs
        } else {
            // there is a remainder, we add 1 to the batch size
            // this is to ensure that all rows are processed
            (num_rows / num_jobs) + 1
        }
    } else {
        batch_size
    }
}

// comment out for now as this is still WIP
// pub fn create_json_record(
//     no_headers: bool,
//     headers: &csv::ByteRecord,
//     record: &csv::ByteRecord,
//     is_first: &mut bool,
// ) -> CliResult<String> {
//     let header_vec: Vec<String> = headers
//         .iter()
//         .enumerate()
//         .map(|(col_idx, b)| {
//             if no_headers {
//                 col_idx.to_string()
//             } else {
//                 String::from_utf8_lossy(b).to_string()
//             }
//         })
//         .collect();

//     let mut json_record = String::new();

//     let rec_len = header_vec.len().saturating_sub(1);
//     let mut temp_val;
//     let mut json_string_val: serde_json::Value;
//     let null_val = "null".to_string();

//     if *is_first {
//         // write!(json_wtr, "{{")?;
//         json_record.push('{');
//         *is_first = false;
//     } else {
//         // write!(json_wtr, ",{{")?;
//         json_record.push_str(",{");
//     }
//     for (idx, b) in record.iter().enumerate() {
//         if let Ok(val) = simdutf8::basic::from_utf8(b) {
//             temp_val = val.to_owned();
//         } else {
//             temp_val = String::from_utf8_lossy(b).to_string();
//         }
//         if temp_val.is_empty() {
//             temp_val.clone_from(&null_val);
//         } else {
//             json_string_val = serde_json::Value::String(temp_val);
//             temp_val = json_string_val.to_string();
//         }
//         if idx < rec_len {
//             unsafe {
//                 // write!(
//                 //     json_wtr,
//                 //     r#""{key}":{value},"#,
//                 //     key = header_vec.get_unchecked(idx),
//                 //     value = temp_val
//                 // )?;
//                 json_record.push_str(&format!(
//                     r#""{key}":{value},"#,
//                     key = header_vec.get_unchecked(idx),
//                     value = temp_val
//                 ));
//             }
//         } else {
//             unsafe {
//                 // write!(
//                 //     json_wtr,
//                 //     r#""{key}":{value}"#,
//                 //     key = header_vec.get_unchecked(idx),
//                 //     value = temp_val
//                 // )?;
//                 json_record.push_str(&format!(
//                     r#""{key}":{value}"#,
//                     key = header_vec.get_unchecked(idx),
//                     value = temp_val
//                 ));
//             }
//         }
//     }
//     // Ok(write!(json_wtr, "}}")?)
//     json_record.push('}');
//     Ok(json_record)
// }
