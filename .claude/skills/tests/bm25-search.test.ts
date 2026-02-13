/**
 * Tests for BM25 Search Module
 */

import { test } from "node:test";
import assert from "node:assert";
import { ToolSearchIndex } from "../src/bm25-search.js";
import type { QsvSkill } from "../src/types.js";

/**
 * Create a mock skill for testing
 */
function createMockSkill(
  name: string,
  category: string,
  description: string,
  examples: string[] = [],
): QsvSkill {
  return {
    name: `qsv-${name}`,
    version: "1.0.0",
    description,
    category,
    command: {
      subcommand: name,
      args: [],
      options: [],
    },
    examples: examples.map((desc) => ({ description: desc, command: `qsv ${name}` })),
    hints: {
      streamable: true,
      memory: "constant" as const,
    },
  };
}

test("ToolSearchIndex initializes correctly", () => {
  const index = new ToolSearchIndex();
  assert.strictEqual(index.isIndexed(), false);
  assert.strictEqual(index.getIndexedCount(), 0);
});

test("ToolSearchIndex indexes skills", () => {
  const index = new ToolSearchIndex();
  // BM25 requires at least 2 documents for consolidation
  const skills: QsvSkill[] = [
    createMockSkill("select", "selection", "Select columns from CSV"),
    createMockSkill("stats", "analysis", "Calculate statistics"),
    createMockSkill("join", "joining", "Join two CSV files"),
  ];

  index.indexTools(skills);

  assert.strictEqual(index.isIndexed(), true);
  assert.strictEqual(index.getIndexedCount(), 3);
});

test("ToolSearchIndex searches by command name", () => {
  const index = new ToolSearchIndex();
  const skills: QsvSkill[] = [
    createMockSkill("select", "selection", "Select columns from CSV"),
    createMockSkill("stats", "analysis", "Calculate statistics"),
    createMockSkill("join", "joining", "Join two CSV files"),
  ];

  index.indexTools(skills);

  const results = index.search("select");
  assert.ok(results.length > 0, "Should find at least one result for 'select'");
  assert.strictEqual(
    results[0].command.subcommand,
    "select",
    "First result should be 'select' command",
  );
});

test("ToolSearchIndex searches by description content", () => {
  const index = new ToolSearchIndex();
  const skills: QsvSkill[] = [
    createMockSkill("select", "selection", "Select and reorder columns from CSV"),
    createMockSkill("stats", "analysis", "Calculate numeric statistics on data"),
    createMockSkill("join", "joining", "Join two CSV files by common columns"),
  ];

  index.indexTools(skills);

  const results = index.search("statistics");
  assert.ok(results.length > 0, "Should find at least one result for 'statistics'");
  // Stats should be in the results since description contains "statistics"
  const hasStats = results.some((r) => r.command.subcommand === "stats");
  assert.ok(hasStats, "Results should include 'stats' command");
});

test("ToolSearchIndex searches by category", () => {
  const index = new ToolSearchIndex();
  const skills: QsvSkill[] = [
    createMockSkill("select", "selection", "Select columns"),
    createMockSkill("search", "filtering", "Filter rows by pattern"),
    createMockSkill("dedup", "transformation", "Remove duplicate rows"),
  ];

  index.indexTools(skills);

  const results = index.search("filtering");
  assert.ok(results.length > 0, "Should find at least one result for 'filtering'");
  const hasSearch = results.some((r) => r.command.subcommand === "search");
  assert.ok(hasSearch, "Results should include 'search' command in filtering category");
});

test("ToolSearchIndex respects limit parameter", () => {
  const index = new ToolSearchIndex();
  const skills: QsvSkill[] = [
    createMockSkill("select", "selection", "Select columns"),
    createMockSkill("stats", "analysis", "Calculate statistics"),
    createMockSkill("join", "joining", "Join files"),
    createMockSkill("sort", "transformation", "Sort rows"),
    createMockSkill("dedup", "transformation", "Remove duplicates"),
  ];

  index.indexTools(skills);

  const results = index.search("csv", 2);
  assert.ok(results.length <= 2, "Should return at most 2 results when limit is 2");
});

test("ToolSearchIndex returns empty array when not indexed", () => {
  const index = new ToolSearchIndex();

  const results = index.search("anything");
  assert.deepStrictEqual(results, [], "Should return empty array when not indexed");
});

test("ToolSearchIndex reset clears the index", () => {
  const index = new ToolSearchIndex();
  // Need at least 2 documents for BM25 consolidation
  const skills: QsvSkill[] = [
    createMockSkill("select", "selection", "Select columns from CSV"),
    createMockSkill("stats", "analysis", "Calculate statistics on data"),
    createMockSkill("join", "joining", "Join two CSV files"),
  ];

  index.indexTools(skills);
  assert.strictEqual(index.isIndexed(), true);
  assert.strictEqual(index.getIndexedCount(), 3);

  index.reset();
  assert.strictEqual(index.isIndexed(), false);
  assert.strictEqual(index.getIndexedCount(), 0);

  // Verify can re-index after reset
  const newSkills: QsvSkill[] = [
    createMockSkill("count", "utility", "Count rows in CSV file"),
    createMockSkill("headers", "utility", "Display header names"),
    createMockSkill("search", "filtering", "Filter rows by pattern"),
  ];
  index.indexTools(newSkills);
  assert.strictEqual(index.isIndexed(), true);
  assert.strictEqual(index.getIndexedCount(), 3);
});

test("ToolSearchIndex BM25 ranking prioritizes name matches", () => {
  const index = new ToolSearchIndex();
  const skills: QsvSkill[] = [
    // This has "join" in description but not name
    createMockSkill("stats", "analysis", "Calculate stats, can join with other data"),
    // This has "join" in name
    createMockSkill("join", "joining", "Combine two files"),
    // This has "join" in examples
    createMockSkill("select", "selection", "Pick columns", ["join columns together"]),
  ];

  index.indexTools(skills);

  const results = index.search("join");
  assert.ok(results.length > 0, "Should find results for 'join'");
  // Due to field weights (name: 3), the join command should rank first
  assert.strictEqual(
    results[0].command.subcommand,
    "join",
    "Command with 'join' in name should rank first",
  );
});
