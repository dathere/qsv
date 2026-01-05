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

test('VERSION is not empty and has valid format', () => {
  assert.ok(VERSION.length > 0);
  // Validate semver format (accepts 0.0.0 fallback and real versions)
  assert.match(VERSION, /^\d+\.\d+\.\d+$/);
});
