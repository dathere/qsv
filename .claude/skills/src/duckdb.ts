/**
 * DuckDB Integration Module
 *
 * Provides lazy DuckDB detection, SQL translation, and query execution.
 * When DuckDB is available, SQL queries are routed through it instead of sqlp
 * for better PostgreSQL compatibility and performance.
 */

import { execFileSync } from "child_process";
import { statSync } from "fs";
import { homedir } from "os";
import { join } from "path";
import { config } from "./config.js";
import { runQsvSimple } from "./executor.js";
import type { ChildProcess } from "child_process";
import { spawn } from "child_process";

/**
 * Timeout for DuckDB binary validation in milliseconds (5 seconds)
 */
const DUCKDB_VALIDATION_TIMEOUT_MS = 5000;

/**
 * DuckDB detection state — sticky once resolved
 */
type DuckDbState =
  | { status: "pending" }
  | { status: "available"; binPath: string; version: string }
  | { status: "unavailable"; reason: string };

let duckDbState: DuckDbState = { status: "pending" };

/**
 * Options for SQL translation
 */
export interface TranslateSqlOptions {
  /** CSV delimiter character (maps to DuckDB read_csv delim option) */
  delimiter?: string;
  /** Null value strings (maps to DuckDB read_csv nullstr option) */
  rnullValues?: string;
}

/**
 * Options for DuckDB query execution
 */
export interface DuckDbExecutionOptions {
  /** Output format: csv, json, parquet */
  format?: string;
  /** Output file path (for parquet output) */
  outputFile?: string;
  /** Whether to use decimal comma */
  decimalComma?: boolean;
  /** Compression codec for parquet output */
  compression?: string;
  /** Timeout in milliseconds */
  timeoutMs?: number;
  /** Callback when process spawns (for shutdown tracking) */
  onSpawn?: (proc: ChildProcess) => void;
  /** Callback when process exits */
  onExit?: (proc: ChildProcess) => void;
}

/**
 * Result from DuckDB query execution
 */
export interface DuckDbResult {
  /** Query output (CSV or JSON text) */
  output: string;
  /** DuckDB version string */
  version: string;
  /** Exit code */
  exitCode: number;
  /** Error output */
  stderr: string;
}

/**
 * Check if DuckDB is enabled via environment variable.
 * Returns false only if explicitly disabled.
 */
export function isDuckDbEnabled(): boolean {
  return config.useDuckDb;
}

/**
 * Get the current DuckDB detection state
 */
export function getDuckDbStatus(): DuckDbState {
  return duckDbState;
}

/**
 * Reset DuckDB state (for testing)
 */
export function resetDuckDbState(): void {
  duckDbState = { status: "pending" };
}

/**
 * Mark DuckDB as unavailable (e.g., after a binary-level failure)
 */
export function markDuckDbUnavailable(reason: string): void {
  duckDbState = { status: "unavailable", reason };
}

/**
 * Detect DuckDB binary lazily — called on first SQL query only.
 *
 * Priority:
 * 1. QSV_MCP_DUCKDB_BIN_PATH env var
 * 2. `which duckdb` / `where duckdb`
 * 3. Common installation locations
 *
 * Validates by running `duckdb --version` with 5s timeout.
 * State is sticky once resolved (available or unavailable).
 */
export function detectDuckDb(): DuckDbState {
  // Return cached state if already resolved
  if (duckDbState.status !== "pending") {
    return duckDbState;
  }

  // Check if disabled
  if (!isDuckDbEnabled()) {
    duckDbState = { status: "unavailable", reason: "Disabled via QSV_MCP_USE_DUCKDB=false" };
    return duckDbState;
  }

  // Try explicit path first
  const explicitPath = config.duckDbBinPath;
  if (explicitPath) {
    const result = validateDuckDbBinary(explicitPath);
    if (result) {
      duckDbState = { status: "available", binPath: explicitPath, version: result };
      console.error(`[DuckDB] Found at configured path: ${explicitPath} (v${result})`);
      return duckDbState;
    }
  }

  // Try which/where
  try {
    const command = process.platform === "win32" ? "where" : "which";
    const whichResult = execFileSync(command, ["duckdb"], {
      encoding: "utf8",
      stdio: ["ignore", "pipe", "ignore"],
    });
    const binPath = whichResult.trim().split("\n")[0];
    if (binPath) {
      const version = validateDuckDbBinary(binPath);
      if (version) {
        duckDbState = { status: "available", binPath, version };
        console.error(`[DuckDB] Found in PATH: ${binPath} (v${version})`);
        return duckDbState;
      }
    }
  } catch {
    // Not in PATH
  }

  // Check common installation locations
  const commonLocations =
    process.platform === "win32"
      ? [
          "C:\\Program Files\\DuckDB\\duckdb.exe",
          join(homedir(), "scoop", "shims", "duckdb.exe"),
          join(homedir(), "AppData", "Local", "Programs", "duckdb", "duckdb.exe"),
        ]
      : [
          "/usr/local/bin/duckdb",
          "/opt/homebrew/bin/duckdb",
          "/usr/bin/duckdb",
          join(homedir(), ".local", "bin", "duckdb"),
          join(homedir(), ".duckdb", "duckdb"),
        ];

  for (const location of commonLocations) {
    try {
      const stats = statSync(location);
      if (stats.isFile()) {
        const version = validateDuckDbBinary(location);
        if (version) {
          duckDbState = { status: "available", binPath: location, version };
          console.error(`[DuckDB] Found at: ${location} (v${version})`);
          return duckDbState;
        }
      }
    } catch {
      // Location doesn't exist
    }
  }

  duckDbState = { status: "unavailable", reason: "DuckDB binary not found" };
  console.error(`[DuckDB] Not found (checked PATH and common locations)`);
  return duckDbState;
}

/**
 * Validate a DuckDB binary by running `duckdb --version`
 * Returns version string on success, null on failure
 */
function validateDuckDbBinary(binPath: string): string | null {
  try {
    const result = execFileSync(binPath, ["--version"], {
      encoding: "utf8",
      stdio: ["ignore", "pipe", "ignore"],
      timeout: DUCKDB_VALIDATION_TIMEOUT_MS,
    });
    // DuckDB version output: "v1.2.0 ..." or "DuckDB v1.2.0 ..."
    const match = result.match(/v?(\d+\.\d+\.\d+)/);
    return match ? match[1] : null;
  } catch {
    return null;
  }
}

/**
 * Translate sqlp-style SQL for DuckDB execution.
 *
 * Replaces `_t_1` (case-insensitive) with `read_parquet('path')` or `read_csv('path')`
 * based on file extension. If a Parquet file exists with the same stem as a CSV input,
 * the Parquet file is used instead.
 *
 * Passes through SKIP_INPUT queries unchanged (already have explicit file refs).
 *
 * @param sql - The SQL query with _t_1 table references
 * @param inputFile - The primary input file path
 * @param options - Translation options (delimiter, null values)
 * @returns Translated SQL ready for DuckDB execution
 */
export function translateSql(
  sql: string,
  inputFile: string,
  options?: TranslateSqlOptions,
): string {
  // Normalize path separators for SQL (Windows backslashes → forward slashes)
  const normalizedPath = inputFile.replace(/\\/g, "/");
  // Escape single quotes in path
  const escapedPath = normalizedPath.replace(/'/g, "''");

  const lowerPath = normalizedPath.toLowerCase();

  // Determine the read function based on file extension
  let readExpr: string;
  if (lowerPath.endsWith(".parquet")) {
    readExpr = `read_parquet('${escapedPath}')`;
  } else if (lowerPath.endsWith(".jsonl") || lowerPath.endsWith(".ndjson")) {
    readExpr = `read_json('${escapedPath}')`;
  } else {
    // CSV-like: build read_csv with options
    const csvOptions: string[] = [];
    if (options?.delimiter) {
      csvOptions.push(`delim = '${options.delimiter}'`);
    }
    if (options?.rnullValues) {
      // Parse comma-separated null values into array, escaping single quotes
      const nullStrs = options.rnullValues
        .split(",")
        .map((s) => `'${s.trim().replace(/'/g, "''")}'`)
        .join(", ");
      csvOptions.push(`nullstr = [${nullStrs}]`);
    }
    if (csvOptions.length > 0) {
      readExpr = `read_csv('${escapedPath}', ${csvOptions.join(", ")})`;
    } else {
      readExpr = `read_csv('${escapedPath}', auto_detect = true)`;
    }
  }

  // Replace _t_1 (case-insensitive, word boundary) with the read expression
  const translated = sql.replace(/\b_t_1\b/gi, readExpr);
  return translated;
}

/**
 * Execute a SQL query using DuckDB.
 *
 * @param sql - The SQL query to execute (already translated via translateSql)
 * @param options - Execution options
 * @returns DuckDB result, or null if format is unsupported (arrow/avro)
 */
export async function executeDuckDbQuery(
  sql: string,
  options?: DuckDbExecutionOptions,
): Promise<DuckDbResult | null> {
  if (duckDbState.status !== "available") {
    throw new Error("DuckDB is not available");
  }

  const { binPath, version } = duckDbState;
  const format = options?.format?.toLowerCase() ?? "csv";

  // Unsupported formats fall back to sqlp
  if (format === "arrow" || format === "avro") {
    return null;
  }

  const timeoutMs = options?.timeoutMs ?? config.operationTimeoutMs;
  const maxOutputSize = config.maxOutputSize;

  // Build the SQL to execute
  let fullSql = "";

  // Decimal comma setting
  if (options?.decimalComma) {
    fullSql += "SET decimal_separator = ',';\n";
  }

  if (format === "parquet") {
    // For parquet output, use COPY TO
    const outputFile = options?.outputFile;
    if (!outputFile) {
      throw new Error("output_file is required for parquet format output with DuckDB");
    }
    const normalizedOutput = outputFile.replace(/\\/g, "/").replace(/'/g, "''");
    const codec = options?.compression ?? "zstd";
    fullSql += `COPY (${sql}) TO '${normalizedOutput}' (FORMAT PARQUET, CODEC '${codec}');`;
  } else {
    fullSql += sql;
  }

  // Build DuckDB arguments
  const args: string[] = [];
  if (format === "csv") {
    args.push("-csv");
  } else if (format === "json") {
    args.push("-json");
  }
  // For parquet format, no output flag needed (COPY handles it)
  args.push("-c", fullSql);

  return new Promise((resolve, reject) => {
    const proc = spawn(binPath, args, {
      stdio: ["ignore", "pipe", "pipe"],
    });

    options?.onSpawn?.(proc);

    let stdout = "";
    let stderr = "";
    let stdoutTruncated = false;
    let timedOut = false;
    let timer: ReturnType<typeof setTimeout> | null = null;

    timer = setTimeout(() => {
      timedOut = true;
      proc.kill("SIGTERM");
      setTimeout(() => {
        if (proc.exitCode === null) {
          try { proc.kill("SIGKILL"); } catch { /* ignore */ }
          proc.unref();
        }
      }, 1000);
    }, timeoutMs);

    proc.stdout!.on("data", (chunk) => {
      const data = chunk.toString();
      if (stdout.length + data.length > maxOutputSize) {
        if (!stdoutTruncated) {
          stdoutTruncated = true;
          stdout += "\n\n[OUTPUT TRUNCATED - Result too large. Use --format parquet with --output to write to file.]\n";
        }
        return;
      }
      stdout += data;
    });

    const maxStderrSize = 1024 * 1024; // 1 MB cap for stderr
    proc.stderr!.on("data", (chunk) => {
      if (stderr.length < maxStderrSize) {
        stderr += chunk.toString();
        if (stderr.length > maxStderrSize) {
          stderr = stderr.slice(0, maxStderrSize) + "\n[STDERR TRUNCATED]";
        }
      }
    });

    proc.on("close", (exitCode) => {
      if (timer) clearTimeout(timer);
      options?.onExit?.(proc);

      if (timedOut) {
        resolve({
          output: stdout,
          version,
          exitCode: 124,
          stderr: stderr + `\n[TIMEOUT] DuckDB query exceeded ${timeoutMs}ms timeout.`,
        });
        return;
      }

      resolve({
        output: format === "parquet" ? `Parquet file written to: ${options?.outputFile}` : stdout,
        version,
        exitCode: exitCode ?? 0,
        stderr,
      });
    });

    proc.on("error", (err) => {
      if (timer) clearTimeout(timer);
      options?.onExit?.(proc);

      // Binary-level failure (e.g., ENOENT)
      reject(err);
    });
  });
}
