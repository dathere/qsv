use serde_json::Value;

use crate::workdir::Workdir;

const DRUF_SPEC: &str = include_str!("resources/profile/dataset-druf.yaml");

/// Drop a 5-row geo CSV into the workdir.
fn seed_geo_csv(wrk: &Workdir) {
    wrk.create(
        "in.csv",
        vec![
            svec!["id", "name", "created_at", "latitude", "longitude", "kind"],
            svec!["1", "Alpha", "2024-01-15", "40.7128", "-74.0060", "store"],
            svec!["2", "Bravo", "2024-02-20", "34.0522", "-118.2437", "store"],
            svec![
                "3",
                "Charlie",
                "2024-03-10",
                "41.8781",
                "-87.6298",
                "office"
            ],
            svec!["4", "Delta", "2024-04-25", "29.7604", "-95.3698", "store"],
            svec!["5", "Echo", "2024-05-30", "33.4484", "-112.0740", "office"],
        ],
    );
}

fn write_spec(wrk: &Workdir, name: &str) -> std::path::PathBuf {
    let path = wrk.path(name);
    std::fs::write(&path, DRUF_SPEC).unwrap();
    path
}

fn read_output(wrk: &Workdir, name: &str) -> Value {
    let raw = std::fs::read_to_string(wrk.path(name)).unwrap();
    serde_json::from_str(&raw).unwrap()
}

/// Strip the only path-dependent field (qsv:sourcePath) from a dcat
/// block so it matches the goldens captured by jq normalization in
/// Stage 2 of the YAML-engine migration.
fn normalize_dcat_for_parity(mut dcat: Value) -> Value {
    fn strip_dist(dist_array: &mut Value) {
        if let Some(arr) = dist_array.as_array_mut() {
            for d in arr {
                if let Some(o) = d.as_object_mut() {
                    o.remove("qsv:sourcePath");
                }
            }
        }
    }
    if let Some(obj) = dcat.as_object_mut() {
        if let Some(d) = obj.get_mut("dcat:distribution") {
            strip_dist(d);
        }
        // Catalog mode nests the Dataset under dcat:dataset[0].
        if let Some(Value::Array(arr)) = obj.get_mut("dcat:dataset") {
            for ds in arr {
                if let Some(ds_obj) = ds.as_object_mut()
                    && let Some(d) = ds_obj.get_mut("dcat:distribution")
                {
                    strip_dist(d);
                }
            }
        }
    }
    dcat
}

#[test]
fn dcat_us_v3_golden_parity_dataset() {
    for fix in ["nyc-311-subset", "usda-soil-subset", "wprdc-311-subset"] {
        let wrk = Workdir::new(&format!("parity_dataset_{fix}"));
        let src = format!("tests/resources/profile/golden/{fix}.csv");
        let abs_src = std::env::current_dir().unwrap().join(&src);
        std::fs::copy(&abs_src, wrk.path("in.csv")).expect("copy fixture");
        let ic_src = std::env::current_dir()
            .unwrap()
            .join("tests/resources/profile/dcat-init-context.json");
        std::fs::copy(&ic_src, wrk.path("ic.json")).expect("copy ic");

        let mut cmd = wrk.command("profile");
        cmd.args([
            "in.csv",
            "--profile",
            "dcat-us-v3",
            "--initial-context",
            "ic.json",
            "-o",
            "out.json",
        ]);
        wrk.assert_success(&mut cmd);

        let out = read_output(&wrk, "out.json");
        let actual = normalize_dcat_for_parity(out["dcat"].clone());
        let golden_path = format!("tests/resources/profile/golden/{fix}.dataset.expected.json");
        let golden_raw =
            std::fs::read_to_string(std::env::current_dir().unwrap().join(&golden_path))
                .expect("read golden");
        let golden: Value = serde_json::from_str(&golden_raw).expect("parse golden");
        assert_eq!(actual, golden, "dcat-us-v3 dataset parity drift on `{fix}`");
    }
}

#[test]
fn dcat_us_v3_golden_parity_catalog() {
    for fix in ["nyc-311-subset", "usda-soil-subset", "wprdc-311-subset"] {
        let wrk = Workdir::new(&format!("parity_catalog_{fix}"));
        let src = format!("tests/resources/profile/golden/{fix}.csv");
        let abs_src = std::env::current_dir().unwrap().join(&src);
        std::fs::copy(&abs_src, wrk.path("in.csv")).expect("copy fixture");
        let ic_src = std::env::current_dir()
            .unwrap()
            .join("tests/resources/profile/dcat-init-context.json");
        std::fs::copy(&ic_src, wrk.path("ic.json")).expect("copy ic");

        let mut cmd = wrk.command("profile");
        cmd.args([
            "in.csv",
            "--profile",
            "dcat-us-v3",
            "--catalog",
            "--initial-context",
            "ic.json",
            "-o",
            "out.json",
        ]);
        wrk.assert_success(&mut cmd);

        let out = read_output(&wrk, "out.json");
        let actual = normalize_dcat_for_parity(out["dcat"].clone());
        let golden_path = format!("tests/resources/profile/golden/{fix}.catalog.expected.json");
        let golden_raw =
            std::fs::read_to_string(std::env::current_dir().unwrap().join(&golden_path))
                .expect("read golden");
        let golden: Value = serde_json::from_str(&golden_raw).expect("parse golden");
        assert_eq!(actual, golden, "dcat-us-v3 catalog parity drift on `{fix}`");
    }
}

// =========================================================================
// DCAT-AP v3 profile smoke tests
// =========================================================================

#[test]
fn dcat_ap_v3_emits_no_dcat_us_extensions() {
    // DCAT-AP intentionally drops dcat-us:* extensions (bureauCode,
    // programCode, accessLevel, purpose, liabilityStatement). The
    // profile must not surface them even when the initial-context
    // would have provided values.
    let wrk = Workdir::new("dcat_ap_v3_no_us_ext");
    let src = std::env::current_dir()
        .unwrap()
        .join("tests/resources/profile/golden/nyc-311-subset.csv");
    std::fs::copy(&src, wrk.path("in.csv")).expect("copy fixture");
    let ic_src = std::env::current_dir()
        .unwrap()
        .join("tests/resources/profile/dcat-init-context.json");
    std::fs::copy(&ic_src, wrk.path("ic.json")).expect("copy ic");

    let mut cmd = wrk.command("profile");
    cmd.args([
        "in.csv",
        "--profile",
        "dcat-ap-v3",
        "--initial-context",
        "ic.json",
        "-o",
        "out.json",
    ]);
    wrk.assert_success(&mut cmd);

    let out = read_output(&wrk, "out.json");
    let dcat_keys: Vec<String> = out
        .pointer("/dcat")
        .and_then(|v| v.as_object())
        .map(|o| o.keys().cloned().collect())
        .unwrap_or_default();
    let us_keys: Vec<&String> = dcat_keys
        .iter()
        .filter(|k| k.starts_with("dcat-us:"))
        .collect();
    assert!(
        us_keys.is_empty(),
        "DCAT-AP must not surface dcat-us:* extensions, found: {us_keys:?}",
    );
}

#[test]
fn dcat_ap_v3_distribution_carries_access_url() {
    // DCAT-AP mandates dcat:accessURL on every Distribution.
    let wrk = Workdir::new("dcat_ap_v3_access_url");
    let src = std::env::current_dir()
        .unwrap()
        .join("tests/resources/profile/golden/nyc-311-subset.csv");
    std::fs::copy(&src, wrk.path("in.csv")).expect("copy fixture");
    let ic_src = std::env::current_dir()
        .unwrap()
        .join("tests/resources/profile/dcat-init-context.json");
    std::fs::copy(&ic_src, wrk.path("ic.json")).expect("copy ic");

    let mut cmd = wrk.command("profile");
    cmd.args([
        "in.csv",
        "--profile",
        "dcat-ap-v3",
        "--initial-context",
        "ic.json",
        "-o",
        "out.json",
    ]);
    wrk.assert_success(&mut cmd);

    let out = read_output(&wrk, "out.json");
    let access_url = out
        .pointer("/dcat/dcat:distribution/0/dcat:accessURL")
        .and_then(|v| v.as_str())
        .expect("dcat:accessURL on Distribution[0]");
    assert!(
        access_url.starts_with("http"),
        "dcat:accessURL must be an absolute IRI, got `{access_url}`",
    );
}

#[test]
fn dcat_ap_v3_conforms_to_targets_spec_url() {
    // dct:conformsTo must point at the DCAT-AP v3 release URL — that's
    // how downstream consumers detect the profile.
    let wrk = Workdir::new("dcat_ap_v3_conforms_to");
    let src = std::env::current_dir()
        .unwrap()
        .join("tests/resources/profile/golden/nyc-311-subset.csv");
    std::fs::copy(&src, wrk.path("in.csv")).expect("copy fixture");

    let mut cmd = wrk.command("profile");
    cmd.args(["in.csv", "--profile", "dcat-ap-v3", "-o", "out.json"]);
    wrk.assert_success(&mut cmd);

    let out = read_output(&wrk, "out.json");
    let conforms = out
        .pointer("/dcat/dct:conformsTo/0/@id")
        .and_then(|v| v.as_str())
        .expect("dct:conformsTo[0].@id");
    assert!(
        conforms.contains("semiceu.github.io/DCAT-AP"),
        "dct:conformsTo target must reference DCAT-AP spec, got `{conforms}`",
    );
}

#[test]
fn dcat_ap_v3_validation_is_disabled_noop() {
    // DCAT-AP ships SHACL, not JSON Schema. --validate-dcat with this
    // profile must succeed without producing schema-level violations
    // (in-projection required_level warnings are still allowed).
    let wrk = Workdir::new("dcat_ap_v3_validation_off");
    let src = std::env::current_dir()
        .unwrap()
        .join("tests/resources/profile/golden/nyc-311-subset.csv");
    std::fs::copy(&src, wrk.path("in.csv")).expect("copy fixture");
    let ic_src = std::env::current_dir()
        .unwrap()
        .join("tests/resources/profile/dcat-init-context.json");
    std::fs::copy(&ic_src, wrk.path("ic.json")).expect("copy ic");

    let mut cmd = wrk.command("profile");
    cmd.args([
        "in.csv",
        "--profile",
        "dcat-ap-v3",
        "--initial-context",
        "ic.json",
        "--validate-dcat",
        "-o",
        "out.json",
    ]);
    wrk.assert_success(&mut cmd);

    let out = read_output(&wrk, "out.json");
    let warnings = out
        .get("dcat_warnings")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    // Filter out the "minijinja could not render X" / required-level
    // warnings — those aren't schema validation noise. We just want
    // no schema-validator errors to slip through.
    let schema_warnings: Vec<_> = warnings
        .iter()
        .filter(|w| {
            w.get("field")
                .and_then(|v| v.as_str())
                .is_some_and(|f| f == "dcat_validate")
        })
        .collect();
    assert!(
        schema_warnings.is_empty(),
        "DCAT-AP profile must not invoke JSON Schema validator, got: {schema_warnings:#?}",
    );
}

// =========================================================================
// Geoconnex profile smoke tests
// =========================================================================
// Phase 1 dataset-level projection only (DatasetShape / ProviderShape /
// PublisherShape / DistributionShape from the upstream SHACL). The
// row-per-feature LocationOrientedShape is deferred to a follow-up PR
// that introduces a `for_each_row` projection mode.
//
// Gated behind the `geoconnex` cargo feature — present in qsv (via
// `distrib_features`) and as an opt-in for qsvdp; absent from qsvlite
// / qsvmcp. The test runner picks up these tests only when the
// integration-test crate is built with the feature on.

#[cfg(feature = "geoconnex")]
#[test]
fn geoconnex_emits_schema_dataset_type() {
    // Phase 1 emits a schema.org-rooted Dataset, NOT a dcat:Dataset.
    // The SHACL DatasetShape targets schema:Dataset by class, so the
    // top-level @type is what triggers SHACL evaluation downstream.
    let wrk = Workdir::new("geoconnex_dataset_type");
    let src = std::env::current_dir()
        .unwrap()
        .join("tests/resources/profile/golden/nyc-311-subset.csv");
    std::fs::copy(&src, wrk.path("in.csv")).expect("copy fixture");

    let mut cmd = wrk.command("profile");
    cmd.args(["in.csv", "--profile", "geoconnex", "-o", "out.json"]);
    wrk.assert_success(&mut cmd);

    let out = read_output(&wrk, "out.json");
    assert_eq!(
        out.pointer("/dcat/@type").and_then(|v| v.as_str()),
        Some("schema:Dataset"),
        "Geoconnex Dataset @type must be schema:Dataset",
    );
}

#[cfg(feature = "geoconnex")]
#[test]
fn geoconnex_context_declares_schema_org_with_trailing_slash() {
    // pyshacl expands prefixes by literal concatenation, so the
    // schema.org IRI in @context MUST have the trailing slash —
    // `https://schema.org/` exactly. A missing slash would make
    // every schema:* triple un-matchable against the SHACL graph
    // (which uses the same `@prefix schema: <https://schema.org/>`
    // declaration).
    let wrk = Workdir::new("geoconnex_context_trailing_slash");
    let src = std::env::current_dir()
        .unwrap()
        .join("tests/resources/profile/golden/nyc-311-subset.csv");
    std::fs::copy(&src, wrk.path("in.csv")).expect("copy fixture");

    let mut cmd = wrk.command("profile");
    cmd.args(["in.csv", "--profile", "geoconnex", "-o", "out.json"]);
    wrk.assert_success(&mut cmd);

    let out = read_output(&wrk, "out.json");
    let context = out
        .pointer("/dcat/@context")
        .and_then(|v| v.as_object())
        .expect("@context object");
    assert_eq!(
        context.get("schema").and_then(|v| v.as_str()),
        Some("https://schema.org/"),
        "Geoconnex @context.schema must be `https://schema.org/` (trailing slash is load-bearing)",
    );
    // dc: must resolve to dcterms — same IRI DCAT uses for `dct:`
    // (the SHACL Turtle uses the `dc:` prefix shorthand).
    assert_eq!(
        context.get("dc").and_then(|v| v.as_str()),
        Some("http://purl.org/dc/terms/"),
        "Geoconnex @context.dc must be `http://purl.org/dc/terms/`",
    );
}

#[cfg(feature = "geoconnex")]
#[test]
fn geoconnex_provider_is_always_emitted() {
    // SHACL DatasetShape has `sh:property [ sh:path schema:provider;
    // sh:minCount 1 ]` — provider is mandatory. The profile emits an
    // "Unknown" fallback when no publisher/author is available so the
    // SHACL constraint always passes structurally; users supplying
    // their own publisher info via --initial-context override the
    // fallback. ProviderShape itself requires only schema:name
    // (minCount 1), which the fallback satisfies.
    let wrk = Workdir::new("geoconnex_provider_always");
    let src = std::env::current_dir()
        .unwrap()
        .join("tests/resources/profile/golden/nyc-311-subset.csv");
    std::fs::copy(&src, wrk.path("in.csv")).expect("copy fixture");

    let mut cmd = wrk.command("profile");
    cmd.args(["in.csv", "--profile", "geoconnex", "-o", "out.json"]);
    wrk.assert_success(&mut cmd);

    let out = read_output(&wrk, "out.json");
    let provider = out
        .pointer("/dcat/schema:provider")
        .expect("schema:provider must be emitted unconditionally");
    let provider_type = provider
        .pointer("/@type")
        .and_then(|v| v.as_str())
        .expect("provider @type");
    assert_eq!(
        provider_type, "schema:Organization",
        "provider @type must be schema:Organization (one of the SHACL-allowed classes)",
    );
    assert!(
        provider
            .pointer("/schema:name")
            .and_then(|v| v.as_str())
            .is_some_and(|s| !s.is_empty()),
        "provider must carry a non-empty schema:name to satisfy ProviderShape minCount 1",
    );
}

#[cfg(feature = "geoconnex")]
#[test]
fn geoconnex_validation_is_disabled_noop() {
    // Geoconnex ships SHACL upstream, not JSON Schema. --validate-dcat
    // with this profile must succeed without producing any
    // dcat_validate-field warnings (the in-process JSON-Schema
    // validator is gated by validation.enabled = false). pyshacl-side
    // findings, if any, surface under `external_validate`; we don't
    // assert on those here because pyshacl may not be installed in CI.
    let wrk = Workdir::new("geoconnex_validation_off");
    let src = std::env::current_dir()
        .unwrap()
        .join("tests/resources/profile/golden/nyc-311-subset.csv");
    std::fs::copy(&src, wrk.path("in.csv")).expect("copy fixture");

    let mut cmd = wrk.command("profile");
    cmd.args([
        "in.csv",
        "--profile",
        "geoconnex",
        "--validate-dcat",
        "-o",
        "out.json",
    ]);
    wrk.assert_success(&mut cmd);

    let out = read_output(&wrk, "out.json");
    let warnings = out
        .get("dcat_warnings")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    let schema_warnings: Vec<_> = warnings
        .iter()
        .filter(|w| {
            w.get("field")
                .and_then(|v| v.as_str())
                .is_some_and(|f| f == "dcat_validate")
        })
        .collect();
    assert!(
        schema_warnings.is_empty(),
        "Geoconnex profile must not invoke JSON Schema validator, got: {schema_warnings:#?}",
    );
}

// =========================================================================
// Croissant 1.0 profile smoke tests
// =========================================================================

#[test]
fn croissant_uses_schema_org_context_and_sc_dataset_type() {
    let wrk = Workdir::new("croissant_schema_org");
    let src = std::env::current_dir()
        .unwrap()
        .join("tests/resources/profile/golden/nyc-311-subset.csv");
    std::fs::copy(&src, wrk.path("in.csv")).expect("copy fixture");

    let mut cmd = wrk.command("profile");
    cmd.args(["in.csv", "--profile", "croissant", "-o", "out.json"]);
    wrk.assert_success(&mut cmd);

    let out = read_output(&wrk, "out.json");
    assert_eq!(
        out.pointer("/dcat/@type").and_then(|v| v.as_str()),
        Some("sc:Dataset"),
        "Croissant Dataset @type must be sc:Dataset",
    );
    let context = out
        .pointer("/dcat/@context")
        .and_then(|v| v.as_object())
        .expect("@context object");
    assert_eq!(
        context.get("@vocab").and_then(|v| v.as_str()),
        Some("https://schema.org/"),
        "Croissant @context.@vocab must be schema.org",
    );
}

#[test]
fn croissant_conforms_to_targets_mlcommons_spec() {
    let wrk = Workdir::new("croissant_conforms_to");
    let src = std::env::current_dir()
        .unwrap()
        .join("tests/resources/profile/golden/nyc-311-subset.csv");
    std::fs::copy(&src, wrk.path("in.csv")).expect("copy fixture");

    let mut cmd = wrk.command("profile");
    cmd.args(["in.csv", "--profile", "croissant", "-o", "out.json"]);
    wrk.assert_success(&mut cmd);

    let out = read_output(&wrk, "out.json");
    let conforms = out
        .pointer("/dcat/conformsTo")
        .and_then(|v| v.as_str())
        .expect("conformsTo");
    assert!(
        conforms.contains("mlcommons.org/croissant"),
        "conformsTo must reference Croissant spec, got `{conforms}`",
    );
}

#[test]
fn croissant_emits_recordset_with_one_field_per_csv_column() {
    let wrk = Workdir::new("croissant_recordset");
    let src = std::env::current_dir()
        .unwrap()
        .join("tests/resources/profile/golden/nyc-311-subset.csv");
    std::fs::copy(&src, wrk.path("in.csv")).expect("copy fixture");

    let mut cmd = wrk.command("profile");
    cmd.args(["in.csv", "--profile", "croissant", "-o", "out.json"]);
    wrk.assert_success(&mut cmd);

    let out = read_output(&wrk, "out.json");
    let record_sets = out
        .pointer("/dcat/recordSet")
        .and_then(|v| v.as_array())
        .expect("recordSet array");
    assert_eq!(
        record_sets.len(),
        1,
        "Croissant minimal Dataset has 1 RecordSet"
    );
    assert_eq!(
        record_sets[0].get("@type").and_then(|v| v.as_str()),
        Some("cr:RecordSet"),
    );
    let fields = record_sets[0]
        .get("field")
        .and_then(|v| v.as_array())
        .expect("recordSet[0].field array");
    // nyc-311-subset.csv has 10 columns.
    assert_eq!(fields.len(), 10, "must emit one cr:Field per CSV column");
    // All fields are cr:Field with a schema.org dataType.
    for f in fields {
        assert_eq!(
            f.get("@type").and_then(|v| v.as_str()),
            Some("cr:Field"),
            "every field entry must be cr:Field",
        );
        let dtype = f
            .get("dataType")
            .and_then(|v| v.as_str())
            .expect("field.dataType");
        assert!(
            dtype.starts_with("sc:"),
            "dataType must use schema.org vocab, got `{dtype}`",
        );
    }
}

#[test]
fn croissant_uses_bare_distribution_key_not_dcat_namespaced() {
    // Croissant's @vocab=schema.org resolves bare `distribution` →
    // schema.org/distribution. DCAT-namespaced `dcat:distribution`
    // would break the JSON-LD interpretation.
    let wrk = Workdir::new("croissant_distribution_key");
    let src = std::env::current_dir()
        .unwrap()
        .join("tests/resources/profile/golden/nyc-311-subset.csv");
    std::fs::copy(&src, wrk.path("in.csv")).expect("copy fixture");

    let mut cmd = wrk.command("profile");
    cmd.args(["in.csv", "--profile", "croissant", "-o", "out.json"]);
    wrk.assert_success(&mut cmd);

    let out = read_output(&wrk, "out.json");
    assert!(
        out.pointer("/dcat/distribution").is_some(),
        "Croissant Dataset must carry bare `distribution`",
    );
    assert!(
        out.pointer("/dcat/dcat:distribution").is_none(),
        "Croissant Dataset must not carry dcat:distribution",
    );
}

#[test]
fn croissant_distribution_uses_file_object_type() {
    let wrk = Workdir::new("croissant_file_object");
    let src = std::env::current_dir()
        .unwrap()
        .join("tests/resources/profile/golden/nyc-311-subset.csv");
    std::fs::copy(&src, wrk.path("in.csv")).expect("copy fixture");

    let mut cmd = wrk.command("profile");
    cmd.args(["in.csv", "--profile", "croissant", "-o", "out.json"]);
    wrk.assert_success(&mut cmd);

    let out = read_output(&wrk, "out.json");
    let file_obj_type = out
        .pointer("/dcat/distribution/0/@type")
        .and_then(|v| v.as_str())
        .expect("distribution[0].@type");
    assert_eq!(file_obj_type, "sc:FileObject");
}

// =========================================================================
// Roborev #2490 regression guards
// =========================================================================

#[test]
fn catalog_envelope_carries_top_level_context() {
    // Finding #2: the Catalog envelope contains CURIE keys
    // (`dct:title`, `dct:conformsTo`, `dcat:dataset`) so it needs its
    // own @context to be valid JSON-LD. Without one downstream
    // JSON-LD consumers can't resolve the outer keys.
    let wrk = Workdir::new("catalog_context_guard");
    seed_geo_csv(&wrk);
    let mut cmd = wrk.command("profile");
    cmd.args(["in.csv", "--catalog", "-o", "out.json"]);
    wrk.assert_success(&mut cmd);
    let out = read_output(&wrk, "out.json");
    let context = out
        .pointer("/dcat/@context")
        .and_then(|v| v.as_str())
        .expect("Catalog envelope must carry @context");
    assert!(
        context.contains("doi-do") || context.contains("dcat-us"),
        "Catalog @context must be the DCAT-US context URI, got `{context}`",
    );
}

#[test]
fn catalog_mode_merges_discovered_into_inner_dataset_not_envelope() {
    // Finding #1: in Catalog mode, discovered metadata must land on
    // the inner Dataset (dcat:dataset[0]), not on the outer Catalog
    // envelope. The test runs through the orchestrator end-to-end;
    // since seed_geo_csv has no URL-discovered DCAT it's a structural
    // assertion that envelope keys don't accidentally pick up
    // Dataset-only fields (`dct:contactPoint`, `dcat:keyword`, etc).
    let wrk = Workdir::new("catalog_merge_target");
    seed_geo_csv(&wrk);
    let mut cmd = wrk.command("profile");
    cmd.args(["in.csv", "--catalog", "-o", "out.json"]);
    wrk.assert_success(&mut cmd);
    let out = read_output(&wrk, "out.json");
    // Outer envelope keys: @context, @type, dct:title, dct:conformsTo,
    // dcat:dataset, plus optionally dct:publisher.
    let envelope_keys: Vec<String> = out
        .pointer("/dcat")
        .and_then(|v| v.as_object())
        .map(|o| o.keys().cloned().collect())
        .unwrap_or_default();
    let leaked = envelope_keys.iter().find(|k| {
        matches!(
            k.as_str(),
            "dcat:contactPoint" | "dcat:keyword" | "dcat:theme" | "dct:spatial" | "dct:temporal"
        )
    });
    assert!(
        leaked.is_none(),
        "Catalog envelope must not carry Dataset-only keys (found `{leaked:?}`)",
    );
    // The Dataset keys must live in dcat:dataset[0].
    let inner_ds = out
        .pointer("/dcat/dcat:dataset/0")
        .and_then(|v| v.as_object())
        .expect("dcat:dataset[0] missing");
    assert!(
        inner_ds.contains_key("dct:title"),
        "inner Dataset must carry dct:title",
    );
}

#[test]
fn spatial_field_suppressed_when_no_lat_lon_columns() {
    // Finding #3: bbox_from_dpps returning UNDEFINED previously
    // rendered as the literal string "null" via `| tojson` because
    // coerce_json_or_string left non-{,[ strings alone. The
    // emit_when guard now suppresses the field entirely. Fixture:
    // usda-soil-subset.csv has no lat/lon columns.
    let wrk = Workdir::new("spatial_no_latlon");
    let src = std::env::current_dir()
        .unwrap()
        .join("tests/resources/profile/golden/usda-soil-subset.csv");
    std::fs::copy(&src, wrk.path("in.csv")).expect("copy fixture");
    let mut cmd = wrk.command("profile");
    cmd.args(["in.csv", "-o", "out.json"]);
    wrk.assert_success(&mut cmd);
    let out = read_output(&wrk, "out.json");
    assert!(
        out.pointer("/dcat/dct:spatial").is_none(),
        "dct:spatial must be absent when no bbox is available; got `{:?}`",
        out.pointer("/dcat/dct:spatial"),
    );
}

#[test]
fn dcat_legacy_license_emits_dataset_level_license() {
    // Finding #4: --dcat-legacy-license previously parsed but didn't
    // thread into the projection context. With the flag set, the
    // YAML's gated dct:license template must emit on the Dataset
    // alongside the Distribution-level copy.
    let wrk = Workdir::new("dcat_legacy_license");
    seed_geo_csv(&wrk);
    let ctx_path = wrk.path("init.json");
    std::fs::write(&ctx_path, r#"{"package": {"license_id": "cc-by"}}"#).unwrap();
    let mut cmd = wrk.command("profile");
    cmd.args([
        "in.csv",
        "--dcat-legacy-license",
        "--initial-context",
        ctx_path.to_str().unwrap(),
        "-o",
        "out.json",
    ]);
    wrk.assert_success(&mut cmd);
    let out = read_output(&wrk, "out.json");
    let dataset_license = out
        .pointer("/dcat/dct:license")
        .and_then(|v| v.as_str())
        .expect("--dcat-legacy-license must emit dct:license on Dataset");
    assert!(
        dataset_license.contains("creativecommons.org"),
        "Dataset-level license must be the resolved IRI, got `{dataset_license}`",
    );
    // Distribution-level license must STILL be there (v3 mandate).
    assert!(
        out.pointer("/dcat/dcat:distribution/0/dct:license")
            .is_some(),
        "Distribution-level dct:license must also be present (strict v3)",
    );
}

#[test]
fn dcat_legacy_license_off_keeps_license_distribution_only() {
    // Companion to the above: without --dcat-legacy-license, the
    // Dataset must NOT carry dct:license (strict v3 default).
    let wrk = Workdir::new("dcat_legacy_license_off");
    seed_geo_csv(&wrk);
    let ctx_path = wrk.path("init.json");
    std::fs::write(&ctx_path, r#"{"package": {"license_id": "cc-by"}}"#).unwrap();
    let mut cmd = wrk.command("profile");
    cmd.args([
        "in.csv",
        "--initial-context",
        ctx_path.to_str().unwrap(),
        "-o",
        "out.json",
    ]);
    wrk.assert_success(&mut cmd);
    let out = read_output(&wrk, "out.json");
    assert!(
        out.pointer("/dcat/dct:license").is_none(),
        "strict v3 must NOT emit dct:license on Dataset by default",
    );
}

#[test]
fn forced_package_publisher_flows_through_profile_template() {
    // Finding #5: forcing package.publisher previously wrote a raw
    // string to dct:publisher (bypassing the foaf:Agent wrapper). The
    // fix routes CKAN-side forces through normal projection so the
    // template still wraps the value as an Agent object.
    let wrk = Workdir::new("force_publisher_shape");
    seed_geo_csv(&wrk);
    let ctx_path = wrk.path("init.json");
    std::fs::write(
        &ctx_path,
        r#"{"package": {"publisher": {"value": "Forced Publisher", "force": true}}}"#,
    )
    .unwrap();
    let mut cmd = wrk.command("profile");
    cmd.args([
        "in.csv",
        "--initial-context",
        ctx_path.to_str().unwrap(),
        "-o",
        "out.json",
    ]);
    wrk.assert_success(&mut cmd);
    let out = read_output(&wrk, "out.json");
    let publisher = out
        .pointer("/dcat/dct:publisher")
        .expect("dct:publisher must be emitted");
    assert!(
        publisher.is_object(),
        "forced publisher must be a foaf:Agent object, got: {publisher:?}",
    );
    assert_eq!(
        publisher.pointer("/foaf:name").and_then(|v| v.as_str()),
        Some("Forced Publisher"),
    );
    assert_eq!(
        publisher.pointer("/@type").and_then(|v| v.as_str()),
        Some("foaf:Agent"),
    );
}

#[test]
fn forced_package_field_survives_formula_overwrite() {
    // Roborev #2491: regression for the #5 fix. A `force: true` value
    // on `package.title` must NOT be overwritten by a spec formula
    // that targets the same field. merge_formula_results now skips
    // fields recorded in `analysis.forced_package_fields`.
    let wrk = Workdir::new("force_beats_formula");
    seed_geo_csv(&wrk);

    // Spec with a dataset-scope formula that, absent the force, would
    // replace `package.title` with `"Formula Wins"`. The forced
    // initial-context value must beat it.
    let spec_path = wrk.path("spec.yaml");
    std::fs::write(
        &spec_path,
        r#"
scheming_version: 2
dataset_type: test
dataset_fields:
  - field_name: title
    formula: '"Formula Wins"'
"#,
    )
    .unwrap();

    let ctx_path = wrk.path("init.json");
    std::fs::write(
        &ctx_path,
        r#"{"package": {"title": {"value": "Forced Title", "force": true}}}"#,
    )
    .unwrap();

    let mut cmd = wrk.command("profile");
    cmd.args([
        "in.csv",
        "--spec",
        spec_path.to_str().unwrap(),
        "--initial-context",
        ctx_path.to_str().unwrap(),
        "-o",
        "out.json",
    ]);
    wrk.assert_success(&mut cmd);

    let out = read_output(&wrk, "out.json");
    let title = out
        .pointer("/dcat/dct:title")
        .and_then(|v| v.as_str())
        .expect("dct:title");
    assert_eq!(
        title, "Forced Title",
        "force:true must beat a spec formula targeting the same field",
    );
}

#[test]
fn forced_author_locks_publisher_alias() {
    // Roborev #2493: package.author and package.publisher both map to
    // /dcat/dct:publisher. Forcing one must lock the other — a
    // formula writing `publisher` mustn't be able to overwrite a
    // forced `author` value before projection.
    let wrk = Workdir::new("force_alias_publisher");
    seed_geo_csv(&wrk);

    let spec_path = wrk.path("spec.yaml");
    std::fs::write(
        &spec_path,
        r#"
scheming_version: 2
dataset_type: test
dataset_fields:
  - field_name: publisher
    formula: '"Formula Publisher"'
"#,
    )
    .unwrap();

    let ctx_path = wrk.path("init.json");
    std::fs::write(
        &ctx_path,
        r#"{"package": {"author": {"value": "Forced Author", "force": true}}}"#,
    )
    .unwrap();

    let mut cmd = wrk.command("profile");
    cmd.args([
        "in.csv",
        "--spec",
        spec_path.to_str().unwrap(),
        "--initial-context",
        ctx_path.to_str().unwrap(),
        "-o",
        "out.json",
    ]);
    wrk.assert_success(&mut cmd);

    let out = read_output(&wrk, "out.json");
    let publisher_name = out
        .pointer("/dcat/dct:publisher/foaf:name")
        .and_then(|v| v.as_str())
        .expect("dct:publisher.foaf:name");
    assert_eq!(
        publisher_name, "Forced Author",
        "forced package.author must lock the publisher alias against formula overwrite",
    );
}

#[test]
fn forced_license_id_locks_license_alias() {
    // Roborev #2493: resource.license and resource.license_id both
    // map to /dcat/dcat:distribution/0/dct:license. Forcing one
    // must lock the other against formula overwrite.
    let wrk = Workdir::new("force_alias_license");
    seed_geo_csv(&wrk);

    let spec_path = wrk.path("spec.yaml");
    std::fs::write(
        &spec_path,
        r#"
scheming_version: 2
dataset_type: test
resource_fields:
  - field_name: license
    formula: '"cc-by-sa"'
"#,
    )
    .unwrap();

    let ctx_path = wrk.path("init.json");
    std::fs::write(
        &ctx_path,
        r#"{"resource": {"license_id": {"value": "cc-by", "force": true}}}"#,
    )
    .unwrap();

    let mut cmd = wrk.command("profile");
    cmd.args([
        "in.csv",
        "--spec",
        spec_path.to_str().unwrap(),
        "--initial-context",
        ctx_path.to_str().unwrap(),
        "-o",
        "out.json",
    ]);
    wrk.assert_success(&mut cmd);

    let out = read_output(&wrk, "out.json");
    let license = out
        .pointer("/dcat/dcat:distribution/0/dct:license")
        .and_then(|v| v.as_str())
        .expect("Distribution.dct:license");
    // cc-by resolves to the CC-BY 4.0 IRI; cc-by-sa would resolve to
    // a different IRI. The forced cc-by must win.
    assert!(
        license.contains("creativecommons.org/licenses/by/4.0"),
        "forced resource.license_id (cc-by) must lock the license alias against formula \
         overwrite, got `{license}`",
    );
}

#[test]
fn profile_spec_less_emits_dpp_block() {
    let wrk = Workdir::new("profile_spec_less");
    seed_geo_csv(&wrk);

    let mut cmd = wrk.command("profile");
    cmd.args(["in.csv", "-o", "out.json"]);
    wrk.assert_success(&mut cmd);

    let out = read_output(&wrk, "out.json");

    // formulas_evaluated is false in spec-less mode.
    assert_eq!(out.get("formulas_evaluated"), Some(&Value::Bool(false)));

    // dpp.LAT_FIELD / LON_FIELD / DATE_FIELDS are inferred even without a spec.
    assert_eq!(
        out.pointer("/dpp/LAT_FIELD"),
        Some(&Value::String("latitude".into()))
    );
    assert_eq!(
        out.pointer("/dpp/LON_FIELD"),
        Some(&Value::String("longitude".into()))
    );
    let date_fields = out.pointer("/dpp/DATE_FIELDS").unwrap().as_array().unwrap();
    assert_eq!(date_fields, &vec![Value::String("created_at".into())]);
    assert_eq!(
        out.pointer("/dpp/dataset_stats/row_count"),
        Some(&serde_json::json!(5))
    );
    assert_eq!(
        out.pointer("/dpp/dataset_stats/column_count"),
        Some(&serde_json::json!(6))
    );

    // No spec -> no formula_results entries.
    let results = out.get("formula_results").unwrap().as_array().unwrap();
    assert!(
        results.is_empty(),
        "expected empty formula_results, got {results:?}"
    );

    // DCAT block is emitted by default. dct:spatial is an array of
    // dct:Location per DCAT-US v3; the bbox-derived POLYGON lives at
    // index 0 when no formula has run.
    let spatial = out.pointer("/dcat/dct:spatial").expect("dct:spatial");
    assert!(spatial.is_array(), "dct:spatial must be an array");
    let bbox = spatial
        .pointer("/0/dcat:bbox")
        .and_then(|v| v.as_str())
        .expect("dct:spatial[0].dcat:bbox str");
    assert!(
        bbox.contains("POLYGON"),
        "expected POLYGON bbox, got {bbox:?}"
    );

    // tableSchema includes one column per CSV header.
    let cols = out
        .pointer("/dcat/dcat:distribution/0/csvw:tableSchema/columns")
        .and_then(|v| v.as_array())
        .expect("csvw:tableSchema.columns");
    assert_eq!(cols.len(), 6);
    let datatypes: Vec<&str> = cols
        .iter()
        .filter_map(|c| c.get("datatype").and_then(|v| v.as_str()))
        .collect();
    assert!(datatypes.contains(&"integer"));
    assert!(datatypes.contains(&"double"));
    assert!(datatypes.contains(&"date"));
    assert!(datatypes.contains(&"string"));
}

#[test]
fn profile_with_druf_spec_evaluates_spatial_extent_wkt() {
    let wrk = Workdir::new("profile_druf_spatial_extent");
    seed_geo_csv(&wrk);
    let spec_path = write_spec(&wrk, "dataset-druf.yaml");

    let mut cmd = wrk.command("profile");
    cmd.args([
        "in.csv",
        "--spec",
        spec_path.to_str().unwrap(),
        "-o",
        "out.json",
    ]);
    wrk.assert_success(&mut cmd);

    let out = read_output(&wrk, "out.json");
    assert_eq!(out.get("formulas_evaluated"), Some(&Value::Bool(true)));

    // spatial_extent suggestion_formula must have rendered, no error.
    let results = out.get("formula_results").unwrap().as_array().unwrap();
    let se = results
        .iter()
        .find(|r| r.get("field_name").and_then(|v| v.as_str()) == Some("spatial_extent"))
        .expect("spatial_extent result");
    assert_eq!(se.get("error"), Some(&Value::Null));
    let value = se
        .get("value")
        .and_then(|v| v.as_str())
        .expect("rendered value");
    assert!(value.starts_with("SRID=4326;POLYGON(("));
    // Bounding box: longitude span -118.2437..-74.006, latitude 29.7604..41.8781
    assert!(value.contains("-118.2437"));
    assert!(value.contains("41.8781"));

    // Suggestion is merged into package.dpp_suggestions.
    let merged = out
        .pointer("/ckan/package/dpp_suggestions/spatial_extent/value")
        .and_then(|v| v.as_str())
        .expect("dpp_suggestions.spatial_extent.value");
    assert_eq!(merged, value);

    // DCAT spatial picks up the WKT via the GeoSPARQL wktLiteral path now
    // that the suggestion populated it. dct:spatial is an array per v3 —
    // the WKT Location lives at index 0.
    let wkt = out
        .pointer("/dcat/dct:spatial/0/locn:geometry/@value")
        .and_then(|v| v.as_str())
        .expect("dcat spatial wkt");
    assert_eq!(wkt, value);
}

#[test]
fn profile_no_dcat_flag_skips_dcat_block() {
    let wrk = Workdir::new("profile_no_dcat");
    seed_geo_csv(&wrk);

    let mut cmd = wrk.command("profile");
    cmd.args(["in.csv", "--no-dcat", "-o", "out.json"]);
    wrk.assert_success(&mut cmd);

    let out = read_output(&wrk, "out.json");
    assert!(out.get("dcat").is_none(), "expected no dcat block");
    assert!(
        out.get("ckan").is_some(),
        "ckan block should still be present"
    );
}

#[test]
fn profile_no_ckan_flag_skips_ckan_block() {
    let wrk = Workdir::new("profile_no_ckan");
    seed_geo_csv(&wrk);

    let mut cmd = wrk.command("profile");
    cmd.args(["in.csv", "--no-ckan", "-o", "out.json"]);
    wrk.assert_success(&mut cmd);

    let out = read_output(&wrk, "out.json");
    assert!(out.get("ckan").is_none(), "expected no ckan block");
    assert!(
        out.get("dcat").is_some(),
        "dcat block should still be present"
    );
}

#[test]
fn profile_stdin_input_is_accepted() {
    // §5.6: stdin is materialized to a tempfile internally so the rest of
    // the pipeline sees a normal file path. Pipe a small CSV in, assert
    // the command succeeds, and check that the output JSON labels the
    // input as "stdin" (rather than leaking the random tempfile path)
    // and lands at the default "stdin.metadata.json" in the workdir.
    //
    // Regression for roborev #2453: also assert the DCAT distribution's
    // `qsv:sourcePath` reads "stdin" — that field previously kept the
    // tempfile path even after the top-level `input` label was fixed.
    //
    // Regression for roborev #2454: also assert `dcat:byteSize` is
    // populated for stdin. An earlier attempt at the #2453 fix passed
    // the display label to `dcat::build` for the metadata read too,
    // which silently dropped byte-size info; the proper fix splits
    // `local_path` (real file for fs::metadata) from `source_label`
    // (display).
    let wrk = Workdir::new("profile_stdin");
    let mut cmd = wrk.command("profile");
    cmd.arg("-");
    cmd.stdin(std::process::Stdio::piped());
    let mut child = cmd.spawn().expect("spawn qsv profile");
    let payload = b"id,name\n1,alpha\n2,bravo\n3,charlie\n";
    {
        use std::io::Write;
        let stdin = child.stdin.as_mut().expect("child stdin");
        stdin.write_all(payload).expect("write stdin");
    }
    let output = child.wait_with_output().expect("wait qsv profile");
    assert!(
        output.status.success(),
        "expected success on stdin input, got status: {:?}\nstderr: {}",
        output.status,
        String::from_utf8_lossy(&output.stderr),
    );
    let out_path = wrk.path("stdin.metadata.json");
    assert!(
        out_path.exists(),
        "expected stdin.metadata.json to be written, missing at {out_path:?}",
    );
    let body = std::fs::read_to_string(&out_path).expect("read out");
    let parsed: serde_json::Value = serde_json::from_str(&body).expect("parse out");
    assert_eq!(
        parsed.get("input").and_then(|v| v.as_str()),
        Some("stdin"),
        "expected output.input to be \"stdin\" (no tempfile leak), got: {}",
        parsed
            .get("input")
            .map_or("<missing>".to_string(), |v| v.to_string()),
    );
    // dpp block (inferred metadata) is always emitted.
    assert!(parsed.get("dpp").is_some(), "missing dpp block: {body}");

    // DCAT distribution's qsv:sourcePath must also read "stdin", not the
    // tempfile path. This is the specific roborev #2453 regression check.
    let source_path = parsed
        .pointer("/dcat/dcat:distribution/0/qsv:sourcePath")
        .and_then(|v| v.as_str());
    assert_eq!(
        source_path,
        Some("stdin"),
        "expected dcat:distribution[0].qsv:sourcePath to be \"stdin\" (no tempfile leak), got: \
         {source_path:?}\nfull body: {body}",
    );

    // roborev #2454 regression: dcat:byteSize must still reflect the
    // materialized tempfile, even though the display label is "stdin".
    // Emitted as a string per GSA Distribution.json's
    // type=["null","string"] (xsd:nonNegativeInteger stored as string).
    let byte_size = parsed
        .pointer("/dcat/dcat:distribution/0/dcat:byteSize")
        .and_then(serde_json::Value::as_str)
        .and_then(|s| s.parse::<u64>().ok());
    assert_eq!(
        byte_size,
        Some(payload.len() as u64),
        "expected dcat:byteSize == {} (piped payload size), got: {byte_size:?}\nfull body: {body}",
        payload.len(),
    );
}

#[test]
fn profile_initial_context_seeds_package_and_overrides_via_dataset_info() {
    let wrk = Workdir::new("profile_init_context");
    seed_geo_csv(&wrk);

    // Minimal init-context: seeds package fields the projection reads,
    // then forces a JSON-Pointer override into the final output.
    let ctx_path = wrk.path("init.json");
    std::fs::write(
        &ctx_path,
        r#"{
            "package": {
                "title":        "Seeded From Init",
                "notes":        "loaded via --initial-context",
                "license_id":   "cc-by",
                "language":     "en-US",
                "metadata_modified": "R/P1Y"
            },
            "dataset_info": {
                "/dcat/dct:title": "Final Override Wins"
            }
        }"#,
    )
    .unwrap();

    let mut cmd = wrk.command("profile");
    cmd.args([
        "in.csv",
        "--initial-context",
        ctx_path.to_str().unwrap(),
        "-o",
        "out.json",
    ]);
    wrk.assert_success(&mut cmd);
    let out = read_output(&wrk, "out.json");

    // dataset_info override is last-write-wins.
    assert_eq!(
        out.pointer("/dcat/dct:title").and_then(|v| v.as_str()),
        Some("Final Override Wins"),
        "dataset_info JSON-Pointer override must win over the package seed"
    );

    // package.notes flows into the projection as dct:description.
    assert_eq!(
        out.pointer("/dcat/dct:description")
            .and_then(|v| v.as_str()),
        Some("loaded via --initial-context")
    );

    // language is normalized en-US → en (Phase 2d behaviour).
    assert_eq!(
        out.pointer("/dcat/dct:language").and_then(|v| v.as_str()),
        Some("en"),
    );

    // metadata_modified was a repeating-interval ("R/P1Y"); Phase 2e
    // sanitizer drops it so dct:modified is absent (frequency goes to
    // accrualPeriodicity, queued for Phase 5).
    assert!(
        out.pointer("/dcat/dct:modified").is_none(),
        "ISO 8601 interval must be rejected from dct:modified"
    );

    // license moved to Distribution in Phase 2c — must not appear on
    // the Dataset by default.
    assert!(
        out.pointer("/dcat/dct:license").is_none(),
        "dct:license must live on Distribution in strict v3"
    );
    let dist_license = out
        .pointer("/dcat/dcat:distribution/0/dct:license")
        .and_then(|v| v.as_str())
        .expect("dct:license on Distribution");
    assert!(dist_license.contains("creativecommons.org"));
}

#[test]
fn profile_with_full_initial_context_emits_all_recommended_v3_fields() {
    let wrk = Workdir::new("profile_full_v3");
    seed_geo_csv(&wrk);

    let ctx_path = wrk.path("init.json");
    std::fs::write(
        &ctx_path,
        r#"{
            "package": {
                "title":              "Demo Dataset",
                "notes":              "Full DCAT-US v3 population.",
                "name":               "demo-dataset",
                "license_id":         "cc-by",
                "publisher":          "Demo Agency",
                "metadata_modified":  "2024-12-15",
                "language":           "en-US",
                "contact_point":      {"fn": "Jane Doe", "hasEmail": "jane@example.gov"},
                "bureauCode":         ["015:11"],
                "programCode":        ["015:000"],
                "accrualPeriodicity": "annually",
                "accessRights":       "public",
                "rights":             "U.S. Government Work",
                "landing_page":       "https://example.gov/dataset",
                "describedBy":        "https://example.gov/dataset/schema.json",
                "purpose":            "Track example metric.",
                "scopeNote":          "Years 2020-2024 only.",
                "liabilityStatement": "As-is.",
                "inSeries":           "https://example.gov/series"
            },
            "resource": {
                "accessURL":          "https://example.gov/dataset",
                "last_modified":      "2024-12-15T08:30:00",
                "rights":             "U.S. Government Work",
                "access_restriction": {"type": "none"}
            }
        }"#,
    )
    .unwrap();

    let mut cmd = wrk.command("profile");
    cmd.args([
        "in.csv",
        "--initial-context",
        ctx_path.to_str().unwrap(),
        "-o",
        "out.json",
    ]);
    wrk.assert_success(&mut cmd);
    let out = read_output(&wrk, "out.json");

    // Mandatory v3 fields.
    for path in [
        "/dcat/dct:title",
        "/dcat/dct:description",
        "/dcat/dct:identifier",
        "/dcat/dct:publisher",
        "/dcat/dcat:contactPoint",
    ] {
        assert!(
            out.pointer(path).is_some(),
            "mandatory v3 field missing at {path}: {out:#}"
        );
    }
    // Recommended v3 fields added in Phase 5.
    for path in [
        "/dcat/dcat:landingPage",
        "/dcat/dcat:describedBy",
        "/dcat/dct:rights",
        "/dcat/dct:accessRights",
        "/dcat/dcat-us:bureauCode",
        "/dcat/dcat-us:programCode",
        "/dcat/dct:accrualPeriodicity",
        "/dcat/dcat-us:purpose",
        "/dcat/skos:scopeNote",
        "/dcat/dcat-us:liabilityStatement",
        "/dcat/dcat:inSeries",
        "/dcat/dct:language",
        "/dcat/dct:conformsTo",
    ] {
        assert!(
            out.pointer(path).is_some(),
            "recommended v3 field missing at {path}: {out:#}"
        );
    }
    // Distribution-level v3 additions.
    for path in [
        "/dcat/dcat:distribution/0/dct:license",
        "/dcat/dcat:distribution/0/dcat:accessURL",
        "/dcat/dcat:distribution/0/dct:modified",
        "/dcat/dcat:distribution/0/dct:rights",
        "/dcat/dcat:distribution/0/dcat-us:accessRestriction",
    ] {
        assert!(
            out.pointer(path).is_some(),
            "distribution v3 addition missing at {path}: {out:#}"
        );
    }
    // No dcat_warnings expected — every mandatory/recommended slot was seeded.
    assert!(
        out.get("dcat_warnings").is_none(),
        "expected no dcat_warnings when everything is populated, got: {:?}",
        out.get("dcat_warnings"),
    );
}

#[test]
fn profile_warns_when_contactpoint_missing() {
    let wrk = Workdir::new("profile_warn_contact");
    seed_geo_csv(&wrk);
    let mut cmd = wrk.command("profile");
    cmd.args(["in.csv", "-o", "out.json"]);
    wrk.assert_success(&mut cmd);
    let out = read_output(&wrk, "out.json");
    let warnings = out
        .get("dcat_warnings")
        .and_then(|v| v.as_array())
        .expect("dcat_warnings array");
    let cp = warnings
        .iter()
        .find(|w| w.get("field").and_then(|v| v.as_str()) == Some("dcat:contactPoint"))
        .expect("dcat:contactPoint warning");
    assert_eq!(
        cp.get("severity").and_then(|v| v.as_str()),
        Some("required")
    );
}

#[test]
fn profile_runs_validation_when_spec_declares_validators() {
    // §5.8: when the scheming spec declares one or more `validators`,
    // profile should invoke `qsv validate` against the input and merge
    // any RFC4180 failures into dcat_warnings. The presence of
    // validators is the trigger; their string content isn't
    // interpreted yet (auto-generating a JSON Schema from declared
    // types + validators is a future enhancement).
    //
    // This test uses a clean CSV so we assert two things:
    //   1. The helper *ran* — stderr shows the `ran `validate`` marker (mirroring the existing `ran
    //      `frequency`` / `ran `count`` status lines).
    //   2. The clean CSV produces NO `qsv:validation` entry under dcat_warnings (validation
    //      succeeded).
    let wrk = Workdir::new("profile_validation_trigger");
    seed_geo_csv(&wrk);
    std::fs::copy(
        "tests/resources/profile/dataset-druf.yaml",
        wrk.path("spec.yaml"),
    )
    .expect("copy spec fixture");

    let mut cmd = wrk.command("profile");
    cmd.args(["in.csv", "--spec", "spec.yaml", "-o", "out.json"]);
    let output = cmd.output().expect("spawn qsv profile");
    assert!(
        output.status.success(),
        "profile with validators failed: {}",
        String::from_utf8_lossy(&output.stderr),
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("ran `validate`"),
        "expected stderr to confirm validate was invoked, got: {stderr}",
    );

    let out = read_output(&wrk, "out.json");
    let warnings = out
        .get("dcat_warnings")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    let validation_warnings: Vec<_> = warnings
        .iter()
        .filter(|w| w.get("field").and_then(|f| f.as_str()) == Some("qsv:validation"))
        .collect();
    assert!(
        validation_warnings.is_empty(),
        "expected no qsv:validation warnings on clean CSV, got: {validation_warnings:?}",
    );
}

#[test]
fn profile_validation_honors_forwarded_delimiter_flag() {
    // §5.8 regression: `run_profile_validation` must forward
    // `--delimiter` to the spawned `qsv validate`. Without the
    // forwarding, validate would parse this semicolon-delimited input
    // as comma-separated and the row "store, retail" would split the
    // record into more fields than the 1-field header, yielding an
    // RFC4180 failure surfaced as a `qsv:validation` dcat_warning.
    //
    // With the forwarding wired correctly, validate parses on `;` and
    // sees six consistent fields per row, so NO `qsv:validation`
    // warning is emitted. The assertion below would fail if the flag
    // were ever dropped or misordered.
    let wrk = Workdir::new("profile_validation_delimiter_forwarding");
    wrk.create_from_string(
        "in.csv",
        "id;name;created_at;latitude;longitude;kind\n1;Alpha;2024-01-15;40.7128;-74.0060;store, \
         retail\n2;Bravo;2024-02-20;34.0522;-118.2437;office, \
         hq\n3;Charlie;2024-03-10;41.8781;-87.6298;office, branch\n",
    );
    std::fs::copy(
        "tests/resources/profile/dataset-druf.yaml",
        wrk.path("spec.yaml"),
    )
    .expect("copy spec fixture");

    let mut cmd = wrk.command("profile");
    cmd.args([
        "in.csv",
        "--spec",
        "spec.yaml",
        "--delimiter",
        ";",
        "-o",
        "out.json",
    ]);
    let output = cmd.output().expect("spawn qsv profile");
    assert!(
        output.status.success(),
        "profile with --delimiter ; failed: {}",
        String::from_utf8_lossy(&output.stderr),
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("ran `validate`"),
        "expected stderr to confirm validate was invoked, got: {stderr}",
    );

    let out = read_output(&wrk, "out.json");
    let warnings = out
        .get("dcat_warnings")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    let validation_warnings: Vec<_> = warnings
        .iter()
        .filter(|w| w.get("field").and_then(|f| f.as_str()) == Some("qsv:validation"))
        .collect();
    assert!(
        validation_warnings.is_empty(),
        "expected no qsv:validation warnings when --delimiter ; is forwarded to validate, got: \
         {validation_warnings:?}",
    );
}

#[test]
fn dataset_info_force_blocks_discovered_overlay_at_forced_path() {
    // §5.4: when a dataset_info entry is wrapped {"value": ..., "force": true},
    // the corresponding DCAT path is recorded as "forced" and discovered-DCAT
    // merging is forbidden from overlaying it — even when inferred is absent.
    //
    // This test stages a custom initial-context with two interesting wrappers:
    //   * /dcat/dct:license set to {"value": "MIT", "force": true} — the value lands via the
    //     pointer-override pass.
    //   * /dcat/dct:rights set to {"value": null, "force": true} — the null wrapper unwraps to a
    //     `null`, and the FORCE half means "don't let any future merge fill this in either".
    // Discovered DCAT isn't simulated here (no URL input) so we only
    // assert the static wiring: forced paths are collected, the
    // override applies, and the per-test workdir output reads back
    // the expected shape.
    let wrk = Workdir::new("profile_force_semantics");
    seed_geo_csv(&wrk);
    let ic = serde_json::json!({
        "package": {
            "title":             "Forced Title",
            "notes":             "An abstract",
            "contact_point":     {"fn": "Data Steward", "hasEmail": "ds@example.gov"},
            "publisher":         {"name": "Forced Agency"},
            "metadata_modified": "2024-12-01"
        },
        "resource": {"name": "data"},
        "dataset_info": {
            "/dcat/dct:license": {"value": "https://creativecommons.org/licenses/by/4.0/", "force": true},
            "/dcat/dct:rights":  {"value": null, "force": true}
        }
    });
    wrk.create_from_string("ic.json", &serde_json::to_string_pretty(&ic).unwrap());

    let mut cmd = wrk.command("profile");
    cmd.args(["in.csv", "--initial-context", "ic.json", "-o", "out.json"]);
    wrk.assert_success(&mut cmd);

    let out = read_output(&wrk, "out.json");
    // 1. Pointer-override-wrapped license lands as the inner string.
    assert_eq!(
        out.pointer("/dcat/dct:license").and_then(|v| v.as_str()),
        Some("https://creativecommons.org/licenses/by/4.0/"),
        "forced-with-value license must land as the inner value (no wrapper leak), got: {}",
        out.pointer("/dcat/dct:license")
            .map(ToString::to_string)
            .unwrap_or_default(),
    );
    // 2. The {value: null, force: true} wrapper unwraps to literal null; pointer-override writes
    //    that null at the path. Round-trip check.
    assert_eq!(
        out.pointer("/dcat/dct:rights"),
        Some(&serde_json::Value::Null),
        "forced-null rights must round-trip to literal null, got: {:?}",
        out.pointer("/dcat/dct:rights"),
    );
}

#[test]
fn profile_skips_validation_when_spec_has_no_validators() {
    // §5.8 negative: spec-less profile must NOT spawn `qsv validate`.
    // Inverse signal: the stderr "ran `validate`" marker is absent.
    let wrk = Workdir::new("profile_no_validation_trigger");
    seed_geo_csv(&wrk);

    let mut cmd = wrk.command("profile");
    cmd.args(["in.csv", "-o", "out.json"]);
    let output = cmd.output().expect("spawn qsv profile");
    assert!(
        output.status.success(),
        "spec-less profile failed: {}",
        String::from_utf8_lossy(&output.stderr),
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.contains("ran `validate`"),
        "expected validate to NOT run without spec validators, got stderr: {stderr}",
    );
}

#[test]
fn validate_dcat_passes_on_full_initial_context() {
    let wrk = Workdir::new("profile_validate_pass");
    seed_geo_csv(&wrk);
    let ctx_path = wrk.path("init.json");
    std::fs::write(
        &ctx_path,
        r#"{
            "package": {
                "title":         "Valid Dataset",
                "notes":         "Passes the minimal v3 schema.",
                "name":          "valid-dataset",
                "license_id":    "cc-by",
                "publisher":     "Demo Agency",
                "contact_point": {"fn": "Jane Doe", "hasEmail": "jane@example.gov"},
                "bureauCode":    ["015:11"],
                "programCode":   ["015:000"]
            }
        }"#,
    )
    .unwrap();
    let mut cmd = wrk.command("profile");
    cmd.args([
        "in.csv",
        "--initial-context",
        ctx_path.to_str().unwrap(),
        "--validate-dcat",
        "-o",
        "out.json",
    ]);
    wrk.assert_success(&mut cmd);
    let out = read_output(&wrk, "out.json");
    // No schema-violation warnings expected — all mandatory keys populated.
    let warnings = out
        .get("dcat_warnings")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    let schema_warnings: Vec<_> = warnings
        .iter()
        .filter(|w| {
            let msg = w.get("message").and_then(|v| v.as_str()).unwrap_or("");
            // schema violations come from jsonschema; in-projection
            // warnings have human-readable messages like "DCAT-US v3
            // mandatory field missing".
            !msg.starts_with("DCAT-US v3")
        })
        .collect();
    assert!(
        schema_warnings.is_empty(),
        "expected no schema warnings, got: {schema_warnings:#?}",
    );
}

#[test]
fn validate_dcat_flags_missing_contactpoint() {
    let wrk = Workdir::new("profile_validate_missing_cp");
    seed_geo_csv(&wrk);
    let mut cmd = wrk.command("profile");
    cmd.args(["in.csv", "--validate-dcat", "-o", "out.json"]);
    wrk.assert_success(&mut cmd);
    let out = read_output(&wrk, "out.json");
    let warnings = out
        .get("dcat_warnings")
        .and_then(|v| v.as_array())
        .expect("dcat_warnings array");
    // The contact-point warning from the in-projection check fires
    // either way; schema validation also fires its own. At minimum one
    // entry should mention contactPoint.
    assert!(
        warnings.iter().any(|w| w
            .get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .contains("contactPoint")),
        "expected a contactPoint warning, got: {warnings:#?}",
    );
}

#[test]
fn strict_dcat_fails_command_on_violation() {
    let wrk = Workdir::new("profile_strict");
    seed_geo_csv(&wrk);
    let mut cmd = wrk.command("profile");
    cmd.args([
        "in.csv",
        "--validate-dcat",
        "--strict-dcat",
        "-o",
        "out.json",
    ]);
    let output = cmd.output().expect("spawn qsv profile");
    assert!(
        !output.status.success(),
        "expected non-zero exit under --strict-dcat with missing fields, got: {:?}",
        output.status,
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("strict-dcat") && stderr.contains("violation"),
        "expected strict-dcat violation message in stderr, got: {stderr}"
    );
    // out.json must not exist when the command failed.
    assert!(
        !wrk.path("out.json").exists(),
        "out.json should not be written when --strict-dcat fails"
    );
}

#[test]
fn dataset_info_override_supplies_field_before_strict_validation() {
    // Roborev finding 2439#4: validation must run AFTER dataset_info
    // overrides. A user who supplies a missing mandatory field via a
    // JSON-Pointer override should not be blocked by --strict-dcat.
    let wrk = Workdir::new("profile_strict_with_override");
    seed_geo_csv(&wrk);
    let ctx_path = wrk.path("init.json");
    // Note: NO contact_point in package — would normally fail
    // --strict-dcat. The dataset_info override supplies it directly.
    std::fs::write(
        &ctx_path,
        r#"{
            "package": {"title": "X", "notes": "Y", "name": "x", "publisher": "P"},
            "dataset_info": {
                "/dcat/dcat:contactPoint": {
                    "@type":          "vcard:Individual",
                    "vcard:fn":       "Override",
                    "vcard:hasEmail": "mailto:override@example.gov"
                }
            }
        }"#,
    )
    .unwrap();
    let mut cmd = wrk.command("profile");
    cmd.args([
        "in.csv",
        "--initial-context",
        ctx_path.to_str().unwrap(),
        "--validate-dcat",
        "--strict-dcat",
        "-o",
        "out.json",
    ]);
    wrk.assert_success(&mut cmd);
    let out = read_output(&wrk, "out.json");
    assert_eq!(
        out.pointer("/dcat/dcat:contactPoint/vcard:fn")
            .and_then(|v| v.as_str()),
        Some("Override"),
        "dataset_info override must apply before validation"
    );
}

#[test]
fn dataset_info_override_clears_stale_warnings() {
    // Roborev 2440#1: when dataset_info supplies a missing field,
    // the stale build-time "missing X" warning must NOT survive into
    // the final dcat_warnings array.
    let wrk = Workdir::new("profile_clear_stale");
    seed_geo_csv(&wrk);
    let ctx_path = wrk.path("init.json");
    std::fs::write(
        &ctx_path,
        r#"{
            "dataset_info": {
                "/dcat/dcat:contactPoint": {
                    "@type":          "vcard:Individual",
                    "vcard:fn":       "Override",
                    "vcard:hasEmail": "mailto:o@x.gov"
                }
            }
        }"#,
    )
    .unwrap();
    let mut cmd = wrk.command("profile");
    cmd.args([
        "in.csv",
        "--initial-context",
        ctx_path.to_str().unwrap(),
        "-o",
        "out.json",
    ]);
    wrk.assert_success(&mut cmd);
    let out = read_output(&wrk, "out.json");
    let warnings: Vec<Value> = out
        .get("dcat_warnings")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    assert!(
        !warnings
            .iter()
            .any(|w| w.get("field").and_then(|v| v.as_str()) == Some("dcat:contactPoint")),
        "stale contactPoint warning must be filtered out, got: {warnings:#?}",
    );
}

#[test]
fn wrapped_dataset_info_override_rescues_strict_validation() {
    // Roborev 2440#2: dataset_info entries written in the
    // {"value": ..., "force": true} wrapper form must unwrap to their
    // inner value before being applied. With the unwrap in place,
    // a wrapped override of a mandatory field rescues --strict-dcat.
    let wrk = Workdir::new("profile_wrapped_strict");
    seed_geo_csv(&wrk);
    let ctx_path = wrk.path("init.json");
    std::fs::write(
        &ctx_path,
        r#"{
            "package": {"title": "X", "notes": "Y", "name": "x", "publisher": "P"},
            "dataset_info": {
                "/dcat/dcat:contactPoint": {
                    "value": {
                        "@type":          "vcard:Individual",
                        "vcard:fn":       "Wrapped",
                        "vcard:hasEmail": "mailto:wrapped@example.gov"
                    },
                    "force": true
                }
            }
        }"#,
    )
    .unwrap();
    let mut cmd = wrk.command("profile");
    cmd.args([
        "in.csv",
        "--initial-context",
        ctx_path.to_str().unwrap(),
        "--validate-dcat",
        "--strict-dcat",
        "-o",
        "out.json",
    ]);
    wrk.assert_success(&mut cmd);
    let out = read_output(&wrk, "out.json");
    // Wrapper unwrapped → inner vcard:Individual landed at the
    // contactPoint slot, validation passed (otherwise --strict-dcat
    // would have aborted).
    assert_eq!(
        out.pointer("/dcat/dcat:contactPoint/vcard:fn")
            .and_then(|v| v.as_str()),
        Some("Wrapped"),
        "wrapper must unwrap; the {{value, force}} object itself must NOT become the \
         dcat:contactPoint value"
    );
}

// =============================================================================
// DCAT-US v3 comprehensive coverage (--catalog, force overrides, new fields,
// GSA bundle validation).
// =============================================================================

#[test]
fn profile_catalog_flag_wraps_dataset() {
    let wrk = Workdir::new("profile_catalog");
    seed_geo_csv(&wrk);

    let mut cmd = wrk.command("profile");
    cmd.arg("in.csv").arg("--catalog");
    wrk.assert_success(&mut cmd);

    let out = read_output(&wrk, "in.csv.metadata.json");
    assert_eq!(
        out.pointer("/dcat/@type").and_then(|v| v.as_str()),
        Some("dcat:Catalog"),
        "expected Catalog envelope when --catalog is set: {out:#}"
    );
    assert!(
        out.pointer("/dcat/dcat:dataset")
            .and_then(|v| v.as_array())
            .is_some_and(|a| a.len() == 1),
        "Catalog must carry exactly one Dataset"
    );
    assert_eq!(
        out.pointer("/dcat/dcat:dataset/0/@type")
            .and_then(|v| v.as_str()),
        Some("dcat:Dataset"),
        "inner element of dcat:dataset must keep its Dataset shape"
    );
    // dct:modified must NOT be auto-emitted on the Catalog envelope
    // (single-CSV inputs have no independent catalog-level mtime).
    assert!(
        out.pointer("/dcat/dct:modified").is_none(),
        "Catalog envelope must omit dct:modified for single-CSV runs"
    );
}

#[test]
fn profile_omits_catalog_wrapper_by_default() {
    // Regression: without --catalog the dcat block stays a Dataset.
    let wrk = Workdir::new("profile_no_catalog");
    seed_geo_csv(&wrk);
    let mut cmd = wrk.command("profile");
    cmd.arg("in.csv");
    wrk.assert_success(&mut cmd);
    let out = read_output(&wrk, "in.csv.metadata.json");
    assert_eq!(
        out.pointer("/dcat/@type").and_then(|v| v.as_str()),
        Some("dcat:Dataset"),
        "default mode must keep Dataset shape: {out:#}"
    );
}

#[test]
fn profile_emits_checksum_for_local_file() {
    use std::io::Read;

    let wrk = Workdir::new("profile_checksum");
    seed_geo_csv(&wrk);

    let mut cmd = wrk.command("profile");
    cmd.arg("in.csv");
    wrk.assert_success(&mut cmd);

    let out = read_output(&wrk, "in.csv.metadata.json");
    let alg = out
        .pointer("/dcat/dcat:distribution/0/dcat:checksum/spdx:algorithm")
        .and_then(|v| v.as_str());
    assert_eq!(alg, Some("SHA-256"));
    let emitted = out
        .pointer("/dcat/dcat:distribution/0/dcat:checksum/spdx:checksumValue")
        .and_then(|v| v.as_str())
        .expect("checksumValue present");
    assert_eq!(emitted.len(), 64, "SHA-256 hex is 64 chars: got {emitted}");
    assert!(
        emitted
            .chars()
            .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()),
        "checksumValue must be lowercase hex per GSA Checksum schema"
    );
    // Independently compute SHA-256 of the on-disk file and compare.
    use sha2::{Digest, Sha256};
    let mut f = std::fs::File::open(wrk.path("in.csv")).unwrap();
    let mut bytes = Vec::new();
    f.read_to_end(&mut bytes).unwrap();
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    let want = hasher.finalize();
    let want_hex: String = want.iter().map(|b| format!("{b:02x}")).collect();
    assert_eq!(emitted, want_hex, "checksum mismatch");
}

#[test]
fn profile_emits_compress_format_for_csv_gz_input() {
    // Drop a plain CSV, gzip it, then profile the .csv.gz file.
    let wrk = Workdir::new("profile_compress_gz");
    seed_geo_csv(&wrk);
    // Spawn `gzip` to keep the test free of compression deps.
    let csv_path = wrk.path("in.csv");
    let status = std::process::Command::new("gzip")
        .arg("-9")
        .arg(&csv_path)
        .status()
        .expect("spawn gzip");
    assert!(status.success(), "gzip exited non-zero");
    let gz_path = wrk.path("in.csv.gz");
    assert!(gz_path.exists(), "expected in.csv.gz at {gz_path:?}");

    let mut cmd = wrk.command("profile");
    cmd.arg(gz_path.to_str().unwrap());
    wrk.assert_success(&mut cmd);

    let out = read_output(&wrk, "in.csv.gz.metadata.json");
    assert_eq!(
        out.pointer("/dcat/dcat:distribution/0/dcat:compressFormat")
            .and_then(|v| v.as_str()),
        Some("application/gzip"),
        "expected dcat:compressFormat=application/gzip for .csv.gz input: {out:#}"
    );
    assert!(
        out.pointer("/dcat/dcat:distribution/0/dcat:packageFormat")
            .is_none(),
        "single-file compression must NOT also emit packageFormat"
    );
}

#[test]
fn profile_emits_dataset_level_created_version_versionnotes() {
    let wrk = Workdir::new("profile_dataset_extras");
    seed_geo_csv(&wrk);
    let ctx_path = wrk.path("init.json");
    std::fs::write(
        &ctx_path,
        r#"{
          "package": {
            "title":        "Foo",
            "notes":        "Bar",
            "publisher":    "Agency",
            "created":      "2023-06-15",
            "version":      "1.2.0",
            "versionNotes": "Q3 refresh"
          },
          "resource": {}
        }"#,
    )
    .unwrap();
    let mut cmd = wrk.command("profile");
    cmd.arg("in.csv")
        .arg("--initial-context")
        .arg(ctx_path.to_str().unwrap());
    wrk.assert_success(&mut cmd);
    let out = read_output(&wrk, "in.csv.metadata.json");
    assert_eq!(
        out.pointer("/dcat/dct:created").and_then(|v| v.as_str()),
        Some("2023-06-15")
    );
    assert_eq!(
        out.pointer("/dcat/dcat:version").and_then(|v| v.as_str()),
        Some("1.2.0")
    );
    assert_eq!(
        out.pointer("/dcat/dcat:versionNotes")
            .and_then(|v| v.as_str()),
        Some("Q3 refresh")
    );
}

#[test]
fn profile_emits_distribution_language_and_conformsto() {
    let wrk = Workdir::new("profile_distribution_extras");
    seed_geo_csv(&wrk);
    let ctx_path = wrk.path("init.json");
    std::fs::write(
        &ctx_path,
        r#"{
          "package":  {"title":"Foo","notes":"Bar","publisher":"Agency"},
          "resource": {
            "language":   "en",
            "conformsTo": "https://www.w3.org/TR/tabular-data-model/"
          }
        }"#,
    )
    .unwrap();
    let mut cmd = wrk.command("profile");
    cmd.arg("in.csv")
        .arg("--initial-context")
        .arg(ctx_path.to_str().unwrap());
    wrk.assert_success(&mut cmd);
    let out = read_output(&wrk, "in.csv.metadata.json");
    assert_eq!(
        out.pointer("/dcat/dcat:distribution/0/dct:language")
            .and_then(|v| v.as_str()),
        Some("en")
    );
    // conformsTo is an array of dct:Standard objects per v3 cardinality.
    assert_eq!(
        out.pointer("/dcat/dcat:distribution/0/dct:conformsTo/0/@type")
            .and_then(|v| v.as_str()),
        Some("dct:Standard")
    );
    assert_eq!(
        out.pointer("/dcat/dcat:distribution/0/dct:conformsTo/0/@id")
            .and_then(|v| v.as_str()),
        Some("https://www.w3.org/TR/tabular-data-model/")
    );
}

#[test]
fn profile_force_on_package_title_flows_via_ckan_to_dcat() {
    let wrk = Workdir::new("profile_force_package_title");
    seed_geo_csv(&wrk);
    let ctx_path = wrk.path("init.json");
    // Note: `title` carries a wrapper; the inner value lands at
    // /dcat/dct:title (translated by ckan_to_dcat) and must beat any
    // inferred value.
    std::fs::write(
        &ctx_path,
        r#"{
          "package": {
            "title":     {"value": "FORCED VIA PACKAGE", "force": true},
            "notes":     "Bar",
            "publisher": "Agency"
          },
          "resource": {}
        }"#,
    )
    .unwrap();
    let mut cmd = wrk.command("profile");
    cmd.arg("in.csv")
        .arg("--initial-context")
        .arg(ctx_path.to_str().unwrap());
    wrk.assert_success(&mut cmd);
    let out = read_output(&wrk, "in.csv.metadata.json");
    assert_eq!(
        out.pointer("/dcat/dct:title").and_then(|v| v.as_str()),
        Some("FORCED VIA PACKAGE"),
        "package.title force=true must land at /dcat/dct:title: {out:#}"
    );
}

#[test]
fn profile_force_on_resource_url_translates_to_download_url() {
    let wrk = Workdir::new("profile_force_resource_url");
    seed_geo_csv(&wrk);
    let ctx_path = wrk.path("init.json");
    std::fs::write(
        &ctx_path,
        r#"{
          "package":  {"title":"Foo","notes":"Bar","publisher":"Agency"},
          "resource": {
            "url": {"value":"https://forced.example.gov/data.csv","force":true}
          }
        }"#,
    )
    .unwrap();
    let mut cmd = wrk.command("profile");
    cmd.arg("in.csv")
        .arg("--initial-context")
        .arg(ctx_path.to_str().unwrap());
    wrk.assert_success(&mut cmd);
    let out = read_output(&wrk, "in.csv.metadata.json");
    assert_eq!(
        out.pointer("/dcat/dcat:distribution/0/dcat:downloadURL")
            .and_then(|v| v.as_str()),
        Some("https://forced.example.gov/data.csv"),
        "resource.url force=true must land at /dcat/dcat:distribution/0/dcat:downloadURL: {out:#}"
    );
}

#[test]
fn profile_force_on_dataset_info_beats_plain_dataset_info() {
    // Two dataset_info entries target the same path; the forced one
    // is applied LAST and wins.
    let wrk = Workdir::new("profile_force_dataset_info");
    seed_geo_csv(&wrk);
    let ctx_path = wrk.path("init.json");
    std::fs::write(
        &ctx_path,
        r#"{
          "package":      {"title":"Plain","notes":"Bar","publisher":"Agency"},
          "resource":     {},
          "dataset_info": {
            "/dcat/dct:title": {"value": "Forced via dataset_info", "force": true}
          }
        }"#,
    )
    .unwrap();
    let mut cmd = wrk.command("profile");
    cmd.arg("in.csv")
        .arg("--initial-context")
        .arg(ctx_path.to_str().unwrap());
    wrk.assert_success(&mut cmd);
    let out = read_output(&wrk, "in.csv.metadata.json");
    assert_eq!(
        out.pointer("/dcat/dct:title").and_then(|v| v.as_str()),
        Some("Forced via dataset_info"),
        "dataset_info force=true must beat inferred: {out:#}"
    );
}

#[test]
fn profile_validate_catalog_runs_catalog_overlay() {
    // With --catalog --validate-dcat, the validator picks the Catalog
    // overlay schema by @type. A minimal-but-valid Catalog (one
    // fully-populated Dataset inside) must validate clean against the
    // GSA Catalog schema — no Required-severity warnings.
    let wrk = Workdir::new("profile_validate_catalog");
    seed_geo_csv(&wrk);
    let ctx_path = wrk.path("init.json");
    std::fs::write(
        &ctx_path,
        r#"{
          "package": {
            "title":       "Catalog Test",
            "notes":       "Bar",
            "name":        "catalog-test-001",
            "publisher":   "Agency",
            "bureauCode":  ["015:11"],
            "programCode": ["015:001"],
            "contact_point": {"fn":"X","hasEmail":"x@y.gov"}
          },
          "resource": {
            "url": "https://x.gov/d.csv"
          }
        }"#,
    )
    .unwrap();
    let mut cmd = wrk.command("profile");
    cmd.arg("in.csv")
        .arg("--initial-context")
        .arg(ctx_path.to_str().unwrap())
        .arg("--catalog")
        .arg("--validate-dcat");
    wrk.assert_success(&mut cmd);
    let out = read_output(&wrk, "in.csv.metadata.json");
    assert_eq!(
        out.pointer("/dcat/@type").and_then(|v| v.as_str()),
        Some("dcat:Catalog"),
        "--catalog must produce a Catalog envelope"
    );
    // No Required-severity warnings — Catalog overlay required keys are
    // satisfied. Recommended-severity warnings would surface as
    // best-practice nudges; this test guards the strict bar.
    let required_warnings: Vec<&Value> = out
        .get("dcat_warnings")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter(|w| w.get("severity").and_then(|s| s.as_str()) == Some("required"))
                .collect()
        })
        .unwrap_or_default();
    assert!(
        required_warnings.is_empty(),
        "expected no Required-severity schema warnings, got: {required_warnings:?}",
    );
}

// =============================================================================
// Vendored DCAT-US v3 schema bundle pin guard.
// =============================================================================

/// Recompute the SHA-256 of every file listed in
/// `resources/dcat-us-v3/MANIFEST.json` and assert each matches the
/// hash recorded in the manifest. Guards against silent edits to the
/// vendored upstream schemas — the refresh procedure in
/// `resources/dcat-us-v3/README.md` is the only blessed way to update
/// them.
#[test]
fn dcat_us_v3_bundle_pin_manifest_matches_files() {
    use std::io::Read;

    use sha2::{Digest, Sha256};

    let manifest_path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/dcat-us-v3/MANIFEST.json");
    let bundle_root = manifest_path.parent().unwrap();
    let raw = std::fs::read_to_string(&manifest_path)
        .unwrap_or_else(|e| panic!("could not read {}: {e}", manifest_path.display()));
    let manifest: serde_json::Value = serde_json::from_str(&raw).expect("manifest is valid JSON");
    let files = manifest
        .get("files")
        .and_then(|v| v.as_array())
        .expect("MANIFEST.json must carry a `files` array");
    assert!(!files.is_empty(), "MANIFEST.json `files` array is empty");

    for entry in files {
        let rel_path = entry
            .get("path")
            .and_then(|v| v.as_str())
            .expect("each file entry must carry a `path`");
        let want_sha = entry
            .get("sha256")
            .and_then(|v| v.as_str())
            .expect("each file entry must carry a `sha256`");

        let abs_path = bundle_root.join(rel_path);
        let mut f = std::fs::File::open(&abs_path)
            .unwrap_or_else(|e| panic!("could not open vendored file {}: {e}", abs_path.display()));
        let mut hasher = Sha256::new();
        let mut buf = [0u8; 64 * 1024];
        loop {
            let n = f.read(&mut buf).unwrap_or_else(|e| {
                panic!("could not read vendored file {}: {e}", abs_path.display())
            });
            if n == 0 {
                break;
            }
            hasher.update(&buf[..n]);
        }
        let got: String = hasher
            .finalize()
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();
        assert_eq!(
            got, want_sha,
            "SHA-256 mismatch for vendored DCAT-US v3 schema `{rel_path}`: re-vendor via \
             resources/dcat-us-v3/README.md procedure or restore the file"
        );
    }
}

// =========================================================================
// External validator trust gate (Roborev #2509)
// =========================================================================
// The `validation.external` block can spawn arbitrary commands. Bundled
// profiles are vetted at qsv release time and run frictionlessly; file-
// loaded profiles must opt in via `--allow-external-validator`. These
// tests lock in the gate so a future refactor can't silently regress
// the security boundary.

/// Write a minimal profile YAML to the workdir that declares a
/// stderr-emitting `validation.external` command (so we can both
/// confirm the spawn happened and see the parsed finding). Returns
/// the path the test should pass via `--profile`.
fn write_external_validator_profile(wrk: &Workdir, label: &str) -> std::path::PathBuf {
    // sh -c '...; exit 1' is portable on macOS and Linux CI runners.
    // The findings include a recognizable marker string so the test
    // can verify the validator actually ran.
    let yaml = format!(
        r#"name: ext-gate-test
dataset:
  type: dcat:Dataset
  fields:
    - path: dct:title
      template: "{{{{ pkg.title | default('Untitled') }}}}"
validation:
  enabled: false
  external:
    command: "sh"
    args: ["-c", "echo SPAWNED-{label} 1>&2; exit 1"]
    label: "{label}"
    default_severity: "recommended"
"#
    );
    let path = wrk.path("ext-gate.yaml");
    std::fs::write(&path, yaml).expect("write ext-gate.yaml");
    path
}

#[cfg(unix)]
#[test]
fn external_validator_gated_for_file_loaded_profile_without_flag() {
    let wrk = Workdir::new("ext_gate_file_no_flag");
    let src = std::env::current_dir()
        .unwrap()
        .join("tests/resources/profile/golden/nyc-311-subset.csv");
    std::fs::copy(&src, wrk.path("in.csv")).expect("copy fixture");
    let yaml_path = write_external_validator_profile(&wrk, "fake-validator");

    let mut cmd = wrk.command("profile");
    cmd.args([
        "in.csv",
        "--profile",
        yaml_path.to_str().unwrap(),
        "--validate-dcat",
        "-o",
        "out.json",
    ]);
    wrk.assert_success(&mut cmd);

    let out = read_output(&wrk, "out.json");
    let warnings = out
        .get("dcat_warnings")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

    // The gate warning must be present.
    let gate_warning = warnings.iter().find(|w| {
        w.get("field").and_then(|v| v.as_str()) == Some("external_validate")
            && w.get("message").and_then(|v| v.as_str()).is_some_and(|m| {
                m.contains("was NOT run") && m.contains("--allow-external-validator")
            })
    });
    assert!(
        gate_warning.is_some(),
        "file-loaded profile must emit the gate warning explaining the opt-in; got warnings: \
         {warnings:#?}"
    );

    // The validator must NOT have spawned — no SPAWNED- marker anywhere.
    let any_spawn_evidence = warnings.iter().any(|w| {
        w.get("message")
            .and_then(|v| v.as_str())
            .is_some_and(|m| m.contains("SPAWNED-"))
    });
    assert!(
        !any_spawn_evidence,
        "validator command must NOT have been spawned without --allow-external-validator; got \
         warnings: {warnings:#?}"
    );
}

#[cfg(unix)]
#[test]
fn external_validator_runs_for_file_loaded_profile_with_flag() {
    let wrk = Workdir::new("ext_gate_file_with_flag");
    let src = std::env::current_dir()
        .unwrap()
        .join("tests/resources/profile/golden/nyc-311-subset.csv");
    std::fs::copy(&src, wrk.path("in.csv")).expect("copy fixture");
    let yaml_path = write_external_validator_profile(&wrk, "fake-validator");

    let mut cmd = wrk.command("profile");
    cmd.args([
        "in.csv",
        "--profile",
        yaml_path.to_str().unwrap(),
        "--validate-dcat",
        "--allow-external-validator",
        "-o",
        "out.json",
    ]);
    wrk.assert_success(&mut cmd);

    let out = read_output(&wrk, "out.json");
    let warnings = out
        .get("dcat_warnings")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

    // The gate warning must NOT be present.
    let any_gate_warning = warnings.iter().any(|w| {
        w.get("message")
            .and_then(|v| v.as_str())
            .is_some_and(|m| m.contains("was NOT run") && m.contains("--allow-external-validator"))
    });
    assert!(
        !any_gate_warning,
        "opt-in flag must skip the gate warning; got warnings: {warnings:#?}"
    );

    // Validator output must surface with the `<label>: <line>` format
    // and the stable `external_validate` field.
    let finding = warnings.iter().find(|w| {
        w.get("field").and_then(|v| v.as_str()) == Some("external_validate")
            && w.get("message")
                .and_then(|v| v.as_str())
                .is_some_and(|m| m == "fake-validator: SPAWNED-fake-validator")
    });
    assert!(
        finding.is_some(),
        "validator must have spawned and produced a finding; got warnings: {warnings:#?}"
    );
}

#[cfg(unix)]
#[test]
fn external_validator_strict_dcat_fails_on_file_loaded_findings() {
    // With --strict-dcat AND --allow-external-validator, non-Info
    // findings (Recommended/Required) from an external validator
    // must fail the command the same way schema violations do.
    let wrk = Workdir::new("ext_gate_strict");
    let src = std::env::current_dir()
        .unwrap()
        .join("tests/resources/profile/golden/nyc-311-subset.csv");
    std::fs::copy(&src, wrk.path("in.csv")).expect("copy fixture");
    let yaml_path = write_external_validator_profile(&wrk, "fake-validator");

    let mut cmd = wrk.command("profile");
    cmd.args([
        "in.csv",
        "--profile",
        yaml_path.to_str().unwrap(),
        "--validate-dcat",
        "--allow-external-validator",
        "--strict-dcat",
        "-o",
        "out.json",
    ]);
    let got_err = wrk.output_stderr(&mut cmd);
    assert!(
        got_err.contains("external validator finding"),
        "strict mode must surface the external validator failure: got `{got_err}`"
    );
}

#[test]
fn external_validator_embedded_profile_skips_gate_warning() {
    // Embedded profiles (Croissant ships `validation.external` for
    // mlcroissant) MUST NOT emit the file-loaded gate warning. The
    // mlcroissant binary itself may or may not be installed on CI;
    // either outcome is acceptable here — we're locking in that the
    // gate doesn't accidentally apply to vetted bundled profiles.
    let wrk = Workdir::new("ext_gate_embedded");
    let src = std::env::current_dir()
        .unwrap()
        .join("tests/resources/profile/golden/nyc-311-subset.csv");
    std::fs::copy(&src, wrk.path("in.csv")).expect("copy fixture");

    let mut cmd = wrk.command("profile");
    cmd.args([
        "in.csv",
        "--profile",
        "croissant",
        "--validate-dcat",
        "-o",
        "out.json",
    ]);
    wrk.assert_success(&mut cmd);

    let out = read_output(&wrk, "out.json");
    let warnings = out
        .get("dcat_warnings")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    let gate_warning_present = warnings.iter().any(|w| {
        w.get("message")
            .and_then(|v| v.as_str())
            .is_some_and(|m| m.contains("was NOT run") && m.contains("--allow-external-validator"))
    });
    assert!(
        !gate_warning_present,
        "embedded profile must not trip the file-loaded gate; got warnings: {warnings:#?}"
    );
}
