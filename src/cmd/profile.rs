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
    --package-meta <json>     Optional JSON file with seed package fields (title,
                              owner_org, etc.) merged into the formula context
                              before evaluation.
    --resource-meta <json>    Same, for the resource dict.
    --no-dcat                 Skip the DCAT-US v3 projection block.
    --no-ckan                 Skip the CKAN-shape block.
    --dcat-legacy-license     Transitional: re-emit dct:license on the
                              Dataset alongside the v3-required
                              Distribution-level copy. Default: off
                              (strict v3, license on Distribution only).
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
mod formula_engine;
mod formula_helpers;
mod spec;
mod sql_backend;

#[derive(Debug, Deserialize)]
struct Args {
    arg_input:                Option<String>,
    flag_spec:                Option<String>,
    flag_package_meta:        Option<String>,
    flag_resource_meta:       Option<String>,
    flag_no_dcat:             bool,
    flag_dcat_legacy_license: bool,
    flag_no_ckan:             bool,
    flag_force:               bool,
    flag_jobs:                Option<usize>,
    flag_output:              Option<String>,
    flag_no_headers:          bool,
    flag_delimiter:           Option<crate::config::Delimiter>,
    flag_memcheck:            bool,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    // For v1 we require a real input file path — running stats + frequency in
    // subprocess form against stdin would require materializing it to a
    // tempfile, and that's a follow-up.
    let input_path = match args.arg_input.as_deref() {
        Some("-") | None => {
            return Err(CliError::Other(
                "qsv profile requires an input file path; reading from stdin is not yet supported."
                    .into(),
            ));
        },
        Some(p) => p.to_string(),
    };

    // --- 1. parse spec (optional) -----------------------------------------
    let spec_opt = match args.flag_spec.as_deref() {
        Some(p) => Some(spec::load_from_path(p)?),
        None => None,
    };

    // --- 2. run stats + frequency, build analysis context -----------------
    let ctx_args = context::ContextArgs {
        input_path:    &input_path,
        no_headers:    args.flag_no_headers,
        delimiter:     args.flag_delimiter,
        jobs:          args.flag_jobs,
        force:         args.flag_force,
        memcheck:      args.flag_memcheck,
        package_meta:  args.flag_package_meta.as_deref(),
        resource_meta: args.flag_resource_meta.as_deref(),
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
        let dcat_block = dcat::build(
            &package,
            &[resource.clone()],
            &dpp,
            &stats,
            &input_path,
            args.flag_dcat_legacy_license,
        );
        out_map.insert("dcat".to_string(), dcat_block);
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
