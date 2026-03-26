#!/usr/bin/env node

// log-turn-summary.cjs — Stop hook
// Logs the model's last assistant message to the qsv audit log (qsvmcp.log)
// at the end of each turn, providing a persistent record of what Claude did.
// Uses CommonJS so it works standalone without package.json declaring "type": "module".

const { execFile } = require('node:child_process');
const { execFileSync } = require('node:child_process');
const { randomUUID } = require('node:crypto');

/** Maximum message length to log (matches MAX_LOG_MESSAGE_LEN in mcp-tools.ts). */
const MAX_LOG_MESSAGE_LEN = 4096;

/**
 * Find qsvmcp binary. Only qsvmcp has the `log` command.
 * Checks QSV_MCP_BIN_PATH env var first, then PATH via which/where.
 */
function findQsvBinary() {
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

  let lastMessage = String(parsed.last_assistant_message || '').trim();
  if (!lastMessage) return;

  // Use cwd from hook input so qsvmcp.log lands in the session working directory
  const cwd = parsed.cwd || process.cwd();

  // Truncate to MAX_LOG_MESSAGE_LEN (Unicode-safe via Array.from)
  if (Array.from(lastMessage).length > MAX_LOG_MESSAGE_LEN) {
    lastMessage = Array.from(lastMessage).slice(0, MAX_LOG_MESSAGE_LEN).join('');
  }

  const bin = findQsvBinary();
  if (!bin) {
    process.stderr.write('[log-turn-summary] qsvmcp binary not found\n');
    return;
  }

  const logId = `t-${randomUUID()}`;
  const message = `[turn_summary] ${lastMessage}`;

  execFile(bin, ['log', 'turn_summary', logId, message], { timeout: 5000, cwd }, (err) => {
    if (err) {
      process.stderr.write(`[log-turn-summary] qsv log failed: ${err.message}\n`);
    }
  });
});
