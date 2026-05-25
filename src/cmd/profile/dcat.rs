//! DCAT-US v3 projection.
//!
//! Best-effort mapping from the CKAN-shaped output qsv profile produces to a
//! `dcat:Dataset` JSON-LD shape compatible with the DCAT-US v3 recommendation
//! (<https://doi-do.github.io/dcat-us/>). The mapping intentionally stays
//! pragmatic: only fields we can populate from the analysis context or the
//! seed package/resource meta are emitted. Anything else is left absent
//! rather than guessed.
//!
//! The full DCAT-US v3 vocabulary is large; v1 here covers the core dataset
//! properties (title, description, identifier, modified, keyword, theme,
//! spatial, temporal, accessLevel) plus a single `dcat:Distribution` per
//! resource describing the CSV itself, with an inlined `tableSchema`
//! derived from qsv stats. Future work can extend the mapping driven by
//! issue follow-ups.

use std::path::Path;

use serde_json::{Map, Value, json};

/// Build the DCAT-US v3 projection block.
///
/// `ckan_package` is the merged `ckan.package` object (post-formula
/// evaluation); `ckan_resources` is the matching list of resources (today
/// just one); `dpp` is the inferred metadata block; `input_path` is used to
/// derive default title and downloadURL when the package/resource don't
/// provide them.
pub fn build(
    ckan_package: &Value,
    ckan_resources: &[Value],
    dpp: &Value,
    stats: &Value,
    input_path: &str,
) -> Value {
    let mut ds: Map<String, Value> = Map::new();
    ds.insert(
        "@context".to_string(),
        Value::String("https://doi-do.github.io/dcat-us/context.jsonld".to_string()),
    );
    ds.insert(
        "@type".to_string(),
        Value::String("dcat:Dataset".to_string()),
    );

    let title = string_or(
        ckan_package.get("title"),
        Path::new(input_path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("dataset"),
    );
    ds.insert("dct:title".to_string(), Value::String(title));

    if let Some(desc) = string_opt(ckan_package.get("notes")) {
        ds.insert("dct:description".to_string(), Value::String(desc));
    }
    if let Some(id) = string_opt(ckan_package.get("name")) {
        ds.insert("dct:identifier".to_string(), Value::String(id));
    }
    if let Some(modif) = string_opt(ckan_package.get("metadata_modified")) {
        ds.insert("dct:modified".to_string(), Value::String(modif));
    }
    if let Some(issued) = string_opt(ckan_package.get("metadata_created")) {
        ds.insert("dct:issued".to_string(), Value::String(issued));
    }
    if let Some(license) = string_opt(ckan_package.get("license_id"))
        .or_else(|| string_opt(ckan_package.get("license")))
    {
        // Only emit `@id` for slugs we map to canonical IRIs or for values
        // that already look like absolute IRIs. Unknown identifiers fall
        // through to a literal string -- avoids fabricating bogus JSON-LD
        // IRIs from arbitrary CKAN license_id values.
        let license_value = match license_iri(&license) {
            Some(iri) => json!({ "@id": iri }),
            None => Value::String(license),
        };
        ds.insert("dct:license".to_string(), license_value);
    }
    if let Some(publisher) =
        string_opt(ckan_package.get("publisher")).or_else(|| string_opt(ckan_package.get("author")))
    {
        ds.insert(
            "dct:publisher".to_string(),
            json!({ "@type": "foaf:Agent", "foaf:name": publisher }),
        );
    }

    // keyword: CKAN tags list → simple string array.
    if let Some(tags) = ckan_package.get("tags").and_then(|v| v.as_array()) {
        let kw: Vec<Value> = tags
            .iter()
            .filter_map(|t| {
                t.as_str()
                    .map(|s| Value::String(s.to_string()))
                    .or_else(|| t.get("name").cloned())
            })
            .collect();
        if !kw.is_empty() {
            ds.insert("dcat:keyword".to_string(), Value::Array(kw));
        }
    }
    // theme: CKAN groups list.
    if let Some(groups) = ckan_package.get("groups").and_then(|v| v.as_array()) {
        let theme: Vec<Value> = groups
            .iter()
            .filter_map(|g| g.get("name").cloned().or_else(|| Some(g.clone())))
            .collect();
        if !theme.is_empty() {
            ds.insert("dcat:theme".to_string(), Value::Array(theme));
        }
    }

    // spatial: prefer WKT from the suggestion_formula output, falling back
    // to inferred lat/lon column bounds.
    if let Some(wkt) = ckan_package
        .pointer("/dpp_suggestions/spatial_extent/value")
        .and_then(|v| v.as_str())
    {
        ds.insert(
            "dct:spatial".to_string(),
            json!({
                "@type":      "dct:Location",
                "locn:geometry": {
                    "@type": "http://www.opengis.net/ont/geosparql#wktLiteral",
                    "@value": wkt,
                }
            }),
        );
    } else if let Some(bbox) = bbox_from_dpps(dpp, stats) {
        ds.insert("dct:spatial".to_string(), bbox);
    }

    // temporal: derive from the first inferred date column's stats min/max.
    if let Some(temporal) = temporal_from_dpps(dpp, stats) {
        ds.insert("dct:temporal".to_string(), temporal);
    }

    // accessLevel defaults to public unless the package already declared one.
    let access = string_opt(ckan_package.get("dcat-us:accessLevel"))
        .or_else(|| string_opt(ckan_package.get("access_level")))
        .unwrap_or_else(|| "public".to_string());
    ds.insert("dcat-us:accessLevel".to_string(), Value::String(access));

    // distribution: one entry per ckan resource. Inline the table schema
    // derived from qsv stats so DCAT consumers get a column-level dictionary
    // without re-running analysis.
    let distributions: Vec<Value> = ckan_resources
        .iter()
        .map(|r| build_distribution(r, stats, input_path))
        .collect();
    ds.insert("dcat:distribution".to_string(), Value::Array(distributions));

    Value::Object(ds)
}

fn build_distribution(resource: &Value, stats: &Value, input_path: &str) -> Value {
    let mut d: Map<String, Value> = Map::new();
    d.insert(
        "@type".to_string(),
        Value::String("dcat:Distribution".to_string()),
    );
    let title = string_or(
        resource.get("name"),
        Path::new(input_path)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("data.csv"),
    );
    d.insert("dct:title".to_string(), Value::String(title));
    if let Some(desc) = string_opt(resource.get("description")) {
        d.insert("dct:description".to_string(), Value::String(desc));
    }
    let url = string_opt(resource.get("url")).unwrap_or_else(|| input_path.to_string());
    d.insert("dcat:downloadURL".to_string(), Value::String(url));
    d.insert(
        "dcat:mediaType".to_string(),
        Value::String("text/csv".to_string()),
    );
    let format = string_opt(resource.get("format")).unwrap_or_else(|| "CSV".to_string());
    d.insert("dct:format".to_string(), Value::String(format));

    if let Ok(meta) = std::fs::metadata(input_path) {
        d.insert("dcat:byteSize".to_string(), json!(meta.len()));
    }

    // tableSchema: per-column dictionary derived from qsv stats. Mirrors the
    // CSVW shape (https://www.w3.org/TR/tabular-metadata/).
    if let Some(cols) = stats.as_object() {
        let columns: Vec<Value> = cols
            .iter()
            .map(|(name, blob)| {
                let stats_obj = blob.get("stats").unwrap_or(blob);
                json!({
                    "name":      name,
                    "titles":    [name],
                    "datatype":  csvw_datatype(stats_obj.get("type")),
                    "qsv:cardinality":  stats_obj.get("cardinality"),
                    "qsv:nullcount":    stats_obj.get("nullcount"),
                    "qsv:min":          stats_obj.get("min"),
                    "qsv:max":          stats_obj.get("max"),
                })
            })
            .collect();
        d.insert(
            "csvw:tableSchema".to_string(),
            json!({ "columns": columns }),
        );
    }
    Value::Object(d)
}

fn bbox_from_dpps(dpp: &Value, stats: &Value) -> Option<Value> {
    let lat = dpp.get("LAT_FIELD").and_then(|v| v.as_str())?;
    let lon = dpp.get("LON_FIELD").and_then(|v| v.as_str())?;
    let min_lon = stats_lookup(stats, lon, "min").and_then(json_to_f64)?;
    let max_lon = stats_lookup(stats, lon, "max").and_then(json_to_f64)?;
    let min_lat = stats_lookup(stats, lat, "min").and_then(json_to_f64)?;
    let max_lat = stats_lookup(stats, lat, "max").and_then(json_to_f64)?;
    Some(json!({
        "@type": "dct:Location",
        "dcat:bbox": format!(
            "POLYGON(({min_lon} {min_lat}, {min_lon} {max_lat}, {max_lon} {max_lat}, {max_lon} {min_lat}, {min_lon} {min_lat}))"
        )
    }))
}

fn temporal_from_dpps(dpp: &Value, stats: &Value) -> Option<Value> {
    let dates = dpp.get("DATE_FIELDS").and_then(|v| v.as_array())?;
    let first = dates.iter().filter_map(|v| v.as_str()).next()?;
    let start = stats_lookup(stats, first, "min")
        .and_then(|v| v.as_str())?
        .to_string();
    let end = stats_lookup(stats, first, "max")
        .and_then(|v| v.as_str())?
        .to_string();
    Some(json!({
        "@type":      "dct:PeriodOfTime",
        "dcat:startDate": start,
        "dcat:endDate":   end,
    }))
}

/// Look up `stats[<col_name>].stats.<field>` via direct map access. Used
/// instead of `Value::pointer` so header names containing `/` or `~`
/// (legal CSV but JSON-Pointer-significant characters) don't silently
/// resolve to `None`.
fn stats_lookup<'a>(stats: &'a Value, col_name: &str, field: &str) -> Option<&'a Value> {
    stats.as_object()?.get(col_name)?.get("stats")?.get(field)
}

/// Best-effort mapping from qsv stats type strings to CSVW datatypes.
fn csvw_datatype(t: Option<&Value>) -> &'static str {
    match t.and_then(|v| v.as_str()) {
        Some("Integer") => "integer",
        Some("Float") => "double",
        Some("Boolean") => "boolean",
        Some("Date") => "date",
        Some("DateTime") => "dateTime",
        Some("NULL") => "string",
        _ => "string",
    }
}

fn string_opt(v: Option<&Value>) -> Option<String> {
    v.and_then(|x| x.as_str().map(|s| s.to_string()))
        .filter(|s| !s.is_empty())
}

fn string_or(v: Option<&Value>, default: &str) -> String {
    string_opt(v).unwrap_or_else(|| default.to_string())
}

fn json_to_f64(v: &Value) -> Option<f64> {
    v.as_f64()
        .or_else(|| v.as_i64().map(|i| i as f64))
        .or_else(|| v.as_str().and_then(|s| s.parse::<f64>().ok()))
}

/// Map a CKAN-style license string to a canonical IRI when we recognize it.
///
/// Returns `Some(iri)` for known slugs and for values that already look like
/// absolute IRIs (starts with `http://` / `https://`). Returns `None` for
/// everything else, so callers can emit a literal `dct:license` string
/// instead of fabricating a JSON-LD `@id` that won't resolve.
fn license_iri(license: &str) -> Option<String> {
    let trimmed = license.trim();
    if trimmed.is_empty() {
        return None;
    }
    match trimmed {
        "cc-by" => Some("http://creativecommons.org/licenses/by/4.0/".to_string()),
        "cc-by-sa" => Some("http://creativecommons.org/licenses/by-sa/4.0/".to_string()),
        "cc-zero" => Some("http://creativecommons.org/publicdomain/zero/1.0/".to_string()),
        "odc-by" => Some("http://opendatacommons.org/licenses/by/1.0/".to_string()),
        "odc-pddl" => Some("http://opendatacommons.org/licenses/pddl/1.0/".to_string()),
        other if other.starts_with("http://") || other.starts_with("https://") => {
            Some(other.to_string())
        },
        _ => None,
    }
}
