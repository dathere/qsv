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
 *
 * Only symbols consumed by external callers (mcp-server.ts, index.ts, tests)
 * are re-exported here. Sibling modules import directly from each other.
 */

// ── tool-constants ──────────────────────────────────────────────────────────
export {
  PIPELINE_METADATA,
  MAX_LOG_MESSAGE_LEN,
  isBinaryOutputFormat,
  COMMON_COMMANDS,
} from "./tool-constants.js";
export type { PipelineMetadata } from "./tool-constants.js";

// ── concurrency ─────────────────────────────────────────────────────────────
export {
  initiateShutdown,
  killAllProcesses,
  getActiveProcessCount,
  getActiveOperationCount,
  getQueueStatus,
  _testConcurrency,
} from "./concurrency.js";
export type { SlotResult } from "./concurrency.js";

// ── file-operations ─────────────────────────────────────────────────────────
export {
  setToolsWorkingDir,
  getToolsWorkingDir,
  buildConversionArgs,
  paramKeyToFlag,
  looksLikeFilePath,
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
  statsFilePath,
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
