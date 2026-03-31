/**
 * Unit tests for pipeline-manifest.ts
 */

import { test } from "node:test";
import assert from "node:assert";
import { writeFileSync, mkdirSync, rmSync, readFileSync, existsSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";
import { randomUUID } from "node:crypto";

import {
  classifyKind,
  isDeterministic,
  PipelineManifest,
  type PipelineStep,
  type PipelineManifestJson,
} from "../src/pipeline-manifest.js";

// Helper: create a unique temp directory for each test
function makeTempDir(): string {
  const dir = join(tmpdir(), `qsv-manifest-test-${randomUUID()}`);
  mkdirSync(dir, { recursive: true });
  return dir;
}

// ── classifyKind ──────────────────────────────────────────────────────────

test("classifyKind: meta tools", () => {
  assert.strictEqual(classifyKind("qsv_config"), "meta");
  assert.strictEqual(classifyKind("qsv_set_working_dir"), "meta");
  assert.strictEqual(classifyKind("qsv_get_working_dir"), "meta");
  assert.strictEqual(classifyKind("qsv_browse_directory"), "meta");
  assert.strictEqual(classifyKind("qsv_list_files"), "meta");
  assert.strictEqual(classifyKind("qsv_log"), "meta");
  assert.strictEqual(classifyKind("qsv_search_tools"), "meta");
  assert.strictEqual(classifyKind("qsv_setup"), "meta");
});

test("classifyKind: exploratory tools", () => {
  assert.strictEqual(classifyKind("qsv_stats"), "exploratory");
  assert.strictEqual(classifyKind("qsv_frequency"), "exploratory");
  assert.strictEqual(classifyKind("qsv_count"), "exploratory");
  assert.strictEqual(classifyKind("qsv_headers"), "exploratory");
  assert.strictEqual(classifyKind("qsv_sniff"), "exploratory");
  assert.strictEqual(classifyKind("qsv_schema"), "exploratory");
  assert.strictEqual(classifyKind("qsv_describegpt"), "exploratory");
  assert.strictEqual(classifyKind("qsv_moarstats"), "exploratory");
});

test("classifyKind: transformative tools (default)", () => {
  assert.strictEqual(classifyKind("qsv_select"), "transformative");
  assert.strictEqual(classifyKind("qsv_sqlp"), "transformative");
  assert.strictEqual(classifyKind("qsv_joinp"), "transformative");
  assert.strictEqual(classifyKind("qsv_sort"), "transformative");
  assert.strictEqual(classifyKind("qsv_dedup"), "transformative");
  assert.strictEqual(classifyKind("qsv_fill"), "transformative");
  assert.strictEqual(classifyKind("qsv_excel"), "transformative");
});

// ── isDeterministic ───────────────────────────────────────────────────────

test("isDeterministic: non-sample/sort commands are deterministic", () => {
  assert.deepStrictEqual(isDeterministic("qsv_select", {}), { deterministic: true });
  assert.deepStrictEqual(isDeterministic("qsv_sqlp", { sql: "SELECT *" }), { deterministic: true });
});

test("isDeterministic: sample without seed is non-deterministic", () => {
  assert.deepStrictEqual(isDeterministic("qsv_sample", { size: 10 }), { deterministic: false });
});

test("isDeterministic: sample with seed is deterministic", () => {
  assert.deepStrictEqual(isDeterministic("qsv_sample", { seed: 42 }), { deterministic: true, seed: 42 });
  assert.deepStrictEqual(isDeterministic("qsv_sample", { "--seed": 7 }), { deterministic: true, seed: 7 });
});

test("isDeterministic: sort without --random is deterministic", () => {
  assert.deepStrictEqual(isDeterministic("qsv_sort", { select: "name" }), { deterministic: true });
});

test("isDeterministic: sort with --random without seed is non-deterministic", () => {
  assert.deepStrictEqual(isDeterministic("qsv_sort", { random: true }), { deterministic: false });
  assert.deepStrictEqual(isDeterministic("qsv_sort", { "--random": true }), { deterministic: false });
});

test("isDeterministic: sort with --random and --seed is deterministic", () => {
  assert.deepStrictEqual(isDeterministic("qsv_sort", { random: true, seed: 99 }), { deterministic: true, seed: 99 });
});

// ── PipelineManifest ──────────────────────────────────────────────────────

test("PipelineManifest: recordStep writes to JSONL", async () => {
  const dir = makeTempDir();
  try {
    const manifest = new PipelineManifest("test-session", dir, "18.0.0", "18.0.5");

    await manifest.recordStep({
      invocationId: "inv-1",
      toolName: "qsv_select",
      toolArgs: { select: "name,age" },
      reason: "Select name columns",
      commandLine: "qsv select name,age input.csv",
      inputFile: null,
      outputFile: null,
      additionalInputFiles: [],
      durationMs: 100,
      success: true,
    });

    const jsonlPath = join(dir, ".qsv-pipeline-steps.jsonl");
    assert.ok(existsSync(jsonlPath), "JSONL file should exist");

    const content = readFileSync(jsonlPath, "utf-8").trim();
    const step = JSON.parse(content);
    assert.strictEqual(step.step, 1);
    assert.strictEqual(step.tool, "qsv_select");
    assert.strictEqual(step.kind, "transformative");
    assert.strictEqual(step.deterministic, true);
    assert.strictEqual(step.reason, "Select name columns");
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

test("PipelineManifest: step counter increments", async () => {
  const dir = makeTempDir();
  try {
    const manifest = new PipelineManifest("test-session", dir, "18.0.0", "18.0.5");

    const baseParams = {
      invocationId: "inv-1",
      toolName: "qsv_stats",
      toolArgs: {},
      reason: null,
      commandLine: "qsv stats input.csv",
      inputFile: null,
      outputFile: null,
      additionalInputFiles: [],
      durationMs: 50,
      success: true,
    };

    await manifest.recordStep(baseParams);
    await manifest.recordStep({ ...baseParams, invocationId: "inv-2" });
    await manifest.recordStep({ ...baseParams, invocationId: "inv-3" });

    const steps = manifest.getSteps();
    assert.strictEqual(steps.length, 3);
    assert.strictEqual(steps[0].step, 1);
    assert.strictEqual(steps[1].step, 2);
    assert.strictEqual(steps[2].step, 3);
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

test("PipelineManifest: hashFile returns null for non-existent file", async () => {
  const dir = makeTempDir();
  try {
    const manifest = new PipelineManifest("test-session", dir, "18.0.0", "18.0.5");
    const result = await manifest.hashFile("/non/existent/file.csv");
    assert.strictEqual(result, null);
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

test("PipelineManifest: hashFile returns size even without b3sum", async () => {
  const dir = makeTempDir();
  const testFile = join(dir, "test.csv");
  writeFileSync(testFile, "name,age\nAlice,30\n");
  try {
    const manifest = new PipelineManifest("test-session", dir, "18.0.0", "18.0.5");
    const result = await manifest.hashFile(testFile);
    assert.ok(result, "Should return file hash info");
    assert.ok(result.size_bytes > 0, "Should have positive size");
    assert.strictEqual(result.file, testFile);
    // blake3 will be null if b3sum is not installed, or a hex string if it is
    if (result.blake3 !== null) {
      assert.ok(typeof result.blake3 === "string" && result.blake3.length === 64, "BLAKE3 hash should be 64 hex chars");
    }
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

test("PipelineManifest: hashFile caches by path+mtime", async () => {
  const dir = makeTempDir();
  const testFile = join(dir, "cached.csv");
  writeFileSync(testFile, "a,b\n1,2\n");
  try {
    const manifest = new PipelineManifest("test-session", dir, "18.0.0", "18.0.5");

    const first = await manifest.hashFile(testFile);
    const second = await manifest.hashFile(testFile);

    // Both should return identical results
    assert.deepStrictEqual(first, second);
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

test("PipelineManifest: finalize writes pipeline.json and pipeline.sh", async () => {
  const dir = makeTempDir();
  try {
    const manifest = new PipelineManifest("test-session", dir, "18.0.0", "18.0.5");

    await manifest.recordStep({
      invocationId: "inv-1",
      toolName: "qsv_sqlp",
      toolArgs: { sql: "SELECT * FROM _t" },
      reason: "Query all rows",
      commandLine: 'qsv sqlp input.csv "SELECT * FROM _t"',
      inputFile: null,
      outputFile: null,
      additionalInputFiles: [],
      durationMs: 200,
      success: true,
    });

    const result = manifest.finalize("2026-03-30T12:00:00Z");
    assert.ok(result, "finalize should return paths");

    // Check pipeline.json
    assert.ok(existsSync(result.jsonPath));
    const json: PipelineManifestJson = JSON.parse(readFileSync(result.jsonPath, "utf-8"));
    assert.strictEqual(json.version, "1.0.0");
    assert.strictEqual(json.session.id, "test-session");
    assert.strictEqual(json.session.qsv_version, "18.0.0");
    assert.strictEqual(json.steps.length, 1);
    assert.strictEqual(json.steps[0].tool, "qsv_sqlp");
    assert.strictEqual(json.steps[0].kind, "transformative");

    // Check pipeline.sh
    assert.ok(existsSync(result.shPath));
    const sh = readFileSync(result.shPath, "utf-8");
    assert.ok(sh.includes("#!/usr/bin/env bash"));
    assert.ok(sh.includes('qsv sqlp input.csv "SELECT * FROM _t"'));

    // JSONL should be cleaned up
    assert.ok(!existsSync(join(dir, ".qsv-pipeline-steps.jsonl")));
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

test("PipelineManifest: finalize returns null when no steps", () => {
  const dir = makeTempDir();
  try {
    const manifest = new PipelineManifest("test-session", dir, "18.0.0", "18.0.5");
    const result = manifest.finalize();
    assert.strictEqual(result, null);
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

test("PipelineManifest: generateReplayScript skips exploratory and failed steps", async () => {
  const dir = makeTempDir();
  try {
    const manifest = new PipelineManifest("test-session", dir, "18.0.0", "18.0.5");

    // Exploratory step (should be skipped)
    await manifest.recordStep({
      invocationId: "inv-1",
      toolName: "qsv_stats",
      toolArgs: {},
      reason: null,
      commandLine: "qsv stats input.csv",
      inputFile: null,
      outputFile: null,
      additionalInputFiles: [],
      durationMs: 100,
      success: true,
    });

    // Failed step (should be commented out)
    await manifest.recordStep({
      invocationId: "inv-2",
      toolName: "qsv_select",
      toolArgs: { select: "nonexistent" },
      reason: null,
      commandLine: "qsv select nonexistent input.csv",
      inputFile: null,
      outputFile: null,
      additionalInputFiles: [],
      durationMs: 50,
      success: false,
      errorMessage: "Column not found",
    });

    // Successful transformative step
    await manifest.recordStep({
      invocationId: "inv-3",
      toolName: "qsv_select",
      toolArgs: { select: "name" },
      reason: "Keep only name",
      commandLine: "qsv select name input.csv",
      inputFile: null,
      outputFile: null,
      additionalInputFiles: [],
      durationMs: 80,
      success: true,
    });

    const script = manifest.generateReplayScript();
    assert.ok(!script.includes("qsv stats"), "Should not include exploratory step");
    assert.ok(script.includes("# SKIPPED (failed"), "Should comment out failed step");
    assert.ok(script.includes("qsv select name input.csv"), "Should include successful transformative step");
    assert.ok(script.includes("# Keep only name"), "Should include reason as comment");
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

test("PipelineManifest: non-deterministic steps get WARNING in replay script", async () => {
  const dir = makeTempDir();
  try {
    const manifest = new PipelineManifest("test-session", dir, "18.0.0", "18.0.5");

    await manifest.recordStep({
      invocationId: "inv-1",
      toolName: "qsv_sample",
      toolArgs: { size: 10 },
      reason: null,
      commandLine: "qsv sample 10 input.csv",
      inputFile: null,
      outputFile: null,
      additionalInputFiles: [],
      durationMs: 50,
      success: true,
    });

    const script = manifest.generateReplayScript();
    assert.ok(script.includes("WARNING: non-deterministic"));
    assert.ok(script.includes("qsv sample 10 input.csv"));
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

test("PipelineManifest: file inventory tracks roles correctly", async () => {
  const dir = makeTempDir();
  try {
    const manifest = new PipelineManifest("test-session", dir, "18.0.0", "18.0.5");

    // Step 1: input.csv → intermediate.csv
    await manifest.recordStep({
      invocationId: "inv-1",
      toolName: "qsv_select",
      toolArgs: {},
      reason: null,
      commandLine: "qsv select name input.csv --output intermediate.csv",
      inputFile: null,
      outputFile: null,
      additionalInputFiles: [],
      durationMs: 50,
      success: true,
    });

    // Since we don't have real files (input/output are null),
    // the inventory will be empty — test finalize produces valid structure
    const result = manifest.finalize();
    assert.ok(result);
    const json: PipelineManifestJson = JSON.parse(readFileSync(result.jsonPath, "utf-8"));
    assert.strictEqual(json.steps.length, 1);
    // Inventory will be empty since input/output are null in this test
    assert.deepStrictEqual(json.file_inventory, {});
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

test("PipelineManifest: web sources are attached to steps", async () => {
  const dir = makeTempDir();
  try {
    const manifest = new PipelineManifest("test-session", dir, "18.0.0", "18.0.5");

    manifest.addWebSource("https://example.com/data.csv");
    manifest.addWebSource("https://example.com/reference.csv");

    await manifest.recordStep({
      invocationId: "inv-1",
      toolName: "qsv_sqlp",
      toolArgs: {},
      reason: null,
      commandLine: "qsv sqlp data.csv",
      inputFile: null,
      outputFile: null,
      additionalInputFiles: [],
      durationMs: 100,
      success: true,
    });

    const steps = manifest.getSteps();
    assert.ok(steps[0].web_sources);
    assert.deepStrictEqual(steps[0].web_sources, [
      "https://example.com/data.csv",
      "https://example.com/reference.csv",
    ]);

    // Pending web sources should be drained
    await manifest.recordStep({
      invocationId: "inv-2",
      toolName: "qsv_select",
      toolArgs: {},
      reason: null,
      commandLine: "qsv select name data.csv",
      inputFile: null,
      outputFile: null,
      additionalInputFiles: [],
      durationMs: 50,
      success: true,
    });

    assert.strictEqual(steps[1].web_sources, undefined);
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

// ── Hook script consolidation test ────────────────────────────────────────

test("consolidatePipelineManifest: builds manifest from JSONL", async () => {
  const consolidatePipelineManifest = await loadConsolidate();

  const dir = makeTempDir();
  try {
    // Write test JSONL
    const step: PipelineStep = {
      step: 1,
      invocation_id: "inv-1",
      tool: "qsv_sqlp",
      command: 'qsv sqlp data.csv "SELECT * FROM _t"',
      args: { sql: "SELECT * FROM _t" },
      reason: null,
      timestamp: "2026-03-30T12:00:00Z",
      duration_ms: 200,
      success: true,
      kind: "transformative",
      deterministic: true,
      input: { file: "data.csv", blake3: "abc123", size_bytes: 1024 },
      output: null,
      additional_inputs: [],
    };

    writeFileSync(
      join(dir, ".qsv-pipeline-steps.jsonl"),
      JSON.stringify(step) + "\n",
    );

    consolidatePipelineManifest(dir, "test-session");

    // Verify pipeline.json was created
    const jsonPath = join(dir, "pipeline.json");
    assert.ok(existsSync(jsonPath), "pipeline.json should exist");
    const manifest: PipelineManifestJson = JSON.parse(readFileSync(jsonPath, "utf-8"));
    assert.strictEqual(manifest.version, "1.0.0");
    assert.strictEqual(manifest.session.id, "test-session");
    assert.strictEqual(manifest.steps.length, 1);
    assert.strictEqual(manifest.steps[0].tool, "qsv_sqlp");

    // Verify file inventory
    assert.ok(manifest.file_inventory["data.csv"]);
    assert.strictEqual(manifest.file_inventory["data.csv"].role, "input");

    // Verify pipeline.sh was created
    assert.ok(existsSync(join(dir, "pipeline.sh")));

    // JSONL should be cleaned up
    assert.ok(!existsSync(join(dir, ".qsv-pipeline-steps.jsonl")));
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

test("consolidatePipelineManifest: no-op when JSONL absent", async () => {
  const consolidatePipelineManifest = await loadConsolidate();

  const dir = makeTempDir();
  try {
    consolidatePipelineManifest(dir, "test-session");
    // No files should be created
    assert.ok(!existsSync(join(dir, "pipeline.json")));
    assert.ok(!existsSync(join(dir, "pipeline.sh")));
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

// Helper to load the CJS consolidation function (avoids repeating dynamic imports)
async function loadConsolidate() {
  const { createRequire } = await import("node:module");
  const { fileURLToPath } = await import("node:url");
  const { resolve, dirname } = await import("node:path");
  const require2 = createRequire(import.meta.url);
  const __filename2 = fileURLToPath(import.meta.url);
  const projectRoot = resolve(dirname(__filename2), "..", "..");
  return require2(resolve(projectRoot, "scripts", "log-session-end.cjs")).consolidatePipelineManifest;
}

test("consolidatePipelineManifest: attaches web_source to next step", async () => {
  const consolidatePipelineManifest = await loadConsolidate();

  const dir = makeTempDir();
  try {
    // Write a web_source entry followed by a pipeline step
    const webSource = {
      type: "web_source",
      tool: "WebFetch",
      url: "https://example.com/data.csv",
      timestamp: "2026-03-30T12:00:00Z",
    };
    const step: PipelineStep = {
      step: 1,
      invocation_id: "inv-1",
      tool: "qsv_sqlp",
      command: 'qsv sqlp data.csv "SELECT * FROM _t"',
      args: { sql: "SELECT * FROM _t" },
      reason: null,
      timestamp: "2026-03-30T12:00:01Z",
      duration_ms: 200,
      success: true,
      kind: "transformative",
      deterministic: true,
      input: null,
      output: null,
      additional_inputs: [],
    };

    writeFileSync(
      join(dir, ".qsv-pipeline-steps.jsonl"),
      JSON.stringify(webSource) + "\n" + JSON.stringify(step) + "\n",
    );

    consolidatePipelineManifest(dir, "test-session");

    const jsonPath = join(dir, "pipeline.json");
    assert.ok(existsSync(jsonPath));
    const manifest: PipelineManifestJson = JSON.parse(readFileSync(jsonPath, "utf-8"));
    assert.strictEqual(manifest.steps.length, 1);
    assert.deepStrictEqual(manifest.steps[0].web_sources, ["https://example.com/data.csv"]);
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

test("consolidatePipelineManifest: skips rebuild when valid pipeline.json exists", async () => {
  const consolidatePipelineManifest = await loadConsolidate();

  const dir = makeTempDir();
  try {
    // Write a valid pipeline.json (as if MCP server wrote it)
    const existingManifest = {
      version: "1.0.0",
      session: { id: "server-session", started_at: "2026-03-30T12:00:00Z", ended_at: "2026-03-30T12:01:00Z", qsv_version: "18.0.0", mcp_server_version: "18.0.5", working_directory: dir },
      steps: [{ step: 1, tool: "qsv_stats", command: "qsv stats data.csv" }],
      file_inventory: {},
    };
    writeFileSync(join(dir, "pipeline.json"), JSON.stringify(existingManifest), "utf-8");

    // Write a JSONL with a different step (simulating crash before JSONL cleanup)
    const step: PipelineStep = {
      step: 1, invocation_id: "inv-1", tool: "qsv_select",
      command: "qsv select name data.csv", args: {}, reason: null,
      timestamp: "2026-03-30T12:00:30Z", duration_ms: 50, success: true,
      kind: "transformative", deterministic: true, input: null, output: null, additional_inputs: [],
    };
    writeFileSync(join(dir, ".qsv-pipeline-steps.jsonl"), JSON.stringify(step) + "\n");

    consolidatePipelineManifest(dir, "crash-session");

    // The existing pipeline.json should NOT be overwritten
    const manifest = JSON.parse(readFileSync(join(dir, "pipeline.json"), "utf-8"));
    assert.strictEqual(manifest.session.id, "server-session", "Should preserve server-written manifest");
    assert.strictEqual(manifest.steps[0].tool, "qsv_stats", "Should have original step, not JSONL step");

    // JSONL should be cleaned up
    assert.ok(!existsSync(join(dir, ".qsv-pipeline-steps.jsonl")), "Stale JSONL should be deleted");
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

test("consolidatePipelineManifest: web_search entries produce search: prefix", async () => {
  const consolidatePipelineManifest = await loadConsolidate();

  const dir = makeTempDir();
  try {
    const webSearch = {
      type: "web_search",
      tool: "WebSearch",
      query: "qsv tutorial",
      timestamp: "2026-03-30T12:00:00Z",
    };
    const webSource = {
      type: "web_source",
      tool: "WebFetch",
      url: "https://example.com/docs",
      timestamp: "2026-03-30T12:00:01Z",
    };
    const step: PipelineStep = {
      step: 1, invocation_id: "inv-1", tool: "qsv_sqlp",
      command: "qsv sqlp data.csv", args: {}, reason: null,
      timestamp: "2026-03-30T12:00:02Z", duration_ms: 100, success: true,
      kind: "transformative", deterministic: true, input: null, output: null, additional_inputs: [],
    };

    writeFileSync(
      join(dir, ".qsv-pipeline-steps.jsonl"),
      [webSearch, webSource, step].map(e => JSON.stringify(e)).join("\n") + "\n",
    );

    consolidatePipelineManifest(dir, "test-session");

    const manifest: PipelineManifestJson = JSON.parse(readFileSync(join(dir, "pipeline.json"), "utf-8"));
    assert.deepStrictEqual(manifest.steps[0].web_sources, [
      "search:qsv tutorial",
      "https://example.com/docs",
    ]);
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

test("consolidatePipelineManifest: filters out empty/whitespace url and query", async () => {
  const consolidatePipelineManifest = await loadConsolidate();

  const dir = makeTempDir();
  try {
    const emptyUrl = { type: "web_source", tool: "WebFetch", url: "", timestamp: "2026-03-30T12:00:00Z" };
    const whitespaceUrl = { type: "web_source", tool: "WebFetch", url: "   ", timestamp: "2026-03-30T12:00:01Z" };
    const nullUrl = { type: "web_source", tool: "WebFetch", url: null, timestamp: "2026-03-30T12:00:02Z" };
    const emptyQuery = { type: "web_search", tool: "WebSearch", query: "", timestamp: "2026-03-30T12:00:03Z" };
    const whitespaceQuery = { type: "web_search", tool: "WebSearch", query: "  ", timestamp: "2026-03-30T12:00:04Z" };
    const validUrl = { type: "web_source", tool: "WebFetch", url: "https://valid.com", timestamp: "2026-03-30T12:00:05Z" };
    const step: PipelineStep = {
      step: 1, invocation_id: "inv-1", tool: "qsv_select",
      command: "qsv select name data.csv", args: {}, reason: null,
      timestamp: "2026-03-30T12:00:06Z", duration_ms: 50, success: true,
      kind: "transformative", deterministic: true, input: null, output: null, additional_inputs: [],
    };

    writeFileSync(
      join(dir, ".qsv-pipeline-steps.jsonl"),
      [emptyUrl, whitespaceUrl, nullUrl, emptyQuery, whitespaceQuery, validUrl, step]
        .map(e => JSON.stringify(e)).join("\n") + "\n",
    );

    consolidatePipelineManifest(dir, "test-session");

    const manifest: PipelineManifestJson = JSON.parse(readFileSync(join(dir, "pipeline.json"), "utf-8"));
    // Only the valid URL should survive; empty/whitespace/null entries filtered out
    assert.deepStrictEqual(manifest.steps[0].web_sources, ["https://valid.com"]);
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});
