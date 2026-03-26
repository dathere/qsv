#!/usr/bin/env node

// log-web-results.cjs — PostToolUse hook for WebSearch and WebFetch
// Logs web search queries/results and fetched URLs/content to qsvmcp.log
// for citation and reproducibility.
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
    return;
  }

  const toolName = parsed.tool_name || '';
  const toolInput = parsed.tool_input || {};
  const toolResult = parsed.tool_result || '';
  const cwd = parsed.cwd || process.cwd();

  const bin = findQsvMcpBinary();
  if (!bin) {
    process.stderr.write('[log-web-results] qsvmcp binary not found\n');
    return;
  }

  let logCategory;
  let message;

  if (toolName === 'WebSearch') {
    const query = toolInput.query || toolInput.search_query || '';
    if (!query) return;
    logCategory = 'web_search';
    const resultText = typeof toolResult === 'string' ? toolResult : JSON.stringify(toolResult);
    message = truncateMessage(`[web_search] query="${query}" results=${resultText}`);
  } else if (toolName === 'WebFetch') {
    const url = toolInput.url || '';
    if (!url) return;
    logCategory = 'web_fetch';
    const resultText = typeof toolResult === 'string' ? toolResult : JSON.stringify(toolResult);
    message = truncateMessage(`[web_fetch] url="${url}" content=${resultText}`);
  } else {
    return;
  }

  const logId = `w-${randomUUID()}`;

  execFile(bin, ['log', logCategory, logId, message], { timeout: 5000, cwd }, (err) => {
    if (err) {
      process.stderr.write(`[log-web-results] qsv log failed: ${err.message}\n`);
    }
  });
});
