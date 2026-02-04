/**
 * Integration tests that exercise actual qsv binary commands
 * These tests require qsv to be installed and available in PATH
 */

import { test } from 'node:test';
import assert from 'node:assert';
import { writeFile, readFile, unlink, mkdir, rm, stat, access } from 'fs/promises';
import { join } from 'path';
import { tmpdir } from 'os';
import { handleToolCall, handleToParquetCall } from '../src/mcp-tools.js';
import { SkillLoader } from '../src/loader.js';
import { SkillExecutor } from '../src/executor.js';
import { FilesystemResourceProvider } from '../src/mcp-filesystem.js';
import { config } from '../src/config.js';

// Skip tests if qsv is not available
const QSV_AVAILABLE = config.qsvValidation.valid;

/**
 * Create a temporary test directory
 */
async function createTestDir(): Promise<string> {
  const testDir = join(tmpdir(), `qsv-integration-test-${Date.now()}`);
  await mkdir(testDir, { recursive: true });
  return testDir;
}

/**
 * Create a test CSV file
 */
async function createTestCSV(dir: string, filename: string, content: string): Promise<string> {
  const filepath = join(dir, filename);
  await writeFile(filepath, content, 'utf8');
  return filepath;
}

/**
 * Clean up test directory
 */
async function cleanupTestDir(dir: string): Promise<void> {
  try {
    await rm(dir, { recursive: true, force: true });
  } catch (error) {
    // Ignore cleanup errors
  }
}

test('qsv_count returns row count', { skip: !QSV_AVAILABLE }, async () => {
  const testDir = await createTestDir();
  const loader = new SkillLoader();
  const executor = new SkillExecutor();

  try {
    await loader.loadAll();

    // Create test CSV with 3 data rows
    const csvPath = await createTestCSV(
      testDir,
      'test.csv',
      'id,name,age\n1,Alice,30\n2,Bob,25\n3,Charlie,35\n'
    );

    const result = await handleToolCall(
      'qsv_count',
      { input_file: csvPath },
      executor,
      loader,
    );

    assert.ok(!result.isError, 'Command should succeed');
    assert.ok(result.content[0].text?.includes('3'), 'Should count 3 rows');
  } finally {
    await cleanupTestDir(testDir);
  }
});

test('qsv_headers lists column names', { skip: !QSV_AVAILABLE }, async () => {
  const testDir = await createTestDir();
  const loader = new SkillLoader();
  const executor = new SkillExecutor();

  try {
    await loader.loadAll();

    const csvPath = await createTestCSV(
      testDir,
      'test.csv',
      'id,name,age,city\n1,Alice,30,NYC\n2,Bob,25,LA\n'
    );

    const result = await handleToolCall(
      'qsv_headers',
      { input_file: csvPath },
      executor,
      loader,
    );

    assert.ok(!result.isError, 'Command should succeed');
    const output = result.content[0].text || '';
    assert.ok(output.includes('id'), 'Should list id column');
    assert.ok(output.includes('name'), 'Should list name column');
    assert.ok(output.includes('age'), 'Should list age column');
    assert.ok(output.includes('city'), 'Should list city column');
  } finally {
    await cleanupTestDir(testDir);
  }
});

test('qsv_select extracts specific columns', { skip: !QSV_AVAILABLE }, async () => {
  const testDir = await createTestDir();
  const loader = new SkillLoader();
  const executor = new SkillExecutor();

  try {
    await loader.loadAll();

    const csvPath = await createTestCSV(
      testDir,
      'test.csv',
      'id,name,age,city\n1,Alice,30,NYC\n2,Bob,25,LA\n'
    );

    const result = await handleToolCall(
      'qsv_select',
      {
        input_file: csvPath,
        selection: 'name,age',
      },
      executor,
      loader,
    );

    assert.ok(!result.isError, 'Command should succeed');
    const output = result.content[0].text || '';
    assert.ok(output.includes('name'), 'Should include name column');
    assert.ok(output.includes('age'), 'Should include age column');
    assert.ok(output.includes('Alice'), 'Should include Alice');
    assert.ok(output.includes('Bob'), 'Should include Bob');
    assert.ok(!output.includes('city'), 'Should not include city column');
  } finally {
    await cleanupTestDir(testDir);
  }
});

test('qsv_search filters rows by pattern', { skip: !QSV_AVAILABLE }, async () => {
  const testDir = await createTestDir();
  const loader = new SkillLoader();
  const executor = new SkillExecutor();

  try {
    await loader.loadAll();

    const csvPath = await createTestCSV(
      testDir,
      'test.csv',
      'id,name,city\n1,Alice,NYC\n2,Bob,LA\n3,Charlie,NYC\n'
    );

    const result = await handleToolCall(
      'qsv_search',
      {
        input_file: csvPath,
        regex: 'NYC',
      },
      executor,
      loader,
    );

    assert.ok(!result.isError, 'Command should succeed');
    const output = result.content[0].text || '';
    assert.ok(output.includes('Alice'), 'Should include Alice (NYC)');
    assert.ok(output.includes('Charlie'), 'Should include Charlie (NYC)');
    assert.ok(!output.includes('Bob'), 'Should not include Bob (LA)');
  } finally {
    await cleanupTestDir(testDir);
  }
});

test('qsv_stats calculates statistics', { skip: !QSV_AVAILABLE }, async () => {
  const testDir = await createTestDir();
  const loader = new SkillLoader();
  const executor = new SkillExecutor();

  try {
    await loader.loadAll();

    const csvPath = await createTestCSV(
      testDir,
      'test.csv',
      'id,age\n1,30\n2,25\n3,35\n4,40\n'
    );

    const result = await handleToolCall(
      'qsv_stats',
      { input_file: csvPath },
      executor,
      loader,
    );

    assert.ok(!result.isError, 'Command should succeed');
    const output = result.content[0].text || '';
    assert.ok(output.includes('age'), 'Should include age column stats');
    assert.ok(output.includes('mean') || output.includes('avg'), 'Should include mean/avg');
  } finally {
    await cleanupTestDir(testDir);
  }
});

test('qsv_sort sorts rows by column', { skip: !QSV_AVAILABLE }, async () => {
  const testDir = await createTestDir();
  const loader = new SkillLoader();
  const executor = new SkillExecutor();

  try {
    await loader.loadAll();

    const csvPath = await createTestCSV(
      testDir,
      'test.csv',
      'id,age\n1,40\n2,25\n3,35\n4,30\n'
    );

    const result = await handleToolCall(
      'qsv_sort',
      {
        input_file: csvPath,
        select: 'age',
      },
      executor,
      loader,
    );

    assert.ok(!result.isError, 'Command should succeed');
    const output = result.content[0].text || '';
    const lines = output.split('\n').filter(l => l.trim());
    // Check that ages appear in ascending order (25, 30, 35, 40)
    const ageIndex = lines.findIndex(l => l.includes('25'));
    const age40Index = lines.findIndex(l => l.includes('40'));
    assert.ok(ageIndex >= 0 && age40Index > ageIndex, 'Should be sorted by age');
  } finally {
    await cleanupTestDir(testDir);
  }
});

test('qsv_frequency shows value distribution', { skip: !QSV_AVAILABLE }, async () => {
  const testDir = await createTestDir();
  const loader = new SkillLoader();
  const executor = new SkillExecutor();

  try {
    await loader.loadAll();

    const csvPath = await createTestCSV(
      testDir,
      'test.csv',
      'id,city\n1,NYC\n2,LA\n3,NYC\n4,NYC\n5,LA\n'
    );

    const result = await handleToolCall(
      'qsv_frequency',
      {
        input_file: csvPath,
        select: 'city',
      },
      executor,
      loader,
    );

    assert.ok(!result.isError, 'Command should succeed');
    const output = result.content[0].text || '';
    assert.ok(output.includes('NYC'), 'Should include NYC');
    assert.ok(output.includes('LA'), 'Should include LA');
    assert.ok(output.includes('3') || output.includes('count'), 'Should show counts');
  } finally {
    await cleanupTestDir(testDir);
  }
});

test('qsv command with invalid file returns error', { skip: !QSV_AVAILABLE }, async () => {
  const loader = new SkillLoader();
  const executor = new SkillExecutor();
  await loader.loadAll();

  const result = await handleToolCall(
    'qsv_count',
    { input_file: '/nonexistent/file.csv' },
    executor,
    loader,
  );

  assert.strictEqual(result.isError, true, 'Should return error for nonexistent file');
  const errorText = result.content[0].text?.toLowerCase() || '';
  assert.ok(
    errorText.includes('not found') ||
    errorText.includes('no such file') ||
    errorText.includes('cannot find the path'),
    'Error message should mention file not found'
  );
});

test('filesystem provider getFileMetadata returns CSV info', { skip: !QSV_AVAILABLE }, async () => {
  const testDir = await createTestDir();

  try {
    const csvPath = await createTestCSV(
      testDir,
      'test.csv',
      'id,name,age\n1,Alice,30\n2,Bob,25\n3,Charlie,35\n'
    );

    const provider = new FilesystemResourceProvider({
      workingDirectory: testDir,
      allowedDirectories: [testDir],
    });

    const metadata = await provider.getFileMetadata(csvPath);

    assert.ok(metadata !== null, 'Should return metadata');
    assert.strictEqual(metadata?.rowCount, 3, 'Should count 3 rows');
    assert.strictEqual(metadata?.columnCount, 3, 'Should count 3 columns');
    assert.deepStrictEqual(
      metadata?.columnNames,
      ['id', 'name', 'age'],
      'Should list column names'
    );
  } finally {
    await cleanupTestDir(testDir);
  }
});

test('filesystem provider caches metadata', { skip: !QSV_AVAILABLE }, async () => {
  const testDir = await createTestDir();

  try {
    const csvPath = await createTestCSV(
      testDir,
      'test.csv',
      'id,name\n1,Alice\n2,Bob\n'
    );

    const provider = new FilesystemResourceProvider({
      workingDirectory: testDir,
      allowedDirectories: [testDir],
    });

    // First call - should execute qsv
    const metadata1 = await provider.getFileMetadata(csvPath);
    assert.ok(metadata1 !== null, 'First call should return metadata');

    // Second call - should use cache
    const metadata2 = await provider.getFileMetadata(csvPath);
    assert.ok(metadata2 !== null, 'Second call should return cached metadata');

    assert.strictEqual(metadata1?.rowCount, metadata2?.rowCount, 'Cached metadata should match');
  } finally {
    await cleanupTestDir(testDir);
  }
});

test('filesystem provider deduplicates concurrent metadata requests', { skip: !QSV_AVAILABLE }, async () => {
  const testDir = await createTestDir();

  try {
    const csvPath = await createTestCSV(
      testDir,
      'test.csv',
      'id,name,age\n1,Alice,30\n2,Bob,25\n3,Charlie,35\n'
    );

    const provider = new FilesystemResourceProvider({
      workingDirectory: testDir,
      allowedDirectories: [testDir],
    });

    // Make 5 concurrent requests for the same file
    // Only one qsv call should be made, others should wait for the same promise
    const promises = Array.from({ length: 5 }, () => provider.getFileMetadata(csvPath));

    const results = await Promise.all(promises);

    // All results should be non-null
    results.forEach((result, index) => {
      assert.ok(result !== null, `Request ${index + 1} should return metadata`);
    });

    // All results should be identical (same reference or same values)
    const firstResult = results[0];
    results.forEach((result, index) => {
      assert.strictEqual(result?.rowCount, firstResult?.rowCount, `Request ${index + 1} should have same row count`);
      assert.strictEqual(result?.columnCount, firstResult?.columnCount, `Request ${index + 1} should have same column count`);
      assert.deepStrictEqual(result?.columnNames, firstResult?.columnNames, `Request ${index + 1} should have same column names`);
    });

    // Verify the correct metadata was returned
    assert.strictEqual(firstResult?.rowCount, 3, 'Should count 3 rows');
    assert.strictEqual(firstResult?.columnCount, 3, 'Should count 3 columns');
    assert.deepStrictEqual(firstResult?.columnNames, ['id', 'name', 'age'], 'Should list column names');
  } finally {
    await cleanupTestDir(testDir);
  }
});

// ============================================================================
// qsv_to_parquet Sniff-Based Date Detection Integration Tests
// ============================================================================

test('qsv_to_parquet converts CSV with date columns and uses --infer-dates', { skip: !QSV_AVAILABLE }, async () => {
  const testDir = await createTestDir();

  try {
    // Create a CSV with DateTime columns
    const csvPath = await createTestCSV(
      testDir,
      'dates.csv',
      'id,name,created_date,updated_date,amount\n' +
      '1,Alice,2024-01-15 10:30:00,2024-02-01 14:00:00,100.50\n' +
      '2,Bob,2024-03-20 09:15:00,2024-04-10 16:45:00,200.75\n' +
      '3,Charlie,2024-05-05 08:00:00,2024-06-15 12:30:00,300.25\n'
    );

    const parquetPath = join(testDir, 'dates.parquet');
    const result = await handleToParquetCall({
      input_file: csvPath,
      output_file: parquetPath,
    });

    assert.ok(!result.isError, `Conversion should succeed: ${result.content[0].text}`);
    const output = result.content[0].text || '';
    assert.ok(output.includes('Successfully converted'), 'Should report success');
    assert.ok(output.includes(parquetPath), 'Should mention output file');

    // Verify the parquet file was created
    const parquetStat = await stat(parquetPath);
    assert.ok(parquetStat.size > 0, 'Parquet file should be non-empty');

    // Verify stats cache was generated
    // qsv stats replaces .csv with .stats.csv (e.g., dates.csv -> dates.stats.csv)
    const statsPath = csvPath.replace(/\.csv$/, '.stats.csv');
    const statsStat = await stat(statsPath);
    assert.ok(statsStat.size > 0, 'Stats cache should be generated');

    // Verify Polars schema was generated (appends .pschema.json to full filename)
    const schemaPath = csvPath + '.pschema.json';
    const schemaStat = await stat(schemaPath);
    assert.ok(schemaStat.size > 0, 'Polars schema should be generated');

    // Read the stats cache and verify date columns were detected as DateTime
    // (--infer-dates with --dates-whitelist should cause stats to type them as DateTime)
    const statsContent = await readFile(statsPath, 'utf8');
    assert.ok(
      statsContent.includes('created_date,DateTime'),
      'Stats should detect created_date as DateTime type'
    );
    assert.ok(
      statsContent.includes('updated_date,DateTime'),
      'Stats should detect updated_date as DateTime type'
    );
  } finally {
    await cleanupTestDir(testDir);
  }
});

test('qsv_to_parquet converts CSV without date columns (no --infer-dates)', { skip: !QSV_AVAILABLE }, async () => {
  const testDir = await createTestDir();

  try {
    // Create a CSV with no date columns
    const csvPath = await createTestCSV(
      testDir,
      'nodates.csv',
      'id,name,age,score\n' +
      '1,Alice,30,95.5\n' +
      '2,Bob,25,87.3\n' +
      '3,Charlie,35,92.1\n'
    );

    const parquetPath = join(testDir, 'nodates.parquet');
    const result = await handleToParquetCall({
      input_file: csvPath,
      output_file: parquetPath,
    });

    assert.ok(!result.isError, `Conversion should succeed: ${result.content[0].text}`);
    const output = result.content[0].text || '';
    assert.ok(output.includes('Successfully converted'), 'Should report success');

    // Verify the parquet file was created
    const parquetStat = await stat(parquetPath);
    assert.ok(parquetStat.size > 0, 'Parquet file should be non-empty');

    // Verify stats cache and schema were generated
    const statsPath = csvPath.replace(/\.csv$/, '.stats.csv');
    const statsStat = await stat(statsPath);
    assert.ok(statsStat.size > 0, 'Stats cache should be generated');

    const schemaPath = csvPath + '.pschema.json';
    const schemaStat = await stat(schemaPath);
    assert.ok(schemaStat.size > 0, 'Polars schema should be generated');
  } finally {
    await cleanupTestDir(testDir);
  }
});

test('qsv_to_parquet defaults output path from input extension', { skip: !QSV_AVAILABLE }, async () => {
  const testDir = await createTestDir();

  try {
    const csvPath = await createTestCSV(
      testDir,
      'autoname.csv',
      'id,value\n1,100\n2,200\n'
    );

    const result = await handleToParquetCall({
      input_file: csvPath,
    });

    assert.ok(!result.isError, `Conversion should succeed: ${result.content[0].text}`);
    const output = result.content[0].text || '';

    // Should auto-generate output path as autoname.parquet
    const expectedParquet = join(testDir, 'autoname.parquet');
    assert.ok(output.includes('autoname.parquet'), 'Should use auto-generated parquet filename');

    const parquetStat = await stat(expectedParquet);
    assert.ok(parquetStat.size > 0, 'Auto-named parquet file should be created');
  } finally {
    await cleanupTestDir(testDir);
  }
});

test('qsv_to_parquet skips regeneration when stats and schema are up-to-date', { skip: !QSV_AVAILABLE }, async () => {
  const testDir = await createTestDir();

  try {
    const csvPath = await createTestCSV(
      testDir,
      'cached.csv',
      'id,name,date\n1,Alice,2024-01-15\n2,Bob,2024-03-20\n'
    );

    const parquetPath = join(testDir, 'cached.parquet');

    // First conversion - should generate stats and schema
    const result1 = await handleToParquetCall({
      input_file: csvPath,
      output_file: parquetPath,
    });
    assert.ok(!result1.isError, 'First conversion should succeed');
    const output1 = result1.content[0].text || '';
    assert.ok(output1.includes('Stats cache: generated'), 'First run should generate stats');
    assert.ok(output1.includes('Polars schema: generated'), 'First run should generate schema');

    // Second conversion - should reuse existing stats and schema
    const result2 = await handleToParquetCall({
      input_file: csvPath,
      output_file: parquetPath,
    });
    assert.ok(!result2.isError, 'Second conversion should succeed');
    const output2 = result2.content[0].text || '';
    assert.ok(output2.includes('reused (up-to-date)'), 'Second run should reuse cached files');
  } finally {
    await cleanupTestDir(testDir);
  }
});

if (!QSV_AVAILABLE) {
  console.log('\n⚠️  qsv integration tests skipped - qsv binary not available or version too old');
  console.log(`   Current validation: ${JSON.stringify(config.qsvValidation, null, 2)}`);
}
