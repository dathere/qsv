/**
 * Unit tests for tool filtering logic
 *
 * Tests that MCP tools are properly filtered based on available commands
 * in the qsv binary (e.g., qsvlite has fewer commands than full qsv).
 */

import { test } from 'node:test';
import assert from 'node:assert';
import { COMMON_COMMANDS } from '../src/mcp-tools.js';

/**
 * Simulates the tool filtering logic from mcp-server.ts
 */
function filterCommands(
  availableCommands: string[] | undefined,
  commonCommands: readonly string[]
): { filtered: string[]; skipped: string[] } {
  const filtered = availableCommands
    ? commonCommands.filter(cmd => availableCommands.includes(cmd))
    : [...commonCommands]; // Fallback to all if availableCommands not detected

  const skipped = availableCommands
    ? commonCommands.filter(cmd => !availableCommands.includes(cmd))
    : [];

  return { filtered, skipped };
}

test('COMMON_COMMANDS is defined and non-empty', () => {
  assert.ok(Array.isArray(COMMON_COMMANDS));
  assert.ok(COMMON_COMMANDS.length > 0);
});

test('COMMON_COMMANDS contains expected core commands', () => {
  // Updated for token-optimized list of 11 most essential commands
  const coreCommands = ['count', 'headers', 'stats', 'select', 'search', 'index', 'frequency', 'slice', 'sqlp', 'joinp', 'moarstats'];
  const commandsArray: string[] = [...COMMON_COMMANDS]; // Convert to mutable string array
  for (const cmd of coreCommands) {
    assert.ok(commandsArray.includes(cmd), `Expected COMMON_COMMANDS to include '${cmd}'`);
  }
});

test('filterCommands returns all when availableCommands is undefined', () => {
  const { filtered, skipped } = filterCommands(undefined, COMMON_COMMANDS);

  assert.strictEqual(filtered.length, COMMON_COMMANDS.length);
  assert.strictEqual(skipped.length, 0);
  assert.deepStrictEqual(filtered, [...COMMON_COMMANDS]);
});

test('filterCommands filters based on available commands', () => {
  const availableCommands = ['count', 'headers', 'stats', 'select'];
  const { filtered, skipped } = filterCommands(availableCommands, COMMON_COMMANDS);

  // Should only include commands in availableCommands
  assert.strictEqual(filtered.length, 4);
  assert.ok(filtered.includes('count'));
  assert.ok(filtered.includes('headers'));
  assert.ok(filtered.includes('stats'));
  assert.ok(filtered.includes('select'));

  // Should skip commands not in availableCommands
  assert.ok(skipped.length > 0);
  assert.ok(!filtered.includes('sort')); // sort is not in availableCommands
});

test('filterCommands correctly identifies skipped commands', () => {
  const availableCommands = ['count', 'headers'];
  const commonCommands = ['count', 'headers', 'stats', 'select', 'sort'];
  const { filtered, skipped } = filterCommands(availableCommands, commonCommands);

  assert.deepStrictEqual(filtered, ['count', 'headers']);
  assert.deepStrictEqual(skipped, ['stats', 'select', 'sort']);
});

test('filterCommands handles empty available commands', () => {
  const availableCommands: string[] = [];
  const { filtered, skipped } = filterCommands(availableCommands, COMMON_COMMANDS);

  assert.strictEqual(filtered.length, 0);
  assert.strictEqual(skipped.length, COMMON_COMMANDS.length);
});

test('filterCommands simulates qsvlite filtering', () => {
  // Simulate qsvlite which has fewer commands (no moarstats, no Polars commands)
  // Updated for token-optimized COMMON_COMMANDS (11 essential commands)
  const qsvliteCommands = [
    'cat', 'count', 'dedup', 'diff', 'frequency', 'headers',
    'index', 'join', 'rename', 'sample', 'schema', 'search',
    'select', 'slice', 'sort', 'stats', 'validate'
  ];

  const { filtered, skipped } = filterCommands(qsvliteCommands, COMMON_COMMANDS);

  // Feature-gated commands in COMMON_COMMANDS should be skipped for qsvlite
  assert.ok(skipped.includes('moarstats'), 'moarstats should be skipped for qsvlite (needs all_features)');
  assert.ok(skipped.includes('sqlp'), 'sqlp should be skipped for qsvlite (needs Polars)');
  assert.ok(skipped.includes('joinp'), 'joinp should be skipped for qsvlite (needs Polars)');

  // Core commands should be filtered in
  assert.ok(filtered.includes('count'), 'count should be available for qsvlite');
  assert.ok(filtered.includes('stats'), 'stats should be available for qsvlite');
  assert.ok(filtered.includes('select'), 'select should be available for qsvlite');
  assert.ok(filtered.includes('search'), 'search should be available for qsvlite');
  assert.ok(filtered.includes('index'), 'index should be available for qsvlite');
  assert.ok(filtered.includes('frequency'), 'frequency should be available for qsvlite');
});

test('filterCommands preserves order of COMMON_COMMANDS', () => {
  const availableCommands = ['stats', 'count', 'headers']; // Different order
  const commonCommands = ['count', 'headers', 'stats', 'select'];
  const { filtered } = filterCommands(availableCommands, commonCommands);

  // Should preserve order of commonCommands, not availableCommands
  assert.deepStrictEqual(filtered, ['count', 'headers', 'stats']);
});
