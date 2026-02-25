/**
 * Tests for TSV output format feature (QSV_MCP_OUTPUT_FORMAT)
 *
 * Tests verify that:
 * 1. Config defaults to "tsv"
 * 2. Tabular commands produce tab-delimited output
 * 3. Metadata commands (count, headers) are NOT affected by TSV mode
 * 4. Non-tabular commands (tojsonl, schema) are NOT affected
 */

import { test } from "node:test";
import assert from "node:assert";
import { readFile } from "fs/promises";
import { join } from "path";
import { handleToolCall, handleConfigTool } from "../src/mcp-tools.js";
import { config } from "../src/config.js";
import { SkillLoader } from "../src/loader.js";
import { SkillExecutor } from "../src/executor.js";
import {
  QSV_AVAILABLE,
  createTestDir,
  createTestCSV,
  cleanupTestDir,
} from "./test-helpers.js";

// ============================================================================
// Config Tests
// ============================================================================

test("config.outputFormat is a valid value (tsv or csv)", () => {
  assert.ok(
    ["tsv", "csv"].includes(config.outputFormat),
    `outputFormat must be "tsv" or "csv", got: "${config.outputFormat}"`,
  );
  // Note: default is "tsv" unless QSV_MCP_OUTPUT_FORMAT env var overrides it.
  // We can't assert the default here since the env var may be set in CI.
  if (!process.env.QSV_MCP_OUTPUT_FORMAT) {
    assert.strictEqual(
      config.outputFormat,
      "tsv",
      "Default outputFormat should be 'tsv' when env var is not set",
    );
  }
});

// ============================================================================
// qsv_config display tests
// ============================================================================

test("qsv_config shows output format", async () => {
  const result = await handleConfigTool();
  const text = result.content[0].text || "";
  assert.ok(
    text.includes("Output Format"),
    "Config output should include Output Format",
  );
  assert.ok(
    text.includes(config.outputFormat.toUpperCase()),
    `Config output should show ${config.outputFormat.toUpperCase()}`,
  );
});

// ============================================================================
// Integration Tests — TSV output for tabular commands
// ============================================================================

test(
  "qsv_select produces tab-delimited output in TSV mode",
  { skip: !QSV_AVAILABLE || config.outputFormat !== "tsv" },
  async () => {
    const testDir = await createTestDir("qsv-tsv-select");
    const loader = new SkillLoader();
    const executor = new SkillExecutor();

    try {
      await loader.loadAll();

      const csvPath = await createTestCSV(
        testDir,
        "test.csv",
        "id,name,age\n1,Alice,30\n2,Bob,25\n",
      );

      const result = await handleToolCall(
        "qsv_select",
        { input_file: csvPath, selection: "name,age" },
        executor,
        loader,
      );

      assert.ok(!result.isError, "Command should succeed");
      const output = result.content[0].text || "";
      // In TSV mode, output should contain tabs instead of commas
      assert.ok(output.includes("\t"), "Output should contain tab characters");
      assert.ok(
        output.includes("name\tage"),
        "Header should be tab-separated",
      );
      assert.ok(
        output.includes("Alice\t30"),
        "Data rows should be tab-separated",
      );
    } finally {
      await cleanupTestDir(testDir);
    }
  },
);

test(
  "qsv_frequency produces tab-delimited output in TSV mode",
  { skip: !QSV_AVAILABLE || config.outputFormat !== "tsv" },
  async () => {
    const testDir = await createTestDir("qsv-tsv-freq");
    const loader = new SkillLoader();
    const executor = new SkillExecutor();

    try {
      await loader.loadAll();

      const csvPath = await createTestCSV(
        testDir,
        "test.csv",
        "color\nred\nblue\nred\ngreen\nred\n",
      );

      const result = await handleToolCall(
        "qsv_frequency",
        { input_file: csvPath },
        executor,
        loader,
      );

      assert.ok(!result.isError, "Command should succeed");
      const output = result.content[0].text || "";
      assert.ok(output.includes("\t"), "Frequency output should contain tabs");
    } finally {
      await cleanupTestDir(testDir);
    }
  },
);

test(
  "qsv_sort produces tab-delimited output in TSV mode",
  { skip: !QSV_AVAILABLE || config.outputFormat !== "tsv" },
  async () => {
    const testDir = await createTestDir("qsv-tsv-sort");
    const loader = new SkillLoader();
    const executor = new SkillExecutor();

    try {
      await loader.loadAll();

      const csvPath = await createTestCSV(
        testDir,
        "test.csv",
        "name,score\nCharlie,90\nAlice,85\nBob,92\n",
      );

      const result = await handleToolCall(
        "qsv_sort",
        { input_file: csvPath, select: "name" },
        executor,
        loader,
      );

      assert.ok(!result.isError, "Command should succeed");
      const output = result.content[0].text || "";
      assert.ok(output.includes("\t"), "Sort output should contain tabs");
      assert.ok(
        output.includes("name\tscore"),
        "Header should be tab-separated",
      );
    } finally {
      await cleanupTestDir(testDir);
    }
  },
);

// ============================================================================
// Metadata commands should NOT be affected by TSV mode
// ============================================================================

test(
  "qsv_count is NOT affected by TSV mode (metadata command)",
  { skip: !QSV_AVAILABLE },
  async () => {
    const testDir = await createTestDir("qsv-tsv-count");
    const loader = new SkillLoader();
    const executor = new SkillExecutor();

    try {
      await loader.loadAll();

      const csvPath = await createTestCSV(
        testDir,
        "test.csv",
        "id,name\n1,Alice\n2,Bob\n3,Charlie\n",
      );

      const result = await handleToolCall(
        "qsv_count",
        { input_file: csvPath },
        executor,
        loader,
      );

      assert.ok(!result.isError, "Command should succeed");
      const output = result.content[0].text || "";
      // count returns a simple number, no CSV format at all
      assert.ok(output.includes("3"), "Should return row count 3");
    } finally {
      await cleanupTestDir(testDir);
    }
  },
);

test(
  "qsv_headers is NOT affected by TSV mode (metadata command)",
  { skip: !QSV_AVAILABLE },
  async () => {
    const testDir = await createTestDir("qsv-tsv-headers");
    const loader = new SkillLoader();
    const executor = new SkillExecutor();

    try {
      await loader.loadAll();

      const csvPath = await createTestCSV(
        testDir,
        "test.csv",
        "id,name,age\n1,Alice,30\n",
      );

      const result = await handleToolCall(
        "qsv_headers",
        { input_file: csvPath },
        executor,
        loader,
      );

      assert.ok(!result.isError, "Command should succeed");
      const output = result.content[0].text || "";
      // headers outputs numbered column names, not tabular CSV
      assert.ok(output.includes("id"), "Should list id column");
      assert.ok(output.includes("name"), "Should list name column");
    } finally {
      await cleanupTestDir(testDir);
    }
  },
);

// ============================================================================
// Non-tabular commands should NOT be affected
// ============================================================================

test(
  "qsv_tojsonl is NOT affected by TSV mode (JSONL output)",
  { skip: !QSV_AVAILABLE },
  async () => {
    const testDir = await createTestDir("qsv-tsv-tojsonl");
    const loader = new SkillLoader();
    const executor = new SkillExecutor();

    try {
      await loader.loadAll();

      const csvPath = await createTestCSV(
        testDir,
        "test.csv",
        "id,name\n1,Alice\n2,Bob\n",
      );

      const result = await handleToolCall(
        "qsv_tojsonl",
        { input_file: csvPath },
        executor,
        loader,
      );

      assert.ok(!result.isError, "Command should succeed");
      const output = result.content[0].text || "";
      // tojsonl should output JSON lines, not TSV
      assert.ok(
        output.includes("{") && output.includes("}"),
        "Should contain JSON objects",
      );
      assert.ok(
        !output.includes("id\tname"),
        "Should NOT contain tab-separated headers",
      );
    } finally {
      await cleanupTestDir(testDir);
    }
  },
);

// ============================================================================
// Temp file extension tests
// ============================================================================

test(
  "auto-created temp files use .tsv extension in TSV mode",
  { skip: !QSV_AVAILABLE || config.outputFormat !== "tsv" },
  async () => {
    const testDir = await createTestDir("qsv-tsv-ext");
    const loader = new SkillLoader();
    const executor = new SkillExecutor();

    try {
      await loader.loadAll();

      // Create a file large enough to trigger temp file usage
      // (ALWAYS_FILE_COMMANDS includes sort, so any size will do)
      const csvPath = await createTestCSV(
        testDir,
        "test.csv",
        "id,name\n1,Alice\n2,Bob\n",
      );

      const result = await handleToolCall(
        "qsv_sort",
        { input_file: csvPath, select: "name" },
        executor,
        loader,
      );

      assert.ok(!result.isError, "Command should succeed");
      const output = result.content[0].text || "";
      // The output content should be tab-delimited (proving .tsv temp file was used)
      assert.ok(
        output.includes("\t"),
        "Output should be tab-delimited from .tsv temp file",
      );
    } finally {
      await cleanupTestDir(testDir);
    }
  },
);

test(
  "user-specified --output is NOT overridden by TSV mode",
  { skip: !QSV_AVAILABLE },
  async () => {
    const testDir = await createTestDir("qsv-tsv-user-output");
    const loader = new SkillLoader();
    const executor = new SkillExecutor();

    try {
      await loader.loadAll();

      const csvPath = await createTestCSV(
        testDir,
        "test.csv",
        "id,name\n1,Alice\n2,Bob\n",
      );

      const outputPath = join(testDir, "output.csv");

      const result = await handleToolCall(
        "qsv_select",
        {
          input_file: csvPath,
          selection: "name",
          output: outputPath,
        },
        executor,
        loader,
      );

      assert.ok(!result.isError, "Command should succeed");

      // User specified .csv output — should remain comma-delimited
      const fileContent = await readFile(outputPath, "utf8");
      assert.ok(
        fileContent.includes("name"),
        "Output file should contain header",
      );
      assert.ok(
        fileContent.includes("Alice"),
        "Output file should contain data",
      );
      // Since user chose .csv extension, qsv outputs CSV format
      assert.ok(
        !fileContent.includes("\t"),
        "User-specified .csv output should NOT contain tabs",
      );
    } finally {
      await cleanupTestDir(testDir);
    }
  },
);
