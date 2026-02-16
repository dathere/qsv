/**
 * Tests for DuckDB integration module
 */

import { test, describe, beforeEach } from "node:test";
import assert from "node:assert";
import {
  translateSql,
  isDuckDbEnabled,
  getDuckDbStatus,
  resetDuckDbState,
  detectDuckDb,
  markDuckDbUnavailable,
  MULTI_TABLE_PATTERN,
  normalizeTableRefs,
} from "../src/duckdb.js";

// ============================================================
// SQL Translation Tests
// ============================================================
describe("translateSql", () => {
  test("replaces _t_1 with read_parquet for .parquet files", () => {
    const sql = "SELECT * FROM _t_1 WHERE id > 10";
    const result = translateSql(sql, "/data/test.parquet");
    assert.strictEqual(
      result,
      "SELECT * FROM (read_parquet('/data/test.parquet')) AS _t_1 WHERE id > 10",
    );
  });

  test("replaces _t_1 with read_csv for .csv files", () => {
    const sql = "SELECT * FROM _t_1";
    const result = translateSql(sql, "/data/test.csv");
    assert.strictEqual(
      result,
      "SELECT * FROM (read_csv('/data/test.csv', auto_detect = true)) AS _t_1",
    );
  });

  test("replaces _t_1 with read_json for .jsonl files", () => {
    const sql = "SELECT * FROM _t_1";
    const result = translateSql(sql, "/data/test.jsonl");
    assert.strictEqual(
      result,
      "SELECT * FROM (read_json('/data/test.jsonl')) AS _t_1",
    );
  });

  test("replaces _t_1 with read_json for .ndjson files", () => {
    const sql = "SELECT * FROM _t_1";
    const result = translateSql(sql, "/data/test.ndjson");
    assert.strictEqual(
      result,
      "SELECT * FROM (read_json('/data/test.ndjson')) AS _t_1",
    );
  });

  test("case-insensitive _t_1 replacement preserves qualified column refs", () => {
    const sql = "SELECT * FROM _T_1 WHERE _t_1.id > 0";
    const result = translateSql(sql, "/data/test.parquet");
    // _T_1 (standalone) is replaced with aliased read expression;
    // _t_1.id (qualified column ref) is preserved via the alias
    assert.strictEqual(
      result,
      "SELECT * FROM (read_parquet('/data/test.parquet')) AS _t_1 WHERE _t_1.id > 0",
    );
  });

  test("includes delimiter option in read_csv", () => {
    const sql = "SELECT * FROM _t_1";
    const result = translateSql(sql, "/data/test.tsv", { delimiter: "\t" });
    assert.strictEqual(
      result,
      "SELECT * FROM (read_csv('/data/test.tsv', delim = '\t')) AS _t_1",
    );
  });

  test("includes null values option in read_csv", () => {
    const sql = "SELECT * FROM _t_1";
    const result = translateSql(sql, "/data/test.csv", {
      rnullValues: "NA, N/A, null",
    });
    assert.strictEqual(
      result,
      "SELECT * FROM (read_csv('/data/test.csv', nullstr = ['NA', 'N/A', 'null'])) AS _t_1",
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
      "SELECT * FROM (read_csv('/data/test.csv', delim = ';', nullstr = ['NA'])) AS _t_1",
    );
  });

  test("normalizes Windows backslashes in paths", () => {
    const sql = "SELECT * FROM _t_1";
    const result = translateSql(sql, "C:\\Users\\data\\test.parquet");
    assert.strictEqual(
      result,
      "SELECT * FROM (read_parquet('C:/Users/data/test.parquet')) AS _t_1",
    );
  });

  test("escapes single quotes in file paths", () => {
    const sql = "SELECT * FROM _t_1";
    const result = translateSql(sql, "/data/it's a test.parquet");
    assert.strictEqual(
      result,
      "SELECT * FROM (read_parquet('/data/it''s a test.parquet')) AS _t_1",
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
      "SELECT * FROM (read_parquet('/data/test''); DROP TABLE x; --.parquet')) AS _t_1",
    );
  });

  test("handles .tsv files as CSV-like", () => {
    const sql = "SELECT * FROM _t_1";
    const result = translateSql(sql, "/data/test.tsv");
    assert.strictEqual(
      result,
      "SELECT * FROM (read_csv('/data/test.tsv', auto_detect = true)) AS _t_1",
    );
  });

  test("handles .ssv files as CSV-like", () => {
    const sql = "SELECT * FROM _t_1";
    const result = translateSql(sql, "/data/test.ssv");
    assert.strictEqual(
      result,
      "SELECT * FROM (read_csv('/data/test.ssv', auto_detect = true)) AS _t_1",
    );
  });

  test("treats unknown file extensions as CSV-like", () => {
    const sql = "SELECT * FROM _t_1";

    const xlsxResult = translateSql(sql, "/data/test.xlsx");
    assert.strictEqual(
      xlsxResult,
      "SELECT * FROM (read_csv('/data/test.xlsx', auto_detect = true)) AS _t_1",
    );

    const txtResult = translateSql(sql, "/data/test.txt");
    assert.strictEqual(
      txtResult,
      "SELECT * FROM (read_csv('/data/test.txt', auto_detect = true)) AS _t_1",
    );

    const noExtResult = translateSql(sql, "/data/testfile");
    assert.strictEqual(
      noExtResult,
      "SELECT * FROM (read_csv('/data/testfile', auto_detect = true)) AS _t_1",
    );
  });

  test("does not replace _t_1 inside single-quoted SQL string literals", () => {
    const sql = "SELECT '_t_1' AS label FROM _t_1";
    const result = translateSql(sql, "/data/test.parquet");
    assert.strictEqual(
      result,
      "SELECT '_t_1' AS label FROM (read_parquet('/data/test.parquet')) AS _t_1",
    );
  });

  test("handles escaped quotes inside SQL string literals", () => {
    const sql = "SELECT 'it''s _t_1' AS label FROM _t_1";
    const result = translateSql(sql, "/data/test.parquet");
    assert.strictEqual(
      result,
      "SELECT 'it''s _t_1' AS label FROM (read_parquet('/data/test.parquet')) AS _t_1",
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
    // Standalone _t_1 (FROM) is translated with alias; _t_1.id preserved via alias; _t_2 untouched
    assert.ok(result.includes("(read_parquet('/data/test.parquet')) AS _t_1"));
    assert.ok(result.includes("_t_1.id"));
    assert.ok(result.includes("_t_2"));
  });

  test("multiple standalone _t_1 refs — only first gets alias", () => {
    const sql =
      "SELECT * FROM _t_1 UNION SELECT * FROM _t_1 WHERE _t_1.val > 5";
    const result = translateSql(sql, "/data/test.parquet");
    // First standalone _t_1 gets aliased, second gets bare readExpr, qualified ref preserved
    assert.strictEqual(
      result,
      "SELECT * FROM (read_parquet('/data/test.parquet')) AS _t_1 UNION SELECT * FROM read_parquet('/data/test.parquet') WHERE _t_1.val > 5",
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
