/**
 * Pure constants for MCP tool definitions and handlers.
 * Zero internal dependencies — this module can be imported by any other module.
 */

// ── Pipeline Metadata (for reproducibility manifest) ─────────────────────────

/**
 * Well-known Symbols for attaching pipeline metadata to MCP tool results.
 * Symbol properties are invisible to JSON.stringify, so they never leak
 * into the MCP protocol response.
 */
export const PIPELINE_METADATA = Symbol.for("qsv.pipelineMetadata");
export const FINAL_OUTPUT_FILE = Symbol.for("qsv.finalOutputFile");

/**
 * Metadata about a tool invocation's inputs/outputs, attached to the result
 * via the PIPELINE_METADATA symbol so the server can record it in the manifest.
 */
export interface PipelineMetadata {
  inputFile?: string;
  outputFile?: string;
  commandLine?: string;
  durationMs?: number;
  success: boolean;
  additionalInputFiles?: Array<{ file: string; param: string }>;
}

/**
 * Maximum length for qsv_log messages (in characters).
 * Messages exceeding this limit are silently truncated.
 */
export const MAX_LOG_MESSAGE_LEN = 4096;

/**
 * Commands that always return full CSV data and should use temp files
 */
export const ALWAYS_FILE_COMMANDS = new Set([
  "stats",
  "moarstats",
  "frequency",
  "slice",
  "sample",
  "template",
  "geocode",
  "sort",
  "dedup",
  "join",
  "joinp",
  "select",
  "search",
  "searchset",
  "schema",
  "validate",
  "diff",
  "cat",
  "transpose",
  "partition",
  "split",
  "explode",
  "pseudo",
  "rename",
  "replace",
  "datefmt",
  "reverse",
  "safenames",
  "sqlp",
  "pivotp",
  "pragmastat",
  "tojsonl",
]);

/**
 * Commands that return small metadata (not full CSV) and should use stdout
 */
export const METADATA_COMMANDS = new Set(["count", "headers", "index", "sniff"]);

/** Commands whose output is NOT tabular CSV — skip TSV conversion */
export const NON_TABULAR_COMMANDS = new Set([
  ...METADATA_COMMANDS,  // count, headers, index, sniff
  "tojsonl",             // JSONL output
  "template",            // Free-form text
  "schema",              // JSON Schema output
  "validate",            // Validation messages, not CSV data
  "describegpt",         // Markdown output (data dictionaries, descriptions, tags)
]);

/** Binary output formats from sqlp that should never get a .tsv extension */
export const BINARY_OUTPUT_FORMATS = new Set(["parquet", "arrow", "avro"]);

/**
 * Options that accept file paths as input (read from).
 * Values are resolved to absolute paths via filesystemProvider.resolvePath().
 */
export const FILE_PATH_INPUT_OPTIONS = new Set([
  "--prompt-file",
  "--tag-vocab",
  "--template-file",
  "--globals-json",
]);

/**
 * Options that accept file paths as output (written to).
 * Values are resolved to absolute paths via filesystemProvider.resolvePath()
 * and checked against reserved cache paths.
 */
export const FILE_PATH_OUTPUT_OPTIONS = new Set([
  "--dupes-output",
  "--keys-output",
  "--unmatched-output",
  "--sql-results",
  "--export-prompt",
]);

/**
 * Check if the command+params produce binary output (not tabular text).
 * Used to skip auto temp file creation and prevent .tsv extensions for
 * binary formats (parquet/arrow/avro) that can't be read back as UTF-8.
 */
export function isBinaryOutputFormat(commandName: string, params: Record<string, unknown>): boolean {
  return commandName === "sqlp" &&
    BINARY_OUTPUT_FORMATS.has(String(params.format ?? "").toLowerCase());
}

/**
 * Input file size threshold (in bytes) for auto temp file
 */
export const LARGE_FILE_THRESHOLD_BYTES = 10 * 1024 * 1024; // 10MB

/**
 * Maximum size for MCP response (in bytes)
 * Outputs larger than this will be saved to working directory instead of returned directly
 * Claude Desktop has a 1MB limit, so we use 850KB to stay safely under
 */
export const MAX_MCP_RESPONSE_SIZE = 850 * 1024; // 850KB - safe for Claude Desktop (< 1MB limit)

/**
 * 13 most essential qsv commands exposed as individual MCP tools
 * Optimized for token efficiency while maintaining high-value tool access
 *
 * Commands promoted to CORE_TOOLS (always loaded):
 * stats, index
 *
 * Commands moved to qsv_command generic tool:
 * join, sort, dedup, rename, validate, sample, template, diff, schema
 */
export const COMMON_COMMANDS = [
  "select", // Column selection (most frequently used)
  "moarstats", // Comprehensive statistics with data type inference
  "search", // Pattern-based filtering
  "frequency", // Value distribution
  "headers", // Header operations (quick discovery)
  "count", // Row counting (instant with index)
  "slice", // Row selection
  "sniff", // Format detection (referenced by all command workflows)
  "sqlp", // SQL queries (Polars engine)
  "joinp", // High-performance joins (Polars engine)
  "cat", // Concatenate CSV files (rows/columns)
  "geocode", // Geocoding operations
  "describegpt", // AI-powered data description and documentation
] as const;

/** Valid entry types for qsv_log */
export const LOG_ENTRY_TYPES = new Set([
  "agent_reasoning",
  "agent_action",
  "result_summary",
  "note",
]);

/**
 * Auto-indexing threshold in MB
 */
export const AUTO_INDEX_SIZE_MB = 10;
