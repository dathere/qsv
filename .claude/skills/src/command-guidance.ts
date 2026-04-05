/**
 * Command guidance system — provides contextual hints for tool selection and usage.
 */

import type { QsvSkill } from "./types.js";
import { config } from "./config.js";

/**
 * Consolidated guidance for each command.
 * Combines when-to-use, common patterns, error prevention,
 * and behavioral flags into a single lookup.
 */
export interface CommandGuidance {
  whenToUse?: string;
  commonPattern?: string;
  errorPrevention?: string;
  needsMemoryWarning?: boolean;
  needsIndexHint?: boolean;
  hasCommonMistakes?: boolean;
}

export const COMMAND_GUIDANCE: Record<string, CommandGuidance> = {
  select: {
    whenToUse: 'Choose columns. Syntax: "1,3,5" (specific), "1-10" (range), "!SSN" (exclude), "/<regex>/" (pattern), "_" (last).',
    commonPattern: "First step: select columns → filter → sort → output. Speeds up downstream ops.",
  },
  slice: {
    whenToUse: "Select rows by position: first N, last N, skip N, range.",
  },
  search: {
    whenToUse: "Filter rows matching pattern/regex. Search applied to selected fields. For complex conditions, use qsv_sqlp.",
    commonPattern: "Combine with select: search (filter rows) → select (pick columns).",
    needsIndexHint: true,
  },
  stats: {
    whenToUse: "Quick numeric stats (mean, min/max, stddev). Creates cache for other commands. Run 2nd after index.",
    commonPattern: `Run 2nd (after index). Creates cache used by frequency, schema, tojsonl, sqlp, joinp, pivotp, describegpt, moarstats, sample. Moarstats is auto-run after stats to enrich the cache with ~18 additional columns.`,
    errorPrevention: "Works with CSV/TSV/SSV files only. For SQL queries, use sqlp. Run qsv_index first for files >10MB.",
    needsIndexHint: true,
  },
  moarstats: {
    whenToUse: "Basic moarstats is auto-run after stats. Only invoke manually for --advanced (kurtosis, entropy, gini, etc.) or --bivariate (pairwise correlations).",
    commonPattern: "Basic moarstats runs automatically after stats to enrich the .stats.csv cache. Invoke manually only for --advanced or --bivariate. When running manually, set output_file to the stats cache path (<FILESTEM>.stats.csv, e.g. for data.csv use output_file=data.stats.csv). Enriches .stats.csv with ~18 additional columns for richer LLM analysis — moarstats enriches .stats.csv only, not .data.jsonl; smart commands still use .data.jsonl. With --bivariate: main stats to --output, bivariate stats to <FILESTEM>.stats.bivariate.csv (separate file next to input).",
    errorPrevention: "Run stats first to create cache. IMPORTANT: Only run --bivariate when requested as it's expensive. It writes results to a SEPARATE file: <FILESTEM>.stats.bivariate.csv (located next to the input file, NOT in stdout/output). Always read this file to get bivariate results. With --join-inputs, the file is <FILESTEM>.stats.bivariate.joined.csv.",
    needsMemoryWarning: true,
    hasCommonMistakes: true,
  },
  pragmastat: {
    whenToUse:
      "Robust outlier-resistant statistics (Hodges-Lehmann center, Shamos spread). Use when data is messy, heavy-tailed, or outlier-prone. Use --twosample to compare column pairs (shift, ratio, disparity).",
    commonPattern:
      "Index → Pragmastat for single-sample analysis. For comparisons: --twosample --select col1,col2. Use --misrate 1e-6 for critical decisions (default 1e-3).",
    errorPrevention:
      "Only processes numeric columns (non-numeric appear with n=0). All numeric values loaded into memory. Blank cells in output mean insufficient data or positivity requirement not met.",
    needsMemoryWarning: true,
  },
  frequency: {
    whenToUse: "Count unique values. Best for low-cardinality categorical columns. Run qsv_stats --cardinality first to identify high-cardinality columns to exclude.",
    commonPattern: "Stats → Frequency: Use qsv_stats --cardinality first to identify high-cardinality columns (IDs) to exclude. The frequency cache (--frequency-jsonl) is auto-created on first run for faster subsequent analysis.",
    errorPrevention: "High-cardinality columns (IDs, timestamps) can produce huge output. Use qsv_stats --cardinality to inspect column cardinality before running frequency. Do NOT set a client-side timeout shorter than the server's operation timeout (default 10 min) — let frequency run to completion. If the server timeout is exceeded on very large files, fall back to qsv_sqlp: 'SELECT col, COUNT(*) FROM _t_1 GROUP BY col ORDER BY COUNT(*) DESC LIMIT 20'. Use --select to target specific columns instead of computing frequency on all columns.",
    needsMemoryWarning: true,
    hasCommonMistakes: true,
  },
  join: {
    whenToUse: "Join CSV files (<50MB). For large/complex joins, use qsv_joinp.",
    commonPattern: "Run qsv_index first on both files for speed.",
    errorPrevention: "Both files need join column(s). Column names case-sensitive. Check with qsv_headers.",
    hasCommonMistakes: true,
  },
  joinp: {
    whenToUse: "Fast Polars-powered joins for large files (>50MB) or SQL-like joins (inner/left/right/outer/cross/asof). Asof joins match on the nearest key rather than exact equality — ideal for time-series data. Use stats cache (qsv_stats --cardinality) to determine optimal table order (smaller cardinality on right).",
    commonPattern: "Stats → Join: Use qsv_stats --cardinality on both files, put lower-cardinality join column on right for efficiency. Check nullcount on join columns — nulls never match in joins and high null rates explain missing rows. For time-series joins, use --asof to match on nearest key rather than exact equality; both datasets are auto-sorted on join columns unless --no-sort is set.",
    errorPrevention: "Use --try-parsedates for date joins. Check column types with qsv_stats — mismatched types (String vs Integer) cause silent join failures.",
    hasCommonMistakes: true,
  },
  dedup: {
    whenToUse: "Remove duplicates. Loads entire CSV. For large files (>1GB), use qsv_extdedup. Use qsv_stats --cardinality to check column cardinality - if key column has unique values only, dedup will be a no-op.",
    commonPattern: "Often followed by stats: dedup → stats for distribution.",
    errorPrevention: "May OOM on files >1GB. Use qsv_extdedup for large files.",
    needsMemoryWarning: true,
    hasCommonMistakes: true,
  },
  sort: {
    whenToUse: "Sort by columns. Loads entire file. For large files (>1GB), use qsv_extsort. Use stats cache to check if data is already sorted.",
    commonPattern: "Before joins or top-N: sort DESC → slice --end 10.",
    errorPrevention: "May OOM on files >1GB. Use qsv_extsort for large files.",
    needsMemoryWarning: true,
    hasCommonMistakes: true,
  },
  count: {
    whenToUse: "Count rows. Very fast with index. Run qsv_index first for files >10MB.",
    errorPrevention: "Works with CSV/TSV/SSV files only. Very fast with index file (.idx).",
  },
  headers: {
    whenToUse: "View/rename column names. Quick CSV structure discovery.",
    errorPrevention: "Works with CSV/TSV/SSV files only. For Parquet schema, use sqlp with DESCRIBE.",
  },
  sample: {
    whenToUse: "Random sampling. Fast, memory-efficient. Good for previews or test datasets.",
    commonPattern: "Quick preview (100 rows) or test data (1000 rows). Faster than qsv_slice for random.",
    needsIndexHint: true,
  },
  schema: {
    whenToUse: "Infer data types, generate Polars Schema & JSON Schema.",
    commonPattern:
      "Iterate: qsv_schema → validate → fix → validate until clean. Use --polars to generate Polars schema for qsv_to_parquet.",
    errorPrevention:
      "Run qsv_stats first for best type inference. Use --polars for Parquet conversion workflows.",
    hasCommonMistakes: true,
  },
  validate: {
    whenToUse: "Validate against JSON Schema. Check data quality, type correctness. Also use this without a JSON Schema to check if a CSV is well-formed.",
    commonPattern: "Iterate: qsv_schema ��� validate → fix → validate until clean.",
    needsIndexHint: true,
  },
  sqlp: {
    whenToUse:
      "Run SQL queries on tabular data. Auto-converts CSV to Parquet for performance, then routes to DuckDB when available (faster, PostgreSQL-compatible). Falls back to Polars SQL (sqlp) otherwise.",
    commonPattern:
      "Stats → SQL: Read qsv_stats output before writing queries. Use type for correct casts (don't quote integers, use date functions for Date/DateTime). Use min/max/range for precise WHERE clauses. Use cardinality to optimize GROUP BY (low = fast, high = consider LIMIT). Use sort_order to skip redundant ORDER BY. For value distributions, run qsv_frequency on relevant columns. For multi-file queries, convert all files to Parquet first with qsv_to_parquet, then use read_parquet() in SQL. For complex queries on large files, use EXPLAIN to review the query plan before execution.",
    errorPrevention:
      "Column names are case-sensitive in Polars SQL but case-insensitive in DuckDB. For unsupported output formats (Arrow, Avro), sqlp is used automatically. Use nullcount from qsv_stats to add COALESCE/IS NOT NULL only where nulls actually exist — skip null handling for columns with nullcount=0. In Claude Cowork, ensure DuckDB runs on the host, not the Linux container.",
    hasCommonMistakes: true,
  },
  rename: {
    whenToUse: "Rename columns. Supports bulk/regex.",
  },
  template: {
    whenToUse: "Generate formatted output from CSV using Mini Jinja templates. For reports, markdown, HTML.",
    needsIndexHint: true,
  },
  index: {
    whenToUse: "Create .idx index. Run FIRST for files >10MB queried multiple times. Enables instant counts, fast slicing.",
    commonPattern: "Run 1st for files >10MB. Makes count instant, slice 100x faster.",
    errorPrevention: "Creates .idx index for CSV/TSV/SSV files only. Parquet files don't need indexing.",
  },
  diff: {
    whenToUse: "Compare CSV files (added/deleted/modified rows). Requires same schema.",
  },
  cat: {
    whenToUse: "Concatenate CSV files. Subcommands: rows (stack vertically), rowskey (different schemas), columns (side-by-side). Specify via subcommand parameter.",
    commonPattern: "Combine files: cat rows → headers from first file only. cat rowskey → handles different schemas. cat columns → side-by-side merge.",
    errorPrevention: "rows mode requires same column order. Use rowskey for different schemas.",
    hasCommonMistakes: true,
  },
  geocode: {
    whenToUse: "Geocode locations using Geonames/MaxMind. Subcommands: suggest, reverse, countryinfo, iplookup. Specify via subcommand parameter.",
    commonPattern: "Common: suggest for city lookup, reverse for lat/lon → city, iplookup for IP → location.",
    errorPrevention: "Needs Geonames index (auto-downloads on first use). iplookup needs MaxMind GeoLite2 DB.",
    needsIndexHint: true,
  },
  pivotp: {
    whenToUse: "Polars-powered pivot tables. Use --agg for aggregation (sum/mean/count/first/last/min/max/smart). Use qsv_stats --cardinality to check pivot column cardinality.",
    commonPattern: "Stats → Pivot: Use qsv_stats --cardinality to estimate pivot output width (pivot column cardinality × value columns) and keep estimated columns below ~1000 to avoid overly wide pivots. Use stats type column to pick the right --agg: sum/mean for numeric, count for categorical.",
    errorPrevention: "High-cardinality pivot columns create wide output. Use qsv_stats --cardinality to check cardinality of potential pivot columns.",
    hasCommonMistakes: true,
  },
  excel: {
    whenToUse: "Convert spreadsheets (Excel and OpenDocument) to CSV. Also can be used to get workbook metadata. Supports multi-sheet workbooks.",
  },
  searchset: {
    whenToUse:
      "Filter rows matching any pattern from a regex file. For multiple patterns at once. Use qsv_search for single patterns.",
    errorPrevention: "Needs regex file. qsv_search easier for simple patterns.",
    needsIndexHint: true,
    hasCommonMistakes: true,
  },
  datefmt: {
    whenToUse:
      "Parse and reformat date/time columns. Supports diverse input formats and strftime output patterns.",
    needsIndexHint: true,
  },
  luau: {
    whenToUse:
      "Run Luau scripts per row. Use map to create new columns, filter to select rows. For complex custom logic beyond apply.",
    needsIndexHint: true,
  },
  replace: {
    whenToUse:
      "Find and replace text in columns using regex. For bulk text substitution across the dataset.",
    needsIndexHint: true,
  },
  split: {
    whenToUse:
      "Split CSV into chunks of N rows each, writing separate files. For breaking large files into manageable pieces.",
    needsIndexHint: true,
  },
  tojsonl: {
    whenToUse:
      "Convert CSV to JSONL/NDJSON with smart type inference. Uses stats cache for accurate types.",
    commonPattern:
      "Run qsv_stats first for best type inference. Output uses correct JSON types (numbers, booleans, nulls).",
    needsIndexHint: true,
  },
  transpose: {
    whenToUse:
      "Swap rows and columns. Best for small datasets or creating wide-format summaries.",
    needsMemoryWarning: true,
    needsIndexHint: true,
  },
  // Commands added for guidance coverage
  reverse: {
    whenToUse:
      "Reverse row order preserving relative order (stable). With index: constant memory. Without index: loads entire CSV.",
    errorPrevention:
      "Without an index file, loads entire CSV into memory. Run qsv_index first, or use qsv_sort --reverse for sorted reversal.",
    needsMemoryWarning: true,
    needsIndexHint: true,
  },
  safenames: {
    whenToUse:
      "Make headers database-ready/CKAN-ready. Removes special chars, spaces, ensures unique names.",
  },
  sniff: {
    whenToUse:
      "Detect CSV metadata (delimiter, header, preamble, quote char, encoding, field types). Also a general mime type detector. Supports URLs.",
    commonPattern:
      "First step for unknown files: sniff → headers → stats → frequency. Use --json for parseable output.",
    errorPrevention:
      "For remote URLs, use --quick for faster detection. Use --sample to control inference depth.",
  },
  extdedup: {
    whenToUse:
      "Remove duplicates from arbitrarily large files (>1GB) using on-disk hash table. Constant memory. Use instead of qsv_dedup for large files.",
    errorPrevention:
      "Does not sort output (unlike dedup). Requires explicit output file argument. Use --dupes-output to capture removed rows.",
  },
  extsort: {
    whenToUse:
      "Sort arbitrarily large files (>1GB) using external merge sort. Use instead of qsv_sort for large files.",
    errorPrevention:
      "Sorts entire rows as text (no column-specific sorting). For column-specific sorting of large files, use qsv_sqlp.",
    needsIndexHint: true,
  },
  partition: {
    whenToUse:
      "Split CSV into separate files by column value. One output file per unique value in the partition column.",
    errorPrevention:
      "High-cardinality columns create many files. Use qsv_stats --cardinality to check column cardinality first.",
    hasCommonMistakes: true,
  },
  explode: {
    whenToUse:
      "Unnest multi-value cells into separate rows. Splits a column on a separator, creating one row per value.",
  },
  pseudo: {
    whenToUse:
      "Pseudonymize column values with incremental IDs. For de-identification or anonymization before sharing data.",
  },
  describegpt: {
    whenToUse: "Generate data dictionaries, descriptions, and tags for CSV data using LLM inference.",
    commonPattern: "Through MCP: no API key needed — uses the connected LLM automatically. Use --dictionary, --description, --tags, or --all. May require two tool calls (first returns prompts, second processes your responses via _llm_responses). For natural language questions, use sqlp or other qsv tools directly instead of --prompt.",
    errorPrevention: "In MCP mode: do NOT use --prompt (SQL RAG mode) — ask the LLM directly instead. Do NOT pass --base-url or --api-key. LLM results may be inaccurate. Run stats first for best results.",
  },
  fixlengths: {
    whenToUse:
      "Fix ragged CSVs where rows have inconsistent field counts. Pads short rows, truncates long rows to match the longest row.",
    commonPattern:
      "Early in cleaning: safenames → fixlengths → trim → dedup. Run count before/after to detect how many rows were ragged.",
  },
  fmt: {
    whenToUse:
      "Reformat CSV: change delimiter, quoting style, line endings. Use --out-delimiter to convert between CSV/TSV/SSV.",
    commonPattern:
      "CSV ↔ TSV: fmt --out-delimiter '\\t'. Also useful for normalizing quoting before downstream tools.",
  },
  input: {
    whenToUse:
      "Normalize encoding to UTF-8, strip BOM, and handle non-UTF-8 CSV files. First step for files with encoding issues.",
  },
  table: {
    whenToUse:
      "Pretty-print CSV as an aligned text table. For small files or terminal display only.",
    needsMemoryWarning: true,
  },
  fill: {
    whenToUse:
      "Fill empty fields with values from previous rows or a specified groupby column. Useful for sparse data with repeated group headers.",
  },
  sortcheck: {
    whenToUse:
      "Check if CSV is already sorted on specified columns. Avoids unnecessary sort operations.",
  },
  enum: {
    whenToUse:
      "Add a row number, UUID, or constant column. Useful for creating IDs or tracking row provenance.",
  },
  exclude: {
    whenToUse:
      "Remove rows from one CSV that match rows in another. Inverse of join — keeps only non-matching rows. For anti-join semantics, joinp --anti is faster for large files.",
  },
  flatten: {
    whenToUse:
      "Display each row vertically (one field per line). For inspecting wide CSVs with many columns in the terminal.",
  },
};

/**
 * Enhance parameter descriptions with examples and common values
 */
export function enhanceParameterDescription(
  paramName: string,
  description: string,
): string {
  let enhanced = description;

  // Add examples for common parameters
  switch (paramName) {
    case "delimiter":
      enhanced += ' e.g. "," "\\t" "|" ";"';
      break;
    case "select":
      enhanced +=
        ' e.g. "1,3,5" (specific columns), "1-10" (range), "!SSN,!password" (exclude), "name,age,city" (by name), "_" (last column), "/<regex>/" (regex).';
      break;
    case "output":
    case "output_file":
      enhanced +=
        " Tip: Use absolute paths. Omit for small results (returned directly), or specify for large datasets (auto-saved if >850KB).";
      break;
    case "no_headers":
      enhanced +=
        " Use when CSV has no header row. First row will be treated as data.";
      break;
    case "ignore_case":
      enhanced += " Makes pattern matching case-insensitive.";
      break;
  }

  return enhanced;
}


/**
 * Enhance tool description with contextual guidance
 *
 * Uses concise description from README.md and adds guidance hints
 * that help Claude select the right tool. For detailed help,
 * use the qsv_help tool which calls `qsv <command> --help`.
 */
export function enhanceDescription(skill: QsvSkill): string {
  const commandName = skill.command.subcommand;
  const guidance = COMMAND_GUIDANCE[commandName];

  // Use concise description from README.md
  let description = skill.description;

  // Add when-to-use guidance (critical for tool selection)
  if (guidance?.whenToUse) {
    description += `\n\n💡 ${guidance.whenToUse}`;
  }

  // Add subcommand requirement for commands that need it
  if (commandName === "cat") {
    description += `\n\n🔧 SUBCOMMAND: Must pass subcommand via args (e.g., args: {subcommand: "rows", input: "file.csv"}).`;
  } else if (commandName === "geocode") {
    description += `\n\n🔧 SUBCOMMAND: Must pass subcommand via args (e.g., args: {subcommand: "suggest", column: "city", input: "data.csv"}).`;
  }

  // Add common patterns (helps Claude compose workflows)
  if (guidance?.commonPattern) {
    description += `\n\n📋 ${guidance.commonPattern}`;
  }

  // Add performance hints only for commands that benefit from indexing
  if (skill.hints) {
    // Only show memory warnings for memory-intensive commands
    if (guidance?.needsMemoryWarning) {
      if (skill.hints.memory === "full") {
        description += "\n\n⚠️  Loads entire CSV. Best <100MB.";
      } else if (skill.hints.memory === "proportional") {
        description += "\n\n⚠️  Memory ∝ unique values.";
      }
    }

    // Only show index hints for commands that are index-accelerated
    if (guidance?.needsIndexHint && skill.hints.indexed) {
      description +=
        "\n\n🚀 Index-accelerated. Run qsv_index first on files >10MB.";
    }
  }

  // Add error prevention hints only for commands with common mistakes
  if (guidance?.hasCommonMistakes && guidance?.errorPrevention) {
    description += `\n\n⚠️  ${guidance.errorPrevention}`;
  }

  // Add usage examples from skill JSON (if available)
  // Configurable via QSV_MCP_MAX_EXAMPLES environment variable (default: 5, max: 20, 0 to disable)
  if (skill.examples && skill.examples.length > 0 && config.maxExamples > 0) {
    const maxExamples = config.maxExamples;
    const examplesToShow = skill.examples.slice(0, maxExamples);

    description += "\n\n📝 EXAMPLES:";
    for (const example of examplesToShow) {
      description += `\n• ${example.command}`;
    }

    if (skill.examples.length > maxExamples) {
      description += `\n  (${skill.examples.length - maxExamples} more - use help=true for full list)`;
    }
  }

  return description;
}
