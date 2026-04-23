use crate::workdir::Workdir;

#[test]
fn implode_basic() {
    let wrk = Workdir::new("implode");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "color"],
            svec!["John", "blue"],
            svec!["John", "yellow"],
            svec!["John", "light red"],
            svec!["Mary", "red"],
        ],
    );
    let mut cmd = wrk.command("implode");
    cmd.args(["-k", "name", "-v", "color"])
        .arg("; ")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "color"],
        svec!["John", "blue; yellow; light red"],
        svec!["Mary", "red"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn implode_rename() {
    let wrk = Workdir::new("implode");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "color"],
            svec!["John", "blue"],
            svec!["John", "yellow"],
            svec!["Mary", "red"],
        ],
    );
    let mut cmd = wrk.command("implode");
    cmd.args(["-k", "name", "-v", "color", "-r", "colors"])
        .arg(";")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "colors"],
        svec!["John", "blue;yellow"],
        svec!["Mary", "red"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn implode_composite_key() {
    let wrk = Workdir::new("implode");
    wrk.create(
        "data.csv",
        vec![
            svec!["country", "city", "visitor"],
            svec!["US", "NYC", "Alice"],
            svec!["US", "NYC", "Bob"],
            svec!["US", "LA", "Carol"],
            svec!["UK", "London", "Dan"],
            svec!["US", "NYC", "Eve"],
        ],
    );
    let mut cmd = wrk.command("implode");
    cmd.args(["-k", "country,city", "-v", "visitor"])
        .arg("|")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["country", "city", "visitor"],
        svec!["US", "NYC", "Alice|Bob|Eve"],
        svec!["US", "LA", "Carol"],
        svec!["UK", "London", "Dan"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn implode_empty_included_by_default() {
    let wrk = Workdir::new("implode");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "color"],
            svec!["John", "red"],
            svec!["John", ""],
            svec!["John", "blue"],
        ],
    );
    let mut cmd = wrk.command("implode");
    cmd.args(["-k", "name", "-v", "color"])
        .arg(";")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["name", "color"], svec!["John", "red;;blue"]];
    assert_eq!(got, expected);
}

#[test]
fn implode_skip_empty() {
    let wrk = Workdir::new("implode");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "color"],
            svec!["John", "red"],
            svec!["John", ""],
            svec!["John", "blue"],
        ],
    );
    let mut cmd = wrk.command("implode");
    cmd.args(["-k", "name", "-v", "color", "--skip-empty"])
        .arg(";")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["name", "color"], svec!["John", "red;blue"]];
    assert_eq!(got, expected);
}

#[test]
fn implode_sorted_streaming() {
    let wrk = Workdir::new("implode");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "color"],
            svec!["John", "blue"],
            svec!["John", "yellow"],
            svec!["John", "light red"],
            svec!["Mary", "red"],
        ],
    );
    let mut cmd = wrk.command("implode");
    cmd.args(["-k", "name", "-v", "color", "--sorted"])
        .arg("; ")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "color"],
        svec!["John", "blue; yellow; light red"],
        svec!["Mary", "red"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn implode_no_headers() {
    let wrk = Workdir::new("implode");
    wrk.create(
        "data.csv",
        vec![
            svec!["John", "blue"],
            svec!["John", "yellow"],
            svec!["Mary", "red"],
        ],
    );
    let mut cmd = wrk.command("implode");
    cmd.args(["-k", "1", "-v", "2", "--no-headers"])
        .arg(";")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["John", "blue;yellow"], svec!["Mary", "red"]];
    assert_eq!(got, expected);
}

#[test]
fn implode_roundtrip_with_explode() {
    let wrk = Workdir::new("implode");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "color"],
            svec!["John", "blue"],
            svec!["John", "yellow"],
            svec!["John", "light red"],
            svec!["Mary", "red"],
        ],
    );

    let mut implode_cmd = wrk.command("implode");
    implode_cmd
        .args(["-k", "name", "-v", "color"])
        .arg("|")
        .arg("data.csv");
    let imploded: String = wrk.stdout(&mut implode_cmd);
    wrk.create_from_string("imploded.csv", &format!("{imploded}\n"));

    let mut explode_cmd = wrk.command("explode");
    explode_cmd.arg("color").arg("|").arg("imploded.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut explode_cmd);
    let expected = vec![
        svec!["name", "color"],
        svec!["John", "blue"],
        svec!["John", "yellow"],
        svec!["John", "light red"],
        svec!["Mary", "red"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn implode_value_must_be_single_column() {
    let wrk = Workdir::new("implode");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "color", "shade"],
            svec!["John", "blue", "dark"],
        ],
    );
    let mut cmd = wrk.command("implode");
    cmd.args(["-k", "name", "-v", "color,shade"])
        .arg(";")
        .arg("data.csv");

    let stderr = wrk.output_stderr(&mut cmd);
    assert!(
        stderr.contains("--value must resolve to exactly one column"),
        "unexpected stderr: {stderr}"
    );
}

#[test]
fn implode_value_cannot_be_a_key_column() {
    let wrk = Workdir::new("implode");
    wrk.create(
        "data.csv",
        vec![svec!["name", "color"], svec!["John", "blue"]],
    );
    let mut cmd = wrk.command("implode");
    cmd.args(["-k", "name", "-v", "name"])
        .arg(";")
        .arg("data.csv");

    let stderr = wrk.output_stderr(&mut cmd);
    assert!(
        stderr.contains("--value column must not also be a key column"),
        "unexpected stderr: {stderr}"
    );
}

#[test]
fn implode_unknown_key_column() {
    let wrk = Workdir::new("implode");
    wrk.create(
        "data.csv",
        vec![svec!["name", "color"], svec!["John", "blue"]],
    );
    let mut cmd = wrk.command("implode");
    cmd.args(["-k", "nope", "-v", "color"])
        .arg(";")
        .arg("data.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
fn implode_empty_input() {
    let wrk = Workdir::new("implode");
    wrk.create("data.csv", vec![svec!["name", "color"]]);
    let mut cmd = wrk.command("implode");
    cmd.args(["-k", "name", "-v", "color"])
        .arg(";")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["name", "color"]];
    assert_eq!(got, expected);
}

#[test]
fn implode_sorted_skip_empty() {
    let wrk = Workdir::new("implode");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "color"],
            svec!["John", "red"],
            svec!["John", ""],
            svec!["John", "blue"],
            svec!["Mary", ""],
            svec!["Mary", "green"],
        ],
    );
    let mut cmd = wrk.command("implode");
    cmd.args(["-k", "name", "-v", "color", "--sorted", "--skip-empty"])
        .arg(";")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "color"],
        svec!["John", "red;blue"],
        svec!["Mary", "green"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn implode_sorted_composite_key() {
    let wrk = Workdir::new("implode");
    wrk.create(
        "data.csv",
        vec![
            svec!["country", "city", "visitor"],
            svec!["UK", "London", "Dan"],
            svec!["US", "LA", "Carol"],
            svec!["US", "NYC", "Alice"],
            svec!["US", "NYC", "Bob"],
            svec!["US", "NYC", "Eve"],
        ],
    );
    let mut cmd = wrk.command("implode");
    cmd.args(["-k", "country,city", "-v", "visitor", "--sorted"])
        .arg("|")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["country", "city", "visitor"],
        svec!["UK", "London", "Dan"],
        svec!["US", "LA", "Carol"],
        svec!["US", "NYC", "Alice|Bob|Eve"],
    ];
    assert_eq!(got, expected);
}
