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
/// numeric/date/datetime fields whose `content_type` is `unknown`.
///
/// `time` (time-of-day, e.g. `HH:MM:SS`) and `duration` (elapsed time) ARE
/// included because qsv's stats reports them as `String`, so the deterministic
/// `type` column doesn't cover them; without these tokens `synthesize` would
/// fall through to lorem text for fields that are clearly temporal. They map
/// to `fake::faker::time::en::Time` and `Duration` respectively.
///
/// `duration` accepts an optional `:N` suffix carrying an LLM-inferred
/// upper bound in seconds (e.g. `"duration:3600"` for an hour cap). The
/// suffix is normalized by `normalize_duration_token` and consumed by
/// `synthesize::faker_map::parse_duration_cap`. Bare `"duration"` falls
/// back to a 24-hour default at generation time.
///
/// `unique_id` marks fields where every row has a distinct non-null value
/// (i.e. stats `cardinality == rowcount`, no nulls — primary keys, surrogate
/// keys, sequence numbers). It is set DETERMINISTICALLY by
/// `generate_code_based_dictionary` via a structural check on the field's
/// frequency table (single row whose `count == cardinality`), independent of
/// the literal sentinel text, and overrides any token the LLM produced for
/// that field. The vocabulary entry is still exposed to the LLM so its
/// classification stays consistent, but the deterministic check is
/// authoritative — and LLM-supplied `unique_id` is rejected by
/// `parse_llm_dictionary_response` and `combine_dictionary_entries` to keep
/// the contract one-way.
pub(crate) const CONTENT_TYPE_VOCAB: &[&str] = &[
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
    "unique_id",
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
    // temporal (time-of-day and durations; plain date/datetime fields stay
    // "unknown" so synthesize's build_date() can use real min/max bounds)
    "time",
    "duration",
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

/// Normalize the `duration` content type, optionally with an LLM-inferred
/// per-field upper-bound suffix.
///
/// Accepts:
///   * `"duration"` → returns `Some("duration")` (synthesize falls back to its default 24-hour cap)
///   * `"duration:N"` where N is a positive integer → returns `Some("duration:N")` (whitespace
///     around N is tolerated)
///   * `"duration:<malformed>"` (non-numeric / zero) → returns `Some("duration")` so a bad suffix
///     degrades gracefully to the unbounded form rather than dropping the classification entirely
///
/// Returns `None` for anything that isn't a duration token, so the caller can
/// fall back to the regular `CONTENT_TYPE_VOCAB` membership check.
///
/// Caller is responsible for lowercasing / outer-trimming the input first.
pub(super) fn normalize_duration_token(raw: &str) -> Option<String> {
    if raw == "duration" {
        return Some("duration".to_string());
    }
    let suffix = raw.strip_prefix("duration:")?;
    match suffix.trim().parse::<u64>() {
        Ok(n) if n > 0 => Some(format!("duration:{n}")),
        _ => Some("duration".to_string()),
    }
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

pub(crate) struct StatsRecord {
    pub(crate) field:       String,
    pub(crate) r#type:      String,
    pub(crate) cardinality: u64,
    pub(crate) nullcount:   u64,
    pub(crate) min:         String, // Empty string if not available
    pub(crate) max:         String, // Empty string if not available
    pub(crate) addl_cols:   IndexMap<String, String>, // Additional columns (preserves CSV order)
}

pub(crate) struct FrequencyRecord {
    pub(crate) field:      String,
    pub(crate) value:      String,
    pub(crate) count:      u64,
    pub(crate) percentage: f64,
    pub(crate) rank:       f64,
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
pub(crate) fn parse_stats_csv(stats_csv: &str) -> CliResult<(Vec<StatsRecord>, Vec<String>)> {
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
pub(crate) fn parse_frequency_csv(frequency_csv: &str) -> CliResult<Vec<FrequencyRecord>> {
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
///
/// When `infer_content_type` is set, `content_type` is also deterministically
/// pre-set to `"unique_id"` for fields where `cardinality == rowcount` with no
/// nulls. The detection is structural (single frequency row whose
/// `count == cardinality`), not text-matching, so it works regardless of the
/// `frequency --all-unique-text` setting and won't be confused by fields whose
/// values literally contain the string `<ALL_UNIQUE>`. All other fields'
/// `content_type` stays empty and is filled by the LLM pass.
pub(super) fn generate_code_based_dictionary(
    stats_records: &[StatsRecord],
    frequency_records: &[FrequencyRecord],
    enum_threshold: usize,
    num_examples: u16,
    truncate_str: usize,
    addl_cols: &[String],
    infer_content_type: bool,
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

        // Deterministically classify fields where every row carries a distinct
        // non-null value as `unique_id`. qsv's frequency command emits an
        // ALL_UNIQUE sentinel row exactly when `cardinality == rowcount` (a
        // single row with `count == rowcount` and `percentage == 100.0`). We
        // detect that structurally — `len() == 1` + `count == cardinality` —
        // rather than matching the literal `<ALL_UNIQUE>` text so that:
        //   - a real field whose values literally contain the string "<ALL_UNIQUE>" isn't
        //     mislabeled (constants have `cardinality == 1`; mixed fields produce more than one
        //     frequency row);
        //   - a custom `frequency --all-unique-text` sentinel is still detected correctly (the text
        //     doesn't matter, only the structural cardinality==count invariant does);
        //   - HIGH_CARDINALITY sentinel rows (also single-row, percentage 100.0, count == rowcount)
        //     are excluded because for them `cardinality < rowcount == count`.
        // Also requires `cardinality > 1` and `nullcount == 0` to enforce
        // the semantic contract (every row has a distinct non-null value).
        // Pre-set value takes precedence over whatever the LLM returns (see
        // `combine_dictionary_entries`). Only populate when `--infer-content-type`
        // is on; otherwise the `content_type` column is suppressed entirely.
        let is_all_unique = stats_record.cardinality > 1
            && stats_record.nullcount == 0
            && field_frequencies.len() == 1
            && field_frequencies[0].count == stats_record.cardinality;
        let content_type = if infer_content_type && is_all_unique {
            "unique_id".to_string()
        } else {
            String::new()
        };

        dictionary_entries.push(DictionaryEntry {
            name: stats_record.field.clone(),
            r#type: stats_record.r#type.clone(),
            label: String::new(),       // Filled by LLM
            description: String::new(), // Filled by LLM
            content_type,               /* Pre-set to "unique_id" for ALL_UNIQUE fields;
                                         * otherwise filled by LLM */
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
///
/// Code-derived `content_type` always wins over the LLM-supplied value. Today
/// the only code-derived value is the `"unique_id"` token that
/// `generate_code_based_dictionary` stamps on fields with the `<ALL_UNIQUE>`
/// frequency sentinel. The LLM is also blocked from supplying `"unique_id"`
/// itself (both here and in `parse_llm_dictionary_response`) so non-ALL_UNIQUE
/// fields cannot be misclassified.
///
/// When `infer_content_type` is set, this is the single point that guarantees
/// every entry has a non-empty `content_type`: any field the LLM classified
/// with an invalid token, omitted the `content_type` key for, or left out of
/// its response entirely falls back to `"unknown"`.
pub(super) fn combine_dictionary_entries(
    mut code_entries: Vec<DictionaryEntry>,
    llm_fields: &HashMap<String, LlmDictField>,
    infer_content_type: bool,
) -> Vec<DictionaryEntry> {
    for entry in &mut code_entries {
        if let Some(llm) = llm_fields.get(&entry.name) {
            entry.label = llm.label.clone();
            entry.description = llm.description.clone();
            // Preserve any deterministically pre-set content_type (e.g. the
            // `"unique_id"` classification stamped by
            // `generate_code_based_dictionary` for fields with the
            // `<ALL_UNIQUE>` sentinel). Code-derived facts always win over
            // the LLM's guess.
            //
            // Defense in depth: also refuse to copy `"unique_id"` from the
            // LLM. `parse_llm_dictionary_response` already strips it from
            // LLM output, but rejecting it here too means any future caller
            // that bypasses the parser still can't smuggle in a fabricated
            // `unique_id` for a non-ALL_UNIQUE field.
            if entry.content_type.is_empty() && llm.content_type != "unique_id" {
                entry.content_type = llm.content_type.clone();
            }
        }
        if infer_content_type && entry.content_type.is_empty() {
            entry.content_type = "unknown".to_string();
        }
    }
    code_entries
}

/// Extract the `{field_name: {label, description[, content_type]}}` map from the
/// LLM's JSON response, restricted to the given `field_names`. When
/// `infer_content_type` is set, `content_type` is lowercased and validated against
/// `CONTENT_TYPE_VOCAB`; a missing, empty, or out-of-vocabulary value is left empty
/// here — `combine_dictionary_entries` is the single point that coerces any
/// still-empty `content_type` to `"unknown"`. When the flag is unset, `content_type`
/// is always empty.
///
/// `unique_id` is in the vocab but is REJECTED from LLM input here: it is set
/// deterministically based on the `<ALL_UNIQUE>` frequency sentinel and the LLM
/// has no way to verify that condition, so accepting it would let non-ALL_UNIQUE
/// fields be misclassified.
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
                    // Normalize to lowercase before the vocab lookup — `CONTENT_TYPE_VOCAB` is
                    // all lowercase, and LLMs don't reliably echo casing exactly (e.g. "Email",
                    // "First_Name") even when given an explicit token list. A missing, empty, or
                    // out-of-vocabulary value is left empty here; `combine_dictionary_entries`
                    // coerces any still-empty content_type to "unknown" when the flag is set.
                    //
                    // `duration` is special: the LLM may append an upper-bound suffix (e.g.
                    // "duration:3600") that isn't in `CONTENT_TYPE_VOCAB` literally, so route
                    // it through `normalize_duration_token` first.
                    //
                    // `unique_id` is REJECTED here even though it is in the vocab: it is set
                    // deterministically by `generate_code_based_dictionary` based on the
                    // `<ALL_UNIQUE>` frequency sentinel (`cardinality == rowcount`), and the LLM
                    // has no way to verify that condition. Accepting it from LLM output would let
                    // non-ALL_UNIQUE fields be misclassified as `unique_id`, breaking the
                    // deterministic-only contract documented on `CONTENT_TYPE_VOCAB`.
                    let raw = field_map
                        .get("content_type")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .trim()
                        .to_ascii_lowercase();
                    if let Some(normalized) = normalize_duration_token(&raw) {
                        normalized
                    } else if raw == "unique_id" {
                        String::new()
                    } else if CONTENT_TYPE_VOCAB.contains(&raw.as_str()) {
                        raw
                    } else {
                        String::new()
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
    fn parse_llm_response_normalizes_content_type_casing() {
        // LLMs don't reliably echo casing; a valid-but-differently-cased token must be
        // accepted and stored in its normalized lowercase form, not coerced to "unknown".
        let json = r#"{
            "a": {"label": "A", "description": "d", "content_type": "Email"},
            "b": {"label": "B", "description": "d", "content_type": "FIRST_NAME"},
            "c": {"label": "C", "description": "d", "content_type": "  Mac_Address  "}
        }"#;
        let fields = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let parsed = parse_llm_dictionary_response(json, &fields, true).unwrap();
        assert_eq!(parsed.get("a").unwrap().content_type, "email");
        assert_eq!(parsed.get("b").unwrap().content_type, "first_name");
        assert_eq!(parsed.get("c").unwrap().content_type, "mac_address");
    }

    #[test]
    fn parse_llm_response_rejects_llm_supplied_unique_id() {
        // `unique_id` is in CONTENT_TYPE_VOCAB but must be REJECTED from LLM
        // output: it is only valid when stamped deterministically based on the
        // `<ALL_UNIQUE>` sentinel. Accepting it from LLM input would let
        // non-ALL_UNIQUE fields be misclassified. The parser drops it (empties
        // the field); combine_dictionary_entries then coerces to "unknown".
        let json = r#"{
            "a": {"label": "A", "description": "d", "content_type": "unique_id"},
            "b": {"label": "B", "description": "d", "content_type": "UNIQUE_ID"},
            "c": {"label": "C", "description": "d", "content_type": "  Unique_ID  "}
        }"#;
        let fields = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let parsed = parse_llm_dictionary_response(json, &fields, true).unwrap();
        assert!(
            parsed.get("a").unwrap().content_type.is_empty(),
            "literal 'unique_id' must be stripped from LLM output"
        );
        assert!(
            parsed.get("b").unwrap().content_type.is_empty(),
            "uppercased 'UNIQUE_ID' must also be stripped (lowercased before check)"
        );
        assert!(
            parsed.get("c").unwrap().content_type.is_empty(),
            "padded/cased 'unique_id' must also be stripped after trim+lowercase"
        );
    }

    #[test]
    fn combine_refuses_llm_supplied_unique_id_for_non_all_unique_field() {
        // Even if a future caller bypasses parse_llm_dictionary_response and
        // hands combine_dictionary_entries an LlmDictField with content_type =
        // "unique_id", combine must refuse to copy it onto a field whose
        // code-derived entry was empty (i.e. not ALL_UNIQUE). Such fields fall
        // through to "unknown" when the flag is on.
        let code_entries = vec![blank_entry("not_unique")];
        let mut llm = HashMap::new();
        llm.insert(
            "not_unique".to_string(),
            LlmDictField {
                label:        "X".to_string(),
                description:  "y".to_string(),
                content_type: "unique_id".to_string(),
            },
        );
        let combined = combine_dictionary_entries(code_entries, &llm, true);
        assert_eq!(
            combined[0].content_type, "unknown",
            "smuggled LLM 'unique_id' on a non-ALL_UNIQUE field must be rejected and fall back to \
             'unknown'"
        );
    }

    #[test]
    fn parse_llm_response_drops_out_of_vocab_content_type() {
        // An out-of-vocabulary token is left empty by parsing; combine_dictionary_entries
        // is what coerces it to "unknown".
        let json = r#"{
            "mystery": {"label": "X", "description": "y", "content_type": "made_up_token"}
        }"#;
        let fields = vec!["mystery".to_string()];
        let parsed = parse_llm_dictionary_response(json, &fields, true).unwrap();
        assert!(parsed.get("mystery").unwrap().content_type.is_empty());
    }

    #[test]
    fn normalize_duration_token_handles_all_forms() {
        // Bare token is the trivial accept.
        assert_eq!(
            normalize_duration_token("duration").as_deref(),
            Some("duration")
        );
        // Well-formed positive integer suffix: preserved verbatim.
        assert_eq!(
            normalize_duration_token("duration:3600").as_deref(),
            Some("duration:3600")
        );
        // Whitespace around the number is tolerated.
        assert_eq!(
            normalize_duration_token("duration: 18000").as_deref(),
            Some("duration:18000")
        );
        // Malformed suffixes degrade gracefully to bare "duration" rather
        // than dropping the classification entirely — the LLM picked
        // "duration" correctly, only the cap is bad.
        assert_eq!(
            normalize_duration_token("duration:0").as_deref(),
            Some("duration")
        );
        assert_eq!(
            normalize_duration_token("duration:-5").as_deref(),
            Some("duration")
        );
        assert_eq!(
            normalize_duration_token("duration:abc").as_deref(),
            Some("duration")
        );
        assert_eq!(
            normalize_duration_token("duration:").as_deref(),
            Some("duration")
        );
        // Non-duration tokens return None so the caller falls through to
        // the regular vocab check.
        assert_eq!(normalize_duration_token("time"), None);
        assert_eq!(normalize_duration_token("email"), None);
        assert_eq!(normalize_duration_token(""), None);
    }

    #[test]
    fn parse_llm_response_accepts_duration_suffix() {
        let json = r#"{
            "elapsed":    {"label": "E", "description": "d", "content_type": "duration:3600"},
            "race_time":  {"label": "R", "description": "d", "content_type": "Duration: 18000"},
            "bare":       {"label": "B", "description": "d", "content_type": "duration"},
            "bad_cap":    {"label": "X", "description": "d", "content_type": "duration:0"},
            "bad_suffix": {"label": "Y", "description": "d", "content_type": "duration:abc"}
        }"#;
        let fields = vec![
            "elapsed".to_string(),
            "race_time".to_string(),
            "bare".to_string(),
            "bad_cap".to_string(),
            "bad_suffix".to_string(),
        ];
        let parsed = parse_llm_dictionary_response(json, &fields, true).unwrap();
        assert_eq!(parsed.get("elapsed").unwrap().content_type, "duration:3600");
        // Casing + inner whitespace normalized.
        assert_eq!(
            parsed.get("race_time").unwrap().content_type,
            "duration:18000"
        );
        assert_eq!(parsed.get("bare").unwrap().content_type, "duration");
        // Malformed suffix collapses to bare "duration" rather than empty.
        assert_eq!(parsed.get("bad_cap").unwrap().content_type, "duration");
        assert_eq!(parsed.get("bad_suffix").unwrap().content_type, "duration");
    }

    #[test]
    fn parse_llm_response_missing_content_type_is_empty() {
        // A missing content_type key is left empty by parsing; combine_dictionary_entries
        // coerces it to "unknown" when the flag is set.
        let json = r#"{ "f": {"label": "F", "description": "d"} }"#;
        let fields = vec!["f".to_string()];
        let parsed = parse_llm_dictionary_response(json, &fields, true).unwrap();
        assert!(parsed.get("f").unwrap().content_type.is_empty());
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
        // infer_content_type = false: pure copy, no "unknown" coercion.
        let combined = combine_dictionary_entries(code_entries, &llm, false);
        assert_eq!(combined[0].label, "A");
        assert_eq!(combined[0].description, "desc a");
        assert_eq!(combined[0].content_type, "city");
        // col_b had no LLM entry, so its content_type stays empty when the flag is off.
        assert!(combined[1].content_type.is_empty());
    }

    #[test]
    fn combine_fills_unknown_for_empty_content_type_when_flag_on() {
        // With --infer-content-type set, every entry must end up with a non-empty
        // content_type: a valid token is kept; a field the LLM left empty (e.g. an
        // out-of-vocab token dropped by parsing) or omitted entirely falls back to "unknown".
        let code_entries = vec![
            blank_entry("kept"),
            blank_entry("emptied"),
            blank_entry("omitted"),
        ];
        let mut llm = HashMap::new();
        llm.insert(
            "kept".to_string(),
            LlmDictField {
                label:        "K".to_string(),
                description:  "d".to_string(),
                content_type: "city".to_string(),
            },
        );
        llm.insert(
            "emptied".to_string(),
            LlmDictField {
                label:        "E".to_string(),
                description:  "d".to_string(),
                content_type: String::new(),
            },
        );
        // "omitted" is intentionally absent from the LLM map.
        let combined = combine_dictionary_entries(code_entries, &llm, true);
        assert_eq!(combined[0].content_type, "city");
        assert_eq!(combined[1].content_type, "unknown");
        assert_eq!(combined[2].content_type, "unknown");
    }

    #[test]
    fn combine_preserves_preset_unique_id_over_llm_value() {
        // generate_code_based_dictionary stamps "unique_id" deterministically on
        // ALL_UNIQUE fields; combine_dictionary_entries must keep that value even
        // when the LLM returned a different (in-vocab) token for the same field.
        let mut preset = blank_entry("pk");
        preset.content_type = "unique_id".to_string();
        let code_entries = vec![preset, blank_entry("other")];
        let mut llm = HashMap::new();
        llm.insert(
            "pk".to_string(),
            LlmDictField {
                label:        "Primary Key".to_string(),
                description:  "row identifier".to_string(),
                content_type: "uuid".to_string(),
            },
        );
        llm.insert(
            "other".to_string(),
            LlmDictField {
                label:        "Other".to_string(),
                description:  "city field".to_string(),
                content_type: "city".to_string(),
            },
        );
        let combined = combine_dictionary_entries(code_entries, &llm, true);
        // Deterministic "unique_id" wins over the LLM's "uuid".
        assert_eq!(combined[0].content_type, "unique_id");
        // Label/Description still flow through from the LLM.
        assert_eq!(combined[0].label, "Primary Key");
        assert_eq!(combined[0].description, "row identifier");
        // Non-ALL_UNIQUE field gets the LLM's token unchanged.
        assert_eq!(combined[1].content_type, "city");
    }

    #[test]
    fn generate_marks_all_unique_field_as_unique_id() {
        // Field "id" has the <ALL_UNIQUE> sentinel in its frequency table, so
        // when infer_content_type is on, generate_code_based_dictionary must
        // pre-set its content_type to "unique_id". The peer field "category"
        // has a normal frequency distribution and must stay empty.
        let stats = vec![
            StatsRecord {
                field:       "id".to_string(),
                r#type:      "Integer".to_string(),
                cardinality: 1000,
                nullcount:   0,
                min:         "1".to_string(),
                max:         "1000".to_string(),
                addl_cols:   IndexMap::new(),
            },
            StatsRecord {
                field:       "category".to_string(),
                r#type:      "String".to_string(),
                cardinality: 2,
                nullcount:   0,
                min:         "a".to_string(),
                max:         "b".to_string(),
                addl_cols:   IndexMap::new(),
            },
        ];
        let frequencies = vec![
            FrequencyRecord {
                field:      "id".to_string(),
                value:      "<ALL_UNIQUE>".to_string(),
                count:      1000,
                percentage: 100.0,
                rank:       1.0,
            },
            FrequencyRecord {
                field:      "category".to_string(),
                value:      "a".to_string(),
                count:      600,
                percentage: 60.0,
                rank:       1.0,
            },
            FrequencyRecord {
                field:      "category".to_string(),
                value:      "b".to_string(),
                count:      400,
                percentage: 40.0,
                rank:       2.0,
            },
        ];
        let entries = generate_code_based_dictionary(&stats, &frequencies, 10, 5, 25, &[], true);
        assert_eq!(entries[0].name, "id");
        assert_eq!(entries[0].content_type, "unique_id");
        assert_eq!(entries[1].name, "category");
        assert!(
            entries[1].content_type.is_empty(),
            "non-ALL_UNIQUE field must leave content_type empty for LLM fill"
        );
    }

    #[test]
    fn generate_skips_unique_id_when_infer_content_type_off() {
        // When --infer-content-type is OFF, the content_type column is suppressed
        // entirely, so we must not pre-set "unique_id" even for ALL_UNIQUE fields.
        let stats = vec![StatsRecord {
            field:       "id".to_string(),
            r#type:      "Integer".to_string(),
            cardinality: 100,
            nullcount:   0,
            min:         "1".to_string(),
            max:         "100".to_string(),
            addl_cols:   IndexMap::new(),
        }];
        let frequencies = vec![FrequencyRecord {
            field:      "id".to_string(),
            value:      "<ALL_UNIQUE>".to_string(),
            count:      100,
            percentage: 100.0,
            rank:       1.0,
        }];
        let entries = generate_code_based_dictionary(&stats, &frequencies, 10, 5, 25, &[], false);
        assert!(entries[0].content_type.is_empty());
    }

    #[test]
    fn generate_does_not_mislabel_constant_field_with_all_unique_value() {
        // Pathological: a field whose only value is literally the string
        // "<ALL_UNIQUE>". qsv frequency emits a single row with value=
        // "<ALL_UNIQUE>" and count==row_count, but stats.cardinality==1.
        // Structural detection (count == cardinality, cardinality > 1)
        // correctly excludes this.
        let stats = vec![StatsRecord {
            field:       "weird".to_string(),
            r#type:      "String".to_string(),
            cardinality: 1,
            nullcount:   0,
            min:         "<ALL_UNIQUE>".to_string(),
            max:         "<ALL_UNIQUE>".to_string(),
            addl_cols:   IndexMap::new(),
        }];
        let frequencies = vec![FrequencyRecord {
            field:      "weird".to_string(),
            value:      "<ALL_UNIQUE>".to_string(),
            count:      500,
            percentage: 100.0,
            rank:       1.0,
        }];
        let entries = generate_code_based_dictionary(&stats, &frequencies, 10, 5, 25, &[], true);
        assert!(
            entries[0].content_type.is_empty(),
            "a constant-value field whose value happens to be the string '<ALL_UNIQUE>' must NOT \
             be classified as unique_id"
        );
    }

    #[test]
    fn generate_detects_unique_id_with_custom_all_unique_text() {
        // If qsv frequency was run with `--all-unique-text` set to a custom
        // string (e.g. "<UNIQUE>"), the sentinel row's text differs but the
        // structural invariant (one row, count == cardinality, cardinality > 1,
        // no nulls) still holds. Detection must succeed regardless of the
        // sentinel text.
        let stats = vec![StatsRecord {
            field:       "pk".to_string(),
            r#type:      "Integer".to_string(),
            cardinality: 250,
            nullcount:   0,
            min:         "1".to_string(),
            max:         "250".to_string(),
            addl_cols:   IndexMap::new(),
        }];
        let frequencies = vec![FrequencyRecord {
            field:      "pk".to_string(),
            value:      "<UNIQUE>".to_string(), // user-customized sentinel text
            count:      250,
            percentage: 100.0,
            rank:       1.0,
        }];
        let entries = generate_code_based_dictionary(&stats, &frequencies, 10, 5, 25, &[], true);
        assert_eq!(
            entries[0].content_type, "unique_id",
            "text-independent detection must classify ALL_UNIQUE even when frequency's sentinel \
             text was customized"
        );
    }

    #[test]
    fn generate_does_not_misclassify_high_cardinality_as_unique_id() {
        // HIGH_CARDINALITY fields also produce a single frequency row with
        // count==row_count and percentage==100.0, but their cardinality is
        // strictly less than row_count (some values repeat). The
        // `count == cardinality` check correctly excludes them.
        let stats = vec![StatsRecord {
            field:       "city".to_string(),
            r#type:      "String".to_string(),
            cardinality: 800, // many distinct values, but with repeats
            nullcount:   0,
            min:         "Aachen".to_string(),
            max:         "Zurich".to_string(),
            addl_cols:   IndexMap::new(),
        }];
        let frequencies = vec![FrequencyRecord {
            field:      "city".to_string(),
            value:      "<HIGH_CARDINALITY>".to_string(),
            count:      10_000, // row_count >> cardinality
            percentage: 100.0,
            rank:       1.0,
        }];
        let entries = generate_code_based_dictionary(&stats, &frequencies, 10, 5, 25, &[], true);
        assert!(
            entries[0].content_type.is_empty(),
            "HIGH_CARDINALITY field must not be classified as unique_id"
        );
    }

    #[test]
    fn generate_does_not_mislabel_unique_id_when_nulls_present() {
        // Semantic contract: "every row has a distinct non-null value". A
        // field where cardinality == count but nullcount > 0 doesn't qualify
        // (some rows have no value at all). qsv frequency wouldn't emit the
        // ALL_UNIQUE sentinel for this case anyway, but the explicit
        // nullcount==0 check provides defense in depth for hand-crafted or
        // cached frequency input.
        let stats = vec![StatsRecord {
            field:       "maybe_id".to_string(),
            r#type:      "Integer".to_string(),
            cardinality: 95,
            nullcount:   5, // 5 rows are NULL
            min:         "1".to_string(),
            max:         "95".to_string(),
            addl_cols:   IndexMap::new(),
        }];
        let frequencies = vec![FrequencyRecord {
            field:      "maybe_id".to_string(),
            value:      "<ALL_UNIQUE>".to_string(),
            count:      95,
            percentage: 100.0,
            rank:       1.0,
        }];
        let entries = generate_code_based_dictionary(&stats, &frequencies, 10, 5, 25, &[], true);
        assert!(
            entries[0].content_type.is_empty(),
            "a field with nulls must NOT be classified as unique_id even if non-null cardinality \
             matches the frequency count"
        );
    }
}
