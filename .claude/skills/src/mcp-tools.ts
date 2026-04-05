/**
 * MCP Tool Definitions and Handlers for QSV Commands
 *
 * Barrel re-export — all implementations live in focused modules:
 *   tool-constants.ts    — Pure constants, zero internal dependencies
 *   command-guidance.ts  — Guidance system for tool selection
 *   concurrency.ts       — Slot-based concurrency control
 *   file-operations.ts   — File path resolution, conversion, output formatting
 *   parquet-bridge.ts    — Parquet/DuckDB conversion, schema, stats
 *   tool-definitions.ts  — Tool creation functions
 *   tool-handlers.ts     — Dispatch and handlers
 */

// ── tool-constants ──────────────────────────────────────────────────────────
export {
  PIPELINE_METADATA,
  FINAL_OUTPUT_FILE,
  MAX_LOG_MESSAGE_LEN,
  ALWAYS_FILE_COMMANDS,
  METADATA_COMMANDS,
  NON_TABULAR_COMMANDS,
  BINARY_OUTPUT_FORMATS,
  FILE_PATH_INPUT_OPTIONS,
  FILE_PATH_OUTPUT_OPTIONS,
  isBinaryOutputFormat,
  LARGE_FILE_THRESHOLD_BYTES,
  MAX_MCP_RESPONSE_SIZE,
  COMMON_COMMANDS,
  LOG_ENTRY_TYPES,
  AUTO_INDEX_SIZE_MB,
} from "./tool-constants.js";
export type { PipelineMetadata } from "./tool-constants.js";

// ── command-guidance ────────────────────────────────────────────────────────
export {
  COMMAND_GUIDANCE,
  enhanceParameterDescription,
  enhanceDescription,
} from "./command-guidance.js";
export type { CommandGuidance } from "./command-guidance.js";

// ── concurrency ─────────────────────────────────────────────────────────────
export {
  activeProcesses,
  acquireSlot,
  releaseSlot,
  isShuttingDown,
  initiateShutdown,
  killAllProcesses,
  getActiveProcessCount,
  getActiveOperationCount,
  _testConcurrency,
} from "./concurrency.js";

// ── file-operations ─────────────────────────────────────────────────────────
export {
  statOrNull,
  setToolsWorkingDir,
  getToolsWorkingDir,
  getCurrentWorkingDir,
  runQsvWithTimeout,
  buildConversionArgs,
  mapSchemaType,
  autoIndexIfNeeded,
  shouldUseTempFile,
  resolveAndConvertInputFile,
  buildFileNotFoundError,
  paramKeyToFlag,
  looksLikeFilePath,
  resolveFilePathParams,
  buildSkillExecParams,
  collectAdditionalInputFiles,
  formatToolResult,
  resolveParamAliases,
} from "./file-operations.js";

// ── parquet-bridge ──────────────────────────────────────────────────────────
export {
  detectDelimiter,
  parseCSVLine,
  isDateDtype,
  patchSchemaAmPmDates,
  isCsvLikeFile,
  getParquetPath,
  ensureParquet,
  ensureStatsCache,
  ensurePolarsSchema,
  convertCsvToParquet,
  suggestDuckDbFixes,
  tryDuckDbExecution,
} from "./parquet-bridge.js";

// ── tool-definitions ────────────────────────────────────────────────────────
export {
  createToolDefinition,
  createGenericToolDefinition,
  createListFilesTool,
  createSetWorkingDirTool,
  createBrowseDirectoryTool,
  createGetWorkingDirTool,
  createConfigTool,
  createSearchToolsTool,
  createToParquetTool,
  createLogTool,
  createSetupToolDefinition,
} from "./tool-definitions.js";

// ── tool-handlers ───────────────────────────────────────────────────────────
export {
  handleToolCall,
  handleGenericCommand,
  handleConfigTool,
  handleSearchToolsCall,
  handleToParquetCall,
  handleLogCall,
  handleSetupCall,
} from "./tool-handlers.js";
export type { SetupCallResult } from "./tool-handlers.js";
