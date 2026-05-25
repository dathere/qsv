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
            let csv_path = Some(Path::new(ctx_args.input_path));
            formula_engine::evaluate_spec(spec, &analysis.context, csv_path)?
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
    // resource.url already supplied via formulas / seed metadata.
    if let Some(url) = original_url.as_ref()
        && let Some(res_obj) = resource.as_object_mut()
    {
        res_obj
            .entry("url".to_string())
            .or_insert_with(|| Value::String(url.clone()));
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
        let (dcat_block, dcat_warnings) = dcat::build(
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
        if !dcat_warnings.is_empty() {
            out_map.insert(
                "dcat_warnings".to_string(),
                serde_json::to_value(&dcat_warnings).unwrap_or(json!([])),
            );
        }
        // Surface the raw discovered DCAT alongside the merged block so
        // downstream tooling can diff or audit what came from the
        // publisher vs what qsv inferred.
        if let Some(d) = discovered_dcat {
            out_map.insert("dcat_discovered".to_string(), d);
        }
    }

    // Phase 4a: --initial-context's `dataset_info` JSON-Pointer
    // overrides are the final escape hatch — applied last so they win
    // unconditionally over everything else (inference, discovery, the
    // CKAN block, formula output). Per the plan §4c precedence table.
    if let Some(overrides) = analysis.dataset_info.as_object()
        && !overrides.is_empty()
    {
        apply_pointer_overrides(&mut output, overrides);
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

    use std::{io::Write, time::Duration};

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(120))
        .build()
        .map_err(|e| CliError::Other(format!("qsv profile: HTTP client build: {e}")))?;
    let response = client
        .get(raw)
        .send()
        .and_then(reqwest::blocking::Response::error_for_status)
        .map_err(|e| CliError::Other(format!("qsv profile: download {raw}: {e}")))?;
    let body = response
        .bytes()
        .map_err(|e| CliError::Other(format!("qsv profile: read body from {raw}: {e}")))?;

    // Preserve the URL's file extension (when present) on the tempfile —
    // some downstream qsv code paths sniff by extension.
    let suffix = std::path::Path::new(raw)
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| format!(".{e}"))
        .unwrap_or_else(|| ".csv".to_string());

    let mut temp = tempfile::Builder::new()
        .prefix("qsv-profile-")
        .suffix(&suffix)
        .tempfile()
        .map_err(|e| CliError::Other(format!("qsv profile: create tempfile: {e}")))?;
    temp.write_all(&body)
        .map_err(|e| CliError::Other(format!("qsv profile: write tempfile: {e}")))?;
    temp.flush().ok();

    let local = temp.path().to_string_lossy().to_string();
    Ok((local, Some(raw.to_string()), Some(temp)))
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
        if !cursor.is_object() {
            *cursor = json!({});
        }
        let map = cursor.as_object_mut().unwrap();
        if is_last {
            map.insert(tok.clone(), value);
            return;
        }
        cursor = map.entry(tok.clone()).or_insert_with(|| json!({}));
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
}
