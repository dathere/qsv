/**
 * Unit tests for configuration module
 */

import { test } from 'node:test';
import assert from 'node:assert';
import { config } from '../src/config.js';

test('config has all required properties', () => {
  assert.ok(typeof config.operationTimeoutMs === 'number');
  assert.ok(typeof config.maxFilesPerListing === 'number');
  assert.ok(typeof config.maxPipelineSteps === 'number');
  assert.ok(typeof config.maxConcurrentOperations === 'number');
});

test('config has reasonable defaults', () => {
  assert.ok(config.operationTimeoutMs >= 1000 && config.operationTimeoutMs <= 30 * 60 * 1000);
  assert.ok(config.maxFilesPerListing >= 1 && config.maxFilesPerListing <= 100000);
  assert.ok(config.maxPipelineSteps >= 1 && config.maxPipelineSteps <= 1000);
  assert.ok(config.maxConcurrentOperations >= 1 && config.maxConcurrentOperations <= 100);
});

test('config defaults match expected values', () => {
  // Test with clean environment (save original)
  const originalEnv = process.env.QSV_MCP_OPERATION_TIMEOUT_MS;
  delete process.env.QSV_MCP_OPERATION_TIMEOUT_MS;
  
  // Reload config by re-importing (in real scenario, config is loaded once)
  // For this test, we'll just verify the defaults are set
  assert.strictEqual(config.operationTimeoutMs, 2 * 60 * 1000); // 2 minutes
  assert.strictEqual(config.maxFilesPerListing, 1000);
  assert.strictEqual(config.maxPipelineSteps, 50);
  assert.strictEqual(config.maxConcurrentOperations, 10);
  
  // Restore
  if (originalEnv) {
    process.env.QSV_MCP_OPERATION_TIMEOUT_MS = originalEnv;
  }
});
