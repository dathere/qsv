static USAGE: &str = r#"
Extract and infer DCAT-3 / Croissant metadata from a CSV, optionally driven by a
CKAN scheming YAML spec.

This is the non-interactive, qsv-native counterpart to what datapusher-plus (DP+)
does in CKAN: run statistical + frequency analysis on the input, build a Jinja2
context (`package`, `resource`, `dpps`, `dppf`, `dpp`), then evaluate every
`formula` / `suggestion_formula` field declared in the scheming YAML. The
resulting `.metadata.json` carries both a CKAN-shaped block and a best-effort
DCAT-US v3 projection, ready for qsv pro and DP+ to prepopulate CKAN packages.

Helpers and filters are a native Rust port of DP+'s `jinja2_helpers.py`,
built on `minijinja`. No Python interpreter is required at runtime; the
SQL-requiring helpers (`temporal_resolution`, `guess_accrual_periodicity`)
query the input CSV directly via Polars SQL.

For an example spec file, see:
  https://github.com/dathere/datapusher-plus/blob/main/ckanext/datapusher_plus/dataset-druf.yaml

For more extensive examples, see https://github.com/dathere/qsv/blob/master/tests/test_profile.rs.

Usage:
    qsv profile [options] [<input>]
    qsv profile --help

profile options:
    --spec <yaml>             CKAN scheming YAML spec file. If omitted, only the
                              inferred `dpp` block (lat/lon/date columns, dataset
                              stats) is emitted; no formulas are evaluated.
    --initial-context <json>  JSON file providing seed values for the package /
                              resource dicts plus optional JSON-Pointer
                              overrides for the final DCAT block. Replaces
                              the older --package-meta / --resource-meta
                              flags. Shape:
                                {
                                  "package":  {"title": "...", ...},
                                  "resource": {"format": "CSV", ...},
                                  "dataset_info": {
                                    "/dcat/dct:title": "Force override"
                                  }
                                }
                              Each leaf value may also be wrapped as
                              {"value": ..., "force": true} to mark it
                              as overriding any value discovered from
                              the URL's existing DCAT markup. Phase 4a
                              ships the flag + dataset_info overrides;
                              per-property force semantics land in 4b.
    --no-dcat                 Skip the DCAT-US v3 projection block.
    --no-ckan                 Skip the CKAN-shape block.
    --dcat-legacy-license     Transitional: re-emit dct:license on the
                              Dataset alongside the v3-required
                              Distribution-level copy. Default: off
                              (strict v3, license on Distribution only).
    --no-dcat-discovery       Skip DCAT-markup discovery on URL inputs.
                              Discovery sniffs HTTP Link: rel=describedBy
                              (and, in future, sibling .metadata.json /
                              JSON-LD <script> blocks) to use the
                              publisher's stated metadata as a base layer.
    --dcat-discovery-timeout <secs>  Per-request timeout for DCAT-markup
                              discovery probes. Default: 5.
    --validate-dcat           Validate the emitted dcat block against the
                              embedded minimal DCAT-US v3 schema (covers
                              the mandatory fields). Violations append to
                              dcat_warnings by default.
    --strict-dcat             With --validate-dcat, fail the command on
                              any schema violation instead of warning.
    --force                   Force recomputing cardinality and unique values
                              even if a stats cache file exists.
    -j, --jobs <arg>          The number of jobs to run in parallel for the
                              underlying stats/frequency passes. When not set,
                              the number of jobs is set to the number of CPUs
                              detected.
    -o, --output <file>       Output JSON path. Default: <input>.metadata.json.

Common options:
    -h, --help                Display this message
    -n, --no-headers          When set, the first row will not be interpreted
                              as headers. Namely, it will be processed with the
                              rest of the rows. Otherwise, the first row will
                              always appear as the header row in the output.
    -d, --delimiter <arg>     The field delimiter for reading CSV data.
                              Must be a single character.
    --memcheck                Check if there is enough memory to load the entire
                              CSV into memory using CONSERVATIVE heuristics.
"#;

use std::path::Path;

use serde::Deserialize;
use serde_json::{Value, json};

use crate::{CliError, CliResult, util};

mod context;
mod dcat;
mod dcat_discover;
mod dcat_validate;
mod formula_engine;
mod formula_helpers;
mod spec;
mod sql_backend;

#[derive(Debug, Deserialize)]
struct Args {
    arg_input:                   Option<String>,
    flag_spec:                   Option<String>,
    flag_initial_context:        Option<String>,
    flag_no_dcat:                bool,
    flag_dcat_legacy_license:    bool,
    flag_no_dcat_discovery:      bool,
    flag_dcat_discovery_timeout: Option<u64>,
    flag_validate_dcat:          bool,
    flag_strict_dcat:            bool,
    flag_no_ckan:                bool,
    flag_force:                  bool,
    flag_jobs:                   Option<usize>,
    flag_output:                 Option<String>,
    flag_no_headers:             bool,
    flag_delimiter:              Option<crate::config::Delimiter>,
    flag_memcheck:               bool,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    // For v1 we require a real input file path — running stats + frequency in
    // subprocess form against stdin would require materializing it to a
    // tempfile, and that's a follow-up.
    let raw_input = match args.arg_input.as_deref() {
        Some("-") | None => {
            return Err(CliError::Other(
                "qsv profile requires an input file path; reading from stdin is not yet supported."
                    .into(),
            ));
        },
        Some(p) => p.to_string(),
    };

    // URL inputs are downloaded to a tempfile so the rest of the pipeline
    // (stats, frequency, sqlp-backed helpers) sees a normal file path. The
    // tempfile must outlive `run`'s body — we bind it to a local variable
    // (`_downloaded_temp`) so its Drop runs only at function exit. The
    // original URL is preserved separately so the DCAT projection can use
    // it as `dcat:downloadURL`.
    let (input_path, original_url, _downloaded_temp) = resolve_input(&raw_input)?;

    // URL-only: best-effort DCAT-markup discovery. Stored under
    // `dcat_discovered` in the output JSON for now; the merge with the
    // auto-inferred projection lands in Phase 4 alongside
    // `--initial-context`.
    let discovered_dcat: Option<Value> = match original_url.as_deref() {
        Some(url) if !args.flag_no_dcat_discovery => {
            let timeout =
                std::time::Duration::from_secs(args.flag_dcat_discovery_timeout.unwrap_or(5));
            dcat_discover::discover(url, timeout)
        },
        _ => None,
    };

    // --- 1. parse spec (optional) -----------------------------------------
    let spec_opt = match args.flag_spec.as_deref() {
        Some(p) => Some(spec::load_from_path(p)?),
        None => None,
    };

    // --- 2. run stats + frequency, build analysis context -----------------
    let ctx_args = context::ContextArgs {
        input_path:      &input_path,
        no_headers:      args.flag_no_headers,
        delimiter:       args.flag_delimiter,
        jobs:            args.flag_jobs,
        force:           args.flag_force,
        memcheck:        args.flag_memcheck,
        initial_context: args.flag_initial_context.as_deref(),
    };
    let analysis = context::build(&ctx_args, spec_opt.as_ref())?;

    // --- 3. formula evaluation (minijinja, native Rust) -----------------
    // When a spec is provided, evaluate every `formula` / `suggestion_formula`
    // template against the analysis context. Helpers are the Rust port of
    // DP+'s `jinja2_helpers.py` (see `formula_helpers.rs`).
    let formula_results = match spec_opt.as_ref() {
        Some(spec) => {
            // Build a SQL backend honoring the same CSV parsing options
            // as the rest of the profile pipeline so SQL-backed helpers
            // (temporal_resolution, guess_accrual_periodicity) see the
            // same columns stats/frequency saw. --delimiter overrides
            // Polars' default comma; --no-headers maps to has_header=false.
            let sql_backend = sql_backend::SqlBackend::new(ctx_args.input_path)
                .with_delimiter(ctx_args.delimiter.map(|d| d.0).unwrap_or(b','))
                .with_has_header(!ctx_args.no_headers);
            formula_engine::evaluate_spec(spec, &analysis.context, Some(sql_backend))?
        },
        None => Vec::new(),
    };
    let formulas_evaluated = !formula_results.is_empty();

    // --- 4. assemble output ----------------------------------------------
    let mut output = json!({
        "qsv_version":      env!("CARGO_PKG_VERSION"),
        "generated_at":     chrono::Utc::now().to_rfc3339(),
        "spec_file":        args.flag_spec.clone(),
        "input":            input_path,
        "formulas_evaluated": formulas_evaluated,
    });
    let out_map = output.as_object_mut().unwrap();

    // dpp block (inferred metadata, stats, frequency) is always emitted.
    out_map.insert(
        "dpp".to_string(),
        analysis.context.get("dpp").cloned().unwrap_or(json!({})),
    );
    out_map.insert(
        "stats".to_string(),
        analysis.context.get("dpps").cloned().unwrap_or(json!({})),
    );
    out_map.insert(
        "frequency".to_string(),
        analysis.context.get("dppf").cloned().unwrap_or(json!({})),
    );

    // Build the merged package/resource once so both --no-ckan and --no-dcat
    // share the same post-formula state.
    let mut package = analysis
        .context
        .get("package")
        .cloned()
        .unwrap_or(json!({}));
    let mut resource = analysis
        .context
        .get("resource")
        .cloned()
        .unwrap_or(json!({}));
    if let (Some(pkg), Some(spec)) = (package.as_object_mut(), spec_opt.as_ref()) {
        if let Some(v) = spec.scheming_version {
            pkg.entry("scheming_version").or_insert_with(|| json!(v));
        }
        if let Some(dt) = spec.dataset_type.as_deref() {
            pkg.entry("dataset_type")
                .or_insert_with(|| Value::String(dt.to_string()));
        }
    }
    merge_formula_results(&mut package, &mut resource, &formula_results);

    // When the input was a URL, stamp it as the resource URL so the DCAT
    // projection's `dcat:downloadURL` slot gets populated (subject to the
    // existing absolute-IRI check). Don't overwrite an explicit
    // resource.url already supplied via formulas / seed metadata. Same
    // pattern for package.title / resource.name: when the user hasn't
    // supplied them, derive defaults from the URL basename so the DCAT
    // title slot doesn't surface the random tempfile suffix.
    if let Some(url) = original_url.as_ref() {
        if let Some(res_obj) = resource.as_object_mut() {
            res_obj
                .entry("url".to_string())
                .or_insert_with(|| Value::String(url.clone()));
        }
        if let Some(url_title) = url_title_default(url) {
            // package.title is read by add_core_identity and not touched
            // by context::build — a simple .entry().or_insert() suffices.
            if let Some(pkg_obj) = package.as_object_mut() {
                pkg_obj
                    .entry("title".to_string())
                    .or_insert_with(|| Value::String(url_title.clone()));
            }
            // resource.name is already seeded by context::build from the
            // tempfile path stem before we get here. Replace that default
            // with the URL basename, but leave a real user-supplied value
            // (via --initial-context or formulas) alone — distinguish by
            // checking whether the current value matches the tempfile
            // stem that context::build would have produced.
            if let Some(res_obj) = resource.as_object_mut() {
                let tempfile_stem = Path::new(&input_path)
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .map(str::to_string);
                let current = res_obj
                    .get("name")
                    .and_then(|v| v.as_str())
                    .map(str::to_string);
                if current.is_none() || current == tempfile_stem {
                    res_obj.insert("name".to_string(), Value::String(url_title));
                }
            }
        }
    }

    if !args.flag_no_ckan {
        out_map.insert(
            "ckan".to_string(),
            json!({
                "package":   package.clone(),
                "resources": [resource.clone()],
            }),
        );
    }

    // Always expose the raw per-formula results (including any error strings)
    // so users can debug formula failures and inspect computed values out of
    // band of the merged CKAN block.
    out_map.insert(
        "formula_results".to_string(),
        serde_json::to_value(&formula_results).unwrap_or(json!([])),
    );

    if !args.flag_no_dcat {
        let dpp = analysis.context.get("dpp").cloned().unwrap_or(json!({}));
        let stats = analysis.context.get("dpps").cloned().unwrap_or(json!({}));
        let (dcat_block, dcat_build_warnings) = dcat::build(
            &package,
            &[resource.clone()],
            &dpp,
            &stats,
            &input_path,
            args.flag_dcat_legacy_license,
        );
        let merged_dcat = match discovered_dcat.as_ref() {
            Some(disc) => merge_discovered(dcat_block, disc),
            None => dcat_block,
        };
        out_map.insert("dcat".to_string(), merged_dcat);
        // Stash the build-time warnings for now; we'll insert them
        // after dataset_info overrides + schema validation have had
        // their say so the final dcat_warnings array reflects the
        // emitted dcat block, not an intermediate snapshot.
        out_map.insert(
            "__pending_dcat_warnings".to_string(),
            serde_json::to_value(&dcat_build_warnings).unwrap_or(json!([])),
        );
        // Surface the raw discovered DCAT alongside the merged block so
        // downstream tooling can diff or audit what came from the
        // publisher vs what qsv inferred.
        if let Some(d) = discovered_dcat {
            out_map.insert("dcat_discovered".to_string(), d);
        }
    }

    // --initial-context's `dataset_info` JSON-Pointer overrides are the
    // final escape hatch — applied unconditionally over inference,
    // discovery, the CKAN block, and formula output. Must run BEFORE
    // schema validation so an override that supplies a missing
    // mandatory field doesn't trip --strict-dcat. Per plan §4c.
    if let Some(overrides) = analysis.dataset_info.as_object()
        && !overrides.is_empty()
    {
        apply_pointer_overrides(&mut output, overrides);
    }

    // Phase 6 (post-override): JSON Schema validation runs on the
    // emitted dcat block, after dataset_info overrides have applied.
    // Pulls the stashed build-time warnings back out, drops any whose
    // referenced field is now present in the final dcat block (the
    // dataset_info override or discovered-DCAT merge satisfied them),
    // then merges schema violations into the final dcat_warnings array.
    if !args.flag_no_dcat {
        let out_map = output.as_object_mut().unwrap();
        let stashed: Vec<dcat::DcatWarning> = out_map
            .remove("__pending_dcat_warnings")
            .and_then(|v| serde_json::from_value(v).ok())
            .unwrap_or_default();
        let final_dcat_snapshot = out_map.get("dcat").cloned();
        let mut dcat_warnings: Vec<dcat::DcatWarning> = stashed
            .into_iter()
            .filter(|w| !final_dcat_has_field(final_dcat_snapshot.as_ref(), &w.field))
            .collect();
        if args.flag_validate_dcat
            && let Some(final_dcat) = out_map.get("dcat")
        {
            let validation = dcat_validate::validate_dataset(final_dcat);
            if !validation.is_empty() && args.flag_strict_dcat {
                let summary = validation
                    .iter()
                    .map(|w| format!("  - {}: {}", w.field, w.message))
                    .collect::<Vec<_>>()
                    .join("\n");
                return Err(CliError::Other(format!(
                    "qsv profile --strict-dcat: {} schema violation(s):\n{summary}",
                    validation.len()
                )));
            }
            dcat_warnings.extend(validation);
        }
        if !dcat_warnings.is_empty() {
            out_map.insert(
                "dcat_warnings".to_string(),
                serde_json::to_value(&dcat_warnings).unwrap_or(json!([])),
            );
        }
    }

    let _ = analysis.headers;

    // --- 5. write output --------------------------------------------------
    let out_path = args.flag_output.clone().unwrap_or_else(|| {
        let stem = Path::new(&input_path)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("output");
        format!("{stem}.metadata.json")
    });
    let pretty = serde_json::to_string_pretty(&output)
        .map_err(|e| CliError::Other(format!("could not serialize metadata JSON: {e}")))?;
    std::fs::write(&out_path, pretty)
        .map_err(|e| CliError::Other(format!("could not write `{out_path}`: {e}")))?;

    eprintln!("qsv profile: wrote `{out_path}`");
    Ok(())
}

fn merge_formula_results(
    package: &mut Value,
    resource: &mut Value,
    results: &[formula_engine::FormulaResult],
) {
    if results.is_empty() {
        return;
    }

    // Pass 1: stamp `formula` results onto the package/resource fields.
    {
        let pkg = ensure_object(package);
        let res = ensure_object(resource);
        for r in results {
            if r.kind != "formula" || r.error.is_some() {
                continue;
            }
            let value = r.value.clone().unwrap_or(Value::Null);
            match r.scope.as_str() {
                "dataset" => {
                    pkg.insert(r.field_name.clone(), value);
                },
                "resource" => {
                    res.insert(r.field_name.clone(), value);
                },
                _ => {},
            }
        }
    }

    // Pass 2: collect `suggestion_formula` results under
    // package.dpp_suggestions. Done after pass 1 finishes so we hold no
    // overlapping mutable borrows of `package`.
    let mut sugg_entries: Vec<(String, Value)> = Vec::new();
    for r in results {
        if r.kind != "suggestion_formula" {
            continue;
        }
        let value = r.value.clone().unwrap_or(Value::Null);
        sugg_entries.push((
            r.field_name.clone(),
            json!({
                "value": value,
                "scope": r.scope,
                "error": r.error,
            }),
        ));
    }
    if !sugg_entries.is_empty() {
        let pkg = ensure_object(package);
        let suggestions_v = pkg
            .entry("dpp_suggestions".to_string())
            .or_insert_with(|| json!({}));
        if !suggestions_v.is_object() {
            *suggestions_v = json!({});
        }
        let suggestions = suggestions_v.as_object_mut().unwrap();
        for (k, v) in sugg_entries {
            suggestions.insert(k, v);
        }
    }
}

/// Coerce a JSON value into a mutable object, replacing it with `{}` if it
/// wasn't an object to begin with.
fn ensure_object(v: &mut Value) -> &mut serde_json::Map<String, Value> {
    if !v.is_object() {
        *v = json!({});
    }
    v.as_object_mut().unwrap()
}

/// Resolve the user-supplied input into a local file path. If the input
/// is an http(s) URL, download it to a tempfile so the rest of the
/// pipeline sees a normal file. Returns:
///   * the local file path to feed to stats/frequency/sqlp
///   * the original URL (when the input was one), for later use as `dcat:downloadURL` on the
///     Distribution
///   * a `NamedTempFile` handle that the caller must keep alive until all downstream readers have
///     finished — dropping it deletes the temp on disk
fn resolve_input(
    raw: &str,
) -> CliResult<(String, Option<String>, Option<tempfile::NamedTempFile>)> {
    if !is_http_url(raw) {
        return Ok((raw.to_string(), None, None));
    }

    use std::time::Duration;

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(120))
        .build()
        .map_err(|e| CliError::Other(format!("qsv profile: HTTP client build: {e}")))?;
    let mut response = client
        .get(raw)
        .send()
        .and_then(reqwest::blocking::Response::error_for_status)
        .map_err(|e| CliError::Other(format!("qsv profile: download {raw}: {e}")))?;

    // Preserve the URL's *path* extension on the tempfile so downstream
    // qsv code paths that sniff by extension (e.g. compressed-CSV
    // detection) keep working. Parse the URL first so query strings and
    // fragments don't pollute the suffix.
    let suffix = tempfile_suffix_for_url(raw);

    let mut temp = tempfile::Builder::new()
        .prefix("qsv-profile-")
        .suffix(&suffix)
        .tempfile()
        .map_err(|e| CliError::Other(format!("qsv profile: create tempfile: {e}")))?;

    // Stream the body straight into the tempfile rather than buffering
    // the whole response in memory (large remote CSVs would OOM).
    use std::io::Write;
    std::io::copy(&mut response, temp.as_file_mut())
        .map_err(|e| CliError::Other(format!("qsv profile: stream body from {raw}: {e}")))?;
    temp.as_file_mut().flush().ok();

    let local = temp.path().to_string_lossy().to_string();
    Ok((local, Some(raw.to_string()), Some(temp)))
}

/// Compute the tempfile suffix for a downloaded URL. Strips query
/// strings and fragments by parsing the URL, then preserves the
/// well-known CSV-family compound extensions (`.csv.gz`, `.tsv.gz`,
/// `.csv.zst`, `.tsv.zst`, `.csv.bz2`, `.tsv.bz2`) so qsv's downstream
/// extension sniffing routes compressed CSVs through the right reader.
/// Single-extension paths use the last path component's extension.
/// Falls back to `.csv` when no path or no extension is present.
fn tempfile_suffix_for_url(raw: &str) -> String {
    let path = match url::Url::parse(raw) {
        Ok(u) => u.path().to_string(),
        Err(_) => raw.to_string(),
    };
    let lower = path.to_ascii_lowercase();
    for compound in [
        ".csv.gz", ".tsv.gz", ".ssv.gz", ".csv.zst", ".tsv.zst", ".ssv.zst", ".csv.bz2",
        ".tsv.bz2", ".ssv.bz2", ".csv.xz", ".tsv.xz",
    ] {
        if lower.ends_with(compound) {
            return compound.to_string();
        }
    }
    std::path::Path::new(&path)
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| format!(".{e}"))
        .unwrap_or_else(|| ".csv".to_string())
}

/// Derive a human-readable default title from a URL when the user
/// hasn't supplied `--initial-context.package.title`. Strips the
/// URL's well-known compound extensions (`.csv.gz`, etc.) and any
/// remaining single extension, then returns the last non-empty path
/// segment. Returns `None` when the URL has no usable path (e.g. just
/// a host, or a trailing slash), in which case the caller falls back
/// to the existing tempfile-stem behaviour.
///
/// Opaque/UUID-style basenames (e.g. CKAN's
/// `/datastore/dump/<uuid>`) pass through unchanged — a UUID is still
/// reproducible and traceable back to the input, which is strictly
/// better than the random tempfile suffix `qsv-profile-XXXXXX`.
/// Users who want a prettier title should populate
/// `--initial-context.package.title` explicitly.
fn url_title_default(url: &str) -> Option<String> {
    let parsed = url::Url::parse(url).ok()?;
    let path = parsed.path();
    let cleaned = strip_compound_csv_ext(path);
    let basename = std::path::Path::new(&cleaned)
        .file_stem()
        .and_then(|s| s.to_str())
        .map(str::to_string)
        .filter(|s| !s.is_empty())?;
    Some(basename)
}

/// Strip a CSV-family compound extension off a path (mirrors
/// `tempfile_suffix_for_url`'s compound-extension list). Returns the
/// path with the compound suffix removed, or unchanged when no known
/// compound suffix is present (single extensions are stripped later
/// via `Path::file_stem`).
fn strip_compound_csv_ext(path: &str) -> String {
    let lower = path.to_ascii_lowercase();
    for compound in [
        ".csv.gz", ".tsv.gz", ".ssv.gz", ".csv.zst", ".tsv.zst", ".ssv.zst", ".csv.bz2",
        ".tsv.bz2", ".ssv.bz2", ".csv.xz", ".tsv.xz",
    ] {
        if lower.ends_with(compound) {
            return path[..path.len() - compound.len()].to_string();
        }
    }
    path.to_string()
}

/// Apply `--initial-context.dataset_info` JSON-Pointer → Value
/// overrides to the assembled output JSON. Each key in `overrides`
/// must be an RFC 6901 JSON Pointer (e.g. `/dcat/dct:title`); the
/// value replaces whatever was at that path. Missing parents are
/// created as objects on demand.
///
/// Out of scope: array-index intermediate-parent creation and
/// `-`-suffix appending. Failures (non-pointer keys, traversal through
/// non-object scalars) are silently skipped — overrides are
/// best-effort, not an enforcement mechanism.
fn apply_pointer_overrides(root: &mut Value, overrides: &serde_json::Map<String, Value>) {
    for (ptr, new_value) in overrides {
        if !ptr.starts_with('/') {
            continue;
        }
        set_by_pointer(root, ptr, new_value.clone());
    }
}

/// Merge a discovered DCAT-US v3 dataset object into our auto-inferred
/// projection per the Phase 4b precedence rules:
///
///   * Inferred values (including those seeded from `--initial-context`) always win — discovered
///     DCAT only fills slots the inferred output left absent.
///   * Top-level scalar / object keys present in `discovered` but not in `inferred` are copied over
///     verbatim.
///   * Array slots (`dct:spatial`, `dct:temporal`, `dcat:keyword`, `dcat:theme`) get the discovered
///     array only when the inferred side is missing entirely; we don't try to merge per-element.
///   * `dcat:distribution` is left alone — per-distribution merging is out of scope until we have a
///     per-resource identity scheme.
///   * `@context` and `@type` are never overwritten.
///
/// Future full-force semantics (`force: true` wrappers in
/// `--initial-context` overriding discovered) will layer a "forced
/// keys" skip-set onto this same merge function.
fn merge_discovered(inferred: Value, discovered: &Value) -> Value {
    let (Value::Object(mut inf), Some(disc)) = (inferred, discovered.as_object()) else {
        return Value::Object(serde_json::Map::new());
    };
    for (k, v) in disc {
        if k == "@context" || k == "@type" || k == "dcat:distribution" {
            continue;
        }
        if !inf.contains_key(k) {
            inf.insert(k.clone(), v.clone());
        }
    }
    Value::Object(inf)
}

/// Returns true when the final dcat block carries a non-null,
/// non-empty value for `field` (a JSON-LD key like `"dcat:contactPoint"`
/// or a nested path like `"dcat:distribution/0/dct:license"`). Used
/// to filter stale build-time warnings after `dataset_info` overrides
/// and discovered-DCAT merging have had a chance to populate slots
/// that were originally absent.
///
/// Top-level field names get a fast direct lookup; nested paths are
/// resolved via JSON Pointer (with a leading `/` added if absent).
/// Returns false for any unparseable / missing field — the safe
/// default is "keep the warning".
fn final_dcat_has_field(final_dcat: Option<&Value>, field: &str) -> bool {
    let Some(dcat) = final_dcat else {
        return false;
    };
    if field.is_empty() {
        return false;
    }
    // Top-level field name (the common case for build-time warnings).
    if !field.contains('/')
        && let Some(v) = dcat.get(field)
    {
        return !is_value_empty(v);
    }
    // Nested JSON-Pointer path.
    let pointer = if field.starts_with('/') {
        field.to_string()
    } else {
        format!("/{field}")
    };
    dcat.pointer(&pointer).is_some_and(|v| !is_value_empty(v))
}

fn is_value_empty(v: &Value) -> bool {
    match v {
        Value::Null => true,
        Value::String(s) => s.is_empty(),
        Value::Array(arr) => arr.is_empty(),
        Value::Object(map) => map.is_empty(),
        _ => false,
    }
}

fn set_by_pointer(root: &mut Value, pointer: &str, value: Value) {
    // RFC 6901: split on '/', skip the leading empty token, unescape
    // ~1 → /, ~0 → ~ on each remaining token.
    let tokens: Vec<String> = pointer
        .split('/')
        .skip(1)
        .map(|t| t.replace("~1", "/").replace("~0", "~"))
        .collect();
    if tokens.is_empty() {
        *root = value;
        return;
    }
    let mut cursor: &mut Value = root;
    for (i, tok) in tokens.iter().enumerate() {
        let is_last = i + 1 == tokens.len();
        match cursor {
            // Traverse into an existing array via numeric segment. We
            // never create or extend arrays here — that would require
            // distinguishing array-vs-object intent at every missing
            // parent, which RFC 6901 leaves to the caller. Out-of-range
            // indices and non-numeric tokens are silently skipped so a
            // typo in a single override doesn't corrupt the rest of
            // the output.
            Value::Array(arr) => {
                let Ok(idx) = tok.parse::<usize>() else {
                    return;
                };
                if idx >= arr.len() {
                    return;
                }
                if is_last {
                    arr[idx] = value;
                    return;
                }
                cursor = &mut arr[idx];
            },
            // Object (or any non-object scalar, which we replace with
            // an empty object before descending). Same behaviour as
            // before for object-typed parents.
            _ => {
                if !cursor.is_object() {
                    *cursor = json!({});
                }
                let map = cursor.as_object_mut().unwrap();
                if is_last {
                    map.insert(tok.clone(), value);
                    return;
                }
                cursor = map.entry(tok.clone()).or_insert_with(|| json!({}));
            },
        }
    }
}

fn is_http_url(s: &str) -> bool {
    let lower = s.to_ascii_lowercase();
    lower.starts_with("http://") || lower.starts_with("https://")
}

#[cfg(test)]
mod tests {
    use super::is_http_url;

    #[test]
    fn url_detection_recognizes_http_https_case_insensitive() {
        assert!(is_http_url("http://example.gov/d.csv"));
        assert!(is_http_url("https://example.gov/d.csv"));
        assert!(is_http_url("HTTPS://example.gov/d.csv"));
        assert!(is_http_url("Http://example.gov/d.csv"));
        assert!(!is_http_url("/tmp/local.csv"));
        assert!(!is_http_url("file:///tmp/x.csv"));
        assert!(!is_http_url("ftp://example.com/x.csv"));
        assert!(!is_http_url(""));
    }

    use serde_json::json;

    use super::{apply_pointer_overrides, set_by_pointer};

    #[test]
    fn pointer_overrides_set_existing_leaf() {
        let mut root = json!({"dcat": {"dct:title": "old"}});
        let overrides = json!({"/dcat/dct:title": "new"})
            .as_object()
            .unwrap()
            .clone();
        apply_pointer_overrides(&mut root, &overrides);
        assert_eq!(
            root.pointer("/dcat/dct:title").and_then(|v| v.as_str()),
            Some("new")
        );
    }

    #[test]
    fn pointer_overrides_create_missing_parents() {
        let mut root = json!({});
        let overrides = json!({"/dcat/dcat-us:bureauCode": ["015:11"]})
            .as_object()
            .unwrap()
            .clone();
        apply_pointer_overrides(&mut root, &overrides);
        assert_eq!(
            root.pointer("/dcat/dcat-us:bureauCode/0")
                .and_then(|v| v.as_str()),
            Some("015:11")
        );
    }

    #[test]
    fn pointer_overrides_skip_non_pointer_keys() {
        let mut root = json!({"x": 1});
        let overrides = json!({"no-leading-slash": "ignored"})
            .as_object()
            .unwrap()
            .clone();
        apply_pointer_overrides(&mut root, &overrides);
        // Unchanged
        assert_eq!(root, json!({"x": 1}));
    }

    #[test]
    fn pointer_handles_escape_sequences() {
        // RFC 6901: ~0 → ~ and ~1 → / in path tokens.
        let mut root = json!({});
        set_by_pointer(&mut root, "/has~1slash/has~0tilde", json!("v"));
        assert_eq!(
            root.pointer("/has~1slash/has~0tilde")
                .and_then(|v| v.as_str()),
            Some("v")
        );
    }

    use super::merge_discovered;

    #[test]
    fn merge_fills_gaps_only() {
        let inferred = json!({
            "@type": "dcat:Dataset",
            "dct:title": "Inferred Title",
        });
        let discovered = json!({
            "dct:title": "Discovered Title", // collision — inferred wins
            "dct:rights": "Discovered Rights", // gap — discovered fills
        });
        let merged = merge_discovered(inferred, &discovered);
        assert_eq!(
            merged.pointer("/dct:title").and_then(|v| v.as_str()),
            Some("Inferred Title"),
            "inferred always wins on collision"
        );
        assert_eq!(
            merged.pointer("/dct:rights").and_then(|v| v.as_str()),
            Some("Discovered Rights"),
            "discovered fills the gap"
        );
    }

    #[test]
    fn merge_never_overwrites_context_or_type() {
        let inferred = json!({
            "@context": "https://inferred",
            "@type": "dcat:Dataset",
        });
        let discovered = json!({
            "@context": "https://discovered",
            "@type": "dcat:OtherType",
        });
        let merged = merge_discovered(inferred, &discovered);
        assert_eq!(
            merged.pointer("/@context").and_then(|v| v.as_str()),
            Some("https://inferred")
        );
        assert_eq!(
            merged.pointer("/@type").and_then(|v| v.as_str()),
            Some("dcat:Dataset")
        );
    }

    #[test]
    fn merge_skips_distribution_array() {
        let inferred = json!({"dcat:distribution": [{"@type": "dcat:Distribution"}]});
        let discovered = json!({"dcat:distribution": [{"dct:title": "Discovered Dist"}]});
        let merged = merge_discovered(inferred, &discovered);
        // Distribution array is left untouched — per-resource merging
        // is out of scope until a per-distribution identity scheme exists.
        assert_eq!(
            merged
                .pointer("/dcat:distribution/0/@type")
                .and_then(|v| v.as_str()),
            Some("dcat:Distribution")
        );
        assert!(
            merged.pointer("/dcat:distribution/0/dct:title").is_none(),
            "discovered distribution must not be merged"
        );
    }

    use super::tempfile_suffix_for_url;

    #[test]
    fn url_suffix_preserves_compound_csv_extensions() {
        assert_eq!(
            tempfile_suffix_for_url("https://x.gov/data.csv.gz"),
            ".csv.gz"
        );
        assert_eq!(
            tempfile_suffix_for_url("https://x.gov/data.tsv.gz"),
            ".tsv.gz"
        );
        assert_eq!(
            tempfile_suffix_for_url("https://x.gov/data.csv.zst"),
            ".csv.zst"
        );
        assert_eq!(
            tempfile_suffix_for_url("https://x.gov/data.csv.bz2"),
            ".csv.bz2"
        );
    }

    #[test]
    fn url_suffix_strips_query_and_fragment() {
        assert_eq!(
            tempfile_suffix_for_url("https://x.gov/data.csv?token=secret"),
            ".csv",
            "query string must not bleed into the tempfile suffix"
        );
        assert_eq!(
            tempfile_suffix_for_url("https://x.gov/data.csv.gz?v=2&user=a#frag"),
            ".csv.gz"
        );
    }

    #[test]
    fn url_suffix_handles_plain_csv_and_unknown() {
        assert_eq!(tempfile_suffix_for_url("https://x.gov/data.csv"), ".csv");
        assert_eq!(tempfile_suffix_for_url("https://x.gov/data.tsv"), ".tsv");
        // No extension → fall back to .csv
        assert_eq!(tempfile_suffix_for_url("https://x.gov/export"), ".csv");
        // Malformed URL → treat the whole string as a path
        assert_eq!(tempfile_suffix_for_url("not-a-url.csv"), ".csv");
    }

    use super::url_title_default;

    #[test]
    fn url_title_strips_single_extension() {
        assert_eq!(
            url_title_default("https://example.gov/data/pittsburgh-311.csv"),
            Some("pittsburgh-311".to_string())
        );
        assert_eq!(
            url_title_default("https://x.gov/dir/sub/payments-2024.tsv"),
            Some("payments-2024".to_string())
        );
    }

    #[test]
    fn url_title_strips_compound_csv_extension() {
        assert_eq!(
            url_title_default("https://example.gov/d/snapshot.csv.gz"),
            Some("snapshot".to_string())
        );
        assert_eq!(
            url_title_default("https://example.gov/d/q3.tsv.zst"),
            Some("q3".to_string())
        );
    }

    #[test]
    fn url_title_ignores_query_and_fragment() {
        // url::Url parsing already drops these from the path, but the
        // assertion documents the intended behaviour.
        assert_eq!(
            url_title_default("https://example.gov/data.csv?token=secret&v=2#fragment"),
            Some("data".to_string())
        );
    }

    #[test]
    fn url_title_preserves_uuid_basename_unchanged() {
        // CKAN's `/datastore/dump/<uuid>` shape — uuid is still better
        // than the random tempfile suffix and is traceable to the source.
        assert_eq!(
            url_title_default(
                "https://data.wprdc.org/datastore/dump/5202679a-d243-402e-b82a-63189995a942"
            ),
            Some("5202679a-d243-402e-b82a-63189995a942".to_string())
        );
    }

    #[test]
    fn url_title_returns_none_for_host_only_url() {
        // Host-only URLs (no path) leave no basename to use; caller
        // falls back to the tempfile-stem default. Malformed URLs too.
        assert_eq!(url_title_default("https://example.gov"), None);
        assert_eq!(url_title_default("https://example.gov/"), None);
        assert_eq!(url_title_default("not a url"), None);
    }

    #[test]
    fn url_title_uses_directory_name_for_trailing_slash() {
        // A trailing-slash URL has a directory name — that's still a
        // usable title hint, better than the tempfile suffix.
        assert_eq!(
            url_title_default("https://example.gov/datasets/inventory/"),
            Some("inventory".to_string())
        );
    }

    #[test]
    fn pointer_overrides_descend_into_array_by_numeric_index() {
        // Regression for the array-corruption finding: previously this
        // would replace the distribution array with {"0": {...}}.
        let mut root = json!({
            "dcat": {
                "dcat:distribution": [
                    {"@type": "dcat:Distribution", "dct:license": "old"}
                ]
            }
        });
        let overrides = json!({"/dcat/dcat:distribution/0/dct:license": "new"})
            .as_object()
            .unwrap()
            .clone();
        apply_pointer_overrides(&mut root, &overrides);
        // Array shape must be preserved
        assert!(
            root.pointer("/dcat/dcat:distribution")
                .is_some_and(|v| v.is_array()),
            "distribution must remain an array, got: {root:#}"
        );
        assert_eq!(
            root.pointer("/dcat/dcat:distribution/0/dct:license")
                .and_then(|v| v.as_str()),
            Some("new")
        );
    }

    #[test]
    fn pointer_overrides_skip_out_of_range_array_index() {
        let mut root = json!({"arr": [1, 2, 3]});
        let overrides = json!({"/arr/99": 42}).as_object().unwrap().clone();
        apply_pointer_overrides(&mut root, &overrides);
        // Out-of-range index — silently skipped, array unchanged
        assert_eq!(root, json!({"arr": [1, 2, 3]}));
    }

    #[test]
    fn pointer_overrides_skip_non_numeric_array_token() {
        let mut root = json!({"arr": [{"k": "v"}]});
        let overrides = json!({"/arr/foo": "bar"}).as_object().unwrap().clone();
        apply_pointer_overrides(&mut root, &overrides);
        // Non-numeric token while traversing an array → silently skipped
        assert_eq!(root, json!({"arr": [{"k": "v"}]}));
    }
}
