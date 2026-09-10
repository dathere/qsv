use crate::workdir::Workdir;

// The binary fixtures are cross-checked against pyreadstat, value-for-value,
// rather than round-tripped through the same reader under test:
//   readstat_sample.{sav,zsav,dta,xpt,por} were written with pyreadstat 1.3.6
//   readstat_order.sav likewise (2000 rows, uncompressed - see the ordering test)
//   readstat_sample.sas7bdat is airline.sas7bdat from pandas (BSD-3-Clause),
//   since nothing in the toolchain can write binary SAS datasets.

fn sample(wrk: &Workdir, ext: &str) -> String {
    wrk.load_test_file(&format!("readstat_sample.{ext}"))
}

/// Every format carries the same 4 rows, so they share one expectation.
/// `.por` is the exception: it upper-cases names and stores the missing string
/// as an empty string.
fn expected_rows() -> Vec<Vec<String>> {
    vec![
        svec!["id", "name", "score", "sex", "visited"],
        svec!["1.0", "Ana", "1.5", "1.0", "2020-01-31"],
        svec!["2.0", "Bo", "-2.25", "2.0", "1999-12-31"],
        svec!["3.0", "Chloë", "", "1.0", "2026-03-01"],
        svec!["4.0", "", "0.0", "", "1960-01-01"],
    ]
}

#[test]
fn readstat_spss_sav() {
    let wrk = Workdir::new("readstat_spss_sav");
    let f = sample(&wrk, "sav");
    let mut cmd = wrk.command("readstat");
    cmd.arg(f);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got, expected_rows());
}

#[test]
fn readstat_spss_zsav_compressed() {
    let wrk = Workdir::new("readstat_spss_zsav_compressed");
    let f = sample(&wrk, "zsav");
    let mut cmd = wrk.command("readstat");
    cmd.arg(f);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got, expected_rows());
}

#[test]
fn readstat_stata_dta() {
    let wrk = Workdir::new("readstat_stata_dta");
    let f = sample(&wrk, "dta");
    let mut cmd = wrk.command("readstat");
    cmd.arg(f);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got, expected_rows());
}

#[test]
fn readstat_sas_xport() {
    let wrk = Workdir::new("readstat_sas_xport");
    let f = sample(&wrk, "xpt");
    let mut cmd = wrk.command("readstat");
    cmd.arg(f);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got, expected_rows());
}

#[test]
fn readstat_spss_por() {
    let wrk = Workdir::new("readstat_spss_por");
    let f = sample(&wrk, "por");
    let mut cmd = wrk.command("readstat");
    cmd.arg(f);

    // SPSS portable upper-cases variable names, cannot hold the non-ASCII name,
    // and writes the missing string as an empty string.
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["ID", "NAME", "SCORE", "SEX", "VISITED"],
        svec!["1.0", "Ana", "1.5", "1.0", "2020-01-31"],
        svec!["2.0", "Bo", "-2.25", "2.0", "1999-12-31"],
        svec!["3.0", "Chloe", "", "1.0", "2026-03-01"],
        svec!["4.0", "", "0.0", "", "1960-01-01"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn readstat_sas7bdat() {
    let wrk = Workdir::new("readstat_sas7bdat");
    let f = sample(&wrk, "sas7bdat");
    let mut cmd = wrk.command("readstat");
    cmd.arg(f);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got[0], svec!["YEAR", "Y", "W", "R", "L", "K"]);
    assert_eq!(got.len(), 33); // header + 32 rows
    assert_eq!(got[1][0], "1948.0");
    assert_eq!(got[32][0], "1979.0");
}

#[test]
fn readstat_value_labels() {
    let wrk = Workdir::new("readstat_value_labels");
    let f = sample(&wrk, "sav");

    // off by default - the underlying codes survive
    let mut cmd = wrk.command("readstat");
    cmd.arg(&f);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(
        got.iter().skip(1).map(|r| r[3].clone()).collect::<Vec<_>>(),
        svec!["1.0", "2.0", "1.0", ""]
    );

    // ...and are decoded on request
    let mut cmd = wrk.command("readstat");
    cmd.arg("--value-labels").arg(&f);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(
        got.iter().skip(1).map(|r| r[3].clone()).collect::<Vec<_>>(),
        svec!["Male", "Female", "Male", ""]
    );
}

#[test]
fn readstat_value_labels_rejected_for_sas() {
    let wrk = Workdir::new("readstat_value_labels_rejected_for_sas");
    let f = sample(&wrk, "sas7bdat");
    let mut cmd = wrk.command("readstat");
    cmd.arg("--value-labels").arg(f);

    let (_, stderr) = wrk.stdout_and_stderr_on_error::<String>(&mut cmd);
    assert!(stderr.contains(".sas7bcat catalog"), "{stderr}");
}

/// Row order must not depend on `--jobs`.
///
/// readstat_order.sav is deliberately 2000 rows and uncompressed: the readers
/// only take their chunk-interleaving path with more than one thread on an
/// uncompressed file of over 1000 rows, so a smaller fixture would make this
/// assertion vacuous. Verified load-bearing by flipping `preserve_order` off,
/// which reorders the rows here.
#[test]
fn readstat_row_order_is_jobs_independent() {
    let wrk = Workdir::new("readstat_row_order_is_jobs_independent");
    let f = wrk.load_test_file("readstat_order.sav");

    let run = |jobs: &str| -> Vec<Vec<String>> {
        let mut cmd = wrk.command("readstat");
        cmd.args(["--jobs", jobs]).args(["--batch", "100"]).arg(&f);
        wrk.read_stdout(&mut cmd)
    };

    let single = run("1");
    assert_eq!(single.len(), 2001);
    assert_eq!(single[1][1], "row_0000");
    assert_eq!(single[2000][1], "row_1999");
    assert_eq!(run("8"), single);
}

#[test]
fn readstat_metadata_json() {
    let wrk = Workdir::new("readstat_metadata_json");
    let f = sample(&wrk, "sav");
    let mut cmd = wrk.command("readstat");
    cmd.args(["--metadata", "json"]).arg(f);

    let got = wrk.stdout::<String>(&mut cmd);
    let v: serde_json::Value = serde_json::from_str(&got).unwrap();
    assert_eq!(v["row_count"], 4);
    let vars = v["variables"].as_array().unwrap();
    assert_eq!(vars.len(), 5);
    assert_eq!(vars[3]["name"], "sex");
    assert_eq!(vars[3]["label"], "Respondent sex");
    assert_eq!(vars[3]["value_labels"]["1"], "Male");
    assert_eq!(vars[3]["value_labels"]["2"], "Female");
}

#[test]
fn readstat_metadata_csv() {
    let wrk = Workdir::new("readstat_metadata_csv");
    let f = sample(&wrk, "sav");
    let mut cmd = wrk.command("readstat");
    cmd.args(["--metadata", "csv"]).arg(f);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got[0][0], "name");
    assert_eq!(got.len(), 6); // header + 5 variables

    let label_col = got[0].iter().position(|h| h == "label").unwrap();
    assert_eq!(got[4][0], "sex");
    assert_eq!(got[4][label_col], "Respondent sex");
}

#[test]
fn readstat_output_delimiter_follows_extension() {
    let wrk = Workdir::new("readstat_output_delimiter_follows_extension");
    let f = sample(&wrk, "sav");
    let mut cmd = wrk.command("readstat");
    cmd.arg(f).args(["--output", "out.tsv"]);
    wrk.assert_success(&mut cmd);

    let got = wrk.from_str::<String>(&wrk.path("out.tsv"));
    assert!(got.starts_with("id\tname\tscore\tsex\tvisited"));
}

#[test]
fn readstat_rejects_catalog() {
    let wrk = Workdir::new("readstat_rejects_catalog");
    let f = sample(&wrk, "sas7bdat");
    std::fs::copy(&f, wrk.path("fmts.sas7bcat")).unwrap();

    let mut cmd = wrk.command("readstat");
    cmd.arg(wrk.path("fmts.sas7bcat"));

    let (_, stderr) = wrk.stdout_and_stderr_on_error::<String>(&mut cmd);
    assert!(
        stderr.contains("is a SAS format catalog, not a dataset"),
        "{stderr}"
    );
}

#[test]
fn readstat_rejects_unknown_extension() {
    let wrk = Workdir::new("readstat_rejects_unknown_extension");
    wrk.create_from_string("data.csv", "a,b\n1,2\n");
    let mut cmd = wrk.command("readstat");
    cmd.arg(wrk.path("data.csv"));

    let (_, stderr) = wrk.stdout_and_stderr_on_error::<String>(&mut cmd);
    assert!(stderr.contains("Don't know how to read"), "{stderr}");
}

#[test]
fn readstat_rejects_stdin() {
    let wrk = Workdir::new("readstat_rejects_stdin");
    let mut cmd = wrk.command("readstat");

    let (_, stderr) = wrk.stdout_and_stderr_on_error::<String>(&mut cmd);
    assert!(
        stderr.contains("reading from stdin is not supported"),
        "{stderr}"
    );
}

#[test]
fn readstat_rejects_bad_metadata_format() {
    let wrk = Workdir::new("readstat_rejects_bad_metadata_format");
    let f = sample(&wrk, "sav");
    let mut cmd = wrk.command("readstat");
    cmd.args(["--metadata", "bogus"]).arg(f);

    let (_, stderr) = wrk.stdout_and_stderr_on_error::<String>(&mut cmd);
    assert!(
        stderr.contains("is not a valid --metadata format"),
        "{stderr}"
    );
}
