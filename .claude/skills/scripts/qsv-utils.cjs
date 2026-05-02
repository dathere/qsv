#!/usr/bin/env node

// qsv-utils.cjs — Shared utilities for qsv hook scripts.
// Uses CommonJS so it works standalone without package.json declaring "type": "module".

const { execFile, execFileSync } = require('node:child_process');

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
 * Async variant of findQsvMcpBinary. Use this in hooks that enforce a wall-clock
 * cap via setTimeout — execFileSync blocks the event loop and would prevent the
 * timer from firing. The optional `onSpawn` callback exposes the spawned child
 * so the caller can SIGKILL it if its own hard timer trips.
 *
 * @param {(child: import('node:child_process').ChildProcess) => void} [onSpawn]
 * @returns {Promise<string | null>}
 */
function findQsvMcpBinaryAsync(onSpawn) {
  const envPath = process.env.QSV_MCP_BIN_PATH;
  if (envPath) return Promise.resolve(envPath);

  const command = process.platform === 'win32' ? 'where' : 'which';
  return new Promise((resolve) => {
    const child = execFile(
      command,
      ['qsvmcp'],
      { encoding: 'utf-8', timeout: 5000 },
      (err, stdout) => {
        if (err) {
          resolve(null);
          return;
        }
        const binPath = String(stdout).trim().split('\n')[0].trim();
        resolve(binPath || null);
      },
    );
    if (typeof onSpawn === 'function') onSpawn(child);
  });
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

module.exports = { findQsvMcpBinary, findQsvMcpBinaryAsync, truncateMessage, readStdin, MAX_LOG_MESSAGE_LEN };
