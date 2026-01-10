#!/usr/bin/env node

/**
 * Cross-platform test runner that handles glob expansion for all Node.js versions
 * Node 20's --test flag doesn't handle globs as well as 22+, so we expand them first
 */

import { spawn } from 'child_process';
import { readdir } from 'fs/promises';
import { join } from 'path';
import { fileURLToPath } from 'url';
import { dirname } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

async function findTestFiles() {
  const testsDir = join(__dirname, '../dist/tests');
  const files = await readdir(testsDir);
  return files
    .filter(f => f.endsWith('.test.js'))
    .map(f => join(testsDir, f));
}

async function runTests() {
  const testFiles = await findTestFiles();

  if (testFiles.length === 0) {
    console.error('No test files found in dist/tests/');
    process.exit(1);
  }

  console.log(`Running ${testFiles.length} test files...\n`);

  const nodeArgs = ['--test', ...testFiles];
  const child = spawn('node', nodeArgs, {
    stdio: 'inherit',
    shell: false
  });

  child.on('exit', (code) => {
    process.exit(code || 0);
  });

  child.on('error', (err) => {
    console.error('Failed to run tests:', err);
    process.exit(1);
  });
}

runTests().catch(err => {
  console.error('Error:', err);
  process.exit(1);
});
