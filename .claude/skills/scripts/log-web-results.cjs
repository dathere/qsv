#!/usr/bin/env node

// log-web-results.cjs — PostToolUse hook for WebSearch and WebFetch
// Logs web search queries/results and fetched URLs/content to qsvmcp.log
// for citation and reproducibility.
// Uses CommonJS so it works standalone without package.json declaring "type": "module".

const { execFile } = require('node:child_process');
const { randomUUID } = require('node:crypto');
const { findQsvMcpBinary, truncateMessage, readStdin } = require('./qsv-utils.cjs');

/**
 * Sanitize a URL for logging — strip userinfo (user:pass@) and redact
 * query parameters that look like secrets (key, token, secret, password, auth, api_key).
 * Keeps the rest of the URL intact for citation/reproducibility.
 */
function sanitizeUrl(rawUrl) {
  try {
    const u = new URL(rawUrl);
    // Strip embedded credentials
    u.username = '';
    u.password = '';
    // Redact sensitive query parameters
    const sensitivePattern = /^(key|token|secret|password|passwd|auth|api_key|apikey|access_token|client_secret)$/i;
    for (const [param] of [...u.searchParams]) {
      if (sensitivePattern.test(param)) {
        u.searchParams.set(param, '***REDACTED***');
      }
    }
    return u.toString();
  } catch {
    // Not a valid URL — return as-is (let truncateMessage handle length)
    return rawUrl;
  }
}

/**
 * Build a log message for a WebSearch or WebFetch PostToolUse event.
 * Returns { logCategory, message } or null if the input is not loggable.
 */
function buildWebLogMessage(toolName, toolInput, toolResult) {
  if (toolName === 'WebSearch') {
    const query = toolInput.query || toolInput.search_query || '';
    if (!query) return null;
    const resultText = typeof toolResult === 'string' ? toolResult : JSON.stringify(toolResult);
    return {
      logCategory: 'web_search',
      message: truncateMessage(`[web_search] query="${query}" results=${resultText}`),
    };
  } else if (toolName === 'WebFetch') {
    const url = toolInput.url || '';
    if (!url) return null;
    const safeUrl = sanitizeUrl(url);
    const resultText = typeof toolResult === 'string' ? toolResult : JSON.stringify(toolResult);
    return {
      logCategory: 'web_fetch',
      message: truncateMessage(`[web_fetch] url="${safeUrl}" content=${resultText}`),
    };
  }
  return null;
}

// Export for testing
module.exports = { sanitizeUrl, buildWebLogMessage };

// Skip main logic when loaded via require() for testing
if (require.main === module) {
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

    const result = buildWebLogMessage(toolName, toolInput, toolResult);
    if (!result) return;

    const logId = `w-${randomUUID()}`;

    execFile(bin, ['log', result.logCategory, logId, result.message], { timeout: 5000, cwd }, (err) => {
      if (err) {
        process.stderr.write(`[log-web-results] qsv log failed: ${err.message}\n`);
      }
    });
  });
}
