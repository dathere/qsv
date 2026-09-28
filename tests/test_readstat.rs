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

// Sentinel (user-defined missing value) fixtures, cross-checked against
// pyreadstat 1.3.6 with `user_missing=True`:
//   readstat_sentinels.sav - written with pyreadstat: `income` declares 99 plus
//     the range 900-999, `rating` declares 8 & 9 (8 labeled "Refused"), `name`
//     declares "NA", and `plain` declares nothing.
//   readstat_sentinels_collide.sav - `income` declares 99, and the file already
//     has an `income_null` variable.
//   readstat_sentinels.sas7bdat - info_nulls_test_data.sas7bdat from
//     polars_readstat (Apache-2.0): 2000 rows of x, y, z with .A-.Z & ._ in y & z.

fn sentinels(wrk: &Workdir, ext: &str) -> String {
    wrk.load_test_file(&format!("readstat_sentinels.{ext}"))
}

#[test]
fn readstat_sentinels_off_by_default() {
    let wrk = Workdir::new("readstat_sentinels_off_by_default");
    let f = sentinels(&wrk, "sav");
    let mut cmd = wrk.command("readstat");
    cmd.arg(f);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["id", "income", "rating", "name", "plain"],
        svec!["1.0", "1500.5", "3.0", "Ana", "1.0"],
        svec!["2.0", "", "", "", "2.0"],
        svec!["3.0", "", "4.0", "Cy", "3.0"],
        svec!["4.0", "", "", "", "4.0"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn readstat_sentinels_spss_value() {
    let wrk = Workdir::new("readstat_sentinels_spss_value");
    let f = sentinels(&wrk, "sav");
    let mut cmd = wrk.command("readstat");
    cmd.args(["--sentinels-as", "value"]).arg(f);

    // discrete (99, 8, 9, "NA") & range (950) sentinels alike; `plain`
    // declares no missing values, so it gets no sentinel column
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![
            "id",
            "income",
            "income_null",
            "rating",
            "rating_null",
            "name",
            "name_null",
            "plain"
        ],
        svec!["1.0", "1500.5", "", "3.0", "", "Ana", "", "1.0"],
        svec!["2.0", "", "99", "", "8", "", "NA", "2.0"],
        svec!["3.0", "", "950", "4.0", "", "Cy", "", "3.0"],
        svec!["4.0", "", "", "", "9", "", "", "4.0"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn readstat_sentinels_spss_label() {
    let wrk = Workdir::new("readstat_sentinels_spss_label");
    let f = sentinels(&wrk, "sav");
    let mut cmd = wrk.command("readstat");
    cmd.args(["--value-labels", "--sentinels-as", "label"])
        .arg(f);

    // 8 is labeled, 9 is not
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got[0][3..5], svec!["rating", "rating_null"]);
    assert_eq!(
        got.iter()
            .skip(1)
            .map(|r| r[3..5].to_vec())
            .collect::<Vec<_>>(),
        vec![
            svec!["Good", ""],
            svec!["", "Refused"],
            svec!["Great", ""],
            svec!["", "9"],
        ]
    );
}

#[test]
fn readstat_sentinels_spss_embedded() {
    let wrk = Workdir::new("readstat_sentinels_spss_embedded");
    let f = sentinels(&wrk, "sav");
    let mut cmd = wrk.command("readstat");
    cmd.args(["--sentinels-as", "value", "--sentinels-embedded"])
        .arg(f);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["id", "income", "rating", "name", "plain"],
        svec!["1.0", "1500.5", "3.0", "Ana", "1.0"],
        svec!["2.0", "99", "8", "NA", "2.0"],
        svec!["3.0", "950", "4.0", "Cy", "3.0"],
        svec!["4.0", "", "9", "", "4.0"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn readstat_sentinels_columns_subset() {
    let wrk = Workdir::new("readstat_sentinels_columns_subset");
    let f = sentinels(&wrk, "sav");
    let mut cmd = wrk.command("readstat");
    // whitespace is trimmed & duplicates are ignored
    cmd.args([
        "--sentinels-as",
        "value",
        "--sentinels-columns",
        " name , income,name",
    ])
    .arg(f);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(
        got[0],
        svec![
            "id",
            "income",
            "income_null",
            "rating",
            "name",
            "name_null",
            "plain"
        ]
    );
    assert_eq!(got[2], svec!["2.0", "", "99", "", "", "NA", "2.0"]);
}

#[test]
fn readstat_sentinels_sas() {
    let wrk = Workdir::new("readstat_sentinels_sas");
    let f = sentinels(&wrk, "sas7bdat");
    let mut cmd = wrk.command("readstat");
    cmd.args(["--sentinels-as", "value"]).arg(f);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got.len(), 2001);
    assert_eq!(got[0], svec!["x", "x_null", "y", "y_null", "z", "z_null"]);
    assert_eq!(got[1], svec!["1.0", "", "", ".C", "", ".B"]);
    assert_eq!(got[4], svec!["4.0", "", "", ".C", "", ".Q"]);
    // per-column sentinel counts, as pyreadstat reports them
    let count = |col: usize| got.iter().skip(1).filter(|r| !r[col].is_empty()).count();
    assert_eq!((count(1), count(3), count(5)), (0, 934, 958));
}

#[test]
fn readstat_sentinels_sas_embedded_subset() {
    let wrk = Workdir::new("readstat_sentinels_sas_embedded_subset");
    let f = sentinels(&wrk, "sas7bdat");
    let mut cmd = wrk.command("readstat");
    cmd.args([
        "--sentinels-as",
        "value",
        "--sentinels-embedded",
        "--sentinels-columns",
        "y",
    ])
    .arg(f);

    // z is left alone, so its sentinels stay empty
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got[0], svec!["x", "y", "z"]);
    assert_eq!(got[1], svec!["1.0", ".C", ""]);
    assert_eq!(got[4], svec!["4.0", ".C", ""]);
}

#[test]
fn readstat_sentinels_sas_jobs_warns() {
    let wrk = Workdir::new("readstat_sentinels_sas_jobs_warns");
    let f = sentinels(&wrk, "sas7bdat");
    let mut cmd = wrk.command("readstat");
    cmd.args(["--sentinels-as", "value", "--jobs", "4"]).arg(f);

    let stderr = wrk.stderr_on_success(&mut cmd);
    assert!(stderr.contains("--jobs has no effect"), "{stderr}");
}

/// Every request the readers would silently get wrong is refused up front.
#[test]
fn readstat_sentinels_rejections() {
    let wrk = Workdir::new("readstat_sentinels_rejections");
    let sav = sentinels(&wrk, "sav");
    let sas = sentinels(&wrk, "sas7bdat");
    let collide = wrk.load_test_file("readstat_sentinels_collide.sav");
    let dta = sample(&wrk, "dta");
    let xpt = sample(&wrk, "xpt");
    let por = sample(&wrk, "por");

    let cases: Vec<(Vec<&str>, &str, &str)> = vec![
        (vec!["--sentinels-embedded"], &sav, "require --sentinels-as"),
        (
            vec!["--sentinels-columns", "income"],
            &sav,
            "require --sentinels-as",
        ),
        (
            vec!["--sentinels-as", "bogus"],
            &sav,
            "not a valid --sentinels-as value",
        ),
        (
            vec!["--sentinels-as", "value"],
            &dta,
            "not supported for Stata files",
        ),
        (
            vec!["--sentinels-as", "value"],
            &xpt,
            "SAS transport (.xpt)",
        ),
        (
            vec!["--sentinels-as", "value"],
            &por,
            "SPSS portable (.por)",
        ),
        (
            vec!["--sentinels-as", "label"],
            &sas,
            "--sentinels-as label is not supported for SAS",
        ),
        (
            vec!["--sentinels-as", "label"],
            &sav,
            "needs --value-labels on SPSS",
        ),
        (
            vec!["--value-labels", "--sentinels-as", "value"],
            &sav,
            "cannot be combined with --value-labels",
        ),
        (
            vec!["--sentinels-as", "value", "--metadata", "json"],
            &sav,
            "not to --metadata",
        ),
        (
            vec![
                "--sentinels-as",
                "value",
                "--sentinels-columns",
                "nope,income",
            ],
            &sav,
            "has no variable named \"nope\"",
        ),
        (
            vec![
                "--sentinels-as",
                "value",
                "--sentinels-columns",
                "plain,income",
            ],
            &sav,
            "\"plain\" cannot hold sentinels",
        ),
        (
            vec![
                "--sentinels-as",
                "value",
                "--sentinels-columns",
                "income,,name",
            ],
            &sav,
            "has an empty variable name",
        ),
        (
            vec!["--sentinels-as", "value"],
            &collide,
            "already has a variable named \"income_null\"",
        ),
    ];
    for (flags, file, expected) in cases {
        let mut cmd = wrk.command("readstat");
        cmd.args(&flags).arg(file);
        let stderr = wrk.stderr_on_error(&mut cmd);
        assert!(stderr.contains(expected), "{flags:?}: {stderr}");
    }
}

#[test]
fn readstat_sentinels_rejection_leaves_output_untouched() {
    let wrk = Workdir::new("readstat_sentinels_rejection_leaves_output_untouched");
    let f = sentinels(&wrk, "sav");
    wrk.create_from_string("out.csv", "keep me\n");
    let mut cmd = wrk.command("readstat");
    cmd.args(["--sentinels-as", "value", "--sentinels-columns", "nope"])
        .args(["--output", "out.csv"])
        .arg(f);

    wrk.assert_err(&mut cmd);
    assert_eq!(wrk.read_to_string("out.csv").unwrap(), "keep me\n");
}
