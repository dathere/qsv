/**
 * Tests for deferred tool loading mechanism
 *
 * Verifies that:
 * - Core tools are always present (10 tools)
 * - Common commands are loaded in deferred mode
 * - Search-discovered tools are tracked and included
 * - Expose-all mode loads all skills
 * - Loader mutex prevents concurrent index rebuilds
 */

import { test } from 'node:test';
import assert from 'node:assert';
import { COMMON_COMMANDS } from '../src/mcp-tools.js';
import { SkillLoader } from '../src/loader.js';

/**
 * The 10 core tools that should always be loaded
 * (matches CORE_TOOLS in mcp-server.ts)
 */
const CORE_TOOLS = [
  "qsv_search_tools",
  "qsv_config",
  "qsv_set_working_dir",
  "qsv_get_working_dir",
  "qsv_list_files",
  "qsv_command",
  "qsv_to_parquet",
  "qsv_index",
  "qsv_stats",
] as const;

// ============================================================================
// Core Tools Count Verification
// ============================================================================

test('CORE_TOOLS has exactly 9 tools', () => {
  assert.strictEqual(CORE_TOOLS.length, 9, 'Should have exactly 9 core tools');
});

test('CORE_TOOLS includes all required utility tools', () => {
  const required = [
    'qsv_search_tools',
    'qsv_config',
    'qsv_set_working_dir',
    'qsv_get_working_dir',
    'qsv_list_files',
    'qsv_command',
    'qsv_to_parquet',
    'qsv_index',
    'qsv_stats',
  ];

  for (const tool of required) {
    assert.ok(
      CORE_TOOLS.includes(tool as typeof CORE_TOOLS[number]),
      `Core tools should include '${tool}'`
    );
  }
});

// ============================================================================
// Common Commands Verification
// ============================================================================

test('COMMON_COMMANDS has 11 commands', () => {
  assert.strictEqual(COMMON_COMMANDS.length, 11, 'Should have 11 common commands');
});

test('COMMON_COMMANDS and CORE_TOOLS are disjoint sets', () => {
  // Core tools are utility/meta tools, common commands are qsv data commands
  // They should not overlap (core tools use qsv_ prefix, common commands are bare names)
  const commonAsToolNames = [...COMMON_COMMANDS].map(cmd => `qsv_${cmd}`);
  const overlap = commonAsToolNames.filter(name =>
    CORE_TOOLS.includes(name as typeof CORE_TOOLS[number])
  );

  assert.strictEqual(overlap.length, 0,
    `COMMON_COMMANDS and CORE_TOOLS should not overlap, but found: ${overlap.join(', ')}`
  );
});

// ============================================================================
// Token Reduction Verification
// ============================================================================

test('deferred loading reduces initial tool count significantly', async () => {
  // In deferred mode: 10 core tools + 11 common commands = 21 tools initially
  // In expose-all mode: all skills from JSON files are available
  // Token reduction = 1 - (initial_tool_count / totalSkillCount)
  // With just core tools (no common), we expect ≥80% reduction

  const loader = new SkillLoader();
  const skills = await loader.loadAll();
  const coreToolCount = CORE_TOOLS.length; // 8
  const totalSkillCount = skills.size;

  const coreOnlyReduction = 1 - (coreToolCount / totalSkillCount);
  assert.ok(
    coreOnlyReduction >= 0.8,
    `Core-only mode should reduce tokens by ≥80% (actual: ${(coreOnlyReduction * 100).toFixed(0)}%)`
  );
});

// ============================================================================
// Skill Loader Tests
// ============================================================================

test('SkillLoader loads all skills from JSON files', async () => {
  const loader = new SkillLoader();
  const skills = await loader.loadAll();

  assert.ok(skills.size > 0, 'Should load at least some skills');
  // Should be approximately 55 skills
  assert.ok(skills.size >= 50, `Should have ≥50 skills (found ${skills.size})`);
  assert.ok(skills.size <= 65, `Should have ≤65 skills (found ${skills.size})`);
});

test('SkillLoader caches loadAll results', async () => {
  const loader = new SkillLoader();

  const skills1 = await loader.loadAll();
  const skills2 = await loader.loadAll();

  // Should return the same Map instance (cached)
  assert.strictEqual(skills1, skills2, 'Second loadAll should return cached result');
  assert.ok(loader.isAllLoaded(), 'isAllLoaded should be true after loadAll');
});

test('SkillLoader concurrent loadAll calls return same result', async () => {
  const loader = new SkillLoader();

  // Make 5 concurrent loadAll calls
  const promises = Array.from({ length: 5 }, () => loader.loadAll());
  const results = await Promise.all(promises);

  // All should return the same Map reference
  const firstResult = results[0];
  for (let i = 1; i < results.length; i++) {
    assert.strictEqual(
      results[i],
      firstResult,
      `Concurrent loadAll call ${i + 1} should return same cached Map`
    );
  }
});

test('SkillLoader loads individual skill by name', async () => {
  const loader = new SkillLoader();

  const skill = await loader.load('qsv-count');
  assert.ok(skill !== null, 'Should load qsv-count skill');
  assert.strictEqual(skill?.name, 'qsv-count', 'Skill name should be qsv-count');
  assert.ok(skill?.description, 'Skill should have a description');
  assert.ok(skill?.command, 'Skill should have a command spec');
});

test('SkillLoader returns null for nonexistent skill', async () => {
  const loader = new SkillLoader();

  const skill = await loader.load('qsv-nonexistent');
  assert.strictEqual(skill, null, 'Should return null for nonexistent skill');
});

test('SkillLoader loadByNames loads multiple skills in parallel', async () => {
  const loader = new SkillLoader();

  const skillNames = ['qsv-count', 'qsv-stats', 'qsv-select'];
  const skills = await loader.loadByNames(skillNames);

  assert.strictEqual(skills.size, 3, 'Should load all 3 requested skills');
  assert.ok(skills.has('qsv-count'), 'Should include qsv-count');
  assert.ok(skills.has('qsv-stats'), 'Should include qsv-stats');
  assert.ok(skills.has('qsv-select'), 'Should include qsv-select');
});

// ============================================================================
// BM25 Search Integration Tests
// ============================================================================

test('SkillLoader builds BM25 index after loadAll', async () => {
  const loader = new SkillLoader();
  await loader.loadAll();

  assert.ok(loader.isBM25Indexed(), 'BM25 index should be built after loadAll');
});

test('SkillLoader search returns relevant results', async () => {
  const loader = new SkillLoader();
  await loader.loadAll();

  const results = loader.search('join');
  assert.ok(results.length > 0, 'Search for "join" should return results');
  // Join-related tools should appear in results
  const names = results.map(s => s.name);
  assert.ok(
    names.some(n => n.includes('join')),
    'Results should include join-related skills'
  );
});

test('SkillLoader search respects limit', async () => {
  const loader = new SkillLoader();
  await loader.loadAll();

  const results = loader.search('data', 3);
  assert.ok(results.length <= 3, 'Should respect the limit parameter');
});

// ============================================================================
// Search-Discovered Tool Tracking Tests
// ============================================================================

test('loadedTools Set tracks discovered tools', () => {
  // Simulate the deferred loading mechanism
  const loadedTools = new Set<string>();

  // Initially empty
  assert.strictEqual(loadedTools.size, 0, 'Should start with no loaded tools');

  // Simulate discovering tools via search
  loadedTools.add('qsv_sort');
  loadedTools.add('qsv_dedup');
  loadedTools.add('qsv_rename');

  assert.strictEqual(loadedTools.size, 3, 'Should track 3 discovered tools');
  assert.ok(loadedTools.has('qsv_sort'), 'Should track qsv_sort');
  assert.ok(loadedTools.has('qsv_dedup'), 'Should track qsv_dedup');

  // Adding same tool again should not increase count
  loadedTools.add('qsv_sort');
  assert.strictEqual(loadedTools.size, 3, 'Should not duplicate tools');
});

test('search-discovered tools are filtered from core tools', () => {
  const loadedTools = new Set<string>();
  loadedTools.add('qsv_sort');
  loadedTools.add('qsv_config'); // This is a core tool
  loadedTools.add('qsv_dedup');

  // Filter out core tools (as done in mcp-server.ts)
  const searchedToolNames = Array.from(loadedTools)
    .filter(name => !CORE_TOOLS.includes(name as typeof CORE_TOOLS[number]))
    .map(name => name.replace('qsv_', 'qsv-'));

  assert.strictEqual(searchedToolNames.length, 2, 'Should exclude core tool qsv_config');
  assert.ok(searchedToolNames.includes('qsv-sort'), 'Should include qsv-sort');
  assert.ok(searchedToolNames.includes('qsv-dedup'), 'Should include qsv-dedup');
  assert.ok(!searchedToolNames.includes('qsv-config'), 'Should not include qsv-config');
});

// ============================================================================
// Category and Stats Tests
// ============================================================================

test('SkillLoader getCategories returns all 10 categories', async () => {
  const loader = new SkillLoader();
  await loader.loadAll();

  const categories = loader.getCategories();
  assert.ok(categories.length >= 8, `Should have at least 8 categories (found ${categories.length})`);

  // Check for expected categories
  const expectedCategories = [
    'selection', 'filtering', 'transformation', 'aggregation',
    'joining', 'validation', 'conversion', 'utility',
  ];
  for (const cat of expectedCategories) {
    assert.ok(
      categories.includes(cat as any),
      `Should include '${cat}' category`
    );
  }
});

test('SkillLoader getStats returns meaningful statistics', async () => {
  const loader = new SkillLoader();
  await loader.loadAll();

  const stats = loader.getStats();
  assert.ok(stats.total >= 50, `Should have ≥50 total skills (found ${stats.total})`);
  assert.ok(stats.totalExamples > 0, 'Should have some examples');
  assert.ok(stats.totalOptions > 0, 'Should have some options');
  assert.ok(stats.totalArgs > 0, 'Should have some args');
  assert.ok(Object.keys(stats.byCategory).length >= 8, 'Should have stats for at least 8 categories');
});
