/**
 * File path resolution, format conversion, output formatting, and related utilities.
 */

import { randomUUID } from "crypto";
import { stat, access, readFile, unlink, rename, copyFile, readdir, mkdir } from "fs/promises";
import { constants } from "fs";
import { basename, dirname, isAbsolute, join } from "path";
import { ConvertedFileManager } from "./converted-file-manager.js";
import type {
  QsvSkill,
  FilesystemProviderExtended,
} from "./types.js";
import {
  ALWAYS_FILE_COMMANDS,
  METADATA_COMMANDS,
  NON_TABULAR_COMMANDS,
  LARGE_FILE_THRESHOLD_BYTES,
  MAX_MCP_RESPONSE_SIZE,
  AUTO_INDEX_SIZE_MB,
  FILE_PATH_INPUT_OPTIONS,
  FILE_PATH_OUTPUT_OPTIONS,
  FINAL_OUTPUT_FILE,
  isBinaryOutputFormat,
} from "./tool-constants.js";
import { isShuttingDown, activeProcesses } from "./concurrency.js";
import { runQsvSimple } from "./executor.js";
import { config } from "./config.js";
import { formatBytes, getErrorMessage, findSimilarFiles, errorResult, successResult, isNodeError } from "./utils.js";

/**
 * Stat a path, returning null for ENOENT (file not found) and rethrowing
 * permission / IO errors.  Shared by ensureStatsCache & ensurePolarsSchema.
 */
export const statOrNull = (path: string) =>
  stat(path).catch((err: unknown) => {
    if (isNodeError(err) && err.code === "ENOENT") return null;
    throw err;
  });

/**
 * Dynamic working directory that tracks runtime changes from qsv_set_working_dir.
 * Initialized from config.workingDir; updated via setToolsWorkingDir().
 */
let currentWorkingDir: string = config.workingDir;

/**
 * Update the module-level working directory.
 * Called from mcp-server.ts when the user changes the working directory at runtime.
 * Expects an already-validated absolute path from the caller.
 */
export function setToolsWorkingDir(dir: string): void {
  const trimmed = typeof dir === 'string' ? dir.trim() : '';
  if (!trimmed || !isAbsolute(trimmed)) {
    throw new Error(`setToolsWorkingDir: expected an absolute path, got "${dir}"`);
  }
  currentWorkingDir = trimmed;
}

/**
 * Return the current module-level working directory.
 * Useful for testing and diagnostics.
 */
export function getToolsWorkingDir(): string {
  return currentWorkingDir;
}

/**
 * Get the current working directory (for use by other modules).
 */
export function getCurrentWorkingDir(): string {
  return currentWorkingDir;
}

/**
 * Run a qsv command with timeout, process tracking, and shutdown awareness.
 * Delegates to shared runQsvSimple for the actual spawning logic.
 */
export async function runQsvWithTimeout(
  qsvBin: string,
  args: string[],
  timeoutMs: number = config.operationTimeoutMs,
): Promise<void> {
  if (isShuttingDown) {
    throw new Error("Server is shutting down, operation rejected");
  }

  // Snapshot the working directory to avoid TOCTOU issues if it changes mid-execution.
  const workDir = currentWorkingDir;

  // Ensure working directory exists before spawning (CWD ENOENT causes
  // a misleading "binary not found" error from Node.js spawn).
  await mkdir(workDir, { recursive: true });

  await runQsvSimple(qsvBin, args, {
    timeoutMs,
    cwd: workDir,
    onSpawn: (proc) => activeProcesses.add(proc),
    onExit: (proc) => activeProcesses.delete(proc),
  });
}

/**
 * Build arguments for file conversion commands.
 * Handles different patterns:
 * - Excel/JSONL: qsv <cmd> <input> --output <output>
 * - Parquet→CSV: qsv sqlp SKIP_INPUT "select * from read_parquet('<input>')" --output <output>
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
  // Standard: qsv <cmd> <input> --output <output>
  return [conversionCmd, inputFile, "--output", outputFile];
}

/**
 * Map QSV argument/option types to JSON Schema types
 * (Arguments and options share the same mapping logic)
 */
export function mapSchemaType(
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
 * Auto-index a file if it's large enough and not already indexed
 * Reusable helper to avoid code duplication
 */
export async function autoIndexIfNeeded(
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
    } catch { /* ignore: index file does not exist */
      indexExists = false;
    }

    // Create index if file is large enough and not already indexed
    if (fileSizeMB > minSizeMB && !indexExists) {
      console.error(
        `[MCP Tools] File is ${fileSizeMB.toFixed(1)}MB, creating index...`,
      );

      const qsvBin = config.qsvBinPath;
      const indexArgs = ["index", filePath];

      try {
        await runQsvWithTimeout(qsvBin, indexArgs);
        console.error(`[MCP Tools] Index created successfully: ${indexPath}`);
      } catch (error: unknown) {
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
  } catch (error: unknown) {
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
export async function shouldUseTempFile(
  command: string,
  inputFile: string,
): Promise<boolean> {
  // Metadata commands always use stdout (small results)
  if (METADATA_COMMANDS.has(command)) {
    return false;
  }

  // TSV mode: force temp file so qsv outputs TSV natively via .tsv extension.
  // This bypasses size-based heuristics below, meaning even small outputs go through
  // temp file I/O. This is an acceptable trade-off because: (1) qsv's file I/O is fast,
  // (2) it avoids a fragile post-processing step to convert CSV→TSV in-memory, and
  // (3) it ensures consistent tab-delimited output for all tabular commands.
  if (config.outputFormat === "tsv" && !NON_TABULAR_COMMANDS.has(command)) {
    return true;
  }

  // Commands that always return full CSV data should use temp files
  if (ALWAYS_FILE_COMMANDS.has(command)) {
    return true;
  }

  // For other commands, check input file size
  try {
    const stats = await stat(inputFile);
    return stats.size > LARGE_FILE_THRESHOLD_BYTES;
  } catch (error: unknown) {
    // If we can't stat the file, default to stdout
    console.error(
      `[MCP Tools] Error checking file size for temp file decision:`,
      error,
    );
    return false;
  }
}

/**
 * Resolve input file path, handle format conversion (Excel/JSONL to CSV),
 * and auto-index large files. Returns the resolved (possibly converted) input path.
 */
export async function resolveAndConvertInputFile(
  inputFile: string,
  filesystemProvider: FilesystemProviderExtended,
): Promise<string> {
  const originalInputFile = inputFile;
  inputFile = await filesystemProvider.resolvePath(inputFile);
  console.error(
    `[MCP Tools] Resolved input file: ${originalInputFile} -> ${inputFile}`,
  );

  // Check if file needs conversion (Excel or JSONL to CSV)
  if (filesystemProvider.needsConversion(inputFile)) {
    const conversionCmd = filesystemProvider.getConversionCommand(inputFile);
    if (!conversionCmd) {
      throw new Error(
        `Unable to determine conversion command for: ${inputFile}`,
      );
    }
    console.error(
      `[MCP Tools] File requires conversion using qsv ${conversionCmd}`,
    );

    const qsvBin = config.qsvBinPath;

    // Generate unique converted file path with UUID to prevent collisions
    // 16 hex chars (64 bits) has 50% collision probability after ~4 billion conversions
    const uuid = randomUUID().replace(/-/g, "").substring(0, 16);
    let convertedPath = `${inputFile}.converted.${uuid}.csv`;

    // Validate the generated converted path for defense-in-depth
    try {
      convertedPath = await filesystemProvider.resolvePath(convertedPath);
    } catch (error: unknown) {
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
    } catch (error: unknown) {
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
        } catch { /* ignore: cleanup */
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
export async function buildFileNotFoundError(
  inputFile: string,
  error: unknown,
  filesystemProvider: FilesystemProviderExtended,
): Promise<string> {
  let errorMessage = `Error resolving file path: ${getErrorMessage(error)}`;

  // Detect Cowork VM paths that won't resolve on the host
  if (/^\/sessions\/|^\/home\/user\/|^\/tmp\/cowork-|^\/workspace\//.test(inputFile)) {
    const workingDir = filesystemProvider.getWorkingDirectory();
    errorMessage =
      `This path appears to be a Cowork VM path. qsv runs on the host machine.\n` +
      `Current qsv working directory: ${workingDir}\n` +
      `Use filenames relative to this directory, or call qsv_set_working_dir first.\n\n` +
      errorMessage;
  }

  const errorStr = getErrorMessage(error);
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
 * Convert a param key to CLI flag format.
 * e.g. "dupes_output" -> "--dupes-output", "--already-flagged" -> "--already-flagged"
 */
export function paramKeyToFlag(key: string): string {
  return key.startsWith("-") ? key : `--${key.replace(/_/g, "-")}`;
}

/**
 * Check whether a value looks like a file path (as opposed to inline script text).
 * Used for ambiguous options like luau --begin/--end that accept both scripts and file paths.
 *
 * Only treats `/` as a path indicator when there are strong path signals:
 * - Starts with `/`, `./`, or `../`
 * - Contains 2+ slashes (multi-segment like `path/to/file` — unlikely to be Luau division)
 * Bare `a/b` (single slash, no prefix) is NOT treated as a path to avoid false positives
 * on inline Luau arithmetic.
 *
 * Backslash `\` is only treated as a path indicator with Windows path signals:
 * drive letter prefix (`C:\`), `.\`, or `..\`.
 */
export function looksLikeFilePath(value: string): boolean {
  const v = value.trim();
  // Multi-segment forward-slash path: 2+ slashes makes division very unlikely.
  // Note: single-slash bare paths like `dir/file.txt` are rejected to avoid false positives
  // on Luau division, but `dir/file.lua` is still caught by the .lua/.luau extension check.
  const firstSlash = v.indexOf("/");
  const multiSegmentSlash = firstSlash !== -1 && v.indexOf("/", firstSlash + 1) !== -1;
  // Windows-style backslash path: require drive letter, .\ or ..\ prefix
  const windowsPath = /^[A-Za-z]:\\/.test(v) || v.startsWith(".\\") || v.startsWith("..\\");
  return (
    v.startsWith("file:") ||
    v.endsWith(".lua") ||
    v.endsWith(".luau") ||
    windowsPath ||
    v.startsWith("~") ||
    v.startsWith("/") ||
    v.startsWith("./") ||
    v.startsWith("../") ||
    multiSegmentSlash
  );
}

/**
 * Resolve file-path parameters (positional args and options) to absolute paths.
 * This handles all file-type args beyond input/output which are already resolved.
 *
 * - Positional args with type "file" (excluding "input") are resolved
 * - Options in FILE_PATH_INPUT_OPTIONS and FILE_PATH_OUTPUT_OPTIONS are resolved
 * - Luau --begin/--end are resolved only when the value looks like a file path
 *
 * On resolution failure, logs a warning but doesn't block execution.
 */
export async function resolveFilePathParams(
  params: Record<string, unknown>,
  skill: QsvSkill,
  filesystemProvider: FilesystemProviderExtended,
): Promise<void> {
  // 1. Resolve positional args with type "file" (excluding "input", already handled)
  for (const arg of skill.command.args) {
    if (arg.type === "file" && arg.name !== "input" && params[arg.name]) {
      const rawValue = String(params[arg.name]);
      try {
        const resolved = await filesystemProvider.resolvePath(rawValue);
        if (resolved !== rawValue) {
          console.error(`[MCP Tools] Resolved arg '${arg.name}': ${rawValue} -> ${resolved}`);
          params[arg.name] = resolved;
        }
      } catch (error: unknown) {
        console.error(`[MCP Tools] Warning: failed to resolve arg '${arg.name}' path '${rawValue}':`, getErrorMessage(error));
      }
    }
  }

  // 2. Resolve option values that are file paths
  for (const [key, value] of Object.entries(params)) {
    if (!value || typeof value !== "string") continue;

    // Convert param key to flag format (e.g. "dupes_output" -> "--dupes-output")
    const flag = paramKeyToFlag(key);

    const isFilePathOption = FILE_PATH_INPUT_OPTIONS.has(flag) || FILE_PATH_OUTPUT_OPTIONS.has(flag);

    // Special case: luau --begin/--end accept both inline scripts and file paths
    const isAmbiguousFileOption = (flag === "--begin" || flag === "--end")
      && skill.name === "luau"
      && looksLikeFilePath(value);

    if (isFilePathOption || isAmbiguousFileOption) {
      try {
        const resolved = await filesystemProvider.resolvePath(value);
        if (resolved !== value) {
          console.error(`[MCP Tools] Resolved option '${flag}': ${value} -> ${resolved}`);
          params[key] = resolved;
        }
      } catch (error: unknown) {
        console.error(`[MCP Tools] Warning: failed to resolve option '${flag}' path '${value}':`, getErrorMessage(error));
      }
    }
  }
}

/**
 * Build args and options for skill execution from MCP tool params
 */
export function buildSkillExecParams(
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

  // Check whether a skill declares a distinct --input or --output CLI option.
  // If so, allow the key through instead of treating it as a file alias.
  // Note: only checks long-form options (--input/--output), not short flags (-i/-o).
  // A positional arg named "input" is already consumed above as the input file.
  const skillHasOption = (name: string) =>
    skill.command.options.some((o) => o.flag === `--${name}`);

  for (const [key, value] of Object.entries(params)) {
    // Skip meta-parameters that are handled separately (not passed as CLI flags).
    // "input" and "output" are aliases for input_file/output_file resolved by
    // resolveParamAliases, so skip them unless the skill declares them as
    // distinct CLI flags (i.e. the skill has an arg or option named "input"/"output").
    if (
      key === "input_file" ||
      key === "output_file" ||
      (key === "input" && !skillHasOption("input")) ||
      (key === "output" && !skillHasOption("output")) ||
      key === "help"
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
 * Collect additional input files from tool params for the pipeline manifest.
 * These are file-type positional args (excluding the primary "input") and
 * FILE_PATH_INPUT_OPTIONS that reference input files.
 */
export function collectAdditionalInputFiles(
  skill: QsvSkill,
  params: Record<string, unknown>,
): Array<{ file: string; param: string }> {
  const files: Array<{ file: string; param: string }> = [];

  // File-type positional args (excluding "input")
  for (const arg of skill.command.args) {
    if (arg.type === "file" && arg.name !== "input" && params[arg.name]) {
      const value = String(params[arg.name]);
      if (value) files.push({ file: value, param: arg.name });
    }
  }

  // FILE_PATH_INPUT_OPTIONS
  for (const [key, value] of Object.entries(params)) {
    if (!value || typeof value !== "string") continue;
    const flag = paramKeyToFlag(key);
    if (FILE_PATH_INPUT_OPTIONS.has(flag)) {
      files.push({ file: value, param: flag });
    }
  }

  return files;
}

/** Format successful tool result, handling temp files and performance tips. */
export async function formatToolResult(
  result: import("./types.js").SkillResult,
  commandName: string,
  inputFile: string | undefined,
  outputFile: string | undefined,
  autoCreatedTempFile: boolean,
  params: Record<string, unknown>,
) {
  // Snapshot the working directory to avoid TOCTOU issues if it changes mid-execution.
  const workDir = currentWorkingDir;

  let responseText = "";
  // Track the final output file path for the pipeline manifest.
  // For auto-created temp files, this becomes the renamed path in the working dir
  // (or undefined if the temp file was small enough to return inline and deleted).
  let finalOutputFile: string | undefined = outputFile;

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
          const savedExt = config.outputFormat === "tsv" && !NON_TABULAR_COMMANDS.has(commandName) && !isBinaryOutputFormat(commandName, params) ? "tsv" : "csv";
          const savedFileName = `qsv-${commandName}-${timestamp}.${savedExt}`;
          const savedPath = join(workDir, savedFileName);

          try {
            await rename(outputFile, savedPath);
          } catch (renameErr: unknown) {
            // Cross-device rename fails with EXDEV; fall back to copy + delete
            if (isNodeError(renameErr) && renameErr.code === "EXDEV") {
              await copyFile(outputFile, savedPath);
              await unlink(outputFile);
            } else {
              throw renameErr;
            }
          }
          console.error(`[MCP Tools] Saved large output to: ${savedPath}`);
          finalOutputFile = savedPath;

          responseText = `✅ Large output saved to file (too large to display in chat)\n\n`;
          responseText += `File: ${savedFileName}\n`;
          responseText += `Location: ${workDir}\n`;
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
          finalOutputFile = undefined; // temp file was deleted; output is inline
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

  // sqlp note: CSV inputs are now auto-converted to Parquet before SQL execution
  // No manual conversion tip needed — ensureParquet() handles it in handleToolCall()

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

  const formatted = successResult(responseText);
  (formatted as Record<string | symbol, unknown>)[FINAL_OUTPUT_FILE] = finalOutputFile;
  return formatted;
}

/**
 * Resolve LLM-friendly parameter aliases to their canonical names.
 * LLMs sometimes send "input"/"output" instead of "input_file"/"output_file".
 * Canonical names take precedence when both are present.
 */
export function resolveParamAliases(params: Record<string, unknown>): {
  inputFile: string | undefined;
  outputFile: string | undefined;
} {
  // Only accept string values; reject numbers, booleans, objects, etc.
  // A numeric value (e.g. { input: 42 }) is almost certainly an LLM mistake,
  // not a valid file path, so we return undefined rather than coercing to "42".
  const coerce = (v: unknown, paramName: string): string | undefined => {
    if (typeof v === "string") return v.trim() || undefined;
    if (v != null) {
      console.error(
        `[MCP Tools] Ignoring non-string ${paramName} value: ${typeof v} (${JSON.stringify(v)})`,
      );
    }
    return undefined;
  };

  const inputFile =
    coerce(params.input_file, "input_file") ??
    coerce(params.input, "input");
  const outputFile =
    coerce(params.output_file, "output_file") ??
    coerce(params.output, "output");
  return { inputFile, outputFile };
}
