use std::process;

use crate::workdir::Workdir;

fn setup(name: &str) -> (Workdir, process::Command) {
    let rows = vec![
        svec!["h1", "h2"],
        svec!["abcdef", "ghijkl"],
        svec!["mnopqr", "stuvwx"],
        svec!["ab\"cd\"ef", "gh,ij,kl"],
    ];

    let wrk = Workdir::new(name);
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("fmt");
    cmd.arg("in.csv");

    (wrk, cmd)
}

#[test]
fn fmt_delimiter() {
    let (wrk, mut cmd) = setup("fmt_delimiter");
    cmd.args(["--out-delimiter", "\t"]);

    let got: String = wrk.stdout(&mut cmd);
    let expected = "\
h1\th2
abcdef\tghijkl
mnopqr\tstuvwx
\"ab\"\"cd\"\"ef\"\tgh,ij,kl";
    assert_eq!(got, expected.to_string());
}

#[test]
fn fmt_weird_delimiter() {
    let (wrk, mut cmd) = setup("fmt_weird_delimiter");
    cmd.args(["--out-delimiter", "h"]);

    let got: String = wrk.stdout(&mut cmd);
    let expected = "\
\"h1\"h\"h2\"
abcdefh\"ghijkl\"
mnopqrhstuvwx
\"ab\"\"cd\"\"ef\"h\"gh,ij,kl\"";
    assert_eq!(got, expected.to_string());
}

#[test]
fn fmt_crlf() {
    let (wrk, mut cmd) = setup("fmt_crlf");
    cmd.arg("--crlf");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "\
h1,h2\r
abcdef,ghijkl\r
mnopqr,stuvwx\r
\"ab\"\"cd\"\"ef\",\"gh,ij,kl\"";
    assert_eq!(got, expected.to_string());
}

#[test]
fn fmt_tab_delimiter() {
    let (wrk, mut cmd) = setup("fmt_tab_delimiter");
    cmd.args(["--out-delimiter", "T"]);

    let got: String = wrk.stdout(&mut cmd);
    let expected = "h1\th2\nabcdef\tghijkl\nmnopqr\tstuvwx\n\"ab\"\"cd\"\"ef\"\tgh,ij,kl";
    assert_eq!(got, expected.to_string());
}

#[test]
fn fmt_nofinalnewline() {
    let (wrk, mut cmd) = setup("fmt_nofinalnewline");
    cmd.arg("--no-final-newline");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"h1,h2
abcdef,ghijkl
mnopqr,stuvwx
"ab""cd""ef","gh,ij,kl""#;
    assert_eq!(got, expected.to_string());
}

#[test]
fn fmt_output() {
    let (wrk, mut cmd) = setup("fmt_output");

    let output_file = wrk.path("output.csv").to_string_lossy().to_string();

    cmd.args(["--output", &output_file]);

    wrk.assert_success(&mut cmd);

    let got = wrk.read_to_string(&output_file).unwrap();

    let expected = r#"h1,h2
abcdef,ghijkl
mnopqr,stuvwx
"ab""cd""ef","gh,ij,kl"
"#;
    assert_eq!(got, expected);
}

#[test]
fn fmt_nofinalnewline_even_records() {
    // Regression: with the old two-record look-ahead loop, --no-final-newline
    // was silently ignored when the input had an even number of records. The
    // fmt_nofinalnewline test above doesn't catch it because wrk.stdout()
    // trims trailing whitespace; assert against the file output instead.
    let (wrk, mut cmd) = setup("fmt_nofinalnewline_even_records");
    let output_file = wrk.path("output.csv").to_string_lossy().to_string();
    cmd.args(["--no-final-newline", "--output", &output_file]);

    wrk.assert_success(&mut cmd);

    let got = wrk.read_to_string(&output_file).unwrap();
    let expected = r#"h1,h2
abcdef,ghijkl
mnopqr,stuvwx
"ab""cd""ef","gh,ij,kl""#;
    assert_eq!(got, expected);
}

#[test]
fn fmt_crlf_no_final_newline() {
    // Regression: pop()-on-String only stripped the trailing '\n', leaving a
    // stray '\r' under --crlf --no-final-newline.
    let (wrk, mut cmd) = setup("fmt_crlf_no_final_newline");
    let output_file = wrk.path("output.csv").to_string_lossy().to_string();
    cmd.args(["--crlf", "--no-final-newline", "--output", &output_file]);

    wrk.assert_success(&mut cmd);

    let got = wrk.read_to_string(&output_file).unwrap();
    let expected = "h1,h2\r\nabcdef,ghijkl\r\nmnopqr,stuvwx\r\n\"ab\"\"cd\"\"ef\",\"gh,ij,kl\"";
    assert_eq!(got, expected);
}

#[test]
fn fmt_ascii_no_final_newline() {
    // ASCII mode uses Record Separator (\x1e) as the terminator; --no-final-newline
    // must strip it from the last record.
    let (wrk, mut cmd) = setup("fmt_ascii_no_final_newline");
    let output_file = wrk.path("output.csv").to_string_lossy().to_string();
    cmd.args(["--ascii", "--no-final-newline", "--output", &output_file]);

    wrk.assert_success(&mut cmd);

    let got_bytes = std::fs::read(wrk.path("output.csv")).unwrap();
    let expected: &[u8] =
        b"h1\x1fh2\x1eabcdef\x1fghijkl\x1emnopqr\x1fstuvwx\x1eab\"cd\"ef\x1fgh,ij,kl";
    assert_eq!(got_bytes, expected);
}

#[test]
fn fmt_ascii_crlf_no_final_newline() {
    // --ascii overrides --crlf in wconfig, so the terminator is RS (\x1e),
    // not CRLF. The buffered last-record writer must mirror that precedence,
    // otherwise the last record could be terminated/quoted differently from
    // the rest. Output bytes must match `fmt_ascii_no_final_newline`.
    let (wrk, mut cmd) = setup("fmt_ascii_crlf_no_final_newline");
    let output_file = wrk.path("output.csv").to_string_lossy().to_string();
    cmd.args([
        "--ascii",
        "--crlf",
        "--no-final-newline",
        "--output",
        &output_file,
    ]);

    wrk.assert_success(&mut cmd);

    let got_bytes = std::fs::read(wrk.path("output.csv")).unwrap();
    let expected: &[u8] =
        b"h1\x1fh2\x1eabcdef\x1fghijkl\x1emnopqr\x1fstuvwx\x1eab\"cd\"ef\x1fgh,ij,kl";
    assert_eq!(got_bytes, expected);
}

#[test]
fn fmt_bom_no_final_newline() {
    // QSV_OUTPUT_BOM=1 makes Config::from_writer prepend a UTF-8 BOM. The
    // --no-final-newline path uses wconfig.from_writer for a final-record
    // buffer too, so without explicit handling the output would have a
    // duplicated BOM in the middle. Lock down: exactly one BOM at start,
    // no trailing terminator at end.
    let (wrk, mut cmd) = setup("fmt_bom_no_final_newline");
    let output_file = wrk.path("output.csv").to_string_lossy().to_string();
    cmd.args(["--no-final-newline", "--output", &output_file]);
    cmd.env("QSV_OUTPUT_BOM", "1");

    wrk.assert_success(&mut cmd);

    let got_bytes = std::fs::read(wrk.path("output.csv")).unwrap();
    let mut expected: Vec<u8> = b"\xEF\xBB\xBF".to_vec();
    expected
        .extend_from_slice(b"h1,h2\nabcdef,ghijkl\nmnopqr,stuvwx\n\"ab\"\"cd\"\"ef\",\"gh,ij,kl\"");
    assert_eq!(got_bytes, expected);
    // Belt-and-suspenders: only one BOM in the entire stream.
    assert_eq!(
        got_bytes
            .windows(3)
            .filter(|w| *w == b"\xEF\xBB\xBF")
            .count(),
        1,
        "expected exactly one UTF-8 BOM in output"
    );
}

#[test]
fn fmt_quote_always() {
    let (wrk, mut cmd) = setup("fmt_quote_always");
    cmd.arg("--quote-always");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "\
\"h1\",\"h2\"
\"abcdef\",\"ghijkl\"
\"mnopqr\",\"stuvwx\"
\"ab\"\"cd\"\"ef\",\"gh,ij,kl\"";
    assert_eq!(got, expected.to_string());
}

#[test]
fn fmt_quote_never() {
    let (wrk, mut cmd) = setup("fmt_quote_never");
    cmd.arg("--quote-never");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "\
h1,h2
abcdef,ghijkl
mnopqr,stuvwx
ab\"cd\"ef,gh,ij,kl";
    assert_eq!(got, expected.to_string());
}
