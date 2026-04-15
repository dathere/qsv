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
