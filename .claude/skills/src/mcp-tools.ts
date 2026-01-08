/**
 * MCP Tool Definitions and Handlers for QSV Commands
 */

import { spawn, type ChildProcess } from 'child_process';
import { stat, access } from 'fs/promises';
import { constants } from 'fs';
import { basename } from 'path';
import { ConvertedFileManager } from './converted-file-manager.js';
import type { QsvSkill, Argument, Option, McpToolDefinition, McpToolProperty, FilesystemProviderExtended } from './types.js';
import type { SkillExecutor } from './executor.js';
import type { SkillLoader } from './loader.js';
import { config } from './config.js';
import { formatBytes, findSimilarFiles } from './utils.js';

/**
 * Auto-indexing threshold in MB
 */
const AUTO_INDEX_SIZE_MB = 10;

/**
 * Commands that always return full CSV data and should use temp files
 */
const ALWAYS_FILE_COMMANDS = new Set([
  'stats',
  'moarstats',
  'frequency',
  'sort',
  'dedup',
  'join',
  'joinp',
  'select',
  'search',
  'searchset',
  'apply',
  'applydp',
  'schema',
  'validate',
  'diff',
  'cat',
  'transpose',
  'flatten',
  'unflatten',
  'partition',
  'split',
  'explode',
  'pseudo',
  'rename',
  'replace',
  'datefmt',
  'formatters',
  'reverse',
  'safenames',
  'sqlp',
  'pivotp',
  'to',
  'tojsonl',
]);

/**
 * Commands that return small metadata (not full CSV) and should use stdout
 */
const METADATA_COMMANDS = new Set([
  'count',
  'headers',
  'index',
  'slice',
  'sample',
]);

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
    throw new Error('Server is shutting down, operation rejected');
  }

  return new Promise((resolve, reject) => {
    const proc = spawn(qsvBin, args, {
      stdio: ['ignore', 'ignore', 'pipe']
    });

    // Track this process
    activeProcesses.add(proc);

    let stderr = '';
    let timedOut = false;

    // Cleanup function
    const cleanup = () => {
      clearTimeout(timer);
      activeProcesses.delete(proc);
    };

    // Set up timeout
    const timer = setTimeout(() => {
      timedOut = true;
      proc.kill('SIGTERM');
      cleanup();
      reject(new Error(`Operation timed out after ${timeoutMs}ms: ${qsvBin} ${args.join(' ')}`));
    }, timeoutMs);

    proc.stderr?.on('data', (chunk) => {
      stderr += chunk.toString();
    });

    proc.on('close', (code) => {
      cleanup();
      if (!timedOut) {
        if (code === 0) {
          resolve();
        } else {
          reject(new Error(`Command failed with exit code ${code}: ${stderr}`));
        }
      }
    });

    proc.on('error', (err) => {
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
function isFilesystemProviderExtended(obj: unknown): obj is FilesystemProviderExtended {
  return (
    typeof obj === 'object' &&
    obj !== null &&
    'resolvePath' in obj &&
    'needsConversion' in obj &&
    'getConversionCommand' in obj &&
    'getWorkingDirectory' in obj &&
    typeof (obj as any).resolvePath === 'function' &&
    typeof (obj as any).needsConversion === 'function' &&
    typeof (obj as any).getConversionCommand === 'function' &&
    typeof (obj as any).getWorkingDirectory === 'function'
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
      filename.endsWith('.csv') || filename.endsWith('.tsv') ||
      filename.endsWith('.tab') || filename.endsWith('.ssv');

    if (!isIndexable) {
      return; // Not an indexable format
    }

    const stats = await stat(filePath);
    const fileSizeMB = stats.size / (1024 * 1024);
    const indexPath = filePath + '.idx';

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
      console.error(`[MCP Tools] File is ${fileSizeMB.toFixed(1)}MB, creating index...`);

      const qsvBin = getQsvBinaryPath();
      const indexArgs = ['index', filePath];

      try {
        await runQsvWithTimeout(qsvBin, indexArgs);
        console.error(`[MCP Tools] Index created successfully: ${indexPath}`);
      } catch (error) {
        // Don't fail if indexing fails or times out - just log and continue
        console.error(`[MCP Tools] Index creation failed (continuing anyway):`, error);
      }
    } else if (indexExists) {
      console.error(`[MCP Tools] Index already exists: ${indexPath}`);
    } else {
      console.error(`[MCP Tools] File is ${fileSizeMB.toFixed(1)}MB, skipping auto-indexing`);
    }
  } catch (error) {
    console.error(`[MCP Tools] Auto-indexing error (continuing anyway):`, error);
  }
}

/**
 * Determine if a command should use a temp output file
 *
 * @param command - The qsv command name
 * @param inputFile - Path to the input file
 * @returns Promise<boolean> - true if temp file should be used
 */
async function shouldUseTempFile(command: string, inputFile: string): Promise<boolean> {
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
    console.error(`[MCP Tools] Error checking file size for temp file decision:`, error);
    return false;
  }
}

/**
 * 20 most common qsv commands exposed as individual MCP tools
 */
export const COMMON_COMMANDS = [
  'select',      // Column selection (most frequently used)
  'stats',       // Statistical analysis
  'frequency',   // Value distribution
  'search',      // Pattern-based filtering
  'sort',        // Sorting operations
  'dedup',       // Duplicate removal
  'join',        // CSV joining
  'count',       // Row counting
  'headers',     // Header operations
  'slice',       // Row selection
  'apply',       // Column transformations
  'rename',      // Column renaming
  'schema',      // Schema inference
  'validate',    // Data validation
  'sample',      // Random sampling
  'moarstats',   // Comprehensive statistics with data type inference
  'index',       // Create index for fast random access
  'template',    // Template-based transformations
  'diff',        // Compare two CSV files
  'cat',         // Concatenate CSV files
] as const;

/**
 * Convert a QSV skill to an MCP tool definition
 */
export function createToolDefinition(skill: QsvSkill): McpToolDefinition {
  const properties: Record<string, McpToolProperty> = {
    input_file: {
      type: 'string',
      description: 'Path to input CSV file (absolute or relative)',
    },
  };

  const required: string[] = ['input_file'];

  // Add positional arguments
  if (skill.command.args && Array.isArray(skill.command.args)) {
    for (const arg of skill.command.args) {
      properties[arg.name] = {
        type: mapArgumentType(arg.type),
        description: arg.description,
      };
      if (arg.required) {
        required.push(arg.name);
      }
    }
  }

  // Add options
  if (skill.command.options && Array.isArray(skill.command.options)) {
    for (const opt of skill.command.options) {
    const optName = opt.flag.replace(/^--/, '').replace(/-/g, '_');

    if (opt.type === 'flag') {
      properties[optName] = {
        type: 'boolean',
        description: opt.description,
      };
    } else {
      properties[optName] = {
        type: mapOptionType(opt.type),
        description: opt.description,
      };
      if (opt.default) {
        properties[optName].default = opt.default;
      }
    }
    }
  }

  // Add output_file (optional for all commands)
  properties.output_file = {
    type: 'string',
    description: 'Path to output CSV file (optional). For large results or data transformation commands, a temp file is automatically used if omitted.',
  };

  return {
    name: skill.name.replace('qsv-', 'qsv_'),
    description: skill.description,
    inputSchema: {
      type: 'object',
      properties,
      required: required.length > 0 ? required : undefined,
    },
  };
}

/**
 * Map QSV argument types to JSON Schema types
 */
function mapArgumentType(type: string): 'string' | 'number' | 'boolean' | 'object' | 'array' {
  switch (type) {
    case 'number':
      return 'number';
    case 'file':
    case 'regex':
    case 'string':
    default:
      return 'string';
  }
}

/**
 * Map QSV option types to JSON Schema types
 */
function mapOptionType(type: string): 'string' | 'number' | 'boolean' | 'object' | 'array' {
  switch (type) {
    case 'number':
      return 'number';
    case 'string':
    default:
      return 'string';
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
      content: [{
        type: 'text' as const,
        text: `Error: Maximum concurrent operations limit reached (${config.maxConcurrentOperations}). Please wait for current operations to complete.`,
      }],
      isError: true,
    };
  }

  try {
    // Extract command name from tool name (qsv_select -> select)
    const commandName = toolName.replace('qsv_', '');

    // Load the skill
    const skillName = `qsv-${commandName}`;
    const skill = await loader.load(skillName);

    if (!skill) {
      // Calculate remaining commands dynamically
      const totalCommands = loader.getStats().total;
      const remainingCommands = totalCommands - COMMON_COMMANDS.length;

      return {
        content: [{
          type: 'text' as const,
          text: `Error: Skill '${skillName}' not found.\n\n` +
                `Please verify the command name is correct. ` +
                `Available commands include: ${COMMON_COMMANDS.join(', ')}, and ${remainingCommands} others. ` +
                `Use 'qsv_command' with the 'command' parameter for less common commands.`,
        }],
        isError: true,
      };
    }

    // Extract input_file and output_file
    let inputFile = params.input_file as string | undefined;
    let outputFile = params.output_file as string | undefined;

    if (!inputFile) {
      return {
        content: [{
          type: 'text' as const,
          text: 'Error: input_file parameter is required',
        }],
        isError: true,
      };
    }

    // Resolve file paths using filesystem provider if available
    if (filesystemProvider) {
      try {
        const originalInputFile = inputFile;
        inputFile = await filesystemProvider.resolvePath(inputFile);
        console.error(`[MCP Tools] Resolved input file: ${originalInputFile} -> ${inputFile}`);

        // Check if file needs conversion (Excel or JSONL to CSV)
        if (isFilesystemProviderExtended(filesystemProvider)) {
          const provider = filesystemProvider;

          if (provider.needsConversion(inputFile)) {
            const conversionCmd = provider.getConversionCommand(inputFile);
            if (!conversionCmd) {
              throw new Error(`Unable to determine conversion command for: ${inputFile}`);
            }
            console.error(`[MCP Tools] File requires conversion using qsv ${conversionCmd}`);

            // Convert file using qsv excel or qsv jsonl
            try {
              const qsvBin = getQsvBinaryPath();

              // Generate unique converted file path with UUID to prevent collisions
              const { randomUUID } = await import('crypto');
              // Use 16 hex chars (64 bits) for better collision resistance
              // Remove hyphens to get pure hex digits (randomUUID() includes hyphens)
              // 8 hex chars (32 bits) has 50% collision probability after ~65k conversions
              // 16 hex chars (64 bits) has 50% collision probability after ~4 billion conversions
              const uuid = randomUUID().replace(/-/g, '').substring(0, 16);
              let convertedPath = `${inputFile}.converted.${uuid}.csv`;

              // Validate the generated converted path for defense-in-depth
              // Even though it's derived from already-validated inputFile, ensure it's safe
              try {
                convertedPath = await provider.resolvePath(convertedPath);
              } catch (error) {
                throw new Error(`Invalid converted file path: ${convertedPath} - ${error}`);
              }

              // Initialize converted file manager
              const workingDir = provider.getWorkingDirectory();
              const convertedManager = new ConvertedFileManager(workingDir);

              // Clean up orphaned entries and partial conversions first
              await convertedManager.cleanupOrphanedEntries();

              // Check if we can reuse an existing converted file
              // Note: This looks for any .converted.*.csv file for this source
              const { basename: getBasename, dirname: getDirname, join: joinPath } = await import('path');
              const { readdir } = await import('fs/promises');

              const baseName = getBasename(inputFile);
              const pattern = `${baseName}.converted.`;
              let validConverted: string | null = null;

              // Search for existing converted files in the same directory as the input file
              try {
                const dir = getDirname(inputFile);
                const files = await readdir(dir);

                for (const file of files) {
                  if (file.startsWith(pattern) && file.endsWith('.csv')) {
                    const filePath = joinPath(dir, file);
                    validConverted = await convertedManager.getValidConvertedFile(inputFile, filePath);
                    if (validConverted) break;
                  }
                }
              } catch (error) {
                // If readdir fails, just proceed with conversion
                console.error('[MCP Tools] Error searching for existing converted file:', error);
              }

              if (validConverted) {
                // Reuse existing converted file and update timestamp
                await convertedManager.touchConvertedFile(inputFile);
                inputFile = validConverted;
                console.error(`[MCP Tools] Reusing existing conversion: ${validConverted}`);
              } else {
                // Register conversion start for failure tracking
                await convertedManager.registerConversionStart(inputFile, convertedPath);

                try {
                  // Run conversion command: qsv excel/jsonl <input> --output <converted.csv>
                  const conversionArgs = [conversionCmd, inputFile, '--output', convertedPath];
                  console.error(`[MCP Tools] Running conversion: ${qsvBin} ${conversionArgs.join(' ')}`);

                  await runQsvWithTimeout(qsvBin, conversionArgs);

                  // Conversion succeeded - first register the converted file in the cache
                  await convertedManager.registerConvertedFile(inputFile, convertedPath);

                  // Only mark conversion as complete after successful cache registration
                  await convertedManager.registerConversionComplete(inputFile);
                  // Use the converted CSV as input
                  inputFile = convertedPath;
                  console.error(`[MCP Tools] Conversion successful: ${convertedPath}`);

                  // Auto-index the converted CSV
                  await autoIndexIfNeeded(convertedPath);
                } catch (conversionError) {
                  // Conversion failed - clean up partial file
                  try {
                    const { unlink } = await import('fs/promises');
                    await unlink(convertedPath);
                    console.error(`[MCP Tools] Cleaned up partial conversion file: ${convertedPath}`);
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
                content: [{
                  type: 'text' as const,
                  text: `Error converting ${originalInputFile}: ${conversionError instanceof Error ? conversionError.message : String(conversionError)}`,
                }],
                isError: true,
              };
            }
          }
        }

        // Auto-index native CSV files if they're large enough and not indexed
        // Note: Snappy-compressed files (.sz) cannot be indexed
        await autoIndexIfNeeded(inputFile);

        if (outputFile) {
          const originalOutputFile = outputFile;
          outputFile = await filesystemProvider.resolvePath(outputFile);
          console.error(`[MCP Tools] Resolved output file: ${originalOutputFile} -> ${outputFile}`);
        }
      } catch (error) {
        console.error(`[MCP Tools] Error resolving file path:`, error);

        // Build enhanced error message with file suggestions
        let errorMessage = `Error resolving file path: ${error instanceof Error ? error.message : String(error)}`;

        // Add file suggestions if this looks like a file-not-found error and we have filesystem provider
        if (filesystemProvider && inputFile) {
          const errorStr = error instanceof Error ? error.message : String(error);
          if (errorStr.includes('outside allowed') ||
              errorStr.includes('not exist') ||
              errorStr.includes('cannot access') ||
              errorStr.includes('ENOENT')) {
            try {
              // Get list of available files
              const { resources } = await filesystemProvider.listFiles(undefined, false);

              if (resources.length > 0) {
                // Find similar files using fuzzy matching
                const suggestions = findSimilarFiles(inputFile, resources, 3);

                errorMessage += '\n\n';

                // Show suggestions if we found close matches
                if (suggestions.length > 0 && suggestions[0].distance <= inputFile.length / 2) {
                  errorMessage += 'Did you mean one of these?\n';
                  suggestions.forEach(({ name, distance }) => {
                    errorMessage += `  - ${name}\n`;
                  });
                } else {
                  // Show available files if no close matches
                  errorMessage += `Available files in working directory (${filesystemProvider.getWorkingDirectory()}):\n`;
                  resources.slice(0, 5).forEach(file => {
                    errorMessage += `  - ${file.name}\n`;
                  });

                  if (resources.length > 5) {
                    errorMessage += `  ... and ${resources.length - 5} more file${resources.length - 5 !== 1 ? 's' : ''}`;
                  }
                }
              }
            } catch (listError) {
              // If listing files fails, just show the original error
              console.error(`[MCP Tools] Failed to list files for suggestions:`, listError);
            }
          }
        }

        return {
          content: [{
            type: 'text' as const,
            text: errorMessage,
          }],
          isError: true,
        };
      }
    }

    // Determine if we should use a temp file for output
    let autoCreatedTempFile = false;
    if (!outputFile && await shouldUseTempFile(commandName, inputFile)) {
      // Auto-create temp file
      const { randomUUID } = await import('crypto');
      const { tmpdir } = await import('os');
      const { join } = await import('path');

      const tempFileName = `qsv-output-${randomUUID()}.csv`;
      outputFile = join(tmpdir(), tempFileName);
      autoCreatedTempFile = true;

      console.error(`[MCP Tools] Auto-created temp output file: ${outputFile}`);
    }

    // Build args and options
    const args: Record<string, unknown> = {};
    const options: Record<string, unknown> = {};

    // Add input file as 'input' argument if the skill expects it
    if (skill.command.args.some(a => a.name === 'input')) {
      args.input = inputFile;
      console.error(`[MCP Tools] Added input arg: ${inputFile}`);
    }

    for (const [key, value] of Object.entries(params)) {
      // Skip input_file and output_file (already handled)
      // Also skip 'input' if we already set it from input_file
      if (key === 'input_file' || key === 'output_file' || (key === 'input' && args.input)) {
        continue;
      }

      // Check if this is a positional argument
      const isArg = skill.command.args.some(a => a.name === key);
      if (isArg) {
        args[key] = value;
      } else {
        // It's an option - convert underscore to dash
        const optFlag = `--${key.replace(/_/g, '-')}`;
        options[optFlag] = value;
      }
    }

    // Add output file option if provided
    if (outputFile) {
      options['--output'] = outputFile;
    }

    console.error(`[MCP Tools] Executing skill with args:`, JSON.stringify(args));
    console.error(`[MCP Tools] Executing skill with options:`, JSON.stringify(options));

    // Execute the skill
    const result = await executor.execute(skill, {
      args,
      options,
    });

    // Format result
    if (result.success) {
      let responseText = '';

      if (outputFile) {
        if (autoCreatedTempFile) {
          // Check temp file size before deciding how to handle it
          try {
            const { stat, readFile, unlink, rename } = await import('fs/promises');
            const { join } = await import('path');

            const tempFileStats = await stat(outputFile);

            if (tempFileStats.size > MAX_MCP_RESPONSE_SIZE) {
              // Output too large for MCP response - save to working directory instead
              console.error(`[MCP Tools] Output file (${formatBytes(tempFileStats.size)}) exceeds MCP response limit (${formatBytes(MAX_MCP_RESPONSE_SIZE)})`);

              const timestamp = new Date().toISOString().replace(/[:.]/g, '-').replace('T', '_').split('.')[0];
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
              console.error(`[MCP Tools] Output file (${formatBytes(tempFileStats.size)}) is small enough to return directly`);
              const fileContents = await readFile(outputFile, 'utf-8');

              // Clean up temp file
              try {
                await unlink(outputFile);
                console.error(`[MCP Tools] Deleted temp file: ${outputFile}`);
              } catch (unlinkError) {
                console.error(`[MCP Tools] Failed to delete temp file:`, unlinkError);
              }

              // Return the file contents
              responseText = fileContents;
            }
          } catch (readError) {
            console.error(`[MCP Tools] Failed to process temp file:`, readError);
            return {
              content: [{
                type: 'text' as const,
                text: `Error processing output from temp file: ${readError instanceof Error ? readError.message : String(readError)}`,
              }],
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
        content: [{
          type: 'text' as const,
          text: responseText,
        }],
      };
    } else {
      return {
        content: [{
          type: 'text' as const,
          text: `Error executing ${commandName}:\n${result.stderr}`,
        }],
        isError: true,
      };
    }
  } catch (error) {
    return {
      content: [{
        type: 'text' as const,
        text: `Unexpected error: ${error instanceof Error ? error.message : String(error)}`,
      }],
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
        content: [{
          type: 'text' as const,
          text: 'Error: command parameter is required',
        }],
        isError: true,
      };
    }

    // Forward to handleToolCall with the qsv_ prefix
    return await handleToolCall(
      `qsv_${commandName}`,
      params,
      executor,
      loader,
      filesystemProvider,
    );
  } catch (error) {
    return {
      content: [{
        type: 'text',
        text: `Unexpected error: ${error instanceof Error ? error.message : String(error)}`,
      }],
      isError: true,
    };
  }
}

/**
 * Create the generic qsv_command tool definition
 */
export function createGenericToolDefinition(loader: SkillLoader): McpToolDefinition {
  // Calculate remaining commands dynamically
  const totalCommands = loader.getStats().total;
  const remainingCommands = totalCommands - COMMON_COMMANDS.length;

  return {
    name: 'qsv_command',
    description: `Execute any qsv command not exposed as a dedicated tool (${remainingCommands} additional commands available)`,
    inputSchema: {
      type: 'object',
      properties: {
        command: {
          type: 'string',
          description: 'The qsv command to execute (e.g., "to", "flatten", "partition")',
        },
        input_file: {
          type: 'string',
          description: 'Path to input CSV file (absolute or relative)',
        },
        args: {
          type: 'object',
          description: 'Command arguments as key-value pairs',
        },
        options: {
          type: 'object',
          description: 'Command options as key-value pairs',
        },
        output_file: {
          type: 'string',
          description: 'Path to output CSV file (optional). For large results or data transformation commands, a temp file is automatically used if omitted.',
        },
      },
      required: ['command', 'input_file'],
    },
  };
}

/**
 * Create qsv_welcome tool definition
 */
export function createWelcomeTool(): McpToolDefinition {
  return {
    name: 'qsv_welcome',
    description: 'Display welcome message and quick start guide for qsv',
    inputSchema: {
      type: 'object',
      properties: {},
      required: [],
    },
  };
}

/**
 * Create qsv_examples tool definition
 */
export function createExamplesTool(): McpToolDefinition {
  return {
    name: 'qsv_examples',
    description: 'Show common qsv usage examples and workflows',
    inputSchema: {
      type: 'object',
      properties: {},
      required: [],
    },
  };
}

/**
 * Handle qsv_welcome tool call
 */
export async function handleWelcomeTool(filesystemProvider?: FilesystemProviderExtended): Promise<{ content: Array<{ type: string; text: string }> }> {
  // Get list of available files in working directory
  let fileListingSection = '';

  if (filesystemProvider) {
    try {
      const { resources } = await filesystemProvider.listFiles(undefined, false);

      if (resources.length > 0) {
        const maxFiles = 10;
        const filesToShow = resources.slice(0, maxFiles);
        const workingDir = filesystemProvider.getWorkingDirectory();

        fileListingSection = `\n## 📁 Available Files in Your Working Directory

I found ${resources.length} file${resources.length !== 1 ? 's' : ''} in \`${workingDir}\`:

| File | Size | Type | Modified |
|------|------|------|----------|
`;

        filesToShow.forEach(file => {
          const description = file.description || file.name;
          const descMatch = description.match(/^(.+?) \((.+?) (\d{4}-\d{2}-\d{2})\)$/);
          const fileName = descMatch ? descMatch[1] : file.name;
          const fileSize = descMatch ? descMatch[2] : '';
          const fileDate = descMatch ? descMatch[3] : '';

          let fileType = 'CSV';
          const mimeType = file.mimeType || '';
          if (mimeType.includes('excel') || mimeType.includes('spreadsheet')) {
            fileType = 'Excel';
          } else if (mimeType.includes('ndjson')) {
            fileType = 'JSONL';
          } else if (mimeType.includes('tab-separated')) {
            fileType = 'TSV';
          } else if (mimeType.includes('snappy')) {
            fileType = 'Snappy';
          }

          fileListingSection += `| ${fileName} | ${fileSize} | ${fileType} | ${fileDate} |\n`;
        });

        if (resources.length > maxFiles) {
          fileListingSection += `\n_... and ${resources.length - maxFiles} more file${resources.length - maxFiles !== 1 ? 's' : ''}_\n`;
        }

        fileListingSection += `\n**Tip:** Use these file names in qsv commands, for example:\n- \`qsv_stats with input_file: "${filesToShow[0].name}"\`\n- \`qsv_headers with input_file: "${filesToShow[0].name}"\`\n`;
      }
    } catch (error) {
      console.error('Error listing files for welcome tool:', error);
    }
  }

  const welcomeText = `# Welcome to qsv Data Wrangling! 🎉

I'm your qsv assistant, ready to help you wrangle CSV, Excel, and JSONL files with ease.

## What is qsv?

qsv is a blazingly-fast command-line toolkit with 66 commands for:
- ✅ **Transforming** data (select, rename, replace, apply)
- ✅ **Analyzing** data (stats, frequency, describe)
- ✅ **Validating** data (schema, validate, safenames)
- ✅ **Querying** data with SQL (sqlp, joinp)
- ✅ **Converting** formats (Excel, JSONL, Parquet)

## 🔒 Privacy & Security

- **100% local processing** - your data never leaves your machine
- **Restricted access** - only works with directories you approve
- **No cloud uploads** - all operations happen on your computer
${fileListingSection}
## Quick Start

**1. List your CSV files**
\`\`\`
List CSV files in my Downloads folder
\`\`\`

**2. Preview a file**
\`\`\`
Show me the first few rows of data.csv
\`\`\`

**3. Get statistics**
\`\`\`
Calculate statistics for sales.csv
\`\`\`

**4. Search and filter**
\`\`\`
Find all rows in orders.csv where status is 'pending'
\`\`\`

**5. Join files**
\`\`\`
Join customers.csv and orders.csv on customer_id
\`\`\`

## Pro Tips

💡 **Auto-indexing**: Files over 10MB are automatically indexed for faster operations
💡 **Stats cache**: Run \`qsv_stats\` first to speed up other commands
💡 **Pipelines**: Combine multiple commands for complex workflows
💡 **Excel support**: Works with .xlsx and .ods files automatically

## Need Help?

- Type \`qsv_examples\` to see common usage patterns
- Ask me anything like "How do I filter rows?" or "Show me statistics for column X"
- All 66 qsv commands are available - just describe what you want to do!

Ready to start wrangling data? 🚀`;

  return {
    content: [{ type: 'text', text: welcomeText }],
  };
}

/**
 * Handle qsv_examples tool call
 */
export async function handleExamplesTool(): Promise<{ content: Array<{ type: string; text: string }> }> {
  const examplesText = `# Common qsv Usage Examples

## Data Exploration

**Preview a CSV file:**
\`\`\`
Show me the first 10 rows of data.csv
\`\`\`

**Get column statistics:**
\`\`\`
Calculate statistics for all columns in sales.csv
\`\`\`

**Show value frequency:**
\`\`\`
Show the frequency distribution of the 'status' column in orders.csv
\`\`\`

## Data Cleaning

**Remove duplicates:**
\`\`\`
Remove duplicate rows from customers.csv and save as cleaned.csv
\`\`\`

**Fill missing values:**
\`\`\`
Fill empty cells in the 'price' column with 0 in products.csv
\`\`\`

**Rename columns:**
\`\`\`
Rename column 'old_name' to 'new_name' in data.csv
\`\`\`

## Data Transformation

**Select specific columns:**
\`\`\`
Select only 'name', 'email', and 'phone' columns from contacts.csv
\`\`\`

**Filter rows:**
\`\`\`
Filter rows where 'age' is greater than 25 in users.csv
\`\`\`

**Sort data:**
\`\`\`
Sort sales.csv by 'date' in descending order
\`\`\`

## Data Analysis

**Join two files:**
\`\`\`
Join customers.csv and orders.csv on 'customer_id' column
\`\`\`

**Run SQL queries:**
\`\`\`
Run SQL: SELECT category, COUNT(*) as total FROM products.csv GROUP BY category
\`\`\`

**Calculate aggregates:**
\`\`\`
Calculate sum, average, min, and max for 'revenue' column in sales.csv
\`\`\`

## Advanced Workflows

**Multi-step pipeline:**
\`\`\`
1. Filter sales.csv for rows where region='West'
2. Select only date, product, and amount columns
3. Sort by amount descending
4. Save to west_sales.csv
\`\`\`

**Convert Excel to CSV:**
\`\`\`
Convert sheet 'Sales' from report.xlsx to sales.csv
\`\`\`

**Validate data schema:**
\`\`\`
Validate data.csv against schema.json and show validation errors
\`\`\`

## Tips for Better Results

✅ **Be specific**: Include column names and file names in your requests
✅ **Chain operations**: I can combine multiple steps into efficient pipelines
✅ **Use natural language**: Describe what you want - I'll figure out the right qsv commands
✅ **Check outputs**: I'll save results to files for you to review

Need more help? Just ask! 🚀`;

  return {
    content: [{ type: 'text', text: examplesText }],
  };
}

/**
 * Initiate graceful shutdown
 */
export function initiateShutdown(): void {
  isShuttingDown = true;
  console.error(`[MCP Tools] Shutdown initiated, ${activeProcesses.size} active processes`);
}

/**
 * Kill all active child processes
 */
export function killAllProcesses(): void {
  for (const proc of activeProcesses) {
    try {
      proc.kill('SIGTERM');
    } catch {
      // Process might have already exited
    }
  }
  activeProcesses.clear();
  console.error('[MCP Tools] All child processes terminated');
}

/**
 * Get count of active processes
 */
export function getActiveProcessCount(): number {
  return activeProcesses.size;
}
