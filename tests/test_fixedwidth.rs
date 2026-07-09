use crate::workdir::Workdir;

#[test]
fn fixedwidth_header_comment() {
    let wrk = Workdir::new("fixedwidth_header_comment");
    wrk.create_from_string(
        "in.txt",
        "#1,11,18\nJohn      Smith  042\nJane      Doe    017\n",
    );

    let mut cmd = wrk.command("fixedwidth");
    cmd.arg("in.txt");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![svec!["John", "Smith", "042"], svec!["Jane", "Doe", "017"]];
    assert_eq!(got, expected);
}

#[test]
fn fixedwidth_explicit_positions() {
    let wrk = Workdir::new("fixedwidth_explicit_positions");
    wrk.create_from_string("in.txt", "John      Smith  042\nJane      Doe    017\n");

    let mut cmd = wrk.command("fixedwidth");
    cmd.arg("in.txt").args(["--positions", "1,11,18"]);

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![svec!["John", "Smith", "042"], svec!["Jane", "Doe", "017"]];
    assert_eq!(got, expected);
}

#[test]
fn fixedwidth_explicit_widths() {
    let wrk = Workdir::new("fixedwidth_explicit_widths");
    wrk.create_from_string("in.txt", "John      Smith  042\nJane      Doe    017\n");

    let mut cmd = wrk.command("fixedwidth");
    cmd.arg("in.txt").args(["--widths", "10,7,3"]);

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![svec!["John", "Smith", "042"], svec!["Jane", "Doe", "017"]];
    assert_eq!(got, expected);
}

#[test]
fn fixedwidth_short_line_pads_with_empty_field() {
    let wrk = Workdir::new("fixedwidth_short_line_pads_with_empty_field");
    // The second data line is shorter than the last column's start position.
    wrk.create_from_string("in.txt", "#1,11,18\nJohn      Smith  042\nJane      Doe\n");

    let mut cmd = wrk.command("fixedwidth");
    cmd.arg("in.txt");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let expected = vec![svec!["John", "Smith", "042"], svec!["Jane", "Doe", ""]];
    assert_eq!(got, expected);
}

#[test]
fn fixedwidth_positions_and_widths_are_mutually_exclusive() {
    let wrk = Workdir::new("fixedwidth_positions_and_widths_are_mutually_exclusive");
    wrk.create_from_string("in.txt", "John      Smith  042\n");

    let mut cmd = wrk.command("fixedwidth");
    cmd.arg("in.txt")
        .args(["--positions", "1,11,18"])
        .args(["--widths", "10,7,3"]);

    wrk.assert_err(&mut cmd);
}

#[test]
fn fixedwidth_no_positions_and_no_header_comment_errors() {
    let wrk = Workdir::new("fixedwidth_no_positions_and_no_header_comment_errors");
    wrk.create_from_string("in.txt", "John      Smith  042\n");

    let mut cmd = wrk.command("fixedwidth");
    cmd.arg("in.txt");

    wrk.assert_err(&mut cmd);
}

#[test]
fn fixedwidth_roundtrips_with_table_leftfwf() {
    let wrk = Workdir::new("fixedwidth_roundtrips_with_table_leftfwf");
    let rows = vec![
        svec!["name", "amount"],
        svec!["John Smith", "42"],
        svec!["Jane Doe", "1017"],
    ];
    wrk.create("in.csv", rows.clone());

    let mut table_cmd = wrk.command("table");
    table_cmd.arg("in.csv").args(["--align", "leftfwf"]);
    let fwf: String = wrk.stdout(&mut table_cmd);
    wrk.create_from_string("fwf.txt", &format!("{fwf}\n"));

    let mut cmd = wrk.command("fixedwidth");
    cmd.arg("fwf.txt");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    assert_eq!(got, rows);
}
