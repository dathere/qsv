#!/usr/bin/env node

// log-session-end.cjs — SessionEnd hook
// Parses the session transcript and writes a human-readable session summary
// to {cwd}/.qsv-session-log.md (append mode) and a final log entry to qsvmcp.log.
// This is critical in Cowork where the container (and transcript) is destroyed on exit.
// Uses CommonJS so it works standalone without package.json declaring "type": "module".

const { execFileSync } = require('node:child_process');
const { randomUUID } = require('node:crypto');
const { readFileSync, appendFileSync, existsSync, writeFileSync, unlinkSync } = require('node:fs');
const { join } = require('node:path');
const { findQsvMcpBinary, truncateMessage, readStdin } = require('./qsv-utils.cjs');

/**
 * Parse a JSONL transcript file and extract tool usage stats.
 * Returns { toolCounts, totalTurns, firstTimestamp, lastTimestamp }.
 */
function parseTranscript(transcriptPath) {
  const toolCounts = {};
  let totalTurns = 0;
  let firstTimestamp = null;
  let lastTimestamp = null;

  if (!transcriptPath || !existsSync(transcriptPath)) {
    return { toolCounts, totalTurns, firstTimestamp, lastTimestamp };
  }

  let content;
  try {
    content = readFileSync(transcriptPath, 'utf-8');
  } catch {
    return { toolCounts, totalTurns, firstTimestamp, lastTimestamp };
  }

  const lines = content.split('\n').filter(Boolean);
  for (const line of lines) {
    let entry;
    try {
      entry = JSON.parse(line);
    } catch {
      continue;
    }

    // Track timestamps
    const ts = entry.timestamp || entry.ts;
    if (ts) {
      if (!firstTimestamp) firstTimestamp = ts;
      lastTimestamp = ts;
    }

    // Count assistant turns
    if (entry.role === 'assistant' || entry.type === 'assistant') {
      totalTurns++;
    }

    // Count tool usage — look for tool_use content blocks
    const contentBlocks = entry.content || entry.message?.content;
    if (Array.isArray(contentBlocks)) {
      for (const block of contentBlocks) {
        if (block.type === 'tool_use' && block.name) {
          toolCounts[block.name] = (toolCounts[block.name] || 0) + 1;
        }
      }
    }
  }

  return { toolCounts, totalTurns, firstTimestamp, lastTimestamp };
}

/**
 * Format duration between two ISO timestamps as a human-readable string.
 */
function formatDuration(startTs, endTs) {
  if (!startTs || !endTs) return 'unknown';
  try {
    const ms = new Date(endTs).getTime() - new Date(startTs).getTime();
    if (ms < 0 || isNaN(ms)) return 'unknown';
    const seconds = Math.floor(ms / 1000);
    const minutes = Math.floor(seconds / 60);
    const hours = Math.floor(minutes / 60);
    if (hours > 0) return `${hours}h ${minutes % 60}m`;
    if (minutes > 0) return `${minutes}m ${seconds % 60}s`;
    return `${seconds}s`;
  } catch {
    return 'unknown';
  }
}

/**
 * Build a markdown summary string from parsed transcript stats.
 */
function buildSummary(sessionId, stats) {
  const now = new Date().toISOString();
  const duration = formatDuration(stats.firstTimestamp, stats.lastTimestamp);
  const totalToolCalls = Object.values(stats.toolCounts).reduce((a, b) => a + b, 0);

  const lines = [
    `## Session ${sessionId || 'unknown'}`,
    `- **Date**: ${now}`,
    `- **Duration**: ${duration}`,
    `- **Assistant turns**: ${stats.totalTurns}`,
    `- **Total tool calls**: ${totalToolCalls}`,
  ];

  if (totalToolCalls > 0) {
    lines.push('- **Tool usage**:');
    // Sort by count descending
    const sorted = Object.entries(stats.toolCounts).sort((a, b) => b[1] - a[1]);
    for (const [tool, count] of sorted) {
      lines.push(`  - \`${tool}\`: ${count}`);
    }
  }

  lines.push(''); // trailing newline
  return lines.join('\n');
}

/**
 * Consolidate .qsv-pipeline-steps.jsonl into pipeline.json + pipeline.sh.
 * This is the crash-recovery path: if the MCP server finalized cleanly,
 * the JSONL file will already be deleted and this is a no-op.
 *
 * Note: The replay script generation logic mirrors PipelineManifest.generateReplayScript()
 * in src/pipeline-manifest.ts. Changes to the script format should be applied in both places.
 */
function consolidatePipelineManifest(cwd, sessionId) {
  const jsonlPath = join(cwd, '.qsv-pipeline-steps.jsonl');
  if (!existsSync(jsonlPath)) return; // MCP server finalized cleanly — nothing to do

  let content;
  try {
    content = readFileSync(jsonlPath, 'utf-8');
  } catch {
    return;
  }

  // Parse JSONL: separate pipeline steps from web_source provenance entries
  const steps = [];
  const allEntries = [];
  for (const line of content.split('\n').filter(Boolean)) {
    try {
      allEntries.push(JSON.parse(line));
    } catch {
      continue;
    }
  }

  // Attach web_source URLs to the next chronological pipeline step
  let pendingWebSources = [];
  for (const entry of allEntries) {
    if (entry.type === 'web_source') {
      pendingWebSources.push(entry.url);
      continue;
    }
    // It's a pipeline step
    if (pendingWebSources.length > 0) {
      entry.web_sources = [...(entry.web_sources || []), ...pendingWebSources];
      pendingWebSources = [];
    }
    steps.push(entry);
  }

  if (steps.length === 0) {
    try { unlinkSync(jsonlPath); } catch { /* ignore */ }
    return;
  }

  // Build file inventory
  const inventory = {};
  for (const step of steps) {
    if (step.input && step.input.file && !inventory[step.input.file]) {
      inventory[step.input.file] = {
        blake3: step.input.blake3,
        size_bytes: step.input.size_bytes,
        first_seen_step: step.step,
        role: 'input',
      };
    }
    if (step.output && step.output.file) {
      if (!inventory[step.output.file]) {
        inventory[step.output.file] = {
          blake3: step.output.blake3,
          size_bytes: step.output.size_bytes,
          first_seen_step: step.step,
          role: 'output',
        };
      }
      if (step.input && step.input.file && inventory[step.input.file] && inventory[step.input.file].role === 'output') {
        inventory[step.input.file].role = 'intermediate';
      }
    }
    for (const ai of (step.additional_inputs || [])) {
      if (!inventory[ai.file]) {
        inventory[ai.file] = {
          blake3: ai.blake3,
          size_bytes: ai.size_bytes,
          first_seen_step: step.step,
          role: 'input',
        };
      }
    }
  }

  const firstTs = steps[0].timestamp || new Date().toISOString();
  const lastTs = steps[steps.length - 1].timestamp || new Date().toISOString();

  const manifest = {
    version: '1.0.0',
    session: {
      id: sessionId,
      started_at: firstTs,
      ended_at: lastTs,
      qsv_version: 'unknown',
      mcp_server_version: 'unknown',
      working_directory: cwd,
    },
    steps,
    file_inventory: inventory,
  };

  // Write pipeline.json
  const jsonPath = join(cwd, 'pipeline.json');
  try {
    writeFileSync(jsonPath, JSON.stringify(manifest, null, 2) + '\n', 'utf-8');
    process.stderr.write(`[log-session-end] Wrote crash-recovery ${jsonPath}\n`);
  } catch (err) {
    process.stderr.write(`[log-session-end] Failed to write pipeline.json: ${err.message}\n`);
    return;
  }

  // Write pipeline.sh (transformative steps only)
  const shLines = [
    '#!/usr/bin/env bash',
    'set -euo pipefail',
    '',
    '# Pipeline replay script generated by qsv MCP Server (crash-recovery)',
    `# Session: ${sessionId}`,
    `# Date: ${new Date().toISOString()}`,
    '',
  ];
  let hasTransformSteps = false;
  for (const step of steps) {
    if (step.kind !== 'transformative' || !step.command) continue;
    hasTransformSteps = true;
    if (!step.success) {
      shLines.push(`# SKIPPED (failed): ${step.command}`);
      shLines.push('');
      continue;
    }
    if (!step.deterministic) {
      shLines.push('# WARNING: non-deterministic — output may differ');
    }
    if (step.reason) {
      shLines.push(`# ${step.reason}`);
    }
    shLines.push(step.command);
    shLines.push('');
  }
  if (!hasTransformSteps) {
    shLines.push('# No transformative steps recorded.');
    shLines.push('');
  }

  const shPath = join(cwd, 'pipeline.sh');
  try {
    writeFileSync(shPath, shLines.join('\n'), { encoding: 'utf-8', mode: 0o755 });
  } catch (err) {
    process.stderr.write(`[log-session-end] Failed to write pipeline.sh: ${err.message}\n`);
  }

  // Clean up JSONL
  try { unlinkSync(jsonlPath); } catch { /* ignore */ }
}

// Export for testing
module.exports = { parseTranscript, formatDuration, buildSummary, consolidatePipelineManifest };

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

    const sessionId = parsed.session_id || 'unknown';
    const transcriptPath = parsed.transcript_path || '';
    const cwd = parsed.cwd || process.cwd();

    // Parse transcript for tool usage stats
    const stats = parseTranscript(transcriptPath);
    const summary = buildSummary(sessionId, stats);

    // 1. Append human-readable summary to .qsv-session-log.md
    const logFile = join(cwd, '.qsv-session-log.md');
    try {
      const header = existsSync(logFile) ? '\n---\n\n' : '# qsv Session Log\n\n';
      appendFileSync(logFile, header + summary, 'utf-8');
    } catch (err) {
      process.stderr.write(`[log-session-end] Could not write ${logFile}: ${err.message}\n`);
    }

    // 2. Log a compact summary to qsvmcp.log
    const bin = findQsvMcpBinary();
    if (!bin) {
      process.stderr.write('[log-session-end] qsvmcp binary not found\n');
      return;
    }

    const totalToolCalls = Object.values(stats.toolCounts).reduce((a, b) => a + b, 0);
    let logMessage = `[session_end] session=${sessionId} turns=${stats.totalTurns} tool_calls=${totalToolCalls}`;

    // Add top 5 tools by usage
    if (totalToolCalls > 0) {
      const top = Object.entries(stats.toolCounts)
        .sort((a, b) => b[1] - a[1])
        .slice(0, 5)
        .map(([name, count]) => `${name}:${count}`)
        .join(',');
      logMessage += ` top_tools=${top}`;
    }

    // Truncate AFTER building the full message
    logMessage = truncateMessage(logMessage);

    const logId = `s-${randomUUID()}`;

    // Use sync here — SessionEnd has limited time before container teardown
    try {
      execFileSync(bin, ['log', 'session_end', logId, logMessage], { timeout: 5000, cwd });
    } catch (err) {
      process.stderr.write(`[log-session-end] qsv log failed: ${err.message}\n`);
    }

    // 3. Consolidate pipeline manifest if MCP server didn't finalize cleanly
    consolidatePipelineManifest(cwd, sessionId);
  });
}
