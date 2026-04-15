/**
 * Unit tests for file-operations.ts
 *
 * Tests path resolution, auto-indexing, temp file decisions,
 * format tool results, param aliases, and file-not-found error building.
 */

import { test, describe, beforeEach, afterEach } from "node:test";
import assert from "node:assert";
import { writeFile, stat } from "fs/promises";
import { join } from "path";
import {
  buildConversionArgs,
  mapSchemaType,
  autoIndexIfNeeded,
  shouldUseTempFile,
  buildSkillExecParams,
  resolveParamAliases,
  paramKeyToFlag,
  looksLikeFilePath,
  collectAdditionalInputFiles,
} from "../src/file-operations.js";
import { createTestDir, cleanupTestDir, createTestCSV, QSV_AVAILABLE } from "./test-helpers.js";
import type { QsvSkill } from "../src/types.js";

// ============================================================================
// buildConversionArgs
// ============================================================================

describe("buildConversionArgs", () => {
  test("builds standard conversion args", () => {
    const args = buildConversionArgs("excel", "/tmp/data.xlsx", "/tmp/data.csv");
    assert.deepStrictEqual(args, ["excel", "/tmp/data.xlsx", "--output", "/tmp/data.csv"]);
  });

  test("builds parquet conversion args with SQL and read_parquet", () => {
    const args = buildConversionArgs("parquet", "/tmp/data.parquet", "/tmp/data.csv");
    assert.strictEqual(args[0], "sqlp");
    assert.strictEqual(args[1], "SKIP_INPUT");
    assert.ok(args[2].includes("read_parquet"));
    assert.ok(args[2].includes("/tmp/data.parquet"));
    assert.strictEqual(args[3], "--output");
    assert.strictEqual(args[4], "/tmp/data.csv");
  });

  test("parquet conversion normalizes Windows backslashes", () => {
    const args = buildConversionArgs("parquet", "C:\\Users\\data.parquet", "/tmp/out.csv");
    assert.ok(args[2].includes("C:/Users/data.parquet"));
    assert.ok(!args[2].includes("\\"));
  });

  test("parquet conversion escapes single quotes in path", () => {
    const args = buildConversionArgs("parquet", "/tmp/it's data.parquet", "/tmp/out.csv");
    assert.ok(args[2].includes("it''s data.parquet"));
  });
});

// ============================================================================
// mapSchemaType
// ============================================================================

describe("mapSchemaType", () => {
  test("maps number to number", () => {
    assert.strictEqual(mapSchemaType("number"), "number");
  });

  test("maps file to string", () => {
    assert.strictEqual(mapSchemaType("file"), "string");
  });

  test("maps regex to string", () => {
    assert.strictEqual(mapSchemaType("regex"), "string");
  });

  test("maps string to string", () => {
    assert.strictEqual(mapSchemaType("string"), "string");
  });

  test("maps unknown type to string", () => {
    assert.strictEqual(mapSchemaType("unknown"), "string");
  });
});

// ============================================================================
// autoIndexIfNeeded
// ============================================================================

describe("autoIndexIfNeeded", () => {
  let testDir: string;

  beforeEach(async () => {
    testDir = await createTestDir("file-ops-idx");
  });

  afterEach(async () => {
    await cleanupTestDir(testDir);
  });

  test("skips non-indexable file extensions", async () => {
    const jsonFile = join(testDir, "data.json");
    await writeFile(jsonFile, "{}");
    await autoIndexIfNeeded(jsonFile, 0); // minSize=0 to bypass size check
    // No .idx file should be created
    try {
      await stat(jsonFile + ".idx");
      assert.fail("Should not have created an index for JSON file");
    } catch (err: unknown) {
      assert.strictEqual((err as NodeJS.ErrnoException).code, "ENOENT");
    }
  });

  test("skips small CSV files", async () => {
    const csvFile = await createTestCSV(testDir, "small.csv", "a,b\n1,2\n");
    await autoIndexIfNeeded(csvFile, 100); // 100MB threshold
    try {
      await stat(csvFile + ".idx");
      assert.fail("Should not have created an index for small file");
    } catch (err: unknown) {
      assert.strictEqual((err as NodeJS.ErrnoException).code, "ENOENT");
    }
  });

  test("skips if index already exists", { skip: !QSV_AVAILABLE }, async () => {
    const csvFile = await createTestCSV(testDir, "has-idx.csv", "a,b\n1,2\n3,4\n");
    // Pre-create the index file
    await writeFile(csvFile + ".idx", "fake index");
    await autoIndexIfNeeded(csvFile, 0);
    // Index should still be the fake one (not overwritten)
    const { readFile } = await import("fs/promises");
    const content = await readFile(csvFile + ".idx", "utf-8");
    assert.strictEqual(content, "fake index");
  });

  test("handles non-existent file gracefully", async () => {
    // Should not throw
    await autoIndexIfNeeded(join(testDir, "nonexistent.csv"), 0);
  });

  test("recognizes TSV and SSV as indexable", async () => {
    const tsvFile = join(testDir, "data.tsv");
    await writeFile(tsvFile, "a\tb\n1\t2\n");
    // Should not throw for TSV (even though it won't actually create index without qsv)
    await autoIndexIfNeeded(tsvFile, 100);

    const ssvFile = join(testDir, "data.ssv");
    await writeFile(ssvFile, "a;b\n1;2\n");
    await autoIndexIfNeeded(ssvFile, 100);
  });
});

// ============================================================================
// shouldUseTempFile
// ============================================================================

describe("shouldUseTempFile", () => {
  let testDir: string;

  beforeEach(async () => {
    testDir = await createTestDir("file-ops-tmp");
  });

  afterEach(async () => {
    await cleanupTestDir(testDir);
  });

  test("returns false for metadata commands (count, headers, index, sniff)", async () => {
    const csvFile = await createTestCSV(testDir, "data.csv", "a\n1\n");
    assert.strictEqual(await shouldUseTempFile("count", csvFile), false);
    assert.strictEqual(await shouldUseTempFile("headers", csvFile), false);
    assert.strictEqual(await shouldUseTempFile("index", csvFile), false);
    assert.strictEqual(await shouldUseTempFile("sniff", csvFile), false);
  });

  test("returns true for always-file commands", async () => {
    const csvFile = await createTestCSV(testDir, "data.csv", "a\n1\n");
    assert.strictEqual(await shouldUseTempFile("stats", csvFile), true);
    assert.strictEqual(await shouldUseTempFile("frequency", csvFile), true);
    assert.strictEqual(await shouldUseTempFile("sort", csvFile), true);
    assert.strictEqual(await shouldUseTempFile("sqlp", csvFile), true);
  });

  test("size-based decision for unknown commands with small files", async () => {
    const csvFile = await createTestCSV(testDir, "tiny.csv", "a\n1\n");
    const result = await shouldUseTempFile("sometool", csvFile);
    // In TSV mode, all non-metadata tabular commands use temp files.
    // In CSV mode, small files use stdout.
    // Either way, this should not throw.
    assert.strictEqual(typeof result, "boolean");
  });

  test("size-based decision for non-existent input files", async () => {
    const result = await shouldUseTempFile("sometool", join(testDir, "nope.csv"));
    // In TSV mode returns true (forced temp file); in CSV mode returns false (stat failed)
    assert.strictEqual(typeof result, "boolean");
  });
});

// ============================================================================
// paramKeyToFlag
// ============================================================================

describe("paramKeyToFlag", () => {
  test("converts underscore keys to dashed flags", () => {
    assert.strictEqual(paramKeyToFlag("dupes_output"), "--dupes-output");
  });

  test("leaves already-flagged keys unchanged", () => {
    assert.strictEqual(paramKeyToFlag("--already-flagged"), "--already-flagged");
  });

  test("converts simple key", () => {
    assert.strictEqual(paramKeyToFlag("select"), "--select");
  });
});

// ============================================================================
// looksLikeFilePath
// ============================================================================

describe("looksLikeFilePath", () => {
  test("recognizes absolute Unix paths", () => {
    assert.strictEqual(looksLikeFilePath("/home/user/script.lua"), true);
    assert.strictEqual(looksLikeFilePath("/tmp/data.csv"), true);
  });

  test("recognizes relative paths", () => {
    assert.strictEqual(looksLikeFilePath("./script.lua"), true);
    assert.strictEqual(looksLikeFilePath("../data/file.csv"), true);
  });

  test("recognizes home-relative paths", () => {
    assert.strictEqual(looksLikeFilePath("~/scripts/run.lua"), true);
  });

  test("recognizes Windows paths", () => {
    assert.strictEqual(looksLikeFilePath("C:\\Users\\script.lua"), true);
    assert.strictEqual(looksLikeFilePath(".\\script.lua"), true);
    assert.strictEqual(looksLikeFilePath("..\\data\\file.csv"), true);
  });

  test("recognizes file: URIs", () => {
    assert.strictEqual(looksLikeFilePath("file:///tmp/script.lua"), true);
  });

  test("recognizes .lua and .luau extensions", () => {
    assert.strictEqual(looksLikeFilePath("script.lua"), true);
    assert.strictEqual(looksLikeFilePath("script.luau"), true);
  });

  test("recognizes multi-segment slash paths", () => {
    assert.strictEqual(looksLikeFilePath("path/to/file.lua"), true);
  });

  test("rejects inline Luau code", () => {
    assert.strictEqual(looksLikeFilePath("col.A / col.B"), false);
    assert.strictEqual(looksLikeFilePath("return x + 1"), false);
  });

  test("rejects single-slash bare paths (Luau division ambiguity)", () => {
    assert.strictEqual(looksLikeFilePath("a/b"), false);
  });
});

// ============================================================================
// resolveParamAliases
// ============================================================================

describe("resolveParamAliases", () => {
  test("resolves input_file and output_file", () => {
    const result = resolveParamAliases({
      input_file: "data.csv",
      output_file: "out.csv",
    });
    assert.strictEqual(result.inputFile, "data.csv");
    assert.strictEqual(result.outputFile, "out.csv");
  });

  test("falls back to input/output aliases", () => {
    const result = resolveParamAliases({
      input: "data.csv",
      output: "out.csv",
    });
    assert.strictEqual(result.inputFile, "data.csv");
    assert.strictEqual(result.outputFile, "out.csv");
  });

  test("canonical names take precedence over aliases", () => {
    const result = resolveParamAliases({
      input_file: "canonical.csv",
      input: "alias.csv",
    });
    assert.strictEqual(result.inputFile, "canonical.csv");
  });

  test("ignores non-string values", () => {
    const result = resolveParamAliases({
      input: 42,
      output: true,
    });
    assert.strictEqual(result.inputFile, undefined);
    assert.strictEqual(result.outputFile, undefined);
  });

  test("trims whitespace from values", () => {
    const result = resolveParamAliases({
      input_file: "  data.csv  ",
    });
    assert.strictEqual(result.inputFile, "data.csv");
  });

  test("treats empty strings as undefined", () => {
    const result = resolveParamAliases({
      input_file: "",
      output_file: "   ",
    });
    assert.strictEqual(result.inputFile, undefined);
    assert.strictEqual(result.outputFile, undefined);
  });
});

// ============================================================================
// buildSkillExecParams
// ============================================================================

describe("buildSkillExecParams", () => {
  const baseSkill: QsvSkill = {
    name: "qsv-stats",
    version: "19.0.0",
    description: "Stats",
    category: "aggregation",
    command: {
      subcommand: "stats",
      args: [{ name: "input", type: "file" as const, required: true, description: "Input CSV" }],
      options: [
        { flag: "--select", type: "string" as const, description: "Select columns" },
        { flag: "--cardinality", type: "flag" as const, description: "Compute cardinality" },
      ],
    },
  };

  test("adds input file as input arg", () => {
    const result = buildSkillExecParams(baseSkill, {}, "/tmp/data.csv", undefined, false);
    assert.strictEqual(result.args.input, "/tmp/data.csv");
  });

  test("maps options to --flag format", () => {
    const result = buildSkillExecParams(
      baseSkill,
      { select: "1,2,3", cardinality: true },
      "/tmp/data.csv",
      undefined,
      false,
    );
    assert.strictEqual(result.options["--select"], "1,2,3");
    assert.strictEqual(result.options["--cardinality"], true);
  });

  test("adds output file as --output option", () => {
    const result = buildSkillExecParams(baseSkill, {}, "/tmp/data.csv", "/tmp/out.csv", false);
    assert.strictEqual(result.options["--output"], "/tmp/out.csv");
  });

  test("adds help flag when isHelpRequest is true", () => {
    const result = buildSkillExecParams(baseSkill, {}, undefined, undefined, true);
    assert.strictEqual(result.options["help"], true);
  });

  test("skips input_file, output_file, input, output, help meta params", () => {
    const result = buildSkillExecParams(
      baseSkill,
      { input_file: "skip.csv", output_file: "skip.csv", help: true },
      "/tmp/data.csv",
      undefined,
      false,
    );
    assert.strictEqual(result.options["--input_file"], undefined);
    assert.strictEqual(result.options["--output_file"], undefined);
  });

  test("converts underscore param keys to dashed flags", () => {
    const result = buildSkillExecParams(
      baseSkill,
      { stats_jsonl: true },
      "/tmp/data.csv",
      undefined,
      false,
    );
    assert.strictEqual(result.options["--stats-jsonl"], true);
  });
});

// ============================================================================
// collectAdditionalInputFiles
// ============================================================================

describe("collectAdditionalInputFiles", () => {
  test("collects file-type positional args (excluding input)", () => {
    const skill: QsvSkill = {
      name: "qsv-join",
      version: "19.0.0",
      description: "Join",
      category: "joining",
      command: {
        subcommand: "join",
        args: [
          { name: "input", type: "file" as const, required: true, description: "Left input" },
          { name: "right_input", type: "file" as const, required: true, description: "Right input" },
        ],
        options: [],
      },
    };

    const files = collectAdditionalInputFiles(skill, {
      input: "left.csv",
      right_input: "/tmp/right.csv",
    });
    assert.strictEqual(files.length, 1);
    assert.strictEqual(files[0].file, "/tmp/right.csv");
    assert.strictEqual(files[0].param, "right_input");
  });

  test("returns empty array when no additional files", () => {
    const skill: QsvSkill = {
      name: "qsv-stats",
      version: "19.0.0",
      description: "Stats",
      category: "aggregation",
      command: {
        subcommand: "stats",
        args: [{ name: "input", type: "file" as const, required: true, description: "Input" }],
        options: [],
      },
    };

    const files = collectAdditionalInputFiles(skill, { input: "data.csv" });
    assert.strictEqual(files.length, 0);
  });
});
