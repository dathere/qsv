#!/usr/bin/env node
/**
 * Package QSV MCP Server as Desktop Extension (.mcpb file)
 *
 * Creates a ZIP archive containing:
 * - manifest.json
 * - server/ (compiled JavaScript from dist/)
 * - node_modules/ (bundled dependencies)
 * - qsv/ (55 skill JSON definitions)
 * - .claude-plugin/ (plugin manifest)
 * - .mcp.json (MCP server configuration)
 * - commands/ (slash commands)
 * - agents/ (subagent definitions)
 * - skills/ (domain knowledge)
 * - icon.png (if exists)
 */

import { createWriteStream, existsSync, mkdirSync, rmSync, readFileSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';
import archiver from 'archiver';
import { execSync } from 'child_process';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const rootDir = join(__dirname, '..');

// Read version from package.json
const packageJson = JSON.parse(readFileSync(join(rootDir, 'package.json'), 'utf-8'));
const version = packageJson.version;

// Configuration
const OUTPUT_FILE = `qsv-mcp-server-${version}.mcpb`;
const OUTPUT_PATH = join(rootDir, OUTPUT_FILE);
const TEMP_DIR = join(rootDir, '.mcpb-build');

console.log('🎁 QSV MCP Server - Desktop Extension Packager');
console.log('='.repeat(50));
console.log(`📦 Packaging version: ${version}`);

/**
 * Clean up any existing build artifacts
 */
function cleanup() {
  console.log('\n🧹 Cleaning up...');
  if (existsSync(TEMP_DIR)) {
    rmSync(TEMP_DIR, { recursive: true, force: true });
  }

  // Remove current version file
  if (existsSync(OUTPUT_PATH)) {
    rmSync(OUTPUT_PATH);
    console.log(`   Removed existing ${OUTPUT_FILE}`);
  }

  console.log('✅ Cleanup complete');
}

/**
 * Build TypeScript code
 */
function buildTypeScript() {
  console.log('\n🔨 Building TypeScript...');
  try {
    execSync('npm run build', { cwd: rootDir, stdio: 'inherit' });
    console.log('✅ TypeScript build successful');
  } catch (error) {
    console.error('❌ TypeScript build failed');
    process.exit(1);
  }
}

/**
 * Validate required files exist
 */
function validateFiles() {
  console.log('\n🔍 Validating required files...');

  const required = [
    { path: join(rootDir, 'manifest.json'), name: 'manifest.json' },
    { path: join(rootDir, 'dist'), name: 'dist/' },
    { path: join(rootDir, 'node_modules'), name: 'node_modules/' },
    { path: join(rootDir, 'qsv'), name: 'qsv/' }
  ];

  const missing = required.filter(({ path }) => !existsSync(path));

  if (missing.length > 0) {
    console.error('❌ Missing required files:');
    missing.forEach(({ name }) => console.error(`   - ${name}`));
    process.exit(1);
  }

  console.log('✅ All required files present');
}

/**
 * Create .mcpb archive
 */
async function createArchive() {
  console.log('\n📦 Creating .mcpb archive...');

  return new Promise((resolve, reject) => {
    const output = createWriteStream(OUTPUT_PATH);
    const archive = archiver('zip', {
      zlib: { level: 9 } // Maximum compression
    });

    // Track progress
    let bytesProcessed = 0;
    archive.on('progress', (progress) => {
      const totalBytes = progress.fs.totalBytes;
      const percent =
        totalBytes > 0
          ? Math.round((progress.fs.processedBytes / totalBytes) * 100)
          : 0;
      if (progress.fs.processedBytes !== bytesProcessed) {
        process.stdout.write(`\r   Progress: ${percent}% (${Math.round(progress.fs.processedBytes / 1024 / 1024)}MB)`);
        bytesProcessed = progress.fs.processedBytes;
      }
    });

    output.on('close', () => {
      const sizeMB = (archive.pointer() / 1024 / 1024).toFixed(2);
      console.log(`\n✅ Archive created: ${OUTPUT_FILE} (${sizeMB} MB)`);
      resolve();
    });

    archive.on('error', (err) => {
      console.error('\n❌ Archive creation failed:', err);
      reject(err);
    });

    archive.on('warning', (err) => {
      if (err.code === 'ENOENT') {
        console.warn('⚠️  Warning:', err.message);
      } else {
        reject(err);
      }
    });

    archive.pipe(output);

    // Add manifest.json
    console.log('   Adding manifest.json...');
    archive.file(join(rootDir, 'manifest.json'), { name: 'manifest.json' });

    // Add compiled server code (dist/ → server/)
    console.log('   Adding server code (dist/ → server/)...');
    archive.directory(join(rootDir, 'dist'), 'server');

    // Add node_modules
    console.log('   Adding dependencies (node_modules/)...');
    archive.directory(join(rootDir, 'node_modules'), 'node_modules', {
      // Exclude development dependencies and large files
      filter: (file) => {
        const name = file.name || '';
        const lowerName = name.toLowerCase();
        // Exclude source maps, TypeScript definitions, and dev files (case-insensitive)
        return !name.endsWith('.map') &&
               !name.endsWith('.ts') &&
               !lowerName.includes('.github') &&
               !lowerName.includes('test') &&
               !lowerName.includes('example');
      }
    });

    // Add skill definitions
    console.log('   Adding skill definitions (qsv/)...');
    archive.directory(join(rootDir, 'qsv'), 'qsv');

    // Add Claude Plugin files
    const pluginDir = join(rootDir, '.claude-plugin');
    if (existsSync(pluginDir)) {
      console.log('   Adding plugin manifest (.claude-plugin/)...');
      archive.directory(pluginDir, '.claude-plugin');
    }

    const mcpJsonPath = join(rootDir, '.mcp.json');
    if (existsSync(mcpJsonPath)) {
      console.log('   Adding .mcp.json...');
      archive.file(mcpJsonPath, { name: '.mcp.json' });
    }

    const commandsDir = join(rootDir, 'commands');
    if (existsSync(commandsDir)) {
      console.log('   Adding slash commands (commands/)...');
      archive.directory(commandsDir, 'commands');
    }

    const agentsDir = join(rootDir, 'agents');
    if (existsSync(agentsDir)) {
      console.log('   Adding subagents (agents/)...');
      archive.directory(agentsDir, 'agents');
    }

    const skillsDir = join(rootDir, 'skills');
    if (existsSync(skillsDir)) {
      console.log('   Adding domain knowledge (skills/)...');
      archive.directory(skillsDir, 'skills');
    }

    // Add hook scripts (for SessionStart hook)
    const scriptsDir = join(rootDir, 'scripts');
    if (existsSync(scriptsDir)) {
      console.log('   Adding hook scripts (scripts/)...');
      archive.directory(scriptsDir, 'scripts');
    }

    // Add cowork CLAUDE.md template (deployed by SessionStart hook)
    const coworkTemplate = join(rootDir, 'cowork-CLAUDE.md');
    if (existsSync(coworkTemplate)) {
      console.log('   Adding cowork-CLAUDE.md...');
      archive.file(coworkTemplate, { name: 'cowork-CLAUDE.md' });
    }

    // Add icon if it exists
    const iconPath = join(rootDir, 'qsv-75x91.png');
    if (existsSync(iconPath)) {
      console.log('   Adding qsv-75x91.png...');
      archive.file(iconPath, { name: 'qsv-75x91.png' });
    } else {
      console.log('   ℹ️  No qsv-75x91.png found (optional)');
    }

    // Finalize archive
    console.log('   Finalizing archive...');
    archive.finalize();
  });
}

/**
 * Display summary
 */
function displaySummary() {
  console.log('\n' + '='.repeat(50));
  console.log('✨ Desktop Extension Package Created!');
  console.log('='.repeat(50));
  console.log(`\n📁 File: ${OUTPUT_FILE}`);
  console.log(`📍 Location: ${OUTPUT_PATH}`);
  console.log('\n🚀 Next Steps:');
  console.log('   1. Test locally: Drag .mcpb file into Claude Desktop settings');
  console.log('   2. Configure qsv binary path in extension settings');
  console.log('   3. Restart Claude Desktop');
  console.log('   4. Verify all 55 qsv skills are available');
  console.log('\n📚 Documentation:');
  console.log('   - Desktop Extension Guide: DESKTOP_EXTENSION.md');
  console.log('   - Installation Guide: README.md');
  console.log('   - Filesystem Usage: FILESYSTEM_USAGE.md');
  console.log('\n');
}

/**
 * Main execution
 */
async function main() {
  try {
    // Step 1: Clean up
    cleanup();

    // Step 2: Build TypeScript
    buildTypeScript();

    // Step 3: Validate files
    validateFiles();

    // Step 4: Create archive
    await createArchive();

    // Step 5: Display summary
    displaySummary();

  } catch (error) {
    console.error('\n❌ Packaging failed:', error?.message || String(error));
    process.exit(1);
  }
}

// Run packager
main();
