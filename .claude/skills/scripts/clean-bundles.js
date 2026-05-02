#!/usr/bin/env node
// Prune historical .mcpb / .plugin bundles, keeping only the current version
// (read from package.json). Run from .claude/skills/ as `npm run clean:bundles`.

import { readdirSync, readFileSync, statSync, unlinkSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const rootDir = join(__dirname, '..');

const { version } = JSON.parse(readFileSync(join(rootDir, 'package.json'), 'utf-8'));
const keep = new Set([
  `qsv-mcp-server-${version}.mcpb`,
  `qsv-data-wrangling-${version}.plugin`,
]);

const candidates = readdirSync(rootDir).filter(
  (f) => f.endsWith('.mcpb') || f.endsWith('.plugin'),
);

const presentKeep = [...keep].filter((f) => candidates.includes(f));
if (presentKeep.length === 0 && candidates.length > 0) {
  console.error(
    `Refusing to clean: no v${version} bundle present yet — would delete every historical bundle.\n` +
      `  Build the current version first (npm run mcpb:package / plugin:package), then re-run.`,
  );
  process.exit(1);
}

let removed = 0;
let removedBytes = 0;
let failed = 0;
for (const f of candidates) {
  if (keep.has(f)) continue;
  const p = join(rootDir, f);
  try {
    const size = statSync(p).size;
    unlinkSync(p);
    removedBytes += size;
    removed += 1;
    console.log(`  removed ${f} (${(size / 1024 / 1024).toFixed(1)} MB)`);
  } catch (err) {
    failed += 1;
    console.error(`  skipped ${f}: ${err.message}`);
  }
}

console.log(
  removed === 0 && failed === 0
    ? 'No old bundles to remove.'
    : `Removed ${removed} bundle(s), freed ${(removedBytes / 1024 / 1024).toFixed(1)} MB${failed ? ` (${failed} skipped)` : ''}.`,
);
console.log(`Kept: ${presentKeep.join(', ') || '(none)'}`);
