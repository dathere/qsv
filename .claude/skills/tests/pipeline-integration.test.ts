/**
 * Pipeline execution integration tests
 *
 * Tests actual multi-step pipeline execution (beyond the existing validation-only tests).
 * Requires qsv binary to be available.
 */

import { test } from 'node:test';
import assert from 'node:assert';
import { writeFile, mkdir, rm, readFile } from 'fs/promises';
import { join } from 'path';
import { tmpdir } from 'os';
import { executePipeline } from '../src/mcp-pipeline.js';
import { SkillLoader } from '../src/loader.js';
import { FilesystemResourceProvider } from '../src/mcp-filesystem.js';
import { config } from '../src/config.js';

// Skip tests if qsv is not available
const QSV_AVAILABLE = config.qsvValidation.valid;

/**
 * Create a temporary test directory
 */
async function createTestDir(): Promise<string> {
  const testDir = join(tmpdir(), `qsv-pipeline-test-${Date.now()}`);
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
  } catch {
    // Ignore cleanup errors
  }
}

// ============================================================================
// Multi-step Pipeline Execution Tests
// ============================================================================

test('pipeline executes select step', { skip: !QSV_AVAILABLE }, async () => {
  const testDir = await createTestDir();
  const loader = new SkillLoader();
  await loader.loadAll();

  const filesystemProvider = new FilesystemResourceProvider({
    workingDirectory: testDir,
    allowedDirectories: [testDir],
  });

  try {
    const csvPath = await createTestCSV(
      testDir,
      'data.csv',
      'id,name,age,city\n1,Alice,30,NYC\n2,Bob,25,LA\n3,Charlie,35,Chicago\n'
    );

    const result = await executePipeline(
      {
        input_file: csvPath,
        steps: [
          { command: 'select', params: { selection: 'name,age' } },
        ],
      },
      loader,
      filesystemProvider,
    );

    assert.ok(!result.isError, `Pipeline should succeed: ${result.content[0].text}`);
    const output = result.content[0].text || '';
    assert.ok(output.includes('name'), 'Output should include name column');
    assert.ok(output.includes('age'), 'Output should include age column');
    assert.ok(output.includes('Alice'), 'Output should include Alice');
    assert.ok(!output.includes('city'), 'Output should not include city column');
  } finally {
    await cleanupTestDir(testDir);
  }
});

test('pipeline executes two-step workflow: select then sort', { skip: !QSV_AVAILABLE }, async () => {
  const testDir = await createTestDir();
  const loader = new SkillLoader();
  await loader.loadAll();

  const filesystemProvider = new FilesystemResourceProvider({
    workingDirectory: testDir,
    allowedDirectories: [testDir],
  });

  try {
    const csvPath = await createTestCSV(
      testDir,
      'data.csv',
      'id,name,age\n1,Charlie,35\n2,Alice,30\n3,Bob,25\n'
    );

    const result = await executePipeline(
      {
        input_file: csvPath,
        steps: [
          { command: 'select', params: { selection: 'name,age' } },
          { command: 'sort', params: { column: 'name' } },
        ],
      },
      loader,
      filesystemProvider,
    );

    assert.ok(!result.isError, `Pipeline should succeed: ${result.content[0].text}`);
    const output = result.content[0].text || '';
    // After sorting by name, order should be Alice, Bob, Charlie
    const lines = output.split('\n').filter(l => l.trim());
    const aliceIdx = lines.findIndex(l => l.includes('Alice'));
    const bobIdx = lines.findIndex(l => l.includes('Bob'));
    const charlieIdx = lines.findIndex(l => l.includes('Charlie'));
    assert.ok(aliceIdx < bobIdx, 'Alice should come before Bob after sort');
    assert.ok(bobIdx < charlieIdx, 'Bob should come before Charlie after sort');
  } finally {
    await cleanupTestDir(testDir);
  }
});

test('pipeline writes output to file when output_file specified', { skip: !QSV_AVAILABLE }, async () => {
  const testDir = await createTestDir();
  const loader = new SkillLoader();
  await loader.loadAll();

  const filesystemProvider = new FilesystemResourceProvider({
    workingDirectory: testDir,
    allowedDirectories: [testDir],
  });

  try {
    const csvPath = await createTestCSV(
      testDir,
      'data.csv',
      'id,name,age\n1,Alice,30\n2,Bob,25\n3,Charlie,35\n'
    );

    const outputPath = join(testDir, 'output.csv');

    const result = await executePipeline(
      {
        input_file: csvPath,
        steps: [
          { command: 'select', params: { selection: 'name' } },
        ],
        output_file: outputPath,
      },
      loader,
      filesystemProvider,
    );

    assert.ok(!result.isError, `Pipeline should succeed: ${result.content[0].text}`);
    const statusText = result.content[0].text || '';
    assert.ok(statusText.includes('Pipeline executed successfully'), 'Should report success');
    assert.ok(statusText.includes(outputPath), 'Should mention output file');

    // Verify the output file was written
    const outputContent = await readFile(outputPath, 'utf8');
    assert.ok(outputContent.includes('name'), 'Output file should have name column');
    assert.ok(outputContent.includes('Alice'), 'Output file should include Alice');
    assert.ok(!outputContent.includes('age'), 'Output file should not include age');
  } finally {
    await cleanupTestDir(testDir);
  }
});

test('pipeline dedup removes duplicate rows', { skip: !QSV_AVAILABLE }, async () => {
  const testDir = await createTestDir();
  const loader = new SkillLoader();
  await loader.loadAll();

  const filesystemProvider = new FilesystemResourceProvider({
    workingDirectory: testDir,
    allowedDirectories: [testDir],
  });

  try {
    // Use fully duplicate rows (all columns match) so dedup can detect them
    const csvPath = await createTestCSV(
      testDir,
      'data.csv',
      'name,city\nAlice,NYC\nBob,LA\nAlice,NYC\nBob,LA\nCharlie,Chicago\n'
    );

    const result = await executePipeline(
      {
        input_file: csvPath,
        steps: [
          { command: 'dedup', params: {} },
        ],
      },
      loader,
      filesystemProvider,
    );

    assert.ok(!result.isError, `Pipeline should succeed: ${result.content[0].text}`);
    const output = result.content[0].text || '';
    // After dedup, should have 3 unique rows (Alice/NYC, Bob/LA, Charlie/Chicago) + header
    const allLines = output.split('\n').filter(l => l.trim());
    // Header + 3 data rows = 4 lines max
    assert.ok(allLines.length <= 4, `Should have at most 4 lines after dedup (found ${allLines.length})`);
  } finally {
    await cleanupTestDir(testDir);
  }
});

test('pipeline fails gracefully with invalid command', { skip: !QSV_AVAILABLE }, async () => {
  const testDir = await createTestDir();
  const loader = new SkillLoader();
  await loader.loadAll();

  const filesystemProvider = new FilesystemResourceProvider({
    workingDirectory: testDir,
    allowedDirectories: [testDir],
  });

  try {
    const csvPath = await createTestCSV(
      testDir,
      'data.csv',
      'id,name\n1,Alice\n2,Bob\n'
    );

    const result = await executePipeline(
      {
        input_file: csvPath,
        steps: [
          { command: 'nonexistent_command', params: {} },
        ],
      },
      loader,
      filesystemProvider,
    );

    // Should return an error, not throw
    assert.strictEqual(result.isError, true, 'Should return error for invalid command');
  } finally {
    await cleanupTestDir(testDir);
  }
});

test('pipeline fails gracefully with nonexistent input file', { skip: !QSV_AVAILABLE }, async () => {
  const testDir = await createTestDir();
  const loader = new SkillLoader();
  await loader.loadAll();

  const filesystemProvider = new FilesystemResourceProvider({
    workingDirectory: testDir,
    allowedDirectories: [testDir],
  });

  try {
    const result = await executePipeline(
      {
        input_file: join(testDir, 'nonexistent.csv'),
        steps: [
          { command: 'select', params: { selection: 'name' } },
        ],
      },
      loader,
      filesystemProvider,
    );

    assert.strictEqual(result.isError, true, 'Should return error for nonexistent file');
  } finally {
    await cleanupTestDir(testDir);
  }
});

test('pipeline three-step workflow: select, search, slice', { skip: !QSV_AVAILABLE }, async () => {
  const testDir = await createTestDir();
  const loader = new SkillLoader();
  await loader.loadAll();

  const filesystemProvider = new FilesystemResourceProvider({
    workingDirectory: testDir,
    allowedDirectories: [testDir],
  });

  try {
    const csvPath = await createTestCSV(
      testDir,
      'data.csv',
      'id,name,city,age\n1,Alice,NYC,30\n2,Bob,LA,25\n3,Charlie,NYC,35\n4,Diana,NYC,28\n5,Eve,LA,32\n'
    );

    const result = await executePipeline(
      {
        input_file: csvPath,
        steps: [
          { command: 'select', params: { selection: 'name,city,age' } },
          { command: 'search', params: { pattern: 'NYC' } },
          // Note: slice options are typed as strings in the skill JSON definition
          { command: 'slice', params: { len: '2' } },
        ],
      },
      loader,
      filesystemProvider,
    );

    assert.ok(!result.isError, `Pipeline should succeed: ${result.content[0].text}`);
    const output = result.content[0].text || '';
    // Should only contain NYC rows, and at most 2 of them
    assert.ok(!output.includes('LA'), 'Should not include LA rows after search');
    const dataLines = output.split('\n').filter(l => l.trim() && !l.startsWith('name'));
    assert.ok(dataLines.length <= 2, 'Should have at most 2 rows after slice');
  } finally {
    await cleanupTestDir(testDir);
  }
});

if (!QSV_AVAILABLE) {
  console.log('\n⚠️  Pipeline integration tests skipped - qsv binary not available or version too old');
}
