#!/usr/bin/env node

// cowork-setup.js — SessionStart hook
// Copies a qsv CLAUDE.md template into the Cowork working folder
// if one doesn't already exist.
// Node.js port of cowork-setup.sh for Windows compatibility.
// Uses CommonJS so it works standalone without package.json declaring "type": "module".

const { existsSync, copyFileSync, realpathSync } = require('node:fs');
const { execFileSync } = require('node:child_process');
const { resolve, normalize, join } = require('node:path');
const { homedir } = require('node:os');

function output(additionalContext) {
  process.stdout.write(JSON.stringify({ additionalContext }) + '\n');
}

// Minimum qsv version required — keep in sync with manifest.json _meta.minimum_qsv_version
const MINIMUM_QSV_VERSION = '19.0.0';

/**
 * Check if qsvmcp or qsv binary is available.
 * Mirrors the detection logic in config.ts: which/where → common paths.
 * Returns the found path, or null if not found.
 */
function findQsvBinary() {
  const command = process.platform === 'win32' ? 'where' : 'which';
  for (const binName of ['qsvmcp', 'qsv']) {
    try {
      const result = execFileSync(command, [binName], {
        encoding: 'utf-8',
        timeout: 3000,
        stdio: ['ignore', 'pipe', 'ignore'],
      }).trim().split(/\r?\n/)[0];
      if (result) return result;
    } catch {
      // not found via which/where — continue
    }
  }

  // Fall back to common installation paths
  const home = homedir();
  const commonPaths =
    process.platform === 'win32'
      ? [
          'C:\\Program Files\\qsv\\qsvmcp.exe',
          'C:\\Program Files\\qsv\\qsv.exe',
          'C:\\qsv\\qsvmcp.exe',
          'C:\\qsv\\qsv.exe',
          join(home, 'scoop', 'shims', 'qsvmcp.exe'),
          join(home, 'scoop', 'shims', 'qsv.exe'),
          join(home, 'AppData', 'Local', 'Programs', 'qsv', 'qsvmcp.exe'),
          join(home, 'AppData', 'Local', 'Programs', 'qsv', 'qsv.exe'),
        ]
      : [
          '/usr/local/bin/qsvmcp',
          '/usr/local/bin/qsv',
          '/opt/homebrew/bin/qsvmcp',
          '/opt/homebrew/bin/qsv',
          '/usr/bin/qsvmcp',
          '/usr/bin/qsv',
          join(home, '.cargo', 'bin', 'qsvmcp'),
          join(home, '.cargo', 'bin', 'qsv'),
          join(home, '.local', 'bin', 'qsvmcp'),
          join(home, '.local', 'bin', 'qsv'),
        ];

  for (const p of commonPaths) {
    if (existsSync(p)) return p;
  }
  return null;
}

/**
 * Compare two semver strings (major.minor.patch).
 * Returns -1 if a < b, 0 if equal, 1 if a > b.
 */
function compareVersions(a, b) {
  const pa = a.split('.').map(Number);
  const pb = b.split('.').map(Number);
  for (let i = 0; i < 3; i++) {
    if ((pa[i] || 0) < (pb[i] || 0)) return -1;
    if ((pa[i] || 0) > (pb[i] || 0)) return 1;
  }
  return 0;
}

/**
 * Validate the qsv MCP server configuration:
 *  1. MCP server entry point exists in the plugin root
 *  2. qsvmcp/qsv binary is found
 *  3. Binary version >= MINIMUM_QSV_VERSION
 *  4. Polars feature is compiled in (mandatory for MCP tools)
 *
 * Returns { valid, warnings[] } — warnings are emitted via output().
 */
function validateMcpServer(pluginRoot) {
  const warnings = [];

  // 1. Check MCP server entry point
  const serverEntry = join(pluginRoot, 'dist', 'mcp-server.js');
  if (!existsSync(serverEntry)) {
    warnings.push(
      `WARNING: MCP server entry point not found at ${serverEntry}. ` +
        `The qsv tools will NOT work. The plugin may not have been built correctly — ` +
        `try reinstalling the plugin from a fresh .plugin archive.`,
    );
    return { valid: false, warnings };
  }

  // 2. Check binary availability
  const qsvBin = findQsvBinary();
  if (!qsvBin) {
    const mcpbUrl = 'https://github.com/dathere/qsv/releases/latest';
    warnings.push(
      `WARNING: The qsv plugin requires the qsvmcp (or qsv) binary, but neither was found on this system. ` +
        `The qsv tools (qsv_stats, qsv_sqlp, etc.) will NOT work until a binary is installed.\n\n` +
        `Install the qsv MCP Server Desktop Extension (.mcpb) — it auto-installs the qsvmcp binary for you:\n` +
        `1. Download the .mcpb file from: ${mcpbUrl}\n` +
        `2. Double-click to install in Claude Desktop\n` +
        `3. Restart Claude Desktop\n\n` +
        `The extension auto-detects and installs qsvmcp. No manual binary setup needed.\n` +
        `If qsvmcp is already installed elsewhere, set QSV_MCP_BIN_PATH to its location.`,
    );
    return { valid: false, warnings };
  }

  // 3. Run --version and validate
  let versionOutput = '';
  try {
    versionOutput = execFileSync(qsvBin, ['--version'], {
      encoding: 'utf-8',
      timeout: 5000,
      stdio: ['ignore', 'pipe', 'ignore'],
    }).trim();
  } catch {
    warnings.push(
      `WARNING: Found qsv binary at ${qsvBin} but could not run '${qsvBin} --version'. ` +
        `The binary may be corrupted or not executable.`,
    );
    return { valid: false, warnings };
  }

  // 3a. Extract and check version (e.g. "qsvmcp 18.0.0 ..." → "18.0.0")
  const versionMatch = versionOutput.match(/qsv(?:mcp)?\s+(\d+\.\d+\.\d+)/);
  if (!versionMatch) {
    warnings.push(
      `WARNING: Could not parse version from '${qsvBin} --version' output. ` +
        `Expected format: "qsvmcp X.Y.Z" or "qsv X.Y.Z".`,
    );
    return { valid: false, warnings };
  }

  const version = versionMatch[1];
  if (compareVersions(version, MINIMUM_QSV_VERSION) < 0) {
    warnings.push(
      `WARNING: qsv version ${version} found at ${qsvBin}, but version ${MINIMUM_QSV_VERSION} or higher is required. ` +
        `Please update: https://github.com/dathere/qsv/releases/latest`,
    );
    return { valid: false, warnings };
  }

  // 3b. Check Polars feature (mandatory — without it, sqlp/joinp/pivotp etc. won't work)
  // Polars appears in version output as e.g. ";polars-0.53.0:54c9168;" or "polars-0.53.0;"
  const polarsMatch = versionOutput.match(/(?:;|\s|\d-)polars-(\d+\.\d+\.\d+)(?::[0-9a-fA-F]+)?(?:;|\s|$)/);
  if (!polarsMatch) {
    warnings.push(
      `WARNING: qsv binary at ${qsvBin} does not have the Polars feature enabled. ` +
        `The MCP server requires Polars-powered commands (sqlp, joinp, pivotp, etc.). ` +
        `Only qsvmcp or full qsv builds include Polars — qsvlite and qsvdp are NOT supported.\n\n` +
        `Install qsvmcp from: https://github.com/dathere/qsv/releases/latest`,
    );
    return { valid: false, warnings };
  }

  return { valid: true, warnings, version, polarsVersion: polarsMatch[1], path: qsvBin };
}

async function main() {
  const debug = process.env.QSV_COWORK_DEBUG === '1';

  function debugLog(msg) {
    if (debug) process.stderr.write(`cowork-setup [DEBUG]: ${msg}\n`);
  }

  debugLog(`pid=${process.pid} platform=${process.platform}`);
  debugLog(`CLAUDE_PLUGIN_ROOT=${process.env.CLAUDE_PLUGIN_ROOT || '(unset)'}`);
  debugLog(`QSV_MCP_BIN_PATH=${process.env.QSV_MCP_BIN_PATH || '(unset)'}`);
  debugLog(`stdin.isTTY=${process.stdin.isTTY}`);

  // Allow opting out via environment variable
  if (process.env.QSV_NO_COWORK_SETUP === '1') {
    debugLog('QSV_NO_COWORK_SETUP=1 — exiting');
    process.exit(0);
  }

  // Read hook input JSON from stdin (limit to 64KB; timeout after 5s to avoid hangs).
  // Destroying stdin on timeout cleanly breaks the for-await loop instead of calling process.exit().
  let input = '';
  const timeoutId = setTimeout(() => {
    process.stderr.write('cowork-setup: stdin read timed out after 5s\n');
    debugLog('stdin timed out — no data received within 5s');
    process.stdin.destroy();
  }, 5000);
  try {
    for await (const chunk of process.stdin) {
      input += chunk.toString();
      if (input.length > 65536) {
        process.stderr.write('cowork-setup: stdin exceeded 64KB limit, truncating\n');
        process.stdin.destroy();
        break;
      }
    }
  } catch (err) {
    // stdin was destroyed by timeout or limit — continue with whatever was read.
    // Log unexpected errors to stderr to aid debugging.
    if (err?.code && err.code !== 'ERR_USE_AFTER_CLOSE' && err.code !== 'ERR_STREAM_PREMATURE_CLOSE') {
      process.stderr.write(`cowork-setup: stdin error: ${err.code}\n`);
    }
  }
  clearTimeout(timeoutId);

  debugLog(`stdin raw (${input.length} bytes): ${input.substring(0, 500)}`);

  let cwd = '';
  try {
    const parsed = JSON.parse(input);
    debugLog(`stdin parsed keys: ${Object.keys(parsed).join(', ')}`);
    debugLog(`stdin parsed JSON: ${JSON.stringify(parsed, null, 2).substring(0, 1000)}`);
    cwd = parsed.cwd || '';
  } catch {
    // Invalid or empty JSON — warn and exit (skip warning for empty stdin)
    if (input.trim()) {
      process.stderr.write('cowork-setup: failed to parse stdin as JSON\n');
      debugLog(`unparseable stdin content: ${input.substring(0, 200)}`);
    } else {
      debugLog('stdin was empty — no JSON received');
    }
    process.exit(0);
  }

  debugLog(`cwd from stdin: "${cwd}"`);

  if (!cwd) {
    debugLog('cwd is empty — exiting');
    process.exit(0);
  }

  // Resolve to real path to prevent path traversal via symlinks
  try {
    cwd = realpathSync(resolve(cwd));
    debugLog(`resolved cwd: ${cwd}`);
  } catch (err) {
    // Directory doesn't exist or isn't accessible
    debugLog(`cwd resolve failed: ${err?.message || err} — exiting`);
    process.exit(0);
  }

  // Guard against deploying into the plugin's own directory tree
  const pluginRoot = process.env.CLAUDE_PLUGIN_ROOT;
  if (!pluginRoot) {
    debugLog('CLAUDE_PLUGIN_ROOT is not set — exiting');
    output('CLAUDE_PLUGIN_ROOT is not set. Skipping qsv CLAUDE.md deployment.');
    process.exit(0);
  }

  let resolvedPluginRoot;
  try {
    resolvedPluginRoot = realpathSync(resolve(pluginRoot));
    debugLog(`resolved pluginRoot: ${resolvedPluginRoot}`);
  } catch (err) {
    debugLog(`pluginRoot resolve failed: ${err?.message || err} — exiting`);
    process.exit(0);
  }

  // Path prefix check — case-insensitive on Windows and macOS.
  // Note: macOS can have case-sensitive APFS volumes, but the default is case-insensitive.
  // On a case-sensitive macOS volume this guard could theoretically be bypassed with different
  // casing, but the consequence is only writing a CLAUDE.md into the plugin's own tree.
  const normalizedCwd = normalize(cwd);
  const normalizedRoot = normalize(resolvedPluginRoot);
  const caseInsensitive = process.platform === 'win32' || process.platform === 'darwin';
  const cwdForCompare = (caseInsensitive ? normalizedCwd.toLowerCase() : normalizedCwd).replace(/[\\/]+$/, '');
  const rootForCompare = (caseInsensitive ? normalizedRoot.toLowerCase() : normalizedRoot).replace(/[\\/]+$/, '');
  const separator = process.platform === 'win32' ? '\\' : '/';

  if (cwdForCompare === rootForCompare || cwdForCompare.startsWith(rootForCompare + separator)) {
    debugLog('cwd is inside plugin root — skipping CLAUDE.md deployment');
    process.exit(0);
  }

  const template = join(resolvedPluginRoot, 'cowork-CLAUDE.md');
  const target = join(cwd, 'CLAUDE.md');
  debugLog(`template: ${template} (exists=${existsSync(template)})`);
  debugLog(`target: ${target} (exists=${existsSync(target)})`);

  // Ensure the template exists
  if (!existsSync(template)) {
    debugLog('template not found — exiting');
    process.exit(0);
  }

  // Deploy CLAUDE.md (used by Claude Code CLI)
  if (existsSync(target)) {
    output(`An existing CLAUDE.md was found at ${target}. It was NOT overwritten. The existing file will be used for workflow guidance.`);
  } else {
    try {
      copyFileSync(template, target);
      output(`A qsv CLAUDE.md was created at ${target}. qsv workflow guidance has been set up in the working folder.`);
    } catch {
      output(`Could not create CLAUDE.md in ${cwd} (directory may not be writable). Skipping qsv workflow guidance setup.`);
    }
  }

  // Deploy .cowork-instructions.md (read natively by Cowork for per-folder context)
  const coworkTarget = join(cwd, '.cowork-instructions.md');
  debugLog(`coworkTarget: ${coworkTarget} (exists=${existsSync(coworkTarget)})`);
  if (!existsSync(coworkTarget)) {
    try {
      copyFileSync(template, coworkTarget);
      debugLog(`deployed .cowork-instructions.md to ${coworkTarget}`);
      output(`A .cowork-instructions.md was deployed to ${cwd} for Cowork per-folder context. Consider adding it to .gitignore.`);
    } catch {
      debugLog(`could not create .cowork-instructions.md in ${cwd}`);
    }
  } else {
    debugLog('.cowork-instructions.md already exists — skipping');
  }

  // Validate the full MCP server configuration — not just the binary
  const validation = validateMcpServer(resolvedPluginRoot);
  for (const warning of validation.warnings) {
    output(warning);
  }
  if (validation.valid) {
    output(
      `qsv MCP server is properly configured: qsv ${validation.version} with Polars ${validation.polarsVersion} at ${validation.path}`,
    );
  }
}

main().catch(() => {
  // Never block session start
  process.exit(0);
});
