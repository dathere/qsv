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
    cmd.args(["--min-count", "1", "-n", "50", "--seed", "42"])
        .arg("data.csv");

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
    first_cmd
        .args(["--min-count", "1", "-n", "20", "--seed", "7"])
        .arg("data.csv");
    let run1: String = wrk.stdout(&mut first_cmd);

    let mut second_cmd = wrk.command("synthesize");
    second_cmd
        .args(["--min-count", "1", "-n", "20", "--seed", "7"])
        .arg("data.csv");
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
    cmd.args([
        "--min-count",
        "1",
        "-n",
        "30",
        "--seed",
        "13",
        "--dictionary",
    ])
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
    cmd.args([
        "--min-count",
        "1",
        "-n",
        "25",
        "--seed",
        "13",
        "--dictionary",
    ])
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
    cmd.args(["--min-count", "1", "-n", "5000", "--seed", "31"])
        .arg("data.csv");
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
        "--min-count",
        "1",
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

/// A `full_name` faker column under `JA_JP` vs EN should produce noticeably
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
    cmd.args([
        "--min-count",
        "1",
        "-n",
        "100",
        "--seed",
        "7",
        "--dictionary",
    ])
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
    cmd.args([
        "--min-count",
        "1",
        "-n",
        "20",
        "--seed",
        "42",
        "--dictionary",
    ])
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
    cmd.args([
        "--min-count",
        "1",
        "-n",
        "200",
        "--seed",
        "42",
        "--dictionary",
    ])
    .arg(wrk.path("dict.json"))
    .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
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
        .args([
            "--min-count",
            "1",
            "-n",
            "60",
            "--seed",
            "42",
            "--dictionary",
        ])
        .arg(wrk.path("dict.json"))
        .arg("data.csv");
    let run1: String = wrk.stdout(&mut first);

    let mut second = wrk.command("synthesize");
    second
        .args([
            "--min-count",
            "1",
            "-n",
            "60",
            "--seed",
            "42",
            "--dictionary",
        ])
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
        "--min-count",
        "1",
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

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
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
        "--min-count",
        "1",
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
            "--min-count",
            "1",
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
        .args([
            "--min-count",
            "1",
            "-n",
            "40",
            "--seed",
            "9",
            "--dictionary",
        ])
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
    cmd.args([
        "--min-count",
        "1",
        "-n",
        "15",
        "--seed",
        "42",
        "--dictionary",
    ])
    .arg(wrk.path("dict.json"))
    .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
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
    cmd.args([
        "--min-count",
        "1",
        "-n",
        "300",
        "--seed",
        "42",
        "--dictionary",
    ])
    .arg(wrk.path("dict.json"))
    .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
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
    cmd.args([
        "--min-count",
        "1",
        "-n",
        "300",
        "--seed",
        "7",
        "--dictionary",
    ])
    .arg(wrk.path("dict.json"))
    .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
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

#[test]
fn synthesize_ordered_mixed_int_float_keeps_per_member_type() {
    // An ordered group with a mix of Integer and Float members must keep each
    // column's declared type — the Integer member must not drift to a float
    // string just because the group also contains a Float member.
    let wrk = Workdir::new("synthesize_ordered_mixed");
    wrk.create(
        "data.csv",
        vec![
            svec!["count", "amount"],
            svec!["3", "10.5"],
            svec!["7", "25.2"],
            svec!["2", "8.1"],
            svec!["12", "40.9"],
            svec!["5", "18.3"],
            svec!["9", "33.7"],
            svec!["4", "14.0"],
            svec!["11", "38.5"],
        ],
    );
    wrk.create_from_string(
        "dict.json",
        r#"{
            "fields": [
                {"name": "count", "type": "Integer"},
                {"name": "amount", "type": "Float"}
            ],
            "relationships": [
                {"kind": "ordered", "members": ["count", "amount"]}
            ]
        }"#,
    );

    let mut cmd = wrk.command("synthesize");
    cmd.args([
        "--min-count",
        "1",
        "-n",
        "200",
        "--seed",
        "42",
        "--dictionary",
    ])
    .arg(wrk.path("dict.json"))
    .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    assert_eq!(got.len(), 201);
    for (i, row) in got[1..].iter().enumerate() {
        // The Integer member must render as an integer — no decimal point.
        assert!(
            !row[0].contains('.') && row[0].parse::<i64>().is_ok(),
            "row {i}: count '{}' should be an integer, not a float string",
            row[0]
        );
        // The Float member parses as a float.
        assert!(
            row[1].parse::<f64>().is_ok(),
            "row {i}: amount '{}' should parse as a float",
            row[1]
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
        "--min-count",
        "1",
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
    cmd.args([
        "--min-count",
        "1",
        "-n",
        "100",
        "--seed",
        "42",
        "--dictionary",
    ])
    .arg(wrk.path("dict.json"))
    .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
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
    cmd.args([
        "--min-count",
        "1",
        "-n",
        "600",
        "--seed",
        "42",
        "--dictionary",
    ])
    .arg(wrk.path("dict.json"))
    .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
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
    cmd.args([
        "--min-count",
        "1",
        "-n",
        "500",
        "--seed",
        "7",
        "--dictionary",
    ])
    .arg(wrk.path("dict.json"))
    .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
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
        .args([
            "--min-count",
            "1",
            "-n",
            "80",
            "--seed",
            "42",
            "--dictionary",
        ])
        .arg(wrk.path("dict.json"))
        .arg("data.csv");
    let run1: String = wrk.stdout(&mut first);

    let mut second = wrk.command("synthesize");
    second
        .args([
            "--min-count",
            "1",
            "-n",
            "80",
            "--seed",
            "42",
            "--dictionary",
        ])
        .arg(wrk.path("dict.json"))
        .arg("data.csv");
    let run2: String = wrk.stdout(&mut second);

    assert_eq!(run1, run2, "copula synthesis must be seed-reproducible");
}

// --- --min-count privacy threshold ----------------------------------------

/// One enumerated column: "A" x20, "B" x10, and a singleton "RARE".
fn min_count_fixture() -> Vec<Vec<String>> {
    let mut rows = vec![svec!["cat"]];
    rows.extend(std::iter::repeat_n(svec!["A"], 20));
    rows.extend(std::iter::repeat_n(svec!["B"], 10));
    rows.push(svec!["RARE"]);
    rows
}

fn count_cat(got: &[Vec<String>], value: &str) -> usize {
    got[1..].iter().filter(|r| r[0] == value).count()
}

#[test]
fn synthesize_min_count_default_suppresses_rare_values() {
    let wrk = Workdir::new("synthesize_min_count_default");
    wrk.create("data.csv", min_count_fixture());

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "2000", "--seed", "42"]).arg("data.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got.len(), 2001);

    assert_eq!(
        count_cat(&got, "RARE"),
        0,
        "singleton leaked at default --min-count"
    );
    let a = count_cat(&got, "A");
    let b = count_cat(&got, "B");
    assert_eq!(a + b, 2000, "only kept values should be emitted");
    // The suppressed share is redistributed over the kept values: 20:10 → 2/3.
    #[allow(clippy::cast_precision_loss)]
    let ratio = a as f64 / 2000.0;
    assert!(
        (ratio - 2.0 / 3.0).abs() < 0.04,
        "A ratio {ratio} not near 2/3"
    );
}

#[test]
fn synthesize_min_count_one_keeps_rare_values() {
    let wrk = Workdir::new("synthesize_min_count_one");
    wrk.create("data.csv", min_count_fixture());

    let mut cmd = wrk.command("synthesize");
    cmd.args(["--min-count", "1", "-n", "2000", "--seed", "42"])
        .arg("data.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // 1/31 of 2000 rows ≈ 64 expected.
    assert!(
        count_cat(&got, "RARE") > 0,
        "--min-count 1 should keep the singleton"
    );
}

/// A joint (city, zip) group: two common tuples, one tuple seen exactly 5 times
/// (the default --min-count, so kept) and one seen 4 times (suppressed).
fn min_count_joint_fixture() -> Vec<Vec<String>> {
    let mut rows = vec![svec!["city", "zip"]];
    rows.extend(std::iter::repeat_n(svec!["Springfield", "10001"], 10));
    rows.extend(std::iter::repeat_n(svec!["Shelbyville", "20002"], 10));
    rows.extend(std::iter::repeat_n(svec!["Ogdenville", "55555"], 5));
    rows.extend(std::iter::repeat_n(svec!["Smallville", "99999"], 4));
    rows
}

const MIN_COUNT_JOINT_DICT: &str = r#"{
    "fields": [
        {"name": "city", "type": "String"},
        {"name": "zip", "type": "String"}
    ],
    "relationships": [
        {"kind": "joint", "members": ["city", "zip"]}
    ]
}"#;

fn has_smallville_tuple(got: &[Vec<String>]) -> bool {
    got[1..]
        .iter()
        .any(|r| r[0] == "Smallville" && r[1] == "99999")
}

#[test]
fn synthesize_min_count_suppresses_rare_joint_tuples() {
    let wrk = Workdir::new("synthesize_min_count_joint");
    wrk.create("data.csv", min_count_joint_fixture());
    wrk.create_from_string("dict.json", MIN_COUNT_JOINT_DICT);

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "1000", "--seed", "42", "--dictionary"])
        .arg(wrk.path("dict.json"))
        .arg("data.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    assert_eq!(got.len(), 1001);
    for row in &got[1..] {
        let pair = (row[0].as_str(), row[1].as_str());
        assert!(
            matches!(
                pair,
                ("Springfield", "10001") | ("Shelbyville", "20002") | ("Ogdenville", "55555")
            ),
            "unexpected tuple {pair:?}: rare tuple leaked or joint group fell apart"
        );
    }
    assert!(
        got[1..].iter().any(|r| r[0] == "Ogdenville"),
        "a tuple seen exactly --min-count times must be kept"
    );

    let mut cmd = wrk.command("synthesize");
    cmd.args([
        "--min-count",
        "1",
        "-n",
        "1000",
        "--seed",
        "42",
        "--dictionary",
    ])
    .arg(wrk.path("dict.json"))
    .arg("data.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    assert!(
        has_smallville_tuple(&got),
        "--min-count 1 should keep the singleton tuple"
    );
}

#[test]
fn synthesize_min_count_all_joint_tuples_suppressed_falls_back() {
    let wrk = Workdir::new("synthesize_min_count_joint_all");
    wrk.create(
        "data.csv",
        vec![
            svec!["city", "zip"],
            svec!["Springfield", "10001"],
            svec!["Springfield", "10001"],
            svec!["Shelbyville", "20002"],
            svec!["Smallville", "99999"],
        ],
    );
    wrk.create_from_string("dict.json", MIN_COUNT_JOINT_DICT);

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "200", "--seed", "42", "--dictionary"])
        .arg(wrk.path("dict.json"))
        .arg("data.csv");
    let (got, err): (Vec<Vec<String>>, String) = wrk.read_stdout_and_stderr_on_success(&mut cmd);
    assert_eq!(got.len(), 201);
    // Full suppression must be visible on the terminal, not just in the log.
    assert!(
        err.contains("all 3 distinct tuple(s) occur fewer than --min-count (5)"),
        "missing joint suppression warning: {err}"
    );
    assert!(
        err.contains("every value of column 'city' occurs fewer than --min-count (5)"),
        "missing column suppression warning: {err}"
    );
    // Every tuple and every city value is below the default --min-count, so the
    // group falls back to independent columns and no real city is emitted. (zip
    // is inferred as Integer and falls back to its min/max range, which can still
    // produce real zips; that is the documented range caveat, not asserted here.)
    let cities = ["Springfield", "Shelbyville", "Smallville"];
    for row in &got[1..] {
        assert_eq!(row.len(), 2);
        assert!(
            !cities.contains(&row[0].as_str()),
            "real city '{}' leaked after full suppression",
            row[0]
        );
    }
}

#[test]
fn synthesize_rejects_zero_min_count() {
    let wrk = Workdir::new("synthesize_min_count_zero");
    wrk.create("data.csv", fixture());

    let mut cmd = wrk.command("synthesize");
    cmd.args(["--min-count", "0"]).arg("data.csv");
    let err = wrk.output_stderr(&mut cmd);
    assert!(
        err.contains("--min-count must be at least 1"),
        "expected a --min-count error, got: {err}"
    );
}

#[test]
fn synthesize_min_count_all_joint_tuples_suppressed_strict_aborts() {
    let wrk = Workdir::new("synthesize_min_count_joint_strict");
    wrk.create("data.csv", min_count_joint_fixture());
    wrk.create_from_string("dict.json", MIN_COUNT_JOINT_DICT);

    let mut cmd = wrk.command("synthesize");
    cmd.args([
        "--min-count",
        "100",
        "--strict-relationships",
        "-n",
        "10",
        "--dictionary",
    ])
    .arg(wrk.path("dict.json"))
    .arg("data.csv");
    let err = wrk.stderr_on_error(&mut cmd);
    assert!(
        err.contains("fewer than --min-count (100) times. Lower --min-count"),
        "expected a strict --min-count abort, got: {err}"
    );
}

/// Nulls are exempt from --min-count on purpose: they are reproduced through
/// the column's null ratio, not sampled as values, so a single blank in the
/// source discloses no value and suppressing it would only distort the rate.
#[test]

fn synthesize_min_count_does_not_suppress_nulls() {
    let wrk = Workdir::new("synthesize_min_count_nulls");
    let mut rows = min_count_fixture();
    rows.push(svec![""]);
    wrk.create("data.csv", rows);

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "2000", "--seed", "42"]).arg("data.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // 1 null in 32 rows ≈ 62 expected blanks out of 2000.
    let blanks = count_cat(&got, "");
    assert!(
        blanks > 0,
        "the single source null should still be reproduced"
    );
    assert_eq!(count_cat(&got, "RARE"), 0, "RARE must still be suppressed");
    // The null must not knock the column out of enumeration: every non-blank
    // value is a kept real value, not fallback filler.
    let a = count_cat(&got, "A");
    let b = count_cat(&got, "B");
    assert!(a > 0 && b > 0, "kept values missing: A={a} B={b}");
    assert_eq!(
        a + b + blanks,
        2000,
        "non-real values emitted for a nullable column"
    );
}

/// Regression: a single null in an enumerated numeric column must not push it
/// to the min/max range fallback, which would emit the suppressed singleton.
#[test]
fn synthesize_min_count_nullable_numeric_column_stays_enumerated() {
    let wrk = Workdir::new("synthesize_min_count_nullable_numeric");
    let mut rows = vec![svec!["n", "x"]];
    rows.extend(std::iter::repeat_n(svec!["10", "a"], 20));
    rows.push(svec!["11", "a"]);
    rows.push(svec!["", "a"]);
    wrk.create("data.csv", rows);

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "1000", "--seed", "1"]).arg("data.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let mut nulls = 0;
    for row in &got[1..] {
        if row[0].is_empty() {
            nulls += 1;
            continue;
        }
        assert_eq!(
            row[0], "10",
            "expected only the kept value 10 or a null, got '{}'",
            row[0]
        );
    }
    assert!(nulls > 0, "the source null should still be reproduced");
}

/// With --consistent-fakes, an enumerated structured column is mapped to fakes,
/// so no real value (rare or common) reaches the output and --min-count has
/// nothing to filter on that path.
#[test]
fn synthesize_min_count_with_consistent_fakes_emits_no_real_values() {
    let wrk = Workdir::new("synthesize_min_count_consistent_fakes");
    let mut rows = vec![svec!["first_name"]];
    rows.extend(std::iter::repeat_n(svec!["SRC_Michael"], 20));
    rows.extend(std::iter::repeat_n(svec!["SRC_Sarah"], 10));
    rows.push(svec!["SRC_Rare"]);
    wrk.create("data.csv", rows);
    wrk.create_from_string(
        "dict.json",
        r#"{"fields": [{"name": "first_name", "type": "String", "content_type": "first_name"}]}"#,
    );

    let mut cmd = wrk.command("synthesize");
    cmd.args([
        "-n",
        "500",
        "--seed",
        "7",
        "--consistent-fakes",
        "--dictionary",
    ])
    .arg(wrk.path("dict.json"))
    .arg("data.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got.len(), 501);
    for row in &got[1..] {
        assert!(
            !row[0].starts_with("SRC_"),
            "real value '{}' leaked through --consistent-fakes",
            row[0]
        );
    }
}

// --- --min-count range trimming -------------------------------------------

/// 30 ages from 20 to 49 plus a lone 999 outlier, and 30 dates in January 2024
/// plus a lone 1901 date. Every value is distinct, so neither column is
/// enumerated: both go to the type-based range generator.
fn range_trim_fixture() -> Vec<Vec<String>> {
    let mut rows = vec![svec!["age", "d"]];
    for i in 0..30 {
        rows.push(vec![format!("{}", 20 + i), format!("2024-01-{:02}", i + 1)]);
    }
    rows.push(svec!["999", "1901-06-15"]);
    rows
}

#[test]
fn synthesize_min_count_trims_numeric_and_date_ranges() {
    let wrk = Workdir::new("synthesize_range_trim");
    wrk.create("data.csv", range_trim_fixture());

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "3000", "--seed", "3"]).arg("data.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got.len(), 3001);

    // k = 5: ages trim to [24, 46] (5th smallest 24; 5th largest 46, since
    // 999 is one of the five largest), and dates to [2024-01-04, 2024-01-26]
    // (the 1901 date is one of the five smallest).
    let ages: Vec<i64> = got[1..].iter().map(|r| r[0].parse().unwrap()).collect();
    let (lo, hi) = (*ages.iter().min().unwrap(), *ages.iter().max().unwrap());
    assert!(lo >= 24 && hi <= 46, "ages not trimmed: [{lo}, {hi}]");
    assert!(
        lo <= 26 && hi >= 44,
        "trimmed age range collapsed: [{lo}, {hi}]"
    );

    for r in &got[1..] {
        assert!(
            r[1].as_str() >= "2024-01-04" && r[1].as_str() <= "2024-01-26",
            "date {} outside trimmed range",
            r[1]
        );
    }
}

#[test]
fn synthesize_min_count_one_keeps_full_range() {
    let wrk = Workdir::new("synthesize_range_trim_off");
    wrk.create("data.csv", range_trim_fixture());

    let mut cmd = wrk.command("synthesize");
    cmd.args(["--min-count", "1", "-n", "3000", "--seed", "3"])
        .arg("data.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let max_age = got[1..]
        .iter()
        .map(|r| r[0].parse::<i64>().unwrap())
        .max()
        .unwrap();
    assert!(
        max_age > 100,
        "--min-count 1 should keep the outlier's range"
    );
    assert!(
        got[1..].iter().any(|r| r[1].as_str() < "2000-01-01"),
        "--min-count 1 should keep the 1901 date in range"
    );
}

#[test]
fn synthesize_min_count_column_too_small_to_trim_is_empty() {
    let wrk = Workdir::new("synthesize_range_trim_small");
    wrk.create(
        "data.csv",
        vec![
            svec!["n", "x"],
            svec!["1", "a"],
            svec!["2", "a"],
            svec!["3", "a"],
        ],
    );

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "50", "--seed", "1"]).arg("data.csv");
    let (got, err): (Vec<Vec<String>>, String) = wrk.read_stdout_and_stderr_on_success(&mut cmd);
    assert!(
        got[1..].iter().all(|r| r[0].is_empty()),
        "a column with fewer than --min-count values must not emit its values"
    );
    assert!(
        err.contains(
            "column 'n' has only 3 non-null value(s); trimming its range by --min-count (5) needs \
             at least 9"
        ),
        "missing too-small warning: {err}"
    );
}

/// Regression: with k <= n < 2k-1 values the k-th smallest and k-th largest
/// cross, and swapping them used to restore the full real range (n = k gave
/// [min, max]), so a lone outlier came back. Such a column is now left empty.
#[test]
fn synthesize_min_count_crossed_bounds_do_not_restore_full_range() {
    let wrk = Workdir::new("synthesize_range_trim_crossed");
    wrk.create(
        "data.csv",
        vec![
            svec!["n", "x"],
            svec!["10", "a"],
            svec!["11", "a"],
            svec!["12", "a"],
            svec!["13", "a"],
            svec!["99999", "a"],
        ],
    );

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "500", "--seed", "1"]).arg("data.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert!(
        got[1..].iter().all(|r| r[0].is_empty()),
        "a 5-value column can't be trimmed at --min-count 5 and must be left empty"
    );
}

/// Regression: an `ordered` member whose range --min-count emptied must stay
/// empty. The anchor + mean-gap chain used to fill it on every row.
#[test]
fn synthesize_min_count_ordered_member_without_range_stays_empty() {
    let wrk = Workdir::new("synthesize_range_trim_ordered_member");
    let mut rows = vec![svec!["a", "b"]];
    for i in 0..30 {
        let b = if i < 3 {
            format!("{}", 11 + i)
        } else {
            String::new()
        };
        rows.push(vec![format!("{}", i + 1), b]);
    }
    wrk.create("data.csv", rows);
    wrk.create_from_string(
        "dict.json",
        r#"{
            "fields": [
                {"name": "a", "type": "Integer"},
                {"name": "b", "type": "Integer"}
            ],
            "relationships": [
                {"kind": "ordered", "members": ["a", "b"]}
            ]
        }"#,
    );

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "200", "--seed", "4", "--dictionary"])
        .arg(wrk.path("dict.json"))
        .arg("data.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    assert!(
        got[1..].iter().all(|r| r[1].is_empty()),
        "ordered member 'b' has no usable range and must stay empty"
    );
    assert!(
        got[1..].iter().all(|r| !r[0].is_empty()),
        "the anchor 'a' should still be generated"
    );
}

/// Regression: under `QSV_NO_HEADERS`, stats names columns "0", "1", …, which
/// match no header, so name-only mapping silently skipped range trimming.
#[test]
fn synthesize_min_count_range_trim_works_without_headers() {
    let wrk = Workdir::new("synthesize_range_trim_no_headers");
    wrk.create(
        "data.csv",
        vec![svec!["100", "a"], svec!["101", "a"], svec!["1000000", "a"]],
    );

    let mut cmd = wrk.command("synthesize");
    cmd.env("QSV_NO_HEADERS", "1")
        .args(["-n", "50", "--seed", "1"])
        .arg("data.csv");
    let (got, err): (Vec<Vec<String>>, String) = wrk.read_stdout_and_stderr_on_success(&mut cmd);
    assert!(
        got.iter().skip(1).all(|r| r[0].is_empty()),
        "3 values can't be trimmed at --min-count 5; the column must be empty: {got:?}"
    );
    assert!(
        err.contains("column '0' has only 3 non-null value(s)"),
        "missing too-small warning: {err}"
    );
}

#[test]
fn synthesize_rejects_column_selection_in_stats_options() {
    let wrk = Workdir::new("synthesize_stats_options_select");
    wrk.create("data.csv", fixture());
    for opts in [
        "--select city",
        "--select=city",
        "-s city",
        "--weight age",
        "-Es 2,1",
        "-Es2,1",
    ] {
        let mut cmd = wrk.command("synthesize");
        cmd.args(["--stats-options", opts]).arg("data.csv");
        let err = wrk.stderr_on_error(&mut cmd);
        assert!(
            err.contains("--stats-options may not include"),
            "'{opts}' should be rejected, got: {err}"
        );
    }
}

/// `ordered` followers are clamped into their own trimmed range: a follower
/// outlier (end = 5000 on one row) must not come back as anchor + gap.
#[test]
fn synthesize_ordered_follower_stays_in_its_trimmed_range() {
    let wrk = Workdir::new("synthesize_ordered_follower_clamp");
    let mut rows = vec![svec!["start", "end"]];
    for i in 0..40 {
        let start = 100 + i;
        let end = if i == 39 { 5000 } else { start + 10 + i % 7 };
        rows.push(vec![format!("{start}"), format!("{end}")]);
    }
    wrk.create("data.csv", rows);
    wrk.create_from_string(
        "dict.json",
        r#"{
            "fields": [
                {"name": "start", "type": "Integer"},
                {"name": "end", "type": "Integer"}
            ],
            "relationships": [{"kind": "ordered", "members": ["start", "end"]}]
        }"#,
    );

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "2000", "--seed", "8", "--dictionary"])
        .arg(wrk.path("dict.json"))
        .arg("data.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    // end's 5th largest source value is 148 (5000 and 149..=151 sit above it).
    for r in &got[1..] {
        let start: i64 = r[0].parse().unwrap();
        let end: i64 = r[1].parse().unwrap();
        assert!(end >= start, "order broken: {r:?}");
        assert!(end <= 148, "end {end} escaped its trimmed range: {r:?}");
    }
}

/// An `ordered` pair with too few co-present rows to trim its gap is dropped,
/// not collapsed to the mean (with one pair, the mean is that pair's gap).
#[test]

fn synthesize_ordered_sparse_gap_drops_relationship() {
    let wrk = Workdir::new("synthesize_ordered_sparse_gap");
    // `start` and `end` each have enough values to trim, but only one row has
    // both, so there is a single gap sample (37).
    let mut rows = vec![svec!["start", "end"]];
    for i in 0..40 {
        let end = if i == 0 {
            "137".to_string()
        } else {
            String::new()
        };
        rows.push(vec![format!("{}", 100 + i), end]);
    }
    for j in 0..21 {
        rows.push(vec![String::new(), format!("{}", 110 + j)]);
    }
    wrk.create("data.csv", rows);
    wrk.create_from_string(
        "dict.json",
        r#"{
            "fields": [
                {"name": "start", "type": "Integer"},
                {"name": "end", "type": "Integer"}
            ],
            "relationships": [{"kind": "ordered", "members": ["start", "end"]}]
        }"#,
    );

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "500", "--seed", "9", "--dictionary"])
        .arg(wrk.path("dict.json"))
        .arg("data.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    // Dropped → independent columns, so order is not enforced. A kept chain
    // (e.g. the old mean-gap fallback) would make end >= start on every row.
    let unordered = got[1..]
        .iter()
        .any(|r| match (r[0].parse::<i64>(), r[1].parse::<i64>()) {
            (Ok(s), Ok(e)) => e < s,
            _ => false,
        });
    assert!(
        unordered,
        "sparse ordered relationship should be dropped, not chained"
    );

    let mut strict = wrk.command("synthesize");
    strict
        .args(["-n", "10", "--strict-relationships", "--dictionary"])
        .arg(wrk.path("dict.json"))
        .arg("data.csv");
    let err = wrk.stderr_on_error(&mut strict);
    assert!(
        err.contains("only 1 row(s) have both 'start' and 'end'"),
        "expected a strict sparse-gap abort, got: {err}"
    );
}

/// Range trimming parses dates with the same DMY preference as `stats`.
/// "01/MM/2024" is the 1st of month MM under DMY, but Jan MM under MDY.
#[test]
fn synthesize_min_count_range_trim_respects_prefer_dmy() {
    let wrk = Workdir::new("synthesize_range_trim_dmy");
    let mut rows = vec![svec!["d", "x"]];
    for m in 1..=12 {
        rows.push(vec![format!("01/{m:02}/2024"), "a".to_string()]);
    }
    wrk.create("data.csv", rows);

    let mut cmd = wrk.command("synthesize");
    cmd.env("QSV_PREFER_DMY", "1")
        .args(["-n", "500", "--seed", "10"])
        .arg("data.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // DMY, k = 5 on 12 monthly dates → [2024-05-01, 2024-08-01]. Under MDY the
    // range would be [2024-01-05, 2024-01-08].
    for r in &got[1..] {
        assert!(
            r[0].as_str() >= "2024-05-01" && r[0].as_str() <= "2024-08-01",
            "date {} outside the DMY-trimmed range",
            r[0]
        );
    }
    assert!(
        got[1..].iter().any(|r| r[0].starts_with("2024-06")),
        "expected dates spread across the DMY range"
    );
}

/// Regression: with differing null patterns the predecessor can exceed a
/// follower's trimmed max; the order floor then pushed the follower past its
/// range, reproducing a suppressed value (end = 100 here, seen once).
#[test]
fn synthesize_ordered_follower_cap_holds_against_order_floor() {
    let wrk = Workdir::new("synthesize_ordered_cap");
    let mut rows = vec![svec!["start", "end"]];
    rows.extend(std::iter::repeat_n(svec!["1", "11"], 9));
    rows.extend(std::iter::repeat_n(svec!["100", ""], 9));
    rows.push(svec!["100", "100"]);
    wrk.create("data.csv", rows);
    wrk.create_from_string(
        "dict.json",
        r#"{
            "fields": [
                {"name": "start", "type": "Integer"},
                {"name": "end", "type": "Integer"}
            ],
            "relationships": [{"kind": "ordered", "members": ["start", "end"]}]
        }"#,
    );

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "1000", "--seed", "12", "--dictionary"])
        .arg(wrk.path("dict.json"))
        .arg("data.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    // end trims to [11, 11]; start is capped by it, so the chain stays in range.
    for r in &got[1..] {
        assert_eq!(r[1], "11", "end escaped its trimmed range: {r:?}");
        let start: i64 = r[0].parse().unwrap();
        assert!(start <= 11, "start {start} exceeds the follower cap: {r:?}");
    }
}

/// An ordered pair with zero co-present rows aborts under --strict-relationships
/// like any other undersized gap sample.
#[test]
fn synthesize_ordered_no_copresent_rows_strict_aborts() {
    let wrk = Workdir::new("synthesize_ordered_no_copresent");
    let mut rows = vec![svec!["start", "end"]];
    for i in 0..9 {
        rows.push(vec![format!("{}", 100 + i), String::new()]);
    }
    for i in 0..9 {
        rows.push(vec![String::new(), format!("{}", 200 + i)]);
    }
    wrk.create("data.csv", rows);
    wrk.create_from_string(
        "dict.json",
        r#"{
            "fields": [
                {"name": "start", "type": "Integer"},
                {"name": "end", "type": "Integer"}
            ],
            "relationships": [{"kind": "ordered", "members": ["start", "end"]}]
        }"#,
    );

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "10", "--strict-relationships", "--dictionary"])
        .arg(wrk.path("dict.json"))
        .arg("data.csv");
    let err = wrk.stderr_on_error(&mut cmd);
    assert!(
        err.contains("no rows have both 'start' and 'end'"),
        "expected a strict abort, got: {err}"
    );
}

/// Regression: in a mixed Integer/Float ordered group the Integer member is
/// printed rounded, so chaining from the unrounded value (or a fractional cap)
/// could print start = 12 above end = 11.6.
#[test]
fn synthesize_ordered_mixed_int_float_keeps_order_after_rounding() {
    let wrk = Workdir::new("synthesize_ordered_mixed_rounding");
    let mut rows = vec![svec!["start", "end"]];
    rows.extend(std::iter::repeat_n(svec!["1", "11.6"], 9));
    rows.extend(std::iter::repeat_n(svec!["100", ""], 9));
    rows.push(svec!["100", "100.6"]);
    wrk.create("data.csv", rows);
    wrk.create_from_string(
        "dict.json",
        r#"{
            "fields": [
                {"name": "start", "type": "Integer"},
                {"name": "end", "type": "Float"}
            ],
            "relationships": [{"kind": "ordered", "members": ["start", "end"]}]
        }"#,
    );

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "1000", "--seed", "13", "--dictionary"])
        .arg(wrk.path("dict.json"))
        .arg("data.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    for r in &got[1..] {
        let start: f64 = r[0].parse().unwrap();
        let end: f64 = r[1].parse().unwrap();
        assert!(start <= end, "order broken after rounding: {r:?}");
        assert!(end <= 11.6, "end escaped its trimmed range: {r:?}");
    }
}

/// Small gaps (< 0.5) in a mixed Integer/Float ordered group: without chaining
/// from the rounded Integer value, start = 12.6 prints as 13 while end = 12.8.
#[test]
fn synthesize_ordered_mixed_int_float_small_gap_keeps_order() {
    let wrk = Workdir::new("synthesize_ordered_mixed_small_gap");
    let mut rows = vec![svec!["start", "end"]];
    for i in 10..50 {
        rows.push(vec![format!("{i}"), format!("{i}.2")]);
    }
    wrk.create("data.csv", rows);
    wrk.create_from_string(
        "dict.json",
        r#"{
            "fields": [
                {"name": "start", "type": "Integer"},
                {"name": "end", "type": "Float"}
            ],
            "relationships": [{"kind": "ordered", "members": ["start", "end"]}]
        }"#,
    );

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "2000", "--seed", "14", "--dictionary"])
        .arg(wrk.path("dict.json"))
        .arg("data.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    for r in &got[1..] {
        let start: f64 = r[0].parse().unwrap();
        let end: f64 = r[1].parse().unwrap();
        assert!(start <= end, "order broken after rounding: {r:?}");
    }
}

#[test]
fn synthesize_ordered_float_then_int_small_gap_keeps_order() {
    let wrk = Workdir::new("synthesize_ordered_float_int_small_gap");
    let mut rows = vec![svec!["start", "end"]];
    for i in 10..50 {
        rows.push(vec![format!("{i}.8"), format!("{}", i + 1)]);
    }
    wrk.create("data.csv", rows);
    wrk.create_from_string(
        "dict.json",
        r#"{
            "fields": [
                {"name": "start", "type": "Float"},
                {"name": "end", "type": "Integer"}
            ],
            "relationships": [{"kind": "ordered", "members": ["start", "end"]}]
        }"#,
    );

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "2000", "--seed", "14", "--dictionary"])
        .arg(wrk.path("dict.json"))
        .arg("data.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    for r in &got[1..] {
        let start: f64 = r[0].parse().unwrap();
        let end: f64 = r[1].parse().unwrap();
        assert!(start <= end, "order broken after rounding: {r:?}");
    }
}

// --- --report ---------------------------------------------------------------

fn read_report(wrk: &Workdir) -> serde_json::Value {
    serde_json::from_str(&std::fs::read_to_string(wrk.path("report.json")).unwrap()).unwrap()
}

/// Dates are emitted in ISO form while the source uses MM/DD/YYYY, so a
/// text-only comparison would find no matches. Every emitted date is one of
/// the ten source dates, so every row is an exact copy by value.
#[test]
fn synthesize_report_matches_dates_by_value() {
    let wrk = Workdir::new("synthesize_report_dates_by_value");
    let mut rows = vec![svec!["created_date"]];
    for d in 1..=10 {
        rows.push(vec![format!("01/{d:02}/2020")]);
    }
    wrk.create("data.csv", rows);

    let mut cmd = wrk.command("synthesize");
    cmd.args([
        "-n",
        "50",
        "--seed",
        "1",
        "--min-count",
        "1",
        "--freq-limit",
        "2",
        "--stats-options",
        "--dates-whitelist all",
        "--report",
    ])
    .arg(wrk.path("report.json"))
    .arg("data.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    assert!(
        got[1..].iter().all(|r| r[0].starts_with("2020-01-")),
        "expected ISO dates from the range generator: {got:?}"
    );

    let report = read_report(&wrk);
    assert_eq!(report["source_rows"], 10);
    assert_eq!(report["synthetic_rows"], 50);
    assert_eq!(report["min_count"], 1);
    assert_eq!(report["privacy"]["exact_matches"], 50);
    assert_eq!(report["privacy"]["exact_match_rate"], 1.0);
    assert_eq!(report["privacy"]["distinct_source_rows"], 10);
    assert_eq!(
        report["privacy"]["compared_columns"],
        serde_json::json!(["created_date"])
    );
}

/// Under `date:%d/%m/%Y` a generated `03/02/2020` is February 3. Reading it
/// with the source's month-first preference would make it March 2 and miss
/// the copy.
#[test]
fn synthesize_report_reads_dates_in_output_format() {
    let wrk = Workdir::new("synthesize_report_dates_output_format");
    let mut rows = vec![svec!["d"]];
    for d in 1..=10 {
        rows.push(vec![format!("2020-02-{d:02}")]);
    }
    wrk.create("data.csv", rows);
    wrk.create_from_string(
        "dict.json",
        r#"{"fields": [{"name": "d", "type": "Date", "content_type": "date:%d/%m/%Y"}]}"#,
    );

    let mut cmd = wrk.command("synthesize");
    cmd.args([
        "-n",
        "50",
        "--seed",
        "1",
        "--min-count",
        "1",
        "--freq-limit",
        "2",
        "--stats-options",
        "--dates-whitelist all",
        "--dictionary",
    ])
    .arg(wrk.path("dict.json"))
    .arg("--report")
    .arg(wrk.path("report.json"))
    .arg("data.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    assert!(
        got[1..].iter().all(|r| r[0].ends_with("/02/2020")),
        "expected DD/MM/YYYY dates: {got:?}"
    );
    assert_eq!(read_report(&wrk)["privacy"]["exact_matches"], 50);
}

/// Integers above 2^53 and fractional seconds would collapse if keyed through
/// f64 or whole seconds.
#[test]
fn synthesize_report_keeps_large_ints_and_subseconds_distinct() {
    let wrk = Workdir::new("synthesize_report_precision");
    for (name, header, a, b) in [
        ("ints", "id", "9007199254740992", "9007199254740993"),
        (
            "times",
            "event_ts",
            "2020-01-01T00:00:00.100Z",
            "2020-01-01T00:00:00.200Z",
        ),
    ] {
        wrk.create(
            &format!("{name}.csv"),
            vec![svec![header], vec![a.to_string()], vec![b.to_string()]],
        );
        let mut cmd = wrk.command("synthesize");
        cmd.args([
            "-n",
            "1",
            "--seed",
            "1",
            "--min-count",
            "1",
            "--stats-options",
            "--dates-whitelist all",
            "--report",
        ])
        .arg(wrk.path("report.json"))
        .arg(format!("{name}.csv"));
        wrk.assert_success(&mut cmd);
        assert_eq!(
            read_report(&wrk)["privacy"]["distinct_source_rows"],
            2,
            "{name}"
        );
    }
}

/// A cell that is not UTF-8 is hashed as bytes instead of failing the run.
#[test]
fn synthesize_report_accepts_non_utf8_source() {
    let wrk = Workdir::new("synthesize_report_non_utf8");
    std::fs::write(wrk.path("data.csv"), b"city\nParis\nParis\nZ\xfcrich\n").unwrap();
    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "5", "--seed", "1", "--freq-limit", "1", "--report"])
        .arg(wrk.path("report.json"))
        .arg("data.csv");
    wrk.assert_success(&mut cmd);
    assert_eq!(read_report(&wrk)["privacy"]["distinct_source_rows"], 2);
}

/// With no column compared every row hashes alike, so the row metrics are
/// null rather than a 100% match rate.
#[test]
fn synthesize_report_nulls_row_metrics_when_nothing_compared() {
    let wrk = Workdir::new("synthesize_report_nothing_compared");
    wrk.create(
        "data.csv",
        vec![svec!["city"], svec!["Paris"], svec!["Rome"], svec!["Oslo"]],
    );
    let mut cmd = wrk.command("synthesize");
    cmd.args([
        "-n",
        "5",
        "--seed",
        "1",
        "--stats-options=--no-headers",
        "--report",
    ])
    .arg(wrk.path("report.json"))
    .arg("data.csv");
    wrk.assert_success(&mut cmd);
    let report = read_report(&wrk);
    assert_eq!(report["privacy"]["compared_columns"], serde_json::json!([]));
    assert!(report["privacy"]["exact_matches"].is_null(), "{report}");
    assert!(report["privacy"]["exact_match_rate"].is_null(), "{report}");
}

/// `--report` naming the input or the output is refused before anything is
/// written, so neither file is replaced by the JSON.
#[test]
fn synthesize_report_rejects_input_or_output_path() {
    let wrk = Workdir::new("synthesize_report_path_clash");
    wrk.create("data.csv", vec![svec!["n"], svec!["1"], svec!["2"]]);

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "1", "--report"])
        .arg(wrk.path("data.csv"))
        .arg("data.csv");
    wrk.assert_err(&mut cmd);
    assert_eq!(
        std::fs::read_to_string(wrk.path("data.csv")).unwrap(),
        "n\n1\n2\n"
    );

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "1", "--output"])
        .arg(wrk.path("out.csv"))
        .arg("--report")
        .arg(wrk.path("./out.csv"))
        .arg("data.csv");
    wrk.assert_err(&mut cmd);
    assert!(!wrk.path("out.csv").exists());
}

/// Without headers stats names columns from 0 and frequency from 1. Unaligned,
/// column 1 took column 0's values and column 0 fell back to filler text.
#[test]
fn synthesize_no_headers_pairs_frequency_with_its_column() {
    let wrk = Workdir::new("synthesize_no_headers_frequency");
    let mut rows = vec![svec!["a", "k"]; 20];
    rows.push(svec!["rare", "k"]);
    wrk.create("data.csv", rows);
    let mut cmd = wrk.command("synthesize");
    cmd.env("QSV_NO_HEADERS", "1")
        .args(["-n", "8", "--seed", "1", "--report"])
        .arg(wrk.path("report.json"))
        .arg("data.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    assert!(
        got[1..].iter().all(|r| r[0] == "a" && r[1] == "k"),
        "each column must sample its own values: {got:?}"
    );
    assert_eq!(
        read_report(&wrk)["privacy"]["suppressed"],
        serde_json::json!([{"columns": ["0"], "distinct": 1, "rows": 1}])
    );
}

#[test]
fn synthesize_report_unique_ids_never_match() {
    let wrk = Workdir::new("synthesize_report_unique_ids");
    let mut rows = vec![svec!["id"]];
    for i in 0..30 {
        rows.push(vec![format!("ID-{i:04}")]);
    }
    wrk.create("data.csv", rows);

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "50", "--seed", "1", "--report"])
        .arg(wrk.path("report.json"))
        .arg("data.csv");
    wrk.assert_success(&mut cmd);

    let report = read_report(&wrk);
    assert_eq!(report["privacy"]["exact_matches"], 0);
    assert_eq!(report["privacy"]["exact_match_rate"], 0.0);
    assert_eq!(report["privacy"]["distinct_source_rows"], 30);
}

#[test]
fn synthesize_report_lists_suppressed_values_and_tuples() {
    let wrk = Workdir::new("synthesize_report_suppressed");
    let mut rows = vec![svec!["color"]];
    rows.extend(std::iter::repeat_n(svec!["red"], 5));
    rows.extend(std::iter::repeat_n(svec!["blue"], 5));
    rows.extend(std::iter::repeat_n(svec!["teal"], 2));
    wrk.create("data.csv", rows);

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "20", "--seed", "1", "--report"])
        .arg(wrk.path("report.json"))
        .arg("data.csv");
    wrk.assert_success(&mut cmd);
    assert_eq!(
        read_report(&wrk)["privacy"]["suppressed"],
        serde_json::json!([{"columns": ["color"], "distinct": 1, "rows": 2}])
    );

    wrk.create("joint.csv", min_count_joint_fixture());
    wrk.create_from_string("dict.json", MIN_COUNT_JOINT_DICT);
    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "20", "--seed", "1", "--dictionary"])
        .arg(wrk.path("dict.json"))
        .arg("--report")
        .arg(wrk.path("report.json"))
        .arg("joint.csv");
    wrk.assert_success(&mut cmd);
    assert_eq!(
        read_report(&wrk)["privacy"]["suppressed"],
        serde_json::json!([{"columns": ["city", "zip"], "distinct": 1, "rows": 4}])
    );
}

/// Fully suppressed columns and groups are the cases the report matters most
/// for: their real values are gone from the output entirely.
#[test]
fn synthesize_report_lists_fully_suppressed_columns_and_groups() {
    let wrk = Workdir::new("synthesize_report_fully_suppressed");
    let mut rows = vec![svec!["color"]];
    rows.extend(std::iter::repeat_n(svec!["red"], 3));
    rows.extend(std::iter::repeat_n(svec!["teal"], 2));
    wrk.create("data.csv", rows);

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "20", "--seed", "1", "--report"])
        .arg(wrk.path("report.json"))
        .arg("data.csv");
    wrk.assert_success(&mut cmd);
    assert_eq!(
        read_report(&wrk)["privacy"]["suppressed"],
        serde_json::json!([{"columns": ["color"], "distinct": 2, "rows": 5}])
    );

    // The joint group is dropped, then each column is suppressed on its own,
    // so the same 29 rows appear under the group and under each column.
    wrk.create("joint.csv", min_count_joint_fixture());
    wrk.create_from_string("dict.json", MIN_COUNT_JOINT_DICT);
    let mut cmd = wrk.command("synthesize");
    cmd.args([
        "-n",
        "20",
        "--seed",
        "1",
        "--min-count",
        "11",
        "--dictionary",
    ])
    .arg(wrk.path("dict.json"))
    .arg("--report")
    .arg(wrk.path("report.json"))
    .arg("joint.csv");
    wrk.assert_success(&mut cmd);
    assert_eq!(
        read_report(&wrk)["privacy"]["suppressed"],
        serde_json::json!([
            {"columns": ["city", "zip"], "distinct": 4, "rows": 29},
            {"columns": ["city"], "distinct": 4, "rows": 29},
            {"columns": ["zip"], "distinct": 4, "rows": 29}
        ])
    );
}

/// `--consistent-fakes` emits fakes for every value, so nothing is dropped by
/// `--min-count` and the report must not claim otherwise.
#[test]
fn synthesize_report_consistent_fakes_reports_no_suppression() {
    let wrk = Workdir::new("synthesize_report_consistent_fakes");
    let mut rows = vec![svec!["first_name"]];
    rows.extend(std::iter::repeat_n(svec!["SRC_Michael"], 20));
    rows.push(svec!["SRC_Rare"]);
    wrk.create("data.csv", rows);
    wrk.create_from_string(
        "dict.json",
        r#"{"fields": [{"name": "first_name", "type": "String", "content_type": "first_name"}]}"#,
    );

    let mut cmd = wrk.command("synthesize");
    cmd.args([
        "-n",
        "20",
        "--seed",
        "7",
        "--consistent-fakes",
        "--dictionary",
    ])
    .arg(wrk.path("dict.json"))
    .arg("--report")
    .arg(wrk.path("report.json"))
    .arg("data.csv");
    wrk.assert_success(&mut cmd);
    assert_eq!(
        read_report(&wrk)["privacy"]["suppressed"],
        serde_json::json!([])
    );
}

#[test]
fn synthesize_report_does_not_change_output() {
    let wrk = Workdir::new("synthesize_report_same_output");
    wrk.create("data.csv", fixture());

    let mut plain = wrk.command("synthesize");
    plain
        .args(["-n", "200", "--seed", "42", "--min-count", "1"])
        .arg("data.csv");
    let without: String = wrk.stdout(&mut plain);

    let mut reported = wrk.command("synthesize");
    reported
        .args(["-n", "200", "--seed", "42", "--min-count", "1", "--report"])
        .arg(wrk.path("report.json"))
        .arg("data.csv");
    let with: String = wrk.stdout(&mut reported);

    assert_eq!(without, with);
    assert_eq!(read_report(&wrk)["synthetic_rows"], 200);
}

/// city/state/zip in a `joint` group, a faker-backed email column and a numeric
/// column, so every generator kind draws from the per-chunk RNG.
fn chunked_fixture(wrk: &Workdir) {
    let mut rows = vec![svec!["city", "state", "zip", "email", "amount"]];
    let geo = [
        ("Springfield", "IL", "62701"),
        ("Chicago", "IL", "60601"),
        ("Boston", "MA", "02101"),
        ("Portland", "OR", "97201"),
    ];
    for i in 0..40 {
        let (city, state, zip) = geo[i % geo.len()];
        rows.push(vec![
            city.to_string(),
            state.to_string(),
            zip.to_string(),
            format!("user{i}@example.com"),
            format!("{}.{}", 100 + i * 37 % 900, i % 100),
        ]);
    }
    wrk.create("data.csv", rows);
    wrk.create_from_string(
        "dict.json",
        r#"{
    "fields": [
        {"name": "city", "type": "String"},
        {"name": "state", "type": "String"},
        {"name": "zip", "type": "String"},
        {"name": "email", "type": "String", "content_type": "email"},
        {"name": "amount", "type": "Float"}
    ],
    "relationships": [
        {"kind": "joint", "members": ["city", "state", "zip"]}
    ]
}"#,
    );
}

#[test]
fn synthesize_seeded_output_is_identical_at_any_jobs() {
    let wrk = Workdir::new("synthesize_chunked_jobs");
    chunked_fixture(&wrk);

    // 20000 rows = two full 8192-row chunks plus a partial third.
    let run = |jobs: &str, report: &str| -> String {
        let mut cmd = wrk.command("synthesize");
        cmd.args(["-n", "20000", "--seed", "7", "--jobs", jobs, "--report"])
            .arg(wrk.path(report))
            .arg("--dictionary")
            .arg(wrk.path("dict.json"))
            .arg("data.csv");
        wrk.stdout(&mut cmd)
    };
    let serial = run("1", "r1.json");
    let parallel = run("2", "r2.json");

    assert_eq!(serial.lines().count(), 20001);
    assert!(
        serial == parallel,
        "seeded output must not depend on --jobs"
    );
    let read = |name: &str| -> serde_json::Value {
        serde_json::from_str(&std::fs::read_to_string(wrk.path(name)).unwrap()).unwrap()
    };
    assert_eq!(
        read("r1.json")["privacy"]["exact_matches"],
        read("r2.json")["privacy"]["exact_matches"]
    );
}

#[test]
fn synthesize_chunks_draw_from_distinct_streams() {
    let wrk = Workdir::new("synthesize_chunked_streams");
    chunked_fixture(&wrk);

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "16384", "--seed", "7", "--dictionary"])
        .arg(wrk.path("dict.json"))
        .arg("data.csv");
    let out: String = wrk.stdout(&mut cmd);
    let rows: Vec<&str> = out.lines().skip(1).collect();

    assert_eq!(rows.len(), 16384);
    assert_ne!(
        rows[..8192],
        rows[8192..],
        "each chunk must have its own random stream"
    );
}

#[test]
fn synthesize_row_count_spans_chunk_boundary() {
    let wrk = Workdir::new("synthesize_chunked_boundary");
    chunked_fixture(&wrk);

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "8193", "--seed", "7"]).arg("data.csv");
    let out: String = wrk.stdout(&mut cmd);

    assert_eq!(out.lines().count(), 8194);
}

#[test]
fn synthesize_chunks_use_the_output_delimiter() {
    let wrk = Workdir::new("synthesize_chunked_tsv");
    chunked_fixture(&wrk);

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "9000", "--seed", "7", "-o"])
        .arg(wrk.path("out.tsv"))
        .arg("data.csv");
    wrk.assert_success(&mut cmd);

    let out = std::fs::read_to_string(wrk.path("out.tsv")).unwrap();
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines.len(), 9001);
    // the last line is in the second chunk
    for line in [lines[0], lines[1], lines[9000]] {
        assert_eq!(line.split('\t').count(), 5, "not tab-delimited: {line}");
    }
}

#[test]
fn synthesize_chunks_write_one_bom() {
    let wrk = Workdir::new("synthesize_chunked_bom");
    chunked_fixture(&wrk);

    let mut cmd = wrk.command("synthesize");
    cmd.env("QSV_OUTPUT_BOM", "1")
        .args(["-n", "9000", "--seed", "7", "-o"])
        .arg(wrk.path("out.csv"))
        .arg("data.csv");
    wrk.assert_success(&mut cmd);

    let out = std::fs::read_to_string(wrk.path("out.csv")).unwrap();
    assert!(out.starts_with('\u{FEFF}'));
    assert_eq!(out.matches('\u{FEFF}').count(), 1);
}

#[test]
fn synthesize_report_counts_matches_in_every_chunk() {
    let wrk = Workdir::new("synthesize_chunked_report_count");
    // Only real (city, state, zip) tuples can be emitted, so every row matches.
    wrk.create("data.csv", geo_fixture());
    wrk.create_from_string("dict.json", GEO_JOINT_DICT);

    let mut cmd = wrk.command("synthesize");
    cmd.args([
        "--min-count",
        "1",
        "-n",
        "20000",
        "--seed",
        "7",
        "--jobs",
        "2",
        "--report",
    ])
    .arg(wrk.path("report.json"))
    .arg("--dictionary")
    .arg(wrk.path("dict.json"))
    .arg("data.csv");
    wrk.assert_success(&mut cmd);

    assert_eq!(read_report(&wrk)["privacy"]["exact_matches"], 20000);
}

#[test]
fn synthesize_shards_join_to_the_unsharded_output() {
    let wrk = Workdir::new("synthesize_shard_join");
    chunked_fixture(&wrk);

    // 20000 rows = 3 chunks, so shard 1/2 gets chunk 0 and shard 2/2 chunks 1-2.
    let run = |shard: Option<&str>| -> String {
        let mut cmd = wrk.command("synthesize");
        cmd.args(["-n", "20000", "--seed", "7", "--dictionary"])
            .arg(wrk.path("dict.json"))
            .arg("data.csv");
        if let Some(shard) = shard {
            cmd.args(["--shard", shard]);
        }
        wrk.stdout(&mut cmd)
    };
    let full = run(None);
    let first = run(Some("1/2"));
    let second = run(Some("2/2"));

    assert_eq!(first.lines().count(), 8193);
    // Each shard has its own header; drop the second's before joining.
    let joined: Vec<&str> = first.lines().chain(second.lines().skip(1)).collect();
    let full: Vec<&str> = full.lines().collect();
    assert_eq!(full.len(), 20001);
    assert!(
        joined == full,
        "shards 1..N joined in order must equal the unsharded output"
    );
}

#[test]
fn synthesize_shard_past_the_last_chunk_is_header_only() {
    let wrk = Workdir::new("synthesize_shard_empty");
    chunked_fixture(&wrk);

    // 10 rows = 1 chunk, which goes to the last shard.
    let run = |shard: &str| -> String {
        let mut cmd = wrk.command("synthesize");
        cmd.args(["-n", "10", "--seed", "7", "--shard", shard])
            .arg("data.csv");
        wrk.stdout(&mut cmd)
    };
    assert_eq!(run("1/2").lines().count(), 1);
    assert_eq!(run("2/2").lines().count(), 11);
}

#[test]
fn synthesize_shard_requires_seed_and_a_valid_spec() {
    let wrk = Workdir::new("synthesize_shard_usage");
    chunked_fixture(&wrk);

    let mut no_seed = wrk.command("synthesize");
    no_seed.args(["--shard", "1/2", "data.csv"]);
    assert!(
        wrk.output_stderr(&mut no_seed)
            .contains("--shard requires --seed")
    );

    for bad in ["0/2", "3/2", "2", "a/b", "1/0"] {
        let mut cmd = wrk.command("synthesize");
        cmd.args(["--seed", "1", "--shard", bad, "data.csv"]);
        assert!(
            wrk.output_stderr(&mut cmd).contains("--shard must be k/N"),
            "--shard {bad} must be rejected"
        );
    }
}

#[test]
fn synthesize_report_counts_only_the_shard() {
    let wrk = Workdir::new("synthesize_shard_report");
    // Only real (city, state, zip) tuples can be emitted, so every row matches.
    wrk.create("data.csv", geo_fixture());
    wrk.create_from_string("dict.json", GEO_JOINT_DICT);

    let mut cmd = wrk.command("synthesize");
    cmd.args([
        "--min-count",
        "1",
        "-n",
        "20000",
        "--seed",
        "7",
        "--shard",
        "2/2",
        "--report",
    ])
    .arg(wrk.path("report.json"))
    .arg("--dictionary")
    .arg(wrk.path("dict.json"))
    .arg("data.csv");
    wrk.assert_success(&mut cmd);

    // Shard 2/2 is chunks 1-2: 20000 - 8192 rows.
    let report = read_report(&wrk);
    assert_eq!(report["synthetic_rows"], 11808);
    assert_eq!(report["shard"], "2/2");
    assert_eq!(report["privacy"]["exact_matches"], 11808);
    assert_eq!(report["privacy"]["exact_match_rate"], 1.0);
}

#[test]
fn synthesize_report_for_an_empty_shard_has_a_null_rate() {
    let wrk = Workdir::new("synthesize_shard_empty_report");
    wrk.create("data.csv", geo_fixture());
    wrk.create_from_string("dict.json", GEO_JOINT_DICT);

    // 10 rows = 1 chunk, which goes to shard 2/2, so 1/2 is header-only.
    let mut cmd = wrk.command("synthesize");
    cmd.args([
        "--min-count",
        "1",
        "-n",
        "10",
        "--seed",
        "7",
        "--shard",
        "1/2",
        "--report",
    ])
    .arg(wrk.path("report.json"))
    .arg("--dictionary")
    .arg(wrk.path("dict.json"))
    .arg("data.csv");
    wrk.assert_success(&mut cmd);

    let report = read_report(&wrk);
    assert_eq!(report["synthetic_rows"], 0);
    assert_eq!(report["privacy"]["exact_matches"], 0);
    assert_eq!(
        report.pointer("/privacy/exact_match_rate"),
        Some(&serde_json::Value::Null)
    );
}

// stats rejects --quantile-method approx on big-endian targets.
#[cfg(target_endian = "little")]
#[test]
fn synthesize_passes_on_stats_warnings() {
    let wrk = Workdir::new("synthesize_stats_warnings");
    chunked_fixture(&wrk);

    // Under approx quantiles stats warns that it drops MAD; synthesize must
    // not swallow stats' warnings, or a memory fallback would go unseen.
    let mut cmd = wrk.command("synthesize");
    cmd.args([
        "-n",
        "5",
        "--seed",
        "1",
        "--stats-options",
        "--mad --quantile-method approx",
        "data.csv",
    ]);
    let stderr = wrk.stderr_on_success(&mut cmd);
    assert!(stderr.contains("disabling MAD"), "got: {stderr}");
}

#[test]
fn synthesize_passes_on_frequency_warnings() {
    let wrk = Workdir::new("synthesize_frequency_warnings");
    chunked_fixture(&wrk);
    // An empty frequency cache makes frequency warn and recompute. It is only
    // read when strictly newer than data.csv, so don't rely on mtime ticks.
    wrk.create_from_string("data.freq.csv.data.jsonl", "");
    std::fs::File::options()
        .write(true)
        .open(wrk.path("data.freq.csv.data.jsonl"))
        .unwrap()
        .set_modified(std::time::SystemTime::now() + std::time::Duration::from_secs(60))
        .unwrap();

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "5", "--seed", "1", "data.csv"]);
    let stderr = wrk.stderr_on_success(&mut cmd);
    assert!(stderr.contains("Frequency cache is empty"), "got: {stderr}");
}

#[test]
fn synthesize_shard_refuses_infer_content_type() {
    let wrk = Workdir::new("synthesize_shard_infer");
    chunked_fixture(&wrk);

    let mut cmd = wrk.command("synthesize");
    cmd.args([
        "--seed",
        "1",
        "--shard",
        "1/2",
        "--infer-content-type",
        "data.csv",
    ]);
    assert!(
        wrk.output_stderr(&mut cmd)
            .contains("--shard needs a shared --dictionary"),
        "--shard with --infer-content-type must be refused before any LLM call"
    );
}

#[test]
fn synthesize_last_shard_of_max_rows_does_not_overflow() {
    let wrk = Workdir::new("synthesize_shard_max_rows");
    chunked_fixture(&wrk);

    // u64::MAX rows = 2^51 chunks; the last holds the final 8191 rows.
    let n = u64::MAX.to_string();
    let last = format!("{0}/{0}", 1_u64 << 51);
    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", &n, "--seed", "1", "--shard", &last, "--report"])
        .arg(wrk.path("report.json"))
        .arg("data.csv");
    let out = wrk.stdout::<String>(&mut cmd);
    assert_eq!(out.lines().count(), 8192);
    assert_eq!(read_report(&wrk)["synthetic_rows"], 8191);
}

/// A source and dictionary that reach every generator a model can hold:
/// enumerated (`tier`), faker pool (`email`), consistent fakes (`first_name`),
/// float range (`score`), free text (`note`), empty (`blank`), date range
/// (`joined`), and joint, ordered and correlated groups.
fn model_fixture(wrk: &Workdir) {
    let cities = [
        ("Springfield", "IL"),
        ("Chicago", "IL"),
        ("Boston", "MA"),
        ("Portland", "OR"),
    ];
    let mut rows = vec![svec![
        "tier",
        "email",
        "first_name",
        "score",
        "note",
        "blank",
        "joined",
        "city",
        "state",
        "created",
        "closed",
        "x",
        "y"
    ]];
    for i in 0..40_u32 {
        let (city, state) = cities[i as usize % 4];
        let (month, day) = (i % 12 + 1, i / 12 + 1);
        rows.push(vec![
            ["gold", "silver", "bronze"][i as usize % 3].to_string(),
            format!("user{i}@example.com"),
            ["SRC_Ann", "SRC_Bob", "SRC_Cy", "SRC_Di"][i as usize % 4].to_string(),
            format!("{:.3}", f64::from(i).mul_add(1.37, 0.123)),
            format!("free text note number {i} about things"),
            String::new(),
            format!("2021-{month:02}-{day:02}"),
            city.to_string(),
            state.to_string(),
            format!("2020-{month:02}-{day:02}"),
            format!("2020-{month:02}-{:02}", day + 10),
            (i * 10).to_string(),
            format!("{:.2}", f64::from(i).mul_add(9.5, f64::from(i % 3))),
        ]);
    }
    wrk.create("data.csv", rows);
    wrk.create_from_string(
        "dict.json",
        r#"{
        "fields": [
            {"name": "email", "type": "String", "content_type": "email"},
            {"name": "first_name", "type": "String", "content_type": "first_name"}
        ],
        "relationships": [
            {"kind": "joint", "members": ["city", "state"]},
            {"kind": "ordered", "members": ["created", "closed"], "anchor": "created"},
            {"kind": "correlated", "members": ["x", "y"]}
        ]
    }"#,
    );
}

/// A fitting run of `rows` rows with the model fixture's options, plus `extra`.
fn model_fit_cmd(wrk: &Workdir, rows: &str, extra: &[&str]) -> std::process::Command {
    let mut cmd = wrk.command("synthesize");
    cmd.args([
        "-n",
        rows,
        "--seed",
        "11",
        "--min-count",
        "1",
        "--consistent-fakes",
        "--dictionary",
    ])
    .arg(wrk.path("dict.json"))
    .arg("data.csv")
    .args(extra);
    cmd
}

#[test]
fn synthesize_model_reproduces_the_fitting_run() {
    let wrk = Workdir::new("synthesize_model_same_output");
    model_fixture(&wrk);

    // Raw stdout bytes: the promise is byte-identical output, line endings included.
    let direct = wrk.output(&mut model_fit_cmd(&wrk, "20000", &[])).stdout;
    let saving = wrk
        .output(&mut model_fit_cmd(
            &wrk,
            "20000",
            &["--save-model", "m.json"],
        ))
        .stdout;
    let mut from_model = wrk.command("synthesize");
    from_model.args(["--model", "m.json", "-n", "20000", "--seed", "11"]);
    let from_model = wrk.output(&mut from_model).stdout;

    let model = wrk.read_to_string("m.json").unwrap();
    for variant in [
        "\"FrequencyWeighted\":",
        "\"Faker\":",
        "\"FakerMapped\":",
        "\"NumericQuantile\":",
        "\"DateQuantile\":",
        "\"LoremFallback\":",
        "\"Empty\"",
        "\"Joint\":",
        "\"Ordered\":",
        "\"Correlated\":",
    ] {
        assert!(model.contains(variant), "model has no {variant} generator");
    }
    assert_eq!(direct.iter().filter(|&&b| b == b'\n').count(), 20001);
    assert!(saving == direct, "--save-model must not change the output");
    assert!(
        from_model == direct,
        "a --model run must write the bytes of the run that saved the model"
    );
}

#[test]
fn synthesize_model_shards_join_to_the_fitting_run() {
    let wrk = Workdir::new("synthesize_model_shards");
    model_fixture(&wrk);

    let direct = wrk.output(&mut model_fit_cmd(&wrk, "20000", &[])).stdout;
    let mut joined = wrk
        .output(&mut model_fit_cmd(
            &wrk,
            "20000",
            &["--shard", "1/3", "--save-model", "m.json"],
        ))
        .stdout;
    for shard in ["2/3", "3/3"] {
        let mut cmd = wrk.command("synthesize");
        cmd.args([
            "--model", "m.json", "-n", "20000", "--seed", "11", "--shard", shard,
        ]);
        let slice = wrk.output(&mut cmd).stdout;
        // Drop only the repeated header line, keeping every byte after it.
        let body = slice.iter().position(|&b| b == b'\n').unwrap() + 1;
        joined.extend_from_slice(&slice[body..]);
    }
    assert!(
        joined == direct,
        "model slices joined in order must equal the fitting run's bytes"
    );
}

#[test]
fn synthesize_model_refuses_fit_options() {
    let wrk = Workdir::new("synthesize_model_refusals");
    model_fixture(&wrk);
    wrk.output(&mut model_fit_cmd(&wrk, "10", &["--save-model", "m.json"]));

    for extra in [
        &["data.csv"][..],
        &["--dictionary", "dict.json"],
        &["--report", "r.json"],
        &["--save-model", "m2.json"],
        &["--delimiter", ";"],
        &["--quantiles", "4"],
    ] {
        let mut cmd = wrk.command("synthesize");
        cmd.args(["--model", "m.json"]).args(extra);
        let err = wrk.output_stderr(&mut cmd);
        assert!(
            err.contains("--model can't be combined with"),
            "{extra:?}: {err}"
        );
    }

    // Defaulted fit options are ignored with --model, even values a fit refuses.
    let mut ignored = wrk.command("synthesize");
    ignored.args([
        "--model",
        "m.json",
        "-n",
        "3",
        "--locale",
        "xx",
        "--min-count",
        "0",
    ]);
    assert_eq!(
        wrk.output(&mut ignored)
            .stdout
            .split(|&b| b == b'\n')
            .count(),
        5
    );

    let before = wrk.read_to_string("m.json").unwrap();
    let mut overwrite = wrk.command("synthesize");
    overwrite.args(["--model", "m.json", "-o", "m.json"]);
    assert!(
        wrk.stderr_on_error(&mut overwrite)
            .contains("is the same file as an input")
    );
    assert_eq!(
        wrk.read_to_string("m.json").unwrap(),
        before,
        "a refused run must leave the model intact"
    );

    let mut clash = model_fit_cmd(&wrk, "10", &["--save-model", "data.csv"]);
    assert!(
        wrk.output_stderr(&mut clash)
            .contains("--save-model must not be the input")
    );
}

#[test]
fn synthesize_model_rejects_other_versions_and_broken_models() {
    let wrk = Workdir::new("synthesize_model_broken");
    model_fixture(&wrk);
    wrk.output(&mut model_fit_cmd(&wrk, "10", &["--save-model", "m.json"]));
    let model: serde_json::Value =
        serde_json::from_str(&wrk.read_to_string("m.json").unwrap()).unwrap();

    let expect_error = |name: &str, model: &serde_json::Value, needle: &str| {
        wrk.create_from_string(name, &model.to_string());
        let mut cmd = wrk.command("synthesize");
        cmd.args(["--model", name]);
        let err = wrk.output_stderr(&mut cmd);
        assert!(
            err.contains(needle),
            "{name}: expected '{needle}', got: {err}"
        );
    };

    let mut newer = model.clone();
    newer["qsv_synthesize_model"] = 99.into();
    expect_error("newer.json", &newer, "model format version 99");
    expect_error(
        "other.json",
        &serde_json::json!({}),
        "not a model written by",
    );

    // Column 0 is filled twice, so the last column is never filled.
    let mut twice = model.clone();
    let units = twice["schedule"].as_array_mut().unwrap();
    let first = units[0].clone();
    *units.last_mut().unwrap() = first;
    expect_error("twice.json", &twice, "is generated twice");

    // A version 1 model has no column types, so --format couldn't use it.
    let mut v1 = model.clone();
    v1["qsv_synthesize_model"] = 1.into();
    expect_error("v1.json", &v1, "model format version 1");

    // A version 2 model has no decimal places, so Float columns would print
    // unrounded.
    let mut v2 = model.clone();
    v2["qsv_synthesize_model"] = 2.into();
    expect_error("v2.json", &v2, "model format version 2");

    // A version 3 model has no --target classes.
    let mut v3 = model.clone();
    v3["qsv_synthesize_model"] = 3.into();
    expect_error("v3.json", &v3, "model format version 3");

    let mut untyped = model.clone();
    untyped["types"].as_array_mut().unwrap().pop();
    expect_error(
        "untyped.json",
        &untyped,
        "a type list that doesn't match its columns",
    );

    let mut short = model.clone();
    short["columns"].as_array_mut().unwrap().pop();
    short["types"].as_array_mut().unwrap().pop();
    expect_error("short.json", &short, "out of range");

    // tier is enumerated: an empty value list would panic when sampled.
    let mut empty = model;
    let tier = &mut empty["schedule"][0]["Independent"]["generator"]["FrequencyWeighted"];
    tier["values"] = serde_json::json!([]);
    tier["cumulative"] = serde_json::json!([]);
    expect_error("empty.json", &empty, "a value list that is empty");
}

#[test]
fn synthesize_model_leaves_out_values_below_min_count() {
    let wrk = Workdir::new("synthesize_model_min_count");
    wrk.create("data.csv", min_count_fixture());

    let mut cmd = wrk.command("synthesize");
    cmd.args([
        "-n",
        "10",
        "--seed",
        "1",
        "--save-model",
        "m.json",
        "data.csv",
    ]);
    wrk.output(&mut cmd);
    let model = wrk.read_to_string("m.json").unwrap();
    assert!(model.contains("\"A\""), "kept values belong in the model");
    assert!(
        !model.contains("RARE"),
        "a value below --min-count must not reach the model"
    );
}

/// The header of a CSV file whose column names hold no commas.
fn csv_header(wrk: &Workdir, name: &str) -> Vec<String> {
    let raw = wrk.read_to_string(name).unwrap();
    raw.lines()
        .next()
        .unwrap()
        .split(',')
        .map(String::from)
        .collect()
}

/// Parses JSONL output into one JSON object per row.
fn jsonl_rows(raw: &str) -> Vec<serde_json::Map<String, serde_json::Value>> {
    raw.lines()
        .map(|line| match serde_json::from_str(line).unwrap() {
            serde_json::Value::Object(map) => map,
            other => panic!("not a JSON object: {other}"),
        })
        .collect()
}

#[test]
fn synthesize_format_jsonl_holds_the_csv_values_typed() {
    let wrk = Workdir::new("synthesize_format_jsonl_values");
    model_fixture(&wrk);
    let mut csv_cmd = model_fit_cmd(&wrk, "300", &["-o", "out.csv"]);
    wrk.assert_success(&mut csv_cmd);
    let mut jsonl_cmd = model_fit_cmd(&wrk, "300", &["--format", "jsonl", "-o", "out.jsonl"]);
    wrk.assert_success(&mut jsonl_cmd);

    let csv_rows = wrk.read_csv("out.csv");
    let header = csv_header(&wrk, "out.csv");
    let rows = jsonl_rows(&wrk.read_to_string("out.jsonl").unwrap());
    assert_eq!((rows.len(), csv_rows.len()), (300, 300));
    for (row, csv_row) in rows.iter().zip(&csv_rows) {
        assert_eq!(
            row.keys().collect::<Vec<_>>(),
            header.iter().collect::<Vec<_>>()
        );
        for (name, cell) in header.iter().zip(csv_row) {
            let value = &row[name];
            match name.as_str() {
                _ if cell.is_empty() => assert!(value.is_null(), "{name}: {value}"),
                "x" => assert_eq!(value.as_i64(), cell.parse().ok(), "{name}"),
                "score" | "y" => assert_eq!(value.as_f64(), cell.parse().ok(), "{name}"),
                _ => assert_eq!(value.as_str(), Some(cell.as_str()), "{name}"),
            }
        }
    }
    assert!(rows.iter().all(|r| r["blank"].is_null()));
}

#[test]
fn synthesize_format_jsonl_keeps_zero_padded_codes_as_text() {
    let wrk = Workdir::new("synthesize_format_jsonl_codes");
    let mut rows = vec![svec!["zip", "n"]];
    for i in 0..40 {
        rows.push(vec![
            ["00501", "02134", "10001", "90210"][i % 4].to_string(),
            (i % 5).to_string(),
        ]);
    }
    wrk.create("data.csv", rows);
    let mut cmd = wrk.command("synthesize");
    cmd.args([
        "-n",
        "50",
        "--seed",
        "3",
        "--min-count",
        "1",
        "--format",
        "jsonl",
    ])
    .arg("data.csv");
    let rows = jsonl_rows(&wrk.stdout::<String>(&mut cmd));
    assert_eq!(rows.len(), 50);
    for row in &rows {
        let zip = row["zip"].as_str().expect("zip is text");
        assert!(["00501", "02134", "10001", "90210"].contains(&zip), "{zip}");
        assert!((0..5).contains(&row["n"].as_i64().expect("n is an integer")));
    }
    assert!(rows.iter().any(|r| r["zip"] == "00501"));
}

#[test]
fn synthesize_format_is_the_same_at_any_jobs_and_from_a_model() {
    let wrk = Workdir::new("synthesize_format_reproducible");
    model_fixture(&wrk);
    let mut formats = vec![("jsonl", "jsonl")];
    if cfg!(feature = "polars") {
        formats.push(("parquet", "parquet"));
    }
    // 70,000 rows span more than one Parquet row group.
    for (format, ext) in formats {
        let fitted = format!("fit.{ext}");
        let mut fit = model_fit_cmd(
            &wrk,
            "70000",
            &[
                "--format",
                format,
                "-j",
                "1",
                "--save-model",
                "m.json",
                "-o",
                &fitted,
            ],
        );
        wrk.assert_success(&mut fit);
        let threaded = format!("threaded.{ext}");
        let mut more_jobs = model_fit_cmd(
            &wrk,
            "70000",
            &["--format", format, "-j", "4", "-o", &threaded],
        );
        wrk.assert_success(&mut more_jobs);
        let from_model = format!("model.{ext}");
        let mut model_run = wrk.command("synthesize");
        model_run.args([
            "--model",
            "m.json",
            "-n",
            "70000",
            "--seed",
            "11",
            "--format",
            format,
            "-j",
            "3",
            "-o",
            &from_model,
        ]);
        wrk.assert_success(&mut model_run);

        let bytes = |name: &str| std::fs::read(wrk.path(name)).unwrap();
        assert!(
            bytes(&fitted) == bytes(&threaded),
            "{format}: --jobs changed the output"
        );
        assert!(
            bytes(&fitted) == bytes(&from_model),
            "{format}: --model changed the output"
        );
    }
}

#[test]
#[cfg(feature = "polars")]
fn synthesize_format_parquet_keeps_types_and_nulls() {
    use polars::prelude::DataType;

    let wrk = Workdir::new("synthesize_format_parquet");
    model_fixture(&wrk);
    let mut cmd = model_fit_cmd(&wrk, "500", &["--format", "parquet", "-o", "out.parquet"]);
    wrk.assert_success(&mut cmd);
    let mut csv_cmd = model_fit_cmd(&wrk, "500", &["-o", "out.csv"]);
    wrk.assert_success(&mut csv_cmd);

    let df = polars::prelude::LazyFrame::scan_parquet(
        polars::prelude::PlRefPath::new(wrk.path("out.parquet").to_string_lossy().as_ref()),
        Default::default(),
    )
    .unwrap()
    .collect()
    .unwrap();
    assert_eq!(df.height(), 500);
    for (name, dtype) in [
        ("tier", DataType::String),
        ("email", DataType::String),
        ("score", DataType::Float64),
        ("blank", DataType::String),
        ("joined", DataType::Date),
        ("city", DataType::String),
        ("created", DataType::Date),
        ("closed", DataType::Date),
        ("x", DataType::Int64),
        ("y", DataType::Float64),
    ] {
        assert_eq!(df.column(name).unwrap().dtype(), &dtype, "{name}");
    }
    assert_eq!(df.column("blank").unwrap().null_count(), 500);

    // Same values as the CSV run, typed.
    let csv_rows = wrk.read_csv("out.csv");
    let header = csv_header(&wrk, "out.csv");
    let col = |name: &str| header.iter().position(|h| h == name).unwrap();
    let x = df.column("x").unwrap().i64().unwrap();
    let joined = df
        .column("joined")
        .unwrap()
        .cast(&DataType::String)
        .unwrap();
    let joined = joined.str().unwrap();
    assert_eq!(csv_rows.len(), 500);
    for (i, row) in csv_rows.iter().enumerate() {
        assert_eq!(x.get(i), row[col("x")].parse().ok(), "x row {i}");
        let cell = &row[col("joined")];
        assert_eq!(
            joined.get(i),
            (!cell.is_empty()).then_some(cell.as_str()),
            "joined row {i}"
        );
    }
}

#[test]
fn synthesize_format_refusals() {
    let wrk = Workdir::new("synthesize_format_refusals");
    wrk.create("data.csv", vec![svec!["a", "a"], svec!["1", "2"]]);

    let mut bad = wrk.command("synthesize");
    bad.args(["--format", "xml", "data.csv"]);
    assert!(
        wrk.stderr_on_error(&mut bad)
            .contains("--format must be csv, jsonl or parquet")
    );

    let mut stdout = wrk.command("synthesize");
    stdout.args(["--format", "parquet", "data.csv"]);
    assert!(
        wrk.stderr_on_error(&mut stdout)
            .contains(if cfg!(feature = "polars") {
                "Parquet can't be written to stdout"
            } else {
                "needs a qsv built with the polars feature"
            })
    );

    let mut dup = wrk.command("synthesize");
    dup.args(["--format", "jsonl", "data.csv"]);
    assert!(
        wrk.stderr_on_error(&mut dup)
            .contains("needs unique column names, but 'a' appears more than once")
    );
}

/// The files a `--rows-per-file` run wrote for `prefix`, in name order.
fn split_files(wrk: &Workdir, prefix: &str) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(wrk.path("."))
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .filter(|n| n.starts_with(prefix))
        .collect();
    names.sort();
    names
}

/// Joins split CSV or JSONL files as `qsv cat rows` or plain concatenation
/// would: CSV files after the first lose their header line.
fn join_split_text(wrk: &Workdir, files: &[String], csv: bool) -> Vec<u8> {
    let mut joined = Vec::new();
    for (i, name) in files.iter().enumerate() {
        let bytes = std::fs::read(wrk.path(name)).unwrap();
        let skip = if csv && i > 0 {
            bytes.iter().position(|&b| b == b'\n').unwrap() + 1
        } else {
            0
        };
        joined.extend_from_slice(&bytes[skip..]);
    }
    joined
}

#[cfg(feature = "polars")]
fn read_parquet(wrk: &Workdir, name: &str) -> polars::prelude::DataFrame {
    polars::prelude::LazyFrame::scan_parquet(
        polars::prelude::PlRefPath::new(wrk.path(name).to_string_lossy().as_ref()),
        Default::default(),
    )
    .unwrap()
    .collect()
    .unwrap()
}

#[test]
fn synthesize_rows_per_file_joins_back_to_the_single_file() {
    let wrk = Workdir::new("synthesize_rows_per_file_join");
    model_fixture(&wrk);
    let mut formats = vec![("csv", "csv"), ("jsonl", "jsonl")];
    if cfg!(feature = "polars") {
        formats.push(("parquet", "parquet"));
    }
    for (format, ext) in formats {
        let single = format!("single.{ext}");
        let mut one = model_fit_cmd(
            &wrk,
            "20000",
            &["--format", format, "--save-model", "m.json", "-o", &single],
        );
        wrk.assert_success(&mut one);
        let model_prefix = format!("{format}_model_");
        let mut from_model = wrk.command("synthesize");
        from_model.args([
            "--model",
            "m.json",
            "-n",
            "20000",
            "--seed",
            "11",
            "--format",
            format,
            "--rows-per-file",
            "10000",
            "-o",
            &format!("{model_prefix}{{}}.{ext}"),
        ]);
        wrk.assert_success(&mut from_model);
        let model_files = split_files(&wrk, &model_prefix);
        assert_eq!(model_files.len(), 2, "{format}: --model file count");
        // Below one 8192-row block, cutting blocks mid-way, and on block
        // boundaries (each later file starts a block).
        for per_file in [3000_u64, 10000, 8192] {
            let starts: Vec<u64> = (0..20000).step_by(per_file as usize).collect();
            let width = starts.last().unwrap().to_string().len();
            let mut previous: Option<Vec<Vec<u8>>> = None;
            for jobs in ["1", "4"] {
                let prefix = format!("{format}_{per_file}_j{jobs}_");
                let mut split = model_fit_cmd(
                    &wrk,
                    "20000",
                    &[
                        "--format",
                        format,
                        "--rows-per-file",
                        &per_file.to_string(),
                        "-j",
                        jobs,
                        "-o",
                        &format!("{prefix}{{}}.{ext}"),
                    ],
                );
                wrk.assert_success(&mut split);
                let files = split_files(&wrk, &prefix);
                let expected: Vec<String> = starts
                    .iter()
                    .map(|s| format!("{prefix}{s:0>width$}.{ext}"))
                    .collect();
                assert_eq!(files, expected, "{format} {per_file}: file names");

                let bytes: Vec<Vec<u8>> = files
                    .iter()
                    .map(|f| std::fs::read(wrk.path(f)).unwrap())
                    .collect();
                if let Some(previous) = &previous {
                    assert!(
                        previous == &bytes,
                        "{format} {per_file}: --jobs changed the files"
                    );
                }
                if per_file == 10000 {
                    let model_bytes: Vec<Vec<u8>> = model_files
                        .iter()
                        .map(|f| std::fs::read(wrk.path(f)).unwrap())
                        .collect();
                    assert!(model_bytes == bytes, "{format}: --model changed the files");
                }
                previous = Some(bytes);

                if format == "parquet" {
                    #[cfg(feature = "polars")]
                    {
                        let mut joined = read_parquet(&wrk, &files[0]);
                        for f in &files[1..] {
                            let part = read_parquet(&wrk, f);
                            assert!(part.height() as u64 <= per_file);
                            joined.vstack_mut(&part).unwrap();
                        }
                        assert!(
                            joined.equals_missing(&read_parquet(&wrk, &single)),
                            "parquet {per_file}: joined files differ from one file"
                        );
                    }
                } else {
                    assert!(
                        join_split_text(&wrk, &files, format == "csv")
                            == std::fs::read(wrk.path(&single)).unwrap(),
                        "{format} {per_file}: joined files differ from one file"
                    );
                }
            }
        }
    }
}

#[test]
fn synthesize_rows_per_file_cuts_between_rows_holding_newlines() {
    let wrk = Workdir::new("synthesize_rows_per_file_newlines");
    let mut rows = vec![svec!["id", "note"]];
    for i in 0..30 {
        rows.push(vec![
            (i % 3).to_string(),
            format!("line one {}\nline two", i % 3),
        ]);
    }
    wrk.create("data.csv", rows);
    let run = |extra: &[&str]| {
        let mut cmd = wrk.command("synthesize");
        cmd.args(["data.csv", "-n", "50", "--seed", "3", "--min-count", "1"])
            .args(extra);
        cmd
    };
    wrk.assert_success(&mut run(&["-o", "single.csv"]));
    wrk.assert_success(&mut run(&["--rows-per-file", "7", "-o", "nl_{}.csv"]));
    let files = split_files(&wrk, "nl_");
    assert_eq!(files.len(), 8);
    let single = std::fs::read_to_string(wrk.path("single.csv")).unwrap();
    assert!(
        single.contains("\nline two"),
        "the pool must hold the newlines"
    );
    assert!(join_split_text(&wrk, &files, true) == single.into_bytes());
    for f in &files[..7] {
        let mut rdr = csv::Reader::from_path(wrk.path(f)).unwrap();
        assert_eq!(rdr.records().count(), 7, "{f}");
    }
}

#[test]
fn synthesize_rows_per_file_refusals() {
    let wrk = Workdir::new("synthesize_rows_per_file_refusals");
    wrk.create("p_10.csv", vec![svec!["a"], svec!["1"], svec!["2"]]);
    let refused = |extra: &[&str], message: &str| {
        let mut cmd = wrk.command("synthesize");
        cmd.args(["p_10.csv", "-n", "20", "--seed", "1"])
            .args(extra);
        let stderr = wrk.stderr_on_error(&mut cmd);
        assert!(stderr.contains(message), "{extra:?}: {stderr}");
    };
    refused(
        &["--rows-per-file", "0", "-o", "o_{}.csv"],
        "must be greater than 0",
    );
    refused(
        &["--rows-per-file", "5", "--shard", "1/2", "-o", "o_{}.csv"],
        "can't be combined with --shard",
    );
    refused(&["--rows-per-file", "5"], "--rows-per-file needs --output");
    refused(
        &["--rows-per-file", "5", "-o", "-"],
        "--rows-per-file needs --output",
    );
    for template in ["out.csv", "o_{}_{}.csv", "dir_{}/out.csv"] {
        refused(
            &["--rows-per-file", "5", "-o", template],
            "needs an --output name with one {} in its file name",
        );
    }
    // p_{}.csv writes p_00.csv and p_10.csv: the second is the input.
    refused(
        &["--rows-per-file", "10", "-o", "p_{}.csv"],
        "--rows-per-file would write over 'p_10.csv'",
    );
    assert_eq!(
        std::fs::read_to_string(wrk.path("p_10.csv")).unwrap(),
        "a\n1\n2\n"
    );
}

#[test]
#[cfg(feature = "polars")]
fn synthesize_rows_per_file_finishes_a_parquet_file_before_opening_the_next() {
    let wrk = Workdir::new("synthesize_rows_per_file_open_fails");
    wrk.create("data.csv", vec![svec!["a"], svec!["1"], svec!["2"]]);
    // The second file's name is taken by a directory, so opening it fails.
    std::fs::create_dir(wrk.path("q_100.parquet")).unwrap();
    let mut cmd = wrk.command("synthesize");
    cmd.args([
        "data.csv",
        "-n",
        "300",
        "--seed",
        "1",
        "--format",
        "parquet",
        "--rows-per-file",
        "100",
        "-o",
        "q_{}.parquet",
    ]);
    wrk.assert_err(&mut cmd);
    assert_eq!(read_parquet(&wrk, "q_000.parquet").height(), 100);
}

#[test]
fn synthesize_format_refuses_parquet_to_stdout_dash() {
    let wrk = Workdir::new("synthesize_format_parquet_dash");
    wrk.create("data.csv", vec![svec!["a"], svec!["1"]]);
    let mut cmd = wrk.command("synthesize");
    cmd.args(["--format", "parquet", "-o", "-", "data.csv"]);
    assert!(
        wrk.stderr_on_error(&mut cmd)
            .contains(if cfg!(feature = "polars") {
                "Parquet can't be written to stdout"
            } else {
                "needs a qsv built with the polars feature"
            })
    );
    assert!(!wrk.path("-").exists(), "no file named '-' is created");
}

#[test]
fn synthesize_format_jsonl_writes_inferred_booleans_as_booleans() {
    let wrk = Workdir::new("synthesize_format_jsonl_booleans");
    let mut rows = vec![svec!["flag"]];
    for i in 0..40 {
        rows.push(vec![if i % 3 == 0 { "true" } else { "false" }.to_string()]);
    }
    wrk.create("data.csv", rows);
    let mut cmd = wrk.command("synthesize");
    cmd.args([
        "-n",
        "30",
        "--seed",
        "5",
        "--min-count",
        "1",
        "--format",
        "jsonl",
        "--stats-options",
        "--infer-boolean",
        "data.csv",
    ]);
    let rows = jsonl_rows(&wrk.stdout::<String>(&mut cmd));
    assert_eq!(rows.len(), 30);
    assert!(rows.iter().all(|r| r["flag"].is_boolean()), "{rows:?}");
    assert!(rows.iter().any(|r| r["flag"] == true) && rows.iter().any(|r| r["flag"] == false));
}

#[test]
fn synthesize_quantiles_4_is_the_default() {
    let wrk = Workdir::new("synthesize_quantiles_default");
    model_fixture(&wrk);

    let default = wrk.output(&mut model_fit_cmd(&wrk, "5000", &[])).stdout;
    let explicit = wrk
        .output(&mut model_fit_cmd(&wrk, "5000", &["--quantiles", "4"]))
        .stdout;
    let finer = wrk
        .output(&mut model_fit_cmd(&wrk, "5000", &["--quantiles", "20"]))
        .stdout;
    assert!(
        explicit == default,
        "--quantiles 4 must write the default bytes"
    );
    assert!(finer != default, "--quantiles 20 must change the buckets");
}

#[test]
fn synthesize_quantiles_is_the_same_at_any_jobs_and_from_a_model() {
    let wrk = Workdir::new("synthesize_quantiles_jobs_model");
    model_fixture(&wrk);

    let q = ["--quantiles", "20"];
    let j1 = wrk
        .output(&mut model_fit_cmd(
            &wrk,
            "20000",
            &[&q[..], &["-j", "1"]].concat(),
        ))
        .stdout;
    let j4 = wrk
        .output(&mut model_fit_cmd(
            &wrk,
            "20000",
            &[&q[..], &["-j", "4", "--save-model", "m.json"]].concat(),
        ))
        .stdout;
    let mut from_model = wrk.command("synthesize");
    from_model.args(["--model", "m.json", "-n", "20000", "--seed", "11"]);
    let from_model = wrk.output(&mut from_model).stdout;

    assert!(j4 == j1, "--quantiles output must not depend on --jobs");
    assert!(
        from_model == j1,
        "a --model run must write the bytes of the --quantiles run that saved it"
    );
}

/// Sorted non-empty values of column `col` of a CSV.
fn sorted_column(csv: &str, col: usize) -> Vec<f64> {
    let mut v: Vec<f64> = csv
        .lines()
        .skip(1)
        .filter_map(|l| l.split(',').nth(col)?.parse().ok())
        .collect();
    v.sort_by(f64::total_cmp);
    v
}

#[test]
fn synthesize_quantiles_keep_a_long_tail() {
    let wrk = Workdir::new("synthesize_quantiles_tail");
    // 2^(i/100): each value doubles every 100 rows, so the top quartile spans
    // three quarters of the range and one uniform bucket can't follow it.
    let mut csv = String::from("v\n");
    for i in 1..=2000 {
        csv.push_str(&format!("{:.3}\n", 2f64.powf(f64::from(i) / 100.0)));
    }
    wrk.create_from_string("data.csv", &csv);
    let source_p90 = 2f64.powi(18);

    let p90 = |quantiles: &str| {
        let mut cmd = wrk.command("synthesize");
        cmd.args(["-n", "20000", "--seed", "3", "--quantiles", quantiles])
            .arg("data.csv");
        let v = sorted_column(&wrk.stdout::<String>(&mut cmd), 0);
        assert_eq!(v.len(), 20000);
        v[v.len() * 9 / 10]
    };
    let coarse = p90("4");
    let fine = p90("20");
    assert!(
        coarse > source_p90 * 1.5,
        "quartile buckets should overshoot the tail: p90 {coarse} vs {source_p90}"
    );
    assert!(
        (fine / source_p90 - 1.0).abs() < 0.1,
        "--quantiles 20 should keep p90 within 10%: {fine} vs {source_p90}"
    );
}

#[test]
fn synthesize_quantiles_rejects_bad_values_and_percentile_stats_options() {
    let wrk = Workdir::new("synthesize_quantiles_refusals");
    wrk.create("data.csv", vec![svec!["v"], svec!["1"], svec!["2"]]);

    for q in ["0", "1", "3", "8", "200"] {
        let mut cmd = wrk.command("synthesize");
        cmd.args(["--quantiles", q, "data.csv"]);
        let err = wrk.output_stderr(&mut cmd);
        assert!(err.contains("--quantiles must be one of"), "{q}: {err}");
    }
    for opt in ["--percentiles", "--percentile-list=5,95"] {
        let mut cmd = wrk.command("synthesize");
        cmd.args(["--quantiles", "10", "--stats-options", opt, "data.csv"]);
        let err = wrk.output_stderr(&mut cmd);
        assert!(
            err.contains("synthesize sets the percentiles itself"),
            "{opt}: {err}"
        );
    }
}

/// Bucket counts of every quantile generator in a saved model, in model order.
fn model_bucket_counts(model: &str) -> Vec<usize> {
    fn walk(v: &serde_json::Value, out: &mut Vec<usize>) {
        match v {
            serde_json::Value::Object(map) => {
                for (k, v) in map {
                    if k == "NumericQuantile" || k == "DateQuantile" {
                        out.push(v["buckets"].as_array().unwrap().len());
                    } else {
                        walk(v, out);
                    }
                }
            },
            serde_json::Value::Array(items) => items.iter().for_each(|v| walk(v, out)),
            _ => {},
        }
    }
    let mut out = Vec::new();
    walk(&serde_json::from_str(model).unwrap(), &mut out);
    out
}

#[test]
fn synthesize_quantiles_cap_thin_buckets_at_min_count() {
    let wrk = Workdir::new("synthesize_quantiles_cap");
    let mut csv = String::from("v\n");
    for i in 0..60 {
        csv.push_str(&format!("{}\n", i * i));
    }
    wrk.create_from_string("data.csv", &csv);

    // Every bucket must hold MORE than --min-count values. 60 at the default 5:
    // 10 buckets of 6 fit, so 100 → 10. At 1: 50 buckets of 1 don't, 25 of 2 do.
    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "10", "--quantiles", "100", "--save-model", "m.json"])
        .arg("data.csv");
    wrk.output(&mut cmd);
    assert_eq!(
        model_bucket_counts(&wrk.read_to_string("m.json").unwrap()),
        [10]
    );

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "10", "--quantiles", "100", "--min-count", "1"])
        .args(["--save-model", "m1.json", "data.csv"]);
    wrk.output(&mut cmd);
    assert_eq!(
        model_bucket_counts(&wrk.read_to_string("m1.json").unwrap()),
        [25]
    );
}

#[test]
fn synthesize_quantiles_reproduce_tied_values() {
    let wrk = Workdir::new("synthesize_quantiles_ties");
    // Few distinct values, so most buckets are zero-width; --freq-limit 1 keeps
    // the columns off frequency sampling and on their buckets.
    let mut csv = String::from("i,d,ts\n");
    for r in 0..1000 {
        let (i, d, ts) = if r % 4 == 0 {
            ("9", "2021-06-30", "2023-05-05T12:00:00+00:00")
        } else {
            ("1", "2020-01-01", "2022-01-01T00:00:00+00:00")
        };
        csv.push_str(&format!("{i},{d},{ts}\n"));
    }
    wrk.create_from_string("data.csv", &csv);

    let mut cmd = wrk.command("synthesize");
    cmd.args([
        "-n",
        "4000",
        "--seed",
        "1",
        "--freq-limit",
        "1",
        "--quantiles",
        "100",
    ])
    .args(["--save-model", "m.json", "data.csv"]);
    let out: String = wrk.stdout(&mut cmd);
    assert_eq!(
        model_bucket_counts(&wrk.read_to_string("m.json").unwrap()),
        [100, 100, 100]
    );
    let rows: Vec<&str> = out.lines().skip(1).collect();
    assert_eq!(rows.len(), 4000);
    // 75% of each source column is its low value; zero-width buckets emit it
    // exactly. Columns are independent, so check each one on its own.
    for (col, low_value) in ["1", "2020-01-01", "2022-01-01T00:00:00+00:00"]
        .iter()
        .enumerate()
    {
        #[allow(clippy::cast_precision_loss)]
        let low = rows
            .iter()
            .filter(|r| r.split(',').nth(col) == Some(low_value))
            .count() as f64
            / 4000.0;
        assert!(
            (low - 0.75).abs() < 0.05,
            "column {col}: low-value share {low}"
        );
    }
}

#[test]
fn synthesize_quantiles_keep_small_float_precision() {
    let wrk = Workdir::new("synthesize_quantiles_precision");
    // 100 distinct values from 1e-7 to 1e-5: rounded to stats' default 4
    // places every percentile would read 0, collapsing the buckets.
    let mut csv = String::from("v\n");
    for i in 1..=100 {
        csv.push_str(&format!("{:e}\n", f64::from(i) * 1e-7));
    }
    wrk.create_from_string("data.csv", &csv);

    let mut cmd = wrk.command("synthesize");
    cmd.args([
        "-n",
        "2000",
        "--seed",
        "1",
        "--quantiles",
        "10",
        "--freq-limit",
        "1",
    ])
    .args(["--min-count", "1", "data.csv"]);
    let v = sorted_column(&wrk.stdout::<String>(&mut cmd), 0);
    let median = v[v.len() / 2];
    assert!(
        (median / 5e-6 - 1.0).abs() < 0.15,
        "median {median} should be near 5e-6"
    );
}

/// Decimal places of a numeric cell as written (`12.5` has 1, `3` has 0).
fn written_decimals(cell: &str) -> usize {
    cell.split_once('.').map_or(0, |(_, frac)| frac.len())
}

/// `price` has 2 places, `ratio` 3, `noise` binary-float noise (stats counts
/// 17 places, so it stays unrounded) and `whole` 1 (`stats` counts `12.0` as
/// 1 place, so whole-number draws print as `12`).
fn float_decimals_fixture(wrk: &Workdir) {
    let mut csv = String::from("price,ratio,noise,whole\n");
    for i in 1..=60 {
        let f = f64::from(i);
        csv.push_str(&format!(
            "{:.2},{:.3},{},{f:.1}\n",
            f * 1.37,
            f / 7.0,
            f * 0.1 + 0.2
        ));
    }
    wrk.create_from_string("data.csv", &csv);
}

#[test]
fn synthesize_floats_keep_source_decimals() {
    let wrk = Workdir::new("synthesize_float_decimals");
    float_decimals_fixture(&wrk);

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "2000", "--seed", "3", "--freq-limit", "1", "data.csv"]);
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let max_places = |col: usize| {
        got[1..]
            .iter()
            .filter(|r| !r[col].is_empty())
            .map(|r| written_decimals(&r[col]))
            .max()
            .unwrap()
    };
    assert_eq!(max_places(0), 2, "price");
    assert_eq!(max_places(1), 3, "ratio");
    assert!(max_places(2) > 10, "noise should stay unrounded");
    assert_eq!(max_places(3), 1, "whole");
    assert!(
        got[1..].iter().any(|r| !r[3].contains('.')),
        "some whole-number draws should print without a point"
    );
    for r in &got[1..] {
        assert!(!r[0].starts_with("-0"), "negative zero: {r:?}");
    }

    // A Float value printed without a point is still typed as a float.
    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "50", "--seed", "3", "--freq-limit", "1"])
        .args(["--format", "jsonl", "data.csv"]);
    for row in jsonl_rows(&wrk.stdout::<String>(&mut cmd)) {
        assert!(row["whole"].is_f64(), "whole should be a float: {row:?}");
    }
}

#[test]
fn synthesize_ordered_mixed_decimals_keep_order_and_range() {
    let wrk = Workdir::new("synthesize_ordered_mixed_decimals");
    // A 2-place Float, a 1-place Float and an Integer, in order, with gaps
    // smaller than the coarser grids: rounding `b` to 1 place can take it
    // below `a` (10.42 -> 10.4) unless it is moved up a step.
    let mut rows = vec![svec!["a", "b", "c"]];
    for i in 10..50 {
        rows.push(vec![
            format!("{i}.37"),
            format!("{i}.4"),
            format!("{}", i + 1),
        ]);
    }
    wrk.create("data.csv", rows.clone());
    wrk.create_from_string(
        "dict.json",
        r#"{
            "fields": [
                {"name": "a", "type": "Float"},
                {"name": "b", "type": "Float"},
                {"name": "c", "type": "Integer"}
            ],
            "relationships": [{"kind": "ordered", "members": ["a", "b", "c"]}]
        }"#,
    );

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "3000", "--seed", "8", "--dictionary"])
        .arg(wrk.path("dict.json"))
        .arg("data.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    // `--min-count 5` trims each range to its 5th smallest and largest values.
    let trimmed: Vec<(f64, f64)> = (0..3)
        .map(|c| {
            let mut v: Vec<f64> = rows[1..].iter().map(|r| r[c].parse().unwrap()).collect();
            v.sort_by(f64::total_cmp);
            (v[4], v[v.len() - 5])
        })
        .collect();
    for r in &got[1..] {
        let v: Vec<f64> = r.iter().map(|s| s.parse().unwrap()).collect();
        assert!(v[0] <= v[1] && v[1] <= v[2], "order broken: {r:?}");
        for (c, &(lo, hi)) in trimmed.iter().enumerate() {
            assert!(
                lo <= v[c] && v[c] <= hi,
                "column {c} out of {lo}..={hi}: {r:?}"
            );
        }
        assert!(written_decimals(&r[0]) <= 2, "{r:?}");
        assert!(written_decimals(&r[1]) <= 1, "{r:?}");
        assert_eq!(written_decimals(&r[2]), 0, "{r:?}");
    }
}

#[test]
fn synthesize_correlated_floats_keep_source_decimals() {
    let wrk = Workdir::new("synthesize_correlated_decimals");
    let mut csv = String::from("x,y\n");
    for i in 1..=40 {
        let f = f64::from(i);
        csv.push_str(&format!("{:.2},{:.1}\n", f * 1.13, f * 2.05 + 1.0));
    }
    wrk.create_from_string("data.csv", &csv);
    wrk.create_from_string(
        "dict.json",
        &CORRELATED_DICT.replace("\"Integer\"", "\"Float\""),
    );

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "2000", "--seed", "5", "--dictionary"])
        .arg(wrk.path("dict.json"))
        .arg("data.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let places = |col: usize| got[1..].iter().map(|r| written_decimals(&r[col])).max();
    assert_eq!(places(0), Some(2), "x");
    assert_eq!(places(1), Some(1), "y");
}

#[test]
fn synthesize_float_decimals_are_the_same_at_any_jobs_and_from_a_model() {
    let wrk = Workdir::new("synthesize_float_decimals_jobs_model");
    float_decimals_fixture(&wrk);
    let run = |extra: &[&str]| {
        let mut cmd = wrk.command("synthesize");
        cmd.args(["-n", "20000", "--seed", "4", "--freq-limit", "1"])
            .args(extra);
        wrk.stdout::<String>(&mut cmd)
    };
    let j1 = run(&["-j", "1", "data.csv"]);
    let j4 = run(&["-j", "4", "--save-model", "m.json", "data.csv"]);
    let from_model = run(&["--model", "m.json"]);
    assert_eq!(j1, j4, "-j 1 vs -j 4");
    assert_eq!(j4, from_model, "fit vs --model");

    // The model carries each column's places (noise: none).
    let model = wrk.read_to_string("m.json").unwrap();
    let decimals: Vec<serde_json::Value> = serde_json::from_str::<serde_json::Value>(&model)
        .unwrap()["schedule"]
        .as_array()
        .unwrap()
        .iter()
        .map(|unit| unit["Independent"]["generator"]["NumericQuantile"]["decimals"].clone())
        .collect();
    assert_eq!(
        decimals,
        [2.into(), 3.into(), serde_json::Value::Null, 1.into()]
    );
}

#[test]
#[cfg(feature = "polars")]
fn synthesize_format_parquet_keeps_whole_number_floats_as_floats() {
    use polars::prelude::DataType;

    let wrk = Workdir::new("synthesize_float_decimals_parquet");
    float_decimals_fixture(&wrk);
    let run = |extra: &[&str]| {
        let mut cmd = wrk.command("synthesize");
        cmd.args(["-n", "500", "--seed", "4", "--freq-limit", "1"])
            .args(extra)
            .arg("data.csv");
        wrk.assert_success(&mut cmd);
    };
    run(&["--format", "parquet", "-o", "out.parquet"]);
    run(&["-o", "out.csv"]);

    let df = read_parquet(&wrk, "out.parquet");
    let whole = df.column("whole").unwrap();
    assert_eq!(whole.dtype(), &DataType::Float64);
    let whole = whole.f64().unwrap();
    let csv_rows = wrk.read_csv("out.csv");
    assert!(
        csv_rows.iter().any(|r| !r[3].contains('.')),
        "the CSV should have whole-number draws"
    );
    for (i, row) in csv_rows.iter().enumerate() {
        assert_eq!(whole.get(i), row[3].parse().ok(), "whole row {i}");
    }
}

/// An imbalanced 0/1 label: 10% of rows are label 1, whose `amount` is about
/// 10x larger and whose `region` is mostly "south" (label 0: mostly "north").
fn target_fixture(wrk: &Workdir) {
    let mut csv = String::from("id,label,amount,region\n");
    for i in 0..2000_u32 {
        let positive = i % 10 == 0;
        let jitter = f64::from(i % 97);
        let (amount, region) = if positive {
            (500.0 + jitter, if i % 20 == 0 { "south" } else { "east" })
        } else {
            (
                50.0 + jitter / 10.0,
                if i % 7 == 0 { "east" } else { "north" },
            )
        };
        csv.push_str(&format!(
            "{i},{},{amount:.2},{region}\n",
            u8::from(positive)
        ));
    }
    wrk.create_from_string("data.csv", &csv);
}

/// Per-class (share of rows, mean `amount`, share of `region == region`).
fn class_profile(rows: &[Vec<String>], label: &str, region: &str) -> (f64, f64, f64) {
    let class: Vec<&Vec<String>> = rows[1..].iter().filter(|r| r[1] == label).collect();
    let n = class.len() as f64;
    let mean = class
        .iter()
        .map(|r| r[2].parse::<f64>().unwrap())
        .sum::<f64>()
        / n;
    let share = class.iter().filter(|r| r[3] == region).count() as f64 / n;
    (n / (rows.len() - 1) as f64, mean, share)
}

#[test]
fn synthesize_target_keeps_class_shares_and_per_class_features() {
    let wrk = Workdir::new("synthesize_target_classes");
    target_fixture(&wrk);
    let source = wrk.read_csv("data.csv");
    let mut cmd = wrk.command("synthesize");
    cmd.args([
        "-n", "20000", "--seed", "1", "--target", "label", "data.csv",
    ]);
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    for (label, region) in [("0", "north"), ("1", "south")] {
        let (src_share, src_mean, src_region) = class_profile(&source, label, region);
        let (share, mean, region_share) = class_profile(&got, label, region);
        assert!(
            (share - src_share).abs() < 0.01,
            "label {label} share {share}"
        );
        assert!(
            (mean / src_mean - 1.0).abs() < 0.05,
            "label {label} amount mean {mean} vs {src_mean}"
        );
        assert!(
            (region_share - src_region).abs() < 0.03,
            "label {label} {region} share {region_share} vs {src_region}"
        );
    }

    // Without --target the features ignore the label.
    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "20000", "--seed", "1", "data.csv"]);
    let plain: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let (_, mean, _) = class_profile(&plain, "1", "south");
    assert!(mean < 300.0, "label 1 amount mean {mean} without --target");
}

#[test]
fn synthesize_target_is_the_same_at_any_jobs_from_a_model_and_split() {
    let wrk = Workdir::new("synthesize_target_jobs_model");
    target_fixture(&wrk);
    let run = |extra: &[&str]| {
        let mut cmd = wrk.command("synthesize");
        cmd.args(["-n", "20000", "--seed", "6"]).args(extra);
        wrk.stdout::<String>(&mut cmd)
    };
    let fit = &["--target", "label", "data.csv"];
    let j1 = run(&[&["-j", "1"][..], fit].concat());
    let j4 = run(&[&["-j", "4", "--save-model", "m.json"][..], fit].concat());
    assert_eq!(j1, j4, "-j 1 vs -j 4");
    assert_eq!(j4, run(&["--model", "m.json"]), "fit vs --model");
    let shards: String = (1..=3)
        .map(|k| {
            let spec = format!("{k}/3");
            let out = run(&["--model", "m.json", "--shard", &spec]);
            if k == 1 {
                out + "\n"
            } else {
                out.split_once('\n')
                    .map(|(_, rest)| rest.to_string() + "\n")
                    .unwrap_or_default()
            }
        })
        .collect();
    assert_eq!(j4 + "\n", shards, "joined --shard slices");

    let mut cmd = wrk.command("synthesize");
    cmd.args(["--model", "m.json", "--target", "label"]);
    assert!(
        wrk.output_stderr(&mut cmd)
            .contains("can't be combined with --target")
    );
}

#[test]
fn synthesize_target_bins_a_numeric_target_by_its_buckets() {
    let wrk = Workdir::new("synthesize_target_bins");
    // 1000 distinct ids, too many to split by value; `double` follows `id`.
    let mut csv = String::from("id,double\n");
    for i in 0..1000 {
        csv.push_str(&format!("{i},{}\n", i * 2));
    }
    wrk.create_from_string("data.csv", &csv);
    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "8000", "--seed", "2", "--target", "id", "data.csv"]);
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let mut ids: Vec<i64> = got[1..].iter().map(|r| r[0].parse().unwrap()).collect();
    ids.sort_unstable();
    // The default --min-count 5 trims the range to [4, 995].
    assert!(
        ids[0] >= 4 && *ids.last().unwrap() <= 995,
        "{:?}",
        (ids[0], ids.last())
    );
    let median = ids[ids.len() / 2];
    assert!((450..=550).contains(&median), "median id {median}");
    // Each row's `double` comes from its id's bucket (quartiles: 250 ids, so
    // `double` spans about 500), never from another bucket.
    for r in &got[1..] {
        let id: f64 = r[0].parse().unwrap();
        let double: f64 = r[1].parse().unwrap();
        assert!((double - 2.0 * id).abs() <= 520.0, "{r:?}");
    }
}

#[test]
fn synthesize_target_refusals_and_thin_classes() {
    let wrk = Workdir::new("synthesize_target_refusals");
    let mut csv = String::from("name,v\n");
    for i in 0..120 {
        csv.push_str(&format!("n{i},{}\n", i % 3));
    }
    // Class "9" appears 3 times, below the default --min-count of 5.
    csv.push_str("x,9\nx,9\nx,9\n");
    wrk.create_from_string("data.csv", &csv);

    let mut cmd = wrk.command("synthesize");
    cmd.args(["--target", "missing", "data.csv"]);
    assert!(
        wrk.output_stderr(&mut cmd)
            .contains("'missing' is not in <input>")
    );
    let mut cmd = wrk.command("synthesize");
    cmd.args(["--target", "name", "data.csv"]);
    assert!(
        wrk.output_stderr(&mut cmd)
            .contains("more than 50 distinct values and isn't numeric")
    );

    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "3000", "--seed", "1", "--target", "v", "--report"])
        .arg(wrk.path("report.json"))
        .arg("data.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    assert!(
        got[1..].iter().all(|r| r[1] != "9"),
        "the thin class leaked"
    );
    let report = read_report(&wrk);
    assert_eq!(report["target"], "v");
    assert!(
        report["privacy"]["suppressed"]
            .as_array()
            .unwrap()
            .contains(&serde_json::json!({"columns": ["v"], "distinct": 1, "rows": 3})),
        "{report}"
    );
}

#[test]
fn synthesize_target_falls_back_for_columns_a_class_cannot_profile() {
    let wrk = Workdir::new("synthesize_target_fallback");
    // Class "b" has 6 rows: too few for --min-count 5 to keep a range of
    // `amount` (it needs 2k-1 = 9 values), so it uses the whole-input one.
    let mut csv = String::from("cls,amount\n");
    for i in 0..200 {
        csv.push_str(&format!("a,{}.5\n", i));
    }
    for i in 0..6 {
        csv.push_str(&format!("b,{}.5\n", 1000 + i));
    }
    wrk.create_from_string("data.csv", &csv);
    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "3000", "--seed", "4", "--target", "cls", "data.csv"]);
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let b: Vec<&Vec<String>> = got[1..].iter().filter(|r| r[0] == "b").collect();
    assert!(!b.is_empty());
    assert!(
        b.iter().all(|r| !r[1].is_empty()),
        "class b amount left empty"
    );
}

#[test]
fn synthesize_target_drops_itself_from_relationships() {
    let wrk = Workdir::new("synthesize_target_relationships");
    target_fixture(&wrk);
    // Both groups name the target. In a class the target is constant, so
    // left in, the correlated pair couldn't be fitted and
    // --strict-relationships would abort; dropped from the members, each
    // group keeps one column and is dropped. The run must succeed and still
    // condition on the label.
    wrk.create_from_string(
        "dict.json",
        r#"{"fields": [], "relationships": [
            {"kind": "correlated", "members": ["label", "amount"]},
            {"kind": "joint", "members": ["label", "region"]}
        ]}"#,
    );
    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "5000", "--seed", "3", "--target", "label"])
        .args(["--strict-relationships", "--dictionary"])
        .arg(wrk.path("dict.json"))
        .arg("data.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let (_, _, south) = class_profile(&got, "1", "south");
    assert!((south - 0.5).abs() < 0.08, "label 1 south share {south}");
}

#[test]
fn synthesize_reads_non_utf8_input() {
    let wrk = Workdir::new("synthesize_non_utf8");
    // Non-UTF8 cells in a plain string column and in a joint member, next to
    // range-trimmed, ordered and correlated columns: every pass over the
    // source (range trimming, each relationship's, `frequency`'s output) must
    // read them, writing invalid bytes as U+FFFD as `stats` does. (`stats`
    // itself refuses a non-UTF8 header.)
    let mut data = b"name,region,kind,start,end,x,y\n".to_vec();
    for i in 0..200_u32 {
        let region: &[u8] = if i % 2 == 0 { b"north\xfe" } else { b"south" };
        data.extend_from_slice(format!("n{}", i % 10).as_bytes());
        data.extend_from_slice(b"\xff,");
        data.extend_from_slice(region);
        data.extend_from_slice(
            format!(",k{},{i},{},{i},{}\n", i % 3, i + 5 + i % 7, 2 * i + i % 3).as_bytes(),
        );
    }
    std::fs::write(wrk.path("data.csv"), data).unwrap();
    wrk.create_from_string(
        "dict.json",
        r#"{"fields": [], "relationships": [
            {"kind": "ordered", "members": ["start", "end"]},
            {"kind": "correlated", "members": ["x", "y"]},
            {"kind": "joint", "members": ["region", "kind"]}
        ]}"#,
    );
    for target in [None, Some("region")] {
        let mut cmd = wrk.command("synthesize");
        cmd.args(["-n", "300", "--seed", "5", "--strict-relationships"])
            .arg("--dictionary")
            .arg(wrk.path("dict.json"))
            .arg("data.csv");
        if let Some(target) = target {
            cmd.args(["--target", target]);
        }
        let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
        assert_eq!(
            got[0],
            ["name", "region", "kind", "start", "end", "x", "y"],
            "{target:?}"
        );
        assert_eq!(got.len(), 301, "{target:?}");
        let mut regions = std::collections::HashSet::new();
        for row in &got[1..] {
            regions.insert(row[1].as_str());
            let (start, end): (f64, f64) = (row[3].parse().unwrap(), row[4].parse().unwrap());
            assert!(start <= end, "{target:?}: {row:?}");
        }
        assert_eq!(
            regions,
            std::collections::HashSet::from(["north\u{fffd}", "south"]),
            "{target:?}"
        );
    }

    // Every synthetic row here copies a source row, so --report must count
    // matches although the copies hold U+FFFD where the source has bytes.
    let mut data = b"name,amount\n".to_vec();
    for i in 1..=6 {
        data.extend_from_slice(b"ab\xff,");
        data.extend_from_slice(format!("{i}\n").as_bytes());
    }
    std::fs::write(wrk.path("copies.csv"), data).unwrap();
    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "20", "--seed", "1", "--min-count", "1", "--report"])
        .arg(wrk.path("report.json"))
        .arg("copies.csv");
    wrk.assert_success(&mut cmd);
    let report: serde_json::Value =
        serde_json::from_str(&wrk.read_to_string("report.json").unwrap()).unwrap();
    assert_eq!(report["privacy"]["exact_matches"], 20, "{report}");

    // Headerless, the first row is read where a header would be.
    let mut data = b"x\xff,1\n".to_vec();
    for i in 2..30 {
        data.extend_from_slice(format!("y{},{i}\n", i % 3).as_bytes());
    }
    std::fs::write(wrk.path("headerless.csv"), data).unwrap();
    let mut cmd = wrk.command("synthesize");
    cmd.env("QSV_NO_HEADERS", "1")
        .args(["-n", "50", "--seed", "5"])
        .arg("headerless.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    assert_eq!(got.len(), 51);
}

#[test]
fn synthesize_target_strict_relationships_abort_on_a_thin_class() {
    let wrk = Workdir::new("synthesize_target_strict");
    // Class "b" has 6 rows of distinct (city, zip) tuples, all below
    // --min-count 5, so its joint group can't be kept.
    let mut csv = String::from("cls,city,zip\n");
    for i in 0..100 {
        csv.push_str(&format!("a,c{},{}\n", i % 2, 100 + i % 2));
    }
    for i in 0..6 {
        csv.push_str(&format!("b,u{i},{}\n", 900 + i));
    }
    wrk.create_from_string("data.csv", &csv);
    wrk.create_from_string(
        "dict.json",
        r#"{"fields": [], "relationships": [{"kind": "joint", "members": ["city", "zip"]}]}"#,
    );
    let run = |strict: bool| {
        let mut cmd = wrk.command("synthesize");
        cmd.args([
            "-n",
            "100",
            "--seed",
            "1",
            "--target",
            "cls",
            "--dictionary",
        ])
        .arg(wrk.path("dict.json"));
        if strict {
            cmd.arg("--strict-relationships");
        }
        cmd.arg("data.csv");
        cmd
    };
    wrk.assert_success(&mut run(false));
    // Without --strict-relationships the same message is only a warning, so
    // the exit status is what shows the abort.
    let out = wrk.output(&mut run(true));
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "expected the thin class to abort: {err}"
    );
    assert!(
        err.contains("all 6 distinct tuple(s) occur fewer than --min-count"),
        "{err}"
    );
}

#[test]
fn synthesize_target_keeps_one_consistent_fake_per_value_across_classes() {
    let wrk = Workdir::new("synthesize_target_consistent_fakes");
    let mut csv = String::from("cls,first_name\n");
    for i in 0..300 {
        let name = ["SRC_Michael", "SRC_Sarah", "SRC_Tom"][i % 3];
        csv.push_str(&format!(
            "{},{name}\n",
            ["a", "b", "c"][i % 2 + (i % 5 == 0) as usize]
        ));
    }
    wrk.create_from_string("data.csv", &csv);
    wrk.create_from_string(
        "dict.json",
        r#"{"fields": [{"name": "first_name", "type": "String", "content_type": "first_name"}],
            "enum_threshold": 10}"#,
    );
    let mut cmd = wrk.command("synthesize");
    cmd.args([
        "-n",
        "1000",
        "--seed",
        "5",
        "--consistent-fakes",
        "--target",
        "cls",
    ])
    .arg("--dictionary")
    .arg(wrk.path("dict.json"))
    .arg("data.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let fakes: std::collections::HashSet<&str> = got[1..].iter().map(|r| r[1].as_str()).collect();
    assert_eq!(fakes.len(), 3, "one fake per source value: {fakes:?}");
    assert!(fakes.iter().all(|f| !f.starts_with("SRC_")));
}

#[test]
fn synthesize_target_reads_no_headers_and_a_custom_delimiter() {
    let wrk = Workdir::new("synthesize_target_no_headers");
    let mut csv = String::new();
    for i in 0..400 {
        let positive = i % 4 == 0;
        csv.push_str(&format!(
            "{};{}\n",
            if positive { "y" } else { "n" },
            if positive { 900 + i % 50 } else { 10 + i % 50 }
        ));
    }
    wrk.create_from_string("data.ssv", &csv);
    let mut cmd = wrk.command("synthesize");
    cmd.env("QSV_NO_HEADERS", "1")
        .args([
            "-n",
            "2000",
            "--seed",
            "2",
            "--delimiter",
            ";",
            "--target",
            "0",
        ])
        .arg("data.ssv");
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    for r in &got[1..] {
        let v: i64 = r[1].parse().unwrap();
        assert_eq!(r[0] == "y", v >= 900, "{r:?}");
    }
}

#[test]
#[cfg(feature = "polars")]
fn synthesize_format_parquet_holds_a_column_classes_type_differently() {
    use polars::prelude::DataType;

    let wrk = Workdir::new("synthesize_target_parquet");
    // `score` is all whole numbers in class "a" (Integer) and has decimals in
    // class "b" (Float); the column is Float for the whole input.
    let mut csv = String::from("cls,score\n");
    for i in 0..200 {
        if i % 2 == 0 {
            csv.push_str(&format!("a,{}\n", i));
        } else {
            csv.push_str(&format!("b,{}.25\n", i));
        }
    }
    wrk.create_from_string("data.csv", &csv);
    let mut cmd = wrk.command("synthesize");
    cmd.args([
        "-n", "500", "--seed", "1", "--target", "cls", "--format", "parquet",
    ])
    .args(["-o", "out.parquet", "data.csv"]);
    wrk.assert_success(&mut cmd);
    let df = read_parquet(&wrk, "out.parquet");
    assert_eq!(df.column("score").unwrap().dtype(), &DataType::Float64);
    assert_eq!(df.height(), 500);
}

#[test]
fn synthesize_target_classes_use_the_dictionary_by_column_name() {
    let wrk = Workdir::new("synthesize_target_dictionary");
    let mut csv = String::from("cls,day\n");
    for i in 0..200 {
        csv.push_str(&format!(
            "{},2024-{:02}-{:02}\n",
            ["a", "b"][i % 2],
            1 + i % 12,
            1 + i % 28
        ));
    }
    wrk.create_from_string("data.csv", &csv);
    wrk.create_from_string(
        "dict.json",
        r#"{"fields": [{"name": "day", "type": "Date", "content_type": "date:%d/%m/%Y"}]}"#,
    );
    let mut cmd = wrk.command("synthesize");
    cmd.args([
        "-n",
        "500",
        "--seed",
        "2",
        "--target",
        "cls",
        "--dictionary",
    ])
    .arg(wrk.path("dict.json"))
    .arg("data.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    for r in &got[1..] {
        let parts: Vec<&str> = r[1].split('/').collect();
        assert!(
            parts.len() == 3 && parts[2].len() == 4,
            "day not in the dictionary's %d/%m/%Y format: {r:?}"
        );
    }
}

#[test]
fn synthesize_model_rejects_broken_target_classes() {
    let wrk = Workdir::new("synthesize_target_broken_model");
    target_fixture(&wrk);
    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "10", "--seed", "1", "--target", "label"])
        .args(["--save-model", "m.json", "data.csv"]);
    wrk.assert_success(&mut cmd);
    let model: serde_json::Value =
        serde_json::from_str(&wrk.read_to_string("m.json").unwrap()).unwrap();
    let expect_error = |name: &str, model: &serde_json::Value, needle: &str| {
        wrk.create_from_string(name, &model.to_string());
        let mut cmd = wrk.command("synthesize");
        cmd.args(["--model", name]);
        let err = wrk.output_stderr(&mut cmd);
        assert!(
            err.contains(needle),
            "{name}: expected '{needle}', got: {err}"
        );
    };

    let mut missing = model.clone();
    missing["target"]["classes"][0]["schedule"]
        .as_array_mut()
        .unwrap()
        .pop();
    expect_error("missing.json", &missing, "is never generated");

    let mut out_of_range = model.clone();
    out_of_range["target"]["col_idx"] = 99.into();
    expect_error(
        "range.json",
        &out_of_range,
        "target column 99 is out of range",
    );

    let mut no_classes = model;
    no_classes["target"]["classes"] = serde_json::json!([]);
    expect_error("empty.json", &no_classes, "a target with no classes");
}

#[test]
fn synthesize_target_report_lists_only_suppressions_that_shape_the_output() {
    let wrk = Workdir::new("synthesize_target_report_suppressions");
    // `first_name` is a fake-value column (its whole-input generator is
    // used), with a name rare within each class; `v` is a binned target.
    let mut csv = String::from("v,first_name\n");
    for i in 0..400 {
        let name = if i % 50 == 0 {
            "SRC_Rare"
        } else {
            ["SRC_A", "SRC_B"][i % 2]
        };
        csv.push_str(&format!("{i},{name}\n"));
    }
    wrk.create_from_string("data.csv", &csv);
    wrk.create_from_string(
        "dict.json",
        r#"{"fields": [{"name": "first_name", "type": "String", "content_type": "first_name"}],
            "enum_threshold": 10}"#,
    );
    let mut cmd = wrk.command("synthesize");
    cmd.args([
        "-n",
        "200",
        "--seed",
        "1",
        "--consistent-fakes",
        "--target",
        "v",
    ])
    .arg("--dictionary")
    .arg(wrk.path("dict.json"))
    .arg("--report")
    .arg(wrk.path("report.json"))
    .arg("data.csv");
    wrk.assert_success(&mut cmd);
    let report = read_report(&wrk);
    let suppressed = report["privacy"]["suppressed"].as_array().unwrap();
    assert!(
        suppressed.iter().all(|s| s.get("class").is_none()),
        "suppressions from discarded class generators: {suppressed:?}"
    );
}

/// Class "b" has 3 distinct dates (enumerated in the class) while the column
/// overall has 200 (generated from buckets in the dictionary's format), and
/// unique zero-padded codes (filler text in the class, enumerated overall).
fn target_type_fixture(wrk: &Workdir) {
    let mut csv = String::from("cls,day,code\n");
    for i in 0..200 {
        csv.push_str(&format!(
            "a,2024-{:02}-{:02},{:05}\n",
            1 + i / 28,
            1 + i % 28,
            i % 20
        ));
    }
    for i in 0..30 {
        csv.push_str(&format!("b,2026-03-0{},{:05}\n", 1 + i % 3, 900 + i));
    }
    wrk.create_from_string("data.csv", &csv);
    wrk.create_from_string(
        "dict.json",
        r#"{"fields": [{"name": "day", "type": "Date", "content_type": "date:%d/%m/%Y"}]}"#,
    );
}

#[test]
fn synthesize_target_classes_keep_each_column_written_one_way() {
    let wrk = Workdir::new("synthesize_target_one_form");
    target_type_fixture(&wrk);
    let mut cmd = wrk.command("synthesize");
    cmd.args([
        "-n",
        "600",
        "--seed",
        "1",
        "--min-count",
        "1",
        "--target",
        "cls",
    ])
    .arg("--dictionary")
    .arg(wrk.path("dict.json"))
    .arg("data.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    assert!(got[1..].iter().any(|r| r[0] == "b"));
    for r in &got[1..] {
        assert_eq!(r[1].split('/').count(), 3, "day not in %d/%m/%Y: {r:?}");
        assert!(
            r[2].len() == 5 && r[2].bytes().all(|b| b.is_ascii_digit()),
            "code lost its zero padding: {r:?}"
        );
    }
}

#[test]
#[cfg(feature = "polars")]
fn synthesize_target_parquet_holds_columns_classes_generate_differently() {
    use polars::prelude::DataType;

    let wrk = Workdir::new("synthesize_target_parquet_forms");
    target_type_fixture(&wrk);
    let mut cmd = wrk.command("synthesize");
    cmd.args([
        "-n",
        "600",
        "--seed",
        "1",
        "--min-count",
        "1",
        "--target",
        "cls",
    ])
    .args(["--format", "parquet", "-o", "out.parquet", "--dictionary"])
    .arg(wrk.path("dict.json"))
    .arg("data.csv");
    wrk.assert_success(&mut cmd);
    let df = read_parquet(&wrk, "out.parquet");
    assert_eq!(df.height(), 600);
    assert_eq!(df.column("day").unwrap().dtype(), &DataType::Date);
    assert_eq!(df.column("code").unwrap().dtype(), &DataType::String);
}

#[test]
fn synthesize_target_splits_50_values_and_an_empty_one_by_value() {
    let wrk = Workdir::new("synthesize_target_cap");
    let rows = |values: usize| {
        let mut csv = String::from("cls,v\n");
        for i in 0..values * 6 {
            csv.push_str(&format!("c{:02},{i}\n", i % values));
        }
        csv.push_str(&",1\n".repeat(6));
        csv
    };
    wrk.create_from_string("fifty.csv", &rows(50));
    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "2000", "--seed", "1", "--target", "cls", "fifty.csv"]);
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let classes: std::collections::HashSet<&str> = got[1..].iter().map(|r| r[0].as_str()).collect();
    assert_eq!(classes.len(), 51, "50 values and the empty class");

    wrk.create_from_string("fiftyone.csv", &rows(51));
    let mut cmd = wrk.command("synthesize");
    cmd.args(["--target", "cls", "fiftyone.csv"]);
    assert!(
        wrk.output_stderr(&mut cmd)
            .contains("more than 50 distinct values and isn't numeric")
    );
}

#[test]
fn synthesize_target_trims_labels_like_frequency() {
    let wrk = Workdir::new("synthesize_target_trim");
    let mut csv = String::from("cls,v\n");
    for i in 0..100 {
        // " yes" and "yes" are one class, as `frequency` counts them.
        let label = if i % 2 == 0 { " yes" } else { "yes" };
        csv.push_str(&format!("\"{label}\",{i}\n"));
    }
    for i in 0..100 {
        csv.push_str(&format!("no,{}\n", 1000 + i));
    }
    wrk.create_from_string("data.csv", &csv);
    let mut cmd = wrk.command("synthesize");
    cmd.args(["-n", "1000", "--seed", "1", "--target", "cls", "data.csv"]);
    let got: Vec<Vec<String>> = wrk.read_stdout_on_success(&mut cmd);
    let classes: std::collections::HashSet<&str> = got[1..].iter().map(|r| r[0].as_str()).collect();
    assert_eq!(
        classes,
        ["yes", "no"]
            .into_iter()
            .collect::<std::collections::HashSet<_>>()
    );
    for r in &got[1..] {
        let v: f64 = r[1].parse().unwrap();
        assert_eq!(r[0] == "no", v >= 1000.0, "{r:?}");
    }
}

#[test]
fn synthesize_target_rows_per_file_join_to_the_single_file() {
    let wrk = Workdir::new("synthesize_target_rows_per_file");
    target_fixture(&wrk);
    let mut single = wrk.command("synthesize");
    single.args([
        "-n", "20000", "--seed", "8", "--target", "label", "data.csv",
    ]);
    let single = wrk.stdout::<String>(&mut single);
    let mut split = wrk.command("synthesize");
    split
        .args(["-n", "20000", "--seed", "8", "--target", "label"])
        .args(["--rows-per-file", "7000", "-o", "out_{}.csv", "data.csv"]);
    wrk.assert_success(&mut split);
    let joined: String = ["out_00000.csv", "out_07000.csv", "out_14000.csv"]
        .iter()
        .enumerate()
        .map(|(i, name)| {
            let text = wrk.read_to_string(name).unwrap();
            if i == 0 {
                text
            } else {
                text.split_once('\n').unwrap().1.to_string()
            }
        })
        .collect();
    assert_eq!(single + "\n", joined);
}
