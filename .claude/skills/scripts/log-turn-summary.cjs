#!/usr/bin/env node

// log-turn-summary.cjs — Stop hook
// Logs the model's last assistant message to the qsv audit log (qsvmcp.log)
// at the end of each turn, providing a persistent record of what Claude did.
// Uses CommonJS so it works standalone without package.json declaring "type": "module".

const { execFile } = require('node:child_process');
const { randomUUID } = require('node:crypto');
const { findQsvMcpBinary, truncateMessage, readStdin } = require('./qsv-utils.cjs');

readStdin().then((input) => {
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

  const lastMessage = String(parsed.last_assistant_message || '').trim();
  if (!lastMessage) return;

  // Use cwd from hook input so qsvmcp.log lands in the session working directory
  const cwd = parsed.cwd || process.cwd();

  const bin = findQsvMcpBinary();
  if (!bin) {
    process.stderr.write('[log-turn-summary] qsvmcp binary not found\n');
    return;
  }

  const logId = `t-${randomUUID()}`;
  // Truncate AFTER building the full message (including prefix)
  const message = truncateMessage(`[turn_summary] ${lastMessage}`);

  execFile(bin, ['log', 'turn_summary', logId, message], { timeout: 5000, cwd }, (err) => {
    if (err) {
      process.stderr.write(`[log-turn-summary] qsv log failed: ${err.message}\n`);
    }
  });
});
