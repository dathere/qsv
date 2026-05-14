//! Neuro-procedural data dictionary: parsing `stats` / `frequency` CSVs into
//! records, deterministically generating dictionary entries, and merging them
//! with the LLM-produced Label + Description pairs.
//!
//! Fields on these structs are `pub(super)` so sibling submodules (e.g.
//! `formatters`) can read them; all types themselves remain crate-private.

use foldhash::{HashMap, HashMapExt, HashSet};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use super::{CliError, CliResult, extract_json_from_output};

/// Curated, documented vocabulary of semantic Content Type tokens. Each token is
/// intended to map cleanly to a `fake-rs` faker for a future `synthesize` command.
///
/// Primitive types (`integer`, `decimal`, `boolean`, `date`, `datetime`) are
/// deliberately excluded — they are redundant with the dictionary's deterministic
/// `type` column. `synthesize` falls back to `type` + `min`/`max` for plain
/// numeric/temporal fields whose `content_type` is `unknown`.
pub(super) const CONTENT_TYPE_VOCAB: &[&str] = &[
    // person / identity
    "first_name",
    "last_name",
    "full_name",
    "username",
    "password",
    "email",
    "phone",
    // address / location
    "street_address",
    "building_number",
    "secondary_address",
    "city",
    "state",
    "state_abbr",
    "zip_code",
    "country",
    "country_code",
    "latitude",
    "longitude",
    "time_zone",
    // company / job
    "company_name",
    "job_title",
    // identifiers / technical
    "uuid",
    "credit_card",
    "currency_code",
    "isbn",
    "ip_address",
    "mac_address",
    "url",
    "user_agent",
    "file_name",
    "file_path",
    "mime_type",
    "color_hex",
    // temporal
    "time",
    // generic / fallback
    "category",
    "lorem_word",
    "lorem_sentence",
    "lorem_paragraph",
    "free_text",
    "unknown",
];

/// Render `CONTENT_TYPE_VOCAB` as a comma-separated string for prompt injection.
pub(super) fn content_type_vocab_list() -> String {
    CONTENT_TYPE_VOCAB.join(", ")
}

/// LLM-inferred fields for a single dictionary column, keyed by field name in the
/// map returned by `parse_llm_dictionary_response`. `content_type` stays empty
/// unless `--infer-content-type` is set.
#[derive(Debug, Clone, Default)]
pub(super) struct LlmDictField {
    pub(super) label:        String,
    pub(super) description:  String,
    pub(super) content_type: String,
}

/// Parsed row from the `stats` CSV.
#[derive(Debug, Clone)]
pub(super) struct StatsRecord {
    pub(super) field:       String,
    pub(super) r#type:      String,
    pub(super) cardinality: u64,
    pub(super) nullcount:   u64,
    pub(super) min:         String, // Empty string if not available
    pub(super) max:         String, // Empty string if not available
    pub(super) addl_cols:   IndexMap<String, String>, // Additional columns (preserves CSV order)
}

/// Parsed row from the `frequency` CSV.
#[derive(Debug, Clone)]
pub(super) struct FrequencyRecord {
    pub(super) field:      String,
    pub(super) value:      String,
    pub(super) count:      u64,
    pub(super) percentage: f64,
    pub(super) rank:       f64,
}

/// One row in the generated data dictionary. `label`, `description` and
/// `content_type` start empty and are filled by the LLM pass (`content_type`
/// only when `--infer-content-type` is set); all other fields are populated
/// deterministically from `StatsRecord` + `FrequencyRecord`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct DictionaryEntry {
    pub(super) name:         String,
    pub(super) r#type:       String,
    pub(super) label:        String,
    pub(super) description:  String,
    pub(super) content_type: String, // Curated semantic token; empty unless --infer-content-type
    pub(super) min:          String, // Empty string if not available
    pub(super) max:          String, // Empty string if not available
    pub(super) cardinality:  u64,
    pub(super) enumeration:  String, // Empty if not enumerable, otherwise one value per line
    pub(super) null_count:   u64,
    pub(super) addl_cols:    IndexMap<String, String>, // Preserves column order
    pub(super) examples:     String,                   /* Format: "val1 [cnt1]\nval2 [cnt2]…" or
                                                        * "<ALL_UNIQUE>" */
}

/// Parse the `stats` CSV into structured records, returning the records plus
/// the ordered list of additional (non-standard) column names in CSV order.
pub(super) fn parse_stats_csv(stats_csv: &str) -> CliResult<(Vec<StatsRecord>, Vec<String>)> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(stats_csv.as_bytes());

    let headers = rdr.headers()?.clone();

    let std_cols: HashSet<&str> = ["field", "type", "cardinality", "nullcount", "min", "max"]
        .iter()
        .copied()
        .collect();

    let field_idx = headers
        .iter()
        .position(|h| h == "field")
        .ok_or_else(|| CliError::Other("Stats CSV missing 'field' column".to_string()))?;

    let type_idx = headers
        .iter()
        .position(|h| h == "type")
        .ok_or_else(|| CliError::Other("Stats CSV missing 'type' column".to_string()))?;

    let cardinality_idx = headers.iter().position(|h| h == "cardinality");
    let nullcount_idx = headers
        .iter()
        .position(|h| h == "nullcount")
        .ok_or_else(|| CliError::Other("Stats CSV missing 'nullcount' column".to_string()))?;
    let min_idx = headers.iter().position(|h| h == "min");
    let max_idx = headers.iter().position(|h| h == "max");

    let addl_col_indices: Vec<(usize, String)> = headers
        .iter()
        .enumerate()
        .filter_map(|(idx, header)| {
            if std_cols.contains(header) {
                None
            } else {
                Some((idx, header.to_string()))
            }
        })
        .collect();

    let mut records = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let field = record
            .get(field_idx)
            .ok_or_else(|| CliError::Other("Stats CSV record missing field value".to_string()))?
            .to_string();

        let r#type = record
            .get(type_idx)
            .ok_or_else(|| CliError::Other("Stats CSV record missing type value".to_string()))?
            .to_string();

        let cardinality = cardinality_idx
            .and_then(|idx| record.get(idx))
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);

        let nullcount = record
            .get(nullcount_idx)
            .ok_or_else(|| CliError::Other("Stats CSV record missing nullcount value".to_string()))?
            .parse::<u64>()
            .map_err(|e| CliError::Other(format!("Failed to parse nullcount: {e}")))?;

        let min = min_idx
            .and_then(|idx| record.get(idx))
            .map(std::string::ToString::to_string)
            .unwrap_or_default();

        let max = max_idx
            .and_then(|idx| record.get(idx))
            .map(std::string::ToString::to_string)
            .unwrap_or_default();

        let mut addl_cols = IndexMap::new();
        for (idx, col_name) in &addl_col_indices {
            let value = record
                .get(*idx)
                .map(std::string::ToString::to_string)
                .unwrap_or_default();
            addl_cols.insert(col_name.clone(), value);
        }

        records.push(StatsRecord {
            field,
            r#type,
            cardinality,
            nullcount,
            min,
            max,
            addl_cols,
        });
    }

    let ordered_col_names: Vec<String> =
        addl_col_indices.into_iter().map(|(_, name)| name).collect();

    Ok((records, ordered_col_names))
}

/// Parse the `frequency` CSV into structured records.
pub(super) fn parse_frequency_csv(frequency_csv: &str) -> CliResult<Vec<FrequencyRecord>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(frequency_csv.as_bytes());

    let headers = rdr.headers()?.clone();

    let field_idx = headers
        .iter()
        .position(|h| h == "field")
        .ok_or_else(|| CliError::Other("Frequency CSV missing 'field' column".to_string()))?;

    let value_idx = headers
        .iter()
        .position(|h| h == "value")
        .ok_or_else(|| CliError::Other("Frequency CSV missing 'value' column".to_string()))?;

    let count_idx = headers
        .iter()
        .position(|h| h == "count")
        .ok_or_else(|| CliError::Other("Frequency CSV missing 'count' column".to_string()))?;

    let percentage_idx = headers
        .iter()
        .position(|h| h == "percentage")
        .ok_or_else(|| CliError::Other("Frequency CSV missing 'percentage' column".to_string()))?;

    let rank_idx = headers
        .iter()
        .position(|h| h == "rank")
        .ok_or_else(|| CliError::Other("Frequency CSV missing 'rank' column".to_string()))?;

    let mut records = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let field = record
            .get(field_idx)
            .ok_or_else(|| CliError::Other("Frequency CSV record missing field value".to_string()))?
            .to_string();

        let value = record
            .get(value_idx)
            .ok_or_else(|| CliError::Other("Frequency CSV record missing value".to_string()))?
            .to_string();

        let count = record
            .get(count_idx)
            .ok_or_else(|| CliError::Other("Frequency CSV record missing count".to_string()))
            .and_then(|s| {
                if s.is_empty() {
                    Ok(0)
                } else {
                    s.parse::<u64>().map_err(|e| {
                        CliError::Other(format!("Failed to parse count in frequency CSV: {e}"))
                    })
                }
            })?;

        let percentage = record
            .get(percentage_idx)
            .ok_or_else(|| CliError::Other("Frequency CSV record missing percentage".to_string()))
            .and_then(|s| {
                if s.is_empty() {
                    Ok(0.0)
                } else {
                    s.parse::<f64>().map_err(|e| {
                        CliError::Other(format!("Failed to parse percentage in frequency CSV: {e}"))
                    })
                }
            })?;

        let rank = record
            .get(rank_idx)
            .ok_or_else(|| CliError::Other("Frequency CSV record missing rank".to_string()))
            .and_then(|s| {
                if s.is_empty() {
                    Ok(0.0)
                } else {
                    s.parse::<f64>().map_err(|e| {
                        CliError::Other(format!("Failed to parse rank in frequency CSV: {e}"))
                    })
                }
            })?;

        records.push(FrequencyRecord {
            field,
            value,
            count,
            percentage,
            rank,
        });
    }

    Ok(records)
}

/// Generate dictionary entries deterministically from `stats` + `frequency`
/// data. Label and Description are left empty for the LLM pass to fill.
pub(super) fn generate_code_based_dictionary(
    stats_records: &[StatsRecord],
    frequency_records: &[FrequencyRecord],
    enum_threshold: usize,
    num_examples: u16,
    truncate_str: usize,
    addl_cols: &[String],
) -> Vec<DictionaryEntry> {
    let mut frequency_by_field: HashMap<String, Vec<&FrequencyRecord>> = HashMap::new();
    for freq_record in frequency_records {
        frequency_by_field
            .entry(freq_record.field.clone())
            .or_default()
            .push(freq_record);
    }

    let mut dictionary_entries = Vec::new();

    for stats_record in stats_records {
        let field_name = &stats_record.field;
        let field_frequencies = frequency_by_field
            .get(field_name)
            .cloned()
            .unwrap_or_default();

        let enumeration = if stats_record.cardinality <= enum_threshold as u64 {
            // Check for rank=0 "Other" bucket or <ALL_UNIQUE> sentinel
            let has_other = field_frequencies
                .iter()
                .any(|f| f.rank == 0.0 && !f.value.contains("<ALL_UNIQUE>"));
            if has_other {
                String::new()
            } else {
                let mut enum_values: Vec<String> = field_frequencies
                    .iter()
                    .filter(|f| !f.value.contains("<ALL_UNIQUE>"))
                    .map(|f| f.value.clone())
                    .collect();
                enum_values.sort();
                enum_values.join("\n")
            }
        } else {
            String::new()
        };

        let examples = if field_frequencies
            .iter()
            .any(|f| (f.percentage - 100.0).abs() < 0.0001)
        {
            "<ALL_UNIQUE>".to_string()
        } else {
            let mut sorted_freqs = field_frequencies.clone();
            sorted_freqs.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.value.cmp(&b.value)));

            let top_n: Vec<String> = sorted_freqs
                .iter()
                .take(num_examples as usize)
                .map(|f| {
                    // For frequency bucket entries (rank == 0.0), strip the redundant
                    // "(n)" count and append "…" to disambiguate from literal values with
                    // the same name (e.g. bucket "Other… [4,091]" vs literal "Other [2,006]")
                    let raw_value = if f.rank == 0.0 {
                        let base = if let Some(pos) = f.value.rfind(" (") {
                            &f.value[..pos]
                        } else {
                            &f.value
                        };
                        format!("{base}…")
                    } else {
                        f.value.clone()
                    };

                    let v = if truncate_str > 0 && raw_value.chars().count() > truncate_str {
                        let mut s = raw_value.chars().take(truncate_str).collect::<String>();
                        s.push('…');
                        s
                    } else {
                        raw_value
                    };
                    format!("{} [{}]", v, f.count)
                })
                .collect();

            top_n.join("\n")
        };

        let mut entry_addl_cols = IndexMap::new();
        for col_name in addl_cols {
            if let Some(value) = stats_record.addl_cols.get(col_name) {
                entry_addl_cols.insert(col_name.clone(), value.clone());
            }
        }

        dictionary_entries.push(DictionaryEntry {
            name: stats_record.field.clone(),
            r#type: stats_record.r#type.clone(),
            label: String::new(),        // Filled by LLM
            description: String::new(),  // Filled by LLM
            content_type: String::new(), // Filled by LLM when --infer-content-type is set
            min: stats_record.min.clone(),
            max: stats_record.max.clone(),
            cardinality: stats_record.cardinality,
            enumeration,
            null_count: stats_record.nullcount,
            addl_cols: entry_addl_cols,
            examples,
        });
    }

    dictionary_entries
}

/// Merge code-generated dictionary entries with the LLM-generated fields (Label,
/// Description and, when `--infer-content-type` is set, Content Type) keyed by
/// field name.
pub(super) fn combine_dictionary_entries(
    mut code_entries: Vec<DictionaryEntry>,
    llm_fields: &HashMap<String, LlmDictField>,
) -> Vec<DictionaryEntry> {
    for entry in &mut code_entries {
        if let Some(llm) = llm_fields.get(&entry.name) {
            entry.label = llm.label.clone();
            entry.description = llm.description.clone();
            entry.content_type = llm.content_type.clone();
        }
    }
    code_entries
}

/// Extract the `{field_name: {label, description[, content_type]}}` map from the
/// LLM's JSON response, restricted to the given `field_names`. When
/// `infer_content_type` is set, the `content_type` value is validated against
/// `CONTENT_TYPE_VOCAB` and any out-of-vocabulary token is coerced to `"unknown"`;
/// otherwise `content_type` is left empty.
pub(super) fn parse_llm_dictionary_response(
    llm_response: &str,
    field_names: &[String],
    infer_content_type: bool,
) -> CliResult<HashMap<String, LlmDictField>> {
    let json_value = extract_json_from_output(llm_response)?;

    let mut result = HashMap::new();

    if let Some(obj) = json_value.as_object() {
        for field_name in field_names {
            if let Some(field_obj) = obj.get(field_name)
                && let Some(field_map) = field_obj.as_object()
            {
                let label = field_map
                    .get("label")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();

                let description = field_map
                    .get("description")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();

                let content_type = if infer_content_type {
                    let raw = field_map
                        .get("content_type")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .trim();
                    // The prompt instructs the LLM to always emit exactly one token from the
                    // curated vocabulary (using "unknown" when none fits). Treat a missing,
                    // empty, or out-of-vocabulary value the same way — coerce to "unknown" — so
                    // every field has a content_type and the future `synthesize` command stays
                    // deterministic.
                    if CONTENT_TYPE_VOCAB.contains(&raw) {
                        raw.to_string()
                    } else {
                        "unknown".to_string()
                    }
                } else {
                    String::new()
                };

                result.insert(
                    field_name.clone(),
                    LlmDictField {
                        label,
                        description,
                        content_type,
                    },
                );
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn blank_entry(name: &str) -> DictionaryEntry {
        DictionaryEntry {
            name:         name.to_string(),
            r#type:       "String".to_string(),
            label:        String::new(),
            description:  String::new(),
            content_type: String::new(),
            min:          String::new(),
            max:          String::new(),
            cardinality:  0,
            enumeration:  String::new(),
            null_count:   0,
            addl_cols:    IndexMap::new(),
            examples:     String::new(),
        }
    }

    #[test]
    fn parse_llm_response_ignores_content_type_when_flag_off() {
        let json = r#"{
            "name": {"label": "Name", "description": "the name", "content_type": "first_name"}
        }"#;
        let fields = vec!["name".to_string()];
        let parsed = parse_llm_dictionary_response(json, &fields, false).unwrap();
        let f = parsed.get("name").unwrap();
        assert_eq!(f.label, "Name");
        assert_eq!(f.description, "the name");
        assert!(
            f.content_type.is_empty(),
            "content_type must stay empty when infer_content_type is false"
        );
    }

    #[test]
    fn parse_llm_response_extracts_valid_content_type() {
        let json = r#"{
            "email_addr": {"label": "Email", "description": "an email", "content_type": "email"}
        }"#;
        let fields = vec!["email_addr".to_string()];
        let parsed = parse_llm_dictionary_response(json, &fields, true).unwrap();
        assert_eq!(parsed.get("email_addr").unwrap().content_type, "email");
    }

    #[test]
    fn parse_llm_response_coerces_out_of_vocab_content_type_to_unknown() {
        let json = r#"{
            "mystery": {"label": "X", "description": "y", "content_type": "made_up_token"}
        }"#;
        let fields = vec!["mystery".to_string()];
        let parsed = parse_llm_dictionary_response(json, &fields, true).unwrap();
        assert_eq!(parsed.get("mystery").unwrap().content_type, "unknown");
    }

    #[test]
    fn parse_llm_response_missing_content_type_coerced_to_unknown() {
        // A missing content_type key is treated the same as an out-of-vocabulary value:
        // coerced to "unknown" so every field has a token when the flag is on.
        let json = r#"{ "f": {"label": "F", "description": "d"} }"#;
        let fields = vec!["f".to_string()];
        let parsed = parse_llm_dictionary_response(json, &fields, true).unwrap();
        assert_eq!(parsed.get("f").unwrap().content_type, "unknown");
    }

    #[test]
    fn combine_copies_content_type_onto_entry() {
        let code_entries = vec![blank_entry("col_a"), blank_entry("col_b")];
        let mut llm = HashMap::new();
        llm.insert(
            "col_a".to_string(),
            LlmDictField {
                label:        "A".to_string(),
                description:  "desc a".to_string(),
                content_type: "city".to_string(),
            },
        );
        let combined = combine_dictionary_entries(code_entries, &llm);
        assert_eq!(combined[0].label, "A");
        assert_eq!(combined[0].description, "desc a");
        assert_eq!(combined[0].content_type, "city");
        // col_b had no LLM entry, so its content_type stays empty.
        assert!(combined[1].content_type.is_empty());
    }
}
