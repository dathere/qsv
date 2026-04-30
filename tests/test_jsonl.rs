use crate::workdir::Workdir;

#[test]
fn jsonl_simple() {
    let wrk = Workdir::new("jsonl_simple");
    wrk.create_from_string(
        "data.jsonl",
        r#"{"id":1,"father":"Mark","mother":"Charlotte","oldest_child":"Tom","boy":true}
{"id":2,"father":"John","mother":"Ann","oldest_child":"Jessika","boy":false}
{"id":3,"father":"Bob","mother":"Monika","oldest_child":"Jerry","boy":true}"#,
    );
    let mut cmd = wrk.command("jsonl");
    cmd.arg("data.jsonl");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["id", "father", "mother", "oldest_child", "boy"],
        svec!["1", "Mark", "Charlotte", "Tom", "true"],
        svec!["2", "John", "Ann", "Jessika", "false"],
        svec!["3", "Bob", "Monika", "Jerry", "true"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn jsonl_simple_delimiter() {
    let wrk = Workdir::new("jsonl_simple_delimiter");
    wrk.create_from_string(
        "data.jsonl",
        r#"{"id":1,"father":"Mark","mother":"Charlotte","oldest_child":"Tom","boy":true}
{"id":2,"father":"John","mother":"Ann","oldest_child":"Jessika","boy":false}
{"id":3,"father":"Bob","mother":"Monika","oldest_child":"Jerry","boy":true}"#,
    );
    let mut cmd = wrk.command("jsonl");
    cmd.args(["--delimiter", ";"]).arg("data.jsonl");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["id;father;mother;oldest_child;boy"],
        svec!["1;Mark;Charlotte;Tom;true"],
        svec!["2;John;Ann;Jessika;false"],
        svec!["3;Bob;Monika;Jerry;true"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn jsonl_simple_error() {
    let wrk = Workdir::new("jsonl");
    wrk.create_from_string(
        "data.jsonl",
        r#"{"id":1,"father":"Mark","mother":"Charlotte","oldest_child":"Tom","boy":true}
{"id":2,"father":"John","mother":"Ann","oldest_child":"Jessika","boy":false}
{"id":3,"father":"Bob","mother":"Monika","oldest_child":"Jerry","boy":true}
{"id":4,"father":"Gad","mother":"Maria","oldest_child":"Hesus"Espiritu","boy":true}
{"id":5,"father":"Donald","mother":"Melania","oldest_child":"Ivanka","boy":false}"#,
    );
    let mut cmd = wrk.command("jsonl");
    cmd.arg("data.jsonl");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // 4 and 5 are not displayed as jsonl encounters an error and just stops
    let expected = vec![
        svec!["id", "father", "mother", "oldest_child", "boy"],
        svec!["1", "Mark", "Charlotte", "Tom", "true"],
        svec!["2", "John", "Ann", "Jessika", "false"],
        svec!["3", "Bob", "Monika", "Jerry", "true"],
    ];
    assert_eq!(got, expected);

    wrk.assert_err(&mut cmd);
}

#[test]
fn jsonl_simple_ignore_error() {
    let wrk = Workdir::new("jsonl");
    wrk.create_from_string(
        "data.jsonl",
        r#"{"id":1,"father":"Mark","mother":"Charlotte","oldest_child":"Tom","boy":true}
{"id":2,"father":"John","mother":"Ann","oldest_child":"Jessika","boy":false}
{"id":3,"father":"Bob","mother":"Monika","oldest_child":"Jerry","boy":true}
{"id":4,"father":"Gad","mother":"Maria","oldest_child":"Hesus"Espiritu","boy":true}
{"id":5,"father":"Donald","mother":"Melania","oldest_child":"Ivanka","boy":false}"#,
    );
    let mut cmd = wrk.command("jsonl");
    cmd.arg("--ignore-errors").arg("data.jsonl");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // 4 is ignored as its invalid jsonl
    let expected = vec![
        svec!["id", "father", "mother", "oldest_child", "boy"],
        svec!["1", "Mark", "Charlotte", "Tom", "true"],
        svec!["2", "John", "Ann", "Jessika", "false"],
        svec!["3", "Bob", "Monika", "Jerry", "true"],
        svec!["5", "Donald", "Melania", "Ivanka", "false"],
    ];
    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd);
}

#[test]
fn jsonl_ignore_error_first_line_malformed() {
    let wrk = Workdir::new("jsonl_ignore_error_first_line_malformed");
    wrk.create_from_string(
        "data.jsonl",
        r#"{"id":1,"name":"Mark"Espiritu","oldest_child":"Tom"}
{"id":2,"name":"John","oldest_child":"Jessika"}
{"id":3,"name":"Bob","oldest_child":"Jerry"}"#,
    );
    let mut cmd = wrk.command("jsonl");
    cmd.arg("--ignore-errors").arg("data.jsonl");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // line 1 is malformed; --ignore-errors must skip it and infer headers
    // from line 2 (the first parseable line).
    let expected = vec![
        svec!["id", "name", "oldest_child"],
        svec!["2", "John", "Jessika"],
        svec!["3", "Bob", "Jerry"],
    ];
    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd);
}

#[test]
fn jsonl_bare_scalars() {
    let wrk = Workdir::new("jsonl_bare_scalars");
    wrk.create_from_string("data.jsonl", "42\n\"hello\"\ntrue\nnull\n[1,2,3]");
    let mut cmd = wrk.command("jsonl");
    cmd.arg("data.jsonl");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // top-level non-Object roots: header is the synthesized "value" column,
    // each row is the stringified root (null -> empty cell, array -> joined).
    let expected = vec![
        svec!["value"],
        svec!["42"],
        svec!["hello"],
        svec!["true"],
        svec![""],
        svec!["1,2,3"],
    ];
    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd);
}

#[test]
fn jsonl_error_line_number() {
    let wrk = Workdir::new("jsonl_error_line_number");
    wrk.create_from_string(
        "data.jsonl",
        r#"{"id":1,"name":"Alice"}
{"id":2,"name":"Bob"}
{"id":3,"name":"Carol"}
{"id":4,"name":"Dave"NOT_VALID}
{"id":5,"name":"Eve"}"#,
    );
    let mut cmd = wrk.command("jsonl");
    cmd.arg("data.jsonl");

    let stderr = wrk.output_stderr(&mut cmd);
    assert!(
        stderr.contains("Could not parse input line 4 as JSON"),
        "expected stderr to reference input line 4, got: {stderr}"
    );

    wrk.assert_err(&mut cmd);
}

#[test]
fn jsonl_nested() {
    let wrk = Workdir::new("jsonl");
    wrk.create_from_string(
        "data.jsonl",
        r#"{"id":1,"father":"Mark","mother":"Charlotte","children":["Tom"]}
{"id":2,"father":"John","mother":"Ann","children":["Jessika","Antony","Jack"]}
{"id":3,"father":"Bob","mother":"Monika","children":["Jerry","Karol"]}"#,
    );
    let mut cmd = wrk.command("jsonl");
    cmd.arg("data.jsonl");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["id", "father", "mother", "children"],
        svec!["1", "Mark", "Charlotte", "\"Tom\""],
        svec!["2", "John", "Ann", "\"Jessika\",\"Antony\",\"Jack\""],
        svec!["3", "Bob", "Monika", "\"Jerry\",\"Karol\""],
    ];
    assert_eq!(got, expected);
}
