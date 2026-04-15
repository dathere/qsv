#!/usr/bin/env node

// log-user-prompt.cjs — UserPromptSubmit hook
// Logs the user's prompt to the qsv audit log (qsvmcp.log) for reproducibility.
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

  const prompt = String(parsed.prompt || '').trim();
  if (!prompt) return;

  // Use cwd from hook input so qsvmcp.log lands in the session working directory
  const cwd = parsed.cwd || process.cwd();

  const bin = findQsvMcpBinary();
  if (!bin) {
    process.stderr.write('[log-user-prompt] qsvmcp binary not found\n');
    return;
  }

  const logId = `u-${randomUUID()}`;
  // Truncate AFTER building the full message (including prefix)
  const message = truncateMessage(`[user_prompt] ${prompt}`);

  execFile(bin, ['log', 'user_prompt', logId, message], { timeout: 5000, cwd }, (err) => {
    if (err) {
      process.stderr.write(`[log-user-prompt] qsv log failed: ${err.message}\n`);
    }
  });
});
