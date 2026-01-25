/**
 * Unit tests for filesystem resource provider
 */

import { test } from 'node:test';
import assert from 'node:assert';
import { FilesystemResourceProvider } from '../src/mcp-filesystem.js';
import { config } from '../src/config.js';
import { mkdtemp, writeFile, rmdir, unlink } from 'fs/promises';
import { join } from 'path';
import { tmpdir } from 'os';

test('listFiles enforces file limit', async () => {
  // Create temporary directory
  const tempDir = await mkdtemp(join(tmpdir(), 'qsv-test-'));
  
  try {
    // Create more files than the limit
    const fileCount = config.maxFilesPerListing + 10;
    for (let i = 0; i < fileCount; i++) {
      await writeFile(join(tempDir, `test${i}.csv`), 'col1,col2\nval1,val2\n');
    }

    const provider = new FilesystemResourceProvider({
      workingDirectory: tempDir,
    });

    // Pass undefined explicitly to use working directory
    const result = await provider.listFiles(undefined);
    
    // Should return exactly the limit, not more
    assert.strictEqual(result.resources.length, config.maxFilesPerListing);
  } finally {
    // Cleanup
    try {
      const { readdir } = await import('fs/promises');
      const files = await readdir(tempDir);
      for (const file of files) {
        await unlink(join(tempDir, file));
      }
      await rmdir(tempDir);
    } catch {
      // Ignore cleanup errors
    }
  }
});

test('resolvePath prevents directory traversal', async () => {
  const tempDir = await mkdtemp(join(tmpdir(), 'qsv-test-'));
  
  try {
    const provider = new FilesystemResourceProvider({
      workingDirectory: tempDir,
    });

    // Should reject paths with ..
    await assert.rejects(
      async () => {
        await provider.resolvePath('../../etc/passwd');
      },
      /Access denied|outside allowed directories|Path does not exist/
    );

    // Should reject absolute paths outside allowed directories
    await assert.rejects(
      async () => {
        await provider.resolvePath('/etc/passwd');
      },
      /Access denied|outside allowed directories|Path does not exist/
    );
  } finally {
    try {
      await rmdir(tempDir);
    } catch {
      // Ignore cleanup errors
    }
  }
});

test('needsConversion detects Excel and JSONL formats', () => {
  const provider = new FilesystemResourceProvider();
  
  assert.strictEqual(provider.needsConversion('file.xlsx'), true);
  assert.strictEqual(provider.needsConversion('file.xls'), true);
  assert.strictEqual(provider.needsConversion('file.jsonl'), true);
  assert.strictEqual(provider.needsConversion('file.ndjson'), true);
  assert.strictEqual(provider.needsConversion('file.csv'), false);
  assert.strictEqual(provider.needsConversion('file.tsv'), false);
});

test('getConversionCommand returns correct command', () => {
  const provider = new FilesystemResourceProvider();

  assert.strictEqual(provider.getConversionCommand('file.xlsx'), 'excel');
  assert.strictEqual(provider.getConversionCommand('file.jsonl'), 'jsonl');
  assert.strictEqual(provider.getConversionCommand('file.csv'), null);
});

test('listFiles excludes converted files with UUID pattern', async () => {
  const tempDir = await mkdtemp(join(tmpdir(), 'qsv-test-'));

  try {
    // Create a normal CSV file
    await writeFile(join(tempDir, 'normal.csv'), 'col1,col2\nval1,val2\n');

    // Create a converted file (UUID pattern from ConvertedFileManager)
    await writeFile(
      join(tempDir, 'data.xlsx.converted.06488439-4c06-4abc-999e-9e6af19b1234.csv'),
      'col1,col2\nval1,val2\n'
    );

    const provider = new FilesystemResourceProvider({
      workingDirectory: tempDir,
    });

    const result = await provider.listFiles(undefined);

    // Should only include the normal CSV, not the converted file
    assert.strictEqual(result.resources.length, 1);
    assert.strictEqual(result.resources[0].name, 'normal.csv');
  } finally {
    try {
      const { readdir } = await import('fs/promises');
      const files = await readdir(tempDir);
      for (const file of files) {
        await unlink(join(tempDir, file));
      }
      await rmdir(tempDir);
    } catch {
      // Ignore cleanup errors
    }
  }
});

test('listFiles excludes qsv-output temporary files', async () => {
  const tempDir = await mkdtemp(join(tmpdir(), 'qsv-test-'));

  try {
    // Create a normal CSV file
    await writeFile(join(tempDir, 'normal.csv'), 'col1,col2\nval1,val2\n');

    // Create a temporary output file
    await writeFile(
      join(tempDir, 'qsv-output-a1b2c3d4-e5f6-7890-abcd-ef1234567890.csv'),
      'col1,col2\nval1,val2\n'
    );

    const provider = new FilesystemResourceProvider({
      workingDirectory: tempDir,
    });

    const result = await provider.listFiles(undefined);

    // Should only include the normal CSV, not the temp output file
    assert.strictEqual(result.resources.length, 1);
    assert.strictEqual(result.resources[0].name, 'normal.csv');
  } finally {
    try {
      const { readdir } = await import('fs/promises');
      const files = await readdir(tempDir);
      for (const file of files) {
        await unlink(join(tempDir, file));
      }
      await rmdir(tempDir);
    } catch {
      // Ignore cleanup errors
    }
  }
});

test('listFiles excludes files with .tmp. pattern', async () => {
  const tempDir = await mkdtemp(join(tmpdir(), 'qsv-test-'));

  try {
    // Create a normal CSV file
    await writeFile(join(tempDir, 'normal.csv'), 'col1,col2\nval1,val2\n');

    // Create a temp file with .tmp. pattern
    await writeFile(join(tempDir, 'data.tmp.csv'), 'col1,col2\nval1,val2\n');

    const provider = new FilesystemResourceProvider({
      workingDirectory: tempDir,
    });

    const result = await provider.listFiles(undefined);

    // Should only include the normal CSV, not the temp file
    assert.strictEqual(result.resources.length, 1);
    assert.strictEqual(result.resources[0].name, 'normal.csv');
  } finally {
    try {
      const { readdir } = await import('fs/promises');
      const files = await readdir(tempDir);
      for (const file of files) {
        await unlink(join(tempDir, file));
      }
      await rmdir(tempDir);
    } catch {
      // Ignore cleanup errors
    }
  }
});

// Note: listWorkingDirFiles was removed as file resources are no longer exposed via MCP.
// The qsv_list_files tool uses listFiles() which is still available.
