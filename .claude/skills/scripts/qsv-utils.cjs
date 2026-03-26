#!/usr/bin/env node

// qsv-utils.cjs — Shared utilities for qsv hook scripts.
// Uses CommonJS so it works standalone without package.json declaring "type": "module".

const { execFileSync } = require('node:child_process');

/** Maximum message length to log (matches MAX_LOG_MESSAGE_LEN in mcp-tools.ts). */
const MAX_LOG_MESSAGE_LEN = 4096;

/**
 * Find qsvmcp binary. Only qsvmcp has the `log` command (requires the `mcp`
 * feature, which is NOT included in `all_features` / the `qsv` full binary).
 * Checks QSV_MCP_BIN_PATH env var first, then PATH via which/where.
 */
function findQsvMcpBinary() {
  const envPath = process.env.QSV_MCP_BIN_PATH;
  if (envPath) return envPath;

  const command = process.platform === 'win32' ? 'where' : 'which';
  try {
    const result = execFileSync(command, ['qsvmcp'], {
      encoding: 'utf-8',
      stdio: ['pipe', 'pipe', 'pipe'],
      timeout: 5000,
    });
    const binPath = result.trim().split('\n')[0].trim();
    if (binPath) return binPath;
  } catch {
    // Not found
  }
  return null;
}

/**
 * Truncate a string to MAX_LOG_MESSAGE_LEN (Unicode-safe via Array.from).
 * Call this AFTER building the full message (including any prefix).
 */
function truncateMessage(message) {
  const chars = Array.from(message);
  if (chars.length > MAX_LOG_MESSAGE_LEN) {
    return chars.slice(0, MAX_LOG_MESSAGE_LEN).join('');
  }
  return message;
}

/** Maximum stdin size for hook scripts (64KB, same as cowork-setup.cjs). */
const MAX_STDIN_SIZE = 65536;

/** Stdin read timeout in milliseconds. */
const STDIN_TIMEOUT_MS = 5000;

/**
 * Read stdin with a size cap and timeout to prevent hooks from hanging.
 * Returns a Promise that resolves with the raw string (may be empty).
 * Mirrors the defensive pattern from cowork-setup.cjs.
 */
function readStdin() {
  return new Promise((resolve) => {
    let input = '';
    let resolved = false;
    let timeoutId;

    function finish() {
      if (resolved) return;
      resolved = true;
      if (timeoutId) {
        clearTimeout(timeoutId);
      }
      resolve(input);
    }

    timeoutId = setTimeout(() => {
      process.stdin.destroy();
      finish();
    }, STDIN_TIMEOUT_MS);

    process.stdin.on('data', (chunk) => {
      input += chunk;
      if (input.length > MAX_STDIN_SIZE) {
        process.stdin.destroy();
        finish();
      }
    });

    process.stdin.on('end', () => {
      finish();
    });

    process.stdin.on('error', () => {
      finish();
    });

    process.stdin.on('close', () => {
      finish();
    });
  });
}

module.exports = { findQsvMcpBinary, truncateMessage, readStdin, MAX_LOG_MESSAGE_LEN };
