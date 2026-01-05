/**
 * Unit tests for version management
 */

import { test } from 'node:test';
import assert from 'node:assert';
import { VERSION } from '../src/version.js';

test('VERSION is a valid semver string', () => {
  assert.ok(typeof VERSION === 'string');
  // Basic semver format check: major.minor.patch
  assert.ok(/^\d+\.\d+\.\d+/.test(VERSION));
});

test('VERSION is not empty', () => {
  assert.ok(VERSION.length > 0);
  assert.notStrictEqual(VERSION, '0.0.0'); // Should not be fallback value
});
