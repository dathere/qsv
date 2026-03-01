/**
 * Tests for DuckDB integration module
 */

import { test, describe, beforeEach, after } from "node:test";
import assert from "node:assert";
import { existsSync, statSync } from "fs";
import { join, dirname } from "path";
import { fileURLToPath } from "url";
import {
  translateSql,
  isDuckDbEnabled,
  getDuckDbStatus,
  resetDuckDbState,
  detectDuckDb,
  executeDuckDbQuery,
  markDuckDbUnavailable,
  MULTI_TABLE_PATTERN,
  normalizeTableRefs,
} from "../src/duckdb.js";
import { config } from "../src/config.js";
import { handleToParquetCall } from "../src/mcp-tools.js";
import { createTestDir, cleanupTestDir, createTestCSV, QSV_AVAILABLE } from "./test-helpers.js";

// ============================================================
// SQL Translation Tests
// ============================================================
describe("translateSql", () => {
  test("replaces _t_1 with read_parquet for .parquet files", () => {
    const sql = "SELECT * FROM _t_1 WHERE id > 10";
    const result = translateSql(sql, "/data/test.parquet");
    assert.strictEqual(
      result,
      "SELECT * FROM read_parquet('/data/test.parquet') AS _tbl WHERE id > 10",
    );
  });

  test("replaces _t_1 with read_csv for .csv files", () => {
    const sql = "SELECT * FROM _t_1";
    const result = translateSql(sql, "/data/test.csv");
    assert.strictEqual(
      result,
      "SELECT * FROM read_csv('/data/test.csv', auto_detect = true) AS _tbl",
    );
  });

  test("replaces _t_1 with read_json for .jsonl files", () => {
    const sql = "SELECT * FROM _t_1";
    const result = translateSql(sql, "/data/test.jsonl");
    assert.strictEqual(
      result,
      "SELECT * FROM read_json('/data/test.jsonl') AS _tbl",
    );
  });

  test("replaces _t_1 with read_json for .ndjson files", () => {
    const sql = "SELECT * FROM _t_1";
    const result = translateSql(sql, "/data/test.ndjson");
    assert.strictEqual(
      result,
      "SELECT * FROM read_json('/data/test.ndjson') AS _tbl",
    );
  });

  test("case-insensitive _t_1 replacement preserves qualified column refs", () => {
    const sql = "SELECT * FROM _T_1 WHERE _t_1.id > 0";
    const result = translateSql(sql, "/data/test.parquet");
    // _T_1 (standalone) is replaced with aliased read expression;
    // _t_1.id (qualified column ref) is rewritten to _tbl.id via the alias
    assert.strictEqual(
      result,
      "SELECT * FROM read_parquet('/data/test.parquet') AS _tbl WHERE _tbl.id > 0",
    );
  });

  test("includes delimiter option in read_csv", () => {
    const sql = "SELECT * FROM _t_1";
    const result = translateSql(sql, "/data/test.tsv", { delimiter: "\t" });
    assert.strictEqual(
      result,
      "SELECT * FROM read_csv('/data/test.tsv', delim = '\t') AS _tbl",
    );
  });

  test("includes null values option in read_csv", () => {
    const sql = "SELECT * FROM _t_1";
    const result = translateSql(sql, "/data/test.csv", {
      rnullValues: "NA, N/A, null",
    });
    assert.strictEqual(
      result,
      "SELECT * FROM read_csv('/data/test.csv', nullstr = ['NA', 'N/A', 'null']) AS _tbl",
    );
  });

  test("includes both delimiter and null values", () => {
    const sql = "SELECT * FROM _t_1";
    const result = translateSql(sql, "/data/test.csv", {
      delimiter: ";",
      rnullValues: "NA",
    });
    assert.strictEqual(
      result,
      "SELECT * FROM read_csv('/data/test.csv', delim = ';', nullstr = ['NA']) AS _tbl",
    );
  });

  test("normalizes Windows backslashes in paths", () => {
    const sql = "SELECT * FROM _t_1";
    const result = translateSql(sql, "C:\\Users\\data\\test.parquet");
    assert.strictEqual(
      result,
      "SELECT * FROM read_parquet('C:/Users/data/test.parquet') AS _tbl",
    );
  });

  test("escapes single quotes in file paths", () => {
    const sql = "SELECT * FROM _t_1";
    const result = translateSql(sql, "/data/it's a test.parquet");
    assert.strictEqual(
      result,
      "SELECT * FROM read_parquet('/data/it''s a test.parquet') AS _tbl",
    );
  });

  test("escapes SQL injection attempts in file paths", () => {
    const sql = "SELECT * FROM _t_1";
    const result = translateSql(sql, "/data/test'); DROP TABLE x; --.parquet");
    // Single quotes in the path are doubled, so the malicious payload becomes
    // a literal string inside read_parquet(). True injection safety relies on
    // DuckDB treating the doubled-quote content as a filename string, not
    // executable SQL — which is standard SQL string literal behavior.
    assert.strictEqual(
      result,
      "SELECT * FROM read_parquet('/data/test''); DROP TABLE x; --.parquet') AS _tbl",
    );
  });

  test("handles .tsv files as CSV-like", () => {
    const sql = "SELECT * FROM _t_1";
    const result = translateSql(sql, "/data/test.tsv");
    assert.strictEqual(
      result,
      "SELECT * FROM read_csv('/data/test.tsv', auto_detect = true) AS _tbl",
    );
  });

  test("handles .ssv files as CSV-like", () => {
    const sql = "SELECT * FROM _t_1";
    const result = translateSql(sql, "/data/test.ssv");
    assert.strictEqual(
      result,
      "SELECT * FROM read_csv('/data/test.ssv', auto_detect = true) AS _tbl",
    );
  });

  test("treats unknown file extensions as CSV-like", () => {
    const sql = "SELECT * FROM _t_1";

    const xlsxResult = translateSql(sql, "/data/test.xlsx");
    assert.strictEqual(
      xlsxResult,
      "SELECT * FROM read_csv('/data/test.xlsx', auto_detect = true) AS _tbl",
    );

    const txtResult = translateSql(sql, "/data/test.txt");
    assert.strictEqual(
      txtResult,
      "SELECT * FROM read_csv('/data/test.txt', auto_detect = true) AS _tbl",
    );

    const noExtResult = translateSql(sql, "/data/testfile");
    assert.strictEqual(
      noExtResult,
      "SELECT * FROM read_csv('/data/testfile', auto_detect = true) AS _tbl",
    );
  });

  test("does not replace _t_1 inside single-quoted SQL string literals", () => {
    const sql = "SELECT '_t_1' AS label FROM _t_1";
    const result = translateSql(sql, "/data/test.parquet");
    assert.strictEqual(
      result,
      "SELECT '_t_1' AS label FROM read_parquet('/data/test.parquet') AS _tbl",
    );
  });

  test("handles escaped quotes inside SQL string literals", () => {
    const sql = "SELECT 'it''s _t_1' AS label FROM _t_1";
    const result = translateSql(sql, "/data/test.parquet");
    assert.strictEqual(
      result,
      "SELECT 'it''s _t_1' AS label FROM read_parquet('/data/test.parquet') AS _tbl",
    );
  });

  test("rejects multi-character delimiter strings", () => {
    const sql = "SELECT * FROM _t_1";
    assert.throws(
      () => translateSql(sql, "/data/test.csv", { delimiter: "\\t" }),
      /must be exactly 1 character/,
    );
  });

  test("does not replace partial matches like _t_10 or _t_1x", () => {
    const sql = "SELECT * FROM _t_10";
    const result = translateSql(sql, "/data/test.parquet");
    // _t_10 should NOT be replaced (word boundary prevents it)
    assert.strictEqual(
      result,
      "SELECT * FROM _t_10",
    );
  });

  test("preserves SQL without _t_1 references", () => {
    const sql = "SELECT * FROM read_parquet('/other/file.parquet')";
    const result = translateSql(sql, "/data/test.parquet");
    assert.strictEqual(
      result,
      "SELECT * FROM read_parquet('/other/file.parquet')",
    );
  });

  test("only translates _t_1 — _t_2 and higher are left untranslated", () => {
    const sql = "SELECT * FROM _t_1 JOIN _t_2 ON _t_1.id = _t_2.id";
    const result = translateSql(sql, "/data/test.parquet");
    // Standalone _t_1 (FROM) is translated with alias; _t_1.id rewritten to _tbl.id; _t_2 untouched
    assert.ok(result.includes("read_parquet('/data/test.parquet') AS _tbl"));
    assert.ok(result.includes("_tbl.id"));
    assert.ok(result.includes("_t_2"));
  });

  test("multiple standalone _t_1 refs — only first gets alias", () => {
    const sql =
      "SELECT * FROM _t_1 UNION SELECT * FROM _t_1 WHERE _t_1.val > 5";
    const result = translateSql(sql, "/data/test.parquet");
    // First standalone _t_1 gets aliased, second gets bare readExpr, qualified ref rewritten to _tbl
    assert.strictEqual(
      result,
      "SELECT * FROM read_parquet('/data/test.parquet') AS _tbl UNION SELECT * FROM read_parquet('/data/test.parquet') WHERE _tbl.val > 5",
    );
  });

  test("multi-table regex detects _t_2, _t_3, _t_10 references", () => {
    assert.ok(MULTI_TABLE_PATTERN.test("SELECT * FROM _t_1 JOIN _t_2"));
    assert.ok(MULTI_TABLE_PATTERN.test("SELECT * FROM _t_1, _t_3"));
    assert.ok(MULTI_TABLE_PATTERN.test("SELECT * FROM _t_10"));
    assert.ok(!MULTI_TABLE_PATTERN.test("SELECT * FROM _t_1"));
    assert.ok(!MULTI_TABLE_PATTERN.test("SELECT * FROM _t_1 WHERE x > 0"));
    // Case-insensitive: uppercase _T_N also matches (defense-in-depth)
    assert.ok(MULTI_TABLE_PATTERN.test("SELECT * FROM _T_2"));
    assert.ok(MULTI_TABLE_PATTERN.test("SELECT * FROM _T_10"));
  });

  test("normalizeTableRefs lowercases _T_N references", () => {
    assert.strictEqual(
      normalizeTableRefs("SELECT * FROM _T_1 JOIN _T_2 ON _T_1.id = _T_2.id"),
      "SELECT * FROM _t_1 JOIN _t_2 ON _t_1.id = _t_2.id",
    );
    // Already lowercase — no change
    assert.strictEqual(
      normalizeTableRefs("SELECT * FROM _t_1 JOIN _t_2"),
      "SELECT * FROM _t_1 JOIN _t_2",
    );
    // Mixed case
    assert.strictEqual(
      normalizeTableRefs("SELECT * FROM _T_10"),
      "SELECT * FROM _t_10",
    );
    // No table refs — unchanged
    assert.strictEqual(
      normalizeTableRefs("SELECT 1"),
      "SELECT 1",
    );
    // Lowercase _t_N passes through unchanged (idempotent)
    assert.strictEqual(
      normalizeTableRefs("SELECT * FROM _t_1 WHERE _t_2.x > 0"),
      "SELECT * FROM _t_1 WHERE _t_2.x > 0",
    );
  });

  test("normalizeTableRefs + MULTI_TABLE_PATTERN handles uppercase agents", () => {
    const sql = "SELECT * FROM _T_2";
    const normalized = normalizeTableRefs(sql);
    assert.ok(MULTI_TABLE_PATTERN.test(normalized));
  });
});

// ============================================================
// Detection State Tests
// ============================================================
describe("DuckDB detection state", () => {
  beforeEach(() => {
    resetDuckDbState();
  });

  test("initial state is pending", () => {
    const status = getDuckDbStatus();
    assert.strictEqual(status.status, "pending");
  });

  test("markDuckDbUnavailable sets state correctly", () => {
    markDuckDbUnavailable("test reason");
    const status = getDuckDbStatus();
    assert.strictEqual(status.status, "unavailable");
    if (status.status === "unavailable") {
      assert.strictEqual(status.reason, "test reason");
    }
  });

  test("state is sticky after detection", () => {
    // After first detection, state should not change
    detectDuckDb(); // Will detect or not, but state becomes non-pending
    const firstStatus = getDuckDbStatus();
    assert.notStrictEqual(firstStatus.status, "pending");

    // Calling again should return same state
    detectDuckDb();
    const secondStatus = getDuckDbStatus();
    assert.deepStrictEqual(secondStatus, firstStatus);
  });

  test("isDuckDbEnabled returns false by default", () => {
    // Default config has useDuckDb: false (opt-in)
    assert.strictEqual(isDuckDbEnabled(), false);
  });
});

// ============================================================
// DuckDB Live Integration Tests
// ============================================================

// Snapshot the original config value once so both the IIFE and the describe
// block's after() hook restore to the same value (avoids dual save/restore).
const savedUseDuckDb = config.useDuckDb;

// Detect DuckDB availability for skip flags (must run at module scope since
// node:test evaluates `skip` options at registration time). Uses try/catch/finally
// to guarantee config is restored even if detection throws.
// NOTE: This temporarily mutates config.useDuckDb because detectDuckDb() checks
// isDuckDbEnabled() which reads config.useDuckDb (defaults to false). The mutation
// is scoped to the IIFE and restored in the finally block. Tests in this file must
// run serially (the default for node:test) to avoid cross-test config races.
const DUCKDB_AVAILABLE = (() => {
  try {
    (config as Record<string, unknown>).useDuckDb = true;
    resetDuckDbState();
    return detectDuckDb().status === "available";
  } catch {
    return false;
  } finally {
    (config as Record<string, unknown>).useDuckDb = savedUseDuckDb;
    resetDuckDbState();
  }
})();

// 100-row NYC 311 sample fixture (55KB, checked into the repo)
// Resolve from project root (dist/ compiled output → source tests/fixtures/)
const TESTS_DIR = dirname(fileURLToPath(import.meta.url));
const PROJECT_ROOT = join(TESTS_DIR, "..", "..");
const NYC_311_FILE = join(PROJECT_ROOT, "tests", "fixtures", "nyc311-100.csv");

/** Escape a file path for use inside a SQL single-quoted string literal.
 *  Only safe for trusted paths (test fixtures); not a general-purpose SQL escaper. */
function sqlPath(p: string): string {
  return p.replace(/\\/g, "/").replace(/'/g, "''");
}

const NYC_311_SQL = sqlPath(NYC_311_FILE);

describe("DuckDB live integration", { concurrency: false }, () => {
  beforeEach(() => {
    // Enable DuckDB and reset state for each test
    (config as Record<string, unknown>).useDuckDb = true;
    resetDuckDbState();
    detectDuckDb();
  });

  after(() => {
    (config as Record<string, unknown>).useDuckDb = savedUseDuckDb;
    resetDuckDbState();
  });

  // No `skip: !DUCKDB_AVAILABLE` — this test exercises the "unavailable" path
  // and works regardless of whether the real binary is installed.
  test("executeDuckDbQuery throws when DuckDB is unavailable", async () => {
    markDuckDbUnavailable("test: simulating unavailable");
    try {
      await assert.rejects(
        () => executeDuckDbQuery("SELECT 1"),
        { message: /not available/i },
        "executeDuckDbQuery should throw when DuckDB is unavailable",
      );
    } finally {
      // Restore to "pending" so beforeEach/afterEach re-detection works cleanly.
      // resetDuckDbState sets status to "pending", fully clearing the state
      // set by markDuckDbUnavailable. detectDuckDb then re-resolves from scratch.
      resetDuckDbState();
      detectDuckDb();
    }
  });

  test("detectDuckDb finds real binary and reports version", { skip: !DUCKDB_AVAILABLE }, () => {
    const status = getDuckDbStatus();
    assert.strictEqual(status.status, "available");
    assert.ok(status.binPath.length > 0, "binPath should be non-empty");
    assert.match(status.version, /^\d+\.\d+\.\d+$/, "version should be semver-like");
  });

  test("simple CSV query returns header + data rows", { skip: !DUCKDB_AVAILABLE, timeout: 30_000 }, async () => {
    const sql = `SELECT "Complaint Type", COUNT(*) as cnt FROM read_csv('${NYC_311_SQL}', auto_detect = true) GROUP BY "Complaint Type" ORDER BY cnt DESC LIMIT 5`;
    const result = await executeDuckDbQuery(sql, { format: "csv" });
    assert.ok(result, "result should not be null");
    assert.strictEqual(result.exitCode, 0, `DuckDB failed: ${result.stderr}`);

    const lines = result.output.trim().split("\n");
    // Header + 5 data rows
    assert.strictEqual(lines.length, 6, `Expected 6 lines (header + 5 rows), got ${lines.length}`);
    // Header should contain our column names
    assert.ok(lines[0].includes("Complaint Type"), "Header should contain 'Complaint Type'");
    assert.ok(lines[0].includes("cnt"), "Header should contain 'cnt'");
  });

  test("JSON output returns valid JSON array", { skip: !DUCKDB_AVAILABLE, timeout: 30_000 }, async () => {
    const sql = `SELECT "Complaint Type", COUNT(*) as cnt FROM read_csv('${NYC_311_SQL}', auto_detect = true) GROUP BY "Complaint Type" ORDER BY cnt DESC LIMIT 5`;
    const result = await executeDuckDbQuery(sql, { format: "json" });
    assert.ok(result, "result should not be null");
    assert.strictEqual(result.exitCode, 0, `DuckDB failed: ${result.stderr}`);

    const parsed = JSON.parse(result.output);
    assert.ok(Array.isArray(parsed), "JSON output should be an array");
    assert.strictEqual(parsed.length, 5, "Should have 5 rows");
    assert.ok("Complaint Type" in parsed[0], "First row should have 'Complaint Type' key");
    assert.ok("cnt" in parsed[0], "First row should have 'cnt' key");
  });

  test("parquet output writes valid file", { skip: !DUCKDB_AVAILABLE, timeout: 30_000 }, async () => {
    const dir = await createTestDir("duckdb-parquet");
    try {
      const outputFile = join(dir, "test-output.parquet");
      const sql = `SELECT "Complaint Type", COUNT(*) as cnt FROM read_csv('${NYC_311_SQL}', auto_detect = true) GROUP BY "Complaint Type" ORDER BY cnt DESC LIMIT 5`;
      const result = await executeDuckDbQuery(sql, {
        format: "parquet",
        outputFile,
      });
      assert.ok(result, "result should not be null");
      assert.strictEqual(result.exitCode, 0, `DuckDB failed: ${result.stderr}`);
      assert.ok(existsSync(outputFile), "Parquet file should exist");
      const fileStats = statSync(outputFile);
      assert.ok(fileStats.size > 0, "Parquet file should be non-empty");
    } finally {
      await cleanupTestDir(dir);
    }
  });

  test("invalid SQL returns non-zero exit code", { skip: !DUCKDB_AVAILABLE, timeout: 30_000 }, async () => {
    const result = await executeDuckDbQuery("SELECT * FROM nonexistent_table_xyz");
    assert.ok(result, "result should not be null");
    assert.notStrictEqual(result.exitCode, 0, "Invalid SQL should produce non-zero exit code");
    assert.ok(result.stderr.length > 0, "stderr should contain error message");
  });

  test("multi-column grouping returns correct columns", { skip: !DUCKDB_AVAILABLE, timeout: 30_000 }, async () => {
    const sql = `SELECT "Agency", "Borough", COUNT(*) as cnt FROM read_csv('${NYC_311_SQL}', auto_detect = true) GROUP BY "Agency", "Borough" ORDER BY cnt DESC LIMIT 3`;
    const result = await executeDuckDbQuery(sql, { format: "csv" });
    assert.ok(result, "result should not be null");
    assert.strictEqual(result.exitCode, 0, `DuckDB failed: ${result.stderr}`);

    const lines = result.output.trim().split("\n");
    assert.strictEqual(lines.length, 4, `Expected 4 lines (header + 3 rows), got ${lines.length}`);
    const header = lines[0];
    assert.ok(header.includes("Agency"), "Header should contain 'Agency'");
    assert.ok(header.includes("Borough"), "Header should contain 'Borough'");
    assert.ok(header.includes("cnt"), "Header should contain 'cnt'");
  });

  test("qsv_to_parquet → DuckDB SQL query end-to-end", { skip: !(DUCKDB_AVAILABLE && QSV_AVAILABLE), timeout: 30_000 }, async () => {
    const dir = await createTestDir("duckdb-qsv-parquet");
    try {
      // Create a small test CSV in the temp dir
      const csvPath = await createTestCSV(dir, "cities.csv", [
        "city,state,population",
        "New York,NY,8336817",
        "Los Angeles,CA,3979576",
        "Chicago,IL,2693976",
        "Houston,TX,2320268",
        "Phoenix,AZ,1680992",
      ].join("\n") + "\n");

      const parquetPath = join(dir, "cities.parquet");

      // Step 1: Convert CSV to Parquet using qsv_to_parquet (the MCP tool)
      const convertResult = await handleToParquetCall({
        input_file: csvPath,
        output_file: parquetPath,
      });

      assert.ok(!convertResult.isError, `qsv_to_parquet failed: ${convertResult.content[0]?.text}`);
      assert.ok(existsSync(parquetPath), "Parquet file should exist after conversion");
      const parquetStats = statSync(parquetPath);
      assert.ok(parquetStats.size > 0, "Parquet file should be non-empty");

      // Step 2: Query the Parquet file with DuckDB
      const sql = `SELECT city, population FROM read_parquet('${sqlPath(parquetPath)}') WHERE population > 3000000 ORDER BY population DESC`;
      const result = await executeDuckDbQuery(sql, { format: "csv" });
      assert.ok(result, "DuckDB result should not be null");
      assert.strictEqual(result.exitCode, 0, `DuckDB query failed: ${result.stderr}`);

      const lines = result.output.trim().split("\n");
      // Header + 2 data rows (New York 8.3M, Los Angeles 3.9M)
      assert.strictEqual(lines.length, 3, `Expected 3 lines (header + 2 rows), got ${lines.length}: ${result.output}`);
      assert.ok(lines[0].includes("city"), "Header should contain 'city'");
      assert.ok(lines[0].includes("population"), "Header should contain 'population'");
      assert.ok(lines[1].includes("New York"), "First row should be New York");
      assert.ok(lines[2].includes("Los Angeles"), "Second row should be Los Angeles");
    } finally {
      await cleanupTestDir(dir);
    }
  });

  test("translateSql + executeDuckDbQuery end-to-end with WHERE clause", {
    skip: !(DUCKDB_AVAILABLE && QSV_AVAILABLE), timeout: 30_000,
  }, async () => {
    const sql = translateSql(
      `SELECT COUNT(*) as total FROM _t_1 WHERE "Borough" = 'BROOKLYN'`,
      NYC_311_FILE,
    );
    // Verify translation happened — read_csv should be present, and _tbl is the alias
    assert.ok(sql.includes("read_csv"), "SQL should contain read_csv after translation");
    assert.ok(sql.includes("AS _tbl"), "Translated SQL should alias the table as _tbl");

    const result = await executeDuckDbQuery(sql, { format: "csv" });
    assert.ok(result, "result should not be null");
    assert.strictEqual(result.exitCode, 0, `DuckDB failed: ${result.stderr}`);

    const lines = result.output.trim().split("\n");
    assert.strictEqual(lines.length, 2, "Should have header + 1 data row");
    const count = parseInt(lines[1], 10);
    assert.ok(count > 0, `Brooklyn count should be positive, got ${count}`);
  });
});
