use crate::workdir::Workdir;

#[test]
fn safenames_conditional() {
    let wrk = Workdir::new("safenames");
    wrk.create(
        "in.csv",
        vec![
            svec![
                "col1",
                " This is a column with invalid chars!# and leading & trailing spaces ",
                "",
                "this is already a Postgres Safe Column",
                "1starts with 1",
                "col1",
                "col1",
                "",
                "",
            ],
            svec!["1", "b", "33", "1", "b", "33", "34", "z", "42"],
            svec!["2", "c", "34", "3", "d", "31", "3", "y", "3.14"],
        ],
    );

    let mut cmd = wrk.command("safenames");
    cmd.arg("--mode").arg("c").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![
            "col1",
            "this_is_a_column_with_invalid_chars___and_leading___trailing",
            // null column names are not allowed in postgres
            "unsafe_",
            // though this is "safe", it's generally discouraged
            // to have embedded spaces and mixed case column names
            // as you will have to use quotes to refer to these columns
            // in Postgres
            "this is already a Postgres Safe Column",
            // a column cannot start with a digit
            "unsafe_1starts_with_1",
            // duplicate cols are not allowed in one table in postgres
            "col1_2",
            "col1_3",
            "unsafe__2",
            "unsafe__3"
        ],
        svec!["1", "b", "33", "1", "b", "33", "34", "z", "42"],
        svec!["2", "c", "34", "3", "d", "31", "3", "y", "3.14"],
    ];
    assert_eq!(got, expected);

    let changed_headers = wrk.output_stderr(&mut cmd);
    let expected_count = "7\n";
    assert_eq!(changed_headers, expected_count);
}

#[test]
fn safenames_always() {
    let wrk = Workdir::new("safenames");
    wrk.create(
        "in.csv",
        vec![
            svec![
                "col1",
                // not valid in postgres
                " This is a column with invalid chars!# and leading & trailing spaces ",
                "",
                // postgres allows for embedded spaces
                "this is already a Postgres Safe Column",
                "1starts with 1",
                "col1",
                "col1"
            ],
            svec!["1", "b", "33", "1", "b", "33", "34"],
            svec!["2", "c", "34", "3", "d", "31", "3"],
        ],
    );

    let mut cmd = wrk.command("safenames");
    cmd.arg("--mode").arg("always").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![
            "col1",
            "this_is_a_column_with_invalid_chars___and_leading___trailing",
            "unsafe_",
            // we were using Always mode, so even though the
            // original header name was already valid,
            // we replaced spaces with _ regardless
            "this_is_already_a_postgres_safe_column",
            "unsafe_1starts_with_1",
            "col1_2",
            "col1_3"
        ],
        svec!["1", "b", "33", "1", "b", "33", "34"],
        svec!["2", "c", "34", "3", "d", "31", "3"],
    ];
    assert_eq!(got, expected);

    let changed_headers = wrk.output_stderr(&mut cmd);
    let expected_count = "6\n";
    assert_eq!(changed_headers, expected_count);
}

#[test]
fn safenames_verify() {
    let wrk = Workdir::new("safenames");
    wrk.create(
        "in.csv",
        vec![
            svec![
                "col1",
                " This is a column with invalid chars!# and leading & trailing spaces ",
                "",
                "this is already a Postgres Safe Column",
                "1starts with 1",
                "col1",
                "col1",
                "",
                "",
            ],
            svec!["1", "b", "33", "1", "b", "33", "34", "z", "42"],
            svec!["2", "c", "34", "3", "d", "31", "3", "y", "3.14"],
        ],
    );

    let mut cmd = wrk.command("safenames");
    cmd.arg("--mode").arg("verify").arg("in.csv");

    let changed_headers = wrk.output_stderr(&mut cmd);
    // 8 unsafe = 4 distinct invalid headers + 2 duplicate `col1` slots
    // (renamed col1_2, col1_3) + 2 duplicate empty slots (renamed unsafe__2,
    // unsafe__3). Matches always-mode's changed_count for the same input.
    let expected_count = "8\n";
    assert_eq!(changed_headers, expected_count);

    wrk.assert_success(&mut cmd);
}

#[test]
fn safenames_verify_verbose() {
    let wrk = Workdir::new("safenames");
    wrk.create(
        "in.csv",
        vec![
            svec![
                "col1",
                " This is a column with invalid chars!# and leading & trailing spaces ",
                "",
                "this is already a Postgres Safe Column",
                "1starts with 1",
                "col1",
                "col1",
                "col1",
                "",
                "",
                "",
                "col1",
                "_1",
            ],
            svec![
                "1", "b", "33", "1", "b", "33", "34", "z", "42", "3", "2", "1", "0"
            ],
        ],
    );

    let mut cmd = wrk.command("safenames");
    cmd.arg("--mode").arg("V").arg("in.csv");

    let got_stderr = wrk.output_stderr(&mut cmd);

    let expected_stderr = r#"13 header/s
2 duplicate/s: ":4, col1:5"
12 unsafe header/s: ["This is a column with invalid chars!# and leading & trailing spaces", "", "this is already a Postgres Safe Column", "1starts with 1", "col1", "col1", "col1", "", "", "", "col1", "_1"]
1 safe header/s: ["col1"]
"#;

    assert_eq!(got_stderr, expected_stderr);

    wrk.assert_success(&mut cmd);
}

#[test]
fn safenames_verify_verbose_pretty_json() {
    let wrk = Workdir::new("safenames");
    wrk.create(
        "in.csv",
        vec![
            svec![
                "col1",
                " This is a column with invalid chars!# and leading & trailing spaces ",
                "",
                "this is already a Postgres Safe Column",
                "1starts with 1",
                "col1",
                "col1",
                "col1",
                "",
                "",
                "",
                "col1",
                "_1",
            ],
            svec![
                "1", "b", "33", "1", "b", "33", "34", "z", "42", "3", "2", "1", "0"
            ],
        ],
    );

    let mut cmd = wrk.command("safenames");
    cmd.arg("--mode").arg("J").arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);

    let expected = r#"{
  "header_count": 13,"duplicate_count": 2,"duplicate_headers": [
    ":4",
    "col1:5"
  ],"unsafe_headers": [
    "This is a column with invalid chars!# and leading & trailing spaces",
    "",
    "this is already a Postgres Safe Column",
    "1starts with 1",
    "col1",
    "col1",
    "col1",
    "",
    "",
    "",
    "col1",
    "_1"
  ],"safe_headers": [
    "col1"
  ]
}"#;
    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd);
}

#[test]
fn safenames_verify_verbose_json() {
    let wrk = Workdir::new("safenames");
    wrk.create(
        "in.csv",
        vec![
            svec![
                "col1",
                " This is a column with invalid chars!# and leading & trailing spaces ",
                "",
                "this is already a Postgres Safe Column",
                "1starts with 1",
                "col1",
                "col1",
                "col1",
                "",
                "",
                "",
                "col1",
                "_1",
            ],
            svec![
                "1", "b", "33", "1", "b", "33", "34", "z", "42", "3", "2", "1", "0"
            ],
        ],
    );

    let mut cmd = wrk.command("safenames");
    cmd.arg("--mode").arg("j").arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);

    let expected = r#"{"header_count":13,"duplicate_count":2,"duplicate_headers":[":4","col1:5"],"unsafe_headers":["This is a column with invalid chars!# and leading & trailing spaces","","this is already a Postgres Safe Column","1starts with 1","col1","col1","col1","","","","col1","_1"],"safe_headers":["col1"]}"#;

    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd);
}

#[test]
fn safenames_invalid_mode() {
    let wrk = Workdir::new("safenames");
    wrk.create(
        "in.csv",
        vec![
            svec![
                "col1",
                " This is a column with invalid chars!# and leading & trailing spaces ",
                "",
                "this is already a postgres safe column",
                "1starts with 1",
                "col1",
                "col1"
            ],
            svec!["1", "b", "33", "1", "b", "33", "34"],
            svec!["2", "c", "34", "3", "d", "31", "3"],
        ],
    );

    let mut cmd = wrk.command("safenames");
    cmd.arg("--mode").arg("invalidmode").arg("in.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
fn safenames_reserved_names_default() {
    let wrk = Workdir::new("safenames");
    wrk.create(
        "in.csv",
        vec![
            svec![
                "col1",
                " This is a column with invalid chars!# and leading & trailing spaces ",
                "_id",
                "this is already a postgres safe column",
                "1starts with 1",
                "col1",
                "col1"
            ],
            svec!["1", "b", "33", "1", "b", "33", "34"],
            svec!["2", "c", "34", "3", "d", "31", "3"],
        ],
    );

    let mut cmd = wrk.command("safenames");
    cmd.arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![
            "col1",
            "this_is_a_column_with_invalid_chars___and_leading___trailing",
            "reserved__id",
            "this_is_already_a_postgres_safe_column",
            "unsafe_1starts_with_1",
            "col1_2",
            "col1_3"
        ],
        svec!["1", "b", "33", "1", "b", "33", "34"],
        svec!["2", "c", "34", "3", "d", "31", "3"],
    ];
    assert_eq!(got, expected);

    let changed_headers = wrk.output_stderr(&mut cmd);
    let expected_count = "6\n";
    assert_eq!(changed_headers, expected_count);
}

#[test]
fn safenames_mode_s_ascii_collapse() {
    let wrk = Workdir::new("safenames");
    wrk.create(
        "in.csv",
        vec![
            svec![
                "c1",
                "12_col",
                "Col with Embedded Spaces",
                "",
                "Column!@Invalid+Chars",
                "c1"
            ],
            svec!["1", "a2", "a3", "a4", "a5", "a6"],
        ],
    );

    let mut cmd = wrk.command("safenames");
    cmd.arg("--mode").arg("s").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![
            "c1",
            "unsafe_12_col",
            "col_with_embedded_spaces",
            "unsafe_",
            // runs of non-alphanumerics collapse to a single _
            // ("column_invalid_chars", not "column__invalid_chars")
            "column_invalid_chars",
            "c1_2"
        ],
        svec!["1", "a2", "a3", "a4", "a5", "a6"],
    ];
    assert_eq!(got, expected);

    let changed_headers = wrk.output_stderr(&mut cmd);
    assert_eq!(changed_headers, "5\n");
}

#[test]
fn safenames_collapse_flag_equivalence() {
    // --mode s is shorthand for --mode a --collapse; both must produce
    // identical output.
    let wrk = Workdir::new("safenames");
    wrk.create(
        "in.csv",
        vec![svec!["Column!@Invalid+Chars", "a__b"], svec!["1", "2"]],
    );

    let mut cmd_s = wrk.command("safenames");
    cmd_s.arg("--mode").arg("s").arg("in.csv");
    let got_s: Vec<Vec<String>> = wrk.read_stdout(&mut cmd_s);

    let mut cmd_flag = wrk.command("safenames");
    cmd_flag
        .arg("--mode")
        .arg("a")
        .arg("--collapse")
        .arg("in.csv");
    let got_flag: Vec<Vec<String>> = wrk.read_stdout(&mut cmd_flag);

    assert_eq!(got_s, got_flag);
    assert_eq!(
        got_s,
        vec![svec!["column_invalid_chars", "a_b"], svec!["1", "2"],]
    );
}

#[test]
fn safenames_mode_s_lowercase() {
    let wrk = Workdir::new("safenames");
    wrk.create("in.csv", vec![svec!["MixedCASE"], svec!["1"]]);

    let mut cmd = wrk.command("safenames");
    cmd.arg("--mode").arg("s").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got, vec![svec!["mixedcase"], svec!["1"]]);
}

#[test]
fn safenames_mode_S_unicode_preserve() {
    let wrk = Workdir::new("safenames");
    wrk.create(
        "in.csv",
        vec![svec!["Café #Ñ5", "naïve Column"], svec!["1", "2"]],
    );

    let mut cmd = wrk.command("safenames");
    cmd.arg("--mode").arg("S").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // unicode letters & numbers preserved, lowercased, separators collapsed
    assert_eq!(got, vec![svec!["café_ñ5", "naïve_column"], svec!["1", "2"]]);

    let changed_headers = wrk.output_stderr(&mut cmd);
    assert_eq!(changed_headers, "2\n");
}

#[test]
fn safenames_mode_s_vs_S_unicode_difference() {
    // ASCII mode s strips the unicode chars; mode S preserves them.
    let wrk = Workdir::new("safenames");
    wrk.create("in.csv", vec![svec!["Café #Ñ5"], svec!["1"]]);

    let mut cmd_s = wrk.command("safenames");
    cmd_s.arg("--mode").arg("s").arg("in.csv");
    let got_s: Vec<Vec<String>> = wrk.read_stdout(&mut cmd_s);
    assert_eq!(got_s, vec![svec!["caf_5"], svec!["1"]]);
}

#[test]
fn safenames_collapse_prefix_interaction() {
    // A leading run of non-alphanumerics collapses to a single _, which then
    // triggers the unsafe_ prefix prepend (documents the prefix-join _ adjacency).
    let wrk = Workdir::new("safenames");
    wrk.create("in.csv", vec![svec!["!!weird"], svec!["1"]]);

    let mut cmd = wrk.command("safenames");
    cmd.arg("--mode").arg("s").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got, vec![svec!["unsafe__weird"], svec!["1"]]);
}

#[test]
fn safenames_collapse_leading_digit() {
    // collapse does not interfere with the leading-digit unsafe_ prefix.
    let wrk = Workdir::new("safenames");
    wrk.create("in.csv", vec![svec!["12col", "5 apples"], svec!["1", "2"]]);

    let mut cmd = wrk.command("safenames");
    cmd.arg("--mode").arg("s").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(
        got,
        vec![svec!["unsafe_12col", "unsafe_5_apples"], svec!["1", "2"]]
    );
}

#[test]
fn safenames_collapse_duplicate_suffix() {
    // Two distinct headers that collapse to the same name still get
    // disambiguated with a sequence suffix.
    let wrk = Workdir::new("safenames");
    wrk.create("in.csv", vec![svec!["a!b", "a@b"], svec!["1", "2"]]);

    let mut cmd = wrk.command("safenames");
    cmd.arg("--mode").arg("s").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got, vec![svec!["a_b", "a_b_2"], svec!["1", "2"]]);
}

#[test]
fn safenames_json_mode_collapse() {
    // The collapse & unicode flags compose with the JSON report mode, so the
    // safe/unsafe classification reflects the "safer" rewrite. Without
    // --collapse, "a__b" would be classified safe; with it, it's unsafe.
    let wrk = Workdir::new("safenames");
    wrk.create(
        "in.csv",
        vec![svec!["id", "a__b", "Café"], svec!["1", "2", "3"]],
    );

    let mut cmd = wrk.command("safenames");
    cmd.arg("--mode")
        .arg("j")
        .arg("--collapse")
        .arg("--unicode")
        .arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"header_count":3,"duplicate_count":0,"duplicate_headers":[],"unsafe_headers":["a__b","Café"],"safe_headers":["id"]}"#;
    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd);
}

#[test]
fn safenames_60byte_truncation_collapse() {
    // A long separator-heavy header must still be truncated to <= 60 bytes
    // after collapsing.
    let wrk = Workdir::new("safenames");
    let long_header = "z!".repeat(40); // 80 chars -> collapses to "z_" x40 = 80 chars
    wrk.create("in.csv", vec![vec![long_header], vec!["1".to_string()]]);

    let mut cmd = wrk.command("safenames");
    cmd.arg("--mode").arg("s").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let header = &got[0][0];
    assert_eq!(header.len(), 60);
    assert!(header.starts_with("z_z_"));
}

#[test]
fn safenames_reserved_names_specified() {
    let wrk = Workdir::new("safenames");
    wrk.create(
        "in.csv",
        vec![
            svec![
                "col1",
                " This is a column with invalid chars!# and leading & trailing spaces ",
                "waldo",
                "this is already a postgres safe column",
                "1starts with 1",
                "col1",
                "col1"
            ],
            svec!["1", "b", "33", "1", "b", "33", "34"],
            svec!["2", "c", "34", "3", "d", "31", "3"],
        ],
    );

    let mut cmd = wrk.command("safenames");
    cmd.arg("--reserved").arg("waldo").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![
            "col1",
            "this_is_a_column_with_invalid_chars___and_leading___trailing",
            "reserved_waldo",
            "this_is_already_a_postgres_safe_column",
            "unsafe_1starts_with_1",
            "col1_2",
            "col1_3"
        ],
        svec!["1", "b", "33", "1", "b", "33", "34"],
        svec!["2", "c", "34", "3", "d", "31", "3"],
    ];
    assert_eq!(got, expected);

    let changed_headers = wrk.output_stderr(&mut cmd);
    let expected_count = "6\n";
    assert_eq!(changed_headers, expected_count);
}

#[test]
fn safenames_reserved_names_specified_case_insensitive() {
    let wrk = Workdir::new("safenames");
    wrk.create(
        "in.csv",
        vec![
            svec![
                "col1",
                " This is a column with invalid chars!# and leading & trailing spaces ",
                "WaLdO",
                "this is already a postgres safe column",
                "1starts with 1",
                "col1",
                "col1"
            ],
            svec!["1", "b", "33", "1", "b", "33", "34"],
            svec!["2", "c", "34", "3", "d", "31", "3"],
        ],
    );

    let mut cmd = wrk.command("safenames");
    cmd.arg("--reserved").arg("waldo").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![
            "col1",
            "this_is_a_column_with_invalid_chars___and_leading___trailing",
            "reserved_waldo",
            "this_is_already_a_postgres_safe_column",
            "unsafe_1starts_with_1",
            "col1_2",
            "col1_3"
        ],
        svec!["1", "b", "33", "1", "b", "33", "34"],
        svec!["2", "c", "34", "3", "d", "31", "3"],
    ];
    assert_eq!(got, expected);

    let changed_headers = wrk.output_stderr(&mut cmd);
    let expected_count = "6\n";
    assert_eq!(changed_headers, expected_count);
}
