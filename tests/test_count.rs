use crate::{CsvData, qcheck, workdir::Workdir};

#[test]
fn count_simple() {
    let wrk = Workdir::new("count_simple");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
            svec!["gamma", "37"],
        ],
    );
    let mut cmd = wrk.command("count");
    cmd.arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "3";
    assert_eq!(got, expected.to_string());
}

// issue #1417: `count` reads via `Config` directly (no `util::process_input`), so
// these exercise the `Config` special-format zip path (`select_zip_entry` /
// `extract_zip_to_temp`) — distinct from the `process_input` path used by slice/cat/etc.
// NOT polars-gated: zip extraction needs only the always-compiled `zip` crate, so it
// must work in non-polars builds (qsvlite) too.
#[test]
fn count_from_csvzip() {
    // single-entry zip: boston311-100.csv.zip contains just boston311-100.csv (100 rows)
    let wrk = Workdir::new("count_from_csvzip");
    let test_file = wrk.load_test_file("boston311-100.csv.zip");
    let mut cmd = wrk.command("count");
    cmd.arg(test_file);

    wrk.assert_success(&mut cmd);
    let got: String = wrk.stdout(&mut cmd);
    assert_eq!(got, "100".to_string());
}

#[test]
fn count_from_zip_multientry() {
    // testzip.zip has multiple entries (positions.csv, NYC311-5.ssv, buses.csv,
    // NYC311-5.tsv) plus __MACOSX system files. select_zip_entry skips the system
    // files and picks the FIRST CSV/TSV/TAB/SSV entry in archive order: positions.csv,
    // which has 6 data rows.
    let wrk = Workdir::new("count_from_zip_multientry");
    let test_file = wrk.load_test_file("testzip.zip");
    let mut cmd = wrk.command("count");
    cmd.arg(test_file);

    wrk.assert_success(&mut cmd);
    let got: String = wrk.stdout(&mut cmd);
    assert_eq!(got, "6".to_string());
}

// A non-zip file with a `.zip` extension must surface a clear conversion error
// rather than silently reading the raw bytes as CSV. Regression for the
// `skip_format_check` ordering fix (zip conversion failures are not swallowed).
#[test]
fn count_from_invalid_zip_errors() {
    let wrk = Workdir::new("count_from_invalid_zip_errors");
    wrk.create_from_string("bad.zip", "this is not a zip archive\n");
    let mut cmd = wrk.command("count");
    cmd.arg("bad.zip");
    wrk.assert_err(&mut cmd);
}

// Even with sniffing enabled (which sets skip_format_check up-front), a corrupt/
// non-zip `.zip` must still surface the conversion error rather than be read as CSV.
#[test]
fn count_invalid_zip_errors_even_when_sniffing() {
    let wrk = Workdir::new("count_invalid_zip_errors_even_when_sniffing");
    wrk.create_from_string("bad.zip", "this is not a zip archive\n");
    let mut cmd = wrk.command("count");
    cmd.env("QSV_SNIFF_DELIMITER", "1").arg("bad.zip");
    wrk.assert_err(&mut cmd);
}

#[test]
fn count_empty() {
    let wrk = Workdir::new("count_empty");
    wrk.create_from_string("empty.csv", "");
    let mut cmd = wrk.command("count");
    cmd.arg("empty.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "0";
    assert_eq!(got, expected.to_string());
}

#[test]
fn count_simple_tsv() {
    let wrk = Workdir::new("count_simple_tsv");
    wrk.create_with_delim(
        "in.tsv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
            svec!["gamma", "37"],
        ],
        b'\t',
    );

    let mut cmd = wrk.command("count");
    cmd.arg("in.tsv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "3";

    assert_eq!(got, expected.to_string());
}

#[test]
fn count_simple_ssv() {
    let wrk = Workdir::new("count_simple_ssv");
    wrk.create_with_delim(
        "in.ssv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
            svec!["gamma", "37"],
        ],
        b';',
    );

    let mut cmd = wrk.command("count");
    cmd.arg("in.ssv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "3";

    assert_eq!(got, expected.to_string());
}

#[test]
fn count_simple_custom_delimiter() {
    let wrk = Workdir::new("count_simple_custom_delimiter");
    wrk.create_with_delim(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
            svec!["gamma", "37"],
        ],
        b';',
    );

    let mut cmd = wrk.command("count");
    cmd.env("QSV_CUSTOM_DELIMITER", ";").arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "3";

    assert_eq!(got, expected.to_string());
}

#[test]
fn count_width() {
    let wrk = Workdir::new("count_width");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number", "flag"],
            svec!["alphabetic", "13", "true"],
            svec!["beta", "24", "false"],
            svec!["gamma", "37.1", "true"],
            svec!("delta", "42.5", "false"),
        ],
    );
    let mut cmd = wrk.command("count");
    cmd.arg("--width").arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "4;16-15-15-13-1.5-1.2247-1";
    assert_eq!(got, expected.to_string());
}

#[test]
fn count_width_json() {
    let wrk = Workdir::new("count_width");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number", "flag"],
            svec!["alphabetic", "13", "true"],
            svec!["beta", "24", "false"],
            svec!["gamma", "37.1", "true"],
            svec!("delta", "42.5", "false"),
        ],
    );
    let mut cmd = wrk.command("count");
    cmd.arg("--width").arg("--json").arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"count":4,"max":16,"avg":15,"median":15,"min":13,"variance":1.5,"stddev":1.2247,"mad":1}"#;
    assert_eq!(got, expected.to_string());
}

#[test]
fn count_width_no_delims() {
    let wrk = Workdir::new("count_width_no_delims");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number", "flag"],
            svec!["alphabetic", "13", "true"],
            svec!["beta", "24", "false"],
            svec!["gamma", "37.1", "true"],
            svec!("delta", "42.5", "false"),
        ],
    );
    let mut cmd = wrk.command("count");
    cmd.arg("--width-no-delims").arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "4;16-13.5-13-11-3.25-1.8028-1.5";
    assert_eq!(got, expected.to_string());
}

#[test]
fn count_width_no_delims_human_readable() {
    let wrk = Workdir::new("count_width_no_delims_human_readable");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number", "flag"],
            svec!["alphabetic", "13", "true"],
            svec!["beta", "24", "false"],
            svec!["gamma", "37.1", "true"],
            svec!("delta", "42.5", "false"),
        ],
    );
    let mut cmd = wrk.command("count");
    cmd.arg("--width-no-delims").arg("-H").arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "4;max:16 avg:13.5 median:13 min:11 variance:3.25 stddev:1.8028 mad:1.5";
    assert_eq!(got, expected.to_string());
}

#[test]
fn count_width_human_readable() {
    let wrk = Workdir::new("count_width_human_readable");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number", "flag"],
            svec!["alphabetic", "13", "true"],
            svec!["beta", "24", "false"],
            svec!["gamma", "37.1", "true"],
            svec!("delta", "42.5", "false"),
        ],
    );
    let mut cmd = wrk.command("count");
    cmd.arg("--width").arg("-H").arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "4;max:16 avg:15 median:15 min:13 variance:1.5 stddev:1.2247 mad:1";
    assert_eq!(got, expected.to_string());
}

#[test]
fn count_width_custom_delimiter() {
    let wrk = Workdir::new("count_width_custom_delimiter");
    wrk.create_with_delim(
        "in.csv",
        vec![
            svec!["letter", "number", "flag"],
            svec!["alphabetic", "13", "true"],
            svec!["beta", "24", "false"],
            svec!["gamma", "37.1", "true"],
            svec!("delta", "42.5", "false"),
        ],
        b';',
    );

    let mut cmd = wrk.command("count");
    cmd.env("QSV_CUSTOM_DELIMITER", ";")
        .arg("--width")
        .arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "4;18-15.5-15-13-3.25-1.8028-1.5";

    assert_eq!(got, expected.to_string());
}

#[test]
fn count_flexible() {
    let wrk = Workdir::new("count_flexible");
    wrk.create_from_string(
        "in.csv",
        r#"letter,number,flag
alphabetic,13,true,extra column
beta,24,false
gamma,37.1
delta,42.5,false
"#,
    );
    let mut cmd = wrk.command("count");
    cmd.arg("--flexible").arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "4";
    assert_eq!(got, expected.to_string());
}

#[test]
fn count_comments() {
    let wrk = Workdir::new("count_comments");

    wrk.create(
        "in.csv",
        vec![
            svec!["# this is a comment", ""],
            svec!["# next comment", ""],
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
            svec!["# comment here too!", "24"],
        ],
    );
    let mut cmd = wrk.command("count");
    cmd.arg("in.csv").env("QSV_COMMENT_CHAR", "#");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "2";
    assert_eq!(got, expected.to_string());
}

/// This tests whether `qsv count` gets the right answer.
///
/// It does some simple case analysis to handle whether we want to test counts
/// in the presence of headers and/or indexes.
fn prop_count_len(
    name: &str,
    rows: CsvData,
    headers: bool,
    idx: bool,
    noheaders_env: bool,
    human_readable: bool,
) -> bool {
    let mut expected_count = rows.len();
    if headers && expected_count > 0 {
        expected_count -= 1;
    }

    let wrk = Workdir::new(name);
    if idx {
        wrk.create_indexed("in.csv", rows);
    } else {
        wrk.create("in.csv", rows);
    }

    let mut cmd = wrk.command("count");
    if !headers {
        cmd.arg("--no-headers");
    }
    if noheaders_env {
        cmd.env("QSV_NO_HEADERS", "1");
    }
    if human_readable {
        cmd.arg("--human-readable");
    }
    cmd.arg("in.csv");

    if human_readable {
        use indicatif::HumanCount;

        let got_count: String = wrk.stdout(&mut cmd);
        let expected_count_commas = HumanCount(expected_count as u64).to_string();

        rassert_eq!(got_count, expected_count_commas)
    } else {
        let got_count: usize = wrk.stdout(&mut cmd);
        rassert_eq!(got_count, expected_count)
    }
}

#[cfg(not(feature = "polars"))]
#[test]
fn prop_count() {
    fn p(rows: CsvData) -> bool {
        prop_count_len("prop_count", rows, false, false, false, false)
    }
    qcheck(p as fn(CsvData) -> bool);
}

#[cfg(not(feature = "polars"))]
#[test]
fn prop_count_human_readable() {
    fn p(rows: CsvData) -> bool {
        prop_count_len("prop_count", rows, false, false, false, true)
    }
    qcheck(p as fn(CsvData) -> bool);
}

#[cfg(not(feature = "polars"))]
#[test]
fn prop_count_headers() {
    fn p(rows: CsvData) -> bool {
        prop_count_len("prop_count_headers", rows, true, false, false, false)
    }
    qcheck(p as fn(CsvData) -> bool);
}

#[cfg(not(feature = "polars"))]
#[test]
fn prop_count_headers_human_readable() {
    fn p(rows: CsvData) -> bool {
        prop_count_len("prop_count_headers", rows, true, false, false, true)
    }
    qcheck(p as fn(CsvData) -> bool);
}

#[test]
fn prop_count_indexed() {
    fn p(rows: CsvData) -> bool {
        prop_count_len("prop_count_indexed", rows, false, true, false, false)
    }
    qcheck(p as fn(CsvData) -> bool);
}

#[test]
fn prop_count_indexed_headers() {
    fn p(rows: CsvData) -> bool {
        prop_count_len("prop_count_indexed_headers", rows, true, true, false, false)
    }
    qcheck(p as fn(CsvData) -> bool);
}

#[cfg(not(feature = "polars"))]
#[test]
fn prop_count_noheaders_env() {
    fn p(rows: CsvData) -> bool {
        prop_count_len("prop_count_noheaders_env", rows, false, false, true, false)
    }
    qcheck(p as fn(CsvData) -> bool);
}

#[test]
fn prop_count_noheaders_indexed_env() {
    fn p(rows: CsvData) -> bool {
        prop_count_len(
            "prop_count_noheaders_indexed_env",
            rows,
            false,
            true,
            true,
            false,
        )
    }
    qcheck(p as fn(CsvData) -> bool);
}

#[test]
fn count_custom_delimiter() {
    let wrk = Workdir::new("count_custom_delimiter");
    wrk.create_with_delim(
        "in.csv",
        vec![
            svec!["letter", "number", "flag"],
            svec!["alphabetic", "13", "true"],
            svec!["beta", "24", "false"],
            svec!["gamma", "37.1", "true"],
            svec!("delta", "42.5", "false"),
        ],
        b';',
    );

    let mut cmd = wrk.command("count");
    cmd.arg("--delimiter").arg(";").arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "4";
    assert_eq!(got, expected.to_string());
}

#[test]
fn show_version() {
    let wrk = Workdir::new("show_version");
    let mut cmd = wrk.command("");
    cmd.arg("--version");

    let got: String = wrk.stdout(&mut cmd);
    let expected = format!(" {}", env!("CARGO_PKG_VERSION"));
    assert!(got.contains(&expected));
}

#[test]
fn count_stdin_schema_inference_issue_3103() {
    use std::io::Write;

    let wrk = Workdir::new("count_stdin_schema_inference_issue");

    // Create a CSV file that mimics the issue: a column that starts with boolean
    // values but then contains integers. This can cause Polars to misinfer the schema.
    let mut csv_data = String::from("value\n");
    // Add many "FALSE" or "TRUE" values first (to trigger boolean inference)
    for _ in 0..3_000 {
        csv_data.push_str(if rand::random::<bool>() {
            "true\n"
        } else {
            "false\n"
        });
    }
    // Then add integer values (which should cause schema mismatch)
    for i in 0..50 {
        csv_data.push_str(&format!("{}\n", i));
    }

    wrk.create_from_string("test_data.csv", &csv_data);

    // Test with stdin input (the problematic case from the issue)
    let mut cmd = wrk.command("count");
    cmd.arg("-"); // Use stdin

    let stdin_data = wrk.read_to_string("test_data.csv").unwrap();
    cmd.stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped());

    let mut child = cmd.spawn().unwrap();
    let mut stdin = child.stdin.take().unwrap();
    std::thread::spawn(move || {
        stdin.write_all(stdin_data.as_bytes()).unwrap();
    });

    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());

    let got: String = String::from_utf8_lossy(&output.stdout).trim().to_string();
    // Should count 3050 rows (3000 boolean values + 50 integers), excluding header
    let expected = "3050";
    assert_eq!(
        got, expected,
        "Count should be 3050, not 0, even with schema inference issues"
    );
}

#[test]
fn count_file_schema_inference_issue_3103() {
    let wrk = Workdir::new("count_file_schema_inference_issue");

    // Create a CSV file that mimics the issue: a column that starts with boolean
    // values but then contains integers. This can cause Polars to misinfer the schema.
    let mut csv_data = String::from("value\n");
    // Add many "FALSE" or "TRUE" values first (to trigger boolean inference)
    for _ in 0..3_000 {
        csv_data.push_str(if rand::random::<bool>() {
            "true\n"
        } else {
            "false\n"
        });
    }
    // Then add integer values (which should cause schema mismatch)
    for i in 0..50 {
        csv_data.push_str(&format!("{}\n", i));
    }

    wrk.create_from_string("test_data.csv", &csv_data);

    // Test with file input (should also work)
    let mut cmd = wrk.command("count");
    cmd.arg("test_data.csv");

    let got: String = wrk.stdout(&mut cmd);
    // Should count 3050 rows (3000 boolean values + 50 integers), excluding header
    let expected = "3050";
    assert_eq!(
        got, expected,
        "Count should be 3050, not 0, even with schema inference issues"
    );
}
