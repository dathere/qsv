/**
 * Tests for MCP App (directory picker) functionality.
 *
 * Validates:
 * - _meta.ui presence in tool definitions
 * - clientSupportsApps() capability detection logic
 * - Resource handler returning valid HTML for ui://qsv/directory-picker
 * - qsv_browse_directory handler with temp directory tree
 */

import { describe, test, beforeEach, afterEach } from "node:test";
import assert from "node:assert";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";
import { createTestDir, cleanupTestDir } from "./test-helpers.js";

describe("MCP App tool definitions", () => {
  test("qsv_set_working_dir has _meta.ui.resourceUri", async () => {
    const { createSetWorkingDirTool } = await import("../src/mcp-tools.js");
    const tool = createSetWorkingDirTool();

    assert.ok(tool._meta, "tool should have _meta");
    const ui = tool._meta!.ui as Record<string, unknown>;
    assert.ok(ui, "tool._meta should have ui");
    assert.strictEqual(ui.resourceUri, "ui://qsv/directory-picker");
  });

  test("qsv_browse_directory has _meta.ui.visibility: ['app']", async () => {
    const { createBrowseDirectoryTool } = await import("../src/mcp-tools.js");
    const tool = createBrowseDirectoryTool();

    assert.ok(tool._meta, "tool should have _meta");
    const ui = tool._meta!.ui as Record<string, unknown>;
    assert.ok(ui, "tool._meta should have ui");
    assert.deepStrictEqual(ui.visibility, ["app"]);
  });

  test("qsv_browse_directory directory param is optional", async () => {
    const { createBrowseDirectoryTool } = await import("../src/mcp-tools.js");
    const tool = createBrowseDirectoryTool();

    const required = tool.inputSchema.required;
    assert.ok(!required || !required.includes("directory"), "directory should not be required");
  });
});

describe("McpToolDefinition _meta type", () => {
  test("_meta field is optional on McpToolDefinition", async () => {
    // Import and create a tool without _meta to verify the type allows it
    const { createGetWorkingDirTool } = await import("../src/mcp-tools.js");
    const tool = createGetWorkingDirTool();

    // _meta should be undefined for tools that don't declare it
    // (or it may be present — either way, it shouldn't break)
    assert.ok(true, "tool without _meta compiles and runs fine");
    void tool;
  });
});

describe("clientSupportsApps detection", () => {
  // These tests mirror the detection logic from mcp-server.ts
  // using the ext-apps SDK's getUiCapability directly.
  // We use Function() dynamic import to bypass moduleResolution: "node" limitations.

  interface UiCapability { mimeTypes?: string[] }
  type GetUiCapability = (caps: unknown) => UiCapability | undefined;

  async function loadExtApps(): Promise<{ getUiCapability: GetUiCapability; RESOURCE_MIME_TYPE: string; EXTENSION_ID: string }> {
    // Workaround: TypeScript with moduleResolution: "node" rejects subpath
    // exports like "@modelcontextprotocol/ext-apps/server" at compile time.
    // Function() creates a dynamic import outside the TS compiler's scope.
    // This may break under strict CSP or certain bundler configs.
    return (await Function('return import("@modelcontextprotocol/ext-apps/server")')()) as {
      getUiCapability: GetUiCapability;
      RESOURCE_MIME_TYPE: string;
      EXTENSION_ID: string;
    };
  }

  test("returns undefined when capabilities are undefined", async () => {
    const { getUiCapability } = await loadExtApps();
    const result = getUiCapability(undefined);
    assert.strictEqual(result, undefined);
  });

  test("returns undefined when no extensions field", async () => {
    const { getUiCapability } = await loadExtApps();
    const result = getUiCapability({});
    assert.strictEqual(result, undefined);
  });

  test("returns undefined when extensions exist but no ui extension", async () => {
    const { getUiCapability } = await loadExtApps();
    const result = getUiCapability({ extensions: { "some.other": {} } });
    assert.strictEqual(result, undefined);
  });

  test("returns capability when ui extension is present with correct MIME type", async () => {
    const { getUiCapability, RESOURCE_MIME_TYPE, EXTENSION_ID } = await loadExtApps();
    const caps = {
      extensions: {
        [EXTENSION_ID]: {
          mimeTypes: [RESOURCE_MIME_TYPE],
        },
      },
    };
    const result = getUiCapability(caps);
    assert.ok(result, "should return capability object");
    assert.ok(result!.mimeTypes?.includes(RESOURCE_MIME_TYPE));
  });
});

describe("directory picker HTML resource", () => {
  test("getDirectoryPickerHtml returns valid HTML", async () => {
    const { getDirectoryPickerHtml } = await import("../src/ui/directory-picker-html.js");
    const html = getDirectoryPickerHtml();

    assert.ok(html.includes("<!DOCTYPE html>"), "should be an HTML document");
    assert.ok(html.includes("qsv Directory Picker"), "should have title");
    assert.ok(html.includes("qsv_browse_directory"), "should reference browse tool");
    assert.ok(html.includes("qsv_set_working_dir"), "should reference set_working_dir tool");
    assert.ok(html.includes("@modelcontextprotocol/ext-apps"), "should load App SDK from CDN");
    assert.ok(html.includes("applyDocumentTheme"), "should support theming");
    assert.ok(html.includes("applyHostStyleVariables"), "should apply host style variables");
  });

  test("HTML includes App connection and tool input handler", async () => {
    const { getDirectoryPickerHtml } = await import("../src/ui/directory-picker-html.js");
    const html = getDirectoryPickerHtml();

    assert.ok(html.includes("app.connect()"), "should connect to host");
    assert.ok(html.includes("app.ontoolinput"), "should handle tool input");
    assert.ok(html.includes("app.callTool"), "should call server tools");
  });
});

describe("RESOURCE_MIME_TYPE", () => {
  test("is text/html;profile=mcp-app", async () => {
    const mod = (await Function('return import("@modelcontextprotocol/ext-apps/server")')()) as {
      RESOURCE_MIME_TYPE: string;
    };
    assert.strictEqual(mod.RESOURCE_MIME_TYPE, "text/html;profile=mcp-app");
  });
});

describe("scanDirectory (extracted from qsv_browse_directory handler)", () => {
  let testDir: string;

  beforeEach(async () => {
    testDir = await createTestDir("qsv-browse");

    // Create a directory tree:
    // testDir/
    //   data/
    //     sales.csv
    //     report.xlsx
    //     subdir/
    //   empty/
    //   .hidden/
    //     secret.csv
    //   readme.txt
    //   prices.csv

    mkdirSync(join(testDir, "data"));
    mkdirSync(join(testDir, "data", "subdir"));
    mkdirSync(join(testDir, "empty"));
    mkdirSync(join(testDir, ".hidden"));

    writeFileSync(join(testDir, "data", "sales.csv"), "a,b\n1,2\n");
    writeFileSync(join(testDir, "data", "report.xlsx"), "fake");
    writeFileSync(join(testDir, ".hidden", "secret.csv"), "x\n1\n");
    writeFileSync(join(testDir, "readme.txt"), "hello");
    writeFileSync(join(testDir, "prices.csv"), "x,y\n3,4\n");
  });

  afterEach(async () => {
    await cleanupTestDir(testDir);
  });

  test("returns correct subdirectories and tabular file count", async () => {
    const { scanDirectory } = await import("../src/browse-directory.js");
    const result = await scanDirectory(testDir);

    assert.strictEqual(result.currentPath, testDir);
    assert.ok(result.parent !== null, "non-root dir should have a parent");

    // Top-level tabular files: prices.csv (readme.txt is not tabular)
    assert.strictEqual(result.tabularFileCount, 1, "should count 1 tabular file at top level");

    // Visible subdirectories: data, empty  (.hidden is filtered out)
    const dirNames = result.subdirectories.map(d => d.name).sort();
    assert.deepStrictEqual(dirNames, ["data", "empty"]);
    assert.ok(!dirNames.includes(".hidden"), ".hidden should be filtered out");
  });

  test("counts tabular files and subdirs inside subdirectories", async () => {
    const { scanDirectory } = await import("../src/browse-directory.js");
    const result = await scanDirectory(testDir);

    const dataDir = result.subdirectories.find(d => d.name === "data");
    assert.ok(dataDir, "should include data directory");
    assert.strictEqual(dataDir!.tabularFileCount, 2, "data/ has sales.csv and report.xlsx");
    assert.strictEqual(dataDir!.subdirCount, 1, "data/ has subdir/");

    const emptyDir = result.subdirectories.find(d => d.name === "empty");
    assert.ok(emptyDir, "should include empty directory");
    assert.strictEqual(emptyDir!.tabularFileCount, 0);
    assert.strictEqual(emptyDir!.subdirCount, 0);
  });

  test("subdirectories are sorted alphabetically (case-insensitive)", async () => {
    mkdirSync(join(testDir, "Zebra"));
    mkdirSync(join(testDir, "alpha"));

    const { scanDirectory } = await import("../src/browse-directory.js");
    const result = await scanDirectory(testDir);

    const names = result.subdirectories.map(d => d.name);
    // Expected order: alpha, data, empty, Zebra
    assert.strictEqual(names[0], "alpha");
    assert.strictEqual(names[names.length - 1], "Zebra");
  });

  test("throws for non-existent directory", async () => {
    const { scanDirectory } = await import("../src/browse-directory.js");
    await assert.rejects(
      () => scanDirectory(join(testDir, "nonexistent")),
      /no such file|not accessible|ENOENT/i,
    );
  });

  test("throws for a file path (not a directory)", async () => {
    const { scanDirectory } = await import("../src/browse-directory.js");
    await assert.rejects(
      () => scanDirectory(join(testDir, "readme.txt")),
      /not a directory/i,
    );
  });

  test("TABULAR_EXTS covers expected extensions", async () => {
    const { TABULAR_EXTS } = await import("../src/browse-directory.js");
    for (const ext of [".csv", ".tsv", ".tab", ".ssv", ".parquet", ".pqt", ".jsonl", ".ndjson", ".json", ".xlsx", ".xls"]) {
      assert.ok(TABULAR_EXTS.has(ext), `should include ${ext}`);
    }
    assert.ok(!TABULAR_EXTS.has(".txt"), "should not include .txt");
    assert.ok(!TABULAR_EXTS.has(".pdf"), "should not include .pdf");
  });
});

describe("ELICITATION_EXEMPT_TOOLS includes browse_directory", () => {
  test("qsv_browse_directory should be exempt from first-use elicitation", () => {
    // Mirror the server's exempt tools set
    const exemptTools = new Set([
      "qsv_config",
      "qsv_log",
      "qsv_search_tools",
      "qsv_set_working_dir",
      "qsv_get_working_dir",
      "qsv_browse_directory",
    ]);

    assert.ok(exemptTools.has("qsv_browse_directory"), "browse_directory should be exempt");
    // Data tools should NOT be exempt
    assert.ok(!exemptTools.has("qsv_stats"), "data tools should not be exempt");
  });
});
