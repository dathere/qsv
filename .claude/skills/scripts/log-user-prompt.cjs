#!/usr/bin/env node

// log-user-prompt.cjs — UserPromptSubmit hook
// Logs the user's prompt to the qsv audit log (qsvmcp.log) for reproducibility.
// Uses CommonJS so it works standalone without package.json declaring "type": "module".

const { execFile } = require('node:child_process');
const { execFileSync } = require('node:child_process');
const { randomUUID } = require('node:crypto');

/** Maximum prompt length to log (matches MAX_LOG_MESSAGE_LEN in mcp-tools.ts). */
const MAX_LOG_MESSAGE_LEN = 4096;

/**
 * Find qsvmcp or qsv binary, mirroring cowork-setup.cjs detection logic.
 * Checks QSV_MCP_BIN_PATH env var first, then PATH via which/where.
 */
function findQsvBinary() {
  // Check env var first
  const envPath = process.env.QSV_MCP_BIN_PATH;
  if (envPath) return envPath;

  const command = process.platform === 'win32' ? 'where' : 'which';
  for (const binName of ['qsvmcp', 'qsv']) {
    try {
      const result = execFileSync(command, [binName], {
        encoding: 'utf-8',
        stdio: ['pipe', 'pipe', 'pipe'],
        timeout: 5000,
      });
      const binPath = result.trim().split('\n')[0].trim();
      if (binPath) return binPath;
    } catch {
      // Not found, try next
    }
  }
  return null;
}

let input = '';
process.stdin.on('data', (chunk) => { input += chunk; });
process.stdin.on('end', () => {
  let prompt = '';
  try {
    prompt = JSON.parse(input).prompt || '';
  } catch {
    // Invalid JSON — nothing to log
    return;
  }

  prompt = String(prompt).trim();
  if (!prompt) return;

  // Truncate to MAX_LOG_MESSAGE_LEN (Unicode-safe via Array.from)
  if (Array.from(prompt).length > MAX_LOG_MESSAGE_LEN) {
    prompt = Array.from(prompt).slice(0, MAX_LOG_MESSAGE_LEN).join('');
  }

  const bin = findQsvBinary();
  if (!bin) {
    process.stderr.write('[log-user-prompt] qsv binary not found\n');
    return;
  }

  const logId = `u-${randomUUID()}`;
  const message = `[user_prompt] ${prompt}`;

  execFile(bin, ['log', 'user_prompt', logId, message], { timeout: 5000 }, (err) => {
    if (err) {
      process.stderr.write(`[log-user-prompt] qsv log failed: ${err.message}\n`);
    }
  });
});
