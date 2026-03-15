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
