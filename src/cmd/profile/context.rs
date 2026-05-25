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
    pub input_path:    &'a str,
    pub no_headers:    bool,
    pub delimiter:     Option<Delimiter>,
    pub jobs:          Option<usize>,
    pub force:         bool,
    pub memcheck:      bool,
    pub package_meta:  Option<&'a str>,
    pub resource_meta: Option<&'a str>,
}

/// Result of the analysis pass — the JSON context plus the column headers we
/// extracted along the way (used by the output module).
pub struct AnalysisContext {
    pub context: Value,
    pub headers: Vec<String>,
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
    let freq_args: Vec<&str> = vec!["--limit", "25", "--no-other", "--no-nulls"];
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

    let row_count = count_rows(args.input_path)?;
    let size_bytes = std::fs::metadata(args.input_path)
        .map(|m| m.len())
        .unwrap_or(0);

    let dpp = json!({
        "LAT_FIELD":           lat_field,
        "LON_FIELD":           lon_field,
        "NO_LAT_LON_FIELDS":   lat_field.is_none() && lon_field.is_none(),
        "DATE_FIELDS":         date_fields,
        "NO_DATE_FIELDS":      date_fields.is_empty(),
        "DATETIME_FIELDS":     datetime_fields,
        "NO_DATETIME_FIELDS":  datetime_fields.is_empty(),
        "dataset_stats": {
            "row_count":    row_count,
            "column_count": headers.len(),
            "size_bytes":   size_bytes,
        },
    });

    // --- 6. package / resource seed dicts ---------------------------------
    let package = load_seed_json(args.package_meta)?;
    let mut resource = load_seed_json(args.resource_meta)?;
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

    Ok(AnalysisContext { context, headers })
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
fn count_rows(input_path: &str) -> CliResult<u64> {
    let (stdout, _stderr) =
        match util::run_qsv_cmd("count", &[], input_path, "qsv profile: ran `count`") {
            Ok(t) => t,
            Err(_) => return Ok(0),
        };
    Ok(stdout.trim().parse::<u64>().unwrap_or(0))
}

/// Load a seed JSON file (--package-meta / --resource-meta). Returns
/// `json!({})` when the flag is unset.
fn load_seed_json(path: Option<&str>) -> CliResult<Value> {
    let Some(path) = path else {
        return Ok(json!({}));
    };
    let raw = std::fs::read_to_string(path)
        .map_err(|e| CliError::Other(format!("could not read seed meta file `{path}`: {e}")))?;
    serde_json::from_str::<Value>(&raw).map_err(|e| {
        CliError::Other(format!(
            "could not parse seed meta file `{path}` as JSON: {e}"
        ))
    })
}
