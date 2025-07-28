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
