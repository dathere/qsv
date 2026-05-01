/**
 * Unit tests for UpdateChecker
 *
 * Tests for version checking, comparison, and update detection.
 */

import { test } from 'node:test';
import assert from 'node:assert';
import { writeFile, readFile } from 'fs/promises';
import { join } from 'path';
import { UpdateChecker, getUpdateConfigFromEnv } from '../src/update-checker.js';
import { config } from '../src/config.js';
import { QSV_AVAILABLE, createTestDir, cleanupTestDir } from './test-helpers.js';

test('UpdateChecker initializes with default config', () => {
  const checker = new UpdateChecker();
  assert.ok(checker instanceof UpdateChecker);
});

test('UpdateChecker initializes with custom config', () => {
  const checker = new UpdateChecker('qsv', undefined, {
    autoRegenerateSkills: true,
    checkForUpdatesOnStartup: false,
    notifyOnUpdatesAvailable: false,
  });
  assert.ok(checker instanceof UpdateChecker);
});

test('getQsvBinaryVersion returns valid version', { skip: !QSV_AVAILABLE }, async () => {
  const checker = new UpdateChecker(config.qsvBinPath);
  const version = await checker.getQsvBinaryVersion();

  // Should be a semver-like string
  assert.ok(typeof version === 'string');
  assert.match(version, /^\d+\.\d+\.\d+$/);
});

test('getSkillsVersion returns version string', () => {
  const checker = new UpdateChecker();
  const version = checker.getSkillsVersion();

  // Should return a string (may be 'unknown' if skills not found)
  assert.ok(typeof version === 'string');
});

test('getMcpServerVersion returns version string', () => {
  const checker = new UpdateChecker();
  const version = checker.getMcpServerVersion();

  // Should return a version string from package.json
  assert.ok(typeof version === 'string');
  // If found, should be semver-like
  if (version !== 'unknown') {
    assert.match(version, /^\d+\.\d+\.\d+/);
  }
});

test('loadVersionInfo returns null when no file exists', async () => {
  const testDir = await createTestDir();

  try {
    const checker = new UpdateChecker('qsv', join(testDir, 'nonexistent'));
    const info = checker.loadVersionInfo();
    assert.strictEqual(info, null);
  } finally {
    await cleanupTestDir(testDir);
  }
});

test('saveVersionInfo and loadVersionInfo roundtrip', async () => {
  const testDir = await createTestDir();

  try {
    // Create a checker with custom version file path
    const checker = new UpdateChecker('qsv', testDir);

    const testInfo = {
      qsvBinaryVersion: '13.0.0',
      skillsGeneratedWithVersion: '13.0.0',
      mcpServerVersion: '14.0.0',
      lastChecked: new Date().toISOString(),
    };

    checker.saveVersionInfo(testInfo);
    const loaded = checker.loadVersionInfo();

    assert.ok(loaded !== null);
    assert.strictEqual(loaded?.qsvBinaryVersion, testInfo.qsvBinaryVersion);
    assert.strictEqual(loaded?.skillsGeneratedWithVersion, testInfo.skillsGeneratedWithVersion);
    assert.strictEqual(loaded?.mcpServerVersion, testInfo.mcpServerVersion);
  } finally {
    await cleanupTestDir(testDir);
  }
});

test('quickCheck returns version info', { skip: !QSV_AVAILABLE }, async () => {
  const checker = new UpdateChecker(config.qsvBinPath);
  const result = await checker.quickCheck();

  assert.ok(typeof result.skillsOutdated === 'boolean');
  assert.ok(result.versions !== null);
  assert.ok(typeof result.versions.qsvBinaryVersion === 'string');
  assert.ok(typeof result.versions.skillsGeneratedWithVersion === 'string');
  assert.ok(typeof result.versions.mcpServerVersion === 'string');
  assert.ok(typeof result.versions.lastChecked === 'string');
});

test('checkForUpdates returns update check result', { skip: !QSV_AVAILABLE }, async () => {
  const checker = new UpdateChecker(config.qsvBinPath, undefined, {
    checkForUpdatesOnStartup: true,
    notifyOnUpdatesAvailable: true,
  });

  const result = await checker.checkForUpdates();

  assert.ok(typeof result.qsvBinaryOutdated === 'boolean');
  assert.ok(typeof result.skillsOutdated === 'boolean');
  assert.ok(typeof result.mcpServerOutdated === 'boolean');
  assert.ok(typeof result.currentQsvVersion === 'string');
  assert.ok(typeof result.skillsVersion === 'string');
  assert.ok(typeof result.mcpServerVersion === 'string');
  assert.ok(Array.isArray(result.recommendations));
});

test('autoRegenerateSkills returns false when disabled', async () => {
  const checker = new UpdateChecker('qsv', undefined, {
    autoRegenerateSkills: false,
  });

  const result = await checker.autoRegenerateSkills();
  assert.strictEqual(result, false);
});

test('getUpdateConfigFromEnv reads environment variables', () => {
  // Save original env
  const originalEnv = { ...process.env };

  try {
    // Set test environment variables
    process.env.QSV_MCP_AUTO_REGENERATE_SKILLS = 'true';
    process.env.QSV_MCP_CHECK_UPDATES_ON_STARTUP = 'false';
    process.env.QSV_MCP_NOTIFY_UPDATES = 'false';
    process.env.QSV_MCP_GITHUB_REPO = 'test/repo';
    process.env.MCPB_EXTENSION_MODE = 'true';

    const config = getUpdateConfigFromEnv();

    assert.strictEqual(config.autoRegenerateSkills, true);
    assert.strictEqual(config.checkForUpdatesOnStartup, false);
    assert.strictEqual(config.notifyOnUpdatesAvailable, false);
    assert.strictEqual(config.githubRepo, 'test/repo');
    assert.strictEqual(config.isExtensionMode, true);
  } finally {
    // Restore original env
    process.env = originalEnv;
  }
});

test('extension mode skips MCP server version check', { skip: !QSV_AVAILABLE }, async () => {
  const checker = new UpdateChecker(config.qsvBinPath, undefined, {
    isExtensionMode: true,
  });

  const result = await checker.quickCheck();

  // In extension mode, mcpServerVersion should be 'extension'
  assert.strictEqual(result.versions.mcpServerVersion, 'extension');
});

test('checkGitHubReleases internal timeout returns null when fetch never resolves', async () => {
  // Stub global fetch to honor the AbortSignal but never resolve otherwise.
  const originalFetch = globalThis.fetch;
  // Suppress the expected "[UpdateChecker] Failed to check GitHub releases:"
  // log line so test output stays clean — we re-assert the return value below.
  const originalConsoleError = console.error;
  console.error = () => {};
  globalThis.fetch = ((_url: string, init?: { signal?: AbortSignal }) => {
    return new Promise((_resolve, reject) => {
      if (init?.signal) {
        const signal = init.signal;
        const onAbort = () => {
          const err = new Error("aborted");
          (err as Error & { name: string }).name = "AbortError";
          reject(err);
        };
        if (signal.aborted) onAbort();
        else signal.addEventListener("abort", onAbort, { once: true });
      }
      // Otherwise, never resolve.
    });
  }) as typeof fetch;

  // Pre-abort the signal so fetch rejects immediately without depending on a
  // timer firing during the test's event loop window (CI runners can park
  // unrefed timers and stall the test).
  const controller = new AbortController();
  controller.abort();
  try {
    const checker = new UpdateChecker();
    const result = await checker.checkGitHubReleases(controller.signal);
    assert.strictEqual(result, null, "Aborted fetch should return null, not throw");
  } finally {
    globalThis.fetch = originalFetch;
    console.error = originalConsoleError;
  }
});

if (!QSV_AVAILABLE) {
  console.log('\n  UpdateChecker tests requiring qsv binary were skipped - qsv not available');
  console.log(`   Current validation: ${JSON.stringify(config.qsvValidation, null, 2)}`);
}
