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
