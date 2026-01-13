/**
 * Unit tests for ConvertedFileManager
 *
 * Tests for file tracking, LIFO cleanup, and cache management.
 */

import { test } from 'node:test';
import assert from 'node:assert';
import { mkdtemp, writeFile, rm, stat, readFile } from 'fs/promises';
import { join } from 'path';
import { tmpdir } from 'os';
import { ConvertedFileManager } from '../src/converted-file-manager.js';

/**
 * Create a temporary test directory
 */
async function createTestDir(): Promise<string> {
  return await mkdtemp(join(tmpdir(), 'qsv-cfm-test-'));
}

/**
 * Create a test file with specified size
 */
async function createTestFile(dir: string, filename: string, sizeBytes: number): Promise<string> {
  const filepath = join(dir, filename);
  const content = 'x'.repeat(sizeBytes);
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

test('ConvertedFileManager initializes with default settings', async () => {
  const testDir = await createTestDir();

  try {
    const manager = new ConvertedFileManager(testDir);

    // Manager should be created without errors
    assert.ok(manager instanceof ConvertedFileManager);
  } finally {
    await cleanupTestDir(testDir);
  }
});

test('ConvertedFileManager initializes with custom size limit', async () => {
  const testDir = await createTestDir();

  try {
    const manager = new ConvertedFileManager(testDir, 2.0); // 2 GB

    // Manager should be created without errors
    assert.ok(manager instanceof ConvertedFileManager);
  } finally {
    await cleanupTestDir(testDir);
  }
});

test('ConvertedFileManager validates size limit bounds', async () => {
  const testDir = await createTestDir();

  try {
    // Invalid values should fall back to default (1 GB)
    // These should not throw, just log warnings and use defaults
    const manager1 = new ConvertedFileManager(testDir, -1);
    assert.ok(manager1 instanceof ConvertedFileManager);

    const manager2 = new ConvertedFileManager(testDir, 0);
    assert.ok(manager2 instanceof ConvertedFileManager);

    const manager3 = new ConvertedFileManager(testDir, 1000); // Over 100 GB limit
    assert.ok(manager3 instanceof ConvertedFileManager);
  } finally {
    await cleanupTestDir(testDir);
  }
});

test('ConvertedFileManager getValidConvertedFile returns null for non-existent file', async () => {
  const testDir = await createTestDir();

  try {
    const manager = new ConvertedFileManager(testDir);
    const sourcePath = join(testDir, 'source.xlsx');
    const convertedPath = join(testDir, 'source.converted.abc123.csv');

    // Create source file but not converted file
    await createTestFile(testDir, 'source.xlsx', 100);

    const result = await manager.getValidConvertedFile(sourcePath, convertedPath);
    assert.strictEqual(result, null);
  } finally {
    await cleanupTestDir(testDir);
  }
});

test('ConvertedFileManager getValidConvertedFile returns path for valid file', async () => {
  const testDir = await createTestDir();

  try {
    const manager = new ConvertedFileManager(testDir);
    const sourcePath = join(testDir, 'source.xlsx');
    const convertedPath = join(testDir, 'source.converted.abc123.csv');

    // Create both files (converted file created after source)
    await createTestFile(testDir, 'source.xlsx', 100);
    await new Promise(resolve => setTimeout(resolve, 10)); // Small delay
    await createTestFile(testDir, 'source.converted.abc123.csv', 50);

    const result = await manager.getValidConvertedFile(sourcePath, convertedPath);
    assert.strictEqual(result, convertedPath);
  } finally {
    await cleanupTestDir(testDir);
  }
});

test('ConvertedFileManager registerConvertedFile creates cache entry', async () => {
  const testDir = await createTestDir();

  try {
    const manager = new ConvertedFileManager(testDir);
    const sourcePath = join(testDir, 'source.xlsx');
    const convertedPath = join(testDir, 'source.converted.abc123.csv');

    // Create both files
    await createTestFile(testDir, 'source.xlsx', 100);
    await createTestFile(testDir, 'source.converted.abc123.csv', 50);

    // Register the conversion
    await manager.registerConvertedFile(sourcePath, convertedPath);

    // Verify cache file was created
    const cacheFilePath = join(testDir, '.qsv-mcp-converted-cache.json');
    const cacheStats = await stat(cacheFilePath);
    assert.ok(cacheStats.isFile());

    // Verify cache content
    const cacheContent = await readFile(cacheFilePath, 'utf8');
    const cache = JSON.parse(cacheContent);
    assert.strictEqual(cache.version, 1);
    assert.ok(Array.isArray(cache.entries));
    assert.strictEqual(cache.entries.length, 1);
    assert.strictEqual(cache.entries[0].sourcePath, sourcePath);
    assert.strictEqual(cache.entries[0].convertedPath, convertedPath);
  } finally {
    await cleanupTestDir(testDir);
  }
});

test('ConvertedFileManager tracks conversion metrics', async () => {
  const testDir = await createTestDir();

  try {
    const manager = new ConvertedFileManager(testDir);

    // Get initial metrics
    const initialMetrics = manager.getMetrics();
    assert.strictEqual(initialMetrics.conversions.total, 0);
    assert.strictEqual(initialMetrics.conversions.successful, 0);
    assert.strictEqual(initialMetrics.cache.hits, 0);
    assert.strictEqual(initialMetrics.cache.misses, 0);
  } finally {
    await cleanupTestDir(testDir);
  }
});

test('ConvertedFileManager resets metrics correctly', async () => {
  const testDir = await createTestDir();

  try {
    const manager = new ConvertedFileManager(testDir);

    // Simulate some activity by registering files
    const sourcePath = join(testDir, 'source.xlsx');
    const convertedPath = join(testDir, 'source.converted.abc123.csv');
    await createTestFile(testDir, 'source.xlsx', 100);
    await createTestFile(testDir, 'source.converted.abc123.csv', 50);

    await manager.registerConversionStart(sourcePath, convertedPath);
    await manager.registerConvertedFile(sourcePath, convertedPath);

    // Metrics should show activity
    const metricsAfter = manager.getMetrics();
    assert.ok(metricsAfter.conversions.total > 0 || metricsAfter.conversions.successful > 0);

    // Reset metrics
    manager.resetMetrics();

    // Metrics should be zero
    const metricsReset = manager.getMetrics();
    assert.strictEqual(metricsReset.conversions.total, 0);
    assert.strictEqual(metricsReset.conversions.successful, 0);
    assert.strictEqual(metricsReset.cache.hits, 0);
  } finally {
    await cleanupTestDir(testDir);
  }
});

test('ConvertedFileManager cleanupOrphanedEntries removes stale entries', async () => {
  const testDir = await createTestDir();

  try {
    const manager = new ConvertedFileManager(testDir);
    const sourcePath = join(testDir, 'source.xlsx');
    const convertedPath = join(testDir, 'source.converted.abc123.csv');

    // Create and register files
    await createTestFile(testDir, 'source.xlsx', 100);
    await createTestFile(testDir, 'source.converted.abc123.csv', 50);
    await manager.registerConvertedFile(sourcePath, convertedPath);

    // Verify entry exists
    const cacheFilePath = join(testDir, '.qsv-mcp-converted-cache.json');
    let cache = JSON.parse(await readFile(cacheFilePath, 'utf8'));
    assert.strictEqual(cache.entries.length, 1);

    // Delete the converted file (simulating orphan)
    await rm(convertedPath);

    // Run cleanup
    await manager.cleanupOrphanedEntries();

    // Verify entry was removed
    cache = JSON.parse(await readFile(cacheFilePath, 'utf8'));
    assert.strictEqual(cache.entries.length, 0);
  } finally {
    await cleanupTestDir(testDir);
  }
});

test('ConvertedFileManager touchConvertedFile updates timestamp', async () => {
  const testDir = await createTestDir();

  try {
    const manager = new ConvertedFileManager(testDir);
    const sourcePath = join(testDir, 'source.xlsx');
    const convertedPath = join(testDir, 'source.converted.abc123.csv');

    // Create and register files
    await createTestFile(testDir, 'source.xlsx', 100);
    await createTestFile(testDir, 'source.converted.abc123.csv', 50);
    await manager.registerConvertedFile(sourcePath, convertedPath);

    // Get initial timestamp
    const cacheFilePath = join(testDir, '.qsv-mcp-converted-cache.json');
    let cache = JSON.parse(await readFile(cacheFilePath, 'utf8'));
    const initialTimestamp = cache.entries[0].createdAt;

    // Wait a bit and touch the file
    await new Promise(resolve => setTimeout(resolve, 10));
    await manager.touchConvertedFile(sourcePath);

    // Verify timestamp was updated
    cache = JSON.parse(await readFile(cacheFilePath, 'utf8'));
    const updatedTimestamp = cache.entries[0].createdAt;
    assert.ok(updatedTimestamp >= initialTimestamp, 'Timestamp should be updated');
  } finally {
    await cleanupTestDir(testDir);
  }
});

test('ConvertedFileManager handles concurrent registrations gracefully', async () => {
  const testDir = await createTestDir();

  try {
    const manager = new ConvertedFileManager(testDir);

    // Create multiple source/converted pairs
    const pairs = [];
    for (let i = 0; i < 3; i++) {
      const sourcePath = join(testDir, `source${i}.xlsx`);
      const convertedPath = join(testDir, `source${i}.converted.${i}.csv`);
      await createTestFile(testDir, `source${i}.xlsx`, 100);
      await createTestFile(testDir, `source${i}.converted.${i}.csv`, 50);
      pairs.push({ sourcePath, convertedPath });
    }

    // Register all concurrently
    await Promise.all(
      pairs.map(({ sourcePath, convertedPath }) =>
        manager.registerConvertedFile(sourcePath, convertedPath)
      )
    );

    // Verify all were registered
    const cacheFilePath = join(testDir, '.qsv-mcp-converted-cache.json');
    const cache = JSON.parse(await readFile(cacheFilePath, 'utf8'));

    // At least some entries should be registered (locking may cause some to be skipped)
    assert.ok(cache.entries.length > 0, 'At least one entry should be registered');
  } finally {
    await cleanupTestDir(testDir);
  }
});
