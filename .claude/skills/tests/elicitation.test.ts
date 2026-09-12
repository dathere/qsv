/**
 * Tests for MCP elicitation-based working directory selection.
 *
 * These drive the REAL WorkingDirManager (src/working-dir-manager.ts) with test doubles for its
 * two collaborators, rather than a copy of its logic. Three helpers used to be reimplemented in
 * this file, under a note saying they "must be kept in sync with the server implementation. If
 * these tests diverge from production, bugs may go undetected." They had diverged, and the note
 * was right about the consequence: two tests asserted a capability gate production does not have
 * (see the gate tests below), and the discovery helper hardcoded ~/Downloads, ~/Documents,
 * ~/Desktop and ~ where production reads the filesystem provider's allowed-directory list. The
 * well-known dirs are still the DEFAULT of that list (config.ts `allowedDirs`), so the observable
 * suggestions did not change -- but the list is overridable via QSV_MCP_ALLOWED_DIRS, and the
 * mirror could not see an override at all.
 *
 * The mirror predated the extraction: mcp-server.ts calls main() at module scope and so cannot be
 * imported, which is why the logic was copied. WorkingDirManager is an ordinary exported class,
 * so that reason is gone.
 */

import { describe, test, beforeEach, afterEach } from "node:test";
import assert from "node:assert";
import { join } from "node:path";
import { writeFileSync, mkdirSync } from "node:fs";
import type { Server } from "@modelcontextprotocol/sdk/server/index.js";
import type { FilesystemResourceProvider } from "./../src/mcp-filesystem.js";
import { ELICITATION_EXEMPT_TOOLS } from "../src/tool-constants.js";
import { WorkingDirManager } from "../src/working-dir-manager.js";
import { createTestDir, cleanupTestDir } from "./test-helpers.js";

/** The two `Server` methods WorkingDirManager uses, typed against the real SDK signatures so a
 * changed signature is a compile error here rather than a silently-satisfied fake. */
type ServerDouble = Pick<Server, "getClientCapabilities" | "elicitInput">;
/** Likewise for the filesystem provider. */
type ProviderDouble = Pick<
  FilesystemResourceProvider,
  "getAllowedDirectories" | "getWorkingDirectory"
>;

function makeManager(opts: {
  capabilities?: ReturnType<Server["getClientCapabilities"]>;
  elicitInput?: ServerDouble["elicitInput"];
  allowedDirs?: readonly string[];
  workingDir: string;
}): WorkingDirManager {
  const server: ServerDouble = {
    getClientCapabilities: () => opts.capabilities,
    elicitInput:
      opts.elicitInput ??
      (async () => {
        throw new Error("elicitInput not stubbed for this test");
      }),
  };
  const provider: ProviderDouble = {
    getAllowedDirectories: () => opts.allowedDirs ?? [],
    getWorkingDirectory: () => opts.workingDir,
  };
  // Casts are confined to this call: the doubles above are structurally checked against the real
  // types, and only the unused remainder of each interface is waived.
  return new WorkingDirManager(
    server as Server,
    provider as FilesystemResourceProvider,
    (dir: string) => dir,
  );
}

/** An `elicitInput` stub that records what schema the manager asked for. */
function recordingElicit(
  result: Awaited<ReturnType<ServerDouble["elicitInput"]>>,
): { calls: Record<string, unknown>[]; fn: ServerDouble["elicitInput"] } {
  const calls: Record<string, unknown>[] = [];
  return {
    calls,
    fn: (async (params: Record<string, unknown>) => {
      calls.push(params);
      return result;
    }) as ServerDouble["elicitInput"],
  };
}

/** Capabilities that let the manager through its gate and on to elicitInput. */
const ELICIT_CAPABLE = { elicitation: {} } as ReturnType<Server["getClientCapabilities"]>;

describe("WorkingDirManager.discoverDirectories", () => {
  let testDir: string;

  beforeEach(async () => {
    testDir = await createTestDir("qsv-elicit");
  });

  afterEach(async () => {
    await cleanupTestDir(testDir);
  });

  test("candidates come from the allowed-directory list, labelled by basename", async () => {
    const alpha = join(testDir, "alpha");
    const beta = join(testDir, "beta");
    mkdirSync(alpha);
    mkdirSync(beta);

    const candidates = await makeManager({
      allowedDirs: [alpha, beta],
      workingDir: testDir,
    }).discoverDirectories();

    assert.deepStrictEqual(candidates, [
      { path: alpha, label: "alpha" },
      { path: beta, label: "beta" },
      { path: testDir, label: "Current Directory" },
    ]);
  });

  test("paths that are missing or are not directories are dropped", async () => {
    const real = join(testDir, "real");
    const missing = join(testDir, "missing");
    const file = join(testDir, "a-file.csv");
    mkdirSync(real);
    writeFileSync(file, "h\n1\n");

    const candidates = await makeManager({
      allowedDirs: [real, missing, file],
      workingDir: testDir,
    }).discoverDirectories();

    assert.deepStrictEqual(
      candidates.map((c) => c.path),
      [real, testDir],
      "only the existing directory and the working dir survive",
    );
  });

  test("the working directory is not listed twice when it is also an allowed dir", async () => {
    const candidates = await makeManager({
      allowedDirs: [testDir],
      workingDir: testDir,
    }).discoverDirectories();

    assert.strictEqual(candidates.length, 1);
    assert.notStrictEqual(
      candidates[0]?.label,
      "Current Directory",
      "first occurrence wins, so the allowed-dir label is kept",
    );
  });
});

describe("WorkingDirManager.buildDirectorySuggestions", () => {
  let testDir: string;

  beforeEach(async () => {
    testDir = await createTestDir("qsv-elicit");
  });

  afterEach(async () => {
    await cleanupTestDir(testDir);
  });

  test("lists every discovered directory and offers the first as the example", async () => {
    const alpha = join(testDir, "alpha");
    mkdirSync(alpha);

    const text = await makeManager({
      allowedDirs: [alpha],
      workingDir: testDir,
    }).buildDirectorySuggestions();

    assert.match(text, /No directory specified/);
    assert.match(text, /Available directories:/);
    assert.ok(text.includes(`  - alpha: ${alpha}`), "labelled entry for the allowed dir");
    assert.ok(text.includes(`  - Current Directory: ${testDir}`));
    assert.ok(text.includes(`directory: "${alpha}"`), "first candidate is the worked example");
  });

  test("falls back to the working directory as the example when nothing is discoverable", async () => {
    const text = await makeManager({
      allowedDirs: [join(testDir, "nope")],
      workingDir: join(testDir, "also-gone"),
    }).buildDirectorySuggestions();

    assert.ok(text.includes(`directory: "${join(testDir, "also-gone")}"`));
  });
});

describe("WorkingDirManager.elicitWorkingDirectory", () => {
  let testDir: string;

  beforeEach(async () => {
    testDir = await createTestDir("qsv-elicit");
  });

  afterEach(async () => {
    await cleanupTestDir(testDir);
  });

  // The gate is `if (capabilities && !capabilities.elicitation)`. Two earlier tests here asserted
  // a DIFFERENT gate -- `!capabilities?.elicitation?.form` -- and so claimed that undefined
  // capabilities, and an `elicitation` object with `form: false`, both short-circuit to the
  // suggestion text. Neither does. Production treats any truthy `elicitation` value as support
  // (deliberately: some clients, e.g. MCPB proxies, advertise the key with no details) and treats
  // absent capabilities as "ask and find out". The tests below pin the real three-way split.
  test("a client that sent capabilities without elicitation gets the suggestion text", async () => {
    const { calls, fn } = recordingElicit({ action: "accept", content: {} });
    const result = await makeManager({
      capabilities: {} as ReturnType<Server["getClientCapabilities"]>,
      elicitInput: fn,
      allowedDirs: [testDir],
      workingDir: testDir,
    }).elicitWorkingDirectory();

    assert.strictEqual(result.directory, undefined);
    assert.match(result.fallback ?? "", /No directory specified/);
    assert.strictEqual(calls.length, 0, "it must not attempt elicitation");
  });

  test("an elicitation capability with no details still counts as support", async () => {
    const { calls, fn } = recordingElicit({
      action: "accept",
      content: { selected_directory: testDir },
    });
    const result = await makeManager({
      capabilities: { elicitation: {} } as ReturnType<Server["getClientCapabilities"]>,
      elicitInput: fn,
      workingDir: testDir,
    }).elicitWorkingDirectory();

    assert.strictEqual(result.directory, testDir);
    assert.strictEqual(calls.length, 1);
  });

  test("absent capabilities are not a refusal: it asks, and the throw becomes the fallback", async () => {
    let asked = 0;
    const result = await makeManager({
      capabilities: undefined,
      elicitInput: (async () => {
        asked++;
        throw new Error("client does not support elicitation");
      }) as ServerDouble["elicitInput"],
      allowedDirs: [testDir],
      workingDir: testDir,
    }).elicitWorkingDirectory();

    assert.strictEqual(asked, 1, "the gate does not fire when capabilities are undefined");
    assert.match(result.fallback ?? "", /Available directories:/);
  });

  test("returns the enum selection the user accepted", async () => {
    const result = await makeManager({
      capabilities: ELICIT_CAPABLE,
      elicitInput: recordingElicit({
        action: "accept",
        content: { selected_directory: testDir },
      }).fn,
      workingDir: testDir,
    }).elicitWorkingDirectory();

    assert.strictEqual(result.directory, testDir);
    assert.strictEqual(result.fallback, undefined);
  });

  test("custom_path overrides the enum selection", async () => {
    const custom = join(testDir, "custom");
    mkdirSync(custom);

    const result = await makeManager({
      capabilities: ELICIT_CAPABLE,
      elicitInput: recordingElicit({
        action: "accept",
        content: { selected_directory: testDir, custom_path: custom },
      }).fn,
      workingDir: testDir,
    }).elicitWorkingDirectory();

    assert.strictEqual(result.directory, custom);
  });

  test("custom_path is trimmed", async () => {
    const result = await makeManager({
      capabilities: ELICIT_CAPABLE,
      elicitInput: recordingElicit({
        action: "accept",
        content: { custom_path: `  ${testDir}  ` },
      }).fn,
      workingDir: testDir,
    }).elicitWorkingDirectory();

    assert.strictEqual(result.directory, testDir);
  });

  test("a whitespace-only custom_path falls through to the enum selection", async () => {
    const result = await makeManager({
      capabilities: ELICIT_CAPABLE,
      elicitInput: recordingElicit({
        action: "accept",
        content: { selected_directory: testDir, custom_path: "   " },
      }).fn,
      workingDir: testDir,
    }).elicitWorkingDirectory();

    assert.strictEqual(result.directory, testDir);
  });

  test("accepting without choosing anything asks for an explicit path", async () => {
    const result = await makeManager({
      capabilities: ELICIT_CAPABLE,
      elicitInput: recordingElicit({ action: "accept", content: {} }).fn,
      workingDir: testDir,
    }).elicitWorkingDirectory();

    assert.strictEqual(result.directory, undefined);
    assert.match(result.fallback ?? "", /No directory was selected/);
  });

  test("declining leaves the working directory unchanged", async () => {
    const result = await makeManager({
      capabilities: ELICIT_CAPABLE,
      elicitInput: recordingElicit({ action: "decline" }).fn,
      workingDir: testDir,
    }).elicitWorkingDirectory();

    assert.match(result.fallback ?? "", /declined/);
    assert.match(result.fallback ?? "", /remains unchanged/);
  });

  test("cancelling leaves the working directory unchanged", async () => {
    const result = await makeManager({
      capabilities: ELICIT_CAPABLE,
      elicitInput: recordingElicit({ action: "cancel" }).fn,
      workingDir: testDir,
    }).elicitWorkingDirectory();

    assert.match(result.fallback ?? "", /cancelled/);
    assert.match(result.fallback ?? "", /remains unchanged/);
  });

  test("a chosen path that does not exist is rejected, not adopted", async () => {
    const missing = join(testDir, "no-such-dir");
    const result = await makeManager({
      capabilities: ELICIT_CAPABLE,
      elicitInput: recordingElicit({
        action: "accept",
        content: { custom_path: missing },
      }).fn,
      workingDir: testDir,
    }).elicitWorkingDirectory();

    assert.strictEqual(result.directory, undefined);
    assert.ok(result.fallback?.includes("does not exist or is not accessible"));
    assert.ok(result.fallback?.includes(missing));
  });

  test("a chosen path that is a file is rejected with a distinct message", async () => {
    const file = join(testDir, "data.csv");
    writeFileSync(file, "h\n1\n");

    const result = await makeManager({
      capabilities: ELICIT_CAPABLE,
      elicitInput: recordingElicit({
        action: "accept",
        content: { custom_path: file },
      }).fn,
      workingDir: testDir,
    }).elicitWorkingDirectory();

    assert.strictEqual(result.directory, undefined);
    assert.ok(result.fallback?.includes("is not a directory"));
  });

  test("the form schema offers exactly the discovered directories", async () => {
    const alpha = join(testDir, "alpha");
    mkdirSync(alpha);

    const { calls, fn } = recordingElicit({
      action: "accept",
      content: { selected_directory: alpha },
    });
    await makeManager({
      capabilities: ELICIT_CAPABLE,
      elicitInput: fn,
      allowedDirs: [alpha],
      workingDir: testDir,
    }).elicitWorkingDirectory();

    assert.strictEqual(calls.length, 1);
    const params = calls[0] as {
      mode?: string;
      requestedSchema?: {
        properties?: {
          selected_directory?: { enum?: string[]; oneOf?: { const: string; title: string }[] };
          custom_path?: unknown;
        };
      };
    };
    assert.strictEqual(params.mode, "form");
    const selected = params.requestedSchema?.properties?.selected_directory;
    assert.deepStrictEqual(selected?.enum, [alpha, testDir]);
    assert.deepStrictEqual(selected?.oneOf?.map((o) => o.const), [alpha, testDir]);
    assert.ok(
      selected?.oneOf?.[0]?.title.includes("alpha"),
      "the label the user reads comes from discoverDirectories",
    );
    assert.ok(params.requestedSchema?.properties?.custom_path, "custom path stays available");
  });
});

describe("ELICITATION_EXEMPT_TOOLS", () => {
  // This used to build its own Set and then assert that Set contained what it had just put in,
  // which no change to production could ever have falsified. It had also drifted: the local copy
  // listed five tools, omitting qsv_setup and qsv_browse_directory. The set now lives in
  // tool-constants.ts and is imported, so these assertions are about the server's real behaviour.
  test("configuration, discovery and logging tools are exempt from the first-use prompt", () => {
    for (const tool of [
      "qsv_config",
      "qsv_setup",
      "qsv_log",
      "qsv_search_tools",
      "qsv_set_working_dir",
      "qsv_get_working_dir",
      "qsv_browse_directory",
    ]) {
      assert.ok(ELICITATION_EXEMPT_TOOLS.has(tool), `${tool} must be exempt`);
    }
    assert.strictEqual(
      ELICITATION_EXEMPT_TOOLS.size,
      7,
      "a newly exempted tool needs a deliberate decision here, not a silent pass",
    );
  });

  test("data tools are NOT exempt, so the first one prompts for a working directory", () => {
    for (const tool of [
      "qsv_select",
      "qsv_stats",
      "qsv_count",
      "qsv_frequency",
      "qsv_list_files",
      "qsv_command",
    ]) {
      assert.strictEqual(
        ELICITATION_EXEMPT_TOOLS.has(tool),
        false,
        `${tool} reads or writes data and must NOT be exempt`,
      );
    }
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
