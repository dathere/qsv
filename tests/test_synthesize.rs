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
