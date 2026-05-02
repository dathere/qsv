#!/usr/bin/env node

// log-user-prompt.cjs — UserPromptSubmit hook
// Logs the user's prompt to the qsv audit log (qsvmcp.log) for reproducibility.
// Uses CommonJS so it works standalone without package.json declaring "type": "module".

const { execFile } = require('node:child_process');
const { randomUUID } = require('node:crypto');
const { findQsvMcpBinary, truncateMessage, readStdin } = require('./qsv-utils.cjs');

// Hard wall-clock cap: this hook runs on every UserPromptSubmit, so a hung
// binary lookup or stalled stdin must never block the user's next turn.
// Track any spawned child so we can kill it before exit — otherwise the
// process can be orphaned past execFile's own `timeout`.
const HOOK_HARD_TIMEOUT_MS = 7_000;
let activeChild = null;
const hardTimer = setTimeout(() => {
  if (activeChild && !activeChild.killed) {
    try { activeChild.kill('SIGKILL'); } catch { /* already gone */ }
  }
  process.exit(0);
}, HOOK_HARD_TIMEOUT_MS);
hardTimer.unref();

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

  activeChild = execFile(bin, ['log', 'user_prompt', logId, message], { timeout: 5000, cwd }, (err) => {
    activeChild = null;
    if (err) {
      process.stderr.write(`[log-user-prompt] qsv log failed: ${err.message}\n`);
    }
  });
});
