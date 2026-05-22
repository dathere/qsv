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
fn synthesize_with_dictionary_applies_inferred_date_format() {
    // A describegpt dictionary can tag a Date/DateTime column with a chrono
    // strftime suffix (datetime:<fmt>); synthesize must emit generated values
    // in that format instead of its hardcoded RFC 3339 / %Y-%m-%d output.
    let wrk = Workdir::new("synthesize_date_format");
    wrk.create(
        "data.csv",
        vec![
            svec!["event_ts"],
            svec!["2019-03-04T12:00:00"],
            svec!["2020-07-15T08:30:00"],
            svec!["2021-01-22T23:45:00"],
            svec!["2018-11-30T06:00:00"],
            svec!["2022-05-09T17:10:00"],
            svec!["2017-09-01T00:00:00"],
            svec!["2023-12-25T14:20:00"],
            svec!["2016-02-29T09:05:00"],
        ],
    );

    let dict_json = r#"{
        "fields": [
            {"name": "event_ts", "type": "DateTime", "content_type": "datetime:%m/%d/%Y"}
        ],
        "enum_threshold": 10
    }"#;
    wrk.create_from_string("dict.json", dict_json);

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "25", "--seed", "13", "--dictionary"])
        .arg(wrk.path("dict.json"))
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    assert_eq!(got.len(), 26);
    for row in &got[1..] {
        let v = &row[0];
        // %m/%d/%Y → "MM/DD/YYYY": 10 chars, two slashes, all other chars
        // digits, and crucially NOT the default RFC 3339 (which contains 'T').
        assert!(
            v.len() == 10
                && v.matches('/').count() == 2
                && v.chars().all(|c| c.is_ascii_digit() || c == '/')
                && !v.contains('T'),
            "event_ts '{v}' is not in the inferred %m/%d/%Y format"
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

    // 1 header + 10 data rows.
    assert_eq!(got.len(), 11);
    // Header preserved verbatim from the source CSV.
    assert_eq!(got[0], svec!["email", "tier"]);
    let tiers: std::collections::HashSet<&str> = ["gold", "silver", "bronze"].into_iter().collect();
    for row in &got[1..] {
        // Exactly 2 fields per row (matches the source column count).
        assert_eq!(row.len(), 2, "row should have 2 fields: {row:?}");
        // Email faker output still looks like an email under FR_FR.
        assert!(row[0].contains('@'), "email '{}' should contain @", row[0]);
        // `tier` is enumerated, so the value must come from the real source set
        // regardless of locale.
        assert!(
            tiers.contains(row[1].as_str()),
            "tier '{}' should be from the real set",
            row[1]
        );
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

/// End-to-end check that `qsv stats`' string-length statistics are honored for
/// unstructured / free-text columns. The `blurb` column has varying-length
/// free-text values; `stats` provides `min_length`/`max_length`/`avg_length`/
/// `stddev_length`; `synthesize` should truncate generated values so their
/// character lengths fall within the source range. The dictionary marks the
/// column as `free_text` so it routes to the live-faker (no pool) path where
/// length stats apply.
#[test]
fn synthesize_respects_length_stats_for_free_text_column() {
    let wrk = Workdir::new("synthesize_length_stats");

    // 30 rows of free-text blurbs of varying lengths, all in [10, 40] chars.
    // Use distinct values to defeat frequency enumeration so the column lands
    // on the live-faker path.
    let mut rows: Vec<Vec<String>> = vec![svec!["id", "blurb"]];
    let templates = [
        "alpha bravo",                            // 11
        "charlie delta echo",                     // 18
        "foxtrot golf hotel india",               // 24
        "juliet kilo lima mike november",         // 30
        "oscar papa quebec romeo sierra tango u", // 38
    ];
    for i in 0..30 {
        let blurb = format!("{} {i:02}", templates[i % templates.len()]);
        rows.push(vec![format!("{i}"), blurb]);
    }
    wrk.create("data.csv", rows);

    let dict_json = r#"{
        "fields": [
            {"name": "id", "type": "Integer", "content_type": "unknown"},
            {"name": "blurb", "type": "String", "content_type": "free_text"}
        ],
        "enum_threshold": 10
    }"#;
    wrk.create_from_string("dict.json", dict_json);

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "200", "--seed", "2024", "--dictionary"])
        .arg(wrk.path("dict.json"))
        .arg("data.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // 1 header + 200 data rows.
    assert_eq!(got.len(), 201);
    assert_eq!(got[0], svec!["id", "blurb"]);

    // Every synthesized blurb must be at least 1 char and at most max_length
    // characters wide. We don't assert a tight lower bound — lorem fakers
    // can produce a string shorter than min_length and we don't pad — but
    // we DO assert the cap holds, which is the headline guarantee.
    let mut total = 0_usize;
    for (i, row) in got[1..].iter().enumerate() {
        let len = row[1].chars().count();
        assert!(
            (1..=41).contains(&len),
            "row {i}: blurb '{}' has {len} chars; expected in [1, 41] (max_length ≈ 41 with \
             id-suffix)",
            row[1]
        );
        total += len;
    }
    let mean = total as f64 / 200.0;
    // Source `avg_length` is ~24; with truncation the empirical mean should
    // stay within a generous band rather than blow past max_length.
    assert!(
        (10.0..=40.0).contains(&mean),
        "mean blurb length {mean} not within reasonable [10, 40] band"
    );
}

/// `--consistent-fakes`: a structured-faker column with bounded cardinality must
/// emit a fake (not the real source value), and each distinct source value must
/// map to a stable distinct fake across all output rows.
#[test]
fn synthesize_consistent_fakes_stable_mapping() {
    let wrk = Workdir::new("synthesize_consistent_fakes_stable");

    // `first_name` with three distinct source values × varied counts. The
    // `SRC_` prefix keeps the source values outside the fake-rs first-name
    // value space (`first_name` can legitimately emit "Michael", "Sarah", or
    // "Tom"), so the "no real value leaked" assertion below is robust to
    // fake-rs locale-dataset and version drift. With `--consistent-fakes` and
    // a dictionary that marks the column as `first_name`, the output must:
    //   * contain exactly 3 distinct fakes (one per distinct source value)
    //   * never contain any of the real source values (no SRC_-prefixed strings — the faker never
    //     emits those)
    wrk.create(
        "data.csv",
        vec![
            svec!["first_name"],
            svec!["SRC_Michael"],
            svec!["SRC_Michael"],
            svec!["SRC_Michael"],
            svec!["SRC_Sarah"],
            svec!["SRC_Sarah"],
            svec!["SRC_Tom"],
        ],
    );

    let dict_json = r#"{
        "fields": [
            {"name": "first_name", "type": "String", "content_type": "first_name"}
        ],
        "enum_threshold": 10
    }"#;
    wrk.create_from_string("dict.json", dict_json);

    let mut cmd = wrk.command("synthesize");
    cmd.args([
        "-n",
        "200",
        "--seed",
        "42",
        "--consistent-fakes",
        "--dictionary",
    ])
    .arg(wrk.path("dict.json"))
    .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got.len(), 201);

    let real_values: std::collections::HashSet<&str> = ["SRC_Michael", "SRC_Sarah", "SRC_Tom"]
        .into_iter()
        .collect();
    let mut distinct_fakes: std::collections::HashSet<String> = std::collections::HashSet::new();
    for row in &got[1..] {
        let v = &row[0];
        assert!(
            !real_values.contains(v.as_str()),
            "real source value '{v}' leaked into output — `--consistent-fakes` should emit fakes, \
             not real values"
        );
        assert!(!v.is_empty(), "empty fake should not be emitted");
        distinct_fakes.insert(v.clone());
    }
    // Exactly one fake per distinct source value (no source has a null ratio
    // in this fixture, so all 3 source values are represented across 200 rows).
    assert_eq!(
        distinct_fakes.len(),
        3,
        "expected 3 distinct fakes (one per distinct source value), got {distinct_fakes:?}"
    );
}

/// `--consistent-fakes`: the source frequency distribution must be reproduced
/// in the output — the mapping changes the *labels*, not the proportions.
#[test]
fn synthesize_consistent_fakes_distribution_preserved() {
    let wrk = Workdir::new("synthesize_consistent_fakes_distribution");

    // 6 / 4 / 2 (50% / 33% / 17%) over 12 source rows.
    let mut rows: Vec<Vec<String>> = vec![svec!["first_name"]];
    for _ in 0..6 {
        rows.push(svec!["SRC_Michael"]);
    }
    for _ in 0..4 {
        rows.push(svec!["SRC_Sarah"]);
    }
    for _ in 0..2 {
        rows.push(svec!["SRC_Tom"]);
    }
    wrk.create("data.csv", rows);

    let dict_json = r#"{
        "fields": [
            {"name": "first_name", "type": "String", "content_type": "first_name"}
        ],
        "enum_threshold": 10
    }"#;
    wrk.create_from_string("dict.json", dict_json);

    let mut cmd = wrk.command("synthesize");
    cmd.args([
        "-n",
        "1200",
        "--seed",
        "42",
        "--consistent-fakes",
        "--dictionary",
    ])
    .arg(wrk.path("dict.json"))
    .arg("data.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got.len(), 1201);

    let mut counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for row in &got[1..] {
        *counts.entry(row[0].clone()).or_insert(0) += 1;
    }
    assert_eq!(
        counts.len(),
        3,
        "expected 3 distinct fakes, got {}: {counts:?}",
        counts.len()
    );

    let mut sorted: Vec<_> = counts.values().copied().collect();
    sorted.sort_unstable();
    sorted.reverse(); // descending: most-frequent first
    // Expected ratios 6/4/2 over 1200 rows → ~600/400/200. Allow ±15% band.
    let bands = [(510, 690), (340, 460), (170, 230)];
    for (i, (lo, hi)) in bands.iter().enumerate() {
        assert!(
            (*lo..=*hi).contains(&sorted[i]),
            "rank-{i} count {} outside expected band [{lo}, {hi}] (full counts: {counts:?})",
            sorted[i]
        );
    }
}

/// `--consistent-fakes` must remain seed-reproducible: same input + same seed
/// → byte-identical output across two runs.
#[test]
fn synthesize_consistent_fakes_seed_reproducible() {
    let wrk = Workdir::new("synthesize_consistent_fakes_seed");
    wrk.create(
        "data.csv",
        vec![
            svec!["first_name"],
            svec!["SRC_Michael"],
            svec!["SRC_Michael"],
            svec!["SRC_Sarah"],
            svec!["SRC_Tom"],
        ],
    );
    let dict_json = r#"{
        "fields": [
            {"name": "first_name", "type": "String", "content_type": "first_name"}
        ],
        "enum_threshold": 10
    }"#;
    wrk.create_from_string("dict.json", dict_json);

    let args = [
        "-n",
        "50",
        "--seed",
        "7",
        "--consistent-fakes",
        "--dictionary",
    ];

    let mut first_cmd = wrk.command("synthesize");
    first_cmd
        .args(args)
        .arg(wrk.path("dict.json"))
        .arg("data.csv");
    let run1: String = wrk.stdout(&mut first_cmd);

    let mut second_cmd = wrk.command("synthesize");
    second_cmd
        .args(args)
        .arg(wrk.path("dict.json"))
        .arg("data.csv");
    let run2: String = wrk.stdout(&mut second_cmd);

    assert_eq!(
        run1, run2,
        "same seed + --consistent-fakes should produce identical output"
    );
}

/// Without `--consistent-fakes`, the default behavior for a frequency-enumerated
/// column is unchanged: real source values are emitted (regression guard so the
/// new branch only fires when the flag is set).
#[test]
fn synthesize_consistent_fakes_off_preserves_default() {
    let wrk = Workdir::new("synthesize_consistent_fakes_off");
    wrk.create(
        "data.csv",
        vec![
            svec!["first_name"],
            svec!["SRC_Michael"],
            svec!["SRC_Michael"],
            svec!["SRC_Sarah"],
            svec!["SRC_Tom"],
        ],
    );
    let dict_json = r#"{
        "fields": [
            {"name": "first_name", "type": "String", "content_type": "first_name"}
        ],
        "enum_threshold": 10
    }"#;
    wrk.create_from_string("dict.json", dict_json);

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "100", "--seed", "7", "--dictionary"])
        .arg(wrk.path("dict.json"))
        .arg("data.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got.len(), 101);

    // Without the flag, frequency-enumeration wins: every output row must be
    // one of the real source values.
    let real_values: std::collections::HashSet<&str> = ["SRC_Michael", "SRC_Sarah", "SRC_Tom"]
        .into_iter()
        .collect();
    for row in &got[1..] {
        assert!(
            real_values.contains(row[0].as_str()),
            "without --consistent-fakes the column should emit real values; got '{}'",
            row[0]
        );
    }
}

/// `--consistent-fakes` must NOT steal unstructured-text columns. A column
/// inferred as `free_text` should still route to the regular `Faker` (or
/// `LoremFallback`) path with length-stat truncation intact.
#[test]
fn synthesize_consistent_fakes_unstructured_passthrough() {
    let wrk = Workdir::new("synthesize_consistent_fakes_unstructured");

    // Distinct values to defeat full enumeration; gives `free_text` the live
    // faker path, which is where length-stat truncation lives.
    let mut rows: Vec<Vec<String>> = vec![svec!["blurb"]];
    for i in 0..30 {
        rows.push(vec![format!("alpha bravo charlie delta {i:02}")]); // ~28 chars
    }
    wrk.create("data.csv", rows);

    let dict_json = r#"{
        "fields": [
            {"name": "blurb", "type": "String", "content_type": "free_text"}
        ],
        "enum_threshold": 10
    }"#;
    wrk.create_from_string("dict.json", dict_json);

    let mut cmd = wrk.command("synthesize");
    cmd.args([
        "-n",
        "50",
        "--seed",
        "11",
        "--consistent-fakes",
        "--dictionary",
    ])
    .arg(wrk.path("dict.json"))
    .arg("data.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got.len(), 51);

    // Length cap from `stats` should still kick in — verify no row balloons
    // beyond a small multiple of the source max (~30 chars).
    for (i, row) in got[1..].iter().enumerate() {
        let len = row[0].chars().count();
        assert!(
            len <= 80,
            "row {i}: --consistent-fakes must not disable length-stat truncation for free_text; \
             got {len} chars: '{}'",
            row[0]
        );
    }
}

#[test]
fn synthesize_date_column_ignores_time_content_type() {
    // Regression test: a Date/DateTime column tagged by the LLM (incorrectly)
    // with `content_type: "time"` (or any other temporal/faker token) must
    // still go through `build_date()` so the synthetic values are real dates,
    // not time-of-day strings like "14:30:45". The dictionary's design intent
    // is that real date/datetime fields stay tagged `unknown`; this guards
    // against the LLM contract violation.
    //
    // Fixture uses `T`-separated naive datetimes (`YYYY-MM-DDTHH:MM:SS`) —
    // the form that `datetime.isoformat()` and most ISO 8601 emitters produce
    // without a timezone suffix. Supported by `qsv stats --infer-dates` as
    // of qsv-dateparser 0.15.0 (earlier 0.14.0 only recognized the
    // space-separated form, which is why an earlier revision of this test
    // used `YYYY-MM-DD HH:MM:SS`). Exercising the `T`-separated form here
    // double-validates the cleanup: parser correctly classifies the column
    // as `DateTime`, AND the guard in `ColumnGenerator::build()` prevents
    // the dictionary-supplied `content_type="time"` from intercepting it.
    let wrk = Workdir::new("synthesize_date_ignores_time_ct");

    // Date column with all-unique values to push past `try_frequency_weighted`
    // so the faker branch would otherwise fire. DateTime column likewise.
    wrk.create(
        "data.csv",
        vec![
            svec!["join_date", "last_seen"],
            svec!["2020-01-15", "2020-01-15T08:00:00"],
            svec!["2020-06-20", "2020-06-20T09:15:00"],
            svec!["2020-11-05", "2020-11-05T10:30:00"],
            svec!["2021-03-10", "2021-03-10T11:45:00"],
            svec!["2021-08-22", "2021-08-22T12:00:00"],
            svec!["2022-02-28", "2022-02-28T13:15:00"],
            svec!["2022-07-15", "2022-07-15T14:30:00"],
            svec!["2022-12-01", "2022-12-01T15:45:00"],
            svec!["2023-05-30", "2023-05-30T16:00:00"],
            svec!["2023-09-14", "2023-09-14T17:15:00"],
        ],
    );

    // Dictionary INTENTIONALLY tags both columns with `time` (a content type
    // that, before the fix, would route to the time-of-day faker and emit
    // strings like "14:30:45" instead of dates).
    let dict_json = r#"{
        "fields": [
            {"name": "join_date", "type": "Date", "content_type": "time"},
            {"name": "last_seen", "type": "DateTime", "content_type": "time"}
        ],
        "enum_threshold": 5
    }"#;
    wrk.create_from_string("dict.json", dict_json);

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "20", "--seed", "42", "--dictionary"])
        .arg(wrk.path("dict.json"))
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got.len(), 21);

    // The Date column must be `YYYY-MM-DD` (10 chars, two `-` separators, no
    // `:`); the DateTime column must lead with the same date shape followed
    // by `T` or ` ` and an `HH:MM:SS` time. A time-of-day faker (the pre-fix
    // behavior for `content_type="time"`) would emit `HH:MM:SS` for the Date
    // column — failing both the length and the `:` check.
    let looks_like_date = |s: &str| -> bool {
        s.len() == 10
            && s.as_bytes()[4] == b'-'
            && s.as_bytes()[7] == b'-'
            && !s.contains(':')
            && s.chars()
                .filter(|c| c.is_ascii_digit() || *c == '-')
                .count()
                == s.len()
    };
    let looks_like_datetime = |s: &str| -> bool {
        s.len() >= 19
            && looks_like_date(&s[..10])
            && (s.as_bytes()[10] == b'T' || s.as_bytes()[10] == b' ')
            && s[11..19].chars().enumerate().all(|(i, c)| match i {
                2 | 5 => c == ':',
                _ => c.is_ascii_digit(),
            })
    };

    for (i, row) in got[1..].iter().enumerate() {
        assert!(
            looks_like_date(&row[0]),
            "row {i}: Date column tagged content_type=time must still emit YYYY-MM-DD; got '{}'",
            row[0]
        );
        assert!(
            looks_like_datetime(&row[1]),
            "row {i}: DateTime column tagged content_type=time must still emit YYYY-MM-DD[T \
             ]HH:MM:SS; got '{}'",
            row[1]
        );
    }
}

// ---- Relationships: joint (categorical / functional dependency) ----------

/// city/state/zip data with a functional dependency: only six (city, state,
/// zip) triples co-occur. "Springfield" appears in both IL and MA, so city
/// alone does not determine state — only the whole triple is meaningful.
fn geo_fixture() -> Vec<Vec<String>> {
    vec![
        svec!["city", "state", "zip"],
        svec!["Springfield", "IL", "62701"],
        svec!["Springfield", "MA", "01101"],
        svec!["Chicago", "IL", "60601"],
        svec!["Boston", "MA", "02101"],
        svec!["Portland", "OR", "97201"],
        svec!["Portland", "ME", "04101"],
        svec!["Springfield", "IL", "62701"],
        svec!["Chicago", "IL", "60601"],
        svec!["Boston", "MA", "02101"],
        svec!["Portland", "OR", "97201"],
    ]
}

/// The six valid (city, state, zip) triples in `geo_fixture`.
fn geo_valid_triples() -> std::collections::HashSet<(String, String, String)> {
    geo_fixture()[1..]
        .iter()
        .map(|r| (r[0].clone(), r[1].clone(), r[2].clone()))
        .collect()
}

const GEO_JOINT_DICT: &str = r#"{
    "fields": [
        {"name": "city", "type": "String"},
        {"name": "state", "type": "String"},
        {"name": "zip", "type": "String"}
    ],
    "relationships": [
        {"kind": "joint", "members": ["city", "state", "zip"]}
    ]
}"#;

#[test]
fn synthesize_joint_categorical_only_real_tuples() {
    let wrk = Workdir::new("synthesize_joint_real_tuples");
    wrk.create("data.csv", geo_fixture());
    wrk.create_from_string("dict.json", GEO_JOINT_DICT);

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "200", "--seed", "42", "--dictionary"])
        .arg(wrk.path("dict.json"))
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got.len(), 201);
    assert_eq!(got[0], svec!["city", "state", "zip"]);

    let valid = geo_valid_triples();
    for row in &got[1..] {
        let triple = (row[0].clone(), row[1].clone(), row[2].clone());
        assert!(
            valid.contains(&triple),
            "joint synthesis emitted an invented combination: {triple:?}"
        );
    }
}

#[test]
fn synthesize_relationships_seed_reproducible() {
    let wrk = Workdir::new("synthesize_joint_seed");
    wrk.create("data.csv", geo_fixture());
    wrk.create_from_string("dict.json", GEO_JOINT_DICT);

    let mut first = wrk.command("synthesize");
    first
        .args(["-n", "60", "--seed", "42", "--dictionary"])
        .arg(wrk.path("dict.json"))
        .arg("data.csv");
    let run1: String = wrk.stdout(&mut first);

    let mut second = wrk.command("synthesize");
    second
        .args(["-n", "60", "--seed", "42", "--dictionary"])
        .arg(wrk.path("dict.json"))
        .arg("data.csv");
    let run2: String = wrk.stdout(&mut second);

    assert_eq!(run1, run2, "joint synthesis must be seed-reproducible");
}

#[test]
fn synthesize_joint_cardinality_cap_degrades() {
    // Six distinct triples, cap of 3 → the joint group degrades to independent
    // generation. Synthesis must still succeed and produce the requested rows.
    let wrk = Workdir::new("synthesize_joint_cap_degrade");
    wrk.create("data.csv", geo_fixture());
    wrk.create_from_string("dict.json", GEO_JOINT_DICT);

    let mut cmd = wrk.command("synthesize");
    cmd.args([
        "-n",
        "25",
        "--seed",
        "42",
        "--joint-cardinality-cap",
        "3",
        "--dictionary",
    ])
    .arg(wrk.path("dict.json"))
    .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got.len(), 26, "should degrade gracefully, not abort");
}

#[test]
fn synthesize_joint_cardinality_cap_strict_aborts() {
    // Same over-cap situation, but --strict-relationships turns the degrade
    // into a hard error.
    let wrk = Workdir::new("synthesize_joint_cap_strict");
    wrk.create("data.csv", geo_fixture());
    wrk.create_from_string("dict.json", GEO_JOINT_DICT);

    let mut cmd = wrk.command("synthesize");
    cmd.args([
        "-n",
        "25",
        "--seed",
        "42",
        "--joint-cardinality-cap",
        "3",
        "--strict-relationships",
        "--dictionary",
    ])
    .arg(wrk.path("dict.json"))
    .arg("data.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
fn synthesize_no_relationships_flag_disables_joint() {
    // --no-relationships must make a joint dictionary behave exactly like a
    // dictionary with no relationships at all.
    let wrk = Workdir::new("synthesize_no_relationships");
    wrk.create("data.csv", geo_fixture());
    wrk.create_from_string("dict_joint.json", GEO_JOINT_DICT);
    wrk.create_from_string(
        "dict_plain.json",
        r#"{
            "fields": [
                {"name": "city", "type": "String"},
                {"name": "state", "type": "String"},
                {"name": "zip", "type": "String"}
            ]
        }"#,
    );

    let mut joint_off = wrk.command("synthesize");
    joint_off
        .args([
            "-n",
            "40",
            "--seed",
            "9",
            "--no-relationships",
            "--dictionary",
        ])
        .arg(wrk.path("dict_joint.json"))
        .arg("data.csv");
    let with_flag: String = wrk.stdout(&mut joint_off);

    let mut plain = wrk.command("synthesize");
    plain
        .args(["-n", "40", "--seed", "9", "--dictionary"])
        .arg(wrk.path("dict_plain.json"))
        .arg("data.csv");
    let plain_out: String = wrk.stdout(&mut plain);

    assert_eq!(
        with_flag, plain_out,
        "--no-relationships should match a relationship-free dictionary"
    );
}

#[test]
fn synthesize_relationship_invalid_member_dropped() {
    // A relationship that names a column which does not exist is silently
    // dropped; synthesis still succeeds.
    let wrk = Workdir::new("synthesize_rel_bad_member");
    wrk.create("data.csv", geo_fixture());
    wrk.create_from_string(
        "dict.json",
        r#"{
            "fields": [
                {"name": "city", "type": "String"},
                {"name": "state", "type": "String"},
                {"name": "zip", "type": "String"}
            ],
            "relationships": [
                {"kind": "joint", "members": ["city", "does_not_exist"]}
            ]
        }"#,
    );

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "15", "--seed", "42", "--dictionary"])
        .arg(wrk.path("dict.json"))
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got.len(), 16);
}

// ---- Relationships: ordered (anchor + learned gap) -----------------------

/// Ticket data where `created_date <= closed_date` holds in every row.
fn ticket_fixture() -> Vec<Vec<String>> {
    vec![
        svec!["created_date", "closed_date"],
        svec!["2020-01-10", "2020-01-25"],
        svec!["2020-02-05", "2020-03-10"],
        svec!["2020-03-15", "2020-03-20"],
        svec!["2020-04-01", "2020-05-15"],
        svec!["2020-05-20", "2020-06-01"],
        svec!["2020-06-10", "2020-08-22"],
        svec!["2020-07-04", "2020-07-30"],
        svec!["2020-08-18", "2020-09-09"],
        svec!["2020-09-25", "2020-11-01"],
        svec!["2020-10-30", "2020-12-15"],
        svec!["2020-11-11", "2020-11-29"],
        svec!["2020-12-01", "2021-01-20"],
    ]
}

const TICKET_ORDERED_DICT: &str = r#"{
    "fields": [
        {"name": "created_date", "type": "Date"},
        {"name": "closed_date", "type": "Date"}
    ],
    "relationships": [
        {"kind": "ordered", "members": ["created_date", "closed_date"], "anchor": "created_date"}
    ]
}"#;

#[test]
fn synthesize_ordered_dates_preserves_order() {
    let wrk = Workdir::new("synthesize_ordered_dates");
    wrk.create("data.csv", ticket_fixture());
    wrk.create_from_string("dict.json", TICKET_ORDERED_DICT);

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "300", "--seed", "42", "--dictionary"])
        .arg(wrk.path("dict.json"))
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got.len(), 301);
    assert_eq!(got[0], svec!["created_date", "closed_date"]);

    for (i, row) in got[1..].iter().enumerate() {
        // ISO `%Y-%m-%d` dates compare chronologically as strings.
        assert!(
            row[0].len() == 10 && row[1].len() == 10,
            "row {i}: expected YYYY-MM-DD dates, got {row:?}"
        );
        assert!(
            row[0] <= row[1],
            "row {i}: created_date '{}' is after closed_date '{}'",
            row[0],
            row[1]
        );
    }
}

#[test]
fn synthesize_ordered_numeric_offset() {
    // total = subtotal + tax, so total >= subtotal in every source row.
    let wrk = Workdir::new("synthesize_ordered_numeric");
    wrk.create(
        "data.csv",
        vec![
            svec!["subtotal", "total"],
            svec!["100", "108"],
            svec!["250", "270"],
            svec!["50", "54"],
            svec!["500", "540"],
            svec!["75", "81"],
            svec!["320", "346"],
            svec!["180", "194"],
            svec!["90", "97"],
            svec!["410", "443"],
            svec!["260", "281"],
            svec!["140", "151"],
            svec!["600", "648"],
        ],
    );
    wrk.create_from_string(
        "dict.json",
        r#"{
            "fields": [
                {"name": "subtotal", "type": "Integer"},
                {"name": "total", "type": "Integer"}
            ],
            "relationships": [
                {"kind": "ordered", "members": ["subtotal", "total"]}
            ]
        }"#,
    );

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "300", "--seed", "7", "--dictionary"])
        .arg(wrk.path("dict.json"))
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got.len(), 301);
    for (i, row) in got[1..].iter().enumerate() {
        let subtotal: i64 = row[0].parse().expect("subtotal should be an integer");
        let total: i64 = row[1].parse().expect("total should be an integer");
        assert!(
            total >= subtotal,
            "row {i}: total {total} is less than subtotal {subtotal}"
        );
    }
}

/// Ticket data where the source itself violates the declared order — every
/// `closed_date` precedes its `created_date`.
fn out_of_order_fixture() -> Vec<Vec<String>> {
    vec![
        svec!["created_date", "closed_date"],
        svec!["2020-06-01", "2020-01-01"],
        svec!["2020-07-01", "2020-02-01"],
        svec!["2020-08-01", "2020-03-01"],
        svec!["2020-09-01", "2020-04-01"],
        svec!["2020-10-01", "2020-05-01"],
        svec!["2020-11-01", "2020-06-01"],
        svec!["2020-12-01", "2020-07-01"],
        svec!["2021-01-01", "2020-08-01"],
    ]
}

#[test]
fn synthesize_ordered_strict_aborts_on_violation() {
    let wrk = Workdir::new("synthesize_ordered_strict");
    wrk.create("data.csv", out_of_order_fixture());
    wrk.create_from_string("dict.json", TICKET_ORDERED_DICT);

    let mut cmd = wrk.command("synthesize");
    cmd.args([
        "-n",
        "20",
        "--seed",
        "42",
        "--strict-relationships",
        "--dictionary",
    ])
    .arg(wrk.path("dict.json"))
    .arg("data.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
fn synthesize_ordered_clamps_source_violations() {
    // Without --strict-relationships, a source that violates the declared
    // order still yields ordered output: negative gaps are clamped to zero.
    let wrk = Workdir::new("synthesize_ordered_clamp");
    wrk.create("data.csv", out_of_order_fixture());
    wrk.create_from_string("dict.json", TICKET_ORDERED_DICT);

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "100", "--seed", "42", "--dictionary"])
        .arg(wrk.path("dict.json"))
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got.len(), 101);
    for (i, row) in got[1..].iter().enumerate() {
        assert!(
            row[0] <= row[1],
            "row {i}: created_date '{}' is after closed_date '{}' despite clamping",
            row[0],
            row[1]
        );
    }
}

// ---- Relationships: correlated (Gaussian copula) -------------------------

/// Spearman rank correlation of two equal-length samples — used to check that
/// synthesized output preserves the source's correlation structure.
fn spearman(a: &[f64], b: &[f64]) -> f64 {
    fn ranks(v: &[f64]) -> Vec<f64> {
        let n = v.len();
        let mut idx: Vec<usize> = (0..n).collect();
        idx.sort_by(|&i, &j| v[i].total_cmp(&v[j]));
        let mut r = vec![0.0_f64; n];
        let mut i = 0;
        while i < n {
            let mut j = i;
            while j + 1 < n && v[idx[j + 1]] == v[idx[i]] {
                j += 1;
            }
            let avg = (i + j) as f64 / 2.0 + 1.0;
            for &o in &idx[i..=j] {
                r[o] = avg;
            }
            i = j + 1;
        }
        r
    }
    let (ra, rb) = (ranks(a), ranks(b));
    let n = a.len() as f64;
    let ma = ra.iter().sum::<f64>() / n;
    let mb = rb.iter().sum::<f64>() / n;
    let (mut cov, mut va, mut vb) = (0.0_f64, 0.0_f64, 0.0_f64);
    for (x, y) in ra.iter().zip(&rb) {
        cov += (x - ma) * (y - mb);
        va += (x - ma).powi(2);
        vb += (y - mb).powi(2);
    }
    cov / (va.sqrt() * vb.sqrt())
}

/// Two strongly positively correlated numeric columns.
fn correlated_fixture() -> Vec<Vec<String>> {
    vec![
        svec!["x", "y"],
        svec!["10", "12"],
        svec!["20", "19"],
        svec!["30", "35"],
        svec!["40", "38"],
        svec!["50", "55"],
        svec!["60", "58"],
        svec!["70", "76"],
        svec!["80", "79"],
        svec!["90", "95"],
        svec!["100", "102"],
        svec!["15", "14"],
        svec!["35", "40"],
        svec!["55", "51"],
        svec!["75", "80"],
        svec!["95", "90"],
    ]
}

const CORRELATED_DICT: &str = r#"{
    "fields": [
        {"name": "x", "type": "Integer"},
        {"name": "y", "type": "Integer"}
    ],
    "relationships": [
        {"kind": "correlated", "members": ["x", "y"]}
    ]
}"#;

#[test]
fn synthesize_correlated_numeric_preserves_correlation() {
    let wrk = Workdir::new("synthesize_correlated_corr");
    wrk.create("data.csv", correlated_fixture());
    wrk.create_from_string("dict.json", CORRELATED_DICT);

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "600", "--seed", "42", "--dictionary"])
        .arg(wrk.path("dict.json"))
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got.len(), 601);

    let xs: Vec<f64> = got[1..].iter().map(|r| r[0].parse().unwrap()).collect();
    let ys: Vec<f64> = got[1..].iter().map(|r| r[1].parse().unwrap()).collect();
    let rho = spearman(&xs, &ys);
    assert!(
        rho > 0.5,
        "copula should preserve the strong positive correlation; got Spearman {rho}"
    );
}

#[test]
fn synthesize_correlated_marginals_unchanged() {
    // The copula couples columns without distorting either marginal: each
    // column stays inside its source [min, max] and still spans the range.
    let wrk = Workdir::new("synthesize_correlated_marginals");
    wrk.create("data.csv", correlated_fixture());
    wrk.create_from_string("dict.json", CORRELATED_DICT);

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "500", "--seed", "7", "--dictionary"])
        .arg(wrk.path("dict.json"))
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let xs: Vec<f64> = got[1..].iter().map(|r| r[0].parse().unwrap()).collect();
    let ys: Vec<f64> = got[1..].iter().map(|r| r[1].parse().unwrap()).collect();

    for (i, &x) in xs.iter().enumerate() {
        assert!(
            (10.0..=100.0).contains(&x),
            "row {i}: x {x} out of source range"
        );
    }
    for (i, &y) in ys.iter().enumerate() {
        assert!(
            (12.0..=102.0).contains(&y),
            "row {i}: y {y} out of source range"
        );
    }
    // The marginal must still cover the range, not collapse to a point.
    let x_min = xs.iter().copied().fold(f64::INFINITY, f64::min);
    let x_max = xs.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    assert!(
        x_min < 30.0 && x_max > 80.0,
        "x marginal collapsed: observed [{x_min}, {x_max}]"
    );
}

#[test]
fn synthesize_correlated_seed_reproducible() {
    let wrk = Workdir::new("synthesize_correlated_seed");
    wrk.create("data.csv", correlated_fixture());
    wrk.create_from_string("dict.json", CORRELATED_DICT);

    let mut first = wrk.command("synthesize");
    first
        .args(["-n", "80", "--seed", "42", "--dictionary"])
        .arg(wrk.path("dict.json"))
        .arg("data.csv");
    let run1: String = wrk.stdout(&mut first);

    let mut second = wrk.command("synthesize");
    second
        .args(["-n", "80", "--seed", "42", "--dictionary"])
        .arg(wrk.path("dict.json"))
        .arg("data.csv");
    let run2: String = wrk.stdout(&mut second);

    assert_eq!(run1, run2, "copula synthesis must be seed-reproducible");
}
