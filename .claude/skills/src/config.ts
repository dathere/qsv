/**
 * Centralized Configuration
 * 
 * Manages all configurable settings with environment variable support.
 */

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
 */
function getStringEnv(envVar: string, defaultValue: string): string {
  return process.env[envVar] || defaultValue;
}

/**
 * Get string array from environment variable (split by delimiter)
 */
function getStringArrayEnv(envVar: string, defaultValue: string[], delimiter: string): string[] {
  const value = process.env[envVar];
  if (!value) return defaultValue;
  return value.split(delimiter).filter(s => s.length > 0);
}

/**
 * Get platform-appropriate path delimiter
 */
function getPathDelimiter(): string {
  return process.platform === 'win32' ? ';' : ':';
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
   * Allowed directories for file access (colon-separated on Unix, semicolon on Windows)
   * Default: Empty array (only working directory allowed)
   */
  allowedDirs: getStringArrayEnv('QSV_MCP_ALLOWED_DIRS', [], getPathDelimiter()),

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
} as const;
