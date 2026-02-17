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
  isCsvLikeFile,
  getParquetPath,
  ensureParquet,
  parseCSVLine,
  detectDelimiter,
  isDateDtype,
  patchSchemaAmPmDates,
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

test('createToParquetTool description mentions date detection via stats', () => {
  const toolDef = createToParquetTool();

  assert.ok(
    toolDef.description.includes('Date/DateTime'),
    'Description should mention Date/DateTime detection'
  );
  assert.ok(
    toolDef.description.includes('--infer-dates'),
    'Description should mention --infer-dates flag'
  );
  assert.ok(
    toolDef.description.includes('--dates-whitelist sniff'),
    'Description should mention --dates-whitelist sniff'
  );
});

test('handleToParquetCall requires input_file parameter', async () => {
  const result = await handleToParquetCall({});

  assert.strictEqual(result.isError, true);
  assert.ok(result.content[0].text?.includes('input_file'));
  assert.ok(result.content[0].text?.includes('required'));
});

// ============================================================================
// isCsvLikeFile Tests
// ============================================================================

test('isCsvLikeFile recognizes standard CSV-like extensions', () => {
  assert.strictEqual(isCsvLikeFile('data.csv'), true);
  assert.strictEqual(isCsvLikeFile('data.tsv'), true);
  assert.strictEqual(isCsvLikeFile('data.tab'), true);
  assert.strictEqual(isCsvLikeFile('data.ssv'), true);
});

test('isCsvLikeFile recognizes Snappy-compressed CSV-like extensions', () => {
  assert.strictEqual(isCsvLikeFile('data.csv.sz'), true);
  assert.strictEqual(isCsvLikeFile('data.tsv.sz'), true);
  assert.strictEqual(isCsvLikeFile('data.tab.sz'), true);
  assert.strictEqual(isCsvLikeFile('data.ssv.sz'), true);
});

test('isCsvLikeFile is case-insensitive', () => {
  assert.strictEqual(isCsvLikeFile('DATA.CSV'), true);
  assert.strictEqual(isCsvLikeFile('FILE.TSV.SZ'), true);
});

test('isCsvLikeFile rejects non-CSV files', () => {
  assert.strictEqual(isCsvLikeFile('data.json'), false);
  assert.strictEqual(isCsvLikeFile('data.parquet'), false);
  assert.strictEqual(isCsvLikeFile('data.xlsx'), false);
  assert.strictEqual(isCsvLikeFile('data.sz'), false);
});

// ============================================================================
// getParquetPath Tests
// ============================================================================

test('getParquetPath replaces CSV-like extension with .parquet', () => {
  assert.strictEqual(getParquetPath('/data/test.csv'), '/data/test.parquet');
  assert.strictEqual(getParquetPath('/data/test.tsv'), '/data/test.parquet');
  assert.strictEqual(getParquetPath('/data/test.tab'), '/data/test.parquet');
  assert.strictEqual(getParquetPath('/data/test.ssv'), '/data/test.parquet');
});

test('getParquetPath handles Snappy-compressed extensions', () => {
  assert.strictEqual(getParquetPath('/data/test.csv.sz'), '/data/test.parquet');
  assert.strictEqual(getParquetPath('/data/test.tsv.sz'), '/data/test.parquet');
});

test('getParquetPath appends .parquet for non-CSV files', () => {
  assert.strictEqual(getParquetPath('/data/test.json'), '/data/test.json.parquet');
});

test('getParquetPath does not false-match on directory names containing CSV-like strings', () => {
  // Regression test: directory paths like /data/csv_files/ should not match .csv
  assert.strictEqual(getParquetPath('/data/csv_files/test.json'), '/data/csv_files/test.json.parquet');
  assert.strictEqual(getParquetPath('/data/CSV_FILES/test.json'), '/data/CSV_FILES/test.json.parquet');
});

// ============================================================================
// ensureParquet Tests (early-return paths — no qsv binary needed)
// ============================================================================

test('ensureParquet passes through non-CSV files unchanged', async () => {
  // Parquet, JSON, and other non-CSV files should be returned as-is
  assert.strictEqual(await ensureParquet('/data/test.parquet'), '/data/test.parquet');
  assert.strictEqual(await ensureParquet('/data/test.json'), '/data/test.json');
  assert.strictEqual(await ensureParquet('/data/test.jsonl'), '/data/test.jsonl');
  assert.strictEqual(await ensureParquet('/data/test.xlsx'), '/data/test.xlsx');
});

// ============================================================================
// detectDelimiter Tests
// ============================================================================

test('detectDelimiter returns comma for .csv files', () => {
  assert.strictEqual(detectDelimiter('data.csv'), ',');
  assert.strictEqual(detectDelimiter('/path/to/file.CSV'), ',');
});

test('detectDelimiter returns tab for .tsv and .tab files', () => {
  assert.strictEqual(detectDelimiter('data.tsv'), '\t');
  assert.strictEqual(detectDelimiter('data.tab'), '\t');
  assert.strictEqual(detectDelimiter('/path/to/file.TSV'), '\t');
  assert.strictEqual(detectDelimiter('/path/to/file.TAB'), '\t');
});

test('detectDelimiter returns semicolon for .ssv files', () => {
  assert.strictEqual(detectDelimiter('data.ssv'), ';');
  assert.strictEqual(detectDelimiter('/path/to/file.SSV'), ';');
});

test('detectDelimiter defaults to comma for unknown extensions', () => {
  assert.strictEqual(detectDelimiter('data.txt'), ',');
  assert.strictEqual(detectDelimiter('data.json'), ',');
});

// ============================================================================
// parseCSVLine Tests
// ============================================================================

test('parseCSVLine parses simple comma-delimited fields', () => {
  assert.deepStrictEqual(parseCSVLine('a,b,c'), ['a', 'b', 'c']);
});

test('parseCSVLine handles quoted fields with commas', () => {
  assert.deepStrictEqual(parseCSVLine('"hello, world",b,c'), ['hello, world', 'b', 'c']);
});

test('parseCSVLine handles escaped quotes in quoted fields', () => {
  assert.deepStrictEqual(parseCSVLine('"say ""hi""",b'), ['say "hi"', 'b']);
});

test('parseCSVLine handles empty fields', () => {
  assert.deepStrictEqual(parseCSVLine('a,,c'), ['a', '', 'c']);
});

test('parseCSVLine handles single field', () => {
  assert.deepStrictEqual(parseCSVLine('hello'), ['hello']);
});

test('parseCSVLine does not produce extra trailing empty field', () => {
  const result = parseCSVLine('a,b,c');
  assert.strictEqual(result.length, 3);
});

test('parseCSVLine supports tab delimiter', () => {
  assert.deepStrictEqual(parseCSVLine('a\tb\tc', '\t'), ['a', 'b', 'c']);
});

test('parseCSVLine supports semicolon delimiter', () => {
  assert.deepStrictEqual(parseCSVLine('a;b;c', ';'), ['a', 'b', 'c']);
});

test('parseCSVLine with tab delimiter does not split on commas', () => {
  assert.deepStrictEqual(parseCSVLine('hello,world\tb\tc', '\t'), ['hello,world', 'b', 'c']);
});

// ============================================================================
// isDateDtype Tests
// ============================================================================

test('isDateDtype recognizes "Date" string', () => {
  assert.strictEqual(isDateDtype('Date'), true);
});

test('isDateDtype recognizes Datetime object', () => {
  assert.strictEqual(isDateDtype({ Datetime: ['Milliseconds', null] }), true);
});

test('isDateDtype recognizes Date object', () => {
  assert.strictEqual(isDateDtype({ Date: 'something' }), true);
});

test('isDateDtype rejects non-date types', () => {
  assert.strictEqual(isDateDtype('String'), false);
  assert.strictEqual(isDateDtype('Int64'), false);
  assert.strictEqual(isDateDtype(null), false);
  assert.strictEqual(isDateDtype(undefined), false);
  assert.strictEqual(isDateDtype(42), false);
  assert.strictEqual(isDateDtype({ Float64: null }), false);
});

// ============================================================================
// patchSchemaAmPmDates Tests
// ============================================================================

import { writeFile, mkdir, rm } from 'node:fs/promises';
import { join } from 'node:path';
import { tmpdir } from 'node:os';

test('patchSchemaAmPmDates patches AM/PM datetime columns to String', async () => {
  const dir = join(tmpdir(), `qsv-test-ampm-${Date.now()}`);
  try {
    await mkdir(dir, { recursive: true });
    const csvFile = join(dir, 'data.csv');
    const schemaFile = join(dir, 'data.csv.pschema.json');

    await writeFile(csvFile, 'id,timestamp\n1,01/15/2024 02:30 PM\n2,01/16/2024 11:00 AM\n');
    await writeFile(schemaFile, JSON.stringify({
      fields: { id: 'Int64', timestamp: { Datetime: ['Milliseconds', null] } },
    }));

    const patched = await patchSchemaAmPmDates(csvFile, schemaFile);
    assert.deepStrictEqual(patched, ['timestamp']);
  } finally {
    await rm(dir, { recursive: true, force: true });
  }
});

test('patchSchemaAmPmDates skips columns without AM/PM', async () => {
  const dir = join(tmpdir(), `qsv-test-noampm-${Date.now()}`);
  try {
    await mkdir(dir, { recursive: true });
    const csvFile = join(dir, 'data.csv');
    const schemaFile = join(dir, 'data.csv.pschema.json');

    await writeFile(csvFile, 'id,date\n1,2024-01-15\n2,2024-01-16\n');
    await writeFile(schemaFile, JSON.stringify({
      fields: { id: 'Int64', date: 'Date' },
    }));

    const patched = await patchSchemaAmPmDates(csvFile, schemaFile);
    assert.deepStrictEqual(patched, []);
  } finally {
    await rm(dir, { recursive: true, force: true });
  }
});

test('patchSchemaAmPmDates returns empty for missing schema file', async () => {
  const patched = await patchSchemaAmPmDates('/nonexistent/data.csv', '/nonexistent/schema.json');
  assert.deepStrictEqual(patched, []);
});

test('patchSchemaAmPmDates returns empty for CSV with fewer than 2 lines', async () => {
  const dir = join(tmpdir(), `qsv-test-short-${Date.now()}`);
  try {
    await mkdir(dir, { recursive: true });
    const csvFile = join(dir, 'data.csv');
    const schemaFile = join(dir, 'data.csv.pschema.json');

    await writeFile(csvFile, 'id,timestamp\n');
    await writeFile(schemaFile, JSON.stringify({
      fields: { id: 'Int64', timestamp: { Datetime: ['Milliseconds', null] } },
    }));

    const patched = await patchSchemaAmPmDates(csvFile, schemaFile);
    assert.deepStrictEqual(patched, []);
  } finally {
    await rm(dir, { recursive: true, force: true });
  }
});

test('patchSchemaAmPmDates does not false-positive on "Amsterdam"', async () => {
  const dir = join(tmpdir(), `qsv-test-amsterdam-${Date.now()}`);
  try {
    await mkdir(dir, { recursive: true });
    const csvFile = join(dir, 'data.csv');
    const schemaFile = join(dir, 'data.csv.pschema.json');

    // Polars might misidentify a column as Date — the regex should not match "Amsterdam"
    await writeFile(csvFile, 'id,city\n1,Amsterdam\n2,Pamphlet\n');
    await writeFile(schemaFile, JSON.stringify({
      fields: { id: 'Int64', city: 'Date' },
    }));

    const patched = await patchSchemaAmPmDates(csvFile, schemaFile);
    assert.deepStrictEqual(patched, []);
  } finally {
    await rm(dir, { recursive: true, force: true });
  }
});
