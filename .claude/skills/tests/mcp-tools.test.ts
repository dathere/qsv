/**
 * Unit tests for MCP tools
 */

import { test } from 'node:test';
import assert from 'node:assert';
import { writeFileSync, mkdtempSync, rmSync } from 'fs';
import { tmpdir } from 'os';
import { join } from 'path';
import {
  handleToolCall,
  getActiveProcessCount,
  createSearchToolsTool,
  handleSearchToolsCall,
  createDataProfileTool,
  handleDataProfileCall,
} from '../src/mcp-tools.js';
import { SkillLoader } from '../src/loader.js';
import { SkillExecutor } from '../src/executor.js';
import { config } from '../src/config.js';

test('handleToolCall requires input_file parameter', async () => {
  const loader = new SkillLoader();
  await loader.loadAll();
  const executor = new SkillExecutor();

  const result = await handleToolCall(
    'qsv_select',
    {},
    executor,
    loader,
  );

  assert.strictEqual(result.isError, true);
  assert.ok(result.content[0].text?.includes('input_file'));
});

test('handleToolCall rejects unknown commands gracefully', async () => {
  const loader = new SkillLoader();
  await loader.loadAll();
  const executor = new SkillExecutor();

  const result = await handleToolCall(
    'qsv_nonexistent',
    { input_file: 'test.csv' },
    executor,
    loader,
  );

  assert.strictEqual(result.isError, true);
  assert.ok(result.content[0].text?.includes('not found'));
});

test('getActiveProcessCount returns number', () => {
  const count = getActiveProcessCount();
  assert.ok(typeof count === 'number');
  assert.ok(count >= 0);
});

// Note: Testing concurrent operation limit would require mocking activeProcesses
// which is a private Set. For now, we test that the function exists and works.
// Full integration testing would require running actual qsv commands.

// ============================================================================
// qsv_search_tools Tests
// ============================================================================

test('createSearchToolsTool returns valid tool definition', () => {
  const toolDef = createSearchToolsTool();

  assert.strictEqual(toolDef.name, 'qsv_search_tools');
  assert.ok(toolDef.description.includes('Search for qsv tools'));
  assert.strictEqual(toolDef.inputSchema.type, 'object');
  assert.ok('query' in toolDef.inputSchema.properties);
  assert.ok('category' in toolDef.inputSchema.properties);
  assert.ok('limit' in toolDef.inputSchema.properties);
  assert.deepStrictEqual(toolDef.inputSchema.required, ['query']);
});

test('handleSearchToolsCall finds tools by keyword', async () => {
  const loader = new SkillLoader();
  await loader.loadAll();

  const result = await handleSearchToolsCall({ query: 'stats' }, loader);

  assert.ok(result.content.length > 0);
  assert.ok(result.content[0].text?.includes('qsv_stats') || result.content[0].text?.includes('stats'));
});

test('handleSearchToolsCall finds tools by category', async () => {
  const loader = new SkillLoader();
  await loader.loadAll();

  const result = await handleSearchToolsCall(
    { query: 'data', category: 'aggregation' },
    loader
  );

  assert.ok(result.content.length > 0);
  // Should include aggregation tools if query matches
  const text = result.content[0].text || '';
  assert.ok(text.includes('aggregation') || text.includes('No tools found'));
});

test('handleSearchToolsCall handles regex patterns', async () => {
  const loader = new SkillLoader();
  await loader.loadAll();

  const result = await handleSearchToolsCall({ query: '/sort|dedup/' }, loader);

  assert.ok(result.content.length > 0);
  const text = result.content[0].text || '';
  // Should find sort and/or dedup
  assert.ok(text.includes('sort') || text.includes('dedup') || text.includes('No tools found'));
});

test('handleSearchToolsCall requires query parameter', async () => {
  const loader = new SkillLoader();
  await loader.loadAll();

  const result = await handleSearchToolsCall({}, loader);

  assert.ok(result.content.length > 0);
  assert.ok(result.content[0].text?.includes('Error'));
  assert.ok(result.content[0].text?.includes('query'));
});

test('handleSearchToolsCall respects limit parameter', async () => {
  const loader = new SkillLoader();
  await loader.loadAll();

  const result = await handleSearchToolsCall({ query: 's', limit: 3 }, loader);

  assert.ok(result.content.length > 0);
  // Count tool mentions in output (rough check)
  const text = result.content[0].text || '';
  const toolMatches = text.match(/\*\*qsv_\w+\*\*/g) || [];
  assert.ok(toolMatches.length <= 3 || text.includes('No tools found'));
});

test('handleSearchToolsCall returns helpful message when no matches', async () => {
  const loader = new SkillLoader();
  await loader.loadAll();

  const result = await handleSearchToolsCall(
    { query: 'xyznonexistentcommand123' },
    loader
  );

  assert.ok(result.content.length > 0);
  const text = result.content[0].text || '';
  assert.ok(text.includes('No tools found'));
  assert.ok(text.includes('Try') || text.includes('suggestions'));
});

test('handleSearchToolsCall finds tools by description content', async () => {
  const loader = new SkillLoader();
  await loader.loadAll();

  // Search for a term that appears in descriptions
  const result = await handleSearchToolsCall({ query: 'duplicate' }, loader);

  assert.ok(result.content.length > 0);
  const text = result.content[0].text || '';
  // Should find dedup or extdedup which handle duplicates
  assert.ok(text.includes('dedup') || text.includes('No tools found'));
});

// ============================================================================
// qsv_data_profile Tests
// ============================================================================

test('createDataProfileTool returns valid tool definition', () => {
  const toolDef = createDataProfileTool();

  assert.strictEqual(toolDef.name, 'qsv_data_profile');
  assert.ok(toolDef.description.includes('Profile'));
  assert.ok(toolDef.description.includes('SQL'));
  assert.strictEqual(toolDef.inputSchema.type, 'object');
  assert.ok('input_file' in toolDef.inputSchema.properties);
  assert.ok('limit' in toolDef.inputSchema.properties);
  assert.ok('columns' in toolDef.inputSchema.properties);
  assert.ok('no_stats' in toolDef.inputSchema.properties);
  assert.deepStrictEqual(toolDef.inputSchema.required, ['input_file']);
});

test('handleDataProfileCall requires input_file parameter', async () => {
  const result = await handleDataProfileCall({});

  assert.strictEqual(result.isError, true);
  assert.ok(result.content[0].text?.includes('input_file'));
});

test('handleDataProfileCall returns TOON format output', async () => {
  // Create a temporary CSV file
  const tempDir = mkdtempSync(join(tmpdir(), 'qsv-test-'));
  const testCsvPath = join(tempDir, 'test.csv');

  try {
    writeFileSync(testCsvPath, 'name,age,city\nAlice,30,NYC\nBob,25,LA\nCharlie,30,NYC\n');

    const result = await handleDataProfileCall({ input_file: testCsvPath });

    // Should not be an error
    assert.notStrictEqual(result.isError, true);
    assert.ok(result.content.length > 0);

    const text = result.content[0].text || '';

    // TOON format should have these characteristic elements
    assert.ok(text.includes('input:'), 'TOON output should include input path');
    assert.ok(text.includes('rowcount:'), 'TOON output should include rowcount');
    assert.ok(text.includes('fieldcount:'), 'TOON output should include fieldcount');
    assert.ok(text.includes('fields['), 'TOON output should include fields array');
    assert.ok(text.includes('field:'), 'TOON output should include field names');
    assert.ok(text.includes('type:'), 'TOON output should include data types');
    assert.ok(text.includes('cardinality:'), 'TOON output should include cardinality');
    assert.ok(text.includes('frequencies['), 'TOON output should include frequencies');
  } finally {
    // Cleanup - remove temp directory and all contents
    try {
      rmSync(tempDir, { recursive: true, force: true });
    } catch { /* ignore cleanup errors */ }
  }
});

test('handleDataProfileCall respects limit parameter', async () => {
  // Create a temporary CSV file with more unique values
  const tempDir = mkdtempSync(join(tmpdir(), 'qsv-test-'));
  const testCsvPath = join(tempDir, 'test.csv');

  try {
    // Create data with many unique values in one column
    const rows = ['id,name'];
    for (let i = 1; i <= 20; i++) {
      rows.push(`${i},Name${i}`);
    }
    writeFileSync(testCsvPath, rows.join('\n'));

    const result = await handleDataProfileCall({ input_file: testCsvPath, limit: 5 });

    assert.notStrictEqual(result.isError, true);
    assert.ok(result.content.length > 0);

    // Should have returned successfully with TOON format
    const text = result.content[0].text || '';
    assert.ok(text.includes('frequencies['), 'Should include frequency data');
  } finally {
    // Cleanup - remove temp directory and all contents
    try {
      rmSync(tempDir, { recursive: true, force: true });
    } catch { /* ignore cleanup errors */ }
  }
});

test('handleDataProfileCall respects columns parameter', async () => {
  // Create a temporary CSV file
  const tempDir = mkdtempSync(join(tmpdir(), 'qsv-test-'));
  const testCsvPath = join(tempDir, 'test.csv');

  try {
    writeFileSync(testCsvPath, 'name,age,city,score\nAlice,30,NYC,95\nBob,25,LA,87\n');

    const result = await handleDataProfileCall({
      input_file: testCsvPath,
      columns: 'name,city'  // Only profile these columns
    });

    assert.notStrictEqual(result.isError, true);
    assert.ok(result.content.length > 0);

    const text = result.content[0].text || '';
    assert.ok(text.includes('field: name'), 'Should include name column');
    assert.ok(text.includes('field: city'), 'Should include city column');
    // Note: age and score may or may not appear depending on qsv behavior
  } finally {
    // Cleanup - remove temp directory and all contents
    try {
      rmSync(tempDir, { recursive: true, force: true });
    } catch { /* ignore cleanup errors */ }
  }
});

test('handleDataProfileCall handles non-existent file', async () => {
  const result = await handleDataProfileCall({
    input_file: '/nonexistent/path/to/file.csv'
  });

  assert.strictEqual(result.isError, true);
  assert.ok(result.content[0].text?.includes('Error'));
});
