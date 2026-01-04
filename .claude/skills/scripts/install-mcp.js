#!/usr/bin/env node
/**
 * QSV MCP Server Installation Helper
 *
 * Interactive installer that:
 * 1. Checks for qsv binary
 * 2. Builds TypeScript
 * 3. Updates Claude Desktop config
 * 4. Provides verification steps
 */

import { exec } from 'child_process';
import { promisify } from 'util';
import { readFile, writeFile, mkdir } from 'fs/promises';
import { existsSync } from 'fs';
import { join, dirname, resolve } from 'path';
import { fileURLToPath } from 'url';
import { homedir, platform } from 'os';

const execAsync = promisify(exec);

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const SKILLS_DIR = resolve(__dirname, '..');

/**
 * Color output helpers
 */
const colors = {
  reset: '\x1b[0m',
  bright: '\x1b[1m',
  green: '\x1b[32m',
  yellow: '\x1b[33m',
  red: '\x1b[31m',
  cyan: '\x1b[36m',
};

function success(msg) {
  console.log(`${colors.green}✓${colors.reset} ${msg}`);
}

function info(msg) {
  console.log(`${colors.cyan}ℹ${colors.reset} ${msg}`);
}

function warning(msg) {
  console.log(`${colors.yellow}⚠${colors.reset} ${msg}`);
}

function error(msg) {
  console.error(`${colors.red}✗${colors.reset} ${msg}`);
}

function header(msg) {
  console.log(`\n${colors.bright}${msg}${colors.reset}\n`);
}

/**
 * Check if qsv binary is available
 */
async function checkQsvBinary() {
  try {
    const { stdout } = await execAsync('qsv --version');
    const version = stdout.trim();
    success(`Found qsv: ${version}`);
    return true;
  } catch {
    error('qsv binary not found in PATH');
    console.log('\nPlease install qsv first:');
    console.log('  - macOS: brew install qsv');
    console.log('  - Or download from: https://github.com/dathere/qsv/releases\n');
    return false;
  }
}

/**
 * Build TypeScript files
 */
async function buildTypeScript() {
  try {
    info('Building TypeScript...');
    const { stdout, stderr } = await execAsync('npm run build', {
      cwd: SKILLS_DIR,
    });

    if (stderr && stderr.includes('error')) {
      error('TypeScript build failed');
      console.log(stderr);
      return false;
    }

    success('TypeScript built successfully');
    return true;
  } catch (err) {
    error('Failed to build TypeScript');
    console.error(err.message);
    return false;
  }
}

/**
 * Get Claude Desktop config path
 */
function getClaudeConfigPath() {
  const os = platform();

  switch (os) {
    case 'darwin': // macOS
      return join(homedir(), 'Library', 'Application Support', 'Claude', 'claude_desktop_config.json');
    case 'win32': // Windows
      return join(process.env.APPDATA || '', 'Claude', 'claude_desktop_config.json');
    case 'linux':
      return join(homedir(), '.config', 'Claude', 'claude_desktop_config.json');
    default:
      throw new Error(`Unsupported platform: ${os}`);
  }
}

/**
 * Update Claude Desktop configuration
 */
async function updateClaudeConfig() {
  const configPath = getClaudeConfigPath();
  const mcpServerPath = resolve(SKILLS_DIR, 'dist', 'mcp-server.js');

  info(`Claude Desktop config: ${configPath}`);

  // Ensure config directory exists
  const configDir = dirname(configPath);
  if (!existsSync(configDir)) {
    info('Creating Claude Desktop config directory...');
    await mkdir(configDir, { recursive: true });
  }

  // Read existing config or create new one
  let config = {};
  if (existsSync(configPath)) {
    try {
      const content = await readFile(configPath, 'utf-8');
      config = JSON.parse(content);
      info('Found existing Claude Desktop config');
    } catch (err) {
      warning('Could not parse existing config, creating new one');
    }
  } else {
    info('Creating new Claude Desktop config');
  }

  // Add or update qsv MCP server
  if (!config.mcpServers) {
    config.mcpServers = {};
  }

  config.mcpServers.qsv = {
    command: 'node',
    args: [mcpServerPath],
    env: {
      QSV_BIN_PATH: '/usr/local/bin/qsv',
    },
  };

  // Write config back
  try {
    await writeFile(configPath, JSON.stringify(config, null, 2), 'utf-8');
    success('Updated Claude Desktop config');
    return true;
  } catch (err) {
    error('Failed to write Claude Desktop config');
    console.error(err.message);
    return false;
  }
}

/**
 * Print verification steps
 */
function printVerificationSteps() {
  header('Installation Complete! 🎉');

  console.log('Next steps:\n');
  console.log('1. Restart Claude Desktop');
  console.log('2. Check that qsv tools are available:');
  console.log('   - Look for tools like "qsv_select", "qsv_stats", etc. in Claude\n');
  console.log('3. Try a command:');
  console.log('   "Select columns 1-5 from my data.csv"\n');
  console.log('4. Explore examples:');
  console.log('   "Show me an example of joining two CSV files"\n');

  info('For troubleshooting, check Claude Desktop logs');
  info('MCP Server location: ' + resolve(SKILLS_DIR, 'dist', 'mcp-server.js'));
}

/**
 * Main installation flow
 */
async function main() {
  header('QSV MCP Server Installation');

  // Step 1: Check for qsv binary
  header('Step 1: Checking for qsv binary...');
  if (!(await checkQsvBinary())) {
    process.exit(1);
  }

  // Step 2: Build TypeScript
  header('Step 2: Building TypeScript...');
  if (!(await buildTypeScript())) {
    process.exit(1);
  }

  // Step 3: Update Claude Desktop config
  header('Step 3: Updating Claude Desktop configuration...');
  if (!(await updateClaudeConfig())) {
    process.exit(1);
  }

  // Step 4: Print verification steps
  printVerificationSteps();
}

// Run installer
main().catch(err => {
  error('Installation failed');
  console.error(err);
  process.exit(1);
});
