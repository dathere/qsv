use crate::workdir::Workdir;

/// Small fixture with the four shapes synthesize handles: low-cardinality
/// categorical (`city`), Integer (`age`), Float (`score`), Boolean (`active`),
/// Date (`join_date`).
fn fixture() -> Vec<Vec<String>> {
    vec![
        svec!["city", "age", "score", "active", "join_date"],
        svec!["NYC", "25", "3.5", "true", "2020-01-15"],
        svec!["LA", "30", "4.2", "false", "2021-06-20"],
        svec!["NYC", "22", "3.8", "true", "2022-03-10"],
        svec!["Chicago", "45", "4.9", "true", "2019-11-05"],
        svec!["LA", "35", "3.1", "false", "2023-02-28"],
        svec!["NYC", "28", "4.4", "true", "2020-08-15"],
        svec!["Chicago", "40", "3.7", "false", "2021-12-01"],
        svec!["NYC", "31", "4.1", "true", "2022-07-22"],
        svec!["LA", "27", "3.9", "true", "2020-05-30"],
        svec!["Chicago", "38", "4.6", "false", "2023-08-14"],
    ]
}

#[test]
fn synthesize_no_dictionary_basic() {
    let wrk = Workdir::new("synthesize_basic");
    wrk.create("data.csv", fixture());

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "50", "--seed", "42"]).arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // 1 header + 50 data rows
    assert_eq!(got.len(), 51);
    assert_eq!(got[0], svec!["city", "age", "score", "active", "join_date"]);

    let cities: std::collections::HashSet<&str> = ["NYC", "LA", "Chicago"].into_iter().collect();
    for row in &got[1..] {
        // city: must be one of the real source values (FrequencyWeighted).
        assert!(
            cities.contains(row[0].as_str()),
            "unexpected city '{}'",
            row[0]
        );
        // age: parses as integer in [22, 45].
        let age: i64 = row[1].parse().expect("age should be an integer");
        assert!((22..=45).contains(&age), "age {age} out of range");
        // score: parses as float in [3.1, 4.9] (allow a tiny FP slack).
        let score: f64 = row[2].parse().expect("score should be a float");
        assert!(
            (3.1 - 1e-9..=4.9 + 1e-9).contains(&score),
            "score {score} out of range"
        );
        // active: real value set is {true, false}.
        assert!(
            row[3] == "true" || row[3] == "false",
            "unexpected active '{}'",
            row[3]
        );
        // join_date: YYYY-MM-DD format in [2019-01-01, 2023-12-31].
        assert!(
            row[4].len() == 10 && row[4].starts_with("20"),
            "unexpected join_date '{}'",
            row[4]
        );
    }
}

#[test]
fn synthesize_seed_is_reproducible() {
    let wrk = Workdir::new("synthesize_seed");
    wrk.create("data.csv", fixture());

    let mut first_cmd = wrk.command("synthesize");
    first_cmd.args(["-n", "20", "--seed", "7"]).arg("data.csv");
    let run1: String = wrk.stdout(&mut first_cmd);

    let mut second_cmd = wrk.command("synthesize");
    second_cmd.args(["-n", "20", "--seed", "7"]).arg("data.csv");
    let run2: String = wrk.stdout(&mut second_cmd);

    assert_eq!(run1, run2, "same seed should produce identical output");
}

#[test]
fn synthesize_rows_flag_controls_row_count() {
    let wrk = Workdir::new("synthesize_rows");
    wrk.create("data.csv", fixture());

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "7", "--seed", "1"]).arg("data.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    assert_eq!(got.len(), 8); // 1 header + 7 data rows
}

#[test]
fn synthesize_with_dictionary_uses_faker_for_email_column() {
    let wrk = Workdir::new("synthesize_with_dict");
    // The email column is HIGH cardinality (all unique) so frequency cannot
    // fully enumerate it — synthesize should fall through to the faker.
    wrk.create(
        "data.csv",
        vec![
            svec!["email", "tier"],
            svec!["a@example.com", "gold"],
            svec!["b@example.com", "silver"],
            svec!["c@example.com", "gold"],
            svec!["d@example.com", "bronze"],
            svec!["e@example.com", "silver"],
            svec!["f@example.com", "gold"],
            svec!["g@example.com", "silver"],
            svec!["h@example.com", "bronze"],
        ],
    );

    let dict_json = r#"{
        "fields": [
            {"name": "email", "type": "String", "content_type": "email"},
            {"name": "tier", "type": "String", "content_type": "unknown"}
        ],
        "enum_threshold": 10
    }"#;
    wrk.create_from_string("dict.json", dict_json);

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "30", "--seed", "13", "--dictionary"])
        .arg(wrk.path("dict.json"))
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    assert_eq!(got.len(), 31);
    let tiers: std::collections::HashSet<&str> = ["gold", "silver", "bronze"].into_iter().collect();
    for row in &got[1..] {
        assert!(
            row[0].contains('@'),
            "email '{}' should contain @ (faker-generated)",
            row[0]
        );
        assert!(
            tiers.contains(row[1].as_str()),
            "tier '{}' should be from the real set",
            row[1]
        );
    }
}

#[test]
fn synthesize_null_ratio_is_approximately_reproduced() {
    let wrk = Workdir::new("synthesize_nulls");
    // Build a 100-row fixture where the `note` column is ~30% empty.
    let mut rows: Vec<Vec<String>> = vec![svec!["id", "note"]];
    for i in 0..100 {
        let note = if i % 10 < 3 {
            String::new()
        } else {
            format!("note{i}")
        };
        rows.push(vec![format!("{i}"), note]);
    }
    wrk.create("data.csv", rows);

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "5000", "--seed", "31"]).arg("data.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    let empty = got[1..].iter().filter(|r| r[1].is_empty()).count();
    let ratio = empty as f64 / 5000.0;
    assert!(
        (ratio - 0.30).abs() < 0.05,
        "null ratio {ratio} not near 0.30"
    );
}

#[test]
fn synthesize_output_flag_writes_to_file() {
    let wrk = Workdir::new("synthesize_output");
    wrk.create("data.csv", fixture());

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "10", "--seed", "5", "--output"])
        .arg(wrk.path("out.csv"))
        .arg("data.csv");
    let _ = wrk.output(&mut cmd);

    let written = std::fs::read_to_string(wrk.path("out.csv")).unwrap();
    let lines: Vec<&str> = written.lines().collect();
    assert_eq!(lines.len(), 11); // header + 10 rows
    assert!(lines[0].starts_with("city,age,score,active,join_date"));
}

#[test]
fn synthesize_rejects_invalid_locale() {
    let wrk = Workdir::new("synthesize_invalid_locale");
    wrk.create("data.csv", fixture());

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "5", "--locale", "ZZ_ZZ"]).arg("data.csv");
    let err = wrk.output_stderr(&mut cmd);
    assert!(
        err.contains("Unsupported --locale") && err.contains("ZZ_ZZ"),
        "expected an unsupported-locale error mentioning the bad token, got: {err}"
    );
    assert!(
        err.contains("EN") && err.contains("FR_FR"),
        "error should list the supported locales, got: {err}"
    );
}

/// Faker columns under a non-EN locale should still produce N rows and the
/// expected column shape. We don't assert specific French/Japanese strings —
/// fake-rs data can shift across patch releases — so the test only verifies
/// the dispatch path executes cleanly and the email column still looks like
/// an email (the email faker is locale-aware but always produces "x@y.z").
#[test]
fn synthesize_accepts_fr_fr_locale() {
    let wrk = Workdir::new("synthesize_fr_fr");
    wrk.create(
        "data.csv",
        vec![
            svec!["email", "tier"],
            svec!["a@example.com", "gold"],
            svec!["b@example.com", "silver"],
            svec!["c@example.com", "gold"],
            svec!["d@example.com", "bronze"],
            svec!["e@example.com", "silver"],
            svec!["f@example.com", "gold"],
            svec!["g@example.com", "silver"],
            svec!["h@example.com", "bronze"],
        ],
    );

    let dict_json = r#"{
        "fields": [
            {"name": "email", "type": "String", "content_type": "email"},
            {"name": "tier", "type": "String", "content_type": "unknown"}
        ],
        "enum_threshold": 10
    }"#;
    wrk.create_from_string("dict.json", dict_json);

    let mut cmd = wrk.command("synthesize");
    cmd.args([
        "-n",
        "10",
        "--seed",
        "42",
        "--locale",
        "FR_FR",
        "--dictionary",
    ])
    .arg(wrk.path("dict.json"))
    .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    assert_eq!(got.len(), 11);
    for row in &got[1..] {
        assert!(row[0].contains('@'), "email '{}' should contain @", row[0]);
    }
}

#[test]
fn synthesize_locale_is_case_insensitive() {
    let wrk = Workdir::new("synthesize_locale_case");
    wrk.create("data.csv", fixture());

    let mut upper = wrk.command("synthesize");
    upper
        .args(["-n", "5", "--seed", "1", "--locale", "FR_FR"])
        .arg("data.csv");
    let upper_out: String = wrk.stdout(&mut upper);

    let mut lower = wrk.command("synthesize");
    lower
        .args(["-n", "5", "--seed", "1", "--locale", "fr_fr"])
        .arg("data.csv");
    let lower_out: String = wrk.stdout(&mut lower);

    assert_eq!(
        upper_out, lower_out,
        "lowercase and uppercase locale tokens should be equivalent"
    );
}

/// A `full_name` faker column under JA_JP vs EN should produce noticeably
/// different output — the Japanese name pool shares no surface tokens with
/// the English one. Use a name-only fixture so frequency-weighted enumeration
/// doesn't kick in.
#[test]
fn synthesize_locale_changes_output() {
    let wrk = Workdir::new("synthesize_locale_diff");
    let mut rows: Vec<Vec<String>> = vec![svec!["name"]];
    for i in 0..20 {
        rows.push(vec![format!("Person {i}")]);
    }
    wrk.create("data.csv", rows);

    let dict_json = r#"{
        "fields": [
            {"name": "name", "type": "String", "content_type": "full_name"}
        ],
        "enum_threshold": 10
    }"#;
    wrk.create_from_string("dict.json", dict_json);

    let mut en_cmd = wrk.command("synthesize");
    en_cmd
        .args(["-n", "10", "--seed", "42", "--locale", "EN", "--dictionary"])
        .arg(wrk.path("dict.json"))
        .arg("data.csv");
    let en_out: String = wrk.stdout(&mut en_cmd);

    let mut ja_cmd = wrk.command("synthesize");
    ja_cmd
        .args([
            "-n",
            "10",
            "--seed",
            "42",
            "--locale",
            "JA_JP",
            "--dictionary",
        ])
        .arg(wrk.path("dict.json"))
        .arg("data.csv");
    let ja_out: String = wrk.stdout(&mut ja_cmd);

    assert_ne!(
        en_out, ja_out,
        "EN and JA_JP full_name output should differ"
    );
}

#[test]
fn synthesize_rejects_zero_rows() {
    let wrk = Workdir::new("synthesize_zero");
    wrk.create("data.csv", fixture());

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "0"]).arg("data.csv");
    let err = wrk.output_stderr(&mut cmd);
    assert!(
        err.contains("greater than 0"),
        "expected a --rows error, got: {err}"
    );
}

#[test]
fn synthesize_rejects_missing_input() {
    let wrk = Workdir::new("synthesize_missing");

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "5"]).arg("does_not_exist.csv");
    let err = wrk.output_stderr(&mut cmd);
    assert!(
        err.contains("does not exist") || err.contains("not a file"),
        "expected a missing-input error, got: {err}"
    );
}
