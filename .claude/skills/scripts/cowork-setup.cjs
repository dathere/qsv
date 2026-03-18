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

async function main() {
  // Allow opting out via environment variable
  if (process.env.QSV_NO_COWORK_SETUP === '1') {
    process.exit(0);
  }

  // Read hook input JSON from stdin (limit to 64KB; timeout after 5s to avoid hangs).
  // Destroying stdin on timeout cleanly breaks the for-await loop instead of calling process.exit().
  let input = '';
  const timeoutId = setTimeout(() => {
    process.stderr.write('cowork-setup: stdin read timed out after 5s\n');
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

  let cwd = '';
  try {
    const parsed = JSON.parse(input);
    cwd = parsed.cwd || '';
  } catch {
    // Invalid or empty JSON — warn and exit (skip warning for empty stdin)
    if (input.trim()) {
      process.stderr.write('cowork-setup: failed to parse stdin as JSON\n');
    }
    process.exit(0);
  }

  if (!cwd) {
    process.exit(0);
  }

  // Resolve to real path to prevent path traversal via symlinks
  try {
    cwd = realpathSync(resolve(cwd));
  } catch {
    // Directory doesn't exist or isn't accessible
    process.exit(0);
  }

  // Guard against deploying into the plugin's own directory tree
  const pluginRoot = process.env.CLAUDE_PLUGIN_ROOT;
  if (!pluginRoot) {
    output('CLAUDE_PLUGIN_ROOT is not set. Skipping qsv CLAUDE.md deployment.');
    process.exit(0);
  }

  let resolvedPluginRoot;
  try {
    resolvedPluginRoot = realpathSync(resolve(pluginRoot));
  } catch {
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
    process.exit(0);
  }

  const template = join(resolvedPluginRoot, 'cowork-CLAUDE.md');
  const target = join(cwd, 'CLAUDE.md');

  // Ensure the template exists
  if (!existsSync(template)) {
    process.exit(0);
  }

  if (existsSync(target)) {
    // Existing CLAUDE.md — don't overwrite
    output(`An existing CLAUDE.md was found at ${target}. It was NOT overwritten. The existing file will be used for workflow guidance.`);
  } else {
    // Copy the template
    try {
      copyFileSync(template, target);
      output(`A qsv CLAUDE.md was created at ${target}. qsv workflow guidance has been set up in the working folder.`);
    } catch {
      output(`Could not create CLAUDE.md in ${cwd} (directory may not be writable). Skipping qsv workflow guidance setup.`);
    }
  }

  // Validate that qsvmcp/qsv binary is available — warn early if not
  const qsvBin = findQsvBinary();
  if (!qsvBin) {
    const mcpbUrl = 'https://github.com/dathere/qsv/releases/latest';
    output(
      `WARNING: The qsv plugin requires the qsvmcp (or qsv) binary, but neither was found on this system. ` +
        `The qsv tools (qsv_stats, qsv_sqlp, etc.) will NOT work until a binary is installed.\n\n` +
        `Install the qsv MCP Server Desktop Extension (.mcpb) — it auto-installs the qsvmcp binary for you:\n` +
        `1. Download the .mcpb file from: ${mcpbUrl}\n` +
        `2. Double-click to install in Claude Desktop\n` +
        `3. Restart Claude Desktop\n\n` +
        `The extension auto-detects and installs qsvmcp. No manual binary setup needed.\n` +
        `If qsvmcp is already installed elsewhere, set QSV_MCP_BIN_PATH to its location.`,
    );
  }
}

main().catch(() => {
  // Never block session start
  process.exit(0);
});
