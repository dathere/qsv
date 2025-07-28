// Lesson 3: qsv and JSON
// https://100.dathere.com/lessons/3

use crate::workdir::Workdir;

// Task 1
#[test]
#[cfg(not(feature = "datapusher_plus"))]
// #[ignore = "ignore this test for now as it's flaky coz of json_objects_to_csv crate issues"]
fn flowers_json_to_csv() {
    let wrk = Workdir::new("flowers_json_to_csv");
    let flowers_json_file = wrk.load_test_file("flowers.json");

    let mut json_cmd = wrk.command("json");
    json_cmd.arg(flowers_json_file.as_str());

    wrk.run(&mut json_cmd);

    wrk.assert_success(&mut json_cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut json_cmd);

    let expected = vec![
        svec!["name", "primary_color", "available", "quantity"],
        svec!["tulip", "purple", "true", "4"],
        svec!["rose", "red", "true", "6"],
        svec!["sunflower", "yellow", "false", "0"],
    ];
    assert_eq!(got, expected);
}

// Task 2
#[test]
#[cfg(not(feature = "datapusher_plus"))]
// #[ignore = "ignore this test for now as it's flaky coz of json_objects_to_csv crate issues"]
fn flowers_nested_json_to_csv() {
    let wrk = Workdir::new("flowers_nested_json_to_csv");
    let flowers_nested_json_file = wrk.load_test_file("flowers_nested.json");

    let mut cmd = wrk.command("json");
    cmd.args(vec![flowers_nested_json_file.as_str(), "--jaq", ".roses"]);

    wrk.run(&mut cmd);

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["color", "quantity"],
        svec!["red", "4"],
        svec!["white", "1"],
        svec!["pink", "1"],
    ];
    assert_eq!(got, expected);
}

// Task 3
#[test]
#[cfg(not(feature = "datapusher_plus"))]
// #[ignore = "ignore this test for now as it's flaky coz of json_objects_to_csv crate issues"]
fn buses_csv_to_json() {
    let wrk = Workdir::new("buses_csv_to_json");
    let buses_csv_file = wrk.load_test_file("buses.csv");
    let mut cmd = wrk.command("slice");
    cmd.arg(buses_csv_file.as_str()).arg("--json");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"[{"id":"001","primary_color":"black","secondary_color":"blue","length":"full","air_conditioner":"true","amenities":"wheelchair ramp, tissue boxes, cup holders, USB ports"},{"id":"002","primary_color":"black","secondary_color":"red","length":"full","air_conditioner":"true","amenities":"wheelchair ramp, tissue boxes, USB ports"},{"id":"003","primary_color":"white","secondary_color":"blue","length":"half","air_conditioner":"true","amenities":"wheelchair ramp, tissue boxes"},{"id":"004","primary_color":"orange","secondary_color":"blue","length":"full","air_conditioner":"false","amenities":"wheelchair ramp, tissue boxes, USB ports"},{"id":"005","primary_color":"black","secondary_color":"blue","length":"full","air_conditioner":"true","amenities":"wheelchair ramp, tissue boxes, cup holders, USB ports"}]"#;
    assert_eq!(got, expected);
}
