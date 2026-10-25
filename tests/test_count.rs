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

    let got: String = wrk.stdout_on_success(&mut cmd);
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

    let got: String = wrk.stdout_on_success(&mut cmd);
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
    // this used to set a nonexistent QSV_CUSTOM_DELIMITER env var, so the file was read
    // with a comma. (QSV_DEFAULT_DELIMITER would not help either: a .csv extension maps to
    // a comma before it is consulted.) Pass the delimiter explicitly.
    cmd.arg("--delimiter").arg(";").arg("in.csv");

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
    let expected = "4;18-15.5-15-13-3.25-1.8028-1.5";
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
    let expected = r#"{"count":4,"max":18,"avg":15.5,"median":15,"min":13,"variance":3.25,"stddev":1.8028,"mad":1.5}"#;
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
    let expected = "4;max:18 avg:15.5 median:15 min:13 variance:3.25 stddev:1.8028 mad:1.5";
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
    // this used to set a nonexistent QSV_CUSTOM_DELIMITER env var, so each line was read as
    // ONE field with no delimiters - which made its expected widths right by accident, and
    // hid the first-record width bug. Pass the delimiter explicitly.
    cmd.arg("--delimiter").arg(";").arg("--width").arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "4;18-15.5-15-13-3.25-1.8028-1.5";

    assert_eq!(got, expected.to_string());
}

// quoted fields containing the delimiter or a newline used to make --width and
// --width-no-delims fail ("found record with 3 fields, but the previous record has 2 fields"),
// as quoting was turned off for them. Widths exclude the quote characters.
fn quoted_width_rows() -> Vec<Vec<String>> {
    vec![
        svec!["letter", "text"],
        svec!["beta", "plain"],
        svec!["alpha", "x, y"],
        svec!["gamma", "line1\nline2"],
    ]
}

#[test]
fn count_width_quoted_delimiter_and_newline() {
    let wrk = Workdir::new("count_width_quoted_delimiter_and_newline");
    wrk.create("in.csv", quoted_width_rows());
    let mut cmd = wrk.command("count");
    cmd.arg("--width").arg("--json").arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"count":3,"max":17,"avg":12.3333,"median":10,"min":10,"variance":10.8889,"stddev":3.2998,"mad":0}"#;
    assert_eq!(got, expected.to_string());
}

#[test]
fn count_width_no_delims_quoted_delimiter_and_newline() {
    let wrk = Workdir::new("count_width_no_delims_quoted_delimiter_and_newline");
    wrk.create("in.csv", quoted_width_rows());
    let mut cmd = wrk.command("count");
    cmd.arg("--width-no-delims").arg("--json").arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"count":3,"max":16,"avg":11.3333,"median":9,"min":9,"variance":10.8889,"stddev":3.2998,"mad":0}"#;
    assert_eq!(got, expected.to_string());
}

#[test]
fn count_width_quoted_delimiter_and_newline_indexed() {
    let wrk = Workdir::new("count_width_quoted_delimiter_and_newline_indexed");
    wrk.create_indexed("in.csv", quoted_width_rows());
    let mut cmd = wrk.command("count");
    cmd.arg("--width").arg("--json").arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"count":3,"max":17,"avg":12.3333,"median":10,"min":10,"variance":10.8889,"stddev":3.2998,"mad":0}"#;
    assert_eq!(got, expected.to_string());
}

// each record's delimiters are counted from its own fields: the widths are 9 ("first,2,3"),
// 3 ("4,5") and 1 ("6"). They used to be 7, 4 and 3 - the first record got no delimiters, and
// every later one got the first record's count of 2.
#[test]
fn count_width_flexible_ragged() {
    let wrk = Workdir::new("count_width_flexible_ragged");
    wrk.create_from_string("in.csv", "a,b,c\nfirst,2,3\n4,5\n6\n");
    let mut cmd = wrk.command("count");
    cmd.arg("--width")
        .arg("--flexible")
        .arg("--json")
        .arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"count":3,"max":9,"avg":4.3333,"median":3,"min":1,"variance":11.5556,"stddev":3.3993,"mad":2}"#;
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

// issue #4461: `util::log_end` deletes `TEMP_FILE_DIR`, but that cleanup was gated on the
// `polars` feature while the directory is populated in EVERY build - `extract_zip_to_temp` is
// always compiled (it needs only the non-optional `zip` crate). So a non-polars build such as
// qsvlite left a full decompressed copy of every `.zip` input on disk, once per invocation,
// until the OS reclaimed the temp dir.
//
// The child's temp root is redirected into the Workdir so the assertion is hermetic and does
// not depend on the state of the real system temp dir. TMPDIR covers Unix; TMP/TEMP cover
// Windows (`std::env::temp_dir` reads different vars per platform).
//
// Under `-F all_features` this passes either way, since the polars-gated cleanup was already
// running. It has teeth under `-F lite`, which CI runs (rust-qsvlite, rust-musl,
// rust-windows-lite).
#[test]
fn count_from_zip_leaves_no_temp_behind() {
    let wrk = Workdir::new("count_from_zip_leaves_no_temp_behind");
    let test_file = wrk.load_test_file("boston311-100.csv.zip");

    wrk.create_subdir("tmproot").unwrap();
    let tmproot = wrk.path("tmproot");

    let mut cmd = wrk.command("count");
    cmd.env("TMPDIR", &tmproot)
        .env("TMP", &tmproot)
        .env("TEMP", &tmproot)
        .arg(&test_file);

    // the command must still work - the temp is removed only AFTER the run finishes
    let got: String = wrk.stdout_on_success(&mut cmd);
    assert_eq!(got, "100".to_string());

    let leftovers: Vec<String> = std::fs::read_dir(&tmproot)
        .unwrap()
        .filter_map(|e| e.ok().map(|e| e.file_name().to_string_lossy().into_owned()))
        .collect();
    assert!(
        leftovers.is_empty(),
        "the converted temp was not cleaned up; {} left in the temp root: {leftovers:?}",
        leftovers.len()
    );
}

// issue #4456: polars materializes literal blank lines as all-null rows, while
// qsv's csv reader (and the index) skip them — so `count` on a file with a
// blank line disagreed with itself depending on whether an index existed.
// These exercise the blank-line guard in `polars_count_input`, which falls
// back to the regular csv reader whenever a blank line may be present.
#[test]
fn count_blank_line_trailing_unindexed_matches_indexed() {
    let wrk = Workdir::new("count_blank_line_trailing_unindexed_matches_indexed");
    wrk.create_from_string("in.csv", "a,b\n1,2\n3,4\n\n");

    let mut cmd = wrk.command("count");
    cmd.arg("in.csv");
    let unindexed: String = wrk.stdout(&mut cmd);
    assert_eq!(unindexed, "2");

    let mut cmd = wrk.command("index");
    cmd.arg("in.csv");
    wrk.assert_success(&mut cmd);

    let mut cmd = wrk.command("count");
    cmd.arg("in.csv");
    let indexed: String = wrk.stdout(&mut cmd);
    assert_eq!(indexed, "2");
}

#[test]
fn count_blank_line_middle() {
    let wrk = Workdir::new("count_blank_line_middle");
    wrk.create_from_string("in.csv", "a,b\n1,2\n\n3,4\n");

    let mut cmd = wrk.command("count");
    cmd.arg("in.csv");
    let got: String = wrk.stdout(&mut cmd);
    assert_eq!(got, "2");
}

#[test]
fn count_multiple_trailing_blank_lines() {
    let wrk = Workdir::new("count_multiple_trailing_blank_lines");
    wrk.create_from_string("in.csv", "a,b\n1,2\n3,4\n\n\n\n");

    let mut cmd = wrk.command("count");
    cmd.arg("in.csv");
    let got: String = wrk.stdout(&mut cmd);
    assert_eq!(got, "2");
}

#[test]
fn count_crlf_trailing_blank_line() {
    let wrk = Workdir::new("count_crlf_trailing_blank_line");
    wrk.create_from_string("in.csv", "a,b\r\n1,2\r\n3,4\r\n\r\n");

    let mut cmd = wrk.command("count");
    cmd.arg("in.csv");
    let got: String = wrk.stdout(&mut cmd);
    assert_eq!(got, "2");
}

// A "\n\n" inside a quoted multi-line field is a false positive for the
// blank-line guard: it triggers the csv-reader fallback, which must still
// return the correct count.
#[test]
fn count_quoted_double_newline_field() {
    let wrk = Workdir::new("count_quoted_double_newline_field");
    wrk.create_from_string("in.csv", "a,b\n\"x\n\ny\",2\n3,4\n");

    let mut cmd = wrk.command("count");
    cmd.arg("in.csv");
    let got: String = wrk.stdout(&mut cmd);
    assert_eq!(got, "2");
}

// Regression coverage for the polars py-1.44.0 count fixes (commit 359cf3c8):
// COUNT(*)'s dtype changed from u32 to i64, which made the old `u32()` read fail on
// every count — so every unindexed `count --no-headers` fell back to the regular
// reader (which already accounts for --no-headers) and then applied the polars-only
// first-row +1 adjustment on top, returning N+1. The unindexed no-headers property
// tests above are cfg'd out under the polars feature, so pin the behavior here.
#[cfg(feature = "polars")]
#[test]
fn count_no_headers_polars_path_exact() {
    let wrk = Workdir::new("count_no_headers_polars_path_exact");
    wrk.create_from_string("in.csv", "1,2\n3,4\n5,6\n");

    let mut cmd = wrk.command("count");
    cmd.args(["--no-headers", "in.csv"]);
    let got: String = wrk.stdout(&mut cmd);
    assert_eq!(got, "3");
}

// The same invariant through `polars_count_input`'s regular-reader fallback: a
// trailing blank line deterministically routes through the blank-line-guard
// fallback, whose count must be returned as-is — never with the polars-only
// --no-headers +1 adjustment applied on top.
#[cfg(feature = "polars")]
#[test]
fn count_no_headers_fallback_is_not_double_adjusted() {
    let wrk = Workdir::new("count_no_headers_fallback_is_not_double_adjusted");
    wrk.create_from_string("in.csv", "1,2\n3,4\n5,6\n\n");

    let mut cmd = wrk.command("count");
    cmd.args(["--no-headers", "in.csv"]);
    let got: String = wrk.stdout(&mut cmd);
    assert_eq!(got, "3");
}

// --delimiter used to carry a docopt `[default: ,]`, so count always got an explicit
// comma that overrode the .tsv/.ssv extension delimiters and QSV_DEFAULT_DELIMITER.
// --width-no-delims is the clearest check: read with the wrong delimiter, each line is
// one field and its separators are counted as field bytes.
#[test]
fn count_width_no_delims_tsv_extension() {
    let wrk = Workdir::new("count_width_no_delims_tsv_extension");
    wrk.create_from_string("in.tsv", "a\tbb\nccc\td\nee\tf\n");
    let mut cmd = wrk.command("count");
    cmd.args(["--width-no-delims", "in.tsv"]);
    let got: String = wrk.stdout(&mut cmd);
    assert_eq!(got, "2;4-3.5-3-3-0.25-0.5-0.5");
}

#[test]
fn count_width_no_delims_ssv_extension() {
    let wrk = Workdir::new("count_width_no_delims_ssv_extension");
    wrk.create_from_string("in.ssv", "a;bb\nccc;d\nee;f\n");
    let mut cmd = wrk.command("count");
    cmd.args(["--width-no-delims", "in.ssv"]);
    let got: String = wrk.stdout(&mut cmd);
    assert_eq!(got, "2;4-3.5-3-3-0.25-0.5-0.5");
}

#[test]
fn count_width_no_delims_explicit_delimiter_overrides_extension() {
    let wrk = Workdir::new("count_width_no_delims_explicit_delimiter_overrides_extension");
    wrk.create_from_string("in.ssv", "a;bb\nccc;d\nee;f\n");
    let mut cmd = wrk.command("count");
    // an explicit comma still wins over the .ssv extension: each line is one field
    cmd.args(["--width-no-delims", "--delimiter", ",", "in.ssv"]);
    let got: String = wrk.stdout(&mut cmd);
    assert_eq!(got, "2;5-4.5-4-4-0.25-0.5-0.5");
}

#[test]
fn count_width_no_delims_default_delimiter_env() {
    use std::{io::Write, process};

    let wrk = Workdir::new("count_width_no_delims_default_delimiter_env");
    let mut cmd = wrk.command("count");
    cmd.env("QSV_DEFAULT_DELIMITER", "|")
        .args(["--width-no-delims", "-"])
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped());
    let mut child = cmd.spawn().unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"a|bb\nccc|d\nee|f\n")
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "{output:?}");
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        "2;4-3.5-3-3-0.25-0.5-0.5"
    );
}

#[cfg(feature = "polars")]
#[test]
fn count_low_memory_flag_removed() {
    // --low-memory had no effect on Polars' streaming engine, so it was removed.
    let wrk = Workdir::new("count_low_memory_flag_removed");
    wrk.create("t.csv", vec![svec!["a"], svec!["1"]]);

    let mut cmd = wrk.command("count");
    cmd.arg("t.csv").arg("--low-memory");
    wrk.assert_err(&mut cmd);
}

// a file whose header sits below two preamble lines, as QSV_SNIFF_PREAMBLE detects them
fn preamble_csv() -> String {
    let mut csv = String::from("Report generated 2026-10-24\nSource: city clerk\nid,name,amount\n");
    for i in 1..=40 {
        csv.push_str(&format!("{i},n{i},{}\n", i * 10));
    }
    csv
}

#[test]
fn count_sniff_preamble_skips_preamble_rows() {
    let wrk = Workdir::new("count_sniff_preamble_skips_preamble_rows");
    wrk.create_from_string("p.csv", &preamble_csv());
    let mut cmd = wrk.command("count");
    cmd.env("QSV_SNIFF_PREAMBLE", "1").arg("p.csv");
    let got: String = wrk.stdout(&mut cmd);
    assert_eq!(got, "40");
}

#[test]
fn count_sniff_preamble_ignores_an_index() {
    // an index's offsets describe the raw file, so with skipped preamble rows it isn't used:
    // a (fresh) index planted beside the file, of a 1000-row one, must not be trusted
    let wrk = Workdir::new("count_sniff_preamble_ignores_an_index");
    let mut big = String::from("id\n");
    for i in 0..1000 {
        big.push_str(&format!("{i}\n"));
    }
    wrk.create_from_string("big.csv", &big);
    let mut cmd = wrk.command("index");
    cmd.arg("big.csv");
    wrk.assert_success(&mut cmd);
    wrk.create_from_string("p.csv", &preamble_csv());
    std::fs::rename(wrk.path("big.csv.idx"), wrk.path("p.csv.idx")).unwrap();
    let later = filetime::FileTime::from_unix_time(
        filetime::FileTime::from_last_modification_time(
            &std::fs::metadata(wrk.path("p.csv")).unwrap(),
        )
        .unix_seconds()
            + 4,
        0,
    );
    filetime::set_file_mtime(wrk.path("p.csv.idx"), later).unwrap();

    let mut cmd = wrk.command("count");
    cmd.env("QSV_SNIFF_PREAMBLE", "1").arg("p.csv");
    let got: String = wrk.stdout(&mut cmd);
    assert_eq!(got, "40");
}

#[test]
fn index_under_sniff_preamble_does_not_index_the_skipped_view() {
    // an index must describe the file as stored, whatever the env var, as runs without it
    // trust it. A detected preamble makes the stored file ragged, so it can't be indexed.
    let wrk = Workdir::new("index_under_sniff_preamble_does_not_index_the_skipped_view");
    wrk.create_from_string("p.csv", &preamble_csv());
    let mut cmd = wrk.command("index");
    cmd.env("QSV_SNIFF_PREAMBLE", "1").arg("p.csv");
    wrk.assert_err(&mut cmd);
    assert!(!wrk.path("p.csv.idx").exists());
}

#[test]
fn count_index_rewritten_within_the_same_second() {
    // index staleness compared whole-second mtimes, so data rewritten in the same second the
    // index was built looked current, and `count` reported the index's old row count
    let wrk = Workdir::new("count_index_rewritten_within_the_same_second");
    let mut csv = String::from("a,b\n");
    for i in 0..1_000 {
        csv.push_str(&format!("{i},x\n"));
    }
    wrk.create_from_string("data.csv", &csv);
    let mut cmd = wrk.command("index");
    cmd.arg("data.csv");
    wrk.assert_success(&mut cmd);

    wrk.create_from_string("data.csv", "a,b\n1,y\n2,y\n3,y\n");
    // the data is half a second newer than its index, within the same second
    filetime::set_file_mtime(
        wrk.path("data.csv.idx"),
        filetime::FileTime::from_unix_time(1_700_000_000, 100),
    )
    .unwrap();
    filetime::set_file_mtime(
        wrk.path("data.csv"),
        filetime::FileTime::from_unix_time(1_700_000_000, 500_000_000),
    )
    .unwrap();

    let mut cmd = wrk.command("count");
    cmd.arg("data.csv");
    let got: String = wrk.stdout(&mut cmd);
    assert_eq!(got, "3");
}

// a preamble with a blank line in it: the sniffer counts 2 rows (as the CSV reader skips empty
// lines), but they take up 3 lines
fn blank_line_preamble_csv() -> String {
    let mut csv = String::from("Report\n\nNotes here\nid,name,amount\n");
    for i in 1..=40 {
        csv.push_str(&format!("{i},n{i},{}\n", i * 10));
    }
    csv
}

#[test]
fn count_sniff_preamble_with_a_blank_line() {
    let wrk = Workdir::new("count_sniff_preamble_with_a_blank_line");
    wrk.create_from_string("p.csv", &blank_line_preamble_csv());
    let mut cmd = wrk.command("count");
    cmd.env("QSV_SNIFF_PREAMBLE", "1").arg("p.csv");
    let got: String = wrk.stdout(&mut cmd);
    assert_eq!(got, "40");
}
