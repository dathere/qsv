use crate::workdir::Workdir;

fn setup(name: &str) -> Workdir {
    let data = vec![
        svec!["id", "name", "age", "status", "score"],
        svec!["1", "Alice", "30", "active", "85"],
        svec!["2", "Bob", "25", "inactive", "92"],
        svec!["3", "Carol", "35", "active", "78"],
        svec!["4", "Dave", "28", "active", "95"],
        svec!["5", "Eve", "32", "active", "88"],
        svec!["6", "Frank", "27", "inactive", "76"],
        svec!["7", "Grace", "31", "active", "91"],
        svec!["8", "Heidi", "29", "active", "83"],
        svec!["9", "Ivan", "33", "inactive", "79"],
        svec!["10", "Judy", "26", "active", "94"],
    ];

    let wrk = Workdir::new(name);
    wrk.create("data.csv", data);
    wrk
}

fn setup_join(name: &str) -> Workdir {
    let data1 = vec![
        svec!["id", "name", "dept_id"],
        svec!["1", "Alice", "10"],
        svec!["2", "Bob", "20"],
        svec!["3", "Carol", "10"],
        svec!["4", "Dave", "30"],
        svec!["5", "Eve", "20"],
    ];
    let data2 = vec![
        svec!["dept_id", "dept_name"],
        svec!["10", "Engineering"],
        svec!["20", "Marketing"],
        svec!["30", "Sales"],
    ];

    let wrk = Workdir::new(name);
    wrk.create("employees.csv", data1);
    wrk.create("departments.csv", data2);
    wrk
}

#[test]
fn scoresql_basic() {
    let wrk = setup("scoresql_basic");
    let mut cmd = wrk.command("scoresql");
    cmd.arg("data.csv");
    cmd.arg("SELECT * FROM data WHERE age > 30");

    let got = wrk.output(&mut cmd);
    let stdout = String::from_utf8_lossy(&got.stdout);
    assert!(stdout.contains("Score:"));
    assert!(stdout.contains("/100"));
    assert!(stdout.contains("=== Query Plan ==="));
    assert!(stdout.contains("=== Scoring Breakdown ==="));
}

#[test]
fn scoresql_json_output() {
    let wrk = setup("scoresql_json_output");
    let mut cmd = wrk.command("scoresql");
    cmd.arg("--json");
    cmd.arg("data.csv");
    cmd.arg("SELECT * FROM data WHERE age > 30");

    let got = wrk.output(&mut cmd);
    let stdout = String::from_utf8_lossy(&got.stdout);
    // Should be valid JSON
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert!(parsed["score"].is_number());
    assert!(parsed["max_score"].is_number());
    assert_eq!(parsed["max_score"], 100);
    assert!(parsed["rating"].is_string());
    assert!(parsed["plan"].is_string());
    assert!(parsed["breakdown"].is_array());
    assert!(parsed["suggestions"].is_array());
    assert!(parsed["cache_status"].is_object());
}

#[test]
fn scoresql_select_star_warning() {
    let wrk = setup("scoresql_select_star_warning");
    let mut cmd = wrk.command("scoresql");
    cmd.arg("--json");
    cmd.arg("data.csv");
    cmd.arg("SELECT * FROM data");

    let got = wrk.output(&mut cmd);
    let stdout = String::from_utf8_lossy(&got.stdout);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();

    // Should have SELECT * suggestion
    let suggestions = parsed["suggestions"].as_array().unwrap();
    let has_select_star_warning = suggestions
        .iter()
        .any(|s| s.as_str().unwrap_or_default().contains("SELECT *"));
    assert!(
        has_select_star_warning,
        "Expected SELECT * warning in suggestions"
    );
}

#[test]
fn scoresql_order_by_no_limit() {
    let wrk = setup("scoresql_order_by_no_limit");
    let mut cmd = wrk.command("scoresql");
    cmd.arg("--json");
    cmd.arg("data.csv");
    cmd.arg("SELECT name, score FROM data ORDER BY score DESC");

    let got = wrk.output(&mut cmd);
    let stdout = String::from_utf8_lossy(&got.stdout);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();

    let suggestions = parsed["suggestions"].as_array().unwrap();
    let has_limit_warning = suggestions
        .iter()
        .any(|s| s.as_str().unwrap_or_default().contains("LIMIT"));
    assert!(
        has_limit_warning,
        "Expected LIMIT suggestion for ORDER BY without LIMIT"
    );
}

#[test]
fn scoresql_join_query() {
    let wrk = setup_join("scoresql_join_query");
    let mut cmd = wrk.command("scoresql");
    cmd.arg("--json");
    cmd.arg("employees.csv");
    cmd.arg("departments.csv");
    cmd.arg(
        "SELECT e.name, d.dept_name FROM employees e JOIN departments d ON e.dept_id = d.dept_id",
    );

    let got = wrk.output(&mut cmd);
    let stdout = String::from_utf8_lossy(&got.stdout);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert!(parsed["score"].as_u64().unwrap() > 0);
}

#[test]
fn scoresql_with_limit() {
    let wrk = setup("scoresql_with_limit");
    let mut cmd = wrk.command("scoresql");
    cmd.arg("--json");
    cmd.arg("data.csv");
    cmd.arg("SELECT name, score FROM data ORDER BY score DESC LIMIT 5");

    let got = wrk.output(&mut cmd);
    let stdout = String::from_utf8_lossy(&got.stdout);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();

    // With LIMIT, should not have the ORDER BY without LIMIT warning
    let suggestions = parsed["suggestions"].as_array().unwrap();
    let has_limit_warning = suggestions
        .iter()
        .any(|s| s.as_str().unwrap().contains("Add LIMIT to ORDER BY"));
    assert!(
        !has_limit_warning,
        "Should NOT have LIMIT warning when LIMIT is present"
    );
}

#[test]
fn scoresql_table_aliases() {
    let wrk = setup("scoresql_table_aliases");
    let mut cmd = wrk.command("scoresql");
    cmd.arg("data.csv");
    cmd.arg("SELECT name, score FROM _t_1 WHERE age > 25 LIMIT 5");

    let got = wrk.output(&mut cmd);
    let stdout = String::from_utf8_lossy(&got.stdout);
    assert!(stdout.contains("Score:"));
}

#[test]
fn scoresql_output_file() {
    let wrk = setup("scoresql_output_file");
    let mut cmd = wrk.command("scoresql");
    cmd.arg("--json");
    cmd.arg("-o");
    cmd.arg(wrk.path("report.json"));
    cmd.arg("data.csv");
    cmd.arg("SELECT * FROM data");

    wrk.output(&mut cmd);

    // Check the output file exists and contains valid JSON
    let output = std::fs::read_to_string(wrk.path("report.json")).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
    assert!(parsed["score"].is_number());
}

#[test]
fn scoresql_rating_categories() {
    let wrk = setup("scoresql_rating_categories");
    let mut cmd = wrk.command("scoresql");
    cmd.arg("--json");
    cmd.arg("data.csv");
    cmd.arg("SELECT name FROM data WHERE age > 30 LIMIT 10");

    let got = wrk.output(&mut cmd);
    let stdout = String::from_utf8_lossy(&got.stdout);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();

    let rating = parsed["rating"].as_str().unwrap();
    assert!(
        ["Excellent", "Good", "Fair", "Poor", "Very Poor"].contains(&rating),
        "Rating should be one of the valid categories, got: {rating}"
    );
}

#[test]
fn scoresql_breakdown_structure() {
    let wrk = setup("scoresql_breakdown_structure");
    let mut cmd = wrk.command("scoresql");
    cmd.arg("--json");
    cmd.arg("data.csv");
    cmd.arg("SELECT * FROM data");

    let got = wrk.output(&mut cmd);
    let stdout = String::from_utf8_lossy(&got.stdout);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();

    let breakdown = parsed["breakdown"].as_array().unwrap();
    assert_eq!(breakdown.len(), 5, "Should have 5 scoring categories");

    let categories: Vec<&str> = breakdown
        .iter()
        .map(|b| b["category"].as_str().unwrap())
        .collect();
    assert!(categories.contains(&"Type optimization"));
    assert!(categories.contains(&"Join cardinality"));
    assert!(categories.contains(&"Filter selectivity"));
    assert!(categories.contains(&"Data distribution"));
    assert!(categories.contains(&"Query patterns"));
}

#[test]
fn scoresql_no_where_clause() {
    let wrk = setup("scoresql_no_where_clause");
    let mut cmd = wrk.command("scoresql");
    cmd.arg("--json");
    cmd.arg("data.csv");
    cmd.arg("SELECT name FROM data LIMIT 5");

    let got = wrk.output(&mut cmd);
    let stdout = String::from_utf8_lossy(&got.stdout);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();

    // Filter selectivity should be half score for no WHERE clause
    let breakdown = parsed["breakdown"].as_array().unwrap();
    let filter = breakdown
        .iter()
        .find(|b| b["category"].as_str().unwrap() == "Filter selectivity")
        .unwrap();
    assert_eq!(
        filter["score"].as_u64().unwrap(),
        10,
        "No WHERE clause should give half the filter score"
    );
    assert!(
        filter["detail"]
            .as_str()
            .unwrap()
            .contains("no WHERE clause")
    );
}

#[test]
fn scoresql_invalid_sql() {
    let wrk = setup("scoresql_invalid_sql");
    let mut cmd = wrk.command("scoresql");
    cmd.arg("data.csv");
    cmd.arg("SELECTED * FROM data");

    let got = wrk.output(&mut cmd);
    let stderr = String::from_utf8_lossy(&got.stderr);
    assert!(
        stderr.contains("Failed to execute SQL query"),
        "Expected 'Failed to execute SQL query' in stderr, got: {stderr}"
    );
    assert!(
        stderr.contains("SELECTED * FROM data"),
        "Expected the invalid SQL in stderr, got: {stderr}"
    );
    assert!(
        stderr.contains("Hint: Check your SQL syntax"),
        "Expected syntax hint in stderr, got: {stderr}"
    );
    assert!(!got.status.success(), "Expected command to fail");
}

#[test]
fn scoresql_sql_script() {
    let wrk = setup("scoresql_sql_script");

    // Create a SQL script with comments and multiple queries.
    // Only the last query should be scored.
    let sql_script = r#"
-- This is a comment
SELECT * FROM data;
-- Another comment
SELECT name, score FROM data WHERE age > 30 LIMIT 5;
"#;
    wrk.create_from_string("script.sql", sql_script);

    let mut cmd = wrk.command("scoresql");
    cmd.arg("--json");
    cmd.arg("data.csv");
    cmd.arg(wrk.path("script.sql"));

    let got = wrk.output(&mut cmd);
    assert!(
        got.status.success(),
        "scoresql failed: {}",
        String::from_utf8_lossy(&got.stderr)
    );
    let stdout = String::from_utf8_lossy(&got.stdout);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();

    // Should produce a valid score from the last query
    assert!(parsed["score"].is_number());
    assert!(parsed["max_score"].is_number());

    // The last query has specific columns (not SELECT *), so there should be
    // no SELECT * warning
    let suggestions = parsed["suggestions"].as_array().unwrap();
    let has_select_star_warning = suggestions
        .iter()
        .any(|s| s.as_str().unwrap_or_default().contains("SELECT *"));
    assert!(
        !has_select_star_warning,
        "Last query uses specific columns, should NOT have SELECT * warning"
    );
}

// ─── Regression tests for scoresql review-fix commit ────────────────────────

/// Regression: a malformed `USING` clause where `)` precedes `(` used to panic
/// inside `extract_join_columns` (slice index out-of-order).
#[test]
fn scoresql_malformed_using_no_panic() {
    let wrk = setup("scoresql_malformed_using_no_panic");
    let mut cmd = wrk.command("scoresql");
    cmd.arg("data.csv");
    // The pathological substring "USING ) ... (" will not be syntactically valid
    // SQL, but it must not panic our parser.
    cmd.arg(
        "SELECT * FROM data WHERE name = 'a USING ) WHERE x = (1)' AND status = 'active' LIMIT 5",
    );

    // We don't care whether the score query succeeds — just that we exit
    // cleanly and don't abort with a Rust panic.
    let got = wrk.output_stderr(&mut cmd);
    assert!(
        !got.contains("panicked") && !got.contains("slice index"),
        "unexpected panic: {got}"
    );
}

/// Regression: an alias inside a string literal (`'_t_1'`) used to be
/// rewritten by the polars alias-replacement pass, corrupting the literal.
#[test]
fn scoresql_alias_inside_string_literal() {
    let wrk = setup("scoresql_alias_inside_string_literal");
    let mut cmd = wrk.command("scoresql");
    cmd.arg("--json");
    cmd.arg("data.csv");
    cmd.arg("SELECT '_t_1' AS label, name FROM _t_1 LIMIT 1");

    let got = wrk.output(&mut cmd);
    assert!(
        got.status.success(),
        "scoresql failed: {}",
        String::from_utf8_lossy(&got.stderr)
    );
    let stdout = String::from_utf8_lossy(&got.stdout);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    // We mainly want a clean exit and a parseable score; the literal should
    // not have caused a "table not found" error from polars.
    assert!(parsed["score"].is_number());
}

#[test]
fn scoresql_keyword_inside_string_literal() {
    // Use a self-contained fixture so this test doesn't depend on the shared
    // `setup()` schema (it only needs *a* string column to project out).
    let wrk = Workdir::new("scoresql_keyword_inside_string_literal");
    wrk.create(
        "data.csv",
        vec![
            svec!["id", "name"],
            svec!["1", "Alice"],
            svec!["2", "Bob"],
            svec!["3", "Carol"],
        ],
    );

    let mut cmd = wrk.command("scoresql");
    cmd.arg("--json");
    cmd.arg("data.csv");
    // The literal contains "SELECT *" and "WHERE", but the actual query has
    // explicit columns and no WHERE — the score should reflect the actual
    // query, not the contents of the string.
    cmd.arg("SELECT 'SELECT * WHERE x' AS note, name FROM data LIMIT 5");

    let got = wrk.output(&mut cmd);
    assert!(
        got.status.success(),
        "scoresql failed: {}",
        String::from_utf8_lossy(&got.stderr)
    );
    let stdout = String::from_utf8_lossy(&got.stdout);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let suggestions = parsed["suggestions"].as_array().unwrap();
    let has_select_star_warning = suggestions
        .iter()
        .any(|s| s.as_str().unwrap_or_default().contains("SELECT *"));
    assert!(
        !has_select_star_warning,
        "SELECT * appearing only inside a string literal should not trigger the warning"
    );
}

#[test]
fn scoresql_duplicate_file_stem_rejected() {
    let wrk = setup("scoresql_duplicate_file_stem_rejected");
    // Create a second `data.csv` under a subdirectory so its file stem
    // collides with the top-level one.
    std::fs::create_dir_all(wrk.path("nested")).unwrap();
    wrk.create(
        "nested/data.csv",
        vec![svec!["id", "x"], svec!["1", "a"], svec!["2", "b"]],
    );

    let mut cmd = wrk.command("scoresql");
    cmd.arg("data.csv");
    cmd.arg("nested/data.csv");
    cmd.arg("SELECT * FROM data LIMIT 1");

    // Use `output()` so we can also assert non-zero exit — a future regression
    // that downgrades duplicate-stem detection to a warning would otherwise
    // slip through if we only matched stderr text.
    let got = wrk.output(&mut cmd);
    let stderr = String::from_utf8_lossy(&got.stderr);
    assert!(
        !got.status.success(),
        "scoresql should fail on duplicate stems but succeeded; stderr: {stderr}"
    );
    assert!(
        stderr.contains("Duplicate table name"),
        "expected duplicate-stem error, got: {stderr}"
    );
}

#[test]
fn scoresql_underscore_select_not_subquery() {
    let wrk = setup("scoresql_underscore_select_not_subquery");
    let mut cmd = wrk.command("scoresql");
    cmd.arg("--json");
    cmd.arg("data.csv");
    // The token `_SELECT` inside the literal must not flip `has_subquery`,
    // and the literal-skipping mask should keep the bare WHERE clause clean.
    cmd.arg("SELECT name FROM data WHERE name = '_SELECT(x)' LIMIT 1");

    let got = wrk.output(&mut cmd);
    assert!(
        got.status.success(),
        "scoresql failed: {}",
        String::from_utf8_lossy(&got.stderr)
    );
    let stdout = String::from_utf8_lossy(&got.stdout);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let suggestions = parsed["suggestions"].as_array().unwrap();
    let has_subquery_suggestion = suggestions
        .iter()
        .any(|s| s.as_str().unwrap_or_default().contains("nested subqueries"));
    assert!(
        !has_subquery_suggestion,
        "_SELECT(...) inside a string literal must not be treated as a subquery"
    );
}

#[test]
fn scoresql_filter_on_rare_value_not_penalized() {
    // Self-contained fixture with an explicitly skewed `status` column:
    // 7 'active' rows + 3 'inactive' rows -> 70% / 30%. Filtering on the
    // rare value must NOT trigger the low-selectivity penalty.
    let wrk = Workdir::new("scoresql_filter_on_rare_value_not_penalized");
    wrk.create(
        "data.csv",
        vec![
            svec!["id", "name", "status"],
            svec!["1", "Alice", "active"],
            svec!["2", "Bob", "active"],
            svec!["3", "Carol", "active"],
            svec!["4", "Dave", "active"],
            svec!["5", "Eve", "active"],
            svec!["6", "Frank", "active"],
            svec!["7", "Grace", "active"],
            svec!["8", "Heidi", "inactive"],
            svec!["9", "Ivan", "inactive"],
            svec!["10", "Judy", "inactive"],
        ],
    );

    let mut cmd = wrk.command("scoresql");
    cmd.arg("--json");
    cmd.arg("data.csv");
    cmd.arg("SELECT name FROM data WHERE status = 'inactive'");

    let got = wrk.output(&mut cmd);
    assert!(
        got.status.success(),
        "scoresql failed: {}",
        String::from_utf8_lossy(&got.stderr)
    );
    let stdout = String::from_utf8_lossy(&got.stdout);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let suggestions = parsed["suggestions"].as_array().unwrap();
    let has_unselective_warning = suggestions.iter().any(|s| {
        let txt = s.as_str().unwrap_or_default();
        txt.contains("filter matches") && txt.contains("selective predicate")
    });
    assert!(
        !has_unselective_warning,
        "filter on a rare value should not trigger the low-selectivity penalty: {suggestions:?}"
    );
}


/// Regression: a quoted-string filter `WHERE col = 'NULL'` must NOT collapse
/// into the SQL keyword `NULL` lookup (which matches empty cells). The two
/// forms are tagged differently at extraction time so they keep distinct
/// frequency-cache lookup semantics.
#[test]
fn scoresql_quoted_null_distinct_from_keyword_null() {
    // Build a fixture where the *string* "NULL" is the dominant value but
    // there are no actual nulls. If quoted-vs-keyword tagging is broken,
    // the keyword normalization (`NULL` -> empty cell) would make the
    // quoted lookup spuriously match the empty-cell branch instead.
    let wrk = Workdir::new("scoresql_quoted_null_distinct_from_keyword_null");
    wrk.create(
        "data.csv",
        vec![
            svec!["id", "label"],
            svec!["1", "NULL"],
            svec!["2", "NULL"],
            svec!["3", "NULL"],
            svec!["4", "NULL"],
            svec!["5", "NULL"],
            svec!["6", "NULL"],
            svec!["7", "NULL"],
            svec!["8", "NULL"],
            svec!["9", "active"],
            svec!["10", "active"],
        ],
    );

    let mut cmd = wrk.command("scoresql");
    cmd.arg("--json");
    cmd.arg("data.csv");
    // Quoted-string `'NULL'` should match the literal string "NULL" in the
    // frequency cache (80% of rows) and trigger the low-selectivity warning.
    cmd.arg("SELECT id FROM data WHERE label = 'NULL'");

    let got = wrk.output(&mut cmd);
    assert!(
        got.status.success(),
        "scoresql failed: {}",
        String::from_utf8_lossy(&got.stderr)
    );
    let stdout = String::from_utf8_lossy(&got.stdout);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let suggestions = parsed["suggestions"].as_array().unwrap();
    let has_unselective_warning = suggestions.iter().any(|s| {
        let txt = s.as_str().unwrap_or_default();
        txt.contains("filter matches") && txt.contains("selective predicate")
    });
    assert!(
        has_unselective_warning,
        "filter on quoted string 'NULL' (80% of rows) should trigger low-selectivity \
         warning — got: {suggestions:?}"
    );
}
