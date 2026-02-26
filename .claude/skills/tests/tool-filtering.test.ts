/**
 * Unit tests for tool filtering logic
 *
 * Tests that MCP tools are properly filtered based on available commands
 * in the qsv binary (qsvmcp or full qsv).
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
  // Updated: stats and index moved to CORE_TOOLS in mcp-server.ts
  const coreCommands = ['count', 'headers', 'select', 'search', 'frequency', 'slice', 'sqlp', 'joinp', 'moarstats'];
  const commandsArray: string[] = [...COMMON_COMMANDS]; // Convert to mutable string array
  for (const cmd of coreCommands) {
    assert.ok(commandsArray.includes(cmd), `Expected COMMON_COMMANDS to include '${cmd}'`);
  }
  // stats and index are now in CORE_TOOLS, not COMMON_COMMANDS
  assert.ok(!commandsArray.includes('stats'), 'stats should NOT be in COMMON_COMMANDS (moved to CORE_TOOLS)');
  assert.ok(!commandsArray.includes('index'), 'index should NOT be in COMMON_COMMANDS (moved to CORE_TOOLS)');
});

test('filterCommands returns all when availableCommands is undefined', () => {
  const { filtered, skipped } = filterCommands(undefined, COMMON_COMMANDS);

  assert.strictEqual(filtered.length, COMMON_COMMANDS.length);
  assert.strictEqual(skipped.length, 0);
  assert.deepStrictEqual(filtered, [...COMMON_COMMANDS]);
});

test('filterCommands filters based on available commands', () => {
  const availableCommands = ['count', 'headers', 'select', 'search'];
  const { filtered, skipped } = filterCommands(availableCommands, COMMON_COMMANDS);

  // Should only include commands in availableCommands that are also in COMMON_COMMANDS
  assert.strictEqual(filtered.length, 4);
  assert.ok(filtered.includes('count'));
  assert.ok(filtered.includes('headers'));
  assert.ok(filtered.includes('select'));
  assert.ok(filtered.includes('search'));

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


test('filterCommands preserves order of COMMON_COMMANDS', () => {
  const availableCommands = ['stats', 'count', 'headers']; // Different order
  const commonCommands = ['count', 'headers', 'stats', 'select'];
  const { filtered } = filterCommands(availableCommands, commonCommands);

  // Should preserve order of commonCommands, not availableCommands
  assert.deepStrictEqual(filtered, ['count', 'headers', 'stats']);
});
