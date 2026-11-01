//! LLM-inferred metadata for `qsv profile`: `--describegpt <json>`
//! ingests a `qsv describegpt --format json` result, `--infer` runs
//! describegpt itself.
//!
//! The neural output is VERIFIED symbolically before it reaches the
//! projection, and always as the inferred tier (a `fallback:`), so
//! anything the publisher supplies or discovery finds wins:
//!
//!   * the Description loses describegpt's attribution footer (command line, model, URL, timestamp
//!     — which would otherwise be published verbatim) and gets a one-line disclosure from the
//!     profile instead; the footers go to the `llm_attribution` output block.
//!   * Tags become `keyword`s (trimmed, de-duplicated, never empty). With `--tag-vocab`, a tag must
//!     be a member of the vocabulary — describegpt only ASKS the model to keep to it — and members
//!     also become `theme` Concepts carrying the vocabulary's definition.
//!   * Dictionary labels and descriptions annotate the CSVW columns they name.

use std::path::Path;

use serde_json::{Map, Value, json};

use crate::{CliError, CliResult, util};

/// The `llm` projection-context value and its by-products.
pub(super) struct LlmMetadata {
    /// Context for the templates: `{description, model, keywords, themes, dictionary}`.
    pub context:     Value,
    /// The attribution footers, per describegpt phase.
    pub attribution: Value,
    /// Info observations (dropped off-vocabulary tags, an unmatched dictionary).
    pub notes:       Vec<(String, String)>,
}

/// A tag vocabulary: its name (the file stem) and `(tag, description)`
/// rows.
pub(super) struct TagVocab {
    name: String,
    tags: Vec<(String, String)>,
}

/// Read a `tag,description` CSV — the format `describegpt --tag-vocab`
/// takes. A malformed file is an error: the user named it.
pub(super) fn read_tag_vocab(path: &str) -> CliResult<TagVocab> {
    let mut rdr = csv::ReaderBuilder::new()
        .from_path(path)
        .map_err(|e| CliError::Other(format!("--tag-vocab: cannot read {path}: {e}")))?;
    let headers = rdr.headers()?.clone();
    if headers.len() != 2
        || headers.get(0).unwrap_or("").trim() != "tag"
        || headers.get(1).unwrap_or("").trim() != "description"
    {
        return fail_incorrectusage_clierror!(
            "--tag-vocab: the CSV must have exactly 2 columns, `tag` and `description`."
        );
    }
    let mut tags = Vec::new();
    for rec in rdr.records() {
        let rec = rec.map_err(|e| CliError::Other(format!("--tag-vocab: {e}")))?;
        let tag = rec.get(0).unwrap_or("").trim();
        if !tag.is_empty() {
            tags.push((tag.to_string(), rec.get(1).unwrap_or("").trim().to_string()));
        }
    }
    let name = Path::new(path)
        .file_stem()
        .map_or_else(|| path.to_string(), |s| s.to_string_lossy().to_string());
    Ok(TagVocab { name, tags })
}

/// Read a `describegpt --format json` file. A file that is not JSON is
/// an error: the user named it.
pub(super) fn read_describegpt_json(path: &str) -> CliResult<Value> {
    let raw = std::fs::read_to_string(path)
        .map_err(|e| CliError::Other(format!("--describegpt: cannot read {path}: {e}")))?;
    let value: Value = serde_json::from_str(&raw).map_err(|e| {
        CliError::Other(format!(
            "--describegpt: {path} is not describegpt JSON: {e}"
        ))
    })?;
    if !["Description", "Tags", "Dictionary"]
        .iter()
        .any(|k| value.get(k).is_some())
    {
        return fail_incorrectusage_clierror!(
            "--describegpt: {path} has no Description, Tags or Dictionary (expected the output of \
             `qsv describegpt --format json`)."
        );
    }
    Ok(value)
}

/// Run `qsv describegpt <input> --all --format json` (LLM settings from
/// the `QSV_LLM_*` environment, describegpt's cache honored). `Err` is
/// a message for a Recommended warning: `--infer` never fails the run.
pub(super) fn run_describegpt(
    input_path: &str,
    tag_vocab: Option<&str>,
    delimiter: u8,
    no_headers: bool,
) -> Result<Value, String> {
    // describegpt has no --delimiter / --no-headers, so when profile
    // reads the file differently from the defaults, hand it a normalized
    // copy whose columns are the ones profile analyzed.
    let normalized = if delimiter != b',' || no_headers {
        Some(normalized_copy(input_path, delimiter, no_headers).map_err(|e| e.to_string())?)
    } else {
        None
    };
    let input_path = normalized.as_ref().map_or_else(
        || input_path.to_string(),
        |f| f.path().to_string_lossy().to_string(),
    );
    let out = tempfile::Builder::new()
        .prefix("qsv-profile-describegpt")
        .suffix(".json")
        .tempfile()
        .map_err(|e| e.to_string())?;
    let out_path = out.path().to_string_lossy().to_string();
    let mut args = vec!["--all", "--format", "json", "--output", &out_path];
    if let Some(v) = tag_vocab {
        args.extend(["--tag-vocab", v]);
    }
    util::run_qsv_cmd(
        "describegpt",
        &args,
        &input_path,
        "qsv profile: ran `describegpt` for --infer",
    )
    .map_err(|_| {
        "`qsv describegpt` did not complete (is an LLM configured? see QSV_LLM_BASE_URL, \
         QSV_LLM_MODEL and QSV_LLM_APIKEY)"
            .to_string()
    })?;
    read_describegpt_json(&out_path).map_err(|e| e.to_string())
}

/// A comma-delimited copy of `input_path` read with `delimiter`; with
/// `no_headers`, headed by the 0-based column indices that name
/// profile's columns in that mode.
fn normalized_copy(
    input_path: &str,
    delimiter: u8,
    no_headers: bool,
) -> CliResult<tempfile::NamedTempFile> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .has_headers(false)
        .flexible(true)
        .from_path(input_path)?;
    let out = tempfile::Builder::new()
        .prefix("qsv-profile-describegpt-input")
        .suffix(".csv")
        .tempfile()?;
    {
        let mut wtr = csv::Writer::from_path(out.path())?;
        let mut record = csv::ByteRecord::new();
        let mut first = true;
        while rdr.read_byte_record(&mut record)? {
            if first && no_headers {
                wtr.write_record((0..record.len()).map(|i| i.to_string()))?;
            }
            first = false;
            wtr.write_byte_record(&record)?;
        }
        wtr.flush()?;
    }
    Ok(out)
}

/// A phase's `response`: older describegpt versions wrote Tags and
/// Dictionary as JSON-encoded strings, so a string that parses as JSON
/// is used parsed.
fn response(dg: &Value, phase: &str) -> Option<Value> {
    let r = dg.get(phase)?.get("response")?;
    match r {
        Value::String(s) => {
            Some(serde_json::from_str::<Value>(s.trim()).unwrap_or_else(|_| r.clone()))
        },
        _ => Some(r.clone()),
    }
}

/// Case-insensitive object key lookup (`name` / `Name`).
fn get_ci<'a>(obj: &'a Value, key: &str) -> Option<&'a Value> {
    obj.as_object()?
        .iter()
        .find(|(k, _)| k.eq_ignore_ascii_case(key))
        .map(|(_, v)| v)
}

/// The model named on an attribution footer's `Model:` line.
fn model_of(attribution: &str) -> Option<String> {
    attribution
        .lines()
        .find_map(|l| l.trim().strip_prefix("Model:"))
        .map(|m| m.trim().to_string())
        .filter(|m| !m.is_empty())
}

/// The description text without describegpt's attribution footer and
/// without a leading heading that only says "Description".
fn clean_description(text: &str) -> String {
    let body = crate::cmd::describegpt::strip_attribution_block(text);
    let mut lines: Vec<&str> = body.lines().collect();
    while lines.first().is_some_and(|l| l.trim().is_empty()) {
        lines.remove(0);
    }
    if let Some(first) = lines.first() {
        let bare = first
            .trim()
            .trim_start_matches('#')
            .trim()
            .trim_matches('*')
            .trim()
            .trim_end_matches(':')
            .trim();
        if bare.eq_ignore_ascii_case("description") {
            lines.remove(0);
        }
    }
    lines.join("\n").trim().to_string()
}

/// Build the `llm` context from describegpt JSON. `headers` are the
/// profiled file's column names, used to check the dictionary describes
/// this file.
pub(super) fn build(dg: &Value, vocab: Option<&TagVocab>, headers: &[String]) -> LlmMetadata {
    let mut ctx = Map::new();
    let mut attribution = Map::new();
    let mut notes: Vec<(String, String)> = Vec::new();
    let mut model: Option<String> = None;

    if let Some(Value::String(text)) = response(dg, "Description") {
        if let Some(idx) = text.find("Generated by") {
            let footer = text[idx..].trim().to_string();
            model = model.or_else(|| model_of(&footer));
            attribution.insert("Description".into(), json!(footer));
        }
        let cleaned = clean_description(&text);
        if !cleaned.is_empty() {
            ctx.insert("description".into(), json!(cleaned));
        }
    }

    if let Some(tags_resp) = response(dg, "Tags") {
        if let Some(att) = get_ci(&tags_resp, "attribution").and_then(Value::as_str) {
            model = model.or_else(|| model_of(att));
            attribution.insert("Tags".into(), json!(att.trim()));
        }
        let raw: Vec<String> = get_ci(&tags_resp, "tags")
            .or(Some(&tags_resp))
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .map(|t| t.trim().to_string())
            .filter(|t| !t.is_empty())
            .collect();
        let mut keywords: Vec<String> = Vec::new();
        let mut themes: Vec<Value> = Vec::new();
        let mut dropped: Vec<String> = Vec::new();
        for tag in raw {
            if keywords.iter().any(|k| k.eq_ignore_ascii_case(&tag)) {
                continue;
            }
            match vocab {
                None => keywords.push(tag),
                Some(v) => match v.tags.iter().find(|(t, _)| t.eq_ignore_ascii_case(&tag)) {
                    Some((t, definition)) => {
                        let mut concept = json!({
                            "@type": "Concept",
                            "prefLabel": t,
                            "inScheme": { "@type": "ConceptScheme", "title": v.name },
                        });
                        if !definition.is_empty() {
                            concept["definition"] = json!(definition);
                        }
                        themes.push(concept);
                        keywords.push(t.clone());
                    },
                    None => dropped.push(tag),
                },
            }
        }
        if !dropped.is_empty() {
            notes.push((
                "keyword".into(),
                format!(
                    "dropped {} LLM tag(s) that are not in the --tag-vocab vocabulary: {}",
                    dropped.len(),
                    dropped.join(", ")
                ),
            ));
        }
        if !keywords.is_empty() {
            ctx.insert("keywords".into(), json!(keywords));
        }
        if !themes.is_empty() {
            ctx.insert("themes".into(), json!(themes));
        }
    }

    if let Some(dict) = response(dg, "Dictionary") {
        if let Some(att) = get_ci(&dict, "attribution").and_then(Value::as_str) {
            model = model.or_else(|| model_of(att));
            attribution.insert("Dictionary".into(), json!(att.trim()));
        }
        let fields = get_ci(&dict, "fields")
            .or(Some(&dict))
            .and_then(Value::as_array);
        let mut by_name = Map::new();
        for f in fields.into_iter().flatten() {
            let text = |k: &str| {
                get_ci(f, k)
                    .and_then(Value::as_str)
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(str::to_string)
            };
            let Some(name) = text("name") else { continue };
            if !headers.contains(&name) {
                continue;
            }
            let mut entry = Map::new();
            if let Some(label) = text("label") {
                entry.insert("label".into(), json!(label));
            }
            if let Some(desc) = text("description") {
                entry.insert("description".into(), json!(desc));
            }
            if !entry.is_empty() {
                by_name.insert(name, Value::Object(entry));
            }
        }
        if by_name.is_empty() && fields.is_some_and(|f| !f.is_empty()) {
            notes.push((
                "distribution/0/csvw:tableSchema".into(),
                "the describegpt Dictionary names none of this file's columns (was it generated \
                 for a different file?); it was not used"
                    .into(),
            ));
        } else if !by_name.is_empty() {
            ctx.insert("dictionary".into(), Value::Object(by_name));
        }
    }

    if ctx.is_empty() {
        return LlmMetadata {
            context: Value::Null,
            attribution: Value::Object(attribution),
            notes,
        };
    }
    ctx.insert(
        "model".into(),
        json!(model.unwrap_or_else(|| "unknown".into())),
    );
    LlmMetadata {
        context: Value::Object(ctx),
        attribution: Value::Object(attribution),
        notes,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FOOTER: &str = "Generated by qsv v27.0.1 describegpt\nCommand line: qsv describegpt \
                          x.csv --all\nModel: openai/gpt-oss-20b\nTimestamp: \
                          2026-10-31T00:00:00+00:00\n\nWARNING: Description generated by an LLM \
                          and may contain inaccuracies. Verify before using!\n";

    fn vocab() -> TagVocab {
        TagVocab {
            name: "city-themes".into(),
            tags: vec![
                (
                    "public_safety".into(),
                    "Police, fire and emergency services".into(),
                ),
                (
                    "sanitation".into(),
                    "Trash, recycling and street cleaning".into(),
                ),
                ("parks".into(), "Parks and recreation".into()),
            ],
        }
    }

    fn dg() -> Value {
        json!({
            "Description": {
                "response": format!("**Description**\n\n311 service requests in Boston.\n\n---\n\n{FOOTER}")
            },
            "Tags": { "response": {
                "tags": ["Sanitation", "public_safety", "  ", "pothole_repair", "sanitation"],
                "attribution": FOOTER,
            }},
            "Dictionary": { "response": {
                "fields": [
                    {"name": "case_id", "label": "Case ID", "description": "Unique case number"},
                    {"name": "not_a_column", "label": "X", "description": "Y"}
                ],
                "attribution": FOOTER,
            }},
        })
    }

    #[test]
    fn description_loses_heading_and_footer_and_keeps_the_model() {
        let m = build(&dg(), None, &["case_id".into()]);
        assert_eq!(m.context["description"], "311 service requests in Boston.");
        assert_eq!(m.context["model"], "openai/gpt-oss-20b");
        assert!(
            m.attribution["Description"]
                .as_str()
                .unwrap()
                .starts_with("Generated by qsv")
        );
    }

    #[test]
    fn tags_without_vocab_are_trimmed_and_deduplicated_keywords() {
        let m = build(&dg(), None, &[]);
        assert_eq!(
            m.context["keywords"],
            json!(["Sanitation", "public_safety", "pothole_repair"])
        );
        assert!(m.context.get("themes").is_none());
    }

    #[test]
    fn vocab_keeps_members_as_concepts_and_drops_the_rest() {
        let m = build(&dg(), Some(&vocab()), &[]);
        // the vocabulary's own spelling, in the LLM's order; the decoy `parks` is never added
        assert_eq!(
            m.context["keywords"],
            json!(["sanitation", "public_safety"])
        );
        let themes = m.context["themes"].as_array().unwrap();
        assert_eq!(themes.len(), 2);
        assert_eq!(themes[0]["@type"], "Concept");
        assert_eq!(themes[0]["prefLabel"], "sanitation");
        assert_eq!(
            themes[0]["definition"],
            "Trash, recycling and street cleaning"
        );
        assert_eq!(themes[0]["inScheme"]["title"], "city-themes");
        assert!(
            m.notes
                .iter()
                .any(|(f, n)| f == "keyword" && n.contains("pothole_repair"))
        );
    }

    #[test]
    fn dictionary_keeps_only_this_files_columns() {
        let m = build(&dg(), None, &["case_id".into()]);
        assert_eq!(m.context["dictionary"]["case_id"]["label"], "Case ID");
        assert!(m.context["dictionary"].get("not_a_column").is_none());
    }

    #[test]
    fn dictionary_for_another_file_is_noted_not_used() {
        let m = build(&dg(), None, &["other".into()]);
        assert!(m.context.get("dictionary").is_none());
        assert!(
            m.notes
                .iter()
                .any(|(f, _)| f == "distribution/0/csvw:tableSchema")
        );
    }

    #[test]
    fn legacy_string_encoded_responses_are_parsed() {
        let legacy = json!({
            "Tags": { "response": "[\"housing\",\"noise\"]" },
            "Dictionary": { "response":
                "[{\"Name\":\"a\",\"Label\":\"Alpha\",\"Description\":\"First\"}]" },
        });
        let m = build(&legacy, None, &["a".into()]);
        assert_eq!(m.context["keywords"], json!(["housing", "noise"]));
        assert_eq!(m.context["dictionary"]["a"]["description"], "First");
        assert_eq!(m.context["model"], "unknown");
    }

    #[test]
    fn nothing_usable_is_null_context() {
        let m = build(
            &json!({ "Tags": { "response": { "tags": [] } } }),
            None,
            &[],
        );
        assert!(m.context.is_null());
    }
}

#[cfg(test)]
mod normalize_tests {
    use std::io::Write as _;

    use super::*;

    #[test]
    fn normalized_copy_rewrites_the_delimiter_and_names_headerless_columns() {
        let mut f = tempfile::Builder::new().suffix(".csv").tempfile().unwrap();
        f.write_all(b"1;a,b\n2;c\n").unwrap();
        let out = normalized_copy(&f.path().to_string_lossy(), b';', true).unwrap();
        let text = std::fs::read_to_string(out.path()).unwrap();
        assert_eq!(text, "0,1\n1,\"a,b\"\n2,c\n");
    }
}
