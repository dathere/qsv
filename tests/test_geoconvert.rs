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

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
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
    cmd.args(&["name,Shape_Length,Shape_Area"])
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

    wrk.assert_success(&mut cmd);

    let got: String = wrk.stdout(&mut cmd);
    // Coordinates must be [longitude, latitude]: longitude (125.6) first.
    assert!(
        got.contains("\"coordinates\":[125.6,10.1]"),
        "expected [lon, lat] order, got: {got}"
    );
    assert!(
        !got.contains("\"coordinates\":[10.1,125.6]"),
        "found [lat, lon] order (regression!), got: {got}"
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

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
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

    let got = wrk.output_stderr(&mut cmd);
    assert!(
        got.contains("--latitude and --longitude must be used together"),
        "expected partial-flag error, got: {got}"
    );
    wrk.assert_err(&mut cmd);
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

    wrk.assert_success(&mut cmd);

    let got: String = wrk.stdout(&mut cmd);

    let expected = r#"geometry,OBJECTID,name,Shape_Length,Shape_Area
POLYGON((-...,1,Abbott,0,0
MULTIPOLYG...,2,Abernathy,0,0
POLYGON((-...,3,Abilene,0,0
POLYGON((-...,4,Ackerly,0,0
POLYGON((-...,5,Addison,0,0"#;
    assert_eq!(got, expected);
}
