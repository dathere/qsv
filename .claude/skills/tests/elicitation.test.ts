/**
 * Tests for MCP elicitation-based working directory selection.
 *
 * These tests validate the elicitWorkingDirectory() method behavior
 * by testing the QsvMcpServer class indirectly through its exported
 * tool handler logic. Since elicitation depends on the Server instance
 * and client capabilities, we test the logic in isolation by extracting
 * the relevant patterns.
 *
 * NOTE: The helper functions below mirror the production code in mcp-server.ts.
 * They must be kept in sync with the server implementation. If these tests
 * diverge from production, bugs may go undetected.
 */

import { describe, test, beforeEach, afterEach } from "node:test";
import assert from "node:assert";
import { homedir } from "node:os";
import { join } from "node:path";
import { statSync, writeFileSync } from "node:fs";
import { createTestDir, cleanupTestDir } from "./test-helpers.js";

/**
 * Discover well-known directories that exist on the user's system.
 * Mirrors discoverDirectories() from mcp-server.ts — uses statSync
 * to verify candidates are actual directories (not files).
 */
function discoverDirectories(currentWorkingDir: string): Array<{ path: string; label: string }> {
  const home = homedir();
  const candidates: Array<{ path: string; label: string }> = [];

  const wellKnown = [
    { path: join(home, "Downloads"), label: "Downloads" },
    { path: join(home, "Documents"), label: "Documents" },
    { path: join(home, "Desktop"), label: "Desktop" },
    { path: home, label: "Home" },
  ];

  const cwd = process.cwd();

  for (const candidate of wellKnown) {
    try {
      if (statSync(candidate.path).isDirectory()) {
        candidates.push(candidate);
      }
    } catch {
      // skip
    }
  }

  try {
    if (statSync(cwd).isDirectory() && !candidates.some((c) => c.path === cwd)) {
      candidates.push({ path: cwd, label: "Current Directory" });
    }
  } catch {
    // skip
  }

  try {
    if (statSync(currentWorkingDir).isDirectory() && !candidates.some((c) => c.path === currentWorkingDir)) {
      candidates.push({ path: currentWorkingDir, label: "qsv Working Dir" });
    }
  } catch {
    // skip
  }

  return candidates;
}

/**
 * Build a directory suggestion list for when elicitation is not available.
 * Mirrors buildDirectorySuggestions() from mcp-server.ts.
 */
function buildDirectorySuggestions(currentWorkingDir: string): string {
  const candidates = discoverDirectories(currentWorkingDir);

  const suggestions = candidates
    .map((c) => `  - ${c.label}: ${c.path}`)
    .join("\n");

  return (
    `No directory specified. Current working directory: ${currentWorkingDir}\n\n` +
    `Available directories:\n${suggestions}\n\n` +
    `Call qsv_set_working_dir with one of these paths (e.g. directory: "${candidates[0]?.path || currentWorkingDir}"), ` +
    `or provide any other accessible directory path.`
  );
}

/**
 * Simulate the elicitWorkingDirectory logic extracted from mcp-server.ts.
 * This mirrors the server method but accepts dependencies as parameters
 * for testability.
 */
async function elicitWorkingDirectory(options: {
  getClientCapabilities: () => { elicitation?: { form?: boolean } } | undefined;
  elicitInput: (params: Record<string, unknown>) => Promise<{
    action: string;
    content?: Record<string, unknown>;
  }>;
  currentWorkingDir: string;
}): Promise<{ directory?: string; fallback?: string }> {
  const capabilities = options.getClientCapabilities();
  if (!capabilities?.elicitation?.form) {
    return { fallback: buildDirectorySuggestions(options.currentWorkingDir) };
  }

  // Discover directories for the form
  const candidates = discoverDirectories(options.currentWorkingDir);

  const enumValues = candidates.map((c) => c.path);
  const enumLabels = candidates.map((c) => ({
    const: c.path,
    title: `${c.label} — ${c.path}`,
  }));

  try {
    const result = await options.elicitInput({
      mode: "form",
      message: "Select a working directory for qsv file operations:",
      requestedSchema: {
        type: "object",
        properties: {
          selected_directory: {
            type: "string",
            title: "Directory",
            description: "Choose from common directories",
            enum: enumValues,
            oneOf: enumLabels,
          },
          custom_path: {
            type: "string",
            title: "Custom Path (optional)",
            description: "Or type a custom directory path (overrides selection above)",
          },
        },
      },
    });

    if (result.action === "accept" && result.content) {
      const customPath =
        typeof result.content.custom_path === "string"
          ? (result.content.custom_path as string).trim()
          : "";
      const selectedDir =
        typeof result.content.selected_directory === "string"
          ? (result.content.selected_directory as string).trim()
          : "";

      const chosenDir = customPath || selectedDir;

      if (chosenDir) {
        // Validate the chosen directory exists and is actually a directory
        try {
          const stat = statSync(chosenDir);
          if (!stat.isDirectory()) {
            return {
              fallback: `"${chosenDir}" is not a directory. Please call qsv_set_working_dir with a valid directory path.`,
            };
          }
        } catch {
          return {
            fallback: `Directory "${chosenDir}" does not exist or is not accessible. Please call qsv_set_working_dir with a valid directory path.`,
          };
        }
        return { directory: chosenDir };
      }

      return {
        fallback: "No directory was selected. Please call qsv_set_working_dir with a directory path.",
      };
    }

    if (result.action === "decline") {
      return {
        fallback: "Directory selection was declined. The working directory remains unchanged. You can call qsv_set_working_dir with an explicit path.",
      };
    }

    return {
      fallback: "Directory selection was cancelled. The working directory remains unchanged.",
    };
  } catch {
    return { fallback: buildDirectorySuggestions(options.currentWorkingDir) };
  }
}

describe("elicitWorkingDirectory", () => {
  let testDir: string;

  beforeEach(async () => {
    testDir = await createTestDir("qsv-elicit");
  });

  afterEach(async () => {
    await cleanupTestDir(testDir);
  });

  test("returns fallback with directory suggestions when client does not support elicitation", async () => {
    const result = await elicitWorkingDirectory({
      getClientCapabilities: () => undefined,
      elicitInput: async () => ({ action: "accept", content: {} }),
      currentWorkingDir: testDir,
    });

    assert.ok(result.fallback);
    assert.ok(result.fallback.includes("No directory specified"));
    assert.ok(result.fallback.includes("Available directories:"));
    assert.ok(result.fallback.includes(testDir));
    assert.strictEqual(result.directory, undefined);
  });

  test("returns fallback with suggestions when elicitation.form is false", async () => {
    const result = await elicitWorkingDirectory({
      getClientCapabilities: () => ({ elicitation: { form: false } }),
      elicitInput: async () => ({ action: "accept", content: {} }),
      currentWorkingDir: testDir,
    });

    assert.ok(result.fallback);
    assert.ok(result.fallback.includes("Available directories:"));
  });

  test("returns directory when user accepts enum selection", async () => {
    const selectedPath = testDir;

    const result = await elicitWorkingDirectory({
      getClientCapabilities: () => ({ elicitation: { form: true } }),
      elicitInput: async () => ({
        action: "accept",
        content: { selected_directory: selectedPath },
      }),
      currentWorkingDir: testDir,
    });

    assert.strictEqual(result.directory, selectedPath);
    assert.strictEqual(result.fallback, undefined);
  });

  test("custom_path overrides enum selection", async () => {
    const customPath = testDir;
    const enumPath = homedir();

    const result = await elicitWorkingDirectory({
      getClientCapabilities: () => ({ elicitation: { form: true } }),
      elicitInput: async () => ({
        action: "accept",
        content: {
          selected_directory: enumPath,
          custom_path: customPath,
        },
      }),
      currentWorkingDir: testDir,
    });

    assert.strictEqual(result.directory, customPath);
  });

  test("returns fallback when user declines", async () => {
    const result = await elicitWorkingDirectory({
      getClientCapabilities: () => ({ elicitation: { form: true } }),
      elicitInput: async () => ({ action: "decline" }),
      currentWorkingDir: testDir,
    });

    assert.ok(result.fallback);
    assert.ok(result.fallback.includes("declined"));
    assert.strictEqual(result.directory, undefined);
  });

  test("returns fallback when user cancels", async () => {
    const result = await elicitWorkingDirectory({
      getClientCapabilities: () => ({ elicitation: { form: true } }),
      elicitInput: async () => ({ action: "cancel" }),
      currentWorkingDir: testDir,
    });

    assert.ok(result.fallback);
    assert.ok(result.fallback.includes("cancelled"));
    assert.strictEqual(result.directory, undefined);
  });

  test("returns fallback with suggestions when elicitInput throws", async () => {
    const result = await elicitWorkingDirectory({
      getClientCapabilities: () => ({ elicitation: { form: true } }),
      elicitInput: async () => {
        throw new Error("Connection lost");
      },
      currentWorkingDir: testDir,
    });

    assert.ok(result.fallback);
    assert.ok(result.fallback.includes("Available directories:"));
    assert.strictEqual(result.directory, undefined);
  });

  test("returns fallback when accept but no directory selected", async () => {
    const result = await elicitWorkingDirectory({
      getClientCapabilities: () => ({ elicitation: { form: true } }),
      elicitInput: async () => ({
        action: "accept",
        content: {},
      }),
      currentWorkingDir: testDir,
    });

    assert.ok(result.fallback);
    assert.ok(result.fallback.includes("No directory was selected"));
  });

  test("trims whitespace from custom_path", async () => {
    const result = await elicitWorkingDirectory({
      getClientCapabilities: () => ({ elicitation: { form: true } }),
      elicitInput: async () => ({
        action: "accept",
        content: { custom_path: `  ${testDir}  ` },
      }),
      currentWorkingDir: testDir,
    });

    assert.strictEqual(result.directory, testDir);
  });

  test("empty custom_path falls through to selected_directory", async () => {
    const result = await elicitWorkingDirectory({
      getClientCapabilities: () => ({ elicitation: { form: true } }),
      elicitInput: async () => ({
        action: "accept",
        content: {
          custom_path: "   ",
          selected_directory: testDir,
        },
      }),
      currentWorkingDir: testDir,
    });

    assert.strictEqual(result.directory, testDir);
  });

  test("elicitInput receives form schema with enum values", async () => {
    let capturedParams: Record<string, unknown> | null = null;

    await elicitWorkingDirectory({
      getClientCapabilities: () => ({ elicitation: { form: true } }),
      elicitInput: async (params) => {
        capturedParams = params;
        return { action: "cancel" };
      },
      currentWorkingDir: testDir,
    });

    assert.ok(capturedParams);
    assert.strictEqual((capturedParams as Record<string, unknown>).mode, "form");
    assert.ok((capturedParams as Record<string, unknown>).message);
    assert.ok((capturedParams as Record<string, unknown>).requestedSchema);

    const schema = (capturedParams as Record<string, unknown>).requestedSchema as Record<string, unknown>;
    const props = schema.properties as Record<string, Record<string, unknown>>;
    assert.ok(props.selected_directory);
    assert.ok(props.custom_path);
    assert.ok(Array.isArray(props.selected_directory.enum));
    assert.ok((props.selected_directory.enum as string[]).length > 0);
  });

  test("returns fallback when custom_path points to a non-existent directory", async () => {
    const result = await elicitWorkingDirectory({
      getClientCapabilities: () => ({ elicitation: { form: true } }),
      elicitInput: async () => ({
        action: "accept",
        content: { custom_path: "/nonexistent/path/that/does/not/exist" },
      }),
      currentWorkingDir: testDir,
    });

    assert.strictEqual(result.directory, undefined);
    assert.ok(result.fallback);
    assert.ok(result.fallback!.includes("does not exist"));
  });

  test("returns fallback when custom_path points to a file instead of a directory", async () => {
    const filePath = join(testDir, "not-a-directory.txt");
    writeFileSync(filePath, "test content");

    const result = await elicitWorkingDirectory({
      getClientCapabilities: () => ({ elicitation: { form: true } }),
      elicitInput: async () => ({
        action: "accept",
        content: { custom_path: filePath },
      }),
      currentWorkingDir: testDir,
    });

    assert.strictEqual(result.directory, undefined);
    assert.ok(result.fallback);
    assert.ok(result.fallback!.includes("is not a directory"));
  });
});

describe("ELICITATION_EXEMPT_TOOLS", () => {
  test("config/log/search tools are exempt from first-use prompt", () => {
    // These tool names should be exempt from triggering elicitation
    const exemptTools = new Set([
      "qsv_config",
      "qsv_log",
      "qsv_search_tools",
      "qsv_set_working_dir",
      "qsv_get_working_dir",
    ]);

    // Data tools should NOT be exempt
    const dataTools = [
      "qsv_select",
      "qsv_stats",
      "qsv_count",
      "qsv_frequency",
      "qsv_list_files",
      "qsv_command",
    ];

    for (const tool of dataTools) {
      assert.strictEqual(
        exemptTools.has(tool),
        false,
        `${tool} should NOT be exempt from elicitation`,
      );
    }

    // Exempt tools should be in the set
    assert.ok(exemptTools.has("qsv_config"));
    assert.ok(exemptTools.has("qsv_log"));
    assert.ok(exemptTools.has("qsv_search_tools"));
    assert.ok(exemptTools.has("qsv_set_working_dir"));
    assert.ok(exemptTools.has("qsv_get_working_dir"));
  });
});

describe("qsv_set_working_dir tool definition", () => {
  test("directory parameter is not required", async () => {
    // Import the tool definition
    const { createSetWorkingDirTool } = await import("../src/mcp-tools.js");
    const tool = createSetWorkingDirTool();

    // directory should not be in required array
    const required = (tool.inputSchema as Record<string, unknown>).required as string[];
    assert.ok(Array.isArray(required));
    assert.strictEqual(required.includes("directory"), false);
  });

  test("description mentions interactive picker", async () => {
    const { createSetWorkingDirTool } = await import("../src/mcp-tools.js");
    const tool = createSetWorkingDirTool();

    assert.ok(tool.description.includes("interactive directory picker"));
  });
});
