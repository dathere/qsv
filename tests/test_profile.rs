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

    // DCAT block is emitted by default. dct:spatial falls back to the
    // bbox-derived POLYGON because no formula ran.
    let spatial = out.pointer("/dcat/dct:spatial").expect("dct:spatial");
    let bbox = spatial
        .get("dcat:bbox")
        .and_then(|v| v.as_str())
        .expect("dcat:bbox str");
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
    // that the suggestion populated it.
    let wkt = out
        .pointer("/dcat/dct:spatial/locn:geometry/@value")
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
fn profile_stdin_input_is_rejected() {
    // For v1 we require a real input file path -- piping stdin should fail
    // with a clear message rather than producing a silent no-op.
    let wrk = Workdir::new("profile_stdin");
    let mut cmd = wrk.command("profile");
    cmd.args(["-", "-o", "out.json"]);
    let got = wrk.output_stderr(&mut cmd);
    assert!(
        got.contains("does not exist") || got.contains("not yet supported"),
        "expected stdin-rejection error, got: {got}"
    );
}
