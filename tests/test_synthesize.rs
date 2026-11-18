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
