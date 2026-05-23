/**
 * Tests for the parquet-bridge module.
 *
 * Covers the exported pure helpers that don't require spawning duckdb or qsv:
 *   - detectDelimiter
 *   - parseCSVLine (RFC-4180-style fields with quoting/embedded delimiters)
 *   - isDateDtype (Polars dtype shape detection)
 *   - isCsvLikeFile / getParquetPath / statsFilePath (path utilities)
 *   - suggestDuckDbFixes (error-message → hint mapping)
 *
 * Functions that orchestrate child processes (convertCsvToParquet,
 * ensureParquet, ensureStatsCache, ensurePolarsSchema, tryDuckDbExecution,
 * spawnDuckDbCommands, doParquetConversion) are integration-tested via
 * qsv-integration.test.ts and duckdb.test.ts; this file focuses on the
 * deterministic pieces that are cheap to assert without binaries.
 */

import { test } from "node:test";
import assert from "node:assert";
import {
  detectDelimiter,
  parseCSVLine,
  isDateDtype,
  isCsvLikeFile,
  getParquetPath,
  statsFilePath,
  suggestDuckDbFixes,
} from "../src/parquet-bridge.js";

// ─────────────────────────────────────────────────────────────────────────────
// detectDelimiter
// ─────────────────────────────────────────────────────────────────────────────

test("detectDelimiter returns comma for .csv files", () => {
  assert.strictEqual(detectDelimiter("data.csv"), ",");
  assert.strictEqual(detectDelimiter("/path/to/DATA.CSV"), ",");
});

test("detectDelimiter returns tab for .tsv/.tab variants", () => {
  assert.strictEqual(detectDelimiter("data.tsv"), "\t");
  assert.strictEqual(detectDelimiter("data.tab"), "\t");
  assert.strictEqual(detectDelimiter("data.TSV"), "\t");
  assert.strictEqual(detectDelimiter("data.TAB"), "\t");
});

test("detectDelimiter handles snappy-compressed extensions", () => {
  assert.strictEqual(detectDelimiter("data.tsv.sz"), "\t");
  assert.strictEqual(detectDelimiter("data.tab.sz"), "\t");
  assert.strictEqual(detectDelimiter("data.ssv.sz"), ";");
});

test("detectDelimiter returns semicolon for .ssv files", () => {
  assert.strictEqual(detectDelimiter("data.ssv"), ";");
  assert.strictEqual(detectDelimiter("data.SSV"), ";");
});

test("detectDelimiter defaults to comma for unknown extensions", () => {
  assert.strictEqual(detectDelimiter("data.txt"), ",");
  assert.strictEqual(detectDelimiter("data"), ",");
  assert.strictEqual(detectDelimiter(""), ",");
});

// ─────────────────────────────────────────────────────────────────────────────
// parseCSVLine
// ─────────────────────────────────────────────────────────────────────────────

test("parseCSVLine returns a single empty field for empty input", () => {
  assert.deepStrictEqual(parseCSVLine(""), [""]);
});

test("parseCSVLine handles simple unquoted fields", () => {
  assert.deepStrictEqual(parseCSVLine("a,b,c"), ["a", "b", "c"]);
});

test("parseCSVLine preserves empty fields between commas", () => {
  assert.deepStrictEqual(parseCSVLine("a,,c"), ["a", "", "c"]);
});

test("parseCSVLine recognizes a trailing empty field after a comma", () => {
  assert.deepStrictEqual(parseCSVLine("a,b,"), ["a", "b", ""]);
});

test("parseCSVLine strips outer quotes from quoted fields", () => {
  assert.deepStrictEqual(parseCSVLine('"a","b","c"'), ["a", "b", "c"]);
});

test("parseCSVLine treats delimiters inside quotes as literal text", () => {
  assert.deepStrictEqual(parseCSVLine('"a,b","c"'), ["a,b", "c"]);
});

test("parseCSVLine unescapes doubled quotes inside a quoted field", () => {
  // RFC 4180: "" inside a quoted field represents a single literal "
  assert.deepStrictEqual(parseCSVLine('"he said ""hi""","ok"'), ['he said "hi"', "ok"]);
});

test("parseCSVLine respects a custom tab delimiter", () => {
  assert.deepStrictEqual(parseCSVLine("a\tb\tc", "\t"), ["a", "b", "c"]);
  assert.deepStrictEqual(parseCSVLine('"a\tb"\tc', "\t"), ["a\tb", "c"]);
});

test("parseCSVLine respects a custom semicolon delimiter", () => {
  assert.deepStrictEqual(parseCSVLine("a;b;c", ";"), ["a", "b", "c"]);
});

test("parseCSVLine handles a trailing delimiter after a quoted field", () => {
  assert.deepStrictEqual(parseCSVLine('"a",', ","), ["a", ""]);
});

// ─────────────────────────────────────────────────────────────────────────────
// isDateDtype
// ─────────────────────────────────────────────────────────────────────────────

test("isDateDtype recognizes the string 'Date'", () => {
  assert.strictEqual(isDateDtype("Date"), true);
});

test("isDateDtype recognizes Polars object dtypes with Date or Datetime keys", () => {
  assert.strictEqual(isDateDtype({ Date: null }), true);
  assert.strictEqual(isDateDtype({ Datetime: ["us", null] }), true);
});

test("isDateDtype rejects non-date dtypes", () => {
  assert.strictEqual(isDateDtype("String"), false);
  assert.strictEqual(isDateDtype("Int64"), false);
  assert.strictEqual(isDateDtype("Float64"), false);
  assert.strictEqual(isDateDtype({ Int64: null }), false);
});

test("isDateDtype rejects null / undefined / non-objects", () => {
  assert.strictEqual(isDateDtype(null), false);
  assert.strictEqual(isDateDtype(undefined), false);
  assert.strictEqual(isDateDtype(0), false);
  assert.strictEqual(isDateDtype(false), false);
});

// ─────────────────────────────────────────────────────────────────────────────
// isCsvLikeFile
// ─────────────────────────────────────────────────────────────────────────────

test("isCsvLikeFile recognizes native CSV-family extensions", () => {
  assert.strictEqual(isCsvLikeFile("data.csv"), true);
  assert.strictEqual(isCsvLikeFile("data.tsv"), true);
  assert.strictEqual(isCsvLikeFile("data.tab"), true);
  assert.strictEqual(isCsvLikeFile("data.ssv"), true);
});

test("isCsvLikeFile recognizes snappy-compressed CSV variants", () => {
  assert.strictEqual(isCsvLikeFile("data.csv.sz"), true);
  assert.strictEqual(isCsvLikeFile("data.tsv.sz"), true);
  assert.strictEqual(isCsvLikeFile("data.tab.sz"), true);
  assert.strictEqual(isCsvLikeFile("data.ssv.sz"), true);
});

test("isCsvLikeFile rejects unrelated extensions and parquet", () => {
  assert.strictEqual(isCsvLikeFile("data.parquet"), false);
  assert.strictEqual(isCsvLikeFile("data.json"), false);
  assert.strictEqual(isCsvLikeFile("data.xlsx"), false);
  assert.strictEqual(isCsvLikeFile("README.md"), false);
});

test("isCsvLikeFile only matches the basename, not the directory", () => {
  // A directory containing 'csv' in its name must not falsely qualify.
  assert.strictEqual(isCsvLikeFile("/data/csv_files/notes.json"), false);
});

test("isCsvLikeFile is case-insensitive on the basename", () => {
  assert.strictEqual(isCsvLikeFile("DATA.CSV"), true);
  assert.strictEqual(isCsvLikeFile("Report.Tsv"), true);
});

// ─────────────────────────────────────────────────────────────────────────────
// getParquetPath
// ─────────────────────────────────────────────────────────────────────────────

test("getParquetPath swaps the CSV-family extension for .parquet", () => {
  assert.strictEqual(getParquetPath("/data/x.csv"), "/data/x.parquet");
  assert.strictEqual(getParquetPath("/data/x.tsv"), "/data/x.parquet");
  assert.strictEqual(getParquetPath("/data/x.tab"), "/data/x.parquet");
  assert.strictEqual(getParquetPath("/data/x.ssv"), "/data/x.parquet");
});

test("getParquetPath swaps snappy-compressed extensions correctly", () => {
  // Strips the full '.csv.sz' (not just '.sz') before appending '.parquet'.
  assert.strictEqual(getParquetPath("/data/x.csv.sz"), "/data/x.parquet");
  assert.strictEqual(getParquetPath("/data/x.tsv.sz"), "/data/x.parquet");
});

test("getParquetPath appends .parquet when extension is unrecognized", () => {
  assert.strictEqual(getParquetPath("/data/x.txt"), "/data/x.txt.parquet");
  assert.strictEqual(getParquetPath("/data/x"), "/data/x.parquet");
});

test("getParquetPath ignores case and directory components", () => {
  assert.strictEqual(getParquetPath("/data/csv_dir/Y.CSV"), "/data/csv_dir/Y.parquet");
});

// ─────────────────────────────────────────────────────────────────────────────
// statsFilePath
// ─────────────────────────────────────────────────────────────────────────────

test("statsFilePath places the cache next to the input file", () => {
  assert.strictEqual(statsFilePath("/tmp/foo.csv"), "/tmp/foo.stats.csv");
  assert.strictEqual(statsFilePath("/tmp/foo.tsv"), "/tmp/foo.stats.csv");
});

test("statsFilePath uses the basename without its extension", () => {
  // parse() drops the final extension, so 'foo.csv.sz' becomes 'foo.csv'.
  // This mirrors how qsv writes the stats cache for compressed inputs.
  assert.strictEqual(statsFilePath("/tmp/foo.csv.sz"), "/tmp/foo.csv.stats.csv");
});

test("statsFilePath handles a bare filename without a directory", () => {
  assert.strictEqual(statsFilePath("foo.csv"), "foo.stats.csv");
});

// ─────────────────────────────────────────────────────────────────────────────
// suggestDuckDbFixes
// ─────────────────────────────────────────────────────────────────────────────

test("suggestDuckDbFixes returns an empty string when no triggers match", () => {
  assert.strictEqual(suggestDuckDbFixes("some unrelated message"), "");
});

test("suggestDuckDbFixes surfaces generic syntax-error hints", () => {
  const out = suggestDuckDbFixes("Parser Error: syntax error at end of input");
  assert.match(out, /trailing commas/i);
  assert.match(out, /parentheses/i);
  assert.match(out, /single quotes/i);
});

test("suggestDuckDbFixes flags a trailing comma when the SQL contains one before FROM", () => {
  const sql = "SELECT a, b, FROM _t_1";
  const out = suggestDuckDbFixes("Parser Error: syntax error near 'FROM'", sql);
  assert.match(out, /trailing comma detected/i);
});

test("suggestDuckDbFixes does NOT flag a trailing comma when SQL has none", () => {
  const sql = "SELECT a, b FROM _t_1";
  const out = suggestDuckDbFixes("Parser Error: syntax error", sql);
  assert.doesNotMatch(out, /trailing comma detected/i);
});

test("suggestDuckDbFixes warns about double-quoted identifiers when the error mentions one", () => {
  const sql = `SELECT * FROM _t_1 WHERE name = "alice"`;
  const out = suggestDuckDbFixes(
    "Parser Error: syntax error near 'alice' (column 'alice' does not exist)",
    sql,
  );
  assert.match(out, /double-quoted identifier/i);
});

test("suggestDuckDbFixes does not warn about double-quoted identifiers when the error mentions none", () => {
  const sql = `SELECT * FROM _t_1 WHERE name = "alice"`;
  const out = suggestDuckDbFixes("Parser Error: syntax error", sql);
  // No bare token from the SQL appears in the error → no double-quote warning.
  assert.doesNotMatch(out, /double-quoted identifier/i);
});

test("suggestDuckDbFixes hints at column case-sensitivity for 'not found' errors", () => {
  const out = suggestDuckDbFixes(`Binder Error: Referenced column "Foo" not found`);
  assert.match(out, /case-sensitive/i);
  assert.match(out, /qsv_headers/i);
});

test("suggestDuckDbFixes recommends TRY_CAST for conversion errors", () => {
  const out = suggestDuckDbFixes("Conversion Error: could not parse '12x' as INTEGER");
  assert.match(out, /TRY_CAST/);
  assert.match(out, /qsv_stats/);
});

test("suggestDuckDbFixes mentions the default _t_1 alias on binder errors", () => {
  const out = suggestDuckDbFixes("Binder Error: table alias mismatch");
  assert.match(out, /_t_1/);
});

test("suggestDuckDbFixes combines multiple categories when applicable", () => {
  const stderr =
    "Parser Error: syntax error\nBinder Error: column 'Foo' not found\nConversion Error: could not convert";
  const out = suggestDuckDbFixes(stderr);
  assert.match(out, /trailing commas/i);
  assert.match(out, /case-sensitive/i);
  assert.match(out, /TRY_CAST/);
});
