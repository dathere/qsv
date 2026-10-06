use serde_json::Value;

use crate::workdir::Workdir;

// Round trips: `readstat --dictionary` turns a fixture into a CSV + data
// dictionary, `writestat` writes them back as a binary file, and `readstat`
// reads that file again. The fixtures' CSVs are pinned value-for-value against
// pyreadstat in test_readstat.rs, so a final CSV identical to the first one
// means writestat put back exactly what readstat took out.

/// `readstat <flags> --dictionary dict.json <fixture> -o orig.csv`; returns
/// the CSV.
fn to_csv(wrk: &Workdir, fixture: &str, flags: &[&str]) -> String {
    let f = wrk.load_test_file(fixture);
    let mut cmd = wrk.command("readstat");
    cmd.args(flags)
        .args(["--dictionary", "dict.json"])
        .arg(f)
        .args(["-o", "orig.csv"]);
    wrk.assert_success(&mut cmd);
    wrk.read_to_string("orig.csv").unwrap()
}

/// `writestat --dictionary dict.json orig.csv -o <out> <extra>`
fn writestat(wrk: &Workdir, out: &str, extra: &[&str]) {
    let mut cmd = wrk.command("writestat");
    cmd.args(["--dictionary", "dict.json", "orig.csv", "-o", out])
        .args(extra);
    wrk.assert_success(&mut cmd);
}

/// `readstat <flags> <file> -o back.csv`; returns the CSV.
fn read_back(wrk: &Workdir, file: &str, flags: &[&str]) -> String {
    let mut cmd = wrk.command("readstat");
    cmd.args(flags).arg(file).args(["-o", "back.csv"]);
    wrk.assert_success(&mut cmd);
    wrk.read_to_string("back.csv").unwrap()
}

fn metadata(wrk: &Workdir, file: &str) -> Value {
    let mut cmd = wrk.command("readstat");
    cmd.args(["--metadata", "json", file]);
    serde_json::from_str(&wrk.stdout::<String>(&mut cmd)).unwrap()
}

/// Each variable's name & the given metadata keys - not offsets or
/// timestamps, which differ by construction.
fn vars(meta: &Value, keys: &[&str]) -> Vec<Value> {
    let list = meta
        .get("variables")
        .or_else(|| meta.get("columns"))
        .and_then(Value::as_array)
        .expect("metadata lists the variables");
    list.iter()
        .map(|v| {
            let mut o = serde_json::Map::new();
            o.insert("name".into(), v["name"].clone());
            for k in keys {
                o.insert((*k).into(), v.get(*k).cloned().unwrap_or(Value::Null));
            }
            Value::Object(o)
        })
        .collect()
}

fn var<'a>(meta: &'a Value, name: &str) -> &'a Value {
    meta.get("variables")
        .or_else(|| meta.get("columns"))
        .and_then(Value::as_array)
        .and_then(|l| l.iter().find(|v| v["name"] == name))
        .unwrap_or_else(|| panic!("no variable {name}"))
}

const SAV_KEYS: &[&str] = &[
    "type",
    "label",
    "value_labels",
    "missing_doubles",
    "missing_range",
    "missing_strings",
    "measure",
    "alignment",
    "display_width",
    "format_type",
    "format_width",
    "format_decimals",
];

/// An SPSS fixture -> CSV -> .sav -> CSV, both CSVs & the metadata identical.
fn sav_round_trip(name: &str, fixture: &str, flags: &[&str]) -> (Workdir, String) {
    let wrk = Workdir::new(name);
    let orig = to_csv(&wrk, fixture, flags);
    writestat(&wrk, "rt.sav", &[]);
    assert_eq!(read_back(&wrk, "rt.sav", flags), orig);
    let fixture_path = wrk.path(fixture).to_string_lossy().to_string();
    assert_eq!(
        vars(&metadata(&wrk, "rt.sav"), SAV_KEYS),
        vars(&metadata(&wrk, &fixture_path), SAV_KEYS)
    );
    (wrk, orig)
}

#[test]
fn writestat_sav_round_trip() {
    let (wrk, orig) = sav_round_trip("writestat_sav_round_trip", "readstat_sample.sav", &[]);
    assert_eq!(
        orig,
        "id,name,score,sex,visited\n1.0,Ana,1.5,1.0,2020-01-31\n2.0,Bo,-2.25,2.0,1999-12-31\n3.0,\
         Chloë,,1.0,2026-03-01\n4.0,,0.0,,1960-01-01\n"
    );
    let meta = metadata(&wrk, "rt.sav");
    assert_eq!(var(&meta, "id")["label"], "Record id");
    assert_eq!(
        var(&meta, "sex")["value_labels"],
        serde_json::json!({"1": "Male", "2": "Female"})
    );
    // DATE11
    assert_eq!(var(&meta, "visited")["format_type"], 20);
    assert_eq!(var(&meta, "visited")["format_width"], 11);
}

// --value-labels wrote "Male"/"Female": they go back to their codes
#[test]
fn writestat_sav_value_labels_back_to_codes() {
    let (_wrk, orig) = sav_round_trip(
        "writestat_sav_value_labels_back_to_codes",
        "readstat_sample.sav",
        &["--value-labels"],
    );
    assert!(orig.contains(",Male,") && orig.contains(",Female,"));
}

#[test]
fn writestat_sav_partial_value_labels() {
    sav_round_trip(
        "writestat_sav_partial_value_labels",
        "readstat_partial_labels.sav",
        &["--value-labels"],
    );
}

// DTIME durations & TIME/MTIME formats keep their format & values
#[test]
fn writestat_sav_durations() {
    sav_round_trip("writestat_sav_durations", "readstat_dtime.sav", &[]);
}

#[test]
fn writestat_sav_measure_alignment_width() {
    let (wrk, _) = sav_round_trip(
        "writestat_sav_measure_alignment_width",
        "readstat_measure.sav",
        &[],
    );
    let meta = metadata(&wrk, "rt.sav");
    let measures: Vec<&Value> = meta["variables"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| &v["measure"])
        .collect();
    assert!(measures.iter().any(|m| *m != "Unknown"));
}

// Every way readstat writes SPSS sentinels becomes declared missing values
// again, the text variable's "NA" included.
#[test]
fn writestat_sav_sentinels() {
    for (i, flags) in [
        &["--sentinels-as", "value"][..],
        &["--sentinels-as", "label"],
        &["--sentinels-as", "value", "--sentinels-embedded"],
        &["--sentinels-as", "label", "--sentinels-embedded"],
        &["--sentinels-as", "label", "--sentinels-columns", "name"],
    ]
    .into_iter()
    .enumerate()
    {
        let (wrk, orig) = sav_round_trip(
            &format!("writestat_sav_sentinels_{i}"),
            "readstat_sentinels.sav",
            flags,
        );
        assert!(orig.contains("NA"), "{flags:?}: {orig}");
        let meta = metadata(&wrk, "rt.sav");
        assert_eq!(
            var(&meta, "name")["missing_strings"],
            serde_json::json!(["NA"]),
            "{flags:?}"
        );
        assert_eq!(
            var(&meta, "income")["missing_doubles"],
            serde_json::json!([900.0, 999.0, 99.0]),
            "{flags:?}"
        );
    }
}

#[test]
fn writestat_sav_compress() {
    let wrk = Workdir::new("writestat_sav_compress");
    let orig = to_csv(&wrk, "readstat_sample.sav", &[]);
    writestat(&wrk, "rt.sav", &["--compress"]);
    assert_eq!(read_back(&wrk, "rt.sav", &[]), orig);
    assert_eq!(metadata(&wrk, "rt.sav")["compression"], "RLE");
}

// SPSS can't label text codes: refused, & --lossy writes the file without them
#[test]
fn writestat_sav_text_value_labels_refused() {
    let wrk = Workdir::new("writestat_sav_text_value_labels_refused");
    to_csv(&wrk, "readstat_sentinels_labels.sav", &[]);
    let mut cmd = wrk.command("writestat");
    cmd.args(["--dictionary", "dict.json", "orig.csv", "-o", "rt.sav"]);
    let err = wrk.stderr_on_error(&mut cmd);
    assert!(
        err.contains("the value labels of \"code\": its codes are text") && err.contains("--lossy"),
        "{err}"
    );
    assert!(!wrk.path("rt.sav").exists());

    let mut cmd = wrk.command("writestat");
    cmd.args([
        "--lossy",
        "--dictionary",
        "dict.json",
        "orig.csv",
        "-o",
        "rt.sav",
    ]);
    let warn = wrk.stderr_on_success(&mut cmd);
    assert!(warn.contains("--lossy: left out"), "{warn}");
    assert_eq!(
        var(&metadata(&wrk, "rt.sav"), "code")["value_labels"],
        Value::Null
    );
}

#[test]
fn writestat_sav_invalid_names_refused() {
    let wrk = Workdir::new("writestat_sav_invalid_names_refused");
    // reserved words in any case, as SPSS syntax is case-insensitive
    wrk.create_from_string(
        "in.csv",
        "a b,1x,WITH,to,ok_name,a.b#$@,éa,toll\n1,2,3,4,5,6,7,8\n",
    );
    let mut cmd = wrk.command("writestat");
    cmd.args(["in.csv", "-o", "out.sav"]);
    let err = wrk.stderr_on_error(&mut cmd);
    assert!(
        err.contains("\"a b\", \"1x\", \"WITH\", \"to\".") && !err.contains("ok_name"),
        "{err}"
    );

    wrk.create_from_string("ok.csv", "ok_name,a.b#$@,éa,toll\n1,2,3,4\n");
    let mut cmd = wrk.command("writestat");
    cmd.args(["ok.csv", "-o", "ok.sav"]);
    wrk.assert_success(&mut cmd);
}

// SPSS portable files follow the same rules: the writer would otherwise
// rename "a b" to "A_B" silently, & keep "1x"
#[test]
fn writestat_por_invalid_names_refused() {
    let wrk = Workdir::new("writestat_por_invalid_names_refused");
    wrk.create_from_string("in.csv", "a b,1x,ok\n1,2,3\n");
    let mut cmd = wrk.command("writestat");
    cmd.args(["in.csv", "-o", "out.por"]);
    let err = wrk.stderr_on_error(&mut cmd);
    assert!(
        err.contains("aren't valid SPSS variable names: \"a b\", \"1x\"."),
        "{err}"
    );
}

// Written by readstat with --sentinels-as value: the codes are in the CSV, so
// the value 1.0 stays 1.0 though the sentinel 99 is labeled "1.0".
#[test]
fn writestat_sav_embedded_codes_not_labels() {
    let wrk = Workdir::new("writestat_sav_embedded_codes_not_labels");
    let flags = [
        "--select",
        "score",
        "--sentinels-as",
        "value",
        "--sentinels-embedded",
    ];
    let orig = to_csv(&wrk, "readstat_sentinels_labels.sav", &flags);
    assert_eq!(orig, "score\n1.0\n2.0\n950\n99\n");
    writestat(&wrk, "rt.sav", &[]);
    assert_eq!(read_back(&wrk, "rt.sav", &flags), orig);
}

/// writestat refuses the CSV: the code a label stands for can't be told.
fn refused_as_ambiguous(wrk: &Workdir, out: &str, expected: &str) {
    for lossy in [false, true] {
        let mut cmd = wrk.command("writestat");
        cmd.args(["--dictionary", "dict.json", "orig.csv", "-o", out]);
        if lossy {
            cmd.arg("--lossy");
        }
        let err = wrk.stderr_on_error(&mut cmd);
        assert!(
            err.contains(expected) && err.contains("Write the CSV again with 'qsv readstat'"),
            "{err}"
        );
    }
    assert!(!wrk.path(out).exists());
}

// codes 1 & 2 are both labeled "Yes" (written with pyreadstat 1.3.6)
#[test]
fn writestat_duplicate_labels_refused() {
    let wrk = Workdir::new("writestat_duplicate_labels_refused");
    let orig = to_csv(&wrk, "writestat_dup_labels.sav", &["--value-labels"]);
    assert_eq!(orig, "q\nYes\nYes\nNo\n");
    refused_as_ambiguous(
        &wrk,
        "rt.sav",
        "Column \"q\" holds \"Yes\", which is the label of 1, 2,",
    );
}

// Embedded sentinel labels that read as numbers: .b's "1.0" can't be told
// from the value 1.0, so the CSV is refused, even with --lossy.
#[test]
fn writestat_numeric_sentinel_labels_refused() {
    let wrk = Workdir::new("writestat_numeric_sentinel_labels_refused");
    to_csv(
        &wrk,
        "readstat_stata_numeric_labels.dta",
        &["--sentinels-as", "label", "--sentinels-embedded"],
    );
    refused_as_ambiguous(
        &wrk,
        "rt.dta",
        "Column \"v\" holds \"1.0\", which is both a number and the label of .b,",
    );
}

// An embedded label of an SPSS string sentinel goes back to its code, & so
// is a declared missing value again.
#[test]
fn writestat_sav_embedded_string_sentinel_label() {
    let wrk = Workdir::new("writestat_sav_embedded_string_sentinel_label");
    let flags = [
        "--sentinels-as",
        "label",
        "--sentinels-embedded",
        "--sentinels-columns",
        "code",
    ];
    let orig = to_csv(&wrk, "readstat_sentinels_labels.sav", &flags);
    assert!(orig.contains(",Not asked\n"), "{orig}");
    // its text value labels can't be written to an SPSS file, so the
    // sentinel comes back as its code
    writestat(&wrk, "rt.sav", &["--lossy"]);
    assert_eq!(
        read_back(&wrk, "rt.sav", &flags),
        orig.replace("Not asked", "NA")
    );
    assert_eq!(
        read_back(&wrk, "rt.sav", &[]),
        "score,code\n1.0,a\n2.0,\n,b\n,a\n"
    );
}

/// A datetime with microseconds.
fn microsecond_datetime(wrk: &Workdir) {
    wrk.create_from_string(
        "dict.json",
        r#"{"properties": {"ts": {"x-qsv": {"qsv_type": "DateTime"}}}}"#,
    );
    wrk.create_from_string(
        "orig.csv",
        "ts\n2024-01-02T03:04:05.123456\n2024-01-02T03:04:06\n",
    );
}

// SAS transport files hold microseconds
#[test]
fn writestat_xpt_datetime_microseconds() {
    let wrk = Workdir::new("writestat_xpt_datetime_microseconds");
    microsecond_datetime(&wrk);
    writestat(&wrk, "out.xpt", &[]);
    assert_eq!(
        read_back(&wrk, "out.xpt", &[]),
        "ts\n2024-01-02T03:04:05.123456\n2024-01-02T03:04:06.000000\n"
    );
}

// Stata files hold milliseconds, & both SPSS writers whole seconds
#[test]
fn writestat_datetime_precision_refused() {
    let wrk = Workdir::new("writestat_datetime_precision_refused");
    microsecond_datetime(&wrk);
    for (out, what) in [
        (
            "out.sav",
            "the fractions of a second of the datetimes in \"ts\"",
        ),
        ("out.dta", "the microseconds of the datetimes in \"ts\""),
        (
            "out.por",
            "the fractions of a second of the datetimes in \"ts\"",
        ),
    ] {
        let mut cmd = wrk.command("writestat");
        cmd.args(["--dictionary", "dict.json", "orig.csv", "-o", out]);
        let err = wrk.stderr_on_error(&mut cmd);
        assert!(err.contains(what), "{out}: {err}");
    }
    let mut cmd = wrk.command("writestat");
    cmd.args([
        "--lossy",
        "--dictionary",
        "dict.json",
        "orig.csv",
        "-o",
        "out.por",
    ]);
    wrk.assert_success(&mut cmd);
    assert_eq!(
        read_back(&wrk, "out.por", &[]),
        "TS\n2024-01-02T03:04:05.000\n2024-01-02T03:04:06.000\n"
    );
}

// Inferred datetimes are microseconds, which the SPSS portable writer would
// take for milliseconds.
#[test]
fn writestat_por_inferred_datetime() {
    let wrk = Workdir::new("writestat_por_inferred_datetime");
    wrk.create_from_string("in.csv", "ts\n2024-01-02T03:04:05\n");
    let mut cmd = wrk.command("writestat");
    cmd.args(["in.csv", "-o", "out.por"]);
    wrk.assert_success(&mut cmd);
    assert_eq!(
        read_back(&wrk, "out.por", &[]),
        "TS\n2024-01-02T03:04:05.000\n"
    );
}

// A 60-byte SPSS name is valid, though its <name>_null column's 65 bytes
// aren't: that column becomes the variable's missing values, not a variable.
#[test]
fn writestat_sav_long_name_with_sentinels() {
    let (_wrk, orig) = sav_round_trip(
        "writestat_sav_long_name_with_sentinels",
        "writestat_long_name.sav",
        &["--sentinels-as", "value"],
    );
    assert!(
        orig.starts_with(&format!("{0},{0}_null\n", "v".repeat(60))),
        "{orig}"
    );
}

// Text sentinels, written with pyreadstat 1.3.6: `s` declares NA & XX
// missing, & labels NA "XX". Under --sentinels-as value the <name>_null
// column holds codes, so its XX stays XX.
// (Text value labels can't be written to an SPSS file, hence --lossy.)
#[test]
fn writestat_sav_sentinel_codes_not_labels() {
    let wrk = Workdir::new("writestat_sav_sentinel_codes_not_labels");
    let flags = ["--sentinels-as", "value"];
    let orig = to_csv(&wrk, "writestat_sentinel_label_is_code.sav", &flags);
    assert_eq!(orig, "s,s_null\na,\n,NA\n,XX\nb,\n");
    writestat(&wrk, "rt.sav", &["--lossy"]);
    assert_eq!(read_back(&wrk, "rt.sav", &flags), orig);
    assert_eq!(
        var(&metadata(&wrk, "rt.sav"), "s")["missing_strings"],
        serde_json::json!(["NA", "XX"])
    );
}

// NA & XX are both labeled "Refused"
#[test]
fn writestat_sav_sentinels_sharing_a_label_refused() {
    let wrk = Workdir::new("writestat_sav_sentinels_sharing_a_label_refused");
    let orig = to_csv(
        &wrk,
        "writestat_sentinels_share_label.sav",
        &["--sentinels-as", "label"],
    );
    assert_eq!(orig, "s,s_null\na,\n,Refused\n,Refused\nb,\n");
    refused_as_ambiguous(
        &wrk,
        "rt.sav",
        "Column \"s_null\" holds \"Refused\", which is the label of NA, XX,",
    );
}

// NA is labeled "Not asked", which is also an ordinary value: readstat
// records the collision, as nothing in the CSV tells them apart.
#[test]
fn writestat_sav_label_that_is_a_value_refused() {
    let wrk = Workdir::new("writestat_sav_label_that_is_a_value_refused");
    let orig = to_csv(
        &wrk,
        "writestat_sentinel_label_is_value.sav",
        &["--sentinels-as", "label", "--sentinels-embedded"],
    );
    assert_eq!(orig, "s\nNot asked\nNot asked\nb\n");
    let dict: Value = serde_json::from_str(&wrk.read_to_string("dict.json").unwrap()).unwrap();
    assert_eq!(
        dict["properties"]["s"]["x-qsv"]["label_collisions"],
        serde_json::json!(["Not asked"])
    );
    refused_as_ambiguous(
        &wrk,
        "rt.sav",
        "Column \"s\" holds \"Not asked\", which is both a value and the label of NA,",
    );
}

// Under --value-labels the sentinel is written empty, so its label is no
// collision: the round trip keeps the ordinary "Not asked".
#[test]
fn writestat_sav_label_of_dropped_sentinel() {
    let wrk = Workdir::new("writestat_sav_label_of_dropped_sentinel");
    let flags = ["--value-labels"];
    let orig = to_csv(&wrk, "writestat_sentinel_label_is_value.sav", &flags);
    assert_eq!(orig, "s\nNot asked\n\nb\n");
    writestat(&wrk, "rt.sav", &["--lossy"]);
    assert_eq!(read_back(&wrk, "rt.sav", &flags), orig);
    let dict: Value = serde_json::from_str(&wrk.read_to_string("dict.json").unwrap()).unwrap();
    assert_eq!(
        dict["properties"]["s"]["x-qsv"]["label_collisions"],
        Value::Null
    );
}

// An SPSS text code may start with a dot: "." labeled "Skipped" is a
// declared missing value, not a SAS/Stata tag.
#[test]
fn writestat_sav_dot_text_sentinel() {
    let wrk = Workdir::new("writestat_sav_dot_text_sentinel");
    let flags = ["--sentinels-as", "label", "--sentinels-embedded"];
    let orig = to_csv(&wrk, "writestat_dot_sentinel.sav", &flags);
    assert_eq!(orig, "s\na\nSkipped\nb\n");
    // its text value labels can't be written to an SPSS file
    let mut cmd = wrk.command("writestat");
    cmd.args([
        "--lossy",
        "--dictionary",
        "dict.json",
        "orig.csv",
        "-o",
        "rt.sav",
    ]);
    let warn = wrk.stderr_on_success(&mut cmd);
    assert!(
        warn.contains("its codes are text") && !warn.contains("tagged missing"),
        "{warn}"
    );
    assert_eq!(read_back(&wrk, "rt.sav", &[]), "s\na\n\nb\n");
}

// Codes 1 & 99 share "Yes", but 99 is a declared missing value, written
// empty without --sentinels-as: "Yes" can only be 1.
#[test]
fn writestat_sav_missing_value_sharing_a_label() {
    let (_wrk, orig) = sav_round_trip(
        "writestat_sav_missing_value_sharing_a_label",
        "writestat_sentinel_shares_label.sav",
        &["--value-labels"],
    );
    assert_eq!(orig, "q\nYes\n\nNo\n");
}

// Finer than microseconds is a loss for every format
#[test]
fn writestat_datetime_nanoseconds_refused() {
    let wrk = Workdir::new("writestat_datetime_nanoseconds_refused");
    microsecond_datetime(&wrk);
    wrk.create_from_string("orig.csv", "ts\n2024-01-02T03:04:05.123000999\n");
    let mut cmd = wrk.command("writestat");
    cmd.args(["--dictionary", "dict.json", "orig.csv", "-o", "out.xpt"]);
    let err = wrk.stderr_on_error(&mut cmd);
    assert!(
        err.contains("the nanoseconds of the datetimes in \"ts\""),
        "{err}"
    );
}

// A <name>_null column for a variable whose missing values the dictionary
// doesn't declare: its sentinels have nothing to become, so they're a loss.
#[test]
fn writestat_sav_text_sentinels_without_missing_values() {
    let wrk = Workdir::new("writestat_sav_text_sentinels_without_missing_values");
    to_csv(&wrk, "readstat_sentinels.sav", &["--sentinels-as", "value"]);
    let mut dict: Value = serde_json::from_str(&wrk.read_to_string("dict.json").unwrap()).unwrap();
    dict["properties"]["name"]["x-qsv"]
        .as_object_mut()
        .unwrap()
        .remove("missing_values");
    wrk.create_from_string("dict.json", &dict.to_string());

    let mut cmd = wrk.command("writestat");
    cmd.args(["--dictionary", "dict.json", "orig.csv", "-o", "rt.sav"]);
    let err = wrk.stderr_on_error(&mut cmd);
    assert!(
        err.contains("the sentinels of \"name\": the dictionary declares no missing values"),
        "{err}"
    );
    writestat(&wrk, "rt.sav", &["--lossy"]);
    // left out, not written as the value "NA"
    assert!(!read_back(&wrk, "rt.sav", &[]).contains("NA"));
}

// SPSS has no tagged missing values, so its label ".A" is just a label
// (code 1, written with pyreadstat 1.3.6)
#[test]
fn writestat_sav_tag_shaped_label() {
    let flags = [
        "--value-labels",
        "--sentinels-as",
        "label",
        "--sentinels-embedded",
    ];
    let (_wrk, orig) = sav_round_trip(
        "writestat_sav_tag_shaped_label",
        "writestat_spss_tag_shaped_label.sav",
        &flags,
    );
    assert_eq!(orig, "q\n.A\nRefused\n2\n");
}

// Code a is labeled "NA", which is also a missing code - written empty under
// --value-labels alone, so "NA" can only be a.
#[test]
fn writestat_sav_label_is_a_dropped_missing_code() {
    let wrk = Workdir::new("writestat_sav_label_is_a_dropped_missing_code");
    let orig = to_csv(
        &wrk,
        "writestat_label_is_missing_code.sav",
        &["--value-labels"],
    );
    assert_eq!(orig, "s\nNA\n\nb\n");
    // text value labels can't be written to an SPSS file
    writestat(&wrk, "rt.sav", &["--lossy"]);
    assert_eq!(read_back(&wrk, "rt.sav", &[]), "s\na\n\nb\n");
}

const DTA_KEYS: &[&str] = &["label", "value_labels", "format"];

fn dta_round_trip(name: &str, fixture: &str, flags: &[&str]) -> (Workdir, String) {
    let wrk = Workdir::new(name);
    let orig = to_csv(&wrk, fixture, flags);
    writestat(&wrk, "rt.dta", &[]);
    assert_eq!(read_back(&wrk, "rt.dta", flags), orig);
    let fixture_path = wrk.path(fixture).to_string_lossy().to_string();
    assert_eq!(
        vars(&metadata(&wrk, "rt.dta"), DTA_KEYS),
        vars(&metadata(&wrk, &fixture_path), DTA_KEYS)
    );
    (wrk, orig)
}

#[test]
fn writestat_dta_round_trip() {
    let (wrk, orig) = dta_round_trip("writestat_dta_round_trip", "readstat_sample.dta", &[]);
    assert!(orig.contains("\n1.0,Ana,1.5,1.0,2020-01-31\n"), "{orig}");
    let meta = metadata(&wrk, "rt.dta");
    assert_eq!(
        var(&meta, "sex")["value_labels"],
        serde_json::json!({"1": "Male", "2": "Female"})
    );
    // a labeled double stays a double
    assert_eq!(var(&meta, "sex")["type"], "Numeric(Double)");
}

#[test]
fn writestat_dta_value_labels_back_to_codes() {
    let (wrk, orig) = dta_round_trip(
        "writestat_dta_value_labels_back_to_codes",
        "readstat_sample.dta",
        &["--value-labels"],
    );
    assert!(orig.contains(",Male,") && orig.contains(",Female,"));
    // the codes, not their labels as text
    assert_eq!(
        var(&metadata(&wrk, "rt.dta"), "sex")["type"],
        "Numeric(Double)"
    );
}

#[test]
fn writestat_dta_nan_label() {
    dta_round_trip(
        "writestat_dta_nan_label",
        "readstat_stata_nan_label.dta",
        &[],
    );
}

// Labeled whole numbers from SPSS (doubles there) are stored as integers.
#[test]
fn writestat_sav_to_dta_labeled_integers() {
    let wrk = Workdir::new("writestat_sav_to_dta_labeled_integers");
    to_csv(&wrk, "readstat_sample.sav", &[]);
    writestat(&wrk, "out.dta", &[]);
    let meta = metadata(&wrk, "out.dta");
    assert_eq!(var(&meta, "sex")["type"], "Numeric(Byte)");
    assert_eq!(var(&meta, "score")["type"], "Numeric(Double)");
    assert_eq!(
        var(&meta, "sex")["value_labels"],
        serde_json::json!({"1": "Male", "2": "Female"})
    );
}

// Stata's tagged missing values (.a) can't be written, so neither can their
// labels.
#[test]
fn writestat_dta_tagged_missing_labels_refused() {
    let wrk = Workdir::new("writestat_dta_tagged_missing_labels_refused");
    to_csv(&wrk, "readstat_stata_numeric_labels.dta", &[]);
    let mut cmd = wrk.command("writestat");
    cmd.args(["--dictionary", "dict.json", "orig.csv", "-o", "rt.dta"]);
    let err = wrk.stderr_on_error(&mut cmd);
    assert!(
        err.contains("the labels of \"w\"'s sentinels: tagged missing values"),
        "{err}"
    );

    let mut cmd = wrk.command("writestat");
    cmd.args([
        "--lossy",
        "--dictionary",
        "dict.json",
        "orig.csv",
        "-o",
        "rt.dta",
    ]);
    wrk.assert_success(&mut cmd);
    let meta = metadata(&wrk, "rt.dta");
    assert_eq!(
        var(&meta, "v")["value_labels"],
        serde_json::json!({"1": "one"})
    );
}

#[test]
fn writestat_dta_rename_refused() {
    let wrk = Workdir::new("writestat_dta_rename_refused");
    wrk.create_from_string("in.csv", "a long name,b\n1,2\n");
    let mut cmd = wrk.command("writestat");
    cmd.args(["in.csv", "-o", "out.dta"]);
    let err = wrk.stderr_on_error(&mut cmd);
    assert!(
        err.contains("\"a long name\": Stata would rename it \"a_long_name\""),
        "{err}"
    );

    let mut cmd = wrk.command("writestat");
    cmd.args(["--lossy", "in.csv", "-o", "out.dta"]);
    wrk.assert_success(&mut cmd);
    assert_eq!(read_back(&wrk, "out.dta", &[]), "a_long_name,b\n1,2\n");
}

const XPT_KEYS: &[&str] = &["label", "format", "type"];

#[test]
fn writestat_xpt_round_trip() {
    for (fixture, version, table) in [
        ("readstat_sample.xpt", "xpt5", "DATASET"),
        ("readstat_sample.xpt", "xpt8", "DATASET"),
        ("readstat_sample.sas7bdat", "xpt5", "AIRLINE"),
        ("readstat_sample.sas7bdat", "xpt8", "AIRLINE"),
    ] {
        let wrk = Workdir::new(&format!("writestat_xpt_round_trip_{fixture}_{version}"));
        let orig = to_csv(&wrk, fixture, &[]);
        writestat(&wrk, "rt.xpt", &["--format", version]);
        assert_eq!(read_back(&wrk, "rt.xpt", &[]), orig, "{fixture} {version}");
        let meta = metadata(&wrk, "rt.xpt");
        let fixture_path = wrk.path(fixture).to_string_lossy().to_string();
        assert_eq!(
            vars(&meta, XPT_KEYS),
            vars(&metadata(&wrk, &fixture_path), XPT_KEYS),
            "{fixture} {version}"
        );
        assert_eq!(meta["table_name"], table);
        assert_eq!(meta["xpt_version"], version[3..].parse::<u8>().unwrap());
    }
}

/// A dictionary of my own: a file label, & variable labels.
const LABELED_DICT: &str = r#"{
  "properties": {
    "x": {"title": "The x variable", "x-qsv": {"qsv_type": "Integer"}},
    "y": {"title": "The y variable", "x-qsv": {"qsv_type": "String"}}
  },
  "x-qsv": {"file_label": "My survey"}
}"#;

// The file label goes in, & 'readstat --dictionary' brings it back out.
#[test]
fn writestat_xpt_file_label() {
    let wrk = Workdir::new("writestat_xpt_file_label");
    wrk.create_from_string("dict.json", LABELED_DICT);
    wrk.create_from_string("orig.csv", "x,y\n1,a\n2,b\n");
    writestat(&wrk, "out.xpt", &["--table-name", "SURVEY"]);
    let meta = metadata(&wrk, "out.xpt");
    assert_eq!(meta["file_label"], "My survey");
    assert_eq!(meta["table_name"], "SURVEY");
    assert_eq!(var(&meta, "x")["label"], "The x variable");

    let mut cmd = wrk.command("readstat");
    cmd.args(["--dictionary", "back.json", "out.xpt", "-o", "back.csv"]);
    wrk.assert_success(&mut cmd);
    let back: Value = serde_json::from_str(&wrk.read_to_string("back.json").unwrap()).unwrap();
    assert_eq!(back["x-qsv"]["file_label"], "My survey");
}

#[test]
fn writestat_xpt_limits_refused() {
    let wrk = Workdir::new("writestat_xpt_limits_refused");
    wrk.create_from_string(
        "dict.json",
        &LABELED_DICT.replace(
            "The x variable",
            "A variable label well past the forty byte limit of SAS transport",
        ),
    );
    wrk.create_from_string("orig.csv", "x,y\n1,a\n");
    let mut cmd = wrk.command("writestat");
    cmd.args(["--dictionary", "dict.json", "orig.csv", "-o", "out.xpt"]);
    let err = wrk.stderr_on_error(&mut cmd);
    assert!(
        err.contains("the label of \"x\": longer than 40 bytes"),
        "{err}"
    );

    let mut cmd = wrk.command("writestat");
    cmd.args([
        "--lossy",
        "--dictionary",
        "dict.json",
        "orig.csv",
        "-o",
        "out.xpt",
    ]);
    wrk.assert_success(&mut cmd);
    let meta = metadata(&wrk, "out.xpt");
    assert_eq!(var(&meta, "x")["label"], Value::Null);
    assert_eq!(var(&meta, "y")["label"], "The y variable");

    wrk.create_from_string("long.csv", "longername,y\n1,a\n");
    let mut cmd = wrk.command("writestat");
    cmd.args(["long.csv", "-o", "out.xpt", "--format", "xpt5"]);
    let err = wrk.stderr_on_error(&mut cmd);
    assert!(
        err.contains("at most 8 bytes; these are longer: \"longername\""),
        "{err}"
    );
    let mut cmd = wrk.command("writestat");
    cmd.args(["long.csv", "-o", "out.xpt", "--format", "xpt8"]);
    wrk.assert_success(&mut cmd);

    let mut cmd = wrk.command("writestat");
    cmd.args(["orig.csv", "-o", "out.xpt", "--format", "xpt5"])
        .args(["--table-name", "TOOLONGNAME"]);
    let err = wrk.stderr_on_error(&mut cmd);
    assert!(
        err.contains("\"TOOLONGNAME\" is longer than 8 bytes"),
        "{err}"
    );
}

// The sample's last row has an empty name, which polars-readstat-rs wrote as
// " " until 0.24.2 (jrothbaum/polars_readstat#70).
#[test]
fn writestat_por_round_trip() {
    let wrk = Workdir::new("writestat_por_round_trip");
    let orig = to_csv(&wrk, "readstat_sample.por", &[]);
    writestat(&wrk, "rt.por", &[]);
    let back = read_back(&wrk, "rt.por", &[]);
    let head = |csv: &str| csv.lines().take(4).collect::<Vec<_>>().join("\n");
    assert_eq!(
        head(&back),
        "ID,NAME,SCORE,SEX,VISITED\n1.0,Ana,1.5,1.0,2020-01-31\n2.0,Bo,-2.25,2.0,1999-12-31\n3.0,\
         Chloe,,1.0,2026-03-01"
    );
    assert_eq!(back, orig);
}

#[test]
fn writestat_por_labels() {
    let wrk = Workdir::new("writestat_por_labels");
    wrk.create_from_string("dict.json", LABELED_DICT);
    wrk.create_from_string("orig.csv", "x,y\n1,a\n2,b\n");
    writestat(&wrk, "out.por", &[]);
    let meta = metadata(&wrk, "out.por");
    assert_eq!(meta["file_label"], "My survey");
    assert_eq!(var(&meta, "X")["label"], "The x variable");
    assert_eq!(read_back(&wrk, "out.por", &[]), "X,Y\n1.0,a\n2.0,b\n");
}

// SPSS portable files can't hold value labels or missing values
#[test]
fn writestat_por_value_labels_refused() {
    let wrk = Workdir::new("writestat_por_value_labels_refused");
    to_csv(&wrk, "readstat_sample.sav", &[]);
    let mut cmd = wrk.command("writestat");
    cmd.args(["--dictionary", "dict.json", "orig.csv", "-o", "out.por"]);
    let err = wrk.stderr_on_error(&mut cmd);
    assert!(err.contains("the value labels of \"sex\""), "{err}");
}

#[test]
fn writestat_zsav_refused() {
    let wrk = Workdir::new("writestat_zsav_refused");
    wrk.create_from_string("in.csv", "x\n1\n");
    let mut cmd = wrk.command("writestat");
    cmd.args(["in.csv", "-o", "out.zsav"]);
    let err = wrk.stderr_on_error(&mut cmd);
    assert!(
        err.contains("Compressed SPSS .zsav files can't be written"),
        "{err}"
    );
}

#[test]
fn writestat_requires_output() {
    let wrk = Workdir::new("writestat_requires_output");
    wrk.create_from_string("in.csv", "x\n1\n");
    let mut cmd = wrk.command("writestat");
    cmd.arg("in.csv");
    let err = wrk.stderr_on_error(&mut cmd);
    assert!(err.contains("needs --output"), "{err}");
}

#[test]
fn writestat_compress_is_sav_only() {
    let wrk = Workdir::new("writestat_compress_is_sav_only");
    wrk.create_from_string("in.csv", "x\n1\n");
    let mut cmd = wrk.command("writestat");
    cmd.args(["in.csv", "-o", "out.dta", "--compress"]);
    let err = wrk.stderr_on_error(&mut cmd);
    assert!(
        err.contains("--compress applies to SPSS .sav files only"),
        "{err}"
    );
}

#[test]
fn writestat_dictionary_mismatch() {
    let wrk = Workdir::new("writestat_dictionary_mismatch");
    wrk.create_from_string("dict.json", LABELED_DICT);
    wrk.create_from_string("orig.csv", "x,z\n1,a\n");
    let mut cmd = wrk.command("writestat");
    cmd.args(["--dictionary", "dict.json", "orig.csv", "-o", "out.sav"]);
    let err = wrk.stderr_on_error(&mut cmd);
    assert!(
        err.contains("describes columns the CSV doesn't have: \"y\""),
        "{err}"
    );
}

// No dictionary: the types are inferred, from stdin
#[test]
fn writestat_stdin_inferred_types() {
    let wrk = Workdir::new("writestat_stdin_inferred_types");
    wrk.create_from_string("in.csv", "n,s,d\n1,a,2024-01-02\n2.5,b,2024-03-04\n");
    let mut cmd = wrk.command("writestat");
    cmd.args(["-o", "out.sav"])
        .stdin(std::fs::File::open(wrk.path("in.csv")).unwrap());
    wrk.assert_success(&mut cmd);
    let meta = metadata(&wrk, "out.sav");
    assert_eq!(var(&meta, "n")["type"], "Numeric");
    assert_eq!(var(&meta, "s")["type"], "Str");
    assert_eq!(var(&meta, "d")["format_class"], "Date");
    assert_eq!(
        read_back(&wrk, "out.sav", &[]),
        "n,s,d\n1.0,a,2024-01-02\n2.5,b,2024-03-04\n"
    );
}

// No dictionary: the numeric columns are typed from the stats cache, and the
// text columns holding dates, times or booleans are recognised as polars does.
// Each column below must come out exactly as polars' own whole-file inference
// (QSV_STATSCACHE_MODE=none) types it.
const INFERENCE_CSV: &str = "\
i,f,b,d,dt,t,nyc,e,s,dmix
1,1.1,true,2024-01-02,2024-01-02T03:04:05,12:34:56,04/18/2019 09:55:45 PM,,a,2024-01-02
,2,FALSE,2024-01-03,2024-01-02T03:04:06,01:02:03,04/19/2019 10:00:00 AM,,b,x
3,,,,,,,,c,2024-01-04
";

fn inferred(wrk: &Workdir, out: &str, statscache: bool) {
    let mut cmd = wrk.command("writestat");
    cmd.args(["in.csv", "-o", out]);
    if !statscache {
        cmd.env("QSV_STATSCACHE_MODE", "none");
    }
    wrk.assert_success(&mut cmd);
}

fn assert_inference_matches_polars(name: &str, ext: &str) -> (Workdir, Value) {
    let wrk = Workdir::new(name);
    wrk.create_from_string("in.csv", INFERENCE_CSV);
    let (stats, polars) = (format!("stats.{ext}"), format!("polars.{ext}"));
    inferred(&wrk, &polars, false);
    assert!(!wrk.path("in.stats.csv.data.jsonl").exists());
    inferred(&wrk, &stats, true);
    // the stats path was taken
    assert!(wrk.path("in.stats.csv.data.jsonl").exists());
    assert_eq!(read_back(&wrk, &stats, &[]), read_back(&wrk, &polars, &[]));
    let meta = metadata(&wrk, &stats);
    assert_eq!(
        vars(&meta, &["type", "format_class"]),
        vars(&metadata(&wrk, &polars), &["type", "format_class"])
    );
    (wrk, meta)
}

#[test]
fn writestat_inferred_types_match_polars_sav() {
    let (wrk, meta) = assert_inference_matches_polars("writestat_inferred_types_sav", "sav");
    for (name, ty) in [
        ("i", "Numeric"),
        ("f", "Numeric"),
        ("nyc", "Str"),
        ("e", "Str"),
    ] {
        assert_eq!(var(&meta, name)["type"], ty, "{name}");
    }
    assert_eq!(var(&meta, "d")["format_class"], "Date");
    assert_eq!(var(&meta, "dt")["format_class"], "DateTime");
    assert_eq!(var(&meta, "t")["format_class"], "Time");
    // one value that isn't a date leaves the column as text
    assert_eq!(var(&meta, "dmix")["type"], "Str");
    // read as a double, never a float: 1.1 stays 1.1
    let back = read_back(&wrk, "stats.sav", &["--select", "f"]);
    assert_eq!(back, "f\n1.1\n2.0\n\n");
}

#[test]
fn writestat_inferred_types_match_polars_dta() {
    assert_inference_matches_polars("writestat_inferred_types_dta", "dta");
}

#[test]
fn writestat_inferred_types_match_polars_xpt() {
    assert_inference_matches_polars("writestat_inferred_types_xpt", "xpt");
}

// stdin is spooled to a directory of its own, which takes the stats cache
// written beside the spool away with it
#[test]
fn writestat_stdin_leaves_no_stats_cache() {
    let wrk = Workdir::new("writestat_stdin_leaves_no_stats_cache");
    wrk.create_from_string("in.csv", "n,s\n1,a\n2,b\n");
    std::fs::create_dir(wrk.path("tmp")).unwrap();
    let mut cmd = wrk.command("writestat");
    cmd.args(["-o", "out.sav"])
        .env("TMPDIR", wrk.path("tmp"))
        .stdin(std::fs::File::open(wrk.path("in.csv")).unwrap());
    wrk.assert_success(&mut cmd);
    assert_eq!(std::fs::read_dir(wrk.path("tmp")).unwrap().count(), 0);
}

// A stats cache that no longer matches its file (same size, older input) is
// reported, with how to get past it.
#[test]
fn writestat_stale_stats_cache() {
    let wrk = Workdir::new("writestat_stale_stats_cache");
    wrk.create_from_string("in.csv", "n\n1\n2\n");
    let mut cmd = wrk.command("writestat");
    cmd.args(["in.csv", "-o", "first.sav"]);
    wrk.assert_success(&mut cmd);
    let cache = std::fs::metadata(wrk.path("in.stats.csv.data.jsonl")).unwrap();
    let cache_mtime = filetime::FileTime::from_last_modification_time(&cache);
    wrk.create_from_string("in.csv", "n\n1\nx\n");
    let older = filetime::FileTime::from_unix_time(cache_mtime.unix_seconds() - 60, 0);
    filetime::set_file_mtime(wrk.path("in.csv"), older).unwrap();

    let mut cmd = wrk.command("writestat");
    cmd.args(["in.csv", "-o", "second.sav"]);
    let err = wrk.stderr_on_error(&mut cmd);
    assert!(err.contains("may be out of date"), "{err}");
    assert!(err.contains("QSV_STATSCACHE_MODE=none"), "{err}");

    let mut cmd = wrk.command("writestat");
    cmd.args(["in.csv", "-o", "third.sav"])
        .env("QSV_STATSCACHE_MODE", "none");
    wrk.assert_success(&mut cmd);
    assert_eq!(read_back(&wrk, "third.sav", &[]), "n\n1\nx\n");
}

// The one intended change from polars' own inference: zero-padded numbers
// (ZIP, FIPS or BBL codes) keep their zeros as text, where polars read them
// as numbers.
#[test]
fn writestat_zero_padded_stays_text() {
    let wrk = Workdir::new("writestat_zero_padded_stays_text");
    wrk.create_from_string("in.csv", "code,n\n00123,1\n00456,2\n,3\n");
    inferred(&wrk, "out.sav", true);
    let meta = metadata(&wrk, "out.sav");
    assert_eq!(var(&meta, "code")["type"], "Str");
    assert_eq!(var(&meta, "n")["type"], "Numeric");
    assert_eq!(
        read_back(&wrk, "out.sav", &[]),
        "code,n\n00123,1.0\n00456,2.0\n,3.0\n"
    );
}
