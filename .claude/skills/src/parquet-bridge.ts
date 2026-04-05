/**
 * Parquet/DuckDB conversion, schema management, and stats cache utilities.
 */

import { spawn } from "child_process";
import { readFile, writeFile, open, stat, unlink, mkdir } from "fs/promises";
import { basename, dirname, join, parse } from "path";
import { isShuttingDown, activeProcesses } from "./concurrency.js";
import { statOrNull, runQsvWithTimeout, getCurrentWorkingDir } from "./file-operations.js";
import {
  detectDuckDb,
  markDuckDbUnavailable,
  translateSql,
  executeDuckDbQuery,
  CSV_LIKE_EXTENSIONS,
} from "./duckdb.js";
import { config } from "./config.js";
import { getErrorMessage, errorResult, successResult } from "./utils.js";

/**
 * Map a qsv stats `type` + min/max range to the tightest DuckDB SQL type.
 * Returns null for types that should remain VARCHAR (String, NULL, unknown).
 */
function statsTypeToDuckDb(statsType: string, minStr: string, maxStr: string): string | null {
  switch (statsType) {
    case "Integer": {
      // Use BigInt to avoid precision loss for values beyond Number.MAX_SAFE_INTEGER
      let min: bigint;
      let max: bigint;
      try {
        min = BigInt(minStr);
        max = BigInt(maxStr);
      } catch {
        return "BIGINT";
      }
      if (min >= 0n) {
        // DuckDB unsigned integer ranges
        if (max <= 255n) return "UTINYINT";
        if (max <= 65535n) return "USMALLINT";
        if (max <= 4294967295n) return "UINTEGER";
        // DuckDB signed BIGINT max:  2^63 - 1
        const BIGINT_MAX = 9223372036854775807n;
        // DuckDB unsigned UBIGINT max: 2^64 - 1
        const UBIGINT_MAX = 18446744073709551615n;
        if (max <= BIGINT_MAX) return "BIGINT";
        if (max <= UBIGINT_MAX) return "UBIGINT";
        // Values larger than UBIGINT cannot be safely cast to an integer type
        return null;
      }
      // signed
      if (min >= -128n && max <= 127n) return "TINYINT";
      if (min >= -32768n && max <= 32767n) return "SMALLINT";
      if (min >= -2147483648n && max <= 2147483647n) return "INTEGER";
      return "BIGINT";
    }
    case "Float":
      return "DOUBLE";
    case "Date":
      return "DATE";
    case "DateTime":
      return "TIMESTAMP";
    default:
      // String, NULL, or unknown — let DuckDB default to VARCHAR
      return null;
  }
}

/**
 * Parse a qsv stats cache CSV file into a map of column stats.
 * Returns null on any read/parse failure.
 *
 * NOTE: Uses line-by-line parsing which doesn't handle embedded newlines in
 * quoted fields. This is acceptable because qsv stats output column names
 * come from CSV headers which rarely contain newlines, but if they do, this
 * parser will produce incorrect results and return null (falling back to sqlp).
 */
async function parseStatsCsv(statsFile: string): Promise<Map<string, { type: string; min: string; max: string }> | null> {
  let content: string;
  try {
    content = await readFile(statsFile, "utf-8");
  } catch {
    console.error(`[MCP Tools] DuckDB Parquet: Cannot read stats file: ${statsFile}`);
    return null;
  }

  const lines = content.split(/\r?\n/).filter(l => l.length > 0);
  if (lines.length < 2) {
    console.error(`[MCP Tools] DuckDB Parquet: Stats file has no data rows: ${statsFile}`);
    return null;
  }

  const header = parseCSVLine(lines[0]);
  const fieldIdx = header.indexOf("field");
  const typeIdx = header.indexOf("type");
  const minIdx = header.indexOf("min");
  const maxIdx = header.indexOf("max");

  if (fieldIdx < 0 || typeIdx < 0 || minIdx < 0 || maxIdx < 0) {
    console.error(`[MCP Tools] DuckDB Parquet: Stats file missing required columns (field/type/min/max): ${statsFile}`);
    return null;
  }

  const expectedCols = header.length;
  const result = new Map<string, { type: string; min: string; max: string }>();
  for (let i = 1; i < lines.length; i++) {
    const cols = parseCSVLine(lines[i]);
    if (cols.length < expectedCols) {
      console.error(`[MCP Tools] DuckDB Parquet: Row ${i} has ${cols.length} fields, expected ${expectedCols} — skipping: ${statsFile}`);
      continue;
    }
    const field = cols[fieldIdx];
    if (field) {
      result.set(field, {
        type: cols[typeIdx] ?? "",
        min: cols[minIdx] ?? "",
        max: cols[maxIdx] ?? "",
      });
    }
  }

  if (result.size === 0) {
    console.error(`[MCP Tools] DuckDB Parquet: No fields parsed from stats file: ${statsFile}`);
    return null;
  }

  return result;
}

/**
 * Build DuckDB SQL for CSV→Parquet conversion using the stats cache for tight type casting.
 * Returns null if the stats file cannot be read, parsed, or validated, or if the
 * input/output paths fail validation (e.g., contain null bytes); caller should fall back to sqlp.
 */
async function buildDuckDbParquetSql(
  inputFile: string,
  outputFile: string,
  statsFile: string,
): Promise<string | null> {
  const statsMap = await parseStatsCsv(statsFile);
  if (statsMap === null) return null;

  // Build SELECT columns with CASTs where needed
  const selectParts: string[] = [];
  for (const [colName, colStats] of statsMap) {
    const duckType = statsTypeToDuckDb(colStats.type, colStats.min, colStats.max);
    // Double-quote column names for safety
    const quotedCol = `"${colName.replace(/"/g, '""')}"`;
    if (duckType) {
      selectParts.push(`CAST(${quotedCol} AS ${duckType}) AS ${quotedCol}`);
    } else {
      selectParts.push(quotedCol);
    }
  }

  // Validate file paths don't contain null bytes that could break SQL escaping.
  const dangerousPathPattern = /\x00/;
  if (dangerousPathPattern.test(inputFile) || dangerousPathPattern.test(outputFile)) {
    console.error(`[MCP Tools] DuckDB Parquet: Rejecting paths with null bytes`);
    return null;
  }

  // Normalize paths for SQL: backslash to forward slash (Windows compat),
  // then single-quote escaping for SQL string literals.
  const normInput = inputFile.replace(/\\/g, "/").replace(/'/g, "''");
  const normOutput = outputFile.replace(/\\/g, "/").replace(/'/g, "''");

  const selectClause = selectParts.length > 0 ? selectParts.join(", ") : "*";

  // Detect delimiter for non-comma-delimited files (.tsv/.tab/.ssv)
  const delimiter = detectDelimiter(inputFile);
  // Defensive allowlist to prevent SQL injection via delimiter interpolation
  if (![",", "\t", ";"].includes(delimiter)) {
    console.error(`[MCP Tools] DuckDB Parquet: Unexpected delimiter value, rejecting`);
    return null;
  }
  const delimArg = delimiter !== "," ? `, delim='${delimiter === "\t" ? "\\t" : delimiter}'` : "";

  // Stream directly via COPY (SELECT ...) TO ... to avoid materializing an intermediate table
  const sql =
    `COPY (SELECT ${selectClause} FROM read_csv('${normInput}', auto_detect=true${delimArg})) ` +
    `TO '${normOutput}' (FORMAT PARQUET, COMPRESSION ZSTD);`;

  return sql;
}

/**
 * Spawn a DuckDB process to execute SQL commands against the specified database path
 * (either a persistent database file or ':memory:' for an in-memory database).
 * Integrates with the activeProcesses set for graceful shutdown.
 */
async function spawnDuckDbCommands(
  binPath: string,
  dbPath: string,
  sql: string,
  timeoutMs: number = config.operationTimeoutMs,
): Promise<void> {
  if (isShuttingDown) {
    throw new Error("Server is shutting down, operation rejected");
  }

  // Snapshot the working directory to avoid TOCTOU issues if it changes mid-execution.
  const workDir = getCurrentWorkingDir();

  // Ensure working directory exists before spawning
  await mkdir(workDir, { recursive: true });

  return new Promise((resolve, reject) => {
    const proc = spawn(binPath, [dbPath, "-c", sql], {
      // stdout ignored: COPY ... TO produces no result set; prevents backpressure hangs.
      // If future SQL returns rows, change to "pipe" and consume the stream.
      stdio: ["ignore", "ignore", "pipe"],
      cwd: workDir,
    });

    activeProcesses.add(proc);

    let stderr = "";
    let timedOut = false;
    let timer: ReturnType<typeof setTimeout> | null = null;
    let killTimer: ReturnType<typeof setTimeout> | null = null;

    timer = setTimeout(() => {
      timedOut = true;
      proc.kill("SIGTERM");
      killTimer = setTimeout(() => {
        if (proc.exitCode === null) {
          try { proc.kill("SIGKILL"); } catch { /* ignore */ }
          proc.unref();
        }
      }, 1000);
    }, timeoutMs);

    proc.stderr!.on("data", (chunk) => {
      if (stderr.length < MAX_STDERR_SIZE) {
        stderr += chunk.toString();
        if (stderr.length > MAX_STDERR_SIZE) {
          stderr = stderr.slice(0, MAX_STDERR_SIZE) + "\n[STDERR TRUNCATED]";
        }
      }
    });

    proc.on("close", (exitCode, signal) => {
      if (timer) clearTimeout(timer);
      if (killTimer) clearTimeout(killTimer);
      activeProcesses.delete(proc);

      if (timedOut) {
        reject(new Error(`DuckDB Parquet conversion timed out after ${timeoutMs}ms`));
        return;
      }
      if (exitCode !== 0) {
        const exitInfo = exitCode !== null ? `exit ${exitCode}` : `killed by signal ${signal ?? "unknown"}`;
        reject(new Error(`DuckDB Parquet conversion failed (${exitInfo}): ${stderr}`));
        return;
      }
      resolve();
    });

    proc.on("error", (err) => {
      if (timer) clearTimeout(timer);
      if (killTimer) clearTimeout(killTimer);
      activeProcesses.delete(proc);
      reject(err);
    });
  });
}

/**
 * Maximum stderr buffer size for DuckDB spawns (1 MB)
 */
const MAX_STDERR_SIZE = 1024 * 1024;

/**
 * Convert CSV to Parquet, using DuckDB (with ZSTD) when available,
 * falling back to `qsv to parquet` (ZSTD, with --try-parse-dates).
 * Returns the engine description string for reporting.
 */
export async function convertCsvToParquet(
  inputFile: string,
  parquetPath: string,
  statsFile: string,
): Promise<{ engine: string; needSchema: boolean; schemaFile: string; schemaSkipped: boolean; outputPath: string }> {
  const state = detectDuckDb();

  if (state.status === "available") {
    // Try DuckDB path — only needs stats cache, not Polars schema
    const sql = await buildDuckDbParquetSql(inputFile, parquetPath, statsFile);
    if (sql !== null) {
      const dbPath = ":memory:";
      console.error(`[MCP Tools] Converting to Parquet via DuckDB (ZSTD) [in-memory]`);
      // Truncate SQL log for wide CSVs (one CAST per column can get very large)
      const sqlPreview = sql.length > 500 ? `${sql.slice(0, 500)}... (${sql.length} chars total)` : sql;
      console.error(`[MCP Tools] DuckDB SQL: ${sqlPreview}`);

      try {
        await spawnDuckDbCommands(state.binPath, dbPath, sql);
        // DuckDB writes directly to parquetPath (not directory-based)
        return { engine: `DuckDB v${state.version} (ZSTD)`, needSchema: false, schemaFile: "N/A (DuckDB)", schemaSkipped: true, outputPath: parquetPath };
      } catch (error: unknown) {
        // Clean up partial parquet on DuckDB failure, then fall back to qsv to parquet
        try { await unlink(parquetPath); } catch { /* ignore: cleanup */ }
        console.error(`[MCP Tools] DuckDB runtime failure, falling back to qsv to parquet: ${getErrorMessage(error)}`);
      }
    }
    if (sql === null) {
      // SQL generation failed (stats cache unreadable, path validation, or delimiter issue) — fall through to qsv to parquet
      console.error(`[MCP Tools] DuckDB: SQL generation failed (see earlier log for details), falling back to qsv to parquet`);
    }
  }

  // Fallback: generate Polars schema (Step 2, no AM/PM patching — --try-parse-dates handles it)
  const { needSchema, schemaFile } = await ensurePolarsSchema(inputFile);

  // Use `qsv to parquet` with ZSTD compression and native date parsing.
  // `to parquet` reads .pschema.json automatically when present (see to.rs:711-729).
  // It uses directory-based output, so we pass the parent dir and use --table for the filename stem.
  // `to parquet` always writes `{outputDir}/{table}.parquet`, so compute the effective path
  // to ensure unlink/stat/reporting are consistent even if parquetPath has a non-.parquet extension.
  const outputDir = dirname(parquetPath);
  // Strip .parquet extension case-insensitively to derive the table name stem.
  // basename() only strips exact case matches, so "out.PARQUET" would not be stripped.
  const base = basename(parquetPath);
  const outputStem = base.replace(/\.parquet$/i, "") || base;
  const effectiveParquetPath = join(outputDir, outputStem + ".parquet");

  // Remove existing parquet file — `to parquet` won't overwrite
  try { await unlink(effectiveParquetPath); } catch { /* ignore: file may not exist */ }

  console.error(`[MCP Tools] Fallback: Converting to Parquet via qsv to parquet`);
  // --compression omitted: zstd is the default in versions that support it.
  const toParquetArgs = [
    "to", "parquet", outputDir,
    "--table", outputStem,
    "--try-parse-dates",
    inputFile,
  ];
  try {
    await runQsvWithTimeout(config.qsvBinPath, toParquetArgs);
  } catch (error: unknown) {
    // Clean up partial parquet on failure
    try { await unlink(effectiveParquetPath); } catch { /* ignore: cleanup */ }
    const message = `Parquet conversion failed for ${inputFile} \u2192 ${effectiveParquetPath}: ${getErrorMessage(error)}`;
    throw new Error(message, { cause: error });
  }
  return { engine: "qsv to parquet (ZSTD)", needSchema, schemaFile, schemaSkipped: false, outputPath: effectiveParquetPath };
}

/**
 * Detect the delimiter for a CSV file based on its extension.
 * Returns ',' for .csv (and unknown), '\t' for .tsv/.tab, ';' for .ssv.
 */
export function detectDelimiter(filePath: string): string {
  const lower = filePath.toLowerCase();
  if (lower.endsWith(".tsv") || lower.endsWith(".tab") || lower.endsWith(".tsv.sz") || lower.endsWith(".tab.sz")) return "\t";
  if (lower.endsWith(".ssv") || lower.endsWith(".ssv.sz")) return ";";
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

/** In-flight Parquet conversions keyed by CSV path — prevents duplicate concurrent work */
const parquetConversionLocks = new Map<string, Promise<string>>();

// CSV_LIKE_EXTENSIONS sorted by descending length so longer extensions
// (e.g., ".csv.sz") are matched before shorter ones (e.g., ".csv").
// Sorted once at module level to avoid re-sorting on every call.
const ORDERED_CSV_EXTENSIONS = [...CSV_LIKE_EXTENSIONS].sort(
  (a, b) => b.length - a.length,
);

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

/**
 * Compute the stats cache file path for a given input file.
 * Mirrors qsv's Rust `stats_path()`: `{parent}/{file_stem}.stats.csv`.
 * e.g. `/tmp/cities.csv` → `/tmp/cities.stats.csv`
 *      `/tmp/data.tsv.sz` → `/tmp/data.tsv.stats.csv`
 */
export function statsFilePath(inputFile: string): string {
  const { dir, name } = parse(inputFile);
  return join(dir, `${name}.stats.csv`);
}

/**
 * Ensure the stats cache (.stats.csv) is up-to-date for the given input file.
 * This is Step 1 of the Parquet conversion pipeline and is needed by both
 * the DuckDB and sqlp paths.
 */
export async function ensureStatsCache(
  inputFile: string,
): Promise<{ needStats: boolean; statsFile: string }> {
  const qsvBin = config.qsvBinPath;
  const statsFile = statsFilePath(inputFile);

  const [inputFileStats, existingStats] = await Promise.all([
    statOrNull(inputFile),
    statOrNull(statsFile),
  ]);

  if (!inputFileStats) {
    throw new Error(`Input file not found: ${inputFile}`);
  }

  const needStats = !existingStats || existingStats.mtimeMs < inputFileStats.mtimeMs;

  if (needStats) {
    console.error(
      `[MCP Tools] Step 1: ${existingStats ? "Stats cache outdated" : "Stats cache not found"}, generating...`,
    );
    try {
      const statsArgs = [
        "stats", inputFile,
        "--cardinality", "--stats-jsonl",
        "--infer-dates", "--dates-whitelist", "sniff",
      ];
      await runQsvWithTimeout(qsvBin, statsArgs);
    } catch (error: unknown) {
      throw new Error(`Stats generation failed for ${inputFile}: ${getErrorMessage(error)}`);
    }
  } else {
    console.error(`[MCP Tools] Step 1: Using existing stats cache (up-to-date)`);
  }

  return { needStats, statsFile };
}

/**
 * Ensure the Polars schema (.pschema.json) is up-to-date for the given input file.
 * Covers Step 2 (schema generation) and optionally Step 2.5 (AM/PM date patching).
 *
 * The `to parquet` fallback uses --try-parse-dates for native date handling,
 * so AM/PM patching is only needed for the sqlp path (kept for backward compatibility
 * in case sqlp is used elsewhere).
 */
export async function ensurePolarsSchema(
  inputFile: string,
  options: { patchAmPmDates?: boolean } = {},
): Promise<{ needSchema: boolean; schemaFile: string }> {
  const { patchAmPmDates = false } = options;
  const qsvBin = config.qsvBinPath;
  const schemaFile = inputFile + ".pschema.json";

  // Stat input file for mtime comparison and schema file for freshness check.
  const [inputFileStats, existingSchema] = await Promise.all([
    statOrNull(inputFile),
    statOrNull(schemaFile),
  ]);

  // Safety check — should never happen since ensureStatsCache runs first.
  if (!inputFileStats) {
    throw new Error(`Input file not found: ${inputFile}`);
  }

  const needSchema = !existingSchema || existingSchema.mtimeMs < inputFileStats.mtimeMs;

  // Step 2: Generate Polars schema
  if (needSchema) {
    console.error(
      `[MCP Tools] Step 2: ${existingSchema ? "Polars schema outdated" : "Polars schema not found"}, generating...`,
    );
    try {
      const schemaArgs = ["schema", "--polars", inputFile];
      await runQsvWithTimeout(qsvBin, schemaArgs);
    } catch (error: unknown) {
      throw new Error(`Schema generation failed for ${inputFile}: ${getErrorMessage(error)}`);
    }
  } else {
    console.error(`[MCP Tools] Step 2: Using existing Polars schema (up-to-date)`);
  }

  // Step 2.5: Patch schema for AM/PM date formats that Polars can't parse
  // Only needed for sqlp path; `to parquet` uses --try-parse-dates instead
  if (patchAmPmDates) {
    await patchSchemaAndLog(inputFile, schemaFile);
  }

  return { needSchema, schemaFile };
}

async function doParquetConversion(inputFile: string, parquetPath: string): Promise<string> {
  console.error(`[MCP Tools] ensureParquet: Auto-converting CSV to Parquet: ${inputFile}`);

  const { statsFile } = await ensureStatsCache(inputFile);

  // Step 3: Convert to Parquet (DuckDB with ZSTD when available, qsv to parquet with ZSTD otherwise)
  const { engine, outputPath } = await convertCsvToParquet(inputFile, parquetPath, statsFile);
  console.error(`[MCP Tools] ensureParquet: Conversion engine: ${engine}`);

  // Verify the output file was actually created and is non-empty.
  // Use outputPath (the actual file written) rather than parquetPath (the requested path),
  // since the fallback engine may normalize the path (e.g. ensure .parquet extension).
  let outStats;
  try {
    outStats = await stat(outputPath);
  } catch (error: unknown) {
    console.warn(`[MCP Tools] Output file stat failed after conversion: ${outputPath}`, error);
    throw new Error(`Parquet conversion completed but output file not found: ${outputPath}`);
  }
  if (outStats.size === 0) {
    try { await unlink(outputPath); } catch { /* ignore: cleanup */ }
    throw new Error(`Parquet conversion produced an empty file: ${outputPath}`);
  }

  console.error(`[MCP Tools] ensureParquet: Successfully converted to ${outputPath}`);
  return outputPath;
}

/**
 * Suggest fixes for common DuckDB SQL errors.
 * Pattern-matches stderr to provide actionable guidance.
 */
export function suggestDuckDbFixes(stderr: string, sql?: string): string {
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
export async function tryDuckDbExecution(
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
    const errMsg = getErrorMessage(error);
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
