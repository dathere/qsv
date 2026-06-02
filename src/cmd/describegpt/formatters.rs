//! Dictionary formatters for describegpt output in markdown, JSON, and TSV forms.
//!
//! These formatters render a slice of `DictionaryEntry` into three textual
//! representations. Kept together because they share the `extract_ordered_addl_cols`
//! helper and identical field-escaping logic.

use std::fmt::Write as _;

use indicatif::HumanCount;
use serde::Serialize;
use serde_json::{Value, json};

use super::dictionary::{DictionaryEntry, FreqDetail};

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
///
/// `relationships` carries the LLM-inferred inter-column relationships (see
/// `dictionary::parse_llm_relationships`). A top-level `relationships` array is
/// emitted only when it is non-empty, so dictionaries without relationships stay
/// byte-identical to the legacy output.
pub(super) fn format_dictionary_json(
    entries: &[DictionaryEntry],
    enum_threshold: usize,
    num_examples: u16,
    truncate_str: usize,
    infer_content_type: bool,
    relationships: &[Value],
) -> Value {
    let entries_json: Vec<Value> = entries
        .iter()
        .map(|e| {
            // Reformat date Min/Max to the inferred format only when
            // --infer-content-type is set; otherwise the legacy output stays
            // byte-identical (content_type is also only emitted in that path).
            let minmax = |v: &str| -> Value {
                if v.is_empty() {
                    Value::Null
                } else if infer_content_type {
                    Value::String(super::dictionary::format_date_value(&e.content_type, v).into_owned())
                } else {
                    Value::String(v.to_string())
                }
            };
            let mut entry_obj = json!({
                "name": e.name,
                "type": e.r#type,
                "label": e.label,
                "description": e.description,
                "min": minmax(&e.min),
                "max": minmax(&e.max),
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
                // Reformat date Examples to the inferred format only when
                // --infer-content-type is set, mirroring the `minmax` closure
                // so legacy output stays byte-identical when the flag is off.
                let examples = if infer_content_type {
                    super::dictionary::format_date_examples(&e.content_type, &e.examples)
                } else {
                    e.examples.clone()
                };
                obj.insert("examples".to_string(), json!(examples));
            }

            entry_obj
        })
        .collect();

    let mut doc = json!({
        "fields": entries_json,
        "enum_threshold": enum_threshold,
        "num_examples": num_examples,
        "truncate_str": truncate_str,
        "attribution": "{GENERATED_BY_SIGNATURE}",
    });
    if !relationships.is_empty()
        && let Some(obj) = doc.as_object_mut()
    {
        obj.insert("relationships".to_string(), json!(relationships));
    }
    doc
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
/// `strict_dates` toggles whether columns whose stats type is Date / DateTime
/// emit `format: "date"` / `"date-time"`. Off by default because qsv's
/// `--infer-dates` accepts many non-RFC-3339 strings (e.g. "June 27, 1968")
/// that would fail JSON Schema format validation. Mirrors
/// `src/cmd/schema.rs`'s `--strict-dates` flag (lines 462,469).
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
    strict_dates: bool,
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
            build_property_schema(entry, infer_content_type, strict_dates),
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
            "strict_dates": strict_dates,
        },
    })
}

/// Build the per-property JSON Schema for one `DictionaryEntry`.
///
/// Type mapping mirrors `src/cmd/schema.rs::infer_schema_from_stats`:
/// Integer→integer, Float→number, String→string, Boolean→boolean,
/// Date→string (+`format: "date"` only when `strict_dates`),
/// DateTime→string (+`format: "date-time"` only when `strict_dates`),
/// NULL→null. Nullable columns (null_count > 0) get `"null"` appended to
/// the `type` array.
fn build_property_schema(
    entry: &DictionaryEntry,
    infer_content_type: bool,
    strict_dates: bool,
) -> Value {
    let qsv_type = entry.r#type.as_str();
    let (json_type, _format_hint) = map_qsv_type(qsv_type);
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

    // `format` emission is opt-in via `--strict-dates` (mirrors schema.rs).
    // Without the flag, qsv's permissive --infer-dates (e.g. "June 27, 1968")
    // would otherwise produce a schema that rejects its own source CSV under
    // RFC 3339 format validation.
    if strict_dates {
        match qsv_type {
            "Date" => {
                prop.insert("format".to_string(), Value::String("date".to_string()));
            },
            "DateTime" => {
                prop.insert("format".to_string(), Value::String("date-time".to_string()));
            },
            _ => {},
        }
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
    //
    // Every emitted example must validate against the property's own `type`:
    // the `frequency` "Other"/"(NULL)" aggregation-bucket rows (rendered as
    // "Other…"/"(NULL)…") are not real data values, and `coerce_value` keeps
    // them as JSON strings when the column is numeric/boolean. The
    // `value_matches_json_type` filter drops those so they can't leak into a
    // numeric/boolean property's `examples` array and fail validation against
    // that property's subschema.
    if !entry.examples.is_empty() && entry.examples != "<ALL_UNIQUE>" {
        let example_vals: Vec<Value> = entry
            .examples
            .split('\n')
            .filter_map(|line| {
                let bare = strip_count_suffix(line);
                if bare.is_empty() {
                    return None;
                }
                // Reformat date values to the inferred format so schema examples
                // match the date-formatted Min/Max in x-qsv, but only when
                // --infer-content-type is set (mirrors x-qsv min/max gating).
                let bare = if infer_content_type {
                    super::dictionary::format_date_value(&entry.content_type, bare)
                } else {
                    std::borrow::Cow::Borrowed(bare)
                };
                let value = coerce_value(&bare, qsv_type);
                value_matches_json_type(&value, json_type).then_some(value)
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
        // Date/DateTime Min/Max aren't representable as JSON Schema
        // minimum/maximum, so for an inferred date format surface the range in
        // x-qsv, formatted to match the column's real presentation (and the
        // markdown/JSON/TOON dictionaries).
        if super::dictionary::content_type_date_format(&entry.content_type).is_some() {
            if !entry.min.is_empty() {
                x_qsv.insert(
                    "min".to_string(),
                    Value::String(
                        super::dictionary::format_date_value(&entry.content_type, &entry.min)
                            .into_owned(),
                    ),
                );
            }
            if !entry.max.is_empty() {
                x_qsv.insert(
                    "max".to_string(),
                    Value::String(
                        super::dictionary::format_date_value(&entry.content_type, &entry.max)
                            .into_owned(),
                    ),
                );
            }
        }
    }
    if !entry.examples.is_empty() {
        // Reformat date values to the inferred format (gated on
        // infer_content_type) so the weighted example_counts stay consistent
        // with the formatted `examples` array and x-qsv min/max above.
        let example_counts = if infer_content_type {
            super::dictionary::format_date_examples(&entry.content_type, &entry.examples)
        } else {
            entry.examples.clone()
        };
        x_qsv.insert("example_counts".to_string(), Value::String(example_counts));
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
    #[allow(clippy::match_same_arms)]
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

/// Whether a `coerce_value` result is a valid instance of the given JSON Schema
/// scalar `type` keyword.
///
/// `coerce_value` falls back to a JSON string when a stats-derived value can't
/// be parsed as the column's inferred numeric/boolean type — most notably the
/// `frequency` "Other"/"(NULL)" aggregation-bucket sentinels (rendered as
/// "Other…"/"(NULL)…"). This guard keeps such strings out of a numeric/boolean
/// property's `examples` array, where they would otherwise fail validation
/// against the property's own subschema. `"number"` accepts integer-valued
/// numbers; an unknown `json_type` passes through unfiltered.
fn value_matches_json_type(value: &Value, json_type: &str) -> bool {
    match json_type {
        "integer" => value.is_i64() || value.is_u64(),
        "number" => value.is_number(),
        "boolean" => value.is_boolean(),
        "string" => value.is_string(),
        "null" => value.is_null(),
        _ => true,
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

/// Map a qsv stats type to the semantic-md data-dictionary type vocabulary.
///
/// semantic-md uses a small human-friendly type set (`integer`, `number`,
/// `boolean`, `timestamp`, `text`) rather than JSON Schema's keywords, so this
/// differs from `map_qsv_type`: Date/DateTime collapse to `timestamp` and any
/// non-numeric/boolean type (including String and NULL) becomes `text`.
pub(super) fn semanticmd_type(qsv_type: &str) -> &'static str {
    match qsv_type {
        "Integer" => "integer",
        "Float" => "number",
        "Boolean" => "boolean",
        "Date" | "DateTime" => "timestamp",
        _ => "text",
    }
}

/// One `### Frequency for ...` table row (`Choice | Frequency | Percentage | Rank`).
#[derive(Debug, Serialize)]
pub(super) struct SemanticMdFreqRow {
    pub(super) choice:     String,
    pub(super) count:      String,
    pub(super) percentage: String,
    /// Integer rank, or empty for aggregation buckets (`Other…`/`(NULL)…`, rank 0).
    pub(super) rank:       String,
}

/// Per-column render data for the semantic-md template. Precomputed in Rust so the
/// Mini-Jinja template stays presentation-only and the derivation is unit-testable.
#[derive(Debug, Serialize)]
pub(super) struct SemanticMdEntry {
    pub(super) name:          String,
    pub(super) sem_type:      String,
    pub(super) required:      bool,
    pub(super) label:         String,
    pub(super) description:   String,
    pub(super) is_numeric:    bool,
    pub(super) min:           String,
    pub(super) max:           String,
    pub(super) cardinality:   u64,
    pub(super) null_count:    u64,
    pub(super) choices:       Vec<String>,
    pub(super) frequency:     Vec<SemanticMdFreqRow>,
    pub(super) has_frequency: bool,
}

/// Top-level render data for the semantic-md template.
#[derive(Debug, Serialize)]
pub(super) struct SemanticMdData {
    pub(super) entries:     Vec<SemanticMdEntry>,
    /// Heuristically-inferred single-column primary key, if unambiguous.
    pub(super) primary_key: Option<String>,
}

/// Build the semantic-md render data from dictionary entries.
///
/// Choices come from `enumeration` (populated only when `cardinality <=
/// enum_threshold`); the per-column Frequency table reuses the structured
/// `freq_details` (value, count, percentage, rank — empty for the `<ALL_UNIQUE>`
/// sentinel). The primary key is inferred only when exactly one column is fully
/// unique and non-null; otherwise it is omitted.
pub(super) fn build_semanticmd_data(entries: &[DictionaryEntry]) -> SemanticMdData {
    // Primary key candidates rely on the structural `is_unique_id` flag set by
    // `generate_code_based_dictionary` (cardinality == rowcount, no nulls). This is
    // the deterministic unique-id detector — NOT the overloaded `examples ==
    // "<ALL_UNIQUE>"` sentinel, which is also set for constant-value and
    // HIGH_CARDINALITY columns (any frequency row at 100%), and NOT a row-count
    // estimate. Either of those would falsely flag a non-unique column as a key.
    let mut pk_candidates = entries
        .iter()
        .filter(|e| e.is_unique_id)
        .map(|e| e.name.clone());
    let primary_key = match (pk_candidates.next(), pk_candidates.next()) {
        (Some(pk), None) => Some(pk),
        _ => None,
    };

    let entries = entries
        .iter()
        .map(|e| {
            let is_numeric = e.r#type == "Integer" || e.r#type == "Float";
            let choices: Vec<String> = if e.enumeration.is_empty() {
                Vec::new()
            } else {
                e.enumeration
                    .lines()
                    .map(str::trim)
                    .filter(|l| !l.is_empty())
                    .map(ToString::to_string)
                    .collect()
            };
            let frequency = build_freq_rows(&e.freq_details);
            SemanticMdEntry {
                name: e.name.clone(),
                sem_type: semanticmd_type(&e.r#type).to_string(),
                required: e.null_count == 0,
                label: e.label.clone(),
                description: e.description.clone(),
                is_numeric,
                min: e.min.clone(),
                max: e.max.clone(),
                cardinality: e.cardinality,
                null_count: e.null_count,
                choices,
                has_frequency: !frequency.is_empty(),
                frequency,
            }
        })
        .collect();

    SemanticMdData {
        entries,
        primary_key,
    }
}

/// Map structured `freq_details` into Frequency table rows, formatting the
/// percentage to two decimals and rendering the rank as an integer (blank for
/// aggregation buckets whose rank is 0).
fn build_freq_rows(details: &[FreqDetail]) -> Vec<SemanticMdFreqRow> {
    details
        .iter()
        .map(|d| SemanticMdFreqRow {
            choice:     d.value.clone(),
            count:      d.count.to_string(),
            percentage: format!("{:.2}%", d.percentage),
            rank:       if d.rank > 0.0 {
                (d.rank as u64).to_string()
            } else {
                String::new()
            },
        })
        .collect()
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
            freq_details: Vec::new(),
            is_unique_id: false,
        }
    }

    #[test]
    fn json_omits_content_type_when_flag_off() {
        let entries = vec![sample_entry("col", "email")];
        let json = format_dictionary_json(&entries, 10, 5, 25, false, &[]);
        assert!(
            json["fields"][0].get("content_type").is_none(),
            "content_type must be absent when infer_content_type is false"
        );
    }

    #[test]
    fn json_includes_content_type_when_flag_on() {
        let entries = vec![sample_entry("col", "email")];
        let json = format_dictionary_json(&entries, 10, 5, 25, true, &[]);
        assert_eq!(json["fields"][0]["content_type"], "email");
    }

    fn date_entry(
        name: &str,
        content_type: &str,
        ty: &str,
        min: &str,
        max: &str,
    ) -> DictionaryEntry {
        let mut e = sample_entry(name, content_type);
        e.r#type = ty.to_string();
        e.min = min.to_string();
        e.max = max.to_string();
        e
    }

    #[test]
    fn json_formats_date_min_max_to_inferred_format() {
        // date with an inferred `:<fmt>` suffix: RFC3339 Min/Max reformatted.
        let date = date_entry(
            "created",
            "date:%m/%d/%Y",
            "Date",
            "2013-01-24",
            "2013-12-31",
        );
        // datetime whose inferred format contains colons.
        let dt = date_entry(
            "ts",
            "datetime:%m/%d/%Y %I:%M:%S %p",
            "DateTime",
            "2013-01-24T13:30:00+00:00",
            "",
        );
        // bare date token (no suffix) is left unchanged.
        let bare = date_entry("plain", "date", "Date", "2013-01-24", "2013-12-31");
        // non-date content type is left unchanged.
        let other = date_entry("amount", "category", "Integer", "1", "1000");

        let json = format_dictionary_json(&[date, dt, bare, other], 10, 5, 25, true, &[]);
        assert_eq!(json["fields"][0]["min"], "01/24/2013");
        assert_eq!(json["fields"][0]["max"], "12/31/2013");
        assert_eq!(json["fields"][1]["min"], "01/24/2013 01:30:00 PM");
        assert_eq!(json["fields"][1]["max"], Value::Null); // empty stays null
        assert_eq!(json["fields"][2]["min"], "2013-01-24"); // bare token unchanged
        assert_eq!(json["fields"][3]["min"], "1"); // non-date unchanged
    }

    #[test]
    fn json_reformats_date_examples_to_inferred_format() {
        // Examples carry a time component from raw frequency values; with the
        // flag on they're reformatted to the inferred date-only format (counts
        // preserved), consistent with Min/Max. With the flag off, unchanged.
        let mut date = date_entry(
            "created",
            "date:%m/%d/%Y",
            "Date",
            "2013-01-24",
            "2013-12-31",
        );
        date.examples = "01/24/2013 12:00:00 AM [5]\n01/07/2014 12:00:00 AM [3]".to_string();

        let on = format_dictionary_json(std::slice::from_ref(&date), 10, 5, 25, true, &[]);
        assert_eq!(
            on["fields"][0]["examples"],
            "01/24/2013 [5]\n01/07/2014 [3]"
        );

        let off = format_dictionary_json(std::slice::from_ref(&date), 10, 5, 25, false, &[]);
        assert_eq!(
            off["fields"][0]["examples"],
            "01/24/2013 12:00:00 AM [5]\n01/07/2014 12:00:00 AM [3]"
        );
    }

    #[test]
    fn json_does_not_reformat_min_max_when_flag_off() {
        // With infer_content_type=false the legacy output must stay byte-identical,
        // even if an entry carries a date content_type: Min/Max are NOT reformatted
        // and content_type is omitted.
        let date = date_entry(
            "created",
            "date:%m/%d/%Y",
            "Date",
            "2013-01-24",
            "2013-12-31",
        );
        let json = format_dictionary_json(&[date], 10, 5, 25, false, &[]);
        assert_eq!(json["fields"][0]["min"], "2013-01-24");
        assert_eq!(json["fields"][0]["max"], "2013-12-31");
        assert!(json["fields"][0].get("content_type").is_none());
    }

    #[test]
    fn jsonschema_x_qsv_carries_formatted_date_min_max() {
        let mut date = date_entry(
            "created",
            "date:%m/%d/%Y",
            "Date",
            "2013-01-24",
            "2013-12-31",
        );
        date.examples = "01/24/2013 12:00:00 AM [5]\n01/07/2014 12:00:00 AM [3]".to_string();
        let bare = date_entry("plain", "date", "Date", "2013-01-24", "2013-12-31");
        let schema =
            format_dictionary_jsonschema(&[date, bare], "test.csv", 10, 5, 25, true, false, false);
        let xq = &schema["properties"]["created"]["x-qsv"];
        assert_eq!(xq["min"], "01/24/2013");
        assert_eq!(xq["max"], "12/31/2013");
        // weighted example_counts must also be date-formatted, consistent with
        // the `examples` array and x-qsv min/max.
        assert_eq!(xq["example_counts"], "01/24/2013 [5]\n01/07/2014 [3]");
        // bare date token: no inferred format, so no x-qsv min/max.
        let xq_bare = &schema["properties"]["plain"]["x-qsv"];
        assert!(
            xq_bare.get("min").is_none() && xq_bare.get("max").is_none(),
            "bare date token must not add x-qsv min/max: {xq_bare}"
        );
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

    #[test]
    fn jsonschema_reformats_date_examples_to_inferred_format() {
        // A Date property (json type "string") whose raw examples carry a time
        // component: with the flag on, the property's `examples` array is
        // reformatted to the inferred date-only format like x-qsv min/max.
        let mut date = date_entry(
            "created",
            "date:%m/%d/%Y",
            "Date",
            "2013-01-24",
            "2013-12-31",
        );
        date.examples = "01/24/2013 12:00:00 AM [5]\n01/07/2014 12:00:00 AM [3]".to_string();
        let schema = format_dictionary_jsonschema(
            std::slice::from_ref(&date),
            "test.csv",
            10,
            5,
            25,
            true,
            false,
            false,
        );
        let examples = schema["properties"]["created"]["examples"]
            .as_array()
            .expect("date property should emit examples");
        assert_eq!(examples, &[json!("01/24/2013"), json!("01/07/2014")]);
    }

    #[test]
    fn jsonschema_drops_non_numeric_examples_from_numeric_property() {
        // A numeric column whose `frequency` examples lead with the "Other" and
        // "(NULL)" aggregation-bucket sentinels. Those coerce to JSON strings
        // and must be filtered so the property's `examples` array validates
        // against its own (`integer`/`null`) `type`.
        let entry = DictionaryEntry {
            name:         "X Coordinate".to_string(),
            r#type:       "Integer".to_string(),
            label:        "X".to_string(),
            description:  "Desc".to_string(),
            content_type: String::new(),
            min:          "100".to_string(),
            max:          "999".to_string(),
            cardinality:  500,
            enumeration:  String::new(),
            null_count:   10,
            addl_cols:    Default::default(),
            examples:     "Other… [900]\n(NULL)… [10]\n123 [5]\n456 [3]".to_string(),
            freq_details: Vec::new(),
            is_unique_id: false,
        };
        let schema = format_dictionary_jsonschema(
            std::slice::from_ref(&entry),
            "test.csv",
            10,
            5,
            25,
            false,
            false,
            false,
        );
        let examples = schema["properties"]["X Coordinate"]["examples"]
            .as_array()
            .expect("numeric property should still emit its real examples");
        assert_eq!(
            examples.len(),
            2,
            "Other…/(NULL)… bucket sentinels must be dropped: {examples:?}"
        );
        assert!(
            examples.iter().all(serde_json::Value::is_number),
            "every example of a numeric property must be a number: {examples:?}"
        );
    }

    #[test]
    fn jsonschema_keeps_examples_for_string_property() {
        // String columns must not be over-filtered: any string is a valid
        // instance of a `string`-typed property.
        let mut entry = sample_entry("name", "");
        entry.examples = "Other… [9]\nAlice [3]\nBob [2]".to_string();
        let schema = format_dictionary_jsonschema(
            std::slice::from_ref(&entry),
            "test.csv",
            10,
            5,
            25,
            false,
            false,
            false,
        );
        let examples = schema["properties"]["name"]["examples"]
            .as_array()
            .expect("string property should emit examples");
        assert_eq!(
            examples.len(),
            3,
            "string examples must be preserved: {examples:?}"
        );
        assert!(examples.iter().all(serde_json::Value::is_string));
    }

    #[test]
    fn semanticmd_type_mapping() {
        assert_eq!(semanticmd_type("Integer"), "integer");
        assert_eq!(semanticmd_type("Float"), "number");
        assert_eq!(semanticmd_type("Boolean"), "boolean");
        assert_eq!(semanticmd_type("Date"), "timestamp");
        assert_eq!(semanticmd_type("DateTime"), "timestamp");
        assert_eq!(semanticmd_type("String"), "text");
        assert_eq!(semanticmd_type("NULL"), "text");
    }

    #[test]
    fn semanticmd_frequency_rows() {
        assert!(build_freq_rows(&[]).is_empty());

        let details = vec![
            // Aggregation bucket: rank 0 => blank rank cell.
            FreqDetail {
                value:      "Other…".to_string(),
                count:      900,
                percentage: 73.93,
                rank:       0.0,
            },
            FreqDetail {
                value:      "Closed".to_string(),
                count:      150,
                percentage: 12.34,
                rank:       1.0,
            },
        ];
        let rows = build_freq_rows(&details);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].choice, "Other…");
        assert_eq!(rows[0].count, "900");
        assert_eq!(rows[0].percentage, "73.93%");
        assert_eq!(rows[0].rank, ""); // bucket => blank rank
        assert_eq!(rows[1].choice, "Closed");
        assert_eq!(rows[1].percentage, "12.34%");
        assert_eq!(rows[1].rank, "1");
    }

    #[test]
    fn semanticmd_data_derivation_and_primary_key() {
        // Unique non-null Integer column => inferred primary key + numeric flag.
        let mut id = sample_entry("id", "");
        id.r#type = "Integer".to_string();
        id.min = "1".to_string();
        id.cardinality = 1000;
        id.null_count = 0;
        id.enumeration = String::new();
        id.examples = "<ALL_UNIQUE>".to_string();
        id.is_unique_id = true;

        // Low-cardinality nullable String column => choices + frequency, not required.
        let mut status = sample_entry("status", "");
        status.cardinality = 3;
        status.null_count = 50;
        status.enumeration = "Assigned\nClosed\nOpen".to_string();
        status.examples = "Closed [800]\nOpen [150]".to_string();
        status.freq_details = vec![
            FreqDetail {
                value:      "Closed".to_string(),
                count:      800,
                percentage: 84.21,
                rank:       1.0,
            },
            FreqDetail {
                value:      "Open".to_string(),
                count:      150,
                percentage: 15.79,
                rank:       2.0,
            },
        ];

        let data = build_semanticmd_data(&[id, status]);
        assert_eq!(data.primary_key.as_deref(), Some("id"));

        let id_e = &data.entries[0];
        assert_eq!(id_e.sem_type, "integer");
        assert!(id_e.is_numeric);
        assert!(id_e.required);
        assert!(id_e.choices.is_empty());
        assert!(!id_e.has_frequency);

        let status_e = &data.entries[1];
        assert_eq!(status_e.sem_type, "text");
        assert!(!status_e.is_numeric);
        assert!(!status_e.required);
        assert_eq!(status_e.choices, vec!["Assigned", "Closed", "Open"]);
        assert!(status_e.has_frequency);
        assert_eq!(status_e.frequency.len(), 2);
        assert_eq!(status_e.frequency[0].percentage, "84.21%");
        assert_eq!(status_e.frequency[0].rank, "1");
    }

    #[test]
    fn semanticmd_primary_key_ambiguous_omitted() {
        // Two structurally-unique non-null columns => ambiguous => no primary key.
        let mut a = sample_entry("a", "");
        a.r#type = "Integer".to_string();
        a.cardinality = 100;
        a.null_count = 0;
        a.is_unique_id = true;
        let mut b = sample_entry("b", "");
        b.r#type = "Integer".to_string();
        b.cardinality = 100;
        b.null_count = 0;
        b.is_unique_id = true;
        let data = build_semanticmd_data(&[a, b]);
        assert!(data.primary_key.is_none());
    }

    #[test]
    fn semanticmd_primary_key_ignores_high_cardinality_non_unique() {
        // A high-cardinality / constant column whose only frequency row is at 100%
        // gets the overloaded `<ALL_UNIQUE>` examples sentinel, but is NOT a unique id
        // (is_unique_id is false). It must not be inferred as a primary key. Built
        // through generate_code_based_dictionary so the structural detector is exercised.
        let stats = vec![
            // HIGH_CARDINALITY: single freq row at 100% but count (rowcount) != cardinality.
            crate::cmd::describegpt::dictionary::StatsRecord {
                field:       "hi".to_string(),
                r#type:      "String".to_string(),
                cardinality: 900, // distinct < rowcount(1000) => not a unique id
                nullcount:   0,
                min:         String::new(),
                max:         String::new(),
                addl_cols:   Default::default(),
            },
            // Constant column: cardinality 1, single value covers 100%.
            crate::cmd::describegpt::dictionary::StatsRecord {
                field:       "konst".to_string(),
                r#type:      "String".to_string(),
                cardinality: 1,
                nullcount:   0,
                min:         String::new(),
                max:         String::new(),
                addl_cols:   Default::default(),
            },
        ];
        let freqs = vec![
            crate::cmd::describegpt::dictionary::FrequencyRecord {
                field:      "hi".to_string(),
                value:      "<ALL_UNIQUE>".to_string(),
                count:      1000, // rowcount, != cardinality(900)
                percentage: 100.0,
                rank:       1.0,
            },
            crate::cmd::describegpt::dictionary::FrequencyRecord {
                field:      "konst".to_string(),
                value:      "K".to_string(),
                count:      1000,
                percentage: 100.0,
                rank:       1.0,
            },
        ];
        let entries = crate::cmd::describegpt::dictionary::generate_code_based_dictionary(
            &stats,
            &freqs,
            10,
            5,
            25,
            &[],
            false,
        );
        // Both carry the <ALL_UNIQUE> examples sentinel...
        assert_eq!(entries[0].examples, "<ALL_UNIQUE>");
        // ...but neither is a structural unique id.
        assert!(!entries[0].is_unique_id);
        assert!(!entries[1].is_unique_id);
        let data = build_semanticmd_data(&entries);
        assert!(
            data.primary_key.is_none(),
            "high-cardinality / constant columns must not be inferred as a primary key"
        );
    }
}
