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
///
/// `legacy_license` (the `--dcat-legacy-license` CLI flag, default off)
/// re-emits `dct:license` at the Dataset level for back-compat. In strict
/// v3 (the default) the license lives only on the Distribution.
pub fn build(
    ckan_package: &Value,
    ckan_resources: &[Value],
    dpp: &Value,
    stats: &Value,
    input_path: &str,
    legacy_license: bool,
) -> Value {
    let mut ds: Map<String, Value> = Map::new();
    add_context_and_type(&mut ds);
    add_core_identity(&mut ds, ckan_package, input_path);
    add_provenance(&mut ds, ckan_package);
    add_classification(&mut ds, ckan_package);
    add_coverage(&mut ds, ckan_package, dpp, stats);
    add_governance(&mut ds, ckan_package);
    add_distributions(
        &mut ds,
        ckan_package,
        ckan_resources,
        stats,
        input_path,
        legacy_license,
    );
    Value::Object(ds)
}

/// `@context` + `@type` header.
fn add_context_and_type(ds: &mut Map<String, Value>) {
    ds.insert(
        "@context".to_string(),
        Value::String("https://doi-do.github.io/dcat-us/context.jsonld".to_string()),
    );
    ds.insert(
        "@type".to_string(),
        Value::String("dcat:Dataset".to_string()),
    );
}

/// dct:title, dct:description, dct:identifier, dct:modified, dct:issued.
///
/// Phase 2e: `dct:modified` is sanitized to strip ISO 8601 interval
/// syntax (`R/P1Y`, `2020-01-01/P1Y`, etc.) — DCAT-US v3 requires a
/// discrete date here. Frequency-of-update values belong on
/// `dct:accrualPeriodicity` (Phase 5).
fn add_core_identity(ds: &mut Map<String, Value>, ckan_package: &Value, input_path: &str) {
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
    if let Some(modif) = string_opt(ckan_package.get("metadata_modified"))
        && let Some(clean) = sanitize_discrete_date(&modif)
    {
        ds.insert("dct:modified".to_string(), Value::String(clean));
    }
    if let Some(issued) = string_opt(ckan_package.get("metadata_created")) {
        ds.insert("dct:issued".to_string(), Value::String(issued));
    }
}

/// Reject ISO 8601 interval / repeating-interval values for slots that
/// require a discrete date (`dct:modified`). Returns `None` for inputs
/// that are clearly interval syntax (`R/...`, `Pn...`, or any value with
/// embedded `/` separating two date-like halves). Otherwise returns the
/// trimmed input unchanged.
fn sanitize_discrete_date(input: &str) -> Option<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }
    // Repeating interval: "R/P1Y", "R3/2020-01-01/P1Y"
    if trimmed.starts_with('R') && trimmed.contains('/') {
        return None;
    }
    // Bare period: "P1Y", "PT1H", "P1DT12H"
    if trimmed.starts_with('P') {
        return None;
    }
    // "start/end" or "start/duration" interval
    if trimmed.contains('/') {
        return None;
    }
    Some(trimmed.to_string())
}

/// dct:publisher only. Per DCAT-US v3 (and the v1.1 → v3 migration guide),
/// `dct:license` moves to the Distribution; see `build_distribution`.
/// The `--dcat-legacy-license` flag (handled in `add_distributions`)
/// re-emits it at Dataset level for back-compat.
fn add_provenance(ds: &mut Map<String, Value>, ckan_package: &Value) {
    if let Some(publisher) = take_first_str(ckan_package, &["publisher", "author"]) {
        ds.insert(
            "dct:publisher".to_string(),
            json!({ "@type": "foaf:Agent", "foaf:name": publisher }),
        );
    }
}

/// dcat:keyword (from CKAN tags) and dcat:theme (from CKAN groups).
fn add_classification(ds: &mut Map<String, Value>, ckan_package: &Value) {
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
    if let Some(groups) = ckan_package.get("groups").and_then(|v| v.as_array()) {
        let theme: Vec<Value> = groups
            .iter()
            .filter_map(|g| g.get("name").cloned().or_else(|| Some(g.clone())))
            .collect();
        if !theme.is_empty() {
            ds.insert("dcat:theme".to_string(), Value::Array(theme));
        }
    }
}

/// dct:spatial + dct:temporal — both emitted as JSON arrays per
/// DCAT-US v3 (Location[], PeriodOfTime[]).
fn add_coverage(ds: &mut Map<String, Value>, ckan_package: &Value, dpp: &Value, stats: &Value) {
    // spatial: prefer WKT from the suggestion_formula output, falling back
    // to inferred lat/lon column bounds. Both branches yield an array of
    // Location objects.
    let spatial: Option<Vec<Value>> = if let Some(wkt) = ckan_package
        .pointer("/dpp_suggestions/spatial_extent/value")
        .and_then(|v| v.as_str())
    {
        // The GeoSPARQL wktLiteral datatype is identified by its canonical
        // W3C OGC IRI, which uses http://. This is a stable identifier, not
        // a URL to fetch; changing the scheme would break interop with
        // every DCAT/GeoSPARQL consumer.
        Some(vec![json!({
            "@type":      "dct:Location",
            "locn:geometry": {
                "@type": "http://www.opengis.net/ont/geosparql#wktLiteral", //DevSkim: ignore DS137138
                "@value": wkt,
            }
        })])
    } else {
        bbox_from_dpps(dpp, stats)
    };
    if let Some(arr) = spatial {
        ds.insert("dct:spatial".to_string(), Value::Array(arr));
    }

    if let Some(arr) = temporal_from_dpps(dpp, stats) {
        ds.insert("dct:temporal".to_string(), Value::Array(arr));
    }
}

/// dcat-us:accessLevel + dct:conformsTo + dct:language.
///
/// `dct:conformsTo` is always emitted as a `dct:Standard` object pointing
/// at the DCAT-US v3 resource page — this projection always claims v3
/// conformance.
///
/// `dct:language`, when provided, is normalized to ISO 639-1 (the
/// DCAT-US v3 migration guide narrowed `language` from RFC 5646 to
/// 2-letter codes). Values that don't match a known ISO 639-1 code pass
/// through unchanged (we don't reject the user's input) but are
/// (Phase 5) reported as a warning.
fn add_governance(ds: &mut Map<String, Value>, ckan_package: &Value) {
    let access = take_first_str(ckan_package, &["dcat-us:accessLevel", "access_level"])
        .unwrap_or_else(|| "public".to_string());
    ds.insert("dcat-us:accessLevel".to_string(), Value::String(access));

    ds.insert(
        "dct:conformsTo".to_string(),
        json!({
            "@type": "dct:Standard",
            "@id":   "https://resources.data.gov/resources/dcat-us3/",
        }),
    );

    if let Some(lang) = take_first_str(ckan_package, &["language"])
        && let Some(normalized) = normalize_iso_639_1(&lang)
    {
        ds.insert("dct:language".to_string(), Value::String(normalized));
    }
}

/// Map a free-text language tag to its ISO 639-1 2-letter code.
/// Accepts plain codes ("en"), RFC 5646 with region ("en-US"), or the
/// expanded form some CKAN catalogs use ("English"). Returns `None` for
/// anything unrecognized — caller decides whether to warn or pass through.
fn normalize_iso_639_1(input: &str) -> Option<String> {
    let cleaned = input.trim().to_lowercase();
    if cleaned.is_empty() {
        return None;
    }
    // RFC 5646 subtag stripping: "en-US" → "en", "zh-Hans-CN" → "zh".
    let base = cleaned.split(&['-', '_'][..]).next().unwrap_or(&cleaned);
    // Curated allow-list covering the most common DCAT-US catalog usage;
    // extend as needed. Codes per ISO 639-1.
    const KNOWN: &[(&str, &str)] = &[
        ("en", "en"),
        ("english", "en"),
        ("es", "es"),
        ("spanish", "es"),
        ("fr", "fr"),
        ("french", "fr"),
        ("de", "de"),
        ("german", "de"),
        ("it", "it"),
        ("italian", "it"),
        ("pt", "pt"),
        ("portuguese", "pt"),
        ("nl", "nl"),
        ("dutch", "nl"),
        ("ja", "ja"),
        ("japanese", "ja"),
        ("ko", "ko"),
        ("korean", "ko"),
        ("zh", "zh"),
        ("chinese", "zh"),
        ("ru", "ru"),
        ("russian", "ru"),
        ("ar", "ar"),
        ("arabic", "ar"),
        ("hi", "hi"),
        ("hindi", "hi"),
    ];
    KNOWN
        .iter()
        .find(|(k, _)| *k == cleaned || *k == base)
        .map(|(_, v)| v.to_string())
}

/// dcat:distribution — one Distribution per CKAN resource, each carrying
/// the per-resource license (with the package license as fallback) and
/// a `csvw:tableSchema` derived from qsv stats. When `legacy_license` is
/// set, the package license is also re-emitted on the Dataset (inserted
/// just before the distribution array so output ordering remains stable).
fn add_distributions(
    ds: &mut Map<String, Value>,
    ckan_package: &Value,
    ckan_resources: &[Value],
    stats: &Value,
    input_path: &str,
    legacy_license: bool,
) {
    let pkg_license = take_first_str(ckan_package, &["license_id", "license"]);

    if legacy_license && let Some(slug) = &pkg_license {
        ds.insert("dct:license".to_string(), license_value(slug));
    }

    let distributions: Vec<Value> = ckan_resources
        .iter()
        .map(|r| build_distribution(r, stats, input_path, pkg_license.as_deref()))
        .collect();
    ds.insert("dcat:distribution".to_string(), Value::Array(distributions));
}

/// Map a license slug or absolute IRI to its JSON-LD representation:
/// known slugs / absolute URLs → `{"@id": "..."}`; opaque strings →
/// literal `Value::String`. Shared between Dataset-level (legacy) and
/// Distribution-level emission so the wire shape stays consistent.
fn license_value(slug: &str) -> Value {
    match license_iri(slug) {
        Some(iri) => json!({ "@id": iri }),
        None => Value::String(slug.to_string()),
    }
}

/// Read the first non-empty string value from `obj` matching one of
/// `keys` in priority order. Replaces the
/// `string_opt(get(k1)).or_else(|| string_opt(get(k2)))` chains scattered
/// through the dataset builder.
fn take_first_str(obj: &Value, keys: &[&str]) -> Option<String> {
    for k in keys {
        if let Some(s) = string_opt(obj.get(k)) {
            return Some(s);
        }
    }
    None
}

/// Build one `dcat:Distribution` entry. `package_license_fallback` is
/// the package-level license slug (or absolute IRI) to use when the
/// resource itself doesn't declare one — per DCAT-US v3 the license
/// lives on the Distribution rather than the Dataset.
fn build_distribution(
    resource: &Value,
    stats: &Value,
    input_path: &str,
    package_license_fallback: Option<&str>,
) -> Value {
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
    // `dcat:downloadURL` is specified as an IRI, so only emit it when the
    // resource actually carries an absolute http(s) / ftp(s) URL. A bare
    // local filesystem path is not a valid IRI and would break strict
    // JSON-LD consumers. The input file path is still preserved under
    // `qsv:sourcePath` for human inspection.
    // Trim before insertion: `is_absolute_iri` accepts surrounding
    // whitespace (CKAN URL fields sometimes round-trip with stray ws),
    // but the value we serialize into the IRI slot must be exactly what
    // we validated -- otherwise an input like `"   https://x   "` would
    // pass the filter and land in the JSON-LD with literal spaces.
    if let Some(url) = string_opt(resource.get("url")).and_then(|u| {
        let trimmed = u.trim().to_string();
        is_absolute_iri(&trimmed).then_some(trimmed)
    }) {
        d.insert("dcat:downloadURL".to_string(), Value::String(url));
    }
    d.insert(
        "qsv:sourcePath".to_string(),
        Value::String(input_path.to_string()),
    );
    d.insert(
        "dcat:mediaType".to_string(),
        Value::String("text/csv".to_string()),
    );
    let format = string_opt(resource.get("format")).unwrap_or_else(|| "CSV".to_string());
    d.insert("dct:format".to_string(), Value::String(format));

    // dct:license: prefer per-resource value; fall back to package-level.
    // Strict DCAT-US v3 location for license (Dataset-level emission only
    // happens behind --dcat-legacy-license).
    if let Some(license) = string_opt(resource.get("license_id"))
        .or_else(|| string_opt(resource.get("license")))
        .or_else(|| package_license_fallback.map(str::to_string))
    {
        d.insert("dct:license".to_string(), license_value(&license));
    }

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

/// Derive a `dct:Location` array from inferred LAT/LON columns. v3
/// allows multiple Locations per Dataset, so we return a `Vec` even
/// though today's column-inference only yields one bounding box.
fn bbox_from_dpps(dpp: &Value, stats: &Value) -> Option<Vec<Value>> {
    let lat = dpp.get("LAT_FIELD").and_then(|v| v.as_str())?;
    let lon = dpp.get("LON_FIELD").and_then(|v| v.as_str())?;
    let min_lon = stats_lookup(stats, lon, "min").and_then(json_to_f64)?;
    let max_lon = stats_lookup(stats, lon, "max").and_then(json_to_f64)?;
    let min_lat = stats_lookup(stats, lat, "min").and_then(json_to_f64)?;
    let max_lat = stats_lookup(stats, lat, "max").and_then(json_to_f64)?;
    Some(vec![json!({
        "@type": "dct:Location",
        "dcat:bbox": format!(
            "POLYGON(({min_lon} {min_lat}, {min_lon} {max_lat}, {max_lon} {max_lat}, {max_lon} {min_lat}, {min_lon} {min_lat}))"
        )
    })])
}

/// Derive a `dct:PeriodOfTime` array — one entry per inferred date
/// column. DCAT-US v3 allows multiple temporal coverages per Dataset;
/// previously only the first DATE_FIELDS entry was consumed.
fn temporal_from_dpps(dpp: &Value, stats: &Value) -> Option<Vec<Value>> {
    let dates = dpp.get("DATE_FIELDS").and_then(|v| v.as_array())?;
    let mut out: Vec<Value> = Vec::new();
    for field_v in dates {
        let Some(field) = field_v.as_str() else {
            continue;
        };
        let Some(start) =
            stats_lookup(stats, field, "min").and_then(|v| v.as_str().map(str::to_string))
        else {
            continue;
        };
        let Some(end) =
            stats_lookup(stats, field, "max").and_then(|v| v.as_str().map(str::to_string))
        else {
            continue;
        };
        out.push(json!({
            "@type":          "dct:PeriodOfTime",
            "dcat:startDate": start,
            "dcat:endDate":   end,
        }));
    }
    if out.is_empty() { None } else { Some(out) }
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

/// Cheap check that `s` looks like an absolute IRI suitable for a
/// `dcat:downloadURL` slot. Accepts the common web + file URL schemes
/// (http, https, ftp, ftps, file). Local filesystem paths return false.
///
/// URI schemes are case-insensitive per RFC 3986 §3.1, so `HTTPS://`,
/// `Http://`, etc. all count -- otherwise valid CKAN resource URLs that
/// happen to come in non-lowercase form would be silently dropped from
/// the DCAT distribution. Compares with `eq_ignore_ascii_case` rather
/// than allocating a lowercased copy.
fn is_absolute_iri(s: &str) -> bool {
    const SCHEMES: &[&str] = &["http", "https", "ftp", "ftps", "file"];
    let s = s.trim();
    let Some((scheme, rest)) = s.split_once("://") else {
        return false;
    };
    !rest.is_empty() && SCHEMES.iter().any(|cand| scheme.eq_ignore_ascii_case(cand))
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
    // The Creative Commons + Open Data Commons license IRIs below use
    // http:// because those are the *canonical* identifiers published by
    // CC / ODC and used by data.gov, DCAT-US sample catalogs, and Dublin
    // Core. Changing to https:// would produce different IRIs that no
    // longer round-trip with other DCAT consumers, so DS137138 is
    // suppressed inline.
    match trimmed {
        "cc-by" => Some("http://creativecommons.org/licenses/by/4.0/".to_string()), /* DevSkim: ignore DS137138 */
        "cc-by-sa" => Some("http://creativecommons.org/licenses/by-sa/4.0/".to_string()), /* DevSkim: ignore DS137138 */
        "cc-zero" => Some("http://creativecommons.org/publicdomain/zero/1.0/".to_string()), /* DevSkim: ignore DS137138 */
        "odc-by" => Some("http://opendatacommons.org/licenses/by/1.0/".to_string()), /* DevSkim: ignore DS137138 */
        "odc-pddl" => Some("http://opendatacommons.org/licenses/pddl/1.0/".to_string()), /* DevSkim: ignore DS137138 */
        other if other.starts_with("http://") || other.starts_with("https://") => {
            Some(other.to_string())
        },
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// URL schemes are case-insensitive per RFC 3986 §3.1; the IRI check
    /// must accept `HTTPS://`, `Http://`, etc. so CKAN resource URLs that
    /// happen to come in non-lowercase form aren't silently dropped from
    /// the DCAT distribution.
    #[test]
    fn is_absolute_iri_is_case_insensitive() {
        for ok in [
            "http://example.com/data.csv",
            "https://example.com/data.csv",
            "HTTPS://example.com/data.csv",
            "Http://example.com/data.csv",
            "FTP://example.com/data.csv",
            "ftps://example.com/data.csv",
            "FILE:///tmp/x.csv",
            "   https://example.com/data.csv   ", // tolerated leading/trailing ws
        ] {
            assert!(is_absolute_iri(ok), "should accept: {ok:?}");
        }
        for bad in [
            "data.csv",
            "/tmp/data.csv",
            "./data.csv",
            "C:\\data.csv",
            "javascript://oops", // unsupported scheme
            "https://",          // no authority/path
            "",
        ] {
            assert!(!is_absolute_iri(bad), "should reject: {bad:?}");
        }
    }

    /// Known CKAN license slugs map to canonical IRIs; absolute http(s)
    /// IRIs are passed through; everything else returns None so the caller
    /// emits a literal string instead of a fabricated `@id`.
    #[test]
    fn license_iri_maps_known_slugs() {
        assert_eq!(
            license_iri("cc-by").as_deref(),
            Some("http://creativecommons.org/licenses/by/4.0/")
        );
        assert_eq!(
            license_iri("https://example.com/license").as_deref(),
            Some("https://example.com/license")
        );
        assert!(license_iri("uk-ogl").is_none());
        assert!(license_iri("").is_none());
    }

    /// Regression: a whitespace-padded URL that passes `is_absolute_iri`
    /// must be emitted *trimmed* into `dcat:downloadURL`, not with literal
    /// surrounding spaces (which would no longer be a valid IRI).
    #[test]
    fn download_url_is_trimmed_before_insertion() {
        let resource = serde_json::json!({
            "name": "data",
            "url":  "   https://example.com/data.csv   ",
        });
        let stats = serde_json::json!({});
        let dist = build_distribution(&resource, &stats, "/local/data.csv", None);
        let url = dist
            .get("dcat:downloadURL")
            .and_then(|v| v.as_str())
            .expect("dcat:downloadURL should be present");
        assert_eq!(url, "https://example.com/data.csv");
    }

    /// Bare local filesystem path -> no `dcat:downloadURL` (not an IRI),
    /// but `qsv:sourcePath` still records the source.
    #[test]
    fn local_path_omits_download_url_but_keeps_source_path() {
        let resource = serde_json::json!({ "name": "data" });
        let stats = serde_json::json!({});
        let dist = build_distribution(&resource, &stats, "/tmp/data.csv", None);
        assert!(dist.get("dcat:downloadURL").is_none());
        assert_eq!(
            dist.get("qsv:sourcePath").and_then(|v| v.as_str()),
            Some("/tmp/data.csv"),
        );
    }

    /// Phase 2a: dct:spatial is emitted as an array of dct:Location
    /// objects (not a single object) when WKT is provided via the
    /// suggestion_formula path.
    #[test]
    fn spatial_is_array_with_single_wkt_extent() {
        let pkg = serde_json::json!({
            "dpp_suggestions": {
                "spatial_extent": {
                    "value": "SRID=4326;POLYGON((-180 -90, -180 90, 180 90, 180 -90, -180 -90))"
                }
            }
        });
        let resources = vec![serde_json::json!({})];
        let dpp = serde_json::json!({});
        let stats = serde_json::json!({});
        let ds = build(&pkg, &resources, &dpp, &stats, "/tmp/data.csv", false);
        let spatial = ds.pointer("/dct:spatial").expect("dct:spatial");
        assert!(spatial.is_array(), "dct:spatial must be an array");
        let arr = spatial.as_array().unwrap();
        assert_eq!(arr.len(), 1);
        let wkt = arr[0]
            .pointer("/locn:geometry/@value")
            .and_then(|v| v.as_str())
            .unwrap();
        assert!(wkt.starts_with("SRID=4326;POLYGON(("));
    }

    /// Phase 2b: dct:temporal emits one PeriodOfTime per inferred date
    /// column (previously only the first DATE_FIELDS entry was used).
    #[test]
    fn temporal_emits_one_period_per_date_column() {
        let pkg = serde_json::json!({});
        let resources = vec![serde_json::json!({})];
        let dpp = serde_json::json!({ "DATE_FIELDS": ["created", "updated"] });
        let stats = serde_json::json!({
            "created": {"stats": {"min": "2024-01-01", "max": "2024-12-31"}},
            "updated": {"stats": {"min": "2024-02-01", "max": "2024-11-30"}},
        });
        let ds = build(&pkg, &resources, &dpp, &stats, "/tmp/data.csv", false);
        let temporal = ds.pointer("/dct:temporal").expect("dct:temporal");
        assert!(temporal.is_array());
        let arr = temporal.as_array().unwrap();
        assert_eq!(arr.len(), 2, "one PeriodOfTime per date column");
        assert_eq!(
            arr[0].pointer("/dcat:startDate").and_then(|v| v.as_str()),
            Some("2024-01-01")
        );
        assert_eq!(
            arr[1].pointer("/dcat:startDate").and_then(|v| v.as_str()),
            Some("2024-02-01")
        );
    }

    /// Phase 2c: strict v3 — dct:license lives on the Distribution,
    /// not the Dataset.
    #[test]
    fn license_lives_on_distribution_not_dataset() {
        let pkg = serde_json::json!({ "license_id": "cc-by" });
        let resources = vec![serde_json::json!({})];
        let dpp = serde_json::json!({});
        let stats = serde_json::json!({});
        let ds = build(&pkg, &resources, &dpp, &stats, "/tmp/data.csv", false);
        assert!(
            ds.get("dct:license").is_none(),
            "dct:license must not be on Dataset in strict v3"
        );
        let dist_license = ds
            .pointer("/dcat:distribution/0/dct:license/@id")
            .and_then(|v| v.as_str())
            .expect("dct:license on Distribution");
        assert!(
            dist_license.contains("creativecommons.org"),
            "got: {dist_license}"
        );
    }

    /// Phase 2c: --dcat-legacy-license re-emits dct:license at the
    /// Dataset level alongside the Distribution-level copy.
    #[test]
    fn legacy_license_flag_re_emits_on_dataset() {
        let pkg = serde_json::json!({ "license_id": "cc-by" });
        let resources = vec![serde_json::json!({})];
        let dpp = serde_json::json!({});
        let stats = serde_json::json!({});
        let ds = build(&pkg, &resources, &dpp, &stats, "/tmp/data.csv", true);
        assert!(
            ds.get("dct:license").is_some(),
            "dct:license must be re-emitted on Dataset under legacy flag"
        );
        assert!(
            ds.pointer("/dcat:distribution/0/dct:license").is_some(),
            "dct:license must still appear on Distribution under legacy flag"
        );
    }

    /// Phase 2d: dct:conformsTo is always emitted as a dct:Standard
    /// object pointing at the DCAT-US v3 spec URL.
    #[test]
    fn conforms_to_emits_standard_object() {
        let pkg = serde_json::json!({});
        let resources = vec![serde_json::json!({})];
        let dpp = serde_json::json!({});
        let stats = serde_json::json!({});
        let ds = build(&pkg, &resources, &dpp, &stats, "/tmp/data.csv", false);
        assert_eq!(
            ds.pointer("/dct:conformsTo/@type").and_then(|v| v.as_str()),
            Some("dct:Standard")
        );
        assert_eq!(
            ds.pointer("/dct:conformsTo/@id").and_then(|v| v.as_str()),
            Some("https://resources.data.gov/resources/dcat-us3/")
        );
    }

    /// Phase 2d: dct:language is normalized to ISO 639-1.
    #[test]
    fn language_normalized_to_iso_639_1() {
        assert_eq!(normalize_iso_639_1("en"), Some("en".to_string()));
        assert_eq!(normalize_iso_639_1("en-US"), Some("en".to_string()));
        assert_eq!(normalize_iso_639_1("English"), Some("en".to_string()));
        assert_eq!(normalize_iso_639_1("ZH-Hans-CN"), Some("zh".to_string()));
        assert_eq!(normalize_iso_639_1("xx-Klingon"), None);
        assert_eq!(normalize_iso_639_1(""), None);
        assert_eq!(normalize_iso_639_1("  fr  "), Some("fr".to_string()));
    }

    /// Phase 2e: dct:modified rejects ISO 8601 interval syntax.
    #[test]
    fn modified_strips_interval_syntax() {
        assert_eq!(
            sanitize_discrete_date("2024-01-15"),
            Some("2024-01-15".to_string())
        );
        assert_eq!(
            sanitize_discrete_date(" 2024-01-15T10:30:00 "),
            Some("2024-01-15T10:30:00".to_string())
        );
        // ISO 8601 repeating-interval (frequency, not a discrete date)
        assert_eq!(sanitize_discrete_date("R/P1Y"), None);
        assert_eq!(sanitize_discrete_date("R3/2020-01-01/P1Y"), None);
        // Bare period
        assert_eq!(sanitize_discrete_date("P1Y"), None);
        assert_eq!(sanitize_discrete_date("PT1H"), None);
        // start/end interval
        assert_eq!(sanitize_discrete_date("2020-01-01/2021-01-01"), None);
        // Empty / whitespace
        assert_eq!(sanitize_discrete_date(""), None);
        assert_eq!(sanitize_discrete_date("   "), None);
    }
}
