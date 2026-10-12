use crate::workdir::Workdir;

#[test]
fn geoconvert_geojson_to_csv_basic() {
    let wrk = Workdir::new("geojson_to_csv_basic");
    wrk.create_from_string(
        "data.geojson",
        r#"{
  "type": "Feature",
  "geometry": {
    "type": "Point",
    "coordinates": [125.6, 10.1]
  },
  "properties": {
    "name": "Dinagat Islands"
  }
}"#,
    );
    let mut cmd = wrk.command("geoconvert");
    cmd.arg("data.geojson").arg("geojson").arg("csv");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![
        svec!["geometry", "name"],
        svec!["POINT(125.6 10.1)", "Dinagat Islands"],
    ];
    assert_eq!(got, expected);
}

#[test]
#[ignore = "requires large TX_Cities.geojson fixture removed to reduce repo size"]
fn geoconvert_geojson_to_csv() {
    let wrk = Workdir::new("geoconvert_geojson_to_csv");
    let txcities_geojson = wrk.load_test_file("TX_Cities.geojson");
    let txcities_csv = wrk.path("TX_cities.csv").to_string_lossy().to_string();

    let mut cmd = wrk.command("geoconvert");
    cmd.arg(txcities_geojson)
        .arg("geojson")
        .arg("csv")
        .args(["--output", &txcities_csv]);

    wrk.assert_success(&mut cmd);

    let txcities_csv_first5 = wrk
        .path("TX_cities-first5.csv")
        .to_string_lossy()
        .to_string();

    let mut cmd = wrk.command("slice");
    cmd.arg(txcities_csv)
        .args(["--len", "5"])
        .args(["--output", &txcities_csv_first5]);

    wrk.assert_success(&mut cmd);

    let mut cmd = wrk.command("select");
    cmd.args(["name,Shape_Length,Shape_Area"])
        .arg(&txcities_csv_first5);

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"name,Shape_Length,Shape_Area
Abbott,0,0
Abernathy,0,0
Abilene,0,0
Ackerly,0,0
Addison,0,0"#;
    assert_eq!(got, expected);
}

#[test]
fn geoconvert_csv_to_geojson_latlon_order() {
    // Regression: GeoJSON RFC 7946 §3.1.1 requires coordinates as [longitude, latitude].
    // A prior version of the --latitude/--longitude path emitted [lat, lon].
    let wrk = Workdir::new("geoconvert_csv_to_geojson_latlon_order");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "lat", "lon"],
            svec!["Dinagat Islands", "10.1", "125.6"],
        ],
    );

    let mut cmd = wrk.command("geoconvert");
    cmd.arg("data.csv")
        .arg("csv")
        .arg("geojson")
        .args(["--latitude", "lat"])
        .args(["--longitude", "lon"]);

    let got: String = wrk.stdout_on_success(&mut cmd);
    let geojson: serde_json::Value = serde_json::from_str(&got).unwrap();
    let feature = &geojson["features"][0];
    // Coordinates must be [longitude, latitude]: longitude (125.6) first.
    assert_eq!(
        feature["geometry"],
        serde_json::json!({"type": "Point", "coordinates": [125.6, 10.1]}),
        "expected [lon, lat] order, got: {got}"
    );
    // the latitude/longitude columns become the geometry, not properties
    assert_eq!(
        feature["properties"],
        serde_json::json!({"name": "Dinagat Islands"})
    );
}

#[test]
fn geoconvert_geojson_to_csv_max_length_inline() {
    // Regression: --max-length must not panic on multi-byte UTF-8 in property values
    // and must truncate values >max_len to "<first max_len bytes>...".
    let wrk = Workdir::new("geoconvert_geojson_to_csv_max_length_inline");
    // "Café — Île" has multi-byte UTF-8 chars near byte 5 ('é' is 2 bytes).
    wrk.create_from_string(
        "data.geojson",
        r#"{
  "type": "Feature",
  "geometry": { "type": "Point", "coordinates": [125.6, 10.1] },
  "properties": { "name": "Café — Île de Dinagat" }
}"#,
    );

    let mut cmd = wrk.command("geoconvert");
    cmd.arg("data.geojson")
        .arg("geojson")
        .arg("csv")
        .args(["--max-length", "5"]);

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    // Header row plus the truncated data row.
    assert_eq!(got[0], svec!["geometry", "name"]);
    // geometry "POINT(125.6 10.1)" is longer than 5, so it must be truncated to <=5 bytes + "...".
    assert!(got[1][0].ends_with("..."));
    assert!(got[1][0].len() <= 5 + 3);
    // name: first 5 bytes of "Café — Île de Dinagat" land mid-char on é if naive byte slicing
    // is used; verify it didn't panic and ends with the ellipsis.
    assert!(got[1][1].ends_with("..."));
}

#[test]
fn geoconvert_csv_partial_latlon_flag_errors() {
    // Regression: only one of --latitude/--longitude must produce a clear error,
    // not the generic "specify --geometry or --latitude/--longitude" message.
    let wrk = Workdir::new("geoconvert_csv_partial_latlon_flag_errors");
    wrk.create(
        "data.csv",
        vec![svec!["name", "lat", "lon"], svec!["x", "10.1", "125.6"]],
    );

    let mut cmd = wrk.command("geoconvert");
    cmd.arg("data.csv")
        .arg("csv")
        .arg("geojson")
        .args(["--latitude", "lat"]);

    let got = wrk.stderr_on_error(&mut cmd);
    assert!(
        got.contains("--latitude and --longitude must be used together"),
        "expected partial-flag error, got: {got}"
    );
}

#[test]
#[ignore = "requires large TX_Cities.geojson fixture removed to reduce repo size"]
fn geoconvert_geojson_to_csv_max_length() {
    let wrk = Workdir::new("geoconvert_geojson_to_csv_max_length");
    let txcities_geojson = wrk.load_test_file("TX_Cities.geojson");
    let txcities_csv = wrk
        .path("TX_cities_max_length.csv")
        .to_string_lossy()
        .to_string();

    // Convert GeoJSON to CSV with max-length option set to 10
    let mut cmd = wrk.command("geoconvert");
    cmd.arg(txcities_geojson)
        .arg("geojson")
        .arg("csv")
        .args(["--max-length", "10"])
        .args(["--output", &txcities_csv]);

    wrk.assert_success(&mut cmd);

    let mut cmd = wrk.command("slice");
    cmd.arg(txcities_csv).args(["--len", "5"]);

    let got: String = wrk.stdout_on_success(&mut cmd);

    let expected = r#"geometry,OBJECTID,name,Shape_Length,Shape_Area
POLYGON((-...,1,Abbott,0,0
MULTIPOLYG...,2,Abernathy,0,0
POLYGON((-...,3,Abilene,0,0
POLYGON((-...,4,Ackerly,0,0
POLYGON((-...,5,Addison,0,0"#;
    assert_eq!(got, expected);
}

/// Parse geoconvert's GeoJSON output and return each feature's `name` property.
fn feature_names(geojson: &str) -> Vec<String> {
    let value: serde_json::Value = serde_json::from_str(geojson)
        .unwrap_or_else(|e| panic!("output is not valid JSON ({e}): {geojson}"));
    value["features"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| f["properties"]["name"].as_str().unwrap().to_string())
        .collect()
}

#[test]
fn geoconvert_csv_latlon_skips_unusable_rows() {
    // The FIRST data row is unusable: skipped rows must not leave a gap in the
    // feature indexes, or GeoJsonWriter emits `"features": [,{...}` (invalid JSON).
    let wrk = Workdir::new("geoconvert_csv_latlon_skips_unusable_rows");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "lat", "lon"],
            svec!["empty", "", ""],
            svec!["A", " 40.7 ", " -74.0 "],
            svec!["nan", "NaN", "inf"],
            svec!["text", "abc", "1"],
            svec!["range", "200", "500"],
            svec!["B", "41", "-73"],
        ],
    );
    let mut cmd = wrk.command("geoconvert");
    cmd.args(["data.csv", "csv", "geojson", "-y", "lat", "-x", "lon"]);

    let output = wrk.output(&mut cmd);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    // padded values are trimmed and kept; everything else is skipped
    assert_eq!(feature_names(&stdout), vec!["A", "B"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains(
            "Skipped 4 of 6 rows with unusable geometry (1 empty, 2 invalid coordinates, 1 out of \
             range; first at line 2)."
        ),
        "got: {stderr}"
    );
    assert!(!stderr.contains("swapped"), "got: {stderr}");
}

#[test]
fn geoconvert_csv_latlon_swapped_hint() {
    let wrk = Workdir::new("geoconvert_csv_latlon_swapped_hint");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "lat", "lon"],
            svec!["ok", "37.77", "-122.42"],
            svec!["swapped1", "-122.42", "37.77"],
            svec!["swapped2", "-118.24", "34.05"],
        ],
    );
    let mut cmd = wrk.command("geoconvert");
    cmd.args(["data.csv", "csv", "geojson", "-y", "lat", "-x", "lon"]);

    let output = wrk.output(&mut cmd);
    assert!(output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("2 out of range") && stderr.contains("swapped?"),
        "got: {stderr}"
    );
}

#[test]
fn geoconvert_csv_latlon_all_skipped_is_an_error() {
    // State Plane feet in the lat/lon columns: every row is out of range, which
    // must fail rather than exit 0 with an empty FeatureCollection.
    let wrk = Workdir::new("geoconvert_csv_latlon_all_skipped_is_an_error");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "lat", "lon"],
            svec!["A", "221000", "992000"],
            svec!["B", "183000", "999000"],
        ],
    );
    let mut cmd = wrk.command("geoconvert");
    cmd.args(["data.csv", "csv", "geojson", "-y", "lat", "-x", "lon"]);

    let stderr = wrk.stderr_on_error(&mut cmd);
    assert!(
        stderr.contains("All 2 rows were skipped") && stderr.contains("projected coordinates"),
        "got: {stderr}"
    );
}

#[test]
fn geoconvert_csv_wkt_skips_unusable_rows() {
    let wrk = Workdir::new("geoconvert_csv_wkt_skips_unusable_rows");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "geometry"],
            svec!["A", "POINT(-74 40.7)"],
            svec!["bad", "NOT WKT"],
            svec!["empty", ""],
            svec!["B", "LINESTRING(1 2, 3 4)"],
        ],
    );
    let mut cmd = wrk.command("geoconvert");
    cmd.args(["data.csv", "csv", "geojson", "-g", "geometry"]);

    let output = wrk.output(&mut cmd);
    assert!(output.status.success());
    assert_eq!(
        feature_names(&String::from_utf8_lossy(&output.stdout)),
        vec!["A", "B"]
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Skipped 2 of 4 rows") && stderr.contains("1 empty, 1 invalid WKT"),
        "got: {stderr}"
    );
}

#[test]
fn geoconvert_csv_missing_column_writes_no_output() {
    // A misnamed column must fail before the output file is created.
    let wrk = Workdir::new("geoconvert_csv_missing_column_writes_no_output");
    wrk.create(
        "data.csv",
        vec![svec!["name", "geometry"], svec!["A", "POINT(1 2)"]],
    );
    let mut cmd = wrk.command("geoconvert");
    cmd.args([
        "data.csv",
        "csv",
        "geojson",
        "-g",
        "nope",
        "-o",
        "out.geojson",
    ]);

    let stderr = wrk.stderr_on_error(&mut cmd);
    assert!(
        stderr.contains("Geometry column 'nope' not found"),
        "got: {stderr}"
    );
    assert!(!wrk.path("out.geojson").exists());
}

#[test]
fn geoconvert_csv_wkt_to_csv() {
    // CSV-to-CSV no longer requires --max-length in --geometry mode
    let wrk = Workdir::new("geoconvert_csv_wkt_to_csv");
    wrk.create(
        "data.csv",
        vec![svec!["name", "geometry"], svec!["A", "POINT(1 2)"]],
    );
    let mut cmd = wrk.command("geoconvert");
    cmd.args(["data.csv", "csv", "csv", "-g", "geometry"]);

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    assert_eq!(
        got,
        vec![svec!["geometry", "name"], svec!["POINT(1 2)", "A"]]
    );
}

#[test]
fn geoconvert_tsv_and_delimiter_input() {
    let wrk = Workdir::new("geoconvert_tsv_and_delimiter_input");
    wrk.create_from_string("data.tsv", "name\tlat\tlon\nA\t40.7\t-74\n");
    wrk.create_from_string("semicolons.csv", "name;lat;lon\nA;40.7;-74\n");
    let expected = vec![svec!["geometry", "name"], svec!["POINT(-74 40.7)", "A"]];

    let mut cmd = wrk.command("geoconvert");
    cmd.args(["data.tsv", "csv", "csv", "-y", "lat", "-x", "lon"]);
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    assert_eq!(got, expected);

    let mut cmd = wrk.command("geoconvert");
    cmd.args([
        "semicolons.csv",
        "csv",
        "csv",
        "-y",
        "lat",
        "-x",
        "lon",
        "-d",
        ";",
    ]);
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    assert_eq!(got, expected);
}

#[test]
fn geoconvert_csv_to_geojsonl_and_svg() {
    let wrk = Workdir::new("geoconvert_csv_to_geojsonl_and_svg");
    wrk.create(
        "data.csv",
        vec![svec!["name", "lat", "lon"], svec!["A", "40.7", "-74"]],
    );

    let mut cmd = wrk.command("geoconvert");
    cmd.args(["data.csv", "csv", "geojsonl", "-y", "lat", "-x", "lon"]);
    let got: String = wrk.stdout_on_success(&mut cmd);
    let feature: serde_json::Value = serde_json::from_str(got.trim()).unwrap();
    assert_eq!(
        feature["geometry"]["coordinates"],
        serde_json::json!([-74, 40.7])
    );
    assert_eq!(feature["properties"]["name"], "A");

    let mut cmd = wrk.command("geoconvert");
    cmd.args(["data.csv", "csv", "svg", "-y", "lat", "-x", "lon"]);
    let got: String = wrk.stdout_on_success(&mut cmd);
    assert!(got.contains(r#"<path d="M -74 40.7 Z"/>"#), "got: {got}");
}

/// Copy a test shapefile's .shp/.shx/.dbf/.prj into the workdir and return the
/// .shp path. Fixtures were generated with pyshp (MIT) from made-up data:
/// three NYC borough points in WGS-84 (Queens has a null `pop`), and two in
/// NY State Plane Long Island (EPSG:2263, US feet).
fn load_shapefile(wrk: &Workdir, stem: &str) -> String {
    for ext in ["shx", "dbf", "prj"] {
        wrk.load_test_file(&format!("{stem}.{ext}"));
    }
    wrk.load_test_file(&format!("{stem}.shp"))
}

#[test]
fn geoconvert_shp_to_csv_keeps_dbf_column_order() {
    // geozero yields a record's fields in HashMap order and drops nulls, so
    // CsvWriter used to panic ("CSV features must all have the same column names").
    // Repeat to catch a lucky HashMap order.
    let wrk = Workdir::new("geoconvert_shp_to_csv_keeps_dbf_column_order");
    let shp = load_shapefile(&wrk, "geoconvert_points_wgs84");
    let expected = vec![
        svec!["geometry", "name", "pop"],
        svec!["POINT(-73.9712 40.7831)", "Manhattan", "1694251"],
        svec!["POINT(-73.9442 40.6782)", "Brooklyn", "2736074"],
        svec!["POINT(-73.7949 40.7282)", "Queens", ""],
    ];
    for _ in 0..5 {
        let mut cmd = wrk.command("geoconvert");
        cmd.arg(&shp).args(["shp", "csv"]);
        let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
        assert_eq!(got, expected);
    }

    let mut cmd = wrk.command("geoconvert");
    cmd.arg(&shp).args(["shp", "csv", "--max-length", "12"]);
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    assert_eq!(got[1], svec!["POINT(-73.97...", "Manhattan", "1694251"]);
}

#[test]
fn geoconvert_shp_to_geojson_geojsonl_svg() {
    let wrk = Workdir::new("geoconvert_shp_to_geojson_geojsonl_svg");
    let shp = load_shapefile(&wrk, "geoconvert_points_wgs84");

    let mut cmd = wrk.command("geoconvert");
    cmd.arg(&shp).args(["shp", "geojson"]);
    let output = wrk.output(&mut cmd);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(
        feature_names(&stdout),
        vec!["Manhattan", "Brooklyn", "Queens"]
    );
    // properties come out in .dbf field order
    assert!(
        stdout.contains(r#""properties": {"name": "Manhattan", "pop": 1694251}"#),
        "got: {stdout}"
    );
    // a WGS-84 .prj is not warned about
    assert!(
        output.stderr.is_empty(),
        "{:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    let mut cmd = wrk.command("geoconvert");
    cmd.arg(&shp).args(["shp", "geojsonl"]);
    let got: String = wrk.stdout_on_success(&mut cmd);
    assert_eq!(got.lines().count(), 3);

    let mut cmd = wrk.command("geoconvert");
    cmd.arg(&shp).args(["shp", "svg"]);
    let got: String = wrk.stdout_on_success(&mut cmd);
    assert_eq!(got.matches("<path ").count(), 3, "got: {got}");
}

#[test]
fn geoconvert_shp_projected_prj_warns() {
    let wrk = Workdir::new("geoconvert_shp_projected_prj_warns");
    let shp = load_shapefile(&wrk, "geoconvert_points_nysp");

    let mut cmd = wrk.command("geoconvert");
    cmd.arg(&shp).args(["shp", "geojsonl"]);
    let output = wrk.output(&mut cmd);
    assert!(output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr
            .contains(r#"projected CRS "NAD_1983_StatePlane_New_York_Long_Island_FIPS_3104_Feet""#),
        "got: {stderr}"
    );
}

#[test]
fn geoconvert_shp_missing_component_names_file() {
    let wrk = Workdir::new("geoconvert_shp_missing_component_names_file");
    wrk.load_test_file("geoconvert_points_wgs84.shp");
    wrk.load_test_file("geoconvert_points_wgs84.shx");

    let mut cmd = wrk.command("geoconvert");
    cmd.args([
        "geoconvert_points_wgs84.shp",
        "shp",
        "geojson",
        "-o",
        "out.geojson",
    ]);
    let stderr = wrk.stderr_on_error(&mut cmd);
    assert!(
        stderr.contains("Shapefile component 'geoconvert_points_wgs84.dbf' not found"),
        "got: {stderr}"
    );
    assert!(!wrk.path("out.geojson").exists());
}
