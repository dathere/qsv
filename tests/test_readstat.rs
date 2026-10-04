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
    // no .sas7bcat beside it, so there are no labels to decode
    let f = sample(&wrk, "sas7bdat");
    wrk.create_from_string("out.csv", "keep me\n");
    let mut cmd = wrk.command("readstat");
    cmd.arg("--value-labels")
        .args(["--output", "out.csv"])
        .arg(f);

    let (_, stderr) = wrk.stdout_and_stderr_on_error::<String>(&mut cmd);
    assert!(
        stderr.contains("needs its .sas7bcat format catalog"),
        "{stderr}"
    );
    assert!(stderr.contains("Pass it with --sas7bcat"), "{stderr}");
    assert_eq!(wrk.read_to_string("out.csv").unwrap(), "keep me\n");
}

// SAS format catalog (.sas7bcat) fixtures, each a data file & its catalog,
// cross-checked against pyreadstat 1.3.6 (`read_sas7bdat(catalog_file=...)`):
//   readstat_catalog.{sas7bdat,sas7bcat} - hadley.sas7bdat & formats.sas7bcat
//     from haven (MIT): numeric format WORKSHOP (1 "R", 2 "SAS") on `workshop`,
//     character format $GENDER ("f" "Female", "m" "Male") on `gender`.
//   readstat_catalog_tagged.{sas7bdat,sas7bcat} - tagged-na.* from haven: `x`
//     holds 1-5, .A, .H & .Z; format XFMT labels only .A "Apple" & .Z "Zebra".
//   readstat_catalog_{linux,win}.{sas7bdat,sas7bcat} - test_data_*.sas7bdat &
//     test_formats_*.sas7bcat from pyreadstat (Apache-2.0): character formats
//     $A & $B on `SEXA` & `SEXB`, in both catalog layouts.

fn catalog_rows() -> Vec<Vec<String>> {
    vec![
        svec!["id", "workshop", "gender", "q1", "q2", "q3", "q4"],
        svec!["1.0", "R", "Female", "1.0", "1.0", "5.0", "1.0"],
        svec!["2.0", "SAS", "Female", "2.0", "1.0", "4.0", "1.0"],
        svec!["3.0", "R", "Female", "2.0", "2.0", "4.0", "3.0"],
        svec!["4.0", "SAS", "", "3.0", "1.0", "", "3.0"],
        svec!["5.0", "R", "Male", "4.0", "5.0", "2.0", "4.0"],
        svec!["6.0", "SAS", "Male", "5.0", "4.0", "5.0", "5.0"],
        svec!["7.0", "R", "Male", "5.0", "3.0", "4.0", "4.0"],
        svec!["8.0", "SAS", "Male", "4.0", "5.0", "5.0", "5.0"],
    ]
}

/// A copy of a fixture under another name, for the discovery tests.
fn copy_fixture(wrk: &Workdir, fixture: &str, name: &str) {
    let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("resources/test")
        .join(fixture);
    std::fs::copy(src, wrk.path(name)).unwrap();
}

#[test]
fn readstat_sas7bcat() {
    let wrk = Workdir::new("readstat_sas7bcat");
    let f = wrk.load_test_file("readstat_catalog.sas7bdat");
    let cat = wrk.load_test_file("readstat_catalog.sas7bcat");

    // --sas7bcat names the catalog for --value-labels
    let mut cmd = wrk.command("readstat");
    cmd.args(["--value-labels", "--sas7bcat", &cat]).arg(&f);
    let (got, stderr): (Vec<Vec<String>>, String) = wrk.read_stdout_and_stderr_on_success(&mut cmd);
    assert_eq!(got, catalog_rows());
    assert!(stderr.contains("Using SAS format catalog"), "{stderr}");

    // --value-labels finds <name>.sas7bcat beside the data file
    let mut cmd = wrk.command("readstat");
    cmd.arg("--value-labels").arg(&f);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got, catalog_rows());

    // without either, codes as usual
    let mut cmd = wrk.command("readstat");
    cmd.arg(&f);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got[1][1..3], svec!["1.0", "f"]);
}

#[test]
fn readstat_sas7bcat_discovery_order() {
    let wrk = Workdir::new("readstat_sas7bcat_discovery_order");
    let f = wrk.load_test_file("readstat_catalog.sas7bdat");

    // only formats.sas7bcat: used
    copy_fixture(&wrk, "readstat_catalog.sas7bcat", "formats.sas7bcat");
    let mut cmd = wrk.command("readstat");
    cmd.arg("--value-labels").arg(&f);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got, catalog_rows());

    // <name>.sas7bcat wins over formats.sas7bcat, here a catalog without
    // the file's formats
    copy_fixture(&wrk, "readstat_catalog_tagged.sas7bcat", "formats.sas7bcat");
    copy_fixture(
        &wrk,
        "readstat_catalog.sas7bcat",
        "readstat_catalog.sas7bcat",
    );
    let mut cmd = wrk.command("readstat");
    cmd.arg("--value-labels").arg(&f);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got, catalog_rows());
}

#[test]
fn readstat_sas7bcat_character_formats() {
    let wrk = Workdir::new("readstat_sas7bcat_character_formats");
    // the two catalog layouts, 64-bit (linux) & 32-bit (win)
    for os in ["linux", "win"] {
        let f = wrk.load_test_file(&format!("readstat_catalog_{os}.sas7bdat"));
        wrk.load_test_file(&format!("readstat_catalog_{os}.sas7bcat"));
        let mut cmd = wrk.command("readstat");
        cmd.arg("--value-labels").arg(&f);
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![
            svec!["ID", "SEXA", "SEXB"],
            svec!["ID1", "Male", "Male"],
            svec!["ID2", "Female", "Female"],
            svec!["ID3", "Male", "Male"],
        ];
        assert_eq!(got, expected, "{os}");
    }
}

/// A labeled numeric column is text; its unlabeled numbers are written like
/// Stata's & SPSS's, without ".0", and its sentinels are left alone.
#[test]
fn readstat_sas7bcat_sentinels() {
    let wrk = Workdir::new("readstat_sas7bcat_sentinels");
    let f = wrk.load_test_file("readstat_catalog_tagged.sas7bdat");
    wrk.load_test_file("readstat_catalog_tagged.sas7bcat");
    let numbers = ["1", "2", "3", "4", "5"];

    // a one-column CSV writes an empty cell as an empty line, so compare text
    let mut cmd = wrk.command("readstat");
    cmd.arg("--value-labels").arg(&f);
    let out = cmd.output().unwrap();
    assert!(out.status.success());
    assert_eq!(
        String::from_utf8(out.stdout).unwrap(),
        "x\n1\n2\n3\n4\n5\n\n\n\n"
    );

    let mut cmd = wrk.command("readstat");
    cmd.args([
        "--value-labels",
        "--sentinels-as",
        "value",
        "--sentinels-embedded",
    ])
    .arg(&f);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let col: Vec<&str> = got.iter().skip(1).map(|r| r[0].as_str()).collect();
    assert_eq!(col[..5], numbers);
    assert_eq!(col[5..], [".A", ".H", ".Z"]);

    let mut cmd = wrk.command("readstat");
    cmd.args(["--value-labels", "--sentinels-as", "value"])
        .arg(&f);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got[0], svec!["x", "x_null"]);
    assert_eq!(got[1], svec!["1", ""]);
    assert_eq!(got[6], svec!["", ".A"]);
}

/// SAS sentinel labels need polars-readstat-rs 0.23.3: before it, the catalog
/// reader dropped which sentinel (.A-.Z, ._) a label was for
/// (`jrothbaum/polars_readstat#65`). pyreadstat reads `x` as 1-5, "Apple", "H",
/// "Zebra"; an unlabeled sentinel keeps its code, as for Stata & SPSS.
/// `--sentinels-as label` labels only the sentinels: the values stay codes
/// unless `--value-labels`.
#[test]
fn readstat_sas7bcat_sentinel_labels() {
    let wrk = Workdir::new("readstat_sas7bcat_sentinel_labels");
    let f = wrk.load_test_file("readstat_catalog_tagged.sas7bdat");
    let cat = wrk.load_test_file("readstat_catalog_tagged.sas7bcat");

    // the catalog is found beside the data file, as for --value-labels
    let mut cmd = wrk.command("readstat");
    cmd.args(["--sentinels-as", "label"]).arg(&f);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got[0], svec!["x", "x_null"]);
    assert_eq!(got[1], svec!["1.0", ""]);
    assert_eq!(got[6], svec!["", "Apple"]);
    assert_eq!(got[7], svec!["", ".H"]);
    assert_eq!(got[8], svec!["", "Zebra"]);

    let mut cmd = wrk.command("readstat");
    cmd.args(["--sentinels-as", "label", "--sentinels-columns", "x"])
        .arg(&f);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got[6], svec!["", "Apple"]);

    // with --value-labels, the values are labeled too (x has none, so they
    // lose their ".0" as labeled columns' unlabeled numbers do)
    let mut cmd = wrk.command("readstat");
    cmd.args(["--value-labels", "--sentinels-as", "label"])
        .arg(&f);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got[1], svec!["1", ""]);
    assert_eq!(got[6], svec!["", "Apple"]);

    // --sas7bcat names the catalog; --compress-numeric leaves labels alone
    for embedded in [false, true] {
        let mut cmd = wrk.command("readstat");
        cmd.args(["--sas7bcat", &cat, "--sentinels-as", "label"])
            .arg("--compress-numeric");
        if embedded {
            cmd.arg("--sentinels-embedded");
        }
        cmd.arg(&f);
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let last = got[0].len() - 1;
        let col: Vec<&str> = got.iter().skip(1).map(|r| r[last].as_str()).collect();
        assert_eq!(col[5..], ["Apple", ".H", "Zebra"], "embedded: {embedded}");
        assert_eq!(got[1][0], "1", "embedded: {embedded}");
    }

    // a labeled value is not decoded without --value-labels
    let f = wrk.load_test_file("readstat_catalog.sas7bdat");
    wrk.load_test_file("readstat_catalog.sas7bcat");
    let mut cmd = wrk.command("readstat");
    cmd.args(["--sentinels-as", "label"]).arg(&f);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got[0][2], "workshop");
    assert_eq!(got[1][2], "1.0");
}

#[test]
fn readstat_sas7bcat_compress_numeric() {
    let wrk = Workdir::new("readstat_sas7bcat_compress_numeric");
    let f = wrk.load_test_file("readstat_catalog.sas7bdat");
    wrk.load_test_file("readstat_catalog.sas7bcat");
    let mut cmd = wrk.command("readstat");
    cmd.args(["--value-labels", "--compress-numeric"]).arg(&f);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got[1], svec!["1", "R", "Female", "1", "1", "5", "1"]);
    assert_eq!(got[4], svec!["4", "SAS", "", "3", "1", "", "3"]);
}

#[test]
fn readstat_sas7bcat_metadata() {
    let wrk = Workdir::new("readstat_sas7bcat_metadata");
    let f = wrk.load_test_file("readstat_catalog.sas7bdat");
    let cat = wrk.load_test_file("readstat_catalog.sas7bcat");

    let mut cmd = wrk.command("readstat");
    cmd.args(["--metadata", "json", "--sas7bcat", &cat]).arg(&f);
    let got: serde_json::Value = serde_json::from_str(&wrk.stdout::<String>(&mut cmd)).unwrap();
    let cols = got["columns"].as_array().unwrap();
    assert_eq!(cols[0]["name"], "id");
    assert!(cols[0].get("value_labels").is_none());
    assert_eq!(
        cols[1]["value_labels"],
        serde_json::json!({"1": "R", "2": "SAS"})
    );
    assert_eq!(
        cols[2]["value_labels"],
        serde_json::json!({"f": "Female", "m": "Male"})
    );

    let mut cmd = wrk.command("readstat");
    cmd.args(["--metadata", "csv", "--sas7bcat", &cat]).arg(&f);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let at = got[0].iter().position(|h| h == "value_labels").unwrap();
    assert_eq!(got[2][at], r#"{"1":"R","2":"SAS"}"#);
    assert_eq!(got[1][at], "");

    // XFMT labels only sentinels, keyed as the Stata metadata keys .a
    let f = wrk.load_test_file("readstat_catalog_tagged.sas7bdat");
    let cat = wrk.load_test_file("readstat_catalog_tagged.sas7bcat");
    let mut cmd = wrk.command("readstat");
    cmd.args(["--metadata", "json", "--sas7bcat", &cat]).arg(&f);
    let got: serde_json::Value = serde_json::from_str(&wrk.stdout::<String>(&mut cmd)).unwrap();
    assert_eq!(
        got["columns"][0]["value_labels"],
        serde_json::json!({"MISSING_A": "Apple", "MISSING_Z": "Zebra"})
    );
}

#[test]
fn readstat_sas7bcat_rejections() {
    let wrk = Workdir::new("readstat_sas7bcat_rejections");
    let sas = wrk.load_test_file("readstat_catalog.sas7bdat");
    let cat = wrk.load_test_file("readstat_catalog.sas7bcat");
    let dta = sample(&wrk, "dta");
    let xpt = sample(&wrk, "xpt");

    let cases: Vec<(Vec<&str>, &str, &str)> = vec![
        (
            vec!["--sas7bcat", &cat],
            &dta,
            "--sas7bcat applies to SAS .sas7bdat files only",
        ),
        (
            vec!["--value-labels"],
            &xpt,
            "not supported for SAS transport (.xpt) files",
        ),
        (
            vec!["--value-labels", "--sas7bcat", "nope.sas7bcat"],
            &sas,
            "Cannot find the SAS format catalog \"nope.sas7bcat\"",
        ),
        (
            vec!["--sas7bcat", &cat],
            &sas,
            "--sas7bcat only names the SAS format catalog",
        ),
    ];
    for (flags, file, expected) in cases {
        let mut cmd = wrk.command("readstat");
        cmd.args(&flags).arg(file);
        let stderr = wrk.stderr_on_error(&mut cmd);
        assert!(stderr.contains(expected), "{flags:?}: {stderr}");
    }

    // a catalog holding none of the file's formats warns & writes codes
    let other = wrk.load_test_file("readstat_catalog_tagged.sas7bcat");
    let mut cmd = wrk.command("readstat");
    cmd.args(["--value-labels", "--sas7bcat", &other]).arg(&sas);
    let (got, stderr): (Vec<Vec<String>>, String) = wrk.read_stdout_and_stderr_on_success(&mut cmd);
    assert!(stderr.contains("None of the formats"), "{stderr}");
    assert_eq!(got[1][1..3], svec!["1.0", "f"]);
}

/// The shared --output guard only checks command-line arguments, so a catalog
/// found by --value-labels needs its own check, or it would be overwritten.
#[test]
fn readstat_sas7bcat_output_is_catalog() {
    let wrk = Workdir::new("readstat_sas7bcat_output_is_catalog");
    let f = wrk.load_test_file("readstat_catalog.sas7bdat");
    let cat = wrk.load_test_file("readstat_catalog.sas7bcat");
    copy_fixture(&wrk, "readstat_catalog.sas7bcat", "formats.sas7bcat");
    let before = std::fs::read(&cat).unwrap();

    for out in [cat.as_str(), "./readstat_catalog.sas7bcat"] {
        let mut cmd = wrk.command("readstat");
        cmd.arg("--value-labels").args(["--output", out]).arg(&f);
        let stderr = wrk.stderr_on_error(&mut cmd);
        assert!(
            stderr.contains("is the SAS format catalog being read"),
            "{out}: {stderr}"
        );
        assert_eq!(std::fs::read(&cat).unwrap(), before, "{out}");
    }

    // formats.sas7bcat, found when <name>.sas7bcat is absent
    std::fs::remove_file(&cat).unwrap();
    let shared = wrk.path("formats.sas7bcat");
    let before = std::fs::read(&shared).unwrap();
    let mut cmd = wrk.command("readstat");
    cmd.arg("--value-labels")
        .args(["--output", "formats.sas7bcat"])
        .arg(&f);
    let stderr = wrk.stderr_on_error(&mut cmd);
    assert!(
        stderr.contains("is the SAS format catalog being read"),
        "{stderr}"
    );
    assert_eq!(std::fs::read(&shared).unwrap(), before);
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
        stderr.contains(
            "is a SAS format catalog, not a dataset. Pass the .sas7bdat file, with this catalog \
             as --sas7bcat"
        ),
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

/// Without --sentinels-as, the sentinels are tracked to warn about them, in
/// indicator columns that must never reach the output: it is byte-identical
/// to an explicit `--sentinels-as none`, which skips the check.
#[test]
fn readstat_sentinel_check_leaves_output_alone() {
    let wrk = Workdir::new("readstat_sentinel_check_leaves_output_alone");
    let files = [
        "readstat_sentinels.sas7bdat",
        "readstat_sentinels.sav",
        "readstat_sentinels_labels.sav",
        "readstat_sentinels_dot.sav",
        "readstat_stata_extmiss.dta",
        "readstat_stata_float_missing.dta",
        "readstat_stata_numeric_labels.dta",
        "readstat_partial_labels.sav",
        "readstat_sample.sas7bdat",
        "readstat_sample.dta",
        "readstat_sample.sav",
        "readstat_catalog_tagged.sas7bdat",
    ];
    wrk.load_test_file("readstat_catalog_tagged.sas7bcat");
    for file in files {
        let f = wrk.load_test_file(file);
        for (flags, max_bytes) in [
            (vec![], None),
            (vec!["--value-labels"], None),
            (vec!["--compress-numeric"], None),
            (vec!["--batch", "1"], None),
            (vec!["--jobs", "2"], None),
            // the SPSS check names the declaring variables instead of counting
            (vec![], Some("1")),
        ] {
            let run = |extra: &[&str]| {
                let mut cmd = wrk.command("readstat");
                if let Some(max) = max_bytes {
                    cmd.env("QSV_TEST_READSTAT_SPSS_COUNT_MAX_BYTES", max);
                }
                cmd.args(extra).args(&flags).arg(&f);
                cmd.output().unwrap()
            };
            let checked = run(&[]);
            let unchecked = run(&["--sentinels-as", "none"]);
            assert_eq!(checked.status.success(), unchecked.status.success());
            assert_eq!(
                String::from_utf8_lossy(&checked.stdout),
                String::from_utf8_lossy(&unchecked.stdout),
                "{file} {flags:?}"
            );
        }
    }
}

/// The check counts the sentinels written as empty cells, per variable.
/// Expected counts: pyreadstat finds 1892 `.A`-`.Z` in the SAS file (y, z);
/// the SPSS file holds 99 & 950 in `income`, 8 & 9 in `rating`, "NA" in
/// `name`; the Stata file 9 in `d`, `f`, `i`, `b` & `l`.
#[test]
fn readstat_sentinel_check_warns() {
    let wrk = Workdir::new("readstat_sentinel_check_warns");
    let cases = [
        (
            sentinels(&wrk, "sas7bdat"),
            "1892 sentinels (user-defined missing values) in 2 variables (y, z)",
        ),
        (
            sentinels(&wrk, "sav"),
            "5 sentinels (user-defined missing values) in 3 variables (income, rating, name) were \
             written as empty cells, e.g. 99 in income.",
        ),
        (
            wrk.load_test_file("readstat_stata_extmiss.dta"),
            "9 sentinels (user-defined missing values) in 5 variables (d, f, i, b, l)",
        ),
    ];
    for (f, expected) in cases {
        let mut cmd = wrk.command("readstat");
        cmd.arg(&f);
        let stderr = wrk.stderr_on_success(&mut cmd);
        assert!(stderr.contains(expected), "{f}: {stderr}");
        assert!(stderr.contains("--sentinels-as none"), "{f}: {stderr}");
    }

    // counts add up across batches
    let mut cmd = wrk.command("readstat");
    cmd.args(["--batch", "1"]).arg(sentinels(&wrk, "sav"));
    let stderr = wrk.stderr_on_success(&mut cmd);
    assert!(stderr.contains("5 sentinels"), "{stderr}");

    // the advice is one qsv accepts: SPSS refuses `value` with --value-labels
    let mut cmd = wrk.command("readstat");
    cmd.arg("--value-labels").arg(sentinels(&wrk, "sav"));
    let stderr = wrk.stderr_on_success(&mut cmd);
    assert!(
        stderr.contains("Use --sentinels-as label to keep"),
        "{stderr}"
    );

    // readstat_sentinels_dot.sav - written with pyreadstat 1.3.6: `code`
    // ("a", ".", "b", ".") declares "." missing. Only SAS's bare "." is
    // system missing; in SPSS it can be a sentinel.
    let mut cmd = wrk.command("readstat");
    cmd.arg(wrk.load_test_file("readstat_sentinels_dot.sav"));
    let stderr = wrk.stderr_on_success(&mut cmd);
    assert!(stderr.contains("2 sentinels"), "{stderr}");
}

/// No warning without sentinels, when told not to check, when --jobs makes
/// the check too costly for SAS & Stata (which, unlike SPSS, don't declare
/// their sentinels), or for the readers that don't report sentinels.
#[test]
fn readstat_sentinel_check_quiet() {
    let wrk = Workdir::new("readstat_sentinel_check_quiet");
    let mut runs: Vec<(Vec<&str>, String)> = ["sas7bdat", "dta", "sav", "zsav", "xpt", "por"]
        .into_iter()
        .map(|ext| (vec![], sample(&wrk, ext)))
        .collect();
    for ext in ["sav", "zsav"] {
        runs.push((vec!["--jobs", "2"], sample(&wrk, ext)));
    }
    for f in [sentinels(&wrk, "sav"), sentinels(&wrk, "sas7bdat")] {
        runs.push((vec!["--sentinels-as", "none"], f.clone()));
        runs.push((vec!["--sentinels-as", "none", "--jobs", "2"], f));
    }
    runs.push((vec!["--jobs", "2"], sentinels(&wrk, "sas7bdat")));
    runs.push((
        vec!["--jobs", "2"],
        wrk.load_test_file("readstat_stata_extmiss.dta"),
    ));
    for (flags, f) in runs {
        let mut cmd = wrk.command("readstat");
        cmd.args(&flags).arg(&f);
        let stderr = wrk.stderr_on_success(&mut cmd);
        assert!(!stderr.contains("sentinel"), "{flags:?} {f}: {stderr}");
    }
}

/// Where counting the SPSS sentinels would cost the reader its parallelism
/// (--jobs above 1) or read too large a file whole, the warning names the
/// variables that declare missing values instead. `id` & `plain` declare none.
#[test]
fn readstat_sentinel_check_spss_declared() {
    let wrk = Workdir::new("readstat_sentinel_check_spss_declared");
    let f = sentinels(&wrk, "sav");
    let declared = "3 variables (income, rating, name) declare user-defined missing values";
    let run = |flags: &[&str], max_bytes: Option<&str>| {
        let mut cmd = wrk.command("readstat");
        if let Some(max) = max_bytes {
            cmd.env("QSV_TEST_READSTAT_SPSS_COUNT_MAX_BYTES", max);
        }
        cmd.args(flags).arg(&f);
        wrk.stderr_on_success(&mut cmd)
    };

    let stderr = run(&["--jobs", "2"], None);
    assert!(stderr.contains(declared), "{stderr}");
    assert!(stderr.contains("--jobs is above 1"), "{stderr}");
    assert!(!stderr.contains("5 sentinels"), "{stderr}");
    assert!(
        stderr.contains("Use --sentinels-as value to keep"),
        "{stderr}"
    );

    // the advice is one qsv accepts: SPSS refuses `value` with --value-labels
    let stderr = run(&["--jobs", "2", "--value-labels"], None);
    assert!(stderr.contains(declared), "{stderr}");
    assert!(
        stderr.contains("Use --sentinels-as label to keep"),
        "{stderr}"
    );

    // the file holds 160 bytes of data (4 rows of 5 8-byte variables): counted
    // at that cutoff, named only above it
    let stderr = run(&[], Some("160"));
    assert!(stderr.contains("5 sentinels"), "{stderr}");
    let stderr = run(&[], Some("159"));
    assert!(stderr.contains(declared), "{stderr}");
    assert!(stderr.contains("over about 128 MB of data"), "{stderr}");
    assert!(!stderr.contains("5 sentinels"), "{stderr}");

    // one declaring variable reads in the singular
    let mut cmd = wrk.command("readstat");
    cmd.env("QSV_TEST_READSTAT_SPSS_COUNT_MAX_BYTES", "1")
        .args(["--jobs", "1"])
        .arg(wrk.load_test_file("readstat_sentinels_dot.sav"));
    let stderr = wrk.stderr_on_success(&mut cmd);
    assert!(
        stderr.contains("1 variable (code) declares user-defined missing values"),
        "{stderr}"
    );
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

/// Without --value-labels, `--sentinels-as label` labels only the sentinels:
/// qsv looks each one up among its variable's value labels, as the SPSS reader
/// labels sentinels only along with every value. `rating` labels 3, 4 & 8 and
/// declares 8 & 9 missing, so 8 is "Refused", 9 keeps its code, and 3 & 4 stay
/// codes. `income` (range 900-999 & 99) and `name` ("NA") have no labels.
#[test]
fn readstat_sentinels_spss_label_only() {
    let wrk = Workdir::new("readstat_sentinels_spss_label_only");
    let f = sentinels(&wrk, "sav");

    let mut cmd = wrk.command("readstat");
    cmd.args(["--sentinels-as", "label"]).arg(&f);
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
        svec!["2.0", "", "99", "", "Refused", "", "NA", "2.0"],
        svec!["3.0", "", "950", "4.0", "", "Cy", "", "3.0"],
        svec!["4.0", "", "", "", "9", "", "", "4.0"],
    ];
    assert_eq!(got, expected);

    let mut cmd = wrk.command("readstat");
    cmd.args(["--sentinels-as", "label", "--sentinels-embedded"])
        .arg(&f);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let rating: Vec<&str> = got.iter().skip(1).map(|r| r[2].as_str()).collect();
    assert_eq!(rating, ["3.0", "Refused", "4.0", "9"]);

    // only the selected variables' sentinels are kept & labeled
    let mut cmd = wrk.command("readstat");
    cmd.args(["--sentinels-as", "label", "--sentinels-columns", "rating"])
        .arg(&f);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(
        got[0],
        svec!["id", "income", "rating", "rating_null", "name", "plain"]
    );
    assert_eq!(got[2][3], "Refused");
}

// readstat_sentinels_labels.sav - written with pyreadstat 1.3.6: `score`
// (1, 2, 950, 99) declares the range 900-999 & 99 missing and labels 1 "one",
// 950 "Skipped" & 99 "1.0"; `code` ("a", "NA", "b", "a") declares "NA"
// missing and labels "a" "Alpha" & "NA" "Not asked".

/// Label-only sentinels from a missing range & a missing string, with a
/// non-missing labeled value next to each that must stay a code.
#[test]
fn readstat_sentinels_spss_label_only_ranges_strings() {
    let wrk = Workdir::new("readstat_sentinels_spss_label_only_ranges_strings");
    let f = wrk.load_test_file("readstat_sentinels_labels.sav");

    let mut cmd = wrk.command("readstat");
    cmd.args(["--sentinels-as", "label", "--sentinels-embedded"])
        .arg(&f);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["score", "code"],
        svec!["1.0", "a"],
        svec!["2.0", "Not asked"],
        svec!["Skipped", "b"],
        svec!["1.0", "a"],
    ];
    assert_eq!(got, expected);

    // the "1.0" label keeps every number of `score` its ".0", so it isn't
    // rewritten to "1"; in a separate column it can't be mistaken for one
    let mut cmd = wrk.command("readstat");
    cmd.args(["--sentinels-as", "label", "--sentinels-embedded"])
        .arg("--compress-numeric")
        .arg(&f);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got, expected);

    let mut cmd = wrk.command("readstat");
    cmd.args(["--sentinels-as", "label", "--compress-numeric"])
        .arg(&f);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got[1], svec!["1", "", "a", ""]);
    assert_eq!(got[4], svec!["", "1.0", "a", ""]);

    // only a tracked variable's labels hold back its ".0"
    let mut cmd = wrk.command("readstat");
    cmd.args(["--sentinels-as", "label", "--sentinels-embedded"])
        .args(["--sentinels-columns", "code", "--compress-numeric"])
        .arg(&f);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got[1], svec!["1", "a"]);
}

// readstat_sentinels_suffix.sav - written with pyreadstat 1.3.6: `rating`
// (labels & declares missing 8) & `score` (9) sit next to variables of the
// file named `rating_null` (text, holding "8") & `score_null` (numeric, 9).

/// Sentinel labels go only into the `<name>_null` columns qsv adds. A variable
/// of the file with such a name is left alone when its namesake isn't
/// tracked; when it is, the collision check refuses the request instead.
#[test]
fn readstat_sentinels_label_leaves_suffix_named_variables() {
    let wrk = Workdir::new("readstat_sentinels_label_leaves_suffix_named_variables");
    let f = wrk.load_test_file("readstat_sentinels_suffix.sav");

    for embedded in [false, true] {
        let mut cmd = wrk.command("readstat");
        cmd.args(["--sentinels-as", "label", "--sentinels-columns", "other"]);
        if embedded {
            cmd.arg("--sentinels-embedded");
        }
        cmd.arg(&f);
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        assert_eq!(
            got[1][..4],
            svec!["3.0", "8", "1.0", "9.0"],
            "embedded: {embedded}"
        );
        assert_eq!(
            got[2][..4],
            svec!["", "x", "", "2.0"],
            "embedded: {embedded}"
        );
    }

    let mut cmd = wrk.command("readstat");
    cmd.args(["--sentinels-as", "label"]).arg(&f);
    let stderr = wrk.stderr_on_error(&mut cmd);
    assert!(stderr.contains("named \"rating_null\""), "{stderr}");
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
    // every reader goes serial when tracking sentinels (SPSS decodes the whole
    // file on one thread); at --jobs 1 there is nothing to warn about
    let files = [
        sentinels(&wrk, "sas7bdat"),
        wrk.load_test_file("readstat_stata_extmiss.dta"),
        sentinels(&wrk, "sav"),
    ];
    for f in files {
        for (jobs, warns) in [("4", true), ("1", false)] {
            let mut cmd = wrk.command("readstat");
            cmd.args(["--sentinels-as", "value", "--jobs", jobs])
                .arg(&f);
            let stderr = wrk.stderr_on_success(&mut cmd);
            assert_eq!(
                stderr.contains("--jobs has no effect"),
                warns,
                "{f} --jobs {jobs}: {stderr}"
            );
        }
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
            "--sentinels-as label on a SAS file needs its .sas7bcat format catalog",
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
    // pass 1 reads only the float columns; xpt's reader garbled projected
    // columns before polars-readstat-rs 0.23.3 (jrothbaum/polars_readstat#64)
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

/// `readstat_vls_collide.sav` was written by the polars-readstat 0.23.3 writer, which named
/// the continuation records of the very long string `S35r1` `S35R11`..`S35R14` - so the
/// next variable, itself a very long string with short name `S35R11`, collides with one.
/// Readers before 0.24.0 (`jrothbaum/polars_readstat#68`) attached its width to the ghost and
/// split it into 255-byte pieces; pyreadstat reads the 4 columns below.
#[test]
fn readstat_spss_very_long_string_name_collision() {
    let wrk = Workdir::new("readstat_spss_very_long_string_name_collision");
    let f = wrk.load_test_file("readstat_vls_collide.sav");
    let mut cmd = wrk.command("readstat");
    cmd.arg(f);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let long = "x".repeat(1000);
    let expected = vec![
        svec!["S35r1", "S35r11", "S35r12", "n"],
        vec![
            format!("{long}A"),
            format!("{long}B"),
            "short".to_string(),
            "1.0".to_string(),
        ],
    ];
    assert_eq!(got, expected);
}

/// `expected_rows()` (cross-checked with pyreadstat) projected to `cols`, by
/// header name, in the order given.
fn project(rows: &[Vec<String>], cols: &[&str]) -> Vec<Vec<String>> {
    let idx: Vec<usize> = cols
        .iter()
        .map(|c| rows[0].iter().position(|h| h == c).unwrap())
        .collect();
    rows.iter()
        .map(|r| idx.iter().map(|&i| r[i].clone()).collect())
        .collect()
}

#[test]
fn readstat_select_names_ranges_negation() {
    let wrk = Workdir::new("readstat_select_names_ranges_negation");
    let all = expected_rows();
    let cases: [(&str, &str, &[&str]); 7] = [
        // order follows --select, not the file
        ("sav", "score,id", &["score", "id"]),
        ("zsav", "2-3", &["name", "score"]),
        ("dta", "!name", &["id", "score", "sex", "visited"]),
        // a leading ! excludes every variable listed after it
        ("sav", "!name,sex", &["id", "score", "visited"]),
        ("dta", "/^s/", &["score", "sex"]),
        // XPT's reader decoded projected columns from the wrong variable
        // before polars-readstat-rs 0.23.3 (jrothbaum/polars_readstat#64)
        ("xpt", "3,1", &["score", "id"]),
        ("xpt", "visited", &["visited"]),
    ];
    for (ext, select, cols) in cases {
        let mut cmd = wrk.command("readstat");
        cmd.args(["--select", select]).arg(sample(&wrk, ext));
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        assert_eq!(got, project(&all, cols), "{ext} --select {select}");
    }
}

#[test]
fn readstat_select_por() {
    let wrk = Workdir::new("readstat_select_por");
    let mut cmd = wrk.command("readstat");
    cmd.args(["--select", "SCORE,ID", "--offset", "2"])
        .arg(sample(&wrk, "por"));
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["SCORE", "ID"], svec!["", "3.0"], svec!["0.0", "4.0"]];
    assert_eq!(got, expected);

    // .por is read whole, so qsv itself ends the window at --limit
    let mut cmd = wrk.command("readstat");
    cmd.args(["--select", "ID,SCORE", "--offset", "1", "--limit", "2"])
        .arg(sample(&wrk, "por"));
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["ID", "SCORE"],
        svec!["2.0", "-2.25"],
        svec!["3.0", ""],
    ];
    assert_eq!(got, expected);
}

#[test]
fn readstat_offset_limit() {
    let wrk = Workdir::new("readstat_offset_limit");
    let all = expected_rows();
    // (offset, limit, rows of `all` expected, 1-based); batch 1 puts every
    // row in its own batch, so the window spans batches
    let cases: [(&str, Option<&str>, &[usize]); 5] = [
        ("1", Some("2"), &[2, 3]),
        ("0", Some("1"), &[1]),
        ("3", Some("10"), &[4]),
        ("2", None, &[3, 4]),
        ("9", None, &[]),
    ];
    for ext in ["sav", "dta", "xpt"] {
        for (offset, limit, rows) in cases {
            for batch in ["1", "50000"] {
                let mut cmd = wrk.command("readstat");
                cmd.args(["--offset", offset, "--batch", batch]);
                if let Some(limit) = limit {
                    cmd.args(["--limit", limit]);
                }
                let f = sample(&wrk, ext);
                cmd.arg(&f);
                let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
                let mut expected = vec![all[0].clone()];
                expected.extend(rows.iter().map(|&r| all[r].clone()));
                assert_eq!(
                    got, expected,
                    "{ext} --offset {offset} --limit {limit:?} -b {batch}"
                );
            }
        }
    }
}

#[test]
fn readstat_offset_limit_sas7bdat_values() {
    let wrk = Workdir::new("readstat_offset_limit_sas7bdat_values");
    let mut cmd = wrk.command("readstat");
    cmd.args(["--select", "YEAR,Y", "--offset", "30", "--limit", "5"])
        .arg(sample(&wrk, "sas7bdat"));
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // pyreadstat: read_sas7bdat(usecols=["Y","YEAR"], row_offset=30)
    assert_eq!(got[0], svec!["YEAR", "Y"]);
    assert_eq!(got.len(), 3, "{got:?}");
    for (row, (year, y)) in got[1..]
        .iter()
        .zip([("1978.0", 22.726_f64), ("1979.0", 23.619)])
    {
        assert_eq!(row[0], year);
        let v: f64 = row[1].parse().unwrap();
        assert!((v - y).abs() < 1e-3, "{row:?}");
    }
}

/// The parallel readers keep rows in order on a partial read too: `--limit`
/// and `--offset` must give the same rows at any --jobs.
#[test]
fn readstat_offset_limit_keeps_order_with_jobs() {
    let wrk = Workdir::new("readstat_offset_limit_keeps_order_with_jobs");
    let f = wrk.load_test_file("readstat_order.sav");

    let mut serial = wrk.command("readstat");
    serial.arg(&f);
    let all: Vec<Vec<String>> = wrk.read_stdout(&mut serial);

    let mut cmd = wrk.command("readstat");
    cmd.args([
        "--jobs", "4", "--batch", "100", "--offset", "1000", "--limit", "500",
    ])
    .arg(&f);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let mut expected = vec![all[0].clone()];
    expected.extend_from_slice(&all[1001..1501]);
    assert_eq!(got, expected);
}

#[test]
fn readstat_sample_seeded() {
    let wrk = Workdir::new("readstat_sample_seeded");
    let f = sample(&wrk, "sas7bdat");

    let mut full = wrk.command("readstat");
    full.arg(&f);
    let all: Vec<Vec<String>> = wrk.read_stdout(&mut full);

    let run = |seed: &str, batch: &str| -> Vec<Vec<String>> {
        let mut cmd = wrk.command("readstat");
        cmd.args(["--sample", "5", "--seed", seed, "--batch", batch])
            .arg(&f);
        wrk.read_stdout(&mut cmd)
    };
    let got = run("42", "50000");
    assert_eq!(got.len(), 6, "{got:?}");
    assert_eq!(got[0], all[0]);
    // the sample is drawn by position, so batching doesn't change it
    assert_eq!(got, run("42", "3"));
    // a subsequence of the file, in file order
    let mut pos = 1;
    for row in &got[1..] {
        pos += all[pos..].iter().position(|r| r == row).unwrap() + 1;
    }
}

#[test]
fn readstat_sample_within_window() {
    let wrk = Workdir::new("readstat_sample_within_window");
    let f = sample(&wrk, "sas7bdat");
    let mut full = wrk.command("readstat");
    full.arg(&f);
    let all: Vec<Vec<String>> = wrk.read_stdout(&mut full);

    let mut cmd = wrk.command("readstat");
    cmd.args([
        "--sample", "4", "--seed", "7", "--offset", "10", "--limit", "8",
    ])
    .arg(&f);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got.len(), 5, "{got:?}");
    for row in &got[1..] {
        assert!(all[11..19].contains(row), "{row:?} is outside rows 10..18");
    }

    // a sample no smaller than the window keeps all of it
    let mut cmd = wrk.command("readstat");
    cmd.args(["--sample", "50", "--offset", "30"]).arg(&f);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let mut expected = vec![all[0].clone()];
    expected.extend_from_slice(&all[31..]);
    assert_eq!(got, expected);
}

#[test]
fn readstat_select_sentinel_columns() {
    let wrk = Workdir::new("readstat_select_sentinel_columns");
    let mut cmd = wrk.command("readstat");
    cmd.args(["--sentinels-as", "value", "--select", "rating,id"])
        .arg(sentinels(&wrk, "sav"));
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // pyreadstat: rating's declared missings are 8 & 9, in rows 2 & 4
    let expected = vec![
        svec!["rating", "rating_null", "id"],
        svec!["3.0", "", "1.0"],
        svec!["", "8", "2.0"],
        svec!["4.0", "", "3.0"],
        svec!["", "9", "4.0"],
    ];
    assert_eq!(got, expected);

    let mut cmd = wrk.command("readstat");
    cmd.args([
        "--sentinels-as",
        "value",
        "--sentinels-columns",
        "income",
        "--select",
        "id",
    ])
    .arg(sentinels(&wrk, "sav"));
    let stderr = wrk.stderr_on_error(&mut cmd);
    assert!(
        stderr.contains("--sentinels-columns names \"income\", which --select leaves out"),
        "{stderr}"
    );

    // every reader adds the sentinel columns of the projected variables
    for (f, select, header) in [
        (
            wrk.load_test_file("readstat_stata_extmiss.dta"),
            "d,id",
            svec!["d", "d_null", "id", "id_null"],
        ),
        (
            sentinels(&wrk, "sas7bdat"),
            "y,x",
            svec!["y", "y_null", "x", "x_null"],
        ),
    ] {
        let mut cmd = wrk.command("readstat");
        cmd.args(["--sentinels-as", "value", "--select", select])
            .arg(&f);
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        assert_eq!(got[0], header, "{f}");
    }

    // a variable of the file named `<name>_null` is selected like any other,
    // and is not taken for its neighbour's sentinel column
    let mut cmd = wrk.command("readstat");
    cmd.args([
        "--sentinels-as",
        "value",
        "--sentinels-columns",
        "other",
        "--select",
        "rating,rating_null,other",
    ])
    .arg(wrk.load_test_file("readstat_sentinels_suffix.sav"));
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["rating", "rating_null", "other", "other_null"],
        svec!["3.0", "8", "1.0", ""],
        svec!["", "x", "", "5"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn readstat_sentinel_warning_counts_only_rows_and_variables_written() {
    let wrk = Workdir::new("readstat_sentinel_warning_counts_only_rows_and_variables_written");
    let cases: [(&[&str], Option<&str>); 5] = [
        (
            &["--select", "rating"],
            Some("2 sentinels (user-defined missing values) in 1 variable (rating)"),
        ),
        // pyreadstat, row_offset=2: income 950 (in its 900-999 range) & rating 9
        (
            &["--offset", "2"],
            Some("2 sentinels (user-defined missing values) in 2 variables (income, rating)"),
        ),
        (&["--select", "id,plain"], None),
        // above one job, SPSS names the variables declaring missing values
        // instead of counting - only the selected ones
        (
            &["--select", "rating,id", "--jobs", "2"],
            Some("1 variable (rating) declares user-defined missing values"),
        ),
        (&["--select", "id,plain", "--jobs", "2"], None),
    ];
    for (args, expected) in cases {
        let mut cmd = wrk.command("readstat");
        cmd.args(args).arg(sentinels(&wrk, "sav"));
        let stderr = wrk.stderr_on_success(&mut cmd);
        match expected {
            Some(expected) => assert!(stderr.contains(expected), "{args:?}: {stderr}"),
            None => assert!(!stderr.contains("sentinel"), "{args:?}: {stderr}"),
        }
    }

    // SAS: z's sentinels are left out with z
    let mut cmd = wrk.command("readstat");
    cmd.args(["--select", "y"]).arg(sentinels(&wrk, "sas7bdat"));
    let stderr = wrk.stderr_on_success(&mut cmd);
    assert!(stderr.contains("in 1 variable (y)"), "{stderr}");
}

#[test]
fn readstat_compress_numeric_over_rows_written() {
    let wrk = Workdir::new("readstat_compress_numeric_over_rows_written");
    // score holds 1.5 & -2.25 in rows 1-2 only, so from row 3 on it is whole
    let mut cmd = wrk.command("readstat");
    cmd.args([
        "--compress-numeric",
        "--offset",
        "2",
        "--select",
        "id,score",
    ])
    .arg(sample(&wrk, "sav"));
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["id", "score"], svec!["3", ""], svec!["4", "0"]];
    assert_eq!(got, expected);
}

#[test]
fn readstat_metadata_select() {
    let wrk = Workdir::new("readstat_metadata_select");
    // SAS names its list `columns`, Stata & SPSS `variables`
    for (ext, key, select, names) in [
        ("sav", "variables", "score,id", ["score", "id"]),
        ("dta", "variables", "2,1", ["name", "id"]),
        ("sas7bdat", "columns", "Y,YEAR", ["Y", "YEAR"]),
    ] {
        let mut cmd = wrk.command("readstat");
        cmd.args(["--metadata", "json", "--select", select])
            .arg(sample(&wrk, ext));
        let got: String = wrk.stdout(&mut cmd);
        let json: serde_json::Value = serde_json::from_str(&got).unwrap();
        let listed: Vec<&str> = json[key]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v["name"].as_str().unwrap())
            .collect();
        assert_eq!(listed, names, "{ext}");
        assert!(
            json.get("row_count").is_some(),
            "{ext}: file-level keys kept"
        );
    }
}

#[test]
fn readstat_selection_refusals() {
    let wrk = Workdir::new("readstat_selection_refusals");
    let cases: [(&[&str], &str); 6] = [
        (
            &["--metadata", "json", "--limit", "2"],
            "don't apply to --metadata",
        ),
        (&["--seed", "3"], "--seed requires --sample"),
        (&["--sample", "0"], "--sample must be at least 1"),
        (
            &["--select", "id,id"],
            "picks the variable \"id\" more than once",
        ),
        (&["--select", "!1-5"], "--select picks no variables"),
        (
            &["--select", "nope"],
            "Selector name 'nope' is not a variable of",
        ),
    ];
    for (args, expected) in cases {
        let out = wrk.path("refused.csv");
        std::fs::write(&out, "keep me").unwrap();
        let mut cmd = wrk.command("readstat");
        cmd.args(args)
            .args(["--output", out.to_str().unwrap()])
            .arg(sample(&wrk, "sav"));
        let stderr = wrk.stderr_on_error(&mut cmd);
        assert!(stderr.contains(expected), "{args:?}: {stderr}");
        // refused before --output is created
        assert_eq!(
            std::fs::read_to_string(&out).unwrap(),
            "keep me",
            "{args:?}"
        );
    }
}

/// Without --sentinels-columns, sentinels are kept for the selected variables
/// only, so an unselected `<name>_null` clash doesn't block them.
#[test]
fn readstat_select_ignores_unselected_sentinel_clash() {
    let wrk = Workdir::new("readstat_select_ignores_unselected_sentinel_clash");
    let f = wrk.load_test_file("readstat_sentinels_suffix.sav");

    let mut cmd = wrk.command("readstat");
    cmd.args(["--sentinels-as", "value", "--select", "other"])
        .arg(&f);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["other", "other_null"],
        svec!["1.0", ""],
        svec!["", "5"],
    ];
    assert_eq!(got, expected);

    let mut cmd = wrk.command("readstat");
    cmd.args(["--select", "other"]).arg(&f);
    let stderr = wrk.stderr_on_success(&mut cmd);
    assert!(
        stderr.contains("1 sentinel (user-defined missing values) in 1 variable (other)"),
        "{stderr}"
    );

    // a clash among the selected variables is still refused
    let mut cmd = wrk.command("readstat");
    cmd.args(["--sentinels-as", "value", "--select", "rating,other"])
        .arg(&f);
    let stderr = wrk.stderr_on_error(&mut cmd);
    assert!(stderr.contains("named \"rating_null\""), "{stderr}");
}

/// SPSS DTIME variables are written as seconds, the unit SPSS stores them in.
/// The readers return them as polars Durations, which the CSV writer can't
/// write, so any file with one failed outright.
///   `readstat_dtime.sav` - written with pyreadstat 1.3.6: `dur` is DTIME23.2,
///     `whole` DTIME11, `t` TIME8. Expected values are pyreadstat's
///     `read_sav(..., disable_datetime_conversion=True)`.
#[test]
fn readstat_spss_dtime_as_seconds() {
    let wrk = Workdir::new("readstat_spss_dtime_as_seconds");
    let f = wrk.load_test_file("readstat_dtime.sav");

    let mut cmd = wrk.command("readstat");
    cmd.arg(&f);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["id", "dur", "whole", "t"],
        svec!["1.0", "3725.0", "60.0", "10:30:15.000000000"],
        svec!["2.0", "1.5", "90061.0", "00:00:01.000000000"],
        svec!["3.0", "", "0.0", ""],
    ];
    assert_eq!(got, expected);

    // they are plain numbers to --compress-numeric
    let mut cmd = wrk.command("readstat");
    cmd.args(["--compress-numeric", "--select", "dur,whole"])
        .arg(&f);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["dur", "whole"],
        svec!["3725.0", "60"],
        svec!["1.5", "90061"],
        svec!["", "0"],
    ];
    assert_eq!(got, expected);
}

/// Convert `file` with `flags` & `--dictionary`, and return the dictionary.
fn dictionary_of(wrk: &Workdir, file: &str, flags: &[&str]) -> serde_json::Value {
    let dict = wrk.path("dict.schema.json");
    let mut cmd = wrk.command("readstat");
    cmd.args(flags)
        .args([
            "--dictionary",
            dict.to_str().unwrap(),
            "--output",
            "out.csv",
        ])
        .arg(file);
    wrk.assert_success(&mut cmd);
    serde_json::from_str(&std::fs::read_to_string(dict).unwrap()).unwrap()
}

#[test]
fn readstat_dictionary_spss() {
    let wrk = Workdir::new("readstat_dictionary_spss");
    let dict = dictionary_of(&wrk, &sample(&wrk, "sav"), &[]);
    assert_eq!(
        dict["$schema"],
        "https://json-schema.org/draft/2020-12/schema"
    );
    assert_eq!(dict["additionalProperties"], false);
    assert_eq!(
        dict["required"],
        serde_json::json!(["id", "name", "score", "sex", "visited"])
    );
    let props = &dict["properties"];
    // variable labels, as pyreadstat's column_names_to_labels gives them
    for (name, label) in [
        ("id", "Record id"),
        ("name", "Given name"),
        ("score", "Test score"),
        ("sex", "Respondent sex"),
        ("visited", "Date of visit"),
    ] {
        assert_eq!(props[name]["title"], label, "{name}");
        assert_eq!(props[name]["description"], format!("{label} ({name})"));
    }
    assert_eq!(props["id"]["type"], serde_json::json!(["number"]));
    assert_eq!(
        props["score"]["type"],
        serde_json::json!(["number", "null"])
    );
    assert_eq!(props["visited"]["format"], "date");
    assert_eq!(props["visited"]["x-qsv"]["qsv_type"], "Date");
    // sex is fully labeled (1 Male, 2 Female), so its codes are its enum
    assert_eq!(props["sex"]["enum"], serde_json::json!([1.0, 2.0, null]));
    assert_eq!(
        props["sex"]["x-qsv"]["value_labels"],
        serde_json::json!([{"value": 1, "label": "Male"}, {"value": 2, "label": "Female"}])
    );
    assert!(props["score"].get("enum").is_none());
    let top = &dict["x-qsv"];
    assert_eq!(top["source_format"], "SPSS (.sav)");
    assert_eq!(top["row_count"], 4);
    assert!(
        top["generated_by"]
            .as_str()
            .unwrap()
            .starts_with("Generated by qsv v")
    );

    // decoded, the enum is the labels
    let dict = dictionary_of(&wrk, &sample(&wrk, "sav"), &["--value-labels"]);
    let sex = &dict["properties"]["sex"];
    assert_eq!(sex["type"], serde_json::json!(["string", "null"]));
    assert_eq!(sex["enum"], serde_json::json!(["Male", "Female", null]));
    // label text is not a sentinel: only --sentinels-embedded columns have null_values
    assert!(sex["x-qsv"].get("null_values").is_none(), "{sex}");
    assert_eq!(dict["x-qsv"]["readstat_flags"]["value_labels"], true);
}

#[test]
fn readstat_dictionary_partial_labels_have_no_enum() {
    let wrk = Workdir::new("readstat_dictionary_partial_labels_have_no_enum");
    let f = wrk.load_test_file("readstat_partial_labels.sav");
    for flags in [&[][..], &["--value-labels"][..]] {
        let dict = dictionary_of(&wrk, &f, flags);
        for name in ["q", "r"] {
            let prop = &dict["properties"][name];
            // values outside the labels are written too, so an enum would
            // make validate reject the data
            assert!(prop.get("enum").is_none(), "{flags:?} {name}: {prop}");
            assert!(prop["x-qsv"]["value_labels"].is_array(), "{flags:?} {name}");
        }
    }
}

#[test]
fn readstat_dictionary_spss_missing_values_and_sentinel_columns() {
    let wrk = Workdir::new("readstat_dictionary_spss_missing_values_and_sentinel_columns");
    let f = sentinels(&wrk, "sav");
    let dict = dictionary_of(&wrk, &f, &["--sentinels-as", "value"]);
    let props = &dict["properties"];
    // pyreadstat missing_ranges: income 900-999 & 99, rating 8 & 9, name "NA"
    assert_eq!(
        props["income"]["x-qsv"]["missing_values"],
        serde_json::json!({"range": [900, 999], "discrete": [99]})
    );
    assert_eq!(
        props["rating"]["x-qsv"]["missing_values"],
        serde_json::json!({"discrete": [8, 9]})
    );
    assert_eq!(
        props["name"]["x-qsv"]["missing_values"],
        serde_json::json!({"strings": ["NA"]})
    );
    // 8 is labeled "Refused" but declared missing, so it is never a value
    assert_eq!(props["rating"]["enum"], serde_json::json!([3.0, 4.0, null]));
    let ind = &props["rating_null"];
    assert_eq!(ind["x-qsv"]["indicator_for"], "rating");
    assert_eq!(ind["type"], serde_json::json!(["string", "null"]));
    assert_eq!(ind["enum"], serde_json::json!(["8", "9", null]));
    assert_eq!(
        dict["required"],
        serde_json::json!([
            "id",
            "income",
            "income_null",
            "rating",
            "rating_null",
            "name",
            "name_null",
            "plain"
        ])
    );

    // embedded, the sentinels are null_values: literals present in the column
    let dict = dictionary_of(
        &wrk,
        &f,
        &["--sentinels-as", "value", "--sentinels-embedded"],
    );
    let props = &dict["properties"];
    assert_eq!(
        props["income"]["x-qsv"]["null_values"],
        serde_json::json!(["950", "99"])
    );
    assert_eq!(
        props["name"]["x-qsv"]["null_values"],
        serde_json::json!(["NA"])
    );
    assert!(props["plain"]["x-qsv"].get("null_values").is_none());
    // not embedded, sentinels are empty cells: no null_values
    let dict = dictionary_of(&wrk, &f, &[]);
    assert!(
        dict["properties"]["income"]["x-qsv"]
            .get("null_values")
            .is_none()
    );
}

#[test]
fn readstat_dictionary_stata_and_sas_labels() {
    let wrk = Workdir::new("readstat_dictionary_stata_and_sas_labels");
    // Stata sentinels keep their tag in the value labels
    let dict = dictionary_of(
        &wrk,
        &wrk.load_test_file("readstat_stata_numeric_labels.dta"),
        &[],
    );
    assert_eq!(
        dict["properties"]["x"]["x-qsv"]["value_labels"],
        serde_json::json!([
            {"value": 1, "label": "001.0"},
            {"value": ".a", "label": "Refused"}
        ])
    );

    // SAS takes its labels from the format catalog; codes may be text
    let f = wrk.load_test_file("readstat_catalog.sas7bdat");
    wrk.load_test_file("readstat_catalog.sas7bcat");
    let dict = dictionary_of(&wrk, &f, &["--value-labels"]);
    let props = &dict["properties"];
    assert_eq!(props["workshop"]["enum"], serde_json::json!(["R", "SAS"]));
    assert_eq!(
        props["gender"]["x-qsv"]["value_labels"],
        serde_json::json!([
            {"value": "f", "label": "Female"},
            {"value": "m", "label": "Male"}
        ])
    );
    assert_eq!(props["q1"]["title"], "The instructor was well prepared");
    assert_eq!(props["workshop"]["x-qsv"]["source_format"], "WORKSHOP");
    assert_eq!(dict["x-qsv"]["table_name"], "HADLEY");
    assert_eq!(
        dict["x-qsv"]["readstat_flags"]["sas7bcat"],
        "readstat_catalog.sas7bcat"
    );
}

#[test]
fn readstat_dictionary_follows_the_options() {
    let wrk = Workdir::new("readstat_dictionary_follows_the_options");
    // --compress-numeric makes whole-number variables integers
    let dict = dictionary_of(&wrk, &sample(&wrk, "sav"), &["--compress-numeric"]);
    assert_eq!(
        dict["properties"]["id"]["type"],
        serde_json::json!(["integer"])
    );
    assert_eq!(dict["properties"]["id"]["x-qsv"]["qsv_type"], "Integer");
    assert_eq!(
        dict["properties"]["sex"]["enum"],
        serde_json::json!([1, 2, null])
    );

    // --select & --sample shape it, & are recorded
    let dict = dictionary_of(
        &wrk,
        &sample(&wrk, "sav"),
        &["--select", "sex,id", "--sample", "2", "--seed", "3"],
    );
    assert_eq!(dict["required"], serde_json::json!(["sex", "id"]));
    assert_eq!(dict["x-qsv"]["row_count"], 2);
    let flags = &dict["x-qsv"]["readstat_flags"];
    assert_eq!(flags["select"], serde_json::json!(["sex", "id"]));
    assert_eq!(flags["seed"], 3);

    // .por stores a missing string as "", an empty cell all the same
    let dict = dictionary_of(&wrk, &sample(&wrk, "por"), &[]);
    assert_eq!(
        dict["properties"]["NAME"]["type"],
        serde_json::json!(["string", "null"])
    );
}

/// Every dictionary validates the CSV it was written with.
#[test]
fn readstat_dictionary_validates_its_csv() {
    let wrk = Workdir::new("readstat_dictionary_validates_its_csv");
    wrk.load_test_file("readstat_catalog.sas7bcat");
    let cases: [(&str, &[&str]); 20] = [
        ("readstat_sample.sav", &[]),
        (
            "readstat_sentinels.sav",
            &[
                "--compress-numeric",
                "--sentinels-as",
                "value",
                "--sentinels-embedded",
                "--limit",
                "1",
            ],
        ),
        (
            "readstat_sentinels_labels.sav",
            &[
                "--value-labels",
                "--sentinels-as",
                "label",
                "--sentinels-embedded",
            ],
        ),
        ("readstat_measure.sav", &[]),
        (
            "readstat_sentinels.sav",
            &["--sentinels-as", "label", "--value-labels"],
        ),
        (
            "readstat_sample.sav",
            &["--value-labels", "--compress-numeric"],
        ),
        ("readstat_sample.dta", &["--value-labels"]),
        ("readstat_sample.xpt", &[]),
        ("readstat_sample.por", &[]),
        ("readstat_sample.sas7bdat", &[]),
        ("readstat_partial_labels.sav", &[]),
        ("readstat_sentinels.sav", &["--sentinels-as", "value"]),
        ("readstat_sentinels.sav", &["--sentinels-as", "label"]),
        (
            "readstat_sentinels.sav",
            &["--sentinels-as", "value", "--sentinels-embedded"],
        ),
        (
            "readstat_sentinels.sas7bdat",
            &["--sentinels-as", "value", "--sentinels-embedded"],
        ),
        ("readstat_stata_numeric_labels.dta", &["--value-labels"]),
        ("readstat_stata_extmiss.dta", &["--sentinels-as", "value"]),
        ("readstat_catalog.sas7bdat", &["--value-labels"]),
        ("readstat_dtime.sav", &["--compress-numeric"]),
        (
            "readstat_sentinels_labels.sav",
            &["--sentinels-as", "label"],
        ),
    ];
    for (file, flags) in cases {
        let f = wrk.load_test_file(file);
        dictionary_of(&wrk, &f, flags);
        let mut cmd = wrk.command("validate");
        cmd.args(["out.csv", "dict.schema.json"]);
        let got = wrk.output(&mut cmd);
        assert!(
            got.status.success(),
            "{file} {flags:?}: {}",
            String::from_utf8_lossy(&got.stderr)
        );
    }
}

#[test]
fn readstat_dictionary_refusals() {
    let wrk = Workdir::new("readstat_dictionary_refusals");
    let f = sample(&wrk, "sav");
    let input_before = std::fs::read(&f).unwrap();
    let cases: [(&[&str], &str); 3] = [
        (
            &["--metadata", "json", "--dictionary", "d.json"],
            "doesn't apply to --metadata",
        ),
        (
            &["--dictionary", "out.csv", "--output", "out.csv"],
            "--dictionary & --output name the same file",
        ),
        (
            &["--dictionary", f.as_str()],
            "would overwrite the input file",
        ),
    ];
    // an absolute & a relative spelling, before either file exists
    let abs = wrk.path("fresh.csv");
    let mut cmd = wrk.command("readstat");
    cmd.args([
        "--dictionary",
        abs.to_str().unwrap(),
        "--output",
        "fresh.csv",
    ])
    .arg(&f);
    let stderr = wrk.stderr_on_error(&mut cmd);
    assert!(stderr.contains("name the same file"), "{stderr}");
    assert!(!abs.exists(), "refused before --output is created");

    // the SAS format catalog being read is never overwritten
    let data = wrk.load_test_file("readstat_catalog.sas7bdat");
    let catalog = wrk.load_test_file("readstat_catalog.sas7bcat");
    let catalog_before = std::fs::read(&catalog).unwrap();
    let mut cmd = wrk.command("readstat");
    cmd.args([
        "--value-labels",
        "--dictionary",
        catalog.as_str(),
        "--output",
        "c.csv",
    ])
    .arg(&data);
    let stderr = wrk.stderr_on_error(&mut cmd);
    assert!(
        stderr.contains("is the SAS format catalog being read"),
        "{stderr}"
    );
    assert_eq!(std::fs::read(&catalog).unwrap(), catalog_before);

    // ./same.csv & same.csv, before either exists
    let mut cmd = wrk.command("readstat");
    cmd.args(["--dictionary", "./same.csv", "--output", "same.csv"])
        .arg(&f);
    let stderr = wrk.stderr_on_error(&mut cmd);
    assert!(stderr.contains("name the same file"), "{stderr}");
    assert!(!wrk.path("same.csv").exists());

    // a link to an --output not yet written resolves only once it is: caught
    // then, before the dictionary truncates the CSV
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink("linked.csv", wrk.path("link.json")).unwrap();
        let mut cmd = wrk.command("readstat");
        cmd.args(["--dictionary", "link.json", "--output", "linked.csv"])
            .arg(&f);
        let stderr = wrk.stderr_on_error(&mut cmd);
        assert!(stderr.contains("name the same file"), "{stderr}");
        assert!(
            std::fs::read_to_string(wrk.path("linked.csv"))
                .unwrap()
                .starts_with("id,name,score,sex,visited"),
            "the CSV written must survive"
        );
    }
    for (args, expected) in cases {
        let mut cmd = wrk.command("readstat");
        cmd.args(args).arg(&f);
        let stderr = wrk.stderr_on_error(&mut cmd);
        assert!(stderr.contains(expected), "{args:?}: {stderr}");
    }
    assert_eq!(std::fs::read(&f).unwrap(), input_before);
}

/// SPSS records its formats as type codes; the dictionary names them as SPSS
/// does, and maps SPSS's measure to the role viz charts by.
#[test]
fn readstat_dictionary_spss_formats_and_measures() {
    let wrk = Workdir::new("readstat_dictionary_spss_formats_and_measures");
    // pyreadstat original_variable_types: F8.2, DTIME23.2, DTIME11, TIME8
    let dict = dictionary_of(&wrk, &wrk.load_test_file("readstat_dtime.sav"), &[]);
    for (name, format) in [
        ("id", "F8.2"),
        ("dur", "DTIME23.2"),
        ("whole", "DTIME11"),
        ("t", "TIME8"),
    ] {
        assert_eq!(
            dict["properties"][name]["x-qsv"]["source_format"], format,
            "{name}"
        );
    }
    // ... A6 & DATE11, likewise
    let dict = dictionary_of(&wrk, &sample(&wrk, "sav"), &[]);
    assert_eq!(dict["properties"]["name"]["x-qsv"]["source_format"], "A6");
    assert_eq!(
        dict["properties"]["visited"]["x-qsv"]["source_format"],
        "DATE11"
    );

    // readstat_measure.sav - written with pyreadstat 1.3.6, variable_measure
    // a scale, b nominal, c ordinal, d unset
    let dict = dictionary_of(&wrk, &wrk.load_test_file("readstat_measure.sav"), &[]);
    let props = &dict["properties"];
    for (name, measure, role) in [
        ("a", "Scale", "measure"),
        ("b", "Nominal", "dimension"),
        ("c", "Ordinal", "dimension"),
    ] {
        assert_eq!(props[name]["x-qsv"]["measure"], measure, "{name}");
        assert_eq!(props[name]["x-qsv"]["role"], role, "{name}");
    }
    assert!(props["d"]["x-qsv"].get("role").is_none());
    assert!(props["d"]["x-qsv"].get("measure").is_none());
}

/// Under --sentinels-embedded, `null_values` lists the sentinels a column
/// holds, told apart by the variable's metadata: not decoded labels, & not
/// lost in a column of many values.
#[test]
fn readstat_dictionary_embedded_null_values() {
    let wrk = Workdir::new("readstat_dictionary_embedded_null_values");
    // readstat_sentinels_labels.sav: score declares 950 ("Skipped") & 99
    // ("1.0"), labels 1 "one"; code declares "NA" ("Not asked")
    let dict = dictionary_of(
        &wrk,
        &wrk.load_test_file("readstat_sentinels_labels.sav"),
        &[
            "--value-labels",
            "--sentinels-as",
            "label",
            "--sentinels-embedded",
        ],
    );
    let props = &dict["properties"];
    assert_eq!(
        props["score"]["x-qsv"]["null_values"],
        serde_json::json!(["1.0", "Skipped"])
    );
    assert_eq!(
        props["code"]["x-qsv"]["null_values"],
        serde_json::json!(["Not asked"])
    );

    // y holds over 1000 distinct values, beyond what enum tracking keeps
    let dict = dictionary_of(
        &wrk,
        &sentinels(&wrk, "sas7bdat"),
        &["--sentinels-as", "value", "--sentinels-embedded"],
    );
    let y = &dict["properties"]["y"]["x-qsv"];
    assert!(y.get("cardinality").is_none(), "{y}");
    let tags = y["null_values"].as_array().unwrap();
    assert!(
        tags.len() > 5 && tags.iter().all(|t| t.as_str().unwrap().starts_with('.')),
        "{y}"
    );
    // x is numeric, so it can't hold a sentinel
    assert!(
        dict["properties"]["x"]["x-qsv"]
            .get("null_values")
            .is_none()
    );
}

/// A numeric enum only where the CSV holds the codes as numbers.
#[test]
fn readstat_dictionary_numeric_enum_only_as_written() {
    let wrk = Workdir::new("readstat_dictionary_numeric_enum_only_as_written");
    // embedded, rating is text: "3", never the number 3
    let dict = dictionary_of(
        &wrk,
        &sentinels(&wrk, "sav"),
        &[
            "--compress-numeric",
            "--sentinels-as",
            "value",
            "--sentinels-embedded",
            "--limit",
            "1",
        ],
    );
    assert!(dict["properties"]["rating"].get("enum").is_none());

    // readstat_measure.sav: e labels 1.25 & 2.5
    let f = wrk.load_test_file("readstat_measure.sav");
    let dict = dictionary_of(&wrk, &f, &[]);
    assert_eq!(
        dict["properties"]["e"]["enum"],
        serde_json::json!([1.25, 2.5])
    );
    // rounded on output, they may no longer equal their codes
    let out = wrk.path("dict.schema.json");
    let mut cmd = wrk.command("readstat");
    cmd.env("QSV_POLARS_FLOAT_PRECISION", "1")
        .args(["--dictionary", out.to_str().unwrap(), "--output", "out.csv"])
        .arg(&f);
    wrk.assert_success(&mut cmd);
    let dict: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(out).unwrap()).unwrap();
    assert!(dict["properties"]["e"].get("enum").is_none());
}

#[test]
fn readstat_dictionary_sentinel_edge_cases() {
    let wrk = Workdir::new("readstat_dictionary_sentinel_edge_cases");
    // --sentinels-as is case-insensitive, & so is its effect on the dictionary
    let labels = wrk.load_test_file("readstat_sentinels_labels.sav");
    let dict = dictionary_of(
        &wrk,
        &labels,
        &["--sentinels-as", "LABEL", "--sentinels-embedded"],
    );
    assert_eq!(
        dict["properties"]["code"]["x-qsv"]["null_values"],
        serde_json::json!(["Not asked"])
    );
    assert_eq!(dict["x-qsv"]["readstat_flags"]["sentinels_as"], "label");

    // readstat_sentinels_edge.sav - written with pyreadstat 1.3.6: r declares
    // the range -1e9 to -0.5, & 1498 of its 1500 values fall in it
    let dict = dictionary_of(
        &wrk,
        &wrk.load_test_file("readstat_sentinels_edge.sav"),
        &["--sentinels-as", "value", "--sentinels-embedded"],
    );
    let r = &dict["properties"]["r"]["x-qsv"];
    assert_eq!(r["null_values"].as_array().unwrap().len(), 1000, "capped");
    assert_eq!(r["null_values_truncated"], true);
    let p = &dict["properties"]["p"]["x-qsv"];
    assert_eq!(p["null_values"], serde_json::json!(["9"]));
    assert!(p.get("null_values_truncated").is_none());
}
