//! Loading / inferring the field-name → `content_type` map that `synthesize`
//! uses to pick semantic fakers.
//!
//! `synthesize` only needs the `content_type` of each field from the data
//! dictionary — every other dictionary column (type, min/max, cardinality,
//! enumeration, null_count) is recomputed directly from the source CSV via
//! `stats` + `frequency`, which is richer and always in sync with the input.

use std::{collections::HashMap, fs};

use serde::Deserialize;

use crate::{CliError, CliResult, util};

/// The whole data-dictionary JSON file produced by
/// `describegpt --dictionary --infer-content-type --format JSON`.
/// Only `fields[].name` and `fields[].content_type` are consumed; every other
/// key is ignored.
#[derive(Debug, Deserialize)]
struct DictionaryFile {
    fields: Vec<SynthDictField>,
}

/// Extract the `fields` array from a describegpt JSON payload. `describegpt
/// --format JSON` wraps its output as `{"Dictionary": {"response": {"fields":
/// [...], ...}, ...}}`, so peel that wrapper when present. Also accept a raw
/// `{"fields": [...]}` payload so users can hand-author / pre-extract a
/// dictionary file without going through describegpt.
fn parse_dictionary_payload(raw: &str) -> Result<Vec<SynthDictField>, serde_json::Error> {
    let value: serde_json::Value = serde_json::from_str(raw)?;
    let inner = value
        .get("Dictionary")
        .and_then(|d| d.get("response"))
        .cloned()
        .unwrap_or(value);
    let dict: DictionaryFile = serde_json::from_value(inner)?;
    Ok(dict.fields)
}

/// One field entry in the dictionary. Deliberately lenient: `content_type` is
/// absent entirely when the dictionary was generated without
/// `--infer-content-type`, and `null` for fields the LLM left unclassified.
#[derive(Debug, Deserialize)]
struct SynthDictField {
    name:         String,
    #[serde(default)]
    content_type: Option<String>,
}

/// Build the field-name → `content_type` map from parsed dictionary fields.
/// Missing/empty content types are normalized to `"unknown"`.
fn fields_to_map(fields: Vec<SynthDictField>) -> HashMap<String, String> {
    fields
        .into_iter()
        .map(|f| {
            let content_type = f
                .content_type
                .filter(|s| !s.trim().is_empty())
                .unwrap_or_else(|| "unknown".to_string());
            (f.name, content_type)
        })
        .collect()
}

pub(crate) fn load_content_types(path: &str) -> CliResult<HashMap<String, String>> {
    let contents = fs::read_to_string(path)
        .map_err(|e| CliError::Other(format!("Failed to read dictionary file '{path}': {e}")))?;
    let fields = parse_dictionary_payload(&contents).map_err(|e| {
        CliError::Other(format!(
            "Failed to parse dictionary file '{path}' as JSON. `synthesize` expects a dictionary \
             produced with `describegpt --dictionary --infer-content-type --format JSON` (the \
             full `{{\"Dictionary\": {{\"response\": ...}}}}` wrapper is fine, as is a raw \
             `{{\"fields\": [...]}}` payload). Parser error: {e}"
        ))
    })?;
    Ok(fields_to_map(fields))
}

pub(crate) fn infer_content_types(input_path: &str) -> CliResult<HashMap<String, String>> {
    let (stdout, _stderr) = util::run_qsv_cmd(
        "describegpt",
        &["--dictionary", "--infer-content-type", "--format", "JSON"],
        input_path,
        "  Inferred Content Types via describegpt",
    )?;
    let fields = parse_dictionary_payload(&stdout).map_err(|e| {
        CliError::Other(format!(
            "Failed to parse describegpt dictionary output as JSON: {e}. Make sure an LLM is \
             configured — either set `QSV_LLM_APIKEY` (or `--api-key`) for a hosted provider, or \
             set `QSV_LLM_BASE_URL` (or `--base-url`) to a localhost address for a local LLM \
             (e.g. LM Studio, Ollama)."
        ))
    })?;
    Ok(fields_to_map(fields))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_dictionary_with_content_types() {
        let json = r#"{
            "fields": [
                {"name": "email", "type": "String", "content_type": "email", "min": null},
                {"name": "age", "type": "Integer", "content_type": "unknown"}
            ],
            "enum_threshold": 10
        }"#;
        let fields = parse_dictionary_payload(json).unwrap();
        let map = fields_to_map(fields);
        assert_eq!(map.get("email").unwrap(), "email");
        assert_eq!(map.get("age").unwrap(), "unknown");
    }

    #[test]
    fn missing_or_null_content_type_normalizes_to_unknown() {
        let json = r#"{
            "fields": [
                {"name": "a", "type": "String"},
                {"name": "b", "type": "String", "content_type": null},
                {"name": "c", "type": "String", "content_type": "  "}
            ]
        }"#;
        let fields = parse_dictionary_payload(json).unwrap();
        let map = fields_to_map(fields);
        assert_eq!(map.get("a").unwrap(), "unknown");
        assert_eq!(map.get("b").unwrap(), "unknown");
        assert_eq!(map.get("c").unwrap(), "unknown");
    }

    #[test]
    fn parses_describegpt_wrapped_dictionary_output() {
        // Mirrors the actual `describegpt --dictionary --infer-content-type
        // --format JSON` stdout shape: a `Dictionary` key whose `response`
        // holds the dictionary, alongside `reasoning` and `token_usage`.
        let json = r#"{
            "Dictionary": {
                "response": {
                    "fields": [
                        {"name": "email", "type": "String", "content_type": "email"},
                        {"name": "zip", "type": "String", "content_type": "postal_code"}
                    ],
                    "enum_threshold": 10,
                    "attribution": "Generated by describegpt"
                },
                "reasoning": "...",
                "token_usage": {"prompt_tokens": 1, "completion_tokens": 2, "total_tokens": 3}
            }
        }"#;
        let fields = parse_dictionary_payload(json).unwrap();
        let map = fields_to_map(fields);
        assert_eq!(map.get("email").unwrap(), "email");
        assert_eq!(map.get("zip").unwrap(), "postal_code");
    }
}
