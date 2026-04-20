//! Dictionary formatters for describegpt output in markdown, JSON, and TSV forms.
//!
//! These formatters render a slice of `DictionaryEntry` into three textual
//! representations. Kept together because they share the `extract_ordered_addl_cols`
//! helper and identical field-escaping logic.

use std::fmt::Write as _;

use indicatif::HumanCount;
use serde_json::{Value, json};

use super::DictionaryEntry;

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

/// Format dictionary entries as a markdown table.
pub(super) fn format_dictionary_markdown(entries: &[DictionaryEntry]) -> String {
    let addl_col_names = extract_ordered_addl_cols(entries);

    let mut output = String::with_capacity(1024);

    output.push_str(
        "| Name | Type | Label | Description | Min | Max | Cardinality | Enumeration | Null Count",
    );
    for col_name in &addl_col_names {
        let _ = write!(output, " | {col_name}");
    }
    output.push_str(" | Examples |\n");

    output.push_str(
        "|------|------|-------|-------------|-----|-----|-------------|-------------|------------",
    );
    for _ in &addl_col_names {
        output.push_str("|----------");
    }
    output.push_str("|----------|\n");

    for entry in entries {
        let name = entry.name.replace('|', "\\|");
        let r#type = entry.r#type.replace('|', "\\|");
        let label = entry.label.replace('|', "\\|");
        let description = entry.description.replace('|', "\\|").replace('\n', "<br>");
        let min = entry.min.replace('|', "\\|");
        let max = entry.max.replace('|', "\\|");
        let enumeration = entry.enumeration.replace('|', "\\|");
        let examples = entry.examples.replace('|', "\\|");

        let enumeration_display = if enumeration.is_empty() {
            String::new()
        } else {
            enumeration.replace('\n', "<br>")
        };

        // Format examples: replace newlines with <br> and humanize counts in "value [count]" lines
        let examples_display = if examples == "<ALL_UNIQUE>" {
            examples.clone()
        } else {
            examples
                .lines()
                .map(|line| {
                    if let Some(pos) = line.rfind(" [") {
                        let (value_part, count_part) = line.split_at(pos + 2);
                        if let Some(end_pos) = count_part.find(']')
                            && let Ok(count) = count_part[..end_pos].parse::<u64>()
                        {
                            return format!(
                                "{} [{}]",
                                value_part.trim_end_matches(" ["),
                                HumanCount(count)
                            );
                        }
                    }
                    line.to_string()
                })
                .collect::<Vec<String>>()
                .join("<br>")
        };

        let _ = write!(
            output,
            "| **{}** | {} | {} | {} | {} | {} | {} | {} | {}",
            name,
            r#type,
            label,
            description,
            min,
            max,
            HumanCount(entry.cardinality),
            enumeration_display,
            HumanCount(entry.null_count)
        );

        for col_name in &addl_col_names {
            let value = entry
                .addl_cols
                .get(col_name)
                .map(|v| {
                    if col_name == "percentiles" {
                        v.replace(['|', '\n'], "<br>")
                    } else {
                        v.replace('|', "\\|").replace('\n', "<br>")
                    }
                })
                .unwrap_or_default();
            let _ = write!(output, " | {value}");
        }

        let _ = writeln!(output, " | {examples_display} |");
    }

    output.push_str("\n*Attribution: {GENERATED_BY_SIGNATURE}*\n");

    output
}

/// Format dictionary entries as JSON.
///
/// The three numeric parameters (`enum_threshold`, `num_examples`, `truncate_str`)
/// are echoed back into the JSON payload as metadata. They come from the top-level
/// CLI args but are threaded through as primitives to keep this module decoupled
/// from the full `Args` struct.
pub(super) fn format_dictionary_json(
    entries: &[DictionaryEntry],
    enum_threshold: usize,
    num_examples: u16,
    truncate_str: usize,
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
pub(super) fn format_dictionary_tsv(entries: &[DictionaryEntry]) -> String {
    let addl_col_names = extract_ordered_addl_cols(entries);

    let mut output = String::with_capacity(1024);
    output
        .push_str("Name\tType\tLabel\tDescription\tMin\tMax\tCardinality\tEnumeration\tNull Count");
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

        let _ = write!(
            output,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            name,
            r#type,
            label,
            description,
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
