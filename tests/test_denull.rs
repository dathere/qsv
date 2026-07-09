use crate::workdir::Workdir;

/// Numeric column whose only non-numeric value is a known sentinel -> confirmed.
#[test]
fn denull_confirms_numeric_with_sentinel() {
    let wrk = Workdir::new("denull_confirms_numeric_with_sentinel");
    let mut rows = String::from("depth\n");
    for i in 0..50 {
        if i % 5 == 0 {
            rows.push_str("NULL\n");
        } else {
            rows.push_str(&format!("{}\n", i + 1));
        }
    }
    wrk.create_from_string("d.csv", &rows);

    let mut cmd = wrk.command("denull");
    cmd.arg("d.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got[0][0], "field");
    assert_eq!(got[1][0], "depth");
    assert_eq!(got[1][1], "confirmed");
    assert_eq!(got[1][2], "NULL");
    assert_eq!(got[1][3], "10"); // rows_affected
    assert_eq!(got[1][6], "Integer"); // promotes_to
}

/// A float column promotes to Float, not Integer.
#[test]
fn denull_promotes_to_float() {
    let wrk = Workdir::new("denull_promotes_to_float");
    let mut rows = String::from("v\n");
    for i in 0..20 {
        if i % 4 == 0 {
            rows.push_str("N/A\n");
        } else {
            rows.push_str(&format!("{}.5\n", i));
        }
    }
    wrk.create_from_string("d.csv", &rows);

    let mut cmd = wrk.command("denull");
    cmd.arg("d.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got[1][1], "confirmed");
    assert_eq!(got[1][6], "Float");
}

/// The load-bearing guard: a categorical that merely CONTAINS "NULL" must not be
/// touched, because its other values are not sentinels.
#[test]
fn denull_rejects_off_vocab_categorical() {
    let wrk = Workdir::new("denull_rejects_off_vocab_categorical");
    let mut rows = String::from("status\n");
    for i in 0..40 {
        rows.push_str(match i % 4 {
            0 => "NULL\n",
            1 => "OK\n",
            2 => "1\n",
            _ => "2\n",
        });
    }
    wrk.create_from_string("d.csv", &rows);

    let mut cmd = wrk.command("denull");
    cmd.arg("d.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got[1][1], "rejected:off-vocab");
    assert!(
        got[1][7].contains("OK"),
        "evidence should name the off-vocab value, got: {}",
        got[1][7]
    );
    // the `sentinels` column reports what was OBSERVED, so a rejected column still names
    // the sentinel it holds - that is precisely why the user is being told about it.
    assert_eq!(got[1][2], "NULL");
    assert!(
        got[1][6].is_empty(),
        "a rejected column promotes to nothing"
    );
}

/// Zero-padded codes (zip/FIPS) parse as numbers but masking a sentinel would eat
/// their leading zeros. A single sighting disqualifies the column.
#[test]
fn denull_rejects_zero_padded_codes() {
    let wrk = Workdir::new("denull_rejects_zero_padded_codes");
    let mut rows = String::from("zip\n");
    for i in 0..30 {
        if i % 6 == 0 {
            rows.push_str("NULL\n");
        } else {
            rows.push_str(&format!("0{:04}\n", 1000 + i));
        }
    }
    wrk.create_from_string("d.csv", &rows);

    let mut cmd = wrk.command("denull");
    cmd.arg("d.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got[1][1], "rejected:zero-padded");
}

/// Free text disqualifies once it exceeds --max-distinct, bounding memory. It is
/// reported ONLY when it also holds >=2 distinct numeric values; a pure text column
/// is not denull's business and stays out of the report entirely.
#[test]
fn denull_bounds_free_text_and_ignores_pure_text() {
    let wrk = Workdir::new("denull_bounds_free_text_and_ignores_pure_text");
    let mut rows = String::from("mixed,pure\n");
    for i in 0..60 {
        // `mixed` holds a KNOWN sentinel plus many distinct other non-numeric values
        if i % 3 == 0 {
            rows.push_str("NULL,");
        } else if i % 3 == 1 {
            rows.push_str(&format!("note{i},"));
        } else {
            rows.push_str(&format!("{i},"));
        }
        rows.push_str(&format!("text{i}\n"));
    }
    wrk.create_from_string("d.csv", &rows);

    let mut cmd = wrk.command("denull");
    cmd.args(["d.csv", "--max-distinct", "5"]);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let fields: Vec<&str> = got[1..].iter().map(|r| r[0].as_str()).collect();
    assert_eq!(
        fields,
        vec!["mixed"],
        "`pure` has no numeric content and must not be reported"
    );
    assert_eq!(got[1][1], "rejected:too-many-distinct");
    assert_eq!(
        got[1][2], "NULL",
        "the observed sentinel is named even after overflow"
    );
}

/// A clean numeric column has nothing to report, and a column with a single distinct
/// numeric value among letters is an ordinary categorical, not a failed sentinel.
#[test]
fn denull_is_silent_on_clean_and_categorical_columns() {
    let wrk = Workdir::new("denull_is_silent_on_clean_and_categorical_columns");
    let mut rows = String::from("clean,onenum\n");
    for i in 0..30 {
        rows.push_str(&format!("{i},"));
        rows.push_str(if i == 0 { "7\n" } else { "Alpha\n" });
    }
    wrk.create_from_string("d.csv", &rows);

    let mut cmd = wrk.command("denull");
    cmd.arg("d.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got.len(), 1, "header only; got {got:?}");

    // --all-columns surfaces them as `clean`
    let mut cmd = wrk.command("denull");
    cmd.args(["d.csv", "--all-columns"]);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got.len(), 3);
    assert_eq!(got[1][1], "clean");
    assert_eq!(got[2][1], "clean");
}

/// --add-vocab extends the built-in list; the same token is off-vocab without it.
#[test]
fn denull_add_vocab_extends_builtin() {
    let wrk = Workdir::new("denull_add_vocab_extends_builtin");
    let mut rows = String::from("v\n");
    for i in 0..30 {
        if i % 3 == 0 {
            rows.push_str("no reading\n");
        } else {
            rows.push_str(&format!("{i}\n"));
        }
    }
    wrk.create_from_string("d.csv", &rows);

    // predominantly numeric (20 numbers vs 10 markers), so the unknown token surfaces
    // as an off-vocab rejection that points the user at --add-vocab.
    let mut cmd = wrk.command("denull");
    cmd.arg("d.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got[1][1], "rejected:off-vocab");
    assert!(
        got[1][7].contains("--add-vocab"),
        "an unknown token in a numeric column should suggest --add-vocab, got: {}",
        got[1][7]
    );

    let mut cmd = wrk.command("denull");
    cmd.args(["d.csv", "--add-vocab", "no reading"]);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got[1][1], "confirmed");
    assert_eq!(got[1][2], "no reading");
}

/// Sentinels are matched case-insensitively after trimming.
#[test]
fn denull_matches_case_insensitively_and_trims() {
    let wrk = Workdir::new("denull_matches_case_insensitively_and_trims");
    let mut rows = String::from("v\n");
    for i in 0..30 {
        match i % 3 {
            0 => rows.push_str("\"  null \"\n"),
            1 => rows.push_str("N/a\n"),
            _ => rows.push_str(&format!("{i}\n")),
        }
    }
    wrk.create_from_string("d.csv", &rows);

    let mut cmd = wrk.command("denull");
    cmd.arg("d.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got[1][1], "confirmed");
    assert_eq!(got[1][3], "20", "both spellings counted");
}

/// -s/--select scopes the scan.
#[test]
fn denull_select_scopes_the_scan() {
    let wrk = Workdir::new("denull_select_scopes_the_scan");
    let mut rows = String::from("a,b\n");
    for i in 0..20 {
        let cell = if i % 2 == 0 {
            "NULL".to_string()
        } else {
            i.to_string()
        };
        rows.push_str(&format!("{cell},{cell}\n"));
    }
    wrk.create_from_string("d.csv", &rows);

    let mut cmd = wrk.command("denull");
    cmd.args(["d.csv", "-s", "b"]);
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let fields: Vec<&str> = got[1..].iter().map(|r| r[0].as_str()).collect();
    assert_eq!(fields, vec!["b"]);
}

/// Numeric sentinels are never proposed: -999 parses as a real number, and no scan
/// can tell it apart from data. Documented behavior, asserted so it stays true.
#[test]
fn denull_never_proposes_numeric_sentinels() {
    let wrk = Workdir::new("denull_never_proposes_numeric_sentinels");
    let mut rows = String::from("depth\n");
    for i in 0..40 {
        if i % 4 == 0 {
            rows.push_str("-999\n");
        } else {
            rows.push_str(&format!("{}\n", i + 1));
        }
    }
    wrk.create_from_string("d.csv", &rows);

    let mut cmd = wrk.command("denull");
    cmd.arg("d.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(
        got.len(),
        1,
        "-999 is numeric; denull must stay silent, got {got:?}"
    );
}

/// --json emits a machine-readable array carrying the same verdicts.
#[test]
fn denull_json_output() {
    let wrk = Workdir::new("denull_json_output");
    let mut rows = String::from("depth\n");
    for i in 0..20 {
        if i % 5 == 0 {
            rows.push_str("NULL\n");
        } else {
            rows.push_str(&format!("{}\n", i + 1));
        }
    }
    wrk.create_from_string("d.csv", &rows);

    let mut cmd = wrk.command("denull");
    cmd.args(["d.csv", "--json"]);
    let out: String = wrk.stdout(&mut cmd);
    let v: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert_eq!(v[0]["field"], "depth");
    assert_eq!(v[0]["verdict"], "confirmed");
    assert_eq!(v[0]["sentinels"], "NULL");
    assert_eq!(v[0]["promotes_to"], "Integer");
}

#[test]
fn denull_rejected_column_names_every_sentinel_it_holds() {
    // A rejected column may hold MORE than one sentinel token. The `sentinels` column
    // reports what was observed, so it must name all of them - reporting only the first
    // one seen understates the evidence and hides a token the user has to handle.
    let wrk = Workdir::new("denull_rejected_column_names_every_sentinel_it_holds");
    let mut rows = String::from("status\n");
    for i in 0..40 {
        rows.push_str(match i % 5 {
            0 => "NULL\n",
            1 => "N/A\n",
            2 => "OK\n",
            3 => "1\n",
            _ => "2\n",
        });
    }
    wrk.create_from_string("d.csv", &rows);

    let mut cmd = wrk.command("denull");
    cmd.arg("d.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got[1][1], "rejected:off-vocab");
    assert_eq!(got[1][2], "N/A,NULL");
}

#[test]
fn denull_overflowed_column_still_names_a_sentinel_met_after_overflow() {
    // A numeric column buried under more than --max-distinct junk tokens can never be
    // promoted, but it is still worth reporting IF it holds a known sentinel. The
    // sentinel here arrives only AFTER the offender map has overflowed and stopped
    // growing, so it can only be reported because sentinels are tracked independently
    // of that map.
    let wrk = Workdir::new("denull_overflowed_column_still_names_a_sentinel_met_after_overflow");
    let mut rows = String::from("depth\n1\n2\n");
    for i in 0..20 {
        rows.push_str(&format!("junk{i}\n"));
    }
    rows.push_str("NULL\n");
    wrk.create_from_string("d.csv", &rows);

    let mut cmd = wrk.command("denull");
    cmd.arg("d.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got[1][1], "rejected:too-many-distinct");
    assert_eq!(got[1][2], "NULL");
    assert!(
        got[1][6].is_empty(),
        "an overflowed column promotes to nothing"
    );
}

#[test]
fn denull_names_a_sentinel_that_lands_on_the_overflow_boundary() {
    // The cell that tips `offenders` over --max-distinct is never inserted into the map.
    // If that exact cell is the sentinel, it must still be recorded, or the column
    // reports "rejected" while naming no sentinel at all.
    let wrk = Workdir::new("denull_names_a_sentinel_that_lands_on_the_overflow_boundary");
    let mut rows = String::from("depth\n1\n2\n");
    for i in 0..4 {
        rows.push_str(&format!("junk{i}\n"));
    }
    // --max-distinct 4 => junk0..junk3 fill the map; this NULL is the overflowing cell.
    rows.push_str("NULL\n");
    wrk.create_from_string("d.csv", &rows);

    let mut cmd = wrk.command("denull");
    cmd.args(["--max-distinct", "4"]).arg("d.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got[1][1], "rejected:too-many-distinct");
    assert_eq!(got[1][2], "NULL");
}

#[test]
fn denull_names_sentinels_seen_before_and_after_overflow() {
    // Distinct sentinel tokens can straddle the overflow point: "NULL" lands in the
    // offender map before it fills, "N/A" arrives only after it has stopped growing.
    // Both are evidence and both must be named.
    let wrk = Workdir::new("denull_names_sentinels_seen_before_and_after_overflow");
    let mut rows = String::from("depth\n1\n2\nNULL\n");
    for i in 0..17 {
        rows.push_str(&format!("junk{i}\n"));
    }
    rows.push_str("N/A\n");
    wrk.create_from_string("d.csv", &rows);

    let mut cmd = wrk.command("denull");
    cmd.arg("d.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got[1][1], "rejected:too-many-distinct");
    assert_eq!(got[1][2], "N/A,NULL");
}

#[test]
fn denull_rejected_column_names_more_than_eight_distinct_sentinels() {
    // Nine distinct VOCABULARY tokens, not casing variants. A rejected column must still
    // name all of them: the report cap must not silently drop evidence.
    let wrk = Workdir::new("denull_rejected_column_names_more_than_eight_distinct_sentinels");
    let mut rows = String::from("depth\n1\n2\nOK\n");
    for t in ["NULL", "N/A", "NA", "nil", "none", "-", "--", "?", "??"] {
        rows.push_str(t);
        rows.push('\n');
    }
    wrk.create_from_string("d.csv", &rows);

    let mut cmd = wrk.command("denull");
    cmd.arg("d.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    assert_eq!(got[1][1], "rejected:off-vocab");
    assert_eq!(got[1][2], "-,--,?,??,N/A,NA,NULL,nil,none");
}
