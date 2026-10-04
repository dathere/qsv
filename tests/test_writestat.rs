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

// Upstream writes an empty POR string as " ", so the sample's empty name is
// left out of the comparison.
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
    assert_eq!(head(&back), head(&orig));
    assert!(back.ends_with(",0.0,,1960-01-01\n"), "{back}");
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
