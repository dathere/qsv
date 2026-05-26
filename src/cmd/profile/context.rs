//! Build the Jinja2 evaluation context for the `profile` command.
//!
//! Mirrors the shape of `ProcessingContext` in datapusher-plus so the same
//! `formula` / `suggestion_formula` templates that DP+ evaluates against a
//! CKAN package + resource also work here:
//!
//! - `package`   – seed dict from `--package-meta`, plus inferred package keys
//! - `resource`  – seed dict from `--resource-meta`, plus inferred resource keys
//! - `dpps`      – per-column qsv stats, keyed by column name: `{col: {stats: {...}}}`
//! - `dppf`      – per-column qsv frequency entries: `{col: [{value, count, percentage, rank},
//!   ...]}`
//! - `dpp`       – inferred metadata: lat/lon/date columns + dataset-level stats
//!
//! Everything is materialized as a `serde_json::Value` so it can be handed
//! straight to the Python side without further re-serialization.

use std::path::Path;

use foldhash::HashSet;
use serde_json::{Map, Value, json};

use super::spec::Spec;
use crate::{
    CliError, CliResult,
    cmd::{describegpt::dictionary::parse_frequency_csv, stats::StatsData},
    config::Delimiter,
    select::SelectColumns,
    util::{self, SchemaArgs, StatsMode},
};

/// CLI-facing knobs that the context builder cares about. Kept narrow so we
/// don't couple `context.rs` to the full top-level `Args` struct.
pub struct ContextArgs<'a> {
    pub input_path:      &'a str,
    pub no_headers:      bool,
    pub delimiter:       Option<Delimiter>,
    pub jobs:            Option<usize>,
    pub force:           bool,
    pub memcheck:        bool,
    pub initial_context: Option<&'a str>,
}

/// Result of the analysis pass — the JSON context plus the column headers we
/// extracted along the way (used by the output module), the
/// initial-context's `dataset_info` JSON-Pointer override map (applied to
/// the final output by `profile.rs::run` after `dcat::build` returns), and
/// the §5.4 list of `dataset_info` JSON-Pointer paths that the user marked
/// `{value, force: true}` (consulted by `merge_discovered` to skip overlay
/// at those paths).
pub struct AnalysisContext {
    pub context:           Value,
    pub headers:           Vec<String>,
    pub dataset_info:      Value,
    pub forced_dcat_paths: Vec<String>,
}

/// Build the full analysis context for `input_path`.
pub fn build(args: &ContextArgs, _spec: Option<&Spec>) -> CliResult<AnalysisContext> {
    // --- 1. stats ---------------------------------------------------------
    let schema_args = SchemaArgs {
        arg_input:            Some(args.input_path.to_string()),
        flag_no_headers:      args.no_headers,
        flag_delimiter:       args.delimiter,
        flag_jobs:            args.jobs,
        flag_polars:          false,
        flag_memcheck:        args.memcheck,
        flag_force:           args.force,
        flag_prefer_dmy:      false,
        flag_dates_whitelist: "date,time,due,open,close,created".to_string(),
        flag_enum_threshold:  50,
        flag_ignore_case:     false,
        flag_strict_dates:    false,
        flag_strict_formats:  false,
        flag_pattern_columns: SelectColumns::parse("")?,
        flag_stdout:          false,
        flag_output:          None,
    };

    let (headers_record, stats) = util::get_stats_records(&schema_args, StatsMode::Schema)?;
    let headers: Vec<String> = headers_record
        .iter()
        .map(|h| std::str::from_utf8(h).unwrap_or("").to_string())
        .collect();

    // --- 2. frequency ------------------------------------------------------
    // Mirrors describegpt: shell out to `qsv frequency` and parse the CSV.
    // Defaults: top 25 values per column, drop "Other" / NULL aggregate rows
    // so the per-column lists stay tight for downstream consumers.
    //
    // Forwards every CSV-parsing / execution flag that profile took on its
    // own command line so the frequency pass interprets the input under the
    // same assumptions as the stats pass above. Otherwise an input that
    // needed `--no-headers` or `--delimiter ;` would parse one way for
    // stats and another way for frequency, producing inconsistent records.
    let mut freq_owned: Vec<String> = vec![
        "--limit".to_string(),
        "25".to_string(),
        "--no-other".to_string(),
        "--no-nulls".to_string(),
    ];
    append_csv_flags(
        &mut freq_owned,
        args,
        FreqOrCount {
            jobs:     true,
            force:    true,
            memcheck: true,
        },
    );
    let freq_args: Vec<&str> = freq_owned.iter().map(String::as_str).collect();
    let (freq_csv, _stderr) = util::run_qsv_cmd(
        "frequency",
        &freq_args,
        args.input_path,
        "qsv profile: ran `frequency`",
    )?;
    let freq_records = parse_frequency_csv(&freq_csv)?;

    // --- 3. dpps (stats by column) ----------------------------------------
    let dpps = build_dpps(&stats)?;

    // --- 4. dppf (frequency by column) ------------------------------------
    let mut dppf: Map<String, Value> = Map::new();
    for fr in &freq_records {
        let entry = json!({
            "value":      fr.value,
            "count":      fr.count,
            "percentage": fr.percentage,
            "rank":       fr.rank,
        });
        dppf.entry(fr.field.clone())
            .or_insert_with(|| Value::Array(Vec::new()))
            .as_array_mut()
            .unwrap()
            .push(entry);
    }

    // --- 5. dpp (inferred metadata) ---------------------------------------
    let (lat_field, lon_field) = detect_lat_lon(&stats);
    let date_fields = collect_typed(&stats, "Date");
    let datetime_fields = collect_typed(&stats, "DateTime");

    let row_count = count_rows(args)?;
    let size_bytes = std::fs::metadata(args.input_path).map_or(0, |m| m.len());

    let dpp = json!({
        "LAT_FIELD":           lat_field,
        "LON_FIELD":           lon_field,
        // True when *either* lat or lon is missing, matching DP+'s helpers
        // (jinja2_helpers.py:148 uses `is None or`). With `&&`, a
        // single-field-missing case would slip past the guards in
        // spatial_extent_wkt() / spatial_extent_feature_collection() and
        // crash mid-render instead of raising the intended ValueError.
        "NO_LAT_LON_FIELDS":   lat_field.is_none() || lon_field.is_none(),
        "DATE_FIELDS":         date_fields,
        "NO_DATE_FIELDS":      date_fields.is_empty(),
        "DATETIME_FIELDS":     datetime_fields,
        "NO_DATETIME_FIELDS":  datetime_fields.is_empty(),
        // DP+'s jinja2_helpers reads `dpp.RECORD_COUNT` directly (e.g. in
        // `get_column_null_percentage`); expose it as a flat alias so those
        // helpers work unchanged against our context.
        "RECORD_COUNT":        row_count,
        "dataset_stats": {
            "row_count":    row_count,
            "column_count": headers.len(),
            "size_bytes":   size_bytes,
        },
    });

    // --- 6. package / resource seed dicts ---------------------------------
    // Loaded from --initial-context (unified single-file replacement for
    // the old --package-meta + --resource-meta pair). dataset_info
    // (the third returned slot) holds JSON-Pointer overrides applied to
    // the final output by profile.rs::run after dcat::build returns; we
    // round-trip it through the analysis context so the orchestrator
    // doesn't need a separate loader call.
    let (package, mut resource, dataset_info, forced_dcat_paths) =
        load_initial_context(args.initial_context)?;
    // Default resource.name from the input file stem if not explicitly seeded.
    if !resource.is_object() {
        resource = json!({});
    }
    if resource.get("name").is_none()
        && let Some(name) = Path::new(args.input_path)
            .file_stem()
            .and_then(|s| s.to_str())
    {
        resource
            .as_object_mut()
            .unwrap()
            .insert("name".to_string(), Value::String(name.to_string()));
    }

    // --- 7. assemble ------------------------------------------------------
    let context = json!({
        "package":  package,
        "resource": resource,
        "dpps":     dpps,
        "dppf":     dppf,
        "dpp":      dpp,
    });

    Ok(AnalysisContext {
        context,
        headers,
        dataset_info,
        forced_dcat_paths,
    })
}

/// Build the `dpps` dict — `{col_name: {stats: {<StatsData fields>}}}`.
///
/// Wrapping under a `stats` sub-key mirrors how DP+'s helpers expect to read
/// `dpps[col]["stats"]["type"]` etc.
fn build_dpps(stats: &[StatsData]) -> CliResult<Map<String, Value>> {
    let mut out = Map::with_capacity(stats.len());
    for sd in stats {
        let stats_obj = serde_json::to_value(sd).map_err(|e| {
            CliError::Other(format!("could not serialize stats for `{}`: {e}", sd.field))
        })?;
        out.insert(sd.field.clone(), json!({ "stats": stats_obj }));
    }
    Ok(out)
}

/// Detect latitude / longitude columns by name + numeric range, matching
/// `detect_lat_lon_fields` in DP+'s `jinja2_helpers.py`.
fn detect_lat_lon(stats: &[StatsData]) -> (Option<String>, Option<String>) {
    const LAT_CANDIDATES: &[&str] = &["lat", "latitude", "y", "ycoord", "y_coord"];
    const LON_CANDIDATES: &[&str] = &["lon", "lng", "long", "longitude", "x", "xcoord", "x_coord"];
    let by_lower: foldhash::HashMap<String, &StatsData> =
        stats.iter().map(|s| (s.field.to_lowercase(), s)).collect();

    let pick = |candidates: &[&str], lo: f64, hi: f64| -> Option<String> {
        for cand in candidates {
            let Some(sd) = by_lower.get(*cand) else {
                continue;
            };
            // qsv's type strings: "Float", "Integer", "String", "Date", "DateTime", ...
            if sd.r#type != "Float" && sd.r#type != "Integer" {
                continue;
            }
            let parse = |s: &Option<String>| s.as_deref().and_then(|v| v.parse::<f64>().ok());
            let (Some(min), Some(max)) = (parse(&sd.min), parse(&sd.max)) else {
                continue;
            };
            if min >= lo && max <= hi {
                return Some(sd.field.clone());
            }
        }
        None
    };

    let lat = pick(LAT_CANDIDATES, -90.0, 90.0);
    let lon = pick(LON_CANDIDATES, -180.0, 180.0);
    (lat, lon)
}

/// Collect field names whose qsv stats type matches `wanted` (e.g. "Date").
fn collect_typed(stats: &[StatsData], wanted: &str) -> Vec<String> {
    // De-dup by name in case stats includes both base + length rows.
    let mut seen: HashSet<String> = HashSet::default();
    let mut out = Vec::new();
    for sd in stats {
        if sd.r#type == wanted && seen.insert(sd.field.clone()) {
            out.push(sd.field.clone());
        }
    }
    out
}

/// Shell out to `qsv count` for an authoritative row count. Falls back to 0
/// on failure (better to emit metadata with a missing count than to fail the
/// whole command).
///
/// Forwards the same `--no-headers` / `--delimiter` flags the stats and
/// frequency passes use; otherwise headers get counted as a row, or rows
/// with non-comma delimiters split incorrectly. `qsv count` does not accept
/// `--jobs` / `--memcheck` / `--force`, so those are skipped.
fn count_rows(args: &ContextArgs) -> CliResult<u64> {
    let mut owned: Vec<String> = Vec::new();
    append_csv_flags(
        &mut owned,
        args,
        FreqOrCount {
            jobs:     false,
            force:    false,
            memcheck: false,
        },
    );
    let argv: Vec<&str> = owned.iter().map(String::as_str).collect();
    let Ok((stdout, _stderr)) =
        util::run_qsv_cmd("count", &argv, args.input_path, "qsv profile: ran `count`")
    else {
        return Ok(0);
    };
    Ok(stdout.trim().parse::<u64>().unwrap_or(0))
}

/// Which optional execution flags the target subprocess supports. Used by
/// `append_csv_flags` to gate the forwarded flag set per command.
struct FreqOrCount {
    jobs:     bool,
    force:    bool,
    memcheck: bool,
}

/// Append CSV-parsing + execution flags from `args` onto `out` as separate
/// argv tokens. Owns String values so the caller can borrow them as
/// `&[&str]` for `run_qsv_cmd`.
fn append_csv_flags(out: &mut Vec<String>, args: &ContextArgs, gates: FreqOrCount) {
    if args.no_headers {
        out.push("--no-headers".to_string());
    }
    if let Some(d) = args.delimiter {
        out.push("--delimiter".to_string());
        out.push((d.as_byte() as char).to_string());
    }
    if gates.jobs
        && let Some(n) = args.jobs
    {
        out.push("--jobs".to_string());
        out.push(n.to_string());
    }
    if gates.force && args.force {
        out.push("--force".to_string());
    }
    if gates.memcheck && args.memcheck {
        out.push("--memcheck".to_string());
    }
}

/// §5.4: walk the `dataset_info` subtree of the raw initial-context
/// JSON (before `normalize_value_force` strips the wrappers) and
/// collect every JSON-Pointer key that was paired with a
/// `{"value": ..., "force": true}` wrapper.
///
/// Only `dataset_info` is scanned because its keys are already in
/// DCAT JSON-Pointer space (e.g. `/dcat/dct:title`) — the same space
/// `merge_discovered` needs to skip-test against. Forcing a field on
/// the `package` / `resource` side would require a CKAN→DCAT pointer
/// mapping table; deferred to a future enhancement.
///
/// The returned vector preserves insertion order so users can
/// reason about precedence; duplicates are not deduped (the merge
/// path checks set membership semantics on a per-key basis).
fn collect_forced_dataset_info_paths(raw_doc: &Value) -> Vec<String> {
    let Some(ds_info) = raw_doc.get("dataset_info").and_then(Value::as_object) else {
        return Vec::new();
    };
    ds_info
        .iter()
        .filter_map(|(key, val)| is_forced_wrapper(val).then(|| key.clone()))
        .collect()
}

/// Recognize the exact `{"value": ..., "force": true}` shape.
/// Anything else (plain values, force=false wrappers, plain objects
/// that happen to share a key name) returns false.
fn is_forced_wrapper(v: &Value) -> bool {
    let Some(map) = v.as_object() else {
        return false;
    };
    map.len() == 2
        && map.contains_key("value")
        && map.get("force").is_some_and(|f| f.as_bool() == Some(true))
}

/// Load and split a `--initial-context` JSON file into its three
/// top-level components plus the §5.4 forced-paths list. Returns
/// `(package, resource, dataset_info, forced_dataset_info_paths)`
/// where any missing key defaults to `json!({})` and the forced
/// list is empty when no `{value, force: true}` wrappers appear
/// under `dataset_info`.
///
/// Phase 4b: leaf values shaped exactly as `{"value": …, "force": bool}`
/// are normalized to their inner `value` so the rest of the pipeline
/// sees a clean CKAN-shaped object.
///
/// §5.4: `force: true` wrappers under `dataset_info` are *additionally*
/// recorded as JSON-Pointer paths into the forced list. The merge
/// step (`merge_discovered`) consults that list and refuses to overlay
/// discovered values at those paths — so a user can declare a field
/// "intentionally absent" and prevent publisher DCAT discovery from
/// silently filling it in. Forced wrappers on the `package` /
/// `resource` side are accepted and stripped, but their force
/// semantics are NOT honored — that requires a CKAN→DCAT JSON-Pointer
/// mapping table and is a deferred follow-up.
pub(super) fn load_initial_context(
    path: Option<&str>,
) -> CliResult<(Value, Value, Value, Vec<String>)> {
    let Some(path) = path else {
        return Ok((json!({}), json!({}), json!({}), Vec::new()));
    };
    let raw = std::fs::read_to_string(path).map_err(|e| {
        CliError::Other(format!("could not read initial-context file `{path}`: {e}"))
    })?;
    let doc: Value = serde_json::from_str(&raw).map_err(|e| {
        CliError::Other(format!(
            "could not parse initial-context file `{path}` as JSON: {e}"
        ))
    })?;
    if !doc.is_object() {
        return Err(CliError::Other(format!(
            "initial-context file `{path}` must be a JSON object at the top level"
        )));
    }
    // §5.4: collect forced paths from `dataset_info` BEFORE
    // normalize_value_force strips the wrappers.
    let forced_paths = collect_forced_dataset_info_paths(&doc);

    let package = normalize_value_force(doc.get("package").cloned().unwrap_or(json!({})));
    let resource = normalize_value_force(doc.get("resource").cloned().unwrap_or(json!({})));
    // Roborev 2440#2: dataset_info must also pass through wrapper
    // normalization so that an override like
    //   "/dcat/dcat:contactPoint": {"value": {...}, "force": true}
    // unwraps to the inner value before being written to the output.
    // Otherwise the wrapper object itself becomes the DCAT value and
    // the override fails to rescue --strict-dcat validation.
    let dataset_info = normalize_value_force(doc.get("dataset_info").cloned().unwrap_or(json!({})));
    Ok((package, resource, dataset_info, forced_paths))
}

/// Recursively walk a Value; whenever a Map has exactly the two keys
/// `value` and `force` (and `force` is a bool), replace the wrapper
/// with the inner `value`. Other Maps are descended into; non-Map
/// Values pass through unchanged. Structural fields like
/// `contact_point: {fn, hasEmail}` are NOT wrapper-shaped (different
/// keys), so they survive intact.
fn normalize_value_force(v: Value) -> Value {
    match v {
        Value::Object(map) => {
            // Detect wrapper: exactly {"value": ..., "force": <bool>}
            if map.len() == 2
                && map.contains_key("value")
                && map.get("force").is_some_and(Value::is_boolean)
            {
                // Recurse into the inner value in case it's also a Map
                // with nested wrapper-shaped leaves.
                return normalize_value_force(map.get("value").cloned().unwrap_or(Value::Null));
            }
            let normalized: serde_json::Map<String, Value> = map
                .into_iter()
                .map(|(k, v)| (k, normalize_value_force(v)))
                .collect();
            Value::Object(normalized)
        },
        Value::Array(arr) => Value::Array(arr.into_iter().map(normalize_value_force).collect()),
        other => other,
    }
}

#[cfg(test)]
mod load_initial_context_tests {
    use serde_json::json;

    use super::normalize_value_force;

    #[test]
    fn plain_values_pass_through() {
        let v = json!({"title": "X", "tags": ["a", "b"]});
        assert_eq!(normalize_value_force(v.clone()), v);
    }

    #[test]
    fn wrapper_with_force_true_unwraps() {
        let v = json!({"title": {"value": "X", "force": true}});
        assert_eq!(normalize_value_force(v), json!({"title": "X"}));
    }

    #[test]
    fn wrapper_with_force_false_unwraps_too() {
        // force: false is a valid wrapper shape — just means "no override"
        // (current semantics treat both as fill-gap, but the shape must
        // round-trip cleanly for downstream tooling).
        let v = json!({"title": {"value": "X", "force": false}});
        assert_eq!(normalize_value_force(v), json!({"title": "X"}));
    }

    #[test]
    fn structural_object_with_extra_keys_is_not_a_wrapper() {
        // {fn, hasEmail} has 2 keys but neither is `value`/`force`, so
        // it's a legitimate structured field, not a wrapper.
        let v = json!({"contact_point": {"fn": "Jane", "hasEmail": "j@x"}});
        assert_eq!(normalize_value_force(v.clone()), v);
    }

    #[test]
    fn object_with_value_and_force_plus_extras_is_not_a_wrapper() {
        // Only the EXACT two-key shape counts. Adding extras keeps it
        // as a structured field.
        let v = json!({
            "field": {"value": "X", "force": true, "extra": "kept"}
        });
        assert_eq!(normalize_value_force(v.clone()), v);
    }

    #[test]
    fn wrapper_with_object_value_unwraps_and_recurses() {
        let v = json!({
            "publisher": {
                "value": {"name": "Agency", "url": {"value": "https://x", "force": true}},
                "force": true
            }
        });
        // Outer wrapper unwraps to the inner object; the nested wrapper
        // inside `url` also normalizes.
        assert_eq!(
            normalize_value_force(v),
            json!({"publisher": {"name": "Agency", "url": "https://x"}})
        );
    }

    #[test]
    fn wrapper_in_array_element_unwraps() {
        let v = json!({"tags": ["a", {"value": "b", "force": true}, "c"]});
        assert_eq!(normalize_value_force(v), json!({"tags": ["a", "b", "c"]}));
    }

    // §5.4: collect_forced_dataset_info_paths.
    #[test]
    fn collect_forced_collects_only_force_true_under_dataset_info() {
        use super::collect_forced_dataset_info_paths;

        let doc = json!({
            "package": {
                // package-side force is accepted but NOT collected (no
                // CKAN→DCAT mapping yet — see §5.4 docs).
                "title": {"value": "X", "force": true}
            },
            "dataset_info": {
                "/dcat/dct:title":       {"value": "Forced", "force": true},
                "/dcat/dct:description": {"value": "Plain force=false", "force": false},
                "/dcat/dct:identifier":  "plain string",
                "/dcat/dct:rights":      {"value": null, "force": true}
            }
        });
        let mut got = collect_forced_dataset_info_paths(&doc);
        got.sort();
        assert_eq!(
            got,
            vec![
                "/dcat/dct:rights".to_string(),
                "/dcat/dct:title".to_string(),
            ],
        );
    }

    #[test]
    fn collect_forced_empty_when_no_dataset_info() {
        use super::collect_forced_dataset_info_paths;

        let doc = json!({"package": {"title": {"value": "X", "force": true}}});
        assert!(collect_forced_dataset_info_paths(&doc).is_empty());
    }

    #[test]
    fn collect_forced_empty_when_dataset_info_is_not_object() {
        use super::collect_forced_dataset_info_paths;

        // Pathological shape — must not panic.
        let doc = json!({"dataset_info": ["not", "an", "object"]});
        assert!(collect_forced_dataset_info_paths(&doc).is_empty());
    }
}
