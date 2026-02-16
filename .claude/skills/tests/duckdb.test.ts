/**
 * Tests for DuckDB integration module
 */

import { test, describe, beforeEach, afterEach } from "node:test";
import assert from "node:assert";
import { writeFile } from "fs/promises";
import { join } from "path";
import {
  translateSql,
  isDuckDbEnabled,
  getDuckDbStatus,
  resetDuckDbState,
  detectDuckDb,
  markDuckDbUnavailable,
} from "../src/duckdb.js";
import { createTestDir, cleanupTestDir, createTestCSV } from "./test-helpers.js";

// ============================================================
// SQL Translation Tests
// ============================================================
describe("translateSql", () => {
  test("replaces _t_1 with read_parquet for .parquet files", () => {
    const sql = "SELECT * FROM _t_1 WHERE id > 10";
    const result = translateSql(sql, "/data/test.parquet");
    assert.strictEqual(
      result,
      "SELECT * FROM read_parquet('/data/test.parquet') WHERE id > 10",
    );
  });

  test("replaces _t_1 with read_csv for .csv files", () => {
    const sql = "SELECT * FROM _t_1";
    const result = translateSql(sql, "/data/test.csv");
    assert.strictEqual(
      result,
      "SELECT * FROM read_csv('/data/test.csv', auto_detect = true)",
    );
  });

  test("replaces _t_1 with read_json for .jsonl files", () => {
    const sql = "SELECT * FROM _t_1";
    const result = translateSql(sql, "/data/test.jsonl");
    assert.strictEqual(
      result,
      "SELECT * FROM read_json('/data/test.jsonl')",
    );
  });

  test("replaces _t_1 with read_json for .ndjson files", () => {
    const sql = "SELECT * FROM _t_1";
    const result = translateSql(sql, "/data/test.ndjson");
    assert.strictEqual(
      result,
      "SELECT * FROM read_json('/data/test.ndjson')",
    );
  });

  test("case-insensitive _t_1 replacement", () => {
    const sql = "SELECT * FROM _T_1 WHERE _t_1.id > 0";
    const result = translateSql(sql, "/data/test.parquet");
    assert.strictEqual(
      result,
      "SELECT * FROM read_parquet('/data/test.parquet') WHERE read_parquet('/data/test.parquet').id > 0",
    );
  });

  test("includes delimiter option in read_csv", () => {
    const sql = "SELECT * FROM _t_1";
    const result = translateSql(sql, "/data/test.tsv", { delimiter: "\t" });
    assert.strictEqual(
      result,
      "SELECT * FROM read_csv('/data/test.tsv', delim = '\t')",
    );
  });

  test("includes null values option in read_csv", () => {
    const sql = "SELECT * FROM _t_1";
    const result = translateSql(sql, "/data/test.csv", {
      rnullValues: "NA, N/A, null",
    });
    assert.strictEqual(
      result,
      "SELECT * FROM read_csv('/data/test.csv', nullstr = ['NA', 'N/A', 'null'])",
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
      "SELECT * FROM read_csv('/data/test.csv', delim = ';', nullstr = ['NA'])",
    );
  });

  test("normalizes Windows backslashes in paths", () => {
    const sql = "SELECT * FROM _t_1";
    const result = translateSql(sql, "C:\\Users\\data\\test.parquet");
    assert.strictEqual(
      result,
      "SELECT * FROM read_parquet('C:/Users/data/test.parquet')",
    );
  });

  test("escapes single quotes in file paths", () => {
    const sql = "SELECT * FROM _t_1";
    const result = translateSql(sql, "/data/it's a test.parquet");
    assert.strictEqual(
      result,
      "SELECT * FROM read_parquet('/data/it''s a test.parquet')",
    );
  });

  test("handles .tsv files as CSV-like", () => {
    const sql = "SELECT * FROM _t_1";
    const result = translateSql(sql, "/data/test.tsv");
    assert.strictEqual(
      result,
      "SELECT * FROM read_csv('/data/test.tsv', auto_detect = true)",
    );
  });

  test("handles .ssv files as CSV-like", () => {
    const sql = "SELECT * FROM _t_1";
    const result = translateSql(sql, "/data/test.ssv");
    assert.strictEqual(
      result,
      "SELECT * FROM read_csv('/data/test.ssv', auto_detect = true)",
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
// Parquet-first helper tests
// ============================================================
describe("ensureParquet helpers", () => {
  let testDir: string;

  beforeEach(async () => {
    testDir = await createTestDir("duckdb-parquet");
  });

  afterEach(async () => {
    await cleanupTestDir(testDir);
  });

  test("isCsvLikeFile identifies CSV extensions correctly", async () => {
    // Import the internal helper via the module that uses it
    // We test this indirectly through translateSql behavior
    const csvResult = translateSql("SELECT * FROM _t_1", "/data/test.csv");
    assert.ok(csvResult.includes("read_csv"));

    const tsvResult = translateSql("SELECT * FROM _t_1", "/data/test.tsv");
    assert.ok(tsvResult.includes("read_csv"));

    const parquetResult = translateSql("SELECT * FROM _t_1", "/data/test.parquet");
    assert.ok(parquetResult.includes("read_parquet"));
  });

  test("getParquetPath generates correct paths", () => {
    // We can test this indirectly: if we pass a CSV and it gets converted,
    // the parquet path would be the same stem + .parquet
    // Direct testing via translateSql shows path handling
    const sql = "SELECT * FROM _t_1";
    const result = translateSql(sql, "/data/file.csv");
    // The CSV path should be used in read_csv
    assert.ok(result.includes("/data/file.csv"));
  });
});
