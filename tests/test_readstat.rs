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

// Stata extended missings (`.a`-`.z`) must render empty whatever the storage
// type (#4627). polars-readstat-rs before 0.23.2 returned them as NaN in
// `float`/`double` variables but as null in integer ones, so `d`, `f` & `l`
// below printed `NaN` while `i` & `b` printed empty.
//   readstat_stata_extmiss.dta - written with pyreadstat 1.3.6: `id`, `d`, `f`,
//     `l` are `double`, `i`, `b` are `long`, `s` is a string holding the text
//     "NaN"; `.a`/`.b`/`.c`/`.z` scattered through; `l` labels 1 as "low".
//   readstat_stata_float_missing.dta - missing_test.dta from polars_readstat
//     (Apache-2.0): 9 `float` variables, var1-var6 `.a`-`.z`, var7-8 `.`.
#[test]
fn readstat_stata_extended_missing_is_empty() {
    let wrk = Workdir::new("readstat_stata_extended_missing_is_empty");
    let f = wrk.load_test_file("readstat_stata_extmiss.dta");
    let mut cmd = wrk.command("readstat");
    cmd.arg(f);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["id", "d", "f", "i", "b", "l", "s"],
        svec!["1.0", "1.5", "0.5", "10", "1", "1.0", "x"],
        svec!["2.0", "", "", "", "2", "", "NaN"],
        svec!["3.0", "2.25", "", "30", "", "2.5", "y"],
        svec!["4.0", "", "3.0", "", "4", "", "NaN"],
    ];
    assert_eq!(got, expected);
}

// With `--value-labels` the labeled `double` `l` is turned into strings, and
// its `.a`/`.z` used to come out as the text "NaN". The string variable `s`
// genuinely holds "NaN" and must keep it.
#[test]
fn readstat_stata_extended_missing_is_empty_value_labels() {
    let wrk = Workdir::new("readstat_stata_extended_missing_is_empty_value_labels");
    let f = wrk.load_test_file("readstat_stata_extmiss.dta");
    let mut cmd = wrk.command("readstat");
    cmd.arg("--value-labels").arg(f);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["id", "d", "f", "i", "b", "l", "s"],
        svec!["1.0", "1.5", "0.5", "10", "1", "low", "x"],
        svec!["2.0", "", "", "", "2", "", "NaN"],
        svec!["3.0", "2.25", "", "30", "", "2.5", "y"],
        svec!["4.0", "", "3.0", "", "4", "", "NaN"],
    ];
    assert_eq!(got, expected);
}

// A value label may itself be spelled "NaN": `l` labels 1 as "NaN" and holds
// `.a`/`.z` in rows 2 & 4, so only those rows may be empty - a missing must not
// be confused with the label text. `m` has no such label.
//   readstat_stata_nan_label.dta - written with pyreadstat 1.3.6: `id`, `l`,
//     `m` are `double`; `l` labels 1 "NaN", 2 "two"; `m` labels 1 "one".
#[test]
fn readstat_stata_value_label_spelled_nan_is_kept() {
    let wrk = Workdir::new("readstat_stata_value_label_spelled_nan_is_kept");
    let f = wrk.load_test_file("readstat_stata_nan_label.dta");
    let expected = vec![
        svec!["id", "l", "m"],
        svec!["1.0", "NaN", "one"],
        svec!["2.0", "", ""],
        svec!["3.0", "two", "3"],
        svec!["4.0", "", "4"],
    ];
    // batch 1 puts every row in its own batch, exercising the row offsets
    for batch in ["0", "1"] {
        let mut cmd = wrk.command("readstat");
        cmd.arg("--value-labels").args(["--batch", batch]).arg(&f);
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        assert_eq!(got, expected, "--batch {batch}");
    }
}

#[test]
fn readstat_stata_float_extended_missing_is_empty() {
    let wrk = Workdir::new("readstat_stata_float_extended_missing_is_empty");
    let f = wrk.load_test_file("readstat_stata_float_missing.dta");
    let mut cmd = wrk.command("readstat");
    cmd.arg(f);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![
            "var1", "var2", "var3", "var4", "var5", "var6", "var7", "var8", "var9"
        ],
        svec!["", "", "", "", "", "", "", "", "1.0"],
    ];
    assert_eq!(got, expected);
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
//   readstat_sentinels_collide.dta - written with pyreadstat: `double`
//     variables `x` & `x_null`.
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
fn readstat_sentinels_jobs_warns() {
    let wrk = Workdir::new("readstat_sentinels_jobs_warns");
    // SAS & Stata read serially when tracking sentinels; SPSS does not
    let files = [
        (sentinels(&wrk, "sas7bdat"), true),
        (wrk.load_test_file("readstat_stata_extmiss.dta"), true),
        (sentinels(&wrk, "sav"), false),
    ];
    for (f, warns) in files {
        let mut cmd = wrk.command("readstat");
        cmd.args(["--sentinels-as", "value", "--jobs", "4"]).arg(&f);
        let stderr = wrk.stderr_on_success(&mut cmd);
        assert_eq!(
            stderr.contains("--jobs has no effect"),
            warns,
            "{f}: {stderr}"
        );
    }
}

// Stata sentinels need polars-readstat-rs 0.23.2: before it, the `float` &
// `double` ones were lost (wrong bit step between `.a`-`.z`) and the labels of
// `.a`-`.z` dropped. `readstat_stata_extmiss.dta` mixes `double` (`d`, `f`,
// `l`) & `long` (`i`, `b`) variables, so both storage paths are checked; the
// string `s` cannot hold a sentinel and gets no column.
#[test]
fn readstat_sentinels_stata_value() {
    let wrk = Workdir::new("readstat_sentinels_stata_value");
    let f = wrk.load_test_file("readstat_stata_extmiss.dta");
    let mut cmd = wrk.command("readstat");
    cmd.args(["--sentinels-as", "value"]).arg(f);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![
            "id", "id_null", "d", "d_null", "f", "f_null", "i", "i_null", "b", "b_null", "l",
            "l_null", "s"
        ],
        svec![
            "1.0", "", "1.5", "", "0.5", "", "10", "", "1", "", "1.0", "", "x"
        ],
        svec![
            "2.0", "", "", ".a", "", ".b", "", ".a", "2", "", "", ".a", "NaN"
        ],
        svec![
            "3.0", "", "2.25", "", "", ".z", "30", "", "", ".c", "2.5", "", "y"
        ],
        svec![
            "4.0", "", "", ".z", "3.0", "", "", ".z", "4", "", "", ".z", "NaN"
        ],
    ];
    assert_eq!(got, expected);
}

// `readstat_stata_float_missing.dta` labels var1's `.a` "missing"; the other
// sentinels are unlabeled, so they fall back to their codes. Unlike SPSS,
// Stata's sentinel labels do not depend on --value-labels.
#[test]
fn readstat_sentinels_stata_label() {
    let wrk = Workdir::new("readstat_sentinels_stata_label");
    let f = wrk.load_test_file("readstat_stata_float_missing.dta");
    let expected = vec![
        svec![
            "var1",
            "var1_null",
            "var2",
            "var2_null",
            "var3",
            "var3_null",
            "var4",
            "var4_null",
            "var5",
            "var5_null",
            "var6",
            "var6_null",
            "var7",
            "var7_null",
            "var8",
            "var8_null",
            "var9",
            "var9_null"
        ],
        svec![
            "", "missing", "", ".b", "", ".c", "", ".x", "", ".y", "", ".z", "", "", "", "", "1.0",
            ""
        ],
    ];
    for value_labels in [false, true] {
        let mut cmd = wrk.command("readstat");
        if value_labels {
            cmd.arg("--value-labels");
        }
        cmd.args(["--sentinels-as", "label"]).arg(&f);
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        assert_eq!(got, expected, "--value-labels {value_labels}");
    }
}

#[test]
fn readstat_sentinels_stata_embedded_subset() {
    let wrk = Workdir::new("readstat_sentinels_stata_embedded_subset");
    let f = wrk.load_test_file("readstat_stata_extmiss.dta");
    let mut cmd = wrk.command("readstat");
    cmd.args([
        "--sentinels-as",
        "value",
        "--sentinels-embedded",
        "--sentinels-columns",
        "f,i",
    ])
    .arg(f);

    // only `f` & `i` keep their sentinels; `d`, `b` & `l` stay empty
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["id", "d", "f", "i", "b", "l", "s"],
        svec!["1.0", "1.5", "0.5", "10", "1", "1.0", "x"],
        svec!["2.0", "", ".b", ".a", "2", "", "NaN"],
        svec!["3.0", "2.25", ".z", "30", "", "2.5", "y"],
        svec!["4.0", "", "3.0", ".z", "4", "", "NaN"],
    ];
    assert_eq!(got, expected);
}

/// Every request the readers would silently get wrong is refused up front.
#[test]
fn readstat_sentinels_rejections() {
    let wrk = Workdir::new("readstat_sentinels_rejections");
    let sav = sentinels(&wrk, "sav");
    let sas = sentinels(&wrk, "sas7bdat");
    let collide = wrk.load_test_file("readstat_sentinels_collide.sav");
    let collide_dta = wrk.load_test_file("readstat_sentinels_collide.dta");
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
            vec!["--sentinels-as", "value", "--sentinels-columns", "name"],
            &dta,
            "\"name\" cannot hold sentinels. Only numeric variables can.",
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
        (
            vec!["--sentinels-as", "value"],
            &collide_dta,
            "already has a variable named \"x_null\"",
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

// `--compress-numeric` fixture:
//   readstat_compress.sav - written with pyreadstat 1.3.6, all `double`: `flag`
//     holds only 0 & 1, `n` whole numbers (one negative), `mix` 1, 2, 2.5 & a
//     missing, `big` 1e20 then small whole numbers, `empty` only missings.

/// The choice is per column over the whole file, so the output must not
/// depend on the batch size: upstream's own `compress_numeric` decides per
/// batch, and at `--batch 1` it prints `mix` as `2` in one row and `2.0` in
/// another, and 0/1 columns as `true`/`false`.
#[test]
fn readstat_compress_numeric() {
    let wrk = Workdir::new("readstat_compress_numeric");
    let f = wrk.load_test_file("readstat_compress.sav");
    // `mix` has a 2.5 & `big` is past 2^53, so both keep their ".0"
    let expected = vec![
        svec!["flag", "n", "mix", "big", "empty"],
        svec!["0", "1", "1.0", "1e+20", ""],
        svec!["1", "2", "2.0", "2.0", ""],
        svec!["1", "3", "2.5", "3.0", ""],
        svec!["0", "-4", "", "4.0", ""],
    ];
    for (batch, jobs) in [("0", "1"), ("1", "1"), ("2", "1"), ("1", "4")] {
        let mut cmd = wrk.command("readstat");
        cmd.arg("--compress-numeric")
            .args(["--batch", batch, "--jobs", jobs])
            .arg(&f);
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        assert_eq!(got, expected, "--batch {batch} --jobs {jobs}");
    }
}

#[test]
fn readstat_compress_numeric_formats() {
    let wrk = Workdir::new("readstat_compress_numeric_formats");
    let mut expected = expected_rows();
    for row in expected.iter_mut().skip(1) {
        for i in [0, 3] {
            if let Some(whole) = row[i].strip_suffix(".0") {
                row[i] = whole.to_string();
            }
        }
    }
    // xpt: its reader garbles projected columns, which pass 1 must avoid
    for ext in ["sav", "zsav", "dta", "xpt"] {
        let f = sample(&wrk, ext);
        let mut cmd = wrk.command("readstat");
        cmd.arg("--compress-numeric").arg(f);
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        assert_eq!(got, expected, "{ext}");
    }

    // `.por` is read whole rather than streamed
    let f = sample(&wrk, "por");
    let mut cmd = wrk.command("readstat");
    cmd.arg("--compress-numeric").arg(f);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got[1], svec!["1", "Ana", "1.5", "1", "2020-01-31"]);
    assert_eq!(got[4][2], "0.0");
}

/// Stata `float` variables are Float32, with their own whole-number limit.
#[test]
fn readstat_compress_numeric_stata() {
    let wrk = Workdir::new("readstat_compress_numeric_stata");
    let f = wrk.load_test_file("readstat_stata_extmiss.dta");
    let mut cmd = wrk.command("readstat");
    cmd.arg("--compress-numeric").arg(f);

    // `d`, `f` & `l` hold a fraction somewhere, so they keep their ".0"
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["id", "d", "f", "i", "b", "l", "s"],
        svec!["1", "1.5", "0.5", "10", "1", "1.0", "x"],
        svec!["2", "", "", "", "2", "", "NaN"],
        svec!["3", "2.25", "", "30", "", "2.5", "y"],
        svec!["4", "", "3.0", "", "4", "", "NaN"],
    ];
    assert_eq!(got, expected);

    let f = wrk.load_test_file("readstat_stata_float_missing.dta");
    let mut cmd = wrk.command("readstat");
    cmd.arg("--compress-numeric").arg(f);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got[1][8], "1");
}

/// The publishing recipe: sentinels kept in the variable's own column, which
/// turns it into text. Its numbers must still lose their ".0".
#[test]
fn readstat_compress_numeric_sentinels() {
    let wrk = Workdir::new("readstat_compress_numeric_sentinels");
    let f = sentinels(&wrk, "sav");

    let mut cmd = wrk.command("readstat");
    cmd.args([
        "--compress-numeric",
        "--sentinels-as",
        "value",
        "--sentinels-embedded",
    ])
    .arg(&f);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["id", "income", "rating", "name", "plain"],
        svec!["1", "1500.5", "3", "Ana", "1"],
        svec!["2", "99", "8", "NA", "2"],
        svec!["3", "950", "4", "Cy", "3"],
        svec!["4", "", "9", "", "4"],
    ];
    assert_eq!(got, expected);

    let mut cmd = wrk.command("readstat");
    cmd.args(["--compress-numeric", "--sentinels-as", "value"])
        .arg(&f);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got[1], svec!["1", "1500.5", "", "3", "", "Ana", "", "1"]);
    assert_eq!(got[2], svec!["2", "", "99", "", "8", "", "NA", "2"]);
}

// readstat_stata_numeric_labels.dta - written with pyreadstat 1.3.6: `double`
// `v` holds 1, .a, 3, .b with .a labeled "001.0" & .b "1.0"; `v`'s labels
// read as numbers, so embedding them beside its numbers must not rewrite them.
// `w` holds 1, .a, 2, .a with .a labeled "Refused", and is compressed as usual.
// `x` is `w` plus the ordinary value 1 labeled "001.0": without --value-labels
// that label is never written, so it must not stop the compression.
#[test]
fn readstat_compress_numeric_keeps_numeric_labels() {
    let wrk = Workdir::new("readstat_compress_numeric_keeps_numeric_labels");
    let f = wrk.load_test_file("readstat_stata_numeric_labels.dta");
    let mut cmd = wrk.command("readstat");
    cmd.args([
        "--compress-numeric",
        "--sentinels-as",
        "label",
        "--sentinels-embedded",
    ])
    .arg(&f);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["v", "w", "x"],
        svec!["1.0", "1", "1"],
        svec!["001.0", "Refused", "Refused"],
        svec!["3.0", "2", "2"],
        svec!["1.0", "Refused", "Refused"],
    ];
    assert_eq!(got, expected);

    // in their own <name>_null columns the labels are never at risk
    let mut cmd = wrk.command("readstat");
    cmd.args(["--compress-numeric", "--sentinels-as", "label"])
        .arg(&f);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got[2], svec!["", "001.0", "", "Refused", "", "Refused"]);
    assert_eq!(got[3], svec!["3", "", "2", "", "2", ""]);
}

#[test]
fn readstat_compress_numeric_sas() {
    let wrk = Workdir::new("readstat_compress_numeric_sas");
    let f = sample(&wrk, "sas7bdat");
    let mut cmd = wrk.command("readstat");
    cmd.arg("--compress-numeric").arg(f);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got[1][0], "1948");
    assert_eq!(got[1][1], "1.2139999866485596");

    // SAS sentinels embedded as text (.C) next to whole-number codes
    let f = sentinels(&wrk, "sas7bdat");
    let mut cmd = wrk.command("readstat");
    cmd.args([
        "--compress-numeric",
        "--sentinels-as",
        "value",
        "--sentinels-embedded",
    ])
    .arg(f);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got[1], svec!["1", ".C", ".B"]);
    assert_eq!(got[2], svec!["2", "0.9990987514611332", "31"]);
}

// readstat_partial_labels.sav - written with pyreadstat 1.3.6: `q` holds 1, 2,
// 5 & a missing, labeled 1 "yes" & 2 "no"; `r` holds 1, 2.5, 1, 2, labeled 1
// "one". Under --value-labels both become text, and the reader already writes
// their unlabeled whole numbers without ".0" - --compress-numeric must agree.
#[test]
fn readstat_compress_numeric_value_labels() {
    let wrk = Workdir::new("readstat_compress_numeric_value_labels");
    let f = wrk.load_test_file("readstat_partial_labels.sav");
    let expected = vec![
        svec!["q", "r"],
        svec!["yes", "one"],
        svec!["no", "2.5"],
        svec!["5", "one"],
        svec!["", "2"],
    ];
    for compress in [false, true] {
        let mut cmd = wrk.command("readstat");
        if compress {
            cmd.arg("--compress-numeric");
        }
        cmd.arg("--value-labels").arg(&f);
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        assert_eq!(got, expected, "--compress-numeric {compress}");
    }
}

#[test]
fn readstat_compress_numeric_rejects_metadata() {
    let wrk = Workdir::new("readstat_compress_numeric_rejects_metadata");
    let f = sample(&wrk, "sav");
    let mut cmd = wrk.command("readstat");
    cmd.args(["--compress-numeric", "--metadata", "json"])
        .arg(f);

    let stderr = wrk.stderr_on_error(&mut cmd);
    assert!(stderr.contains("not to --metadata"), "{stderr}");
}
