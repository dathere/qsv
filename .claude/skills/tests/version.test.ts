/**
 * Unit tests for version management
 */

import { test } from 'node:test';
import assert from 'node:assert';
import { writeFileSync, mkdirSync, rmSync, existsSync } from 'node:fs';
import { join } from 'node:path';
import { tmpdir } from 'node:os';
import { VERSION, resolveProjectRoot, readVersionFromJson } from '../src/version.js';

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

// --- resolveProjectRoot tests ---

test('resolveProjectRoot returns a path containing package.json', () => {
  const root = resolveProjectRoot();
  assert.ok(existsSync(join(root, 'package.json')),
    `Expected package.json at ${root}`);
});

// --- readVersionFromJson tests ---

test('readVersionFromJson reads version from a valid JSON file', () => {
  const dir = join(tmpdir(), `qsv-version-test-${Date.now()}`);
  mkdirSync(dir, { recursive: true });
  try {
    const filePath = join(dir, 'test.json');
    writeFileSync(filePath, JSON.stringify({ version: '1.2.3' }));
    assert.strictEqual(readVersionFromJson(filePath), '1.2.3');
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

test('readVersionFromJson returns null for non-existent file', () => {
  assert.strictEqual(readVersionFromJson('/nonexistent/path/file.json'), null);
});

test('readVersionFromJson returns null for JSON without version field', () => {
  const dir = join(tmpdir(), `qsv-version-test-${Date.now()}`);
  mkdirSync(dir, { recursive: true });
  try {
    const filePath = join(dir, 'no-version.json');
    writeFileSync(filePath, JSON.stringify({ name: 'test' }));
    assert.strictEqual(readVersionFromJson(filePath), null);
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

test('readVersionFromJson returns null for empty version string', () => {
  const dir = join(tmpdir(), `qsv-version-test-${Date.now()}`);
  mkdirSync(dir, { recursive: true });
  try {
    const filePath = join(dir, 'empty-version.json');
    writeFileSync(filePath, JSON.stringify({ version: '' }));
    assert.strictEqual(readVersionFromJson(filePath), null);
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

test('readVersionFromJson returns null for invalid JSON', () => {
  const dir = join(tmpdir(), `qsv-version-test-${Date.now()}`);
  mkdirSync(dir, { recursive: true });
  try {
    const filePath = join(dir, 'bad.json');
    writeFileSync(filePath, 'not valid json {{{');
    assert.strictEqual(readVersionFromJson(filePath), null);
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

test('readVersionFromJson returns null for non-string version', () => {
  const dir = join(tmpdir(), `qsv-version-test-${Date.now()}`);
  mkdirSync(dir, { recursive: true });
  try {
    const filePath = join(dir, 'numeric-version.json');
    writeFileSync(filePath, JSON.stringify({ version: 123 }));
    assert.strictEqual(readVersionFromJson(filePath), null);
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

// --- VERSION sync validation (integration) ---

test('VERSION matches the version in package.json', () => {
  const root = resolveProjectRoot();
  const packageVersion = readVersionFromJson(join(root, 'package.json'));
  assert.strictEqual(VERSION, packageVersion,
    'VERSION export must equal the version in package.json');
});

// Pre-check manifest availability so the test is reported as "skipped" rather
// than silently passing when manifest.json is absent (common in test layouts).
const MANIFEST_AVAILABLE = readVersionFromJson(
  join(resolveProjectRoot(), 'manifest.json'),
) !== null;

test('package.json and manifest.json versions are in sync', { skip: !MANIFEST_AVAILABLE }, () => {
  const root = resolveProjectRoot();
  const packageVersion = readVersionFromJson(join(root, 'package.json'));
  const manifestVersion = readVersionFromJson(join(root, 'manifest.json'));

  assert.strictEqual(packageVersion, manifestVersion,
    `package.json (${packageVersion}) and manifest.json (${manifestVersion}) versions must match`);
});
