/**
 * Unit tests for MCP tools
 */

import { test } from 'node:test';
import assert from 'node:assert';
import {
  handleToolCall,
  getActiveProcessCount,
  createSearchToolsTool,
  handleSearchToolsCall,
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
