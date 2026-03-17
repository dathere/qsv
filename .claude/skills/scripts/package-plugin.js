#!/usr/bin/env node
/**
 * Package qsv Cowork Plugin (.plugin file)
 *
 * Creates a lightweight ZIP containing ONLY the workflow layer:
 * - .claude-plugin/plugin.json (manifest)
 * - skills/ (domain knowledge SKILL.md files)
 * - commands/ (slash command .md files)
 * - agents/ (subagent .md files)
 * - scripts/cowork-setup.cjs (SessionStart hook script)
 * - cowork-CLAUDE.md (template deployed by hook)
 * - qsv-75x91.png (icon, if exists)
 *
 * The MCP server comes separately from the .mcpb Desktop Extension.
 */

import { createWriteStream, existsSync, readFileSync, rmSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';
import archiver from 'archiver';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const rootDir = join(__dirname, '..');

const packageJson = JSON.parse(readFileSync(join(rootDir, 'package.json'), 'utf-8'));
const version = packageJson.version;

const OUTPUT_FILE = `qsv-data-wrangling-${version}.plugin`;
const OUTPUT_PATH = join(rootDir, OUTPUT_FILE);

console.log('Cowork Plugin Packager');
console.log('='.repeat(50));
console.log(`Packaging version: ${version}`);

// Validate required files
const required = [
  '.claude-plugin/plugin.json',
  'skills/',
  'commands/',
  'agents/',
  'scripts/cowork-setup.cjs',
  'cowork-CLAUDE.md',
];

const missing = required.filter(f => !existsSync(join(rootDir, f)));
if (missing.length > 0) {
  console.error('Missing required files:');
  missing.forEach(f => console.error(`  - ${f}`));
  process.exit(1);
}
console.log('All required files present');

// Remove existing output
if (existsSync(OUTPUT_PATH)) {
  rmSync(OUTPUT_PATH);
  console.log(`Removed existing ${OUTPUT_FILE}`);
}

// Create archive
console.log('\nCreating .plugin archive...');

const output = createWriteStream(OUTPUT_PATH);
const archive = archiver('zip', { zlib: { level: 9 } });

output.on('close', () => {
  const sizeKB = (archive.pointer() / 1024).toFixed(1);
  console.log(`\nArchive created: ${OUTPUT_FILE} (${sizeKB} KB)`);
  console.log('='.repeat(50));
  console.log('\nNext Steps:');
  console.log(`  1. Install locally: bash scripts/install-cowork-plugin.sh ${OUTPUT_FILE}`);
  console.log('  2. Or drag .plugin file into a Cowork session to install');
  console.log('  3. Ensure .mcpb Desktop Extension is also installed (provides qsv MCP tools)');
  console.log('  4. Start a new Cowork session to activate');
  console.log('');
});

output.on('error', (err) => {
  console.error('Write failed:', err);
  process.exit(1);
});

archive.on('error', (err) => {
  console.error('Archive creation failed:', err);
  process.exit(1);
});

archive.pipe(output);

// Add plugin manifest
console.log('  Adding .claude-plugin/plugin.json...');
archive.file(join(rootDir, '.claude-plugin/plugin.json'), { name: '.claude-plugin/plugin.json' });

// Add domain knowledge skills
console.log('  Adding skills/...');
archive.glob('**/*', { cwd: join(rootDir, 'skills'), ignore: ['.*', '**/.*'] }, { prefix: 'skills' });

// Add slash commands
console.log('  Adding commands/...');
archive.glob('**/*', { cwd: join(rootDir, 'commands'), ignore: ['.*', '**/.*'] }, { prefix: 'commands' });

// Add subagents
console.log('  Adding agents/...');
archive.glob('**/*', { cwd: join(rootDir, 'agents'), ignore: ['.*', '**/.*'] }, { prefix: 'agents' });

// Add SessionStart hook script (only cowork-setup.cjs, not all scripts)
console.log('  Adding scripts/cowork-setup.cjs...');
archive.file(join(rootDir, 'scripts/cowork-setup.cjs'), { name: 'scripts/cowork-setup.cjs' });

// Add cowork CLAUDE.md template
console.log('  Adding cowork-CLAUDE.md...');
archive.file(join(rootDir, 'cowork-CLAUDE.md'), { name: 'cowork-CLAUDE.md' });

// Add icon if it exists
const iconPath = join(rootDir, 'qsv-75x91.png');
if (existsSync(iconPath)) {
  console.log('  Adding qsv-75x91.png...');
  archive.file(iconPath, { name: 'qsv-75x91.png' });
}

console.log('  Finalizing...');
archive.finalize();
