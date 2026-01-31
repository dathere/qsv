/**
 * MCP Tool Definitions and Handlers for QSV Commands
 */

import { spawn, type ChildProcess } from "child_process";
import { stat, access } from "fs/promises";
import { constants } from "fs";
import { basename } from "path";
import { ConvertedFileManager } from "./converted-file-manager.js";
import { ProfileCacheManager } from "./profile-cache-manager.js";
import type {
  QsvSkill,
  Argument,
  Option,
  McpToolDefinition,
  McpToolProperty,
  FilesystemProviderExtended,
} from "./types.js";
import type { SkillExecutor } from "./executor.js";
import type { SkillLoader } from "./loader.js";
import { config, getDetectionDiagnostics } from "./config.js";
import { formatBytes, findSimilarFiles } from "./utils.js";

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
  "sort",
  "dedup",
  "join",
  "joinp",
  "select",
  "search",
  "searchset",
  "apply",
  "applydp",
  "schema",
  "validate",
  "diff",
  "cat",
  "transpose",
  "flatten",
  "unflatten",
  "partition",
  "split",
  "explode",
  "pseudo",
  "rename",
  "replace",
  "datefmt",
  "formatters",
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
const METADATA_COMMANDS = new Set([
  "count",
  "headers",
  "index",
  "slice",
  "sample",
  "sniff",
]);

/**
 * Guidance for when to use each command - helps Claude make smart decisions
 */
const WHEN_TO_USE_GUIDANCE: Record<string, string> = {
  select:
    'Choose columns. Syntax: "1,3,5" (specific), "1-10" (range), "!SSN" (exclude), "/<regex>/" (pattern), "_" (last).',
  slice: "Select rows by position: first N, last N, skip N, range.",
  search:
    "Filter rows matching pattern/regex. For complex conditions, use qsv_sqlp.",
  stats:
    "Quick numeric stats (mean, min/max, stddev). Creates cache for other commands. Run 2nd after index.",
  moarstats:
    "Comprehensive stats + bivariate stats + data type inference. Slower but richer than stats.",
  frequency:
    "Count unique values. Best for low-cardinality categorical columns. 📊 Run qsv_data_profile first to identify high- or near-high-cardinality columns (high cardinality or uniqueness_ratio close to 1, often marked <ALL_UNIQUE>) to exclude.",
  join: "Join CSV files (<50MB). For large/complex joins, use qsv_joinp.",
  joinp:
    "Fast Polars-powered joins for large files (>50MB) or SQL-like joins (inner/left/right/outer/cross). 📊 Run qsv_data_profile first to determine optimal table order (smaller cardinality on right).",
  dedup:
    "Remove duplicates. Loads entire CSV. For large files (>1GB), use qsv_extdedup. 📊 Run qsv_data_profile first - uniqueness_ratio=1 means no duplicates exist.",
  sort: "Sort by columns. Loads entire file. For large files (>1GB), use qsv_extsort. 📊 Run qsv_data_profile first - if sort_order shows Ascending/Descending, data is pre-sorted.",
  count:
    "Count rows. Very fast with index. Run qsv_index first for files >10MB.",
  headers: "View/rename column names. Quick CSV structure discovery.",
  sample:
    "Random sampling. Fast, memory-efficient. Good for previews or test datasets.",
  schema:
    "Infer data types, generate Polars Schema & JSON Schema.",
  validate:
    "Validate against JSON Schema. Check data quality, type correctness. Also use this without a JSON Schema to check if a CSV is well-formed.",
  sqlp: "Run Polars SQL queries (PostgreSQL-like). Best for GROUP BY, aggregations, JOINs, WHERE, calculated columns. 📊 For optimal queries: Run qsv_data_profile first to understand column cardinalities, value distributions, null patterns, and data types.",
  apply:
    "Transform columns (trim, upper, lower, squeeze, strip). For custom logic, use qsv_luau.",
  rename:
    "Rename columns. Supports bulk/regex. For simple changes, qsv_headers faster.",
  template:
    "Generate formatted output from CSV using Mini Jinja templates. For reports, markdown, HTML.",
  index:
    "Create .idx index. Run FIRST for files >5MB queried multiple times. Enables instant counts, fast slicing.",
  diff: "Compare CSV files (added/deleted/modified rows). Requires same schema.",
  cat: "Concatenate CSV files. Subcommands: rows (stack vertically), rowskey (different schemas), columns (side-by-side). Specify via subcommand parameter.",
  geocode:
    "Geocode locations using Geonames/MaxMind. Subcommands: suggest, reverse, countryinfo, iplookup. Specify via subcommand parameter.",
  pivotp:
    "Polars-powered pivot tables. Use --agg for aggregation (sum/mean/count/first/last/min/max/smart). 📊 Run qsv_data_profile first to check pivot column cardinality and value column types.",
};

/**
 * Common usage patterns to help Claude compose effective workflows
 */
const COMMON_PATTERNS: Record<string, string> = {
  stats:
    "Run 2nd (after index). Creates cache used by frequency, schema, tojsonl, sqlp, joinp, diff, sample.",
  index: "Run 1st for files >5MB. Makes count instant, slice 100x faster.",
  select:
    "First step: select columns → filter → sort → output. Speeds up downstream ops.",
  search: "Combine with select: search (filter rows) → select (pick columns).",
  frequency:
    "Profile → Frequency: Use qsv_data_profile first to identify columns with <ALL_UNIQUE> (IDs) to exclude from frequency analysis.",
  sqlp: 'Replaces pipelines: "SELECT * FROM data WHERE x > 10 ORDER BY y LIMIT 100" vs select→search→sort→slice.',
  join: "Run qsv_index first on both files for speed.",
  joinp:
    "Profile → Join: qsv_data_profile both files, put lower-cardinality join column on right for efficiency.",
  pivotp:
    "Profile → Pivot: Use qsv_data_profile to verify pivot column cardinality is reasonable (<1000) before pivoting.",
  sample:
    "Quick preview (100 rows) or test data (1000 rows). Faster than qsv_slice for random.",
  validate: "Iterate: qsv_schema → validate → fix → validate until clean.",
  dedup: "Often followed by stats: dedup → stats for distribution.",
  sort: "Before joins or top-N: sort DESC → slice --end 10.",
  cat: "Combine files: cat rows → headers from first file only. cat rowskey → handles different schemas. cat columns → side-by-side merge.",
  geocode:
    "Common: suggest for city lookup, reverse for lat/lon → city, iplookup for IP → location.",
};

/**
 * Error prevention hints for common mistakes
 */
const ERROR_PREVENTION_HINTS: Record<string, string> = {
  join: "Both files need join column(s). Column names case-sensitive. Check with qsv_headers.",
  joinp: "Use --try-parsedates for date joins.",
  dedup: "May OOM on files >1GB. Use qsv_extdedup for large files.",
  sort: "May OOM on files >1GB. Use qsv_extsort for large files.",
  frequency:
    "High-cardinality columns (IDs, timestamps) produce huge output. Use qsv_data_profile first to check for <ALL_UNIQUE> markers.",
  pivotp:
    "High-cardinality pivot columns create wide output. Use qsv_data_profile to check cardinality first.",
  sqlp: "When encountering errors with sqlp, use DuckDB when available instead for complex queries.",
  moarstats:
    "Run stats first to create cache. Slower than stats but richer output.",
  searchset: "Needs regex file. qsv_search easier for simple patterns.",
  cat: "rows mode requires same column order. Use rowskey for different schemas.",
  geocode:
    "Needs Geonames index (auto-downloads on first use). iplookup needs MaxMind GeoLite2 DB.",
};

/**
 * Hints for complementary MCP servers that can enhance qsv workflows
 */
const COMPLEMENTARY_SERVERS: Record<string, string> = {
  geocode:
    "🔗 CENSUS ENRICHMENT: After geocoding US locations, use Census MCP Server to add demographics. Tools: `resolve-geography-fips` (get FIPS codes), `fetch-aggregate-data` (population, income, education). US data only.",
  stats:
    "🔗 CENSUS VALIDATION: If stats show US geographic columns (city, state, county, FIPS), Census MCP Server can validate codes and fetch reference demographics for comparison.",
  moarstats:
    "🔗 CENSUS VALIDATION: If analyzing US geographic data, Census MCP Server can provide demographic baselines for statistical comparison.",
  frequency:
    "🔗 CENSUS INSIGHT: For US geographic columns, Census MCP Server can enrich frequency results with population/demographic data via `fetch-aggregate-data`.",
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
 * Track active child processes for graceful shutdown
 */
const activeProcesses = new Set<ChildProcess>();

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
 * Run a qsv command with timeout and process tracking
 */
async function runQsvWithTimeout(
  qsvBin: string,
  args: string[],
  timeoutMs: number = config.operationTimeoutMs,
): Promise<void> {
  // Reject new operations during shutdown
  if (isShuttingDown) {
    throw new Error("Server is shutting down, operation rejected");
  }

  return new Promise((resolve, reject) => {
    const proc = spawn(qsvBin, args, {
      stdio: ["ignore", "ignore", "pipe"],
    });

    // Track this process
    activeProcesses.add(proc);

    let stderr = "";
    let timedOut = false;

    // Cleanup function
    const cleanup = () => {
      clearTimeout(timer);
      activeProcesses.delete(proc);
    };

    // Set up timeout
    const timer = setTimeout(() => {
      timedOut = true;
      proc.kill("SIGTERM");
      cleanup();
      reject(
        new Error(
          `Operation timed out after ${timeoutMs}ms: ${qsvBin} ${args.join(" ")}`,
        ),
      );
    }, timeoutMs);

    proc.stderr?.on("data", (chunk) => {
      stderr += chunk.toString();
    });

    proc.on("close", (code) => {
      cleanup();
      if (!timedOut) {
        if (code === 0) {
          resolve();
        } else {
          reject(new Error(`Command failed with exit code ${code}: ${stderr}`));
        }
      }
    });

    proc.on("error", (err) => {
      cleanup();
      if (!timedOut) {
        reject(err);
      }
    });
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
 * 13 most essential qsv commands exposed as individual MCP tools
 * Optimized for token efficiency while maintaining high-value tool access
 *
 * Commands moved to qsv_command generic tool:
 * join, sort, dedup, apply, rename, validate, sample, template, diff, schema
 */
export const COMMON_COMMANDS = [
  "select", // Column selection (most frequently used)
  "stats", // Statistical analysis (creates cache)
  "moarstats", // Comprehensive statistics with data type inference
  "index", // Create index for fast random access (run first)
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
 * Commands that need specific guidance hints
 */
const COMMANDS_NEEDING_MEMORY_WARNING = new Set([
  "dedup",
  "sort",
  "frequency",
  "moarstats",
]);
const COMMANDS_NEEDING_INDEX_HINT = new Set([
  "apply",
  "count",
  "datefmt",
  "frequency",
  "geocode",
  "luau",
  "replace",
  "search",
  "searchset",
  "slice",
  "split",
  "stats",
  "sample",
  "template",
  "tojsonl",
  "transpose",
  "validate",
]);
const COMMANDS_WITH_COMMON_MISTAKES = new Set([
  "cat",
  "join",
  "joinp",
  "dedup",
  "sort",
  "sqlp",
  "schema",
  "searchset",
  "moarstats",
  "frequency",
  "pivotp",
]);

/**
 * Enhance tool description with contextual guidance
 *
 * Uses concise description from README.md and adds guidance hints
 * that help Claude select the right tool. For detailed help,
 * use the qsv_help tool which calls `qsv <command> --help`.
 */
function enhanceDescription(skill: QsvSkill): string {
  const commandName = skill.command.subcommand;

  // Use concise description from README.md
  let description = skill.description;

  // Add when-to-use guidance (critical for tool selection)
  const whenToUse = WHEN_TO_USE_GUIDANCE[commandName];
  if (whenToUse) {
    description += `\n\n💡 ${whenToUse}`;
  }

  // Add subcommand requirement for commands that need it
  if (commandName === "cat") {
    description += `\n\n🔧 SUBCOMMAND: Must pass subcommand via args (e.g., args: {subcommand: "rows", input: "file.csv"}).`;
  } else if (commandName === "geocode") {
    description += `\n\n🔧 SUBCOMMAND: Must pass subcommand via args (e.g., args: {subcommand: "suggest", column: "city", input: "data.csv"}).`;
  }

  // Add common patterns (helps Claude compose workflows)
  const patterns = COMMON_PATTERNS[commandName];
  if (patterns) {
    description += `\n\n📋 ${patterns}`;
  }

  // Add performance hints only for commands that benefit from indexing
  if (skill.hints) {
    // Only show memory warnings for memory-intensive commands
    if (COMMANDS_NEEDING_MEMORY_WARNING.has(commandName)) {
      if (skill.hints.memory === "full") {
        description += "\n\n⚠️  Loads entire CSV. Best <100MB.";
      } else if (skill.hints.memory === "proportional") {
        description += "\n\n⚠️  Memory ∝ unique values.";
      }
    }

    // Only show index hints for commands that are index-accelerated
    if (COMMANDS_NEEDING_INDEX_HINT.has(commandName) && skill.hints.indexed) {
      description +=
        "\n\n🚀 Index-accelerated. Run qsv_index first on files >10MB.";
    }
  }

  // Add error prevention hints only for commands with common mistakes
  if (COMMANDS_WITH_COMMON_MISTAKES.has(commandName)) {
    const errorHint = ERROR_PREVENTION_HINTS[commandName];
    if (errorHint) {
      description += `\n\n⚠️  ${errorHint}`;
    }
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
  const censusHint = COMPLEMENTARY_SERVERS[commandName];
  if (censusHint) {
    description += `\n\n${censusHint}`;
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
        type: mapArgumentType(arg.type),
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
          type: mapOptionType(opt.type),
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
 * Map QSV argument types to JSON Schema types
 */
function mapArgumentType(
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
 * Map QSV option types to JSON Schema types
 */
function mapOptionType(
  type: string,
): "string" | "number" | "boolean" | "object" | "array" {
  switch (type) {
    case "number":
      return "number";
    case "string":
    default:
      return "string";
  }
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
  if (activeProcesses.size >= config.maxConcurrentOperations) {
    return {
      content: [
        {
          type: "text" as const,
          text: `Error: Maximum concurrent operations limit reached (${config.maxConcurrentOperations}). Please wait for current operations to complete.`,
        },
      ],
      isError: true,
    };
  }

  try {
    // Extract command name from tool name (qsv_select -> select)
    const commandName = toolName.replace("qsv_", "");

    // Load the skill
    const skillName = `qsv-${commandName}`;
    const skill = await loader.load(skillName);

    if (!skill) {
      // Calculate remaining commands dynamically
      const totalCommands = loader.getStats().total;
      const remainingCommands = totalCommands - COMMON_COMMANDS.length;

      return {
        content: [
          {
            type: "text" as const,
            text:
              `Error: Skill '${skillName}' not found.\n\n` +
              `Please verify the command name is correct. ` +
              `Available commands include: ${COMMON_COMMANDS.join(", ")}, and ${remainingCommands} others. ` +
              `Use 'qsv_command' with the 'command' parameter for less common commands.`,
          },
        ],
        isError: true,
      };
    }

    // Extract input_file and output_file
    let inputFile = params.input_file as string | undefined;
    let outputFile = params.output_file as string | undefined;

    // Check if this is a help request
    const isHelpRequest = params.help === true;

    // Skip input_file requirement for help requests
    if (!inputFile && !isHelpRequest) {
      return {
        content: [
          {
            type: "text" as const,
            text: "Error: input_file parameter is required (unless using help=true to view command documentation)",
          },
        ],
        isError: true,
      };
    }

    // Resolve file paths using filesystem provider if available (skip for help requests)
    if (filesystemProvider && inputFile) {
      try {
        const originalInputFile = inputFile;
        inputFile = await filesystemProvider.resolvePath(inputFile);
        console.error(
          `[MCP Tools] Resolved input file: ${originalInputFile} -> ${inputFile}`,
        );

        // Check if file needs conversion (Excel or JSONL to CSV)
        if (isFilesystemProviderExtended(filesystemProvider)) {
          const provider = filesystemProvider;

          if (provider.needsConversion(inputFile)) {
            const conversionCmd = provider.getConversionCommand(inputFile);
            if (!conversionCmd) {
              throw new Error(
                `Unable to determine conversion command for: ${inputFile}`,
              );
            }
            console.error(
              `[MCP Tools] File requires conversion using qsv ${conversionCmd}`,
            );

            // Convert file using qsv excel or qsv jsonl
            try {
              const qsvBin = getQsvBinaryPath();

              // Generate unique converted file path with UUID to prevent collisions
              const { randomUUID } = await import("crypto");
              // Use 16 hex chars (64 bits) for better collision resistance
              // Remove hyphens to get pure hex digits (randomUUID() includes hyphens)
              // 8 hex chars (32 bits) has 50% collision probability after ~65k conversions
              // 16 hex chars (64 bits) has 50% collision probability after ~4 billion conversions
              const uuid = randomUUID().replace(/-/g, "").substring(0, 16);
              let convertedPath = `${inputFile}.converted.${uuid}.csv`;

              // Validate the generated converted path for defense-in-depth
              // Even though it's derived from already-validated inputFile, ensure it's safe
              try {
                convertedPath = await provider.resolvePath(convertedPath);
              } catch (error) {
                throw new Error(
                  `Invalid converted file path: ${convertedPath} - ${error}`,
                );
              }

              // Initialize converted file manager
              const workingDir = provider.getWorkingDirectory();
              const convertedManager = new ConvertedFileManager(workingDir);

              // Clean up orphaned entries and partial conversions first
              await convertedManager.cleanupOrphanedEntries();

              // Check if we can reuse an existing converted file
              // Note: This looks for any .converted.*.csv file for this source
              const {
                basename: getBasename,
                dirname: getDirname,
                join: joinPath,
              } = await import("path");
              const { readdir } = await import("fs/promises");

              const baseName = getBasename(inputFile);
              const pattern = `${baseName}.converted.`;
              let validConverted: string | null = null;

              // Search for existing converted files in the same directory as the input file
              try {
                const dir = getDirname(inputFile);
                const files = await readdir(dir);

                for (const file of files) {
                  if (file.startsWith(pattern) && file.endsWith(".csv")) {
                    const filePath = joinPath(dir, file);
                    validConverted =
                      await convertedManager.getValidConvertedFile(
                        inputFile,
                        filePath,
                      );
                    if (validConverted) break;
                  }
                }
              } catch (error) {
                // If readdir fails, just proceed with conversion
                console.error(
                  "[MCP Tools] Error searching for existing converted file:",
                  error,
                );
              }

              if (validConverted) {
                // Reuse existing converted file and update timestamp
                await convertedManager.touchConvertedFile(inputFile);
                inputFile = validConverted;
                console.error(
                  `[MCP Tools] Reusing existing conversion: ${validConverted}`,
                );
              } else {
                // Register conversion start for failure tracking
                await convertedManager.registerConversionStart(
                  inputFile,
                  convertedPath,
                );

                try {
                  // Run conversion command: qsv excel/jsonl <input> --output <converted.csv>
                  const conversionArgs = [
                    conversionCmd,
                    inputFile,
                    "--output",
                    convertedPath,
                  ];
                  console.error(
                    `[MCP Tools] Running conversion: ${qsvBin} ${conversionArgs.join(" ")}`,
                  );

                  await runQsvWithTimeout(qsvBin, conversionArgs);

                  // Conversion succeeded - first register the converted file in the cache
                  await convertedManager.registerConvertedFile(
                    inputFile,
                    convertedPath,
                  );

                  // Only mark conversion as complete after successful cache registration
                  await convertedManager.registerConversionComplete(inputFile);
                  // Use the converted CSV as input
                  inputFile = convertedPath;
                  console.error(
                    `[MCP Tools] Conversion successful: ${convertedPath}`,
                  );

                  // Auto-index the converted CSV
                  await autoIndexIfNeeded(convertedPath);
                } catch (conversionError) {
                  // Conversion failed - clean up partial file
                  try {
                    const { unlink } = await import("fs/promises");
                    await unlink(convertedPath);
                    console.error(
                      `[MCP Tools] Cleaned up partial conversion file: ${convertedPath}`,
                    );
                  } catch {
                    // Ignore cleanup errors - cleanupPartialConversions will handle it
                  }

                  // Track conversion failure
                  convertedManager.trackConversionFailure();

                  // Re-throw to outer catch block
                  throw conversionError;
                }
              }
            } catch (conversionError) {
              console.error(`[MCP Tools] Conversion error:`, conversionError);
              return {
                content: [
                  {
                    type: "text" as const,
                    text: `Error converting ${originalInputFile}: ${conversionError instanceof Error ? conversionError.message : String(conversionError)}`,
                  },
                ],
                isError: true,
              };
            }
          }
        }

        // Auto-index native CSV files if they're large enough and not indexed
        // Note: Snappy-compressed files (.sz) cannot be indexed
        // Skip for help requests
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

        // Build enhanced error message with file suggestions
        let errorMessage = `Error resolving file path: ${error instanceof Error ? error.message : String(error)}`;

        // Add file suggestions if this looks like a file-not-found error and we have filesystem provider
        if (filesystemProvider && inputFile) {
          const errorStr =
            error instanceof Error ? error.message : String(error);
          if (
            errorStr.includes("outside allowed") ||
            errorStr.includes("not exist") ||
            errorStr.includes("cannot access") ||
            errorStr.includes("ENOENT")
          ) {
            try {
              // Get list of available files
              const { resources } = await filesystemProvider.listFiles(
                undefined,
                false,
              );

              if (resources.length > 0) {
                // Find similar files using fuzzy matching
                const suggestions = findSimilarFiles(inputFile, resources, 3);

                errorMessage += "\n\n";

                // Show suggestions if we found close matches
                if (
                  suggestions.length > 0 &&
                  suggestions[0].distance <= inputFile.length / 2
                ) {
                  errorMessage += "Did you mean one of these?\n";
                  suggestions.forEach(({ name }) => {
                    errorMessage += `  - ${name}\n`;
                  });
                } else {
                  // Show available files if no close matches
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
              // If listing files fails, just show the original error
              console.error(
                `[MCP Tools] Failed to list files for suggestions:`,
                listError,
              );
            }
          }
        }

        return {
          content: [
            {
              type: "text" as const,
              text: errorMessage,
            },
          ],
          isError: true,
        };
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
      // Auto-create temp file
      const { randomUUID } = await import("crypto");
      const { tmpdir } = await import("os");
      const { join } = await import("path");

      const tempFileName = `qsv-output-${randomUUID()}.csv`;
      outputFile = join(tmpdir(), tempFileName);
      autoCreatedTempFile = true;

      console.error(`[MCP Tools] Auto-created temp output file: ${outputFile}`);
    }

    // Build args and options
    const args: Record<string, unknown> = {};
    const options: Record<string, unknown> = {};

    // Add input file as 'input' argument if the skill expects it
    if (skill.command.args.some((a) => a.name === "input")) {
      args.input = inputFile;
      console.error(`[MCP Tools] Added input arg: ${inputFile}`);
    }

    for (const [key, value] of Object.entries(params)) {
      // Skip input_file, output_file, and help (already handled)
      // Also skip 'input' if we already set it from input_file
      if (
        key === "input_file" ||
        key === "output_file" ||
        key === "help" ||
        (key === "input" && args.input)
      ) {
        continue;
      }

      // Check if this is a positional argument
      const isArg = skill.command.args.some((a) => a.name === key);
      if (isArg) {
        args[key] = value;
      } else {
        // It's an option - convert underscore to dash
        const optFlag = `--${key.replace(/_/g, "-")}`;
        options[optFlag] = value;
      }
    }

    // Add output file option if provided
    if (outputFile) {
      options["--output"] = outputFile;
    }

    // Add help flag if requested
    if (isHelpRequest) {
      options["help"] = true;
    }

    console.error(
      `[MCP Tools] Executing skill with args:`,
      JSON.stringify(args),
    );
    console.error(
      `[MCP Tools] Executing skill with options:`,
      JSON.stringify(options),
    );

    // Execute the skill
    const result = await executor.execute(skill, {
      args,
      options,
    });

    // Format result
    if (result.success) {
      let responseText = "";

      if (outputFile) {
        if (autoCreatedTempFile) {
          // Check temp file size before deciding how to handle it
          try {
            const { stat, readFile, unlink, rename } =
              await import("fs/promises");
            const { join } = await import("path");

            const tempFileStats = await stat(outputFile);

            if (tempFileStats.size > MAX_MCP_RESPONSE_SIZE) {
              // Output too large for MCP response - save to working directory instead
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

              // Move temp file to working directory
              await rename(outputFile, savedPath);
              console.error(`[MCP Tools] Saved large output to: ${savedPath}`);

              responseText = `✅ Large output saved to file (too large to display in chat)\n\n`;
              responseText += `File: ${savedFileName}\n`;
              responseText += `Location: ${config.workingDir}\n`;
              responseText += `Size: ${formatBytes(tempFileStats.size)}\n`;
              responseText += `Duration: ${result.metadata.duration}ms\n\n`;
              responseText += `The file is now available in your working directory and can be processed with additional qsv commands.`;
            } else {
              // Small enough - return contents directly
              console.error(
                `[MCP Tools] Output file (${formatBytes(tempFileStats.size)}) is small enough to return directly`,
              );
              const fileContents = await readFile(outputFile, "utf-8");

              // Clean up temp file
              try {
                await unlink(outputFile);
                console.error(`[MCP Tools] Deleted temp file: ${outputFile}`);
              } catch (unlinkError) {
                console.error(
                  `[MCP Tools] Failed to delete temp file:`,
                  unlinkError,
                );
              }

              // Return the file contents
              responseText = fileContents;
            }
          } catch (readError) {
            console.error(
              `[MCP Tools] Failed to process temp file:`,
              readError,
            );
            return {
              content: [
                {
                  type: "text" as const,
                  text: `Error processing output from temp file: ${readError instanceof Error ? readError.message : String(readError)}`,
                },
              ],
              isError: true,
            };
          }
        } else {
          // User-specified output file - just report success
          responseText = `Successfully wrote output to: ${outputFile}\n\n`;
          responseText += `Metadata:\n`;
          responseText += `- Command: ${result.metadata.command}\n`;
          responseText += `- Duration: ${result.metadata.duration}ms\n`;
          if (result.metadata.rowsProcessed) {
            responseText += `- Rows processed: ${result.metadata.rowsProcessed}\n`;
          }
        }
      } else {
        // Return the CSV output from stdout
        responseText = result.output;
      }

      return {
        content: [
          {
            type: "text" as const,
            text: responseText,
          },
        ],
      };
    } else {
      return {
        content: [
          {
            type: "text" as const,
            text: `Error executing ${commandName}:\n${result.stderr}`,
          },
        ],
        isError: true,
      };
    }
  } catch (error) {
    return {
      content: [
        {
          type: "text" as const,
          text: `Unexpected error: ${error instanceof Error ? error.message : String(error)}`,
        },
      ],
      isError: true,
    };
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
      return {
        content: [
          {
            type: "text" as const,
            text: "Error: command parameter is required",
          },
        ],
        isError: true,
      };
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
    return {
      content: [
        {
          type: "text",
          text: `Unexpected error: ${error instanceof Error ? error.message : String(error)}`,
        },
      ],
      isError: true,
    };
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
            'The qsv command to execute (e.g., "to", "flatten", "partition")',
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

  // Performance Settings
  configText += `\n## Performance Settings\n\n`;
  configText += `⏱️ **Timeout:** ${config.timeoutMs}ms (${Math.round(config.timeoutMs / 1000)}s)\n`;
  configText += `💾 **Max Output Size:** ${formatBytes(config.maxOutputSize)}\n`;
  configText += `🔧 **Auto-Regenerate Skills:** ${config.autoRegenerateSkills ? "Enabled" : "Disabled"}\n`;

  // Update Check Settings
  configText += `\n## Update Settings\n\n`;
  configText += `🔍 **Check Updates on Startup:** ${config.checkUpdatesOnStartup ? "Enabled" : "Disabled"}\n`;
  configText += `📢 **Update Notifications:** ${config.notifyUpdates ? "Enabled" : "Disabled"}\n`;

  // Mode
  configText += `\n## Deployment Mode\n\n`;
  configText += `${extensionMode ? "🧩 **Desktop Extension Mode**" : "🖥️ **Legacy MCP Server Mode**"}\n`;

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
    `[MCP Tools] Shutdown initiated, ${activeProcesses.size} active processes`,
  );
}

/**
 * Kill all active child processes
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
 * Get count of active processes
 */
export function getActiveProcessCount(): number {
  return activeProcesses.size;
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
 */
export async function handleSearchToolsCall(
  params: Record<string, unknown>,
  loader: SkillLoader,
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
    const whenToUse = WHEN_TO_USE_GUIDANCE[skill.command.subcommand];

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
 * Create qsv_data_profile tool definition
 * Profiles CSV data for SQL query optimization using qsv frequency --toon
 */
export function createDataProfileTool(): McpToolDefinition {
  return {
    name: "qsv_data_profile",
    description: `Profile a CSV file for SQL query optimization using qsv frequency --toon.

Returns column statistics in TOON format (token-efficient for LLMs) including:
- Data types (Integer, Float, String, Date, DateTime, Boolean)
- Cardinality and uniqueness_ratio (identifies keys vs categorical columns)
- Null counts and sparsity (affects JOIN/WHERE behavior)
- Min/max values, ranges, and sort_order (for range queries)
- Top frequent values with percentages and counts

💡 USE WHEN: Before writing complex sqlp queries. Helps choose optimal:
- JOIN order (smaller cardinality table first)
- GROUP BY columns (low cardinality = efficient)
- WHERE selectivity (high-cardinality columns filter more)
- Index columns (uniqueness_ratio=1 = good key candidate)

📋 COMMON PATTERN: qsv_data_profile → analyze output → compose optimized sqlp query

🔍 INTERPRETATION GUIDE:
- <ALL_UNIQUE> = ID/key column (uniqueness_ratio=1), good for JOINs
- Low cardinality (<100) = categorical, efficient for GROUP BY
- High sparsity (>0.5) = many NULLs, consider in WHERE clauses
- sort_order=Ascending/Descending = pre-sorted, efficient for ORDER BY`,
    inputSchema: {
      type: "object",
      properties: {
        input_file: {
          type: "string",
          description: "Path to input CSV file to profile",
        },
        limit: {
          type: "number",
          description:
            "Top N frequent values per column (default: 10). Set to 0 for no limit.",
        },
        columns: {
          type: "string",
          description:
            'Optional: specific columns to profile (comma-separated or qsv select syntax). Example: "name,age,city" or "1,3,5"',
        },
        no_stats: {
          type: "boolean",
          description:
            "Exclude additional stats (min, max, mean, etc.) from output. Reduces output size.",
        },
      },
      required: ["input_file"],
    },
  };
}

/**
 * Handle qsv_data_profile tool call
 * Runs qsv frequency --toon to profile data for SQL optimization
 */
export async function handleDataProfileCall(
  params: Record<string, unknown>,
  filesystemProvider?: FilesystemProviderExtended,
): Promise<{ content: Array<{ type: string; text: string }>; isError?: boolean }> {
  // Reject new operations during shutdown
  if (isShuttingDown) {
    return {
      content: [
        {
          type: "text",
          text: "Error: Server is shutting down, operation rejected",
        },
      ],
      isError: true,
    };
  }

  // Check concurrent operation limit
  if (activeProcesses.size >= config.maxConcurrentOperations) {
    return {
      content: [
        {
          type: "text",
          text: `Error: Maximum concurrent operations limit reached (${config.maxConcurrentOperations}). Please wait for current operations to complete.`,
        },
      ],
      isError: true,
    };
  }

  // Validate input_file parameter
  let inputFile = params.input_file as string | undefined;

  if (!inputFile) {
    return {
      content: [
        {
          type: "text",
          text: "Error: input_file parameter is required",
        },
      ],
      isError: true,
    };
  }

  // Resolve file path using filesystem provider if available
  if (filesystemProvider) {
    try {
      const originalInputFile = inputFile;
      inputFile = await filesystemProvider.resolvePath(inputFile);
      console.error(
        `[MCP Tools] data_profile: Resolved input file: ${originalInputFile} -> ${inputFile}`,
      );

      // Check if file needs conversion (Excel or JSONL to CSV)
      if (isFilesystemProviderExtended(filesystemProvider)) {
        const provider = filesystemProvider;

        if (provider.needsConversion(inputFile)) {
          const conversionCmd = provider.getConversionCommand(inputFile);
          if (!conversionCmd) {
            return {
              content: [
                {
                  type: "text",
                  text: `Error: Unable to determine conversion command for: ${inputFile}`,
                },
              ],
              isError: true,
            };
          }

          // For data profiling, we need a CSV file. Convert if necessary.
          console.error(
            `[MCP Tools] data_profile: File requires conversion using qsv ${conversionCmd}`,
          );

          try {
            const qsvBin = getQsvBinaryPath();
            const { randomUUID } = await import("crypto");
            const uuid = randomUUID().replace(/-/g, "").substring(0, 16);
            let convertedPath = `${inputFile}.converted.${uuid}.csv`;

            // Validate the converted path
            try {
              convertedPath = await provider.resolvePath(convertedPath);
            } catch (error) {
              return {
                content: [
                  {
                    type: "text",
                    text: `Error: Invalid converted file path: ${convertedPath} - ${error}`,
                  },
                ],
                isError: true,
              };
            }

            // Initialize converted file manager
            const workingDir = provider.getWorkingDirectory();
            const convertedManager = new ConvertedFileManager(workingDir);
            await convertedManager.cleanupOrphanedEntries();

            // Check for existing converted file
            const {
              basename: getBasename,
              dirname: getDirname,
              join: joinPath,
            } = await import("path");
            const { readdir } = await import("fs/promises");

            const baseName = getBasename(inputFile);
            const pattern = `${baseName}.converted.`;
            let validConverted: string | null = null;

            try {
              const dir = getDirname(inputFile);
              const files = await readdir(dir);

              for (const file of files) {
                if (file.startsWith(pattern) && file.endsWith(".csv")) {
                  const filePath = joinPath(dir, file);
                  validConverted =
                    await convertedManager.getValidConvertedFile(
                      inputFile,
                      filePath,
                    );
                  if (validConverted) break;
                }
              }
            } catch {
              // If readdir fails, proceed with conversion
            }

            if (validConverted) {
              await convertedManager.touchConvertedFile(inputFile);
              inputFile = validConverted;
              console.error(
                `[MCP Tools] data_profile: Reusing existing conversion: ${validConverted}`,
              );
            } else {
              // Run conversion
              await convertedManager.registerConversionStart(
                inputFile,
                convertedPath,
              );

              const conversionArgs = [
                conversionCmd,
                inputFile,
                "--output",
                convertedPath,
              ];

              await runQsvWithTimeout(qsvBin, conversionArgs);
              await convertedManager.registerConvertedFile(
                inputFile,
                convertedPath,
              );
              await convertedManager.registerConversionComplete(inputFile);
              inputFile = convertedPath;
              console.error(
                `[MCP Tools] data_profile: Conversion successful: ${convertedPath}`,
              );

              // Auto-index the converted file
              await autoIndexIfNeeded(convertedPath);
            }
          } catch (conversionError) {
            console.error(
              `[MCP Tools] data_profile: Conversion error:`,
              conversionError,
            );
            return {
              content: [
                {
                  type: "text",
                  text: `Error converting file: ${conversionError instanceof Error ? conversionError.message : String(conversionError)}`,
                },
              ],
              isError: true,
            };
          }
        }
      }

      // Auto-index if needed
      await autoIndexIfNeeded(inputFile);
    } catch (error) {
      return {
        content: [
          {
            type: "text",
            text: `Error resolving file path: ${error instanceof Error ? error.message : String(error)}`,
          },
        ],
        isError: true,
      };
    }
  }

  // Check profile cache (if enabled)
  let profileCache: ProfileCacheManager | null = null;
  if (config.profileCacheEnabled) {
    const workingDir = filesystemProvider
      ? (filesystemProvider as FilesystemProviderExtended).getWorkingDirectory?.() ?? config.workingDir
      : config.workingDir;

    profileCache = new ProfileCacheManager(workingDir, {
      maxSizeMB: config.profileCacheMaxSizeMB,
      ttlMs: config.profileCacheTtlMs,
    });

    const profileOptions = {
      limit: params.limit as number | undefined,
      columns: params.columns as string | undefined,
      no_stats: params.no_stats as boolean | undefined,
    };

    const cachedProfile = await profileCache.getCachedProfile(
      inputFile,
      profileOptions,
    );

    if (cachedProfile !== null) {
      console.error(
        `[MCP Tools] data_profile: Cache hit for ${inputFile}`,
      );
      return {
        content: [
          {
            type: "text",
            text: cachedProfile,
          },
        ],
      };
    }
  }

  // Build qsv frequency command with --toon flag
  const qsvBin = getQsvBinaryPath();
  const qsvArgs = ["frequency", "--toon"];

  // Add --limit option
  if (params.limit !== undefined) {
    qsvArgs.push("--limit", String(params.limit));
  }

  // Add --select option for specific columns
  if (params.columns) {
    qsvArgs.push("--select", String(params.columns));
  }

  // Add --no-stats option
  if (params.no_stats === true) {
    qsvArgs.push("--no-stats");
  }

  // Add input file
  qsvArgs.push(inputFile);

  console.error(`[MCP Tools] data_profile: Running ${qsvBin} ${qsvArgs.join(" ")}`);

  // Execute qsv frequency --toon
  return new Promise((resolve) => {
    const proc = spawn(qsvBin, qsvArgs, {
      stdio: ["ignore", "pipe", "pipe"],
    });

    activeProcesses.add(proc);

    let stdout = "";
    let stderr = "";
    let timedOut = false;

    const cleanup = () => {
      clearTimeout(timer);
      activeProcesses.delete(proc);
    };

    const timer = setTimeout(() => {
      timedOut = true;
      proc.kill("SIGTERM");
      cleanup();
      resolve({
        content: [
          {
            type: "text",
            text: `Error: data_profile operation timed out after ${config.operationTimeoutMs}ms`,
          },
        ],
        isError: true,
      });
    }, config.operationTimeoutMs);

    proc.stdout?.on("data", (chunk) => {
      stdout += chunk.toString();
    });

    proc.stderr?.on("data", (chunk) => {
      stderr += chunk.toString();
    });

    proc.on("close", (code) => {
      cleanup();
      if (!timedOut) {
        if (code === 0) {
          // Resolve immediately with the result
          resolve({
            content: [
              {
                type: "text",
                text: stdout,
              },
            ],
          });

          // Cache the profile fire-and-forget (if caching enabled)
          // This avoids adding latency to the response
          if (profileCache && config.profileCacheEnabled) {
            const profileOptions = {
              limit: params.limit as number | undefined,
              columns: params.columns as string | undefined,
              no_stats: params.no_stats as boolean | undefined,
            };

            profileCache.cacheProfile(inputFile, profileOptions, stdout).catch((err) => {
              console.error(
                `[MCP Tools] data_profile: Failed to cache profile: ${err}`,
              );
            });
          }
        } else {
          resolve({
            content: [
              {
                type: "text",
                text: `Error running qsv frequency --toon:\n${stderr}`,
              },
            ],
            isError: true,
          });
        }
      }
    });

    proc.on("error", (err) => {
      cleanup();
      if (!timedOut) {
        resolve({
          content: [
            {
              type: "text",
              text: `Error spawning qsv: ${err.message}`,
            },
          ],
          isError: true,
        });
      }
    });
  });
}
