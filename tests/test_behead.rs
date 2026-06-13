use crate::workdir::Workdir;

#[test]
fn behead() {
    let wrk = Workdir::new("behead");
    wrk.create(
        "data.csv",
        vec![svec!["letter", "number"], svec!["a", "1"], svec!["b", "2"]],
    );
    let mut cmd = wrk.command("behead");
    cmd.arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["a", "1"], svec!["b", "2"]];
    assert_eq!(got, expected);
}

// Regression test: behead uses explicit .no_headers(false) to force header parsing.
// Even with QSV_NO_HEADERS=1 set, behead must still strip the first row as a header.
#[test]
fn behead_ignores_qsv_no_headers_envvar() {
    let wrk = Workdir::new("behead_ignores_qsv_no_headers_envvar");
    wrk.create(
        "data.csv",
        vec![svec!["letter", "number"], svec!["a", "1"], svec!["b", "2"]],
    );
    let mut cmd = wrk.command("behead");
    cmd.env("QSV_NO_HEADERS", "1");
    cmd.arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // behead should still drop the header row despite QSV_NO_HEADERS=1
    let expected = vec![svec!["a", "1"], svec!["b", "2"]];
    assert_eq!(got, expected);
}

// issue #1417 regression: writing to an EXISTING `.zip` output path must write to
// that file, not to the read-only temp the special-format handler extracts when it
// (eagerly) recognizes the existing zip. `io_writer` targets the user-specified
// output path, so the requested file is the one that gets the command's output.
#[test]
fn behead_output_to_existing_zip_path() {
    let wrk = Workdir::new("behead_output_to_existing_zip_path");
    wrk.create("in.csv", vec![svec!["h1", "h2"], svec!["1", "2"]]);
    // a real, existing zip used as the OUTPUT path
    let out = wrk.load_test_file("boston311-100.csv.zip");

    let mut cmd = wrk.command("behead");
    cmd.arg("in.csv").args(["--output", &out]);
    wrk.assert_success(&mut cmd);

    // the output file is now the beheaded CSV, not the original zip bytes
    let got = std::fs::read_to_string(&out).unwrap();
    assert!(got.contains("1,2"), "got: {got:?}");
    assert!(
        !got.contains("case_enquiry_id"),
        "original zip content should be overwritten"
    );
}

// issue #1417 regression: the WRITER delimiter must come from the output path's
// extension, never from the contents of an existing special-format output file.
// tsv-in-zip.zip's first entry is a `.tsv`; writing to it must still emit comma-
// delimited output (the `.zip` output path implies the default comma delimiter),
// not tab-delimited just because the file being overwritten contained a TSV.
#[test]
fn behead_output_to_existing_zip_uses_output_path_delimiter() {
    let wrk = Workdir::new("behead_output_to_existing_zip_uses_output_path_delimiter");
    wrk.create("in.csv", vec![svec!["h1", "h2"], svec!["1", "2"]]);
    let out = wrk.load_test_file("tsv-in-zip.zip");

    let mut cmd = wrk.command("behead");
    cmd.arg("in.csv").args(["--output", &out]);
    wrk.assert_success(&mut cmd);

    let got = std::fs::read_to_string(&out).unwrap();
    assert!(
        got.contains("1,2"),
        "expected comma-delimited output, got: {got:?}"
    );
    assert!(
        !got.contains("1\t2"),
        "output delimiter must not be inherited from the overwritten zip's .tsv entry"
    );
}
