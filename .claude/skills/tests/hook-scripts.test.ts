/**
 * Unit tests for hook script utilities:
 * - qsv-utils.cjs (truncateMessage)
 * - log-session-end.cjs (parseTranscript, formatDuration, buildSummary)
 */

import { test } from 'node:test';
import assert from 'node:assert';
import { writeFileSync, mkdirSync, rmSync } from 'node:fs';
import { join, resolve, dirname } from 'node:path';
import { tmpdir } from 'node:os';
import { createRequire } from 'node:module';
import { fileURLToPath } from 'node:url';

const require = createRequire(import.meta.url);

// Resolve paths relative to the project root (not dist/tests/)
const __filename = fileURLToPath(import.meta.url);
const projectRoot = resolve(dirname(__filename), '..', '..');
const { findQsvMcpBinary, truncateMessage, MAX_LOG_MESSAGE_LEN } = require(resolve(projectRoot, 'scripts', 'qsv-utils.cjs'));
const { parseTranscript, formatDuration, buildSummary } = require(resolve(projectRoot, 'scripts', 'log-session-end.cjs'));
const { sanitizeUrl, buildWebLogMessage } = require(resolve(projectRoot, 'scripts', 'log-web-results.cjs'));

// --- findQsvMcpBinary tests ---

test('findQsvMcpBinary returns QSV_MCP_BIN_PATH when set', () => {
  const original = process.env.QSV_MCP_BIN_PATH;
  try {
    process.env.QSV_MCP_BIN_PATH = '/fake/path/to/qsvmcp';
    assert.strictEqual(findQsvMcpBinary(), '/fake/path/to/qsvmcp');
  } finally {
    if (original === undefined) {
      delete process.env.QSV_MCP_BIN_PATH;
    } else {
      process.env.QSV_MCP_BIN_PATH = original;
    }
  }
});

test('findQsvMcpBinary returns null when env unset and binary not on PATH', () => {
  const original = process.env.QSV_MCP_BIN_PATH;
  const originalPath = process.env.PATH;
  try {
    delete process.env.QSV_MCP_BIN_PATH;
    // Set PATH to empty so which/where won't find qsvmcp
    process.env.PATH = '';
    assert.strictEqual(findQsvMcpBinary(), null);
  } finally {
    if (original !== undefined) {
      process.env.QSV_MCP_BIN_PATH = original;
    }
    if (originalPath === undefined) {
      delete process.env.PATH;
    } else {
      process.env.PATH = originalPath;
    }
  }
});

// --- truncateMessage tests ---

test('truncateMessage returns short messages unchanged', () => {
  const msg = 'hello world';
  assert.strictEqual(truncateMessage(msg), msg);
});

test('truncateMessage truncates at MAX_LOG_MESSAGE_LEN', () => {
  const msg = 'a'.repeat(MAX_LOG_MESSAGE_LEN + 100);
  const result = truncateMessage(msg);
  assert.strictEqual(Array.from(result).length, MAX_LOG_MESSAGE_LEN);
});

test('truncateMessage handles Unicode correctly', () => {
  // Each emoji is 1 code point but multiple UTF-16 code units
  const emoji = '\u{1F600}'; // 😀
  const msg = emoji.repeat(MAX_LOG_MESSAGE_LEN + 10);
  const result = truncateMessage(msg);
  assert.strictEqual(Array.from(result).length, MAX_LOG_MESSAGE_LEN);
  // Ensure no broken surrogate pairs
  assert.ok(!result.includes('\uFFFD'));
});

test('truncateMessage returns exact-length messages unchanged', () => {
  const msg = 'x'.repeat(MAX_LOG_MESSAGE_LEN);
  assert.strictEqual(truncateMessage(msg), msg);
});

// --- formatDuration tests ---

test('formatDuration returns unknown for missing timestamps', () => {
  assert.strictEqual(formatDuration(null, null), 'unknown');
  assert.strictEqual(formatDuration('2024-01-01T00:00:00Z', null), 'unknown');
  assert.strictEqual(formatDuration(null, '2024-01-01T00:00:00Z'), 'unknown');
});

test('formatDuration formats seconds correctly', () => {
  const start = '2024-01-01T00:00:00Z';
  const end = '2024-01-01T00:00:45Z';
  assert.strictEqual(formatDuration(start, end), '45s');
});

test('formatDuration formats minutes and seconds', () => {
  const start = '2024-01-01T00:00:00Z';
  const end = '2024-01-01T00:05:30Z';
  assert.strictEqual(formatDuration(start, end), '5m 30s');
});

test('formatDuration formats hours and minutes', () => {
  const start = '2024-01-01T00:00:00Z';
  const end = '2024-01-01T02:15:00Z';
  assert.strictEqual(formatDuration(start, end), '2h 15m');
});

test('formatDuration returns unknown for negative duration', () => {
  const start = '2024-01-01T01:00:00Z';
  const end = '2024-01-01T00:00:00Z';
  assert.strictEqual(formatDuration(start, end), 'unknown');
});

test('formatDuration returns 0s for zero duration', () => {
  const ts = '2024-01-01T00:00:00Z';
  assert.strictEqual(formatDuration(ts, ts), '0s');
});

// --- parseTranscript tests ---

test('parseTranscript returns empty stats for missing file', () => {
  const stats = parseTranscript('/nonexistent/path.jsonl');
  assert.deepStrictEqual(stats.toolCounts, {});
  assert.strictEqual(stats.totalTurns, 0);
  assert.strictEqual(stats.firstTimestamp, null);
  assert.strictEqual(stats.lastTimestamp, null);
});

test('parseTranscript returns empty stats for null path', () => {
  const stats = parseTranscript(null);
  assert.deepStrictEqual(stats.toolCounts, {});
  assert.strictEqual(stats.totalTurns, 0);
});

test('parseTranscript counts tool usage correctly', () => {
  const dir = join(tmpdir(), `qsv-test-transcript-${Date.now()}`);
  mkdirSync(dir, { recursive: true });
  const transcriptPath = join(dir, 'transcript.jsonl');

  try {
    const lines = [
      JSON.stringify({
        timestamp: '2024-01-01T00:00:00Z',
        role: 'assistant',
        content: [
          { type: 'tool_use', name: 'Read' },
          { type: 'tool_use', name: 'Bash' },
        ],
      }),
      JSON.stringify({
        timestamp: '2024-01-01T00:01:00Z',
        role: 'assistant',
        content: [
          { type: 'tool_use', name: 'Read' },
          { type: 'text', text: 'some response' },
        ],
      }),
      JSON.stringify({
        timestamp: '2024-01-01T00:02:00Z',
        role: 'user',
        content: [{ type: 'text', text: 'user message' }],
      }),
    ];
    writeFileSync(transcriptPath, lines.join('\n'), 'utf-8');

    const stats = parseTranscript(transcriptPath);
    assert.strictEqual(stats.toolCounts['Read'], 2);
    assert.strictEqual(stats.toolCounts['Bash'], 1);
    assert.strictEqual(stats.totalTurns, 2);
    assert.strictEqual(stats.firstTimestamp, '2024-01-01T00:00:00Z');
    assert.strictEqual(stats.lastTimestamp, '2024-01-01T00:02:00Z');
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

test('parseTranscript handles malformed JSON lines gracefully', () => {
  const dir = join(tmpdir(), `qsv-test-transcript-malformed-${Date.now()}`);
  mkdirSync(dir, { recursive: true });
  const transcriptPath = join(dir, 'transcript.jsonl');

  try {
    const lines = [
      'not valid json',
      JSON.stringify({
        timestamp: '2024-01-01T00:00:00Z',
        role: 'assistant',
        content: [{ type: 'tool_use', name: 'Edit' }],
      }),
      '{ broken json',
    ];
    writeFileSync(transcriptPath, lines.join('\n'), 'utf-8');

    const stats = parseTranscript(transcriptPath);
    assert.strictEqual(stats.toolCounts['Edit'], 1);
    assert.strictEqual(stats.totalTurns, 1);
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

test('parseTranscript handles message.content nesting', () => {
  const dir = join(tmpdir(), `qsv-test-transcript-nested-${Date.now()}`);
  mkdirSync(dir, { recursive: true });
  const transcriptPath = join(dir, 'transcript.jsonl');

  try {
    const lines = [
      JSON.stringify({
        timestamp: '2024-01-01T00:00:00Z',
        type: 'assistant',
        message: {
          content: [
            { type: 'tool_use', name: 'mcp__qsv__qsv_stats' },
            { type: 'tool_use', name: 'mcp__qsv__qsv_sqlp' },
          ],
        },
      }),
    ];
    writeFileSync(transcriptPath, lines.join('\n'), 'utf-8');

    const stats = parseTranscript(transcriptPath);
    assert.strictEqual(stats.toolCounts['mcp__qsv__qsv_stats'], 1);
    assert.strictEqual(stats.toolCounts['mcp__qsv__qsv_sqlp'], 1);
    assert.strictEqual(stats.totalTurns, 1);
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

// --- buildSummary tests ---

test('buildSummary includes session ID and tool counts', () => {
  const stats = {
    toolCounts: { Read: 5, Bash: 3, Edit: 1 },
    totalTurns: 4,
    firstTimestamp: '2024-01-01T00:00:00Z',
    lastTimestamp: '2024-01-01T00:10:00Z',
  };

  const summary = buildSummary('test-session-123', stats);
  assert.ok(summary.includes('## Session test-session-123'));
  assert.ok(summary.includes('**Duration**: 10m 0s'));
  assert.ok(summary.includes('**Assistant turns**: 4'));
  assert.ok(summary.includes('**Total tool calls**: 9'));
  assert.ok(summary.includes('`Read`: 5'));
  assert.ok(summary.includes('`Bash`: 3'));
  assert.ok(summary.includes('`Edit`: 1'));
});

test('buildSummary handles zero tool calls', () => {
  const stats = {
    toolCounts: {},
    totalTurns: 1,
    firstTimestamp: '2024-01-01T00:00:00Z',
    lastTimestamp: '2024-01-01T00:00:30Z',
  };

  const summary = buildSummary('empty-session', stats);
  assert.ok(summary.includes('**Total tool calls**: 0'));
  assert.ok(!summary.includes('**Tool usage**:'));
});

test('buildSummary sorts tools by count descending', () => {
  const stats = {
    toolCounts: { Edit: 1, Read: 10, Bash: 5 },
    totalTurns: 3,
    firstTimestamp: null,
    lastTimestamp: null,
  };

  const summary = buildSummary('sort-test', stats);
  const readIdx = summary.indexOf('`Read`: 10');
  const bashIdx = summary.indexOf('`Bash`: 5');
  const editIdx = summary.indexOf('`Edit`: 1');
  assert.ok(readIdx < bashIdx, 'Read should appear before Bash');
  assert.ok(bashIdx < editIdx, 'Bash should appear before Edit');
});

test('buildSummary uses unknown for missing session ID', () => {
  const stats = {
    toolCounts: {},
    totalTurns: 0,
    firstTimestamp: null,
    lastTimestamp: null,
  };

  const summary = buildSummary(null, stats);
  assert.ok(summary.includes('## Session unknown'));
});

// --- sanitizeUrl tests ---

test('sanitizeUrl strips embedded credentials', () => {
  const result = sanitizeUrl('https://user:pass@example.com/path');
  assert.strictEqual(result, 'https://example.com/path');
});

test('sanitizeUrl redacts sensitive query parameters', () => {
  const result = sanitizeUrl('https://api.example.com/search?q=hello&api_key=abc123&token=secret');
  assert.ok(result.includes('q=hello'));
  assert.ok(result.includes('api_key=***REDACTED***'));
  assert.ok(result.includes('token=***REDACTED***'));
  assert.ok(!result.includes('abc123'));
  assert.ok(!result.includes('secret'));
});

test('sanitizeUrl preserves normal URLs unchanged', () => {
  const url = 'https://docs.example.com/api/v2/guide?page=3';
  const result = sanitizeUrl(url);
  assert.strictEqual(result, url);
});

test('sanitizeUrl returns invalid URLs as-is', () => {
  const notAUrl = 'not-a-url';
  assert.strictEqual(sanitizeUrl(notAUrl), notAUrl);
});

test('sanitizeUrl strips fragments that may carry tokens', () => {
  const result = sanitizeUrl('https://example.com/callback#access_token=abc123&token_type=bearer');
  assert.ok(!result.includes('#'));
  assert.ok(!result.includes('abc123'));
  assert.ok(!result.includes('token_type'));
});

test('sanitizeUrl is case-insensitive for parameter names', () => {
  const result = sanitizeUrl('https://example.com/?API_KEY=secret&Password=hunter2');
  assert.ok(result.includes('API_KEY=***REDACTED***'));
  assert.ok(result.includes('Password=***REDACTED***'));
});

// --- buildWebLogMessage tests ---

test('buildWebLogMessage builds WebSearch message', () => {
  const result = buildWebLogMessage('WebSearch', { query: 'rust csv parser' }, 'some results');
  assert.ok(result !== null);
  assert.strictEqual(result.logCategory, 'web_search');
  assert.ok(result.message.includes('[web_search]'));
  assert.ok(result.message.includes('query="rust csv parser"'));
  assert.ok(result.message.includes('results=some results'));
});

test('buildWebLogMessage builds WebFetch message with sanitized URL', () => {
  const result = buildWebLogMessage('WebFetch', { url: 'https://user:pass@example.com/doc?token=abc' }, 'page content');
  assert.ok(result !== null);
  assert.strictEqual(result.logCategory, 'web_fetch');
  assert.ok(result.message.includes('[web_fetch]'));
  assert.ok(!result.message.includes('user:pass'));
  assert.ok(result.message.includes('token=***REDACTED***'));
  assert.ok(result.message.includes('content=page content'));
});

test('buildWebLogMessage returns null for missing query', () => {
  assert.strictEqual(buildWebLogMessage('WebSearch', {}, 'results'), null);
});

test('buildWebLogMessage returns null for missing URL', () => {
  assert.strictEqual(buildWebLogMessage('WebFetch', {}, 'content'), null);
});

test('buildWebLogMessage returns null for unknown tool', () => {
  assert.strictEqual(buildWebLogMessage('Read', { file_path: '/tmp/x' }, 'data'), null);
});

test('buildWebLogMessage handles object tool_result', () => {
  const result = buildWebLogMessage('WebSearch', { query: 'test' }, { items: [1, 2, 3] });
  assert.ok(result !== null);
  assert.ok(result.message.includes('{"items":[1,2,3]}'));
});

test('buildWebLogMessage uses search_query fallback', () => {
  const result = buildWebLogMessage('WebSearch', { search_query: 'fallback query' }, 'results');
  assert.ok(result !== null);
  assert.ok(result.message.includes('query="fallback query"'));
});
