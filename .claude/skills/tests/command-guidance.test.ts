/**
 * Unit tests for command-guidance.ts
 *
 * Tests the guidance system that enriches tool descriptions with
 * contextual hints, parameter descriptions, and emoji conventions.
 */

import { test, describe, before, afterEach } from "node:test";
import assert from "node:assert";
import {
  loadCommandGuidance,
  getCommandGuidance,
  _resetGuidance,
  _filterGuidanceEntries,
  enhanceParameterDescription,
  enhanceDescription,
} from "../src/command-guidance.js";
import type { QsvSkill } from "../src/types.js";

// Load guidance from YAML before all tests
before(async () => {
  await loadCommandGuidance();
});

// ============================================================================
// COMMAND_GUIDANCE structure
// ============================================================================

describe("COMMAND_GUIDANCE map", () => {
  test("contains entries for all critical commands", () => {
    const guidance = getCommandGuidance();
    const criticalCommands = [
      "select", "stats", "moarstats", "frequency", "sqlp",
      "joinp", "join", "sort", "dedup", "count", "headers",
      "index", "search", "cat", "geocode", "pivotp",
    ];
    for (const cmd of criticalCommands) {
      assert.ok(
        guidance[cmd],
        `Missing guidance for critical command: ${cmd}`,
      );
    }
  });

  test("all entries have at least whenToUse", () => {
    for (const [cmd, guidance] of Object.entries(getCommandGuidance())) {
      assert.ok(
        guidance.whenToUse,
        `Command "${cmd}" is missing whenToUse guidance`,
      );
    }
  });

  test("memory-warning commands have needsMemoryWarning flag", () => {
    const guidance = getCommandGuidance();
    const memoryCommands = ["dedup", "sort", "frequency", "transpose", "table", "reverse"];
    for (const cmd of memoryCommands) {
      const entry = guidance[cmd];
      if (entry) {
        assert.strictEqual(
          entry.needsMemoryWarning,
          true,
          `Memory-intensive command "${cmd}" should have needsMemoryWarning`,
        );
      }
    }
  });

  test("index-accelerated commands have needsIndexHint flag", () => {
    const guidance = getCommandGuidance();
    const indexedCommands = ["search", "sample", "validate", "template", "luau"];
    for (const cmd of indexedCommands) {
      const entry = guidance[cmd];
      if (entry) {
        assert.strictEqual(
          entry.needsIndexHint,
          true,
          `Index-accelerated command "${cmd}" should have needsIndexHint`,
        );
      }
    }
  });

  test("commands with hasCommonMistakes also have errorPrevention", () => {
    for (const [cmd, guidance] of Object.entries(getCommandGuidance())) {
      if (guidance.hasCommonMistakes) {
        assert.ok(
          guidance.errorPrevention,
          `Command "${cmd}" has hasCommonMistakes but no errorPrevention text`,
        );
      }
    }
  });
});

// ============================================================================
// loadCommandGuidance
// ============================================================================

describe("loadCommandGuidance", () => {
  test("validates loaded guidance is non-empty", () => {
    const guidance = getCommandGuidance();
    assert.ok(Object.keys(guidance).length >= 50, "Should have at least 50 entries");
  });

  test("returns cached result on subsequent calls", async () => {
    const first = await loadCommandGuidance();
    const second = await loadCommandGuidance();
    assert.strictEqual(first, second, "Should return same cached object");
  });
});

// Reset-dependent tests in their own describe with independent teardown
describe("loadCommandGuidance reset behavior", () => {
  afterEach(async () => {
    // Always restore guidance so other test suites aren't affected
    _resetGuidance();
    await loadCommandGuidance();
  });

  test("getCommandGuidance returns empty object before load", () => {
    _resetGuidance();
    const guidance = getCommandGuidance();
    assert.strictEqual(Object.keys(guidance).length, 0, "Should be empty before load");
  });

  test("fresh load after reset caches correctly", async () => {
    _resetGuidance();
    const first = await loadCommandGuidance();
    assert.ok(Object.keys(first).length >= 50, "Fresh load should have entries");
    const second = await loadCommandGuidance();
    assert.strictEqual(first, second, "Second call should return cached object");
  });
});

// ============================================================================
// Prototype pollution safety
// ============================================================================

describe("prototype pollution safety", () => {
  test("loaded guidance uses null-prototype object", () => {
    const guidance = getCommandGuidance();
    assert.strictEqual(
      Object.getPrototypeOf(guidance),
      null,
      "Guidance map should have null prototype",
    );
  });

  test("_filterGuidanceEntries rejects dangerous keys while preserving valid entries", () => {
    // Use Object.create(null) so __proto__ is stored as a regular own property,
    // mirroring what the YAML parser produces. In a normal object literal,
    // __proto__ sets the prototype instead of creating an own property.
    const input = Object.create(null) as Record<string, unknown>;
    input["select"] = { whenToUse: "Choose columns" };
    input["__proto__"] = { whenToUse: "Should be rejected" };
    input["constructor"] = { whenToUse: "Should be rejected" };
    input["prototype"] = { whenToUse: "Should be rejected" };
    input["stats"] = { whenToUse: "Quick numeric stats" };

    // Verify __proto__ is actually an own property in our test input
    assert.ok(
      Object.prototype.hasOwnProperty.call(input, "__proto__"),
      "Test setup: __proto__ must be an own property of input",
    );

    const result = _filterGuidanceEntries(input);

    // Valid entries preserved
    assert.ok("select" in result, "Valid key 'select' should be present");
    assert.ok("stats" in result, "Valid key 'stats' should be present");
    assert.strictEqual(result["select"]?.whenToUse, "Choose columns");
    assert.strictEqual(result["stats"]?.whenToUse, "Quick numeric stats");

    // Dangerous keys rejected
    assert.ok(!("__proto__" in result), "__proto__ should be rejected");
    assert.ok(!("constructor" in result), "constructor should be rejected");
    assert.ok(!("prototype" in result), "prototype should be rejected");

    // Result uses null prototype
    assert.strictEqual(Object.getPrototypeOf(result), null);
  });

  test("_filterGuidanceEntries skips entries with only unrecognized keys", () => {
    const input = {
      valid: { whenToUse: "Valid entry" },
      bogus: { typoField: "All keys unrecognized" },
    };
    const result = _filterGuidanceEntries(input);
    assert.ok("valid" in result, "Valid entry should be present");
    assert.ok(!("bogus" in result), "Entry with only unrecognized keys should be rejected");
  });
});

// ============================================================================
// enhanceParameterDescription
// ============================================================================

describe("enhanceParameterDescription", () => {
  test("adds examples for delimiter parameter", () => {
    const enhanced = enhanceParameterDescription("delimiter", "Delimiter character");
    assert.ok(enhanced.includes(","));
    assert.ok(enhanced.includes("\\t"));
    assert.ok(enhanced.includes("|"));
  });

  test("adds syntax examples for select parameter", () => {
    const enhanced = enhanceParameterDescription("select", "Select columns");
    assert.ok(enhanced.includes("1,3,5"));
    assert.ok(enhanced.includes("range"));
    assert.ok(enhanced.includes("regex"));
  });

  test("adds tips for output/output_file parameter", () => {
    const enhanced = enhanceParameterDescription("output", "Output file");
    assert.ok(enhanced.includes("absolute paths"));
    assert.ok(enhanced.includes("850KB"));
  });

  test("adds context for no_headers parameter", () => {
    const enhanced = enhanceParameterDescription("no_headers", "No header row");
    assert.ok(enhanced.includes("no header row"));
  });

  test("adds context for ignore_case parameter", () => {
    const enhanced = enhanceParameterDescription("ignore_case", "Ignore case");
    assert.ok(enhanced.includes("case-insensitive"));
  });

  test("returns original description for unknown parameters", () => {
    const desc = "Some custom parameter";
    const enhanced = enhanceParameterDescription("custom_param", desc);
    assert.strictEqual(enhanced, desc);
  });
});

// ============================================================================
// enhanceDescription
// ============================================================================

describe("enhanceDescription", () => {
  function makeSkill(command: string, overrides?: Partial<QsvSkill>): QsvSkill {
    return {
      name: `qsv-${command}`,
      version: "19.0.0",
      description: `Test description for ${command}`,
      category: "test",
      command: {
        subcommand: command,
        args: [],
        options: [],
      },
      hints: { memory: "constant" as const },
      ...overrides,
    };
  }

  test("includes base description", () => {
    const result = enhanceDescription(makeSkill("unknown_cmd"));
    assert.ok(result.startsWith("Test description for unknown_cmd"));
  });

  test("adds whenToUse guidance with emoji", () => {
    const result = enhanceDescription(makeSkill("select"));
    assert.ok(result.includes("\u{1F4A1}")); // 💡 emoji
    assert.ok(result.includes("Choose columns"));
  });

  test("adds commonPattern guidance with emoji", () => {
    const result = enhanceDescription(makeSkill("stats"));
    assert.ok(result.includes("\u{1F4CB}")); // 📋 emoji
    assert.ok(result.includes("Run 2nd"));
  });

  test("uses 📊 emoji for statsAware commonPattern", () => {
    // frequency is tagged statsAware:true in command-guidance.yaml
    const result = enhanceDescription(makeSkill("frequency"));
    assert.ok(result.includes("\u{1F4CA}"), "expected 📊 emoji for statsAware command"); // 📊
    assert.ok(!result.includes("\u{1F4CB}"), "📋 must not appear at all for a statsAware command"); // 📋
  });

  test("subcommand hint comes from YAML, not hardcoded", () => {
    // cat has subcommandHint set in command-guidance.yaml
    const guidance = getCommandGuidance()["cat"];
    assert.ok(guidance?.subcommandHint, "cat should have subcommandHint in YAML");
    const result = enhanceDescription(makeSkill("cat"));
    assert.ok(result.includes(guidance!.subcommandHint!));
  });

  test("adds memory warning for full-memory commands", () => {
    const result = enhanceDescription(
      makeSkill("dedup", { hints: { memory: "full" as const } }),
    );
    assert.ok(result.includes("\u{26A0}\u{FE0F}")); // ⚠️ emoji
    assert.ok(result.includes("Loads entire CSV"));
  });

  test("adds proportional memory warning", () => {
    const result = enhanceDescription(
      makeSkill("frequency", { hints: { memory: "proportional" as const } }),
    );
    assert.ok(result.includes("Memory \u{221D} unique values")); // ∝
  });

  test("skips memory warning for non-memory-intensive commands", () => {
    const result = enhanceDescription(
      makeSkill("count", { hints: { memory: "full" as const } }),
    );
    // count has no needsMemoryWarning in guidance, so no warning even though hints say "full"
    assert.ok(!result.includes("Loads entire CSV"));
  });

  test("adds index hint for indexed commands with needsIndexHint", () => {
    const result = enhanceDescription(
      makeSkill("search", { hints: { indexed: true, memory: "constant" as const } }),
    );
    assert.ok(result.includes("\u{1F680}")); // 🚀 emoji
    assert.ok(result.includes("Index-accelerated"));
  });

  test("skips index hint when command is not indexed", () => {
    const result = enhanceDescription(
      makeSkill("search", { hints: { memory: "constant" as const } }),
    );
    assert.ok(!result.includes("Index-accelerated"));
  });

  test("adds error prevention for commands with hasCommonMistakes", () => {
    const result = enhanceDescription(makeSkill("cat"));
    assert.ok(result.includes("rows mode requires same column order"));
  });

  test("skips error prevention when hasCommonMistakes is false", () => {
    const result = enhanceDescription(makeSkill("slice"));
    // slice has no hasCommonMistakes, so no errorPrevention text
    const guidance = getCommandGuidance()["slice"];
    if (guidance?.errorPrevention) {
      assert.ok(!result.includes(guidance.errorPrevention));
    }
  });

  test("adds subcommand guidance for cat", () => {
    const result = enhanceDescription(makeSkill("cat"));
    assert.ok(result.includes("SUBCOMMAND"));
    assert.ok(result.includes("rows"));
  });

  test("adds subcommand guidance for geocode", () => {
    const result = enhanceDescription(makeSkill("geocode"));
    assert.ok(result.includes("SUBCOMMAND"));
    assert.ok(result.includes("suggest"));
  });

  test("includes usage examples when available", () => {
    const result = enhanceDescription(
      makeSkill("stats", {
        examples: [
          { description: "Basic stats", command: "qsv stats data.csv" },
          { description: "With cardinality", command: "qsv stats --cardinality data.csv" },
        ],
      }),
    );
    assert.ok(result.includes("\u{1F4DD} EXAMPLES:")); // 📝
    assert.ok(result.includes("qsv stats data.csv"));
  });

  test("limits examples to configured max", () => {
    const manyExamples = Array.from({ length: 10 }, (_, i) => ({
      description: `Example ${i}`,
      command: `qsv cmd${i} data.csv`,
    }));
    const result = enhanceDescription(
      makeSkill("stats", { examples: manyExamples }),
    );
    // Should show max examples (default 5) plus a "more" indicator
    assert.ok(result.includes("more"));
  });

  test("returns plain description for commands without guidance", () => {
    const result = enhanceDescription(makeSkill("no_guidance_cmd"));
    assert.strictEqual(result, "Test description for no_guidance_cmd");
  });
});
