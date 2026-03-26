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
 * Find qsvmcp binary. Only qsvmcp has the `log` command (requires the `mcp`
 * feature, which is NOT included in `all_features` / the `qsv` full binary).
 * Checks QSV_MCP_BIN_PATH env var first, then PATH via which/where.
 */
function findQsvBinary() {
  // Check env var first
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

let input = '';
process.stdin.on('data', (chunk) => { input += chunk; });
process.stdin.on('end', () => {
  // Respect QSV_MCP_LOG_LEVEL — skip logging when audit logging is disabled
  const logLevel = (process.env.QSV_MCP_LOG_LEVEL || 'info').toLowerCase();
  if (logLevel === 'off') return;

  let parsed;
  try {
    parsed = JSON.parse(input);
  } catch {
    // Invalid JSON — nothing to log
    return;
  }

  let prompt = String(parsed.prompt || '').trim();
  if (!prompt) return;

  // Use cwd from hook input so qsvmcp.log lands in the session working directory
  const cwd = parsed.cwd || process.cwd();

  // Truncate to MAX_LOG_MESSAGE_LEN (Unicode-safe via Array.from)
  if (Array.from(prompt).length > MAX_LOG_MESSAGE_LEN) {
    prompt = Array.from(prompt).slice(0, MAX_LOG_MESSAGE_LEN).join('');
  }

  const bin = findQsvBinary();
  if (!bin) {
    process.stderr.write('[log-user-prompt] qsvmcp binary not found\n');
    return;
  }

  const logId = `u-${randomUUID()}`;
  const message = `[user_prompt] ${prompt}`;

  execFile(bin, ['log', 'user_prompt', logId, message], { timeout: 5000, cwd }, (err) => {
    if (err) {
      process.stderr.write(`[log-user-prompt] qsv log failed: ${err.message}\n`);
    }
  });
});
