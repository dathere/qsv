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
    let wrk = Workdir::new("profile_stdin");
    let mut cmd = wrk.command("profile");
    cmd.arg("-");
    cmd.stdin(std::process::Stdio::piped());
    let mut child = cmd.spawn().expect("spawn qsv profile");
    {
        use std::io::Write;
        let stdin = child.stdin.as_mut().expect("child stdin");
        stdin
            .write_all(b"id,name\n1,alpha\n2,bravo\n3,charlie\n")
            .expect("write stdin");
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
        .pointer("/dcat/dcat:distribution/0/dct:license/@id")
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
