#!/usr/bin/env node

// cowork-setup.js — SessionStart hook
// Copies a qsv CLAUDE.md template into the Cowork working folder
// if one doesn't already exist.
// Node.js port of cowork-setup.sh for Windows compatibility.

import { existsSync, copyFileSync, realpathSync } from 'node:fs';
import { resolve, normalize, join } from 'node:path';

function output(additionalContext) {
  process.stdout.write(JSON.stringify({ additionalContext }) + '\n');
}

async function main() {
  // Allow opting out via environment variable
  if (process.env.QSV_NO_COWORK_SETUP === '1') {
    process.exit(0);
  }

  // Read hook input JSON from stdin (limit to 64KB; timeout after 5s to avoid hangs).
  // Destroying stdin on timeout cleanly breaks the for-await loop instead of calling process.exit().
  let input = '';
  const timeoutId = setTimeout(() => {
    process.stderr.write('cowork-setup: stdin read timed out after 5s\n');
    process.stdin.destroy();
  }, 5000);
  try {
    for await (const chunk of process.stdin) {
      input += chunk.toString();
      if (input.length > 65536) {
        process.stderr.write('cowork-setup: stdin exceeded 64KB limit, truncating\n');
        process.stdin.destroy();
        break;
      }
    }
  } catch (err) {
    // stdin was destroyed by timeout or limit — continue with whatever was read.
    // Log unexpected errors to stderr to aid debugging.
    if (err?.code && err.code !== 'ERR_USE_AFTER_CLOSE' && err.code !== 'ERR_STREAM_PREMATURE_CLOSE') {
      process.stderr.write(`cowork-setup: stdin error: ${err.code}\n`);
    }
  }
  clearTimeout(timeoutId);

  let cwd = '';
  try {
    const parsed = JSON.parse(input);
    cwd = parsed.cwd || '';
  } catch {
    // Invalid or empty JSON — warn and exit (skip warning for empty stdin)
    if (input.trim()) {
      process.stderr.write('cowork-setup: failed to parse stdin as JSON\n');
    }
    process.exit(0);
  }

  if (!cwd) {
    process.exit(0);
  }

  // Resolve to real path to prevent path traversal via symlinks
  try {
    cwd = realpathSync(resolve(cwd));
  } catch {
    // Directory doesn't exist or isn't accessible
    process.exit(0);
  }

  // Guard against deploying into the plugin's own directory tree
  const pluginRoot = process.env.CLAUDE_PLUGIN_ROOT;
  if (!pluginRoot) {
    output('CLAUDE_PLUGIN_ROOT is not set. Skipping qsv CLAUDE.md deployment.');
    process.exit(0);
  }

  let resolvedPluginRoot;
  try {
    resolvedPluginRoot = realpathSync(resolve(pluginRoot));
  } catch {
    process.exit(0);
  }

  // Path prefix check — case-insensitive on Windows and macOS.
  // Note: macOS can have case-sensitive APFS volumes, but the default is case-insensitive.
  // On a case-sensitive macOS volume this guard could theoretically be bypassed with different
  // casing, but the consequence is only writing a CLAUDE.md into the plugin's own tree.
  const normalizedCwd = normalize(cwd);
  const normalizedRoot = normalize(resolvedPluginRoot);
  const caseInsensitive = process.platform === 'win32' || process.platform === 'darwin';
  const cwdForCompare = (caseInsensitive ? normalizedCwd.toLowerCase() : normalizedCwd).replace(/[\\/]+$/, '');
  const rootForCompare = (caseInsensitive ? normalizedRoot.toLowerCase() : normalizedRoot).replace(/[\\/]+$/, '');
  const separator = process.platform === 'win32' ? '\\' : '/';

  if (cwdForCompare === rootForCompare || cwdForCompare.startsWith(rootForCompare + separator)) {
    process.exit(0);
  }

  const template = join(resolvedPluginRoot, 'cowork-CLAUDE.md');
  const target = join(cwd, 'CLAUDE.md');

  // Ensure the template exists
  if (!existsSync(template)) {
    process.exit(0);
  }

  if (existsSync(target)) {
    // Existing CLAUDE.md — don't overwrite
    output(`An existing CLAUDE.md was found at ${target}. It was NOT overwritten. The existing file will be used for workflow guidance.`);
  } else {
    // Copy the template
    try {
      copyFileSync(template, target);
      output(`A qsv CLAUDE.md was created at ${target}. qsv workflow guidance has been set up in the working folder.`);
    } catch {
      output(`Could not create CLAUDE.md in ${cwd} (directory may not be writable). Skipping qsv workflow guidance setup.`);
    }
  }
}

main().catch(() => {
  // Never block session start
  process.exit(0);
});
