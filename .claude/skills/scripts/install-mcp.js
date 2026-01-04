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
import * as readline from 'readline';

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
 * Prompt user for input with a default value
 */
function promptUser(question, defaultValue) {
  return new Promise((resolve) => {
    const rl = readline.createInterface({
      input: process.stdin,
      output: process.stdout,
    });

    rl.question(`${question} [${defaultValue}]: `, (answer) => {
      rl.close();
      resolve(answer.trim() || defaultValue);
    });
  });
}

/**
 * Get platform-specific default directories
 */
function getPlatformDefaults() {
  const os = platform();
  const home = homedir();

  switch (os) {
    case 'darwin': // macOS
      return {
        workingDir: join(home, 'Downloads'),
        allowedDirs: [
          join(home, 'Downloads'),
          join(home, 'Documents'),
          join(home, 'Desktop'),
        ].join(':'),
        delimiter: ':',
      };
    case 'win32': // Windows
      return {
        workingDir: join(home, 'Downloads'),
        allowedDirs: [
          join(home, 'Downloads'),
          join(home, 'Documents'),
          join(home, 'Desktop'),
        ].join(';'),
        delimiter: ';',
      };
    case 'linux':
      return {
        workingDir: join(home, 'Downloads'),
        allowedDirs: [
          join(home, 'Downloads'),
          join(home, 'Documents'),
        ].join(':'),
        delimiter: ':',
      };
    default:
      // Fallback for other Unix-like systems
      return {
        workingDir: join(home, 'Downloads'),
        allowedDirs: [
          join(home, 'Downloads'),
          join(home, 'Documents'),
        ].join(':'),
        delimiter: ':',
      };
  }
}

/**
 * Prompt for environment variable configuration
 */
async function promptForEnvVars() {
  const defaults = getPlatformDefaults();

  console.log('\nConfiguring environment variables for QSV MCP Server...\n');
  info(`Platform detected: ${platform()}`);
  info(`Directory delimiter: '${defaults.delimiter}'\n`);

  // Prompt for QSV_WORKING_DIR
  console.log(`${colors.cyan}QSV_WORKING_DIR${colors.reset}`);
  console.log('  Default working directory for relative file paths');
  const workingDir = await promptUser('Enter working directory', defaults.workingDir);

  // Prompt for QSV_ALLOWED_DIRS
  console.log(`\n${colors.cyan}QSV_ALLOWED_DIRS${colors.reset}`);
  console.log(`  Allowed directories (separated by '${defaults.delimiter}')`);
  console.log('  Files outside these directories cannot be accessed');
  const allowedDirs = await promptUser('Enter allowed directories', defaults.allowedDirs);

  return {
    QSV_WORKING_DIR: workingDir,
    QSV_ALLOWED_DIRS: allowedDirs,
  };
}

/**
 * Check if qsv binary is available and get its path
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
 * Get qsv binary path
 */
async function getQsvBinaryPath() {
  try {
    // Try 'which' command (Unix/macOS/Linux)
    const { stdout } = await execAsync('which qsv', { timeout: 5000 });
    return stdout.trim();
  } catch {
    try {
      // Try 'where' command (Windows)
      const { stdout } = await execAsync('where qsv', { timeout: 5000 });
      // 'where' returns all matches, take first one
      return stdout.trim().split('\n')[0];
    } catch {
      // Both 'which' and 'where' failed - qsv is not discoverable
      error('Could not detect qsv binary on your system PATH.');
      console.log('\nPlease ensure qsv is installed and available in your PATH:');
      console.log('  - macOS: brew install qsv');
      console.log('  - Or download from: https://github.com/dathere/qsv/releases\n');
      throw new Error('qsv binary not found in PATH');
    }
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

    // Check for TypeScript errors (error TS1234: ...) to avoid false positives from warnings
    if (stderr && /error TS\d+:/i.test(stderr)) {
      error('TypeScript build failed');
      console.log(stderr);
      return false;
    }

    success('TypeScript built successfully');
    return true;
  } catch (err) {
    error('Failed to build TypeScript');
    console.error(err instanceof Error ? err.message : String(err));
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
async function updateClaudeConfig(envVars) {
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

  // Get qsv binary path dynamically
  const qsvPath = await getQsvBinaryPath();
  info(`Using qsv binary at: ${qsvPath}`);

  // Add or update qsv MCP server
  if (!config.mcpServers) {
    config.mcpServers = {};
  }

  config.mcpServers.qsv = {
    command: 'node',
    args: [mcpServerPath],
    env: {
      QSV_BIN_PATH: qsvPath,
      ...envVars,
    },
  };

  // Write config back
  try {
    await writeFile(configPath, JSON.stringify(config, null, 2), 'utf-8');
    success('Updated Claude Desktop config');
    return true;
  } catch (err) {
    error('Failed to write Claude Desktop config');
    console.error(err instanceof Error ? err.message : String(err));
    return false;
  }
}

/**
 * Print verification steps
 */
function printVerificationSteps(envVars) {
  header('Installation Complete! 🎉');

  console.log('Configuration summary:\n');
  if (envVars) {
    console.log(`  Working Directory: ${envVars.QSV_WORKING_DIR}`);
    console.log(`  Allowed Directories: ${envVars.QSV_ALLOWED_DIRS}\n`);
  }

  console.log('Next steps:\n');
  console.log('1. Restart Claude Desktop');
  console.log('2. Check that qsv tools are available:');
  console.log('   - Look for tools like "qsv_select", "qsv_stats", etc. in Claude\n');
  console.log('3. Try filesystem commands:');
  console.log('   "List CSV files in my working directory"');
  console.log('   "What CSV files are available?"\n');
  console.log('4. Try a data command:');
  console.log('   "Show me the columns in data.csv"');
  console.log('   "Get statistics for sales.csv"\n');
  console.log('5. Explore examples:');
  console.log('   "Show me an example of joining two CSV files"\n');

  info('For troubleshooting, check Claude Desktop logs');
  info('MCP Server location: ' + resolve(SKILLS_DIR, 'dist', 'mcp-server.js'));
  info('Config location: ' + getClaudeConfigPath());
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

  // Step 3: Configure environment variables
  header('Step 3: Configuring environment variables...');
  const envVars = await promptForEnvVars();

  // Step 4: Update Claude Desktop config
  header('Step 4: Updating Claude Desktop configuration...');
  if (!(await updateClaudeConfig(envVars))) {
    process.exit(1);
  }

  // Step 5: Print verification steps
  printVerificationSteps(envVars);
}

// Run installer
main().catch(err => {
  error('Installation failed');
  console.error(err);
  process.exit(1);
});
