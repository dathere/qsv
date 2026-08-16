use newline_converter::dos2unix;
use serial_test::serial;

use crate::workdir::Workdir;

#[test]
#[serial]
fn tojsonl_simple() {
    let wrk = Workdir::new("tojsonl_simple");
    wrk.create(
        "in.csv",
        vec![
            svec!["id", "father", "mother", "oldest_child", "boy", "weight"],
            svec!["1", "Mark", "Charlotte", "Tom", "true", "150.2"],
            svec!["2", "John", "Ann", "Jessika", "false", "175.5"],
            svec!["3", "Bob", "Monika", "Jerry", "true", "199.5"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    let got: String = wrk.stdout_on_success(&mut cmd);
    let expected = r#"{"id":1,"father":"Mark","mother":"Charlotte","oldest_child":"Tom","boy":true,"weight":150.2}
{"id":2,"father":"John","mother":"Ann","oldest_child":"Jessika","boy":false,"weight":175.5}
{"id":3,"father":"Bob","mother":"Monika","oldest_child":"Jerry","boy":true,"weight":199.5}"#;
    assert_eq!(got, expected);
}

#[test]
#[serial]
fn tojsonl_2579() {
    let wrk = Workdir::new("tojsonl_2579");
    wrk.create(
        "in.csv",
        vec![
            svec!["Date", "Product", "Unit", "Price"],
            svec!["1937-01-01", "Milk", "1 gallon", ".1"],
            svec!["1937-01-01", "Bread", "1 loaf", ".09"],
            svec!["1937-01-01", "Movie ticket", "1 ticket", ".25"],
            svec!["1937-01-01", "Milk", "10 gallons", "1.00000"],
            svec!["1937-01-01", "Milk", "100 gallons", "10"],
            svec!["1937-01-01", "Taxi", "1 mile", "0.90000"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    let got: String = wrk.stdout_on_success(&mut cmd);
    let expected = r#"{"Date":"1937-01-01","Product":"Milk","Unit":"1 gallon","Price":0.1}
{"Date":"1937-01-01","Product":"Bread","Unit":"1 loaf","Price":0.09}
{"Date":"1937-01-01","Product":"Movie ticket","Unit":"1 ticket","Price":0.25}
{"Date":"1937-01-01","Product":"Milk","Unit":"10 gallons","Price":1.0}
{"Date":"1937-01-01","Product":"Milk","Unit":"100 gallons","Price":10.0}
{"Date":"1937-01-01","Product":"Taxi","Unit":"1 mile","Price":0.9}"#;
    assert_eq!(got, expected);
}

#[test]
#[serial]
fn tojsonl_2294() {
    let wrk = Workdir::new("tojsonl_simple");
    wrk.create(
        "file.csv",
        vec![
            svec!["col1", "col2", "col3"],
            svec!["a", "b", "c"],
            svec!["d", "e", "f"],
        ],
    );

    wrk.create_subdir("qsv test").unwrap();
    std::fs::rename(wrk.path("file.csv"), wrk.path("qsv test").join("file.csv")).unwrap();

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("qsv test/file.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"col1":"a","col2":"b","col3":"c"}
{"col1":"d","col2":"e","col3":"f"}"#;
    assert_eq!(got, expected);
}

#[test]
#[serial]
fn tojsonl_boolean() {
    let wrk = Workdir::new("tojsonl");
    wrk.create(
        "in.csv",
        vec![
            svec!["col1", "col2"],
            svec!["true", "Mark"],
            svec!["false", "John"],
            svec!["false", "Bob"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"col1":true,"col2":"Mark"}
{"col1":false,"col2":"John"}
{"col1":false,"col2":"Bob"}"#;
    assert_eq!(got, expected);
}

#[test]
#[serial]
fn tojsonl_boolean_tf() {
    let wrk = Workdir::new("tojsonl");
    wrk.create(
        "in.csv",
        vec![
            svec!["col1", "col2"],
            svec!["t", "Mark"],
            svec!["f", "John"],
            svec!["f", "Bob"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"col1":true,"col2":"Mark"}
{"col1":false,"col2":"John"}
{"col1":false,"col2":"Bob"}"#;
    assert_eq!(got, expected);
}

#[test]
#[serial]
fn tojsonl_boolean_upper_tf() {
    let wrk = Workdir::new("tojsonl");
    wrk.create(
        "in.csv",
        vec![
            svec!["col1", "col2"],
            svec!["T", "Mark"],
            svec!["F", "John"],
            svec!["F", "Bob"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"col1":true,"col2":"Mark"}
{"col1":false,"col2":"John"}
{"col1":false,"col2":"Bob"}"#;
    assert_eq!(got, expected);
}

#[test]
#[serial]
fn tojsonl_boolean_1or0() {
    let wrk = Workdir::new("tojsonl");
    wrk.create(
        "in.csv",
        vec![
            svec!["col1", "col2"],
            svec!["1", "Mark"],
            svec!["0", "John"],
            svec!["0", "Bob"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"col1":true,"col2":"Mark"}
{"col1":false,"col2":"John"}
{"col1":false,"col2":"Bob"}"#;
    assert_eq!(got, expected);
}
#[test]
#[serial]
fn tojsonl_noboolean_1or0() {
    let wrk = Workdir::new("tojsonl_noboolean_1or0");
    wrk.create(
        "in.csv",
        vec![
            svec!["col1", "col2"],
            svec!["1", "Mark"],
            svec!["0", "John"],
            svec!["0", "Bob"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("--no-boolean").arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"col1":1,"col2":"Mark"}
{"col1":0,"col2":"John"}
{"col1":0,"col2":"Bob"}"#;
    assert_eq!(got, expected);
}

#[test]
#[serial]
fn tojsonl_noboolean_tworecords() {
    let wrk = Workdir::new("tojsonl_noboolean_tworecords");
    wrk.create(
        "in.csv",
        vec![
            svec!["col1", "col2"],
            svec!["1", "Mark"],
            svec!["0", "John"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"col1":1,"col2":"Mark"}
{"col1":0,"col2":"John"}"#;
    assert_eq!(got, expected);
}

#[test]
#[serial]
fn tojsonl_boolean_1or0_false_positive_handling() {
    let wrk = Workdir::new("tojsonl");
    wrk.create(
        "in.csv",
        vec![
            svec!["col1", "col2"],
            svec!["15", "Mark"],
            svec!["02", "John"],
            svec!["02", "Bob"],
            svec!["15", "Mary"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"col1":"15","col2":"Mark"}
{"col1":"02","col2":"John"}
{"col1":"02","col2":"Bob"}
{"col1":"15","col2":"Mary"}"#;
    assert_eq!(got, expected);
}

#[test]
#[serial]
fn tojsonl_not_boolean_case_sensitive() {
    let wrk = Workdir::new("tojsonl");
    wrk.create(
        "in.csv",
        vec![
            svec!["col1", "col2"],
            svec!["True", "Mark"],
            svec!["False", "John"],
            svec!["false", "Bob"],
            svec!["TRUE", "Mary"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    // properly treated as boolean since col1's domain has two values
    // case-insensitive, even though the enum for col1 is
    // True, False, false and TRUE
    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"col1":true,"col2":"Mark"}
{"col1":false,"col2":"John"}
{"col1":false,"col2":"Bob"}
{"col1":true,"col2":"Mary"}"#;
    assert_eq!(got, expected);
}

#[test]
#[serial]
fn tojsonl_is_boolean_case_sensitive() {
    let wrk = Workdir::new("tojsonl");
    wrk.create(
        "in.csv",
        vec![
            svec!["col1", "col2"],
            svec!["True", "Mark"],
            svec!["False", "John"],
            svec!["False", "Bob"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    // this is treated as boolean since col1's domain has two values
    // True and False
    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"col1":true,"col2":"Mark"}
{"col1":false,"col2":"John"}
{"col1":false,"col2":"Bob"}"#;
    assert_eq!(got, expected);
}

#[test]
#[serial]
fn tojsonl_boolean_yes() {
    let wrk = Workdir::new("tojsonl");
    wrk.create(
        "in.csv",
        vec![
            svec!["col1", "col2"],
            svec!["yes", "Mark"],
            svec!["no", "John"],
            svec!["no", "Bob"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"col1":true,"col2":"Mark"}
{"col1":false,"col2":"John"}
{"col1":false,"col2":"Bob"}"#;
    assert_eq!(got, expected);
}

#[test]
#[serial]
fn tojsonl_boolean_null() {
    let wrk = Workdir::new("tojsonl");
    wrk.create(
        "in.csv",
        vec![
            svec!["col1", "col2"],
            svec!["true", "Mark"],
            svec!["", "John"],
            svec!["", "Bob"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"col1":true,"col2":"Mark"}
{"col1":false,"col2":"John"}
{"col1":false,"col2":"Bob"}"#;
    assert_eq!(got, expected);
}

#[test]
#[serial]
fn tojsonl_boolean_y_null() {
    let wrk = Workdir::new("tojsonl");
    wrk.create(
        "in.csv",
        vec![
            svec!["col1", "col2"],
            svec!["y", "Mark"],
            svec!["", "John"],
            svec!["", "Bob"],
            svec!["y", "Mary"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"col1":true,"col2":"Mark"}
{"col1":false,"col2":"John"}
{"col1":false,"col2":"Bob"}
{"col1":true,"col2":"Mary"}"#;
    assert_eq!(got, expected);
}

#[test]
#[serial]
fn tojsonl_nested() {
    let wrk = Workdir::new("tojsonl_nested");
    wrk.create(
        "in.csv",
        vec![
            svec!["id", "father", "mother", "children"],
            svec!["1", "Mark", "Charlotte", "\"Tom\""],
            svec!["2", "John", "Ann", "\"Jessika\",\"Antony\",\"Jack\""],
            svec!["3", "Bob", "Monika", "\"Jerry\",\"Karol\""],
            svec![
                "4",
                "John\nSmith",
                "Jane \"Smiley\" Doe",
                "\"Jack\",\"Jill\r\n \"Climber\""
            ],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"id":1,"father":"Mark","mother":"Charlotte","children":"\"Tom\""}
{"id":2,"father":"John","mother":"Ann","children":"\"Jessika\",\"Antony\",\"Jack\""}
{"id":3,"father":"Bob","mother":"Monika","children":"\"Jerry\",\"Karol\""}
{"id":4,"father":"John\nSmith","mother":"Jane \"Smiley\" Doe","children":"\"Jack\",\"Jill\r\n \"Climber\""}"#;

    assert_eq!(got, expected);
}

#[test]
#[serial]
fn tojsonl_boston() {
    let wrk = Workdir::new("tojsonl");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("tojsonl");
    cmd.arg(test_file);

    let got: String = wrk.stdout(&mut cmd);

    let expected = wrk.load_test_resource("boston311-100-untrimmed.jsonl");

    assert_eq!(dos2unix(&got), dos2unix(&expected).trim_end());
}

#[test]
#[serial]
fn tojsonl_boston_snappy() {
    let wrk = Workdir::new("tojsonl");
    let test_file = wrk.load_test_file("boston311-100.csv.sz");

    let mut cmd = wrk.command("tojsonl");
    cmd.arg(test_file);

    let got: String = wrk.stdout(&mut cmd);

    let expected = wrk.load_test_resource("boston311-100-untrimmed.jsonl");

    assert_eq!(dos2unix(&got), dos2unix(&expected).trim_end());
}

#[test]
#[serial]
fn tojsonl_boston_trim() {
    let wrk = Workdir::new("tojsonl");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("tojsonl");
    cmd.arg(test_file).arg("--trim");

    let got: String = wrk.stdout(&mut cmd);

    let expected = wrk.load_test_resource("boston311-100.jsonl");

    assert_eq!(dos2unix(&got), dos2unix(&expected).trim_end());
}

#[test]
fn tojsonl_issue_1649_false_positive_tf() {
    let wrk = Workdir::new("tojsonl_issue_1649_false_positive_tf");
    wrk.create(
        "in.csv",
        vec![
            svec!["id", "name"],
            svec!["1", "François Hollande"],
            svec!["2", "Tarja Halonen"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"id":1,"name":"François Hollande"}
{"id":2,"name":"Tarja Halonen"}"#;

    assert_eq!(got, expected);
}

#[test]
fn tojsonl_issue_1649_false_positive_tf_3recs() {
    let wrk = Workdir::new("tojsonl_issue_1649_false_positive_tf_3_recs");
    wrk.create(
        "in.csv",
        vec![
            svec!["id", "name"],
            svec!["1", "Fanuel"],
            svec!["2", "Travis"],
            svec!["3", "Travis"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"id":1,"name":"Fanuel"}
{"id":2,"name":"Travis"}
{"id":3,"name":"Travis"}"#;

    assert_eq!(got, expected);
}

#[test]
#[serial]
fn tojsonl_output_dash_stdout() {
    // -o - should write to stdout and not create a file named "-"
    let wrk = Workdir::new("tojsonl_output_dash_stdout");
    wrk.create(
        "in.csv",
        vec![
            svec!["id", "name"],
            svec!["1", "Alice"],
            svec!["2", "Bob"],
            svec!["3", "Carol"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv").args(["-o", "-"]);

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"id":1,"name":"Alice"}
{"id":2,"name":"Bob"}
{"id":3,"name":"Carol"}"#;

    assert_eq!(got, expected);

    // ensure no file named "-" was created
    let dash_path = wrk.path("-");
    assert!(
        !dash_path.exists(),
        "A file named '-' should not be created when using -o -"
    );
}

#[test]
#[serial]
fn tojsonl_number_empty_field_is_null() {
    // a column inferred as Number (Float) with an empty value should emit `null`,
    // matching how empty String/Integer fields are rendered.
    let wrk = Workdir::new("tojsonl_number_empty_field_is_null");
    wrk.create(
        "in.csv",
        vec![
            svec!["id", "weight"],
            svec!["1", "150.2"],
            svec!["2", ""],
            svec!["3", "199.5"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    let got: String = wrk.stdout_on_success(&mut cmd);
    let expected = r#"{"id":1,"weight":150.2}
{"id":2,"weight":null}
{"id":3,"weight":199.5}"#;
    assert_eq!(got, expected);
}

#[test]
#[serial]
fn tojsonl_number_non_finite_is_null() {
    // a column inferred as Number containing a value that parses to a non-finite f64
    // (overflow → ±Infinity from fast_float2) must emit `null` rather than panic
    // (the previous `Number::from_f64(...).unwrap()` would have aborted the run) and
    // rather than silently coerce to 0 (which would fabricate data).
    let wrk = Workdir::new("tojsonl_number_non_finite_is_null");
    wrk.create(
        "in.csv",
        vec![
            svec!["id", "magnitude"],
            svec!["1", "1.5"],
            svec!["2", "1e400"],
            svec!["3", "-1e400"],
            svec!["4", "2.5"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    let got: String = wrk.stdout_on_success(&mut cmd);
    let expected = r#"{"id":1,"magnitude":1.5}
{"id":2,"magnitude":null}
{"id":3,"magnitude":null}
{"id":4,"magnitude":2.5}"#;
    assert_eq!(got, expected);
}

#[test]
#[serial]
fn tojsonl_number_nan_infinity_literal_is_null() {
    // qsv stats classifies "NaN" / "Infinity" / "-Infinity" as Float, so the column is
    // inferred as Number. fast_float2 parses them as the non-finite IEEE-754 values,
    // which Number::from_f64 rejects — they must come out as `null`, not as
    // "NaN"/"Infinity" (invalid JSON) or panics.
    let wrk = Workdir::new("tojsonl_number_nan_infinity_literal_is_null");
    wrk.create(
        "in.csv",
        vec![
            svec!["id", "score"],
            svec!["1", "1.5"],
            svec!["2", "NaN"],
            svec!["3", "Infinity"],
            svec!["4", "-Infinity"],
            svec!["5", "2.5"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    let got: String = wrk.stdout_on_success(&mut cmd);
    let expected = r#"{"id":1,"score":1.5}
{"id":2,"score":null}
{"id":3,"score":null}
{"id":4,"score":null}
{"id":5,"score":2.5}"#;
    assert_eq!(got, expected);
}

#[test]
#[serial]
fn tojsonl_4410() {
    // issue #4410: a CSV with an empty column name failed with
    // `error parsing stats: Serde("missing field `field`")`, because the stats cache
    // JSONL dropped the `field` key for the empty-named column.
    let wrk = Workdir::new("tojsonl_4410");
    wrk.create(
        "in.csv",
        vec![
            svec!["a", "", "c"],
            svec!["1", "2", "3"],
            svec!["4", "5", "6"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    let got: String = wrk.stdout_on_success(&mut cmd);
    let expected = r#"{"a":1,"":2,"c":3}
{"a":4,"":5,"c":6}"#;
    assert_eq!(got, expected);
}

#[test]
#[serial]
fn tojsonl_4410_legacy_cache_missing_field_key() {
    // issue #4410 migration path: stats caches written by qsv 22.0.1 and earlier dropped
    // the `field` key entirely for an empty-named column. Such caches stay on disk and
    // still pass mtime validation after an upgrade, which is why `StatsData.field` is
    // `#[serde(default)]`. Without that default this fails with `missing field `field``.
    use filetime::{FileTime, set_file_mtime};

    let wrk = Workdir::new("tojsonl_4410_legacy_cache_missing_field_key");
    wrk.create(
        "in.csv",
        vec![
            svec!["a", "", "c"],
            svec!["1", "2", "3"],
            svec!["4", "5", "6"],
        ],
    );

    // Cache reuse requires `cache_mtime > input_mtime` (strict). On coarse (1s)
    // filesystem timestamp resolution the input and the cache written moments later can
    // land in the same tick, forcing a regeneration that would trip the reuse assertion
    // below - the same hazard documented in test_profile.rs. Backdate the input so it is
    // unambiguously older, which is deterministic and costs no wall-clock time.
    let past = FileTime::from_unix_time(FileTime::now().unix_seconds() - 3600, 0);
    set_file_mtime(wrk.path("in.csv"), past).unwrap();

    // prime a valid stats cache, then rewrite it the way a pre-fix qsv would have
    let mut prime = wrk.command("tojsonl");
    prime.arg("in.csv");
    wrk.assert_success(&mut prime);

    let cache_path = wrk.path("in.stats.csv.data.jsonl");
    let cache = std::fs::read_to_string(&cache_path).unwrap();
    let downgraded: Vec<String> = cache
        .lines()
        .map(|line| {
            let mut v: serde_json::Value = serde_json::from_str(line)
                .unwrap_or_else(|_| panic!("cache line should be valid JSON: {line}"));
            let obj = v.as_object_mut().expect("each cache line is a JSON object");
            if obj.get("field").and_then(serde_json::Value::as_str) == Some("") {
                obj.remove("field");
            }
            serde_json::to_string(&v).unwrap()
        })
        .collect();
    assert!(
        downgraded.iter().any(|l| !l.contains("\"field\"")),
        "setup failed: no record ended up missing its `field` key"
    );
    std::fs::write(&cache_path, downgraded.join("\n") + "\n").unwrap();

    // the consumer must load that cache and still map the empty header correctly
    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    let got: String = wrk.stdout_on_success(&mut cmd);
    let expected = r#"{"a":1,"":2,"c":3}
{"a":4,"":5,"c":6}"#;
    assert_eq!(got, expected);

    // the run must have REUSED the downgraded cache rather than regenerating it -
    // otherwise this test would pass even with the serde default removed
    let after = std::fs::read_to_string(&cache_path).unwrap();
    assert!(
        after.lines().any(|l| !l.contains("\"field\"")),
        "cache was regenerated instead of reused, so this no longer guards the migration path: \
         {after}"
    );
}

#[test]
#[serial]
fn tojsonl_duplicate_headers_warns() {
    // Duplicate column names collapse into a single JSON key; warn the user.
    let wrk = Workdir::new("tojsonl_duplicate_headers_warns");
    wrk.create(
        "in.csv",
        vec![
            svec!["id", "id", "name"],
            svec!["1", "red", "bob"],
            svec!["2", "blue", "sue"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");
    let stderr = wrk.output_stderr(&mut cmd);
    assert!(
        stderr.contains("Duplicate column name(s) detected (id)"),
        "expected duplicate-header warning, got stderr: {stderr}"
    );

    // --quiet suppresses the warning
    let mut quiet_cmd = wrk.command("tojsonl");
    quiet_cmd.arg("--quiet").arg("in.csv");
    let quiet_stderr = wrk.output_stderr(&mut quiet_cmd);
    assert!(
        !quiet_stderr.contains("Duplicate column name"),
        "--quiet should suppress the warning, got stderr: {quiet_stderr}"
    );
}
