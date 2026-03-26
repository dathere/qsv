/**
 * Unit tests for hook script utilities:
 * - qsv-utils.cjs (truncateMessage)
 * - log-session-end.cjs (parseTranscript, formatDuration, buildSummary)
 */

import { test } from 'node:test';
import assert from 'node:assert';
import { writeFileSync, mkdirSync, rmSync } from 'node:fs';
import { join } from 'node:path';
import { tmpdir } from 'node:os';
import { createRequire } from 'node:module';
import { resolve, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const require = createRequire(import.meta.url);

// Resolve paths relative to the project root (not dist/tests/)
const __filename = fileURLToPath(import.meta.url);
const projectRoot = resolve(dirname(__filename), '..', '..');
const { truncateMessage, MAX_LOG_MESSAGE_LEN } = require(resolve(projectRoot, 'scripts', 'qsv-utils.cjs'));
const { parseTranscript, formatDuration, buildSummary } = require(resolve(projectRoot, 'scripts', 'log-session-end.cjs'));

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
