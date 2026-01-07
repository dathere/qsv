/**
 * Centralized Configuration
 *
 * Manages all configurable settings with environment variable support.
 * Supports both legacy MCP server and Desktop Extension modes.
 */

import { homedir, tmpdir } from 'os';
import { join } from 'path';

/**
 * Expand template variables in strings
 * Supports: ${HOME}, ${USERPROFILE}, ${DESKTOP}, ${DOCUMENTS}, ${DOWNLOADS}, ${TEMP}, ${TMPDIR}
 */
function expandTemplateVars(value: string): string {
  if (!value) return value;

  const home = homedir();

  // Get platform-specific special directories
  const desktop = process.platform === 'win32'
    ? join(home, 'Desktop')
    : join(home, 'Desktop');

  const documents = process.platform === 'win32'
    ? join(home, 'Documents')
    : join(home, 'Documents');

  const downloads = process.platform === 'win32'
    ? join(home, 'Downloads')
    : join(home, 'Downloads');

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
 * Get string from environment variable with default
 * Expands template variables like ${HOME}, ${USERPROFILE}, etc.
 * Uses nullish coalescing (??) to allow empty strings while falling back for undefined/null
 */
function getStringEnv(envVar: string, defaultValue: string): string {
  const value = process.env[envVar] ?? defaultValue;
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
 * Detect if running in Desktop Extension mode
 * Desktop extensions set MCPB_EXTENSION_MODE=true
 */
export function isExtensionMode(): boolean {
  return getBooleanEnv('MCPB_EXTENSION_MODE', false);
}

/**
 * Configuration object with all configurable settings
 */
export const config = {
  /**
   * Path to qsv binary
   * Default: 'qsv' (assumes qsv is in PATH)
   */
  qsvBinPath: getStringEnv('QSV_MCP_BIN_PATH', 'qsv'),

  /**
   * Working directory for relative paths
   * Default: Current working directory
   */
  workingDir: getStringEnv('QSV_MCP_WORKING_DIR', process.cwd()),

  /**
   * Allowed directories for file access
   * Can be either:
   * - Colon/semicolon-separated paths (legacy MCP)
   * - JSON array (Desktop extension with directory type)
   * Default: Empty array (only working directory allowed)
   */
  allowedDirs: (() => {
    const envValue = process.env['QSV_MCP_ALLOWED_DIRS'];
    if (!envValue) return [];

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
