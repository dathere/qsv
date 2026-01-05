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

/**
 * Auto-indexing threshold in MB
 */
const AUTO_INDEX_SIZE_MB = 10;

/**
 * Default timeout for conversion and indexing operations (5 minutes)
 */
const DEFAULT_OPERATION_TIMEOUT_MS = 5 * 60 * 1000;

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
  return process.env.QSV_MCP_BIN_PATH || 'qsv';
}

/**
 * Run a qsv command with timeout and process tracking
 */
async function runQsvWithTimeout(
  qsvBin: string,
  args: string[],
  timeoutMs: number = DEFAULT_OPERATION_TIMEOUT_MS,
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
  for (const arg of skill.command.args) {
    properties[arg.name] = {
      type: mapArgumentType(arg.type),
      description: arg.description,
    };
    if (arg.required) {
      required.push(arg.name);
    }
  }

  // Add options
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

  // Add output_file (optional for all commands)
  properties.output_file = {
    type: 'string',
    description: 'Path to output CSV file (optional, returns to stdout if omitted)',
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
  filesystemProvider?: { resolvePath: (path: string) => Promise<string> },
) {
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
              // Use 16 chars (64 bits) for better collision resistance
              // 8 chars (32 bits) has 50% collision probability after ~65k conversions
              // 16 chars (64 bits) has 50% collision probability after ~4 billion conversions
              const uuid = randomUUID().substring(0, 16);
              const convertedPath = `${inputFile}.converted.${uuid}.csv`;

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
        return {
          content: [{
            type: 'text' as const,
            text: `Error resolving file path: ${error instanceof Error ? error.message : String(error)}`,
          }],
          isError: true,
        };
      }
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
        responseText = `Successfully wrote output to: ${outputFile}\n\n`;
        responseText += `Metadata:\n`;
        responseText += `- Command: ${result.metadata.command}\n`;
        responseText += `- Duration: ${result.metadata.duration}ms\n`;
        if (result.metadata.rowsProcessed) {
          responseText += `- Rows processed: ${result.metadata.rowsProcessed}\n`;
        }
      } else {
        // Return the CSV output
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
  filesystemProvider?: { resolvePath: (path: string) => Promise<string> },
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
          description: 'Path to output CSV file (optional, returns to stdout if omitted)',
        },
      },
      required: ['command', 'input_file'],
    },
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
