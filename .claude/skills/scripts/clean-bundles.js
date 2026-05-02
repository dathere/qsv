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

let removed = 0;
let removedBytes = 0;
for (const f of candidates) {
  if (keep.has(f)) continue;
  const p = join(rootDir, f);
  const size = statSync(p).size;
  unlinkSync(p);
  removedBytes += size;
  removed += 1;
  console.log(`  removed ${f} (${(size / 1024 / 1024).toFixed(1)} MB)`);
}

console.log(
  removed === 0
    ? 'No old bundles to remove.'
    : `Removed ${removed} bundle(s), freed ${(removedBytes / 1024 / 1024).toFixed(1)} MB.`,
);
console.log(`Kept: ${[...keep].filter((f) => candidates.includes(f)).join(', ') || '(none — current version not built yet)'}`);
