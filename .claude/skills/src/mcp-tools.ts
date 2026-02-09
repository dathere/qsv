/**
 * MCP Tool Definitions and Handlers for QSV Commands
 */

import type { ChildProcess } from "child_process";
import { randomUUID } from "crypto";
import { stat, access, readFile, unlink, rename, copyFile, readdir } from "fs/promises";
import { constants } from "fs";
import { basename, dirname, join } from "path";
import { tmpdir } from "os";
import { ConvertedFileManager } from "./converted-file-manager.js";
import type {
  QsvSkill,
  Argument,
  Option,
  McpToolDefinition,
  McpToolProperty,
  FilesystemProviderExtended,
} from "./types.js";
import { runQsvSimple } from "./executor.js";
import type { SkillExecutor } from "./executor.js";
import type { SkillLoader } from "./loader.js";
import { config, getDetectionDiagnostics } from "./config.js";
import { formatBytes, findSimilarFiles } from "./utils.js";

/**
 * MCP tool result helpers to eliminate repetitive { content: [{ type: "text"... }] } boilerplate
 */
function errorResult(message: string) {
  return { content: [{ type: "text" as const, text: message }], isError: true as const };
}

function successResult(text: string) {
  return { content: [{ type: "text" as const, text }], isError: false as const };
}

/**
 * Auto-indexing threshold in MB
 */
const AUTO_INDEX_SIZE_MB = 10;

/**
 * Commands that always return full CSV data and should use temp files
 */
const ALWAYS_FILE_COMMANDS = new Set([
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
  "apply",
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
  "to",
  "tojsonl",
]);

/**
 * Commands that return small metadata (not full CSV) and should use stdout
 */
const METADATA_COMMANDS = new Set(["count", "headers", "index", "sniff"]);

/**
 * Consolidated guidance for each command.
 * Combines when-to-use, common patterns, error prevention, complementary servers,
 * and behavioral flags into a single lookup.
 */
interface CommandGuidance {
  whenToUse?: string;
  commonPattern?: string;
  errorPrevention?: string;
  complementaryServer?: string;
  needsMemoryWarning?: boolean;
  needsIndexHint?: boolean;
  hasCommonMistakes?: boolean;
}

const COMMAND_GUIDANCE: Record<string, CommandGuidance> = {
  select: {
    whenToUse: 'Choose columns. Syntax: "1,3,5" (specific), "1-10" (range), "!SSN" (exclude), "/<regex>/" (pattern), "_" (last).',
    commonPattern: "First step: select columns → filter → sort → output. Speeds up downstream ops.",
  },
  slice: {
    whenToUse: "Select rows by position: first N, last N, skip N, range.",
  },
  search: {
    whenToUse: "Filter rows matching pattern/regex. Search applied to selected fields. For complex conditions, use qsv_sqlp.",
    commonPattern: "Combine with select: search (filter rows) → select (pick columns).",
    needsIndexHint: true,
  },
  stats: {
    whenToUse: "Quick numeric stats (mean, min/max, stddev). Creates cache for other commands. Run 2nd after index.",
    commonPattern: "Run 2nd (after index). Creates cache used by frequency, schema, tojsonl, sqlp, joinp, diff, sample.",
    errorPrevention: "Works with CSV/TSV/SSV files only. For SQL queries, use sqlp. Run qsv_index first for files >10MB.",
    complementaryServer: "🔗 CENSUS VALIDATION: If stats show US geographic columns (city, state, county, FIPS), Census MCP Server can validate codes and fetch reference demographics for comparison.",
    needsIndexHint: true,
  },
  moarstats: {
    whenToUse: "Comprehensive stats + bivariate stats + outlier details + data type inference. Slower but richer than stats.",
    commonPattern: "Index → Stats → Moarstats for richest analysis. With --bivariate: main stats to --output, bivariate stats to <FILESTEM>.stats.bivariate.csv (separate file next to input).",
    errorPrevention: "Run stats first to create cache. Slower than stats but richer output. IMPORTANT: --bivariate writes results to a SEPARATE file: <FILESTEM>.stats.bivariate.csv (located next to the input file, NOT in stdout/output). Always read this file to get bivariate results. With --join-inputs, the file is <FILESTEM>.stats.bivariate.joined.csv.",
    complementaryServer: "🔗 CENSUS VALIDATION: If analyzing US geographic data, Census MCP Server can provide demographic baselines for statistical comparison.",
    needsMemoryWarning: true,
    hasCommonMistakes: true,
  },
  frequency: {
    whenToUse: "Count unique values. Best for low-cardinality categorical columns. Run qsv_stats --cardinality first to identify high-cardinality columns to exclude.",
    commonPattern: "Stats → Frequency: Use qsv_stats --cardinality first to identify high-cardinality columns (IDs) to exclude from frequency analysis.",
    errorPrevention: "High-cardinality columns (IDs, timestamps) can produce huge output. Use qsv_stats --cardinality to inspect column cardinality before running frequency.",
    complementaryServer: "🔗 CENSUS INSIGHT: For US geographic columns, Census MCP Server can enrich frequency results with population/demographic data via `fetch-aggregate-data`.",
    needsMemoryWarning: true,
    hasCommonMistakes: true,
  },
  join: {
    whenToUse: "Join CSV files (<50MB). For large/complex joins, use qsv_joinp.",
    commonPattern: "Run qsv_index first on both files for speed.",
    errorPrevention: "Both files need join column(s). Column names case-sensitive. Check with qsv_headers.",
    hasCommonMistakes: true,
  },
  joinp: {
    whenToUse: "Fast Polars-powered joins for large files (>50MB) or SQL-like joins (inner/left/right/outer/cross). Use stats cache (qsv_stats --cardinality) to determine optimal table order (smaller cardinality on right).",
    commonPattern: "Stats → Join: Use qsv_stats --cardinality on both files, put lower-cardinality join column on right for efficiency.",
    errorPrevention: "Use --try-parsedates for date joins.",
    hasCommonMistakes: true,
  },
  dedup: {
    whenToUse: "Remove duplicates. Loads entire CSV. For large files (>1GB), use qsv_extdedup. Use qsv_stats --cardinality to check column cardinality - if key column has unique values only, dedup will be a no-op.",
    commonPattern: "Often followed by stats: dedup → stats for distribution.",
    errorPrevention: "May OOM on files >1GB. Use qsv_extdedup for large files.",
    needsMemoryWarning: true,
    hasCommonMistakes: true,
  },
  sort: {
    whenToUse: "Sort by columns. Loads entire file. For large files (>1GB), use qsv_extsort. Use stats cache to check if data is already sorted.",
    commonPattern: "Before joins or top-N: sort DESC → slice --end 10.",
    errorPrevention: "May OOM on files >1GB. Use qsv_extsort for large files.",
    needsMemoryWarning: true,
    hasCommonMistakes: true,
  },
  count: {
    whenToUse: "Count rows. Very fast with index. Run qsv_index first for files >10MB.",
    errorPrevention: "Works with CSV/TSV/SSV files only. Very fast with index file (.idx).",
  },
  headers: {
    whenToUse: "View/rename column names. Quick CSV structure discovery.",
    errorPrevention: "Works with CSV/TSV/SSV files only. For Parquet schema, use sqlp with DESCRIBE.",
  },
  sample: {
    whenToUse: "Random sampling. Fast, memory-efficient. Good for previews or test datasets.",
    commonPattern: "Quick preview (100 rows) or test data (1000 rows). Faster than qsv_slice for random.",
    needsIndexHint: true,
  },
  schema: {
    whenToUse: "Infer data types, generate Polars Schema & JSON Schema.",
    hasCommonMistakes: true,
  },
  validate: {
    whenToUse: "Validate against JSON Schema. Check data quality, type correctness. Also use this without a JSON Schema to check if a CSV is well-formed.",
    commonPattern: "Iterate: qsv_schema → validate → fix → validate until clean.",
    needsIndexHint: true,
  },
  sqlp: {
    whenToUse: "Run Polars SQL queries (PostgreSQL-like). Best for GROUP BY, aggregations, JOINs, WHERE, calculated columns. Supports CSV/TSV/SSV, Parquet, JSONL, and Arrow input. For CSV files >10MB needing SQL queries, convert to Parquet first with qsv_to_parquet (same file stem, working dir). Then query the Parquet file - prefer DuckDB if available, otherwise use sqlp with SKIP_INPUT and read_parquet().",
    commonPattern: "For CSV >10MB: convert to Parquet once (qsv_to_parquet), then query Parquet with DuckDB (preferred) or sqlp using SKIP_INPUT + read_parquet('file.parquet'). Parquet is for sqlp/DuckDB only - use CSV/TSV/SSV for all other qsv commands.",
    errorPrevention: "For CSV files >10MB, always convert to Parquet first (qsv_to_parquet with same file stem in working dir). Query the Parquet file: prefer DuckDB if available; otherwise use sqlp with SKIP_INPUT and read_parquet('file.parquet'). For complex query errors, try DuckDB. Parquet works ONLY with sqlp and DuckDB - use CSV/TSV/SSV for all other qsv commands.",
    hasCommonMistakes: true,
  },
  apply: {
    whenToUse: "Transform columns (trim, upper, lower, squeeze, strip). Subcommands: operations, dynfmt, emptyreplace, calcconv. For custom logic, use qsv_luau.",
    needsIndexHint: true,
  },
  rename: {
    whenToUse: "Rename columns. Supports bulk/regex.",
  },
  template: {
    whenToUse: "Generate formatted output from CSV using Mini Jinja templates. For reports, markdown, HTML.",
    needsIndexHint: true,
  },
  index: {
    whenToUse: "Create .idx index. Run FIRST for files >5MB queried multiple times. Enables instant counts, fast slicing.",
    commonPattern: "Run 1st for files >5MB. Makes count instant, slice 100x faster.",
    errorPrevention: "Creates .idx index for CSV/TSV/SSV files only. Parquet files don't need indexing.",
  },
  diff: {
    whenToUse: "Compare CSV files (added/deleted/modified rows). Requires same schema.",
  },
  cat: {
    whenToUse: "Concatenate CSV files. Subcommands: rows (stack vertically), rowskey (different schemas), columns (side-by-side). Specify via subcommand parameter.",
    commonPattern: "Combine files: cat rows → headers from first file only. cat rowskey → handles different schemas. cat columns → side-by-side merge.",
    errorPrevention: "rows mode requires same column order. Use rowskey for different schemas.",
    hasCommonMistakes: true,
  },
  geocode: {
    whenToUse: "Geocode locations using Geonames/MaxMind. Subcommands: suggest, reverse, countryinfo, iplookup. Specify via subcommand parameter.",
    commonPattern: "Common: suggest for city lookup, reverse for lat/lon → city, iplookup for IP → location.",
    errorPrevention: "Needs Geonames index (auto-downloads on first use). iplookup needs MaxMind GeoLite2 DB.",
    complementaryServer: "🔗 CENSUS ENRICHMENT: After geocoding US locations, use Census MCP Server to add demographics. Tools: `resolve-geography-fips` (get FIPS codes), `fetch-aggregate-data` (population, income, education). US data only.",
    needsIndexHint: true,
  },
  pivotp: {
    whenToUse: "Polars-powered pivot tables. Use --agg for aggregation (sum/mean/count/first/last/min/max/smart). Use qsv_stats --cardinality to check pivot column cardinality.",
    commonPattern: "Stats → Pivot: Use qsv_stats --cardinality to estimate pivot output width (pivot column cardinality × value columns) and keep estimated columns below ~1000 to avoid overly wide pivots.",
    errorPrevention: "High-cardinality pivot columns create wide output. Use qsv_stats --cardinality to check cardinality of potential pivot columns.",
    hasCommonMistakes: true,
  },
  excel: {
    whenToUse: "Convert spreadsheets (Excel and OpenDocument) to CSV. Also can be used to get workbook metadata. Supports multi-sheet workbooks.",
  },
  searchset: {
    errorPrevention: "Needs regex file. qsv_search easier for simple patterns.",
    needsIndexHint: true,
    hasCommonMistakes: true,
  },
  // Commands with only behavioral flags (no guidance text)
  datefmt: { needsIndexHint: true },
  luau: { needsIndexHint: true },
  replace: { needsIndexHint: true },
  split: { needsIndexHint: true },
  tojsonl: { needsIndexHint: true },
  transpose: { needsIndexHint: true },
};

/**
 * Input file size threshold (in bytes) for auto temp file
 */
const LARGE_FILE_THRESHOLD_BYTES = 10 * 1024 * 1024; // 10MB

/**
 * Maximum size for MCP response (in bytes)
 * Outputs larger than this will be saved to working directory instead of returned directly
 * Claude Desktop has a 1MB limit, so we use 850KB to stay safely under
 */
const MAX_MCP_RESPONSE_SIZE = 850 * 1024; // 850KB - safe for Claude Desktop (< 1MB limit)

/**
 * Track active child processes for graceful shutdown (SIGTERM on exit).
 */
const activeProcesses = new Set<ChildProcess>();

/**
 * Track in-flight operation count for concurrency limiting.
 * Incremented/decremented in handleToolCall to cover the entire execution path
 * (both runQsvSimple and SkillExecutor.runQsv).
 */
let activeOperationCount = 0;

/**
 * Flag indicating shutdown is in progress
 */
let isShuttingDown = false;

/**
 * Get QSV binary path (centralized)
 */
function getQsvBinaryPath(): string {
  return config.qsvBinPath;
}

/**
 * Run a qsv command with timeout, process tracking, and shutdown awareness.
 * Delegates to shared runQsvSimple for the actual spawning logic.
 */
async function runQsvWithTimeout(
  qsvBin: string,
  args: string[],
  timeoutMs: number = config.operationTimeoutMs,
): Promise<void> {
  if (isShuttingDown) {
    throw new Error("Server is shutting down, operation rejected");
  }

  await runQsvSimple(qsvBin, args, {
    timeoutMs,
    onSpawn: (proc) => activeProcesses.add(proc),
    onExit: (proc) => activeProcesses.delete(proc),
  });
}

/**
 * Check if an object has filesystem provider capabilities
 */
function isFilesystemProviderExtended(
  obj: unknown,
): obj is FilesystemProviderExtended {
  return (
    typeof obj === "object" &&
    obj !== null &&
    "resolvePath" in obj &&
    "needsConversion" in obj &&
    "getConversionCommand" in obj &&
    "getWorkingDirectory" in obj &&
    typeof (obj as any).resolvePath === "function" &&
    typeof (obj as any).needsConversion === "function" &&
    typeof (obj as any).getConversionCommand === "function" &&
    typeof (obj as any).getWorkingDirectory === "function"
  );
}

/**
 * Build arguments for file conversion commands.
 * Handles different patterns:
 * - Excel/JSONL: qsv <cmd> <input> --output <output>
 * - Parquet→CSV: qsv sqlp SKIP_INPUT "select * from read_parquet('<input>')" --output <output>
 * - CSV→Parquet: qsv sqlp <input> "SELECT * FROM _t_1" --format parquet --output <output>
 *   (passes input directly so sqlp can detect .pschema.json for type inference)
 */
export function buildConversionArgs(
  conversionCmd: string,
  inputFile: string,
  outputFile: string,
): string[] {
  if (conversionCmd === "parquet") {
    // Parquet→CSV conversion
    // Normalize path separators for SQL (Windows backslashes → forward slashes)
    const normalizedPath = inputFile.replace(/\\/g, "/");
    // Escape single quotes in path for SQL string safety
    const escapedPath = normalizedPath.replace(/'/g, "''");
    const sql = `select * from read_parquet('${escapedPath}')`;
    return ["sqlp", "SKIP_INPUT", sql, "--output", outputFile];
  }
  if (conversionCmd === "csv-to-parquet") {
    // CSV→Parquet conversion: pass input directly so sqlp can detect .pschema.json for type inference
    return [
      "sqlp",
      inputFile,
      "SELECT * FROM _t_1",
      "--format",
      "parquet",
      "--compression",
      "snappy",
      "--statistics",
      "--output",
      outputFile,
    ];
  }
  // Standard: qsv <cmd> <input> --output <output>
  return [conversionCmd, inputFile, "--output", outputFile];
}

/**
 * Auto-index a file if it's large enough and not already indexed
 * Reusable helper to avoid code duplication
 */
async function autoIndexIfNeeded(
  filePath: string,
  minSizeMB: number = AUTO_INDEX_SIZE_MB,
): Promise<void> {
  try {
    // Check if this is an indexable CSV format (not snappy-compressed)
    const filename = basename(filePath).toLowerCase();
    const isIndexable =
      filename.endsWith(".csv") ||
      filename.endsWith(".tsv") ||
      filename.endsWith(".tab") ||
      filename.endsWith(".ssv");

    if (!isIndexable) {
      return; // Not an indexable format
    }

    const stats = await stat(filePath);
    const fileSizeMB = stats.size / (1024 * 1024);
    const indexPath = filePath + ".idx";

    // Check if index already exists
    let indexExists = false;
    try {
      await access(indexPath, constants.F_OK);
      indexExists = true;
    } catch {
      indexExists = false;
    }

    // Create index if file is large enough and not already indexed
    if (fileSizeMB > minSizeMB && !indexExists) {
      console.error(
        `[MCP Tools] File is ${fileSizeMB.toFixed(1)}MB, creating index...`,
      );

      const qsvBin = getQsvBinaryPath();
      const indexArgs = ["index", filePath];

      try {
        await runQsvWithTimeout(qsvBin, indexArgs);
        console.error(`[MCP Tools] Index created successfully: ${indexPath}`);
      } catch (error) {
        // Don't fail if indexing fails or times out - just log and continue
        console.error(
          `[MCP Tools] Index creation failed (continuing anyway):`,
          error,
        );
      }
    } else if (indexExists) {
      console.error(`[MCP Tools] Index already exists: ${indexPath}`);
    } else {
      console.error(
        `[MCP Tools] File is ${fileSizeMB.toFixed(1)}MB, skipping auto-indexing`,
      );
    }
  } catch (error) {
    console.error(
      `[MCP Tools] Auto-indexing error (continuing anyway):`,
      error,
    );
  }
}

/**
 * Determine if a command should use a temp output file
 *
 * @param command - The qsv command name
 * @param inputFile - Path to the input file
 * @returns Promise<boolean> - true if temp file should be used
 */
async function shouldUseTempFile(
  command: string,
  inputFile: string,
): Promise<boolean> {
  // Metadata commands always use stdout (small results)
  if (METADATA_COMMANDS.has(command)) {
    return false;
  }

  // Commands that always return full CSV data should use temp files
  if (ALWAYS_FILE_COMMANDS.has(command)) {
    return true;
  }

  // For other commands, check input file size
  try {
    const stats = await stat(inputFile);
    return stats.size > LARGE_FILE_THRESHOLD_BYTES;
  } catch (error) {
    // If we can't stat the file, default to stdout
    console.error(
      `[MCP Tools] Error checking file size for temp file decision:`,
      error,
    );
    return false;
  }
}

/**
 * 11 most essential qsv commands exposed as individual MCP tools
 * Optimized for token efficiency while maintaining high-value tool access
 *
 * Commands promoted to CORE_TOOLS (always loaded):
 * stats, index
 *
 * Commands moved to qsv_command generic tool:
 * join, sort, dedup, apply, rename, validate, sample, template, diff, schema
 */
export const COMMON_COMMANDS = [
  "select", // Column selection (most frequently used)
  "moarstats", // Comprehensive statistics with data type inference
  "search", // Pattern-based filtering
  "frequency", // Value distribution
  "headers", // Header operations (quick discovery)
  "count", // Row counting (instant with index)
  "slice", // Row selection
  "sqlp", // SQL queries (Polars engine)
  "joinp", // High-performance joins (Polars engine)
  "cat", // Concatenate CSV files (rows/columns)
  "geocode", // Geocoding operations
] as const;

/**
 * Enhance parameter descriptions with examples and common values
 */
function enhanceParameterDescription(
  paramName: string,
  description: string,
): string {
  let enhanced = description;

  // Add examples for common parameters
  switch (paramName) {
    case "delimiter":
      enhanced += ' e.g. "," "\\t" "|" ";"';
      break;
    case "select":
      enhanced +=
        ' e.g. "1,3,5" (specific columns), "1-10" (range), "!SSN,!password" (exclude), "name,age,city" (by name), "_" (last column), "/<regex>/" (regex).';
      break;
    case "output":
    case "output_file":
      enhanced +=
        " Tip: Use absolute paths. Omit for small results (returned directly), or specify for large datasets (auto-saved if >850KB).";
      break;
    case "no_headers":
      enhanced +=
        " Use when CSV has no header row. First row will be treated as data.";
      break;
    case "ignore_case":
      enhanced += " Makes pattern matching case-insensitive.";
      break;
  }

  return enhanced;
}


/**
 * Enhance tool description with contextual guidance
 *
 * Uses concise description from README.md and adds guidance hints
 * that help Claude select the right tool. For detailed help,
 * use the qsv_help tool which calls `qsv <command> --help`.
 */
function enhanceDescription(skill: QsvSkill): string {
  const commandName = skill.command.subcommand;
  const guidance = COMMAND_GUIDANCE[commandName];

  // Use concise description from README.md
  let description = skill.description;

  // Add when-to-use guidance (critical for tool selection)
  if (guidance?.whenToUse) {
    description += `\n\n💡 ${guidance.whenToUse}`;
  }

  // Add subcommand requirement for commands that need it
  if (commandName === "cat") {
    description += `\n\n🔧 SUBCOMMAND: Must pass subcommand via args (e.g., args: {subcommand: "rows", input: "file.csv"}).`;
  } else if (commandName === "geocode") {
    description += `\n\n🔧 SUBCOMMAND: Must pass subcommand via args (e.g., args: {subcommand: "suggest", column: "city", input: "data.csv"}).`;
  }

  // Add common patterns (helps Claude compose workflows)
  if (guidance?.commonPattern) {
    description += `\n\n📋 ${guidance.commonPattern}`;
  }

  // Add performance hints only for commands that benefit from indexing
  if (skill.hints) {
    // Only show memory warnings for memory-intensive commands
    if (guidance?.needsMemoryWarning) {
      if (skill.hints.memory === "full") {
        description += "\n\n⚠️  Loads entire CSV. Best <100MB.";
      } else if (skill.hints.memory === "proportional") {
        description += "\n\n⚠️  Memory ∝ unique values.";
      }
    }

    // Only show index hints for commands that are index-accelerated
    if (guidance?.needsIndexHint && skill.hints.indexed) {
      description +=
        "\n\n🚀 Index-accelerated. Run qsv_index first on files >10MB.";
    }
  }

  // Add error prevention hints only for commands with common mistakes
  if (guidance?.hasCommonMistakes && guidance?.errorPrevention) {
    description += `\n\n⚠️  ${guidance.errorPrevention}`;
  }

  // Add usage examples from skill JSON (if available)
  // Configurable via QSV_MCP_MAX_EXAMPLES environment variable (default: 5, max: 20, 0 to disable)
  if (skill.examples && skill.examples.length > 0 && config.maxExamples > 0) {
    const maxExamples = config.maxExamples;
    const examplesToShow = skill.examples.slice(0, maxExamples);

    description += "\n\n📝 EXAMPLES:";
    for (const example of examplesToShow) {
      description += `\n• ${example.command}`;
    }

    if (skill.examples.length > maxExamples) {
      description += `\n  (${skill.examples.length - maxExamples} more - use help=true for full list)`;
    }
  }

  // Add complementary server hints (Census MCP integration)
  if (guidance?.complementaryServer) {
    description += `\n\n${guidance.complementaryServer}`;
  }

  return description;
}

/**
 * Convert a QSV skill to an MCP tool definition
 */
export function createToolDefinition(skill: QsvSkill): McpToolDefinition {
  const properties: Record<string, McpToolProperty> = {
    input_file: {
      type: "string",
      description:
        "Path to input CSV file. Use absolute paths for reliability.",
    },
  };

  const required: string[] = ["input_file"];

  // Add positional arguments
  if (skill.command.args && Array.isArray(skill.command.args)) {
    for (const arg of skill.command.args) {
      // Skip 'input' argument - we already have 'input_file' which maps to this
      if (arg.name === "input") {
        continue;
      }

      properties[arg.name] = {
        type: mapSchemaType(arg.type),
        description: arg.description,
      };

      // Add enum if present (for subcommands)
      if ("enum" in arg && Array.isArray(arg.enum) && arg.enum.length > 0) {
        properties[arg.name].enum = arg.enum;
      }

      if (arg.required) {
        required.push(arg.name);
      }
    }
  }

  // Add options
  if (skill.command.options && Array.isArray(skill.command.options)) {
    for (const opt of skill.command.options) {
      const optName = opt.flag.replace(/^--/, "").replace(/-/g, "_");

      if (opt.type === "flag") {
        properties[optName] = {
          type: "boolean",
          description: enhanceParameterDescription(optName, opt.description),
        };
      } else {
        properties[optName] = {
          type: mapSchemaType(opt.type),
          description: enhanceParameterDescription(optName, opt.description),
        };
        if (opt.default) {
          properties[optName].default = opt.default;
        }
      }
    }
  }

  // Add output_file (optional for all commands)
  properties.output_file = {
    type: "string",
    description:
      "Path to output CSV file (optional). Use absolute paths for reliability. For large results, a temp file is automatically used if omitted.",
  };

  // Add help flag (universally available for all qsv commands)
  properties.help = {
    type: "boolean",
    description:
      "Display detailed help text for this command (equivalent to --help flag). Returns usage documentation instead of executing the command.",
  };

  return {
    name: skill.name.replace("qsv-", "qsv_"),
    description: enhanceDescription(skill),
    inputSchema: {
      type: "object",
      properties,
      required: required.length > 0 ? required : undefined,
    },
  };
}

/**
 * Map QSV argument/option types to JSON Schema types
 * (Arguments and options share the same mapping logic)
 */
function mapSchemaType(
  type: string,
): "string" | "number" | "boolean" | "object" | "array" {
  switch (type) {
    case "number":
      return "number";
    case "file":
    case "regex":
    case "string":
    default:
      return "string";
  }
}

/**
 * Resolve input file path, handle format conversion (Excel/JSONL to CSV),
 * and auto-index large files. Returns the resolved (possibly converted) input path.
 */
async function resolveAndConvertInputFile(
  inputFile: string,
  filesystemProvider: FilesystemProviderExtended,
): Promise<string> {
  const originalInputFile = inputFile;
  inputFile = await filesystemProvider.resolvePath(inputFile);
  console.error(
    `[MCP Tools] Resolved input file: ${originalInputFile} -> ${inputFile}`,
  );

  // Check if file needs conversion (Excel or JSONL to CSV)
  if (
    isFilesystemProviderExtended(filesystemProvider) &&
    filesystemProvider.needsConversion(inputFile)
  ) {
    const conversionCmd = filesystemProvider.getConversionCommand(inputFile);
    if (!conversionCmd) {
      throw new Error(
        `Unable to determine conversion command for: ${inputFile}`,
      );
    }
    console.error(
      `[MCP Tools] File requires conversion using qsv ${conversionCmd}`,
    );

    const qsvBin = getQsvBinaryPath();

    // Generate unique converted file path with UUID to prevent collisions
    // 16 hex chars (64 bits) has 50% collision probability after ~4 billion conversions
    const uuid = randomUUID().replace(/-/g, "").substring(0, 16);
    let convertedPath = `${inputFile}.converted.${uuid}.csv`;

    // Validate the generated converted path for defense-in-depth
    try {
      convertedPath = await filesystemProvider.resolvePath(convertedPath);
    } catch (error) {
      throw new Error(
        `Invalid converted file path: ${convertedPath} - ${error}`,
      );
    }

    // Initialize converted file manager
    const workingDir = filesystemProvider.getWorkingDirectory();
    const convertedManager = new ConvertedFileManager(workingDir);

    // Clean up orphaned entries and partial conversions first
    await convertedManager.cleanupOrphanedEntries();

    // Check if we can reuse an existing converted file
    const baseName = basename(inputFile);
    const pattern = `${baseName}.converted.`;
    let validConverted: string | null = null;

    try {
      const dir = dirname(inputFile);
      const files = await readdir(dir);

      for (const file of files) {
        if (file.startsWith(pattern) && file.endsWith(".csv")) {
          const filePath = join(dir, file);
          validConverted =
            await convertedManager.getValidConvertedFile(
              inputFile,
              filePath,
            );
          if (validConverted) break;
        }
      }
    } catch (error) {
      console.error(
        "[MCP Tools] Error searching for existing converted file:",
        error,
      );
    }

    if (validConverted) {
      await convertedManager.touchConvertedFile(inputFile);
      inputFile = validConverted;
      console.error(
        `[MCP Tools] Reusing existing conversion: ${validConverted}`,
      );
    } else {
      await convertedManager.registerConversionStart(
        inputFile,
        convertedPath,
      );

      try {
        const conversionArgs = buildConversionArgs(
          conversionCmd,
          inputFile,
          convertedPath,
        );
        console.error(
          `[MCP Tools] Running conversion: ${qsvBin} ${conversionArgs.join(" ")}`,
        );

        await runQsvWithTimeout(qsvBin, conversionArgs);

        await convertedManager.registerConvertedFile(
          inputFile,
          convertedPath,
        );
        await convertedManager.registerConversionComplete(inputFile);
        inputFile = convertedPath;
        console.error(
          `[MCP Tools] Conversion successful: ${convertedPath}`,
        );

        await autoIndexIfNeeded(convertedPath);
      } catch (conversionError) {
        try {
          await unlink(convertedPath);
          console.error(
            `[MCP Tools] Cleaned up partial conversion file: ${convertedPath}`,
          );
        } catch {
          // cleanupPartialConversions will handle it
        }
        convertedManager.trackConversionFailure();
        throw conversionError;
      }
    }
  }

  return inputFile;
}

/**
 * Build enhanced file-not-found error message with file suggestions
 */
async function buildFileNotFoundError(
  inputFile: string,
  error: unknown,
  filesystemProvider: FilesystemProviderExtended,
): Promise<string> {
  let errorMessage = `Error resolving file path: ${error instanceof Error ? error.message : String(error)}`;

  const errorStr = error instanceof Error ? error.message : String(error);
  if (
    errorStr.includes("outside allowed") ||
    errorStr.includes("not exist") ||
    errorStr.includes("cannot access") ||
    errorStr.includes("ENOENT")
  ) {
    try {
      const { resources } = await filesystemProvider.listFiles(
        undefined,
        false,
      );

      if (resources.length > 0) {
        const suggestions = findSimilarFiles(inputFile, resources, 3);

        errorMessage += "\n\n";

        if (
          suggestions.length > 0 &&
          suggestions[0].distance <= inputFile.length / 2
        ) {
          errorMessage += "Did you mean one of these?\n";
          suggestions.forEach(({ name }) => {
            errorMessage += `  - ${name}\n`;
          });
        } else {
          errorMessage += `Available files in working directory (${filesystemProvider.getWorkingDirectory()}):\n`;
          resources.slice(0, 5).forEach((file) => {
            errorMessage += `  - ${file.name}\n`;
          });

          if (resources.length > 5) {
            errorMessage += `  ... and ${resources.length - 5} more file${resources.length - 5 !== 1 ? "s" : ""}`;
          }
        }
      }
    } catch (listError) {
      console.error(
        `[MCP Tools] Failed to list files for suggestions:`,
        listError,
      );
    }
  }

  return errorMessage;
}

/**
 * Build args and options for skill execution from MCP tool params
 */
function buildSkillExecParams(
  skill: QsvSkill,
  params: Record<string, unknown>,
  inputFile: string | undefined,
  outputFile: string | undefined,
  isHelpRequest: boolean,
): { args: Record<string, unknown>; options: Record<string, unknown> } {
  const args: Record<string, unknown> = {};
  const options: Record<string, unknown> = {};

  // Add input file as 'input' argument if the skill expects it
  if (skill.command.args.some((a) => a.name === "input")) {
    args.input = inputFile;
    console.error(`[MCP Tools] Added input arg: ${inputFile}`);
  }

  for (const [key, value] of Object.entries(params)) {
    if (
      key === "input_file" ||
      key === "output_file" ||
      key === "help" ||
      (key === "input" && args.input)
    ) {
      continue;
    }

    const isArg = skill.command.args.some((a) => a.name === key);
    if (isArg) {
      args[key] = value;
    } else {
      const optFlag = key.startsWith("-")
        ? key
        : `--${key.replace(/_/g, "-")}`;
      options[optFlag] = value;
    }
  }

  if (outputFile) {
    options["--output"] = outputFile;
  }

  if (isHelpRequest) {
    options["help"] = true;
  }

  return { args, options };
}

/**
 * Format successful tool result, handling temp files and performance tips
 */
async function formatToolResult(
  result: import("./types.js").SkillResult,
  commandName: string,
  inputFile: string | undefined,
  outputFile: string | undefined,
  autoCreatedTempFile: boolean,
  params: Record<string, unknown>,
) {
  let responseText = "";

  if (outputFile) {
    if (autoCreatedTempFile) {
      try {
        const tempFileStats = await stat(outputFile);

        if (tempFileStats.size > MAX_MCP_RESPONSE_SIZE) {
          console.error(
            `[MCP Tools] Output file (${formatBytes(tempFileStats.size)}) exceeds MCP response limit (${formatBytes(MAX_MCP_RESPONSE_SIZE)})`,
          );

          const timestamp = new Date()
            .toISOString()
            .replace(/[:.]/g, "-")
            .replace("T", "_")
            .split(".")[0];
          const savedFileName = `qsv-${commandName}-${timestamp}.csv`;
          const savedPath = join(config.workingDir, savedFileName);

          try {
            await rename(outputFile, savedPath);
          } catch (renameErr: unknown) {
            // Cross-device rename fails with EXDEV; fall back to copy + delete
            if (renameErr instanceof Error && "code" in renameErr && (renameErr as NodeJS.ErrnoException).code === "EXDEV") {
              await copyFile(outputFile, savedPath);
              await unlink(outputFile);
            } else {
              throw renameErr;
            }
          }
          console.error(`[MCP Tools] Saved large output to: ${savedPath}`);

          responseText = `✅ Large output saved to file (too large to display in chat)\n\n`;
          responseText += `File: ${savedFileName}\n`;
          responseText += `Location: ${config.workingDir}\n`;
          responseText += `Size: ${formatBytes(tempFileStats.size)}\n`;
          responseText += `Duration: ${result.metadata.duration}ms\n\n`;
          responseText += `The file is now available in your working directory and can be processed with additional qsv commands.`;
        } else {
          console.error(
            `[MCP Tools] Output file (${formatBytes(tempFileStats.size)}) is small enough to return directly`,
          );
          const fileContents = await readFile(outputFile, "utf-8");

          try {
            await unlink(outputFile);
            console.error(`[MCP Tools] Deleted temp file: ${outputFile}`);
          } catch (unlinkError) {
            console.error(
              `[MCP Tools] Failed to delete temp file:`,
              unlinkError,
            );
          }

          responseText = fileContents;
        }
      } catch (readError) {
        console.error(
          `[MCP Tools] Failed to process temp file:`,
          readError,
        );
        return errorResult(`Error processing output from temp file: ${readError instanceof Error ? readError.message : String(readError)}`);
      }
    } else {
      responseText = `Successfully wrote output to: ${outputFile}\n\n`;
      responseText += `Metadata:\n`;
      responseText += `- Command: ${result.metadata.command}\n`;
      responseText += `- Duration: ${result.metadata.duration}ms\n`;
      if (result.metadata.rowsProcessed) {
        responseText += `- Rows processed: ${result.metadata.rowsProcessed}\n`;
      }
    }
  } else {
    responseText = result.output;
  }

  // sqlp performance tip for large CSV files
  if (commandName === "sqlp" && inputFile) {
    try {
      const filename = basename(inputFile).toLowerCase();
      const isCsvLike =
        filename.endsWith(".csv") ||
        filename.endsWith(".tsv") ||
        filename.endsWith(".tab") ||
        filename.endsWith(".ssv");
      if (isCsvLike) {
        const fileStats = await stat(inputFile);
        if (fileStats.size > LARGE_FILE_THRESHOLD_BYTES) {
          const sizeMB = (fileStats.size / (1024 * 1024)).toFixed(1);
          responseText =
            `⚡ PERFORMANCE TIP: This CSV is ${sizeMB}MB. Convert to Parquet first with qsv_to_parquet for dramatically faster SQL queries. ` +
            `Prefer DuckDB if available; otherwise use sqlp with input_file="SKIP_INPUT" and sql="SELECT ... FROM read_parquet('file.parquet')".\n\n` +
            responseText;
        }
      }
    } catch {
      // Ignore stat errors (e.g. SKIP_INPUT)
    }
  }

  // moarstats bivariate output notification (skip for help requests where inputFile is undefined)
  if (commandName === "moarstats" && params.bivariate && inputFile) {
    const inputDir = dirname(inputFile as string);
    const inputStem = basename(inputFile as string).replace(/\.[^.]+$/, "");
    const bivariateFileName = params.join_inputs
      ? `${inputStem}.stats.bivariate.joined.csv`
      : `${inputStem}.stats.bivariate.csv`;
    const bivariatePath = join(inputDir, bivariateFileName);

    responseText += `\n\n📊 Bivariate statistics were written to a SEPARATE file:\n`;
    responseText += `File: ${bivariateFileName}\n`;
    responseText += `Location: ${bivariatePath}\n`;
    responseText += `Use qsv_command or read this file to view the bivariate correlation results.`;
  }

  return successResult(responseText);
}

/**
 * Handle execution of a qsv tool
 */
export async function handleToolCall(
  toolName: string,
  params: Record<string, unknown>,
  executor: SkillExecutor,
  loader: SkillLoader,
  filesystemProvider?: FilesystemProviderExtended,
) {
  // Check concurrent operation limit
  if (activeOperationCount >= config.maxConcurrentOperations) {
    return errorResult(`Error: Maximum concurrent operations limit reached (${config.maxConcurrentOperations}). Please wait for current operations to complete.`);
  }

  activeOperationCount++;
  try {
    // Extract command name from tool name (qsv_select -> select)
    const commandName = toolName.replace("qsv_", "");

    // Load the skill
    const skillName = `qsv-${commandName}`;
    const skill = await loader.load(skillName);

    if (!skill) {
      const totalCommands = loader.getStats().total;
      const remainingCommands = totalCommands - COMMON_COMMANDS.length;

      return errorResult(
        `Error: Skill '${skillName}' not found.\n\n` +
        `Please verify the command name is correct. ` +
        `Available commands include: ${COMMON_COMMANDS.join(", ")}, and ${remainingCommands} others. ` +
        `Use 'qsv_command' with the 'command' parameter for less common commands.`,
      );
    }

    // Extract input_file and output_file
    let inputFile = params.input_file as string | undefined;
    let outputFile = params.output_file as string | undefined;
    const isHelpRequest = params.help === true;

    if (!inputFile && !isHelpRequest) {
      return errorResult("Error: input_file parameter is required (unless using help=true to view command documentation)");
    }

    // Resolve file paths using filesystem provider if available (skip for help requests)
    if (filesystemProvider && inputFile) {
      try {
        inputFile = await resolveAndConvertInputFile(inputFile, filesystemProvider);

        // Auto-index native CSV files (skip for help requests)
        if (!isHelpRequest) {
          await autoIndexIfNeeded(inputFile);
        }

        if (outputFile) {
          const originalOutputFile = outputFile;
          outputFile = await filesystemProvider.resolvePath(outputFile);
          console.error(
            `[MCP Tools] Resolved output file: ${originalOutputFile} -> ${outputFile}`,
          );
        }
      } catch (error) {
        console.error(`[MCP Tools] Error resolving file path:`, error);
        const errorMessage = await buildFileNotFoundError(
          inputFile,
          error,
          filesystemProvider,
        );
        return errorResult(errorMessage);
      }
    }

    // Determine if we should use a temp file for output (skip for help requests)
    let autoCreatedTempFile = false;
    if (
      !outputFile &&
      !isHelpRequest &&
      inputFile &&
      (await shouldUseTempFile(commandName, inputFile))
    ) {
      const tempFileName = `qsv-output-${randomUUID()}.csv`;
      outputFile = join(tmpdir(), tempFileName);
      autoCreatedTempFile = true;
      console.error(`[MCP Tools] Auto-created temp output file: ${outputFile}`);
    }

    // Build execution parameters
    const { args, options } = buildSkillExecParams(
      skill,
      params,
      inputFile,
      outputFile,
      isHelpRequest,
    );

    console.error(
      `[MCP Tools] Executing skill with args:`,
      JSON.stringify(args),
    );
    console.error(
      `[MCP Tools] Executing skill with options:`,
      JSON.stringify(options),
    );

    // Execute the skill
    const result = await executor.execute(skill, { args, options });

    // Format and return result
    if (result.success) {
      return await formatToolResult(
        result,
        commandName,
        inputFile,
        outputFile,
        autoCreatedTempFile,
        params,
      );
    } else {
      return errorResult(`Error executing ${commandName}:\n${result.stderr}`);
    }
  } catch (error) {
    return errorResult(`Unexpected error: ${error instanceof Error ? error.message : String(error)}`);
  } finally {
    activeOperationCount--;
  }
}

/**
 * Handle execution of the generic qsv_command tool
 */
export async function handleGenericCommand(
  params: Record<string, unknown>,
  executor: SkillExecutor,
  loader: SkillLoader,
  filesystemProvider?: FilesystemProviderExtended,
) {
  try {
    const commandName = params.command as string | undefined;

    if (!commandName) {
      return errorResult("Error: command parameter is required");
    }

    // Flatten nested args and options objects into the params
    // This handles cases where Claude passes:
    // {"command": "apply", "args": {...}, "options": {...}, "input_file": "...", "output_file": "..."}
    const flattenedParams: Record<string, unknown> = {};

    // Copy top-level params (except 'args' and 'options')
    for (const [key, value] of Object.entries(params)) {
      if (key !== "args" && key !== "options") {
        flattenedParams[key] = value;
      }
    }

    // Flatten nested 'args' object
    if (params.args && typeof params.args === "object") {
      const argsObj = params.args as Record<string, unknown>;
      for (const [key, value] of Object.entries(argsObj)) {
        flattenedParams[key] = value;
      }
    }

    // Flatten nested 'options' object
    if (params.options && typeof params.options === "object") {
      const optionsObj = params.options as Record<string, unknown>;
      for (const [key, value] of Object.entries(optionsObj)) {
        flattenedParams[key] = value;
      }
    }

    console.error(
      `[handleGenericCommand] Flattened params:`,
      JSON.stringify(flattenedParams),
    );

    // Forward to handleToolCall with the qsv_ prefix and flattened params
    return await handleToolCall(
      `qsv_${commandName}`,
      flattenedParams,
      executor,
      loader,
      filesystemProvider,
    );
  } catch (error) {
    return errorResult(`Unexpected error: ${error instanceof Error ? error.message : String(error)}`);
  }
}

/**
 * Create the generic qsv_command tool definition
 */
export function createGenericToolDefinition(
  loader: SkillLoader,
): McpToolDefinition {
  // Calculate remaining commands dynamically
  const totalCommands = loader.getStats().total;
  const remainingCommands = totalCommands - COMMON_COMMANDS.length;

  return {
    name: "qsv_command",
    description: `Execute any qsv command not exposed as a dedicated tool (${remainingCommands} additional commands available).

Common commands via this tool: join, sort, dedup, apply, rename, validate, sample, template, diff, schema, and 40+ more.

❓ HELP: For any command details, use options={"--help": true}. Example: command="apply", options={"--help": true}`,
    inputSchema: {
      type: "object",
      properties: {
        command: {
          type: "string",
          description:
            'The qsv command to execute (e.g., "to", "sample", "partition")',
        },
        input_file: {
          type: "string",
          description: "Path to input CSV file (absolute or relative)",
        },
        args: {
          type: "object",
          description: "Command arguments as key-value pairs",
        },
        options: {
          type: "object",
          description: "Command options as key-value pairs",
        },
        output_file: {
          type: "string",
          description:
            "Path to output CSV file (optional). For large results or data transformation commands, a temp file is automatically used if omitted.",
        },
      },
      required: ["command", "input_file"],
    },
  };
}

/**
 * Create qsv_config tool definition
 */
export function createConfigTool(): McpToolDefinition {
  return {
    name: "qsv_config",
    description:
      "Display current qsv configuration (binary path, version, working directory, etc.)",
    inputSchema: {
      type: "object",
      properties: {},
      required: [],
    },
  };
}

/**
 * Handle qsv_config tool call
 */
export async function handleConfigTool(
  filesystemProvider?: FilesystemProviderExtended,
): Promise<{ content: Array<{ type: string; text: string }> }> {
  const validation = config.qsvValidation;
  const extensionMode = config.isExtensionMode;

  let configText = `# qsv Configuration\n\n`;

  // qsv Binary Information
  configText += `## qsv Binary\n\n`;
  if (validation.valid) {
    configText += `✅ **Status:** Validated\n`;
    configText += `📍 **Path:** \`${validation.path}\`\n`;
    configText += `🏷️ **Version:** ${validation.version}\n`;
    if (validation.commandCount) {
      configText += `🔧 **Available Commands:** ${validation.commandCount}\n`;
    }
    if (validation.totalMemory) {
      configText += `💾 **System Total Memory:** ${validation.totalMemory}\n`;
    }
  } else {
    configText += `❌ **Status:** Validation Failed\n`;
    configText += `⚠️ **Error:** ${validation.error}\n`;

    // Show auto-detection diagnostics
    const diagnostics = getDetectionDiagnostics();
    if (diagnostics.whichAttempted) {
      configText += `\n### 🔍 Auto-Detection Diagnostics\n\n`;

      // Show which/where attempt
      configText += `**PATH search (which/where):**\n`;
      if (diagnostics.whichResult) {
        configText += `✅ Found: \`${diagnostics.whichResult}\`\n\n`;
      } else if (diagnostics.whichError) {
        configText += `❌ Failed: ${diagnostics.whichError}\n\n`;
      } else {
        configText += `❌ Not found in PATH\n\n`;
      }

      // Show common locations checked
      if (diagnostics.locationsChecked.length > 0) {
        configText += `**Common locations checked:**\n\n`;
        diagnostics.locationsChecked.forEach((loc) => {
          configText += `- \`${loc.path}\`\n`;
          if (loc.exists) {
            configText += `  - ✅ File exists\n`;
            if (loc.isFile !== undefined) {
              configText += `  - ${loc.isFile ? "✅" : "❌"} Is regular file: ${loc.isFile}\n`;
            }
            if (loc.executable !== undefined) {
              configText += `  - ${loc.executable ? "✅" : "❌"} Executable: ${loc.executable}\n`;
            }
            if (loc.version) {
              configText += `  - ✅ Version: ${loc.version}\n`;
            }
            if (loc.error) {
              configText += `  - ⚠️ Error: ${loc.error}\n`;
            }
          } else {
            configText += `  - ❌ Does not exist\n`;
            if (loc.error) {
              configText += `  - ⚠️ Error: ${loc.error}\n`;
            }
          }
        });
        configText += `\n`;
      }
    }
  }

  // Working Directory
  configText += `\n## Working Directory\n\n`;
  if (filesystemProvider) {
    const workingDir = filesystemProvider.getWorkingDirectory();
    configText += `📁 **Current:** \`${workingDir}\`\n`;
  } else {
    configText += `📁 **Current:** \`${config.workingDir}\`\n`;
  }

  // Allowed Directories
  configText += `\n## Allowed Directories\n\n`;
  if (config.allowedDirs.length > 0) {
    configText += `🔓 **Access granted to:**\n`;
    config.allowedDirs.forEach((dir) => {
      configText += `   - \`${dir}\`\n`;
    });
  } else {
    configText += `ℹ️ Only working directory is accessible\n`;
  }
  if (config.isPluginMode) {
    configText += `\n📌 _Plugin mode: additional directories are auto-added as needed at runtime._\n`;
  }

  // Performance Settings
  configText += `\n## Performance Settings\n\n`;
  configText += `⏱️ **Timeout:** ${config.operationTimeoutMs}ms (${Math.round(config.operationTimeoutMs / 1000)}s)\n`;
  configText += `💾 **Max Output Size:** ${formatBytes(config.maxOutputSize)}\n`;
  configText += `🔧 **Auto-Regenerate Skills:** ${config.autoRegenerateSkills ? "Enabled" : "Disabled"}\n`;

  // Update Check Settings
  configText += `\n## Update Settings\n\n`;
  configText += `🔍 **Check Updates on Startup:** ${config.checkUpdatesOnStartup ? "Enabled" : "Disabled"}\n`;
  configText += `📢 **Update Notifications:** ${config.notifyUpdates ? "Enabled" : "Disabled"}\n`;

  // Mode
  configText += `\n## Deployment Mode\n\n`;
  if (config.isPluginMode) {
    configText += `🔌 **Claude Plugin Mode** (relaxed directory security)\n`;
  } else if (extensionMode) {
    configText += `🧩 **Desktop Extension Mode**\n`;
  } else {
    configText += `🖥️ **Legacy MCP Server Mode**\n`;
  }

  // Help Text
  configText += `\n---\n\n`;
  if (!validation.valid) {
    configText += `### ⚠️ Action Required\n\n`;
    if (extensionMode) {
      configText += `To fix the qsv binary issue:\n`;
      configText += `1. Install qsv from https://github.com/dathere/qsv#installation\n`;
      configText += `2. Open Claude Desktop Settings > Extensions > qsv\n`;
      configText += `3. Update "qsv Binary Path" or ensure qsv is in your system PATH\n`;
      configText += `4. Save settings (extension will auto-restart)\n`;
    } else {
      configText += `To fix the qsv binary issue:\n`;
      configText += `1. Install qsv from https://github.com/dathere/qsv#installation\n`;
      configText += `2. Ensure qsv is in your PATH or set QSV_MCP_BIN_PATH\n`;
      configText += `3. Restart the MCP server\n`;
    }
  } else {
    configText += `### 💡 Tip\n\n`;
    configText += `These are the actual resolved values used by the server. The configuration UI may show template variables like \`\${HOME}/Downloads\` which get expanded to the paths shown above.\n`;
  }

  return {
    content: [{ type: "text", text: configText }],
  };
}

/**
 * Initiate graceful shutdown
 */
export function initiateShutdown(): void {
  isShuttingDown = true;
  console.error(
    `[MCP Tools] Shutdown initiated, ${activeOperationCount} active operations, ${activeProcesses.size} tracked processes`,
  );
}

/**
 * Kill all active child processes for graceful shutdown.
 */
export function killAllProcesses(): void {
  for (const proc of activeProcesses) {
    try {
      proc.kill("SIGTERM");
    } catch {
      // Process might have already exited
    }
  }
  activeProcesses.clear();
  console.error("[MCP Tools] All child processes terminated");
}

/**
 * Get count of active child processes tracked for shutdown
 */
export function getActiveProcessCount(): number {
  return activeProcesses.size;
}

/**
 * Get count of active operations (in-flight tool calls)
 */
export function getActiveOperationCount(): number {
  return activeOperationCount;
}

/**
 * Create qsv_search_tools tool definition
 * Enables tool discovery for MCP clients without native tool search
 */
export function createSearchToolsTool(): McpToolDefinition {
  return {
    name: "qsv_search_tools",
    description: `Search for qsv tools by keyword, category, or use case.

💡 USE WHEN:
- Looking for the right qsv command for a specific task
- Discovering available commands by category (filtering, transformation, etc.)
- Finding commands by capability (regex, SQL, joins, etc.)

🔍 SEARCH MODES:
- **Keyword**: Matches tool names, descriptions, and examples
- **Category**: Filter by category (selection, filtering, transformation, aggregation, joining, validation, formatting, conversion, analysis, utility)
- **Regex**: Use regex patterns for advanced matching

📋 RETURNS: List of matching tools with names and descriptions, suitable for tool discovery.`,
    inputSchema: {
      type: "object",
      properties: {
        query: {
          type: "string",
          description:
            'Search query - keyword, regex pattern, or natural language description. Examples: "join", "duplicate", "SQL", "/sort|order/", "remove columns"',
        },
        category: {
          type: "string",
          description:
            "Filter by category: selection, filtering, transformation, aggregation, joining, validation, formatting, conversion, analysis, utility",
          enum: [
            "selection",
            "filtering",
            "transformation",
            "aggregation",
            "joining",
            "validation",
            "formatting",
            "conversion",
            "analysis",
            "utility",
          ],
        },
        limit: {
          type: "number",
          description:
            "Maximum number of results to return (default: 5, max: 20)",
        },
      },
      required: ["query"],
    },
  };
}

/**
 * Handle qsv_search_tools call
 * Searches loaded skills and returns matching tools
 * Marks found tools as loaded for deferred loading
 *
 * @param params - Search parameters (query, category, limit)
 * @param loader - SkillLoader instance for searching skills
 * @param loadedTools - Optional Set to track loaded tools for deferred loading
 */
export async function handleSearchToolsCall(
  params: Record<string, unknown>,
  loader: SkillLoader,
  loadedTools?: Set<string>,
): Promise<{ content: Array<{ type: string; text: string }> }> {
  const query = params.query as string;
  const category = params.category as string | undefined;
  const limit = Math.min(Math.max(1, (params.limit as number) || 5), 20);

  if (!query || query.trim().length === 0) {
    return {
      content: [
        {
          type: "text",
          text: "Error: query parameter is required",
        },
      ],
    };
  }

  // Ensure all skills are loaded
  await loader.loadAll();

  // Search using the loader's search method
  let results = loader.search(query);

  // Apply category filter if specified
  if (category) {
    results = results.filter((skill) => skill.category === category);
  }

  // Also check if query matches category name (for discovery)
  const categories = [
    "selection",
    "filtering",
    "transformation",
    "aggregation",
    "joining",
    "validation",
    "formatting",
    "conversion",
    "analysis",
    "utility",
  ];
  const queryLower = query.toLowerCase();
  const matchedCategory = categories.find((cat) => queryLower.includes(cat));

  if (matchedCategory && !category) {
    // Add skills from matching category that weren't already found
    const categorySkills = loader.getByCategory(matchedCategory as any);
    const existingNames = new Set(results.map((r) => r.name));
    for (const skill of categorySkills) {
      if (!existingNames.has(skill.name)) {
        results.push(skill);
      }
    }
  }

  // Try regex matching if query looks like a regex pattern
  const isRegexPattern = query.startsWith("/") && query.endsWith("/");
  if (isRegexPattern) {
    try {
      const regexStr = query.slice(1, -1);
      const regex = new RegExp(regexStr, "i");
      const allSkills = loader.getAll();

      results = allSkills.filter(
        (skill) =>
          regex.test(skill.name) ||
          regex.test(skill.description) ||
          regex.test(skill.command.subcommand) ||
          skill.examples.some((ex) => regex.test(ex.description)),
      );
    } catch (regexError) {
      // Invalid regex, fall back to text search (already done above)
    }
  }

  // Sort by relevance (exact name match first, then description match)
  results.sort((a, b) => {
    const queryLower = query.toLowerCase();
    const aNameMatch = a.name.toLowerCase().includes(queryLower) ? 1 : 0;
    const bNameMatch = b.name.toLowerCase().includes(queryLower) ? 1 : 0;
    const aCommandMatch = a.command.subcommand
      .toLowerCase()
      .includes(queryLower)
      ? 1
      : 0;
    const bCommandMatch = b.command.subcommand
      .toLowerCase()
      .includes(queryLower)
      ? 1
      : 0;

    const aScore = aNameMatch * 2 + aCommandMatch * 2;
    const bScore = bNameMatch * 2 + bCommandMatch * 2;

    return bScore - aScore;
  });

  // Limit results
  const limitedResults = results.slice(0, limit);

  // Mark found tools as loaded for deferred loading
  // This allows them to appear in subsequent ListTools responses
  if (loadedTools) {
    for (const skill of limitedResults) {
      const toolName = skill.name.replace("qsv-", "qsv_");
      loadedTools.add(toolName);
    }
    console.error(
      `[MCP Tools] Marked ${limitedResults.length} tools as loaded for deferred loading`,
    );
  }

  if (limitedResults.length === 0) {
    // Provide helpful suggestions
    const allCategories = loader.getCategories();
    const totalSkills = loader.getStats().total;

    return {
      content: [
        {
          type: "text",
          text:
            `No tools found matching "${query}".\n\n` +
            `Try:\n` +
            `- Different keywords (e.g., "filter", "join", "sort", "stats")\n` +
            `- Category filter: ${allCategories.join(", ")}\n` +
            `- Regex pattern: /pattern/\n\n` +
            `Total available tools: ${totalSkills}`,
        },
      ],
    };
  }

  // Format results as tool references
  let resultText = `Found ${results.length} tool${results.length !== 1 ? "s" : ""} matching "${query}"`;
  if (category) {
    resultText += ` in category "${category}"`;
  }
  if (results.length > limit) {
    resultText += ` (showing top ${limit})`;
  }
  resultText += ":\n\n";

  for (const skill of limitedResults) {
    const toolName = skill.name.replace("qsv-", "qsv_");
    // Truncate description to first sentence for conciseness
    let shortDesc = skill.description.split(".")[0];
    if (shortDesc.length > 100) {
      shortDesc = shortDesc.substring(0, 97) + "...";
    }

    // Get when-to-use guidance if available
    const whenToUse = COMMAND_GUIDANCE[skill.command.subcommand]?.whenToUse;

    resultText += `**${toolName}** [${skill.category}]\n`;
    resultText += `  ${shortDesc}\n`;
    if (whenToUse) {
      resultText += `  💡 ${whenToUse}\n`;
    }
    resultText += "\n";
  }

  // Add tip for using the tools
  resultText += `---\n`;
  resultText += `💡 To use a tool, call it directly: e.g., \`qsv_${limitedResults[0].command.subcommand}\` with \`input_file\` parameter.\n`;
  resultText += `📖 For detailed help on any command, use \`help: true\` parameter.`;

  return {
    content: [
      {
        type: "text",
        text: resultText,
      },
    ],
  };
}

/**
 * Create qsv_to_parquet tool definition
 * Converts CSV files to Parquet format for optimized SQL operations
 */
export function createToParquetTool(): McpToolDefinition {
  return {
    name: "qsv_to_parquet",
    description: `Convert CSV to Parquet format with guaranteed data type inference.

💡 USE WHEN: CSV file is >10MB and needs SQL queries. Convert once with same file stem in working directory, then query the Parquet file. Prefer DuckDB if available; otherwise use sqlp with SKIP_INPUT and read_parquet('file.parquet').

📋 AUTO-OPTIMIZATION: Runs stats with --infer-dates --dates-whitelist sniff for automatic Date/DateTime detection. Generates Polars schema for correct data types (integers, floats, dates, booleans).

📋 COMMON PATTERN: Convert once, query many times with DuckDB or sqlp SKIP_INPUT + read_parquet(). Parquet is for sqlp/DuckDB ONLY. Keep CSV/TSV/SSV for all other qsv commands.

⚠️ IMPORTANT: Parquet files work ONLY with sqlp and DuckDB. All other qsv commands (including joinp and pivotp) require CSV/TSV/SSV input.`,
    inputSchema: {
      type: "object",
      properties: {
        input_file: {
          type: "string",
          description: "Path to input CSV file to convert",
        },
        output_file: {
          type: "string",
          description:
            "Path for output Parquet file (optional - defaults to input_file.parquet in same directory)",
        },
      },
      required: ["input_file"],
    },
  };
}

/**
 * Handle qsv_to_parquet tool call
 * Converts CSV to Parquet using sqlp's read_csv and --format parquet
 */
export async function handleToParquetCall(
  params: Record<string, unknown>,
  filesystemProvider?: FilesystemProviderExtended,
): Promise<{
  content: Array<{ type: string; text: string }>;
  isError?: boolean;
}> {
  let inputFile = params.input_file as string | undefined;
  let outputFile = params.output_file as string | undefined;

  if (!inputFile) {
    return errorResult("Error: input_file parameter is required");
  }

  // Resolve input file path using filesystem provider if available
  if (filesystemProvider) {
    try {
      const originalInputFile = inputFile;
      inputFile = await filesystemProvider.resolvePath(inputFile);
      console.error(
        `[MCP Tools] Resolved input file: ${originalInputFile} -> ${inputFile}`,
      );
    } catch (error) {
      return errorResult(`Error resolving input file path: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  // Generate output path if not provided
  if (!outputFile) {
    // Replace common CSV-like extensions with .parquet, or append .parquet if none match
    const lowerInput = inputFile.toLowerCase();
    // Supported CSV-like extensions, including snappy-compressed variants
    const csvLikeExtensions = [
      ".csv.sz",
      ".tsv.sz",
      ".tab.sz",
      ".ssv.sz",
      ".csv",
      ".tsv",
      ".tab",
      ".ssv",
    ];

    let matched = false;
    for (const ext of csvLikeExtensions) {
      if (lowerInput.endsWith(ext)) {
        outputFile = inputFile.slice(0, -ext.length) + ".parquet";
        matched = true;
        break;
      }
    }

    if (!matched) {
      outputFile = inputFile + ".parquet";
    }
  }

  // At this point outputFile is guaranteed to be defined (either provided or generated)
  let resolvedOutputFile: string = outputFile as string;

  // Resolve output file path using filesystem provider if available
  if (filesystemProvider) {
    try {
      const originalOutputFile = resolvedOutputFile;
      resolvedOutputFile =
        await filesystemProvider.resolvePath(resolvedOutputFile);
      console.error(
        `[MCP Tools] Resolved output file: ${originalOutputFile} -> ${resolvedOutputFile}`,
      );
    } catch (error) {
      return errorResult(`Error resolving output file path: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  const qsvBin = getQsvBinaryPath();
  const startTime = Date.now();
  const statsFile = inputFile + ".stats.csv";
  const schemaFile = inputFile + ".pschema.json";

  // Check if regeneration is needed based on file mtimes
  let needStats = true;
  let needSchema = true;

  try {
    const inputFileStats = await stat(inputFile);

    try {
      const existingStats = await stat(statsFile);
      if (existingStats.mtimeMs >= inputFileStats.mtimeMs) {
        needStats = false;
      }
    } catch {
      // Stats cache doesn't exist - need to generate
    }

    try {
      const existingSchema = await stat(schemaFile);
      if (existingSchema.mtimeMs >= inputFileStats.mtimeMs) {
        needSchema = false;
      }
    } catch {
      // Schema doesn't exist - need to generate
    }
  } catch {
    // Can't stat input file - fall back to regenerating
  }

  try {
    // Step 1: Generate stats cache for accurate type inference (if needed)
    // This creates .stats.csv and .stats.csv.data.jsonl files
    // Uses --dates-whitelist sniff to let qsv stats detect Date/DateTime columns natively
    if (needStats) {
      console.error(
        `[MCP Tools] Step 1: Generating stats cache for ${inputFile}`,
      );
      const statsArgs = [
        "stats",
        inputFile,
        "--cardinality",
        "--stats-jsonl",
        "--infer-dates",
        "--dates-whitelist",
        "sniff",
      ];
      await runQsvWithTimeout(qsvBin, statsArgs);
      console.error(`[MCP Tools] Stats cache generated successfully`);
    } else {
      console.error(
        `[MCP Tools] Step 1: Using existing stats cache (up-to-date)`,
      );
    }

    // Step 2: Generate Polars schema using the stats cache (if needed)
    // This creates a .pschema.json file that sqlp will auto-detect
    if (needSchema) {
      console.error(
        `[MCP Tools] Step 2: Generating Polars schema for ${inputFile}`,
      );
      const schemaArgs = ["schema", "--polars", inputFile];
      await runQsvWithTimeout(qsvBin, schemaArgs);
      console.error(`[MCP Tools] Polars schema generated: ${schemaFile}`);
    } else {
      console.error(
        `[MCP Tools] Step 2: Using existing Polars schema (up-to-date)`,
      );
    }

    // Step 3: Convert to Parquet (sqlp will auto-detect .pschema.json)
    console.error(`[MCP Tools] Step 3: Converting to Parquet with schema`);
    const conversionArgs = buildConversionArgs(
      "csv-to-parquet",
      inputFile,
      resolvedOutputFile,
    );
    console.error(
      `[MCP Tools] Running CSV→Parquet conversion: ${qsvBin} ${conversionArgs.join(" ")}`,
    );
    await runQsvWithTimeout(qsvBin, conversionArgs);
    const duration = Date.now() - startTime;

    // Get output file size for reporting
    let fileSizeInfo = "";
    try {
      const outputStats = await stat(resolvedOutputFile);
      fileSizeInfo = ` (${formatBytes(outputStats.size)})`;
    } catch {
      // Ignore stat errors
    }

    const statsStatus = needStats ? "generated" : "reused (up-to-date)";
    const schemaStatus = needSchema ? "generated" : "reused (up-to-date)";

    return successResult(
      `✅ Successfully converted CSV to Parquet with optimized schema\n\n` +
      `Input: ${inputFile}\n` +
      `Output: ${resolvedOutputFile}${fileSizeInfo}\n` +
      `Schema: ${schemaFile}\n` +
      `Duration: ${duration}ms\n\n` +
      `Stats cache: ${statsStatus}\n` +
      `Polars schema: ${schemaStatus}\n` +
      `The Parquet file is now ready for fast SQL queries.\n` +
      `Prefer DuckDB if available. Otherwise use: qsv_sqlp with input_file="SKIP_INPUT" and sql="SELECT ... FROM read_parquet('${resolvedOutputFile}')".`,
    );
  } catch (error) {
    return errorResult(`Error converting CSV to Parquet: ${error instanceof Error ? error.message : String(error)}`);
  }
}
