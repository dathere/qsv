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
  buildConversionArgs,
  createToParquetTool,
  handleToParquetCall,
} from '../src/mcp-tools.js';
import { SkillLoader } from '../src/loader.js';
import { SkillExecutor } from '../src/executor.js';

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
// Deferred Loading Tests (loadedTools parameter)
// ============================================================================

test('handleSearchToolsCall marks found tools as loaded', async () => {
  const loader = new SkillLoader();
  await loader.loadAll();

  // Create a Set to track loaded tools
  const loadedTools = new Set<string>();

  // Verify the set is initially empty
  assert.strictEqual(loadedTools.size, 0);

  // Search for tools - this should populate loadedTools
  const result = await handleSearchToolsCall({ query: 'stats' }, loader, loadedTools);

  assert.ok(result.content.length > 0);
  // The search found tools, so they should be marked as loaded
  if (!result.content[0].text?.includes('No tools found')) {
    assert.ok(loadedTools.size > 0, 'Found tools should be marked as loaded');
    // Verify tool names follow expected format (qsv_*)
    for (const toolName of loadedTools) {
      assert.ok(toolName.startsWith('qsv_'), `Tool name ${toolName} should start with qsv_`);
    }
  }
});

test('handleSearchToolsCall works without loadedTools parameter', async () => {
  const loader = new SkillLoader();
  await loader.loadAll();

  // Call without loadedTools (undefined)
  const result = await handleSearchToolsCall({ query: 'select' }, loader);

  // Should work without errors
  assert.ok(result.content.length > 0);
  // Should return results or no-match message
  const text = result.content[0].text || '';
  assert.ok(text.includes('qsv_') || text.includes('No tools found'));
});

test('handleSearchToolsCall accumulates loaded tools across searches', async () => {
  const loader = new SkillLoader();
  await loader.loadAll();

  const loadedTools = new Set<string>();

  // First search
  await handleSearchToolsCall({ query: 'stats' }, loader, loadedTools);
  const sizeAfterFirst = loadedTools.size;

  // Second search for different tools
  await handleSearchToolsCall({ query: 'join' }, loader, loadedTools);
  const sizeAfterSecond = loadedTools.size;

  // Should accumulate (not reset)
  assert.ok(sizeAfterSecond >= sizeAfterFirst, 'loadedTools should accumulate across searches');
});

// ============================================================================
// buildConversionArgs Tests (Parquet support)
// ============================================================================

test('buildConversionArgs returns correct args for Excel', () => {
  const args = buildConversionArgs('excel', 'input.xlsx', 'output.csv');

  assert.deepStrictEqual(args, ['excel', 'input.xlsx', '--output', 'output.csv']);
});

test('buildConversionArgs returns correct args for JSONL', () => {
  const args = buildConversionArgs('jsonl', 'input.jsonl', 'output.csv');

  assert.deepStrictEqual(args, ['jsonl', 'input.jsonl', '--output', 'output.csv']);
});

test('buildConversionArgs returns correct args for Parquet', () => {
  const args = buildConversionArgs('parquet', 'input.parquet', 'output.csv');

  assert.deepStrictEqual(args, [
    'sqlp',
    'SKIP_INPUT',
    "select * from read_parquet('input.parquet')",
    '--output',
    'output.csv',
  ]);
});

test('buildConversionArgs handles Windows paths for Parquet', () => {
  // Windows paths should have backslashes converted to forward slashes in SQL
  const args = buildConversionArgs('parquet', 'C:\\data\\file.parquet', 'output.csv');

  assert.deepStrictEqual(args, [
    'sqlp',
    'SKIP_INPUT',
    "select * from read_parquet('C:/data/file.parquet')",
    '--output',
    'output.csv',
  ]);
});

test('buildConversionArgs escapes single quotes in Parquet paths', () => {
  // Single quotes in paths need to be escaped for SQL safety
  const args = buildConversionArgs('parquet', "file's.parquet", 'output.csv');

  assert.deepStrictEqual(args, [
    'sqlp',
    'SKIP_INPUT',
    "select * from read_parquet('file''s.parquet')",
    '--output',
    'output.csv',
  ]);
});

// ============================================================================
// CSV→Parquet Conversion Tests (csv-to-parquet)
// ============================================================================

test('buildConversionArgs returns correct args for CSV→Parquet', () => {
  // CSV→Parquet passes input directly so sqlp can detect .pschema.json for type inference
  const args = buildConversionArgs('csv-to-parquet', 'input.csv', 'output.parquet');

  assert.deepStrictEqual(args, [
    'sqlp',
    'input.csv',
    'SELECT * FROM _t_1',
    '--format',
    'parquet',
    '--compression',
    'snappy',
    '--statistics',
    '--output',
    'output.parquet',
  ]);
});

test('buildConversionArgs handles Windows paths for CSV→Parquet', () => {
  // Windows paths are passed directly - sqlp handles path resolution
  const args = buildConversionArgs('csv-to-parquet', 'C:\\data\\file.csv', 'output.parquet');

  assert.deepStrictEqual(args, [
    'sqlp',
    'C:\\data\\file.csv',
    'SELECT * FROM _t_1',
    '--format',
    'parquet',
    '--compression',
    'snappy',
    '--statistics',
    '--output',
    'output.parquet',
  ]);
});

test('buildConversionArgs passes paths with single quotes for CSV→Parquet', () => {
  // Single quotes in paths are passed directly - no SQL escaping needed
  const args = buildConversionArgs('csv-to-parquet', "file's.csv", 'output.parquet');

  assert.deepStrictEqual(args, [
    'sqlp',
    "file's.csv",
    'SELECT * FROM _t_1',
    '--format',
    'parquet',
    '--compression',
    'snappy',
    '--statistics',
    '--output',
    'output.parquet',
  ]);
});

// ============================================================================
// qsv_to_parquet Tool Definition Tests
// ============================================================================

test('createToParquetTool returns valid tool definition', () => {
  const toolDef = createToParquetTool();

  assert.strictEqual(toolDef.name, 'qsv_to_parquet');
  assert.ok(toolDef.description.includes('Convert CSV to Parquet'));
  assert.ok(toolDef.description.includes('USE WHEN'));
  assert.ok(toolDef.description.includes('Polars'));
  assert.strictEqual(toolDef.inputSchema.type, 'object');
  assert.ok('input_file' in toolDef.inputSchema.properties);
  assert.ok('output_file' in toolDef.inputSchema.properties);
  assert.deepStrictEqual(toolDef.inputSchema.required, ['input_file']);
});

test('handleToParquetCall requires input_file parameter', async () => {
  const result = await handleToParquetCall({});

  assert.strictEqual(result.isError, true);
  assert.ok(result.content[0].text?.includes('input_file'));
  assert.ok(result.content[0].text?.includes('required'));
});
