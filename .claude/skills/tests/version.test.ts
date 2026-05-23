/**
 * Unit tests for version management
 */

import { test } from 'node:test';
import assert from 'node:assert';
import { writeFileSync, mkdirSync, rmSync, existsSync } from 'node:fs';
import { join } from 'node:path';
import { tmpdir } from 'node:os';
import { VERSION, resolveProjectRoot, readVersionFromJson, readMinimumQsvVersionFromManifest } from '../src/version.js';

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

// --- MINIMUM_QSV_VERSION single-source-of-truth ---

test('readMinimumQsvVersionFromManifest returns a semver string from real manifest',
  { skip: !MANIFEST_AVAILABLE }, () => {
    const root = resolveProjectRoot();
    const minVersion = readMinimumQsvVersionFromManifest(root);
    // Explicit narrow-via-throw rather than `minVersion!` — keeps the
    // assertion honest under strict-null-checks and satisfies Biome's
    // no-non-null-assertion rule. assert.ok's `asserts value` signature
    // doesn't always propagate through the default `import assert from
    // 'node:assert'` style used in this file.
    if (!minVersion) {
      throw new Error('manifest.json must declare _meta.com.dathere.qsv.minimum_qsv_version');
    }
    assert.match(minVersion, /^\d+\.\d+\.\d+$/,
      `minimum_qsv_version must be semver, got: ${minVersion}`);
  });

test('readMinimumQsvVersionFromManifest returns null for non-existent project root', () => {
  assert.strictEqual(readMinimumQsvVersionFromManifest('/nonexistent/path'), null);
});

test('readMinimumQsvVersionFromManifest returns null when manifest lacks the field', () => {
  const dir = join(tmpdir(), `qsv-min-version-test-${Date.now()}`);
  mkdirSync(dir, { recursive: true });
  try {
    writeFileSync(join(dir, 'manifest.json'), JSON.stringify({ version: '1.0.0' }));
    assert.strictEqual(readMinimumQsvVersionFromManifest(dir), null);
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

test('readMinimumQsvVersionFromManifest returns the nested string', () => {
  const dir = join(tmpdir(), `qsv-min-version-test-${Date.now()}`);
  mkdirSync(dir, { recursive: true });
  try {
    writeFileSync(join(dir, 'manifest.json'), JSON.stringify({
      _meta: { 'com.dathere.qsv': { minimum_qsv_version: '99.0.0' } },
    }));
    assert.strictEqual(readMinimumQsvVersionFromManifest(dir), '99.0.0');
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

test('readMinimumQsvVersionFromManifest accepts semver with pre-release and build metadata', () => {
  const dir = join(tmpdir(), `qsv-min-version-test-${Date.now()}-pre`);
  mkdirSync(dir, { recursive: true });
  try {
    for (const v of ['20.1.0-alpha.1', '20.1.0+build.42', '20.1.0-rc.1+sha.abc123']) {
      writeFileSync(join(dir, 'manifest.json'), JSON.stringify({
        _meta: { 'com.dathere.qsv': { minimum_qsv_version: v } },
      }));
      assert.strictEqual(readMinimumQsvVersionFromManifest(dir), v,
        `expected ${v} to be accepted as valid semver`);
    }
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

test('readMinimumQsvVersionFromManifest rejects non-semver strings (returns null)', () => {
  const dir = join(tmpdir(), `qsv-min-version-test-${Date.now()}-bad`);
  mkdirSync(dir, { recursive: true });
  try {
    // Each of these would silently degrade compareVersions() if accepted:
    //   "v20.1.0" → ["v20", "1", "0"] → [NaN, 1, 0] → coerced to [0, 1, 0]
    //   "20.1"    → 2 parts only → missing patch implicitly 0 → "20.1.0" (accidentally fine, but ambiguous)
    //   "20"      → 1 part → wildly wrong floor
    //   empty/whitespace/garbage → guaranteed bypass
    for (const bad of ['v20.1.0', '20.1', '20', '', '   ', 'not-a-version', '20.1.0-', '1.2.3.4', '20.1.0 ']) {
      writeFileSync(join(dir, 'manifest.json'), JSON.stringify({
        _meta: { 'com.dathere.qsv': { minimum_qsv_version: bad } },
      }));
      assert.strictEqual(readMinimumQsvVersionFromManifest(dir), null,
        `expected ${JSON.stringify(bad)} to be rejected`);
    }
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

test('config.MINIMUM_QSV_VERSION matches manifest.json',
  { skip: !MANIFEST_AVAILABLE }, async () => {
    // Dynamic import so config.ts (which calls loadMinimumQsvVersion at module load)
    // resolves against the real project root only when this test actually runs.
    const { MINIMUM_QSV_VERSION } = await import('../src/config.js');
    const root = resolveProjectRoot();
    const manifestMinVersion = readMinimumQsvVersionFromManifest(root);
    assert.strictEqual(MINIMUM_QSV_VERSION, manifestMinVersion,
      `config.MINIMUM_QSV_VERSION (${MINIMUM_QSV_VERSION}) must equal ` +
      `manifest.json _meta.com.dathere.qsv.minimum_qsv_version (${manifestMinVersion})`);
  });
