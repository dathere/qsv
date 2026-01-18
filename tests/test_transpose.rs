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
    cmd.args(["--long", "1"]).arg("in.csv");

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
    cmd.args(["--long", "1"]).arg("in.csv");

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
    cmd.args(["--long", "1"]).arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Should only have headers, all values were empty and skipped
    let expected = vec![svec!["field", "attribute", "value"]];

    wrk.assert_success(&mut cmd);
    assert_eq!(got, expected);
}

#[test]
fn transpose_long_format_single_column() {
    let wrk = Workdir::new("transpose_long_format_single_column");

    // Create CSV with only one column (field column, no attributes)
    let wide_format = vec![svec!["field"], svec!["name"], svec!["age"]];

    wrk.create("in.csv", wide_format);

    let mut cmd = wrk.command("transpose");
    cmd.args(["--long", "1"]).arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Should only have headers, no attribute columns to process
    let expected = vec![svec!["field", "attribute", "value"]];

    wrk.assert_success(&mut cmd);
    assert_eq!(got, expected);
}

#[test]
fn transpose_long_multipass_mutually_exclusive() {
    let wrk = Workdir::new("transpose_long_multipass_mutually_exclusive");

    // Create a test CSV file
    let wide_format = vec![
        svec!["field", "type", "value"],
        svec!["name", "String", "Alice"],
    ];

    wrk.create("in.csv", wide_format);

    // Test that --long and --multipass are mutually exclusive
    let mut cmd = wrk.command("transpose");
    cmd.args(["--long", "1"]).arg("--multipass").arg("in.csv");

    // Should fail with an error
    wrk.assert_err(&mut cmd);

    // Verify the error message mentions mutual exclusivity
    let stderr: String = wrk.output_stderr(&mut cmd);
    assert!(
        stderr.contains("mutually exclusive") || stderr.contains("mutually-exclusive"),
        "Expected error message about mutual exclusivity, got: {}",
        stderr
    );
}

#[test]
fn transpose_long_format_by_name() {
    let wrk = Workdir::new("transpose_long_format_by_name");

    // Create a wide-format CSV with a named field column
    let wide_format = vec![
        svec!["id", "field", "type", "value"],
        svec!["1", "name", "String", "Alice"],
        svec!["2", "age", "Integer", "25"],
    ];

    wrk.create("in.csv", wide_format);

    let mut cmd = wrk.command("transpose");
    cmd.args(["--long", "field"]).arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    let expected = vec![
        svec!["field", "attribute", "value"],
        svec!["name", "id", "1"],
        svec!["name", "type", "String"],
        svec!["name", "value", "Alice"],
        svec!["age", "id", "2"],
        svec!["age", "type", "Integer"],
        svec!["age", "value", "25"],
    ];

    wrk.assert_success(&mut cmd);
    assert_eq!(got, expected);
}

#[test]
fn transpose_long_format_by_index() {
    let wrk = Workdir::new("transpose_long_format_by_index");

    // Create a wide-format CSV
    let wide_format = vec![
        svec!["id", "field", "type", "value"],
        svec!["1", "name", "String", "Alice"],
        svec!["2", "age", "Integer", "25"],
    ];

    wrk.create("in.csv", wide_format);

    let mut cmd = wrk.command("transpose");
    cmd.args(["--long", "2"]).arg("in.csv"); // Select second column (1-based index)

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Expected: second column (field) selected as field column
    let expected = vec![
        svec!["field", "attribute", "value"],
        svec!["name", "id", "1"],
        svec!["name", "type", "String"],
        svec!["name", "value", "Alice"],
        svec!["age", "id", "2"],
        svec!["age", "type", "Integer"],
        svec!["age", "value", "25"],
    ];

    wrk.assert_success(&mut cmd);
    assert_eq!(got, expected);
}

#[test]
fn transpose_long_format_by_range() {
    let wrk = Workdir::new("transpose_long_format_by_range");

    // Create a wide-format CSV
    let wide_format = vec![
        svec!["id", "category", "field", "type", "value"],
        svec!["1", "person", "name", "String", "Alice"],
        svec!["2", "person", "age", "Integer", "25"],
    ];

    wrk.create("in.csv", wide_format);

    let mut cmd = wrk.command("transpose");
    cmd.args(["--long", "2-3"]).arg("in.csv"); // Select columns 2-3 as field columns

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Expected: columns 2-3 (category, field) concatenated with | separator
    let expected = vec![
        svec!["field", "attribute", "value"],
        svec!["person|name", "id", "1"],
        svec!["person|name", "type", "String"],
        svec!["person|name", "value", "Alice"],
        svec!["person|age", "id", "2"],
        svec!["person|age", "type", "Integer"],
        svec!["person|age", "value", "25"],
    ];

    wrk.assert_success(&mut cmd);
    assert_eq!(got, expected);
}

#[test]
fn transpose_long_format_by_range_to_end() {
    let wrk = Workdir::new("transpose_long_format_by_range_to_end");

    // Create a wide-format CSV
    let wide_format = vec![
        svec!["id", "category", "field", "type"],
        svec!["1", "person", "name", "String"],
        svec!["2", "person", "age", "Integer"],
    ];

    wrk.create("in.csv", wide_format);

    let mut cmd = wrk.command("transpose");
    cmd.args(["--long", "2-"]).arg("in.csv"); // Select from column 2 to end

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Expected: columns 2-4 (category, field, type) concatenated with | separator
    let expected = vec![
        svec!["field", "attribute", "value"],
        svec!["person|name|String", "id", "1"],
        svec!["person|age|Integer", "id", "2"],
    ];

    wrk.assert_success(&mut cmd);
    assert_eq!(got, expected);
}

#[test]
fn transpose_long_format_by_regex() {
    let wrk = Workdir::new("transpose_long_format_by_regex");

    // Create a wide-format CSV with columns matching a pattern
    let wide_format = vec![
        svec!["id", "field_name", "field_type", "value"],
        svec!["1", "name", "String", "Alice"],
        svec!["2", "age", "Integer", "25"],
    ];

    wrk.create("in.csv", wide_format);

    let mut cmd = wrk.command("transpose");
    cmd.args(["--long", "/^field/"]).arg("in.csv"); // Select columns starting with "field"

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Expected: field_name and field_type concatenated with | separator
    let expected = vec![
        svec!["field", "attribute", "value"],
        svec!["name|String", "id", "1"],
        svec!["name|String", "value", "Alice"],
        svec!["age|Integer", "id", "2"],
        svec!["age|Integer", "value", "25"],
    ];

    wrk.assert_success(&mut cmd);
    assert_eq!(got, expected);
}

#[test]
fn transpose_long_format_multiple_fields_by_name() {
    let wrk = Workdir::new("transpose_long_format_multiple_fields_by_name");

    // Create a wide-format CSV
    let wide_format = vec![
        svec!["id", "category", "field", "type", "value"],
        svec!["1", "person", "name", "String", "Alice"],
        svec!["2", "person", "age", "Integer", "25"],
    ];

    wrk.create("in.csv", wide_format);

    let mut cmd = wrk.command("transpose");
    cmd.args(["--long", "category,field"]).arg("in.csv"); // Select multiple columns by name

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Expected: category and field concatenated with | separator
    let expected = vec![
        svec!["field", "attribute", "value"],
        svec!["person|name", "id", "1"],
        svec!["person|name", "type", "String"],
        svec!["person|name", "value", "Alice"],
        svec!["person|age", "id", "2"],
        svec!["person|age", "type", "Integer"],
        svec!["person|age", "value", "25"],
    ];

    wrk.assert_success(&mut cmd);
    assert_eq!(got, expected);
}

#[test]
fn transpose_long_format_multiple_fields_by_index() {
    let wrk = Workdir::new("transpose_long_format_multiple_fields_by_index");

    // Create a wide-format CSV
    let wide_format = vec![
        svec!["id", "category", "field", "type", "value"],
        svec!["1", "person", "name", "String", "Alice"],
        svec!["2", "person", "age", "Integer", "25"],
    ];

    wrk.create("in.csv", wide_format);

    let mut cmd = wrk.command("transpose");
    cmd.args(["--long", "2,3"]).arg("in.csv"); // Select multiple columns by index

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Expected: columns 2 and 3 (category, field) concatenated with | separator
    let expected = vec![
        svec!["field", "attribute", "value"],
        svec!["person|name", "id", "1"],
        svec!["person|name", "type", "String"],
        svec!["person|name", "value", "Alice"],
        svec!["person|age", "id", "2"],
        svec!["person|age", "type", "Integer"],
        svec!["person|age", "value", "25"],
    ];

    wrk.assert_success(&mut cmd);
    assert_eq!(got, expected);
}

#[test]
fn transpose_long_format_invalid_column() {
    let wrk = Workdir::new("transpose_long_format_invalid_column");

    // Create a test CSV file
    let wide_format = vec![
        svec!["field", "type", "value"],
        svec!["name", "String", "Alice"],
    ];

    wrk.create("in.csv", wide_format);

    // Test that invalid column name fails
    let mut cmd = wrk.command("transpose");
    cmd.args(["--long", "nonexistent"]).arg("in.csv");

    // Should fail with an error
    wrk.assert_err(&mut cmd);
}

#[test]
fn transpose_long_format_invalid_regex() {
    let wrk = Workdir::new("transpose_long_format_invalid_regex");

    // Create a test CSV file
    let wide_format = vec![
        svec!["field", "type", "value"],
        svec!["name", "String", "Alice"],
    ];

    wrk.create("in.csv", wide_format);

    // Test that invalid regex pattern fails
    let mut cmd = wrk.command("transpose");
    cmd.args(["--long", "/[invalid/"]).arg("in.csv"); // Invalid regex: unclosed character class

    // Should fail with an error
    wrk.assert_err(&mut cmd);
}

#[test]
fn transpose_long_format_no_columns_selected() {
    let wrk = Workdir::new("transpose_long_format_no_columns_selected");

    // Create a test CSV file
    let wide_format = vec![
        svec!["field", "type", "value"],
        svec!["name", "String", "Alice"],
    ];

    wrk.create("in.csv", wide_format);

    // Test that regex matching no columns fails
    let mut cmd = wrk.command("transpose");
    cmd.args(["--long", "/^nonexistent/"]).arg("in.csv"); // Regex that matches nothing

    // Should fail with an error
    wrk.assert_err(&mut cmd);

    // Verify the error message mentions no columns selected
    let stderr: String = wrk.output_stderr(&mut cmd);
    let expected = "Column selection error: Selector regex '^nonexistent' does not match any \
                    columns in the CSV header.\n";
    assert_eq!(stderr, expected);
}

#[test]
fn transpose_long_format_all_columns_as_fields() {
    let wrk = Workdir::new("transpose_long_format_all_columns_as_fields");

    // Create a wide-format CSV
    let wide_format = vec![
        svec!["field", "type", "value"],
        svec!["name", "String", "Alice"],
        svec!["age", "Integer", "25"],
    ];

    wrk.create("in.csv", wide_format);

    let mut cmd = wrk.command("transpose");
    cmd.args(["--long", "1-3"]).arg("in.csv"); // Select all columns as fields

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Expected: only headers, no attribute columns to process
    let expected = vec![svec!["field", "attribute", "value"]];

    wrk.assert_success(&mut cmd);
    assert_eq!(got, expected);
}

#[test]
fn transpose_long_format_quoted_column_name() {
    let wrk = Workdir::new("transpose_long_format_quoted_column_name");

    // Create a wide-format CSV with column name containing spaces
    let wide_format = vec![
        svec!["id", "field name", "type", "value"],
        svec!["1", "name", "String", "Alice"],
        svec!["2", "age", "Integer", "25"],
    ];

    wrk.create("in.csv", wide_format);

    let mut cmd = wrk.command("transpose");
    cmd.args(["--long", r#""field name""#]).arg("in.csv"); // Select quoted column name with space

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Expected: "field name" column selected as field column
    let expected = vec![
        svec!["field", "attribute", "value"],
        svec!["name", "id", "1"],
        svec!["name", "type", "String"],
        svec!["name", "value", "Alice"],
        svec!["age", "id", "2"],
        svec!["age", "type", "Integer"],
        svec!["age", "value", "25"],
    ];

    wrk.assert_success(&mut cmd);
    assert_eq!(got, expected);
}

// --select tests

#[test]
fn transpose_select_by_index() {
    let wrk = Workdir::new("transpose_select_by_index");

    // Create CSV: a,b,c / 1,2,3 / 4,5,6
    let data = vec![
        svec!["a", "b", "c"],
        svec!["1", "2", "3"],
        svec!["4", "5", "6"],
    ];

    wrk.create("in.csv", data);

    let mut cmd = wrk.command("transpose");
    cmd.args(["--select", "1,3"]).arg("in.csv"); // Select columns 1 and 3 (a and c)

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Expected: only columns a and c transposed
    let expected = vec![svec!["a", "1", "4"], svec!["c", "3", "6"]];

    wrk.assert_success(&mut cmd);
    assert_eq!(got, expected);
}

#[test]
fn transpose_select_by_name() {
    let wrk = Workdir::new("transpose_select_by_name");

    // Create CSV: a,b,c / 1,2,3 / 4,5,6
    let data = vec![
        svec!["a", "b", "c"],
        svec!["1", "2", "3"],
        svec!["4", "5", "6"],
    ];

    wrk.create("in.csv", data);

    let mut cmd = wrk.command("transpose");
    cmd.args(["--select", "a,c"]).arg("in.csv"); // Select columns a and c by name

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Expected: only columns a and c transposed
    let expected = vec![svec!["a", "1", "4"], svec!["c", "3", "6"]];

    wrk.assert_success(&mut cmd);
    assert_eq!(got, expected);
}

#[test]
fn transpose_select_by_range() {
    let wrk = Workdir::new("transpose_select_by_range");

    // Create CSV: a,b,c,d / 1,2,3,4 / 5,6,7,8
    let data = vec![
        svec!["a", "b", "c", "d"],
        svec!["1", "2", "3", "4"],
        svec!["5", "6", "7", "8"],
    ];

    wrk.create("in.csv", data);

    let mut cmd = wrk.command("transpose");
    cmd.args(["--select", "2-3"]).arg("in.csv"); // Select columns 2-3 (b and c)

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Expected: only columns b and c transposed
    let expected = vec![svec!["b", "2", "6"], svec!["c", "3", "7"]];

    wrk.assert_success(&mut cmd);
    assert_eq!(got, expected);
}

#[test]
fn transpose_select_by_regex() {
    let wrk = Workdir::new("transpose_select_by_regex");

    // Create CSV: val_a,val_b,other / 1,2,3 / 4,5,6
    let data = vec![
        svec!["val_a", "val_b", "other"],
        svec!["1", "2", "3"],
        svec!["4", "5", "6"],
    ];

    wrk.create("in.csv", data);

    let mut cmd = wrk.command("transpose");
    cmd.args(["--select", "/^val/"]).arg("in.csv"); // Select columns starting with "val"

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Expected: only columns val_a and val_b transposed
    let expected = vec![svec!["val_a", "1", "4"], svec!["val_b", "2", "5"]];

    wrk.assert_success(&mut cmd);
    assert_eq!(got, expected);
}

#[test]
fn transpose_select_multipass() {
    let wrk = Workdir::new("transpose_select_multipass");

    // Create CSV: a,b,c / 1,2,3 / 4,5,6
    let data = vec![
        svec!["a", "b", "c"],
        svec!["1", "2", "3"],
        svec!["4", "5", "6"],
    ];

    wrk.create("in.csv", data);

    let mut cmd = wrk.command("transpose");
    cmd.args(["--multipass", "--select", "a,c"]).arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Expected: only columns a and c transposed
    let expected = vec![svec!["a", "1", "4"], svec!["c", "3", "6"]];

    wrk.assert_success(&mut cmd);
    assert_eq!(got, expected);
}

#[test]
fn transpose_select_long_format() {
    let wrk = Workdir::new("transpose_select_long_format");

    // Create a wide-format CSV
    let wide_format = vec![
        svec!["id", "name", "val1", "val2", "val3"],
        svec!["1", "foo", "10", "20", "30"],
        svec!["2", "bar", "40", "50", "60"],
    ];

    wrk.create("in.csv", wide_format);

    let mut cmd = wrk.command("transpose");
    // Select only val1 and val3 as attributes, use id as field column
    cmd.args(["--long", "id", "--select", "val1,val3"])
        .arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Expected: only val1 and val3 become attribute rows
    let expected = vec![
        svec!["field", "attribute", "value"],
        svec!["1", "val1", "10"],
        svec!["1", "val3", "30"],
        svec!["2", "val1", "40"],
        svec!["2", "val3", "60"],
    ];

    wrk.assert_success(&mut cmd);
    assert_eq!(got, expected);
}

#[test]
fn transpose_select_long_format_by_range() {
    let wrk = Workdir::new("transpose_select_long_format_by_range");

    // Create a wide-format CSV
    let wide_format = vec![
        svec!["id", "name", "val1", "val2", "val3"],
        svec!["1", "foo", "10", "20", "30"],
        svec!["2", "bar", "40", "50", "60"],
    ];

    wrk.create("in.csv", wide_format);

    let mut cmd = wrk.command("transpose");
    // Select columns 3-5 (val1, val2, val3) as attributes, use id as field column
    cmd.args(["--long", "id", "--select", "3-5"]).arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Expected: val1, val2, val3 become attribute rows (name excluded)
    let expected = vec![
        svec!["field", "attribute", "value"],
        svec!["1", "val1", "10"],
        svec!["1", "val2", "20"],
        svec!["1", "val3", "30"],
        svec!["2", "val1", "40"],
        svec!["2", "val2", "50"],
        svec!["2", "val3", "60"],
    ];

    wrk.assert_success(&mut cmd);
    assert_eq!(got, expected);
}

#[test]
fn transpose_select_invalid_column() {
    let wrk = Workdir::new("transpose_select_invalid_column");

    let data = vec![svec!["a", "b", "c"], svec!["1", "2", "3"]];

    wrk.create("in.csv", data);

    let mut cmd = wrk.command("transpose");
    cmd.args(["--select", "nonexistent"]).arg("in.csv");

    // Should fail with an error
    wrk.assert_err(&mut cmd);
}

#[test]
fn transpose_select_empty_result() {
    let wrk = Workdir::new("transpose_select_empty_result");

    let data = vec![svec!["a", "b", "c"], svec!["1", "2", "3"]];

    wrk.create("in.csv", data);

    let mut cmd = wrk.command("transpose");
    cmd.args(["--select", "/^nonexistent/"]).arg("in.csv"); // Regex matching nothing

    // Should fail with an error
    wrk.assert_err(&mut cmd);

    // Verify the error message
    let stderr: String = wrk.output_stderr(&mut cmd);
    assert!(
        stderr.contains("does not match any columns"),
        "Expected error about no columns selected, got: {}",
        stderr
    );
}

#[test]
fn transpose_select_single_column() {
    let wrk = Workdir::new("transpose_select_single_column");

    let data = vec![
        svec!["a", "b", "c"],
        svec!["1", "2", "3"],
        svec!["4", "5", "6"],
    ];

    wrk.create("in.csv", data);

    let mut cmd = wrk.command("transpose");
    cmd.args(["--select", "b"]).arg("in.csv"); // Select only column b

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Expected: only column b transposed
    let expected = vec![svec!["b", "2", "5"]];

    wrk.assert_success(&mut cmd);
    assert_eq!(got, expected);
}

#[test]
fn transpose_select_all_columns() {
    let wrk = Workdir::new("transpose_select_all_columns");

    let data = vec![
        svec!["a", "b", "c"],
        svec!["1", "2", "3"],
        svec!["4", "5", "6"],
    ];

    wrk.create("in.csv", data);

    let mut cmd = wrk.command("transpose");
    cmd.args(["--select", "1-"]).arg("in.csv"); // Select all columns (1 to end)

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // Expected: same as regular transpose
    let expected = vec![
        svec!["a", "1", "4"],
        svec!["b", "2", "5"],
        svec!["c", "3", "6"],
    ];

    wrk.assert_success(&mut cmd);
    assert_eq!(got, expected);
}
