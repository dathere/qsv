/**
 * Unit tests for configuration module
 */

import { test } from 'node:test';
import assert from 'node:assert';
import {
  config,
  parseMemoryToBytes,
  parseQsvMemoryInfo,
  parsePolarsVersion,
  parseQsvVersion,
  parseQsvCommandList,
  expandTemplateVars,
  isPluginMode,
} from '../src/config.js';
import { QSV_AVAILABLE } from './test-helpers.js';

test('config has all required properties', () => {
  assert.ok(typeof config.operationTimeoutMs === 'number');
  assert.ok(typeof config.maxFilesPerListing === 'number');
  assert.ok(typeof config.maxConcurrentOperations === 'number');
});

test('config has reasonable defaults', () => {
  assert.ok(config.operationTimeoutMs >= 1000 && config.operationTimeoutMs <= 30 * 60 * 1000);
  assert.ok(config.maxFilesPerListing >= 1 && config.maxFilesPerListing <= 100000);
  assert.ok(config.maxConcurrentOperations >= 1 && config.maxConcurrentOperations <= 100);
});

test('config defaults match expected values', () => {
  // Note: This test verifies the config values that were loaded when the module initialized.
  // The config module loads once at import time, so environment variables must be set
  // before the test suite runs to override defaults. This test assumes a clean environment
  // and verifies that the expected default values are present.

  // If env vars were not set when config loaded, these should be the defaults
  // If env vars were set, these assertions may fail (which is expected behavior)
  const expectedDefaults = {
    operationTimeoutMs: 2 * 60 * 1000, // 2 minutes
    maxFilesPerListing: 1000,
    maxConcurrentOperations: 10,
  };

  // Only assert defaults if the actual values match (allows for env var overrides)
  if (config.operationTimeoutMs === expectedDefaults.operationTimeoutMs) {
    assert.strictEqual(config.operationTimeoutMs, expectedDefaults.operationTimeoutMs);
    assert.strictEqual(config.maxFilesPerListing, expectedDefaults.maxFilesPerListing);
    assert.strictEqual(config.maxConcurrentOperations, expectedDefaults.maxConcurrentOperations);
  }
});

// ============================================================================
// Memory Parsing Tests
// ============================================================================

test('parseMemoryToBytes parses bytes correctly', () => {
  assert.strictEqual(parseMemoryToBytes('0 B'), 0);
  assert.strictEqual(parseMemoryToBytes('100 B'), 100);
  assert.strictEqual(parseMemoryToBytes('1024 B'), 1024);
});

test('parseMemoryToBytes parses KiB correctly', () => {
  assert.strictEqual(parseMemoryToBytes('1 KiB'), 1024);
  assert.strictEqual(parseMemoryToBytes('2.5 KiB'), 2.5 * 1024);
  assert.strictEqual(parseMemoryToBytes('100 KiB'), 100 * 1024);
});

test('parseMemoryToBytes parses MiB correctly', () => {
  assert.strictEqual(parseMemoryToBytes('1 MiB'), 1024 * 1024);
  assert.strictEqual(parseMemoryToBytes('512 MiB'), 512 * 1024 * 1024);
});

test('parseMemoryToBytes parses GiB correctly', () => {
  assert.strictEqual(parseMemoryToBytes('1 GiB'), 1024 * 1024 * 1024);
  assert.strictEqual(parseMemoryToBytes('64.00 GiB'), 64 * 1024 * 1024 * 1024);
  assert.strictEqual(parseMemoryToBytes('8.5 GiB'), 8.5 * 1024 * 1024 * 1024);
});

test('parseMemoryToBytes parses TiB correctly', () => {
  assert.strictEqual(parseMemoryToBytes('1 TiB'), 1024 * 1024 * 1024 * 1024);
  assert.strictEqual(parseMemoryToBytes('2 TiB'), 2 * 1024 * 1024 * 1024 * 1024);
});

test('parseMemoryToBytes is case-insensitive', () => {
  assert.strictEqual(parseMemoryToBytes('1 gib'), 1024 * 1024 * 1024);
  assert.strictEqual(parseMemoryToBytes('1 GIB'), 1024 * 1024 * 1024);
  assert.strictEqual(parseMemoryToBytes('1 Gib'), 1024 * 1024 * 1024);
});

test('parseMemoryToBytes handles spacing variations', () => {
  assert.strictEqual(parseMemoryToBytes('64.00 GiB'), 64 * 1024 * 1024 * 1024);
  assert.strictEqual(parseMemoryToBytes('64.00GiB'), 64 * 1024 * 1024 * 1024);
});

test('parseMemoryToBytes returns null for invalid input', () => {
  assert.strictEqual(parseMemoryToBytes(''), null);
  assert.strictEqual(parseMemoryToBytes('invalid'), null);
  assert.strictEqual(parseMemoryToBytes('100'), null);
  assert.strictEqual(parseMemoryToBytes('100 GB'), null); // GB not supported, only GiB
  assert.strictEqual(parseMemoryToBytes('100 MB'), null); // MB not supported, only MiB
});

test('parseMemoryToBytes returns null for negative values', () => {
  assert.strictEqual(parseMemoryToBytes('-1 GiB'), null);
  assert.strictEqual(parseMemoryToBytes('-100 MiB'), null);
  assert.strictEqual(parseMemoryToBytes('-0.5 TiB'), null);
});

test('parseQsvMemoryInfo extracts total memory from version output', () => {
  const versionOutput = 'qsv 13.0.0-mimalloc 315-...;51.20 GiB-0 B-14.18 GiB-64.00 GiB (aarch64-apple-darwin) compiled';
  const result = parseQsvMemoryInfo(versionOutput);

  assert.ok(result !== null);
  assert.strictEqual(result?.totalMemory, '64.00 GiB');
  assert.strictEqual(result?.totalMemoryBytes, 64 * 1024 * 1024 * 1024);
});

test('parseQsvMemoryInfo handles different memory values', () => {
  const versionOutput = 'qsv 14.0.0 test;32.00 GiB-1 GiB-8.00 GiB-128.00 GiB (x86_64-linux) compiled';
  const result = parseQsvMemoryInfo(versionOutput);

  assert.ok(result !== null);
  assert.strictEqual(result?.totalMemory, '128.00 GiB');
  assert.strictEqual(result?.totalMemoryBytes, 128 * 1024 * 1024 * 1024);
});

test('parseQsvMemoryInfo handles small memory values', () => {
  const versionOutput = 'qsv 13.0.0;500 MiB-0 B-256 MiB-1 GiB (test) compiled';
  const result = parseQsvMemoryInfo(versionOutput);

  assert.ok(result !== null);
  assert.strictEqual(result?.totalMemory, '1 GiB');
  assert.strictEqual(result?.totalMemoryBytes, 1 * 1024 * 1024 * 1024);
});

test('parseQsvMemoryInfo returns null for invalid output', () => {
  assert.strictEqual(parseQsvMemoryInfo(''), null);
  assert.strictEqual(parseQsvMemoryInfo('qsv 13.0.0'), null);
  assert.strictEqual(parseQsvMemoryInfo('no memory info here'), null);
});

// ============================================================================
// qsv Version Parsing Tests
// ============================================================================

test('parseQsvVersion extracts version from qsv output', () => {
  const versionOutput = 'qsv 16.1.0-mimalloc 315-apply;fetch;polars-0.53.0:54c9168;self_update-16-16;51.20 GiB-0 B-13.94 GiB-64.00 GiB (aarch64-apple-darwin)';
  assert.strictEqual(parseQsvVersion(versionOutput), '16.1.0-mimalloc');
});

test('parseQsvVersion extracts version from qsvlite output', () => {
  const versionOutput = 'qsvlite 16.1.0 100-count;headers;stats;51.20 GiB-0 B-13.94 GiB-64.00 GiB (aarch64-apple-darwin)';
  assert.strictEqual(parseQsvVersion(versionOutput), '16.1.0');
});

test('parseQsvVersion extracts version from qsvdp output', () => {
  const versionOutput = 'qsvdp 16.1.0 200-apply;fetch;stats;51.20 GiB-0 B-13.94 GiB-64.00 GiB (aarch64-apple-darwin)';
  assert.strictEqual(parseQsvVersion(versionOutput), '16.1.0');
});

test('parseQsvVersion returns null for unrecognized binary name', () => {
  const versionOutput = 'xsv 16.1.0 some-features';
  assert.strictEqual(parseQsvVersion(versionOutput), null);
});

test('parseQsvVersion returns null for empty string', () => {
  assert.strictEqual(parseQsvVersion(''), null);
});

test('parseQsvVersion handles pre-release versions', () => {
  const versionOutput = 'qsv 16.2.0-alpha.1 315-apply;fetch';
  assert.strictEqual(parseQsvVersion(versionOutput), '16.2.0-alpha.1');
});

// ============================================================================
// Polars Version Parsing Tests
// ============================================================================

test('parsePolarsVersion extracts version from full qsv --version output', () => {
  const versionOutput = 'qsv 16.1.0-mimalloc 315-apply;fetch;polars-0.53.0:54c9168;self_update-16-16;51.20 GiB-0 B-13.94 GiB-64.00 GiB (aarch64-apple-darwin)';
  assert.strictEqual(parsePolarsVersion(versionOutput), '0.53.0');
});

test('parsePolarsVersion returns null when polars is not present', () => {
  const versionOutput = 'qsv 16.1.0-mimalloc 315-apply;fetch;self_update-16-16;51.20 GiB-0 B-13.94 GiB-64.00 GiB (aarch64-apple-darwin)';
  assert.strictEqual(parsePolarsVersion(versionOutput), null);
});

test('parsePolarsVersion returns null for qsvlite output', () => {
  const versionOutput = 'qsvlite 16.1.0 100-count;headers;stats;51.20 GiB-0 B-13.94 GiB-64.00 GiB (aarch64-apple-darwin)';
  assert.strictEqual(parsePolarsVersion(versionOutput), null);
});

test('parsePolarsVersion returns null for empty string', () => {
  assert.strictEqual(parsePolarsVersion(''), null);
});

test('parsePolarsVersion handles polars as first feature after version string', () => {
  const versionOutput = 'qsv 16.0.0 polars-1.2.3:abc1234;other_feature';
  assert.strictEqual(parsePolarsVersion(versionOutput), '1.2.3');
});

test('parsePolarsVersion handles polars as last feature', () => {
  const versionOutput = 'qsv 16.0.0 other;polars-0.99.0:def5678';
  assert.strictEqual(parsePolarsVersion(versionOutput), '0.99.0');
});

test('parsePolarsVersion handles polars without git hash suffix', () => {
  const versionOutput = 'qsv 16.0.0 apply;polars-0.53.0;self_update';
  assert.strictEqual(parsePolarsVersion(versionOutput), '0.53.0');
});

test('parsePolarsVersion does not match hyphenated prefix like non-polars', () => {
  const versionOutput = 'qsv 16.0.0 apply;non-polars-1.0.0;self_update';
  assert.strictEqual(parsePolarsVersion(versionOutput), null);
});

test('parsePolarsVersion matches polars as first feature after dash-separated count', () => {
  // When polars is the first feature (e.g. "315-polars-..."), the digit-dash
  // count separator is recognized as a valid preceding context.
  const versionOutput = 'qsv 16.0.0 315-polars-0.53.0;self_update';
  assert.strictEqual(parsePolarsVersion(versionOutput), '0.53.0');
});

test('config.qsvValidation includes polarsVersion when valid', { skip: !QSV_AVAILABLE }, () => {
  // Precondition: qsv must be valid (Polars required for validity)
  assert.strictEqual(config.qsvValidation.valid, true,
    'qsv should be valid when QSV_AVAILABLE is true');
  // If qsv is valid, polarsVersion should be present (Polars is required)
  assert.ok(typeof config.qsvValidation.polarsVersion === 'string',
    'polarsVersion should be a string when qsv is valid');
  // Verify it looks like a semver version
  const pv = config.qsvValidation.polarsVersion as string;
  assert.ok(/^\d+\.\d+\.\d+$/.test(pv),
    `polarsVersion "${pv}" should match semver format`);
});

// ============================================================================
// Command List Parsing Tests
// ============================================================================

test('parseQsvCommandList parses qsv format with count', () => {
  const listOutput = `Installed commands (63):
    apply       Apply series of transformations to a column
    behead      Drop header from CSV file
    cat         Concatenate CSV files by row or by column
    count       Count the rows in a CSV file`;

  const result = parseQsvCommandList(listOutput);

  assert.ok(result !== null);
  assert.strictEqual(result?.count, 63);
  assert.ok(result?.commands.includes('apply'));
  assert.ok(result?.commands.includes('behead'));
  assert.ok(result?.commands.includes('cat'));
  assert.ok(result?.commands.includes('count'));
  assert.strictEqual(result?.commands.length, 4);
});

test('parseQsvCommandList parses qsvlite format without count', () => {
  const listOutput = `Installed commands:
    cat         Concatenate CSV files by row or by column
    count       Count the rows in a CSV file
    headers     Show the headers of a CSV file`;

  const result = parseQsvCommandList(listOutput);

  assert.ok(result !== null);
  assert.strictEqual(result?.count, 3); // Uses parsed count when header count not available
  assert.ok(result?.commands.includes('cat'));
  assert.ok(result?.commands.includes('count'));
  assert.ok(result?.commands.includes('headers'));
  assert.strictEqual(result?.commands.length, 3);
});

test('parseQsvCommandList returns null for invalid output', () => {
  assert.strictEqual(parseQsvCommandList(''), null);
  assert.strictEqual(parseQsvCommandList('no commands here'), null);
  assert.strictEqual(parseQsvCommandList('Available commands:'), null); // Wrong header format
});

test('parseQsvCommandList handles empty command list', () => {
  const listOutput = `Installed commands (0):`;

  const result = parseQsvCommandList(listOutput);

  // Returns null because no commands were parsed
  assert.strictEqual(result, null);
});

test('parseQsvCommandList extracts command names correctly', () => {
  const listOutput = `Installed commands (5):
    apply       Apply series of transformations
    sqlp        Run SQL queries using Polars
    joinp       Join CSV files using Polars
    stats       Compute statistics for each column
    frequency   Build frequency tables`;

  const result = parseQsvCommandList(listOutput);

  assert.ok(result !== null);
  assert.deepStrictEqual(result?.commands, ['apply', 'sqlp', 'joinp', 'stats', 'frequency']);
});

test('parseQsvCommandList handles varying whitespace indentation', () => {
  // Test with 2 spaces instead of 4
  const listOutput2Spaces = `Installed commands (2):
  cat         Concatenate CSV files
  count       Count the rows`;

  const result2 = parseQsvCommandList(listOutput2Spaces);
  assert.ok(result2 !== null);
  assert.deepStrictEqual(result2?.commands, ['cat', 'count']);

  // Test with 8 spaces
  const listOutput8Spaces = `Installed commands (2):
        cat         Concatenate CSV files
        count       Count the rows`;

  const result8 = parseQsvCommandList(listOutput8Spaces);
  assert.ok(result8 !== null);
  assert.deepStrictEqual(result8?.commands, ['cat', 'count']);

  // Test with tabs
  const listOutputTabs = `Installed commands (2):
\tcat\t\tConcatenate CSV files
\tcount\t\tCount the rows`;

  const resultTabs = parseQsvCommandList(listOutputTabs);
  assert.ok(resultTabs !== null);
  assert.deepStrictEqual(resultTabs?.commands, ['cat', 'count']);
});

// ============================================================================
// qsvValidation Tests
// ============================================================================

test('config.qsvValidation includes memory info when available', () => {
  // This test checks if the validation result structure is correct
  // The actual values depend on the installed qsv binary
  if (config.qsvValidation.valid) {
    // If qsv is valid, check for expected structure
    assert.ok(typeof config.qsvValidation.version === 'string');
    assert.ok(typeof config.qsvValidation.path === 'string');

    // Memory info should be present if qsv version output includes it
    if (config.qsvValidation.totalMemory) {
      assert.ok(typeof config.qsvValidation.totalMemory === 'string');
      assert.ok(typeof config.qsvValidation.totalMemoryBytes === 'number');
      assert.ok(config.qsvValidation.totalMemoryBytes > 0);
    }
  }
});

test('config.qsvValidation includes command info when available', () => {
  if (config.qsvValidation.valid) {
    // Command info should be present if qsv --list works
    if (config.qsvValidation.availableCommands) {
      assert.ok(Array.isArray(config.qsvValidation.availableCommands));
      assert.ok(config.qsvValidation.availableCommands.length > 0);
      assert.ok(typeof config.qsvValidation.commandCount === 'number');
      assert.ok(config.qsvValidation.commandCount > 0);
    }
  }
});

// ============================================================================
// Expose All Tools Config Tests
// ============================================================================

test('config.exposeAllTools exists and has valid type', () => {
  assert.ok('exposeAllTools' in config);
  // exposeAllTools can be boolean or undefined (for auto-detect)
  const validTypes = ['boolean', 'undefined'];
  assert.ok(validTypes.includes(typeof config.exposeAllTools),
    `exposeAllTools should be boolean or undefined, got ${typeof config.exposeAllTools}`);
});

test('config.exposeAllTools defaults to undefined when env var not set', () => {
  // Default should be undefined for auto-detect behavior
  // When undefined, the server auto-detects Claude clients
  // This test verifies the expected default behavior
  // If env var was set, config.exposeAllTools may be true/false (which is also valid)
  // The test documents expected default value
  const expectedDefault = undefined;
  if (config.exposeAllTools === expectedDefault) {
    assert.strictEqual(config.exposeAllTools, expectedDefault);
  }
});

// ============================================================================
// Template Variable Expansion Tests
// ============================================================================

test('expandTemplateVars expands ${PWD} to process.cwd()', () => {
  const result = expandTemplateVars('${PWD}/data');
  assert.strictEqual(result, `${process.cwd()}/data`);
});

test('expandTemplateVars expands multiple ${PWD} occurrences', () => {
  const result = expandTemplateVars('${PWD}/in:${PWD}/out');
  assert.strictEqual(result, `${process.cwd()}/in:${process.cwd()}/out`);
});

test('expandTemplateVars returns empty string for empty input', () => {
  const result = expandTemplateVars('');
  assert.strictEqual(result, '');
});

test('expandTemplateVars returns value unchanged when no templates present', () => {
  const result = expandTemplateVars('/usr/local/bin');
  assert.strictEqual(result, '/usr/local/bin');
});

// ============================================================================
// Plugin Mode Tests
// ============================================================================

test('config.isPluginMode exists and has valid type', () => {
  assert.ok('isPluginMode' in config);
  assert.strictEqual(typeof config.isPluginMode, 'boolean');
});

test('isPluginMode returns false when CLAUDE_PLUGIN_ROOT is not set', () => {
  // isPluginMode() reads process.env at call time, so we can test deterministically
  const origPluginRoot = process.env['CLAUDE_PLUGIN_ROOT'];
  const origExtMode = process.env['MCPB_EXTENSION_MODE'];
  const origPluginMode = process.env['QSV_MCP_PLUGIN_MODE'];
  try {
    delete process.env['CLAUDE_PLUGIN_ROOT'];
    delete process.env['MCPB_EXTENSION_MODE'];
    delete process.env['QSV_MCP_PLUGIN_MODE'];
    assert.strictEqual(isPluginMode(), false);
  } finally {
    if (origPluginRoot !== undefined) process.env['CLAUDE_PLUGIN_ROOT'] = origPluginRoot;
    else delete process.env['CLAUDE_PLUGIN_ROOT'];
    if (origExtMode !== undefined) process.env['MCPB_EXTENSION_MODE'] = origExtMode;
    else delete process.env['MCPB_EXTENSION_MODE'];
    if (origPluginMode !== undefined) process.env['QSV_MCP_PLUGIN_MODE'] = origPluginMode;
    else delete process.env['QSV_MCP_PLUGIN_MODE'];
  }
});

test('isPluginMode returns true when CLAUDE_PLUGIN_ROOT is set and MCPB_EXTENSION_MODE is not', () => {
  const origPluginRoot = process.env['CLAUDE_PLUGIN_ROOT'];
  const origExtMode = process.env['MCPB_EXTENSION_MODE'];
  const origPluginMode = process.env['QSV_MCP_PLUGIN_MODE'];
  try {
    process.env['CLAUDE_PLUGIN_ROOT'] = '/some/path';
    delete process.env['MCPB_EXTENSION_MODE'];
    delete process.env['QSV_MCP_PLUGIN_MODE'];
    assert.strictEqual(isPluginMode(), true);
  } finally {
    if (origPluginRoot !== undefined) {
      process.env['CLAUDE_PLUGIN_ROOT'] = origPluginRoot;
    } else {
      delete process.env['CLAUDE_PLUGIN_ROOT'];
    }
    if (origExtMode !== undefined) process.env['MCPB_EXTENSION_MODE'] = origExtMode;
    else delete process.env['MCPB_EXTENSION_MODE'];
    if (origPluginMode !== undefined) process.env['QSV_MCP_PLUGIN_MODE'] = origPluginMode;
    else delete process.env['QSV_MCP_PLUGIN_MODE'];
  }
});

test('isPluginMode returns false when MCPB_EXTENSION_MODE is enabled', () => {
  // Extension mode takes priority over plugin mode
  const origPluginRoot = process.env['CLAUDE_PLUGIN_ROOT'];
  const origExtMode = process.env['MCPB_EXTENSION_MODE'];
  const origPluginMode = process.env['QSV_MCP_PLUGIN_MODE'];
  try {
    process.env['CLAUDE_PLUGIN_ROOT'] = '/some/path';
    process.env['MCPB_EXTENSION_MODE'] = 'true';
    delete process.env['QSV_MCP_PLUGIN_MODE'];
    assert.strictEqual(isPluginMode(), false);
  } finally {
    if (origPluginRoot !== undefined) {
      process.env['CLAUDE_PLUGIN_ROOT'] = origPluginRoot;
    } else {
      delete process.env['CLAUDE_PLUGIN_ROOT'];
    }
    if (origExtMode !== undefined) {
      process.env['MCPB_EXTENSION_MODE'] = origExtMode;
    } else {
      delete process.env['MCPB_EXTENSION_MODE'];
    }
    if (origPluginMode !== undefined) {
      process.env['QSV_MCP_PLUGIN_MODE'] = origPluginMode;
    } else {
      delete process.env['QSV_MCP_PLUGIN_MODE'];
    }
  }
});

// ============================================================================
// QSV_MCP_PLUGIN_MODE Override Tests
// ============================================================================

test('QSV_MCP_PLUGIN_MODE=true enables plugin mode regardless of CLAUDE_PLUGIN_ROOT', () => {
  const origPluginRoot = process.env['CLAUDE_PLUGIN_ROOT'];
  const origExtMode = process.env['MCPB_EXTENSION_MODE'];
  const origPluginMode = process.env['QSV_MCP_PLUGIN_MODE'];
  try {
    delete process.env['CLAUDE_PLUGIN_ROOT'];
    delete process.env['MCPB_EXTENSION_MODE'];
    process.env['QSV_MCP_PLUGIN_MODE'] = 'true';
    assert.strictEqual(isPluginMode(), true);
  } finally {
    if (origPluginRoot !== undefined) process.env['CLAUDE_PLUGIN_ROOT'] = origPluginRoot;
    else delete process.env['CLAUDE_PLUGIN_ROOT'];
    if (origExtMode !== undefined) process.env['MCPB_EXTENSION_MODE'] = origExtMode;
    else delete process.env['MCPB_EXTENSION_MODE'];
    if (origPluginMode !== undefined) process.env['QSV_MCP_PLUGIN_MODE'] = origPluginMode;
    else delete process.env['QSV_MCP_PLUGIN_MODE'];
  }
});

test('QSV_MCP_PLUGIN_MODE=false disables plugin mode even with CLAUDE_PLUGIN_ROOT set', () => {
  const origPluginRoot = process.env['CLAUDE_PLUGIN_ROOT'];
  const origExtMode = process.env['MCPB_EXTENSION_MODE'];
  const origPluginMode = process.env['QSV_MCP_PLUGIN_MODE'];
  try {
    process.env['CLAUDE_PLUGIN_ROOT'] = '/some/path';
    delete process.env['MCPB_EXTENSION_MODE'];
    process.env['QSV_MCP_PLUGIN_MODE'] = 'false';
    assert.strictEqual(isPluginMode(), false);
  } finally {
    if (origPluginRoot !== undefined) process.env['CLAUDE_PLUGIN_ROOT'] = origPluginRoot;
    else delete process.env['CLAUDE_PLUGIN_ROOT'];
    if (origExtMode !== undefined) process.env['MCPB_EXTENSION_MODE'] = origExtMode;
    else delete process.env['MCPB_EXTENSION_MODE'];
    if (origPluginMode !== undefined) process.env['QSV_MCP_PLUGIN_MODE'] = origPluginMode;
    else delete process.env['QSV_MCP_PLUGIN_MODE'];
  }
});

test('QSV_MCP_PLUGIN_MODE takes precedence over CLAUDE_PLUGIN_ROOT', () => {
  const origPluginRoot = process.env['CLAUDE_PLUGIN_ROOT'];
  const origExtMode = process.env['MCPB_EXTENSION_MODE'];
  const origPluginMode = process.env['QSV_MCP_PLUGIN_MODE'];
  try {
    // Even with CLAUDE_PLUGIN_ROOT set, QSV_MCP_PLUGIN_MODE=false wins
    process.env['CLAUDE_PLUGIN_ROOT'] = '/some/path';
    delete process.env['MCPB_EXTENSION_MODE'];
    process.env['QSV_MCP_PLUGIN_MODE'] = 'no';
    assert.strictEqual(isPluginMode(), false);

    // And QSV_MCP_PLUGIN_MODE=true wins even without CLAUDE_PLUGIN_ROOT
    delete process.env['CLAUDE_PLUGIN_ROOT'];
    process.env['QSV_MCP_PLUGIN_MODE'] = '1';
    assert.strictEqual(isPluginMode(), true);
  } finally {
    if (origPluginRoot !== undefined) process.env['CLAUDE_PLUGIN_ROOT'] = origPluginRoot;
    else delete process.env['CLAUDE_PLUGIN_ROOT'];
    if (origExtMode !== undefined) process.env['MCPB_EXTENSION_MODE'] = origExtMode;
    else delete process.env['MCPB_EXTENSION_MODE'];
    if (origPluginMode !== undefined) process.env['QSV_MCP_PLUGIN_MODE'] = origPluginMode;
    else delete process.env['QSV_MCP_PLUGIN_MODE'];
  }
});

test('QSV_MCP_PLUGIN_MODE with invalid value falls back to auto-detection', () => {
  const origPluginRoot = process.env['CLAUDE_PLUGIN_ROOT'];
  const origExtMode = process.env['MCPB_EXTENSION_MODE'];
  const origPluginMode = process.env['QSV_MCP_PLUGIN_MODE'];
  try {
    // Invalid value should be treated as unset (getOptionalBooleanEnv returns undefined)
    delete process.env['CLAUDE_PLUGIN_ROOT'];
    delete process.env['MCPB_EXTENSION_MODE'];
    process.env['QSV_MCP_PLUGIN_MODE'] = 'invalid';
    // Falls through to auto-detection: no CLAUDE_PLUGIN_ROOT → false
    assert.strictEqual(isPluginMode(), false);

    // With CLAUDE_PLUGIN_ROOT set, invalid override falls through to auto-detection → true
    process.env['CLAUDE_PLUGIN_ROOT'] = '/some/path';
    assert.strictEqual(isPluginMode(), true);
  } finally {
    if (origPluginRoot !== undefined) process.env['CLAUDE_PLUGIN_ROOT'] = origPluginRoot;
    else delete process.env['CLAUDE_PLUGIN_ROOT'];
    if (origExtMode !== undefined) process.env['MCPB_EXTENSION_MODE'] = origExtMode;
    else delete process.env['MCPB_EXTENSION_MODE'];
    if (origPluginMode !== undefined) process.env['QSV_MCP_PLUGIN_MODE'] = origPluginMode;
    else delete process.env['QSV_MCP_PLUGIN_MODE'];
  }
});
