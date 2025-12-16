use crate::{CsvData, qcheck, workdir::Workdir};

fn prop_transpose(name: &str, rows: CsvData, streaming: bool) -> bool {
    let wrk = Workdir::new(name);
    wrk.create("in.csv", rows.clone());

    let mut cmd = wrk.command("transpose");
    cmd.arg("in.csv");
    if streaming {
        cmd.arg("--multipass");
    }

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    let mut expected = vec![];

    let nrows = rows.len();
    let ncols = if !rows.is_empty() { rows[0].len() } else { 0 };

    for i in 0..ncols {
        let mut expected_row = vec![];
        for j in 0..nrows {
            expected_row.push(rows[j][i].clone());
        }
        expected.push(expected_row);
    }
    rassert_eq!(got, expected)
}

#[test]
fn prop_transpose_in_memory() {
    fn p(rows: CsvData) -> bool {
        prop_transpose("prop_transpose_in_memory", rows, false)
    }
    qcheck(p as fn(CsvData) -> bool);
}

#[test]
fn prop_transpose_multipass() {
    fn p(rows: CsvData) -> bool {
        prop_transpose("prop_transpose_multipass", rows, true)
    }
    qcheck(p as fn(CsvData) -> bool);
}

#[test]
fn transpose_long_format() {
    let wrk = Workdir::new("transpose_long_format");

    // Create a wide-format CSV similar to stats output
    let wide_format = vec![
        svec!["field", "type", "is_ascii", "sum", "min", "max"],
        svec!["name", "String", "true", "", "Alice", "John"],
        svec!["age", "Integer", "", "104", "6", "53"],
    ];

    wrk.create("in.csv", wide_format);

    let mut cmd = wrk.command("transpose");
    cmd.arg("--long").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Expected long format: field, attribute, value
    // Empty values should be skipped
    let expected = vec![
        svec!["field", "attribute", "value"],
        svec!["name", "type", "String"],
        svec!["name", "is_ascii", "true"],
        svec!["name", "min", "Alice"],
        svec!["name", "max", "John"],
        svec!["age", "type", "Integer"],
        svec!["age", "sum", "104"],
        svec!["age", "min", "6"],
        svec!["age", "max", "53"],
    ];

    wrk.assert_success(&mut cmd);
    assert_eq!(got, expected);
}

#[test]
fn transpose_long_format_empty_csv() {
    let wrk = Workdir::new("transpose_long_format_empty_csv");

    // Create CSV with only headers
    let wide_format = vec![svec!["field", "type", "is_ascii"]];

    wrk.create("in.csv", wide_format);

    let mut cmd = wrk.command("transpose");
    cmd.arg("--long").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Should only have headers, no data rows
    let expected = vec![svec!["field", "attribute", "value"]];

    wrk.assert_success(&mut cmd);
    assert_eq!(got, expected);
}

#[test]
fn transpose_long_format_all_empty() {
    let wrk = Workdir::new("transpose_long_format_all_empty");

    // Create CSV where all attribute values are empty
    let wide_format = vec![
        svec!["field", "type", "sum", "min"],
        svec!["name", "", "", ""],
    ];

    wrk.create("in.csv", wide_format);

    let mut cmd = wrk.command("transpose");
    cmd.arg("--long").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Should only have headers, all values were empty and skipped
    let expected = vec![svec!["field", "attribute", "value"]];

    wrk.assert_success(&mut cmd);
    assert_eq!(got, expected);
}
