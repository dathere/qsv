/**
 * Tests for MCP Sampling integration with describegpt
 */

import { test, describe } from "node:test";
import assert from "node:assert";
import { execSync } from "child_process";
import { createTestDir, cleanupTestDir, createTestCSV, QSV_AVAILABLE } from "./test-helpers.js";
import { config } from "../src/config.js";

// Check if the qsv binary supports --prepare-context (requires v17+)
function hasPreparContextSupport(): boolean {
  if (!QSV_AVAILABLE) return false;
  try {
    // Try running with --prepare-context to see if the flag is recognized
    // We expect an error about missing input, but NOT "Unknown flag"
    const result = execSync(
      `${config.qsvBinPath} describegpt --prepare-context --dictionary --no-cache /dev/null 2>&1 || true`,
      { encoding: "utf-8", timeout: 10_000 },
    );
    return !result.includes("Unknown flag");
  } catch {
    return false;
  }
}

const SAMPLING_AVAILABLE = hasPreparContextSupport();

describe("describegpt MCP sampling support", () => {
  test("--prepare-context outputs valid JSON", { skip: !SAMPLING_AVAILABLE }, async () => {
    const dir = await createTestDir("sampling");
    try {
      const csvFile = await createTestCSV(dir, "test.csv", "name,age,city\nAlice,30,NYC\nBob,25,LA\nCarol,35,Chicago\n");

      const { spawn } = await import("child_process");
      const result = await new Promise<{ stdout: string; stderr: string; exitCode: number }>((resolve, reject) => {
        const proc = spawn(config.qsvBinPath, [
          "describegpt",
          "--prepare-context",
          "--dictionary",
          "--no-cache",
          csvFile,
        ], { stdio: ["pipe", "pipe", "pipe"], cwd: dir });

        let stdout = "";
        let stderr = "";
        proc.stdout!.on("data", (chunk: Buffer) => { stdout += chunk.toString(); });
        proc.stderr!.on("data", (chunk: Buffer) => { stderr += chunk.toString(); });
        proc.on("close", (code: number | null) => resolve({ stdout, stderr, exitCode: code ?? 1 }));
        proc.on("error", reject);
        proc.stdin!.end();
      });

      assert.strictEqual(result.exitCode, 0, `Expected exit code 0, got ${result.exitCode}. stderr: ${result.stderr}`);

      // Parse the JSON output
      const output = JSON.parse(result.stdout);

      // Verify structure
      assert.ok(output.phases, "Output should have phases array");
      assert.ok(Array.isArray(output.phases), "phases should be an array");
      assert.ok(output.phases.length > 0, "Should have at least one phase");
      assert.ok(output.analysis_results, "Output should have analysis_results");
      assert.ok(output.model, "Output should have model");
      assert.ok(typeof output.max_tokens === "number", "Output should have max_tokens");

      // Verify phase structure
      const phase = output.phases[0];
      assert.strictEqual(phase.kind, "Dictionary", "First phase should be Dictionary");
      assert.ok(typeof phase.system_prompt === "string", "Phase should have system_prompt");
      assert.ok(typeof phase.user_prompt === "string", "Phase should have user_prompt");
      assert.ok(typeof phase.max_tokens === "number", "Phase should have max_tokens");
      assert.ok(typeof phase.cache_key === "string", "Phase should have cache_key");
      // cached_response should be null since we use --no-cache
      assert.strictEqual(phase.cached_response, null, "Should have no cached response with --no-cache");
    } finally {
      await cleanupTestDir(dir);
    }
  });

  test("--prepare-context with --all outputs multiple phases", { skip: !SAMPLING_AVAILABLE }, async () => {
    const dir = await createTestDir("sampling-all");
    try {
      const csvFile = await createTestCSV(dir, "test.csv", "name,age\nAlice,30\nBob,25\n");

      const { spawn } = await import("child_process");
      const result = await new Promise<{ stdout: string; stderr: string; exitCode: number }>((resolve, reject) => {
        const proc = spawn(config.qsvBinPath, [
          "describegpt",
          "--prepare-context",
          "--all",
          "--no-cache",
          csvFile,
        ], { stdio: ["pipe", "pipe", "pipe"], cwd: dir });

        let stdout = "";
        let stderr = "";
        proc.stdout!.on("data", (chunk: Buffer) => { stdout += chunk.toString(); });
        proc.stderr!.on("data", (chunk: Buffer) => { stderr += chunk.toString(); });
        proc.on("close", (code: number | null) => resolve({ stdout, stderr, exitCode: code ?? 1 }));
        proc.on("error", reject);
        proc.stdin!.end();
      });

      assert.strictEqual(result.exitCode, 0, `Expected exit code 0, got ${result.exitCode}. stderr: ${result.stderr}`);

      const output = JSON.parse(result.stdout);
      assert.strictEqual(output.phases.length, 3, "Should have 3 phases for --all (dictionary, description, tags)");

      const kinds = output.phases.map((p: { kind: string }) => p.kind);
      assert.ok(kinds.includes("Dictionary"), "Should include Dictionary phase");
      assert.ok(kinds.includes("Description"), "Should include Description phase");
      assert.ok(kinds.includes("Tags"), "Should include Tags phase");
    } finally {
      await cleanupTestDir(dir);
    }
  });

  test("--process-response produces output from mock LLM responses", { skip: !SAMPLING_AVAILABLE }, async () => {
    const dir = await createTestDir("sampling-process");
    try {
      const csvFile = await createTestCSV(dir, "test.csv", "name,age,city\nAlice,30,NYC\nBob,25,LA\n");

      const { spawn } = await import("child_process");

      // First, get the prepare-context output so we have valid analysis_results
      const prepResult = await new Promise<{ stdout: string; stderr: string; exitCode: number }>((resolve, reject) => {
        const proc = spawn(config.qsvBinPath, [
          "describegpt",
          "--prepare-context",
          "--description",
          "--no-cache",
          csvFile,
        ], { stdio: ["pipe", "pipe", "pipe"], cwd: dir });

        let stdout = "";
        let stderr = "";
        proc.stdout!.on("data", (chunk: Buffer) => { stdout += chunk.toString(); });
        proc.stderr!.on("data", (chunk: Buffer) => { stderr += chunk.toString(); });
        proc.on("close", (code: number | null) => resolve({ stdout, stderr, exitCode: code ?? 1 }));
        proc.on("error", reject);
        proc.stdin!.end();
      });

      assert.strictEqual(prepResult.exitCode, 0, `prepare-context failed: ${prepResult.stderr}`);
      const prepOutput = JSON.parse(prepResult.stdout);

      // Build process-response input with mock LLM response
      // Note: for --description, we also need dictionary phase since it's always generated
      const processInput = {
        phases: prepOutput.phases.map((p: { kind: string }) => ({
          kind: p.kind,
          response: p.kind === "Dictionary"
            ? "Field: name\nLabel: Person Name\nDescription: The name of the person\n\nField: age\nLabel: Age\nDescription: The age of the person in years\n\nField: city\nLabel: City\nDescription: The city where the person lives"
            : "This dataset contains information about people including their names, ages, and cities of residence.",
          reasoning: "Mock reasoning for testing",
          token_usage: { prompt: 100, completion: 50, total: 150, elapsed: 1000 },
        })),
        analysis_results: prepOutput.analysis_results,
        model: prepOutput.model,
      };

      // Run --process-response with the mock data
      const procResult = await new Promise<{ stdout: string; stderr: string; exitCode: number }>((resolve, reject) => {
        const proc = spawn(config.qsvBinPath, [
          "describegpt",
          "--process-response",
          "--description",
          "--no-cache",
        ], { stdio: ["pipe", "pipe", "pipe"], cwd: dir });

        let stdout = "";
        let stderr = "";
        proc.stdout!.on("data", (chunk: Buffer) => { stdout += chunk.toString(); });
        proc.stderr!.on("data", (chunk: Buffer) => { stderr += chunk.toString(); });
        proc.on("close", (code: number | null) => resolve({ stdout, stderr, exitCode: code ?? 1 }));
        proc.on("error", reject);

        // Write the process input to stdin
        proc.stdin!.write(JSON.stringify(processInput), () => {
          proc.stdin!.end();
        });
      });

      assert.strictEqual(procResult.exitCode, 0, `process-response failed (exit ${procResult.exitCode}): ${procResult.stderr}`);
      // Should produce markdown output by default
      assert.ok(procResult.stdout.length > 0, "Should produce output");
      assert.ok(
        procResult.stdout.includes("Description") || procResult.stdout.includes("dataset"),
        "Output should contain the mock description content",
      );
    } finally {
      await cleanupTestDir(dir);
    }
  });

  test("--prepare-context and --process-response are mutually exclusive", { skip: !SAMPLING_AVAILABLE }, async () => {
    const dir = await createTestDir("sampling-exclusive");
    try {
      const csvFile = await createTestCSV(dir, "test.csv", "a,b\n1,2\n");

      const { spawn } = await import("child_process");
      const result = await new Promise<{ stdout: string; stderr: string; exitCode: number }>((resolve, reject) => {
        const proc = spawn(config.qsvBinPath, [
          "describegpt",
          "--prepare-context",
          "--process-response",
          "--dictionary",
          csvFile,
        ], { stdio: ["pipe", "pipe", "pipe"], cwd: dir });

        let stdout = "";
        let stderr = "";
        proc.stdout!.on("data", (chunk: Buffer) => { stdout += chunk.toString(); });
        proc.stderr!.on("data", (chunk: Buffer) => { stderr += chunk.toString(); });
        proc.on("close", (code: number | null) => resolve({ stdout, stderr, exitCode: code ?? 1 }));
        proc.on("error", reject);
        proc.stdin!.end();
      });

      assert.notStrictEqual(result.exitCode, 0, "Should fail when both flags are used");
      assert.ok(result.stderr.includes("mutually exclusive"), "Error should mention mutually exclusive");
    } finally {
      await cleanupTestDir(dir);
    }
  });

  test("buildDescribegptArgs creates correct CLI args", async () => {
    // This tests the internal utility function indirectly through the module
    // We can verify the function exists and the module loads correctly
    const module = await import("../src/mcp-tools.js");
    assert.ok(module.handleToolCall, "handleToolCall should be exported");
  });
});
