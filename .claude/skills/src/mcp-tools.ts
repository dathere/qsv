/**
 * MCP Tool Definitions and Handlers for QSV Commands
 */

import type { ChildProcess } from "child_process";
import { randomUUID } from "crypto";
import { stat, access, readFile, writeFile, open, unlink, rename, copyFile, readdir } from "fs/promises";
import { constants } from "fs";
import { basename, dirname, join } from "path";
import { tmpdir } from "os";
import { ConvertedFileManager } from "./converted-file-manager.js";
import type {
  QsvSkill,
  Argument,
  Option,
  McpToolDefinition,
  McpToolProperty,
  FilesystemProviderExtended,
  SkillCategory,
} from "./types.js";
import { runQsvSimple } from "./executor.js";
import type { SkillExecutor } from "./executor.js";
import type { SkillLoader } from "./loader.js";
import { config, getDetectionDiagnostics } from "./config.js";
import { formatBytes, findSimilarFiles, errorResult, successResult } from "./utils.js";
import {
  detectDuckDb,
  getDuckDbStatus,
  markDuckDbUnavailable,
  translateSql,
  executeDuckDbQuery,
  MULTI_TABLE_PATTERN,
  CSV_LIKE_EXTENSIONS,
  normalizeTableRefs,
} from "./duckdb.js";

/**
 * Auto-indexing threshold in MB
 */
const AUTO_INDEX_SIZE_MB = 10;

/**
 * Commands that always return full CSV data and should use temp files
 */
const ALWAYS_FILE_COMMANDS = new Set([
  "stats",
  "moarstats",
  "frequency",
  "slice",
  "sample",
  "template",
  "geocode",
  "sort",
  "dedup",
  "join",
  "joinp",
  "select",
  "search",
  "searchset",
  "schema",
  "validate",
  "diff",
  "cat",
  "transpose",
  "partition",
  "split",
  "explode",
  "pseudo",
  "rename",
  "replace",
  "datefmt",
  "reverse",
  "safenames",
  "sqlp",
  "pivotp",
  "pragmastat",
  "tojsonl",
]);

/**
 * Commands that return small metadata (not full CSV) and should use stdout
 */
const METADATA_COMMANDS = new Set(["count", "headers", "index", "sniff"]);

/** Commands whose output is NOT tabular CSV — skip TSV conversion */
const NON_TABULAR_COMMANDS = new Set([
  ...METADATA_COMMANDS,  // count, headers, index, sniff
  "tojsonl",             // JSONL output
  "template",            // Free-form text
  "schema",              // JSON Schema output
  "validate",            // Validation messages, not CSV data
]);

/** Binary output formats from sqlp that should never get a .tsv extension */
const BINARY_OUTPUT_FORMATS = new Set(["parquet", "arrow", "avro"]);

/**
 * Check if the command+params produce binary output (not tabular text).
 * Used to skip auto temp file creation and prevent .tsv extensions for
 * binary formats (parquet/arrow/avro) that can't be read back as UTF-8.
 */
export function isBinaryOutputFormat(commandName: string, params: Record<string, unknown>): boolean {
  return commandName === "sqlp" &&
    BINARY_OUTPUT_FORMATS.has(String(params.format ?? "").toLowerCase());
}

/**
 * Consolidated guidance for each command.
 * Combines when-to-use, common patterns, error prevention,
 * and behavioral flags into a single lookup.
 */
interface CommandGuidance {
  whenToUse?: string;
  commonPattern?: string;
  errorPrevention?: string;
  needsMemoryWarning?: boolean;
  needsIndexHint?: boolean;
  hasCommonMistakes?: boolean;
}

const COMMAND_GUIDANCE: Record<string, CommandGuidance> = {
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
    commonPattern: "Run 2nd (after index). Creates cache used by frequency, schema, tojsonl, sqlp, joinp, pivotp, describegpt, moarstats, sample.",
    errorPrevention: "Works with CSV/TSV/SSV files only. For SQL queries, use sqlp. Run qsv_index first for files >10MB.",
    needsIndexHint: true,
  },
  moarstats: {
    whenToUse: "Comprehensive stats + bivariate stats + outlier details + data type inference. Slower but richer than stats.",
    commonPattern: "Index → Stats → Moarstats for richest analysis. With --bivariate: main stats to --output, bivariate stats to <FILESTEM>.stats.bivariate.csv (separate file next to input).",
    errorPrevention: "Run stats first to create cache. Slower than stats but richer output. IMPORTANT: --bivariate writes results to a SEPARATE file: <FILESTEM>.stats.bivariate.csv (located next to the input file, NOT in stdout/output). Always read this file to get bivariate results. With --join-inputs, the file is <FILESTEM>.stats.bivariate.joined.csv.",
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
    commonPattern: "Iterate: qsv_schema → validate → fix → validate until clean.",
    needsIndexHint: true,
  },
  sqlp: {
    whenToUse:
      "Run SQL queries on tabular data. Auto-converts CSV to Parquet for performance, then routes to DuckDB when available (faster, PostgreSQL-compatible). Falls back to Polars SQL (sqlp) otherwise.",
    commonPattern:
      "Stats → SQL: Read qsv_stats output before writing queries. Use type for correct casts (don't quote integers, use date functions for Date/DateTime). Use min/max/range for precise WHERE clauses. Use cardinality to optimize GROUP BY (low = fast, high = consider LIMIT). Use sort_order to skip redundant ORDER BY. For value distributions, run qsv_frequency on relevant columns. For multi-file queries, convert all files to Parquet first with qsv_to_parquet, then use read_parquet() in SQL.",
    errorPrevention:
      "Column names are case-sensitive in Polars SQL but case-insensitive in DuckDB. For unsupported output formats (Arrow, Avro), sqlp is used automatically. Use nullcount from qsv_stats to add COALESCE/IS NOT NULL only where nulls actually exist — skip null handling for columns with nullcount=0.",
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
};

/**
 * Input file size threshold (in bytes) for auto temp file
 */
const LARGE_FILE_THRESHOLD_BYTES = 10 * 1024 * 1024; // 10MB

/**
 * Maximum size for MCP response (in bytes)
 * Outputs larger than this will be saved to working directory instead of returned directly
 * Claude Desktop has a 1MB limit, so we use 850KB to stay safely under
 */
const MAX_MCP_RESPONSE_SIZE = 850 * 1024; // 850KB - safe for Claude Desktop (< 1MB limit)

/**
 * Track active child processes for graceful shutdown (SIGTERM on exit).
 */
const activeProcesses = new Set<ChildProcess>();

/**
 * Track in-flight operation count for concurrency limiting.
 * Incremented/decremented via acquireSlot/releaseSlot in handleToolCall
 * to cover the entire execution path (both runQsvSimple and SkillExecutor.runQsv).
 */
let activeOperationCount = 0;

/**
 * A slot waiter with an explicit settled flag for reliable handoff detection.
 * `settled` is true if the waiter already timed out (callback becomes a no-op).
 */
interface SlotWaiter {
  settled: boolean;
  callback: () => void;
}

/**
 * Queue of waiters for concurrency slots.
 * Each entry carries a settled flag so releaseSlot can skip timed-out waiters
 * without relying on observable side-effects of the callback.
 */
const slotWaiters: SlotWaiter[] = [];

/**
 * Acquire a concurrency slot, waiting up to timeoutMs if all slots are busy.
 * Returns true if slot acquired, false if timed out.
 */
async function acquireSlot(timeoutMs: number): Promise<boolean> {
  // IMPORTANT: The check-then-increment below is safe because it's synchronous
  // (no `await` between check and increment). Node.js single-threaded execution
  // guarantees atomicity for synchronous code. Do NOT insert an `await` here.
  if (activeOperationCount < config.maxConcurrentOperations) {
    activeOperationCount++;
    return true;
  }

  // Prune all settled (timed-out) waiters before adding a new one,
  // so the array doesn't grow unboundedly if releases are rare.
  // Filter the entire array, not just the front, to handle cases where
  // early waiters have long timeouts and later ones time out first.
  for (let i = slotWaiters.length - 1; i >= 0; i--) {
    if (slotWaiters[i].settled) slotWaiters.splice(i, 1);
  }

  // No immediate slot — wait in queue
  return new Promise<boolean>((resolve) => {
    const waiter: SlotWaiter = { settled: false, callback: () => {} };

    const timer = setTimeout(() => {
      if (!waiter.settled) {
        waiter.settled = true;
        resolve(false);
      }
    }, timeoutMs);

    waiter.callback = () => {
      if (!waiter.settled) {
        waiter.settled = true;
        clearTimeout(timer);
        activeOperationCount++;
        resolve(true);
      }
    };

    slotWaiters.push(waiter);
  });
}

/**
 * Release a concurrency slot and wake the next waiter if any.
 */
function releaseSlot(): void {
  // Try to hand off to the next live waiter. Skip any that already timed out.
  while (slotWaiters.length > 0) {
    const waiter = slotWaiters.shift();
    if (waiter && !waiter.settled) {
      // The callback increments activeOperationCount for the new operation,
      // so we must also decrement for the releasing operation to keep the
      // count correct (net effect: count stays the same).
      waiter.callback();
      if (activeOperationCount > 0) {
        activeOperationCount--;
      } else {
        console.warn("releaseSlot: activeOperationCount already at 0 during waiter handoff — count/waiter mismatch");
      }
      return; // handed off successfully
    }
    // timed-out waiter, skip
  }
  // No waiters (or all timed out) — just release the slot.
  if (activeOperationCount > 0) {
    activeOperationCount--;
  } else {
    console.warn("releaseSlot: activeOperationCount already at 0 — possible double-release");
  }
}

/**
 * Flag indicating shutdown is in progress
 */
let isShuttingDown = false;

/**
 * Get QSV binary path (centralized)
 */
function getQsvBinaryPath(): string {
  return config.qsvBinPath;
}

/**
 * Run a qsv command with timeout, process tracking, and shutdown awareness.
 * Delegates to shared runQsvSimple for the actual spawning logic.
 */
async function runQsvWithTimeout(
  qsvBin: string,
  args: string[],
  timeoutMs: number = config.operationTimeoutMs,
): Promise<void> {
  if (isShuttingDown) {
    throw new Error("Server is shutting down, operation rejected");
  }

  await runQsvSimple(qsvBin, args, {
    timeoutMs,
    onSpawn: (proc) => activeProcesses.add(proc),
    onExit: (proc) => activeProcesses.delete(proc),
  });
}

/**
 * Check if an object has filesystem provider capabilities
 */
function isFilesystemProviderExtended(
  obj: unknown,
): obj is FilesystemProviderExtended {
  if (typeof obj !== "object" || obj === null) return false;
  const record = obj as Record<string, unknown>;
  return (
    typeof record.resolvePath === "function" &&
    typeof record.needsConversion === "function" &&
    typeof record.getConversionCommand === "function" &&
    typeof record.getWorkingDirectory === "function"
  );
}

/**
 * Build arguments for file conversion commands.
 * Handles different patterns:
 * - Excel/JSONL: qsv <cmd> <input> --output <output>
 * - Parquet→CSV: qsv sqlp SKIP_INPUT "select * from read_parquet('<input>')" --output <output>
 * - CSV→Parquet: qsv sqlp <input> "SELECT * FROM _t_1" --format parquet --output <output>
 *   (passes input directly so sqlp can detect .pschema.json for type inference)
 */
export function buildConversionArgs(
  conversionCmd: string,
  inputFile: string,
  outputFile: string,
): string[] {
  if (conversionCmd === "parquet") {
    // Parquet→CSV conversion
    // Normalize path separators for SQL (Windows backslashes → forward slashes)
    const normalizedPath = inputFile.replace(/\\/g, "/");
    // Escape single quotes in path for SQL string safety
    const escapedPath = normalizedPath.replace(/'/g, "''");
    const sql = `select * from read_parquet('${escapedPath}')`;
    return ["sqlp", "SKIP_INPUT", sql, "--output", outputFile];
  }
  if (conversionCmd === "csv-to-parquet") {
    // CSV→Parquet conversion: pass input directly so sqlp can detect .pschema.json for type inference
    return [
      "sqlp",
      inputFile,
      "SELECT * FROM _t_1",
      "--format",
      "parquet",
      "--compression",
      "snappy",
      "--statistics",
      "--output",
      outputFile,
    ];
  }
  // Standard: qsv <cmd> <input> --output <output>
  return [conversionCmd, inputFile, "--output", outputFile];
}

/**
 * Detect the delimiter for a CSV file based on its extension.
 * Returns ',' for .csv (and unknown), '\t' for .tsv/.tab, ';' for .ssv.
 */
export function detectDelimiter(filePath: string): string {
  const lower = filePath.toLowerCase();
  if (lower.endsWith(".tsv") || lower.endsWith(".tab")) return "\t";
  if (lower.endsWith(".ssv")) return ";";
  return ",";
}

/**
 * Parse a single CSV line respecting RFC 4180 quoted fields.
 * Returns an array of field values with quotes stripped.
 */
export function parseCSVLine(line: string, delimiter: string = ","): string[] {
  const fields: string[] = [];
  if (line.length === 0) return [""];
  let i = 0;
  while (i < line.length) {
    if (line[i] === '"') {
      // Quoted field
      let value = "";
      i++; // skip opening quote
      while (i < line.length) {
        if (line[i] === '"') {
          if (i + 1 < line.length && line[i + 1] === '"') {
            value += '"';
            i += 2;
          } else {
            i++; // skip closing quote
            break;
          }
        } else {
          value += line[i];
          i++;
        }
      }
      fields.push(value);
      if (i < line.length && line[i] === delimiter) {
        i++; // skip delimiter
        // Handle trailing delimiter after quoted field
        if (i === line.length) fields.push("");
      }
    } else {
      // Unquoted field
      const next = line.indexOf(delimiter, i);
      if (next === -1) {
        fields.push(line.slice(i));
        break;
      }
      fields.push(line.slice(i, next));
      i = next + 1;
      // Handle trailing delimiter
      if (i === line.length) fields.push("");
    }
  }
  return fields;
}

/**
 * Check if a Polars schema dtype represents a Date or Datetime type.
 * Polars schema dtypes can be a simple string like "Date" or an object like
 * {"Datetime": ["Milliseconds", null]}.
 */
export function isDateDtype(dtype: unknown): boolean {
  if (typeof dtype === "string") {
    return dtype === "Date";
  }
  if (typeof dtype === "object" && dtype !== null) {
    return "Datetime" in dtype || "Date" in dtype;
  }
  return false;
}

/**
 * Patch a Polars schema (.pschema.json) to change Date/Datetime columns
 * that contain AM/PM values to String, since Polars cannot parse 12-hour formats.
 *
 * The schema has the structure: { fields: { "ColName": dtype, ... }, metadata: ... }
 * where dtype is either a string (e.g. "String", "Date") or an object
 * (e.g. {"Datetime": ["Milliseconds", null]}).
 *
 * Reads the first ~50KB of the CSV to sample a few rows, then checks
 * date/datetime columns for AM/PM patterns.
 *
 * @returns List of column names that were patched to String.
 */
export async function patchSchemaAmPmDates(inputFile: string, schemaFile: string): Promise<string[]> {
  // Read and parse the schema
  let schemaText: string;
  try {
    schemaText = await readFile(schemaFile, "utf-8");
  } catch (error: unknown) {
    console.warn(`[MCP Tools] Schema file not found: ${schemaFile}`, error);
    return [];
  }

  let schema: { fields: Record<string, unknown>; metadata?: unknown };
  try {
    schema = JSON.parse(schemaText);
  } catch (error: unknown) {
    console.warn(`[MCP Tools] Invalid schema JSON in: ${schemaFile}`, error);
    return [];
  }

  if (!schema.fields || typeof schema.fields !== "object") return [];

  // Collect date/datetime column names from the schema
  const dateColNames: string[] = [];
  for (const [name, dtype] of Object.entries(schema.fields)) {
    if (isDateDtype(dtype)) {
      dateColNames.push(name);
    }
  }
  if (dateColNames.length === 0) return [];

  // Read first 50KB of the CSV to sample rows
  const SAMPLE_BYTES = 50 * 1024;
  let sampleText: string;
  let truncated = false;
  try {
    const fh = await open(inputFile, "r");
    try {
      const buf = Buffer.alloc(SAMPLE_BYTES);
      const { bytesRead } = await fh.read(buf, 0, SAMPLE_BYTES, 0);
      sampleText = buf.toString("utf-8", 0, bytesRead);
      truncated = bytesRead === SAMPLE_BYTES;
    } finally {
      await fh.close();
    }
  } catch (error: unknown) {
    console.warn(`[MCP Tools] Failed to read input file for AM/PM sampling: ${inputFile}`, error);
    return [];
  }

  // Split into lines; only drop last line if we truncated the read (potentially partial)
  const lines = sampleText.split(/\r?\n/);
  if (truncated) {
    lines.pop(); // remove potentially incomplete trailing line
  }
  // Remove trailing empty line from final newline
  if (lines.length > 0 && lines[lines.length - 1] === "") {
    lines.pop();
  }
  if (lines.length < 2) return [];

  // Detect delimiter from file extension
  const delimiter = detectDelimiter(inputFile);

  // Parse header to map column names to indices
  const headers = parseCSVLine(lines[0], delimiter);
  const colIndices = new Map<string, number>();
  for (let i = 0; i < headers.length; i++) {
    colIndices.set(headers[i], i);
  }

  // Resolve date column positions
  const targets: Array<{ colIdx: number; name: string }> = [];
  for (const name of dateColNames) {
    const idx = colIndices.get(name);
    if (idx !== undefined) {
      targets.push({ colIdx: idx, name });
    }
  }
  if (targets.length === 0) return [];

  // Check sample data rows for AM/PM pattern
  // Match digit(s) followed by whitespace then AM/PM to avoid false positives
  // on words like "Amsterdam" or "Pamphlet"
  const ampmRe = /\d\s*[AP]M\b/i;
  const patchedNames: string[] = [];
  for (const t of targets) {
    let found = false;
    for (let r = 1; r < lines.length && !found; r++) {
      if (lines[r].trim() === "") continue;
      const fields = parseCSVLine(lines[r], delimiter);
      if (t.colIdx < fields.length && ampmRe.test(fields[t.colIdx])) {
        found = true;
      }
    }
    if (found) {
      schema.fields[t.name] = "String";
      patchedNames.push(t.name);
    }
  }

  // Write back patched schema if any changes
  if (patchedNames.length > 0) {
    await writeFile(schemaFile, JSON.stringify(schema, null, 2), "utf-8");
  }

  return patchedNames;
}

/**
 * Patch the Polars schema for AM/PM dates and log results.
 * Shared helper used by both doParquetConversion and handleToParquetCall.
 */
async function patchSchemaAndLog(inputFile: string, schemaFile: string): Promise<void> {
  const patchedCols = await patchSchemaAmPmDates(inputFile, schemaFile);
  if (patchedCols.length > 0) {
    console.error(`[MCP Tools] Patched AM/PM date columns to String: ${patchedCols.join(", ")}`);
  }
}

/**
 * Auto-index a file if it's large enough and not already indexed
 * Reusable helper to avoid code duplication
 */
async function autoIndexIfNeeded(
  filePath: string,
  minSizeMB: number = AUTO_INDEX_SIZE_MB,
): Promise<void> {
  try {
    // Check if this is an indexable CSV format (not snappy-compressed)
    const filename = basename(filePath).toLowerCase();
    const isIndexable =
      filename.endsWith(".csv") ||
      filename.endsWith(".tsv") ||
      filename.endsWith(".tab") ||
      filename.endsWith(".ssv");

    if (!isIndexable) {
      return; // Not an indexable format
    }

    const stats = await stat(filePath);
    const fileSizeMB = stats.size / (1024 * 1024);
    const indexPath = filePath + ".idx";

    // Check if index already exists
    let indexExists = false;
    try {
      await access(indexPath, constants.F_OK);
      indexExists = true;
    } catch { /* ignore: index file does not exist */
      indexExists = false;
    }

    // Create index if file is large enough and not already indexed
    if (fileSizeMB > minSizeMB && !indexExists) {
      console.error(
        `[MCP Tools] File is ${fileSizeMB.toFixed(1)}MB, creating index...`,
      );

      const qsvBin = getQsvBinaryPath();
      const indexArgs = ["index", filePath];

      try {
        await runQsvWithTimeout(qsvBin, indexArgs);
        console.error(`[MCP Tools] Index created successfully: ${indexPath}`);
      } catch (error: unknown) {
        // Don't fail if indexing fails or times out - just log and continue
        console.error(
          `[MCP Tools] Index creation failed (continuing anyway):`,
          error,
        );
      }
    } else if (indexExists) {
      console.error(`[MCP Tools] Index already exists: ${indexPath}`);
    } else {
      console.error(
        `[MCP Tools] File is ${fileSizeMB.toFixed(1)}MB, skipping auto-indexing`,
      );
    }
  } catch (error: unknown) {
    console.error(
      `[MCP Tools] Auto-indexing error (continuing anyway):`,
      error,
    );
  }
}

/**
 * Determine if a command should use a temp output file
 *
 * @param command - The qsv command name
 * @param inputFile - Path to the input file
 * @returns Promise<boolean> - true if temp file should be used
 */
async function shouldUseTempFile(
  command: string,
  inputFile: string,
): Promise<boolean> {
  // Metadata commands always use stdout (small results)
  if (METADATA_COMMANDS.has(command)) {
    return false;
  }

  // TSV mode: force temp file so qsv outputs TSV natively via .tsv extension.
  // This bypasses size-based heuristics below, meaning even small outputs go through
  // temp file I/O. This is an acceptable trade-off because: (1) qsv's file I/O is fast,
  // (2) it avoids a fragile post-processing step to convert CSV→TSV in-memory, and
  // (3) it ensures consistent tab-delimited output for all tabular commands.
  if (config.outputFormat === "tsv" && !NON_TABULAR_COMMANDS.has(command)) {
    return true;
  }

  // Commands that always return full CSV data should use temp files
  if (ALWAYS_FILE_COMMANDS.has(command)) {
    return true;
  }

  // For other commands, check input file size
  try {
    const stats = await stat(inputFile);
    return stats.size > LARGE_FILE_THRESHOLD_BYTES;
  } catch (error: unknown) {
    // If we can't stat the file, default to stdout
    console.error(
      `[MCP Tools] Error checking file size for temp file decision:`,
      error,
    );
    return false;
  }
}

/**
 * 11 most essential qsv commands exposed as individual MCP tools
 * Optimized for token efficiency while maintaining high-value tool access
 *
 * Commands promoted to CORE_TOOLS (always loaded):
 * stats, index
 *
 * Commands moved to qsv_command generic tool:
 * join, sort, dedup, rename, validate, sample, template, diff, schema
 */
export const COMMON_COMMANDS = [
  "select", // Column selection (most frequently used)
  "moarstats", // Comprehensive statistics with data type inference
  "search", // Pattern-based filtering
  "frequency", // Value distribution
  "headers", // Header operations (quick discovery)
  "count", // Row counting (instant with index)
  "slice", // Row selection
  "sqlp", // SQL queries (Polars engine)
  "joinp", // High-performance joins (Polars engine)
  "cat", // Concatenate CSV files (rows/columns)
  "geocode", // Geocoding operations
] as const;

/**
 * Enhance parameter descriptions with examples and common values
 */
function enhanceParameterDescription(
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
function enhanceDescription(skill: QsvSkill): string {
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

/**
 * Convert a QSV skill to an MCP tool definition
 */
export function createToolDefinition(skill: QsvSkill): McpToolDefinition {
  const properties: Record<string, McpToolProperty> = {
    input_file: {
      type: "string",
      description:
        "Path to input CSV file. Use absolute paths for reliability.",
    },
  };

  const required: string[] = ["input_file"];

  // Add positional arguments
  if (skill.command.args && Array.isArray(skill.command.args)) {
    for (const arg of skill.command.args) {
      // Skip 'input' argument - we already have 'input_file' which maps to this
      if (arg.name === "input") {
        continue;
      }

      properties[arg.name] = {
        type: mapSchemaType(arg.type),
        description: arg.description,
      };

      // Add enum if present (for subcommands)
      if ("enum" in arg && Array.isArray(arg.enum) && arg.enum.length > 0) {
        properties[arg.name].enum = arg.enum;
      }

      if (arg.required) {
        required.push(arg.name);
      }
    }
  }

  // Add options
  if (skill.command.options && Array.isArray(skill.command.options)) {
    for (const opt of skill.command.options) {
      const optName = opt.flag.replace(/^--/, "").replace(/-/g, "_");

      if (opt.type === "flag") {
        properties[optName] = {
          type: "boolean",
          description: enhanceParameterDescription(optName, opt.description),
        };
      } else {
        properties[optName] = {
          type: mapSchemaType(opt.type),
          description: enhanceParameterDescription(optName, opt.description),
        };
        if (opt.default) {
          properties[optName].default = opt.default;
        }
      }
    }
  }

  // Add output_file (optional for all commands)
  properties.output_file = {
    type: "string",
    description:
      "Path to output CSV file (optional). Use absolute paths for reliability. For large results, a temp file is automatically used if omitted.",
  };

  // Add help flag (universally available for all qsv commands)
  properties.help = {
    type: "boolean",
    description:
      "Display detailed help text for this command (equivalent to --help flag). Returns usage documentation instead of executing the command.",
  };

  return {
    name: skill.name.replace("qsv-", "qsv_"),
    description: enhanceDescription(skill),
    inputSchema: {
      type: "object",
      properties,
      required: required.length > 0 ? required : undefined,
    },
  };
}

/**
 * Map QSV argument/option types to JSON Schema types
 * (Arguments and options share the same mapping logic)
 */
function mapSchemaType(
  type: string,
): "string" | "number" | "boolean" | "object" | "array" {
  switch (type) {
    case "number":
      return "number";
    case "file":
    case "regex":
    case "string":
    default:
      return "string";
  }
}

/**
 * Resolve input file path, handle format conversion (Excel/JSONL to CSV),
 * and auto-index large files. Returns the resolved (possibly converted) input path.
 */
async function resolveAndConvertInputFile(
  inputFile: string,
  filesystemProvider: FilesystemProviderExtended,
): Promise<string> {
  const originalInputFile = inputFile;
  inputFile = await filesystemProvider.resolvePath(inputFile);
  console.error(
    `[MCP Tools] Resolved input file: ${originalInputFile} -> ${inputFile}`,
  );

  // Check if file needs conversion (Excel or JSONL to CSV)
  if (
    isFilesystemProviderExtended(filesystemProvider) &&
    filesystemProvider.needsConversion(inputFile)
  ) {
    const conversionCmd = filesystemProvider.getConversionCommand(inputFile);
    if (!conversionCmd) {
      throw new Error(
        `Unable to determine conversion command for: ${inputFile}`,
      );
    }
    console.error(
      `[MCP Tools] File requires conversion using qsv ${conversionCmd}`,
    );

    const qsvBin = getQsvBinaryPath();

    // Generate unique converted file path with UUID to prevent collisions
    // 16 hex chars (64 bits) has 50% collision probability after ~4 billion conversions
    const uuid = randomUUID().replace(/-/g, "").substring(0, 16);
    let convertedPath = `${inputFile}.converted.${uuid}.csv`;

    // Validate the generated converted path for defense-in-depth
    try {
      convertedPath = await filesystemProvider.resolvePath(convertedPath);
    } catch (error: unknown) {
      throw new Error(
        `Invalid converted file path: ${convertedPath} - ${error}`,
      );
    }

    // Initialize converted file manager
    const workingDir = filesystemProvider.getWorkingDirectory();
    const convertedManager = new ConvertedFileManager(workingDir);

    // Clean up orphaned entries and partial conversions first
    await convertedManager.cleanupOrphanedEntries();

    // Check if we can reuse an existing converted file
    const baseName = basename(inputFile);
    const pattern = `${baseName}.converted.`;
    let validConverted: string | null = null;

    try {
      const dir = dirname(inputFile);
      const files = await readdir(dir);

      for (const file of files) {
        if (file.startsWith(pattern) && file.endsWith(".csv")) {
          const filePath = join(dir, file);
          validConverted =
            await convertedManager.getValidConvertedFile(
              inputFile,
              filePath,
            );
          if (validConverted) break;
        }
      }
    } catch (error: unknown) {
      console.error(
        "[MCP Tools] Error searching for existing converted file:",
        error,
      );
    }

    if (validConverted) {
      await convertedManager.touchConvertedFile(inputFile);
      inputFile = validConverted;
      console.error(
        `[MCP Tools] Reusing existing conversion: ${validConverted}`,
      );
    } else {
      await convertedManager.registerConversionStart(
        inputFile,
        convertedPath,
      );

      try {
        const conversionArgs = buildConversionArgs(
          conversionCmd,
          inputFile,
          convertedPath,
        );
        console.error(
          `[MCP Tools] Running conversion: ${qsvBin} ${conversionArgs.join(" ")}`,
        );

        await runQsvWithTimeout(qsvBin, conversionArgs);

        await convertedManager.registerConvertedFile(
          inputFile,
          convertedPath,
        );
        await convertedManager.registerConversionComplete(inputFile);
        inputFile = convertedPath;
        console.error(
          `[MCP Tools] Conversion successful: ${convertedPath}`,
        );

        await autoIndexIfNeeded(convertedPath);
      } catch (conversionError) {
        try {
          await unlink(convertedPath);
          console.error(
            `[MCP Tools] Cleaned up partial conversion file: ${convertedPath}`,
          );
        } catch { /* ignore: cleanup */
        }
        convertedManager.trackConversionFailure();
        throw conversionError;
      }
    }
  }

  return inputFile;
}

/**
 * Build enhanced file-not-found error message with file suggestions
 */
async function buildFileNotFoundError(
  inputFile: string,
  error: unknown,
  filesystemProvider: FilesystemProviderExtended,
): Promise<string> {
  let errorMessage = `Error resolving file path: ${error instanceof Error ? error.message : String(error)}`;

  // Detect Cowork VM paths that won't resolve on the host
  if (/^\/sessions\/|^\/home\/user\/|^\/tmp\/cowork-|^\/workspace\//.test(inputFile)) {
    const workingDir = filesystemProvider.getWorkingDirectory();
    errorMessage =
      `This path appears to be a Cowork VM path. qsv runs on the host machine.\n` +
      `Current qsv working directory: ${workingDir}\n` +
      `Use filenames relative to this directory, or call qsv_set_working_dir first.\n\n` +
      errorMessage;
  }

  const errorStr = error instanceof Error ? error.message : String(error);
  if (
    errorStr.includes("outside allowed") ||
    errorStr.includes("not exist") ||
    errorStr.includes("cannot access") ||
    errorStr.includes("ENOENT")
  ) {
    try {
      const { resources } = await filesystemProvider.listFiles(
        undefined,
        false,
      );

      if (resources.length > 0) {
        const suggestions = findSimilarFiles(inputFile, resources, 3);

        errorMessage += "\n\n";

        if (
          suggestions.length > 0 &&
          suggestions[0].distance <= inputFile.length / 2
        ) {
          errorMessage += "Did you mean one of these?\n";
          suggestions.forEach(({ name }) => {
            errorMessage += `  - ${name}\n`;
          });
        } else {
          errorMessage += `Available files in working directory (${filesystemProvider.getWorkingDirectory()}):\n`;
          resources.slice(0, 5).forEach((file) => {
            errorMessage += `  - ${file.name}\n`;
          });

          if (resources.length > 5) {
            errorMessage += `  ... and ${resources.length - 5} more file${resources.length - 5 !== 1 ? "s" : ""}`;
          }
        }
      }
    } catch (listError) {
      console.error(
        `[MCP Tools] Failed to list files for suggestions:`,
        listError,
      );
    }
  }

  return errorMessage;
}

/**
 * Build args and options for skill execution from MCP tool params
 */
function buildSkillExecParams(
  skill: QsvSkill,
  params: Record<string, unknown>,
  inputFile: string | undefined,
  outputFile: string | undefined,
  isHelpRequest: boolean,
): { args: Record<string, unknown>; options: Record<string, unknown> } {
  const args: Record<string, unknown> = {};
  const options: Record<string, unknown> = {};

  // Add input file as 'input' argument if the skill expects it
  if (skill.command.args.some((a) => a.name === "input")) {
    args.input = inputFile;
    console.error(`[MCP Tools] Added input arg: ${inputFile}`);
  }

  // Check whether a skill declares a distinct --input or --output CLI option.
  // If so, allow the key through instead of treating it as a file alias.
  // Note: only checks long-form options (--input/--output), not short flags (-i/-o).
  // A positional arg named "input" is already consumed above as the input file.
  const skillHasOption = (name: string) =>
    skill.command.options.some((o) => o.flag === `--${name}`);

  for (const [key, value] of Object.entries(params)) {
    // Skip meta-parameters that are handled separately (not passed as CLI flags).
    // "input" and "output" are aliases for input_file/output_file resolved by
    // resolveParamAliases, so skip them unless the skill declares them as
    // distinct CLI flags (i.e. the skill has an arg or option named "input"/"output").
    if (
      key === "input_file" ||
      key === "output_file" ||
      (key === "input" && !skillHasOption("input")) ||
      (key === "output" && !skillHasOption("output")) ||
      key === "help"
    ) {
      continue;
    }

    const isArg = skill.command.args.some((a) => a.name === key);
    if (isArg) {
      args[key] = value;
    } else {
      const optFlag = key.startsWith("-")
        ? key
        : `--${key.replace(/_/g, "-")}`;
      options[optFlag] = value;
    }
  }

  if (outputFile) {
    options["--output"] = outputFile;
  }

  if (isHelpRequest) {
    options["help"] = true;
  }

  return { args, options };
}

/**
 * Format successful tool result, handling temp files and performance tips
 */
async function formatToolResult(
  result: import("./types.js").SkillResult,
  commandName: string,
  inputFile: string | undefined,
  outputFile: string | undefined,
  autoCreatedTempFile: boolean,
  params: Record<string, unknown>,
) {
  let responseText = "";

  if (outputFile) {
    if (autoCreatedTempFile) {
      try {
        const tempFileStats = await stat(outputFile);

        if (tempFileStats.size > MAX_MCP_RESPONSE_SIZE) {
          console.error(
            `[MCP Tools] Output file (${formatBytes(tempFileStats.size)}) exceeds MCP response limit (${formatBytes(MAX_MCP_RESPONSE_SIZE)})`,
          );

          const timestamp = new Date()
            .toISOString()
            .replace(/[:.]/g, "-")
            .replace("T", "_")
            .split(".")[0];
          const savedExt = config.outputFormat === "tsv" && !NON_TABULAR_COMMANDS.has(commandName) && !isBinaryOutputFormat(commandName, params) ? "tsv" : "csv";
          const savedFileName = `qsv-${commandName}-${timestamp}.${savedExt}`;
          const savedPath = join(config.workingDir, savedFileName);

          try {
            await rename(outputFile, savedPath);
          } catch (renameErr: unknown) {
            // Cross-device rename fails with EXDEV; fall back to copy + delete
            if (renameErr instanceof Error && "code" in renameErr && (renameErr as NodeJS.ErrnoException).code === "EXDEV") {
              await copyFile(outputFile, savedPath);
              await unlink(outputFile);
            } else {
              throw renameErr;
            }
          }
          console.error(`[MCP Tools] Saved large output to: ${savedPath}`);

          responseText = `✅ Large output saved to file (too large to display in chat)\n\n`;
          responseText += `File: ${savedFileName}\n`;
          responseText += `Location: ${config.workingDir}\n`;
          responseText += `Size: ${formatBytes(tempFileStats.size)}\n`;
          responseText += `Duration: ${result.metadata.duration}ms\n\n`;
          responseText += `The file is now available in your working directory and can be processed with additional qsv commands.`;
        } else {
          console.error(
            `[MCP Tools] Output file (${formatBytes(tempFileStats.size)}) is small enough to return directly`,
          );
          const fileContents = await readFile(outputFile, "utf-8");

          try {
            await unlink(outputFile);
            console.error(`[MCP Tools] Deleted temp file: ${outputFile}`);
          } catch (unlinkError) {
            console.error(
              `[MCP Tools] Failed to delete temp file:`,
              unlinkError,
            );
          }

          responseText = fileContents;
        }
      } catch (readError) {
        console.error(
          `[MCP Tools] Failed to process temp file:`,
          readError,
        );
        return errorResult(`Error processing output from temp file: ${readError instanceof Error ? readError.message : String(readError)}`);
      }
    } else {
      responseText = `Successfully wrote output to: ${outputFile}\n\n`;
      responseText += `Metadata:\n`;
      responseText += `- Command: ${result.metadata.command}\n`;
      responseText += `- Duration: ${result.metadata.duration}ms\n`;
      if (result.metadata.rowsProcessed) {
        responseText += `- Rows processed: ${result.metadata.rowsProcessed}\n`;
      }
    }
  } else {
    responseText = result.output;
  }

  // sqlp note: CSV inputs are now auto-converted to Parquet before SQL execution
  // No manual conversion tip needed — ensureParquet() handles it in handleToolCall()

  // moarstats bivariate output notification (skip for help requests where inputFile is undefined)
  if (commandName === "moarstats" && params.bivariate && inputFile) {
    const inputDir = dirname(inputFile as string);
    const inputStem = basename(inputFile as string).replace(/\.[^.]+$/, "");
    const bivariateFileName = params.join_inputs
      ? `${inputStem}.stats.bivariate.joined.csv`
      : `${inputStem}.stats.bivariate.csv`;
    const bivariatePath = join(inputDir, bivariateFileName);

    responseText += `\n\n📊 Bivariate statistics were written to a SEPARATE file:\n`;
    responseText += `File: ${bivariateFileName}\n`;
    responseText += `Location: ${bivariatePath}\n`;
    responseText += `Use qsv_command or read this file to view the bivariate correlation results.`;
  }

  return successResult(responseText);
}

/** In-flight Parquet conversions keyed by CSV path — prevents duplicate concurrent work */
const parquetConversionLocks = new Map<string, Promise<string>>();

/**
 * Check if a file is a CSV-like format that can be converted to Parquet
 */
export function isCsvLikeFile(filePath: string): boolean {
  const lower = basename(filePath).toLowerCase();
  return CSV_LIKE_EXTENSIONS.some((ext) => lower.endsWith(ext));
}

/**
 * Get the Parquet path for a CSV-like file (same stem, .parquet extension).
 * For non-CSV files (e.g., `.json`), appends `.parquet` (e.g., `test.json.parquet`).
 * Callers should gate on `isCsvLikeFile()` to avoid surprising double-extensions.
 */
// CSV_LIKE_EXTENSIONS sorted by descending length so longer extensions
// (e.g., ".csv.sz") are matched before shorter ones (e.g., ".csv").
// Sorted once at module level to avoid re-sorting on every call.
const ORDERED_CSV_EXTENSIONS = [...CSV_LIKE_EXTENSIONS].sort(
  (a, b) => b.length - a.length,
);

export function getParquetPath(csvPath: string): string {
  // Match against lowercased basename to avoid false matches on directory names
  // (e.g., /data/csv_files/test.json). Slicing from original csvPath is safe because
  // ext and the actual extension have the same length regardless of case.
  const base = basename(csvPath).toLowerCase();
  for (const ext of ORDERED_CSV_EXTENSIONS) {
    if (base.endsWith(ext)) {
      return csvPath.slice(0, -ext.length) + ".parquet";
    }
  }
  return csvPath + ".parquet";
}

/**
 * Ensure a Parquet file exists for a CSV input.
 * If a .parquet file with the same stem already exists and is newer than the CSV, use it.
 * Otherwise, auto-convert using the same 3-step pipeline as handleToParquetCall():
 *   1. Generate stats cache
 *   2. Generate Polars schema
 *   3. Convert to Parquet
 *
 * Returns the Parquet file path, or the original path if not a CSV-like file.
 */
export async function ensureParquet(inputFile: string): Promise<string> {
  // Only convert CSV-like files (non-CSV files like .parquet, .jsonl are returned as-is)
  if (!isCsvLikeFile(inputFile)) {
    return inputFile;
  }

  // Check lock map BEFORE any async operations to avoid TOCTOU race
  const existing = parquetConversionLocks.get(inputFile);
  if (existing) {
    console.error(`[MCP Tools] ensureParquet: Waiting on in-flight conversion for: ${inputFile}`);
    return existing;
  }

  // Establish the lock (conversion promise) BEFORE any awaited operations to prevent
  // race where two concurrent calls both pass the lock check above, then both start
  // conversions. By setting the promise synchronously, the second caller will always
  // find the lock and await the first caller's promise.
  const parquetPath = getParquetPath(inputFile);
  const conversionPromise = (async () => {
    // Check if Parquet already exists and is newer than CSV
    try {
      const csvStats = await stat(inputFile);
      const parquetStats = await stat(parquetPath);
      if (parquetStats.mtimeMs >= csvStats.mtimeMs) {
        console.error(`[MCP Tools] ensureParquet: Using existing Parquet file (up-to-date): ${parquetPath}`);
        return parquetPath;
      }
    } catch (error: unknown) {
      console.warn(`[MCP Tools] Parquet stat check failed (will convert): ${parquetPath}`, error);
    }

    return await doParquetConversion(inputFile, parquetPath);
  })();
  parquetConversionLocks.set(inputFile, conversionPromise);
  try {
    return await conversionPromise;
  } finally {
    parquetConversionLocks.delete(inputFile);
  }
}

async function doParquetConversion(inputFile: string, parquetPath: string): Promise<string> {
  console.error(`[MCP Tools] ensureParquet: Auto-converting CSV to Parquet: ${inputFile}`);
  const qsvBin = getQsvBinaryPath();
  const statsFile = inputFile + ".stats.csv";
  const schemaFile = inputFile + ".pschema.json";

  // Validate input file exists before attempting conversion
  let inputFileStats;
  try {
    inputFileStats = await stat(inputFile);
  } catch (error: unknown) {
    console.warn(`[MCP Tools] Input file stat failed: ${inputFile}`, error);
    throw new Error(`Input file not found: ${inputFile}`);
  }

  // Check if stats/schema need regeneration
  let needStats = true;
  let needSchema = true;
  try {
    const existingStats = await stat(statsFile);
    if (existingStats.mtimeMs >= inputFileStats.mtimeMs) needStats = false;
  } catch { /* ignore: needs stats generation */ }
  try {
    const existingSchema = await stat(schemaFile);
    if (existingSchema.mtimeMs >= inputFileStats.mtimeMs) needSchema = false;
  } catch { /* ignore: needs schema generation */ }

  // Step 1: Generate stats cache
  if (needStats) {
    try {
      const statsArgs = [
        "stats", inputFile,
        "--cardinality", "--stats-jsonl",
        "--infer-dates", "--dates-whitelist", "sniff",
      ];
      await runQsvWithTimeout(qsvBin, statsArgs);
    } catch (error: unknown) {
      throw new Error(`Stats generation failed for ${inputFile}: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  // Step 2: Generate Polars schema
  if (needSchema) {
    try {
      const schemaArgs = ["schema", "--polars", inputFile];
      await runQsvWithTimeout(qsvBin, schemaArgs);
    } catch (error: unknown) {
      throw new Error(`Schema generation failed for ${inputFile}: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  // Step 2.5: Patch schema for AM/PM date formats that Polars can't parse
  await patchSchemaAndLog(inputFile, schemaFile);

  // Step 3: Convert to Parquet
  try {
    const conversionArgs = buildConversionArgs("csv-to-parquet", inputFile, parquetPath);
    await runQsvWithTimeout(qsvBin, conversionArgs);
  } catch (error: unknown) {
    // Clean up potentially corrupted partial Parquet file
    try { await unlink(parquetPath); } catch { /* ignore: cleanup */ }
    throw new Error(`Parquet conversion failed for ${inputFile}: ${error instanceof Error ? error.message : String(error)}`);
  }

  // Verify the output file was actually created and is non-empty
  let outStats;
  try {
    outStats = await stat(parquetPath);
  } catch (error: unknown) {
    console.warn(`[MCP Tools] Output file stat failed after conversion: ${parquetPath}`, error);
    throw new Error(`Parquet conversion completed but output file not found: ${parquetPath}`);
  }
  if (outStats.size === 0) {
    try { await unlink(parquetPath); } catch { /* ignore: cleanup */ }
    throw new Error(`Parquet conversion produced an empty file: ${parquetPath}`);
  }

  console.error(`[MCP Tools] ensureParquet: Successfully converted to ${parquetPath}`);
  return parquetPath;
}

/**
 * Suggest fixes for common DuckDB SQL errors.
 * Pattern-matches stderr to provide actionable guidance.
 */
function suggestDuckDbFixes(stderr: string, sql?: string): string {
  const suggestions: string[] = [];
  const lower = stderr.toLowerCase();

  if (lower.includes("syntax error")) {
    suggestions.push("- Check for trailing commas before FROM/WHERE/GROUP BY clauses");
    suggestions.push("- Verify all parentheses are matched");
    suggestions.push("- Ensure string literals use single quotes, not double quotes");
    // Query-aware: detect trailing commas in the SQL itself
    if (sql && /,\s*(?:FROM|WHERE|GROUP\s+BY|ORDER\s+BY|HAVING|LIMIT)\b/i.test(sql)) {
      suggestions.push("- ⚠ Trailing comma detected before a SQL keyword in your query");
    }
    // Only flag double-quoted tokens when the error mentions an unresolved name
    // that matches one, suggesting the user intended a string literal, not an identifier.
    if (sql) {
      const doubleQuoted = sql.match(/"([^"]+)"/g);
      if (doubleQuoted) {
        const errorMentionsQuotedToken = doubleQuoted.some((token) => {
          const bare = token.replace(/"/g, "");
          return lower.includes(bare.toLowerCase());
        });
        if (errorMentionsQuotedToken) {
          suggestions.push(
            "- ⚠ A double-quoted identifier in your query matches the error — " +
              "DuckDB treats double quotes as identifiers; use single quotes for string literals",
          );
        }
      }
    }
  }

  if (lower.includes("column") && lower.includes("not found")) {
    suggestions.push("- Column names are case-sensitive in DuckDB — use qsv_headers to check exact column names");
    suggestions.push("- Use double quotes around column names with spaces or special characters");
  }

  if (lower.includes("conversion error") || lower.includes("could not parse") || lower.includes("could not convert")) {
    suggestions.push("- Type mismatch detected — use TRY_CAST(column AS type) instead of CAST to handle invalid values gracefully");
    suggestions.push("- Check column types with qsv_stats to verify expected data types");
  }

  if (lower.includes("binder error")) {
    suggestions.push("- Verify table alias is correct — the default table alias is _t_1");
    suggestions.push("- Check that all referenced column names exist using qsv_headers");
  }

  return suggestions.join("\n");
}

/**
 * Try executing a SQL query via DuckDB.
 *
 * Returns a formatted tool result if DuckDB handles the query,
 * or null if DuckDB is unavailable or the format is unsupported (fall through to sqlp).
 *
 * SQL errors from DuckDB are returned to the agent (no silent fallback).
 * Binary-level failures (exit code 127, ENOENT) mark DuckDB unavailable and return null.
 *
 * TRUST ASSUMPTION: The `sql` parameter comes from the MCP agent (a trusted caller).
 * File paths are escaped against injection, but the SQL string itself is passed through
 * to `duckdb -c` without sanitization. Do not expose this to untrusted input.
 */
async function tryDuckDbExecution(
  sql: string,
  parquetFile: string,
  params: Record<string, unknown>,
  outputFile: string | undefined,
): Promise<{ content: Array<{ type: string; text: string }>; isError?: boolean } | null> {
  // Detect DuckDB (lazy, first-call-only)
  const state = detectDuckDb();
  if (state.status !== "available") {
    return null; // Fall through to sqlp
  }

  const format = String(params.format ?? "csv").toLowerCase();

  // Unsupported formats fall back to sqlp
  if (format === "arrow" || format === "avro") {
    console.error(`[MCP Tools] DuckDB: format '${format}' not supported, falling back to sqlp`);
    return null;
  }

  // Translate SQL: _t_1 → read_parquet/read_csv
  const translatedSql = translateSql(sql, parquetFile, {
    delimiter: params.delimiter as string | undefined,
    rnullValues: params["rnull-values"] as string | undefined,
  });

  console.error(`[MCP Tools] DuckDB: Executing translated SQL: ${translatedSql}`);

  try {
    const result = await executeDuckDbQuery(translatedSql, {
      format,
      outputFile,
      decimalComma: params["decimal-comma"] === true,
      compression: params.compression as string | undefined,
      timeoutMs: config.operationTimeoutMs,
      onSpawn: (proc) => activeProcesses.add(proc),
      onExit: (proc) => activeProcesses.delete(proc),
    });

    // null means unsupported format — fall through
    if (result === null) {
      return null;
    }

    // Binary-level failure
    if (result.exitCode === 127) {
      markDuckDbUnavailable("DuckDB binary returned exit code 127");
      console.error(`[MCP Tools] DuckDB: Binary failure (exit 127), falling back to sqlp`);
      return null;
    }

    // SQL error — return to agent with enhanced diagnostics (no silent fallback)
    if (result.exitCode !== 0) {
      const suggestions = suggestDuckDbFixes(result.stderr, translatedSql);
      return errorResult(
        `🦆 Engine: DuckDB v${result.version}\n\n` +
        `Error executing SQL:\n${result.stderr}\n\n` +
        `SQL query:\n${translatedSql}` +
        (suggestions ? `\n\n💡 Suggestions:\n${suggestions}` : ""),
      );
    }

    // Success — prepend engine indicator
    const engineHeader = `🦆 Engine: DuckDB v${result.version}\n\n`;
    return successResult(engineHeader + result.output);
  } catch (error: unknown) {
    // ENOENT or similar binary-level error
    const errMsg = error instanceof Error ? error.message : String(error);
    if (
      errMsg.includes("ENOENT") ||
      errMsg.includes("not found") ||
      errMsg.includes("spawn")
    ) {
      markDuckDbUnavailable(`Binary error: ${errMsg}`);
      console.error(`[MCP Tools] DuckDB: Binary error, falling back to sqlp: ${errMsg}`);
      return null;
    }
    // Other errors — return to agent
    return errorResult(`🦆 Engine: DuckDB\n\nUnexpected error: ${errMsg}`);
  }
}

/**
 * Resolve LLM-friendly parameter aliases to their canonical names.
 * LLMs sometimes send "input"/"output" instead of "input_file"/"output_file".
 * Canonical names take precedence when both are present.
 */
function resolveParamAliases(params: Record<string, unknown>): {
  inputFile: string | undefined;
  outputFile: string | undefined;
} {
  // Only accept string values; reject numbers, booleans, objects, etc.
  // A numeric value (e.g. { input: 42 }) is almost certainly an LLM mistake,
  // not a valid file path, so we return undefined rather than coercing to "42".
  const coerce = (v: unknown, paramName: string): string | undefined => {
    if (typeof v === "string") return v.trim() || undefined;
    if (v != null) {
      console.error(
        `[MCP Tools] Ignoring non-string ${paramName} value: ${typeof v} (${JSON.stringify(v)})`,
      );
    }
    return undefined;
  };

  const inputFile =
    coerce(params.input_file, "input_file") ??
    coerce(params.input, "input");
  const outputFile =
    coerce(params.output_file, "output_file") ??
    coerce(params.output, "output");
  return { inputFile, outputFile };
}

/**
 * Handle execution of a qsv tool
 */
export async function handleToolCall(
  toolName: string,
  params: Record<string, unknown>,
  executor: SkillExecutor,
  loader: SkillLoader,
  filesystemProvider?: FilesystemProviderExtended,
) {
  // Acquire concurrency slot (queue if all slots are busy)
  const acquired = await acquireSlot(config.concurrencyWaitTimeoutMs);
  if (!acquired) {
    return errorResult(
      `Operation queued but timed out after ${Math.round(config.concurrencyWaitTimeoutMs / 1000)}s waiting for a slot. ` +
      `${activeOperationCount} operation${activeOperationCount !== 1 ? "s" : ""} still running. ` +
      `Try running operations sequentially.`,
    );
  }

  try {
    // Extract command name from tool name (qsv_select -> select)
    const commandName = toolName.replace("qsv_", "");

    // Load the skill
    const skillName = `qsv-${commandName}`;
    const skill = await loader.load(skillName);

    if (!skill) {
      const totalCommands = loader.getStats().total;
      const remainingCommands = totalCommands - COMMON_COMMANDS.length;

      return errorResult(
        `Error: Skill '${skillName}' not found.\n\n` +
        `Please verify the command name is correct. ` +
        `Available commands include: ${COMMON_COMMANDS.join(", ")}, and ${remainingCommands} others. ` +
        `Use 'qsv_command' with the 'command' parameter for less common commands.`,
      );
    }

    // Extract input_file and output_file (with LLM alias resolution)
    const { inputFile: rawInputFile, outputFile: rawOutputFile } = resolveParamAliases(params);
    let inputFile = rawInputFile;
    let outputFile = rawOutputFile;
    const isHelpRequest = params.help === true;

    if (!inputFile && !isHelpRequest) {
      return errorResult("Error: input_file parameter is required (unless using help=true to view command documentation)");
    }

    // Resolve file paths using filesystem provider if available (skip for help requests)
    if (filesystemProvider && inputFile) {
      try {
        inputFile = await resolveAndConvertInputFile(inputFile, filesystemProvider);

        // Auto-index native CSV files (skip for help requests)
        if (!isHelpRequest) {
          await autoIndexIfNeeded(inputFile);
        }

        if (outputFile) {
          const originalOutputFile = outputFile;
          outputFile = await filesystemProvider.resolvePath(outputFile);
          console.error(
            `[MCP Tools] Resolved output file: ${originalOutputFile} -> ${outputFile}`,
          );
        }
      } catch (error: unknown) {
        console.error(`[MCP Tools] Error resolving file path:`, error);
        const errorMessage = await buildFileNotFoundError(
          inputFile,
          error,
          filesystemProvider,
        );
        return errorResult(errorMessage);
      }
    }

    // DuckDB/Parquet-first interception for sqlp queries
    let parquetConversionWarning = "";
    if (commandName === "sqlp" && !isHelpRequest && inputFile) {
      const rawSql = params.sql as string | undefined;
      if (rawSql) {
        // Normalize uppercase _T_N references to lowercase _t_N for consistent handling
        const sql = normalizeTableRefs(rawSql);
        params.sql = sql;
        try {
          // Skip DuckDB for multi-table queries (_t_2, _t_3, etc.) — sqlp handles
          // multiple input files natively, so let the original input flow through.
          // NOTE: Multi-table queries don't benefit from Parquet auto-conversion;
          // users should manually convert all input files with qsv_to_parquet first.
          if (MULTI_TABLE_PATTERN.test(sql)) {
            console.error(`[MCP Tools] DuckDB: Multi-table query detected (_t_2+), falling back to sqlp`);
          } else {
            // Auto-convert CSV to Parquet (skip for SKIP_INPUT which already has explicit refs)
            let parquetFile = inputFile;
            if (inputFile !== "SKIP_INPUT") {
              parquetFile = await ensureParquet(inputFile);
            }

            // Try DuckDB execution (single-table path)
            const duckDbResult = await tryDuckDbExecution(sql, parquetFile, params, outputFile);
            if (duckDbResult !== null) {
              return duckDbResult;
            }

            // DuckDB unavailable or unsupported format — fall through to sqlp
            // If we converted to Parquet, rewrite SQL via translateSql and use SKIP_INPUT
            if (parquetFile !== inputFile && parquetFile.endsWith(".parquet")) {
              const rewrittenSql = translateSql(sql, parquetFile);
              params.sql = rewrittenSql;
              inputFile = "SKIP_INPUT";
              console.error(`[MCP Tools] sqlp fallback with Parquet: ${rewrittenSql}`);
            }
          }
        } catch (error: unknown) {
          // Parquet conversion or DuckDB failed — warn and fall through to sqlp with original input
          const errorMsg = error instanceof Error ? error.message : String(error);
          console.error(
            `[MCP Tools] Parquet/DuckDB interception failed, falling back to sqlp:`,
            errorMsg,
          );
          // Include warning in result so the agent/user knows the optimization was skipped
          parquetConversionWarning = `[Warning] Parquet auto-conversion was skipped (${errorMsg}). Query ran against original CSV which may be slower.`;
        }
      }
    }

    // Determine if we should use a temp file for output (skip for help requests
    // and binary output formats like parquet/arrow/avro which can't be read as UTF-8)
    let autoCreatedTempFile = false;
    if (
      !outputFile &&
      !isHelpRequest &&
      inputFile &&
      !isBinaryOutputFormat(commandName, params) &&
      (await shouldUseTempFile(commandName, inputFile))
    ) {
      const tempExt = config.outputFormat === "tsv" && !NON_TABULAR_COMMANDS.has(commandName) ? "tsv" : "csv";
      const tempFileName = `qsv-output-${randomUUID()}.${tempExt}`;
      outputFile = join(tmpdir(), tempFileName);
      autoCreatedTempFile = true;
      console.error(`[MCP Tools] Auto-created temp output file: ${outputFile}`);
    }

    // Build execution parameters
    const { args, options } = buildSkillExecParams(
      skill,
      params,
      inputFile,
      outputFile,
      isHelpRequest,
    );

    console.error(
      `[MCP Tools] Executing skill with args:`,
      JSON.stringify(args),
    );
    console.error(
      `[MCP Tools] Executing skill with options:`,
      JSON.stringify(options),
    );

    // Execute the skill
    const result = await executor.execute(skill, { args, options });

    // Format and return result
    if (result.success) {
      const formattedResult = await formatToolResult(
        result,
        commandName,
        inputFile,
        outputFile,
        autoCreatedTempFile,
        params,
      );
      // Prepend Parquet conversion warning if any
      if (parquetConversionWarning) {
        if (formattedResult.content?.[0]?.type === "text") {
          formattedResult.content[0].text = parquetConversionWarning + "\n\n" + formattedResult.content[0].text;
        } else {
          console.error(`[MCP Tools] Could not prepend Parquet warning to result: unexpected content structure`);
        }
      }
      return formattedResult;
    } else {
      const cmdLine = result.metadata?.command ? `\nCommand: ${result.metadata.command}` : "";
      const errorMsg = parquetConversionWarning
        ? `${parquetConversionWarning}\n\nError executing ${commandName}:\n${result.stderr}${cmdLine}`
        : `Error executing ${commandName}:\n${result.stderr}${cmdLine}`;
      return errorResult(errorMsg);
    }
  } catch (error: unknown) {
    return errorResult(`Unexpected error: ${error instanceof Error ? error.message : String(error)}`);
  } finally {
    releaseSlot();
  }
}

/**
 * Handle execution of the generic qsv_command tool
 */
export async function handleGenericCommand(
  params: Record<string, unknown>,
  executor: SkillExecutor,
  loader: SkillLoader,
  filesystemProvider?: FilesystemProviderExtended,
) {
  try {
    const commandName = params.command as string | undefined;

    if (!commandName) {
      return errorResult("Error: command parameter is required");
    }

    // Flatten nested args and options objects into the params
    // This handles cases where Claude passes:
    // {"command": "luau", "args": {...}, "options": {...}, "input_file": "...", "output_file": "..."}
    const flattenedParams: Record<string, unknown> = {};

    // Copy top-level params (except 'args' and 'options')
    for (const [key, value] of Object.entries(params)) {
      if (key !== "args" && key !== "options") {
        flattenedParams[key] = value;
      }
    }

    // Flatten nested 'args' object
    if (params.args && typeof params.args === "object") {
      const argsObj = params.args as Record<string, unknown>;
      for (const [key, value] of Object.entries(argsObj)) {
        flattenedParams[key] = value;
      }
    }

    // Flatten nested 'options' object
    if (params.options && typeof params.options === "object") {
      const optionsObj = params.options as Record<string, unknown>;
      for (const [key, value] of Object.entries(optionsObj)) {
        flattenedParams[key] = value;
      }
    }

    console.error(
      `[handleGenericCommand] Flattened params:`,
      JSON.stringify(flattenedParams),
    );

    // Forward to handleToolCall with the qsv_ prefix and flattened params
    return await handleToolCall(
      `qsv_${commandName}`,
      flattenedParams,
      executor,
      loader,
      filesystemProvider,
    );
  } catch (error: unknown) {
    return errorResult(`Unexpected error: ${error instanceof Error ? error.message : String(error)}`);
  }
}

/**
 * Create the generic qsv_command tool definition
 */
export function createGenericToolDefinition(
  loader: SkillLoader,
): McpToolDefinition {
  // Calculate remaining commands dynamically
  const totalCommands = loader.getStats().total;
  const remainingCommands = totalCommands - COMMON_COMMANDS.length;

  return {
    name: "qsv_command",
    description: `Execute any qsv command not exposed as a dedicated tool (${remainingCommands} additional commands available).

Common commands via this tool: join, sort, dedup, rename, validate, sample, template, diff, schema, and 30+ more.

❓ HELP: For any command details, use options={"--help": true}. Example: command="sort", options={"--help": true}`,
    inputSchema: {
      type: "object",
      properties: {
        command: {
          type: "string",
          description:
            'The qsv command to execute (e.g., "sort", "sample", "partition")',
        },
        input_file: {
          type: "string",
          description: "Path to input CSV file (absolute or relative)",
        },
        args: {
          type: "object",
          description: "Command arguments as key-value pairs",
        },
        options: {
          type: "object",
          description: "Command options as key-value pairs",
        },
        output_file: {
          type: "string",
          description:
            "Path to output CSV file (optional). For large results or data transformation commands, a temp file is automatically used if omitted.",
        },
      },
      required: ["command", "input_file"],
    },
  };
}

/**
 * Create qsv_list_files tool definition
 */
export function createListFilesTool(): McpToolDefinition {
  return {
    name: "qsv_list_files",
    description: `List tabular data files in a directory for browsing and discovery.

💡 USE WHEN:
- User asks "what files do I have?" or "what's in my Downloads folder?"
- Starting a session and need to discover available datasets
- User mentions a directory but not a specific file
- Verifying files exist before processing

🔍 SHOWS: File name, size, format type, last modified date.

📂 SUPPORTED FORMATS:
- **Native CSV**: .csv, .tsv, .tab, .ssv (and .sz snappy-compressed)
- **Excel** (auto-converts): .xls, .xlsx, .xlsm, .xlsb, .ods
- **JSONL** (auto-converts): .jsonl, .ndjson

🚀 WORKFLOW: Always list files first when user mentions a directory. This helps you:
1. See what files are available
2. Get exact file names (avoid typos)
3. Check file sizes (prepare for large files)
4. Identify file formats (know if conversion needed)

💡 TIP: Use non-recursive (default) for faster listing, recursive when searching subdirectories.`,
    inputSchema: {
      type: "object",
      properties: {
        directory: {
          type: "string",
          description:
            "Directory path (absolute or relative to working directory). Omit to use current working directory.",
        },
        recursive: {
          type: "boolean",
          description:
            "Scan subdirectories recursively (default: false). Enable for deep directory searches. May be slow for large directory trees.",
        },
      },
    },
  };
}

/**
 * Create qsv_set_working_dir tool definition
 */
export function createSetWorkingDirTool(): McpToolDefinition {
  return {
    name: "qsv_set_working_dir",
    description: `Change the working directory for all subsequent file operations.

💡 USE WHEN:
- User says "work with files in my Downloads folder"
- Switching between different data directories
- User provides directory path without specific file
- Setting up environment for multiple file operations

⚙️  BEHAVIOR:
- All relative file paths resolved from this directory
- Affects: qsv_list_files, all qsv commands with input_file
- Persists for entire session (until changed again)
- Validates directory exists and is accessible
- Pass "auto" as directory to re-enable automatic root-based sync

🔒 SECURITY: Only allowed directories can be set (configured in server settings).

💡 TIP: Set working directory once at session start, then use simple filenames like "data.csv" instead of full paths.`,
    inputSchema: {
      type: "object",
      properties: {
        directory: {
          type: "string",
          description:
            "New working directory path (absolute or relative). Must be within allowed directories for security.",
        },
      },
      required: ["directory"],
    },
  };
}

/**
 * Create qsv_get_working_dir tool definition
 */
export function createGetWorkingDirTool(): McpToolDefinition {
  return {
    name: "qsv_get_working_dir",
    description: `Get the current working directory path.

💡 USE WHEN:
- Confirming where files will be read from/written to
- User asks "where am I working?" or "what's my current directory?"
- Debugging file path issues
- Verifying working directory before operations

📍 RETURNS: Absolute path to current working directory.

💡 TIP: Call this after qsv_set_working_dir to confirm the change succeeded.`,
    inputSchema: {
      type: "object",
      properties: {},
    },
  };
}

/**
 * Create qsv_config tool definition
 */
export function createConfigTool(): McpToolDefinition {
  return {
    name: "qsv_config",
    description:
      "Display current qsv configuration (binary path, version, working directory, etc.)",
    inputSchema: {
      type: "object",
      properties: {},
      required: [],
    },
  };
}

/**
 * Handle qsv_config tool call
 */
export async function handleConfigTool(
  filesystemProvider?: FilesystemProviderExtended,
): Promise<{ content: Array<{ type: string; text: string }> }> {
  const validation = config.qsvValidation;
  const extensionMode = config.isExtensionMode;

  let configText = `# qsv Configuration\n\n`;

  // qsv Binary Information
  configText += `## qsv Binary\n\n`;
  if (validation.valid) {
    configText += `✅ **Status:** Validated\n`;
    configText += `📍 **Path:** \`${validation.path}\`\n`;
    configText += `🏷️ **Version:** ${validation.version}\n`;
    if (validation.commandCount) {
      configText += `🔧 **Available Commands:** ${validation.commandCount}\n`;
    }
    if (validation.totalMemory) {
      configText += `💾 **System Total Memory:** ${validation.totalMemory}\n`;
    }
  } else {
    configText += `❌ **Status:** Validation Failed\n`;
    configText += `⚠️ **Error:** ${validation.error}\n`;

    // Show auto-detection diagnostics
    const diagnostics = getDetectionDiagnostics();
    if (diagnostics.whichAttempted) {
      configText += `\n### 🔍 Auto-Detection Diagnostics\n\n`;

      // Show which/where attempt
      configText += `**PATH search (which/where):**\n`;
      if (diagnostics.whichResult) {
        configText += `✅ Found: \`${diagnostics.whichResult}\`\n\n`;
      } else if (diagnostics.whichError) {
        configText += `❌ Failed: ${diagnostics.whichError}\n\n`;
      } else {
        configText += `❌ Not found in PATH\n\n`;
      }

      // Show common locations checked
      if (diagnostics.locationsChecked.length > 0) {
        configText += `**Common locations checked:**\n\n`;
        diagnostics.locationsChecked.forEach((loc) => {
          configText += `- \`${loc.path}\`\n`;
          if (loc.exists) {
            configText += `  - ✅ File exists\n`;
            if (loc.isFile !== undefined) {
              configText += `  - ${loc.isFile ? "✅" : "❌"} Is regular file: ${loc.isFile}\n`;
            }
            if (loc.executable !== undefined) {
              configText += `  - ${loc.executable ? "✅" : "❌"} Executable: ${loc.executable}\n`;
            }
            if (loc.version) {
              configText += `  - ✅ Version: ${loc.version}\n`;
            }
            if (loc.error) {
              configText += `  - ⚠️ Error: ${loc.error}\n`;
            }
          } else {
            configText += `  - ❌ Does not exist\n`;
            if (loc.error) {
              configText += `  - ⚠️ Error: ${loc.error}\n`;
            }
          }
        });
        configText += `\n`;
      }
    }
  }

  // DuckDB Information
  configText += `\n## DuckDB\n\n`;
  const duckDbStatus = getDuckDbStatus();
  if (!config.useDuckDb) {
    configText += `⏸️ **Status:** Disabled (QSV_MCP_USE_DUCKDB=false)\n`;
  } else if (duckDbStatus.status === "available") {
    configText += `✅ **Status:** Available\n`;
    configText += `📍 **Path:** \`${duckDbStatus.binPath}\`\n`;
    configText += `🏷️ **Version:** ${duckDbStatus.version}\n`;
    configText += `ℹ️ SQL queries are routed through DuckDB for better compatibility and performance.\n`;
  } else if (duckDbStatus.status === "unavailable") {
    configText += `❌ **Status:** Unavailable\n`;
    configText += `⚠️ **Reason:** ${duckDbStatus.reason}\n`;
    configText += `ℹ️ SQL queries use Polars SQL (sqlp) as fallback.\n`;
  } else {
    configText += `⏳ **Status:** Pending (detected on first SQL query)\n`;
    configText += `ℹ️ DuckDB will be auto-detected when the first SQL query runs.\n`;
  }

  // Working Directory
  configText += `\n## Working Directory\n\n`;
  if (filesystemProvider) {
    const workingDir = filesystemProvider.getWorkingDirectory();
    configText += `📁 **Current:** \`${workingDir}\`\n`;
  } else {
    configText += `📁 **Current:** \`${config.workingDir}\`\n`;
  }

  // Allowed Directories
  configText += `\n## Allowed Directories\n\n`;
  if (config.allowedDirs.length > 0) {
    configText += `🔓 **Access granted to:**\n`;
    config.allowedDirs.forEach((dir) => {
      configText += `   - \`${dir}\`\n`;
    });
  } else {
    configText += `ℹ️ Only working directory is accessible\n`;
  }
  if (config.isPluginMode) {
    configText += `\n📌 _Plugin mode: additional directories are auto-added as needed at runtime._\n`;
  }

  // Performance Settings
  configText += `\n## Performance Settings\n\n`;
  configText += `⏱️ **Timeout:** ${config.operationTimeoutMs}ms (${Math.round(config.operationTimeoutMs / 1000)}s)\n`;
  configText += `💾 **Max Output Size:** ${formatBytes(config.maxOutputSize)}\n`;
  configText += `🔧 **Auto-Regenerate Skills:** ${config.autoRegenerateSkills ? "Enabled" : "Disabled"}\n`;
  configText += `📄 **Output Format:** ${config.outputFormat.toUpperCase()}\n`;

  // Update Check Settings
  configText += `\n## Update Settings\n\n`;
  configText += `🔍 **Check Updates on Startup:** ${config.checkUpdatesOnStartup ? "Enabled" : "Disabled"}\n`;
  configText += `📢 **Update Notifications:** ${config.notifyUpdates ? "Enabled" : "Disabled"}\n`;

  // Mode
  configText += `\n## Deployment Mode\n\n`;
  if (config.isPluginMode) {
    configText += `🔌 **Claude Plugin Mode** (relaxed directory security)\n`;
  } else if (extensionMode) {
    configText += `🧩 **Desktop Extension Mode**\n`;
  } else {
    configText += `🖥️ **Legacy MCP Server Mode**\n`;
  }

  // Help Text
  configText += `\n---\n\n`;
  if (!validation.valid) {
    configText += `### ⚠️ Action Required\n\n`;
    if (extensionMode) {
      configText += `To fix the qsv binary issue:\n`;
      configText += `1. Install qsv from https://github.com/dathere/qsv#installation\n`;
      configText += `2. Open Claude Desktop Settings > Extensions > qsv\n`;
      configText += `3. Update "qsv Binary Path" or ensure qsv is in your system PATH\n`;
      configText += `4. Save settings (extension will auto-restart)\n`;
    } else {
      configText += `To fix the qsv binary issue:\n`;
      configText += `1. Install qsv from https://github.com/dathere/qsv#installation\n`;
      configText += `2. Ensure qsv is in your PATH or set QSV_MCP_BIN_PATH\n`;
      configText += `3. Restart the MCP server\n`;
    }
  } else {
    configText += `### 💡 Tip\n\n`;
    configText += `These are the actual resolved values used by the server. The configuration UI may show template variables like \`\${HOME}/Downloads\` which get expanded to the paths shown above.\n`;
  }

  return {
    content: [{ type: "text", text: configText }],
  };
}

/**
 * Initiate graceful shutdown
 */
export function initiateShutdown(): void {
  isShuttingDown = true;
  console.error(
    `[MCP Tools] Shutdown initiated, ${activeOperationCount} active operations, ${activeProcesses.size} tracked processes`,
  );
}

/**
 * Kill all active child processes for graceful shutdown.
 */
export function killAllProcesses(): void {
  for (const proc of activeProcesses) {
    try {
      proc.kill("SIGTERM");
    } catch {
      // Process might have already exited
    }
  }
  activeProcesses.clear();
  console.error("[MCP Tools] All child processes terminated");
}

/**
 * Get count of active child processes tracked for shutdown
 */
export function getActiveProcessCount(): number {
  return activeProcesses.size;
}

/**
 * Get count of active operations (in-flight tool calls)
 */
export function getActiveOperationCount(): number {
  return activeOperationCount;
}

/**
 * Test-only exports for concurrency slot logic.
 * Exported to enable unit testing of acquireSlot/releaseSlot behavior.
 */
export const _testConcurrency = {
  acquireSlot,
  releaseSlot,
  getSlotWaiterCount: () => slotWaiters.length,
  setMaxConcurrent: (n: number) => {
    // Test-only escape hatch: bypasses type safety to mutate config directly.
    // Will break if config is ever frozen or made readonly.
    (config as Record<string, unknown>).maxConcurrentOperations = n;
  },
  reset: () => {
    activeOperationCount = 0;
    slotWaiters.length = 0;
  },
};

/**
 * Create qsv_search_tools tool definition
 * Enables tool discovery for MCP clients without native tool search
 */
export function createSearchToolsTool(): McpToolDefinition {
  return {
    name: "qsv_search_tools",
    description: `Search for qsv tools by keyword, category, or use case.

💡 USE WHEN:
- Looking for the right qsv command for a specific task
- Discovering available commands by category (filtering, transformation, etc.)
- Finding commands by capability (regex, SQL, joins, etc.)

🔍 SEARCH MODES:
- **Keyword**: Matches tool names, descriptions, and examples
- **Category**: Filter by category (selection, filtering, transformation, aggregation, joining, validation, formatting, conversion, documentation, utility)
- **Regex**: Use regex patterns for advanced matching

📋 RETURNS: List of matching tools with names and descriptions, suitable for tool discovery.`,
    inputSchema: {
      type: "object",
      properties: {
        query: {
          type: "string",
          description:
            'Search query - keyword, regex pattern, or natural language description. Examples: "join", "duplicate", "SQL", "/sort|order/", "remove columns"',
        },
        category: {
          type: "string",
          description:
            "Filter by category: selection, filtering, transformation, aggregation, joining, validation, formatting, conversion, documentation, utility",
          enum: [
            "selection",
            "filtering",
            "transformation",
            "aggregation",
            "joining",
            "validation",
            "formatting",
            "conversion",
            "documentation",
            "utility",
          ],
        },
        limit: {
          type: "number",
          description:
            "Maximum number of results to return (default: 5, max: 20)",
        },
      },
      required: ["query"],
    },
  };
}

/**
 * Handle qsv_search_tools call
 * Searches loaded skills and returns matching tools
 * Marks found tools as loaded for deferred loading
 *
 * @param params - Search parameters (query, category, limit)
 * @param loader - SkillLoader instance for searching skills
 * @param loadedTools - Optional Set to track loaded tools for deferred loading
 */
export async function handleSearchToolsCall(
  params: Record<string, unknown>,
  loader: SkillLoader,
  loadedTools?: Set<string>,
): Promise<{ content: Array<{ type: string; text: string }> }> {
  const query = params.query as string;
  const category = params.category as string | undefined;
  const limit = Math.min(Math.max(1, (params.limit as number) || 5), 20);

  if (!query || query.trim().length === 0) {
    return {
      content: [
        {
          type: "text",
          text: "Error: query parameter is required",
        },
      ],
    };
  }

  // Ensure all skills are loaded
  await loader.loadAll();

  // Search using the loader's search method
  let results = loader.search(query);

  // Apply category filter if specified
  if (category) {
    results = results.filter((skill) => skill.category === category);
  }

  // Also check if query matches category name (for discovery)
  const categories = [
    "selection",
    "filtering",
    "transformation",
    "aggregation",
    "joining",
    "validation",
    "formatting",
    "conversion",
    "documentation",
    "utility",
  ];
  const queryLower = query.toLowerCase();
  const matchedCategory = categories.find((cat) => queryLower.includes(cat));

  if (matchedCategory && !category) {
    // Add skills from matching category that weren't already found
    const categorySkills = loader.getByCategory(matchedCategory as SkillCategory);
    const existingNames = new Set(results.map((r) => r.name));
    for (const skill of categorySkills) {
      if (!existingNames.has(skill.name)) {
        results.push(skill);
      }
    }
  }

  // Try regex matching if query looks like a regex pattern
  const isRegexPattern = query.startsWith("/") && query.endsWith("/");
  if (isRegexPattern) {
    try {
      const regexStr = query.slice(1, -1);
      const regex = new RegExp(regexStr, "i");
      const allSkills = loader.getAll();

      results = allSkills.filter(
        (skill) =>
          regex.test(skill.name) ||
          regex.test(skill.description) ||
          regex.test(skill.command.subcommand) ||
          skill.examples?.some((ex) => regex.test(ex.description)),
      );
    } catch (regexError) {
      // Invalid regex, fall back to text search (already done above)
    }
  }

  // Sort by relevance (exact name match first, then description match)
  results.sort((a, b) => {
    const aNameMatch = a.name.toLowerCase().includes(queryLower) ? 1 : 0;
    const bNameMatch = b.name.toLowerCase().includes(queryLower) ? 1 : 0;
    const aCommandMatch = a.command.subcommand
      .toLowerCase()
      .includes(queryLower)
      ? 1
      : 0;
    const bCommandMatch = b.command.subcommand
      .toLowerCase()
      .includes(queryLower)
      ? 1
      : 0;

    const aScore = aNameMatch * 2 + aCommandMatch * 2;
    const bScore = bNameMatch * 2 + bCommandMatch * 2;

    return bScore - aScore;
  });

  // Limit results
  const limitedResults = results.slice(0, limit);

  // Mark found tools as loaded for deferred loading
  // This allows them to appear in subsequent ListTools responses
  if (loadedTools) {
    for (const skill of limitedResults) {
      const toolName = skill.name.replace("qsv-", "qsv_");
      loadedTools.add(toolName);
    }
    console.error(
      `[MCP Tools] Marked ${limitedResults.length} tools as loaded for deferred loading`,
    );
  }

  if (limitedResults.length === 0) {
    // Provide helpful suggestions
    const allCategories = loader.getCategories();
    const totalSkills = loader.getStats().total;

    return {
      content: [
        {
          type: "text",
          text:
            `No tools found matching "${query}".\n\n` +
            `Try:\n` +
            `- Different keywords (e.g., "filter", "join", "sort", "stats")\n` +
            `- Category filter: ${allCategories.join(", ")}\n` +
            `- Regex pattern: /pattern/\n\n` +
            `Total available tools: ${totalSkills}`,
        },
      ],
    };
  }

  // Format results as tool references
  let resultText = `Found ${results.length} tool${results.length !== 1 ? "s" : ""} matching "${query}"`;
  if (category) {
    resultText += ` in category "${category}"`;
  }
  if (results.length > limit) {
    resultText += ` (showing top ${limit})`;
  }
  resultText += ":\n\n";

  for (const skill of limitedResults) {
    const toolName = skill.name.replace("qsv-", "qsv_");
    // Truncate description to first sentence for conciseness
    let shortDesc = skill.description.split(".")[0];
    if (shortDesc.length > 100) {
      shortDesc = shortDesc.substring(0, 97) + "...";
    }

    // Get when-to-use guidance if available
    const whenToUse = COMMAND_GUIDANCE[skill.command.subcommand]?.whenToUse;

    resultText += `**${toolName}** [${skill.category}]\n`;
    resultText += `  ${shortDesc}\n`;
    if (whenToUse) {
      resultText += `  💡 ${whenToUse}\n`;
    }
    resultText += "\n";
  }

  // Add tip for using the tools
  resultText += `---\n`;
  resultText += `💡 To use a tool, call it directly: e.g., \`qsv_${limitedResults[0].command.subcommand}\` with \`input_file\` parameter.\n`;
  resultText += `📖 For detailed help on any command, use \`help: true\` parameter.`;

  return {
    content: [
      {
        type: "text",
        text: resultText,
      },
    ],
  };
}

/**
 * Create qsv_to_parquet tool definition
 * Converts CSV files to Parquet format for optimized SQL operations
 */
export function createToParquetTool(): McpToolDefinition {
  return {
    name: "qsv_to_parquet",
    description: `Convert CSV to Parquet format with guaranteed data type inference.

💡 USE WHEN: CSV file is >10MB and needs SQL queries. Convert once with same file stem in working directory, then query the Parquet file. Prefer DuckDB if available; otherwise use sqlp with SKIP_INPUT and read_parquet('file.parquet').

📋 AUTO-OPTIMIZATION: Runs stats with --infer-dates --dates-whitelist sniff for automatic Date/DateTime detection. Generates Polars schema for correct data types (integers, floats, dates, booleans).

📋 COMMON PATTERN: Convert once, query many times with DuckDB or sqlp SKIP_INPUT + read_parquet(). Parquet is for sqlp/DuckDB ONLY. Keep CSV/TSV/SSV for all other qsv commands.

⚠️ IMPORTANT: Parquet files work ONLY with sqlp and DuckDB. All other qsv commands (including joinp and pivotp) require CSV/TSV/SSV input.`,
    inputSchema: {
      type: "object",
      properties: {
        input_file: {
          type: "string",
          description: "Path to input CSV file to convert",
        },
        output_file: {
          type: "string",
          description:
            "Path for output Parquet file (optional - defaults to input_file.parquet in same directory)",
        },
      },
      required: ["input_file"],
    },
  };
}

/**
 * Handle qsv_to_parquet tool call
 * Converts CSV to Parquet using sqlp's read_csv and --format parquet
 */
export async function handleToParquetCall(
  params: Record<string, unknown>,
  filesystemProvider?: FilesystemProviderExtended,
): Promise<{
  content: Array<{ type: string; text: string }>;
  isError?: boolean;
}> {
  // Extract input_file and output_file (with LLM alias resolution)
  const { inputFile: rawInputFile, outputFile: rawOutputFile } = resolveParamAliases(params);
  let inputFile = rawInputFile;
  let outputFile = rawOutputFile;

  if (!inputFile) {
    return errorResult("Error: input_file parameter is required");
  }

  // Resolve input file path using filesystem provider if available
  if (filesystemProvider) {
    try {
      const originalInputFile = inputFile;
      inputFile = await filesystemProvider.resolvePath(inputFile);
      console.error(
        `[MCP Tools] Resolved input file: ${originalInputFile} -> ${inputFile}`,
      );
    } catch (error: unknown) {
      return errorResult(`Error resolving input file path: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  // Generate output path if not provided
  if (!outputFile) {
    // Replace common CSV-like extensions with .parquet, or append .parquet if none match
    const lowerInput = inputFile.toLowerCase();
    // Supported CSV-like extensions, including snappy-compressed variants
    const csvLikeExtensions = [
      ".csv.sz",
      ".tsv.sz",
      ".tab.sz",
      ".ssv.sz",
      ".csv",
      ".tsv",
      ".tab",
      ".ssv",
    ];

    let matched = false;
    for (const ext of csvLikeExtensions) {
      if (lowerInput.endsWith(ext)) {
        outputFile = inputFile.slice(0, -ext.length) + ".parquet";
        matched = true;
        break;
      }
    }

    if (!matched) {
      outputFile = inputFile + ".parquet";
    }
  }

  // At this point outputFile is guaranteed to be defined (either provided or generated)
  let resolvedOutputFile: string = outputFile as string;

  // Resolve output file path using filesystem provider if available
  if (filesystemProvider) {
    try {
      const originalOutputFile = resolvedOutputFile;
      resolvedOutputFile =
        await filesystemProvider.resolvePath(resolvedOutputFile);
      console.error(
        `[MCP Tools] Resolved output file: ${originalOutputFile} -> ${resolvedOutputFile}`,
      );
    } catch (error: unknown) {
      return errorResult(`Error resolving output file path: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  const qsvBin = getQsvBinaryPath();
  const startTime = Date.now();
  const statsFile = inputFile + ".stats.csv";
  const schemaFile = inputFile + ".pschema.json";

  // Check if regeneration is needed based on file mtimes
  let needStats = true;
  let needSchema = true;

  try {
    const inputFileStats = await stat(inputFile);

    try {
      const existingStats = await stat(statsFile);
      if (existingStats.mtimeMs >= inputFileStats.mtimeMs) {
        needStats = false;
      }
    } catch (error: unknown) {
      console.warn(`[MCP Tools] Stats cache not found, will generate: ${statsFile}`, error);
    }

    try {
      const existingSchema = await stat(schemaFile);
      if (existingSchema.mtimeMs >= inputFileStats.mtimeMs) {
        needSchema = false;
      }
    } catch (error: unknown) {
      console.warn(`[MCP Tools] Schema not found, will generate: ${schemaFile}`, error);
    }
  } catch (error: unknown) {
    console.warn(`[MCP Tools] Cannot stat input file, will regenerate stats/schema: ${inputFile}`, error);
  }

  try {
    // Step 1: Generate stats cache for accurate type inference (if needed)
    // This creates .stats.csv and .stats.csv.data.jsonl files
    // Uses --dates-whitelist sniff to let qsv stats detect Date/DateTime columns natively
    if (needStats) {
      console.error(
        `[MCP Tools] Step 1: Generating stats cache for ${inputFile}`,
      );
      const statsArgs = [
        "stats",
        inputFile,
        "--cardinality",
        "--stats-jsonl",
        "--infer-dates",
        "--dates-whitelist",
        "sniff",
      ];
      await runQsvWithTimeout(qsvBin, statsArgs);
      console.error(`[MCP Tools] Stats cache generated successfully`);
    } else {
      console.error(
        `[MCP Tools] Step 1: Using existing stats cache (up-to-date)`,
      );
    }

    // Step 2: Generate Polars schema using the stats cache (if needed)
    // This creates a .pschema.json file that sqlp will auto-detect
    if (needSchema) {
      console.error(
        `[MCP Tools] Step 2: Generating Polars schema for ${inputFile}`,
      );
      const schemaArgs = ["schema", "--polars", inputFile];
      await runQsvWithTimeout(qsvBin, schemaArgs);
      console.error(`[MCP Tools] Polars schema generated: ${schemaFile}`);
    } else {
      console.error(
        `[MCP Tools] Step 2: Using existing Polars schema (up-to-date)`,
      );
    }

    // Step 2.5: Patch schema for AM/PM date formats that Polars can't parse
    await patchSchemaAndLog(inputFile, schemaFile);

    // Step 3: Convert to Parquet (sqlp will auto-detect .pschema.json)
    console.error(`[MCP Tools] Step 3: Converting to Parquet with schema`);
    const conversionArgs = buildConversionArgs(
      "csv-to-parquet",
      inputFile,
      resolvedOutputFile,
    );
    console.error(
      `[MCP Tools] Running CSV→Parquet conversion: ${qsvBin} ${conversionArgs.join(" ")}`,
    );
    await runQsvWithTimeout(qsvBin, conversionArgs);
    const duration = Date.now() - startTime;

    // Get output file size for reporting
    let fileSizeInfo = "";
    try {
      const outputStats = await stat(resolvedOutputFile);
      fileSizeInfo = ` (${formatBytes(outputStats.size)})`;
    } catch (error: unknown) {
      console.warn(`[MCP Tools] Could not stat output file for size reporting: ${resolvedOutputFile}`, error);
    }

    const statsStatus = needStats ? "generated" : "reused (up-to-date)";
    const schemaStatus = needSchema ? "generated" : "reused (up-to-date)";

    return successResult(
      `✅ Successfully converted CSV to Parquet with optimized schema\n\n` +
      `Input: ${inputFile}\n` +
      `Output: ${resolvedOutputFile}${fileSizeInfo}\n` +
      `Schema: ${schemaFile}\n` +
      `Duration: ${duration}ms\n\n` +
      `Stats cache: ${statsStatus}\n` +
      `Polars schema: ${schemaStatus}\n` +
      `The Parquet file is now ready for fast SQL queries.\n` +
      (getDuckDbStatus().status === "available"
        ? `🦆 DuckDB detected — qsv_sqlp will auto-route SQL queries through DuckDB for this file.`
        : `Use: qsv_sqlp with input_file="SKIP_INPUT" and sql="SELECT ... FROM read_parquet('${resolvedOutputFile}')".`),
    );
  } catch (error: unknown) {
    return errorResult(`Error converting CSV to Parquet: ${error instanceof Error ? error.message : String(error)}`);
  }
}
