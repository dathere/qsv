/**
 * Centralized Configuration
 *
 * Manages all configurable settings with environment variable support.
 * Supports both legacy MCP server and Desktop Extension modes.
 */

import { homedir, tmpdir } from 'os';
import { join } from 'path';
import { execSync, execFileSync } from 'child_process';
import { statSync } from 'fs';

/**
 * Timeout for qsv binary validation commands in milliseconds (5 seconds)
 */
const QSV_VALIDATION_TIMEOUT_MS = 5000;

/**
 * Expand template variables in strings
 * Supports: ${HOME}, ${USERPROFILE}, ${DESKTOP}, ${DOCUMENTS}, ${DOWNLOADS}, ${TEMP}, ${TMPDIR}
 */
function expandTemplateVars(value: string): string {
  if (!value) return value;

  const home = homedir();

  // Get special directories (currently same for all platforms)
  const desktop = join(home, 'Desktop');
  const documents = join(home, 'Documents');
  const downloads = join(home, 'Downloads');
  const temp = tmpdir();

  // Replace template variables
  return value
    .replace(/\$\{HOME\}/g, home)
    .replace(/\$\{USERPROFILE\}/g, home)
    .replace(/\$\{DESKTOP\}/g, desktop)
    .replace(/\$\{DOCUMENTS\}/g, documents)
    .replace(/\$\{DOWNLOADS\}/g, downloads)
    .replace(/\$\{TEMP\}/g, temp)
    .replace(/\$\{TMPDIR\}/g, temp);
}

/**
 * Parse integer from environment variable with validation
 */
function parseIntEnv(envVar: string, defaultValue: number, min?: number, max?: number): number {
  const value = process.env[envVar];
  if (!value) return defaultValue;

  const parsed = parseInt(value, 10);
  if (isNaN(parsed)) {
    console.error(`[Config] Invalid value for ${envVar}: ${value}, using default: ${defaultValue}`);
    return defaultValue;
  }

  if (min !== undefined && parsed < min) {
    console.error(`[Config] Value for ${envVar} (${parsed}) is below minimum (${min}), using default: ${defaultValue}`);
    return defaultValue;
  }

  if (max !== undefined && parsed > max) {
    console.error(`[Config] Value for ${envVar} (${parsed}) exceeds maximum (${max}), using default: ${defaultValue}`);
    return defaultValue;
  }

  return parsed;
}

/**
 * Parse float from environment variable with validation
 */
function parseFloatEnv(envVar: string, defaultValue: number, min?: number, max?: number): number {
  const value = process.env[envVar];
  if (!value) return defaultValue;

  const parsed = parseFloat(value);
  if (isNaN(parsed) || !Number.isFinite(parsed)) {
    console.error(`[Config] Invalid value for ${envVar}: ${value}, using default: ${defaultValue}`);
    return defaultValue;
  }

  if (min !== undefined && parsed < min) {
    console.error(`[Config] Value for ${envVar} (${parsed}) is below minimum (${min}), using default: ${defaultValue}`);
    return defaultValue;
  }

  if (max !== undefined && parsed > max) {
    console.error(`[Config] Value for ${envVar} (${parsed}) exceeds maximum (${max}), using default: ${defaultValue}`);
    return defaultValue;
  }

  return parsed;
}

/**
 * Regular expression to detect unexpanded template variables from Claude Desktop
 * Matches ${user_config.*} patterns that indicate an empty/unset configuration field
 */
const UNEXPANDED_TEMPLATE_REGEX = /\$\{user_config\.[^}]+\}/;

/**
 * Get string from environment variable with default
 * Expands template variables like ${HOME}, ${USERPROFILE}, etc.
 * Uses nullish coalescing (??) to allow empty strings while falling back for undefined/null
 * Also treats empty strings and unexpanded template vars as missing values for user convenience
 */
function getStringEnv(envVar: string, defaultValue: string): string {
  const value = process.env[envVar];
  // Treat empty string, null, undefined, or unexpanded template as missing - use default
  // Unexpanded template happens when Claude Desktop config field is empty
  if (!value || value.trim() === '' || UNEXPANDED_TEMPLATE_REGEX.test(value)) {
    return expandTemplateVars(defaultValue);
  }
  return expandTemplateVars(value);
}

/**
 * Get string array from environment variable (split by delimiter)
 * Expands template variables in each path
 */
function getStringArrayEnv(envVar: string, defaultValue: string[], delimiter: string): string[] {
  const value = process.env[envVar];
  if (!value) return defaultValue;
  return value
    .split(delimiter)
    .filter(s => s.length > 0)
    .map(s => expandTemplateVars(s));
}

/**
 * Get platform-appropriate path delimiter
 */
function getPathDelimiter(): string {
  return process.platform === 'win32' ? ';' : ':';
}

/**
 * Parse boolean from environment variable
 */
function getBooleanEnv(envVar: string, defaultValue: boolean): boolean {
  const value = process.env[envVar];
  if (!value) return defaultValue;

  const lower = value.toLowerCase();
  if (lower === 'true' || lower === '1' || lower === 'yes') return true;
  if (lower === 'false' || lower === '0' || lower === 'no') return false;

  console.error(`[Config] Invalid boolean value for ${envVar}: ${value}, using default: ${defaultValue}`);
  return defaultValue;
}

/**
 * Minimum required qsv version
 * Set to 13.0.0 to minimize support issues and encourage users to update
 */
export const MINIMUM_QSV_VERSION = '13.0.0';

/**
 * Validation result interface
 */
export interface QsvValidationResult {
  valid: boolean;
  version?: string;
  path?: string;
  error?: string;
  totalMemory?: string;       // e.g., "64.00 GiB"
  totalMemoryBytes?: number;  // Numeric value in bytes for comparisons
  availableCommands?: string[];  // List of available qsv commands
  commandCount?: number;         // Number of installed commands
}

// Global diagnostic info for auto-detection
let lastDetectionDiagnostics: {
  whichAttempted: boolean;
  whichResult?: string;
  whichError?: string;
  locationsChecked: Array<{
    path: string;
    exists: boolean;
    isFile?: boolean;
    executable?: boolean;
    error?: string;
    version?: string;
  }>;
} = {
  whichAttempted: false,
  locationsChecked: []
};

/**
 * Get diagnostic information about the last auto-detection attempt
 */
export function getDetectionDiagnostics() {
  return lastDetectionDiagnostics;
}

/**
 * Auto-detect absolute path to qsv binary
 * 1. Uses 'which' on Unix/macOS or 'where' on Windows
 * 2. If that fails, checks common installation locations
 */
function detectQsvBinaryPath(): string | null {
  // Reset diagnostics
  lastDetectionDiagnostics = {
    whichAttempted: true,
    locationsChecked: []
  };

  // Try using which/where first
  try {
    const command = process.platform === 'win32' ? 'where' : 'which';
    const result = execFileSync(command, ['qsv'], {
      encoding: 'utf8',
      stdio: ['ignore', 'pipe', 'ignore']
    });
    const path = result.trim().split('\n')[0]; // Take first result
    if (path) {
      lastDetectionDiagnostics.whichResult = path;
      return path;
    }
  } catch (error) {
    lastDetectionDiagnostics.whichError = error instanceof Error ? error.message : String(error);
  }

  // Check common installation locations
  // This helps when running in desktop apps (like Claude Desktop) that don't have full PATH
  const commonLocations = process.platform === 'win32'
    ? [
        'C:\\Program Files\\qsv\\qsv.exe',
        'C:\\qsv\\qsv.exe',
        join(homedir(), 'scoop', 'shims', 'qsv.exe'),
        join(homedir(), 'AppData', 'Local', 'Programs', 'qsv', 'qsv.exe'),
      ]
    : [
        '/usr/local/bin/qsv',
        '/opt/homebrew/bin/qsv',  // Apple Silicon homebrew
        '/usr/bin/qsv',
        join(homedir(), '.cargo', 'bin', 'qsv'),
        join(homedir(), '.local', 'bin', 'qsv'),
      ];

  // Try each common location
  for (const location of commonLocations) {
    const diagnostic: any = { path: location, exists: false };

    try {
      // Check if file exists and is executable
      const stats = statSync(location);
      diagnostic.exists = true;
      diagnostic.isFile = stats.isFile();

      if (stats.isFile()) {
        // Verify it's actually qsv by trying to run it
        try {
          const versionOutput = execFileSync(location, ['--version'], {
            encoding: 'utf8',
            stdio: ['ignore', 'pipe', 'ignore'],
            timeout: QSV_VALIDATION_TIMEOUT_MS
          });
          diagnostic.executable = true;
          diagnostic.version = versionOutput.trim().split('\n')[0];
          lastDetectionDiagnostics.locationsChecked.push(diagnostic);
          return location; // Found it!
        } catch (execError) {
          diagnostic.executable = false;
          diagnostic.error = execError instanceof Error ? execError.message : String(execError);
        }
      }
    } catch (statError) {
      diagnostic.error = statError instanceof Error ? statError.message : String(statError);
    }

    lastDetectionDiagnostics.locationsChecked.push(diagnostic);
  }

  return null;
}

/**
 * Parse version string from qsv --version output
 * Examples:
 *   "qsv 0.135.0" -> "0.135.0"
 *   "qsv 0.135.0-alpha.1" -> "0.135.0-alpha.1"
 *   "qsv 0.135.0+build.123" -> "0.135.0+build.123"
 */
function parseQsvVersion(versionOutput: string): string | null {
  // Match semantic version with optional pre-release and build metadata
  const match = versionOutput.match(/qsv\s+(\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?(?:\+[0-9A-Za-z.-]+)?)/);
  return match ? match[1] : null;
}

/**
 * Parse memory unit string to bytes
 * Supports: B, KiB, MiB, GiB, TiB
 * Exported for testing
 */
export function parseMemoryToBytes(memoryStr: string): number | null {
  const match = memoryStr.match(/^([\d.]+)\s*(B|KiB|MiB|GiB|TiB)$/i);
  if (!match) return null;

  const value = parseFloat(match[1]);
  // Validate parsed value
  if (isNaN(value) || !isFinite(value) || value < 0) return null;

  const unit = match[2].toLowerCase();

  const multipliers: Record<string, number> = {
    'b': 1,
    'kib': 1024,
    'mib': 1024 * 1024,
    'gib': 1024 * 1024 * 1024,
    'tib': 1024 * 1024 * 1024 * 1024,
  };

  return value * (multipliers[unit] || 1);
}

/**
 * Parse total memory from qsv --version output
 * Memory info format: maxInputSize-freeSwap-availableMemory-totalMemory
 * Example: "51.20 GiB-0 B-13.94 GiB-64.00 GiB"
 * Exported for testing
 */
export function parseQsvMemoryInfo(versionOutput: string): { totalMemory: string; totalMemoryBytes: number } | null {
  // Pattern: captures 4 memory values before the parentheses with system info
  const memoryPattern = /([\d.]+\s*(?:B|KiB|MiB|GiB|TiB))-([\d.]+\s*(?:B|KiB|MiB|GiB|TiB))-([\d.]+\s*(?:B|KiB|MiB|GiB|TiB))-([\d.]+\s*(?:B|KiB|MiB|GiB|TiB))\s*\(/i;

  const match = versionOutput.match(memoryPattern);
  if (!match) return null;

  const totalMemoryStr = match[4].trim();
  const totalMemoryBytes = parseMemoryToBytes(totalMemoryStr);

  if (totalMemoryBytes === null) return null;

  return { totalMemory: totalMemoryStr, totalMemoryBytes };
}

/**
 * Parse available commands from qsv --list output
 * Handles both formats:
 *   qsv:     "Installed commands (63):"
 *   qsvlite: "Installed commands:"
 * Example output:
 *       apply       Apply series of transformations to a column
 *       behead      Drop header from CSV file
 *       ...
 * Exported for testing
 */
export function parseQsvCommandList(listOutput: string): { commands: string[]; count: number } | null {
  // Extract command count from header line (optional - qsvlite doesn't include count)
  const headerMatch = listOutput.match(/Installed commands(?: \((\d+)\))?:/);
  if (!headerMatch) return null;

  const reportedCount = headerMatch[1] ? parseInt(headerMatch[1], 10) : 0;

  // Extract command names (first word of each indented line)
  const commands: string[] = [];
  const lines = listOutput.split('\n');

  for (const line of lines) {
    // Match lines that start with any whitespace followed by a command name
    // Using \s+ instead of \s{4} for resilience to formatting variations
    const match = line.match(/^\s+(\w+)\s+/);
    if (match) {
      commands.push(match[1]);
    }
  }

  if (commands.length === 0) return null;

  // Use reported count if available, otherwise use parsed count
  return { commands, count: reportedCount || commands.length };
}

/**
 * Compare two semantic version strings
 * Returns: -1 if v1 < v2, 0 if equal, 1 if v1 > v2
 */
function compareVersions(v1: string, v2: string): number {
  const parts1 = v1.split('.').map(Number);
  const parts2 = v2.split('.').map(Number);

  for (let i = 0; i < Math.max(parts1.length, parts2.length); i++) {
    const part1 = parts1[i] || 0;
    const part2 = parts2[i] || 0;
    if (part1 < part2) return -1;
    if (part1 > part2) return 1;
  }
  return 0;
}

/**
 * Validate qsv binary at given path
 * Runs 'qsv --version' to check if binary exists and meets minimum version
 */
export function validateQsvBinary(binPath: string): QsvValidationResult {
  try {
    // Use execFileSync instead of execSync to prevent command injection
    const result = execFileSync(binPath, ['--version'], {
      encoding: 'utf8',
      stdio: ['ignore', 'pipe', 'pipe'],
      timeout: QSV_VALIDATION_TIMEOUT_MS
    });

    const version = parseQsvVersion(result);
    if (!version) {
      return {
        valid: false,
        error: `Could not parse version from qsv output: ${result.trim()}`,
      };
    }

    if (compareVersions(version, MINIMUM_QSV_VERSION) < 0) {
      return {
        valid: false,
        version,
        path: binPath,
        error: `qsv version ${version} found, but ${MINIMUM_QSV_VERSION} or higher is required`,
      };
    }

    // Parse memory information from version output
    const memoryInfo = parseQsvMemoryInfo(result);

    // Get list of available commands
    let commandInfo: { commands: string[]; count: number } | null = null;
    try {
      const listResult = execFileSync(binPath, ['--list'], {
        encoding: 'utf8',
        stdio: ['ignore', 'pipe', 'pipe'],
        timeout: QSV_VALIDATION_TIMEOUT_MS
      });
      commandInfo = parseQsvCommandList(listResult);
    } catch {
      // --list failed, but binary is still valid - commands info just won't be available
    }

    return {
      valid: true,
      version,
      path: binPath,
      totalMemory: memoryInfo?.totalMemory,
      totalMemoryBytes: memoryInfo?.totalMemoryBytes,
      availableCommands: commandInfo?.commands,
      commandCount: commandInfo?.count,
    };
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : String(error);
    return {
      valid: false,
      error: `Failed to execute qsv binary at "${binPath}": ${errorMessage}`,
    };
  }
}

/**
 * Initialize qsv binary path with auto-detection and validation
 *
 * This function runs at server startup and validates the qsv binary.
 * In Extension Mode, Claude Desktop restarts the server whenever the user
 * changes the qsv binary path setting, so validation occurs on every change.
 *
 * Priority:
 * 1. Explicit QSV_MCP_BIN_PATH environment variable (user-configured path)
 * 2. Auto-detected path from system PATH (via which/where command)
 * 3. Fall back to 'qsv' (legacy MCP mode only)
 *
 * In Extension Mode, always requires a fully qualified path and valid qsv binary
 * (version >= 13.0.0). Invalid paths or versions will fail validation with clear
 * error messages shown in server logs.
 */
function initializeQsvBinaryPath(): { path: string; validation: QsvValidationResult } {
  const inExtensionMode = getBooleanEnv('MCPB_EXTENSION_MODE', false);
  const explicitPath = process.env['QSV_MCP_BIN_PATH'];

  // If user explicitly configured a path (non-empty), use it
  if (explicitPath && explicitPath.trim() !== '' && !UNEXPANDED_TEMPLATE_REGEX.test(explicitPath)) {
    const expanded = expandTemplateVars(explicitPath);
    const validation = validateQsvBinary(expanded);
    return { path: expanded, validation };
  }

  // Try to auto-detect from PATH
  const detectedPath = detectQsvBinaryPath();
  if (detectedPath) {
    const validation = validateQsvBinary(detectedPath);
    if (validation.valid) {
      // In extension mode, ensure path is fully qualified
      return { path: detectedPath, validation };
    }
    // Detected but invalid version - continue to fallback
  }

  // Extension mode requires fully qualified, valid qsv binary
  if (inExtensionMode) {
    return {
      path: detectedPath || 'qsv',
      validation: {
        valid: false,
        error: detectedPath
          ? `qsv binary found at ${detectedPath} but version validation failed. Please install qsv ${MINIMUM_QSV_VERSION} or higher from https://github.com/dathere/qsv#installation`
          : `qsv binary not found in PATH. Extension mode requires qsv to be installed. Please install from https://github.com/dathere/qsv#installation and ensure it's in your system PATH.`,
      },
    };
  }

  // Legacy MCP mode: Fall back to 'qsv' (will work if in PATH, otherwise will fail)
  const fallbackPath = 'qsv';
  const validation = validateQsvBinary(fallbackPath);
  return { path: fallbackPath, validation };
}

/**
 * Detect if running in Desktop Extension mode
 * Desktop extensions set MCPB_EXTENSION_MODE=true
 */
export function isExtensionMode(): boolean {
  return getBooleanEnv('MCPB_EXTENSION_MODE', false);
}

/**
 * Initialize qsv binary path with auto-detection
 */
const qsvBinaryInit = initializeQsvBinaryPath();

/**
 * Configuration object with all configurable settings
 */
export const config = {
  /**
   * Path to qsv binary
   * Auto-detected from PATH if not explicitly configured
   */
  qsvBinPath: qsvBinaryInit.path,

  /**
   * Validation result for qsv binary
   * Contains version info and any errors
   */
  qsvValidation: qsvBinaryInit.validation,

  /**
   * Working directory for relative paths
   * Default: Current working directory
   */
  workingDir: getStringEnv('QSV_MCP_WORKING_DIR', '${DOWNLOADS}'),

  /**
   * Allowed directories for file access
   * Can be either:
   * - Colon/semicolon-separated paths (legacy MCP)
   * - JSON array (Desktop extension with directory type)
   * Default: Empty array (only working directory allowed)
   */
  allowedDirs: (() => {
    const envValue = process.env['QSV_MCP_ALLOWED_DIRS'];
    // Treat empty, undefined, or unexpanded template as empty array
    if (!envValue || envValue.trim() === '' || UNEXPANDED_TEMPLATE_REGEX.test(envValue)) {
      return [];
    }

    // Try parsing as JSON array first (Desktop extension mode)
    try {
      const parsed = JSON.parse(envValue);
      if (Array.isArray(parsed)) {
        return parsed.map(p => expandTemplateVars(p));
      }
    } catch {
      // Not JSON, treat as delimited string
    }

    // Fall back to delimited string (legacy MCP mode)
    return getStringArrayEnv('QSV_MCP_ALLOWED_DIRS', [], getPathDelimiter());
  })(),

  /**
   * Maximum size for converted file cache in GB
   * Default: 1 GB
   */
  convertedLifoSizeGB: parseFloatEnv(
    'QSV_MCP_CONVERTED_LIFO_SIZE_GB',
    1.0, // 1 GB
    0.1, // Minimum: 0.1 GB
    100.0, // Maximum: 100 GB
  ),

  /**
   * Operation timeout in milliseconds
   * Default: 2 minutes (better for interactive use)
   */
  operationTimeoutMs: parseIntEnv(
    'QSV_MCP_OPERATION_TIMEOUT_MS',
    2 * 60 * 1000, // 2 minutes
    1000, // Minimum: 1 second
    30 * 60 * 1000, // Maximum: 30 minutes
  ),

  /**
   * Maximum number of files to return in a single listing
   * Default: 1000 files
   */
  maxFilesPerListing: parseIntEnv(
    'QSV_MCP_MAX_FILES_PER_LISTING',
    1000,
    1, // Minimum: 1 file
    100000, // Maximum: 100k files
  ),

  /**
   * Maximum number of steps in a pipeline
   * Default: 50 steps
   */
  maxPipelineSteps: parseIntEnv(
    'QSV_MCP_MAX_PIPELINE_STEPS',
    50,
    1, // Minimum: 1 step
    1000, // Maximum: 1000 steps
  ),

  /**
   * Maximum number of concurrent operations
   * Default: 10 operations
   */
  maxConcurrentOperations: parseIntEnv(
    'QSV_MCP_MAX_CONCURRENT_OPERATIONS',
    10,
    1, // Minimum: 1 operation
    100, // Maximum: 100 operations
  ),

  /**
   * Command timeout in milliseconds (alternative name for operationTimeoutMs)
   * Desktop extensions use QSV_MCP_TIMEOUT_MS, legacy MCP uses operationTimeoutMs
   * Default: 5 minutes
   */
  timeoutMs: parseIntEnv(
    'QSV_MCP_TIMEOUT_MS',
    5 * 60 * 1000, // 5 minutes
    10 * 1000, // Minimum: 10 seconds
    60 * 60 * 1000, // Maximum: 1 hour
  ),

  /**
   * Maximum output size in bytes
   * Large outputs are automatically saved to disk
   * Default: 50 MB
   */
  maxOutputSize: parseIntEnv(
    'QSV_MCP_MAX_OUTPUT_SIZE',
    50 * 1024 * 1024, // 50 MB
    1 * 1024 * 1024, // Minimum: 1 MB
    100 * 1024 * 1024, // Maximum: 100 MB
  ),

  /**
   * Auto-regenerate skills when qsv version changes
   * Default: false (manual regeneration)
   */
  autoRegenerateSkills: getBooleanEnv('QSV_MCP_AUTO_REGENERATE_SKILLS', false),

  /**
   * Check for qsv updates on startup
   * Default: true
   */
  checkUpdatesOnStartup: getBooleanEnv('QSV_MCP_CHECK_UPDATES_ON_STARTUP', true),

  /**
   * Show update notifications in logs
   * Default: true
   */
  notifyUpdates: getBooleanEnv('QSV_MCP_NOTIFY_UPDATES', true),

  /**
   * Detect if running in Desktop Extension mode
   * Desktop extensions set MCPB_EXTENSION_MODE=true
   */
  isExtensionMode: isExtensionMode(),
} as const;
