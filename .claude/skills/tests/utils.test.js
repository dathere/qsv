/**
 * Unit tests for utility functions
 */
import { test } from 'node:test';
import assert from 'node:assert';
import { formatBytes } from '../src/utils.js';
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
//# sourceMappingURL=utils.test.js.map