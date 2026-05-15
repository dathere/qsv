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

/// Load a data-dictionary JSON file and return the field-name → `content_type` map.
pub(crate) fn load_content_types(path: &str) -> CliResult<HashMap<String, String>> {
    let contents = fs::read_to_string(path)
        .map_err(|e| CliError::Other(format!("Failed to read dictionary file '{path}': {e}")))?;
    let dict: DictionaryFile = serde_json::from_str(&contents).map_err(|e| {
        CliError::Other(format!(
            "Failed to parse dictionary file '{path}' as JSON. `synthesize` expects a dictionary \
             produced with `describegpt --dictionary --infer-content-type --format JSON`. Parser \
             error: {e}"
        ))
    })?;
    Ok(fields_to_map(dict.fields))
}

/// Infer the dictionary on the fly by invoking `describegpt --dictionary
/// --infer-content-type --format JSON` on `input_path`. Requires an LLM API key
/// in the environment (`QSV_LLM_APIKEY`). Returns the field-name → `content_type` map.
pub(crate) fn infer_content_types(input_path: &str) -> CliResult<HashMap<String, String>> {
    let (stdout, _stderr) = util::run_qsv_cmd(
        "describegpt",
        &["--dictionary", "--infer-content-type", "--format", "JSON"],
        input_path,
        "  Inferred Content Types via describegpt",
    )?;
    let dict: DictionaryFile = serde_json::from_str(&stdout).map_err(|e| {
        CliError::Other(format!(
            "Failed to parse describegpt dictionary output as JSON: {e}. Make sure an LLM API key \
             is configured (QSV_LLM_APIKEY)."
        ))
    })?;
    Ok(fields_to_map(dict.fields))
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
        let dict: DictionaryFile = serde_json::from_str(json).unwrap();
        let map = fields_to_map(dict.fields);
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
        let dict: DictionaryFile = serde_json::from_str(json).unwrap();
        let map = fields_to_map(dict.fields);
        assert_eq!(map.get("a").unwrap(), "unknown");
        assert_eq!(map.get("b").unwrap(), "unknown");
        assert_eq!(map.get("c").unwrap(), "unknown");
    }
}
