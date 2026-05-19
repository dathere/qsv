//! Dictionary formatters for describegpt output in markdown, JSON, and TSV forms.
//!
//! These formatters render a slice of `DictionaryEntry` into three textual
//! representations. Kept together because they share the `extract_ordered_addl_cols`
//! helper and identical field-escaping logic.

use std::fmt::Write as _;

use indicatif::HumanCount;
use serde_json::{Value, json};

use super::dictionary::DictionaryEntry;

/// Extract ordered additional column names from entries.
///
/// Returns columns in the order they appear in the first entry's `IndexMap`,
/// which preserves insertion order across all entries.
pub(super) fn extract_ordered_addl_cols(entries: &[DictionaryEntry]) -> Vec<String> {
    entries
        .first()
        .map(|e| e.addl_cols.keys().cloned().collect())
        .unwrap_or_default()
}

/// Format dictionary entries as JSON.
///
/// The three numeric parameters (`enum_threshold`, `num_examples`, `truncate_str`)
/// are echoed back into the JSON payload as metadata. They come from the top-level
/// CLI args but are threaded through as primitives to keep this module decoupled
/// from the full `Args` struct.
///
/// When `infer_content_type` is true, a `content_type` key is added to each field
/// object; when false, the output is unchanged from the legacy format.
pub(super) fn format_dictionary_json(
    entries: &[DictionaryEntry],
    enum_threshold: usize,
    num_examples: u16,
    truncate_str: usize,
    infer_content_type: bool,
) -> Value {
    let entries_json: Vec<Value> = entries
        .iter()
        .map(|e| {
            let mut entry_obj = json!({
                "name": e.name,
                "type": e.r#type,
                "label": e.label,
                "description": e.description,
                "min": if e.min.is_empty() { Value::Null } else { Value::String(e.min.clone()) },
                "max": if e.max.is_empty() { Value::Null } else { Value::String(e.max.clone()) },
                "cardinality": e.cardinality,
                "enumeration": if e.enumeration.is_empty() { Value::Null } else { Value::String(e.enumeration.clone()) },
                "null_count": e.null_count,
            });

            if let Some(obj) = entry_obj.as_object_mut() {
                if infer_content_type {
                    obj.insert("content_type".to_string(), json!(e.content_type));
                }
                for (key, value) in &e.addl_cols {
                    let json_value = if value.is_empty() {
                        Value::Null
                    } else if key == "percentiles" {
                        Value::String(value.replace('|', "\n"))
                    } else {
                        Value::String(value.clone())
                    };
                    obj.insert(key.clone(), json_value);
                }
                obj.insert("examples".to_string(), json!(e.examples));
            }

            entry_obj
        })
        .collect();

    json!({
        "fields": entries_json,
        "enum_threshold": enum_threshold,
        "num_examples": num_examples,
        "truncate_str": truncate_str,
        "attribution": "{GENERATED_BY_SIGNATURE}",
    })
}

/// Format dictionary entries as a JSON Schema (draft 2020-12) document.
///
/// Standard JSON Schema keywords (`type`, `title`, `description`, `minimum`,
/// `maximum`, `enum`, `const`, `format`, `examples`) come from the deterministic
/// stats data plus the LLM-inferred Label/Description. qsv- and LLM-specific data
/// that doesn't map to standard keywords (`content_type`, `cardinality`,
/// `null_count`, weighted example counts, additional stats columns) is preserved
/// via a single `x-qsv` annotation object per property. Per draft 2020-12,
/// unknown keywords are ignored by validators, so this flows through validation
/// cleanly.
///
/// `allow_extra_cols` toggles the schema-root `additionalProperties` between
/// `false` (strict, the default) and `true` (permissive).
///
/// The schema's top-level `x-qsv.generated_by` is left as the literal
/// `{GENERATED_BY_SIGNATURE}` placeholder; the caller substitutes the resolved
/// attribution after building, mirroring the pattern used by
/// `format_dictionary_json`.
#[allow(clippy::too_many_arguments)]
pub(super) fn format_dictionary_jsonschema(
    entries: &[DictionaryEntry],
    input_filename: &str,
    enum_threshold: usize,
    num_examples: u16,
    truncate_str: usize,
    infer_content_type: bool,
    allow_extra_cols: bool,
) -> Value {
    let mut properties = serde_json::Map::with_capacity(entries.len());
    // Every column is listed in `required`, matching `qsv schema`'s behavior.
    // `required` means the property KEY must be present in each row-object;
    // nullability is expressed independently via the per-property `type` array
    // (which gains `"null"` when `null_count > 0`). Callers who want a record
    // shape that allows omitting a column should re-emit `required` themselves.
    let mut required: Vec<Value> = Vec::with_capacity(entries.len());

    for entry in entries {
        required.push(json!(entry.name));
        properties.insert(
            entry.name.clone(),
            build_property_schema(entry, infer_content_type),
        );
    }

    json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "title": format!("Data Dictionary for {input_filename}"),
        "description": format!("JSON Schema (draft 2020-12) Data Dictionary inferred from {input_filename} by qsv describegpt --dictionary."),
        "type": "object",
        "properties": Value::Object(properties),
        "required": Value::Array(required),
        "additionalProperties": allow_extra_cols,
        "x-qsv": {
            "generated_by": "{GENERATED_BY_SIGNATURE}",
            "enum_threshold": enum_threshold,
            "num_examples": num_examples,
            "truncate_str": truncate_str,
            "infer_content_type": infer_content_type,
        },
    })
}

/// Build the per-property JSON Schema for one `DictionaryEntry`.
///
/// Type mapping mirrors `src/cmd/schema.rs::infer_schema_from_stats`:
/// Integer→integer, Float→number, String→string, Boolean→boolean,
/// Date→string+format:date, DateTime→string+format:date-time, NULL→null.
/// Nullable columns (null_count > 0) get `"null"` appended to the `type` array.
fn build_property_schema(entry: &DictionaryEntry, infer_content_type: bool) -> Value {
    let qsv_type = entry.r#type.as_str();
    let (json_type, format_hint) = map_qsv_type(qsv_type);
    let nullable = entry.null_count > 0 && json_type != "null";

    let mut prop = serde_json::Map::new();

    // For a fully-null column (qsv_type == "NULL"), the only consistent shape
    // is `type: "null"` with no enum/const/examples — those would contradict
    // the type. Short-circuit so a stray sentinel in `examples` can't produce
    // a self-contradictory schema.
    if json_type == "null" {
        prop.insert("type".to_string(), Value::String("null".to_string()));
        if !entry.label.is_empty() {
            prop.insert("title".to_string(), Value::String(entry.label.clone()));
        }
        let description = if entry.description.is_empty() {
            format!("{} column", entry.name)
        } else {
            entry.description.clone()
        };
        prop.insert("description".to_string(), Value::String(description));
        prop.insert(
            "x-qsv".to_string(),
            Value::Object(build_x_qsv(entry, infer_content_type)),
        );
        return Value::Object(prop);
    }

    let mut type_array: Vec<Value> = vec![Value::String(json_type.to_string())];
    if nullable {
        type_array.push(Value::String("null".to_string()));
    }
    prop.insert("type".to_string(), Value::Array(type_array));

    if !entry.label.is_empty() {
        prop.insert("title".to_string(), Value::String(entry.label.clone()));
    }
    let description = if entry.description.is_empty() {
        format!("{} column", entry.name)
    } else {
        entry.description.clone()
    };
    prop.insert("description".to_string(), Value::String(description));

    if let Some(fmt) = format_hint {
        prop.insert("format".to_string(), Value::String(fmt.to_string()));
    }

    // Numeric range constraints. Skip silently if the qsv stats min/max can't be
    // parsed as the inferred numeric type — better to omit the keyword than to
    // emit a malformed schema.
    match qsv_type {
        "Integer" => {
            if let Ok(min_i) = entry.min.parse::<i64>() {
                prop.insert("minimum".to_string(), json!(min_i));
            }
            if let Ok(max_i) = entry.max.parse::<i64>() {
                prop.insert("maximum".to_string(), json!(max_i));
            }
        },
        "Float" => {
            if let Ok(min_f) = entry.min.parse::<f64>()
                && let Some(n) = serde_json::Number::from_f64(min_f)
            {
                prop.insert("minimum".to_string(), Value::Number(n));
            }
            if let Ok(max_f) = entry.max.parse::<f64>()
                && let Some(n) = serde_json::Number::from_f64(max_f)
            {
                prop.insert("maximum".to_string(), Value::Number(n));
            }
        },
        _ => {},
    }

    // enum / const inference from the enumeration string. The enumeration field
    // is non-empty only when cardinality <= enum_threshold (set by the caller).
    //
    // `const` requires the instance to exactly equal the value, so it's only
    // safe when the column is NOT nullable. For a nullable single-value column
    // we emit `enum: [value, null]` instead, which permits both the constant
    // and a null in the same way the `type` array does.
    if !entry.enumeration.is_empty() {
        let values: Vec<Value> = entry
            .enumeration
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(|s| coerce_value(s, qsv_type))
            .collect();

        if entry.cardinality == 1
            && !nullable
            && let Some(single) = values.first()
        {
            prop.insert("const".to_string(), single.clone());
        } else if !values.is_empty() {
            let mut enum_vals = values;
            if nullable {
                enum_vals.push(Value::Null);
            }
            prop.insert("enum".to_string(), Value::Array(enum_vals));
        }
    }

    // examples: parse the "val [cnt]\nval [cnt]" form to bare typed values.
    // "<ALL_UNIQUE>" sentinel and the empty case both skip emitting `examples`.
    if !entry.examples.is_empty() && entry.examples != "<ALL_UNIQUE>" {
        let example_vals: Vec<Value> = entry
            .examples
            .split('\n')
            .filter_map(|line| {
                let bare = strip_count_suffix(line);
                if bare.is_empty() {
                    None
                } else {
                    Some(coerce_value(bare, qsv_type))
                }
            })
            .collect();
        if !example_vals.is_empty() {
            prop.insert("examples".to_string(), Value::Array(example_vals));
        }
    }

    prop.insert(
        "x-qsv".to_string(),
        Value::Object(build_x_qsv(entry, infer_content_type)),
    );

    Value::Object(prop)
}

/// Build the per-property `x-qsv` annotation object. Extracted so the NULL
/// short-circuit path can reuse it without duplicating the field map.
fn build_x_qsv(
    entry: &DictionaryEntry,
    infer_content_type: bool,
) -> serde_json::Map<String, Value> {
    let mut x_qsv = serde_json::Map::new();
    x_qsv.insert("qsv_type".to_string(), Value::String(entry.r#type.clone()));
    x_qsv.insert("cardinality".to_string(), json!(entry.cardinality));
    x_qsv.insert("null_count".to_string(), json!(entry.null_count));
    if infer_content_type && !entry.content_type.is_empty() {
        x_qsv.insert(
            "content_type".to_string(),
            Value::String(entry.content_type.clone()),
        );
    }
    if !entry.examples.is_empty() {
        x_qsv.insert(
            "example_counts".to_string(),
            Value::String(entry.examples.clone()),
        );
    }
    if !entry.addl_cols.is_empty() {
        let mut addl = serde_json::Map::with_capacity(entry.addl_cols.len());
        for (k, v) in &entry.addl_cols {
            let value = if v.is_empty() {
                Value::Null
            } else if k == "percentiles" {
                // Mirror format_dictionary_json: '|' is the percentiles delimiter.
                Value::String(v.replace('|', "\n"))
            } else {
                Value::String(v.clone())
            };
            addl.insert(k.clone(), value);
        }
        x_qsv.insert("addl".to_string(), Value::Object(addl));
    }
    x_qsv
}

/// Map a qsv stats type string to a JSON Schema `type` keyword and optional
/// `format` keyword. Unknown types default to `string` to keep the emitted
/// schema valid even if qsv's stats add a new type in the future.
///
/// Date/DateTime intentionally return `None` for the format hint — qsv's
/// `--infer-dates` is permissive (it classifies many real-world strings like
/// "June 27, 1968" as Date) but JSON Schema's `format: "date"` / `"date-time"`
/// require RFC 3339 full-date / date-time. Emitting the format keyword by
/// default would break the `qsv validate` roundtrip for permissively-inferred
/// date columns. This mirrors `src/cmd/schema.rs:462,469`, which only emits
/// these formats when `--strict-dates` is set.
fn map_qsv_type(qsv_type: &str) -> (&'static str, Option<&'static str>) {
    match qsv_type {
        "Integer" => ("integer", None),
        "Float" => ("number", None),
        "Boolean" => ("boolean", None),
        "Date" | "DateTime" => ("string", None),
        "NULL" => ("null", None),
        _ => ("string", None),
    }
}

/// Convert a stats-derived string value into a JSON Schema-typed value matching
/// the property's declared type. Falls back to a JSON string when the value
/// doesn't parse — preferable to dropping the value silently.
fn coerce_value(s: &str, qsv_type: &str) -> Value {
    match qsv_type {
        "Integer" => s
            .parse::<i64>()
            .map_or_else(|_| Value::String(s.to_string()), |i| json!(i)),
        "Float" => s
            .parse::<f64>()
            .ok()
            .and_then(serde_json::Number::from_f64)
            .map_or_else(|| Value::String(s.to_string()), Value::Number),
        "Boolean" => match s.to_ascii_lowercase().as_str() {
            "true" | "t" | "1" | "yes" | "y" => Value::Bool(true),
            "false" | "f" | "0" | "no" | "n" => Value::Bool(false),
            _ => Value::String(s.to_string()),
        },
        _ => Value::String(s.to_string()),
    }
}

/// Strip a trailing ` [count]` suffix from an example line.
/// Input is one line of `entry.examples` ("val1 [cnt1]" form). Returns the
/// bare value (trimmed). The count is preserved in `x-qsv.example_counts`.
fn strip_count_suffix(line: &str) -> &str {
    // Use rfind so values that themselves contain "[" aren't truncated mid-value.
    if let Some(idx) = line.rfind(" [")
        && line.ends_with(']')
    {
        line[..idx].trim()
    } else {
        line.trim()
    }
}

/// Format dictionary entries as TSV.
///
/// When `infer_content_type` is true, a `Content Type` column is inserted after
/// `Description`; when false, the header and rows are unchanged from the legacy
/// format.
pub(super) fn format_dictionary_tsv(
    entries: &[DictionaryEntry],
    infer_content_type: bool,
) -> String {
    let addl_col_names = extract_ordered_addl_cols(entries);

    let mut output = String::with_capacity(1024);
    output.push_str("Name\tType\tLabel\tDescription");
    if infer_content_type {
        output.push_str("\tContent Type");
    }
    output.push_str("\tMin\tMax\tCardinality\tEnumeration\tNull Count");
    for col_name in &addl_col_names {
        let _ = write!(output, "\t{col_name}");
    }
    output.push_str("\tExamples\n");

    for entry in entries {
        let name = entry.name.replace(['\t', '\n', '\r'], " ");
        let r#type = entry.r#type.replace(['\t', '\n', '\r'], " ");
        let label = entry.label.replace(['\t', '\n', '\r'], " ");
        let description = entry.description.replace(['\t', '\n', '\r'], " ");
        let min = entry.min.replace(['\t', '\n', '\r'], " ");
        let max = entry.max.replace(['\t', '\n', '\r'], " ");
        let enumeration = entry.enumeration.replace(['\t', '\n', '\r'], " ");
        let examples = entry.examples.replace(['\t', '\n', '\r'], " ");
        // Either a leading-tab-prefixed cell or empty, so the legacy layout stays
        // byte-identical when --infer-content-type is not set.
        let content_type = if infer_content_type {
            format!("\t{}", entry.content_type.replace(['\t', '\n', '\r'], " "))
        } else {
            String::new()
        };

        let _ = write!(
            output,
            "{}\t{}\t{}\t{}{}\t{}\t{}\t{}\t{}\t{}",
            name,
            r#type,
            label,
            description,
            content_type,
            min,
            max,
            HumanCount(entry.cardinality),
            enumeration,
            HumanCount(entry.null_count)
        );

        for col_name in &addl_col_names {
            let value = entry
                .addl_cols
                .get(col_name)
                .map(|v| {
                    if col_name == "percentiles" {
                        v.replace(['|', '\n'], "; ").replace(['\t', '\r'], " ")
                    } else {
                        v.replace(['\t', '\n', '\r'], " ")
                    }
                })
                .unwrap_or_default();
            let _ = write!(output, "\t{value}");
        }

        let _ = writeln!(output, "\t{examples}");
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_entry(name: &str, content_type: &str) -> DictionaryEntry {
        DictionaryEntry {
            name:         name.to_string(),
            r#type:       "String".to_string(),
            label:        "Label".to_string(),
            description:  "Desc".to_string(),
            content_type: content_type.to_string(),
            min:          String::new(),
            max:          String::new(),
            cardinality:  3,
            enumeration:  String::new(),
            null_count:   0,
            addl_cols:    Default::default(),
            examples:     "a [1]".to_string(),
        }
    }

    #[test]
    fn json_omits_content_type_when_flag_off() {
        let entries = vec![sample_entry("col", "email")];
        let json = format_dictionary_json(&entries, 10, 5, 25, false);
        assert!(
            json["fields"][0].get("content_type").is_none(),
            "content_type must be absent when infer_content_type is false"
        );
    }

    #[test]
    fn json_includes_content_type_when_flag_on() {
        let entries = vec![sample_entry("col", "email")];
        let json = format_dictionary_json(&entries, 10, 5, 25, true);
        assert_eq!(json["fields"][0]["content_type"], "email");
    }

    #[test]
    fn tsv_header_unchanged_when_flag_off() {
        let entries = vec![sample_entry("col", "email")];
        let tsv = format_dictionary_tsv(&entries, false);
        let header = tsv.lines().next().unwrap();
        assert_eq!(
            header,
            "Name\tType\tLabel\tDescription\tMin\tMax\tCardinality\tEnumeration\tNull \
             Count\tExamples"
        );
        assert!(
            !tsv.contains("Content Type"),
            "Content Type column leaked when infer_content_type is false"
        );
    }

    #[test]
    fn tsv_inserts_content_type_column_when_flag_on() {
        let entries = vec![sample_entry("col", "email")];
        let tsv = format_dictionary_tsv(&entries, true);
        let mut lines = tsv.lines();
        let header = lines.next().unwrap();
        assert_eq!(
            header,
            "Name\tType\tLabel\tDescription\tContent \
             Type\tMin\tMax\tCardinality\tEnumeration\tNull Count\tExamples"
        );
        let row = lines.next().unwrap();
        // ...Label <tab> Description <tab> Content Type <tab> Min...
        assert!(
            row.contains("Label\tDesc\temail\t"),
            "row missing content_type cell: {row}"
        );
    }
}
