/**
 * Unit tests for utility functions
 */

import { test } from 'node:test';
import assert from 'node:assert';
import { formatBytes, compareVersions, isReservedCachePath } from '../src/utils.js';

test('formatBytes formats bytes correctly', () => {
  assert.strictEqual(formatBytes(0), '0 Bytes');
  assert.strictEqual(formatBytes(1024), '1 KB');
  assert.strictEqual(formatBytes(1024 * 1024), '1 MB');
  assert.strictEqual(formatBytes(1024 * 1024 * 1024), '1 GB');
  assert.strictEqual(formatBytes(1536), '1.5 KB');
  assert.strictEqual(formatBytes(1536 * 1024), '1.5 MB');
});

test('formatBytes handles edge cases', () => {
  assert.strictEqual(formatBytes(-1), '0 Bytes');
  assert.strictEqual(formatBytes(1), '1 Bytes');
  assert.strictEqual(formatBytes(999), '999 Bytes');
});

// ============================================================================
// compareVersions Tests
// ============================================================================

test('compareVersions returns correct order for valid versions', () => {
  assert.strictEqual(compareVersions('1.0.0', '1.0.0'), 0);
  assert.strictEqual(compareVersions('1.0.0', '2.0.0'), -1);
  assert.strictEqual(compareVersions('2.0.0', '1.0.0'), 1);
  assert.strictEqual(compareVersions('1.2.3', '1.2.4'), -1);
  assert.strictEqual(compareVersions('1.3.0', '1.2.99'), 1);
});

test('compareVersions strips pre-release and build metadata', () => {
  assert.strictEqual(compareVersions('1.2.3-alpha.1', '1.2.3'), 0);
  assert.strictEqual(compareVersions('1.2.3+build.123', '1.2.3'), 0);
  assert.strictEqual(compareVersions('1.2.3-alpha.1+build', '1.2.3'), 0);
});

test('compareVersions returns NaN for invalid version strings', () => {
  assert.ok(Number.isNaN(compareVersions('abc', '1.2.3')));
  assert.ok(Number.isNaN(compareVersions('1.2.3', 'xyz')));
  assert.ok(Number.isNaN(compareVersions('not.a.version', 'also.not')));
});

test('compareVersions handles different-length versions', () => {
  assert.strictEqual(compareVersions('1.0', '1.0.0'), 0);
  assert.strictEqual(compareVersions('1.0.0.0', '1.0.0'), 0);
  assert.strictEqual(compareVersions('1.0', '1.0.1'), -1);
});

// ============================================================================
// isReservedCachePath Tests
// ============================================================================

test('isReservedCachePath returns true for each reserved suffix', () => {
  assert.strictEqual(isReservedCachePath('data.stats.csv'), true);
  assert.strictEqual(isReservedCachePath('data.stats.csv.data.jsonl'), true);
  assert.strictEqual(isReservedCachePath('data.stats.bivariate.csv'), true);
  assert.strictEqual(isReservedCachePath('data.stats.bivariate.joined.csv'), true);
  assert.strictEqual(isReservedCachePath('data.freq.csv.data.jsonl'), true);
  assert.strictEqual(isReservedCachePath('data.pschema.json'), true);
});

test('isReservedCachePath returns true with full paths', () => {
  assert.strictEqual(isReservedCachePath('/tmp/output/data.stats.csv'), true);
  assert.strictEqual(isReservedCachePath('/home/user/results.pschema.json'), true);
});

test('isReservedCachePath returns false for normal output paths', () => {
  assert.strictEqual(isReservedCachePath('output.csv'), false);
  assert.strictEqual(isReservedCachePath('results.parquet'), false);
  assert.strictEqual(isReservedCachePath('data.freq.csv'), false);
  assert.strictEqual(isReservedCachePath('schema.json'), false);
  assert.strictEqual(isReservedCachePath('my_stats.csv.bak'), false);
});

test('isReservedCachePath is case-insensitive', () => {
  assert.strictEqual(isReservedCachePath('DATA.STATS.CSV'), true);
  assert.strictEqual(isReservedCachePath('Data.Stats.Csv.Data.Jsonl'), true);
  assert.strictEqual(isReservedCachePath('FILE.PSCHEMA.JSON'), true);
});
