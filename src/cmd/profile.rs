pub(crate) static USAGE: &str = r#"
Profile a CSV (local path or URL) and emit a `.metadata.json` file carrying
five top-level blocks:

  `dpp`        — inferred dataset signals: lat/lon/date columns, file size,
                 row count, encoding, etc. (the legacy datapusher-plus
                 inference block).
  `stats`      — per-column summary statistics from `qsv stats`.
  `frequency`  — per-column value counts from `qsv frequency`.
  `ckan`       — a CKAN-shaped block (package + resources) that
                 datapusher-plus consumes to prepopulate CKAN packages.
  `projection` — the dataset re-expressed in the active profile's metadata
                 vocabulary. Default is DCAT-US v3, emitted in its canonical
                 form: plain JSON with unprefixed keys and no `@context`,
                 validating against the schemas data.gov's own validator at
                 https://harvest.data.gov/validate/ runs. Bundled alternates are
                 dcat-ap-v3 (EU portals), croissant (ML/AI registries) and
                 geoconnex (water-data federations). Consumable directly by
                 data.gov harvesters, EU DCAT-AP catalogs, mlcommons /
                 Hugging Face / Kaggle, and Internet of Water tooling.

Behind the scenes qsv runs the same statistical + frequency analysis
datapusher-plus (DP+) runs in CKAN, builds a Jinja2 evaluation context from
the results, and — when an optional CKAN scheming YAML spec is supplied —
evaluates the spec's `formula` / `suggestion_formula` templates against that
context. Jinja2 helpers and filters are a native Rust port of DP+'s
`jinja2_helpers.py`, built on `minijinja`.

For DCAT-US v3 the projection also COMPUTES properties that are tedious to
author by hand, from the same statistics: temporal coverage and resolution
(an exact xsd:duration: a regular series' step, else recording precision),
spatial bounding box (excluding out-of-range and (0, 0) placeholder
coordinates) and resolution, an update schedule (only for
a regular calendar series), the content language, DQV quality measurements
(completeness, plus duplicate rows and CSV/UTF-8 validity with `--validate`),
and a CSVW table schema with per-column constraints (required, ranges,
lengths, enumerations, primary key). Values
that are generated or detected rather than measured - a description, the
language, the schedule - form an "inferred" tier that the publisher's own
metadata replaces; `projection_provenance` lists them. A generated
description carries a disclosure and a review warning, so it never passes
for a human-authored one.

When the input is a URL whose response carries DCAT markup (HTTP
`Link: rel=describedBy`), qsv discovers the publisher's stated metadata and
merges it as a base layer beneath the inferred projection.

For an example CKAN scheming YAML spec, see:
  https://github.com/dathere/datapusher-plus/blob/main/ckanext/datapusher_plus/dataset-druf.yaml

For more extensive examples, see https://github.com/dathere/qsv/blob/master/tests/test_profile.rs.
See also https://github.com/dathere/qsv/wiki/Metadata-Profiling

Examples:

  # Quick: dpp/stats/frequency + default DCAT-US v3 projection.
  $ qsv profile data.csv

  # Pipe stdin; output defaults to `stdin.metadata.json`.
  $ cat data.csv | qsv profile

  # URL input: discover the publisher's DCAT markup and merge it as a base layer.
  $ qsv profile https://data.example.gov/datasets/sample.csv

  # Seed publisher/contact info from a JSON file; write to a chosen output path.
  $ qsv profile data.csv --initial-context publisher.json -o data.metadata.json

  # data.gov-style harvest: validate against DCAT-US v3 JSON Schema, abort on
  # mandatory-field violations, wrap in a Catalog envelope.
  $ qsv profile data.csv --validate --strict --catalog -o data.metadata.json

  # DCAT-AP v3 for EU data portals; `pyshacl` validates the bundled SHACL shapes.
  $ qsv profile open-data.csv --profile dcat-ap-v3 --validate --strict

  # Croissant JSON-LD for an ML dataset; `mlcroissant` validates the output.
  $ qsv profile train.csv --profile croissant --validate -o train.croissant.json

  # Geoconnex JSON-LD for hydrologic data (qsv built with the `geoconnex` feature).
  $ qsv profile gages.csv --profile geoconnex --validate --strict

  # Evaluate a CKAN scheming spec: Jinja2 formulas compute spatial/temporal
  # extents, accrual periodicity, and other derived fields.
  $ qsv profile data.csv --spec dataset-druf.yaml -o data.metadata.json

  # CKAN-only output: drop the `projection` block, keep dpp/stats/frequency/ckan.
  $ qsv profile data.csv --no-projection --spec dataset-druf.yaml

  # Custom YAML profile from disk (embedded names always win over same-named
  # files, so use a non-clashing name for custom profiles).
  $ qsv profile data.csv --profile ./my-org-dcat.yaml --validate

Usage:
    qsv profile [options] [<input>]
    qsv profile --help

profile argument:
    <input>                   Path or URL to the CSV to profile. When `-` or
                              omitted, reads from stdin.
                              When the URL has DCAT markup, qsv will attempt to
                              discover and ingest it as a base layer of metadata
                              (unless --no-dcat-discovery is set). See --no-dcat-discovery
                              and --dcat-discovery-timeout for details and opt-out.

profile options:
    --spec <yaml>             CKAN scheming YAML spec file. If omitted, only the
                              inferred `dpp` block (lat/lon/date columns, dataset
                              stats) is emitted; no formulas are evaluated.
    --initial-context <json>  JSON file providing seed values for the package /
                              resource dicts plus optional JSON-Pointer
                              overrides for the final projection block. Replaces
                              the older --package-meta / --resource-meta
                              flags. Top-level keys: `package`, `resource`,
                              `dataset_info`. Each leaf value may be wrapped
                              as {"value": ..., "force": true} to mark it as
                              overriding any value discovered from URL DCAT
                              markup AND any value qsv inferred. Force is
                              honored across all three subtrees: dataset_info
                              entries override their target path verbatim;
                              package / resource entries route through the
                              active profile's `field_mappings:` table (e.g.
                              `package.title force=true` lands at
                              `/projection/title`, beating inference and
                              discovery). Forced values for slots the profile
                              does not surface are silently dropped (no-op).
                              See tests/resources/profile/dcat-init-context.README.md
                              for a fully-populated example.
    --no-projection           Skip the metadata projection block (dcat/croissant/
                              geoconnex, depending on the active profile).
    --no-ckan                 Skip the CKAN-shape block.
    --croissant-frequency     Embed per-column value-frequency distributions in
                              the metadata projection. The croissant profile
                              renders them as inline cr:RecordSets (one
                              `<col>-frequency` RecordSet of {value, count,
                              percentage} rows per column), per the spec's
                              "distribution of values is a statistic on the
                              field" guidance. Off by default (keeps the
                              projection compact); the raw counts always remain
                              in the top-level `frequency` block regardless.
                              Other bundled profiles ignore this flag.
    --locate <mode>           Name the places the data covers, as DCAT-US v3
                              `spatial` Locations, by looking up the densest
                              coordinate cells (up to 40, row-weighted) with
                              `qsv geocode`. <mode> is one of:
                                geonames - label the bounding box with the
                                           place, admin1 or country that 80%
                                           of the sampled rows share (offline
                                           Geonames index).
                                census   - add each US state, and each county
                                           holding at least 5% of the sampled
                                           rows, keyed by Census GEOID (US
                                           Census Geocoder: online, keyless,
                                           cached). Without coordinates, a
                                           state column (USPS, FIPS or name)
                                           is used instead.
                                both     - geonames and census.
                              Also adds the coordinates' centroid. A lookup
                              that cannot run is noted in projection_warnings;
                              the bounding box is kept. Default: off.
    --describegpt <json>      Use a `qsv describegpt --format json` result as
                              LLM-inferred metadata (see also --infer). Its
                              Description becomes the description, its Tags
                              the keywords and its Dictionary labels and
                              descriptions annotate the CSVW columns - all as
                              inferred values, which supplied or discovered
                              metadata replaces. describegpt's attribution
                              footer moves to the `llm_attribution` block and
                              the description gets a one-line disclosure.
    --infer                   Run `qsv describegpt --all` on the input and use
                              its result as with --describegpt. The LLM is
                              configured through describegpt's QSV_LLM_*
                              environment variables. If it cannot run, a
                              warning is added and the statistics-based
                              description is used.
    --tag-vocab <file>        With --describegpt or --infer: a `tag,description`
                              CSV (as for describegpt). Only tags in it are
                              kept - describegpt only asks the LLM to keep to
                              the vocabulary - and they also become `theme`
                              Concepts carrying the vocabulary's description.
    --dcat-legacy-license     Also emit `license` on the Dataset, alongside
                              the Distribution-level copy. Both are valid
                              DCAT-US v3 (the Dataset-level property was
                              added upstream), but the migration guide
                              directs publishers to put license on each
                              Distribution, so that is what qsv emits by
                              default. Default: off.
    --no-dcat-discovery       Skip DCAT-markup discovery on URL inputs.
                              Discovery sniffs HTTP Link: rel=describedBy
                              (and, in future, sibling .metadata.json /
                              JSON-LD <script> blocks) to use the
                              publisher's stated metadata as a base layer.
    --dcat-discovery-timeout <secs>  Per-request timeout for DCAT-markup
                              discovery probes. Default: 5.
    --validate                Validate the emitted projection block against
                              the active profile's declared validators. For
                              dcat-us-v3 that's the vendored GSA JSON Schema
                              bundle (see resources/dcat-us-v3/); for
                              dcat-ap-v3 / geoconnex it's pyshacl over the
                              bundled SHACL shapes; for croissant it's
                              mlcroissant. Catches missing mandatory fields,
                              cardinality issues, and shape violations.
                              Violations append to projection_warnings by
                              default. Also runs `qsv validate` over the
                              input (CSV structure + UTF-8), so a passing
                              file is declared UTF-8 encoded and its
                              syntactic validity is recorded as a
                              quality measurement.
    --strict                  With --validate, fail the command on
                              Required-severity schema violations, or on
                              non-Info external-validator findings,
                              instead of just warning. Severity comes
                              from the DCAT-US v3 schema's own
                              requirementLevel annotations, so a missing
                              mandatory property aborts while a
                              recommended-field advisory does not.
                              Note:
                              RFC4180 structural failures from
                              `qsv validate` (emitted when a spec
                              declares `validators`) are always
                              appended as warnings, regardless of this
                              flag.
    --allow-external-validator
                              Opt in to spawning the validator binary
                              declared by `validation.external` when the
                              profile was loaded from an arbitrary YAML
                              file. Bundled profiles (dcat-us-v3,
                              dcat-ap-v3, croissant, geoconnex) always run
                              their declared external validators because
                              the profile content is vetted at qsv release
                              time. Without this flag, file-loaded
                              profiles emit a Recommended-severity
                              warning instead of running the binary, so
                              an untrusted YAML can't silently execute
                              arbitrary commands. Default: off.
    --catalog                 Wrap the emitted DCAT-US v3 Dataset inside a
                              Catalog envelope (Catalog{dataset:[...]}).
                              Useful for federation harvesters (data.gov,
                              CKAN ingest) that expect Catalog-shaped
                              top-level metadata. Default: off
                              (Dataset-only, backwards-compatible).
    --profile <name|path>     Metadata projection profile to use. Embedded
                              names: dcat-us-v3 (default), dcat-ap-v3,
                              croissant; geoconnex (when built with the
                              `geoconnex` feature — qsv default; qsvdp
                              opt-in via -F datapusher_plus,geoconnex).
                              A path to a custom YAML profile is also
                              accepted; embedded names always win over
                              same-named files. See
                              resources/profiles/README.md for the schema
                              and authoring guide.
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
use serde_json::{Map, Value, json};

use crate::{CliError, CliResult, config::Config, util};

mod context;
mod dcat_discover;
mod dcat_validate;
mod derive;
mod discovery_merge;
mod external_validate;
mod formula_engine;
mod formula_helpers;
mod llm;
mod locate;
mod profile_spec;
mod projection;
mod spec;
mod sql_backend;

#[derive(Debug, Deserialize)]
struct Args {
    arg_input:                     Option<String>,
    flag_spec:                     Option<String>,
    flag_initial_context:          Option<String>,
    flag_no_projection:            bool,
    flag_dcat_legacy_license:      bool,
    flag_no_dcat_discovery:        bool,
    flag_dcat_discovery_timeout:   Option<u64>,
    flag_validate:                 bool,
    flag_strict:                   bool,
    flag_allow_external_validator: bool,
    flag_catalog:                  bool,
    flag_profile:                  Option<String>,
    flag_no_ckan:                  bool,
    flag_croissant_frequency:      bool,
    flag_locate:                   Option<String>,
    flag_describegpt:              Option<String>,
    flag_infer:                    bool,
    flag_tag_vocab:                Option<String>,
    flag_force:                    bool,
    flag_jobs:                     Option<usize>,
    flag_output:                   Option<String>,
    flag_no_headers:               bool,
    flag_delimiter:                Option<crate::config::Delimiter>,
    flag_memcheck:                 bool,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    // URL and stdin inputs are materialized to a tempfile so the rest of the
    // pipeline (stats, frequency, sqlp-backed helpers) sees a normal file
    // path. The tempfile must outlive `run`'s body — we bind it to a local
    // variable (`_kept_temp`) so its Drop runs only at function exit.
    //
    // `display_input` is the human-readable label that surfaces in the
    // output JSON's `input` field (and the default `-o` filename when
    // stdin): "stdin" for piped input, the original path/URL otherwise.
    // `original_url` is preserved separately so the DCAT projection can
    // use it as `dcat:downloadURL` and to derive a title fallback.
    let (input_path, original_url, display_input, from_stdin, _kept_temp, http_last_modified) =
        match args.arg_input.as_deref() {
            Some("-") | None => {
                let temp = stdin_to_tempfile()?;
                let local = temp.path().to_string_lossy().to_string();
                (local, None, "stdin".to_string(), true, Some(temp), None)
            },
            Some(p) => {
                let resolved = resolve_input(p)?;
                let display = resolved.url.clone().unwrap_or_else(|| p.to_string());
                (
                    resolved.local,
                    resolved.url,
                    display,
                    false,
                    resolved.kept,
                    resolved.last_modified,
                )
            },
        };

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

    // --- 1b. load the projection profile (YAML-driven engine) -------------
    // Defaults to "dcat-us-v3" when --profile is omitted, preserving the
    // pre-YAML-engine behavior. The dry_compile pass catches malformed
    // templates BEFORE we run stats / frequency / formula evaluation, so
    // a typo in a profile YAML doesn't burn 30s of upstream work first.
    let profile_arg = args.flag_profile.as_deref().unwrap_or("dcat-us-v3");
    // load() validates templates via dry_compile internally — a
    // malformed embedded profile or a user-supplied file with a typo
    // surfaces here before stats / frequency / formulas run.
    let profile = profile_spec::load(profile_arg)?;

    // --- 2. run stats + frequency, build analysis context -----------------
    if args.flag_describegpt.is_some() && args.flag_infer {
        return fail_incorrectusage_clierror!("--describegpt and --infer cannot be used together.");
    }
    if args.flag_tag_vocab.is_some() && args.flag_describegpt.is_none() && !args.flag_infer {
        return fail_incorrectusage_clierror!("--tag-vocab needs --describegpt or --infer.");
    }
    // A named file that is unreadable or malformed is a usage error, so
    // check both before the (slow) analysis runs.
    let tag_vocab = args
        .flag_tag_vocab
        .as_deref()
        .map(llm::read_tag_vocab)
        .transpose()?;
    let describegpt_json = args
        .flag_describegpt
        .as_deref()
        .map(llm::read_describegpt_json)
        .transpose()?;

    let locate_mode = args
        .flag_locate
        .as_deref()
        .map(locate::LocateMode::parse)
        .transpose()?;

    let ctx_args = context::ContextArgs {
        input_path:      &input_path,
        no_headers:      args.flag_no_headers,
        delimiter:       args.flag_delimiter,
        jobs:            args.flag_jobs,
        force:           args.flag_force,
        memcheck:        args.flag_memcheck,
        initial_context: args.flag_initial_context.as_deref(),
        profile:         &profile,
        check_rfc4180:   args.flag_validate && !args.flag_no_projection,
    };
    let analysis = context::build(&ctx_args, spec_opt.as_ref())?;

    // One SQL backend for both the spec's formulas and the projection
    // templates, honoring the same CSV parsing options as the stats /
    // frequency passes so SQL-backed helpers see the same columns.
    // The delimiter resolves exactly as stats' does (--delimiter, then
    // the file extension / QSV_DEFAULT_DELIMITER), so a `.tsv` input is
    // not read as one comma-less column. --no-headers maps to
    // has_header=false.
    let resolved_delimiter = Config::new(Some(&input_path))
        .delimiter(args.flag_delimiter)
        .get_delimiter();
    let sql_backend = sql_backend::SqlBackend::new(&input_path)
        .with_delimiter(resolved_delimiter)
        .with_has_header(!args.flag_no_headers);

    // --- 3. formula evaluation (minijinja, native Rust) -----------------
    // When a spec is provided, evaluate every `formula` / `suggestion_formula`
    // template against the analysis context. Helpers are the Rust port of
    // DP+'s `jinja2_helpers.py` (see `formula_helpers.rs`).
    let formula_results = match spec_opt.as_ref() {
        Some(spec) => {
            formula_engine::evaluate_spec(spec, &analysis.context, Some(sql_backend.clone()))?
        },
        None => Vec::new(),
    };
    let formulas_evaluated = !formula_results.is_empty();

    // --- 4. assemble output ----------------------------------------------
    let mut output = json!({
        "qsv_version":      env!("CARGO_PKG_VERSION"),
        "generated_at":     chrono::Utc::now().to_rfc3339(),
        "spec_file":        args.flag_spec.clone(),
        "input":            display_input,
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

    // Build the merged package/resource once so both --no-ckan and --no-projection
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
    merge_formula_results(
        &mut package,
        &mut resource,
        &formula_results,
        &analysis.forced_package_fields,
        &analysis.forced_resource_fields,
    );

    // When the input was a URL, stamp it as the resource URL so the DCAT
    // projection's `dcat:downloadURL` slot gets populated (subject to the
    // existing absolute-IRI check). Don't overwrite an explicit
    // resource.url already supplied via formulas / seed metadata. Same
    // pattern for package.title / resource.name: when the user hasn't
    // supplied them, derive defaults from the URL basename so the DCAT
    // title slot doesn't surface the random tempfile suffix.
    // Mirror of the URL block below: when input came from stdin, the
    // tempfile-stem default for resource.name leaks the random suffix into
    // the DCAT title. Replace it with "stdin" unless the user explicitly
    // supplied a resource.name via --initial-context or formulas.
    if from_stdin && let Some(res_obj) = resource.as_object_mut() {
        let tempfile_stem = Path::new(&input_path)
            .file_stem()
            .and_then(|s| s.to_str())
            .map(str::to_string);
        let current = res_obj
            .get("name")
            .and_then(|v| v.as_str())
            .map(str::to_string);
        if current.is_none() || current == tempfile_stem {
            res_obj.insert("name".to_string(), Value::String("stdin".to_string()));
        }
    }

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

    // Build-time projection warnings, each paired with the JSON-Pointer
    // prefix (no leading `/`) of the object its `field` is relative to:
    // `""` for the projection root (Dataset mode, or the Catalog
    // envelope's own fields) and `<dataset_key>/0/` for Dataset fields
    // once wrapped in a Catalog. Consumed after overrides + validation.
    let mut pending_warnings: Vec<(projection::ProjectionWarning, String)> = Vec::new();
    // Every value that came from a field's inferred-tier `fallback`, as
    // projected — before discovery merge and overrides. Compared against
    // the final projection to report provenance and to keep review notes
    // only while the inferred value actually survives.
    let mut inferred_snapshot: Vec<InferredValue> = Vec::new();

    if !args.flag_no_projection {
        let dpp = analysis.context.get("dpp").cloned().unwrap_or(json!({}));
        let stats = analysis.context.get("dpps").cloned().unwrap_or(json!({}));
        // Per-column frequency (dppf), threaded into the projection
        // context so profiles can re-express value-count distributions in
        // their own vocabulary (e.g. the Croissant profile emits one
        // nested cr:RecordSet of inline {value,count,percentage} rows per
        // column, per the spec's "distribution of values is a statistic on
        // the field" guidance). Opt-in via --croissant-frequency to keep
        // the projection compact by default; an empty map makes the
        // profile templates emit no frequency block. The raw counts always
        // remain in the top-level `frequency` output block regardless.
        // Filled from the normalized `dppf` below.
        let mut frequency = json!({});
        // Build the projection context: the YAML's field templates
        // reference these top-level names. `pkg` and `res` are the
        // merged CKAN package + resource; `source_label` is the
        // user-facing path/URL/"stdin"; `local_path` is the actual
        // on-disk file (tempfile path for URL/stdin inputs); `stats`
        // is the dpps stats array used by csvw:tableSchema.
        // Roborev #2490 finding #4: thread the --dcat-legacy-license
        // flag into the projection context so the YAML templates can
        // optionally re-emit dct:license at the Dataset level (DCAT
        // v1.1 back-compat). Strict v3 keeps license on Distribution
        // only; the flag is off by default.
        //
        // `dppf` is the always-present per-column frequency (top 25,
        // nulls excluded) that stats-derived helpers read — CSVW
        // enumerations, language detection — independent of the
        // opt-in `frequency` key above. `rfc4180` carries the `qsv
        // validate` RFC 4180 / UTF-8 result, and `http_last_modified`
        // the URL response's Last-Modified (Zulu), when available.
        // With --no-headers the three column namings disagree: stats
        // keys columns 0-based (`0`, `1`), frequency 1-based (`1`, `2`),
        // and `analysis.headers` holds the first DATA row. Re-key
        // frequency to the stats identities and use those as the
        // ordered column names, or helpers attach column 0's values
        // to column 1 and describe data values as column names.
        let mut dppf = analysis.context.get("dppf").cloned().unwrap_or(json!({}));
        let headers: Vec<String> = if args.flag_no_headers {
            if let Some(obj) = dppf.as_object() {
                dppf = Value::Object(
                    obj.iter()
                        .filter_map(|(k, v)| {
                            let n = k.parse::<usize>().ok()?.checked_sub(1)?;
                            Some((n.to_string(), v.clone()))
                        })
                        .collect(),
                );
            }
            stats
                .as_object()
                .map(|o| o.keys().cloned().collect())
                .unwrap_or_default()
        } else {
            analysis.headers.clone()
        };
        let rfc4180 = analysis
            .context
            .get("rfc4180")
            .cloned()
            .unwrap_or(Value::Null);
        // The opt-in Croissant histograms must index the same
        // (normalized) per-column frequency the derived helpers read.
        if args.flag_croissant_frequency {
            frequency = dppf.clone();
        }
        // --locate: the place lookups run once, here, as `qsv geocode`
        // child processes; templates only read the result.
        let located = match locate_mode {
            Some(mode) => locate::locate(mode, &sql_backend, &dpp, &dppf, &headers),
            None => Value::Null,
        };
        // Where Dataset fields sit in the output: under the Catalog
        // envelope's dataset array with --catalog, at the top otherwise.
        let dataset_prefix = if args.flag_catalog {
            let dataset_key = profile
                .catalog
                .as_ref()
                .and_then(|c| c.dataset_key.as_deref())
                .unwrap_or("dcat:dataset");
            format!("{}/0/", dataset_key.replace('~', "~0").replace('/', "~1"))
        } else {
            String::new()
        };
        // --describegpt / --infer: LLM output, verified (tag vocabulary,
        // this file's columns) before the templates see it.
        let llm_json = if args.flag_infer {
            match llm::run_describegpt(&input_path, args.flag_tag_vocab.as_deref()) {
                Ok(v) => Some(v),
                Err(e) => {
                    pending_warnings.push((
                        projection::ProjectionWarning {
                            field:    "description".to_string(),
                            severity: projection::Severity::Recommended,
                            message:  format!(
                                "--infer: {e}; the statistics-based description is used"
                            ),
                        },
                        dataset_prefix.clone(),
                    ));
                    None
                },
            }
        } else {
            describegpt_json.clone()
        };
        let llm_ctx = match llm_json.as_ref() {
            Some(dg) => {
                let built = llm::build(dg, tag_vocab.as_ref(), &headers);
                for (field, message) in built.notes {
                    pending_warnings.push((
                        projection::ProjectionWarning {
                            field,
                            severity: projection::Severity::Info,
                            message,
                        },
                        dataset_prefix.clone(),
                    ));
                }
                if built.attribution.as_object().is_some_and(|a| !a.is_empty()) {
                    out_map.insert("llm_attribution".to_string(), built.attribution);
                }
                built.context
            },
            None => Value::Null,
        };
        let projection_ctx = json!({
            "pkg":                package,
            "res":                resource,
            "stats":              stats,
            "frequency":          frequency,
            "dppf":               dppf,
            // Column names in FILE order. Template maps iterate sorted
            // (minijinja is built without preserve_order), so anything
            // order-sensitive — CSVW columns, the generated
            // description's column list — must iterate this instead.
            "headers":            headers,
            "dpp":                dpp,
            "rfc4180":            rfc4180,
            "http_last_modified": http_last_modified,
            "located":            located,
            "llm":                llm_ctx,
            "source_label":       display_input,
            "local_path":         input_path,
            "legacy_license":     args.flag_dcat_legacy_license,
        });
        // SQL-backed and stats-derived helpers (temporal resolution,
        // robust bbox, duplicate rows, ...) need the backend during the
        // projection pass too, not only during spec evaluation. The
        // guard clears it on every exit path.
        let _sql_guard = formula_helpers::SqlBackendGuard::install(sql_backend.clone());
        // Always project in Dataset mode first, so the discovery merge
        // applies to the Dataset's top-level keys before the Catalog
        // envelope is wrapped around them (Roborev #2490 finding #1).
        // Wrapping first would push discovered Dataset metadata onto
        // the outer Catalog (where keys like `dct:contactPoint`
        // shouldn't live) instead of the inner Dataset.
        let _ = projection::take_inferred_paths(); // discard anything stale
        let (dataset_block, dataset_warnings) = projection::project(
            &profile,
            &projection_ctx,
            projection::ProjectionMode::Dataset,
        )?;
        let inferred_paths = projection::take_inferred_paths();
        let inferred_pointers: Vec<String> =
            inferred_paths.iter().map(|p| p.pointer.clone()).collect();
        let merged_dataset = discovery_merge::merge_with_inferred(
            &profile,
            dataset_block.clone(),
            discovered_dcat.as_ref(),
            &analysis.forced_paths,
            &inferred_pointers,
        );
        let merged_dcat = if args.flag_catalog {
            let (catalog_block, cat_warnings) =
                projection::wrap_in_catalog_envelope(&profile, merged_dataset, &projection_ctx)?;
            let prefix = dataset_prefix.clone();
            inferred_snapshot.extend(inferred_values(&dataset_block, &inferred_paths, &prefix));
            // Catalog-envelope fields that came from a `fallback`.
            let catalog_inferred = projection::take_inferred_paths();
            inferred_snapshot.extend(inferred_values(&catalog_block, &catalog_inferred, ""));
            pending_warnings.extend(dataset_warnings.into_iter().map(|w| (w, prefix.clone())));
            pending_warnings.extend(cat_warnings.into_iter().map(|w| (w, String::new())));
            catalog_block
        } else {
            inferred_snapshot.extend(inferred_values(&dataset_block, &inferred_paths, ""));
            pending_warnings.extend(dataset_warnings.into_iter().map(|w| (w, String::new())));
            merged_dataset
        };
        out_map.insert("projection".to_string(), merged_dcat);
        // Surface the raw discovered DCAT alongside the merged block so
        // downstream tooling can diff or audit what came from the
        // publisher vs what qsv inferred. The discovered payload is
        // always DCAT-shaped (HTTP Link: rel=describedBy markup) even
        // when the active profile is Croissant or Geoconnex, so the
        // `dcat_discovered` key keeps its DCAT-specific name.
        if let Some(d) = discovered_dcat {
            out_map.insert("dcat_discovered".to_string(), d);
        }
    }

    // --initial-context's `dataset_info` JSON-Pointer overrides are
    // applied first — unconditionally over inference, discovery, the
    // CKAN block, and formula output. Must run BEFORE schema
    // validation so an override that supplies a missing mandatory
    // field doesn't trip --strict. Per plan §4c.
    if let Some(overrides) = analysis.dataset_info.as_object()
        && !overrides.is_empty()
    {
        apply_pointer_overrides(&mut output, overrides);
    }

    // §5.4: forced `(projection_pointer, value)` pairs from all three
    // subtrees (dataset_info, package, resource — package/resource keys
    // translated via the active profile's `field_mappings:` table, see
    // `ProfileSpec::translate_ckan_ptr`). Applied AFTER apply_pointer_overrides so a
    // `{value, force: true}` wrapper beats both inferred metadata
    // AND publisher-discovered DCAT, with last-write-wins on
    // aliasing pointers. Still runs BEFORE schema validation so a
    // forced field can rescue --strict the same way dataset_info
    // entries do.
    if !analysis.forced_values.is_empty() {
        apply_force_overrides(&mut output, &analysis.forced_values);
    }

    // Phase 6 (post-override): JSON Schema validation runs on the
    // emitted projection block, after dataset_info overrides have
    // applied. Takes the pending build-time warnings, drops
    // any whose referenced field is now present in the final
    // projection block (the dataset_info override or discovered-DCAT
    // merge satisfied them), then merges schema violations into the
    // final projection_warnings array.
    if !args.flag_no_projection {
        let out_map = output.as_object_mut().unwrap();
        let final_projection_snapshot = out_map.get("projection").cloned();

        // An inferred value survives only if the final projection still
        // holds it: discovered publisher metadata (inferred tier) or a
        // dataset_info / forced override may have replaced it.
        let still_inferred = |ptr: &str, value: &Value| {
            final_projection_snapshot
                .as_ref()
                .and_then(|p| p.pointer(ptr))
                .is_some_and(|v| v == value)
        };
        let mut provenance = Map::new();
        for iv in &inferred_snapshot {
            let source = if still_inferred(&iv.pointer, &iv.value) {
                "inferred"
            } else {
                "replaced"
            };
            provenance.insert(iv.pointer.clone(), Value::String(source.to_string()));
        }
        if !provenance.is_empty() {
            out_map.insert(
                "projection_provenance".to_string(),
                Value::Object(provenance),
            );
        }

        let (mut projection_warnings, warning_prefixes): (
            Vec<projection::ProjectionWarning>,
            Vec<String>,
        ) = pending_warnings
            .into_iter()
            .filter(|(w, prefix)| {
                // Info notes describe how a value was derived (or why
                // it was not); they stay regardless.
                if w.severity == projection::Severity::Info {
                    return true;
                }
                // A review note on an inferred value stays exactly as
                // long as that value does.
                if let Some(iv) = inferred_snapshot
                    .iter()
                    .find(|iv| iv.field == w.field && iv.prefix == *prefix)
                {
                    return still_inferred(&iv.pointer, &iv.value);
                }
                !final_projection_has_field(final_projection_snapshot.as_ref(), prefix, &w.field)
            })
            .unzip();

        if args.flag_validate
            && let Some(final_projection) = out_map.get("projection")
        {
            let validation = dcat_validate::validate(&profile, final_projection);
            // --strict aborts on Required-severity findings only.
            //
            // It used to abort on ANY non-empty finding vector, which
            // made severity decorative: a Recommended-level advisory
            // was as fatal as a missing mandatory property. Now that
            // severity is derived from the schema bundle's own
            // `requirementLevel` annotations (see
            // `dcat_validate::classify_severity`), gate on it — and
            // match how --strict already treats external-validator
            // findings, which filter out Info before escalating.
            // Sub-Required findings still surface in
            // projection_warnings.
            let blocking: Vec<_> = validation
                .iter()
                .filter(|w| matches!(w.severity, projection::Severity::Required))
                .collect();
            if !blocking.is_empty() && args.flag_strict {
                let summary = blocking
                    .iter()
                    .map(|w| format!("  - {}: {}", w.field, w.message))
                    .collect::<Vec<_>>()
                    .join("\n");
                return Err(CliError::Other(format!(
                    "qsv profile --strict: {} schema violation(s):\n{summary}",
                    blocking.len()
                )));
            }
            // The schema validator and the template stage both report a
            // missing Required property; keep only the schema finding,
            // which is the one --strict counts.
            // `retain` visits elements exactly once, in order, so the
            // parallel prefix iterator stays aligned.
            let mut prefixes = warning_prefixes.iter();
            projection_warnings.retain(|w| {
                let prefix = prefixes.next().map_or("", String::as_str);
                !is_dup_of_schema_required(w, prefix, &validation)
            });
            projection_warnings.extend(validation);

            // Out-of-process validator (e.g. mlcroissant, pyshacl).
            // Runs in parallel to the built-in JSON-Schema validator;
            // a profile may use either, both, or neither.
            //
            // Trust gate: a `--profile <path>` argument can load an
            // arbitrary YAML file from disk, and that file controls
            // `validation.external.command`. Spawning whatever the
            // file says would let an untrusted profile execute
            // arbitrary commands. So: embedded profiles always run
            // their declared external validators (qsv release vets
            // those YAMLs); file-loaded profiles need an explicit
            // `--allow-external-validator` opt-in, otherwise we skip
            // the spawn and emit a Recommended-severity warning that
            // tells the user what would have run and how to opt in.
            // Strict mode honors external findings the same way it
            // honors schema violations — once you've opted in to
            // validation, all findings escalate.
            let allow_external = profile.source.is_embedded() || args.flag_allow_external_validator;
            if let Some(cfg) = profile.validation.external.as_ref()
                && !allow_external
            {
                let label = cfg.label.as_deref().unwrap_or(cfg.command.as_str());
                projection_warnings.push(projection::ProjectionWarning {
                    field:    "external_validate".to_string(),
                    severity: projection::Severity::Recommended,
                    message:  format!(
                        "external validator `{label}` declared by file-loaded profile `{}` was \
                         NOT run. Spawning arbitrary commands from untrusted profiles is gated \
                         for safety. Pass --allow-external-validator to opt in.",
                        profile.name,
                    ),
                });
            } else if allow_external {
                let external = external_validate::validate(&profile, final_projection);
                let external_findings: Vec<_> = external
                    .iter()
                    .filter(|w| !matches!(w.severity, projection::Severity::Info))
                    .collect();
                if !external_findings.is_empty() && args.flag_strict {
                    let summary = external_findings
                        .iter()
                        .map(|w| format!("  - {}: {}", w.field, w.message))
                        .collect::<Vec<_>>()
                        .join("\n");
                    return Err(CliError::Other(format!(
                        "qsv profile --strict: {} external validator finding(s):\n{summary}",
                        external_findings.len()
                    )));
                }
                projection_warnings.extend(external);
            }
        }

        // §5.8: profile-driven validation. When the spec opts in by
        // declaring any `validators`, run `qsv validate` against the
        // input and merge any RFC4180 failures into projection_warnings.
        // Triggered regardless of --validate so users get a useful
        // structural signal even without enabling JSON-Schema
        // validation against the emitted projection block.
        if spec_opt
            .as_ref()
            .is_some_and(super::profile::spec::Spec::has_validators)
        {
            projection_warnings.extend(run_profile_validation(
                &input_path,
                args.flag_no_headers,
                args.flag_delimiter,
            ));
        }

        if !projection_warnings.is_empty() {
            out_map.insert(
                "projection_warnings".to_string(),
                serde_json::to_value(&projection_warnings).unwrap_or(json!([])),
            );
        }
    }

    let _ = analysis.headers;

    // --- 5. write output --------------------------------------------------
    let out_path = args.flag_output.clone().unwrap_or_else(|| {
        if from_stdin {
            // Avoid leaking the random tempfile suffix into the default
            // filename for piped input.
            "stdin.metadata.json".to_string()
        } else {
            let stem = Path::new(&input_path)
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("output");
            format!("{stem}.metadata.json")
        }
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
    forced_package_fields: &std::collections::HashSet<String>,
    forced_resource_fields: &std::collections::HashSet<String>,
) {
    if results.is_empty() {
        return;
    }

    // Pass 1: stamp `formula` results onto the package/resource fields.
    // Roborev #2491: skip fields the user marked `force: true` in the
    // initial-context so a spec formula can't overwrite the forced
    // CKAN value before projection. The forced value already lives in
    // package/resource via normalize_value_force; preserving it here
    // honors the documented "force beats inferred" guarantee while
    // still letting the value flow through the profile templates for
    // shaping.
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
                    if forced_package_fields.contains(&r.field_name) {
                        continue;
                    }
                    pkg.insert(r.field_name.clone(), value);
                },
                "resource" => {
                    if forced_resource_fields.contains(&r.field_name) {
                        continue;
                    }
                    res.insert(r.field_name.clone(), value);
                },
                _ => {},
            }
        }
    }

    // Pass 2: collect `suggestion_formula` results under
    // package.dpp_suggestions. Done after pass 1 finishes so we hold no
    // overlapping mutable borrows of `package`. Suggestions are
    // advisory and live in a separate namespace; they are NOT subject
    // to the force-skip (a suggestion that targets `spatial_extent`
    // doesn't overwrite a forced `pkg.publisher`).
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

/// A value a field's inferred-tier `fallback` supplied, as projected.
struct InferredValue {
    /// JSON Pointer under the final projection root.
    pointer: String,
    value:   Value,
    /// Logical field its review note carries (`ProjectionWarning::field`).
    field:   String,
    /// Pending-warning prefix of the block it was projected into.
    prefix:  String,
}

/// Snapshot the inferred values of a freshly projected block. `prefix`
/// is the JSON-Pointer prefix (escaped, no leading `/`) of that block
/// once placed in the final projection: `""` for a Dataset in Dataset
/// mode or for the Catalog envelope, `<dataset_key>/0/` for a Dataset
/// inside a Catalog.
fn inferred_values(
    block: &Value,
    inferred_paths: &[projection::InferredPath],
    prefix: &str,
) -> Vec<InferredValue> {
    inferred_paths
        .iter()
        .filter_map(|p| {
            let value = block.pointer(&format!("/{}", p.pointer))?.clone();
            Some(InferredValue {
                pointer: format!("/{prefix}{}", p.pointer),
                value,
                field: p.field.clone(),
                prefix: prefix.to_string(),
            })
        })
        .collect()
}

/// Coerce a JSON value into a mutable object, replacing it with `{}` if it
/// wasn't an object to begin with.
fn ensure_object(v: &mut Value) -> &mut serde_json::Map<String, Value> {
    if !v.is_object() {
        *v = json!({});
    }
    v.as_object_mut().unwrap()
}

/// A user-supplied input resolved to a local file. For an http(s) URL
/// the body is downloaded to a tempfile so the rest of the pipeline
/// sees a normal file.
struct ResolvedInput {
    /// Local file path to feed to stats / frequency / sqlp.
    local:         String,
    /// The original URL (when the input was one), for later use as
    /// `downloadURL` on the Distribution.
    url:           Option<String>,
    /// Tempfile handle the caller must keep alive until all downstream
    /// readers have finished — dropping it deletes the temp on disk.
    kept:          Option<tempfile::NamedTempFile>,
    /// The response's `Last-Modified` header as a Zulu RFC 3339
    /// timestamp — the server's own statement of when the file last
    /// changed, used as the Distribution's inferred `modified`.
    last_modified: Option<String>,
}

/// Parse an HTTP-date (`Sun, 06 Nov 1994 08:49:37 GMT`, RFC 9110 §5.6.7)
/// into a Zulu RFC 3339 timestamp; `None` when it is not one.
fn http_date_to_zulu(raw: &str) -> Option<String> {
    chrono::DateTime::parse_from_rfc2822(raw.trim())
        .ok()
        .map(|dt| {
            dt.with_timezone(&chrono::Utc)
                .to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
        })
}

/// Resolve the user-supplied input into a local file, downloading an
/// http(s) URL to a tempfile. See `ResolvedInput` for what is returned.
fn resolve_input(raw: &str) -> CliResult<ResolvedInput> {
    use std::io::Write;

    if !is_http_url(raw) {
        return Ok(ResolvedInput {
            local:         raw.to_string(),
            url:           None,
            kept:          None,
            last_modified: None,
        });
    }

    // Reuse qsv's shared blocking HTTP client (user-agent,
    // gzip/brotli/zstd compression, rustls, retry on 503) so URL inputs
    // behave consistently with `fetch`, `validate`, `describegpt` etc.
    let client = util::create_reqwest_blocking_client(None, 120, None)?;
    let mut response = client
        .get(raw)
        .send()
        .and_then(reqwest::blocking::Response::error_for_status)
        .map_err(|e| CliError::Other(format!("qsv profile: download {raw}: {e}")))?;

    let last_modified = response
        .headers()
        .get(reqwest::header::LAST_MODIFIED)
        .and_then(|v| v.to_str().ok())
        .and_then(http_date_to_zulu);

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
    std::io::copy(&mut response, temp.as_file_mut())
        .map_err(|e| CliError::Other(format!("qsv profile: stream body from {raw}: {e}")))?;
    temp.as_file_mut().flush().ok();

    let local = temp.path().to_string_lossy().to_string();
    Ok(ResolvedInput {
        local,
        url: Some(raw.to_string()),
        kept: Some(temp),
        last_modified,
    })
}

/// Materialize stdin into a `.csv`-suffixed tempfile so the rest of the
/// pipeline (stats, frequency, sqlp-backed helpers) sees a normal file
/// path. The caller must keep the returned handle alive until all
/// downstream readers have finished — dropping it deletes the temp on
/// disk. Mirrors `resolve_input`'s URL→tempfile branch.
fn stdin_to_tempfile() -> CliResult<tempfile::NamedTempFile> {
    use std::io::Write;

    let mut temp = tempfile::Builder::new()
        .prefix("qsv-profile-stdin-")
        .suffix(".csv")
        .tempfile()
        .map_err(|e| CliError::Other(format!("qsv profile: create stdin tempfile: {e}")))?;

    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    std::io::copy(&mut handle, temp.as_file_mut())
        .map_err(|e| CliError::Other(format!("qsv profile: read stdin: {e}")))?;
    temp.as_file_mut().flush().ok();

    Ok(temp)
}

/// §5.8: run `qsv validate` against `input_path` and return any RFC4180
/// failures as `ProjectionWarning`s. Best-effort — spawn errors, missing
/// binary, or non-UTF-8 stderr all silently degrade to "no warnings"
/// rather than failing the whole profile run.
///
/// The trigger lives in the caller: this helper is only invoked when
/// the spec declares one or more `validators` (see
/// `Spec::has_validators`). The validators' string content isn't
/// interpreted yet — auto-generating a JSON Schema from declared
/// types + validators is a future enhancement; for now the presence
/// of any validators opts the user into RFC4180 structural checks.
///
/// `--no-headers` and `--delimiter` are forwarded so `qsv validate`
/// parses the input the same way the rest of the profile pipeline
/// (stats / frequency / count) does. Without this, a profile run with
/// non-default CSV options would yield spurious RFC4180 failures (or
/// miss real ones) because validate would default to comma-separated
/// input with headers. Other flags `qsv validate` supports (e.g.
/// `--jobs`, `--trim`) aren't surfaced by `profile` itself, so we
/// don't forward them.
fn run_profile_validation(
    input_path: &str,
    no_headers: bool,
    delimiter: Option<crate::config::Delimiter>,
) -> Vec<projection::ProjectionWarning> {
    let start = std::time::Instant::now();
    let Ok(qsv_path) = util::current_exe() else {
        return Vec::new();
    };
    let mut cmd = std::process::Command::new(qsv_path);
    crate::util::pass_stdin_spool(&mut cmd);
    cmd.arg("validate").arg(input_path);
    if no_headers {
        cmd.arg("--no-headers");
    }
    if let Some(d) = delimiter {
        cmd.arg("--delimiter")
            .arg((d.as_byte() as char).to_string());
    }
    let Ok(output) = cmd.output() else {
        return Vec::new();
    };
    util::print_status("qsv profile: ran `validate`", Some(start.elapsed()));

    if output.status.success() {
        return Vec::new();
    }
    let stderr = String::from_utf8_lossy(&output.stderr);
    // First non-empty line is the headline error. Strip the
    // "Validation error: " prefix qsv validate prepends so the
    // surfaced message reads naturally inside a dcat_warning.
    let msg = stderr.lines().find(|l| !l.trim().is_empty()).map_or_else(
        || "qsv validate failed with no error message".to_string(),
        |s| {
            s.trim()
                .trim_start_matches("Validation error: ")
                .to_string()
        },
    );
    vec![projection::ProjectionWarning {
        field:    "qsv:validation".to_string(),
        severity: projection::Severity::Required,
        message:  format!("input failed `qsv validate` (RFC4180): {msg}"),
    }]
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
        .map_or_else(|| ".csv".to_string(), |e| format!(".{e}"))
}

/// Derive a human-readable default title from a URL when the user
/// hasn't supplied `--initial-context.package.title`. Strips the
/// URL's well-known compound extensions (`.csv.gz`, etc.) and any
/// remaining single extension, then returns the last non-empty path
/// segment.
///
/// §5.9: when the leaf segment looks UUID-like (canonical 8-4-4-4-12
/// hex with dashes, or the compact 32-char hex variant — common in
/// CKAN's `/datastore/dump/<uuid>` shape and elsewhere), walk up the
/// path until we find a non-UUID-like segment, up to a 3-level cap.
/// That yields meaningful titles like "dump" instead of opaque UUIDs.
/// If every candidate up the cap is UUID-like, fall back to the leaf
/// UUID — still better than the random tempfile suffix.
///
/// Returns `None` when the URL has no usable path (e.g. just a host,
/// or a trailing slash with no segments), in which case the caller
/// falls back to the existing tempfile-stem behaviour.
///
/// Users who want a prettier title than what URL-walking can derive
/// should populate `--initial-context.package.title` explicitly — a
/// CKAN `/api/3/action/resource_show?id=<uuid>` lookup is a deferred
/// follow-up.
fn url_title_default(url: &str) -> Option<String> {
    let parsed = url::Url::parse(url).ok()?;
    let segments: Vec<&str> = parsed.path().split('/').filter(|s| !s.is_empty()).collect();
    if segments.is_empty() {
        return None;
    }

    // Walk from leaf upward, up to 3 levels. Return the first
    // non-UUID-like stem we find.
    for seg in segments.iter().rev().take(3) {
        let cleaned = strip_compound_csv_ext(seg);
        let stem = std::path::Path::new(&cleaned)
            .file_stem()
            .and_then(|s| s.to_str())
            .map(str::to_string)
            .filter(|s| !s.is_empty());
        if let Some(name) = stem
            && !is_uuid_like(&name)
        {
            return Some(name);
        }
    }

    // Every candidate up the cap was UUID-like. Fall back to the leaf
    // basename so the title is at least reproducible.
    let leaf = segments.last()?;
    let cleaned = strip_compound_csv_ext(leaf);
    std::path::Path::new(&cleaned)
        .file_stem()
        .and_then(|s| s.to_str())
        .map(str::to_string)
        .filter(|s| !s.is_empty())
}

/// §5.9 helper: does this segment look like a UUID? Matches:
///   * canonical RFC 4122 form: 8-4-4-4-12 hex with dashes (e.g.
///     `5202679a-d243-402e-b82a-63189995a942`);
///   * compact form: 32 contiguous hex characters (e.g. `5202679ad243402eb82a63189995a942`).
///
/// Case-insensitive. Other ID-like patterns (`MongoDB` `ObjectId` at 24
/// hex, ULIDs, slugified IDs) are intentionally NOT matched — UUIDs
/// dominate CKAN/data.gov URLs, and over-eager matching would walk
/// past legitimate titles like "2024-Q3".
fn is_uuid_like(s: &str) -> bool {
    let bytes = s.as_bytes();
    if bytes.len() == 36 {
        // Canonical: 8-4-4-4-12 with dashes at fixed positions.
        // Explicit length assert (already implied by the outer
        // `len() == 36` check) lets the compiler elide bounds
        // checks on the byte indexes below and satisfies
        // `clippy::missing_asserts_for_indexing`.
        assert!(bytes.len() > 23);
        if bytes[8] != b'-' || bytes[13] != b'-' || bytes[18] != b'-' || bytes[23] != b'-' {
            return false;
        }
        return bytes
            .iter()
            .enumerate()
            .all(|(i, &b)| matches!(i, 8 | 13 | 18 | 23) || b.is_ascii_hexdigit());
    }
    if bytes.len() == 32 {
        return bytes.iter().all(u8::is_ascii_hexdigit);
    }
    false
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
/// must be an RFC 6901 JSON Pointer (e.g. `/projection/dct:title`); the
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

/// §5.4: apply forced `(dcat_pointer, value)` pairs after
/// `apply_pointer_overrides` so a user's `{value, force: true}`
/// markings beat both inferred output AND discovered DCAT. Mirrors
/// `apply_pointer_overrides`'s skip-non-pointer behaviour but takes
/// a `&[(String, Value)]` instead of a Map so insertion order is
/// preserved (matters when two forced leaves alias the same pointer
/// via the active profile's `field_mappings:` table — last-write-wins).
///
/// `forced_values` comes from `context::collect_forced_paths` and
/// already has any `package/<key>` / `resource/<key>` pointers
/// translated to their target counterparts.
fn apply_force_overrides(root: &mut Value, forced_values: &[(String, Value)]) {
    for (ptr, new_value) in forced_values {
        if !ptr.starts_with('/') {
            continue;
        }
        set_by_pointer(root, ptr, new_value.clone());
    }
}

/// Returns true when the final projection block carries a non-null,
/// non-empty value for `field` (a key like `"contactPoint"` or a nested
/// path like `"distribution/0/license"`) inside the object at `prefix`
/// (a JSON-Pointer prefix without the leading `/`: `""` for the
/// projection root, `"dataset/0/"` for a Dataset wrapped in a Catalog).
/// Used to filter stale build-time warnings after `dataset_info`
/// overrides and discovered-DCAT merging have had a chance to populate
/// slots that were originally absent.
///
/// Top-level field names get a fast direct lookup; nested paths are
/// resolved via JSON Pointer (with a leading `/` added if absent).
/// Returns false for any unparseable / missing field — the safe
/// default is "keep the warning".
fn final_projection_has_field(final_projection: Option<&Value>, prefix: &str, field: &str) -> bool {
    let Some(root) = final_projection else {
        return false;
    };
    if field.is_empty() {
        return false;
    }
    let scope = if prefix.is_empty() {
        Some(root)
    } else {
        root.pointer(&format!("/{}", prefix.trim_end_matches('/')))
    };
    let Some(projection) = scope else {
        return false;
    };
    // Top-level field name (the common case for build-time warnings).
    if !field.contains('/')
        && let Some(v) = projection.get(field)
    {
        return !is_value_empty(v);
    }
    // Nested JSON-Pointer path.
    let pointer = if field.starts_with('/') {
        field.to_string()
    } else {
        format!("/{field}")
    };
    projection
        .pointer(&pointer)
        .is_some_and(|v| !is_value_empty(v))
}

/// True when `w` is a template-stage "missing X" warning that the
/// schema validator also reported as a Required missing property at
/// the same location (`prefix` + `X`, see [`final_projection_has_field`]).
/// Render-error warnings at the same field are deliberately not
/// matched — they explain *why* the field is missing.
fn is_dup_of_schema_required(
    w: &projection::ProjectionWarning,
    prefix: &str,
    schema: &[projection::ProjectionWarning],
) -> bool {
    if !matches!(w.severity, projection::Severity::Required)
        || w.message != format!("missing {}", w.field)
    {
        return false;
    }
    let path = format!("{prefix}{}", w.field);
    schema
        .iter()
        .any(|s| matches!(s.severity, projection::Severity::Required) && s.field == path)
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
        if let Value::Array(arr) = cursor {
            // Traverse into an existing array via numeric segment. We
            // never create or extend arrays here — that would require
            // distinguishing array-vs-object intent at every missing
            // parent, which RFC 6901 leaves to the caller. Out-of-range
            // indices and non-numeric tokens are silently skipped so a
            // typo in a single override doesn't corrupt the rest of
            // the output.
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
        } else {
            // Object (or any non-object scalar, which we replace with
            // an empty object before descending). Same behaviour as
            // before for object-typed parents.
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
}

fn is_http_url(s: &str) -> bool {
    let lower = s.to_ascii_lowercase();
    lower.starts_with("http://") || lower.starts_with("https://")
}

#[cfg(test)]
mod tests {
    use super::{http_date_to_zulu, is_http_url};

    #[test]
    fn url_detection_recognizes_http_https_case_insensitive() {
        // The bare http:// literals in this test are detector inputs,
        // not URLs to fetch — that's exactly what is_http_url is
        // testing. Suppress DevSkim's TLS-URL warning rule.
        assert!(is_http_url("http://example.gov/d.csv")); // DevSkim: ignore DS137138
        assert!(is_http_url("https://example.gov/d.csv"));
        assert!(is_http_url("HTTPS://example.gov/d.csv"));
        assert!(is_http_url("Http://example.gov/d.csv")); // DevSkim: ignore DS137138
        assert!(!is_http_url("/tmp/local.csv"));
        assert!(!is_http_url("file:///tmp/x.csv"));
        assert!(!is_http_url("ftp://example.com/x.csv"));
        assert!(!is_http_url(""));
    }

    #[test]
    fn http_date_normalizes_to_zulu() {
        // RFC 9110 §5.6.7's own example IMF-fixdate.
        assert_eq!(
            http_date_to_zulu("Sun, 06 Nov 1994 08:49:37 GMT").as_deref(),
            Some("1994-11-06T08:49:37Z")
        );
        // A non-GMT offset (not HTTP-conformant, but seen) still lands in UTC.
        assert_eq!(
            http_date_to_zulu("Sun, 06 Nov 1994 03:49:37 -0500").as_deref(),
            Some("1994-11-06T08:49:37Z")
        );
        assert_eq!(http_date_to_zulu("yesterday"), None);
        assert_eq!(http_date_to_zulu(""), None);
    }

    use serde_json::json;

    use super::apply_pointer_overrides;

    #[test]
    fn pointer_overrides_set_existing_leaf() {
        let mut root = json!({"projection": {"dct:title": "old"}});
        let overrides = json!({"/projection/dct:title": "new"})
            .as_object()
            .unwrap()
            .clone();
        apply_pointer_overrides(&mut root, &overrides);
        assert_eq!(
            root.pointer("/projection/dct:title")
                .and_then(|v| v.as_str()),
            Some("new")
        );
    }

    #[test]
    fn pointer_overrides_create_missing_parents() {
        let mut root = json!({});
        let overrides = json!({"/projection/dcat-us:bureauCode": ["015:11"]})
            .as_object()
            .unwrap()
            .clone();
        apply_pointer_overrides(&mut root, &overrides);
        assert_eq!(
            root.pointer("/projection/dcat-us:bureauCode/0")
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

    // merge_discovered tests removed — discovery merge now lives in
    // `super::discovery_merge::merge` and is covered by that module's
    // unit tests plus the new integration tests in tests/test_profile.rs
    // (Roborev PR-#3908 Copilot finding).

    // RFC 6901 escape coverage lives in
    // `discovery_merge::tests::escape_token_handles_rfc6901_round_trip`;
    // profile.rs's own escape helper was deleted (PR follow-up
    // sweep) since it had no non-test callers.

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

    use super::{is_uuid_like, url_title_default};

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
    fn url_title_walks_past_uuid_to_parent_segment() {
        // §5.9: CKAN's `/datastore/dump/<uuid>` shape — walk one level up
        // past the UUID basename to yield "dump", which is meaningful
        // (and much better than a random tempfile suffix or an opaque hex).
        assert_eq!(
            url_title_default(
                "https://data.wprdc.org/datastore/dump/5202679a-d243-402e-b82a-63189995a942"
            ),
            Some("dump".to_string())
        );
        // Compact 32-hex variant — same treatment.
        assert_eq!(
            url_title_default(
                "https://example.gov/path/snapshots/5202679ad243402eb82a63189995a942"
            ),
            Some("snapshots".to_string())
        );
    }

    #[test]
    fn url_title_returns_leaf_when_all_parents_uuid_like() {
        // Degenerate case: every segment in the leaf-most 3 is UUID-like.
        // We fall back to the leaf UUID — still reproducible / traceable,
        // strictly better than the random tempfile suffix.
        let leaf = "5202679a-d243-402e-b82a-63189995a942";
        let mid = "abcdef01-2345-6789-abcd-ef0123456789";
        let parent = "fedcba98-7654-3210-fedc-ba9876543210";
        let url = format!("https://example.gov/{parent}/{mid}/{leaf}");
        assert_eq!(url_title_default(&url), Some(leaf.to_string()));
    }

    #[test]
    fn url_title_does_not_walk_for_normal_basenames() {
        // Regression: non-UUID basenames pass straight through; no walking.
        assert_eq!(
            url_title_default("https://example.gov/datastore/dump/2024-Q3-payments.csv"),
            Some("2024-Q3-payments".to_string()),
        );
        // Length-collision check: 36-char non-hex must NOT be misclassified.
        assert_eq!(
            url_title_default("https://example.gov/path/this-is-thirty-six-chars-long-XYZQRS"),
            Some("this-is-thirty-six-chars-long-XYZQRS".to_string()),
        );
    }

    #[test]
    fn is_uuid_like_recognizes_both_canonical_and_compact_forms() {
        // Canonical 8-4-4-4-12 with dashes.
        assert!(is_uuid_like("5202679a-d243-402e-b82a-63189995a942"));
        // Mixed case.
        assert!(is_uuid_like("5202679A-D243-402E-B82A-63189995A942"));
        // Compact 32 hex.
        assert!(is_uuid_like("5202679ad243402eb82a63189995a942")); // devskim: ignore DS173237
        // Negatives:
        assert!(!is_uuid_like("dump"));
        assert!(!is_uuid_like("2024-Q3-payments"));
        // Right length, wrong char.
        assert!(!is_uuid_like("5202679a-d243-402e-b82a-63189995a94Z"));
        // Right length, dashes in wrong place.
        assert!(!is_uuid_like("5202679a-d243-402-eb82a-63189995a942"));
        // Compact wrong length.
        assert!(!is_uuid_like("5202679ad243402eb82a"));
        // Compact non-hex.
        assert!(!is_uuid_like("zzzzzzzzd243402eb82a63189995a942"));
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
            "projection": {
                "dcat:distribution": [
                    {"@type": "dcat:Distribution", "dct:license": "old"}
                ]
            }
        });
        let overrides = json!({"/projection/dcat:distribution/0/dct:license": "new"})
            .as_object()
            .unwrap()
            .clone();
        apply_pointer_overrides(&mut root, &overrides);
        // Array shape must be preserved
        assert!(
            root.pointer("/projection/dcat:distribution")
                .is_some_and(|v| v.is_array()),
            "distribution must remain an array, got: {root:#}"
        );
        assert_eq!(
            root.pointer("/projection/dcat:distribution/0/dct:license")
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
