static USAGE: &str = r#"
Analyze a SQL query against CSV file caches (stats, moarstats, frequency) to produce a
performance score with actionable optimization suggestions BEFORE running the query.

Accepts the same input/SQL arguments as sqlp. Outputs a human-readable performance report
(default) or JSON (--json). Supports Polars mode (default) and DuckDB mode (--duckdb).

Scoring factors include:
  * Query plan analysis (EXPLAIN output from Polars or DuckDB)
  * Type optimization (column types vs. usage in query)
  * Join key cardinality and data distribution
  * Filter selectivity from frequency cache
  * Query anti-pattern detection (SELECT *, missing LIMIT, cartesian joins, etc.)
  * Infrastructure checks (index files, cache freshness)

Caches are auto-generated when missing:
  * stats cache via `qsv stats --everything --stats-jsonl`
  * frequency cache via `qsv frequency --frequency-jsonl`

Examples:

  $ qsv scoresql data.csv "SELECT * FROM data WHERE col1 > 10"

  $ qsv scoresql --json data.csv "SELECT col1, col2 FROM data ORDER BY col1"

  $ qsv scoresql data.csv data2.csv "SELECT * FROM data JOIN data2 ON data.id = data2.id"

  $ qsv scoresql --duckdb data.csv "SELECT * FROM data WHERE status = 'active'"

  # use _t_N aliases just like sqlp (see sqlp documentation)
  $ qsv scoresql data.csv data2.csv "SELECT * FROM _t_1 JOIN _t_2 ON _t_1.id = _t_2.id"

For more examples, see https://github.com/dathere/qsv/blob/master/tests/test_scoresql.rs.

Usage:
    qsv scoresql [options] <input>... <sql>
    qsv scoresql --help

scoresql arguments:
    input                     The CSV file/s to analyze. Use '-' for standard input.
                              If input is a directory, all files in the directory will
                              be read as input.
                              If the input is a file with a '.infile-list' extension,
                              the file will be read as a list of input files.

    sql                       The SQL query to score/analyze.

scoresql options:
    --json                    Output results as JSON instead of human-readable report.
    --duckdb                  Use DuckDB for query plan analysis instead of Polars.
                              Requires the QSV_DESCRIBEGPT_DB_ENGINE environment variable
                              to be set to the path of the DuckDB binary.
    --try-parsedates          Automatically try to parse dates/datetimes and time.
    --infer-len <arg>         Number of rows to scan when inferring schema.
                              [default: 10000]
    --ignore-errors           Ignore errors when parsing CSVs.
    --truncate-ragged-lines   Truncate lines with more fields than the header.

Common options:
    -h, --help                Display this message
    -o, --output <file>       Write output to <file> instead of stdout.
    -d, --delimiter <arg>     The field delimiter for reading CSV data.
                              Must be a single character. [default: ,]
    -q, --quiet               Do not print informational messages to stderr.
"#;

use std::{
    borrow::Cow,
    env,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
    sync::OnceLock,
};

use foldhash::{HashMap, HashMapExt};
use polars::{prelude::*, sql::SQLContext};
use serde::{Deserialize, Serialize};

use crate::{CliResult, cmd::joinp::tsvssv_delim, config::Delimiter, util, util::process_input};

#[derive(Deserialize, Clone)]
struct Args {
    arg_input:                  Vec<PathBuf>,
    arg_sql:                    String,
    flag_json:                  bool,
    flag_duckdb:                bool,
    flag_try_parsedates:        bool,
    flag_infer_len:             usize,
    flag_ignore_errors:         bool,
    flag_truncate_ragged_lines: bool,
    flag_output:                Option<String>,
    flag_delimiter:             Option<Delimiter>,
    flag_quiet:                 bool,
}

// ── Scoring constants ──────────────────────────────────────────────────────

const MAX_SCORE: u32 = 100;
const WEIGHT_TYPE_OPT: u32 = 20;
const WEIGHT_JOIN_CARD: u32 = 20;
const WEIGHT_FILTER_SEL: u32 = 20;
const WEIGHT_DATA_DIST: u32 = 20;
const WEIGHT_PATTERNS: u32 = 20;

static DUCKDB_PATH: OnceLock<String> = OnceLock::new();

// ── Output structs ─────────────────────────────────────────────────────────

#[derive(Serialize)]
struct ScoreReport {
    score:        u32,
    max_score:    u32,
    rating:       String,
    plan:         String,
    breakdown:    Vec<ScoreBreakdown>,
    suggestions:  Vec<String>,
    cache_status: CacheStatus,
}

#[derive(Serialize)]
struct ScoreBreakdown {
    category: String,
    score:    u32,
    max:      u32,
    detail:   String,
}

#[derive(Serialize)]
struct CacheStatus {
    stats_available:     Vec<String>,
    stats_missing:       Vec<String>,
    frequency_available: Vec<String>,
    frequency_missing:   Vec<String>,
    index_available:     Vec<String>,
    index_missing:       Vec<String>,
}

// ── Parsed SQL info ────────────────────────────────────────────────────────

struct SqlInfo {
    has_select_star: bool,
    has_order_by:    bool,
    has_limit:       bool,
    has_join:        bool,
    has_where:       bool,
    has_subquery:    bool,
    where_columns:   Vec<String>,
    join_columns:    Vec<String>,
    order_columns:   Vec<String>,
}

// ── Cache data per input file ──────────────────────────────────────────────

struct InputCacheData {
    file_path:      PathBuf,
    table_name:     String,
    stats:          Vec<crate::cmd::stats::StatsData>,
    has_freq_cache: bool,
    has_index:      bool,
    freq_entries:   Vec<FreqEntry>,
}

#[derive(Clone)]
#[allow(dead_code)]
struct FreqEntry {
    field:       String,
    cardinality: u64,
    frequencies: Vec<FreqValue>,
}

#[derive(Clone)]
#[allow(dead_code)]
struct FreqValue {
    value:      String,
    count:      u64,
    percentage: f64,
}

// ════════════════════════════════════════════════════════════════════════════
// run()
// ════════════════════════════════════════════════════════════════════════════

pub fn run(argv: &[&str]) -> CliResult<()> {
    let mut args: Args = util::get_args(USAGE, argv)?;

    let tmpdir = tempfile::tempdir()?;
    args.arg_input = process_input(args.arg_input, &tmpdir, "")?;

    if args.arg_input.is_empty() {
        return fail_incorrectusage_clierror!("No input files provided.");
    }

    let delim = if let Some(delimiter) = args.flag_delimiter {
        delimiter.as_byte()
    } else if let Ok(delim) = env::var("QSV_DEFAULT_DELIMITER") {
        Delimiter::decode_delimiter(&delim)?.as_byte()
    } else {
        b','
    };

    // ── 1. Resolve table names & aliases ───────────────────────────────────
    let mut table_aliases: HashMap<String, String> = HashMap::with_capacity(args.arg_input.len());
    let mut lossy_table_name = Cow::default();
    let mut table_name;
    let mut table_names: Vec<String> = Vec::with_capacity(args.arg_input.len());

    for (idx, table) in args.arg_input.iter().enumerate() {
        table_name = Path::new(table)
            .file_stem()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap_or_else(|| {
                lossy_table_name = table.to_string_lossy();
                &lossy_table_name
            });
        table_aliases.insert(table_name.to_string(), format!("_t_{}", idx + 1));
        table_names.push(table_name.to_string());
    }

    // ── 2. Parse SQL for patterns ──────────────────────────────────────────
    let sql_info = parse_sql(&args.arg_sql);

    // ── 3. Load / generate caches ──────────────────────────────────────────
    let mut input_caches: Vec<InputCacheData> = Vec::with_capacity(args.arg_input.len());

    for (idx, input_path) in args.arg_input.iter().enumerate() {
        let tname = &table_names[idx];
        let cache = load_or_generate_caches(input_path, tname, delim, args.flag_quiet)?;
        input_caches.push(cache);
    }

    // ── 4. Get query plan ──────────────────────────────────────────────────
    let plan = if args.flag_duckdb {
        get_duckdb_plan(&args, &table_names)?
    } else {
        get_polars_plan(&args, &table_aliases, delim)?
    };

    // ── 5. Scoring ─────────────────────────────────────────────────────────
    let mut suggestions: Vec<String> = Vec::new();

    let type_score = score_type_optimization(&sql_info, &input_caches, &mut suggestions);
    let join_score = score_join_cardinality(&sql_info, &input_caches, &mut suggestions);
    let filter_score = score_filter_selectivity(&sql_info, &input_caches, &mut suggestions);
    let dist_score = score_data_distribution(&sql_info, &input_caches, &mut suggestions);
    let pattern_score = score_query_patterns(&sql_info, &plan, &mut suggestions);

    // Infrastructure suggestions
    for cache in &input_caches {
        if !cache.has_index {
            suggestions.push(format!(
                "Run `qsv index {}` to create an index for faster access",
                cache.file_path.display()
            ));
        }
    }

    let total_score = type_score.0 + join_score.0 + filter_score.0 + dist_score.0 + pattern_score.0;

    let rating = match total_score {
        90..=100 => "Excellent",
        75..=89 => "Good",
        50..=74 => "Fair",
        25..=49 => "Poor",
        _ => "Very Poor",
    }
    .to_string();

    // ── 6. Build cache status ──────────────────────────────────────────────
    let cache_status = CacheStatus {
        stats_available:     input_caches
            .iter()
            .filter(|c| !c.stats.is_empty())
            .map(|c| c.table_name.clone())
            .collect(),
        stats_missing:       input_caches
            .iter()
            .filter(|c| c.stats.is_empty())
            .map(|c| c.table_name.clone())
            .collect(),
        frequency_available: input_caches
            .iter()
            .filter(|c| c.has_freq_cache)
            .map(|c| c.table_name.clone())
            .collect(),
        frequency_missing:   input_caches
            .iter()
            .filter(|c| !c.has_freq_cache)
            .map(|c| c.table_name.clone())
            .collect(),
        index_available:     input_caches
            .iter()
            .filter(|c| c.has_index)
            .map(|c| c.table_name.clone())
            .collect(),
        index_missing:       input_caches
            .iter()
            .filter(|c| !c.has_index)
            .map(|c| c.table_name.clone())
            .collect(),
    };

    let breakdown = vec![
        ScoreBreakdown {
            category: "Type optimization".to_string(),
            score:    type_score.0,
            max:      WEIGHT_TYPE_OPT,
            detail:   type_score.1,
        },
        ScoreBreakdown {
            category: "Join cardinality".to_string(),
            score:    join_score.0,
            max:      WEIGHT_JOIN_CARD,
            detail:   join_score.1,
        },
        ScoreBreakdown {
            category: "Filter selectivity".to_string(),
            score:    filter_score.0,
            max:      WEIGHT_FILTER_SEL,
            detail:   filter_score.1,
        },
        ScoreBreakdown {
            category: "Data distribution".to_string(),
            score:    dist_score.0,
            max:      WEIGHT_DATA_DIST,
            detail:   dist_score.1,
        },
        ScoreBreakdown {
            category: "Query patterns".to_string(),
            score:    pattern_score.0,
            max:      WEIGHT_PATTERNS,
            detail:   pattern_score.1,
        },
    ];

    let report = ScoreReport {
        score: total_score,
        max_score: MAX_SCORE,
        rating,
        plan,
        breakdown,
        suggestions,
        cache_status,
    };

    // ── 7. Output ──────────────────────────────────────────────────────────
    let output = if args.flag_json {
        serde_json::to_string_pretty(&report)?
    } else {
        format_human_report(&report)
    };

    if let Some(output_path) = &args.flag_output {
        let mut file = std::fs::File::create(output_path)?;
        file.write_all(output.as_bytes())?;
    } else {
        woutinfo!("{output}");
    }

    Ok(())
}

// ════════════════════════════════════════════════════════════════════════════
// SQL Parsing (lightweight, pattern-based)
// ════════════════════════════════════════════════════════════════════════════

fn parse_sql(sql: &str) -> SqlInfo {
    let upper = sql.to_ascii_uppercase();
    let tokens: Vec<&str> = upper.split_whitespace().collect();

    let has_select_star = tokens.windows(2).any(|w| w[0] == "SELECT" && w[1] == "*");

    let has_order_by = tokens.windows(2).any(|w| w[0] == "ORDER" && w[1] == "BY");

    let has_limit = tokens.contains(&"LIMIT");

    let has_join = tokens.contains(&"JOIN");
    let has_where = tokens.contains(&"WHERE");

    // Detect subqueries by looking for SELECT inside parentheses, which avoids
    // false positives from "SELECT" appearing in string literals or comments.
    let has_subquery = {
        let mut depth = 0i32;
        let mut found = false;
        let bytes = upper.as_bytes();
        let select_bytes = b"SELECT";
        for (i, &b) in bytes.iter().enumerate() {
            if b == b'(' {
                depth += 1;
            } else if b == b')' {
                depth -= 1;
            } else if depth > 0
                && b == b'S'
                && bytes.get(i..i + select_bytes.len()) == Some(select_bytes)
            {
                found = true;
                break;
            }
        }
        found
    };

    // Extract columns from WHERE clause (simplified)
    let where_columns = extract_columns_after_keyword(sql, "WHERE");
    let join_columns = extract_join_columns(sql);
    let order_columns = extract_columns_after_keyword(sql, "ORDER BY");

    SqlInfo {
        has_select_star,
        has_order_by,
        has_limit,
        has_join,
        has_where,
        has_subquery,
        where_columns,
        join_columns,
        order_columns,
    }
}

fn extract_columns_after_keyword(sql: &str, keyword: &str) -> Vec<String> {
    let upper = sql.to_ascii_uppercase();
    let Some(pos) = upper.find(keyword) else {
        return Vec::new();
    };
    let after = &sql[pos + keyword.len()..];
    // Take tokens until next SQL keyword
    let stop_keywords = [
        "SELECT",
        "FROM",
        "WHERE",
        "ORDER",
        "GROUP",
        "HAVING",
        "LIMIT",
        "JOIN",
        "ON",
        "UNION",
        "EXCEPT",
        "INTERSECT",
    ];

    let mut columns = Vec::new();
    for token in after.split_whitespace() {
        let clean = token.trim_matches(|c: char| !c.is_alphanumeric() && c != '_' && c != '.');
        let upper_clean = clean.to_ascii_uppercase();
        if stop_keywords.contains(&upper_clean.as_str()) {
            break;
        }
        if !clean.is_empty()
            && ![
                "AND", "OR", "NOT", "BY", "ASC", "DESC", "IS", "NULL", "IN", "BETWEEN", "LIKE",
            ]
            .contains(&upper_clean.as_str())
            && !clean.starts_with('\'')
            && !clean.starts_with('"')
            && clean.parse::<f64>().is_err()
        {
            // Strip table prefix (e.g. "data.col1" -> "col1")
            let col = if let Some(dot_pos) = clean.rfind('.') {
                &clean[dot_pos + 1..]
            } else {
                clean
            };
            if !col.is_empty() {
                columns.push(col.to_string());
            }
        }
    }
    columns
}

fn extract_join_columns(sql: &str) -> Vec<String> {
    let upper = sql.to_ascii_uppercase();
    let mut columns = Vec::new();

    // Look for ON clause
    for (i, _) in upper.match_indices(" ON ") {
        let after = &sql[i + 4..];
        for token in after.split_whitespace() {
            let clean = token.trim_matches(|c: char| !c.is_alphanumeric() && c != '_' && c != '.');
            let upper_clean = clean.to_ascii_uppercase();
            if ["WHERE", "ORDER", "GROUP", "HAVING", "LIMIT", "JOIN"]
                .contains(&upper_clean.as_str())
            {
                break;
            }
            if clean.contains('.') {
                if let Some(col) = clean.split('.').last() {
                    if !col.is_empty() && !["AND", "OR", "="].contains(&upper_clean.as_str()) {
                        columns.push(col.to_string());
                    }
                }
            }
        }
    }

    // Look for USING clause
    for (i, _) in upper.match_indices("USING") {
        let after = &sql[i + 5..];
        if let Some(paren_start) = after.find('(') {
            if let Some(paren_end) = after.find(')') {
                let cols = &after[paren_start + 1..paren_end];
                for col in cols.split(',') {
                    let col = col.trim();
                    if !col.is_empty() {
                        columns.push(col.to_string());
                    }
                }
            }
        }
    }

    columns
}

// ════════════════════════════════════════════════════════════════════════════
// Cache Loading / Generation
// ════════════════════════════════════════════════════════════════════════════

fn load_or_generate_caches(
    input_path: &Path,
    table_name: &str,
    _delim: u8,
    quiet: bool,
) -> CliResult<InputCacheData> {
    let canonical = input_path.canonicalize()?;

    // Check for stats cache
    let stats_path = canonical.with_extension("stats.csv.data.jsonl");
    let stats = if stats_path.exists() && is_cache_fresh(&stats_path, &canonical) {
        load_stats_cache(&stats_path)?
    } else {
        if !quiet {
            winfo!("Generating stats cache for {table_name}...");
        }
        generate_stats_cache(input_path)?;
        if stats_path.exists() {
            load_stats_cache(&stats_path)?
        } else {
            Vec::new()
        }
    };

    // Check for frequency cache
    let freq_path = canonical.with_extension("freq.csv.data.jsonl");
    let (has_freq_cache, freq_entries) =
        if freq_path.exists() && is_cache_fresh(&freq_path, &canonical) {
            (true, load_freq_cache(&freq_path)?)
        } else {
            if !quiet {
                winfo!("Generating frequency cache for {table_name}...");
            }
            generate_freq_cache(input_path)?;
            if freq_path.exists() {
                (true, load_freq_cache(&freq_path)?)
            } else {
                (false, Vec::new())
            }
        };

    // Check for index
    let idx_path = canonical.with_extension("csv.idx");
    let has_index = idx_path.exists();

    Ok(InputCacheData {
        file_path: input_path.to_path_buf(),
        table_name: table_name.to_string(),
        stats,
        has_freq_cache,
        has_index,
        freq_entries,
    })
}

fn is_cache_fresh(cache_path: &Path, input_path: &Path) -> bool {
    let Ok(cache_meta) = cache_path.metadata() else {
        return false;
    };
    let Ok(input_meta) = input_path.metadata() else {
        return false;
    };
    let Ok(cache_mtime) = cache_meta.modified() else {
        return false;
    };
    let Ok(input_mtime) = input_meta.modified() else {
        return false;
    };
    cache_mtime > input_mtime
}

fn load_stats_cache(stats_path: &Path) -> CliResult<Vec<crate::cmd::stats::StatsData>> {
    let content = std::fs::read_to_string(stats_path)?;
    let mut stats = Vec::new();
    for (i, line) in content.lines().enumerate() {
        if line.is_empty() {
            continue;
        }
        if i == 0 {
            // First line is metadata, skip
            continue;
        }
        if let Ok(sd) = serde_json::from_str::<crate::cmd::stats::StatsData>(line) {
            stats.push(sd);
        }
    }
    Ok(stats)
}

fn load_freq_cache(freq_path: &Path) -> CliResult<Vec<FreqEntry>> {
    let content = std::fs::read_to_string(freq_path)?;
    let mut entries = Vec::new();
    for (i, line) in content.lines().enumerate() {
        if line.is_empty() {
            continue;
        }
        if i == 0 {
            // First line is metadata, skip
            continue;
        }
        // Deserialize using serde_json into a generic value
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(line) {
            let field = val["field"].as_str().unwrap_or("").to_string();
            let cardinality = val["cardinality"].as_u64().unwrap_or(0);
            let frequencies = if let Some(freqs) = val["frequencies"].as_array() {
                freqs
                    .iter()
                    .map(|f| FreqValue {
                        value:      f["value"].as_str().unwrap_or("").to_string(),
                        count:      f["count"].as_u64().unwrap_or(0),
                        percentage: f["percentage"].as_f64().unwrap_or(0.0),
                    })
                    .collect()
            } else {
                Vec::new()
            };
            entries.push(FreqEntry {
                field,
                cardinality,
                frequencies,
            });
        }
    }
    Ok(entries)
}

fn generate_stats_cache(input_path: &Path) -> CliResult<()> {
    let qsv_bin = env::current_exe()?;
    let output = Command::new(&qsv_bin)
        .args(["stats", "--everything", "--stats-jsonl"])
        .arg(input_path)
        .output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::warn!("Stats cache generation failed: {stderr}");
        wwarn!(
            "Stats cache generation failed for {p} - scoring may be inaccurate: {stderr}",
            p = input_path.display()
        );
    }
    Ok(())
}

fn generate_freq_cache(input_path: &Path) -> CliResult<()> {
    let qsv_bin = env::current_exe()?;
    let output = Command::new(&qsv_bin)
        .args(["frequency", "--frequency-jsonl"])
        .arg(input_path)
        .output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::warn!("Frequency cache generation failed: {stderr}");
        wwarn!(
            "Frequency cache generation failed for {p} - scoring may be inaccurate: {stderr}",
            p = input_path.display()
        );
    }
    Ok(())
}

// ════════════════════════════════════════════════════════════════════════════
// Query Plan Generation
// ════════════════════════════════════════════════════════════════════════════

fn get_polars_plan(
    args: &Args,
    table_aliases: &HashMap<String, String>,
    delim: u8,
) -> CliResult<String> {
    let mut ctx = SQLContext::new();

    let optflags = OptFlags::PROJECTION_PUSHDOWN
        | OptFlags::PREDICATE_PUSHDOWN
        | OptFlags::CLUSTER_WITH_COLUMNS
        | OptFlags::TYPE_COERCION
        | OptFlags::SIMPLIFY_EXPR
        | OptFlags::SLICE_PUSHDOWN
        | OptFlags::COMM_SUBPLAN_ELIM
        | OptFlags::COMM_SUBEXPR_ELIM
        | OptFlags::ROW_ESTIMATE
        | OptFlags::FAST_PROJECTION;

    let mut lossy_table_name = Cow::default();
    let mut table_name_str;

    for table in &args.arg_input {
        table_name_str = Path::new(table)
            .file_stem()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap_or_else(|| {
                lossy_table_name = table.to_string_lossy();
                &lossy_table_name
            });

        let separator = tsvssv_delim(table, delim);
        let table_plpath = PlRefPath::new(&*table.to_string_lossy());

        let lf = LazyCsvReader::new(table_plpath)
            .with_has_header(true)
            .with_missing_is_null(true)
            .with_separator(separator)
            .with_infer_schema_length(Some(args.flag_infer_len))
            .with_try_parse_dates(args.flag_try_parsedates)
            .with_ignore_errors(args.flag_ignore_errors)
            .with_truncate_ragged_lines(args.flag_truncate_ragged_lines)
            .finish()?;

        ctx.register(table_name_str, lf.with_optimizations(optflags));
    }

    // Replace aliases in query
    let mut query = args.arg_sql.clone();
    for (tname, talias) in table_aliases {
        query = query.replace(talias, &format!(r#""{tname}""#));
    }

    let explain_query = format!("EXPLAIN {query}");
    match ctx.execute(&explain_query) {
        Ok(lf) => match lf.collect() {
            Ok(df) => Ok(format!("{df}")),
            Err(e) => Ok(format!("Plan generation error: {e}")),
        },
        Err(e) => Ok(format!("Plan generation error: {e}")),
    }
}

fn get_duckdb_plan(args: &Args, table_names: &[String]) -> CliResult<String> {
    let duckdb_path = get_duckdb_path()?;

    // Translate _t_N aliases and table names to read_csv('path').
    // Process replacements longest-first to avoid partial matches
    // (e.g., "data" matching inside "data2").
    let mut query = args.arg_sql.clone();
    let mut replacements: Vec<(String, String)> = Vec::new();
    for (idx, input) in args.arg_input.iter().enumerate() {
        let alias = format!("_t_{}", idx + 1);
        let table_name = &table_names[idx];
        let escaped = input.to_string_lossy().replace('\'', "''");
        let read_csv = format!("read_csv_auto('{escaped}')");
        replacements.push((alias, read_csv.clone()));
        replacements.push((table_name.clone(), read_csv));
    }
    // Sort by length descending so longer names are replaced first
    replacements.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
    for (from, to) in &replacements {
        query = query.replace(from, to);
    }

    let explain_query = format!("EXPLAIN {query}");
    let output = Command::new(duckdb_path)
        .arg("-csv")
        .arg("-c")
        .arg(&explain_query)
        .output()
        .map_err(|e| format!("Error executing DuckDB: {e}"))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Ok(format!("DuckDB plan error: {stderr}"))
    }
}

fn get_duckdb_path() -> CliResult<String> {
    if let Some(path) = DUCKDB_PATH.get() {
        return Ok(path.clone());
    }

    let duckdb_path = env::var("QSV_DESCRIBEGPT_DB_ENGINE")
        .map_err(|_| "QSV_DESCRIBEGPT_DB_ENGINE env var not set. Required for --duckdb mode.")?;

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

    let _ = DUCKDB_PATH.set(duckdb_path.clone());
    Ok(duckdb_path)
}

// ════════════════════════════════════════════════════════════════════════════
// Scoring Functions — each returns (score, detail_string)
// ════════════════════════════════════════════════════════════════════════════

fn score_type_optimization(
    sql_info: &SqlInfo,
    caches: &[InputCacheData],
    suggestions: &mut Vec<String>,
) -> (u32, String) {
    if caches.iter().all(|c| c.stats.is_empty()) {
        return (
            WEIGHT_TYPE_OPT,
            "No stats available — full score assumed".to_string(),
        );
    }

    let mut score = WEIGHT_TYPE_OPT;
    let mut issues = Vec::new();

    // Check columns used in WHERE/JOIN — ideally they should be numeric types for comparisons
    let comparison_cols: Vec<&str> = sql_info
        .where_columns
        .iter()
        .chain(sql_info.join_columns.iter())
        .map(String::as_str)
        .collect();

    for cache in caches {
        for stat in &cache.stats {
            let col_upper = stat.field.to_ascii_uppercase();
            let is_referenced = comparison_cols
                .iter()
                .any(|c| c.to_ascii_uppercase() == col_upper);

            if is_referenced && stat.r#type == "String" {
                // String type used in comparison — potential issue
                score = score.saturating_sub(3);
                issues.push(format!(
                    "{}.{} used in comparison but typed as String",
                    cache.table_name, stat.field
                ));
                suggestions.push(format!(
                    "Column '{}.{}' is typed as String but used in comparisons — consider casting \
                     or ensuring proper type inference",
                    cache.table_name, stat.field
                ));
            }
        }
    }

    let detail = if issues.is_empty() {
        "all referenced columns have appropriate types".to_string()
    } else {
        issues.join("; ")
    };

    (score, detail)
}

fn score_join_cardinality(
    sql_info: &SqlInfo,
    caches: &[InputCacheData],
    suggestions: &mut Vec<String>,
) -> (u32, String) {
    if !sql_info.has_join {
        return (WEIGHT_JOIN_CARD, "no joins in query".to_string());
    }

    if sql_info.join_columns.is_empty() {
        // Potential cartesian join!
        suggestions.push(
            "Possible cartesian join detected — missing ON/USING clause. This will produce a \
             cross product of all rows."
                .to_string(),
        );
        return (
            0,
            "possible cartesian join — missing join condition".to_string(),
        );
    }

    let mut score = WEIGHT_JOIN_CARD;
    let mut details = Vec::new();

    for join_col in &sql_info.join_columns {
        let col_upper = join_col.to_ascii_uppercase();
        for cache in caches {
            if let Some(stat) = cache
                .stats
                .iter()
                .find(|s| s.field.to_ascii_uppercase() == col_upper)
            {
                // Check cardinality
                if stat.cardinality < 10 {
                    score = score.saturating_sub(3);
                    details.push(format!(
                        "{}.{} low cardinality ({}) — may cause skew",
                        cache.table_name, stat.field, stat.cardinality
                    ));
                    suggestions.push(format!(
                        "Join key '{}.{}' has very low cardinality ({}) — may cause data skew in \
                         join results",
                        cache.table_name, stat.field, stat.cardinality
                    ));
                }

                // Check nulls in join key
                if stat.nullcount > 0 {
                    let sparsity = stat.sparsity.unwrap_or(0.0);
                    if sparsity > 0.1 {
                        score = score.saturating_sub(2);
                        details.push(format!(
                            "{}.{} has {:.0}% nulls — wasteful in joins",
                            cache.table_name,
                            stat.field,
                            sparsity * 100.0
                        ));
                        suggestions.push(format!(
                            "Join key '{}.{}' has {:.0}% null values — consider filtering nulls \
                             before joining",
                            cache.table_name,
                            stat.field,
                            sparsity * 100.0
                        ));
                    }
                }
            }
        }
    }

    let detail = if details.is_empty() {
        "good cardinality on join keys".to_string()
    } else {
        details.join("; ")
    };

    (score, detail)
}

fn score_filter_selectivity(
    sql_info: &SqlInfo,
    caches: &[InputCacheData],
    suggestions: &mut Vec<String>,
) -> (u32, String) {
    if !sql_info.has_where {
        return (
            WEIGHT_FILTER_SEL / 2,
            "no WHERE clause — full table scan".to_string(),
        );
    }

    let mut score = WEIGHT_FILTER_SEL;
    let mut details = Vec::new();

    // Check selectivity from frequency cache
    for col in &sql_info.where_columns {
        let col_upper = col.to_ascii_uppercase();
        for cache in caches {
            if let Some(freq) = cache
                .freq_entries
                .iter()
                .find(|f| f.field.to_ascii_uppercase() == col_upper)
            {
                // If the most common value accounts for > 70% of rows, selectivity is low
                if let Some(top) = freq.frequencies.first() {
                    if top.percentage > 70.0
                        && top.value != "<ALL_UNIQUE>"
                        && top.value != "<HIGH_CARDINALITY>"
                    {
                        score = score.saturating_sub(5);
                        details.push(format!(
                            "{}.{} top value '{}' is {:.0}% of rows (low selectivity)",
                            cache.table_name, freq.field, top.value, top.percentage
                        ));
                        suggestions.push(format!(
                            "Column '{}.{}' filter may match {:.0}% of rows — consider a more \
                             selective filter",
                            cache.table_name, freq.field, top.percentage
                        ));
                    }
                }
            }
        }
    }

    let detail = if details.is_empty() {
        "filter columns have reasonable selectivity".to_string()
    } else {
        details.join("; ")
    };

    (score, detail)
}

fn score_data_distribution(
    sql_info: &SqlInfo,
    caches: &[InputCacheData],
    suggestions: &mut Vec<String>,
) -> (u32, String) {
    let relevant_cols: Vec<&str> = sql_info
        .join_columns
        .iter()
        .chain(sql_info.where_columns.iter())
        .chain(sql_info.order_columns.iter())
        .map(String::as_str)
        .collect();

    if relevant_cols.is_empty() {
        return (
            WEIGHT_DATA_DIST,
            "no columns to check distribution for".to_string(),
        );
    }

    let mut score = WEIGHT_DATA_DIST;
    let mut details = Vec::new();

    for cache in caches {
        for stat in &cache.stats {
            let col_upper = stat.field.to_ascii_uppercase();
            let is_referenced = relevant_cols
                .iter()
                .any(|c| c.to_ascii_uppercase() == col_upper);
            if !is_referenced {
                continue;
            }

            // Check skewness
            if let Some(skewness) = stat.skewness {
                if skewness.abs() > 2.0 {
                    score = score.saturating_sub(2);
                    details.push(format!(
                        "{}.{} highly skewed (skewness={:.2})",
                        cache.table_name, stat.field, skewness
                    ));
                }
            }

            // Check gini coefficient (from moarstats)
            if let Some(gini) = stat.gini_coefficient {
                if gini > 0.5 {
                    score = score.saturating_sub(2);
                    details.push(format!(
                        "{}.{} high inequality (gini={:.2})",
                        cache.table_name, stat.field, gini
                    ));
                    suggestions.push(format!(
                        "Column '{}.{}' has high inequality (gini={:.2}) — may cause performance \
                         issues in joins/aggregations",
                        cache.table_name, stat.field, gini
                    ));
                }
            }

            // Check entropy (from moarstats)
            if let Some(entropy) = stat.normalized_entropy {
                if entropy < 0.3 {
                    score = score.saturating_sub(1);
                    details.push(format!(
                        "{}.{} low entropy ({:.2}) — data is highly concentrated",
                        cache.table_name, stat.field, entropy
                    ));
                }
            }
        }
    }

    let detail = if details.is_empty() {
        "data distribution looks balanced".to_string()
    } else {
        details.join("; ")
    };

    (score, detail)
}

fn score_query_patterns(
    sql_info: &SqlInfo,
    plan: &str,
    suggestions: &mut Vec<String>,
) -> (u32, String) {
    let mut score = WEIGHT_PATTERNS;
    let mut issues = Vec::new();

    // SELECT * anti-pattern
    if sql_info.has_select_star {
        score = score.saturating_sub(4);
        issues.push("SELECT * detected".to_string());
        suggestions
            .push("Replace SELECT * with specific columns to reduce memory usage".to_string());
    }

    // ORDER BY without LIMIT on what could be large results
    if sql_info.has_order_by && !sql_info.has_limit {
        score = score.saturating_sub(3);
        issues.push("ORDER BY without LIMIT".to_string());
        suggestions.push(
            "Add LIMIT to ORDER BY queries to avoid sorting the entire result set".to_string(),
        );
    }

    // Nested subqueries (could be CTEs)
    if sql_info.has_subquery {
        score = score.saturating_sub(2);
        issues.push("nested subquery detected".to_string());
        suggestions.push(
            "Consider using CTEs (WITH clause) instead of nested subqueries for better \
             readability and potential optimization"
                .to_string(),
        );
    }

    // Cartesian join (JOIN without ON)
    if sql_info.has_join && sql_info.join_columns.is_empty() {
        score = score.saturating_sub(5);
        issues.push("cartesian join — missing ON/WHERE clause".to_string());
    }

    // Check plan for full table scans
    let plan_upper = plan.to_ascii_uppercase();
    if plan_upper.contains("FULL SCAN") || plan_upper.contains("TABLE SCAN") {
        score = score.saturating_sub(2);
        issues.push("full table scan in query plan".to_string());
    }

    let detail = if issues.is_empty() {
        "no anti-patterns detected".to_string()
    } else {
        issues.join("; ")
    };

    (score, detail)
}

// ════════════════════════════════════════════════════════════════════════════
// Output Formatting
// ════════════════════════════════════════════════════════════════════════════

fn format_human_report(report: &ScoreReport) -> String {
    let mut out = String::with_capacity(2048);

    out.push_str(&format!(
        "Score: {}/{} ({})\n\n",
        report.score, report.max_score, report.rating
    ));

    out.push_str("=== Query Plan ===\n");
    out.push_str(&report.plan);
    out.push('\n');
    if !report.plan.ends_with('\n') {
        out.push('\n');
    }

    out.push_str("\n=== Scoring Breakdown ===\n");
    for b in &report.breakdown {
        out.push_str(&format!(
            "  {:25} {:>2}/{:<2}  - {}\n",
            format!("{}:", b.category),
            b.score,
            b.max,
            b.detail
        ));
    }

    if !report.suggestions.is_empty() {
        out.push_str("\n=== Suggestions ===\n");
        for (i, s) in report.suggestions.iter().enumerate() {
            out.push_str(&format!("  {}. {s}\n", i + 1));
        }
    }

    // Cache status summary
    if !report.cache_status.stats_missing.is_empty()
        || !report.cache_status.frequency_missing.is_empty()
        || !report.cache_status.index_missing.is_empty()
    {
        out.push_str("\n=== Cache Status ===\n");
        if !report.cache_status.stats_missing.is_empty() {
            out.push_str(&format!(
                "  Stats missing: {}\n",
                report.cache_status.stats_missing.join(", ")
            ));
        }
        if !report.cache_status.frequency_missing.is_empty() {
            out.push_str(&format!(
                "  Frequency missing: {}\n",
                report.cache_status.frequency_missing.join(", ")
            ));
        }
        if !report.cache_status.index_missing.is_empty() {
            out.push_str(&format!(
                "  Index missing: {}\n",
                report.cache_status.index_missing.join(", ")
            ));
        }
    }

    out
}
