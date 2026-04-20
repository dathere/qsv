//! DuckDB discovery, SQL escaping, query execution, SQL scoring + refinement,
//! and SQL-error recovery for describegpt.
//!
//! The LLM produces SQL in answer to a user question; everything else in this
//! module is the deterministic plumbing around running it and scoring it.

use std::{
    env, fs,
    path::Path,
    process::Command,
    time::{Duration, Instant},
};

use super::{
    Args, CacheType, DUCKDB_PATH, PromptType, QSV_DUCKDB_PATH_ENV, invalidate_cache_entry,
};
use crate::{
    CliError, CliResult,
    util::{self, print_status},
};

/// Escape a string for safe usage as a SQL **string literal** (i.e. inside
/// `'…'`). Do not use for identifiers — those need a separate escape that
/// doubles `"` inside `"…"`.
///
/// - Single quotes are escaped by doubling (`'` → `''`), per the SQL standard.
/// - Backslashes are doubled (`\` → `\\`) — non-standard but matches DuckDB and prevents C-style
///   escape injection. Must be applied first.
/// - Newline, CR, and null byte are replaced with their `\n` / `\r` / `\0` escape-sequence
///   spellings.
pub(super) fn escape_sql_string(s: &str) -> String {
    s.replace('\\', "\\\\") // Backslash must be first!
        .replace('\'', "''")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\0', "\\0")
}

/// Extract the header row plus the first 10 data rows from a CSV file.
/// Used to show the LLM a tiny sample of its own SQL query's result.
pub(super) fn extract_sql_sample(csv_path: &Path) -> CliResult<String> {
    use std::io::{BufRead, BufReader};

    let file = fs::File::open(csv_path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut result = String::new();

    if let Some(Ok(header)) = lines.next() {
        result.push_str(&header);
        result.push('\n');
    }

    for _ in 0..10 {
        if let Some(Ok(line)) = lines.next() {
            result.push_str(&line);
            result.push('\n');
        } else {
            break;
        }
    }

    Ok(result.trim().to_string())
}

/// Whether DuckDB should be used — true iff `QSV_DUCKDB_PATH` is set and
/// non-empty. (Existence / executable checks happen later in `get_duckdb_path`.)
pub(super) fn should_use_duckdb() -> bool {
    env::var(QSV_DUCKDB_PATH_ENV).is_ok_and(|val| !val.is_empty())
}

/// Resolve the DuckDB binary path from `QSV_DUCKDB_PATH`, verify it, and cache
/// the result in a process-wide `OnceLock`.
pub(super) fn get_duckdb_path() -> CliResult<String> {
    if let Some(path) = DUCKDB_PATH.get() {
        return Ok(path.clone());
    }

    let duckdb_path =
        env::var(QSV_DUCKDB_PATH_ENV).map_err(|_| "QSV_DUCKDB_PATH env var not set")?;

    let path = Path::new(&duckdb_path);
    if !path.exists() {
        return fail_clierror!("DuckDB binary not found at path: {duckdb_path}");
    }
    if !path.is_file() {
        return fail_clierror!("DuckDB path is not a file: {duckdb_path}");
    }
    if !util::is_executable(&duckdb_path)? {
        return fail_clierror!("DuckDB path is not executable: {duckdb_path}");
    }

    // Safety: this is the first and only set of DUCKDB_PATH.
    DUCKDB_PATH.set(duckdb_path.clone()).unwrap();

    Ok(duckdb_path)
}

/// Run a SQL query through the DuckDB CLI and return `(stdout, stderr)`.
///
/// On non-zero exit, writes the failed query to `output_path.sql` for
/// debugging and returns an error. On success, if `output_path` is non-empty,
/// writes stdout to `output_path.csv`. Also fails if DuckDB prints `" error:"`
/// to stderr even with a 0 exit — a belt-and-suspenders check.
pub(super) fn run_duckdb_query(
    sql_query: &str,
    output_path: &str,
    status_msg: &str,
) -> CliResult<(String, String)> {
    let duckdb_path = get_duckdb_path()?;
    let start_time = Instant::now();

    let mut cmd = Command::new(duckdb_path);
    cmd.arg("-csv").arg("-c").arg(sql_query);

    let output = cmd
        .output()
        .map_err(|e| CliError::Other(format!("Error while executing DuckDB command: {e:?}")))?;

    if !status_msg.is_empty() {
        print_status(status_msg, Some(start_time.elapsed()));
    }
    // Silence unused-warning if `Duration` import ends up redundant across build configs.
    let _ = Duration::from_secs(0);

    if !output.status.success() {
        // SQL execution failed — preserve the failing query as a .sql sibling for debugging.
        let output_path = Path::new(output_path).with_extension("sql");
        if let Err(e) = fs::write(&output_path, sql_query) {
            return fail_clierror!("Failed to write SQL query to {output_path:?}: {e}");
        }
        let stderr_str =
            simdutf8::basic::from_utf8(&output.stderr).unwrap_or("<unable to parse stderr>");
        return fail_clierror!(
            "DuckDB SQL query execution failed:\n{stderr_str}\nFailed SQL query saved to \
             {output_path:?}"
        );
    }

    let Ok(stdout_str) = simdutf8::basic::from_utf8(&output.stdout) else {
        return fail_clierror!("Unable to parse stdout of DuckDB command:\n{output:?}");
    };
    let Ok(stderr_str) = simdutf8::basic::from_utf8(&output.stderr) else {
        return fail_clierror!("Unable to parse stderr of DuckDB command:\n{output:?}");
    };

    // Defensive: some DuckDB builds may print error text even with exit 0.
    if stderr_str.to_ascii_lowercase().contains(" error:") {
        return fail_clierror!("DuckDB SQL query error detected:\n{stderr_str}");
    }

    if !output_path.is_empty() {
        let output_path = Path::new(output_path).with_extension("csv");
        if let Err(e) = fs::write(&output_path, stdout_str) {
            return fail_clierror!("Failed to write SQL results to {output_path:?}: {e}");
        }
    }

    Ok((stdout_str.to_string(), stderr_str.to_string()))
}

/// Invoke `qsv scoresql` on a SQL query, returning `(score, rating, raw_report_json)`.
pub(super) fn score_sql_query(
    input_path: &str,
    sql_query: &str,
    use_duckdb: bool,
) -> CliResult<(u32, String, String)> {
    let qsv_path = std::env::current_exe()?;
    let mut cmd = Command::new(qsv_path);
    cmd.arg("scoresql").arg(input_path).arg("--json").arg("-q");
    if use_duckdb {
        cmd.arg("--duckdb");
    }
    cmd.arg(sql_query);

    let output = cmd
        .output()
        .map_err(|e| format!("Failed to run scoresql: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return fail_clierror!("scoresql failed: {stderr}");
    }

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let report: serde_json::Value =
        serde_json::from_str(&stdout).map_err(|e| format!("Failed to parse scoresql JSON: {e}"))?;

    let score = report["score"].as_u64().unwrap_or(0) as u32;
    let rating = report["rating"].as_str().unwrap_or("Unknown").to_string();

    Ok((score, rating, stdout))
}

/// Build a refinement prompt for the LLM to improve a low-scoring SQL query,
/// including the scoresql suggestions and attempt counter.
pub(super) fn build_score_refinement_prompt(
    sql_query: &str,
    score_report_json: &str,
    attempt: u32,
    max_retries: u32,
    table_name: &str,
) -> String {
    let report: serde_json::Value = serde_json::from_str(score_report_json).unwrap_or_default();
    let score = report["score"].as_u64().unwrap_or(0);
    let rating = report["rating"].as_str().unwrap_or("Unknown");
    let suggestions: Vec<&str> = report["suggestions"]
        .as_array()
        .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
        .unwrap_or_default();
    let suggestions_text = suggestions
        .iter()
        .enumerate()
        .map(|(i, s)| format!("{}. {s}", i + 1))
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "The SQL query scored {score}/100 ({rating}), below the acceptable threshold. Retry \
         {attempt} of {max_retries}.\n\nCurrent \
         SQL:\n```sql\n{sql_query}\n```\n\nSuggestions:\n{suggestions_text}\n\nImprove the SQL \
         query addressing these suggestions. Use `{table_name}` as the table name. Return ONLY \
         the improved SQL in a ```sql code block. Keep the same logic/results, optimize structure \
         and performance."
    )
}

/// Recover from a SQL execution failure: invalidate the `Prompt` cache entry
/// (so a retry with `--fresh` isn't needed), persist the failing query for
/// inspection, then return an error to the caller.
#[allow(dead_code)]
pub(super) fn handle_sql_error(
    args: &Args,
    cache_type: &CacheType,
    sql_query_file: &Path,
    sql_results_path: &Path,
    error_msg: &str,
) -> CliResult<()> {
    if cache_type != &CacheType::Fresh && cache_type != &CacheType::None {
        let _ = invalidate_cache_entry(args, PromptType::Prompt);
    }
    let output_path = Path::new(sql_results_path).with_extension("sql");
    if let Err(e) = fs::copy(sql_query_file, &output_path) {
        return fail_clierror!("Failed to copy SQL query to {sql_results_path:?}: {e}");
    }
    fail_clierror!("{error_msg}")
}
